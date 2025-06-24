// LLVM code generation for error propagation expressions
//
// This module provides LLVM code generation support for the enhanced error propagation
// system, including question mark expressions, type assertions, and error recovery.

use crate::ast::traits::Expression;

use crate::error::{CursedError, Error};
use crate::parser::error_propagation::{
    EnhancedQuestionMarkExpression, TypedErrorPropagation, UnwrapOrExpression, 
    TryExpression, FieldAccessExpression, MethodCallExpression
};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum, PointerType, StructType};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, IntValue, PointerValue, StructValue};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate};
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};

/// LLVM code generator for error propagation expressions
pub trait ErrorPropagationCodegen<'ctx> {
    /// Compile enhanced question mark expression
    fn compile_enhanced_question_mark(
        &mut self,
        expr: &EnhancedQuestionMarkExpression,
    ) -> Result<(), Error>;

    /// Compile typed error propagation expression
    fn compile_typed_error_propagation(
        &mut self,
        expr: &TypedErrorPropagation,
    ) -> Result<(), Error>;

    /// Compile unwrap-or expression
    fn compile_unwrap_or_expression(
        &mut self,
        expr: &UnwrapOrExpression,
    ) -> Result<(), Error>;

    /// Compile try expression
    fn compile_try_expression(
        &mut self,
        expr: &TryExpression,
    ) -> Result<(), Error>;

    /// Compile field access expression
    fn compile_field_access_expression(
        &mut self,
        expr: &FieldAccessExpression,
    ) -> Result<(), Error>;

    /// Compile method call expression
    fn compile_method_call_expression(
        &mut self,
        expr: &MethodCallExpression,
    ) -> Result<(), Error>;

    /// Generate error handling code
    fn generate_error_handling(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
        error_handler: Option<BasicValueEnum<'ctx>>,
    ) -> Result<(), Error>;

    /// Check if value represents an error
    fn is_error_value(
        &mut self,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), Error>;

    /// Extract success value from Result/Option
    fn extract_success_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
    ) -> Result<(), Error>;

    /// Extract error value from Result
    fn extract_error_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
    ) -> Result<(), Error>;

    /// Generate early return for error propagation
    fn generate_early_return(
        &mut self,
        error_value: BasicValueEnum<'ctx>,
    ) -> Result<(), Error>;
}

/// Implementation of error propagation codegen
pub struct ErrorPropagationCompiler<'ctx> {
    context: &'ctx Context,
    module: &'ctx Module<'ctx>,
    builder: &'ctx Builder<'ctx>,
    function_stack: Vec<FunctionValue<'ctx>>,
}

