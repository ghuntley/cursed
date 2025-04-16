//! Continue statement compilation for LLVM code generation
//!
//! This module handles the compilation of continue statements in loops
//! for the CURSED language to LLVM IR.

use crate::ast::control_flow::ContinueStatement;
use crate::error::Error;
use super::context::LlvmCodeGenerator;

/// Trait for continue statement compilation
pub trait ContinueStatementCompilation<'ctx> {
    /// Compile a continue statement
    fn compile_continue_statement(&mut self, stmt: &ContinueStatement) -> Result<(), Error>;
}

impl<'ctx> ContinueStatementCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, stmt), level = "debug")]
    fn compile_continue_statement(&mut self, stmt: &ContinueStatement) -> Result<(), Error> {
        tracing::debug!("Compiling continue statement");
        
        // Get the current loop context
        let loop_context = self.get_current_loop_context()
            .ok_or_else(|| Error::from_str("Continue statement outside of loop"))?;
        
        // Build an unconditional branch to the loop's continue block
        self.builder().build_unconditional_branch(loop_context.continue_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch to loop continue: {}", e)))?;
        
        Ok(())
    }
}

// Loop context is already defined in break_statement.rs, so we don't need to redefine it here