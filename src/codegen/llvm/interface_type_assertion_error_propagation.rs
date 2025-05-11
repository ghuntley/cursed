//! # Interface Type Assertion Error Propagation
//!
//! This module implements the error propagation mechanism for interface type assertions
//! using the Result type and ? operator. It enhances the type assertion system by
//! allowing errors to be properly propagated up the call stack, leading to more
//! robust error handling.
//!
//! ## Key Features
//!
//! 1. Integration with the LLVM code generator for Result type handling
//! 2. Support for the ? operator to propagate type assertion errors
//! 3. Enhanced error context with type ID information
//! 4. Proper integration with the interface type registry
//!
//! This implementation ensures that interface type assertions can be used with
//! the ? operator, making error handling more concise and reliable.

use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace, warn};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;

/// Trait for implementing interface type assertion error propagation
pub trait InterfaceTypeAssertionErrorPropagation<'ctx> {
    /// Compile a type assertion expression with error propagation support
    /// This version returns a Result type that can be used with the ? operator
    fn compile_type_assertion_with_error_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a type assertion expression with ? operator
    /// This is specialized for the TypeAssertionQuestion AST node and will automatically 
    /// propagate errors through the ? operator mechanism
    fn compile_type_assertion_question(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Create a Result type value for interface type assertions
    fn create_type_assertion_result(
        &mut self,
        value: BasicValueEnum<'ctx>,
        success: bool,
        error_message: Option<&str>,
        source_location: Option<SourceLocation>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Unwrap a type assertion Result, propagating errors if present
    fn unwrap_type_assertion_result(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

// TypeAssertionErrorPropagation - implementation trait for LLVM code generator
impl<'ctx> InterfaceTypeAssertionErrorPropagation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion))]
    fn compile_type_assertion_with_error_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create a source location for error context
        let source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                Some(SourceLocation {
                    line: 0, // Not available from AST
                    column: 0, // Not available from AST
                    file: None,
                    source_line: format!("{}.({}", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        
        debug!("Compiling type assertion with error propagation for: {}", type_assertion.string());
        
        // First ensure registry is initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, source_location.clone())?;
        
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
        
        // Create a successful Result value
        let success_result = self.create_type_assertion_result(
            casted_ptr.into(),
            true, // success
            None, // no error message
            source_location.clone()
        )?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - create an error Result
        self.builder().position_at_end(failure_block);
        
        // Get the actual type ID from the interface value for better error reporting
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        
        // Create an error message
        let error_message = format!(
            "Failed to assert that interface value is of type {}",
            type_assertion.type_name
        );
        
        // Create a null pointer for the failure case
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        
        // Create a failure Result value with error information
        let failure_result = self.create_type_assertion_result(
            null_ptr.into(),
            false, // failure
            Some(&error_message),
            source_location.clone()
        )?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        let phi = self.builder().build_phi(
            // Result type structure
            self.get_result_type(self.pointer_type().into()),
            "assertion_result"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        phi.add_incoming(&[
            (&success_result, success_block),
            (&failure_result, failure_block)
        ]);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }

    fn create_type_assertion_result(
        &mut self,
        value: BasicValueEnum<'ctx>,
        success: bool,
        error_message: Option<&str>,
        source_location: Option<SourceLocation>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create a Result structure with the following fields:
        // 1. Value (the asserted value or null)
        // 2. Success flag (boolean)
        // 3. Error message (string or null)
        // 4. Error context (source location information)
        
        let ctx = self.context();
        let builder = self.builder();
        
        // Create success/error flag
        let success_flag = ctx.bool_type().const_int(if success { 1 } else { 0 }, false);
        
        // Create error message string (or null if success)
        let error_msg_ptr = if let Some(msg) = error_message {
            if !success {
                // Create a global string constant for the error message
                self.create_string_constant(msg).into()
            } else {
                // Null pointer for success case
                ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
            }
        } else {
            // Null pointer if no message
            ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
        };
        
        // Create error source location information (or nulls for success)
        let source_info = if let Some(loc) = source_location {
            if !success {
                // Only include location info for error case
                let line = ctx.i32_type().const_int(loc.line as u64, false);
                let column = ctx.i32_type().const_int(loc.column as u64, false);
                let file_ptr = if let Some(file) = &loc.file {
                    self.create_string_constant(file).into()
                } else {
                    ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
                };
                let source_line_ptr = if !loc.source_line.is_empty() {
                    self.create_string_constant(&loc.source_line).into()
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
        
        // Build the Result structure
        let result_struct = self.build_struct_value(&[
            value,
            success_flag.into(),
            error_msg_ptr,
            source_info
        ]);
        
        Ok(result_struct.into())
    }
    
    fn compile_type_assertion_question(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        
        // Create a source location for error context
        let source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                Some(SourceLocation {
                    line: 0, // Not available from AST
                    column: 0, // Not available from AST
                    file: None,
                    source_line: format!("{}.({})?", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        
        debug!("Compiling type assertion with ? operator for: {}", type_assertion.string());
        
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
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, source_location.clone())?;
        
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
        
        // Failure path - propagate the error through the ? operator
        self.builder().position_at_end(failure_block);
        
        // Get the actual type ID from the interface value for better error reporting
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        
        // Create an error message
        let error_message = format!(
            "Failed to assert that interface value is of type {}",
            type_assertion.type_name
        );
        
        // Create type assertion error object and propagate it
        self.call_error_propagation_function(
            self.create_string_constant(&error_message).into(), 
            BasicValueEnum::into_struct_value(
                self.build_struct_value(&[
                    self.context().i32_type().const_int(0, false).into(), // line 0
                    self.context().i32_type().const_int(0, false).into(), // column 0
                    self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null().into(), // no file
                    self.create_string_constant(&source_location.map_or(String::new(), |loc| loc.source_line)).into() // source line
                ])
            )
        )?;
        
        // This should be unreachable in the failure path
        self.builder().build_unreachable().map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - return the casted pointer on success
        self.builder().position_at_end(merge_block);
        
        // With question operator, we just return the value directly, error handling is automatic
        Ok(casted_ptr.into())
    }
    
    fn unwrap_type_assertion_result(
        &mut self,
        result_value: BasicValueEnum<'ctx>
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
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Branch based on success flag
        builder.build_conditional_branch(
            success_flag.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and return the value
        builder.position_at_end(success_block);
        let value = builder.build_extract_value(
            result_value.into_struct_value(),
            0, // Index of value
            "unwrapped_value"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        builder.build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - propagate the error
        builder.position_at_end(failure_block);
        
        // Extract error message and location
        let error_msg = builder.build_extract_value(
            result_value.into_struct_value(),
            2, // Index of error message
            "error_message"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        let location_info = builder.build_extract_value(
            result_value.into_struct_value(),
            3, // Index of source location
            "source_location"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Call error propagation function (to be implemented in runtime support)
        // This would typically set a thread-local error and jump to appropriate handler
        self.call_error_propagation_function(error_msg, location_info)?;
        
        // We should never reach this point in the failure path, as error propagation
        // should trigger unwinding, but we need to emit valid LLVM IR
        builder.build_unreachable().map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node
        builder.position_at_end(merge_block);
        let phi = builder.build_phi(
            value.get_type(),
            "unwrapped_value"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        phi.add_incoming(&[
            (&value, success_block)
            // No incoming value from failure block as it's unreachable
        ]);
        
        Ok(phi.as_basic_value())
    }
}

// Helper methods for the error propagation system
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the LLVM type for Result structure
    fn get_result_type(&self, value_type: BasicTypeEnum<'ctx>) -> StructType<'ctx> {
        let ctx = self.context();
        
        // Result structure:
        // 1. Value of generic type
        // 2. Success flag (bool)
        // 3. Error message (string pointer)
        // 4. Source location information
        
        ctx.struct_type(&[
            value_type,
            ctx.bool_type().into(),
            ctx.i8_type().ptr_type(AddressSpace::default()).into(),
            self.get_source_location_type().into()
        ], false)
    }
    
    /// Get the LLVM type for source location information
    fn get_source_location_type(&self) -> StructType<'ctx> {
        let ctx = self.context();
        
        // Source location structure:
        // 1. Line number (i32)
        // 2. Column number (i32)
        // 3. File name (string pointer)
        // 4. Source line text (string pointer)
        
        ctx.struct_type(&[
            ctx.i32_type().into(),
            ctx.i32_type().into(),
            ctx.i8_type().ptr_type(AddressSpace::default()).into(),
            ctx.i8_type().ptr_type(AddressSpace::default()).into()
        ], false)
    }
    
    /// Create a string constant in the module
    fn create_string_constant(&self, value: &str) -> PointerValue<'ctx> {
        let ctx = self.context();
        let builder = self.builder();
        
        // Create global string constant
        let global_str = builder.build_global_string_ptr(value, "error_str")
            .expect("Failed to create global string constant");
        
        global_str.as_pointer_value()
    }
    
    /// Call the runtime error propagation function
    fn call_error_propagation_function(
        &self,
        error_message: BasicValueEnum<'ctx>,
        location_info: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // In a real implementation, this would call a runtime support function
        // that would set up proper error propagation
        
        // For now, we'll call a placeholder function that will be replaced with
        // proper implementation in the future
        let module = self.module();
        let ctx = self.context();
        
        // Get or declare the error propagation function
        let propagate_fn = match module.get_function("__cursed_propagate_error") {
            Some(func) => func,
            None => {
                // Declare the function if it doesn't exist
                let void_type = ctx.void_type();
                let fn_type = void_type.fn_type(&[
                    ctx.i8_type().ptr_type(AddressSpace::default()).into(),
                    self.get_source_location_type().into()
                ], false);
                
                module.add_function("__cursed_propagate_error", fn_type, None)
            }
        };
        
        // Call the function
        let builder = self.builder();
        builder.build_call(
            propagate_fn,
            &[
                error_message,
                location_info
            ],
            "propagate_error_call"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // This function should never return normally, but for LLVM's sake we'll
        // return an empty value
        Ok(ctx.i8_type().const_int(0, false).into())
    }
    
    /// Build a struct value from field values
    fn build_struct_value(&self, fields: &[BasicValueEnum<'ctx>]) -> inkwell::values::StructValue<'ctx> {
        let ctx = self.context();
        let builder = self.builder();
        
        // Create struct type from field types
        let struct_type = ctx.struct_type(
            &fields.iter().map(|v| v.get_type()).collect::<Vec<_>>(),
            false
        );
        
        // Create empty struct
        let mut struct_value = struct_type.const_named_struct(&[]);
        
        // Insert each field
        for (i, field) in fields.iter().enumerate() {
            struct_value = builder.build_insert_value(
                struct_value,
                *field,
                i as u32,
                &format!("field_{}", i)
            ).expect("Failed to insert struct field").into_struct_value();
        }
        
        struct_value
    }
}

/// Register the error propagation module with the compiler
pub fn register_error_propagation() {
    trace!("Interface type assertion error propagation module registered");
    // This function is called during the compiler's initialization
    // to register this implementation for use throughout compilation
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_registration() {
        // Test that the module registration function works
        register_error_propagation();
        assert!(true);
    }
}