impl<'ctx> ErrorPropagationCompiler<'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'ctx Module<'ctx>,
        builder: &'ctx Builder<'ctx>,
    ) -> Self {
        Self {
            context,
            module,
            builder,
            function_stack: Vec::new(),
        }
    }

    /// Compile an expression (placeholder for integration with expression compiler)
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // This is a placeholder that would integrate with the main expression compiler
        // For now, return a simple value to enable testing
        debug!("Compiling expression (placeholder)");
        
        // Create a mock Result<i32, i32> for testing
        let success_type = self.context.i32_type().into();
        let error_type = self.context.i32_type().into();
        let success_value = self.context.i32_type().const_int(42, false);
        
        // Create a successful result
        let result = self.create_result_ok(success_value.into(), error_type)?;
        Ok(result.into())
    }

    /// Enter a function context
    pub fn enter_function(&mut self, function: FunctionValue<'ctx>) {
        self.function_stack.push(function);
    }

    /// Exit a function context
    pub fn exit_function(&mut self) {
        self.function_stack.pop();
    }

    /// Get current function
    fn current_function(&self) -> Option<FunctionValue<'ctx>> {
        self.function_stack.last().cloned()
    }

    /// Get Result<T, E> type
    fn get_result_type(&self, success_type: BasicTypeEnum<'ctx>, error_type: BasicTypeEnum<'ctx>) -> StructType<'ctx> {
        // Result is represented as { is_ok: i1, value: union { success: T, error: E } }
        let is_ok_type = self.context.bool_type();
        let value_union_type = self.context.struct_type(&[success_type, error_type], false);
        self.context.struct_type(&[is_ok_type.into(), value_union_type.into()], false)
    }

    /// Get Option<T> type
    fn get_option_type(&self, inner_type: BasicTypeEnum<'ctx>) -> StructType<'ctx> {
        // Option is represented as { is_some: i1, value: T }
        let is_some_type = self.context.bool_type();
        self.context.struct_type(&[is_some_type.into(), inner_type], false)
    }

    /// Create Result::Ok value
    fn create_result_ok(&self, value: BasicValueEnum<'ctx>, error_type: BasicTypeEnum<'ctx>) -> Result<(), Error> {
        let result_type = self.get_result_type(value.get_type(), error_type);
        let result_value = result_type.get_undef();
        
        // Set is_ok = true
        let is_ok_true = self.context.bool_type().const_int(1, false);
        let result_with_flag = self.builder.build_insert_value(
            result_value,
            is_ok_true,
            0,
            "result_ok_flag"
        ).map_err(|e| CursedError::Codegen(format!("Failed to set result ok flag: {}", e)))?;

        // Set value in union (index 0 for success)
        let union_value = result_type.get_field_type_at_index(1).unwrap().into_struct_type().get_undef();
        let union_with_value = self.builder.build_insert_value(
            union_value,
            value,
            0,
            "union_success"
        ).map_err(|e| CursedError::Codegen(format!("Failed to set union success value: {}", e)))?;

        let final_result = self.builder.build_insert_value(
            result_with_flag.into_struct_value(),
            union_with_value,
            1,
            "result_ok"
        ).map_err(|e| CursedError::Codegen(format!("Failed to create result ok: {}", e)))?;

        Ok(final_result.into_struct_value())
    }

    /// Create Result::Err value
    fn create_result_err(&self, error: BasicValueEnum<'ctx>, success_type: BasicTypeEnum<'ctx>) -> Result<(), Error> {
        let result_type = self.get_result_type(success_type, error.get_type());
        let result_value = result_type.get_undef();
        
        // Set is_ok = false
        let is_ok_false = self.context.bool_type().const_int(0, false);
        let result_with_flag = self.builder.build_insert_value(
            result_value,
            is_ok_false,
            0,
            "result_err_flag"
        ).map_err(|e| CursedError::Codegen(format!("Failed to set result err flag: {}", e)))?;

        // Set error in union (index 1 for error)
        let union_value = result_type.get_field_type_at_index(1).unwrap().into_struct_type().get_undef();
        let union_with_error = self.builder.build_insert_value(
            union_value,
            error,
            1,
            "union_error"
        ).map_err(|e| CursedError::Codegen(format!("Failed to set union error value: {}", e)))?;

        let final_result = self.builder.build_insert_value(
            result_with_flag.into_struct_value(),
            union_with_error,
            1,
            "result_err"
        ).map_err(|e| CursedError::Codegen(format!("Failed to create result err: {}", e)))?;

        Ok(final_result.into_struct_value())
    }

    /// Create Option::Some value
    fn create_option_some(&self, value: BasicValueEnum<'ctx>) -> Result<(), Error> {
        let option_type = self.get_option_type(value.get_type());
        let option_value = option_type.get_undef();
        
        // Set is_some = true
        let is_some_true = self.context.bool_type().const_int(1, false);
        let option_with_flag = self.builder.build_insert_value(
            option_value,
            is_some_true,
            0,
            "option_some_flag"
        ).map_err(|e| CursedError::Codegen(format!("Failed to set option some flag: {}", e)))?;

        // Set value
        let final_option = self.builder.build_insert_value(
            option_with_flag.into_struct_value(),
            value,
            1,
            "option_some"
        ).map_err(|e| CursedError::Codegen(format!("Failed to create option some: {}", e)))?;

        Ok(final_option.into_struct_value())
    }

    /// Create Option::None value
    fn create_option_none(&self, inner_type: BasicTypeEnum<'ctx>) -> Result<(), Error> {
        let option_type = self.get_option_type(inner_type);
        let option_value = option_type.get_undef();
        
        // Set is_some = false
        let is_some_false = self.context.bool_type().const_int(0, false);
        let final_option = self.builder.build_insert_value(
            option_value,
            is_some_false,
            0,
            "option_none"
        ).map_err(|e| CursedError::Codegen(format!("Failed to create option none: {}", e)))?;

        Ok(final_option.into_struct_value())
    }
}

