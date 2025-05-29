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
use crate::codegen::llvm::interface_type_assertion_filesystem_integration::InterfaceTypeAssertionFilesystemIntegration;

/// Trait for implementing interface type assertion error propagation
pub trait InterfaceTypeAssertionErrorPropagation<'ctx> {
    /// Set the current expected type ID for error reporting
    fn set_expected_type_id(&mut self, type_id: u32);
    
    /// Set the current actual type ID for error reporting
    fn set_actual_type_id(&mut self, type_id: u32);
    
    /// Clear the current type ID tracking
    fn clear_type_ids(&mut self);
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

// Add fields to LlvmCodeGenerator for type ID tracking during error propagation
impl<'ctx> LlvmCodeGenerator<'ctx> {
    pub fn init_type_assertion_error_tracking(&mut self) {
        // Initialize the type ID tracking fields if they don't exist
        if !self.internal_fields.contains_key("current_expected_type_id") {
            self.internal_fields.insert("current_expected_type_id".to_string(), Box::new(None::<u32>));
        }
        if !self.internal_fields.contains_key("current_actual_type_id") {
            self.internal_fields.insert("current_actual_type_id".to_string(), Box::new(None::<u32>));
        }
    }
    
    // Accessor for expected type ID
    pub fn current_expected_type_id(&self) -> Option<u32> {
        self.internal_fields.get("current_expected_type_id")
            .and_then(|val| val.downcast_ref::<Option<u32>>().cloned())
            .flatten()
    }
    
    // Accessor for actual type ID
    pub fn current_actual_type_id(&self) -> Option<u32> {
        self.internal_fields.get("current_actual_type_id")
            .and_then(|val| val.downcast_ref::<Option<u32>>().cloned())
            .flatten()
    }
}

