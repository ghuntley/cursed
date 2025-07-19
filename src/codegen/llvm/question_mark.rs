//! Question Mark Operator Code Generation for CURSED
//!
//! This module implements LLVM IR generation for the `?` operator equivalent (`shook` in CURSED),
//! providing automatic error unwrapping and propagation for Result<T,E> and Option<T> types.

use crate::ast::{Expression, Type, ShookExpression};
use crate::error::CursedError;
use crate::codegen::llvm::register_tracker::RegisterTracker;
use crate::codegen::llvm::error_propagation::ErrorPropagationCodegen;
use std::collections::HashMap;

/// Question mark operator code generator
pub struct QuestionMarkCodegen {
    /// Register tracker for LLVM IR generation
    register_tracker: RegisterTracker,
    /// Error propagation generator
    error_propagation: ErrorPropagationCodegen,
    /// Type inference context
    type_context: HashMap<String, Type>,
    /// Current function return type
    function_return_type: Option<Type>,
}

impl QuestionMarkCodegen {
    pub fn new() -> Self {
        Self {
            register_tracker: RegisterTracker::new(),
            error_propagation: ErrorPropagationCodegen::new(),
            type_context: HashMap::new(),
            function_return_type: None,
        }
    }

    /// Set the current function return type for error propagation
    pub fn set_function_return_type(&mut self, return_type: Type) {
        self.function_return_type = Some(return_type.clone());
        self.error_propagation.set_function_return_type(return_type);
    }

    /// Generate LLVM IR for the `shook` operator (equivalent to `?`)
    pub fn generate_question_mark_operator(&mut self, expr: &ShookExpression) -> Result<String, CursedError> {
        let mut ir = String::new();

        // Determine the type of the expression being unwrapped
        let expr_type = self.infer_expression_type(&expr.expression)?;

        match expr_type {
            Type::Result(ok_type, err_type) => {
                ir.push_str(&self.generate_result_question_mark(&expr.expression, &ok_type, &err_type)?);
            }
            Type::Option(inner_type) => {
                ir.push_str(&self.generate_option_question_mark(&expr.expression, &inner_type)?);
            }
            _ => {
                return Err(CursedError::General(
                    "Question mark operator can only be used with Result<T,E> or Option<T> types".to_string()
                ));
            }
        }

        Ok(ir)
    }

