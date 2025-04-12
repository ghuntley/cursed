//! Code generation for control flow statements in the CURSED language.
//!
//! This module handles the translation of CURSED control flow constructs to LLVM IR:
//! - If statements (`if` in CURSED): Conditional branching with optional else clauses
//! - While loops (`periodt` in CURSED): Condition-controlled loops
//! - For loops (`bestie` in CURSED): Flexible loop construct with initialization, condition, and increment sections
//!
//! The code generator creates appropriate LLVM basic blocks for each part of the control flow
//! and connects them with conditional and unconditional branches. It also manages break and
//! continue targets for loops, enabling proper loop exit and continuation.

use inkwell::basic_block::BasicBlock;
use crate::ast::control_flow::{IfStatement, WhileStatement, ForStatement, SwitchStatement, CaseStatement};
use crate::error::Error;
use super::generator::{LlvmCodeGenerator, LoopContext};

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles an if statement to LLVM IR.
    ///
    /// This method translates a CURSED if statement into a series of LLVM basic blocks:
    /// - A condition evaluation block
    /// - A 'then' block for the consequence code
    /// - An optional 'else' block for alternative code
    /// - A merge ('ifcont') block where control flow reunites
    ///
    /// The method creates appropriate conditional branches based on the condition's
    /// evaluation and ensures proper control flow through all possible paths.
    ///
    /// # Arguments
    ///
    /// * `if_stmt` - The AST if statement node to compile
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Success or a compilation error
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
    
    /// Compiles a while loop ("periodt" in CURSED) to LLVM IR.
    ///
    /// This method translates a CURSED while loop into a set of LLVM basic blocks:
    /// - A condition block that evaluates the loop condition
    /// - A body block that contains the loop's statements
    /// - A continue block for handling continue statements
    /// - A break block for handling break statements
    /// - An after block where execution continues after the loop
    ///
    /// The method sets up proper control flow between these blocks and registers
    /// the loop in the loop context stack to handle break and continue statements.
    ///
    /// # Arguments
    ///
    /// * `while_stmt` - The AST while statement node to compile
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Success or a compilation error
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
    
    /// Compiles a for loop ("bestie" in CURSED) to LLVM IR.
    ///
    /// This method translates a CURSED for loop into a complex set of LLVM basic blocks:
    /// - An initialization block for setting up loop variables
    /// - A condition block that evaluates the loop continuation condition
    /// - An increment block for updating loop variables after each iteration
    /// - A body block containing the loop's statements
    /// - A continue block that targets the increment block
    /// - A break block for exiting the loop
    /// - An after block where execution continues after the loop
    ///
    /// This structure handles all three forms of CURSED for loops:
    /// 1. C-style for loops with init, condition, and increment expressions
    /// 2. Condition-only loops (similar to while loops)
    /// 3. Infinite loops (when no condition is provided)
    ///
    /// # Arguments
    ///
    /// * `for_stmt` - The AST for statement node to compile
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Success or a compilation error
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
    
    /// Compiles a switch statement (vibe_check in CURSED) to LLVM IR.
    ///
    /// This method translates a CURSED vibe_check statement into LLVM's switch instruction
    /// and associated basic blocks. It creates a block for each case and handles the control
    /// flow between them, including fallthrough behavior when no explicit break is present.
    ///
    /// # Arguments
    ///
    /// * `switch_stmt` - The AST switch statement node to compile
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Success or a compilation error
    pub fn compile_switch_statement(
        &mut self, 
        switch_stmt: &SwitchStatement
    ) -> Result<(), Error> {
        // Get the current function
        let current_fn = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        // Create a block for the switch header and the switch end (merge point)
        let switch_header = self.context.append_basic_block(current_fn, "switch.header");
        let switch_end = self.context.append_basic_block(current_fn, "switch.end");
        
        // Branch to the switch header
        self.builder.build_unconditional_branch(switch_header)
            .map_err(|e| Error::codegen(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Position at the switch header block
        self.builder.position_at_end(switch_header);
        
        // Compile the switch value expression
        let switch_value = self.compile_expression(&*switch_stmt.value)?;
        
        // We'll need the current function for string switch in the future
        
        // Handle switch statement based on the type of the value
        if switch_value.is_int_value() {
            // Integer switch case - use LLVM's native switch instruction
            let default_case = match &switch_stmt.default {
                Some(_) => {
                    // Create a block for the default case
                    self.context.append_basic_block(current_fn, "switch.default")
                },
                None => switch_end
            };
            
            // Build the switch instruction with the default case
            let switch_instr = self.builder.build_switch(
                switch_value.into_int_value(), 
                default_case,
                switch_stmt.cases.len() as u32
            ).map_err(|e| Error::codegen(format!("Failed to build switch instruction: {}", e)))?;
            
            // Process case blocks for integer switch
            self.compile_integer_switch_cases(switch_stmt, switch_instr, switch_end)?
            
            // Continue in the merge block
            self.builder.position_at_end(switch_end);
            
            return Ok(());
        } else if switch_value.is_pointer_value() {
            // For string switch cases, we need to use our string_switch implementation
            // Pass the switch statement and string value to the string switch handler
            return self.compile_string_switch_statement(switch_stmt, switch_value.into_pointer_value());
        } else {
            return Err(Error::codegen("Switch value must be an integer or string".to_string()));
        }
    }
            
    /// Helper method to handle integer-based switch statements
    /// 
    /// This method handles the case blocks and default case for an integer switch statement.
    fn compile_integer_switch_cases(
        &mut self,
        switch_stmt: &SwitchStatement,
        switch_instr: inkwell::values::InstructionValue<'ctx>,
        switch_end: BasicBlock<'ctx>
    ) -> Result<(), Error> {
        // Get the current function
        let current_fn = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        // Create a mapping from cases to their basic blocks for fallthrough handling
        let mut case_blocks: Vec<(&CaseStatement, BasicBlock)> = Vec::new();
        
        // Create blocks for each case
        for case in &switch_stmt.cases {
            let case_block = self.context.append_basic_block(current_fn, "switch.case");
            case_blocks.push((case, case_block));
            
            // For each value in this case, add to the switch instruction
            for expr in &case.expressions {
                let case_value = self.compile_expression(&**expr)?;
                
                if case_value.is_int_value() {
                    // Add this case value to the switch instruction
                    switch_instr.add_case(
                        case_value.into_int_value(), 
                        case_block
                    );
                } else {
                    return Err(Error::codegen("Case value must be an integer".to_string()));
                }
            }
        }
        
        // Process the default case if present
        if let Some(default_block) = &switch_stmt.default {
            let default_bb = switch_instr.get_default_dest();
            
            // Position at the default block
            self.builder.position_at_end(default_bb);
            
            // Create a scope for break detection in the default case
            self.push_loop_context("switch")?;
            let loop_context = self.current_loop_context().unwrap().clone();
            let break_block = loop_context.break_block;
            
            // Compile the default block statements
            for stmt in &default_block.statements {
                self.compile_statement(&**stmt)?;
            }
            
            // Pop the loop context
            self.pop_loop_context();
            
            // Add branch to switch end if there's no terminator (like return)
            let current_block = self.builder.get_insert_block().unwrap();
            if current_block.get_terminator().is_none() {
                self.builder.build_unconditional_branch(switch_end)
                    .map_err(|e| Error::codegen(format!("Failed to build default case branch: {}", e)))?;
            }
            
            // Connect any break statements to the switch end
            self.builder.position_at_end(break_block);
            self.builder.build_unconditional_branch(switch_end)
                .map_err(|e| Error::codegen(format!("Failed to build break branch: {}", e)))?;
        }
        
        // Process each case block
        for (i, (case, block)) in case_blocks.iter().enumerate() {
            // Position at this case's block
            self.builder.position_at_end(*block);
            
            // Create a scope for break detection
            self.push_loop_context("switch")?;
            
            // Get the break block for this switch case
            let loop_context = self.current_loop_context().unwrap().clone();
            let break_block = loop_context.break_block;
            
            // Compile the statements for this case
            for stmt in &case.body.statements {
                self.compile_statement(&**stmt)?;
            }
            
            // Pop the loop context after processing the case
            self.pop_loop_context();
            
            // If there's no terminator (like return or break/ghosted)
            let current_block = self.builder.get_insert_block().unwrap();
            if current_block.get_terminator().is_none() {
                // Fallthrough to the next case if there is one, otherwise to the end
                if i + 1 < case_blocks.len() {
                    // Fallthrough to the next case
                    let next_case_block = case_blocks[i + 1].1;
                    self.builder.build_unconditional_branch(next_case_block)
                        .map_err(|e| Error::codegen(format!("Failed to build fallthrough branch: {}", e)))?;
                } else {
                    // No more cases, go to the end
                    self.builder.build_unconditional_branch(switch_end)
                        .map_err(|e| Error::codegen(format!("Failed to build branch to switch end: {}", e)))?;
                }
            }
            
            // Connect any break statements to the switch end
            self.builder.position_at_end(break_block);
            self.builder.build_unconditional_branch(switch_end)
                .map_err(|e| Error::codegen(format!("Failed to build break branch: {}", e)))?;
        }
        
        Ok(())
    }


}