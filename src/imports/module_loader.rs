use crate::error::Error;
//! Module loading and caching

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::ast::Program;
use crate::lexer::Lexer;
use crate::parser::Parser;
use super::{ImportError, ResolvedImport};

/// Information about a loaded module
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub path: PathBuf,
    pub exports: Vec<String>,
    pub types: Vec<String>,
    pub dependencies: Vec<String>,
}

/// A fully loaded and parsed module
#[derive(Debug)]
pub struct LoadedModule {
    pub info: ModuleInfo,
    pub program: Program,
    pub source: String,
    pub metadata: ModuleMetadata,
}

/// Additional metadata about a loaded module
#[derive(Debug, Clone)]
pub struct ModuleMetadata {
    pub file_size: u64,
    pub load_time: std::time::Instant,
    pub checksum: String,
    pub version: Option<String>,
}

/// Module loader with caching and dependency tracking
#[derive(Debug)]
pub struct ModuleLoader {
    loaded_modules: HashMap<String, Arc<LoadedModule>>,
    loading_stack: std::collections::HashSet<String>,
}

impl ModuleLoader {
    /// Create new module loader
    pub fn new() -> Self {
        Self {
            loaded_modules: HashMap::new(),
            loading_stack: std::collections::HashSet::new(),
        }
    }
    
    /// Load a module from a resolved import
    pub async fn load_module(&mut self, resolved: &ResolvedImport) -> Result<(), Error> {
        let module_key = resolved.get_cache_key();
        
        // Check if already loaded
        if let Some(cached) = self.loaded_modules.get(&module_key) {
            return Ok((**cached).clone());
        }
        
        // Check for circular dependencies
        if self.loading_stack.contains(&module_key) {
            return Err(ImportError::CircularImport {
                cycle: self.loading_stack.iter().cloned().collect(),
            });
        }
        
        self.loading_stack.insert(module_key.clone());
        
        let result = self.load_module_internal(resolved).await;
        
        self.loading_stack.remove(&module_key);
        
        result
    }
    
    /// Internal module loading implementation
    async fn load_module_internal(&mut self, resolved: &ResolvedImport) -> Result<(), Error> {
        let load_start = std::time::Instant::now();
        
        // Read source file
        let source = if resolved.resolved_path.exists() {
            std::fs::read_to_string(&resolved.resolved_path)?
        } else {
            // For stdlib or package modules that might not exist as files,
            // generate appropriate source code
            self.generate_module_source(resolved)?
        };
        
        // Get file metadata
        let file_size = if resolved.resolved_path.exists() {
            std::fs::metadata(&resolved.resolved_path)?.len()
        } else {
            source.len() as u64
        };
        
        // Parse the module
        let lexer = Lexer::new(&source);
        let mut parser = Parser::new(lexer).map_err(|e| ImportError::ModuleLoadError {
            module: resolved.original_path.clone(),
            error: format!("Failed to create parser: {}", e),
        })?;
        
        let program = parser.parse_program().map_err(|e| ImportError::ModuleLoadError {
            module: resolved.original_path.clone(),
            error: format!("Failed to parse module: {}", e),
        })?;
        
        // Check for parse errors
        let errors = parser.errors();
        if !errors.is_empty() {
            return Err(ImportError::ModuleLoadError {
                module: resolved.original_path.clone(),
                error: format!("Parse errors: {}", errors.join(", ")),
            });
        }
        
        // Extract module information
        let module_info = self.extract_module_info(&program, resolved)?;
        
        // Create metadata
        let checksum = self.calculate_checksum(&source);
        let metadata = ModuleMetadata {
            file_size,
            load_time: load_start,
            checksum,
            version: None, // TODO: Extract from package metadata
        };
        
        let loaded_module = LoadedModule {
            info: module_info,
            program,
            source,
            metadata,
        };
        
        tracing::info!(
            module = resolved.original_path,
            load_time = ?load_start.elapsed(),
            "Module loaded successfully"
        );
        
        Ok(loaded_module)
    }
    
