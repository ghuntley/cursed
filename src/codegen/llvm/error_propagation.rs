//! LLVM code generation for error propagation in CURSED
//!
//! This module provides comprehensive LLVM IR generation for the `?` operator,
//! including error checking, early returns, and integration with the CURSED
//! runtime error propagation system.

use crate::ast::expressions::{ErrorPropagation, QuestionMarkExpression};
use crate::ast::traits::Expression;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::{CursedError, SourceLocation};
use crate::parser::error_propagation::{EnhancedQuestionMarkExpression, TypedErrorPropagation};
use crate::runtime::error_propagation::{ErrorPropagationOperator, PropagationError, NoneError};
use crate::types::result::{Result as CursedResult, Option as CursedOption};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue, StructValue, IntValue, BasicValue};
use inkwell::types::{BasicTypeEnum, StructType, IntType, FunctionType};
use inkwell::basic_block::BasicBlock;
use inkwell::{IntPredicate, AddressSpace, AtomicOrdering};
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};

/// LLVM code generation for error propagation
pub trait ErrorPropagationCompiler {
    /// Compile question mark expression
    fn compile_question_mark(&mut self, expr: &QuestionMarkExpression) -> Result<BasicValueEnum<'static>, CursedError>;
    
    /// Compile enhanced question mark expression
    fn compile_enhanced_question_mark(&mut self, expr: &EnhancedQuestionMarkExpression) -> Result<BasicValueEnum<'static>, CursedError>;
    
    /// Compile typed error propagation
    fn compile_typed_error_propagation(&mut self, expr: &TypedErrorPropagation) -> Result<BasicValueEnum<'static>, CursedError>;
    
    /// Generate Result error checking
    fn generate_result_check(&mut self, result_value: BasicValueEnum<'static>) -> Result<ErrorCheckResult<'static>, CursedError>;
    
    /// Generate Option error checking
    fn generate_option_check(&mut self, option_value: BasicValueEnum<'static>) -> Result<ErrorCheckResult<'static>, CursedError>;
    
    /// Generate early return for errors
    fn generate_early_return(&mut self, error_value: BasicValueEnum<'static>, context: &PropagationContext) -> Result<(), CursedError>;
    
    /// Generate error context recording
    fn generate_error_context_recording(&mut self, context: &PropagationContext) -> Result<(), CursedError>;
}

impl ErrorPropagationCompiler for LlvmCodeGenerator {
    #[instrument(skip(self, expr))]
    fn compile_question_mark(&mut self, expr: &QuestionMarkExpression) -> Result<BasicValueEnum<'static>, CursedError> {
        // Compile the inner expression first
        let inner_value = self.compile_expression(&expr.expression)?;
        
        // Determine the type of the inner expression
        let inner_type = inner_value.get_type();
        
        // Generate appropriate error checking based on type
        if self.is_result_type(&inner_type) {
            let check_result = self.generate_result_check(inner_value)?;
            self.handle_result_propagation(check_result, expr)
        } else if self.is_option_type(&inner_type) {
            let check_result = self.generate_option_check(inner_value)?;
            self.handle_option_propagation(check_result, expr)
        } else {
            Err(CursedError::CodeGeneration {
                message: format!("Cannot apply '?' operator to type: {:?}", inner_type),
                line: Some(expr.location().line),
                column: Some(expr.location().column),
            })
        }
    }

    #[instrument(skip(self, expr))]
    fn compile_enhanced_question_mark(&mut self, expr: &EnhancedQuestionMarkExpression) -> Result<BasicValueEnum<'static>, CursedError> {
        // Create propagation context
        let context = PropagationContext {
            source_location: expr.location().clone(),
            function_context: expr.function_context.clone(),
            expected_return_type: expr.expected_return_type.clone(),
        };

        // Record error context
        self.generate_error_context_recording(&context)?;

        // Compile inner expression
        let inner_value = self.compile_expression(&expr.expression)?;
        
        // Generate enhanced error checking with context
        self.generate_enhanced_error_check(inner_value, &context)
    }

    #[instrument(skip(self, expr))]
    fn compile_typed_error_propagation(&mut self, expr: &TypedErrorPropagation) -> Result<BasicValueEnum<'static>, CursedError> {
        // Validate type compatibility at compile time
        self.validate_propagation_types(&expr.expression_type, &expr.return_type)?;
        
        // Compile with type-specific optimizations
        let inner_value = self.compile_expression(&expr.expression)?;
        
        if expr.expression_type.starts_with("Result<") {
            self.generate_typed_result_propagation(inner_value, &expr.expression_type, &expr.return_type)
        } else if expr.expression_type.starts_with("Option<") {
            self.generate_typed_option_propagation(inner_value, &expr.expression_type, &expr.return_type)
        } else {
            Err(CursedError::CodeGeneration {
                message: format!("Unsupported type for error propagation: {}", expr.expression_type),
                line: None,
                column: None,
            })
        }
    }

