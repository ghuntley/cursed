// Import Management System for CURSED
//
// This module provides comprehensive import resolution functionality including:
// - Standard library imports
// - Local module imports  
// - Package manager integration
// - Circular dependency detection
// - Import caching for performance

#[cfg(test)]
mod tests;

// Placeholder types for missing imports
#[derive(Debug, Clone)]
pub struct ImportResolver {
    pub config: ImportResolverConfig,
}

#[derive(Debug, Clone)]
pub struct ImportResolverConfig {
    pub search_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ResolvedImport {
    pub path: PathBuf,
    pub module: LoadedModule,
}

#[derive(Debug, Clone)]
pub struct LoadedModule {
    pub name: String,
    pub path: PathBuf,
    pub program: Option<Arc<Program>>,
    pub symbols: Vec<String>, // Exported symbols
}

#[derive(Debug)]
pub enum ImportSource {
    Local(PathBuf),
    Package(String),
    Stdlib(String),
}

#[derive(Debug)]
pub struct ModuleLoader {
    pub cache: HashMap<String, LoadedModule>,
}

#[derive(Debug)]
pub struct PackageImportResolver {
    pub packages: HashMap<String, PathBuf>,
}

impl ImportResolver {
    pub fn new(config: ImportResolverConfig) -> Self {
        Self { config }
    }
}

impl ImportResolverConfig {
    pub fn default() -> Self {
        Self {
            search_paths: vec![PathBuf::from(".")],
        }
    }
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }
    
    /// Get a module from cache or load it from file
    pub fn get_or_load_module(&mut self, path: &PathBuf) -> Result<LoadedModule> {
        let path_str = path.to_string_lossy().to_string();
        
        // Check cache first
        if let Some(module) = self.cache.get(&path_str) {
            return Ok(module.clone());
        }
        
        // Load the module
        let module = self.load_module_from_file(path)?;
        
        // Cache it
        self.cache.insert(path_str, module.clone());
        
        Ok(module)
    }
    
    /// Load a module from a file
    fn load_module_from_file(&self, path: &PathBuf) -> Result<LoadedModule> {
        if !path.exists() {
            return Err(CursedError::ImportError(format!("Import not found: {}", path.to_string_lossy())));
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| CursedError::ImportError(format!("IO error: {}", e)))?;
        
        // Parse the content to validate it's a valid CURSED file
        let program = crate::ast::parse_program(&content)?;
        
        // Extract exported symbols (for now, just function names)
        let symbols = self.extract_symbols(&program);
        
        Ok(LoadedModule {
            name: path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path.clone(),
            program: Some(Arc::new(program)),
            symbols,
        })
    }
    
    /// Clear the module cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Extract exported symbols from a program
    fn extract_symbols(&self, program: &Program) -> Vec<String> {
        use crate::ast::Statement;
        
        let mut symbols = Vec::new();
        
        for statement in &program.statements {
            match statement {
                Statement::Function(func) => {
                    symbols.push(func.name.clone());
                }
                // Add more symbol types as needed
                _ => {}
            }
        }
        
        symbols
    }
}

impl PackageImportResolver {
    pub fn new() -> Self {
        Self { packages: HashMap::new() }
    }
    
    /// Add a package to the resolver
    pub fn add_package(&mut self, name: String, path: PathBuf) {
        self.packages.insert(name, path);
    }
    
    /// Load packages from a packages.toml file
    pub fn load_packages_from_file(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Ok(()); // No packages file is fine
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| CursedError::ImportError(format!("IO error: {}", e)))?;
        
        // Simple TOML-like parsing for now
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((name, path_str)) = line.split_once('=') {
                let name = name.trim().trim_matches('"');
                let path_str = path_str.trim().trim_matches('"');
                self.packages.insert(name.to_string(), PathBuf::from(path_str));
            }
        }
        
        Ok(())
    }
    
    /// Check if a package is available
    pub fn has_package(&self, name: &str) -> bool {
        self.packages.contains_key(name)
    }
}

