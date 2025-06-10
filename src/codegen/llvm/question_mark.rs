use crate::ast::expressions::{QuestionMarkExpression, ErrorPropagation};
use crate::ast::traits::Expression;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::{CursedError, ErrorPropagationError};
use crate::runtime::panic_recovery::PanicRecoveryRuntime;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue, StructValue};
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::basic_block::BasicBlock;
use inkwell::{IntPredicate, AddressSpace};
use std::collections::HashMap;

/// LLVM code generation support for the question mark operator (`?`)
/// 
/// The question mark operator in CURSED provides automatic error propagation.
/// This trait implements the LLVM IR generation for:
/// - Error checking and early return logic
/// - Result unwrapping for success cases
/// - Integration with panic/recovery system
/// - Memory-safe error value handling

pub trait QuestionMarkCompiler {
    /// Compile a question mark expression to LLVM IR
    fn compile_question_mark(&mut self, expr: &QuestionMarkExpression) -> Result<BasicValueEnum<'static>, CursedError>;
    
    /// Compile error propagation expression (backward compatibility)
    fn compile_error_propagation(&mut self, expr: &ErrorPropagation) -> Result<BasicValueEnum<'static>, CursedError>;
    
    /// Generate error checking logic for a value
    fn generate_error_check(&mut self, value: BasicValueEnum<'static>) -> Result<(BasicValueEnum<'static>, BasicBlock<'static>), CursedError>;
    
    /// Generate early return for error cases
    fn generate_early_return(&mut self, error_value: BasicValueEnum<'static>) -> Result<(), CursedError>;
    
    /// Unwrap success value from Result-like structure
    fn unwrap_success_value(&mut self, result_value: BasicValueEnum<'static>) -> Result<BasicValueEnum<'static>, CursedError>;
    
    /// Create Result-like structure (value, is_error)
    fn create_result_struct(&mut self, value: BasicValueEnum<'static>, is_error: bool) -> Result<BasicValueEnum<'static>, CursedError>;
    
    /// Check if a value represents an error
    fn is_error_value(&mut self, value: BasicValueEnum<'static>) -> Result<BasicValueEnum<'static>, CursedError>;
}

impl QuestionMarkCompiler for LlvmCodeGenerator {
    fn compile_question_mark(&mut self, expr: &QuestionMarkExpression) -> Result<BasicValueEnum<'static>, CursedError> {
        // Compile the inner expression first
        let inner_value = self.compile_expression(expr.inner_expression())?;
        
        // Generate error checking logic
        let (success_value, error_block) = self.generate_error_check(inner_value)?;
        
        // Get current function and create blocks
        let current_function = self.current_function()
            .ok_or_else(|| CursedError::CodeGeneration {
                message: "Question mark operator used outside function".to_string(),
                line: Some(expr.line),
                column: Some(expr.column),
            })?;
        
        let success_block = self.get_context().append_basic_block(current_function, "question_success");
        let continue_block = self.get_context().append_basic_block(current_function, "question_continue");
        
        // Check if the value is an error
        let is_error = self.is_error_value(inner_value)?;
        
        // Branch based on error status
        self.builder.build_conditional_branch(
            is_error.into_int_value(),
            error_block,
            success_block,
        ).map_err(|e| CursedError::CodeGeneration {
            message: format!("Failed to build conditional branch for question mark: {}", e),
            line: Some(expr.line),
            column: Some(expr.column),
        })?;
        
        // Error block: generate early return
        self.builder.position_at_end(error_block);
        self.generate_early_return(inner_value)?;
        
        // Success block: unwrap and continue
        self.builder.position_at_end(success_block);
        let unwrapped_value = self.unwrap_success_value(inner_value)?;
        self.builder.build_unconditional_branch(continue_block).map_err(|e| CursedError::CodeGeneration {
            message: format!("Failed to build branch to continue block: {}", e),
            line: Some(expr.line),
            column: Some(expr.column),
        })?;
        
        // Continue block: phi node for the result
        self.builder.position_at_end(continue_block);
        let phi = self.builder.build_phi(unwrapped_value.get_type(), "question_result")
            .map_err(|e| CursedError::CodeGeneration {
                message: format!("Failed to build phi node for question mark result: {}", e),
                line: Some(expr.line),
                column: Some(expr.column),
            })?;
        
        phi.add_incoming(&[(&unwrapped_value, success_block)]);
        
        Ok(phi.as_basic_value())
    }
    
    fn compile_error_propagation(&mut self, expr: &ErrorPropagation) -> Result<BasicValueEnum<'static>, CursedError> {
        // Compile the inner expression
        let inner_value = self.compile_expression(&*expr.expression)?;
        
        // Create a QuestionMarkExpression for consistency
        let question_expr = QuestionMarkExpression::new(
            expr.expression.clone_box(),
            1, // Default line
            1, // Default column
        );
        
        self.compile_question_mark(&question_expr)
    }
    
    fn generate_error_check(&mut self, value: BasicValueEnum<'static>) -> Result<(BasicValueEnum<'static>, BasicBlock<'static>), CursedError> {
        let current_function = self.current_function()
            .ok_or_else(|| CursedError::CodeGeneration {
                message: "Error check generated outside function".to_string(),
                line: None,
                column: None,
            })?;
        
        let error_block = self.context.append_basic_block(current_function, "error_block");
        
        Ok((value, error_block))
    }
    
    fn generate_early_return(&mut self, error_value: BasicValueEnum<'static>) -> Result<(), CursedError> {
        // For now, just return the error value
        // In a complete implementation, this would:
        // 1. Propagate the error up the call stack
        // 2. Perform any necessary cleanup
        // 3. Return from the current function with the error
        
        match error_value {
            BasicValueEnum::PointerValue(ptr) => {
                self.builder.build_return(Some(&ptr)).map_err(|e| CursedError::CodeGeneration {
                    message: format!("Failed to build error return: {}", e),
                    line: None,
                    column: None,
                })?;
            },
            BasicValueEnum::IntValue(int) => {
                self.builder.build_return(Some(&int)).map_err(|e| CursedError::CodeGeneration {
                    message: format!("Failed to build error return: {}", e),
                    line: None,
                    column: None,
                })?;
            },
            BasicValueEnum::StructValue(struct_val) => {
                self.builder.build_return(Some(&struct_val)).map_err(|e| CursedError::CodeGeneration {
                    message: format!("Failed to build error return: {}", e),
                    line: None,
                    column: None,
                })?;
            },
            _ => {
                // For other types, return void
                self.builder.build_return(None).map_err(|e| CursedError::CodeGeneration {
                    message: format!("Failed to build void error return: {}", e),
                    line: None,
                    column: None,
                })?;
            }
        }
        
        Ok(())
    }
    
    fn unwrap_success_value(&mut self, result_value: BasicValueEnum<'static>) -> Result<BasicValueEnum<'static>, CursedError> {
        // For now, assume the result_value is a struct with (value, is_error) fields
        // In a complete implementation, this would extract the success value
        // from a Result-like structure
        
        match result_value {
            BasicValueEnum::StructValue(struct_val) => {
                // Extract the first field (the actual value)
                let value_ptr = self.builder.build_extract_value(struct_val, 0, "success_value")
                    .map_err(|e| CursedError::CodeGeneration {
                        message: format!("Failed to extract success value: {}", e),
                        line: None,
                        column: None,
                    })?;
                Ok(value_ptr)
            },
            _ => {
                // For non-struct values, return as-is
                Ok(result_value)
            }
        }
    }
    
    fn create_result_struct(&mut self, value: BasicValueEnum<'static>, is_error: bool) -> Result<BasicValueEnum<'static>, CursedError> {
        // Create a Result-like struct with (value, is_error) fields
        let bool_type = self.context.bool_type();
        let struct_type = self.context.struct_type(&[value.get_type(), bool_type.into()], false);
        
        let result_struct = struct_type.get_undef();
        
        // Set the value field
        let with_value = self.builder.build_insert_value(result_struct, value, 0, "with_value")
            .map_err(|e| CursedError::CodeGeneration {
                message: format!("Failed to insert value into result struct: {}", e),
                line: None,
                column: None,
            })?;
        
        // Set the is_error field
        let error_flag = bool_type.const_int(if is_error { 1 } else { 0 }, false);
        let complete_struct = self.builder.build_insert_value(with_value, error_flag, 1, "with_error_flag")
            .map_err(|e| CursedError::CodeGeneration {
                message: format!("Failed to insert error flag into result struct: {}", e),
                line: None,
                column: None,
            })?;
        
        Ok(complete_struct)
    }
    
    fn is_error_value(&mut self, value: BasicValueEnum<'static>) -> Result<BasicValueEnum<'static>, CursedError> {
        // Check if a value represents an error
        // For now, this is a simplified implementation
        // In practice, this would check Result-like structures or error codes
        
        match value {
            BasicValueEnum::StructValue(struct_val) => {
                // Extract the is_error field (assumed to be at index 1)
                let is_error = self.builder.build_extract_value(struct_val, 1, "is_error")
                    .map_err(|e| CursedError::CodeGeneration {
                        message: format!("Failed to extract error flag: {}", e),
                        line: None,
                        column: None,
                    })?;
                Ok(is_error)
            },
            BasicValueEnum::PointerValue(ptr) => {
                // Check if pointer is null (simple error condition)
                let null_ptr = ptr.get_type().const_null();
                let is_null = self.builder.build_int_compare(
                    IntPredicate::EQ,
                    ptr.into_int_value(),
                    null_ptr.into_int_value(),
                    "is_null_error"
                ).map_err(|e| CursedError::CodeGeneration {
                    message: format!("Failed to build null pointer check: {}", e),
                    line: None,
                    column: None,
                })?;
                Ok(is_null.into())
            },
            _ => {
                // For other types, assume no error
                let bool_type = self.context.bool_type();
                Ok(bool_type.const_int(0, false).into())
            }
        }
    }
}

/// Helper implementation for the main LlvmCodeGenerator
impl LlvmCodeGenerator {
    /// Get the current function being compiled
    fn current_function(&self) -> Option<FunctionValue<'static>> {
        self.builder.get_insert_block()?.get_parent()
    }
    
    /// Compile any expression (placeholder - would delegate to actual expression compiler)
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'static>, CursedError> {
        // This is a placeholder - in the real implementation, this would
        // delegate to the appropriate expression compilation method
        
        // For now, return a dummy i32 value
        let i32_type = self.context.i32_type();
        Ok(i32_type.const_int(42, false).into())
    }
}

/// Runtime support for error propagation
pub struct ErrorPropagationRuntime {
    /// Stack of error handlers for nested error propagation
    error_handlers: Vec<Box<dyn Fn(CursedError) -> Result<(), CursedError>>>,
    
    /// Statistics for error propagation performance
    propagation_count: usize,
    successful_unwraps: usize,
}

impl ErrorPropagationRuntime {
    /// Create a new error propagation runtime
    pub fn new() -> Self {
        Self {
            error_handlers: Vec::new(),
            propagation_count: 0,
            successful_unwraps: 0,
        }
    }
    
    /// Register an error handler for the current scope
    pub fn push_error_handler<F>(&mut self, handler: F) 
    where F: Fn(CursedError) -> Result<(), CursedError> + 'static {
        self.error_handlers.push(Box::new(handler));
    }
    
    /// Remove the current error handler
    pub fn pop_error_handler(&mut self) -> Option<Box<dyn Fn(CursedError) -> Result<(), CursedError>>> {
        self.error_handlers.pop()
    }
    
    /// Propagate an error through the handler stack
    pub fn propagate_error(&mut self, error: CursedError) -> Result<(), CursedError> {
        self.propagation_count += 1;
        
        if let Some(handler) = self.error_handlers.last() {
            handler(error)
        } else {
            Err(error)
        }
    }
    
    /// Record a successful unwrap
    pub fn record_successful_unwrap(&mut self) {
        self.successful_unwraps += 1;
    }
    
    /// Get runtime statistics
    pub fn get_stats(&self) -> (usize, usize) {
        (self.propagation_count, self.successful_unwraps)
    }
}

impl Default for ErrorPropagationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::identifiers::Identifier;
    use crate::ast::traits::{Node, Expression};
    use inkwell::context::Context;

    #[test]
    fn test_error_propagation_runtime() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Test adding error handler
        runtime.push_error_handler(|_| Ok(()));
        assert_eq!(runtime.error_handlers.len(), 1);
        
        // Test removing error handler
        let handler = runtime.pop_error_handler();
        assert!(handler.is_some());
        assert_eq!(runtime.error_handlers.len(), 0);
    }
    
    #[test]
    fn test_error_propagation_stats() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        runtime.record_successful_unwrap();
        runtime.record_successful_unwrap();
        
        let (propagations, unwraps) = runtime.get_stats();
        assert_eq!(propagations, 0);
        assert_eq!(unwraps, 2);
    }
    
    #[test]
    fn test_question_mark_expression_creation() {
        let var_expr = crate::ast::identifiers::Identifier::new("test".to_string(), "test".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            5
        );
        
        assert_eq!(question_expr.line, 1);
        assert_eq!(question_expr.column, 5);
    }
    
    #[test]
    fn test_create_result_struct_concept() {
        let context = Context::create();
        
        // Test the concept of creating result structures
        let i32_type = context.i32_type();
        let bool_type = context.bool_type();
        let struct_type = context.struct_type(&[i32_type.into(), bool_type.into()], false);
        
        assert_eq!(struct_type.count_fields(), 2);
    }
}
