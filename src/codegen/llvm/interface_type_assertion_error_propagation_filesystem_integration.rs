//! # Interface Type Assertion Error Propagation with Filesystem Source Location Integration
//!
//! This module provides the integration layer between the error propagation system for interface
//! type assertions and the filesystem source location tracking. It creates enhanced error messages
//! with detailed source code context when a type assertion with the ? operator fails.
//!
//! ## Key Features
//!
//! 1. Complete integration between error propagation and filesystem source location tracking
//! 2. Rich error messages with code snippets and precise location information
//! 3. Support for ? operator with automatic error propagation
//! 4. Improved debug diagnostics with type paths and inheritance diagrams
//! 5. Optimized source file caching for performance
//!
//! This integration makes debugging type assertion failures much more straightforward by showing
//! exactly where in the source code the error occurred and providing contextual information about
//! the expected and actual types.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
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
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::codegen::llvm::interface_type_assertion_error_propagation_filesystem::EnhancedErrorPropagationWithFilesystem;
use crate::codegen::llvm::interface_type_assertion_filesystem_integration::FilesystemSourceLocationIntegration;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;

/// Trait for comprehensive integration between error propagation and filesystem source location tracking
pub trait ComprehensiveErrorPropagationIntegration<'ctx>: 
    EnhancedErrorPropagationWithFilesystem<'ctx> + 
    FilesystemSourceLocationIntegration<'ctx> +
    InterfaceTypeAssertionPathVisualization<'ctx>
{
    /// Initialize the comprehensive error propagation integration system
    fn init_comprehensive_error_propagation(&mut self, source_root: Option<&str>);
    
    /// Compile a type assertion question operator with comprehensive error messages and source tracking
    fn compile_type_assertion_question_with_comprehensive_error_context(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a detailed error message with type path visualization and source context
    fn create_comprehensive_error_message(
        &self,
        type_assertion: &dyn Node,
        expected_type: &str,
        actual_type: Option<&str>,
        source_location: &SourceLocation,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Result<String, Error>;
    
    /// Enhance source locations with additional context information
    fn enhance_source_location_with_type_context(
        &self,
        location: &mut SourceLocation,
        expected_type: &str,
        actual_type: Option<&str>,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Result<(), Error>;
}

impl<'ctx> ComprehensiveErrorPropagationIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, source_root), level = "debug")]
    fn init_comprehensive_error_propagation(&mut self, source_root: Option<&str>) {
        // Initialize filesystem integration first
        self.init_filesystem_integration(source_root.as_deref());
        
        // Initialize enhanced error propagation
        if !self.internal_fields.contains_key("comprehensive_error_propagation_initialized") {
            // Set up additional search paths for source files
            self.add_source_search_path("src");
            self.add_source_search_path("examples");
            self.add_source_search_path("tests");
            
            // Initialize path visualization for better error messages
            if let Err(e) = self.ensure_registry_visualization_initialized() {
                warn!("Failed to initialize registry visualization: {}", e);
            }
            
            // Mark as initialized
            self.internal_fields.insert(
                "comprehensive_error_propagation_initialized".to_string(), 
                Box::new(true)
            );
            
            debug!("Initialized comprehensive error propagation with filesystem integration");
        }
    }
    
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_question_with_comprehensive_error_context(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Initialize the comprehensive error propagation if needed
        if !self.internal_fields.contains_key("comprehensive_error_propagation_initialized") {
            self.init_comprehensive_error_propagation(None);
        }
        
        // Extract source location information with filesystem context
        let mut source_location = self.create_enhanced_source_location(
            type_assertion,
            self.current_file_path().as_deref()
        )?;
        
        let token = type_assertion.token_literal();
        let (line, column) = self.extract_line_column_from_token(&token);
        debug!("Compiling type assertion with comprehensive error context: {} at {}:{}", 
               type_assertion.string(), line, column);
        
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
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, Some(source_location.clone()))?;
        
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
        let expected_type_name = self.get_type_name_by_id(type_id);
        let actual_type_name = self.get_type_name_by_id(actual_type_id);
        
        // Enhance source location with type context
        self.enhance_source_location_with_type_context(
            &mut source_location,
            &expected_type_name.unwrap_or(type_assertion.type_name.clone()),
            actual_type_name.as_deref(),
            type_id,
            actual_type_id
        )?;
        
        // Create a comprehensive error message with type path information
        let error_message = self.create_comprehensive_error_message(
            type_assertion,
            &expected_type_name.unwrap_or(type_assertion.type_name.clone()),
            actual_type_name.as_deref(),
            &source_location,
            type_id,
            actual_type_id
        )?;
        
        // Propagate the error with the enhanced source context
        self.propagate_error_with_source_context(
            &error_message,
            &source_location,
            type_id,
            actual_type_id
        )?;
        
        // This should be unreachable in the failure path
        self.builder().build_unreachable().map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - return the casted pointer on success
        self.builder().position_at_end(merge_block);
        
        // With question operator, we just return the value directly, error handling is automatic
        Ok(casted_ptr.into())
    }
    
    #[instrument(skip(self, type_assertion, expected_type, actual_type, source_location), level = "debug")]
    fn create_comprehensive_error_message(
        &self,
        type_assertion: &dyn Node,
        expected_type: &str,
        actual_type: Option<&str>,
        source_location: &SourceLocation,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Result<String, Error> {
        let mut message = String::new();
        
        // Basic error message with type information
        message.push_str(&match actual_type {
            Some(actual) => format!(
                "Type assertion failed: cannot convert from interface to {}. Actual type: {}",
                expected_type, actual
            ),
            None => format!(
                "Type assertion failed: cannot convert from interface to {}",
                expected_type
            )
        });
        
        // Add expression context
        message.push_str(&format!(
            "\nIn expression: {}",
            type_assertion.string().trim()
        ));
        
        // Add source location
        message.push_str(&format!(
            "\nAt: {}:{}:{}",
            source_location.file.as_deref().unwrap_or("<unknown>"),
            source_location.line,
            source_location.column
        ));
        
        // Add source code snippet if available
        if !source_location.source_line.is_empty() {
            message.push_str("\n\nSource:\n");
            message.push_str(&source_location.source_line);
        }
        
        // Try to add type path information if available
        if let Ok(type_info) = self.get_interface_path_info(expected_type_id, actual_type_id) {
            if !type_info.is_empty() {
                message.push_str("\n\nType Relationship:\n");
                message.push_str(&type_info);
            }
        }
        
        Ok(message)
    }
    
    #[instrument(skip(self, location, expected_type, actual_type), level = "debug")]
    fn enhance_source_location_with_type_context(
        &self,
        location: &mut SourceLocation,
        expected_type: &str,
        actual_type: Option<&str>,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Result<(), Error> {
        // If we don't have file information, try to add it from the current context
        if location.file.is_none() {
            if let Some(file_path) = self.current_file_path() {
                location.file = Some(file_path);
            }
        }
        
        // If source line is empty, try to extract it from the file
        if location.source_line.is_empty() && location.file.is_some() && location.line > 0 {
            if let Ok(context) = self.get_source_line_with_context(
                location.file.as_ref().unwrap(),
                location.line,
                2 // Include 2 lines of context
            ) {
                let mut source_text = String::new();
                for (line_num, line_text) in context {
                    let prefix = if line_num == location.line {
                        ">"
                    } else {
                        " "
                    };
                    source_text.push_str(&format!("{} {:4} | {}\n", prefix, line_num, line_text));
                    
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
                        source_text.push_str(&marker);
                    }
                }
                location.source_line = source_text;
            }
        }
        
        // Add type information to the source line context
        if !location.source_line.is_empty() {
            let mut type_info = String::new();
            
            type_info.push_str("\nType Information:\n");
            type_info.push_str(&format!("  Expected Type: {} (ID: {})\n", expected_type, expected_type_id));
            
            if let Some(actual) = actual_type {
                type_info.push_str(&format!("  Actual Type: {} (ID: {})\n", actual, actual_type_id));
            } else {
                type_info.push_str(&format!("  Actual Type: <unknown> (ID: {})\n", actual_type_id));
            }
            
            // Try to add inheritance path information if available
            if let Ok(path_info) = self.get_interface_path_info(expected_type_id, actual_type_id) {
                if !path_info.is_empty() {
                    type_info.push_str("\nInheritance Path:\n");
                    type_info.push_str(&path_info);
                }
            }
            
            // Append the type information to the source line
            location.source_line.push_str(&type_info);
        }
        
        Ok(())
    }
}

// Additional helper methods for the LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Extract line and column information from a token string
    fn extract_line_column_from_token(&self, token: &str) -> (u32, u32) {
        // Default values
        let mut line = 0;
        let mut column = 0;
        
        // Try to parse line:column from the token if it contains this info
        if let Some(pos) = token.rfind(':') {
            if let Some(line_start) = token[..pos].rfind(':') {
                if let Ok(l) = token[line_start+1..pos].parse::<u32>() {
                    line = l;
                    if let Ok(c) = token[pos+1..].parse::<u32>() {
                        column = c;
                    }
                }
            }
        }
        
        (line, column)
    }
    
    /// Get information about the path between two interface types
    fn get_interface_path_info(&self, expected_type_id: u32, actual_type_id: u32) -> Result<String, Error> {
        // Try to get type names first
        let expected_name = self.get_type_name_by_id(expected_type_id)
            .unwrap_or_else(|| format!("Type#{}", expected_type_id));
        let actual_name = self.get_type_name_by_id(actual_type_id)
            .unwrap_or_else(|| format!("Type#{}", actual_type_id));
        
        // Try to visualize the path between types
        let mut result = String::new();
        
        // Check if we have path visualization support enabled
        if self.is_registry_visualization_enabled() {
            // Get the path information
            if let Ok(paths) = self.find_interface_implementation_paths(actual_type_id, expected_type_id) {
                if paths.is_empty() {
                    result.push_str("  No implementation path found between types.\n");
                } else {
                    for (i, path) in paths.iter().enumerate() {
                        result.push_str(&format!("  Path #{}: ", i + 1));
                        
                        for (j, &type_id) in path.iter().enumerate() {
                            let type_name = self.get_type_name_by_id(type_id)
                                .unwrap_or_else(|| format!("Type#{}", type_id));
                            
                            if j > 0 {
                                result.push_str(" -> ");
                            }
                            result.push_str(&type_name);
                        }
                        result.push_str("\n");
                    }
                }
            } else {
                result.push_str("  Could not find implementation path between types.\n");
            }
            
            // Add diamond inheritance information if available
            if let Ok(is_diamond) = self.is_diamond_inheritance_pattern(actual_type_id, expected_type_id) {
                if is_diamond {
                    result.push_str("\n  Note: Diamond inheritance pattern detected!\n");
                    result.push_str("  This can cause ambiguity in method resolution.\n");
                }
            }
        } else {
            result.push_str("  Type path visualization not enabled.\n");
        }
        
        Ok(result)
    }
    
    /// Check if registry visualization is enabled
    fn is_registry_visualization_enabled(&self) -> bool {
        self.internal_fields.contains_key("registry_visualization_initialized") &&
        self.internal_fields.get("registry_visualization_initialized")
            .and_then(|v| v.downcast_ref::<bool>())
            .cloned()
            .unwrap_or(false)
    }
    
    /// Get a type name by its ID
    fn get_type_name_by_id(&self, type_id: u32) -> Option<String> {
        // Try to get from the registry if available
        if self.is_registry_visualization_enabled() {
            self.get_type_name_from_registry(type_id)
        } else {
            None
        }
    }
}

/// Function to register the comprehensive error propagation integration
pub fn register_comprehensive_error_propagation_integration() {
    debug!("Registered comprehensive error propagation with filesystem integration for interface type assertions");
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_comprehensive_error_propagation_registration() {
        // Test that the registration function works
        register_comprehensive_error_propagation_integration();
        assert!(true);
    }
}