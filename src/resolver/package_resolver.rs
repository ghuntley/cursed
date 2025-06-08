//! Main package resolver that coordinates all resolution activities

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tracing::instrument;
use super::dependency_tracker::DependencyTracker;
use super::package_loader::PackageLoader;
use super::standard_library::StandardLibraryResolver;
use super::symbol_table::{PackageSymbolTable};
use super::errors::{ResolverError, ResolverResult};

/// Main package resolver that coordinates package resolution
#[derive(Debug)]
pub struct PackageResolver {
    /// Dependency tracker for cycle detection
    dependency_tracker: DependencyTracker,
    /// Package loader for filesystem packages
    package_loader: PackageLoader,
    /// Standard library resolver
    stdlib_resolver: StandardLibraryResolver,
    /// Registry of resolved packages
    registry: HashMap<String, PackageSymbolTable>,
    /// Current working directory for relative imports
    working_directory: PathBuf,
    /// Currently resolving packages (for error context)
    resolution_stack: Vec<String>,
}

impl PackageResolver {
    /// Create a new package resolver
    pub fn new() -> Self {
        Self {
            dependency_tracker: DependencyTracker::new(),
            package_loader: PackageLoader::with_default_paths(),
            stdlib_resolver: StandardLibraryResolver::new(),
            registry: HashMap::new(),
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            resolution_stack: Vec::new(),
        }
    }
    
    /// Create a package resolver with custom working directory
    pub fn with_working_directory<P: AsRef<Path>>(working_dir: P) -> Self {
        let mut resolver = Self::new();
        resolver.working_directory = working_dir.as_ref().to_path_buf();
        resolver.package_loader.add_search_path(&resolver.working_directory);
        resolver
    }
    
    /// Add a package search path
    pub fn add_search_path<P: AsRef<Path>>(&mut self, path: P) {
        self.package_loader.add_search_path(path);
    }
    
    /// Resolve a package by import path
    #[instrument(skip(self), level = "info")]
    pub fn resolve_package(&mut self, import_path: &str, requesting_package: Option<&str>) -> ResolverResult<PackageSymbolTable> {
        tracing::info!("Resolving package: {} (requested by: {:?})", import_path, requesting_package);
        
        // Parse the import path
        let package_name = self.parse_import_path(import_path)?;
        
        // Check if already resolved
        if let Some(existing) = self.registry.get(&package_name) {
            tracing::debug!("Package {} already resolved", package_name);
            return Ok(existing.clone());
        }
        
        // Add to resolution stack for cycle detection
        self.resolution_stack.push(package_name.clone());
        
        // Start dependency tracking
        self.dependency_tracker.start_loading(&package_name)?;
        
        let result = self.resolve_package_internal(&package_name, import_path, requesting_package);
        
        // Cleanup
        self.dependency_tracker.finish_loading(&package_name);
        self.resolution_stack.pop();
        
        result
    }
    
    /// Internal package resolution logic
    #[instrument(skip(self), level = "debug")]
    fn resolve_package_internal(
        &mut self,
        package_name: &str,
        import_path: &str,
        requesting_package: Option<&str>,
    ) -> ResolverResult<PackageSymbolTable> {
        // Check if it's a standard library package first
        if self.stdlib_resolver.is_stdlib_package(package_name) {
            tracing::debug!("Resolving standard library package: {}", package_name);
            let package = self.stdlib_resolver.get_package(package_name)?.clone();
            self.registry.insert(package.package_name().to_string(), package.clone());
            return Ok(package);
        }
        
        // Resolve from filesystem
        tracing::debug!("Resolving filesystem package: {}", package_name);
        let mut package = self.package_loader.resolve_package(package_name)?;
        
        // Add dependency relationship if we have a requesting package
        if let Some(requester) = requesting_package {
            self.dependency_tracker.add_dependency(requester, package_name);
            

        }
        
        // Dependencies are now tracked through the dependency tracker
        
        // Validate the package
        self.package_loader.validate_package(&package)?;
        
        // Register the package
        self.registry.insert(package.package_name().to_string(), package.clone());
        
        tracing::info!("Successfully resolved package: {}", package_name);
        Ok(package)
    }
    
    /// Parse an import path into a package name
    #[instrument(skip(self), level = "trace")]
    fn parse_import_path(&self, import_path: &str) -> ResolverResult<String> {
        // Handle different import path formats:
        // - "package" -> "package"
        // - "./package" -> "package" (relative)
        // - "../package" -> "package" (relative up)
        // - "/absolute/path/package" -> "package"
        // - "github.com/user/package" -> "package"
        
        if import_path.is_empty() {
            return Err(ResolverError::invalid_package_path(import_path, "Import path cannot be empty"));
        }
        
        // Extract the last component as the package name
        let package_name = if import_path.contains('/') {
            import_path.split('/').last().unwrap_or(import_path)
        } else {
            import_path
        };
        
        if package_name.is_empty() {
            return Err(ResolverError::invalid_package_path(import_path, "Package name cannot be empty"));
        }
        
        // Validate package name (alphanumeric + underscore, no spaces)
        if !package_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(ResolverError::invalid_package_path(
                import_path,
                "Package name must contain only alphanumeric characters and underscores",
            ));
        }
        
