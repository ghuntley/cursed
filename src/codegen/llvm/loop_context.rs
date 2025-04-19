//! Loop context management for LLVM code generation
//!
//! This module provides functionality for managing loop contexts
//! to support break and continue statements in loops.

use inkwell::basic_block::BasicBlock;
use super::context::LlvmCodeGenerator;
use super::LoopContext;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Replace the current loop's exit block with a new one and return the old one
    pub fn replace_loop_exit(&mut self, new_block: Option<BasicBlock<'ctx>>) -> Option<BasicBlock<'ctx>> {
        if let Some(loop_context) = self.loop_contexts.last_mut() {
            // If we have a loop context, update its break_block
            if let Some(new_break) = new_block {
                let old_break = loop_context.break_block;
                loop_context.break_block = new_break;
                Some(old_break)
            } else {
                None
            }
        } else {
            // If we don't have a loop context, just return None
            None
        }
    }

    /// Replace the current loop's continue block with a new one and return the old one
    pub fn replace_loop_continue(&mut self, new_block: Option<BasicBlock<'ctx>>) -> Option<BasicBlock<'ctx>> {
        if let Some(loop_context) = self.loop_contexts.last_mut() {
            // If we have a loop context, update its continue_block
            if let Some(new_continue) = new_block {
                let old_continue = loop_context.continue_block;
                loop_context.continue_block = new_continue;
                Some(old_continue)
            } else {
                None
            }
        } else {
            // If we don't have a loop context, just return None
            None
        }
    }

    /// Create and push a new loop context
    pub fn create_loop_context(
        &mut self,
        name: &str,
        break_block: BasicBlock<'ctx>,
        continue_block: BasicBlock<'ctx>
    ) {
        let context = LoopContext {
            name: name.to_string(),
            break_block,
            continue_block,
        };
        self.loop_contexts.push(context);
    }

    /// Get the current loop context if any
    pub fn current_loop_context(&self) -> Option<&LoopContext<'ctx>> {
        self.loop_contexts.last()
    }

    /// Remove and return the current loop context
    pub fn pop_loop_context(&mut self) -> Option<LoopContext<'ctx>> {
        self.loop_contexts.pop()
    }
}