//! Interface type assertion with Result type and ? operator integration
//!
//! This module extends the type assertion capability with proper error propagation
//! using the Result type and support for the ? operator, allowing for more idiomatic
//! error handling in CURSED code.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;
use crate::codegen::llvm::basic_value_extensions::{BasicValueExt, NumericValueExt};

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::codegen::llvm::string_utils::StringUtilsExtension;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers};
use crate::error::SourceLocation;

/// Trait for interface type assertions with Result type integration
pub trait InterfaceTypeAssertionResult<'ctx> {
    /// Compile a type assertion expression, returning a Result type
    /// that can be used with the ? operator for error propagation
    fn compile_type_assertion_result(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a Result value (success case) from the given value
    fn create_success_result(
        &mut self, 
        value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a Result value (error case) from the given error value
    fn create_error_result(
        &mut self,
        error_info: TypeAssertionError
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if a value is a success Result and extract the value
    fn extract_success_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if a value is an error Result and extract the error info
    fn extract_error_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> InterfaceTypeAssertionResult<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_type_assertion_result(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create a source location if possible
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

        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Get type IDs and other information needed for potential error reporting
        let interface_name = "interface value"; // Default name
        let target_type_name = &type_assertion.type_name;
        let target_type_id = self.hash_type_name(target_type_name);
        
        // Get the type ID from the interface value's vtable
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        
        // Compare with the expected type ID
        let expected_type_id = self.context().i64_type().const_int(target_type_id, false);
        let is_match = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(self.context()),
            expected_type_id,
            "is_target_type"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_match,
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Cast the pointer to the expected type and create a success Result
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Create a success Result value
        let success_result = self.create_success_result(casted_ptr.into())?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - create an error Result
        self.builder().position_at_end(failure_block);
        
        // Create the type assertion error
        let error_info = helpers::create_detailed_assertion_error(
            interface_name,
            target_type_name,
            None, // Interface type ID not available
            Some(target_type_id),
            None, // Actual type name not known at compile time
            Some(actual_type_id.into_int_value(self.context()).get_zero_extended_constant().unwrap_or(0)),
            source_location.clone(),
        );
        
        // Create an error Result value
        let error_result = self.create_error_result(error_info)?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create a Result type which is a struct containing:
        // 1. A flag indicating success/failure
        // 2. Either the value or error info
        let result_type = self.context().struct_type(&[
            self.context().bool_type().into(), // success flag
            self.pointer_type().into(),        // value/error union
        ], false);
        
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &error_result,
            failure_block
        )]);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    fn create_success_result(
        &mut self, 
        value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create a Result struct with success flag = true and the value
        let true_val = self.context().bool_type().const_int(1, false);
        
        // Convert value to a generic pointer if it's not already
        let value_ptr = match value {
            BasicValueEnum::PointerValue(ptr) => ptr,
            _ => {
                // Allocate space and store the value
                let alloca = self.builder().build_alloca(
                    value.get_type(),
                    "value_alloca"
                ).map_err(|e| Error::Compilation(e.to_string()))?;
                
                self.builder().build_store(alloca, value)
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                
                // Cast to generic pointer
                self.builder().build_bitcast(
                    alloca,
                    self.pointer_type(),
                    "value_ptr"
                ).map_err(|e| Error::Compilation(e.to_string()))?.
                into_pointer_value()
            }
        };
        
        // Build the result struct
        let result_type = self.context().struct_type(&[
            self.context().bool_type().into(),
            self.pointer_type().into(),
        ], false);
        
        let mut result = result_type.const_named_struct(&[]);
        
        // Set the success flag (true)
        result = self.builder().build_insert_value(
            result,
            true_val,
            0,
            "result.success"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        // Set the value pointer
        result = self.builder().build_insert_value(
            result,
            value_ptr,
            1,
            "result.value"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        Ok(result.into())
    }
    
    fn create_error_result(
        &mut self,
        error_info: TypeAssertionError
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Convert the error info to a runtime representation
        // For now, we'll create a string representation of the error
        let error_message = error_info.to_detailed_string();
        let error_ptr = self.create_string_constant(&error_message)?;
        
        // Create a Result struct with success flag = false and the error info
        let false_val = self.context().bool_type().const_int(0, false);
        
        // Build the result struct
        let result_type = self.context().struct_type(&[
            self.context().bool_type().into(),
            self.pointer_type().into(),
        ], false);
        
        let mut result = result_type.const_named_struct(&[]);
        
        // Set the success flag (false)
        result = self.builder().build_insert_value(
            result,
            false_val,
            0,
            "result.success"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        // Set the error pointer
        result = self.builder().build_insert_value(
            result,
            error_ptr,
            1,
            "result.error"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        Ok(result.into())
    }
    
    fn extract_success_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract the value pointer from a success Result
        if !result_value.is_struct_value() {
            return Err(Error::Compilation(format!(
                "Expected Result struct, got {:?}",
                result_value
            )));
        }
        
        // Extract the value pointer (second field)
        let value_ptr = self.builder().build_extract_value(
            result_value.into_struct_value(),
            1, // Index of value pointer
            "result.value"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Return the pointer value
        Ok(value_ptr)
    }
    
    fn extract_error_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract the error info from an error Result
        if !result_value.is_struct_value() {
            return Err(Error::Compilation(format!(
                "Expected Result struct, got {:?}",
                result_value
            )));
        }
        
        // Extract the error pointer (second field)
        let error_ptr = self.builder().build_extract_value(
            result_value.into_struct_value(),
            1, // Index of error pointer
            "result.error"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Return the pointer value
        Ok(error_ptr)
    }
}



/// Helper trait for Result propagation with ? operator
pub trait ResultPropagation<'ctx> {
    /// Check if a Result contains an error and propagate it if using ? operator
    fn check_and_propagate_error(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
        current_function: FunctionValue<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Set up a function for Result propagation with ? operator
    fn setup_result_propagation(
        &mut self,
        function: FunctionValue<'ctx>
    ) -> Result<(), Error>;
}

impl<'ctx> ResultPropagation<'ctx> for LlvmCodeGenerator<'ctx> {
    fn check_and_propagate_error(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
        current_function: FunctionValue<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get or create the blocks for error checking and propagation
        let current_block = self.builder().get_insert_block().unwrap();
        let function = current_block.get_parent().unwrap();
        
        let success_block = self.context().append_basic_block(function, "propagate_success");
        let error_block = self.context().append_basic_block(function, "propagate_error");
        let return_block = self.context().append_basic_block(function, "propagate_return");
        
        // Extract the success flag from the Result
        let success_flag = self.builder().build_extract_value(
            result_value.into_struct_value(),
            0, // Index of success flag
            "result.success"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Branch based on the success flag
        self.builder().build_conditional_branch(
            success_flag.into_int_value(),
            success_block,
            error_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract the value
        self.builder().position_at_end(success_block);
        let value = self.extract_success_value(result_value)?;
        self.builder().build_unconditional_branch(return_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Error path - propagate the error by returning an error Result
        self.builder().position_at_end(error_block);
        let error_value = self.extract_error_value(result_value)?;
        
        // Create a new error Result with the extracted error
        let false_val = self.context().bool_type().const_int(0, false);
        let result_type = self.context().struct_type(&[
            self.context().bool_type().into(),
            self.pointer_type().into(),
        ], false);
        
        let mut error_result = result_type.const_named_struct(&[]);
        error_result = self.builder().build_insert_value(
            error_result,
            false_val,
            0,
            "error_result.success"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        error_result = self.builder().build_insert_value(
            error_result,
            error_value,
            1,
            "error_result.error"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        // Return the error Result
        self.builder().build_return(Some(&error_result))
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Continue with normal execution at the return block
        self.builder().position_at_end(return_block);
        
        Ok(value)
    }
    
    fn setup_result_propagation(
        &mut self,
        function: FunctionValue<'ctx>
    ) -> Result<(), Error> {
        // This would set up any necessary infrastructure for result propagation
        // For now, we just ensure the function's return type is compatible with Result
        let return_type = function.get_type().get_return_type();
        
        // Very basic check - just ensure it returns something
        if return_type.is_none() {
            return Err(Error::Compilation(
                "Function with ? operator must return a Result type".to_string()
            ));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::TypeAssertion;
    use crate::ast::traits::Expression;
    use crate::ast::expressions::Identifier;
    use inkwell::context::Context;
    
    // Mock expression for testing
    struct MockExpression {
        pub token: String,
        pub value: String,
    }
    
    impl Expression for MockExpression {
        fn expression_node(&self) {}
        fn as_any(&self) -> &dyn std::any::Any { self }
        fn clone_box(&self) -> Box<dyn Expression> {
            Box::new(MockExpression {
                token: self.token.clone(),
                value: self.value.clone(),
            })
        }
        fn node_type(&self) -> &str { "MockExpression" }
    }
    
    impl crate::ast::traits::Node for MockExpression {
        fn token_literal(&self) -> String { self.token.clone() }
        fn string(&self) -> String { self.value.clone() }
    }
    
    // Helper to create a type assertion
    fn create_test_assertion(expr_value: &str, type_name: &str) -> TypeAssertion {
        let mock_expr = Box::new(MockExpression {
            token: "IDENT".to_string(),
            value: expr_value.to_string(),
        });
        
        TypeAssertion {
            token: ".".to_string(),
            expression: mock_expr,
            type_name: type_name.to_string(),
        }
    }
    
    #[test]
    fn test_result_struct_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        // Create a minimal LlvmCodeGenerator
        // Note: This test is incomplete and would need a more complete setup
        // of the LlvmCodeGenerator to actually run properly.
        // It serves as a template for how tests would be structured.
    }
}