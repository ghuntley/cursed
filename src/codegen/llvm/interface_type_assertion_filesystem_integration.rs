//! # Interface Type Assertion Filesystem Integration
//!
//! This module implements filesystem integration for source location tracking in interface
//! type assertions. It enhances error messages by providing precise file paths, line numbers,
//! and actual source code snippets when type assertions fail.
//!
//! ## Key Features
//!
//! 1. Filesystem utilities for accessing source files during compilation
//! 2. Source file caching for improved performance
//! 3. Source line extraction and context for error messages
//! 4. Integration with existing error propagation and source location systems
//!
//! This implementation allows errors to include the exact code that caused the type assertion
//! failure, making debugging much easier and more efficient.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use tracing::{debug, error, info, instrument, trace, warn};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::AddressSpace;

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_enhanced_source_location::EnhancedSourceLocationSupport;
use crate::codegen::llvm::interface_type_assertion_error_propagation_source_location::EnhancedSourceLocationErrorPropagation;
use crate::error::Error;
use crate::error::SourceLocation;

/// Represents a source file with its content and metadata for caching
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    pub lines: Vec<String>,
    pub last_modified: std::time::SystemTime,
}

impl SourceFile {
    /// Create a new SourceFile by reading from disk
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let metadata = fs::metadata(path).map_err(Error::IO)?;
        let content = fs::read_to_string(path).map_err(Error::IO)?;
        let lines = content.lines().map(|s| s.to_string()).collect();
        
        Ok(Self {
            path: path.to_path_buf(),
            content,
            lines,
            last_modified: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now()),
        })
    }
    
    /// Get a specific line from the file (0-indexed)
    pub fn get_line(&self, line_number: usize) -> Option<&String> {
        self.lines.get(line_number)
    }
    
    /// Get a range of lines with context
    pub fn get_line_context(&self, line_number: usize, context_lines: usize) -> Vec<(usize, &String)> {
        let start = line_number.saturating_sub(context_lines);
        let end = std::cmp::min(line_number + context_lines + 1, self.lines.len());
        
        (start..end).filter_map(|i| {
            self.lines.get(i).map(|line| (i, line))
        }).collect()
    }
    
    /// Check if the file has been modified since it was loaded
    pub fn is_stale(&self) -> bool {
        match fs::metadata(&self.path) {
            Ok(metadata) => {
                match metadata.modified() {
                    Ok(modified) => modified > self.last_modified,
                    Err(_) => false,
                }
            },
            Err(_) => false,
        }
    }
    
    /// Reload the file contents if needed
    pub fn reload_if_needed(&mut self) -> Result<bool, Error> {
        if self.is_stale() {
            let metadata = fs::metadata(&self.path).map_err(Error::IO)?;
            let content = fs::read_to_string(&self.path).map_err(Error::IO)?;
            let lines = content.lines().map(|s| s.to_string()).collect();
            
            self.content = content;
            self.lines = lines;
            self.last_modified = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Cache for source files to avoid repeated disk access
pub struct SourceFileCache {
    files: HashMap<PathBuf, SourceFile>,
    root_dir: Option<PathBuf>,
    search_paths: Vec<PathBuf>,
}

impl SourceFileCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            root_dir: None,
            search_paths: Vec::new(),
        }
    }
    
    /// Set the root directory for resolving relative paths
    pub fn with_root_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.root_dir = Some(dir.as_ref().to_path_buf());
        self
    }
    
    /// Add a search path for finding source files
    pub fn add_search_path(&mut self, path: impl AsRef<Path>) {
        self.search_paths.push(path.as_ref().to_path_buf());
    }
    
    /// Resolve a file path, trying multiple ways
    pub fn resolve_path(&self, path_str: &str) -> Option<PathBuf> {
        let path = Path::new(path_str);
        
        // First, check if it's an absolute path that exists
        if path.is_absolute() && path.exists() {
            return Some(path.to_path_buf());
        }
        
        // Try relative to the root directory
        if let Some(ref root) = self.root_dir {
            let root_path = root.join(path);
            if root_path.exists() {
                return Some(root_path);
            }
        }
        
        // Try each search path
        for search_path in &self.search_paths {
            let full_path = search_path.join(path);
            if full_path.exists() {
                return Some(full_path);
            }
        }
        
        None
    }
    
    /// Get a source file by path, loading it if needed
    pub fn get_file(&mut self, path_str: &str) -> Result<&SourceFile, Error> {
        let resolved_path = self.resolve_path(path_str)
            .ok_or_else(|| Error::IO(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path_str)
            )))?;
        
        if !self.files.contains_key(&resolved_path) {
            let source_file = SourceFile::new(&resolved_path)?;
            self.files.insert(resolved_path.clone(), source_file);
        } else {
            // Check if file needs to be reloaded
            if let Some(file) = self.files.get_mut(&resolved_path) {
                file.reload_if_needed()?;
            }
        }
        
        Ok(self.files.get(&resolved_path).unwrap())
    }
    
    /// Get a specific line from a file
    pub fn get_line(&mut self, path: &str, line_number: usize) -> Result<Option<String>, Error> {
        let file = self.get_file(path)?;
        // Convert from 1-indexed to 0-indexed
        let adjusted_line = line_number.saturating_sub(1);
        Ok(file.get_line(adjusted_line).cloned())
    }
    
    /// Get a line with surrounding context
    pub fn get_line_with_context(
        &mut self, 
        path: &str, 
        line_number: usize, 
        context_lines: usize
    ) -> Result<Vec<(usize, String)>, Error> {
        let file = self.get_file(path)?;
        // Convert from 1-indexed to 0-indexed
        let adjusted_line = line_number.saturating_sub(1);
        let context = file.get_line_context(adjusted_line, context_lines);
        
        // Convert line numbers back to 1-indexed
        Ok(context.into_iter().map(|(i, s)| (i + 1, s.clone())).collect())
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.files.clear();
    }
}

