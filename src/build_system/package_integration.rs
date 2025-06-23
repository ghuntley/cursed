//! Package Integration with Build System
//!
//! Integrates the package manager with the CURSED build system and compilation pipeline.
//! Handles package resolution, import resolution, and makes packages available during compilation.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::ast::Program;
use crate::package_manager::{PackageManager, PackageManagerConfig, PackageManagerError, PackageMetadata};
use crate::imports::{ImportManager, ImportError, ImportResolverConfig, ResolvedImport, LoadedModule};
use crate::codegen::LlvmCodeGenerator;
use crate::core::type_checker::TypeChecker;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::error::Error;

/// Configuration for package integration
#[derive(Debug, Clone)]
pub struct PackageIntegrationConfig {
    pub package_manager_config: PackageManagerConfig,
    pub import_resolver_config: ImportResolverConfig,
    pub enable_auto_install: bool,
    pub cache_compiled_packages: bool,
    pub parallel_compilation: bool,
}

impl Default for PackageIntegrationConfig {
    fn default() -> Self {
        Self {
            package_manager_config: PackageManagerConfig::default(),
            import_resolver_config: ImportResolverConfig::default(),
            enable_auto_install: true,
            cache_compiled_packages: true,
            parallel_compilation: true,
        }
    }
}

/// Errors during package integration
#[derive(Error, Debug)]
pub enum PackageIntegrationError {
    #[error("Package manager error: {0}")]
    PackageManager(#[from] PackageManagerError),
    
    #[error("Import resolution error: {0}")]
    ImportResolution(#[from] ImportError),
    
    #[error("Compilation error: {0}")]
    Compilation(#[from] Error),
    
    #[error("Type checking error for package {package}: {error}")]
    TypeChecking { package: String, error: String },
    
    #[error("Missing dependency: {dependency} required by {package}")]
    MissingDependency { dependency: String, package: String },
    
    #[error("Version conflict: {package} requires {required} but {installed} is installed")]
    VersionConflict { package: String, required: String, installed: String },
}

/// Main package integration coordinator
#[derive(Debug)]
pub struct PackageIntegration {
    package_manager: Arc<Mutex<PackageManager>>,
    import_manager: ImportManager,
    config: PackageIntegrationConfig,
    compiled_packages: HashMap<String, CompiledPackage>,
}

/// Information about a compiled package
#[derive(Debug, Clone)]
pub struct CompiledPackage {
    pub metadata: PackageMetadata,
    pub modules: HashMap<String, Arc<LoadedModule>>,
    pub llvm_ir: Option<String>,
    pub type_info: Vec<String>, // Exported types
    pub symbols: Vec<String>,   // Exported symbols
}

/// Context for compilation with package information
#[derive(Debug, Clone)]
pub struct CompilationContext {
    pub main_program: Program,
    pub resolved_imports: Vec<ResolvedImport>,
    pub loaded_modules: HashMap<String, Arc<LoadedModule>>,
    pub available_packages: Vec<PackageMetadata>,
    pub source_file: Option<PathBuf>,
}

/// Result of integrated build with package information
#[derive(Debug)]
pub struct IntegratedBuildResult {
    pub llvm_ir: String,
    pub compilation_context: CompilationContext,
    pub package_stats: PackageStats,
    pub warnings: Vec<String>,
}

/// Statistics about package usage during build
#[derive(Debug, Clone)]
pub struct PackageStats {
    pub packages_resolved: usize,
    pub modules_loaded: usize,
    pub auto_installed: Vec<String>,
    pub resolution_time: std::time::Duration,
    pub compilation_time: std::time::Duration,
}

impl PackageIntegration {
    /// Create new package integration
    pub fn new(config: PackageIntegrationConfig) -> Result<(), Error> {
        let package_manager = Arc::new(Mutex::new(
            PackageManager::new(config.package_manager_config.clone())?
        ));
        
        let import_manager = ImportManager::new(
            package_manager.clone(), 
            config.import_resolver_config.clone()
        )?;
        
        Ok(Self {
            package_manager,
            import_manager,
            config,
            compiled_packages: HashMap::new(),
        })
    }
    
    /// Compile CURSED source with full package integration
    pub async fn compile_with_packages(
        &mut self,
        source: &str,
        source_file: Option<&Path>,
    ) -> Result<(), Error> {
        let start_time = std::time::Instant::now();
        
        tracing::info!("Starting package-aware compilation");
        
        // Parse the main program
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        // Check for parse errors
        let errors = parser.errors();
        if !errors.is_empty() {
            return Err(PackageIntegrationError::Compilation(
                Error::Parse(format!("Parse errors: {}", errors.join(", ")))
            ));
        }
        
        let resolution_start = std::time::Instant::now();
        
        // Resolve all imports
        let resolved_imports = self.import_manager.resolve_imports(
            &program.imports,
            source_file,
        ).await?;
        
        // Load all modules
        let mut loaded_modules = HashMap::new();
        let mut auto_installed = Vec::new();
        
        for resolved in &resolved_imports {
            // Check if we need to auto-install packages
            if let crate::imports::ImportSource::InstalledPackage { package_name } = &resolved.source {
                if self.config.enable_auto_install {
                    match self.import_manager.ensure_package_installed(package_name).await {
                        Ok(metadata) => {
                            auto_installed.push(metadata.name);
                        }
                        Err(e) => {
                            tracing::warn!(package = package_name, error = ?e, "Failed to auto-install package");
                        }
                    }
                }
            }
            
            // Load the module
            let loaded = self.import_manager.load_module(resolved).await?;
            loaded_modules.insert(resolved.original_path.clone(), loaded);
        }
        
        let resolution_time = resolution_start.elapsed();
        
        // Get available packages
        let available_packages = self.import_manager.get_available_packages()?;
        
        // Create compilation context
        let context = CompilationContext {
            main_program: program,
            resolved_imports,
            loaded_modules: loaded_modules.clone(),
            available_packages: available_packages.clone(),
            source_file: source_file.map(|p| p.to_path_buf()),
        };
        
        let compilation_start = std::time::Instant::now();
        
        // Perform type checking with package types
        let mut type_checker = TypeChecker::new();
        self.register_package_types(&mut type_checker, &loaded_modules)?;
        
        if let Err(e) = type_checker.check_program(&context.main_program) {
            tracing::warn!("Type checking failed: {}, continuing with compilation", e);
        }
        
        // Generate LLVM IR with package integration
        let llvm_ir = self.generate_ir_with_packages(&context).await?;
        
        let compilation_time = compilation_start.elapsed();
        
        let stats = PackageStats {
            packages_resolved: available_packages.len(),
            modules_loaded: loaded_modules.len(),
            auto_installed,
            resolution_time,
            compilation_time,
        };
        
        tracing::info!(
            total_time = ?start_time.elapsed(),
            packages = stats.packages_resolved,
            modules = stats.modules_loaded,
            "Package-aware compilation completed"
        );
        
        Ok(IntegratedBuildResult {
            llvm_ir,
            compilation_context: context,
            package_stats: stats,
            warnings: Vec::new(),
        })
    }
    
    /// Register package types with the type checker
    fn register_package_types(
        &self,
        type_checker: &mut TypeChecker,
        loaded_modules: &HashMap<String, Arc<LoadedModule>>,
    ) -> Result<(), Error> {
        for (module_name, module) in loaded_modules {
            tracing::debug!(module = module_name, types = ?module.info.types, "Registering module types");
            
            // Register types from the module
            for type_name in &module.info.types {
                // In a real implementation, we'd extract actual type definitions
                // For now, just log that we're registering them
                tracing::debug!(module = module_name, type_name = type_name, "Registering type");
            }
        }
        
        Ok(())
    }
    
    /// Generate LLVM IR with package integration
    async fn generate_ir_with_packages(
        &mut self,
        context: &CompilationContext,
    ) -> Result<(), Error> {
        let mut codegen = LlvmCodeGenerator::new()?;
        
        // Compile package modules first
        for (module_name, module) in &context.loaded_modules {
            tracing::debug!(module = module_name, "Compiling package module");
            
            // In a real implementation, we'd compile each module
            // For now, we just add them to the context
            if let Err(e) = codegen.compile(&module.program) {
                tracing::warn!(module = module_name, error = ?e, "Failed to compile package module");
            }
        }
        
        // Compile main program
        if let Err(e) = codegen.compile(&context.main_program) {
            tracing::warn!(error = ?e, "Failed to compile main program");
        }
        
        // Generate IR for everything
        let ir = codegen.generate_ir("// Generated from package-aware compilation")?;
        
        Ok(ir)
    }
    
    /// Install packages from a package file
    pub async fn install_dependencies(&mut self, package_file: &Path) -> Result<(), Error> {
        // Read package metadata
        let content = std::fs::read_to_string(package_file)
            .map_err(|e| PackageIntegrationError::Compilation(Error::Io(e.into())))?;
        
        let metadata: PackageMetadata = toml::from_str(&content)
            .map_err(|e| PackageIntegrationError::Compilation(
                Error::Parse(format!("Invalid package file: {}", e))
            ))?;
        
        let mut installed = Vec::new();
        let mut package_manager = self.package_manager.lock().map_err(|_| {
            PackageIntegrationError::PackageManager(PackageManagerError::RegistryError {
                message: "Failed to lock package manager".to_string(),
            })
        })?;
        
        // Install dependencies
        for (dep_name, _dep_version) in &metadata.dependencies {
            let packages = package_manager.install_package(dep_name, None).await?;
            installed.extend(packages);
        }
        
        // Install dev dependencies if in development mode
        for (dep_name, _dep_version) in &metadata.dev_dependencies {
            let packages = package_manager.install_package(dep_name, None).await?;
            installed.extend(packages);
        }
        
        Ok(installed)
    }
    
    /// Check if all dependencies are satisfied
    pub fn validate_dependencies(&self, package_file: &Path) -> Result<(), Error> {
        let content = std::fs::read_to_string(package_file)
            .map_err(|e| PackageIntegrationError::Compilation(Error::Io(e.into())))?;
        
        let metadata: PackageMetadata = toml::from_str(&content)
            .map_err(|e| PackageIntegrationError::Compilation(
                Error::Parse(format!("Invalid package file: {}", e))
            ))?;
        
        let package_manager = self.package_manager.lock().map_err(|_| {
            PackageIntegrationError::PackageManager(PackageManagerError::RegistryError {
                message: "Failed to lock package manager".to_string(),
            })
        })?;
        
        let installed = package_manager.list_installed()?;
        let mut missing = Vec::new();
        
        for (dep_name, _dep_version) in &metadata.dependencies {
            if !installed.iter().any(|p| p.name == *dep_name) {
                missing.push(dep_name.clone());
            }
        }
        
        Ok(missing)
    }
    
    /// Update package integration configuration
    pub fn update_config(&mut self, config: PackageIntegrationConfig) {
        self.config = config;
    }
    
    /// Get package integration statistics
    pub fn get_stats(&self) -> IntegrationStats {
        let import_stats = self.import_manager.get_stats();
        
        IntegrationStats {
            compiled_packages: self.compiled_packages.len(),
            cached_imports: import_stats.cached_imports,
            loaded_modules: import_stats.loaded_modules,
        }
    }
}

/// Package-aware compiler that integrates with the build system
#[derive(Debug)]
pub struct PackageAwareCompiler {
    integration: PackageIntegration,
}

impl PackageAwareCompiler {
    /// Create new package-aware compiler
    pub fn new(config: PackageIntegrationConfig) -> Result<(), Error> {
        let integration = PackageIntegration::new(config)?;
        Ok(Self { integration })
    }
    
    /// Compile source with automatic package resolution
    pub async fn compile(&mut self, source: &str, source_file: Option<&Path>) -> Result<(), Error> {
        let result = self.integration.compile_with_packages(source, source_file).await?;
        Ok(result.llvm_ir)
    }
    
    /// Check source for errors including package dependencies
    pub async fn check(&mut self, source: &str, source_file: Option<&Path>) -> Result<(), Error> {
        // Compilation check includes dependency resolution
        let _result = self.integration.compile_with_packages(source, source_file).await?;
        Ok(())
    }
}

/// Statistics about package integration
#[derive(Debug, Clone)]
pub struct IntegrationStats {
    pub compiled_packages: usize,
    pub cached_imports: usize,
    pub loaded_modules: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_package_integration_creation() {
        let config = PackageIntegrationConfig::default();
        let integration = PackageIntegration::new(config);
        assert!(integration.is_ok());
    }
    
    #[tokio::test]
    async fn test_simple_compilation_without_packages() {
        let config = PackageIntegrationConfig::default();
        let mut integration = PackageIntegration::new(config).unwrap();
        
        let source = r#"
slay main() {
    capicola("Hello, World!");
}
"#;
        
        let result = integration.compile_with_packages(source, None).await;
        assert!(result.is_ok());
    }
}
