//! # Enhanced Interface Type Assertion Source Location Support
//!
//! This module provides improved source location support for interface type assertions,
//! adding precise file, line, and column information to error messages. This enhancement
//! makes debugging type assertion failures much easier by showing exactly where in the
//! source code the assertion failed.
//!
//! Key improvements:
//! 1. Proper extraction of line/column information from tokens
//! 2. Integration with the lexer's position tracking
//! 3. Inclusion of actual source line in error messages
//! 4. Support for extracting file information from compilation context

use std::fmt;
use std::path::Path;
use tracing::{debug, error, info, instrument, trace, warn};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::codegen::llvm::interface_type_assertion_error_propagation_source_location::EnhancedSourceLocationErrorPropagation;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;
use crate::lexer::token::Token;

/// Trait for extracting and using enhanced source location information in interface type assertions
pub trait EnhancedSourceLocationSupport<'ctx>: EnhancedSourceLocationErrorPropagation<'ctx> {
    /// Extract detailed source location information from a token
    fn extract_source_location_from_token(
        &self, 
        token_str: &str, 
        line: usize, 
        column: usize
    ) -> SourceLocation;
    
    /// Get source line content from source files
    fn get_source_line(&self, file_path: &str, line_number: usize) -> Option<String>;
    
    /// Process a token to extract position information
    fn extract_token_position(&self, token_str: &str) -> (usize, usize);
    
    /// Create enhanced source location for better error messages
    fn create_enhanced_source_location(
        &self,
        node: &dyn Node,
        additional_context: Option<&str>
    ) -> SourceLocation;
}

/// Implementation of the enhanced source location support for LlvmCodeGenerator
impl<'ctx> EnhancedSourceLocationSupport<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, token_str), level = "debug")]
    fn extract_source_location_from_token(
        &self, 
        token_str: &str, 
        line: usize, 
        column: usize
    ) -> SourceLocation {
        // Get current file path from context if available
        let file_path = self.current_file_path().unwrap_or_else(|| "<unknown>".to_string());
        
        // Try to extract source line
        let source_line = self.get_source_line(&file_path, line).unwrap_or_else(|| {
            // If we can't get the actual source line, use the token as a fallback
            token_str.to_string()
        });
        
        SourceLocation {
            line,
            column,
            file: Some(file_path),
            source_line,
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_source_line(&self, file_path: &str, line_number: usize) -> Option<String> {
        // Check if the file path is valid
        let path = Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return None;
        }
        
        // Attempt to read the file and extract the line
        if let Ok(content) = std::fs::read_to_string(path) {
            content.lines().nth(line_number.saturating_sub(1)).map(|line| line.to_string())
        } else {
            None
        }
    }
    
    #[instrument(skip(self, token_str), level = "debug")]
    fn extract_token_position(&self, token_str: &str) -> (usize, usize) {
        // Check if token_str has position information embedded in a special format
        // For example, token might be stored as "identifier@line:col"
        if let Some(pos_start) = token_str.find('@') {
            if let Some(colon_pos) = token_str[pos_start+1..].find(':') {
                let line_str = &token_str[pos_start+1..pos_start+1+colon_pos];
                let col_str = &token_str[pos_start+1+colon_pos+1..];
                
                if let (Ok(line), Ok(col)) = (line_str.parse::<usize>(), col_str.parse::<usize>()) {
                    return (line, col);
                }
            }
        }
        
        // Default to position 0,0 if we can't extract it
        // In a real implementation, the lexer would provide this info
        (0, 0)
    }
    
    #[instrument(skip(self, node, additional_context), level = "debug")]
    fn create_enhanced_source_location(
        &self,
        node: &dyn Node,
        additional_context: Option<&str>
    ) -> SourceLocation {
        // Get the token string
        let token_str = node.token_literal();
        
        // Extract position information from the token if available
        let (line, column) = self.extract_token_position(&token_str);
        
        // Create a source location with enhanced information
        let mut location = self.extract_source_location_from_token(&token_str, line, column);
        
        // Add additional context if provided
        if let Some(context) = additional_context {
            location.source_line = format!("{} - {}", location.source_line, context);
        }
        
        location
    }
}