    /// Generate LLVM IR for Result<T,E> question mark operation
    fn generate_result_question_mark(&mut self, expr: &Expression, ok_type: &Type, err_type: &Type) -> Result<String, CursedError> {
        let mut ir = String::new();

        // Generate the result expression
        let result_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Evaluate Result<{},{}> expression for ? operator\n", 
                            self.type_to_llvm_string(ok_type)?, self.type_to_llvm_string(err_type)?));
        ir.push_str(&format!("  %result_{} = {}\n", result_register, self.generate_expression_ir(expr)?));

        // Create discriminant check
        let discriminant_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Extract Result discriminant (0=Ok, 1=Err)\n"));
        ir.push_str(&format!("  %discriminant_{} = extractvalue {{ i1, [{}] }} %result_{}, 0\n", 
                            discriminant_register, self.calculate_union_size(ok_type, err_type)?, result_register));

        // Create branch labels
        let ok_label = format!("question_mark_ok_{}", self.register_tracker.next_register());
        let err_label = format!("question_mark_err_{}", self.register_tracker.next_register());
        let continue_label = format!("question_mark_continue_{}", self.register_tracker.next_register());

        // Branch on discriminant (false=Ok, true=Err)
        ir.push_str(&format!("  ; Branch: false=Ok (continue), true=Err (propagate)\n"));
        ir.push_str(&format!("  br i1 %discriminant_{}, label %{}, label %{}\n", 
                            discriminant_register, err_label, ok_label));

        // Ok branch: extract value and continue
        ir.push_str(&format!("{}:\n", ok_label));
        let ok_value_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Extract Ok value for continued execution\n"));
        ir.push_str(&format!("  %ok_value_raw_{} = extractvalue {{ i1, [{}] }} %result_{}, 1\n", 
                            ok_value_register, self.calculate_union_size(ok_type, err_type)?, result_register));
        
        // Cast the union data to the proper Ok type
        let unwrapped_register = self.register_tracker.next_register();
        ir.push_str(&format!("  %unwrapped_value_{} = bitcast [{}] %ok_value_raw_{} to {}\n", 
                            unwrapped_register, self.calculate_union_size(ok_type, err_type)?, 
                            ok_value_register, self.type_to_llvm_string(ok_type)?));
        ir.push_str(&format!("  br label %{}\n", continue_label));

        // Err branch: propagate error (early return)
        ir.push_str(&format!("{}:\n", err_label));
        let err_value_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Extract Err value for propagation\n"));
        ir.push_str(&format!("  %err_value_raw_{} = extractvalue {{ i1, [{}] }} %result_{}, 1\n", 
                            err_value_register, self.calculate_union_size(ok_type, err_type)?, result_register));
        
        // Cast to proper error type
        let error_register = self.register_tracker.next_register();
        ir.push_str(&format!("  %error_value_{} = bitcast [{}] %err_value_raw_{} to {}\n", 
                            error_register, self.calculate_union_size(ok_type, err_type)?, 
                            err_value_register, self.type_to_llvm_string(err_type)?));

        // Generate early return with error
        if let Some(return_type) = &self.function_return_type {
            if self.is_result_type(return_type) {
                let return_err_register = self.register_tracker.next_register();
                ir.push_str(&format!("  ; Create Result::Err for early return\n"));
                ir.push_str(&format!("  %return_result_{} = call {} @cursed_create_result_err({} %error_value_{})\n", 
                                    return_err_register, self.type_to_llvm_string(return_type)?, 
                                    self.type_to_llvm_string(err_type)?, error_register));
                ir.push_str(&format!("  ret {} %return_result_{}\n", 
                                    self.type_to_llvm_string(return_type)?, return_err_register));
            } else {
                return Err(CursedError::General(
                    "Cannot use ? operator in function that doesn't return Result type".to_string()
                ));
            }
        } else {
            return Err(CursedError::General(
                "? operator requires function return type to be known".to_string()
            ));
        }

        // Continue label: the Ok value is available here
        ir.push_str(&format!("{}:\n", continue_label));
        ir.push_str(&format!("  ; Ok value available as %unwrapped_value_{}\n", unwrapped_register));

        Ok(ir)
    }

    /// Generate LLVM IR for Option<T> question mark operation
    fn generate_option_question_mark(&mut self, expr: &Expression, inner_type: &Type) -> Result<String, CursedError> {
        let mut ir = String::new();

        // Generate the option expression
        let option_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Evaluate Option<{}> expression for ? operator\n", 
                            self.type_to_llvm_string(inner_type)?));
        ir.push_str(&format!("  %option_{} = {}\n", option_register, self.generate_expression_ir(expr)?));

        // Create discriminant check
        let discriminant_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Extract Option discriminant (0=None, 1=Some)\n"));
        ir.push_str(&format!("  %option_discriminant_{} = extractvalue {{ i1, [{}] }} %option_{}, 0\n", 
                            discriminant_register, self.calculate_type_size(inner_type)?, option_register));

        // Create branch labels
        let some_label = format!("option_some_{}", self.register_tracker.next_register());
        let none_label = format!("option_none_{}", self.register_tracker.next_register());
        let continue_label = format!("option_continue_{}", self.register_tracker.next_register());

        // Branch on discriminant (false=None, true=Some)
        ir.push_str(&format!("  ; Branch: false=None (early return), true=Some (continue)\n"));
        ir.push_str(&format!("  br i1 %option_discriminant_{}, label %{}, label %{}\n", 
                            discriminant_register, some_label, none_label));

        // Some branch: extract value and continue
        ir.push_str(&format!("{}:\n", some_label));
        let some_value_register = self.register_tracker.next_register();
        ir.push_str(&format!("  ; Extract Some value for continued execution\n"));
        ir.push_str(&format!("  %some_value_raw_{} = extractvalue {{ i1, [{}] }} %option_{}, 1\n", 
                            some_value_register, self.calculate_type_size(inner_type)?, option_register));
        
        let unwrapped_register = self.register_tracker.next_register();
        ir.push_str(&format!("  %unwrapped_option_value_{} = bitcast [{}] %some_value_raw_{} to {}\n", 
                            unwrapped_register, self.calculate_type_size(inner_type)?, 
                            some_value_register, self.type_to_llvm_string(inner_type)?));
        ir.push_str(&format!("  br label %{}\n", continue_label));

        // None branch: early return with None
        ir.push_str(&format!("{}:\n", none_label));
        if let Some(return_type) = &self.function_return_type {
            if self.is_option_type(return_type) {
                let return_none_register = self.register_tracker.next_register();
                ir.push_str(&format!("  ; Create Option::None for early return\n"));
                ir.push_str(&format!("  %return_option_{} = call {} @cursed_create_option_none()\n", 
                                    return_none_register, self.type_to_llvm_string(return_type)?));
                ir.push_str(&format!("  ret {} %return_option_{}\n", 
                                    self.type_to_llvm_string(return_type)?, return_none_register));
            } else {
                return Err(CursedError::General(
                    "Cannot use ? operator on Option in function that doesn't return Option type".to_string()
                ));
            }
        } else {
            return Err(CursedError::General(
                "? operator requires function return type to be known".to_string()
            ));
        }

        // Continue label
        ir.push_str(&format!("{}:\n", continue_label));
        ir.push_str(&format!("  ; Some value available as %unwrapped_option_value_{}\n", unwrapped_register));

        Ok(ir)
    }

    /// Helper: Generate LLVM IR for expression
    fn generate_expression_ir(&self, expr: &Expression) -> Result<String, CursedError> {
        // This would integrate with the main expression compiler
        Ok("call i8* @cursed_evaluate_expression()".to_string())
    }

    /// Helper: Infer type of expression
    fn infer_expression_type(&self, expr: &Expression) -> Result<Type, CursedError> {
        // This would integrate with the type inference system
        // For now, return a sample Result type
        Ok(Type::Result(Box::new(Type::Integer), Box::new(Type::String)))
    }

    /// Helper: Convert CURSED type to LLVM type string
    fn type_to_llvm_string(&self, cursed_type: &Type) -> Result<String, CursedError> {
        match cursed_type {
            Type::Integer => Ok("i32".to_string()),
            Type::String => Ok("i8*".to_string()),
            Type::Boolean => Ok("i1".to_string()),
            Type::Float => Ok("double".to_string()),
            Type::Result(ok_type, err_type) => {
                Ok(format!("{{ i1, [{}] }}", 
                          self.calculate_union_size(ok_type, err_type)?))
            }
            Type::Option(inner_type) => {
                Ok(format!("{{ i1, [{}] }}", 
                          self.calculate_type_size(inner_type)?))
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

    /// Helper: Check if type is a Result type
    fn is_result_type(&self, cursed_type: &Type) -> bool {
        matches!(cursed_type, Type::Result(_, _))
    }

    /// Helper: Check if type is an Option type
    fn is_option_type(&self, cursed_type: &Type) -> bool {
        matches!(cursed_type, Type::Option(_))
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED question mark operator system enabled".to_string())
}