/// Trait for filesystem integration with source location tracking
pub trait FilesystemSourceLocationIntegration<'ctx>: EnhancedSourceLocationSupport<'ctx> {
    /// Initialize the filesystem integration with a root directory
    fn init_filesystem_integration(&mut self, root_dir: Option<&str>);
    
    /// Add a source file search path
    fn add_source_search_path(&mut self, path: &str);
    
    /// Get a source file line with context
    fn get_source_line_with_context(
        &self, 
        file_path: &str, 
        line_number: usize, 
        context_lines: usize
    ) -> Result<Vec<(usize, String)>, Error>;
    
    /// Create a comprehensive source location with file context
    fn create_source_location_with_context(
        &self,
        node: &dyn crate::ast::traits::Node,
        line: usize,
        column: usize,
        file_path: Option<&str>,
        context_lines: usize
    ) -> Result<SourceLocation, Error>;
    
    /// Format error message with source context
    fn format_error_with_source_context(
        &self,
        message: &str,
        location: &SourceLocation,
        context_lines: usize
    ) -> Result<String, Error>;
}

// Extension methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get or create the source file cache
    fn source_file_cache(&mut self) -> &mut SourceFileCache {
        // Initialize cache if it doesn't exist
        if !self.internal_fields.contains_key("source_file_cache") {
            self.internal_fields.insert(
                "source_file_cache".to_string(), 
                Box::new(SourceFileCache::new())
            );
        }
        
        // Return a mutable reference to the cache
        self.internal_fields.get_mut("source_file_cache")
            .and_then(|field| field.downcast_mut::<SourceFileCache>())
            .expect("Failed to get source file cache")
    }
}

