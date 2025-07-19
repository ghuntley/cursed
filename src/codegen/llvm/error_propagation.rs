//! Error Propagation Code Generation for CURSED
//!
//! This module implements LLVM IR generation for error propagation chains,
//! automatic error unwrapping, and control flow for error paths.

use crate::ast::{Expression, Statement, Type, YikesStatement, ShookExpression};
use crate::error::CursedError;
use crate::codegen::llvm::register_tracker::RegisterTracker;
use std::collections::HashMap;

/// Error propagation code generator
pub struct ErrorPropagationCodegen {
    /// Register tracker for LLVM IR generation
    register_tracker: RegisterTracker,
    /// Current error variable mappings
    error_variables: HashMap<String, String>,
    /// Current error propagation context
    propagation_context: Vec<String>,
    /// Error recovery blocks
    recovery_blocks: Vec<RecoveryBlock>,
    /// Function return type for error propagation
    function_return_type: Option<Type>,
}

#[derive(Debug, Clone)]
struct RecoveryBlock {
    label: String,
    error_variable: Option<String>,
    recovery_code: Vec<Statement>,
}

impl ErrorPropagationCodegen {
    pub fn new() -> Self {
        Self {
            register_tracker: RegisterTracker::new(),
            error_variables: HashMap::new(),
            propagation_context: Vec::new(),
            recovery_blocks: Vec::new(),
            function_return_type: None,
        }
    }

    /// Set the function return type for error propagation
    pub fn set_function_return_type(&mut self, return_type: Type) {
        self.function_return_type = Some(return_type);
    }

