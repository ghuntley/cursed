//! CURSED Module Resolution and Import System
//! 
//! This module provides comprehensive import resolution functionality including:
//! - Local .csd file imports
//! - Standard library imports  
//! - Package manager integration
//! - Circular dependency detection
//! - Module compilation and caching

use crate::error::{CursedError, Result};
use crate::ast::{Program, ImportStatement};
use crate::package_manager::{PackageManager, PackageManagerConfig, PackageInfo};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, Mutex};

/// Errors that can occur during import resolution
#[derive(Debug, Clone)]
pub enum ImportError {
    NotFound { import_path: String },
    CircularImport { cycle: Vec<String> },
    PackageNotInstalled { package: String },
    InvalidPath { path: String, reason: String },
    ModuleLoadError { module: String, error: String },
    CompilationError { module: String, error: String },
    PackageManagerError(String),
    IoError(String),
}

/// Import source classification
#[derive(Debug, Clone)]
pub enum ImportSource {
    Local(PathBuf),
    Package(String, Option<String>), // (name, version)
    Stdlib(String),
}

/// A successfully resolved import
#[derive(Debug, Clone)]
pub struct ResolvedImport {
    pub path: PathBuf,
    pub module: CompiledModule,
    pub source: ImportSource,
    pub symbols: Vec<String>,
}

/// A compiled CURSED module
#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub name: String,
    pub path: PathBuf,
    pub program: Arc<Program>,
    pub exported_symbols: Vec<String>,
    pub dependencies: Vec<String>,
    pub compilation_time: std::time::SystemTime,
}

/// Configuration for import resolution
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub search_paths: Vec<PathBuf>,
    pub stdlib_path: PathBuf,
    pub package_cache_dir: PathBuf,
    pub enable_package_manager: bool,
    pub cache_enabled: bool,
    pub max_circular_depth: usize,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            search_paths: vec![PathBuf::from(".")],
            stdlib_path: PathBuf::from("stdlib"),
            package_cache_dir: PathBuf::from(".cursed/packages"),
            enable_package_manager: true,
            cache_enabled: true,
            max_circular_depth: 64,
        }
    }
}

/// Cache for resolved and compiled modules
#[derive(Debug, Default)]
pub struct ModuleCache {
    compiled_modules: HashMap<PathBuf, CompiledModule>,
    resolution_cache: HashMap<String, PathBuf>,
    failed_imports: HashMap<String, ImportError>,
}

/// Main import resolver and module loader
#[derive(Debug)]
pub struct ImportResolver {
    config: ImportConfig,
    cache: ModuleCache,
    package_manager: Option<Arc<Mutex<PackageManager>>>,
    compilation_stack: Vec<String>, // Track modules being compiled to detect cycles
}