// Implementation of FilesystemSourceLocationIntegration for LlvmCodeGenerator
impl<'ctx> FilesystemSourceLocationIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, root_dir), level = "debug")]
    fn init_filesystem_integration(&mut self, root_dir: Option<&str>) {
        // Initialize enhanced source location tracking first
        self.init_enhanced_source_location_tracking(None);
        
        // Initialize the source file cache
        let mut cache = SourceFileCache::new();
        
        if let Some(dir) = root_dir {
            cache = cache.with_root_dir(dir);
            // Also set as current file path
            self.set_current_file_path(dir);
        } else {
            // Try to use current working directory
            if let Ok(cwd) = std::env::current_dir() {
                cache = cache.with_root_dir(cwd);
            }
        }
        
        // Add standard search paths
        cache.add_search_path(".");
        cache.add_search_path("./src");
        cache.add_search_path("./examples");
        
        // Store the cache
        self.internal_fields.insert("source_file_cache".to_string(), Box::new(cache));
        
        debug!("Initialized filesystem integration for source location tracking");
    }
    
    #[instrument(skip(self, path), level = "debug")]
    fn add_source_search_path(&mut self, path: &str) {
        let cache = self.source_file_cache();
        cache.add_search_path(path);
        debug!("Added source search path: {}", path);
    }
    
    #[instrument(skip(self, file_path), level = "debug")]
    fn get_source_line_with_context(
        &self, 
        file_path: &str, 
        line_number: usize, 
        context_lines: usize
    ) -> Result<Vec<(usize, String)>, Error> {
        // Get a mutable reference to the cache
        // This is a bit awkward with the borrow checker, but we need to modify the cache
        // to potentially load new files
        if let Some(cache_field) = self.internal_fields.get_mut("source_file_cache") {
            if let Some(cache) = cache_field.downcast_mut::<SourceFileCache>() {
                return cache.get_line_with_context(file_path, line_number, context_lines);
            }
        }
        
        // Fallback if cache isn't available
        Err(Error::IO(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Source file cache not initialized"
        )))
    }
    
    #[instrument(skip(self, node, file_path), level = "debug")]
    fn create_source_location_with_context(
        &self,
        node: &dyn crate::ast::traits::Node,
        line: usize,
        column: usize,
        file_path: Option<&str>,
        context_lines: usize
    ) -> Result<SourceLocation, Error> {
        // Get the file path - either provided or from context
        let file = file_path.map(|s| s.to_string())
            .or_else(|| self.current_file_path())
            .unwrap_or_else(|| "<unknown>".to_string());
            
        // Get the source line and context
        let source_context = self.get_source_line_with_context(&file, line, context_lines)
            .unwrap_or_default();
        
        // Format the source line with context for error messages
        let source_text = if !source_context.is_empty() {
            let mut result = String::new();
            for (line_num, line_text) in source_context {
                let prefix = if line_num == line {
                    ">"
                } else {
                    " "
                };
                result.push_str(&format!("{} {:4} | {}\n", prefix, line_num, line_text));
                
                // Add a marker for the exact column
                if line_num == line {
                    let mut marker = String::new();
                    marker.push_str("  ");
                    marker.push_str(&" ".repeat(5));
                    marker.push_str("| ");
                    // Add spaces up to the column
                    let actual_column = std::cmp::min(column, line_text.len());
                    marker.push_str(&" ".repeat(actual_column));
                    marker.push_str("^\n");
                    result.push_str(&marker);
                }
            }
            result
        } else {
            // Fallback to the node's string representation
            format!("at {}\n", node.string())
        };
        
        Ok(SourceLocation {
            line,
            column,
            file: Some(file),
            source_line: source_text,
        })
    }
    
    #[instrument(skip(self, message, location), level = "debug")]
    fn format_error_with_source_context(
        &self,
        message: &str,
        location: &SourceLocation,
        context_lines: usize
    ) -> Result<String, Error> {
        // Basic message with location
        let mut formatted = format!(
            "{}\n  at {}:{}:{}",
            message,
            location.file.as_deref().unwrap_or("<unknown>"),
            location.line,
            location.column
        );
        
        // Add source context if we don't already have it
        if location.source_line.is_empty() {
            if let Some(file) = &location.file {
                if let Ok(context) = self.get_source_line_with_context(file, location.line, context_lines) {
                    formatted.push_str("\n\nSource:\n");
                    for (line_num, line_text) in context {
                        let prefix = if line_num == location.line {
                            ">"
                        } else {
                            " "
                        };
                        formatted.push_str(&format!("{} {:4} | {}\n", prefix, line_num, line_text));
                        
                        // Add a marker for the exact column
                        if line_num == location.line {
                            let mut marker = String::new();
                            marker.push_str("  ");
                            marker.push_str(&" ".repeat(5));
                            marker.push_str("| ");
                            // Add spaces up to the column
                            let actual_column = std::cmp::min(location.column, line_text.len());
                            marker.push_str(&" ".repeat(actual_column));
                            marker.push_str("^\n");
                            formatted.push_str(&marker);
                        }
                    }
                }
            }
        } else {
            // We already have source context in the location
            formatted.push_str("\n\nSource:\n");
            formatted.push_str(&location.source_line);
        }
        
        Ok(formatted)
    }
}

