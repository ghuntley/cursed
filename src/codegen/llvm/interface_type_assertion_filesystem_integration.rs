//! # Interface Type Assertion Filesystem Integration
//!
//! This module enhances the interface type assertion error system with filesystem
//! integration for better source location tracking and error reporting. It provides
//! features for loading source files, extracting context around error locations,
//! and presenting rich error diagnostics with code snippets.
//!
//! ## Key Features
//!
//! 1. File content caching for efficient source lookup
//! 2. Contextual error reporting with source code snippets
//! 3. Integration with the type assertion error propagation system
//! 4. Support for rich error visualization with line highlighting

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use tracing::{debug, error, info, instrument, trace, warn};

use crate::error::SourceLocation;
use crate::error::type_assertion_error::TypeAssertionError;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;

/// Maximum number of lines of context to include before and after error location
const DEFAULT_CONTEXT_LINES: usize = 3;

/// Structure for managing source file caching and lookups
#[derive(Debug, Default)]
pub struct SourceFileCache {
    /// Map of file paths to their cached contents
    files: HashMap<PathBuf, SourceFile>,
}

/// Represents a cached source file with its content
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// The absolute path to the file
    pub path: PathBuf,
    /// The raw content of the file
    pub content: String,
    /// The file content split into lines for easier access
    pub lines: Vec<String>,
}

impl SourceFileCache {
    /// Create a new empty source file cache
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Load a file into the cache if not already present
    pub fn load_file(&mut self, path: impl AsRef<Path>) -> Result<&SourceFile, Error> {
        let path = path.as_ref();
        
        if !self.files.contains_key(path) {
            // Read the file content
            let content = fs::read_to_string(path)
                .map_err(|e| Error::Filesystem(format!("Failed to read file '{}': {}", path.display(), e)))?;
            
            // Split the content into lines
            let lines = content.lines().map(String::from).collect();
            
            // Store in cache
            self.files.insert(
                path.to_path_buf(),
                SourceFile {
                    path: path.to_path_buf(),
                    content,
                    lines,
                },
            );
        }
        
        // Return a reference to the cached file
        Ok(self.files.get(path).unwrap())
    }
    
    /// Get a file from the cache if it exists
    pub fn get_file(&self, path: impl AsRef<Path>) -> Option<&SourceFile> {
        self.files.get(path.as_ref())
    }
    
    /// Extract source context around a specific line
    pub fn get_context(
        &self, 
        path: impl AsRef<Path>, 
        line: usize, 
        context_lines: Option<usize>
    ) -> Result<SourceContext, Error> {
        let file = self.load_file(&path)?;
        let context_lines = context_lines.unwrap_or(DEFAULT_CONTEXT_LINES);
        
        // Calculate the line range to include
        let start_line = line.saturating_sub(context_lines);
        let end_line = std::cmp::min(line + context_lines, file.lines.len());
        
        // Extract the lines in the range
        let context = file.lines[start_line..end_line]
            .iter()
            .enumerate()
            .map(|(i, line_content)| {
                ContextLine {
                    line_number: start_line + i + 1, // 1-indexed line numbers
                    content: line_content.clone(),
                    is_focus: start_line + i + 1 == line,
                }
            })
            .collect();
        
        Ok(SourceContext {
            file_path: path.as_ref().to_path_buf(),
            focus_line: line,
            context_lines: context,
        })
    }
}

/// A line of source code with contextual information
#[derive(Debug, Clone)]
pub struct ContextLine {
    /// The 1-indexed line number in the file
    pub line_number: usize,
    /// The content of the line
    pub content: String,
    /// Whether this is the focus line (where the error occurred)
    pub is_focus: bool,
}

/// Represents the source context around an error location
#[derive(Debug, Clone)]
pub struct SourceContext {
    /// The path to the source file
    pub file_path: PathBuf,
    /// The 1-indexed line number where the error occurred
    pub focus_line: usize,
    /// The lines of context around the error
    pub context_lines: Vec<ContextLine>,
}

impl SourceContext {
    /// Format the context as a string with line numbers and highlighting
    pub fn format(&self) -> String {
        let mut result = String::new();
        
        // Add file path header
        result.push_str(&format!("File: {}\n", self.file_path.display()));
        
        // Calculate the width needed for line numbers
        let max_line_num = self.context_lines.iter().map(|l| l.line_number).max().unwrap_or(0);
        let line_num_width = max_line_num.to_string().len();
        
        // Add each context line with appropriate formatting
        for line in &self.context_lines {
            let prefix = if line.is_focus { ">" } else { " " };
            let line_num = format!("{:width$}", line.line_number, width = line_num_width);
            
            result.push_str(&format!("{} {} | {}\n", prefix, line_num, line.content));
            
            // Add marker under the focus line
            if line.is_focus {
                let marker = format!("{: >width$} | {}", "", "^", width = line_num_width + 2);
                result.push_str(&format!("{: >width$}{}\n", "", marker, width = prefix.len()));
            }
        }
        
        result
    }
}