    /// Generate LLVM IR for Result<T,E> unwrapping logic with automatic error propagation
    pub fn generate_result_unwrap(&mut self, expr: &Expression, result_type: &Type, ok_type: &Type, err_type: &Type) -> Result<String, CursedError> {
        let mut ir = String::new();

        // Generate the result expression
        let result_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Evaluate result expression for unwrapping\n"));
        ir.push_str(&format!("  %result_{} = {}\n", result_register, self.generate_expression_value(expr)?));

        // Generate discriminant check for Result<T,E>
        let discriminant_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Extract discriminant from Result<{},{}>\n", 
                            self.type_to_llvm(ok_type)?, self.type_to_llvm(err_type)?));
        ir.push_str(&format!("  %discriminant_{} = extractvalue {{ i1, [{}] }} %result_{}, 0\n", 
                            discriminant_register, self.calculate_union_size(ok_type, err_type)?, result_register));

        // Create branch labels
        let ok_label = format!("result_ok_{}", self.register_tracker.next_register());
        let err_label = format!("result_err_{}", self.register_tracker.next_register());
        let continuation_label = format!("result_continue_{}", self.register_tracker.next_register());

        // Generate conditional branch
        ir.push_str(&format!("  ; Branch on Result discriminant\n"));
        ir.push_str(&format!("  br i1 %discriminant_{}, label %{}, label %{}\n", 
                            discriminant_register, ok_label, err_label));

        // Ok branch - extract value and continue
        ir.push_str(&format!("{}:\n", ok_label));
        let ok_value_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Extract Ok value from Result\n"));
        ir.push_str(&format!("  %ok_value_{} = extractvalue {{ i1, [{}] }} %result_{}, 1\n", 
                            ok_value_register, self.calculate_union_size(ok_type, err_type)?, result_register));
        ir.push_str(&format!("  %unwrapped_value_{} = bitcast [{}] %ok_value_{} to {}\n", 
                            ok_value_register, self.calculate_union_size(ok_type, err_type)?, 
                            ok_value_register, self.type_to_llvm(ok_type)?));
        ir.push_str(&format!("  br label %{}\n", continuation_label));

        // Err branch - automatic error propagation
        ir.push_str(&format!("{}:\n", err_label));
        let err_value_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Extract Err value for propagation\n"));
        ir.push_str(&format!("  %err_value_{} = extractvalue {{ i1, [{}] }} %result_{}, 1\n", 
                            err_value_register, self.calculate_union_size(ok_type, err_type)?, result_register));
        ir.push_str(&format!("  %propagated_error_{} = bitcast [{}] %err_value_{} to {}\n", 
                            err_value_register, self.calculate_union_size(ok_type, err_type)?, 
                            err_value_register, self.type_to_llvm(err_type)?));

        // Generate automatic error return if function can return errors
        if let Some(return_type) = &self.function_return_type {
            if self.is_result_type(return_type) {
                ir.push_str(&format!("  ; Automatic error return propagation\n"));
                let return_err_register = self.register_tracker.next_register();
                ir.push_str(&format!("  %return_err_{} = call {} @cursed_create_result_err({} %propagated_error_{})\n", 
                                    return_err_register, self.type_to_llvm(return_type)?, 
                                    self.type_to_llvm(err_type)?, err_value_register));
                ir.push_str(&format!("  ret {} %return_err_{}\n", 
                                    self.type_to_llvm(return_type)?, return_err_register));
            } else {
                // Function doesn't return Result - this is a compile error
                return Err(CursedError::General("Cannot use error propagation in function that doesn't return Result type".to_string()));
            }
        }

        // Continuation label for Ok case
        ir.push_str(&format!("{}:\n", continuation_label));
        ir.push_str(&format!("  ; Result unwrapped successfully, value available as %unwrapped_value_{}\n", ok_value_register));

        Ok(ir)
    }

    /// Generate LLVM IR for shook operator (equivalent to ? operator)
    pub fn generate_shook_propagation(&mut self, expr: &ShookExpression) -> Result<String, CursedError> {
        let mut ir = String::new();

        // Generate the wrapped expression
        let expr_ir = self.generate_expression_value(&expr.expression)?;
        ir.push_str(&format!("  ; Evaluate expression for shook propagation\n"));
        ir.push_str(&expr_ir);

        // Determine the Result types from the expression
        let result_type = self.infer_expression_type(&expr.expression)?;
        let (ok_type, err_type) = self.extract_result_types(&result_type)?;

        // Generate error checking and propagation
        ir.push_str(&self.generate_result_unwrap(&expr.expression, &result_type, &ok_type, &err_type)?);

        Ok(ir)
    }

    /// Generate LLVM IR for error context preservation during propagation
    pub fn generate_error_context_preservation(&mut self, original_error: &str, context: &str) -> Result<String, CursedError> {
        let mut ir = String::new();

        let context_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Preserve error context during propagation\n"));
        ir.push_str(&format!("  %error_context_{} = call i8* @cursed_preserve_error_context(i8* {}, i8* getelementptr inbounds ([{} x i8], [{}]* @error_context_msg, i32 0, i32 0))\n", 
                            context_register, original_error, context.len() + 1, context.len() + 1));

        Ok(ir)
    }

    /// Generate LLVM IR for error chain construction
    pub fn generate_error_chain(&mut self, errors: &[String]) -> Result<String, CursedError> {
        let mut ir = String::new();

        if errors.is_empty() {
            return Ok(ir);
        }

        ir.push_str(&format!("  ; Construct error chain\n"));
        let chain_register = self.register_tracker.next_register();
        ir.push_str(&format!("  %error_chain_{} = call i8* @cursed_create_error_chain()\n", chain_register));

        for (i, error) in errors.iter().enumerate() {
            let add_register = self.register_tracker.next_register();
            ir.push_str(&format!("  %chain_add_{} = call i8* @cursed_add_to_error_chain(i8* %error_chain_{}, i8* {})\n", 
                                add_register, chain_register, error));
        }

        Ok(ir)
    }

    /// Helper: Generate expression value
    fn generate_expression_value(&self, expr: &Expression) -> Result<String, CursedError> {
        // This would integrate with the main expression compiler
        Ok(format!("call i8* @cursed_evaluate_expression()"))
    }

    /// Helper: Convert CURSED type to LLVM type string
    fn type_to_llvm(&self, cursed_type: &Type) -> Result<String, CursedError> {
        match cursed_type {
            Type::Integer => Ok("i32".to_string()),
            Type::String => Ok("i8*".to_string()),
            Type::Boolean => Ok("i1".to_string()),
            Type::Float => Ok("double".to_string()),
            Type::Result(ok_type, err_type) => {
                Ok(format!("{{ i1, [{}] }}", 
                          self.calculate_union_size(ok_type, err_type)?))
            }
            _ => Ok("i8*".to_string()),
        }
    }

    /// Helper: Calculate union size for Result<T,E>
    fn calculate_union_size(&self, ok_type: &Type, err_type: &Type) -> Result<usize, CursedError> {
        let ok_size = self.calculate_type_size(ok_type)?;
        let err_size = self.calculate_type_size(err_type)?;
        Ok(ok_size.max(err_size))
    }

    /// Helper: Calculate size of a type
    fn calculate_type_size(&self, cursed_type: &Type) -> Result<usize, CursedError> {
        match cursed_type {
            Type::Integer => Ok(4),
            Type::Boolean => Ok(1),
            Type::Float => Ok(8),
            Type::String => Ok(8), // pointer
            _ => Ok(8), // default pointer size
        }
    }

    /// Helper: Infer type of expression
    fn infer_expression_type(&self, expr: &Expression) -> Result<Type, CursedError> {
        // This would integrate with the type system
        Ok(Type::Result(Box::new(Type::Integer), Box::new(Type::String)))
    }

    /// Helper: Extract Ok and Err types from Result type
    fn extract_result_types(&self, result_type: &Type) -> Result<(Type, Type), CursedError> {
        match result_type {
            Type::Result(ok_type, err_type) => Ok((*ok_type.clone(), *err_type.clone())),
            _ => Err(CursedError::General("Expected Result type".to_string()))
        }
    }

    /// Helper: Check if type is a Result type
    fn is_result_type(&self, cursed_type: &Type) -> bool {
        matches!(cursed_type, Type::Result(_, _))
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED error propagation system enabled".to_string())
}
