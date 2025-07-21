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

    /// Resolve all imports for a program with enhanced circular dependency detection
    pub async fn resolve_imports(&mut self, imports: &[ImportStatement]) -> Result<Vec<ResolvedImport>> {
        let mut resolved = Vec::new();
        let mut dependency_graph: HashMap<String, HashSet<String>> = HashMap::new();
        
        // First pass: build dependency graph to detect circular dependencies
        for import in imports {
            let import_paths = if import.path.is_empty() && !import.items.is_empty() {
                // Grouped import
                import.items.clone()
            } else {
                // Single import
                vec![import.path.clone()]
            };
            
            for path in &import_paths {
                let normalized_path = self.normalize_import_path(path)?;
                dependency_graph.entry(normalized_path.clone()).or_insert_with(HashSet::new);
                
                // Extract dependencies from this module
                if let Ok(module_dependencies) = self.get_module_dependencies(&normalized_path).await {
                    for dep in module_dependencies {
                        let normalized_dep = self.normalize_import_path(&dep)?;
                        dependency_graph.entry(normalized_path.clone())
                            .or_insert_with(HashSet::new)
                            .insert(normalized_dep);
                    }
                }
            }
        }
        
        // Check for circular dependencies
        self.detect_circular_dependencies(&dependency_graph)?;
        
        // Second pass: resolve imports in dependency order
        for import in imports {
            // Handle grouped imports (paths stored in items field, empty path indicates grouped import)
            if import.path.is_empty() && !import.items.is_empty() {
                // This is a grouped import - resolve each path individually
                for path in &import.items {
                    let individual_import = ImportStatement {
                        path: path.clone(),
                        alias: None,
                        items: Vec::new(),
                    };
                    let resolved_import = self.resolve_single_import(&individual_import).await?;
                    resolved.push(resolved_import);
                }
            } else {
                // Regular single import
                let resolved_import = self.resolve_single_import(import).await?;
                resolved.push(resolved_import);
            }
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
        // Handle std:: and cursed:: prefixed imports
        if import_path.starts_with("std::") || import_path.starts_with("cursed::") {
            return Ok(ImportSource::Stdlib(import_path.to_string()));
        }
        
        // Handle stdlib/ prefixed imports
        if import_path.starts_with("stdlib/") {
            return Ok(ImportSource::Stdlib(import_path.to_string()));
        }
        
        // Handle relative imports (../testz/mod, ./module, etc.)
        if import_path.starts_with("./") || import_path.starts_with("../") || import_path.ends_with(".csd") {
            return Ok(ImportSource::Local(PathBuf::from(import_path)));
        }
        
        // Handle package with version: "package@1.0.0"
        if import_path.contains("@") {
            let parts: Vec<&str> = import_path.splitn(2, '@').collect();
            return Ok(ImportSource::Package(parts[0].to_string(), Some(parts[1].to_string())));
        }
        
        // For simple names, check if it exists locally first
        if !import_path.contains("/") && !import_path.contains("\\") {
            let path = PathBuf::from(import_path);
            if self.local_import_exists(&path) {
                return Ok(ImportSource::Local(path));
            }
        }
        
        // Check if it's a known stdlib module (only if not found locally)
        if self.is_stdlib_module(import_path) {
            return Ok(ImportSource::Stdlib(import_path.to_string()));
        }
        
        // Handle any other path patterns - treat as local if they contain path separators
        if import_path.contains("/") || import_path.contains("\\") {
            return Ok(ImportSource::Local(PathBuf::from(import_path)));
        }
        
        // Default to package for simple names that don't exist locally
        Ok(ImportSource::Package(import_path.to_string(), None))
    }

    /// Check if a module name is a standard library module
    fn is_stdlib_module(&self, name: &str) -> bool {
        // Comprehensive list of stdlib modules with standardized names
        let stdlib_modules = [
            // Core modules
            "async", "collections", "core", "crypto", "error_drip", "fs", "io", "json", 
            "math", "memory", "net", "process", "string", "testz", "time", "vibez",
            
            // Extended modules  
            "asn1_mood", "atomic_drip", "big_mood", "binary_drip", "bytefit", 
            "chadlogging", "chaos_mode", "compression", "concurrenz", "config", 
            "csv", "debug_tea", "exec_slay", "glowup_http", "grammar_drip", 
            "hash_drip", "heap_slay", "htmlrizzler", "logging", "main_character", 
            "network", "no_cap", "pathing", "pem_drip", "regex", "rpc_vibes", 
            "serialization", "smtp_tea", "sort_slay", "spill_facts", "sql_slay", 
            "string_pure", "tls_vibe", "validation", "vibe_life", "vibe_lock", 
            "x509_certs_tea", "zip_zilla",
            
            // Legacy mappings for backward compatibility
            "mathz", "stringz", "ioz", "dropz", "timez", "encode_mood", "tab_aesthetic",
            
            // New modules that may have been added
            "mime_vibe", "unicode", "database", "web", "select_core", "signal_boost",
            "cryptz", "plugin_system", "stat_flexin", "jit_vibes", "text_aesthetic",
            "error_handling", "fmt", "macro_slay", "user_check", "option", "reflect",
            "vibe_net", "smtp_tea", "parser", "image_processing", "error_management",
            "sus_containers", "mood_map", "tag_core", "plug_vibes", "token_vibe",
            "command_line", "elliptic_curve_tea", "database_complete", "runtime_core",
            "panic_system", "vibe_context", "lookin_glass", "database_drivers"
        ];
        
        // Normalize the name for comparison
        let normalized_name = name.to_lowercase();
        
        // Check for direct module name
        if stdlib_modules.iter().any(|&module| module.to_lowercase() == normalized_name) {
            return true;
        }
        
        // Check if it's a stdlib path (starts with "stdlib/")
        if name.starts_with("stdlib/") {
            let module_name = name.strip_prefix("stdlib/").unwrap_or(name);
            if stdlib_modules.iter().any(|&module| module.to_lowercase() == module_name.to_lowercase()) {
                return true;
            }
        }
        
        // Check if the module exists in the stdlib directory with timeout
        let module_path = self.config.stdlib_path.join(&normalized_name);
        if crate::subprocess_utils::file_exists_with_timeout(&module_path, 10) || 
           crate::subprocess_utils::file_exists_with_timeout(&module_path.join("mod.csd"), 10) {
            return true;
        }
        
        // Check with original case
        let original_module_path = self.config.stdlib_path.join(name);
        crate::subprocess_utils::file_exists_with_timeout(&original_module_path, 10) || 
        crate::subprocess_utils::file_exists_with_timeout(&original_module_path.join("mod.csd"), 10)
    }

    /// Get the stdlib path mapping for a module name
    fn get_stdlib_path_mapping(&self, name: &str) -> Option<String> {
        match name {
            // Legacy mappings for backward compatibility
            "mathz" => Some("math".to_string()),
            "stringz" => Some("string".to_string()),
            "ioz" => Some("io".to_string()),
            
            // Handle stdlib/ prefixed paths
            path if path.starts_with("stdlib/") => {
                let module_name = path.strip_prefix("stdlib/").unwrap_or(path);
                Some(module_name.to_string())
            }
            
            // Direct module name - return as-is if it exists
            _ => {
                let module_path = self.config.stdlib_path.join(name);
                if module_path.exists() || module_path.join("mod.csd").exists() {
                    Some(name.to_string())
                } else {
                    None
                }
            }
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
                // Direct file path
                search_path.join(path),
                // Add .csd extension if not present
                search_path.join(path).with_extension("csd"),
                // Look for mod.csd in directory
                search_path.join(path).join("mod.csd"),
                // Look for lib.csd in directory  
                search_path.join(path).join("lib.csd"),
                // Handle relative imports like "../testz/mod"
                search_path.join(path).with_extension("csd"),
                // Handle imports like "../testz/mod.csd"
                search_path.join(path),
            ];

            for candidate in candidates {
                if candidate.exists() && candidate.is_file() {
                    return Ok(candidate);
                }
            }
        }

        // Special case: if it's a relative import, try from stdlib
        if path.starts_with("../") {
            let stdlib_relative = path.strip_prefix("../").unwrap_or(path);
            let stdlib_candidates = vec![
                self.config.stdlib_path.join(stdlib_relative),
                self.config.stdlib_path.join(stdlib_relative).with_extension("csd"),
                self.config.stdlib_path.join(stdlib_relative).join("mod.csd"),
            ];
            
            for candidate in stdlib_candidates {
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
        // Handle direct stdlib module names like "mathz", "stringz", "vibez"
        if let Some(actual_name) = self.get_stdlib_path_mapping(stdlib_name) {
            let module_path = self.config.stdlib_path.join(&actual_name);
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

            return Err(CursedError::ImportError(format!(
                "Standard library module not found: {} (mapped to {})", 
                stdlib_name, &actual_name
            )));
        }

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

        // Read source file with timeout
        let source = crate::subprocess_utils::read_file_with_timeout(path, 30)
            .map_err(|e| CursedError::ImportError(format!("Failed to read module {}: {}", path.display(), e)))?;

        // Basic syntax validation - check for obvious errors
        if self.has_syntax_errors(&source) {
            return Err(CursedError::ImportError(format!(
                "Module '{}' contains syntax errors",
                path.display()
            )));
        }

        // Extract exported symbols directly from source
        let exported_symbols = self.extract_exported_symbols_from_source(&source);

        // Extract dependencies (imports) from source  
        let dependencies = self.extract_dependencies_from_source(&source);

        let compiled_module = CompiledModule {
            name: path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path.clone(),
            program: Arc::new(Program { statements: vec![], imports: vec![], package: None }),
            exported_symbols,
            dependencies,
            compilation_time: std::time::SystemTime::now(),
        };

        Ok(compiled_module)
    }

    /// Basic syntax error detection
    fn has_syntax_errors(&self, source: &str) -> bool {
        // Check for unmatched parentheses, braces, and other obvious errors
        let mut paren_count = 0;
        let mut brace_count = 0;
        let mut bracket_count = 0;
        
        // Look for patterns that indicate syntax errors
        for line in source.lines() {
            let line = line.trim();
            
            // Check for unbalanced delimiters in function declarations
            if line.starts_with("spill slay ") {
                // Check if function declaration looks malformed
                if line.contains("(((") || line.contains("&&&") || line.contains(")))") {
                    return true;
                }
            }
            
            // Count delimiters
            for ch in line.chars() {
                match ch {
                    '(' => paren_count += 1,
                    ')' => paren_count -= 1,
                    '{' => brace_count += 1,
                    '}' => brace_count -= 1,
                    '[' => bracket_count += 1,
                    ']' => bracket_count -= 1,
                    _ => {}
                }
                
                // If we go negative, we have unmatched closing delimiters
                if paren_count < 0 || brace_count < 0 || bracket_count < 0 {
                    return true;
                }
            }
        }
        
        // Check if we have unmatched opening delimiters
        paren_count != 0 || brace_count != 0 || bracket_count != 0
    }

    /// Extract exported symbols from source code by parsing function declarations
    fn extract_exported_symbols_from_source(&self, source: &str) -> Vec<String> {
        let mut symbols = Vec::new();
        
        // Look for function declarations that start with "spill slay" (public functions)
        let lines = source.lines();
        for line in lines {
            let line = line.trim();
            
            // Match patterns like "spill slay function_name("
            if line.starts_with("spill slay ") {
                if let Some(rest) = line.strip_prefix("spill slay ") {
                    // Find the function name - everything until the first '('
                    if let Some(paren_pos) = rest.find('(') {
                        let function_name = rest[..paren_pos].trim();
                        if !function_name.is_empty() && function_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            symbols.push(function_name.to_string());
                        }
                    }
                }
            }
        }
        
        symbols
    }

    /// Extract dependencies (imports) from source code
    fn extract_dependencies_from_source(&self, source: &str) -> Vec<String> {
        let mut dependencies = Vec::new();
        
        // Look for import statements that start with "yeet"
        let lines = source.lines();
        for line in lines {
            let line = line.trim();
            
            // Match patterns like 'yeet "module_name";'
            if line.starts_with("yeet ") {
                if let Some(rest) = line.strip_prefix("yeet ") {
                    // Extract quoted string
                    if let Some(start_quote) = rest.find('"') {
                        if let Some(end_quote) = rest.rfind('"') {
                            if start_quote < end_quote {
                                let import_path = &rest[start_quote + 1..end_quote];
                                if !import_path.is_empty() {
                                    dependencies.push(import_path.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        dependencies
    }

    /// Extract exported symbols from a program (legacy method for compatibility)
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
                        symbols.push(let_stmt.target.primary_name());
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

    /// Normalize import path for consistent comparison
    pub fn normalize_import_path(&self, path: &str) -> Result<String> {
        // Remove redundant path separators and resolve relative paths
        let normalized = if path.starts_with("./") {
            path.strip_prefix("./").unwrap_or(path).to_string()
        } else if path.starts_with("../") {
            // For relative imports, keep them as-is for now
            path.to_string()
        } else {
            path.to_string()
        };
        
        // Convert to lowercase for case-insensitive comparison
        Ok(normalized.to_lowercase())
    }
    
    /// Get module dependencies without fully resolving the module
    pub async fn get_module_dependencies(&self, module_path: &str) -> Result<Vec<String>> {
        // Classify the import to find the actual file path
        let import_source = self.classify_import(module_path)?;
        
        let file_path = match &import_source {
            ImportSource::Local(path) => self.resolve_local_import(path)?,
            ImportSource::Package(name, version) => {
                // For packages, we can't easily get dependencies without resolving
                return Ok(Vec::new());
            }
            ImportSource::Stdlib(name) => self.resolve_stdlib_import(name)?,
        };
        
        // Read the file and extract dependencies with timeout
        let source = match crate::subprocess_utils::read_file_with_timeout(&file_path, 30) {
            Ok(content) => content,
            Err(_) => return Ok(Vec::new()), // If we can't read the file, assume no dependencies
        };
        
        Ok(self.extract_dependencies_from_source(&source))
    }
    
    /// Detect circular dependencies in the dependency graph
    pub fn detect_circular_dependencies(&self, graph: &HashMap<String, HashSet<String>>) -> Result<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        
        for node in graph.keys() {
            if !visited.contains(node) {
                if self.has_cycle_dfs(node, graph, &mut visited, &mut rec_stack, &mut path)? {
                    return Err(CursedError::ImportError(format!(
                        "Circular dependency detected in chain: {}", 
                        path.join(" -> ")
                    )));
                }
            }
        }
        
        Ok(())
    }
    
    /// DFS-based cycle detection
    fn has_cycle_dfs(
        &self,
        node: &str,
        graph: &HashMap<String, HashSet<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Result<bool> {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());
        
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if self.has_cycle_dfs(neighbor, graph, visited, rec_stack, path)? {
                        return Ok(true);
                    }
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle - add the neighbor to complete the cycle path
                    path.push(neighbor.to_string());
                    return Ok(true);
                }
            }
        }
        
        rec_stack.remove(node);
        path.pop();
        Ok(false)
    }
    
    /// Enhanced module existence check with better error reporting
    pub fn check_module_exists(&self, import_path: &str) -> Result<bool> {
        let import_source = self.classify_import(import_path)?;
        
        match &import_source {
            ImportSource::Local(path) => {
                Ok(self.resolve_local_import(path).is_ok())
            }
            ImportSource::Package(name, _version) => {
                let package_path = self.config.package_cache_dir.join(name);
                Ok(crate::subprocess_utils::file_exists_with_timeout(&package_path, 10))
            }
            ImportSource::Stdlib(name) => {
                Ok(self.resolve_stdlib_import(name).is_ok())
            }
        }
    }
    
    /// Get detailed import resolution information for debugging
    pub fn get_import_info(&self, import_path: &str) -> ImportResolutionInfo {
        let classification_result = self.classify_import(import_path);
        
        match classification_result {
            Ok(import_source) => {
                let path_result = match &import_source {
                    ImportSource::Local(path) => self.resolve_local_import(path),
                    ImportSource::Package(name, version) => {
                        let package_path = self.config.package_cache_dir.join(name);
                        if package_path.exists() {
                            Ok(package_path)
                        } else {
                            Err(CursedError::ImportError(format!("Package not found: {}", name)))
                        }
                    }
                    ImportSource::Stdlib(name) => self.resolve_stdlib_import(name),
                };
                
                let is_ok = path_result.is_ok();
                let resolved_path = path_result.as_ref().ok().map(|p| p.clone());
                let error = path_result.err().map(|e| e.to_string());
                
                ImportResolutionInfo {
                    import_path: import_path.to_string(),
                    classification: Some(import_source),
                    resolved_path,
                    exists: is_ok,
                    error,
                    is_cached: self.cache.resolution_cache.contains_key(import_path),
                }
            }
            Err(err) => {
                ImportResolutionInfo {
                    import_path: import_path.to_string(),
                    classification: None,
                    resolved_path: None,
                    exists: false,
                    error: Some(err.to_string()),
                    is_cached: false,
                }
            }
        }
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

/// Detailed import resolution information for debugging
#[derive(Debug, Clone)]
pub struct ImportResolutionInfo {
    pub import_path: String,
    pub classification: Option<ImportSource>,
    pub resolved_path: Option<PathBuf>,
    pub exists: bool,
    pub error: Option<String>,
    pub is_cached: bool,
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
