//! Statement code generation for LLVM IR

use crate::ast::traits::Statement;
use crate::ast::statements::ExpressionStatement;
use crate::ast::statements::ReturnStatement;
use crate::ast::statements::BlockStatement;
use crate::ast::statements::declarations::LetStatement;
use crate::ast::declarations::FunctionStatement;
use crate::ast::control_flow::{IfStatement, WhileStatement, ForStatement, SwitchStatement};
use crate::error::Error;
use super::generator::LlvmCodeGenerator;
use super::variables::VariableScope;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a statement to LLVM IR
    pub fn compile_statement(
        &mut self, 
        stmt: &dyn Statement
    ) -> Result<(), Error> {
        // Handle different statement types
        let any = stmt.as_any();
        
        // Function declaration
        if let Some(func_stmt) = any.downcast_ref::<FunctionStatement>() {
            // Handle function declaration
            let name = &func_stmt.name.value;
            
            // Create parameter types - for simplicity, we'll use i32 for all parameters
            let i32_type = self.context().i32_type();
            let param_types: Vec<_> = (0..func_stmt.parameters.len())
                .map(|_| i32_type.into())
                .collect();
            
            // Use i32 as the return type for now
            let function_type = i32_type.fn_type(&param_types, false);
            let function = self.module().add_function(&name, function_type, None);
            
            // Create entry block
            let entry_block = self.context().append_basic_block(function, "entry");
            
            // Save current function and position
            let prev_block = self.builder().get_insert_block();
            
            // Position at the entry block of the new function
            self.builder().position_at_end(entry_block);
            
            // Compile the function body
            for statement in &func_stmt.body.statements {
                self.compile_statement(&**statement)?;
            }
            
            // Add a default return value if there isn't one
            let current_block = self.builder().get_insert_block().unwrap();
            if current_block.get_terminator().is_none() {
                // Return 0 by default
                self.builder().build_return(Some(&i32_type.const_int(0, false)))
                    .map_err(|e| Error::codegen(format!("Failed to add default return: {}", e)))?;
            }
            
            // Restore previous position if any
            if let Some(prev_blk) = prev_block {
                self.builder().position_at_end(prev_blk);
            }
            
            return Ok(());
        }
        
        // Variable declaration (let statement)
        if let Some(let_stmt) = any.downcast_ref::<LetStatement>() {
            // Handle variable declaration using the variables module
            return self.compile_let_statement(let_stmt);
        }
        
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
            // Create a new variable scope for the block
            let scope = VariableScope::new();
            self.push_scope(scope);
            
            // Compile each statement in the block
            for stmt in &block_stmt.statements {
                self.compile_statement(&**stmt)?;
            }
            
            // Pop the variable scope when done with the block
            self.pop_scope();
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
        
        // Switch (vibe_check) statement
        if let Some(switch_stmt) = any.downcast_ref::<SwitchStatement>() {
            return self.compile_switch_statement(switch_stmt);
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