impl ImportResolver {
    /// Create a new import resolver with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ImportConfig::default())
    }

    /// Create a new import resolver with custom configuration
    pub fn with_config(config: ImportConfig) -> Result<Self> {
        let package_manager = if config.enable_package_manager {
            match PackageManager::new(PackageManagerConfig::default()) {
                Ok(pm) => Some(Arc::new(Mutex::new(pm))),
                Err(_) => {
                    eprintln!("Warning: Package manager initialization failed, packages disabled");
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            config,
            cache: ModuleCache::default(),
            package_manager,
            compilation_stack: Vec::new(),
        })
    }

    /// Resolve all imports for a program
    pub async fn resolve_imports(&mut self, imports: &[ImportStatement]) -> Result<Vec<ResolvedImport>> {
        let mut resolved = Vec::new();
        
        for import in imports {
            let resolved_import = self.resolve_single_import(import).await?;
            resolved.push(resolved_import);
        }
        
        Ok(resolved)
    }

    /// Resolve a single import statement
    pub async fn resolve_single_import(&mut self, import: &ImportStatement) -> Result<ResolvedImport> {
        // Check for circular imports
        if self.compilation_stack.contains(&import.path) {
            return Err(CursedError::ImportError(format!(
                "Circular import detected: {} -> {}", 
                self.compilation_stack.join(" -> "), 
                import.path
            )));
        }

        // Check if compilation depth is too deep
        if self.compilation_stack.len() > self.config.max_circular_depth {
            return Err(CursedError::ImportError(format!(
                "Import depth limit exceeded ({}): {}", 
                self.config.max_circular_depth,
                import.path
            )));
        }

        // Check cache first
        if let Some(cached_path) = self.cache.resolution_cache.get(&import.path) {
            if let Some(cached_module) = self.cache.compiled_modules.get(cached_path) {
                return Ok(ResolvedImport {
                    path: cached_path.clone(),
                    module: cached_module.clone(),
                    source: self.classify_import(&import.path)?,
                    symbols: cached_module.exported_symbols.clone(),
                });
            }
        }

        // Check for failed imports
        if let Some(error) = self.cache.failed_imports.get(&import.path) {
            return Err(CursedError::ImportError(error.to_string()));
        }

        // Add to compilation stack
        self.compilation_stack.push(import.path.clone());

        // Resolve and compile
        let result = self.resolve_and_compile_import(import).await;

        // Remove from compilation stack
        self.compilation_stack.pop();

        match result {
            Ok(resolved) => {
                // Cache successful resolution
                self.cache.resolution_cache.insert(import.path.clone(), resolved.path.clone());
                self.cache.compiled_modules.insert(resolved.path.clone(), resolved.module.clone());
                Ok(resolved)
            }
            Err(error) => {
                // Cache failed import
                let import_error = ImportError::ModuleLoadError {
                    module: import.path.clone(),
                    error: error.to_string(),
                };
                self.cache.failed_imports.insert(import.path.clone(), import_error.clone());
                Err(error)
            }
        }
    }

    /// Internal method to resolve and compile an import
    async fn resolve_and_compile_import(&mut self, import: &ImportStatement) -> Result<ResolvedImport> {
        // Classify the import source
        let import_source = self.classify_import(&import.path)?;
        
        // Resolve to actual file path
        let resolved_path = match &import_source {
            ImportSource::Local(path) => self.resolve_local_import(path)?,
            ImportSource::Package(name, version) => self.resolve_package_import(name, version.as_deref()).await?,
            ImportSource::Stdlib(name) => self.resolve_stdlib_import(name)?,
        };

        // Compile the module
        let compiled_module = self.compile_module(&resolved_path).await?;

        // Extract symbols based on import specification
        let symbols = if import.items.is_empty() {
            // Import all exported symbols
            compiled_module.exported_symbols.clone()
        } else {
            // Import only specified symbols
            let available_symbols: HashSet<_> = compiled_module.exported_symbols.iter().collect();
            let mut imported_symbols = Vec::new();
            
            for item in &import.items {
                if available_symbols.contains(item) {
                    imported_symbols.push(item.clone());
                } else {
                    return Err(CursedError::ImportError(format!(
                        "Symbol '{}' not found in module '{}'", 
                        item, 
                        import.path
                    )));
                }
            }
            
            imported_symbols
        };

        Ok(ResolvedImport {
            path: resolved_path,
            module: compiled_module,
            source: import_source,
            symbols,
        })
    }

    /// Classify an import path to determine its source type
    pub fn classify_import(&self, import_path: &str) -> Result<ImportSource> {
        if import_path.starts_with("std::") || import_path.starts_with("cursed::") {
            Ok(ImportSource::Stdlib(import_path.to_string()))
        } else if import_path.starts_with("./") || import_path.starts_with("../") || import_path.ends_with(".csd") {
            Ok(ImportSource::Local(PathBuf::from(import_path)))
        } else if import_path.contains("@") {
            // Package with version: "package@1.0.0"
            let parts: Vec<&str> = import_path.splitn(2, '@').collect();
            Ok(ImportSource::Package(parts[0].to_string(), Some(parts[1].to_string())))
        } else if !import_path.contains("/") && !import_path.contains("\\") {
            // Simple name - check if it exists in search paths before treating as package
            let path = PathBuf::from(import_path);
            if self.local_import_exists(&path) {
                Ok(ImportSource::Local(path))
            } else {
                Ok(ImportSource::Package(import_path.to_string(), None))
            }
        } else {
            // Default to local path
            Ok(ImportSource::Local(PathBuf::from(import_path)))
        }
    }

    /// Check if a local import exists in search paths
    fn local_import_exists(&self, path: &Path) -> bool {
        // Try absolute path first
        if path.is_absolute() {
            return path.exists();
        }

        // Try relative to search paths
        for search_path in &self.config.search_paths {
            let candidates = vec![
                search_path.join(path),
                search_path.join(path).with_extension("csd"),
                search_path.join(path).join("mod.csd"),
            ];

            for candidate in candidates {
                if candidate.exists() && candidate.is_file() {
                    return true;
                }
            }
        }

        false
    }

    /// Resolve a local file import
    fn resolve_local_import(&self, path: &Path) -> Result<PathBuf> {
        // Try absolute path first
        if path.is_absolute() {
            return if path.exists() {
                Ok(path.to_path_buf())
            } else {
                Err(CursedError::ImportError(format!("Import not found: {}", path.display())))
            };
        }

        // Try relative to search paths
        for search_path in &self.config.search_paths {
            let candidates = vec![
                search_path.join(path),
                search_path.join(path).with_extension("csd"),
                search_path.join(path).join("mod.csd"),
            ];

            for candidate in candidates {
                if candidate.exists() && candidate.is_file() {
                    return Ok(candidate);
                }
            }
        }

        Err(CursedError::ImportError(format!("Import not found: {}", path.display())))
    }

    /// Resolve a package import
    fn resolve_package_import<'a>(&'a self, package_name: &'a str, version: Option<&'a str>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<PathBuf>> + Send + 'a>> {
        Box::pin(async move {
        if let Some(pm_arc) = &self.package_manager {
            // Try to find installed package
            let package_path = self.config.package_cache_dir.join(package_name);
            
            if package_path.exists() {
                // Check for main module file
                let candidates = vec![
                    package_path.join("lib.csd"),
                    package_path.join("main.csd"),
                    package_path.join("mod.csd"),
                    package_path.join(format!("{}.csd", package_name)),
                ];

                for candidate in candidates {
                    if candidate.exists() {
                        return Ok(candidate);
                    }
                }
            }

            // For now, return an error indicating package is not found
            // Actual installation would require spawning a task or using different architecture
            return Err(CursedError::ImportError(
                format!("Package '{}' not found. Package installation is available but disabled due to async constraints", package_name)
            ));
        }

        Err(CursedError::ImportError(format!(
            "Package '{}' not found and package manager is disabled", 
            package_name
        )))
        })
    }



    /// Resolve a standard library import
    fn resolve_stdlib_import(&self, stdlib_name: &str) -> Result<PathBuf> {
        // Convert std::module::submodule to stdlib/module/submodule.csd
        let path_parts: Vec<&str> = stdlib_name.split("::").collect();
        if path_parts.is_empty() {
            return Err(CursedError::ImportError("Invalid stdlib import path".to_string()));
        }

        // Skip the "std" or "cursed" prefix
        let module_parts = if path_parts[0] == "std" || path_parts[0] == "cursed" {
            &path_parts[1..]
        } else {
            &path_parts
        };

        let module_path = self.config.stdlib_path.join(module_parts.join("/"));
        
        let candidates = vec![
            module_path.with_extension("csd"),
            module_path.join("mod.csd"),
            module_path.join("lib.csd"),
        ];

        for candidate in candidates {
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(CursedError::ImportError(format!(
            "Standard library module not found: {}", 
            stdlib_name
        )))
    }

    /// Compile a CURSED module from source
    async fn compile_module(&mut self, path: &PathBuf) -> Result<CompiledModule> {
        // Check if already compiled and up-to-date
        if let Some(cached) = self.cache.compiled_modules.get(path) {
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    if modified <= cached.compilation_time {
                        return Ok(cached.clone());
                    }
                }
            }
        }

        // Read source file
        let source = fs::read_to_string(path)
            .map_err(|e| CursedError::ImportError(format!("Failed to read module {}: {}", path.display(), e)))?;

        // Parse the source
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::from_tokens(tokens);
        let program = parser.parse_program()?;

        // Extract exported symbols
        let exported_symbols = self.extract_exported_symbols(&program);

        // Extract dependencies
        let dependencies = program.imports.iter().map(|imp| imp.path.clone()).collect();

        // Note: Dependency resolution disabled to avoid recursive async issues
        // TODO: Implement iterative dependency resolution
        let _dependencies = &program.imports; // Acknowledge but don't process

        let compiled_module = CompiledModule {
            name: path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path.clone(),
            program: Arc::new(program),
            exported_symbols,
            dependencies,
            compilation_time: std::time::SystemTime::now(),
        };

        Ok(compiled_module)
    }

    /// Extract exported symbols from a program
    fn extract_exported_symbols(&self, program: &Program) -> Vec<String> {
        use crate::ast::{Statement, Visibility};
        
        let mut symbols = Vec::new();
        
        for statement in &program.statements {
            match statement {
                Statement::Function(func) => {
                    // Only export public functions
                    if func.visibility == Visibility::Public {
                        symbols.push(func.name.clone());
                    }
                }
                Statement::Let(let_stmt) => {
                    // Only export public constants
                    if let_stmt.visibility == Visibility::Public {
                        symbols.push(let_stmt.name.clone());
                    }
                }
                // Add more exportable statement types as needed
                _ => {}
            }
        }
        
        symbols
    }

    /// Clear the module cache
    pub fn clear_cache(&mut self) {
        self.cache.compiled_modules.clear();
        self.cache.resolution_cache.clear();
        self.cache.failed_imports.clear();
    }

    /// Check if an import is cached
    pub fn is_cached(&self, import_path: &str) -> bool {
        self.cache.resolution_cache.contains_key(import_path)
    }

    /// Get import statistics
    pub fn get_stats(&self) -> ImportStats {
        ImportStats {
            cached_modules: self.cache.compiled_modules.len(),
            cached_resolutions: self.cache.resolution_cache.len(),
            failed_imports: self.cache.failed_imports.len(),
            compilation_depth: self.compilation_stack.len(),
        }
    }
}