impl<'ctx> ErrorPropagationCodegen<'ctx> for ErrorPropagationCompiler<'ctx> {
    #[instrument(skip(self, expr))]
    fn compile_enhanced_question_mark(
        &mut self,
        expr: &EnhancedQuestionMarkExpression,
    ) -> Result<(), Error> {
        debug!("Compiling enhanced question mark expression");

        // Compile the inner expression that should return Result<T, E>
        let inner_result = self.compile_expression(&expr.expression)?;
        
        // Get current function for early return
        let current_fn = self.current_function()
            .ok_or_else(|| CursedError::Codegen("Question mark operator used outside function".to_string()))?;

        // Check if the result represents an error
        let is_error = self.is_error_value(inner_result)?;

        // Create basic blocks for error and success paths
        let error_block = self.context.append_basic_block(current_fn, "error_path");
        let success_block = self.context.append_basic_block(current_fn, "success_path");
        let continue_block = self.context.append_basic_block(current_fn, "continue");

        // Branch based on error condition
        self.builder.build_conditional_branch(is_error, error_block, success_block)
            .map_err(|e| CursedError::Codegen(format!("Failed to build conditional branch: {}", e)))?;

        // Handle error path
        self.builder.position_at_end(error_block);
        let error_value = self.extract_error_value(inner_result)?;
        self.generate_early_return(error_value)?;

        // Handle success path
        self.builder.position_at_end(success_block);
        let success_value = self.extract_success_value(inner_result)?;
        self.builder.build_unconditional_branch(continue_block)
            .map_err(|e| CursedError::Codegen(format!("Failed to build branch to continue: {}", e)))?;

        // Continue with success value
        self.builder.position_at_end(continue_block);
        
        Ok(success_value)
    }

    #[instrument(skip(self, expr))]
    fn compile_typed_error_propagation(
        &mut self,
        expr: &TypedErrorPropagation,
    ) -> Result<(), Error> {
        debug!("Compiling typed error propagation expression");

        // Compile the inner expression
        let inner_result = self.compile_expression(&expr.expression)?;
        
        // Apply type checking for the expected error type
        debug!("Expected error type: {}", expr.expected_error_type);
        
        // Generate error handling with type validation
        self.generate_error_handling(inner_result, None)
    }

    #[instrument(skip(self, expr))]
    fn compile_unwrap_or_expression(
        &mut self,
        expr: &UnwrapOrExpression,
    ) -> Result<(), Error> {
        debug!("Compiling unwrap-or expression: {}", expr.method_name);

        // Compile the base expression (Result or Option)
        let base_result = self.compile_expression(&expr.base)?;
        
        // Compile the default value expression
        let default_value = self.compile_expression(&expr.default_value)?;
        
        // Check if the base result is an error/None
        let is_error = self.is_error_value(base_result)?;
        
        let current_fn = self.current_function()
            .ok_or_else(|| CursedError::Codegen("unwrap_or used outside function".to_string()))?;

        // Create basic blocks
        let error_block = self.context.append_basic_block(current_fn, "unwrap_or_error");
        let success_block = self.context.append_basic_block(current_fn, "unwrap_or_success");
        let continue_block = self.context.append_basic_block(current_fn, "unwrap_or_continue");

        // Branch based on error condition
        self.builder.build_conditional_branch(is_error, error_block, success_block)
            .map_err(|e| CursedError::Codegen(format!("Failed to build conditional branch: {}", e)))?;

        // Error path: use default value
        self.builder.position_at_end(error_block);
        self.builder.build_unconditional_branch(continue_block)
            .map_err(|e| CursedError::Codegen(format!("Failed to build branch: {}", e)))?;

        // Success path: extract success value
        self.builder.position_at_end(success_block);
        let success_value = self.extract_success_value(base_result)?;
        self.builder.build_unconditional_branch(continue_block)
            .map_err(|e| CursedError::Codegen(format!("Failed to build branch: {}", e)))?;

        // Continue with the appropriate value
        self.builder.position_at_end(continue_block);
        
        // Use phi node to select between default and success values
        let phi = self.builder.build_phi(success_value.get_type(), "unwrap_or_result")
            .map_err(|e| CursedError::Codegen(format!("Failed to build phi node: {}", e)))?;
        
        phi.add_incoming(&[(
            &default_value,
            error_block
        ), (
            &success_value,
            success_block
        )]);
        
        Ok(phi.as_basic_value())
    }

    #[instrument(skip(self, expr))]
    fn compile_try_expression(
        &mut self,
        expr: &TryExpression,
    ) -> Result<(), Error> {
        debug!("Compiling try expression");

        // Compile the try block
        let try_result = self.compile_expression(&expr.try_block)?;
        
        // If there's a catch block, handle errors
        if let Some(catch_block) = &expr.catch_block {
            debug!("Compiling catch block");
            let catch_handler = self.compile_expression(catch_block)?;
            
            // Generate error handling with the catch block as handler
            self.generate_error_handling(try_result, Some(catch_handler))
        } else {
            // No catch block, just propagate errors
            self.generate_error_handling(try_result, None)
        }
    }

    #[instrument(skip(self, expr))]
    fn compile_field_access_expression(
        &mut self,
        expr: &FieldAccessExpression,
    ) -> Result<(), Error> {
        debug!("Compiling field access expression: {}", expr.field_name);

        // Compile the base expression
        let base_value = self.compile_expression(&expr.base)?;
        
        // Field access on Result/Option types could be error-aware
        // For now, implement basic field access
        
        if let BasicValueEnum::StructValue(struct_val) = base_value {
            // Try to extract field by name (would need field mapping in real implementation)
            // For now, assume field_name maps to an index
            let field_index = match expr.field_name.as_str() {
                "value" => 1,
                "is_ok" | "is_some" => 0,
                _ => 0, // Default to first field
            };
            
            let field_value = self.builder.build_extract_value(
                struct_val,
                field_index,
                &format!("field_{}", expr.field_name)
            ).map_err(|e| CursedError::Codegen(format!("Failed to extract field: {}", e)))?;
            
            Ok(field_value)
        } else {
            Err(CursedError::Codegen(format!("Cannot access field {} on non-struct type", expr.field_name)))
        }
    }

    #[instrument(skip(self, expr))]
    fn compile_method_call_expression(
        &mut self,
        expr: &MethodCallExpression,
    ) -> Result<(), Error> {
        debug!("Compiling method call expression: {}", expr.method_name);

        // Compile the receiver expression
        let receiver = self.compile_expression(&expr.receiver)?;
        
        // Compile method arguments
        let mut args = Vec::new();
        for arg_expr in &expr.arguments {
            let arg_value = self.compile_expression(arg_expr)?;
            args.push(arg_value);
        }
        
        // Handle special error-related methods
        match expr.method_name.as_str() {
            "unwrap" => {
                // Unwrap should panic on error, extract value on success
                let is_error = self.is_error_value(receiver)?;
                
                let current_fn = self.current_function()
                    .ok_or_else(|| CursedError::Codegen("unwrap used outside function".to_string()))?;

                let panic_block = self.context.append_basic_block(current_fn, "unwrap_panic");
                let success_block = self.context.append_basic_block(current_fn, "unwrap_success");

                self.builder.build_conditional_branch(is_error, panic_block, success_block)
                    .map_err(|e| CursedError::Codegen(format!("Failed to build conditional branch: {}", e)))?;

                // Panic block - call panic function or terminate
                self.builder.position_at_end(panic_block);
                self.builder.build_unreachable()
                    .map_err(|e| CursedError::Codegen(format!("Failed to build unreachable: {}", e)))?;

                // Success block - extract value
                self.builder.position_at_end(success_block);
                self.extract_success_value(receiver)
            },
            "is_ok" | "is_some" => {
                // Return boolean indicating success
                let is_error = self.is_error_value(receiver)?;
                let is_ok = self.builder.build_not(is_error, "is_ok")
                    .map_err(|e| CursedError::Codegen(format!("Failed to build not: {}", e)))?;
                Ok(is_ok.into())
            },
            "is_err" | "is_none" => {
                // Return boolean indicating error
                let is_error = self.is_error_value(receiver)?;
                Ok(is_error.into())
            },
            _ => {
                // Generic method call - would integrate with method resolution
                debug!("Generic method call: {} with {} args", expr.method_name, args.len());
                
                // For now, return a placeholder result
                let result_type = self.context.i32_type().into();
                let success_value = self.context.i32_type().const_int(0, false);
                let result = self.create_result_ok(success_value.into(), result_type)?;
                Ok(result.into())
            }
        }
    }

    fn generate_error_handling(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
        error_handler: Option<BasicValueEnum<'ctx>>,
    ) -> Result<(), Error> {
        debug!("Generating error handling code");

        let current_fn = self.current_function()
            .ok_or_else(|| CursedError::Codegen("No current function for error handling".to_string()))?;

        // Check if the result is an error
        let is_error = self.is_error_value(result_value)?;

        // Create basic blocks
        let error_block = self.context.append_basic_block(current_fn, "handle_error");
        let success_block = self.context.append_basic_block(current_fn, "handle_success");
        let continue_block = self.context.append_basic_block(current_fn, "continue_after_error_handling");

        // Branch based on error condition
        self.builder.build_conditional_branch(is_error, error_block, success_block)
            .map_err(|e| CursedError::Codegen(format!("Failed to build conditional branch: {}", e)))?;

        // Handle error case
        self.builder.position_at_end(error_block);
        let handled_error_value = if let Some(handler) = error_handler {
            // If there's a custom error handler, use it
            handler
        } else {
            // Default error handling - extract and propagate the error
            let error_value = self.extract_error_value(result_value)?;
            self.generate_early_return(error_value)?;
            // This block will terminate with return, so this value won't be used
            error_value
        };
        
        // Only build branch if we didn't return early
        if error_handler.is_some() {
            self.builder.build_unconditional_branch(continue_block)
                .map_err(|e| CursedError::Codegen(format!("Failed to build branch: {}", e)))?;
        }

        // Handle success case
        self.builder.position_at_end(success_block);
        let success_value = self.extract_success_value(result_value)?;
        self.builder.build_unconditional_branch(continue_block)
            .map_err(|e| CursedError::Codegen(format!("Failed to build branch: {}", e)))?;

        // Continue block
        self.builder.position_at_end(continue_block);
        
        // Use phi node to merge values from different paths
        if error_handler.is_some() {
            let phi = self.builder.build_phi(success_value.get_type(), "error_handled_result")
                .map_err(|e| CursedError::Codegen(format!("Failed to build phi node: {}", e)))?;
            
            phi.add_incoming(&[(
                &success_value,
                success_block
            ), (
                &handled_error_value,
                error_block
            )]);
            
            Ok(phi.as_basic_value())
        } else {
            // If no error handler, only success path continues
            Ok(success_value)
        }
    }

    fn is_error_value(
        &mut self,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), Error> {
        debug!("Checking if value is error");

        // Extract the is_ok field from the Result struct
        if let BasicValueEnum::StructValue(struct_val) = value {
            // Result is { is_ok: bool, value: union }
            // Extract is_ok field (index 0)
            let is_ok = self.builder.build_extract_value(
                struct_val,
                0,
                "is_ok"
            ).map_err(|e| CursedError::Codegen(format!("Failed to extract is_ok field: {}", e)))?;

            // Return !is_ok (true if error, false if success)
            if let BasicValueEnum::IntValue(is_ok_int) = is_ok {
                let is_error = self.builder.build_not(is_ok_int, "is_error")
                    .map_err(|e| CursedError::Codegen(format!("Failed to build not operation: {}", e)))?;
                Ok(is_error)
            } else {
                Err(CursedError::Codegen("is_ok field is not an integer".to_string()))
            }
        } else {
            // For non-struct values, assume they are success values
            let is_error = self.context.bool_type().const_int(0, false);
            Ok(is_error)
        }
    }

    fn extract_success_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
    ) -> Result<(), Error> {
        debug!("Extracting success value from result");

        if let BasicValueEnum::StructValue(struct_val) = result_value {
            // Result is { is_ok: bool, value: union { success: T, error: E } }
            // Extract the union field (index 1)
            let union_value = self.builder.build_extract_value(
                struct_val,
                1,
                "result_union"
            ).map_err(|e| CursedError::Codegen(format!("Failed to extract union field: {}", e)))?;

            // Extract the success value from the union (index 0)
            if let BasicValueEnum::StructValue(union_struct) = union_value {
                let success_value = self.builder.build_extract_value(
                    union_struct,
                    0,
                    "success_value"
                ).map_err(|e| CursedError::Codegen(format!("Failed to extract success value: {}", e)))?;
                
                Ok(success_value)
            } else {
                Err(CursedError::Codegen("Union field is not a struct".to_string()))
            }
        } else {
            // For non-struct values, return as-is (assume already success value)
            Ok(result_value)
        }
    }

    fn extract_error_value(
        &mut self,
        result_value: BasicValueEnum<'ctx>,
    ) -> Result<(), Error> {
        debug!("Extracting error value from result");

        if let BasicValueEnum::StructValue(struct_val) = result_value {
            // Result is { is_ok: bool, value: union { success: T, error: E } }
            // Extract the union field (index 1)
            let union_value = self.builder.build_extract_value(
                struct_val,
                1,
                "result_union"
            ).map_err(|e| CursedError::Codegen(format!("Failed to extract union field: {}", e)))?;

            // Extract the error value from the union (index 1)
            if let BasicValueEnum::StructValue(union_struct) = union_value {
                let error_value = self.builder.build_extract_value(
                    union_struct,
                    1,
                    "error_value"
                ).map_err(|e| CursedError::Codegen(format!("Failed to extract error value: {}", e)))?;
                
                Ok(error_value)
            } else {
                Err(CursedError::Codegen("Union field is not a struct".to_string()))
            }
        } else {
            // For non-struct values, create a generic error
            let error_code = self.context.i32_type().const_int(1, false);
            Ok(error_code.into())
        }
    }

    fn generate_early_return(
        &mut self,
        error_value: BasicValueEnum<'ctx>,
    ) -> Result<(), Error> {
        debug!("Generating early return for error propagation");

        let current_fn = self.current_function()
            .ok_or_else(|| CursedError::Codegen("No current function for early return".to_string()))?;

        // Get the function's return type
        let return_type = current_fn.get_type().get_return_type()
            .ok_or_else(|| CursedError::Codegen("Function has no return type".to_string()))?;

        // Create a Result::Err with the error value
        if let BasicTypeEnum::StructType(struct_type) = return_type {
            // Assume the return type is Result<T, E>
            // Create an Err result with the extracted error
            let result_err = struct_type.get_undef();
            
            // Set is_ok = false
            let is_ok_false = self.context.bool_type().const_int(0, false);
            let result_with_flag = self.builder.build_insert_value(
                result_err,
                is_ok_false,
                0,
                "early_return_err_flag"
            ).map_err(|e| CursedError::Codegen(format!("Failed to set error flag: {}", e)))?;

            // Create union with error value
            let union_type = struct_type.get_field_type_at_index(1).unwrap().into_struct_type();
            let union_value = union_type.get_undef();
            let union_with_error = self.builder.build_insert_value(
                union_value,
                error_value,
                1,
                "early_return_error"
            ).map_err(|e| CursedError::Codegen(format!("Failed to set error in union: {}", e)))?;

            // Combine everything
            let final_result = self.builder.build_insert_value(
                result_with_flag.into_struct_value(),
                union_with_error,
                1,
                "early_return_result"
            ).map_err(|e| CursedError::Codegen(format!("Failed to create error result: {}", e)))?;

            // Generate the return
            self.builder.build_return(Some(&final_result))
                .map_err(|e| CursedError::Codegen(format!("Failed to build return: {}", e)))?;
        } else {
            // For non-struct return types, just return the error value directly
            self.builder.build_return(Some(&error_value))
                .map_err(|e| CursedError::Codegen(format!("Failed to build return: {}", e)))?;
        }

        Ok(())
    }
}

/// FFI functions for error propagation support
extern "C" {
    /// Check if a result value represents an error
    fn cursed_is_error(result_ptr: *const u8) -> bool;
    
    /// Extract the success value from a result
    fn cursed_extract_success(result_ptr: *const u8, out_ptr: *mut u8) -> bool;
    
    /// Extract the error value from a result
    fn cursed_extract_error(result_ptr: *const u8, out_ptr: *mut u8) -> bool;
    
    /// Propagate an error up the call stack
    fn cursed_propagate_error(error_ptr: *const u8) -> !;
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_error_propagation_compiler_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();

        let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
        assert_eq!(compiler.function_stack.len(), 0);
    }

    #[test]
    fn test_result_type_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();

        let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
        let i32_type = context.i32_type().into();
        let string_type = context.i8_type().ptr_type(AddressSpace::default()).into();
        
        let result_type = compiler.get_result_type(i32_type, string_type);
        assert_eq!(result_type.count_fields(), 2);
    }

    #[test]
    fn test_option_type_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();

        let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
        let i32_type = context.i32_type().into();
        
        let option_type = compiler.get_option_type(i32_type);
        assert_eq!(option_type.count_fields(), 2);
    }
}
