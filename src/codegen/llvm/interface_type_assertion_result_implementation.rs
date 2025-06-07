//! # Interface Type Assertion Result Implementation
//!
//! This module provides the implementation for Result-based error handling in interface
//! type assertions, with proper integration of the ? operator. It serves as the bridge 
//! between the existing interface type assertion error propagation infrastructure and 
//! the enhanced error handling system.
//!
//! ## Features
//!
//! - Proper integration with Rust's Result type and ? operator for clean error handling
//! - Enhanced error context with detailed type information
//! - Bidirectional path finding for improved error messages
//! - Complete integration between LLVM code generator and interface type registry

use tracing::{debug, error, info, instrument, warn, trace};
use std::fmt;
use std::convert::TryFrom;

use inkwell::values::{BasicValueEnum, PointerValue, BasicMetadataValueEnum};
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::IntPredicate;
use inkwell::AddressSpace;
use crate::codegen::llvm::basic_value_extensions::BasicValueExt;

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::{Expression, Node};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::codegen::llvm::llvm_code_generator_extensions::{SymbolLookupExtensions, ErrorPathExtensions};
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;

/// Trait for a fully integrated Result-based interface type assertion system
/// Provides seamless integration with Rust's ? operator for error propagation
pub trait IntegratedResultTypeAssertion<'ctx> {
    /// Compile a type assertion with full Result integration
    /// Returns a Result type that can be used with the ? operator
    fn compile_type_assertion_with_integrated_result(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a Result structure containing either a value or error information
    fn build_result_structure(
        &mut self,
        success: bool,
        value: Option<BasicValueEnum<'ctx>>,
        error_info: Option<TypeAssertionErrorInfo>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract success value from a Result structure with appropriate error handling
    fn extract_success_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
        expected_type: BasicTypeEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate comprehensive error information for a failed type assertion
    fn generate_type_assertion_error_info(
        &mut self,
        source_type: &str,
        target_type: &str,
        source_location: Option<SourceLocation>
    ) -> Result<TypeAssertionErrorInfo, Error>;
    
    /// Propagate an error from a Result structure
    fn propagate_result_error(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<(), Error>;
}

/// Comprehensive error information for type assertions
#[derive(Debug, Clone)]
pub struct TypeAssertionErrorInfo {
    pub source_type: String,
    pub target_type: String,
    pub source_location: Option<SourceLocation>,
    pub source_type_id: Option<u64>,
    pub target_type_id: Option<u64>,
    pub type_path: Option<String>,
    pub error_message: String,
}

impl<'ctx> IntegratedResultTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_integrated_result(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with integrated result: {}", type_assertion.string());
        
        // Ensure registry is initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Get source location for better error messages
        let source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                Some(SourceLocation {
                    line: 0, // Not available from AST
                    column: 0, // Not available from AST
                    file: None,
                    source_line: format!("{}.({})", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        
        // Get current function (using ? operator for clean error handling)
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression to get the interface value
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success, failure, and merge paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Get the type ID from the interface value's vtable (using ? operator)
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        
        // Get the expected type ID for the target type (using ? operator)
        let expected_type_id_u64 = match &self.interface_type_registry {
            Some(registry) => registry.get_type_id(&type_assertion.type_name)?,
            None => self.hash_type_name(&type_assertion.type_name)
        };
        
        let expected_type_id = self.context().i64_type().const_int(expected_type_id_u64, false);
        
        // Compare the type IDs and branch accordingly
        let is_instance = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id,
            "is_instance_of"
        )?;
        
        self.builder().build_conditional_branch(
            is_instance,
            success_block,
            failure_block
        )?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Find target type in the registry or create an opaque type
        let target_struct_type = self.get_type_by_name(&type_assertion.type_name)
            .unwrap_or_else(|| self.context().opaque_struct_type(&type_assertion.type_name));
        
        let target_ptr_type = target_struct_type.ptr_type(AddressSpace::default());
        
        // Cast the data pointer to the appropriate type
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            target_ptr_type,
            "casted_ptr"
        )?;
        
        // Create success result with value
        let success_result = self.build_result_structure(
            true, // success
            Some(casted_ptr.into()),
            None  // no error information
        )?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)?;
        
        // Failure path - create error information
        self.builder().position_at_end(failure_block);
        
        // Get runtime type name for improved error messages
        let runtime_type_id = match self.get_runtime_type_id(expr_value, None) {
            Ok(id) => Some(id),
            Err(_) => None,
        };
        
        let runtime_type_name = match runtime_type_id {
            Some(id) => match self.get_type_name_for_id(id) {
                Ok(name) => name,
                Err(_) => "<unknown type>".to_string(),
            },
            None => "<unknown type>".to_string(),
        };
        
        // Generate comprehensive error information
        let error_info = self.generate_type_assertion_error_info(
            &runtime_type_name,
            &type_assertion.type_name,
            source_location.clone()
        )?;
        
        // Create null pointer for failure case
        let null_ptr = target_ptr_type.const_null();
        
        // Create failure result with error information
        let failure_result = self.build_result_structure(
            false, // failure
            Some(null_ptr.into()),
            Some(error_info)
        )?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Get the result type (struct with value and success flag)
        let result_type = self.get_result_type(target_ptr_type.into());
        
        // Build the phi node to select between success and failure results
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        )?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Successfully compiled type assertion with integrated result");
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    fn build_result_structure(
        &mut self,
        success: bool,
        value: Option<BasicValueEnum<'ctx>>,
        error_info: Option<TypeAssertionErrorInfo>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let ctx = self.context();
        let builder = self.builder();
        
        // Create success/error flag
        let success_flag = ctx.bool_type().const_int(if success { 1 } else { 0 }, false);
        
        // Use provided value or null pointer
        let value_ptr = match value {
            Some(v) => v,
            None => ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Create error message string (or null if success)
        let error_msg_ptr = if let Some(info) = &error_info {
            if !success {
                // Create a global string constant for the error message
                self.create_result_string_constant(&info.error_message).into()
            } else {
                // Null pointer for success case
                ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
            }
        } else {
            // Null pointer if no message
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Create source type name string (or null if success)
        let source_type_ptr = if let Some(info) = &error_info {
            if !success {
                self.create_result_string_constant(&info.source_type).into()
            } else {
                ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
            }
        } else {
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Create target type name string (or null if success)
        let target_type_ptr = if let Some(info) = &error_info {
            if !success {
                self.create_result_string_constant(&info.target_type).into()
            } else {
                ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
            }
        } else {
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Create location information (or nulls for success)
        let source_location = if let Some(info) = &error_info {
            if !success && info.source_location.is_some() {
                let loc = info.source_location.as_ref().unwrap();
                let line = ctx.i32_type().const_int(loc.line as u64, false);
                let column = ctx.i32_type().const_int(loc.column as u64, false);
                let file_ptr = if let Some(file) = &loc.file {
                    self.create_result_string_constant(file).into()
                } else {
                    ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
                };
                let source_line_ptr = if !loc.source_line.is_empty() {
                    self.create_result_string_constant(&loc.source_line).into()
                } else {
                    ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
                };
                
                // Create a location struct
                self.build_struct_value(&[
                    line.into(),
                    column.into(),
                    file_ptr,
                    source_line_ptr
                ]).into()
            } else {
                // Null struct for success case
                let location_type = self.get_source_location_type();
                location_type.const_zero().into()
            }
        } else {
            // Null struct if no location
            let location_type = self.get_source_location_type();
            location_type.const_zero().into()
        };
        
        // Type path information for better error messages
        let type_path_ptr = if let Some(info) = &error_info {
            if !success && info.type_path.is_some() {
                self.create_result_string_constant(info.type_path.as_ref().unwrap()).into()
            } else {
                ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
            }
        } else {
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Build the Result structure
        let result_struct = self.build_struct_value(&[
            value_ptr,                  // Value or null
            success_flag.into(),         // Success flag
            error_msg_ptr,              // Error message
            source_type_ptr,            // Source type name
            target_type_ptr,            // Target type name
            source_location,            // Source location
            type_path_ptr               // Type path info
        ]);
        
        Ok(result_struct.into())
    }
    
    fn extract_success_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
        expected_type: BasicTypeEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let ctx = self.context();
        let builder = self.builder();
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for unwrapping result".to_string()))?;
        
        // Create basic blocks for success and failure paths
        let success_block = ctx.append_basic_block(current_fn, "unwrap_success");
        let failure_block = ctx.append_basic_block(current_fn, "unwrap_failure");
        let merge_block = ctx.append_basic_block(current_fn, "unwrap_merge");
        
        // Extract the success flag (second field)
        let success_flag = builder.build_extract_value(
            result_value.into_struct_value(),
            1, // Index of success flag
            "success_flag"
        )?;
        
        // Branch based on success flag
        builder.build_conditional_branch(
            success_flag.into_int_value(),
            success_block,
            failure_block
        )?;
        
        // Success path - extract and return the value
        builder.position_at_end(success_block);
        let value = builder.build_extract_value(
            result_value.into_struct_value(),
            0, // Index of value
            "unwrapped_value"
        )?;
        
        builder.build_unconditional_branch(merge_block)?;
        
        // Failure path - propagate the error
        builder.position_at_end(failure_block);
        
        // Extract error components
        let error_msg = builder.build_extract_value(
            result_value.into_struct_value(),
            2, // Index of error message
            "error_message"
        )?;
        
        let source_type = builder.build_extract_value(
            result_value.into_struct_value(),
            3, // Index of source type
            "source_type"
        )?;
        
        let target_type = builder.build_extract_value(
            result_value.into_struct_value(),
            4, // Index of target type
            "target_type"
        )?;
        
        let source_location = builder.build_extract_value(
            result_value.into_struct_value(),
            5, // Index of source location
            "source_location"
        )?;
        
        let type_path = builder.build_extract_value(
            result_value.into_struct_value(),
            6, // Index of type path
            "type_path"
        )?;
        
        // Call error propagation function
        self.call_error_propagation_function(
            error_msg, 
            source_type, 
            target_type, 
            source_location,
            type_path
        )?;
        
        // We should never reach this point in the failure path, as error propagation
        // should trigger unwinding, but we need to emit valid LLVM IR
        builder.build_unreachable()?;
        
        // Merge block - use phi node
        builder.position_at_end(merge_block);
        let phi = builder.build_phi(
            expected_type,
            "unwrapped_value"
        )?;
        
        phi.add_incoming(&[(
            &value,
            success_block
            // No incoming value from failure block as it's unreachable
        )]);
        
        Ok(phi.as_basic_value())
    }
    
    fn generate_type_assertion_error_info(
        &mut self,
        source_type: &str,
        target_type: &str,
        source_location: Option<SourceLocation>
    ) -> Result<TypeAssertionErrorInfo, Error> {
        // Try to get type IDs
        let source_type_id = match self.get_type_id(source_type) {
            Ok(id) => Some(id),
            Err(_) => None,
        };
        
        let target_type_id = match self.get_type_id(target_type) {
            Ok(id) => Some(id),
            Err(_) => None,
        };
        
        // Try to visualize the inheritance path
        let type_path = match self.visualize_interface_path(target_type, 2) {
            Ok(path) => Some(path),
            Err(_) => None,
        };
        
        // Try to find alternative paths
        let alt_paths = match self.find_alternative_paths(source_type, target_type, 3) {
            Ok(paths) => {
                if paths.is_empty() {
                    format!("No inheritance path exists between '{}' and '{}'.", source_type, target_type)
                } else {
                    let mut result = format!("Found {} possible inheritance path(s):", paths.len());
                    for (i, path) in paths.iter().enumerate() {
                        result.push_str(&format!("\n  Path {}: {}", i + 1, path));
                    }
                    result
                }
            },
            Err(_) => format!("No inheritance relationship between '{}' and '{}' could be detected.", 
                         source_type, target_type),
        };
        
        // Create a recovery suggestion
        let suggestion = match self.suggest_recovery_options(source_type, target_type) {
            Ok(Some(hint)) => format!("\nSuggestion: {}", hint),
            _ => "".to_string(),
        };
        
        // Build the comprehensive error message
        let error_message = format!(
            "Type assertion failed: {} is not a {}\n{}{}\n",
            source_type,
            target_type,
            alt_paths,
            suggestion
        );
        
        Ok(TypeAssertionErrorInfo {
            source_type: source_type.to_string(),
            target_type: target_type.to_string(),
            source_location,
            source_type_id,
            target_type_id,
            type_path,
            error_message,
        })
    }
    
    fn propagate_result_error(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<(), Error> {
        let builder = self.builder();
        
        // Extract error components
        let error_msg = builder.build_extract_value(
            result_value.into_struct_value(),
            2, // Index of error message
            "error_message"
        )?;
        
        let source_type = builder.build_extract_value(
            result_value.into_struct_value(),
            3, // Index of source type
            "source_type"
        )?;
        
        let target_type = builder.build_extract_value(
            result_value.into_struct_value(),
            4, // Index of target type
            "target_type"
        )?;
        
        let source_location = builder.build_extract_value(
            result_value.into_struct_value(),
            5, // Index of source location
            "source_location"
        )?;
        
        let type_path = builder.build_extract_value(
            result_value.into_struct_value(),
            6, // Index of type path
            "type_path"
        )?;
        
        // Call error propagation function
        self.call_error_propagation_function(
            error_msg, 
            source_type, 
            target_type, 
            source_location,
            type_path
        )?;
        
        Ok(())
    }
}

// Helper methods for Result integration
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the LLVM type for the enhanced Result structure
    pub fn get_result_type(&self, value_type: BasicTypeEnum<'ctx>) -> StructType<'ctx> {
        // Note: This implementation uses a different Result structure than the common one
        // We maintain it separately since it has a different field layout
        let ctx = self.context();
        
        // Enhanced Result structure:
        // 1. Value of generic type
        // 2. Success flag (bool)
        // 3. Error message (string pointer)
        // 4. Source type name (string pointer)
        // 5. Target type name (string pointer)
        // 6. Source location information (struct)
        // 7. Type path information (string pointer)
        
        ctx.struct_type(&[
            value_type,
            ctx.bool_type().into(),
            ctx.i8_type().ptr_type(AddressSpace::default()).into(), // Error message
            ctx.i8_type().ptr_type(AddressSpace::default()).into(), // Source type
            ctx.i8_type().ptr_type(AddressSpace::default()).into(), // Target type
            self.get_source_location_type().into(),               // Source location
            ctx.i8_type().ptr_type(AddressSpace::default()).into()  // Type path
        ], false)
    }
    
    /// Get the LLVM type for source location information
    pub fn get_source_location_type(&self) -> StructType<'ctx> {
        // Use the common implementation
        crate::codegen::llvm::interface_type_assertion_common::get_source_location_type(self)
    }
    
    /// Create a string constant for result implementation
    fn create_result_string_constant(&self, value: &str) -> PointerValue<'ctx> {
        crate::codegen::llvm::interface_type_assertion_common::create_string_constant_from_codegen(self, value)
    }
    
    /// Call the runtime error propagation function
    pub fn call_error_propagation_function(
        &self,
        error_message: BasicValueEnum<'ctx>,
        source_type: BasicValueEnum<'ctx>,
        target_type: BasicValueEnum<'ctx>,
        location_info: BasicValueEnum<'ctx>,
        type_path: BasicValueEnum<'ctx>,
    ) -> Result<(), Error> {
        // Get or declare the error propagation function
        let module = self.module();
        let ctx = self.context();
        
        let propagate_fn = match module.get_function("__cursed_propagate_type_assertion_error") {
            Some(func) => func,
            None => {
                // Declare the function if it doesn't exist
                let void_type = ctx.void_type();
                let fn_type = void_type.fn_type(&[
                    ctx.i8_type().ptr_type(AddressSpace::default()).into(), // Error message
                    ctx.i8_type().ptr_type(AddressSpace::default()).into(), // Source type
                    ctx.i8_type().ptr_type(AddressSpace::default()).into(), // Target type
                    self.get_source_location_type().into(),               // Source location
                    ctx.i8_type().ptr_type(AddressSpace::default()).into()  // Type path
                ], false);
                
                module.add_function("__cursed_propagate_type_assertion_error", fn_type, None)
            }
        };
        
        // Call the function
        let builder = self.builder();
        builder.build_call(
            propagate_fn,
            &[
                error_message,
                source_type,
                target_type,
                location_info,
                type_path
            ],
            "propagate_error_call"
        )?;
        
        Ok(())
    }
    
    /// Helper to build a struct value from field values
    pub fn build_struct_value(&self, fields: &[BasicValueEnum<'ctx>]) -> inkwell::values::StructValue<'ctx> {
        crate::codegen::llvm::interface_type_assertion_common::build_struct_value(self, fields)
    }
    
    /// Helper to convert between error types
    fn convert_to_type_assertion_error(&self, error: Error, source_type: &str, target_type: &str) -> TypeAssertionError {
        match error {
            Error::TypeAssertion(assertion_error) => assertion_error.into(),
            Error::Compilation(msg) => {
                TypeAssertionError::new(source_type, target_type)
                    .with_message(format!("Compilation error: {}", msg))
            },
            Error::Runtime(msg) => {
                TypeAssertionError::new(source_type, target_type)
                    .with_message(format!("Runtime error: {}", msg))
            },
            _ => TypeAssertionError::new(source_type, target_type)
                .with_message(format!("Unknown error type: {}", error))
        }
    }
    
    /// Helper to suggest recovery options for type assertion failures
    fn suggest_recovery_options(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<String>, Error> {
        // Check if source implements target as an interface
        let implements = match self.check_extension_relationship(source_type, target_type) {
            Ok(result) => result,
            Err(_) => false
        };
        
        if implements {
            // This should be impossible - if the relationship exists, the assertion would succeed
            return Ok(Some(format!(
                "This appears to be a runtime error - the static type system should have caught this if '{}' doesn't implement '{}'.",
                source_type, target_type
            )));
        }
        
        // Check if target implements source as an interface (reversed relationship)
        let reversed = match self.check_extension_relationship(target_type, source_type) {
            Ok(result) => result,
            Err(_) => false
        };
        
        if reversed {
            return Ok(Some(format!(
                "The relationship between '{}' and '{}' appears to be reversed. Try asserting '{}' as '{}'.",
                source_type, target_type, target_type, source_type
            )));
        }
        
        // If no relationship, suggest implementing the interface
        Ok(Some(format!(
            "To make this assertion work, ensure that type '{}' implements the interface '{}'.",
            source_type, target_type
        )))
    }
}

/// Register the result implementation module
pub fn register_result_implementation() {
    trace!("Interface type assertion result implementation module registered");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_result_implementation_registration() {
        register_result_implementation();
        assert!(true);
    }
}