/// LLVM compilation for the question mark operator (?)
/// 
/// This module provides LLVM code generation for the `?` operator,
/// supporting both Result<T, E> and Option<T> types with proper
/// error propagation and early returns.

use crate::ast::expressions::QuestionMarkExpression;
use crate::ast::traits::Expression;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::{CursedError, SourceLocation};
use crate::runtime::error_propagation::ErrorPropagationOperator;
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};

/// Question mark operator compiler for LLVM
pub trait QuestionMarkCompiler {
    /// Compile question mark expression for Result types
    fn compile_result_question_mark(&mut self, expr: &QuestionMarkExpression) -> Result<String, CursedError>;
    
    /// Compile question mark expression for Option types
    fn compile_option_question_mark(&mut self, expr: &QuestionMarkExpression) -> Result<String, CursedError>;
    
    /// Generate error propagation runtime call
    fn generate_error_propagation_call(&mut self, expr: &QuestionMarkExpression) -> Result<String, CursedError>;
}

/// Error propagation runtime integration
pub struct ErrorPropagationRuntime {
    /// Runtime operator instances
    operators: HashMap<String, ErrorPropagationOperator>,
    /// Generated function declarations
    function_declarations: Vec<String>,
    /// Error context tracking
    error_contexts: Vec<ErrorContext>,
}

/// Error context for propagation
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Source location
    pub location: SourceLocation,
    /// Function context
    pub function_name: Option<String>,
    /// Error type information
    pub error_type: String,
}

impl QuestionMarkCompiler for LlvmCodeGenerator {
    #[instrument(skip(self, expr))]
    fn compile_result_question_mark(&mut self, expr: &QuestionMarkExpression) -> Result<String, CursedError> {
        debug!("Compiling Result question mark operator");
        
        // Generate temporary values
        let result_temp = self.next_temp_name();
        let is_ok_temp = self.next_temp_name();
        let value_temp = self.next_temp_name();
        let error_temp = self.next_temp_name();
        
        // Compile the inner expression using the expression compiler
        let inner_expr_ir = self.compile_expression_to_string(expr.expression.as_ref())
            .map_err(|e| CursedError::code_generation_error(e.to_string(), None, None))?;
        
        // Extract is_ok flag from Result
        let extract_is_ok = format!(
            "{} = extractvalue {} {}, 0  ; Extract is_ok flag",
            is_ok_temp, self.get_result_type("i32", "String"), inner_expr_ir
        );
        
        // Create basic blocks for control flow
        let success_block = self.next_block_name("result_success");
        let error_block = self.next_block_name("result_error");
        let merge_block = self.next_block_name("result_merge");
        
        // Branch based on is_ok flag
        let branch_ir = format!(
            "br i1 {}, label %{}, label %{}",
            is_ok_temp, success_block, error_block
        );
        
        // Success block: extract value
        let success_ir = format!(
            "{}:\n  {} = extractvalue {} {}, 1  ; Extract success value\n  br label %{}",
            success_block, value_temp, self.get_result_type("i32", "String"), inner_expr_ir, merge_block
        );
        
        // Error block: extract error and return
        let error_ir = format!(
            "{}:\n  {} = extractvalue {} {}, 1  ; Extract error value\n  {} = call {} @cursed_propagate_result_error({} {}, i32 {}, i32 {})\n  ret {} {}",
            error_block,
            error_temp,
            self.get_result_type("i32", "String"),
            inner_expr_ir,
            result_temp,
            self.get_result_type("i32", "String"),
            self.get_error_type(),
            error_temp,
            expr.location().0,
            expr.location().1,
            self.get_result_type("i32", "String"),
            result_temp
        );
        
        // Merge block
        let merge_ir = format!("{}:", merge_block);
        
        // Combine all IR
        let complete_ir = format!(
            "  {}\n  {}\n{}\n{}\n{}",
            extract_is_ok, branch_ir, success_ir, error_ir, merge_ir
        );
        
        Ok(format!("{}\n{}", inner_expr_ir, complete_ir))
    }
    