// Register the filesystem integration module
pub fn register_filesystem_integration() {
    debug!("Registered filesystem integration for interface type assertions source location tracking");
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_source_file_creation() {
        // Create a temporary test file
        use std::io::Write;
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "line 1").unwrap();
        writeln!(file, "line 2").unwrap();
        writeln!(file, "line 3").unwrap();
        
        // Test loading the file
        let source_file = SourceFile::new(&file_path).unwrap();
        assert_eq!(source_file.lines.len(), 3);
        assert_eq!(source_file.get_line(0).unwrap(), "line 1");
        assert_eq!(source_file.get_line(1).unwrap(), "line 2");
        assert_eq!(source_file.get_line(2).unwrap(), "line 3");
        
        // Test line context
        let context = source_file.get_line_context(1, 1);
        assert_eq!(context.len(), 3);
        assert_eq!(context[0].0, 0);
        assert_eq!(context[0].1, "line 1");
        assert_eq!(context[1].0, 1);
        assert_eq!(context[1].1, "line 2");
        assert_eq!(context[2].0, 2);
        assert_eq!(context[2].1, "line 3");
    }
    
    #[test]
    fn test_source_file_cache() {
        // Create a temporary test directory with files
        use std::io::Write;
        let temp_dir = tempfile::tempdir().unwrap();
        let file1_path = temp_dir.path().join("test1.txt");
        let file2_path = temp_dir.path().join("test2.txt");
        
        let mut file1 = std::fs::File::create(&file1_path).unwrap();
        writeln!(file1, "file1 line 1").unwrap();
        writeln!(file1, "file1 line 2").unwrap();
        
        let mut file2 = std::fs::File::create(&file2_path).unwrap();
        writeln!(file2, "file2 line 1").unwrap();
        writeln!(file2, "file2 line 2").unwrap();
        
        // Create a cache with the temp directory as root
        let mut cache = SourceFileCache::new().with_root_dir(temp_dir.path());
        
        // Test getting lines
        let line = cache.get_line("test1.txt", 1).unwrap().unwrap();
        assert_eq!(line, "file1 line 1");
        
        let line = cache.get_line("test2.txt", 2).unwrap().unwrap();
        assert_eq!(line, "file2 line 2");
        
        // Test getting context
        let context = cache.get_line_with_context("test1.txt", 2, 1).unwrap();
        assert_eq!(context.len(), 2);
        assert_eq!(context[0].0, 1);
        assert_eq!(context[0].1, "file1 line 1");
        assert_eq!(context[1].0, 2);
        assert_eq!(context[1].1, "file1 line 2");
    }
}