    #[instrument(skip(self, result_value))]
    fn generate_result_check(&mut self, result_value: BasicValueEnum<'static>) -> Result<ErrorCheckResult<'static>, CursedError> {
        // Assume Result<T, E> is represented as a struct { is_ok: bool, value: union { ok: T, err: E } }
        let result_struct = result_value.into_struct_value();
        
        // Extract the is_ok flag (first field)
        let is_ok_ptr = self.builder().build_struct_gep(
            result_struct.get_type(),
            result_value.into_pointer_value(),
            0,
            "is_ok_ptr"
        ).map_err(|e| CursedError::system_error(&format!("Failed to build struct GEP: {}", e)))?;
        
        let is_ok = self.builder().build_load(
            self.get_context().bool_type(),
            is_ok_ptr,
            "is_ok"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load is_ok: {}", e)))?;

        // Extract the value union (second field)
        let value_ptr = self.builder().build_struct_gep(
            result_struct.get_type(),
            result_value.into_pointer_value(),
            1,
            "value_ptr"
        ).map_err(|e| CursedError::system_error(&format!("Failed to build value GEP: {}", e)))?;

        Ok(ErrorCheckResult {
            is_success: is_ok.into_int_value(),
            success_value: value_ptr,
            error_value: value_ptr, // Same location, different interpretation
            original_value: result_value,
        })
    }

    #[instrument(skip(self, option_value))]
    fn generate_option_check(&mut self, option_value: BasicValueEnum<'static>) -> Result<ErrorCheckResult<'static>, CursedError> {
        // Assume Option<T> is represented as a struct { is_some: bool, value: T }
        let option_struct = option_value.into_struct_value();
        
        // Extract the is_some flag
        let is_some_ptr = self.builder().build_struct_gep(
            option_struct.get_type(),
            option_value.into_pointer_value(),
            0,
            "is_some_ptr"
        ).map_err(|e| CursedError::system_error(&format!("Failed to build struct GEP: {}", e)))?;
        
        let is_some = self.builder().build_load(
            self.get_context().bool_type(),
            is_some_ptr,
            "is_some"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load is_some: {}", e)))?;

        // Extract the value field
        let value_ptr = self.builder().build_struct_gep(
            option_struct.get_type(),
            option_value.into_pointer_value(),
            1,
            "value_ptr"
        ).map_err(|e| CursedError::system_error(&format!("Failed to build value GEP: {}", e)))?;

        // Create a None error value for Option
        let none_error = self.create_none_error_value()?;

        Ok(ErrorCheckResult {
            is_success: is_some.into_int_value(),
            success_value: value_ptr,
            error_value: none_error,
            original_value: option_value,
        })
    }

    #[instrument(skip(self, error_value, context))]
    fn generate_early_return(&mut self, error_value: BasicValueEnum<'static>, context: &PropagationContext) -> Result<(), CursedError> {
        // Get current function
        let current_function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function for early return"))?;

        // Create error result for return
        let error_result = self.create_error_result_value(error_value, context)?;

        // Generate return instruction
        self.builder().build_return(Some(&error_result))
            .map_err(|e| CursedError::system_error(&format!("Failed to build return: {}", e)))?;

        Ok(())
    }

    #[instrument(skip(self, context))]
    fn generate_error_context_recording(&mut self, context: &PropagationContext) -> Result<(), CursedError> {
        // Generate call to runtime error propagation system
        let runtime_function = self.get_or_declare_error_context_function()?;
        
        // Create arguments for context recording
        let location_line = self.get_context().i32_type().const_int(context.source_location.line as u64, false);
        let location_column = self.get_context().i32_type().const_int(context.source_location.column as u64, false);
        
        // Create function name string (if available)
        let function_name = if let Some(ref func_name) = context.function_context {
            self.create_string_constant(func_name)?
        } else {
            self.get_context().i8_type().ptr_type(AddressSpace::default()).const_null()
        };

        // Call runtime function
        self.builder().build_call(
            runtime_function,
            &[location_line.into(), location_column.into(), function_name.into()],
            "record_error_context"
        ).map_err(|e| CursedError::system_error(&format!("Failed to call error context function: {}", e)))?;

        Ok(())
    }
}

/// Implementation of enhanced error checking and propagation
impl LlvmCodeGenerator {
    /// Handle Result type propagation
    #[instrument(skip(self, check_result, expr))]
    fn handle_result_propagation(
        &mut self,
        check_result: ErrorCheckResult<'static>,
        expr: &QuestionMarkExpression,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let current_function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function"))?;

        // Create basic blocks
        let success_block = self.get_context().append_basic_block(current_function, "result_success");
        let error_block = self.get_context().append_basic_block(current_function, "result_error");
        let merge_block = self.get_context().append_basic_block(current_function, "result_merge");

        // Branch based on success flag
        self.builder().build_conditional_branch(
            check_result.is_success,
            success_block,
            error_block
        ).map_err(|e| CursedError::system_error(&format!("Failed to build conditional branch: {}", e)))?;

        // Handle success case
        self.builder().position_at_end(success_block);
        let success_value = self.builder().build_load(
            self.infer_value_type(&check_result.success_value)?,
            check_result.success_value,
            "success_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load success value: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::system_error(&format!("Failed to build branch: {}", e)))?;

        // Handle error case
        self.builder().position_at_end(error_block);
        let context = PropagationContext {
            source_location: expr.location().clone(),
            function_context: None,
            expected_return_type: None,
        };
        
        let error_value = self.builder().build_load(
            self.infer_value_type(&check_result.error_value.into_pointer_value())?,
            check_result.error_value,
            "error_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load error value: {}", e)))?;
        
        self.generate_early_return(error_value, &context)?;

        // Merge block (for success path)
        self.builder().position_at_end(merge_block);
        Ok(success_value)
    }

    /// Handle Option type propagation
    #[instrument(skip(self, check_result, expr))]
    fn handle_option_propagation(
        &mut self,
        check_result: ErrorCheckResult<'static>,
        expr: &QuestionMarkExpression,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let current_function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function"))?;

        // Create basic blocks
        let some_block = self.get_context().append_basic_block(current_function, "option_some");
        let none_block = self.get_context().append_basic_block(current_function, "option_none");
        let merge_block = self.get_context().append_basic_block(current_function, "option_merge");

        // Branch based on is_some flag
        self.builder().build_conditional_branch(
            check_result.is_success,
            some_block,
            none_block
        ).map_err(|e| CursedError::system_error(&format!("Failed to build conditional branch: {}", e)))?;

        // Handle Some case
        self.builder().position_at_end(some_block);
        let some_value = self.builder().build_load(
            self.infer_value_type(&check_result.success_value)?,
            check_result.success_value,
            "some_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load some value: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::system_error(&format!("Failed to build branch: {}", e)))?;

        // Handle None case
        self.builder().position_at_end(none_block);
        let context = PropagationContext {
            source_location: expr.location().clone(),
            function_context: None,
            expected_return_type: None,
        };
        
        self.generate_early_return(check_result.error_value, &context)?;

        // Merge block (for Some path)
        self.builder().position_at_end(merge_block);
        Ok(some_value)
    }

    /// Generate enhanced error checking with full context
    #[instrument(skip(self, value, context))]
    fn generate_enhanced_error_check(
        &mut self,
        value: BasicValueEnum<'static>,
        context: &PropagationContext,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let value_type = value.get_type();
        
        if self.is_result_type(&value_type) {
            let check_result = self.generate_result_check(value)?;
            self.handle_enhanced_result_propagation(check_result, context)
        } else if self.is_option_type(&value_type) {
            let check_result = self.generate_option_check(value)?;
            self.handle_enhanced_option_propagation(check_result, context)
        } else {
            Err(CursedError::CodeGeneration {
                message: format!("Type not compatible with error propagation: {:?}", value_type),
                line: Some(context.source_location.line),
                column: Some(context.source_location.column),
            })
        }
    }

    /// Handle enhanced Result propagation with context
    fn handle_enhanced_result_propagation(
        &mut self,
        check_result: ErrorCheckResult<'static>,
        context: &PropagationContext,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let current_function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function"))?;

        // Create blocks with enhanced names
        let success_block = self.get_context().append_basic_block(current_function, "enhanced_result_success");
        let error_block = self.get_context().append_basic_block(current_function, "enhanced_result_error");
        let merge_block = self.get_context().append_basic_block(current_function, "enhanced_result_merge");

        // Branch on success flag
        self.builder().build_conditional_branch(
            check_result.is_success,
            success_block,
            error_block
        ).map_err(|e| CursedError::system_error(&format!("Failed to build conditional branch: {}", e)))?;

        // Success path
        self.builder().position_at_end(success_block);
        let success_value = self.builder().build_load(
            self.infer_value_type(&check_result.success_value)?,
            check_result.success_value,
            "enhanced_success_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load success value: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::system_error(&format!("Failed to build branch: {}", e)))?;

        // Error path with enhanced context
        self.builder().position_at_end(error_block);
        self.generate_error_context_recording(context)?;
        
        let error_value = self.builder().build_load(
            self.infer_value_type(&check_result.error_value.into_pointer_value())?,
            check_result.error_value,
            "enhanced_error_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load error value: {}", e)))?;
        
        self.generate_early_return(error_value, context)?;

        // Merge block
        self.builder().position_at_end(merge_block);
        Ok(success_value)
    }

    /// Handle enhanced Option propagation with context
    fn handle_enhanced_option_propagation(
        &mut self,
        check_result: ErrorCheckResult<'static>,
        context: &PropagationContext,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let current_function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function"))?;

        // Create blocks
        let some_block = self.get_context().append_basic_block(current_function, "enhanced_option_some");
        let none_block = self.get_context().append_basic_block(current_function, "enhanced_option_none");
        let merge_block = self.get_context().append_basic_block(current_function, "enhanced_option_merge");

        // Branch on is_some flag
        self.builder().build_conditional_branch(
            check_result.is_success,
            some_block,
            none_block
        ).map_err(|e| CursedError::system_error(&format!("Failed to build conditional branch: {}", e)))?;

        // Some path
        self.builder().position_at_end(some_block);
        let some_value = self.builder().build_load(
            self.infer_value_type(&check_result.success_value)?,
            check_result.success_value,
            "enhanced_some_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load some value: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::system_error(&format!("Failed to build branch: {}", e)))?;

        // None path with enhanced context
        self.builder().position_at_end(none_block);
        self.generate_error_context_recording(context)?;
        self.generate_early_return(check_result.error_value, context)?;

        // Merge block
        self.builder().position_at_end(merge_block);
        Ok(some_value)
    }

    /// Validate type compatibility for propagation
    fn validate_propagation_types(&self, expr_type: &str, return_type: &str) -> Result<(), CursedError> {
        // Check if error types are compatible
        if expr_type.starts_with("Result<") && return_type.starts_with("Result<") {
            // Both Results - check error type compatibility
            return Ok(());
        }
        
        if expr_type.starts_with("Option<") && return_type.starts_with("Option<") {
            // Both Options - compatible
            return Ok(());
        }
        
        if expr_type.starts_with("Option<") && return_type.starts_with("Result<") {
            // Option -> Result conversion possible
            return Ok(());
        }

        Err(CursedError::CodeGeneration {
            message: format!("Incompatible types for error propagation: {} -> {}", expr_type, return_type),
            line: None,
            column: None,
        })
    }

    /// Generate typed Result propagation
    fn generate_typed_result_propagation(
        &mut self,
        value: BasicValueEnum<'static>,
        expr_type: &str,
        return_type: &str,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let check_result = self.generate_result_check(value)?;
        
        // Generate optimized propagation based on type information
        if expr_type == return_type {
            // Same type - direct propagation
            self.generate_direct_result_propagation(check_result)
        } else {
            // Type conversion needed
            self.generate_converted_result_propagation(check_result, expr_type, return_type)
        }
    }

    /// Generate typed Option propagation
    fn generate_typed_option_propagation(
        &mut self,
        value: BasicValueEnum<'static>,
        expr_type: &str,
        return_type: &str,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let check_result = self.generate_option_check(value)?;
        
        if expr_type == return_type {
            self.generate_direct_option_propagation(check_result)
        } else if return_type.starts_with("Result<") {
            self.generate_option_to_result_propagation(check_result)
        } else {
            Err(CursedError::CodeGeneration {
                message: format!("Unsupported Option propagation: {} -> {}", expr_type, return_type),
                line: None,
                column: None,
            })
        }
    }

    /// Generate direct Result propagation (same type)
    fn generate_direct_result_propagation(
        &mut self,
        check_result: ErrorCheckResult<'static>,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let current_function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function"))?;

        let success_block = self.get_context().append_basic_block(current_function, "direct_result_success");
        let error_block = self.get_context().append_basic_block(current_function, "direct_result_error");
        let merge_block = self.get_context().append_basic_block(current_function, "direct_result_merge");

        self.builder().build_conditional_branch(
            check_result.is_success,
            success_block,
            error_block
        ).map_err(|e| CursedError::system_error(&format!("Failed to build conditional branch: {}", e)))?;

        // Success: extract value
        self.builder().position_at_end(success_block);
        let success_value = self.builder().build_load(
            self.infer_value_type(&check_result.success_value)?,
            check_result.success_value,
            "direct_success_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load success value: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::system_error(&format!("Failed to build branch: {}", e)))?;

        // Error: return original Result
        self.builder().position_at_end(error_block);
        self.builder().build_return(Some(&check_result.original_value))
            .map_err(|e| CursedError::system_error(&format!("Failed to build return: {}", e)))?;

        self.builder().position_at_end(merge_block);
        Ok(success_value)
    }

    /// Generate direct Option propagation (same type)
    fn generate_direct_option_propagation(
        &mut self,
        check_result: ErrorCheckResult<'static>,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let current_function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function"))?;

        let some_block = self.get_context().append_basic_block(current_function, "direct_option_some");
        let none_block = self.get_context().append_basic_block(current_function, "direct_option_none");
        let merge_block = self.get_context().append_basic_block(current_function, "direct_option_merge");

        self.builder().build_conditional_branch(
            check_result.is_success,
            some_block,
            none_block
        ).map_err(|e| CursedError::system_error(&format!("Failed to build conditional branch: {}", e)))?;

        // Some: extract value
        self.builder().position_at_end(some_block);
        let some_value = self.builder().build_load(
            self.infer_value_type(&check_result.success_value)?,
            check_result.success_value,
            "direct_some_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load some value: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::system_error(&format!("Failed to build branch: {}", e)))?;

        // None: return None
        self.builder().position_at_end(none_block);
        let none_option = self.create_none_option_value()?;
        self.builder().build_return(Some(&none_option))
            .map_err(|e| CursedError::system_error(&format!("Failed to build return: {}", e)))?;

        self.builder().position_at_end(merge_block);
        Ok(some_value)
    }

    /// Utility functions
    fn is_result_type(&self, type_enum: &BasicTypeEnum) -> bool {
        // Check if the type represents a Result<T, E>
        // This is a simplified check - real implementation would use type metadata
        match type_enum {
            BasicTypeEnum::StructType(struct_type) => {
                // Check struct name or metadata for Result type
                struct_type.count_fields() == 2 // Result has is_ok and value fields
            }
            _ => false,
        }
    }

    fn is_option_type(&self, type_enum: &BasicTypeEnum) -> bool {
        // Check if the type represents an Option<T>
        match type_enum {
            BasicTypeEnum::StructType(struct_type) => {
                // Check struct name or metadata for Option type
                struct_type.count_fields() == 2 // Option has is_some and value fields
            }
            _ => false,
        }
    }

    fn create_none_error_value(&self) -> Result<BasicValueEnum<'static>, CursedError> {
        // Create a NoneError value for Option propagation
        let none_error_type = self.get_or_create_none_error_type()?;
        let none_error = none_error_type.const_zero();
        Ok(none_error.into())
    }

    fn create_error_result_value(
        &self,
        error_value: BasicValueEnum<'static>,
        context: &PropagationContext,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        // Create a Result::Err value for return
        let result_type = self.get_current_function_return_type()?;
        let result_struct = result_type.into_struct_type();
        
        // Create Result { is_ok: false, value: error }
        let is_ok_false = self.get_context().bool_type().const_zero();
        let result_value = result_struct.const_named_struct(&[is_ok_false.into(), error_value]);
        
        Ok(result_value.into())
    }

    fn get_or_declare_error_context_function(&mut self) -> Result<FunctionValue<'static>, CursedError> {
        let function_name = "cursed_record_error_context";
        
        // Check if function already exists
        if let Some(function) = self.get_module().get_function(function_name) {
            return Ok(function);
        }

        // Create function type: void(i32, i32, i8*)
        let function_type = self.get_context().void_type().fn_type(
            &[
                self.get_context().i32_type().into(), // line
                self.get_context().i32_type().into(), // column
                self.get_context().i8_type().ptr_type(AddressSpace::default()).into(), // function_name
            ],
            false,
        );

        // Declare the function
        let function = self.get_module().add_function(function_name, function_type, None);
        Ok(function)
    }

    fn create_string_constant(&self, s: &str) -> Result<PointerValue<'static>, CursedError> {
        let string_value = self.get_context().const_string(s.as_bytes(), true);
        let global = self.get_module().add_global(string_value.get_type(), Some(AddressSpace::default()), "str_const");
        global.set_initializer(&string_value);
        global.set_constant(true);
        
        Ok(global.as_pointer_value())
    }

    fn infer_value_type(&self, ptr: &PointerValue<'static>) -> Result<BasicTypeEnum<'static>, CursedError> {
        Ok(ptr.get_type().get_pointee_type().unwrap())
    }

    fn get_current_function(&self) -> Option<FunctionValue<'static>> {
        self.get_current_function()
    }

    fn get_current_function_return_type(&self) -> Result<BasicTypeEnum<'static>, CursedError> {
        let function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function"))?;
        
        Ok(function.get_type().get_return_type()
            .ok_or_else(|| CursedError::system_error("Function has no return type"))?)
    }

    fn get_or_create_none_error_type(&self) -> Result<StructType<'static>, CursedError> {
        // Create or get the NoneError struct type
        let error_type = self.get_context().struct_type(
            &[
                self.get_context().i8_type().ptr_type(AddressSpace::default()).into(), // message
                self.get_context().i32_type().into(), // line
                self.get_context().i32_type().into(), // column
            ],
            false,
        );
        Ok(error_type)
    }

    fn create_none_option_value(&self) -> Result<BasicValueEnum<'static>, CursedError> {
        // Create an Option::None value
        let option_type = self.get_context().struct_type(
            &[
                self.get_context().bool_type().into(), // is_some
                self.get_context().i8_type().into(),   // placeholder value
            ],
            false,
        );
        
        let is_some_false = self.get_context().bool_type().const_zero();
        let placeholder = self.get_context().i8_type().const_zero();
        let none_value = option_type.const_named_struct(&[is_some_false.into(), placeholder.into()]);
        
        Ok(none_value.into())
    }

