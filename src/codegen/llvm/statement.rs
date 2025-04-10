//! Statement code generation for LLVM IR

use crate::ast::traits::Statement;
use crate::ast::statements::ExpressionStatement;
use crate::ast::statements::ReturnStatement;
use crate::ast::statements::BlockStatement;
use crate::ast::control_flow::{IfStatement, WhileStatement, ForStatement};
use crate::error::Error;
use super::generator::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a statement to LLVM IR
    pub fn compile_statement(
        &mut self, 
        stmt: &dyn Statement
    ) -> Result<(), Error> {
        // Handle different statement types
        let any = stmt.as_any();
        
        // Expression statement
        if let Some(expr_stmt) = any.downcast_ref::<ExpressionStatement>() {
            // Just compile the expression and discard the result
            if let Some(expr) = &expr_stmt.expression {
                let _ = self.compile_basic_expression(&**expr)?;
            }
            return Ok(());
        }
        
        // Return statement
        if let Some(return_stmt) = any.downcast_ref::<ReturnStatement>() {
            if let Some(return_value) = &return_stmt.return_value {
                let value = self.compile_basic_expression(&**return_value)?;
                self.builder().build_return(Some(&value)).map_err(|e| {
                    Error::codegen(format!("Failed to build return: {}", e))
                })?;
            } else {
                // Return void if no value
                self.builder().build_return(None).map_err(|e| {
                    Error::codegen(format!("Failed to build void return: {}", e))
                })?;
            }
            return Ok(());
        }
        
        // Block statement
        if let Some(block_stmt) = any.downcast_ref::<BlockStatement>() {
            for stmt in &block_stmt.statements {
                self.compile_statement(&**stmt)?;
            }
            return Ok(());
        }
        
        // If statement
        if let Some(if_stmt) = any.downcast_ref::<IfStatement>() {
            return self.compile_if_statement(if_stmt);
        }
        
        // While statement
        if let Some(while_stmt) = any.downcast_ref::<WhileStatement>() {
            return self.compile_while_statement(while_stmt);
        }
        
        // For statement
        if let Some(for_stmt) = any.downcast_ref::<ForStatement>() {
            return self.compile_for_statement(for_stmt);
        }
        
        // Break statement
        if let Some(break_stmt) = any.downcast_ref::<crate::ast::control_flow::BreakStatement>() {
            return self.compile_break_statement(break_stmt);
        }
        
        // Continue statement
        if let Some(continue_stmt) = any.downcast_ref::<crate::ast::control_flow::ContinueStatement>() {
            return self.compile_continue_statement(continue_stmt);
        }
        
        // If we reach here, we don't know how to compile this statement
        Err(Error::codegen(
            format!("Unsupported statement type: {}", stmt.string())
        ))
    }
}