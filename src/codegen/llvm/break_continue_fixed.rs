//! Code generation for break and continue statements, with support for nested loops
//!
//! This updated version ensures that break/continue statements properly target
//! the correct loop in nested loop contexts.

use crate::ast::{BreakStatement, ContinueStatement};
use crate::error::Error;
use crate::ast::traits::Statement;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a break statement to LLVM IR
    pub fn compile_break_statement(
        &mut self, 
        break_stmt: &BreakStatement
    ) -> Result<(), Error> {
        // Get the current loop context
        if let Some(loop_context) = self.current_loop_context() {
            // Branch to the break block
            self.builder().build_unconditional_branch(loop_context.break_block)
                .map_err(|e| Error::codegen(format!("Failed to build break branch: {}", e)))?;
        } else {
            return Err(Error::codegen("Break statement outside of loop".to_string()));
        }
        
        Ok(())
    }
    
    /// Compile a continue statement to LLVM IR
    pub fn compile_continue_statement(
        &mut self, 
        continue_stmt: &ContinueStatement
    ) -> Result<(), Error> {
        // Get the current loop context
        if let Some(loop_context) = self.current_loop_context() {
            // Branch to the continue block
            self.builder().build_unconditional_branch(loop_context.continue_block)
                .map_err(|e| Error::codegen(format!("Failed to build continue branch: {}", e)))?;
        } else {
            return Err(Error::codegen("Continue statement outside of loop".to_string()));
        }
        
        Ok(())
    }
}