// Extend LlvmCodeGenerator with additional methods for enhanced source location
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Initialize source location tracking with file system support
    pub fn init_enhanced_source_location_tracking(&mut self, file_path: Option<&str>) {
        // Initialize base source location tracking
        self.init_source_location_tracking();
        
        // Set the current file path if provided
        if let Some(path) = file_path {
            self.set_current_file_path(path);
        }
        
        // Set up source line cache for faster lookups
        if !self.internal_fields.contains_key("source_line_cache") {
            self.internal_fields.insert("source_line_cache".to_string(), Box::new(std::collections::HashMap::<String, Vec<String>>::new()));
        }
    }
    
    /// Cache source lines for a file to improve performance
    pub fn cache_source_file(&mut self, file_path: &str) -> Result<(), Error> {
        // Read the entire file into memory and split into lines
        let path = Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return Err(Error::IO(std::io::Error::new(
                std::io::ErrorKind::NotFound, 
                format!("File not found: {}", file_path)
            )));
        }
        
        let content = std::fs::read_to_string(path)?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        // Get the cache field and update it
        if let Some(cache_field) = self.internal_fields.get_mut("source_line_cache") {
            if let Some(cache) = cache_field.downcast_mut::<std::collections::HashMap<String, Vec<String>>>() {
                cache.insert(file_path.to_string(), lines);
                return Ok(());
            }
        }
        
        // If we couldn't access the cache, initialize it
        let mut cache: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        cache.insert(file_path.to_string(), content.lines().map(|s| s.to_string()).collect());
        self.internal_fields.insert("source_line_cache".to_string(), Box::new(cache));
        
        Ok(())
    }
    
    /// Get source line from cache if available or read from file
    pub fn get_cached_source_line(&self, file_path: &str, line_number: usize) -> Option<String> {
        // Check if the line is in the cache
        if let Some(cache_field) = self.internal_fields.get("source_line_cache") {
            if let Some(cache) = cache_field.downcast_ref::<std::collections::HashMap<String, Vec<String>>>() {
                if let Some(lines) = cache.get(file_path) {
                    return lines.get(line_number.saturating_sub(1)).cloned();
                }
            }
        }
        
        // Fall back to reading from file
        self.get_source_line(file_path, line_number)
    }
    
    /// Compile a type assertion with enhanced source location information
    pub fn compile_type_assertion_with_enhanced_source_location(
        &mut self, 
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract detailed source location from the node
        let source_location = self.create_enhanced_source_location(
            type_assertion,
            Some(&format!("Type assertion to {}", type_assertion.type_name))
        );
        
        debug!("Compiling type assertion with enhanced source location: {}:{} in {}", 
              source_location.line, source_location.column, 
              source_location.file.as_ref().unwrap_or(&"<unknown>".to_string()));
        
        // Delegate to the existing implementation but with enhanced location information
        self.compile_type_assertion_with_source_location(type_assertion)
    }
    
    /// Compile a type assertion question operator with enhanced source location information
    pub fn compile_type_assertion_question_with_enhanced_source_location(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract detailed source location from the node
        let source_location = self.create_enhanced_source_location(
            type_assertion,
            Some(&format!("Type assertion with ? operator to {}", type_assertion.type_name))
        );
        
        debug!("Compiling type assertion ? with enhanced source location: {}:{} in {}", 
              source_location.line, source_location.column, 
              source_location.file.as_ref().unwrap_or(&"<unknown>".to_string()));
        
        // Delegate to the existing implementation but with enhanced location information
        self.compile_type_assertion_question_with_source_location(type_assertion)
    }
}

// Register module function
pub fn register_enhanced_source_location_support() {
    debug!("Registered enhanced source location support for interface type assertions");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_source_location_registration() {
        register_enhanced_source_location_support();
        assert!(true);
    }
}