/// Trait for enhancing type assertion errors with filesystem source location information
pub trait InterfaceTypeAssertionFilesystemIntegration {
    /// Initialize the source file cache system
    fn init_source_file_cache(&mut self);
    
    /// Get or create the source file cache
    fn source_file_cache(&mut self) -> &mut SourceFileCache;
    
    /// Enhance a source location with file content information
    fn enhance_source_location(
        &mut self,
        location: &mut SourceLocation,
    ) -> Result<(), Error>;
    
    /// Enhance a type assertion error with filesystem source information
    fn enhance_type_assertion_error(
        &mut self,
        error: &mut TypeAssertionError,
    ) -> Result<(), Error>;
    
    /// Create an enhanced error message with source context
    fn create_enhanced_error_message(
        &mut self,
        message: &str,
        location: &SourceLocation,
    ) -> Result<String, Error>;
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Ensure the source file cache is initialized
    pub fn ensure_source_file_cache_initialized(&mut self) {
        // Initialize the source file cache if it doesn't exist
        if !self.internal_fields.contains_key("source_file_cache") {
            self.internal_fields.insert(
                "source_file_cache".to_string(),
                Box::new(SourceFileCache::new()),
            );
        }
    }
}

impl<'ctx> InterfaceTypeAssertionFilesystemIntegration for LlvmCodeGenerator<'ctx> {
    fn init_source_file_cache(&mut self) {
        self.ensure_source_file_cache_initialized();
    }
    
    fn source_file_cache(&mut self) -> &mut SourceFileCache {
        self.ensure_source_file_cache_initialized();
        self.internal_fields
            .get_mut("source_file_cache")
            .and_then(|val| val.downcast_mut::<SourceFileCache>())
            .expect("Source file cache not initialized properly")
    }
    
    #[instrument(skip(self, location))]
    fn enhance_source_location(
        &mut self,
        location: &mut SourceLocation,
    ) -> Result<(), Error> {
        // Skip if the location doesn't have a file path
        if location.file.is_none() {
            return Ok(());
        }
        
        // Get the file path
        let file_path = location.file.as_ref().unwrap();
        
        // Ensure the cache is initialized
        self.init_source_file_cache();
        
        // Try to load the file - silently return if we can't load it
        if let Ok(file) = self.source_file_cache().load_file(file_path) {
            // If line is within bounds, add the source line content
            if location.line > 0 && location.line <= file.lines.len() {
                location.source_line = file.lines[location.line - 1].clone();
            }
            
            debug!("Enhanced source location with file content: {}:{}", file_path, location.line);
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, error))]
    fn enhance_type_assertion_error(
        &mut self,
        error: &mut TypeAssertionError,
    ) -> Result<(), Error> {
        // Skip if there's no location
        if error.location.is_none() {
            return Ok(());
        }
        
        // Get a mutable reference to the location
        let location = error.location.as_mut().unwrap();
        
        // Enhance the source location with file content
        self.enhance_source_location(location)?;
        
        // Create a detailed error message with source context
        if let Some(file) = &location.file {
            if let Ok(context) = self.source_file_cache().get_context(file, location.line, None) {
                // Add a formatted context to the error message
                let context_message = context.format();
                let full_message = format!("{} at {}:{}\n\n{}",
                    error.get_description(),
                    file,
                    location.line,
                    context_message
                );
                
                // Update the error message
                error.message = Some(full_message);
                
                debug!("Enhanced type assertion error with source context");
            }
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, message, location))]
    fn create_enhanced_error_message(
        &mut self,
        message: &str,
        location: &SourceLocation,
    ) -> Result<String, Error> {
        // Simple case - no file information
        if location.file.is_none() {
            return Ok(format!("{} at line {}, column {}", message, location.line, location.column));
        }
        
        // Get the file path
        let file_path = location.file.as_ref().unwrap();
        
        // Try to get source context
        if let Ok(context) = self.source_file_cache().get_context(file_path, location.line, None) {
            Ok(format!("{} at {}:{}\n\n{}", 
                      message, 
                      file_path, 
                      location.line, 
                      context.format()))
        } else {
            // Fallback if we can't get context
            Ok(format!("{} at {}:{},{}", 
                      message, 
                      file_path, 
                      location.line, 
                      location.column))
        }
    }
}