use crate::error::{CursedError, Result};
use crate::ast::{Program, ImportStatement as AstImportStatement};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;

/// Errors that can occur during import resolution
#[derive(Debug, Clone)]
pub enum ImportError {
    NotFound { import_path: String },
    CircularImport { cycle: Vec<String> },
    PackageNotInstalled { package: String },
    InvalidPath { path: String, reason: String },
    ModuleLoadError { module: String, error: String },
    PackageManagerError(String),
    IoError(String),
}

/// Main import resolution coordinator
#[derive(Debug)]
pub struct ImportManager {
    resolver: ImportResolver,
    package_resolver: PackageImportResolver,
    module_loader: ModuleLoader,
    cache: ImportCache,
    config: ImportConfig,
}

/// Cache for resolved imports to avoid repeated resolution
#[derive(Debug, Default)]
pub struct ImportCache {
    resolved_imports: HashMap<String, PathBuf>,
    failed_imports: HashMap<String, ImportError>,
}

/// Configuration for import resolution
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub search_paths: Vec<PathBuf>,
    pub stdlib_path: PathBuf,
    pub enable_package_manager: bool,
    pub cache_enabled: bool,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            search_paths: vec![PathBuf::from(".")],
            stdlib_path: PathBuf::from("stdlib"),
            enable_package_manager: false, // Disabled for minimal build
            cache_enabled: true,
        }
    }
}

