//! Error handling compilation for LLVM code generation
//!
//! This module implements compilation of error handling constructs in the CURSED language,
//! including error propagation with the `?` operator, error creation, and error checking.

use inkwell::values::{BasicValueEnum, IntValue, PointerValue, StructValue};
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::{FloatPredicate, IntPredicate};
use crate::ast::expressions::ErrorPropagation;
use crate::error::Error;
use crate::core::error_interface::{new_error_object, is_error_type, error_message};
use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use inkwell::values::BasicValue;

/// Trait for error handling compilation
pub trait ErrorHandlingCompilation<'ctx> {
    /// Compile error propagation expression (expr?)
    fn compile_error_propagation(&mut self, error_prop: &ErrorPropagation) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if a value represents an error
    fn compile_error_check(&mut self, value: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error>;
    
    /// Create an error value
    fn compile_error_creation(&mut self, message: &str) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract error message from an error value
    fn compile_error_message(&mut self, error_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a Result-like tuple (value, error)
    fn compile_result_tuple(&mut self, value: Option<BasicValueEnum<'ctx>>, error: Option<BasicValueEnum<'ctx>>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract value from a Result-like tuple
    fn compile_result_value(&mut self, result: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract error from a Result-like tuple  
    fn compile_result_error(&mut self, result: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> ErrorHandlingCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, error_prop), level = "debug")]
    fn compile_error_propagation(&mut self, error_prop: &ErrorPropagation) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Compiling error propagation for expression: {}", error_prop.get_expression().string());
        
        // Compile the inner expression
        let inner_value = self.compile_expression(error_prop.get_expression())?;
        
        // Check if the value is a Result-like tuple (value, error)
        // CURSED functions that can fail return (T, error) tuples
        if let BasicValueEnum::StructValue(struct_val) = inner_value {
            // Extract the error part (second element of the tuple)
            let error_ptr = self.builder.build_extract_value(struct_val, 1, "error_part")
                .map_err(|e| Error::CodeGenError(format!("Failed to extract error from result: {}", e)))?;
            
            // Check if the error is nil (null)
            let is_error = self.compile_error_check(error_ptr)?;
            
            // Get current function for control flow
            let function = self.builder.get_insert_block()
                .and_then(|block| block.get_parent())
                .ok_or_else(|| Error::CodeGenError("No current function for error propagation".to_string()))?;
            
            // Create blocks for error and success cases
            let error_block = self.context.append_basic_block(function, "error_propagate");
            let success_block = self.context.append_basic_block(function, "success_continue");
            
            // Branch based on error check
            self.builder.build_conditional_branch(is_error, error_block, success_block)
                .map_err(|e| Error::CodeGenError(format!("Failed to build error propagation branch: {}", e)))?;
            
            // In error block: return the error
            self.builder.position_at_end(error_block);
            
            // Create a return tuple with nil value and the error
            let nil_value = self.compile_nil_value()?;
            let return_tuple = self.compile_result_tuple(Some(nil_value), Some(error_ptr))?;
            
            self.builder.build_return(Some(&return_tuple))
                .map_err(|e| Error::CodeGenError(format!("Failed to build error return: {}", e)))?;
            
            // In success block: extract and return the value
            self.builder.position_at_end(success_block);
            
            let success_value = self.builder.build_extract_value(struct_val, 0, "success_value")
                .map_err(|e| Error::CodeGenError(format!("Failed to extract success value: {}", e)))?;
            
            Ok(success_value)
        } else {
            // If it's not a tuple, check if it's directly an error value
            let is_error = self.compile_error_check(inner_value)?;
            
            // Get current function for control flow
            let function = self.builder.get_insert_block()
                .and_then(|block| block.get_parent())
                .ok_or_else(|| Error::CodeGenError("No current function for error propagation".to_string()))?;
            
            // Create blocks for error and success cases
            let error_block = self.context.append_basic_block(function, "error_propagate");
            let success_block = self.context.append_basic_block(function, "success_continue");
            
            // Branch based on error check
            self.builder.build_conditional_branch(is_error, error_block, success_block)
                .map_err(|e| Error::CodeGenError(format!("Failed to build error propagation branch: {}", e)))?;
            
            // In error block: return the error
            self.builder.position_at_end(error_block);
            self.builder.build_return(Some(&inner_value))
                .map_err(|e| Error::CodeGenError(format!("Failed to build error return: {}", e)))?;
            
            // In success block: use the value as-is
            self.builder.position_at_end(success_block);
            
            Ok(inner_value)
        }
    }
    
    #[tracing::instrument(skip(self, value), level = "debug")]
    fn compile_error_check(&mut self, value: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
        tracing::debug!("Compiling error check");
        
        // For now, implement a simple null check
        // In a full implementation, this would check the error interface
        match value {
            BasicValueEnum::PointerValue(ptr) => {
                // Check if pointer is null
                let null_ptr = ptr.get_type().const_null();
                let is_not_null = self.builder.build_int_compare(
                    IntPredicate::NE,
                    ptr,
                    null_ptr,
                    "is_error"
                ).map_err(|e| Error::CodeGenError(format!("Failed to build error check: {}", e)))?;
                
                Ok(is_not_null)
            }
            _ => {
                // For non-pointer values, assume they're not errors
                Ok(self.context.bool_type().const_zero())
            }
        }
    }
    
    #[tracing::instrument(skip(self, message), level = "debug")]
    fn compile_error_creation(&mut self, message: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Creating error with message: {}", message);
        
        // Create a string constant for the error message
        let message_str = self.builder.build_global_string_ptr(message, "error_msg")
            .map_err(|e| Error::CodeGenError(format!("Failed to create error message string: {}", e)))?;
        
        // For now, return the string pointer as the error
        // In a full implementation, this would create an Error struct
        Ok(BasicValue::as_basic_value_enum(&message_str))
    }
    
    #[tracing::instrument(skip(self, error_value), level = "debug")]
    fn compile_error_message(&mut self, error_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Extracting error message");
        
        // For now, just return the error value as-is (assuming it's a string)
        // In a full implementation, this would call the Error() method
        Ok(error_value)
    }
    
    #[tracing::instrument(skip(self, value, error), level = "debug")]
    fn compile_result_tuple(&mut self, value: Option<BasicValueEnum<'ctx>>, error: Option<BasicValueEnum<'ctx>>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Creating result tuple");
        
        // Get or create appropriate types
        let value_part = value.unwrap_or_else(|| self.compile_nil_value().unwrap());
        let error_part = error.unwrap_or_else(|| {
            self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).const_null().into()
        });
        
        // Create a struct type for the result tuple
        let result_type = self.context.struct_type(&[
            value_part.get_type(),
            error_part.get_type()
        ], false);
        
        // Build the struct value
        let mut result_struct = result_type.get_undef();
        result_struct = self.builder.build_insert_value(result_struct, value_part, 0, "result_value")
            .map_err(|e| Error::CodeGenError(format!("Failed to insert value in result tuple: {}", e)))?
            .into_struct_value();
        result_struct = self.builder.build_insert_value(result_struct, error_part, 1, "result_error")
            .map_err(|e| Error::CodeGenError(format!("Failed to insert error in result tuple: {}", e)))?
            .into_struct_value();
        
        Ok(result_struct.into())
    }
    
    #[tracing::instrument(skip(self, result), level = "debug")]
    fn compile_result_value(&mut self, result: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Extracting value from result tuple");
        
        if let BasicValueEnum::StructValue(struct_val) = result {
            let value = self.builder.build_extract_value(struct_val, 0, "result_value")
                .map_err(|e| Error::CodeGenError(format!("Failed to extract value from result: {}", e)))?;
            Ok(value)
        } else {
            Err(Error::CodeGenError("Expected struct value for result tuple".to_string()))
        }
    }
    
    #[tracing::instrument(skip(self, result), level = "debug")]
    fn compile_result_error(&mut self, result: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Extracting error from result tuple");
        
        if let BasicValueEnum::StructValue(struct_val) = result {
            let error = self.builder.build_extract_value(struct_val, 1, "result_error")
                .map_err(|e| Error::CodeGenError(format!("Failed to extract error from result: {}", e)))?;
            Ok(error)
        } else {
            Err(Error::CodeGenError("Expected struct value for result tuple".to_string()))
        }
    }
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Helper method to create a nil/null value
    pub fn compile_nil_value(&mut self) -> Result<BasicValueEnum<'ctx>, Error> {
        // Return a null pointer as a placeholder for nil
        let ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        Ok(ptr_type.const_null().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_error_handling_compilation() {
        // This would need a full LLVM context setup to test properly
        // For now, just ensure the module compiles
        assert!(true);
    }
}
