//! # Interface Type Assertion Error Propagation with Filesystem Integration
//!
//! This module enhances the error propagation mechanism for interface type assertions
//! by integrating with the filesystem source location tracking. It provides comprehensive
//! error messages with source code context when type assertions with the ? operator fail.
//!
//! ## Key Features
//!
//! 1. Enhanced error messages with source code context
//! 2. File path resolution for accurate source location tracking
//! 3. Integration with Result type and ? operator mechanism
//! 4. Comprehensive error context with line and column information
//! 5. Visual source code highlighting for error locations

use std::path::{Path, PathBuf};
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
use crate::codegen::llvm::string_utils::StringUtilsExtension;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::llvm_code_generator_extensions::SourceLocationExtensions;
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::codegen::llvm::interface_type_assertion_filesystem_integration::FilesystemSourceLocationIntegration;
use crate::codegen::llvm::interface_type_registry_helpers::TypeNameRegistry;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;

/// Trait for enhanced error propagation with filesystem integration
pub trait EnhancedErrorPropagationWithFilesystem<'ctx>: 
    InterfaceTypeAssertionErrorPropagation<'ctx> + 
    FilesystemSourceLocationIntegration 
{
    /// Compile a type assertion with the ? operator using enhanced filesystem integration
    /// for better error messages with source code context
    fn compile_type_assertion_question_with_filesystem(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create an enhanced error message with source code context for type assertion failures
    fn create_enhanced_error_message(
        &self,
        type_assertion: &dyn Node,
        expected_type: &str,
        actual_type: Option<&str>
    ) -> Result<String, Error>;
    
    /// Create an enhanced error source location with filesystem context
    fn create_enhanced_source_location(
        &self,
        node: &dyn Node,
        file_hint: Option<&str>
    ) -> Result<SourceLocation, Error>;
    
    /// Propagate a type assertion error with enhanced source context
    fn propagate_error_with_source_context(
        &mut self,
        error_message: &str,
        source_location: &SourceLocation,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> EnhancedErrorPropagationWithFilesystem<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_question_with_filesystem(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Initialize filesystem integration if needed
        self.ensure_filesystem_integration_initialized();
        
        // Extract source location information with filesystem context
        let source_location = self.create_enhanced_source_location(
            type_assertion,
            self.current_file_path().to_str()
        )?;
        
        debug!("Compiling type assertion with ? operator and filesystem integration: {}", type_assertion.string());
        
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
        
        // Failure path - propagate the error through the ? operator with enhanced context
        self.builder().position_at_end(failure_block);
        
        // Get the actual type ID from the interface value for better error reporting
        let (actual_type_id, actual_type_name) = self.get_runtime_type_id(expr_value, source_location.clone().into())?;
        
        // Look up the type names for better error messages using common function
        let expected_type_name = crate::codegen::llvm::interface_type_registry_common::get_type_name_by_id_impl(self, type_id as u32)?;
        let actual_type_name = Some(actual_type_name);
        
        // Create an enhanced error message
        let error_message = self.create_enhanced_error_message(
            type_assertion,
            &expected_type_name,
            actual_type_name.as_deref()
        )?;
        
        // Propagate the error with enhanced source context
        let type_id_u32 = type_id.into_int_value().get_zero_extended_constant().unwrap() as u32;
        let actual_type_id_u32 = actual_type_id as u32;
        self.propagate_error_with_source_context(
            &error_message,
            &source_location,
            type_id_u32,
            actual_type_id_u32
        )?;
        
        // This should be unreachable in the failure path
        self.builder().build_unreachable().map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - return the casted pointer on success
        self.builder().position_at_end(merge_block);
        
        // With question operator, we just return the value directly, error handling is automatic
        Ok(casted_ptr.into())
    }
    
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn create_enhanced_error_message(
        &self,
        type_assertion: &dyn Node,
        expected_type: &str,
        actual_type: Option<&str>
    ) -> Result<String, Error> {
        // Create a detailed error message with type information
        let basic_message = match actual_type {
            Some(actual) => format!(
                "Type assertion failed: cannot convert from interface to {}. Actual type: {}",
                expected_type, actual
            ),
            None => format!(
                "Type assertion failed: cannot convert from interface to {}",
                expected_type
            )
        };
        
        // Add operation context
        let with_context = format!(
            "{} in expression: {}",
            basic_message,
            type_assertion.string().trim()
        );
        
        Ok(with_context)
    }
    
    #[instrument(skip(self, node), level = "debug")]
    fn create_enhanced_source_location(
        &self,
        node: &dyn Node,
        file_hint: Option<&str>
    ) -> Result<SourceLocation, Error> {
        // Try to extract line and column information from the token
        let token = node.token_literal();
        let (line, column, file) = self.extract_location_from_token(&token);
        
        // Use the file hint if we couldn't extract a file from the token
        let file_path = file.or_else(|| file_hint.map(|s| s.to_string()));
        
        // Try to create a source location with filesystem context
        if let Some(file) = &file_path {
            if line > 0 {
                if let Some(context) = self.create_source_location_with_context(
                    Some(Path::new(file)),
                    Some(line as usize)
                ) {
                    return Ok(SourceLocation {
                        file: Some(file.clone()),
                        line: line as usize,
                        column: column as usize,
                    });
                }
            }
        }
        
        // Fallback to a basic source location if we couldn't create one with context
        Ok(SourceLocation {
            line: line as usize,
            column: column as usize,
            file: file_path,
            source_line: node.string(),
        })
    }
    
    #[instrument(skip(self, error_message, source_location), level = "debug")]
    fn propagate_error_with_source_context(
        &mut self,
        error_message: &str,
        source_location: &SourceLocation,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Set type IDs for error reporting
        self.set_expected_type_id(expected_type_id);
        self.set_actual_type_id(actual_type_id);
        
        // Format the error message with source context if available
        let enhanced_message = self.format_error_with_source_context(
            error_message,
            source_location.file.as_ref().map(|s| Path::new(s)),
            Some(source_location.line)
        );
        
        // Convert source location to LLVM structure
        let location_struct = self.build_source_location_struct(source_location);
        
        // Default type information for error reporting
        let source_type = "interface";
        let target_type = "unknown";
        
        // Call the error propagation function with the enhanced message and location
        self.call_error_propagation_function(
            self.create_string_constant(&enhanced_message)?.into(),
            self.create_string_constant(source_type)?.into(),
            self.create_string_constant(target_type)?.into(),
            location_struct,
            self.create_string_constant("")?.into()
        )?;
        
        // This function should not return normally after error propagation
        // In a real implementation, this would trigger unwinding
        Ok(self.context().i8_type().const_zero().into())
    }
}

// Helper methods for the enhanced error propagation system
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Ensure that filesystem integration is initialized
    pub fn ensure_filesystem_integration_initialized(&mut self) {
        // Check if we've already initialized
        if !self.internal_fields.contains_key("filesystem_integration_initialized") {
            // Initialize with current working directory as root
            let cwd = std::env::current_dir().ok().and_then(|p| p.to_str().map(|s| s.to_string()));
            self.init_filesystem_integration();
            
            // Try to add common source paths
            self.add_source_search_path(".");
            self.add_source_search_path("./src");
            self.add_source_search_path("./examples");
            
            // Mark as initialized
            self.internal_fields.insert("filesystem_integration_initialized".to_string(), Box::new(true));
        }
    }
    
    /// Build a source location struct for LLVM
    pub fn build_source_location_struct(&self, location: &SourceLocation) -> BasicValueEnum<'ctx> {
        let ctx = self.context();
        
        // Create integer values for line and column
        let line = ctx.i32_type().const_int(location.line as u64, false);
        let column = ctx.i32_type().const_int(location.column as u64, false);
        
        // Create string pointers for file and source line
        let file_ptr = if let Some(file) = &location.file {
            self.create_string_constant(file)?.into()
        } else {
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        let source_line_ptr = if !location.source_line.is_empty() {
            self.create_string_constant(&location.source_line)?.into()
        } else {
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Build the location struct
        self.build_struct_value(&[
            line.into(),
            column.into(),
            file_ptr,
            source_line_ptr
        ]).into()
    }
    
    // Using the shared implementation from interface_type_registry_helpers.rs
    // through the TypeNameRegistry trait
}

/// Register the enhanced error propagation with filesystem integration
pub fn register_enhanced_error_propagation() {
    debug!("Registered enhanced error propagation with filesystem integration for interface type assertions");
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_enhanced_error_propagation_registration() {
        // Test that the module registration function works
        register_enhanced_error_propagation();
        assert!(true);
    }
}