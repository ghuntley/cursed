//! Code generation for control flow statements (if, while, for)

use inkwell::basic_block::BasicBlock;
use crate::ast::control_flow::{IfStatement, WhileStatement, ForStatement};
use crate::error::Error;
use super::generator::{LlvmCodeGenerator, LoopContext};

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile an if statement to LLVM IR
    pub fn compile_if_statement(
        &mut self, 
        if_stmt: &IfStatement
    ) -> Result<(), Error> {
        // Get the current function
        let current_fn = self.builder().get_insert_block().unwrap().get_parent().unwrap();
        
        // Create blocks for then, else, and merge
        let then_block = self.context().append_basic_block(current_fn, "then");
        let else_block = if if_stmt.alternative.is_some() {
            Some(self.context().append_basic_block(current_fn, "else"))
        } else { None };
        let merge_block = self.context().append_basic_block(current_fn, "ifcont");
        
        // Compile the condition
        let condition_value = self.compile_basic_expression(&*if_stmt.condition)?;
        if !condition_value.is_int_value() {
            return Err(Error::codegen("If condition must be a boolean".to_string()));
        }
        let condition = condition_value.into_int_value();
        
        // Create the conditional branch
        if let Some(else_block) = else_block {
            self.builder().build_conditional_branch(condition, then_block, else_block)
                .map_err(|e| Error::codegen(format!("Failed to build conditional branch: {}", e)))?;
        } else {
            self.builder().build_conditional_branch(condition, then_block, merge_block)
                .map_err(|e| Error::codegen(format!("Failed to build conditional branch: {}", e)))?;
        }
        
        // Compile the then block
        self.builder().position_at_end(then_block);
        for stmt in &if_stmt.consequence.statements {
            self.compile_statement(&**stmt)?;
        }
        
        // Add a branch to the merge block if there wasn't a terminator instruction (like return)
        let current_block = self.builder().get_insert_block().unwrap();
        if current_block.get_terminator().is_none() {
            self.builder().build_unconditional_branch(merge_block)
                .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        }
        
        // Compile the else block if it exists
        if let Some(else_block) = else_block {
            self.builder().position_at_end(else_block);
            
            if let Some(alt) = &if_stmt.alternative {
                for stmt in &alt.statements {
                    self.compile_statement(&**stmt)?;
                }
            }
            
            // Add a branch to the merge block if there wasn't a terminator instruction
            let current_block = self.builder().get_insert_block().unwrap();
            if current_block.get_terminator().is_none() {
                self.builder().build_unconditional_branch(merge_block)
                    .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
            }
        }
        
        // Continue in the merge block
        self.builder().position_at_end(merge_block);
        
        // Add a terminator to the merge block based on function return type
        let fn_ret_type = current_fn.get_type().get_return_type();
        if fn_ret_type.is_none() {
            // Void return type
            self.builder().build_return(None)
                .map_err(|e| Error::codegen(format!("Failed to build void return: {}", e)))?;
        } else {
            // Non-void return type - use appropriate type
            let i32_type = self.context().i32_type();
            self.builder().build_return(Some(&i32_type.const_int(0, false)))
                .map_err(|e| Error::codegen(format!("Failed to build return: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Compile a while statement to LLVM IR
    pub fn compile_while_statement(
        &mut self, 
        while_stmt: &WhileStatement
    ) -> Result<(), Error> {
        // Get the current function
        let current_fn = self.builder().get_insert_block().unwrap().get_parent().unwrap();
        
        // Create blocks for condition, loop body, and after
        let cond_block = self.context().append_basic_block(current_fn, "while.cond");
        let body_block = self.context().append_basic_block(current_fn, "while.body");
        let after_block = self.context().append_basic_block(current_fn, "while.end");
        
        // Branch to the condition block
        self.builder().build_unconditional_branch(cond_block)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Push loop context for break/continue statements
        self.push_loop_context("while")?;
        let loop_context = self.current_loop_context().unwrap().clone();
        let break_block = loop_context.break_block;
        let continue_block = loop_context.continue_block;
        
        // Emit the condition code
        self.builder().position_at_end(cond_block);
        let condition_value = self.compile_basic_expression(&*while_stmt.condition)?;
        if !condition_value.is_int_value() {
            return Err(Error::codegen("While condition must be a boolean".to_string()));
        }
        let condition = condition_value.into_int_value();
        
        // Create the conditional branch
        self.builder().build_conditional_branch(condition, body_block, after_block)
            .map_err(|e| Error::codegen(format!("Failed to build conditional branch: {}", e)))?;
        
        // Emit the body code
        self.builder().position_at_end(body_block);
        for stmt in &while_stmt.body.statements {
            self.compile_statement(&**stmt)?;
        }
        
        // Add a branch back to the continue block if there wasn't a terminator
        let current_block = self.builder().get_insert_block().unwrap();
        if current_block.get_terminator().is_none() {
            self.builder().build_unconditional_branch(continue_block)
                .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        }
        
        // Position at the continue block, which jumps back to the condition
        self.builder().position_at_end(continue_block);
        self.builder().build_unconditional_branch(cond_block)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Position at the break block, which jumps to after
        self.builder().position_at_end(break_block);
        self.builder().build_unconditional_branch(after_block)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Continue in the after block
        self.builder().position_at_end(after_block);
        
        // Add a terminator to the after block based on function return type
        let fn_ret_type = current_fn.get_type().get_return_type();
        if fn_ret_type.is_none() {
            // Void return type
            self.builder().build_return(None)
                .map_err(|e| Error::codegen(format!("Failed to build void return: {}", e)))?;
        } else {
            // Non-void return type - use appropriate type
            let i32_type = self.context().i32_type();
            self.builder().build_return(Some(&i32_type.const_int(0, false)))
                .map_err(|e| Error::codegen(format!("Failed to build return: {}", e)))?;
        }
        
        // Pop the loop context
        self.pop_loop_context();
        
        Ok(())
    }
    
    /// Compile a for statement to LLVM IR
    pub fn compile_for_statement(
        &mut self, 
        for_stmt: &ForStatement
    ) -> Result<(), Error> {
        // Get the current function
        let current_fn = self.builder().get_insert_block().unwrap().get_parent().unwrap();
        
        // Create blocks for initialization, condition, increment, loop body, and after
        let init_block = self.context().append_basic_block(current_fn, "for.init");
        let cond_block = self.context().append_basic_block(current_fn, "for.cond");
        let incr_block = self.context().append_basic_block(current_fn, "for.incr");
        let body_block = self.context().append_basic_block(current_fn, "for.body");
        let after_block = self.context().append_basic_block(current_fn, "for.end");
        
        // Branch to the initialization block
        self.builder().build_unconditional_branch(init_block)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Push loop context for break/continue statements
        self.push_loop_context("for")?;
        let loop_context = self.current_loop_context().unwrap().clone();
        let break_block = loop_context.break_block;
        let continue_block = loop_context.continue_block;
        
        // Emit the initialization code
        self.builder().position_at_end(init_block);
        if let Some(init) = &for_stmt.init {
            self.compile_statement(&**init)?;
        }
        self.builder().build_unconditional_branch(cond_block)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Emit the condition code
        self.builder().position_at_end(cond_block);
        let condition = if let Some(cond) = &for_stmt.condition {
            let cond_value = self.compile_basic_expression(&**cond)?;
            if !cond_value.is_int_value() {
                return Err(Error::codegen("For condition must be a boolean".to_string()));
            }
            cond_value.into_int_value()
        } else {
            // If there's no condition, use true (1)
            self.context().bool_type().const_int(1, false)
        };
        
        // Create the conditional branch
        self.builder().build_conditional_branch(condition, body_block, after_block)
            .map_err(|e| Error::codegen(format!("Failed to build conditional branch: {}", e)))?;
        
        // Emit the body code
        self.builder().position_at_end(body_block);
        for stmt in &for_stmt.body.statements {
            self.compile_statement(&**stmt)?;
        }
        
        // Branch to the continue block if there wasn't a terminator
        let current_block = self.builder().get_insert_block().unwrap();
        if current_block.get_terminator().is_none() {
            self.builder().build_unconditional_branch(continue_block)
                .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        }
        
        // Position at the continue block, which jumps to the increment
        self.builder().position_at_end(continue_block);
        self.builder().build_unconditional_branch(incr_block)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Emit the increment code
        self.builder().position_at_end(incr_block);
        if let Some(post) = &for_stmt.post {
            self.compile_statement(&**post)?;
        }
        self.builder().build_unconditional_branch(cond_block)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Position at the break block, which jumps to after
        self.builder().position_at_end(break_block);
        self.builder().build_unconditional_branch(after_block)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Continue in the after block
        self.builder().position_at_end(after_block);
        
        // Add a terminator to the after block based on function return type
        let fn_ret_type = current_fn.get_type().get_return_type();
        if fn_ret_type.is_none() {
            // Void return type
            self.builder().build_return(None)
                .map_err(|e| Error::codegen(format!("Failed to build void return: {}", e)))?;
        } else {
            // Non-void return type - use appropriate type
            let i32_type = self.context().i32_type();
            self.builder().build_return(Some(&i32_type.const_int(0, false)))
                .map_err(|e| Error::codegen(format!("Failed to build return: {}", e)))?;
        }
        
        // Pop the loop context
        self.pop_loop_context();
        
        Ok(())
    }
}