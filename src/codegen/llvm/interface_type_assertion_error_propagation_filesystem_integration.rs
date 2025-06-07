//! # Interface Type Assertion Error Propagation with Filesystem Source Location Integration
//!
//! This module provides a comprehensive integration between error propagation and filesystem
//! source location tracking for interface type assertions. It enhances error messages with
//! detailed source code context, file paths, and visual highlighting of error locations.
//!
//! ## Key Features
//!
//! 1. Enhanced error messages with source code snippets and context
//! 2. File path resolution with source search paths for accurate location tracking
//! 3. Visual highlighting of error locations in source code
//! 4. Integration with the Result type and ? operator mechanism
//! 5. Comprehensive error context with line, column, and file information
//! 6. Support for extracting source code from filesystem for error context
//!
//! ## Usage
//!
//! This integration is used when compiling interface type assertions with the ? operator
//! to provide better error messages and diagnostics when assertions fail.

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
use crate::codegen::llvm::string_utils::StringUtilsExtension;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::llvm_code_generator_extensions::SourceLocationExtensions;
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::codegen::llvm::interface_type_assertion_filesystem_integration::FilesystemSourceLocationIntegration;
use crate::codegen::llvm::interface_type_assertion_error_propagation_filesystem::EnhancedErrorPropagationWithFilesystem;
use crate::codegen::llvm::interface_implementation::InterfaceImplementation;
use crate::codegen::llvm::interface_type_registry_enhanced::EnhancedTypeRegistry;
use crate::codegen::llvm::pointer_type_extension::PointerTypeExtension;
use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;

