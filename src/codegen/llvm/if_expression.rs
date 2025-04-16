//! If expression compilation for LLVM code generation
//!
//! This module handles the compilation of if expressions
//! (if condition {} else {}) to LLVM IR.

use inkwell::values::BasicValueEnum;
use crate::ast::expressions::IfExpression;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use super::statement::StatementCompilation;

/// Trait for if expression compilation
pub trait IfExpressionCompilation<'ctx> {
    /// Compile an if expression
    fn compile_if_expression(&mut self, expr: &IfExpression) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> IfExpressionCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, expr), level = "debug")]
    fn compile_if_expression(&mut self, expr: &IfExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Compiling if expression");
        
        // Get the current function
        let function = self.current_function()
            .ok_or_else(|| Error::from_str("If expression outside of function"))?;
        
        // Compile the condition
        let condition = self.compile_expression(&*expr.condition)?;
        
        // Ensure the condition is a boolean value
        let condition_value = if condition.is_int_value() {
            // For non-boolean integers, compare with zero to get a boolean
            let zero = self.context().i32_type().const_int(0, false);
            self.builder().build_int_compare(
                inkwell::IntPredicate::NE,
                condition.into_int_value(),
                zero,
                "if_cond"
            ).map_err(|e| Error::from_str(&format!("Failed to build condition comparison: {}", e)))?
        } else {
            return Err(Error::from_str("If condition must be a boolean or integer value"));
        };
        
        // Create blocks for then, else, and merge
        let then_block = self.context().append_basic_block(function, "if_then");
        let else_block = self.context().append_basic_block(function, "if_else");
        let merge_block = self.context().append_basic_block(function, "if_merge");
        
        // Branch based on the condition
        self.builder().build_conditional_branch(condition_value, then_block, else_block)
            .map_err(|e| Error::from_str(&format!("Failed to build conditional branch: {}", e)))?;
        
        // Build the then block
        self.builder().position_at_end(then_block);
        
        // Create variable scopes for then and else blocks
        self.push_scope();
        
        // Compile the consequence statements
        let mut then_value = None;
        for stmt in &expr.consequence {
            // For the last statement, we want to get its value as the block result
            if let Some(last_stmt) = expr.consequence.last() {
                if std::ptr::eq(&**stmt, last_stmt) {
                    // If it's an expression statement, get its value
                    if let Some(expr_stmt) = stmt.as_any().downcast_ref::<crate::ast::statements::ExpressionStatement>() {
                        if let Some(last_expr) = &expr_stmt.expression {
                            then_value = Some(self.compile_expression(last_expr.as_ref())?); 
                        }
                    } else {
                        // For non-expression statements, compile normally
                        self.compile_statement(&**stmt)?;
                    }
                } else {
                    // Not the last statement, compile normally
                    self.compile_statement(&**stmt)?;
                }
            }
        }
        
        // Default value if no expression result
        let then_value = then_value.unwrap_or_else(|| {
            self.context().i32_type().const_int(0, false).into()
        });
        
        // Branch to the merge block
        let then_end_block = self.builder().get_insert_block().unwrap();
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch to merge: {}", e)))?;
        
        // Pop the variable scope for the then block
        self.pop_scope();
        
        // Build the else block
        self.builder().position_at_end(else_block);
        
        // Create a new scope for the else block
        self.push_scope();
        
        // Compile the alternative statements
        let mut else_value = None;
        if let Some(alternative) = &expr.alternative {
            for stmt in alternative {
                // For the last statement, we want to get its value as the block result
                if let Some(last_stmt) = alternative.last() {
                    if std::ptr::eq(&**stmt, last_stmt) {
                        // If it's an expression statement, get its value
                        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<crate::ast::statements::ExpressionStatement>() {
                            if let Some(last_expr) = &expr_stmt.expression {
                                else_value = Some(self.compile_expression(last_expr.as_ref())?);
                            }
                        } else {
                            // For non-expression statements, compile normally
                            self.compile_statement(&**stmt)?;
                        }
                    } else {
                        // Not the last statement, compile normally
                        self.compile_statement(&**stmt)?;
                    }
                }
            }
        }
        
        // Default value if no expression result
        let else_value = else_value.unwrap_or_else(|| {
            self.context().i32_type().const_int(0, false).into()
        });
        
        // Branch to the merge block
        let else_end_block = self.builder().get_insert_block().unwrap();
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch to merge: {}", e)))?;
        
        // Pop the variable scope for the else block
        self.pop_scope();
        
        // Build the merge block
        self.builder().position_at_end(merge_block);
        
        // Create a PHI node to merge the results
        let phi_type = then_value.get_type();
        let phi = self.builder().build_phi(phi_type, "if_result")
            .map_err(|e| Error::from_str(&format!("Failed to build phi node: {}", e)))?;
        
        phi.add_incoming(&[(&then_value, then_end_block), (&else_value, else_end_block)]);
        
        Ok(phi.as_basic_value())
    }
}

// Extension methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Push a new variable scope
    pub fn push_scope(&mut self) {
        use super::variables::VariableScope;
        self.var_scopes.push(VariableScope::new());
    }
    
    /// Pop the current variable scope
    pub fn pop_scope(&mut self) {
        self.var_scopes.pop();
    }
    
    /// Compile an if expression (wrapper function)
    pub fn compile_if_expr(&mut self, expr: &IfExpression) -> Result<BasicValueEnum<'ctx>, String> {
        match self.compile_if_expression(expr) {
            Ok(val) => Ok(val),
            Err(e) => Err(e.to_string())
        }
    }
}