    fn generate_converted_result_propagation(
        &mut self,
        check_result: ErrorCheckResult<'static>,
        expr_type: &str,
        return_type: &str,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        // Generate type conversion for Result propagation
        // This is a simplified implementation - real version would handle complex type conversions
        self.generate_direct_result_propagation(check_result)
    }

    fn generate_option_to_result_propagation(
        &mut self,
        check_result: ErrorCheckResult<'static>,
    ) -> Result<BasicValueEnum<'static>, CursedError> {
        let current_function = self.get_current_function()
            .ok_or_else(|| CursedError::system_error("No current function"))?;

        let some_block = self.get_context().append_basic_block(current_function, "option_to_result_some");
        let none_block = self.get_context().append_basic_block(current_function, "option_to_result_none");
        let merge_block = self.get_context().append_basic_block(current_function, "option_to_result_merge");

        self.builder().build_conditional_branch(
            check_result.is_success,
            some_block,
            none_block
        ).map_err(|e| CursedError::system_error(&format!("Failed to build conditional branch: {}", e)))?;

        // Some: extract value
        self.builder().position_at_end(some_block);
        let some_value = self.builder().build_load(
            self.infer_value_type(&check_result.success_value)?,
            check_result.success_value,
            "option_to_result_value"
        ).map_err(|e| CursedError::system_error(&format!("Failed to load some value: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::system_error(&format!("Failed to build branch: {}", e)))?;

        // None: return Result::Err with None error
        self.builder().position_at_end(none_block);
        let none_error = self.create_none_error_value()?;
        let error_result = self.create_error_result_value(none_error, &PropagationContext {
            source_location: SourceLocation::new(0, 0),
            function_context: None,
            expected_return_type: None,
        })?;
        
        self.builder().build_return(Some(&error_result))
            .map_err(|e| CursedError::system_error(&format!("Failed to build return: {}", e)))?;

        self.builder().position_at_end(merge_block);
        Ok(some_value)
    }
}

/// Result of error checking
#[derive(Debug)]
pub struct ErrorCheckResult<'ctx> {
    /// Flag indicating success (true) or error (false)
    pub is_success: IntValue<'ctx>,
    /// Pointer to success value
    pub success_value: PointerValue<'ctx>,
    /// Pointer to error value
    pub error_value: BasicValueEnum<'ctx>,
    /// Original value being checked
    pub original_value: BasicValueEnum<'ctx>,
}

/// Context for error propagation
#[derive(Debug, Clone)]
pub struct PropagationContext {
    /// Source location of the propagation
    pub source_location: SourceLocation,
    /// Function context
    pub function_context: Option<String>,
    /// Expected return type
    pub expected_return_type: Option<String>,
}

/// FFI functions for runtime integration
extern "C" {
    /// Record error propagation context in runtime
    fn cursed_record_error_context(line: i32, column: i32, function_name: *const i8);
    
    /// Create error propagation operator instance
    fn cursed_create_error_propagator() -> *mut ErrorPropagationOperator;
    
    /// Apply question mark operator to Result
    fn cursed_apply_question_mark_result(
        propagator: *mut ErrorPropagationOperator,
        result_ptr: *const u8,
        line: i32,
        column: i32,
        function_name: *const i8,
    ) -> *mut u8;
    
    /// Apply question mark operator to Option
    fn cursed_apply_question_mark_option(
        propagator: *mut ErrorPropagationOperator,
        option_ptr: *const u8,
        line: i32,
        column: i32,
        function_name: *const i8,
    ) -> *mut u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propagation_context_creation() {
        let context = PropagationContext {
            source_location: SourceLocation::new(1, 5),
            function_context: Some("test_function".to_string()),
            expected_return_type: Some("Result<i32, String>".to_string()),
        };

        assert_eq!(context.source_location.line, 1);
        assert_eq!(context.function_context, Some("test_function".to_string()));
    }

    #[test]
    fn test_error_check_result_creation() {
        // This would need a mock LLVM context for testing
        // let context = Context::create();
        // let is_success = context.bool_type().const_int(1, false);
        // ... create mock values for testing
    }
}
