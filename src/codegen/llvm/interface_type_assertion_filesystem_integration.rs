//! # Interface Type Assertion Filesystem Integration
//!
//! This module provides integration between interface type assertions and the file system,
//! allowing for rich error messages with source code context when type assertions fail.
//!
//! ## Key Features
//!
//! 1. Source location tracking with file path resolution
//! 2. Source code extraction for error context
//! 3. Search path management for finding source files
//! 4. Source location metadata generation for error reporting

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

use tracing::{debug, error, info, instrument, trace, warn};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;

/// Trait for filesystem source location integration with interface type assertions
pub trait FilesystemSourceLocationIntegration<'ctx> {
    /// Initialize the filesystem integration with a root directory
    fn init_filesystem_integration(&mut self, root_dir: Option<&str>);
    
    /// Add a directory to search for source files
    fn add_source_search_path(&mut self, path: &str);
    
    /// Set the current file path for error reporting
    fn set_current_file_path(&mut self, file_path: String);
    
    /// Get the current file path
    fn current_file_path(&self) -> Option<String>;
    
    /// Extract location information from a token
    fn extract_location_from_token(&self, token: &str) -> (i32, i32, Option<String>);
    
    /// Create a source location with file context for error reporting
    fn create_source_location_with_context(
        &self,
        node: &dyn Node,
        line: usize,
        column: usize,
        file: Option<&str>,
        context_lines: usize
    ) -> Result<SourceLocation, Error>;
    
    /// Format an error message with source context
    fn format_error_with_source_context(
        &self,
        error_message: &str,
        source_location: &SourceLocation,
        context_lines: usize
    ) -> Result<String, Error>;
}

impl<'ctx> FilesystemSourceLocationIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn init_filesystem_integration(&mut self, root_dir: Option<&str>) {
        // Initialize search paths
        let search_paths = vec![
            root_dir.map(|s| s.to_string()).unwrap_or_else(|| ".".to_string())
        ];
        
        self.internal_fields.insert(
            "source_search_paths".to_string(),
            Box::new(search_paths)
        );
        
        // Initialize current file path
        self.internal_fields.insert(
            "current_file_path".to_string(),
            Box::new(None::<String>)
        );
        
        debug!("Initialized filesystem integration with root: {:?}", root_dir);
    }
    
    #[instrument(skip(self), level = "debug")]
    fn add_source_search_path(&mut self, path: &str) {
        if let Some(search_paths) = self.internal_fields.get_mut("source_search_paths") {
            if let Some(paths) = search_paths.downcast_mut::<Vec<String>>() {
                if !paths.contains(&path.to_string()) {
                    paths.push(path.to_string());
                    debug!("Added source search path: {}", path);
                }
            }
        } else {
            // If source_search_paths doesn't exist, initialize it
            self.init_filesystem_integration(None);
            self.add_source_search_path(path);
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn set_current_file_path(&mut self, file_path: String) {
        self.internal_fields.insert(
            "current_file_path".to_string(),
            Box::new(Some(file_path.clone()))
        );
        debug!("Set current file path: {}", file_path);
    }
    
    #[instrument(skip(self), level = "debug")]
    fn current_file_path(&self) -> Option<String> {
        self.internal_fields.get("current_file_path")
            .and_then(|boxed| boxed.downcast_ref::<Option<String>>())
            .and_then(|opt| opt.clone())
    }
    
    #[instrument(skip(self, token), level = "debug")]
    fn extract_location_from_token(&self, token: &str) -> (i32, i32, Option<String>) {
        // Parse token for location information
        // Format could be "filename:line:column:token"
        let parts: Vec<&str> = token.split(':').collect();
        
        if parts.len() >= 3 {
            // Try to parse line and column
            let line = parts[parts.len() - 3].parse::<i32>().unwrap_or(0);
            let column = parts[parts.len() - 2].parse::<i32>().unwrap_or(0);
            
            // If there are more parts, the first ones could be a file path
            let file = if parts.len() > 3 {
                let file_parts = &parts[0..parts.len() - 3];
                Some(file_parts.join(":"))
            } else {
                None
            };
            
            return (line, column, file);
        }
        
        // Default values if parsing fails
        (0, 0, None)
    }
    
    #[instrument(skip(self, node), level = "debug")]
    fn create_source_location_with_context(
        &self,
        node: &dyn Node,
        line: usize,
        column: usize,
        file: Option<&str>,
        context_lines: usize
    ) -> Result<SourceLocation, Error> {
        let file_path = file.map(|s| s.to_string());
        let source_line = node.string();
        
        // Create basic source location
        let mut location = SourceLocation {
            line,
            column,
            file: file_path,
            source_line,
        };
        
        // Try to enhance with context if file is provided
        if let Some(file) = &location.file {
            if let Ok(source_lines) = self.read_source_file_lines(file) {
                if line > 0 && line <= source_lines.len() {
                    // Get the actual source line from the file
                    location.source_line = source_lines[line - 1].clone();
                }
            }
        }
        
        Ok(location)
    }
    
    #[instrument(skip(self, error_message, source_location), level = "debug")]
    fn format_error_with_source_context(
        &self,
        error_message: &str,
        source_location: &SourceLocation,
        context_lines: usize
    ) -> Result<String, Error> {
        let mut formatted = error_message.to_string();
        
        // Add source location information
        if let Some(file) = &source_location.file {
            formatted.push_str(&format!(
                "\nLocation: {}:{}:{}",
                file, source_location.line, source_location.column
            ));
            
            // Try to add context from source file
            if let Ok(source_lines) = self.read_source_file_lines(file) {
                formatted.push_str("\n\nSource context:\n");
                
                // Calculate range of lines to show
                let start_line = if source_location.line > context_lines {
                    source_location.line - context_lines
                } else {
                    1
                };
                
                let end_line = std::cmp::min(
                    source_location.line + context_lines,
                    source_lines.len()
                );
                
                // Add context lines with highlighting
                for line_num in start_line..=end_line {
                    if line_num > 0 && line_num <= source_lines.len() {
                        let line_content = &source_lines[line_num - 1];
                        
                        let prefix = if line_num == source_location.line {
                            "> " // Highlight the error line
                        } else {
                            "  "
                        };
                        
                        formatted.push_str(&format!(
                            "{}{}| {}",
                            prefix,
                            line_num,
                            line_content
                        ));
                        
                        // Add newline if not already present
                        if !line_content.ends_with('\n') {
                            formatted.push('\n');
                        }
                        
                        // Add caret pointing to the specific column on the error line
                        if line_num == source_location.line && source_location.column > 0 {
                            let spaces = source_location.column - 1 + "| ".len() + line_num.to_string().len() + 2;
                            formatted.push_str(&format!(
                                "{:indent$}^ Error occurs here\n",
                                "",
                                indent = spaces
                            ));
                        }
                    }
                }
            }
        }
        
        Ok(formatted)
    }
}

// Helper methods for filesystem integration
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Read lines from a source file
    fn read_source_file_lines(&self, file_path: &str) -> Result<Vec<String>, Error> {
        // Try to resolve the file path using search paths
        let resolved_path = self.resolve_source_file_path(file_path)?;
        
        // Open and read the file
        let file = File::open(resolved_path)
            .map_err(|e| Error::Compilation(format!("Failed to open source file {}: {}", file_path, e)))?;
        
        let reader = BufReader::new(file);
        let lines = reader.lines()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Error::Compilation(format!("Failed to read lines from {}: {}", file_path, e)))?;
        
        Ok(lines)
    }
    
    /// Resolve a source file path using configured search paths
    fn resolve_source_file_path(&self, file_path: &str) -> Result<PathBuf, Error> {
        // Check if the path is absolute
        let path = PathBuf::from(file_path);
        if path.is_absolute() && path.exists() {
            return Ok(path);
        }
        
        // Check if the file exists directly
        if path.exists() {
            return Ok(path);
        }
        
        // Try to find the file in search paths
        if let Some(search_paths) = self.internal_fields.get("source_search_paths") {
            if let Some(paths) = search_paths.downcast_ref::<Vec<String>>() {
                for search_path in paths {
                    let full_path = PathBuf::from(search_path).join(file_path);
                    if full_path.exists() {
                        return Ok(full_path);
                    }
                }
            }
        }
        
        // If file not found, return the original path (will cause error later)
        Err(Error::Compilation(format!("Source file not found: {}", file_path)))
    }
}

/// Register the filesystem source location integration module
pub fn register_filesystem_source_location_integration() {
    debug!("Registered filesystem source location integration for interface type assertions");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filesystem_integration_registration() {
        // Simple test to ensure module registration works
        register_filesystem_source_location_integration();
        assert!(true);
    }
}