/// Utility functions for working with source files and contexts
pub mod helpers {
    use super::*;
    
    /// Format a source location with context from a file
    pub fn format_location_with_context(
        location: &SourceLocation,
        context_lines: Option<usize>
    ) -> String {
        let mut result = String::new();
        
        // Basic location information
        if let Some(file) = &location.file {
            result.push_str(&format!("{}:{},{}", file, location.line, location.column));
        } else {
            result.push_str(&format!("line {}, column {}", location.line, location.column));
        }
        
        // Add source line if available
        if !location.source_line.is_empty() {
            result.push_str(&format!("\n  | {}", location.source_line));
            
            // Add caret pointing to the column
            if location.column > 0 {
                result.push_str(&format!("\n  | {}{}", " ".repeat(location.column - 1), "^"));
            }
        }
        
        result
    }
    
    /// Create an enhanced error message from a type assertion error
    pub fn create_enhanced_error_message(error: &TypeAssertionError) -> String {
        let mut result = error.get_description();
        
        // Add location information if available
        if let Some(location) = &error.location {
            result.push_str(&format!("\n\nAt {}", format_location_with_context(location, None)));
        }
        
        // Add type ID information if available
        if let (Some(interface_id), Some(target_id)) = (error.interface_type_id, error.target_type_id) {
            result.push_str(&format!("\n\nType IDs:\n  Interface: 0x{:016x}\n  Target:    0x{:016x}", interface_id, target_id));
        }
        
        // Add actual type information if available
        if let Some(actual_type) = &error.actual_type {
            result.push_str(&format!("\n\nActual type: {}", actual_type));
            
            if let Some(actual_id) = error.actual_type_id {
                result.push_str(&format!(", ID: 0x{:016x}", actual_id));
            }
        }
        
        result
    }
    
    /// Try to load a source file and extract context
    pub fn try_load_source_context(
        file_path: impl AsRef<Path>,
        line: usize,
        context_lines: Option<usize>
    ) -> Option<SourceContext> {
        let mut cache = SourceFileCache::new();
        cache.get_context(file_path, line, context_lines).ok()
    }
}

// Register filesystem integration with the compiler
pub fn register_filesystem_integration() {
    trace!("Interface type assertion filesystem integration module registered");
    // This function is called during compiler initialization
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_source_file_cache() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5").unwrap();
        
        let mut cache = SourceFileCache::new();
        let file = cache.load_file(temp_file.path()).unwrap();
        
        assert_eq!(file.lines.len(), 5);
        assert_eq!(file.lines[0], "Line 1");
        assert_eq!(file.lines[4], "Line 5");
    }
    
    #[test]
    fn test_get_context() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5").unwrap();
        
        let mut cache = SourceFileCache::new();
        let context = cache.get_context(temp_file.path(), 3, Some(1)).unwrap();
        
        assert_eq!(context.focus_line, 3);
        assert_eq!(context.context_lines.len(), 3); // 2, 3, 4
        assert_eq!(context.context_lines[0].line_number, 2);
        assert_eq!(context.context_lines[1].line_number, 3);
        assert_eq!(context.context_lines[2].line_number, 4);
        assert_eq!(context.context_lines[1].is_focus, true);
        assert_eq!(context.context_lines[0].is_focus, false);
    }
    
    #[test]
    fn test_format_context() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5").unwrap();
        
        let mut cache = SourceFileCache::new();
        let context = cache.get_context(temp_file.path(), 3, Some(1)).unwrap();
        
        let formatted = context.format();
        assert!(formatted.contains("File:"));
        assert!(formatted.contains("Line 2"));
        assert!(formatted.contains("Line 3"));
        assert!(formatted.contains("Line 4"));
        assert!(formatted.contains(">"));
        assert!(formatted.contains("^"));
    }
    
    #[test]
    fn test_helpers() {
        let location = SourceLocation {
            line: 10,
            column: 5,
            file: Some("test.csd".to_string()),
            source_line: "    x.(Type)?".to_string(),
        };
        
        let formatted = helpers::format_location_with_context(&location, None);
        assert!(formatted.contains("test.csd:10,5"));
        assert!(formatted.contains("x.(Type)?"));
        assert!(formatted.contains("^"));
    }
}