impl ImportManager {
    /// Create new import manager with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ImportConfig::default())
    }

    /// Create new import manager with custom configuration  
    pub fn with_config(config: ImportConfig) -> Result<Self> {
        let resolver_config = ImportResolverConfig {
            search_paths: config.search_paths.clone(),
        };
        let resolver = ImportResolver::new(resolver_config);
        let package_resolver = PackageImportResolver::new();
        let module_loader = ModuleLoader::new();
        let cache = ImportCache::default();
        
        Ok(Self {
            resolver,
            package_resolver,
            module_loader,
            cache,
            config,
        })
    }

    /// Resolve all imports for a program
    pub async fn resolve_imports(&mut self, imports: &[ImportStatement]) -> Result<Vec<ResolvedImport>> {
        let mut resolved = Vec::new();
        let mut processing = HashSet::new();
        
        for import in imports {
            let resolved_import = self.resolve_single_import(import, &mut processing).await?;
            resolved.push(resolved_import);
        }
        
        Ok(resolved)
    }

    /// Resolve a single import statement
    pub async fn resolve_single_import(
        &mut self,
        import: &ImportStatement,
        processing: &mut HashSet<String>
    ) -> Result<ResolvedImport> {
        // Check for circular imports
        if processing.contains(&import.path) {
            let cycle: Vec<String> = processing.iter().cloned().collect();
            return Err(CursedError::ImportError(format!("Circular import detected: {:?}", cycle)));
        }
        
        // Check cache first
        if let Some(cached_path) = self.cache.resolved_imports.get(&import.path) {
            let module = self.module_loader.get_or_load_module(cached_path)?;
            return Ok(ResolvedImport {
                path: cached_path.clone(),
                module,
            });
        }
        
        // Check for failed imports
        if let Some(error) = self.cache.failed_imports.get(&import.path) {
            return Err(CursedError::ImportError(error.to_string()));
        }
        
        // Add to processing set
        processing.insert(import.path.clone());
        
        // Determine import source
        let import_source = self.classify_import(&import.path)?;
        
        // Resolve based on source type
        let resolved_path = match import_source {
            ImportSource::Local(path) => self.resolve_local_import(&path)?,
            ImportSource::Package(name) => self.resolve_package_import(&name)?,
            ImportSource::Stdlib(name) => self.resolve_stdlib_import(&name)?,
        };
        
        // Load the module
        let module = self.module_loader.get_or_load_module(&resolved_path)?;
        
        // Remove from processing set
        processing.remove(&import.path);
        
        // Cache the result
        self.cache.resolved_imports.insert(import.path.clone(), resolved_path.clone());
        
        Ok(ResolvedImport {
            path: resolved_path,
            module,
        })
    }

    /// Check if an import is cached
    pub fn is_cached(&self, import_path: &str) -> bool {
        self.cache.resolved_imports.contains_key(import_path)
    }

    /// Clear the import cache
    pub fn clear_cache(&mut self) {
        self.cache.resolved_imports.clear();
        self.cache.failed_imports.clear();
    }
    
    /// Classify an import path to determine its source type
    pub fn classify_import(&self, import_path: &str) -> Result<ImportSource> {
        if import_path.starts_with("std::") || import_path.starts_with("cursed::") {
            Ok(ImportSource::Stdlib(import_path.to_string()))
        } else if import_path.starts_with("./") || import_path.starts_with("../") || import_path.ends_with(".csd") {
            Ok(ImportSource::Local(PathBuf::from(import_path)))
        } else if !import_path.contains("/") && !import_path.contains("\\") {
            // Assume it's a package if it doesn't contain path separators
            Ok(ImportSource::Package(import_path.to_string()))
        } else {
            Ok(ImportSource::Local(PathBuf::from(import_path)))
        }
    }
    
    /// Resolve a local file import
    fn resolve_local_import(&self, path: &Path) -> Result<PathBuf> {
        // Try the path as-is first
        if path.is_absolute() {
            if path.exists() {
                return Ok(path.to_path_buf());
            }
            return Err(CursedError::ImportError(format!("Import not found: {}", path.to_string_lossy())));
        }
        
        // Try relative to current directory and search paths
        for search_path in &self.config.search_paths {
            let full_path = search_path.join(path);
            
            // Try with .csd extension if not present
            if full_path.exists() {
                return Ok(full_path);
            }
            
            if !path.to_string_lossy().ends_with(".csd") {
                let with_ext = full_path.with_extension("csd");
                if with_ext.exists() {
                    return Ok(with_ext);
                }
            }
        }
        
        Err(CursedError::ImportError(format!("Import not found: {}", path.to_string_lossy())))
    }
    
    /// Resolve a package import
    fn resolve_package_import(&self, package_name: &str) -> Result<PathBuf> {
        if let Some(package_path) = self.package_resolver.packages.get(package_name) {
            Ok(package_path.clone())
        } else {
            Err(CursedError::ImportError(format!("Package not installed: {}", package_name)))
        }
    }
    
    /// Resolve a standard library import
    fn resolve_stdlib_import(&self, stdlib_name: &str) -> Result<PathBuf> {
        let stdlib_path = self.config.stdlib_path.join(stdlib_name.replace("::", "/"));
        let with_ext = stdlib_path.with_extension("csd");
        
        if with_ext.exists() {
            Ok(with_ext)
        } else if stdlib_path.join("mod.csd").exists() {
            Ok(stdlib_path.join("mod.csd"))
        } else {
            Err(CursedError::ImportError(format!("Standard library import not found: {}", stdlib_name)))
        }
    }
}

/// Represents an import statement
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: String,
    pub alias: Option<String>,
    pub items: Vec<String>, // Specific items to import
    pub is_glob: bool, // Import everything (*)
}

impl Default for ImportManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default ImportManager")
    }
}

// Error conversion implementations
impl From<std::io::Error> for ImportError {
    fn from(err: std::io::Error) -> Self {
        ImportError::IoError(err.to_string())
    }
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::NotFound { import_path } => write!(f, "Import not found: {}", import_path),
            ImportError::CircularImport { cycle } => write!(f, "Circular import detected: {:?}", cycle),
            ImportError::PackageNotInstalled { package } => write!(f, "Package not installed: {}", package),
            ImportError::InvalidPath { path, reason } => write!(f, "Invalid import path: {} - {}", path, reason),
            ImportError::ModuleLoadError { module, error } => write!(f, "Module load error: {} - {}", module, error),
            ImportError::PackageManagerError(msg) => write!(f, "Package manager error: {}", msg),
            ImportError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ImportError {}
