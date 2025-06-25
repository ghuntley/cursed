use crate::error::CursedError;
// Core import resolution logic

use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Configuration for import resolution
#[derive(Debug, Clone)]
pub struct ImportResolverConfig {
    pub stdlib_path: PathBuf,
    pub local_search_paths: Vec<PathBuf>,
    pub package_search_paths: Vec<PathBuf>,
    pub allow_relative_imports: bool,
}

impl Default for ImportResolverConfig {
    fn default() -> Self {
        Self {
            stdlib_path: PathBuf::from("src/stdlib"),
            local_search_paths: vec![PathBuf::from("src"), PathBuf::from(".")],
            package_search_paths: vec![
                dirs::cache_dir().unwrap_or_default().join("cursed/packages")
            ],
            allow_relative_imports: true,
        }
    }
}

/// Types of import sources
#[derive(Debug, Clone, PartialEq)]
pub enum ImportSource {
    StandardLibrary,
    InstalledPackage { package_name: String },
    LocalFile { relative_path: PathBuf },
    LocalModule { module_path: PathBuf },
}

/// A resolved import with its source and metadata
#[derive(Debug, Clone)]
pub struct ResolvedImport {
    pub original_path: String,
    pub source: ImportSource,
    pub resolved_path: PathBuf,
    pub alias: Option<String>,
    pub exports: Vec<String>,
    pub types: Vec<String>,
}

impl ResolvedImport {
    /// Get cache key for this resolved import
    pub fn get_cache_key(&self) -> String {
        format!("{}:{}", self.original_path, self.resolved_path.display())
    }
    
    /// Check if this import provides a specific symbol
    pub fn provides_symbol(&self, symbol: &str) -> bool {
        self.exports.contains(&symbol.to_string()) || self.types.contains(&symbol.to_string())
    }
}

/// Core import resolver
#[derive(Debug)]
pub struct ImportResolver {
    config: ImportResolverConfig,
    stdlib_exports: HashMap<String, Vec<String>>,
}

impl ImportResolver {
    /// Create new import resolver
    pub fn new(config: ImportResolverConfig) -> Self {
        let mut resolver = Self {
            config,
            stdlib_exports: HashMap::new(),
        };
        
        resolver.init_stdlib_exports();
        resolver
    }
    
    /// Initialize standard library export mappings
    fn init_stdlib_exports(&mut self) {
        // Define what each stdlib module exports
        self.stdlib_exports.insert(
            "stdlib::io".to_string(),
            vec![
                "print".to_string(),
                "println".to_string(),
                "read_line".to_string(),
                "read_file".to_string(),
                "write_file".to_string(),
            ]
        );
        
        self.stdlib_exports.insert(
            "stdlib::math".to_string(),
            vec![
                "abs".to_string(),
                "max".to_string(),
                "min".to_string(),
                "sqrt".to_string(),
                "pow".to_string(),
                "PI".to_string(),
                "E".to_string(),
            ]
        );
        
        self.stdlib_exports.insert(
            "stdlib::collections".to_string(),
            vec![
                "Vec".to_string(),
                "Map".to_string(),
                "Set".to_string(),
                "Queue".to_string(),
                "Stack".to_string(),
            ]
        );
        
        self.stdlib_exports.insert(
            "stdlib::string".to_string(),
            vec![
                "len".to_string(),
                "trim".to_string(),
                "split".to_string(),
                "join".to_string(),
                "replace".to_string(),
            ]
        );
        
        self.stdlib_exports.insert(
            "stdlib::time".to_string(),
            vec![
                "now".to_string(),
                "sleep".to_string(),
                "Duration".to_string(),
                "Instant".to_string(),
            ]
        );
    }
    
    /// Resolve standard library import
    pub fn resolve_stdlib_import(&self, import_path: &str) -> crate::error::Result<()> {
        // Parse stdlib path (e.g., "stdlib::io::console")
        let parts: Vec<&str> = import_path.split("::").collect();
        if parts.len() < 2 || parts[0] != "stdlib" {
            return Err(ImportError::InvalidPath {
                path: import_path.to_string(),
                reason: "Not a valid stdlib import".to_string(),
            });
        }
        
        let module_name = parts[1];
        let stdlib_module_path = format!("stdlib::{}", module_name);
        
        // Check if module exists in our stdlib exports
        let exports = self.stdlib_exports.get(&stdlib_module_path)
            .cloned()
            .unwrap_or_default();
        
        // Build resolved path
        let mut resolved_path = self.config.stdlib_path.clone();
        for part in &parts[1..] {
            resolved_path = resolved_path.join(part);
        }
        resolved_path.set_extension("rs");
        
        Ok(ResolvedImport {
            original_path: import_path.to_string(),
            source: ImportSource::StandardLibrary,
            resolved_path,
            alias: None,
            exports,
            types: Vec::new(), // TODO: Add type information
        })
    }
    
    /// Resolve local file import
    pub fn resolve_local_import(
        &self, 
        import_path: &str, 
        context_path: Option<&Path>
    ) -> crate::error::Result<()> {
        let mut search_paths = self.config.local_search_paths.clone();
        
        // Add context path if provided
        if let Some(context) = context_path {
            if let Some(parent) = context.parent() {
                search_paths.insert(0, parent.to_path_buf());
            }
        }
        
        // Try different file extensions and paths
        let potential_paths = self.generate_potential_paths(import_path, &search_paths);
        
        for path in potential_paths {
            if path.exists() {
                return Ok(ResolvedImport {
                    original_path: import_path.to_string(),
                    source: ImportSource::LocalFile { 
                        relative_path: path.clone() 
                    },
                    resolved_path: path,
                    alias: None,
                    exports: Vec::new(), // TODO: Parse file to get exports
                    types: Vec::new(),
                });
            }
        }
        
        Err(ImportError::NotFound {
            import_path: import_path.to_string(),
        })
    }
    
    /// Generate potential file paths for an import
    fn generate_potential_paths(&self, import_path: &str, search_paths: &[PathBuf]) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        let extensions = [".csd", ".cursed", ""];
        
        for search_path in search_paths {
            // Direct path
            for ext in &extensions {
                let path = search_path.join(format!("{}{}", import_path, ext));
                paths.push(path);
            }
            
            // Module path (import_path/mod.csd)
            for ext in &extensions {
                let mod_path = search_path.join(import_path).join(format!("mod{}", ext));
                paths.push(mod_path);
            }
            
            // Replace :: with / for module paths
            if import_path.contains("::") {
                let module_path = import_path.replace("::", "/");
                for ext in &extensions {
                    let path = search_path.join(format!("{}{}", module_path, ext));
                    paths.push(path);
                }
            }
        }
        
        paths
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: ImportResolverConfig) {
        self.config = config;
    }
    
    /// Add search path
    pub fn add_search_path(&mut self, path: PathBuf) {
        if !self.config.local_search_paths.contains(&path) {
            self.config.local_search_paths.push(path);
        }
    }
    
    /// Get configuration
    pub fn config(&self) -> &ImportResolverConfig {
        &self.config
    }
}

