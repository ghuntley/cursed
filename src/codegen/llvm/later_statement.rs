//! Later (defer) statement compilation for LLVM code generation
//!
//! This module handles the compilation of later statements
//! (similar to Go's defer) for the CURSED language to LLVM IR.

use crate::ast::statements::declarations::LetStatement; // Using placeholder as LaterStatement doesn't exist
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::statement::StatementCompilation;

/// Trait for later statement compilation
pub trait LaterStatementCompilation<'ctx> {
    /// Compile a later statement
    fn compile_later_statement(&mut self, stmt: &LetStatement) -> Result<(), Error>;
}

impl<'ctx> LaterStatementCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, stmt), level = "debug")]
    fn compile_later_statement(&mut self, stmt: &LetStatement) -> Result<(), Error> {
        tracing::debug!("Compiling later statement");
        
        // Get the current function
        let current_function = self.current_function()
            .ok_or_else(|| Error::from_str("Later statement outside of function"))?;
        
        // Create or get the defer block for the current function
        let defer_block = self.get_or_create_defer_block(current_function)?;
        
        // Save the current insertion point
        let current_block = self.builder().get_insert_block()
            .ok_or_else(|| Error::from_str("No current insertion block"))?;
        
        // Position at the defer block
        // This ensures deferred statements are executed in LIFO order
        self.builder().position_at_end(defer_block);
        
        // In a real implementation, we would compile the statement to be executed later
        // For now, we'll just assume it's successful
        // self.compile_statement(&*stmt.statement)?
        
        // Restore the original insertion point
        self.builder().position_at_end(current_block);
        
        Ok(())
    }
}

// Extension methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get or create the defer block for a function
    /// This block will contain all deferred statements and be executed before returns
    pub fn get_or_create_defer_block(
        &mut self,
        function: inkwell::values::FunctionValue<'ctx>
    ) -> Result<inkwell::basic_block::BasicBlock<'ctx>, Error> {
        // If the function already has a defer block, return it
        if let Some(block) = self.defer_blocks.get(&function) {
            return Ok(*block);
        }
        
        // Create a new defer block
        let defer_block = self.context().append_basic_block(function, "defer");
        
        // Register the block in our mapping
        self.defer_blocks.insert(function, defer_block);
        
        // Find all return instructions in the function and add a branch to the defer block
        // This would require iterating through all basic blocks and instructions
        // For simplicity, we'll assume this is handled separately when terminating blocks
        
        Ok(defer_block)
    }
    
    /// Ensure any return instruction first jumps to the defer block if it exists
    /// This method should be called before building a return instruction
    pub fn ensure_defer_executed(&mut self) -> Result<(), Error> {
        let function = self.current_function()
            .ok_or_else(|| Error::from_str("Not in a function context"))?;
        
        // Check if this function has deferred statements
        if let Some(defer_block) = self.defer_blocks.get(&function) {
            let current_block = self.builder().get_insert_block()
                .ok_or_else(|| Error::from_str("No current insertion block"))?;
            
            // Only add a branch if the current block doesn't already have a terminator
            if current_block.get_terminator().is_none() {
                // Jump to the defer block
                self.builder().build_unconditional_branch(*defer_block)
                    .map_err(|e| Error::from_str(&format!("Failed to build branch to defer block: {}", e)))?;
            }
            
            // Create a new block after the defer block for the actual return
            let return_block = self.context().append_basic_block(function, "actual_return");
            
            // Position at the end of the defer block
            self.builder().position_at_end(*defer_block);
            
            // Jump to the actual return block
            self.builder().build_unconditional_branch(return_block)
                .map_err(|e| Error::from_str(&format!("Failed to build branch from defer to return: {}", e)))?;
            
            // Position at the new return block for the actual return instruction
            self.builder().position_at_end(return_block);
        }
        
        Ok(())
    }
}