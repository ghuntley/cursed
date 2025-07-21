//! Module Loading System for CURSED
//! 
//! This module handles the loading and compilation of CURSED modules from disk,
//! including caching, dependency management, and compilation pipeline integration.

use crate::error::{CursedError, Result};
use crate::ast::{Program, Statement};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;
use std::time::SystemTime;

/// A loaded and compiled CURSED module
#[derive(Debug, Clone)]
pub struct LoadedModule {
    pub name: String,
    pub path: PathBuf,
    pub program: Arc<Program>,
    pub exported_symbols: Vec<String>,
    pub dependencies: Vec<String>,
    pub load_time: SystemTime,
    pub source_hash: u64, // For change detection
}

/// Module loading configuration
#[derive(Debug, Clone)]
pub struct ModuleLoaderConfig {
    pub cache_enabled: bool,
    pub validate_syntax: bool,
    pub extract_symbols: bool,
    pub resolve_dependencies: bool,
    pub max_file_size: usize, // Maximum size in bytes
}

impl Default for ModuleLoaderConfig {
    fn default() -> Self {
        Self {
            cache_enabled: true,
            validate_syntax: true,
            extract_symbols: true,
            resolve_dependencies: true,
            max_file_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Cache for loaded modules
#[derive(Debug, Default)]
pub struct ModuleCache {
    modules: HashMap<PathBuf, LoadedModule>,
    load_times: HashMap<PathBuf, SystemTime>,
    source_hashes: HashMap<PathBuf, u64>,
}

/// Module loader responsible for loading and compiling CURSED modules
#[derive(Debug)]
pub struct ModuleLoader {
    config: ModuleLoaderConfig,
    cache: ModuleCache,
}

impl ModuleLoader {
    /// Create a new module loader with default configuration
    pub fn new() -> Self {
        Self::with_config(ModuleLoaderConfig::default())
    }

    /// Create a new module loader with custom configuration
    pub fn with_config(config: ModuleLoaderConfig) -> Self {
        Self {
            config,
            cache: ModuleCache::default(),
        }
    }

    /// Load a module from file, using cache if available and valid
    pub fn load_module(&mut self, path: &Path) -> Result<LoadedModule> {
        let path_buf = path.to_path_buf();

        // Check if file exists
        if !path.exists() {
            return Err(CursedError::ImportError(format!("Module file not found: {}", path.display())));
        }

        // Check file size
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > self.config.max_file_size as u64 {
                return Err(CursedError::ImportError(format!(
                    "Module file too large: {} bytes (max: {})", 
                    metadata.len(), 
                    self.config.max_file_size
                )));
            }
        }

        // Check cache if enabled
        if self.config.cache_enabled {
            if let Some(cached_module) = self.get_cached_module(&path_buf)? {
                return Ok(cached_module);
            }
        }

        // Load and compile the module
        let loaded_module = self.load_module_from_disk(&path_buf)?;

        // Cache the result if caching is enabled
        if self.config.cache_enabled {
            self.cache_module(&path_buf, &loaded_module);
        }

        Ok(loaded_module)
    }

    /// Load multiple modules concurrently
    pub async fn load_modules(&mut self, paths: &[PathBuf]) -> Result<Vec<LoadedModule>> {
        let mut modules = Vec::new();
        
        for path in paths {
            let module = self.load_module(path)?;
            modules.push(module);
        }
        
        Ok(modules)
    }

    /// Check if a module is cached and up-to-date
    fn get_cached_module(&self, path: &PathBuf) -> Result<Option<LoadedModule>> {
        if let Some(cached_module) = self.cache.modules.get(path) {
            // Check if file has been modified since caching
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(modified_time) = metadata.modified() {
                    if modified_time <= cached_module.load_time {
                        // Check source hash for additional validation
                        let current_hash = self.calculate_source_hash(path)?;
                        if current_hash == cached_module.source_hash {
                            return Ok(Some(cached_module.clone()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Load a module from disk without using cache
    fn load_module_from_disk(&self, path: &PathBuf) -> Result<LoadedModule> {
        // Read source file with timeout
        let source = crate::subprocess_utils::read_file_with_timeout(path, 30)
            .map_err(|e| CursedError::ImportError(format!("Failed to read {}: {}", path.display(), e)))?;

        // Calculate source hash
        let source_hash = self.calculate_source_hash_from_content(&source);

        // Tokenize source
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()
            .map_err(|e| CursedError::ImportError(format!("Lexer error in {}: {}", path.display(), e)))?;

        // Parse tokens into AST
        let mut parser = Parser::from_tokens(tokens);
        let program = parser.parse_program()
            .map_err(|e| CursedError::ImportError(format!("Parse error in {}: {}", path.display(), e)))?;

        // Extract module name from file path
        let module_name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Extract exported symbols if enabled
        let exported_symbols = if self.config.extract_symbols {
            self.extract_exported_symbols(&program)
        } else {
            Vec::new()
        };

        // Extract dependencies if enabled
        let dependencies = if self.config.resolve_dependencies {
            program.imports.iter().map(|imp| imp.path.clone()).collect()
        } else {
            Vec::new()
        };

        Ok(LoadedModule {
            name: module_name,
            path: path.clone(),
            program: Arc::new(program),
            exported_symbols,
            dependencies,
            load_time: SystemTime::now(),
            source_hash,
        })
    }

    /// Extract exported symbols from a program
    fn extract_exported_symbols(&self, program: &Program) -> Vec<String> {
        use crate::ast::Visibility;
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
                // Add more exportable types as needed
                _ => {}
            }
        }
        
        // Remove duplicates and sort
        symbols.sort();
        symbols.dedup();
        symbols
    }

    /// Cache a loaded module
    fn cache_module(&mut self, path: &PathBuf, module: &LoadedModule) {
        self.cache.modules.insert(path.clone(), module.clone());
        self.cache.load_times.insert(path.clone(), module.load_time);
        self.cache.source_hashes.insert(path.clone(), module.source_hash);
    }

    /// Calculate hash of source file for change detection
    fn calculate_source_hash(&self, path: &Path) -> Result<u64> {
        let content = fs::read_to_string(path)
            .map_err(|e| CursedError::ImportError(format!("Failed to read {}: {}", path.display(), e)))?;
        Ok(self.calculate_source_hash_from_content(&content))
    }

    /// Calculate hash of source content
    fn calculate_source_hash_from_content(&self, content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }

    /// Clear the module cache
    pub fn clear_cache(&mut self) {
        self.cache.modules.clear();
        self.cache.load_times.clear();
        self.cache.source_hashes.clear();
    }

    /// Remove a specific module from cache
    pub fn invalidate_cache(&mut self, path: &Path) {
        let path_buf = path.to_path_buf();
        self.cache.modules.remove(&path_buf);
        self.cache.load_times.remove(&path_buf);
        self.cache.source_hashes.remove(&path_buf);
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            cached_modules: self.cache.modules.len(),
            total_memory_usage: self.estimate_cache_memory_usage(),
        }
    }

    /// Estimate memory usage of the cache
    fn estimate_cache_memory_usage(&self) -> usize {
        // Rough estimation - would need more precise calculation in production
        self.cache.modules.len() * 1024 // Assume ~1KB per cached module
    }

    /// Check if a module is in cache
    pub fn is_cached(&self, path: &Path) -> bool {
        self.cache.modules.contains_key(path)
    }

    /// Get all cached module paths
    pub fn get_cached_modules(&self) -> Vec<PathBuf> {
        self.cache.modules.keys().cloned().collect()
    }

    /// Validate that all cached modules are still valid
    pub fn validate_cache(&mut self) -> Result<Vec<PathBuf>> {
        let mut invalidated = Vec::new();
        let cached_paths: Vec<_> = self.cache.modules.keys().cloned().collect();
        
        for path in cached_paths {
            if !path.exists() {
                // File was deleted
                self.invalidate_cache(&path);
                invalidated.push(path);
            } else if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified_time) = metadata.modified() {
                    if let Some(cached_module) = self.cache.modules.get(&path) {
                        if modified_time > cached_module.load_time {
                            // File was modified
                            self.invalidate_cache(&path);
                            invalidated.push(path);
                        }
                    }
                }
            }
        }
        
        Ok(invalidated)
    }

    /// Preload modules from a list of paths
    pub fn preload_modules(&mut self, paths: &[PathBuf]) -> Result<usize> {
        let mut loaded_count = 0;
        
        for path in paths {
            if !self.is_cached(path) {
                match self.load_module(path) {
                    Ok(_) => loaded_count += 1,
                    Err(e) => {
                        eprintln!("Warning: Failed to preload {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        Ok(loaded_count)
    }
}

/// Cache statistics
#[derive(Debug)]
pub struct CacheStats {
    pub cached_modules: usize,
    pub total_memory_usage: usize,
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility function to check if a module file is valid CURSED source
pub fn validate_module_file(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    if !path.extension().map_or(false, |ext| ext == "csd") {
        return Ok(false);
    }

    // Try to parse the file
    let content = fs::read_to_string(path)
        .map_err(|e| CursedError::ImportError(format!("Failed to read {}: {}", path.display(), e)))?;

    let mut lexer = Lexer::new(content);
    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = Parser::from_tokens(tokens);
            match parser.parse_program() {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
        Err(_) => Ok(false),
    }
}

/// Utility function to find all CURSED modules in a directory
pub fn find_modules_in_directory(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut modules = Vec::new();
    
    if !dir.is_dir() {
        return Ok(modules);
    }

    fn collect_modules(dir: &Path, modules: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "csd") {
                modules.push(path);
            } else if path.is_dir() {
                collect_modules(&path, modules)?;
            }
        }
        Ok(())
    }

    collect_modules(dir, &mut modules)
        .map_err(|e| CursedError::ImportError(format!("Failed to scan directory {}: {}", dir.display(), e)))?;

    Ok(modules)
}