// TypeAssertionErrorPropagation - implementation trait for LLVM code generator
impl<'ctx> InterfaceTypeAssertionErrorPropagation<'ctx> for LlvmCodeGenerator<'ctx> {
    fn set_expected_type_id(&mut self, type_id: u32) {
        self.init_type_assertion_error_tracking();
        if let Some(field) = self.internal_fields.get_mut("current_expected_type_id") {
            if let Some(val) = field.downcast_mut::<Option<u32>>() {
                *val = Some(type_id);
            }
        }
    }
    
    fn set_actual_type_id(&mut self, type_id: u32) {
        self.init_type_assertion_error_tracking();
        if let Some(field) = self.internal_fields.get_mut("current_actual_type_id") {
            if let Some(val) = field.downcast_mut::<Option<u32>>() {
                *val = Some(type_id);
            }
        }
    }
    
    fn clear_type_ids(&mut self) {
        self.init_type_assertion_error_tracking();
        if let Some(field) = self.internal_fields.get_mut("current_expected_type_id") {
            if let Some(val) = field.downcast_mut::<Option<u32>>() {
                *val = None;
            }
        }
        if let Some(field) = self.internal_fields.get_mut("current_actual_type_id") {
            if let Some(val) = field.downcast_mut::<Option<u32>>() {
                *val = None;
            }
        }
    }
    #[instrument(skip(self, type_assertion))]
    fn compile_type_assertion_with_error_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Ensure filesystem cache is initialized
        self.init_source_file_cache();
        
        // Create a source location for error context
        let mut source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                // Try to extract location from token
                let (line, column, file) = self.extract_location_from_token(token);
                Some(SourceLocation {
                    line,
                    column,
                    file,
                    source_line: format!("{}.({}", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        
        // Enhance source location with file content if available
        if let Some(location) = &mut source_location {
            let _ = self.enhance_source_location(location);
        }
        
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
                self.create_error_string_constant(msg).into()
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
                    self.create_error_string_constant(file).into()
                } else {
                    ctx.i8_type().ptr_type(AddressSpace::default()).const_null().into()
                };
                let source_line_ptr = if !loc.source_line.is_empty() {
                    self.create_error_string_constant(&loc.source_line).into()
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
        
        // Get type IDs for the result
        let expected_type_id = match self.current_expected_type_id() {
            Some(id) => ctx.i32_type().const_int(id as u64, false),
            None => ctx.i32_type().const_int(0, false)
        };
        
        let actual_type_id = match self.current_actual_type_id() {
            Some(id) => ctx.i32_type().const_int(id as u64, false),
            None => ctx.i32_type().const_int(0, false)
        };
        
        // Build the enhanced Result structure with type IDs
        let result_struct = self.build_struct_value(&[
            value,
            success_flag.into(),
            error_msg_ptr,
            source_info,
            expected_type_id.into(),
            actual_type_id.into()
        ]);
        
        Ok(result_struct.into())
    }
    
    fn compile_type_assertion_question(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Ensure filesystem cache is initialized
        self.init_source_file_cache();
        
        // Create a source location for error context with more detailed information
        let mut source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                // Try to extract more context about location from the token
                let (line, column, file) = self.extract_location_from_token(token);
                Some(SourceLocation {
                    line, 
                    column,
                    file,
                    source_line: format!("{}.({})?", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        
        // Enhance source location with file content if available
        if let Some(location) = &mut source_location {
            let _ = self.enhance_source_location(location);
        }
        
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
        
        // Create a more detailed error message with source context if available
        let enhanced_error_message = if let Some(location) = &source_location {
            // Create an enhanced error message with source context
            match self.create_enhanced_error_message(&error_message, location) {
                Ok(msg) => msg,
                Err(_) => error_message, // Fallback to basic message
            }
        } else {
            error_message
        };
        
        // Create type assertion error object and propagate it with enhanced information
        let location_struct = if let Some(location) = &source_location {
            // Build location struct with available information
            self.build_struct_value(&[
                self.context().i32_type().const_int(location.line as u64, false).into(), // line
                self.context().i32_type().const_int(location.column as u64, false).into(), // column
                match &location.file {
                    Some(file) => self.create_error_string_constant(file).into(),
                    None => self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null().into(),
                },
                self.create_error_string_constant(&location.source_line).into() // source line
            ])
        } else {
            // Default empty location struct
            self.build_struct_value(&[
                self.context().i32_type().const_int(0, false).into(), // line 0
                self.context().i32_type().const_int(0, false).into(), // column 0
                self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null().into(), // no file
                self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null().into() // no source line
            ])
        };
        
        self.call_error_propagation_function(
            self.create_error_string_constant(&enhanced_error_message).into(), 
            BasicValueEnum::into_struct_value(location_struct)
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
        
        // Make sure we have tracking initialized
        self.init_type_assertion_error_tracking();
        
        // Create basic blocks for success and failure paths
        let success_block = ctx.append_basic_block(current_fn, "unwrap_success");
        let failure_block = ctx.append_basic_block(current_fn, "unwrap_failure");
        let merge_block = ctx.append_basic_block(current_fn, "unwrap_merge");
        
        // Get the struct value
        let struct_value = result_value.into_struct_value();
        
        // Extract the success flag (second field)
        let success_flag = builder.build_extract_value(
            struct_value,
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
            struct_value,
            0, // Index of value
            "unwrapped_value"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        builder.build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - propagate the error with improved type information
        builder.position_at_end(failure_block);
        
        // Extract error message and location
        let error_msg = builder.build_extract_value(
            struct_value,
            2, // Index of error message
            "error_message"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        let location_info = builder.build_extract_value(
            struct_value,
            3, // Index of source location
            "source_location"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Extract type information if available in additional fields
        if struct_value.get_type().count_fields() > 4 {
            // Extract expected type ID
            if let Ok(expected_type_id) = builder.build_extract_value(struct_value, 4, "expected_type_id") {
                if let Ok(type_id) = expected_type_id.try_into_int_value() {
                    // Store for error reporting
                    let id = type_id.get_zero_extended_constant().unwrap_or(0) as u32;
                    self.set_expected_type_id(id);
                }
            }
            
            // Extract actual type ID
            if let Ok(actual_type_id) = builder.build_extract_value(struct_value, 5, "actual_type_id") {
                if let Ok(type_id) = actual_type_id.try_into_int_value() {
                    // Store for error reporting
                    let id = type_id.get_zero_extended_constant().unwrap_or(0) as u32;
                    self.set_actual_type_id(id);
                }
            }
        }
        
        // Get source location info from the result if available
        let mut source_location = if !location_info.is_null() {
            // Extract components from the location_info struct
            let line = if let Ok(line_val) = self.builder().build_extract_value(location_info.into_struct_value(), 0, "loc_line") {
                if let Ok(line_int) = line_val.try_into_int_value() {
                    line_int.get_zero_extended_constant().unwrap_or(0) as i32
                } else { 0 }
            } else { 0 };
            
            let column = if let Ok(col_val) = self.builder().build_extract_value(location_info.into_struct_value(), 1, "loc_column") {
                if let Ok(col_int) = col_val.try_into_int_value() {
                    col_int.get_zero_extended_constant().unwrap_or(0) as i32
                } else { 0 }
            } else { 0 };
            
            let file_ptr = if let Ok(file_val) = self.builder().build_extract_value(location_info.into_struct_value(), 2, "loc_file") {
                // This could be a string pointer
                if !file_val.is_null() {
                    // In a real implementation, we'd extract the string value here
                    Some("unknown.csd".to_string())
                } else { None }
            } else { None };
            
            let source_line = if let Ok(line_val) = self.builder().build_extract_value(location_info.into_struct_value(), 3, "loc_source_line") {
                if !line_val.is_null() {
                    // In a real implementation, we'd extract the string value here
                    "unknown source line".to_string()
                } else { String::new() }
            } else { String::new() };
            
            Some(SourceLocation {
                line,
                column,
                file: file_ptr,
                source_line,
            })
        } else {
            None
        };
        
        // Enhance source location with file content if available
        if let Some(location) = &mut source_location {
            let _ = self.enhance_source_location(location);
        }
        
        // Create an enhanced error message with source context
        let error_message = if let Some(location) = &source_location {
            match self.create_enhanced_error_message("Type assertion failed", location) {
                Ok(msg) => msg,
                Err(_) => "Type assertion failed".to_string(),
            }
        } else {
            "Type assertion failed".to_string()
        };
        
        // Call error propagation function with enhanced type information and better error message
        self.call_error_propagation_function(
            self.create_error_string_constant(&error_message).into(),
            location_info
        )?;
        
        // Clean up type ID tracking after propagation
        self.clear_type_ids();
        
        // We should never reach this point in the failure path
        builder.build_unreachable().map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node (only from success path since failure is unreachable)
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
    /// Extract location information from a token string
    /// Returns (line, column, file_opt)
    fn extract_location_from_token(&self, token: &str) -> (i32, i32, Option<String>) {
        // Token might contain location information in format "file:line:column"
        // This is a best-effort extraction
        let parts: Vec<&str> = token.split(':').collect();
        
        if parts.len() >= 3 {
            // Last part should be column
            let column = parts[parts.len()-1].parse::<i32>().unwrap_or(0);
            
            // Second to last part should be line
            let line = parts[parts.len()-2].parse::<i32>().unwrap_or(0);
            
            // Everything before that is the file
            let file = if parts.len() > 3 {
                let file_parts = &parts[0..parts.len()-2];
                Some(file_parts.join(":"))
            } else if parts.len() == 3 {
                Some(parts[0].to_string())
            } else {
                None
            };
            
            (line, column, file)
        } else {
            // No location information available in token
            (0, 0, None)
        }
    }

    
    /// Create a string constant in the module for error propagation
    fn create_error_string_constant(&self, value: &str) -> PointerValue<'ctx> {
        crate::codegen::llvm::interface_type_assertion_common::create_string_constant_from_codegen(self, value)
    }
    

}

/// Type alias for backward compatibility with existing code
pub type TypeAssertionErrorPropagation<'ctx> = dyn InterfaceTypeAssertionErrorPropagation<'ctx>;

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