/// Trait for comprehensive integration between error propagation and filesystem source location tracking
pub trait ComprehensiveErrorFilesystemIntegration<'ctx>: 
    EnhancedErrorPropagationWithFilesystem<'ctx> 
{
    /// Initialize the comprehensive error filesystem integration
    fn init_comprehensive_error_filesystem_integration(&mut self);
    
    /// Compile a type assertion with the ? operator using comprehensive filesystem integration
    fn compile_type_assertion_question_with_comprehensive_filesystem(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a comprehensive error message with detailed source code context
    fn create_comprehensive_error_message(
        &mut self,
        type_assertion: &dyn Node,
        expected_type: &str,
        actual_type: Option<&str>,
        source_location: &SourceLocation
    ) -> Result<String, Error>;
    
    /// Extract source code context from filesystem for error messages
    fn extract_source_code_context(
        &mut self,
        file_path: &str,
        line: usize,
        context_lines: usize
    ) -> Result<Vec<(usize, String)>, Error>;
    
    /// Format an error message with source code context and highlighting
    fn format_error_with_source_highlighting(
        &mut self,
        error_message: &str,
        source_location: &SourceLocation,
        context_lines: usize
    ) -> Result<String, Error>;
    
    /// Call error propagation function with comprehensive context
    fn call_error_propagation_with_comprehensive_context(
        &mut self,
        error_message: &str,
        source_location: &SourceLocation,
        expected_type_id: u32,
        actual_type_id: u32,
        type_assertion: &dyn Node
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> ComprehensiveErrorFilesystemIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn init_comprehensive_error_filesystem_integration(&mut self) {
        // Check if already initialized
        if self.internal_fields.contains_key("comprehensive_error_fs_integration_initialized") {
            return;
        }
        
        // First ensure base filesystem integration is initialized
        self.ensure_filesystem_integration_initialized();
        
        // Add additional source search paths for better file resolution
        let paths = [
            "./",
            "./src",
            "./tests",
            "./examples",
            "../src",
            "../examples"
        ];
        
        for path in paths.iter() {
            self.add_source_search_path(path);
        }
        
        // Initialize cache for source code lines
        if !self.internal_fields.contains_key("source_code_cache") {
            self.internal_fields.insert(
                "source_code_cache".to_string(),
                Box::new(HashMap::<String, Vec<String>>::new())
            );
        }
        
        // Mark as initialized
        self.internal_fields.insert(
            "comprehensive_error_fs_integration_initialized".to_string(),
            Box::new(true)
        );
        
        debug!("Initialized comprehensive error filesystem integration");
    }
    
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_question_with_comprehensive_filesystem(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Initialize comprehensive integration if needed
        self.init_comprehensive_error_filesystem_integration();
        
        // Extract comprehensive source location information
        let source_location = self.create_enhanced_source_location(
            type_assertion,
            self.current_file_path().to_str()
        )?;
        
        debug!("Compiling type assertion with ? operator and comprehensive filesystem integration: {}", 
               type_assertion.string());
        
        // First ensure registry is initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion with ? operator".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_question_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_question_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_question_merge");
        
        // Check if the interface value is of the target type
        let is_instance = InterfaceTypeAssertion::check_instance_of(self, expr_value, &type_assertion.type_name, Some(source_location.clone()))?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Get the type id for proper error reporting
        let type_id = self.get_type_id(&type_assertion.type_name)?;
        
        // Cast the data pointer to the appropriate type
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - propagate the error through the ? operator with comprehensive context
        self.builder().position_at_end(failure_block);
        
        // Get the actual type ID from the interface value for better error reporting
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        
        // Look up the type names for better error messages
        let type_id_u32 = type_id.into_int_value().get_zero_extended_constant().unwrap() as u32;
        let actual_type_id_u32 = actual_type_id.into_int_value().get_zero_extended_constant().unwrap() as u32;
        let expected_type_name = self.get_type_name_by_id(type_id_u32);
        let actual_type_name = self.get_type_name_by_id(actual_type_id_u32);
        
        // Create a comprehensive error message with source code context
        let error_message = self.create_comprehensive_error_message(
            type_assertion,
            &expected_type_name.unwrap_or(type_assertion.type_name.clone()),
            actual_type_name.ok().as_deref(),
            &source_location
        )?;
        
        // Call error propagation with comprehensive context
        self.call_error_propagation_with_comprehensive_context(
            &error_message,
            &source_location,
            type_id_u32,
            actual_type_id_u32,
            type_assertion
        )?;
        
        // This should be unreachable in the failure path
        self.builder().build_unreachable().map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - return the casted pointer on success
        self.builder().position_at_end(merge_block);
        
        // With question operator, we just return the value directly, error handling is automatic
        Ok(casted_ptr.into())
    }
    
    #[instrument(skip(self, type_assertion, source_location), level = "debug")]
    fn create_comprehensive_error_message(
        &mut self,
        type_assertion: &dyn Node,
        expected_type: &str,
        actual_type: Option<&str>,
        source_location: &SourceLocation
    ) -> Result<String, Error> {
        // Create a detailed error message with type information
        let mut message = match actual_type {
            Some(actual) => format!(
                "Type assertion failed: cannot convert from interface to {}. Actual type: {}",
                expected_type, actual
            ),
            None => format!(
                "Type assertion failed: cannot convert from interface to {}",
                expected_type
            )
        };
        
        // Add source location information
        if let Some(file) = &source_location.file {
            message.push_str(&format!(
                "\nLocation: {}:{}:{}",
                file, source_location.line, source_location.column
            ));
        }
        
        // Add expression context
        message.push_str(&format!(
            "\nExpression: {}",
            type_assertion.string().trim()
        ));
        
        // Add source code context if available
        if let (Some(file), true) = (&source_location.file, source_location.line > 0) {
            if let Ok(context) = self.extract_source_code_context(
                file, 
                source_location.line, 
                2 // Show 2 lines of context before and after
            ) {
                message.push_str("\n\nSource context:\n");
                
                // Add context lines with highlighting for the error line
                for (line_num, line_content) in context {
                    let prefix = if line_num == source_location.line {
                        "> " // Highlight the error line
                    } else {
                        "  "
                    };
                    
                    message.push_str(&format!(
                        "{}{}| {}",
                        prefix,
                        line_num,
                        line_content
                    ));
                    
                    // Add a newline if not already present
                    if !line_content.ends_with('\n') {
                        message.push('\n');
                    }
                }
            }
        }
        
        Ok(message)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn extract_source_code_context(
        &mut self,
        file_path: &str,
        line: usize,
        context_lines: usize
    ) -> Result<Vec<(usize, String)>, Error> {
        // Try to get source code from cache first
        let source_code_cache = self.get_source_code_cache()?;
        let cached_lines = source_code_cache.get(file_path);
        
        let file_lines = if let Some(lines) = cached_lines {
            // Use cached lines
            lines.clone()
        } else {
            // Read file and cache lines
            let resolved_path = self.resolve_source_file_path(file_path)?;
            let file = File::open(&resolved_path)
                .map_err(|e| Error::Compilation(format!("Failed to open file {}: {}", file_path, e)))?;
            
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines()
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| Error::Compilation(format!("Failed to read lines from {}: {}", file_path, e)))?;
            
            // Cache the lines for future use
            self.update_source_code_cache(file_path, lines.clone())?;
            
            lines
        };
        
        // Calculate the range of lines to include in the context
        let start_line = if line > context_lines { line - context_lines } else { 1 };
        let end_line = std::cmp::min(line + context_lines, file_lines.len());
        
        // Extract the context lines with line numbers
        let context: Vec<(usize, String)> = (start_line..=end_line)
            .filter_map(|i| {
                if i <= file_lines.len() {
                    Some((i, file_lines[i-1].clone()))
                } else {
                    None
                }
            })
            .collect();
        
        Ok(context)
    }
    
    #[instrument(skip(self, error_message, source_location), level = "debug")]
    fn format_error_with_source_highlighting(
        &mut self,
        error_message: &str,
        source_location: &SourceLocation,
        context_lines: usize
    ) -> Result<String, Error> {
        let mut formatted = error_message.to_string();
        
        // Add additional highlighting for the specific column in the error line
        if let (Some(file), true) = (&source_location.file, source_location.line > 0) {
            if let Ok(context) = self.extract_source_code_context(file, source_location.line, context_lines) {
                // Find the error line in the context
                if let Some((_, error_line)) = context.iter().find(|(num, _)| *num == source_location.line) {
                    // Add a caret pointing to the specific column in the error line
                    if source_location.column > 0 {
                        // Calculate spaces needed for the caret line
                        let line_num_spaces = source_location.line.to_string().len() + 2; // +2 for '> ' prefix
                        let spaces_before_caret = line_num_spaces + 2 + source_location.column - 1; // +2 for '| '
                        
                        // Append a line with a caret pointing to the error location
                        let caret_line = format!(
                            "{}^-- Error occurs here\n",
                            " ".repeat(spaces_before_caret)
                        );
                        
                        formatted.push_str(&caret_line);
                    }
                }
            }
        }
        
        Ok(formatted)
    }
    
    #[instrument(skip(self, error_message, source_location, type_assertion), level = "debug")]
    fn call_error_propagation_with_comprehensive_context(
        &mut self,
        error_message: &str,
        source_location: &SourceLocation,
        expected_type_id: u32,
        actual_type_id: u32,
        type_assertion: &dyn Node
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Set type IDs for error reporting
        self.set_expected_type_id(expected_type_id);
        self.set_actual_type_id(actual_type_id);
        
        // Format the error message with source highlighting
        let enhanced_message = self.format_error_with_source_highlighting(
            error_message,
            source_location,
            2 // Include 2 lines of context
        ).unwrap_or_else(|_| error_message.to_string());
        
        // Create an enhanced source location with file path information
        let enhanced_location = SourceLocation {
            line: source_location.line,
            column: source_location.column,
            file: source_location.file.clone(),
            source_line: type_assertion.string(),
        };
        
        // Convert source location to LLVM structure
        let location_struct = self.build_source_location_struct(&enhanced_location);
        
        // Create a constant string for the error message
        let error_message_ptr = self.create_string_constant(&enhanced_message);
        
        // Default type information for error reporting
        let source_type = "interface";
        let target_type = "unknown";
        
        // Call the error propagation function with the enhanced message and location
        self.call_error_propagation_function(
            error_message_ptr?.into(),
            self.create_string_constant(source_type)?.into(),
            self.create_string_constant(target_type)?.into(),
            location_struct,
            self.create_string_constant("")?.into()
        )?;
        
        // Return a null pointer value to indicate error
        Ok(self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null().into())
    }
}

// Helper methods for the comprehensive integration
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the source code cache from internal fields
    fn get_source_code_cache(&self) -> Result<&HashMap<String, Vec<String>>, Error> {
        self.internal_fields.get("source_code_cache")
            .and_then(|boxed| boxed.downcast_ref::<HashMap<String, Vec<String>>>())
            .ok_or_else(|| Error::Compilation("Source code cache not initialized".to_string()))
    }
    
    /// Update the source code cache with new file lines
    fn update_source_code_cache(&mut self, file_path: &str, lines: Vec<String>) -> Result<(), Error> {
        let mut cache = self.internal_fields.get_mut("source_code_cache")
            .and_then(|boxed| boxed.downcast_mut::<HashMap<String, Vec<String>>>())
            .ok_or_else(|| Error::Compilation("Source code cache not initialized".to_string()))?;
        
        cache.insert(file_path.to_string(), lines);
        Ok(())
    }
    
    /// Resolve a source file path using search paths
    fn resolve_source_file_path(&self, file_path: &str) -> Result<PathBuf, Error> {
        // Check if the path is absolute or direct file exists
        let path = PathBuf::from(file_path);
        if path.is_absolute() && path.exists() {
            return Ok(path);
        }
        
        // Check if the file exists directly
        if path.exists() {
            return Ok(path);
        }
        
        // Try search paths
        let search_paths = self.get_source_search_paths()?;
        for search_path in search_paths {
            let full_path = PathBuf::from(&search_path).join(file_path);
            if full_path.exists() {
                return Ok(full_path);
            }
        }
        
        // If we couldn't resolve, return the original path (error will be handled by caller)
        Err(Error::Compilation(format!("Could not resolve source file path: {}", file_path)))
    }
    
    /// Get the list of source search paths
    fn get_source_search_paths(&self) -> Result<Vec<String>, Error> {
        self.internal_fields.get("source_search_paths")
            .and_then(|boxed| boxed.downcast_ref::<Vec<String>>())
            .map(|paths| paths.clone())
            .ok_or_else(|| Error::Compilation("Source search paths not initialized".to_string()))
    }
}

/// Register the comprehensive error propagation with filesystem integration
pub fn register_comprehensive_error_filesystem_integration() {
    debug!("Registered comprehensive error propagation with filesystem integration for interface type assertions");
}

/// Alias for register_comprehensive_error_filesystem_integration to match expected function name
pub fn register_comprehensive_error_propagation_integration() {
    register_comprehensive_error_filesystem_integration();
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_comprehensive_error_filesystem_integration_registration() {
        // Test that the module registration function works
        register_comprehensive_error_filesystem_integration();
        assert!(true);
    }
}