/// Import resolution statistics
#[derive(Debug)]
pub struct ImportStats {
    pub cached_modules: usize,
    pub cached_resolutions: usize,
    pub failed_imports: usize,
    pub compilation_depth: usize,
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
            ImportError::CompilationError { module, error } => write!(f, "Compilation error in {}: {}", module, error),
            ImportError::PackageManagerError(msg) => write!(f, "Package manager error: {}", msg),
            ImportError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ImportError {}

impl Default for ImportResolver {
    fn default() -> Self {
        Self::new().expect("Failed to create default ImportResolver")
    }
}

/// Convenience function to resolve imports for a program
pub async fn resolve_program_imports(program: &Program) -> Result<Vec<ResolvedImport>> {
    let mut resolver = ImportResolver::new()?;
    resolver.resolve_imports(&program.imports).await
}

/// Convenience function to check if a module exists
pub fn module_exists(import_path: &str) -> bool {
    match ImportResolver::new() {
        Ok(resolver) => {
            match resolver.classify_import(import_path) {
                Ok(ImportSource::Local(path)) => resolver.resolve_local_import(&path).is_ok(),
                Ok(ImportSource::Stdlib(name)) => resolver.resolve_stdlib_import(&name).is_ok(),
                _ => false,
            }
        }
        Err(_) => false,
    }
}
