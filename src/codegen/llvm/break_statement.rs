//! Break statement compilation for LLVM code generation
//!
//! This module handles the compilation of break statements in loops
//! for the CURSED language to LLVM IR.

use crate::ast::control_flow::BreakStatement;
use crate::error::Error;
use super::context::LlvmCodeGenerator;

/// Trait for break statement compilation
pub trait BreakStatementCompilation<'ctx> {
    /// Compile a break statement
    fn compile_break_statement(&mut self, stmt: &BreakStatement) -> Result<(), Error>;
}

impl<'ctx> BreakStatementCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, stmt), level = "debug")]
    fn compile_break_statement(&mut self, stmt: &BreakStatement) -> Result<(), Error> {
        tracing::debug!("Compiling break statement");
        
        // Get the current loop context
        let loop_context = self.get_current_loop_context()
            .ok_or_else(|| Error::from_str("Break statement outside of loop"))?;
        
        // Build an unconditional branch to the loop's exit block
        self.builder().build_unconditional_branch(loop_context.break_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch to loop exit: {}", e)))?;
        
        Ok(())
    }
}

// Extension methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the current loop context from the stack
    pub fn get_current_loop_context(&self) -> Option<&crate::codegen::llvm::LoopContext<'ctx>> {
        self.loop_contexts.last()
    }
}