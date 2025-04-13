//! Statement code generation for LLVM IR
//!
//! This module handles code generation for statements in the CURSED language,
//! translating AST statement nodes into LLVM IR instructions.

use crate::ast::traits::Statement;
use crate::ast::statements::ExpressionStatement;
use crate::ast::statements::ReturnStatement;
use crate::ast::statements::BlockStatement;
use crate::ast::statements::declarations::LetStatement;
use crate::ast::FunctionStatement;
use crate::ast::control_flow::{IfStatement, WhileStatement, ForStatement, SwitchStatement};
use crate::ast::control_flow::{BreakStatement, ContinueStatement};
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::variables::VariableScope;
use super::function_monomorphization::FunctionMonomorphization;
use super::variables::VariableHandling;
use super::expression::ExpressionCompilation;

/// Trait for statement compilation functionality
pub trait StatementCompilation<'ctx> {
    /// Compile a statement to LLVM IR
    fn compile_statement(
        &mut self, 
        stmt: &dyn Statement
    ) -> Result<(), Error>;
}

impl<'ctx> StatementCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    /// Compile a statement to LLVM IR
    fn compile_statement(
        &mut self, 
        stmt: &dyn Statement
    ) -> Result<(), Error> {
        // We need to handle the statement compilation directly here instead of
        // calling back to compile_statement_internal to avoid circular references
        
        // Otherwise, handle different statement types
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
                    .map_err(|e| Error::from_str(&format!("Failed to add default return: {}", e)))?;
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
                let _ = self.compile_expression(&**expr)?;
            }
            return Ok(());
        }
        
        // Return statement
        if let Some(return_stmt) = any.downcast_ref::<ReturnStatement>() {
            if let Some(return_value) = &return_stmt.return_value {
                let value = self.compile_expression(&**return_value)?;
                self.builder().build_return(Some(&value)).map_err(|e| {
                    Error::from_str(&format!("Failed to build return: {}", e))
                })?;
            } else {
                // Return void if no value
                self.builder().build_return(None).map_err(|e| {
                    Error::from_str(&format!("Failed to build void return: {}", e))
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
            self.compile_if_statement_wrapper(if_stmt)?;
            return Ok(());
        }
        
        // While statement
        if let Some(while_stmt) = any.downcast_ref::<WhileStatement>() {
            self.compile_while_statement_wrapper(while_stmt)?;
            return Ok(());
        }
        
        // For statement
        if let Some(for_stmt) = any.downcast_ref::<ForStatement>() {
            self.compile_for_statement_wrapper(for_stmt)?;
            return Ok(());
        }
        
        // Switch (vibe_check) statement
        if let Some(switch_stmt) = any.downcast_ref::<SwitchStatement>() {
            self.compile_switch_statement_wrapper(switch_stmt)?;
            return Ok(());
        }
        
        // Break statement
        if let Some(break_stmt) = any.downcast_ref::<BreakStatement>() {
            self.compile_break_statement(break_stmt)?;
            return Ok(());
        }
        
        // Continue statement
        if let Some(continue_stmt) = any.downcast_ref::<ContinueStatement>() {
            self.compile_continue_statement(continue_stmt)?;
            return Ok(());
        }
        
        // If we reach here, we don't know how to compile this statement
        Err(Error::from_str(
            &format!("Unsupported statement type: {}", stmt.string())
        ))
    }
}

// Extension methods for control flow
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Wrapper methods to avoid collisions with control_flow.rs
    
    /// Compile an if statement (wrapper for control_flow implementation)
    pub fn compile_if_statement_wrapper(&mut self, if_stmt: &IfStatement) -> Result<(), Error> {
        // Call the implementation from control_flow module
        // This is just a stub for now
        println!("DEBUG: Compiling if statement (placeholder)");
        Ok(())
    }
    
    /// Compile a while statement (wrapper for control_flow implementation)
    pub fn compile_while_statement_wrapper(&mut self, while_stmt: &WhileStatement) -> Result<(), Error> {
        // Call the implementation from control_flow module
        // This is just a stub for now
        println!("DEBUG: Compiling while statement (placeholder)");
        Ok(())
    }
    
    /// Compile a for statement (wrapper for control_flow implementation)
    pub fn compile_for_statement_wrapper(&mut self, for_stmt: &ForStatement) -> Result<(), Error> {
        // Call the implementation from control_flow module
        // This is just a stub for now
        println!("DEBUG: Compiling for statement (placeholder)");
        Ok(())
    }
    
    /// Compile a switch statement (wrapper for control_flow implementation)
    pub fn compile_switch_statement_wrapper(&mut self, switch_stmt: &SwitchStatement) -> Result<(), Error> {
        // Call the implementation from control_flow module
        println!("DEBUG: Compiling switch statement");
        use super::control_flow; // Import needed methods
        
        // Call the actual implementation
        match self.compile_switch_statement(switch_stmt) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}