    /// Generate source code for built-in modules
    fn generate_module_source(&self, resolved: &ResolvedImport) -> Result<(), Error> {
        match &resolved.original_path {
            path if path.starts_with("stdlib::io") => {
                Ok(r#"
// Standard Library I/O Module
slay print(msg: String) {
    // Native implementation
}

slay println(msg: String) {
    // Native implementation
}

slay read_line() -> String {
    // Native implementation
}

slay read_file(path: String) -> String {
    // Native implementation
}

slay write_file(path: String, content: String) {
    // Native implementation
}
"#.to_string())
            }
            path if path.starts_with("stdlib::math") => {
                Ok(r#"
// Standard Library Math Module
facts PI: f64 = 3.14159265358979323846;
facts E: f64 = 2.71828182845904523536;

slay abs(x: f64) -> f64 {
    // Native implementation
}

slay max(a: f64, b: f64) -> f64 {
    // Native implementation
}

slay min(a: f64, b: f64) -> f64 {
    // Native implementation
}

slay sqrt(x: f64) -> f64 {
    // Native implementation
}

slay pow(base: f64, exp: f64) -> f64 {
    // Native implementation
}
"#.to_string())
            }
            path if path.starts_with("stdlib::collections") => {
                Ok(r#"
// Standard Library Collections Module
squad Vec<T> {
    data: Array<T>,
    size: i32,
    capacity: i32,
}

squad Map<K, V> {
    buckets: Array<Bucket<K, V>>,
    size: i32,
}

squad Set<T> {
    map: Map<T, bool>,
}

squad Queue<T> {
    data: Vec<T>,
    front: i32,
}

squad Stack<T> {
    data: Vec<T>,
}
"#.to_string())
            }
            _ => {
                // For unknown modules, return empty content
                Ok(format!("// Module: {}\n// Auto-generated stub", resolved.original_path))
            }
        }
    }
    
    /// Extract module information from parsed program
    fn extract_module_info(&self, program: &Program, resolved: &ResolvedImport) -> Result<(), Error> {
        let mut exports = Vec::new();
        let mut types = Vec::new();
        let mut dependencies = Vec::new();
        
        // Extract function exports
        for stmt in &program.statements {
            if let Some(func_name) = self.extract_function_name(stmt.as_ref()) {
                exports.push(func_name);
            }
            
            if let Some(type_name) = self.extract_type_name(stmt.as_ref()) {
                types.push(type_name);
            }
        }
        
        // Extract imports as dependencies
        for import in &program.imports {
            dependencies.push(import.path.clone());
        }
        
        // Use resolved exports if available (for known modules)
        if !resolved.exports.is_empty() {
            exports = resolved.exports.clone();
        }
        if !resolved.types.is_empty() {
            types = resolved.types.clone();
        }
        
        Ok(ModuleInfo {
            name: resolved.original_path.clone(),
            path: resolved.resolved_path.clone(),
            exports,
            types,
            dependencies,
        })
    }
    
    /// Extract function name from statement (simplified)
    fn extract_function_name(&self, _stmt: &dyn crate::ast::Statement) -> Option<String> {
        // TODO: Implement proper AST traversal to extract function names
        None
    }
    
    /// Extract type name from statement (simplified)
    fn extract_type_name(&self, _stmt: &dyn crate::ast::Statement) -> Option<String> {
        // TODO: Implement proper AST traversal to extract type names
        None
    }
    
    /// Calculate checksum for module content
    fn calculate_checksum(&self, source: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// Clear module cache
    pub fn clear_cache(&mut self) {
        self.loaded_modules.clear();
    }
    
    /// Get loaded module by name
    pub fn get_loaded_module(&self, name: &str) -> Option<Arc<LoadedModule>> {
        self.loaded_modules.values()
            .find(|module| module.info.name == name)
            .cloned()
    }
    
    /// Get loading statistics
    pub fn get_stats(&self) -> LoaderStats {
        let total_size: u64 = self.loaded_modules.values()
            .map(|module| module.metadata.file_size)
            .sum();
        
        LoaderStats {
            loaded_modules: self.loaded_modules.len(),
            total_size,
            currently_loading: self.loading_stack.len(),
        }
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for LoadedModule {
    fn clone(&self) -> Self {
        Self {
            info: self.info.clone(),
            program: self.program.clone(),
            source: self.source.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

/// Module loader statistics
#[derive(Debug, Clone)]
pub struct LoaderStats {
    pub loaded_modules: usize,
    pub total_size: u64,
    pub currently_loading: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::{ResolvedImport, ImportSource};
    
    #[tokio::test]
    async fn test_stdlib_module_generation() {
        let mut loader = ModuleLoader::new();
        
        let resolved = ResolvedImport {
            original_path: "stdlib::io".to_string(),
            source: ImportSource::StandardLibrary,
            resolved_path: std::path::PathBuf::from("stdlib/io.rs"),
            alias: None,
            exports: vec!["print".to_string(), "println".to_string()],
            types: vec![],
        };
        
        let loaded = loader.load_module(&resolved).await.unwrap();
        assert!(loaded.source.contains("slay print"));
        assert!(loaded.source.contains("slay println"));
    }
}