    #[instrument(skip(self, expr))]
    fn compile_option_question_mark(&mut self, expr: &QuestionMarkExpression) -> Result<String, CursedError> {
        debug!("Compiling Option question mark operator");
        
        // Generate temporary values
        let option_temp = self.next_temp_name();
        let is_some_temp = self.next_temp_name();
        let value_temp = self.next_temp_name();
        let none_result_temp = self.next_temp_name();
        
        // Compile the inner expression using the expression compiler
        let inner_expr_ir = self.compile_expression_to_string(expr.expression.as_ref())
            .map_err(|e| CursedError::code_generation_error(e.to_string(), None, None))?;
        
        // Extract is_some flag from Option
        let extract_is_some = format!(
            "{} = extractvalue {} {}, 0  ; Extract is_some flag",
            is_some_temp, self.get_option_type("i32"), inner_expr_ir
        );
        
        // Create basic blocks for control flow
        let some_block = self.next_block_name("option_some");
        let none_block = self.next_block_name("option_none");
        let merge_block = self.next_block_name("option_merge");
        
        // Branch based on is_some flag
        let branch_ir = format!(
            "br i1 {}, label %{}, label %{}",
            is_some_temp, some_block, none_block
        );
        
        // Some block: extract value
        let some_ir = format!(
            "{}:\n  {} = extractvalue {} {}, 1  ; Extract some value\n  br label %{}",
            some_block, value_temp, self.get_option_type("i32"), inner_expr_ir, merge_block
        );
        
        // None block: create None result and return
        let none_ir = format!(
            "{}:\n  {} = call {} @cursed_propagate_option_none(i32 {}, i32 {})\n  ret {} {}",
            none_block,
            none_result_temp,
            self.get_option_type("i32"),
            expr.location().0,
            expr.location().1,
            self.get_option_type("i32"),
            none_result_temp
        );
        
        // Merge block
        let merge_ir = format!("{}:", merge_block);
        
        // Combine all IR
        let complete_ir = format!(
            "  {}\n  {}\n{}\n{}\n{}",
            extract_is_some, branch_ir, some_ir, none_ir, merge_ir
        );
        
        Ok(format!("{}\n{}", inner_expr_ir, complete_ir))
    }
    
    #[instrument(skip(self, expr))]
    fn generate_error_propagation_call(&mut self, expr: &QuestionMarkExpression) -> Result<String, CursedError> {
        debug!("Generating error propagation runtime call");
        
        // Determine expression type
        let expr_type = self.infer_expression_type_string(expr.expression.as_ref())
            .map_err(|e| CursedError::code_generation_error(e.to_string(), None, None))?;
        
        if expr_type.starts_with("Result<") {
            self.compile_result_question_mark(expr)
        } else if expr_type.starts_with("Option<") {
            self.compile_option_question_mark(expr)
        } else {
            Err(CursedError::CodeGeneration {
                message: format!("Cannot apply '?' operator to type: {}", expr_type),
                line: Some(expr.location().0),
                column: Some(expr.location().1),
            })
        }
    }
}

impl ErrorPropagationRuntime {
    /// Create new error propagation runtime
    pub fn new() -> Self {
        Self {
            operators: HashMap::new(),
            function_declarations: Vec::new(),
            error_contexts: Vec::new(),
        }
    }
    
    /// Initialize runtime function declarations
    pub fn initialize_runtime_functions(&mut self) -> Vec<String> {
        let mut declarations = Vec::new();
        
        // Result error propagation function
        declarations.push(
            "declare {} @cursed_propagate_result_error({} %error, i32 %line, i32 %column)"
                .replace("{}", "%result_type")
        );
        
        // Option none propagation function
        declarations.push(
            "declare {} @cursed_propagate_option_none(i32 %line, i32 %column)"
                .replace("{}", "%option_type")
        );
        
        // Error context recording function
        declarations.push(
            "declare void @cursed_record_error_context(i32 %line, i32 %column, i8* %function_name)".to_string()
        );
        
        self.function_declarations.extend(declarations.clone());
        declarations
    }
    
    /// Add error context
    pub fn add_error_context(&mut self, context: ErrorContext) {
        self.error_contexts.push(context);
    }
    
    /// Get function declarations
    pub fn get_function_declarations(&self) -> &[String] {
        &self.function_declarations
    }
}

/// Helper implementations for LlvmCodeGenerator - now implemented in main module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_propagation_runtime_creation() {
        let runtime = ErrorPropagationRuntime::new();
        assert!(runtime.operators.is_empty());
        assert!(runtime.function_declarations.is_empty());
    }

    #[test]
    fn test_runtime_function_initialization() {
        let mut runtime = ErrorPropagationRuntime::new();
        let declarations = runtime.initialize_runtime_functions();
        
        assert_eq!(declarations.len(), 3);
        assert!(declarations[0].contains("cursed_propagate_result_error"));
        assert!(declarations[1].contains("cursed_propagate_option_none"));
        assert!(declarations[2].contains("cursed_record_error_context"));
    }

    #[test]
    fn test_error_context_creation() {
        let context = ErrorContext {
            location: SourceLocation::new(10, 5),
            function_name: Some("test_function".to_string()),
            error_type: "String".to_string(),
        };

        assert_eq!(context.location.line, 10);
        assert_eq!(context.location.column, 5);
        assert_eq!(context.function_name, Some("test_function".to_string()));
    }
}