        Ok(package_name.to_string())
    }
    
    /// Resolve a symbol in a package
    #[instrument(skip(self), level = "debug")]
    pub fn resolve_symbol(&self, package_name: &str, symbol_name: &str) -> ResolverResult<super::symbol_table::Symbol> {
        // Check if package is resolved
        if let Some(package) = self.registry.get(package_name) {
            if let Some(symbol) = package.get_symbol(symbol_name) {
                Ok(symbol.clone())
            } else {
                Err(ResolverError::symbol_not_found(
                    package_name,
                    symbol_name,
                ))
            }
        } else {
            Err(ResolverError::package_not_found(package_name, vec![]))
        }
    }
    
    /// Check if a package is resolved
    pub fn is_package_resolved(&self, package_name: &str) -> bool {
        self.registry.contains_key(package_name)
    }
    
    /// Get all resolved packages
    pub fn resolved_packages(&self) -> Vec<String> {
        self.registry.keys().cloned().collect()
    }
    
    /// Get dependency order for compilation
    #[instrument(skip(self), level = "debug")]
    pub fn get_compilation_order(&self) -> ResolverResult<Vec<String>> {
        match self.dependency_tracker.topological_sort() {
            Some(order) => Ok(order),
            None => Err(ResolverError::General("Circular dependency detected".to_string()))
        }
    }
    
    /// Get package statistics
    pub fn get_package_stats(&self, package_name: &str) -> Option<super::package_loader::PackageStats> {
        self.registry.get(package_name)
            .map(|package| self.package_loader.get_package_stats(package))
    }
    
    /// Clear all resolved packages
    pub fn clear(&mut self) {
        self.registry.clear();
        self.dependency_tracker.clear();
        self.package_loader.clear_cache();
        self.resolution_stack.clear();
    }
    
    /// Get dependency information
    pub fn get_dependencies(&self, package_name: &str) -> Vec<String> {
        self.dependency_tracker.get_dependencies(package_name)
    }
    
    /// Get packages that depend on the given package
    pub fn get_dependents(&self, package_name: &str) -> Vec<String> {
        self.dependency_tracker.get_dependents(package_name)
    }
    
    /// Check if package A depends on package B
    pub fn package_depends_on(&self, package_a: &str, package_b: &str) -> bool {
        self.dependency_tracker.depends_on(package_a, package_b)
    }
    
    /// Validate import path format
    #[instrument(skip(self), level = "trace")]
    pub fn validate_import_path(&self, import_path: &str) -> ResolverResult<()> {
        self.parse_import_path(import_path).map(|_| ())
    }
    
    /// Get available standard library packages
    pub fn available_stdlib_packages(&self) -> Vec<String> {
        self.stdlib_resolver.available_packages()
    }
    
    /// Check if a package is from the standard library
    pub fn is_stdlib_package(&self, package_name: &str) -> bool {
        self.stdlib_resolver.is_stdlib_package(package_name)
    }
    
    /// Get the working directory
    pub fn working_directory(&self) -> &Path {
        &self.working_directory
    }
    
    /// Set the working directory
    pub fn set_working_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.working_directory = path.as_ref().to_path_buf();
    }
    
    /// Get current resolution stack (for debugging)
    pub fn resolution_stack(&self) -> &[String] {
        &self.resolution_stack
    }
    
    /// Resolve multiple packages in dependency order
    #[instrument(skip(self), level = "info")]
    pub fn resolve_packages(&mut self, import_paths: &[String], requesting_package: Option<&str>) -> ResolverResult<Vec<PackageSymbolTable>> {
        let mut resolved = Vec::new();
        
        for import_path in import_paths {
            let package = self.resolve_package(import_path, requesting_package)?;
            resolved.push(package);
        }
        
        Ok(resolved)
    }
    
    /// Export resolution summary
    pub fn export_summary(&self) -> ResolutionSummary {
        let packages = self.resolved_packages();
        let total_packages = packages.len();
        let stdlib_packages = packages.iter()
            .filter(|p| self.is_stdlib_package(p))
            .count();
        let user_packages = total_packages - stdlib_packages;
        
        ResolutionSummary {
            total_packages,
            stdlib_packages,
            user_packages,
            packages,
        }
    }
}

impl Default for PackageResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of package resolution
#[derive(Debug, Clone)]
pub struct ResolutionSummary {
    pub total_packages: usize,
    pub stdlib_packages: usize,
    pub user_packages: usize,
    pub packages: Vec<String>,
}

impl std::fmt::Display for ResolutionSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Resolved {} packages ({} stdlib, {} user): {}",
            self.total_packages,
            self.stdlib_packages,
            self.user_packages,
            self.packages.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_package_resolver_creation() {
        let resolver = PackageResolver::new();
        assert!(resolver.resolved_packages().is_empty());
    }
    
    #[test]
    fn test_parse_import_path() {
        let resolver = PackageResolver::new();
        
        assert_eq!(resolver.parse_import_path("package").unwrap(), "package");
        assert_eq!(resolver.parse_import_path("./package").unwrap(), "package");
        assert_eq!(resolver.parse_import_path("../package").unwrap(), "package");
        assert_eq!(resolver.parse_import_path("github.com/user/package").unwrap(), "package");
        
        assert!(resolver.parse_import_path("").is_err());
        assert!(resolver.parse_import_path("invalid package").is_err());
    }
    
    #[test]
    fn test_stdlib_package_resolution() {
        let mut resolver = PackageResolver::new();
        
        // Should be able to resolve core package
        let result = resolver.resolve_package("core", None);
        assert!(result.is_ok());
        
        let package = result.unwrap();
        assert_eq!(package.package_name(), "core");
        assert!(package.get_symbol("len").is_some());
    }
    
    #[test]
    fn test_resolution_summary() {
        let mut resolver = PackageResolver::new();
        let _ = resolver.resolve_package("core", None);
        
        let summary = resolver.export_summary();
        assert_eq!(summary.total_packages, 1);
        assert_eq!(summary.stdlib_packages, 1);
        assert_eq!(summary.user_packages, 0);
        assert!(summary.packages.contains(&"core".to_string()));
    }
}
