//! Control flow code generation for LLVM IR
//!
//! This module handles code generation for control flow constructs like
//! if statements, loops, and switch statements.

use inkwell::basic_block::BasicBlock;
use inkwell::values::{BasicValueEnum, IntValue};
use inkwell::IntPredicate;
use crate::ast::traits::{Expression, Statement};
use crate::ast::control_flow::{IfStatement, WhileStatement, ForStatement, SwitchStatement};
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use super::statement::StatementCompilation;
use super::variables::VariableHandling;
use super::variables::VariableScope;

/// Control flow implementation
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile an if statement with condition and optional else branch
    ///
    /// This is a lower-level implementation used by the higher-level compile_if_statement
    /// that takes an IfStatement AST node.
    pub fn compile_if_statement(
        &mut self,
        condition: &dyn Expression,
        then_branch: &[Box<dyn Statement>],
        else_branch: Option<&[Box<dyn Statement>]>
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        // First, compile the condition expression
        let condition_value = self.compile_expression(condition)?;
        
        // Make sure the condition is a boolean (i1 in LLVM)
        let condition_bool = if condition_value.is_int_value() {
            // Compare with zero to convert to boolean
            let zero = self.context().bool_type().const_int(0, false);
            self.builder().build_int_compare(
                IntPredicate::NE,
                condition_value.into_int_value(),
                zero,
                "if_cond"
            ).map_err(|e| Error::from_str(&format!("Failed to build condition: {}", e)))?
        } else {
            return Err(Error::from_str("If condition must be a boolean value"));
        };
        
        // Create the basic blocks for the then/else branches
        let function = self.current_function().ok_or_else(|| Error::from_str("If statement outside function"))?;
        let then_block = self.context().append_basic_block(function, "then");
        let else_block = self.context().append_basic_block(function, "else");
        let merge_block = self.context().append_basic_block(function, "if_end");
        
        // Create the conditional branch
        self.builder().build_conditional_branch(condition_bool, then_block, else_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the 'then' block
        self.builder().position_at_end(then_block);
        
        // Create a new variable scope for the 'then' block
        let scope = VariableScope::new();
        self.push_scope(scope);
        
        // Compile the 'then' statements
        for stmt in then_branch {
            // Ignore any return values as we've updated the return type
            self.compile_statement(stmt.as_ref())?
        }
        
        // If we reach here, no early return occurred
        self.pop_scope();
        
        // Branch to the merge block (if not already terminated by a return)
        if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder().build_unconditional_branch(merge_block)
                .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        }
        
        // Build the 'else' block if there is one
        self.builder().position_at_end(else_block);
        
        if let Some(else_stmts) = else_branch {
            // Create a new variable scope for the 'else' block
            let scope = VariableScope::new();
            self.push_scope(scope);
            
            // Compile the 'else' statements
            for stmt in else_stmts {
                // Ignore any return values as we've updated the return type
                self.compile_statement(stmt.as_ref())?
            }
            
            // If we reach here, no early return occurred
            self.pop_scope();
        }
        
        // Branch to the merge block (if not already terminated by a return)
        if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder().build_unconditional_branch(merge_block)
                .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        }
        
        // Build the merge block
        self.builder().position_at_end(merge_block);
        
        // No value is returned from an if statement
        Ok(None)
    }
    
    /// Compile a high-level if statement AST node
    pub fn compile_if_statement_direct(
        &mut self, 
        if_stmt: &IfStatement
    ) -> Result<(), Error> {
        // Extract components from the if statement
        let condition = if_stmt.condition.as_ref();
        let then_branch = &if_stmt.consequence.statements;
        
        // Convert the else branch if present
        let else_branch = if let Some(alt) = &if_stmt.alternative {
            Some(alt.statements.as_slice())
        } else {
            None
        };
        
        // Call the lower-level implementation
        let _ = self.compile_if_statement_low_level(condition, then_branch.as_slice(), else_branch)?;
        
        Ok(())
    }
    
    /// Low-level implementation for condition and branches
    pub fn compile_if_statement_low_level(
        &mut self,
        condition: &dyn Expression,
        then_branch: &[Box<dyn Statement>],
        else_branch: Option<&[Box<dyn Statement>]>
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        // First, compile the condition expression
        let condition_value = self.compile_expression(condition)?;
        
        // Make sure the condition is a boolean (i1 in LLVM)
        let condition_bool = if condition_value.is_int_value() {
            // Compare with zero to convert to boolean
            let zero = self.context().bool_type().const_int(0, false);
            self.builder().build_int_compare(
                IntPredicate::NE,
                condition_value.into_int_value(),
                zero,
                "if_cond"
            ).map_err(|e| Error::from_str(&format!("Failed to build condition: {}", e)))?
        } else {
            return Err(Error::from_str("If condition must be a boolean value"));
        };
        
        // Create the basic blocks for the then/else branches
        let function = self.current_function().ok_or_else(|| Error::from_str("If statement outside function"))?;
        let then_block = self.context().append_basic_block(function, "then");
        let else_block = self.context().append_basic_block(function, "else");
        let merge_block = self.context().append_basic_block(function, "if_end");
        
        // Create the conditional branch
        self.builder().build_conditional_branch(condition_bool, then_block, else_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the 'then' block
        self.builder().position_at_end(then_block);
        
        // Create a new variable scope for the 'then' block
        let scope = VariableScope::new();
        self.push_scope(scope);
        
        // Compile the 'then' statements
        for stmt in then_branch {
            self.compile_statement(stmt.as_ref())?;
        }
        
        // If we reach here, no early return occurred
        self.pop_scope();
        
        // Branch to the merge block (if not already terminated by a return)
        if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder().build_unconditional_branch(merge_block)
                .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        }
        
        // Build the 'else' block if there is one
        self.builder().position_at_end(else_block);
        
        if let Some(else_stmts) = else_branch {
            // Create a new variable scope for the 'else' block
            let scope = VariableScope::new();
            self.push_scope(scope);
            
            // Compile the 'else' statements
            for stmt in else_stmts {
                self.compile_statement(stmt.as_ref())?;
            }
            
            // If we reach here, no early return occurred
            self.pop_scope();
        }
        
        // Branch to the merge block (if not already terminated by a return)
        if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder().build_unconditional_branch(merge_block)
                .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        }
        
        // Build the merge block
        self.builder().position_at_end(merge_block);
        
        // No value is returned from an if statement
        Ok(None)
    }
    
    /// Compile a while statement
    pub fn compile_while_statement(
        &mut self, 
        while_stmt: &WhileStatement
    ) -> Result<(), Error> {
        // Create basic blocks for the loop
        let function = self.current_function().ok_or_else(|| Error::from_str("While statement outside function"))?;
        let cond_block = self.context().append_basic_block(function, "while_cond");
        let body_block = self.context().append_basic_block(function, "while_body");
        let end_block = self.context().append_basic_block(function, "while_end");
        
        // Create a loop context for break/continue
        // Each loop pushes its own context onto the stack
        // When break/continue is encountered, it will use the innermost loop context
        let context = super::LoopContext {
            name: "while".to_string(),
            break_block: end_block,
            continue_block: cond_block,
        };
        self.push_loop_context(context);
        
        // Branch to the condition block
        self.builder().build_unconditional_branch(cond_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the condition block
        self.builder().position_at_end(cond_block);
        
        // Compile the condition expression
        let condition = while_stmt.condition.as_ref();
        let condition_value = self.compile_expression(condition)?;
        
        // Convert the condition to a boolean
        let condition_bool = if condition_value.is_int_value() {
            // Compare with zero to convert to boolean
            let zero = self.context().bool_type().const_int(0, false);
            self.builder().build_int_compare(
                IntPredicate::NE,
                condition_value.into_int_value(),
                zero,
                "while_cond"
            ).map_err(|e| Error::from_str(&format!("Failed to build condition: {}", e)))?
        } else {
            return Err(Error::from_str("While condition must be a boolean value"));
        };
        
        // Create the conditional branch
        self.builder().build_conditional_branch(condition_bool, body_block, end_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the body block
        self.builder().position_at_end(body_block);
        
        // Create a new variable scope for the loop body
        let scope = VariableScope::new();
        self.push_scope(scope);
        
        // Compile the body statements
        for stmt in &while_stmt.body.statements {
            self.compile_statement(&**stmt)?;
        }
        
        // Pop the variable scope
        self.pop_scope();
        
        // Branch back to the condition block (if not already terminated by a return)
        if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder().build_unconditional_branch(cond_block)
                .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        }
        
        // Pop the loop context
        self.pop_loop_context();
        
        // Position at the end block
        self.builder().position_at_end(end_block);
        
        Ok(())
    }
    
    /// Compile a for statement
    pub fn compile_for_statement(
        &mut self, 
        for_stmt: &ForStatement
    ) -> Result<(), Error> {
        // Create basic blocks for the loop
        let function = self.current_function().ok_or_else(|| Error::from_str("For statement outside function"))?;
        let init_block = self.context().append_basic_block(function, "for_init");
        let cond_block = self.context().append_basic_block(function, "for_cond");
        let body_block = self.context().append_basic_block(function, "for_body");
        let post_block = self.context().append_basic_block(function, "for_post");
        let end_block = self.context().append_basic_block(function, "for_end");
        
        // Create a loop context for break/continue
        // Each loop pushes its own context onto the stack
        // When break/continue is encountered, it will use the innermost loop context
        let context = super::LoopContext {
            name: "for".to_string(),
            break_block: end_block,
            continue_block: post_block,
        };
        self.push_loop_context(context);
        
        // Branch to the initialization block
        self.builder().build_unconditional_branch(init_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the initialization block
        self.builder().position_at_end(init_block);
        
        // Create a new variable scope for the entire for loop
        let scope = VariableScope::new();
        self.push_scope(scope);
        
        // Compile the initialization statement
        if let Some(init) = &for_stmt.init {
            self.compile_statement(&**init)?;
        }
        
        // Branch to the condition block
        self.builder().build_unconditional_branch(cond_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the condition block
        self.builder().position_at_end(cond_block);
        
        // Compile the condition expression
        let condition = match &for_stmt.condition {
            Some(cond) => {
                let cond_value = self.compile_expression(&**cond)?;
                
                // Convert the condition to a boolean
                if cond_value.is_int_value() {
                    // Compare with zero to convert to boolean
                    let zero = self.context().bool_type().const_int(0, false);
                    self.builder().build_int_compare(
                        IntPredicate::NE,
                        cond_value.into_int_value(),
                        zero,
                        "for_cond"
                    ).map_err(|e| Error::from_str(&format!("Failed to build condition: {}", e)))?
                } else {
                    return Err(Error::from_str("For condition must be a boolean value"));
                }
            },
            None => {
                // No condition means always true
                self.context().bool_type().const_int(1, false)
            }
        };
        
        // Create the conditional branch
        self.builder().build_conditional_branch(condition, body_block, end_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the body block
        self.builder().position_at_end(body_block);
        
        // Compile the body statements
        for stmt in &for_stmt.body.statements {
            self.compile_statement(&**stmt)?;
        }
        
        // Branch to the post block (if not already terminated by a return)
        if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder().build_unconditional_branch(post_block)
                .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        }
        
        // Build the post block
        self.builder().position_at_end(post_block);
        
        // Compile the post statement
        if let Some(post) = &for_stmt.post {
            self.compile_statement(&**post)?;
        }
        
        // Branch back to the condition block
        self.builder().build_unconditional_branch(cond_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        
        // Build the end block
        self.builder().position_at_end(end_block);
        
        // Pop the variable scope and loop context
        self.pop_scope();
        self.pop_loop_context();
        
        Ok(())
    }
    
    /// Compile a switch statement
    pub fn compile_switch_statement(
        &mut self, 
        switch_stmt: &SwitchStatement
    ) -> Result<(), Error> {
        // Compile the switch value
        let switch_value = self.compile_expression(&*switch_stmt.value)?;
        
        // Check if the switch value is a string
        if switch_value.is_pointer_value() && self.is_string_type(switch_value) {
            // Handle string-based switch statement
            return self.compile_string_switch_statement(switch_stmt, switch_value.into_pointer_value());
        }
        
        // If not a string, it must be an integer
        if !switch_value.is_int_value() {
            return Err(Error::from_str("Switch value must be an integer or string"));
        }
        
        let value_int = switch_value.into_int_value();
        
        // Create basic blocks for each case and default
        let function = self.current_function().ok_or_else(|| Error::from_str("Switch statement outside function"))?;
        
        // Create a block for each case
        let mut case_blocks = Vec::with_capacity(switch_stmt.cases.len());
        for _ in &switch_stmt.cases {
            let case_block = self.context().append_basic_block(function, "switch_case");
            case_blocks.push(case_block);
        }
        
        // Create a default block
        let default_block = self.context().append_basic_block(function, "switch_default");
        
        // Create an end block for the switch
        let end_block = self.context().append_basic_block(function, "switch_end");
        
        // Build a simple comparison-based switch
        let current_block = self.builder().get_insert_block().unwrap();
        
        // Collect case values and blocks for the switch instruction
        let mut case_value_blocks = Vec::new();
        
        // Process each case
        for (i, case) in switch_stmt.cases.iter().enumerate() {
            // Get the case's primary expression (there might be multiple expressions in some languages)
            // Compile the case value
            let case_value = self.compile_expression(&*case.value)?;
            
            if !case_value.is_int_value() {
                return Err(Error::from_str("Case value must be an integer for integer switch"));
            }
            
            let case_int_value = case_value.into_int_value();
            case_value_blocks.push((case_int_value, case_blocks[i]));
        }
        
        // Create the switch instruction
        let switch_instr = self.builder().build_switch(
            value_int, 
            default_block,
            &case_value_blocks
        ).map_err(|e| Error::from_str(&format!("Failed to build switch: {}", e)))?;
        
        // Compile case blocks
        for (i, case) in switch_stmt.cases.iter().enumerate() {
            self.builder().position_at_end(case_blocks[i]);
            
            // Compile all the statements in this case
            let context = super::LoopContext {
                name: "switch".to_string(),
                break_block: end_block,
                continue_block: end_block, // No continue in switch
            };
            self.push_loop_context(context);
            
            for stmt in &case.statements {
                self.compile_statement(&**stmt)?;
            }
            
            self.pop_loop_context();
            
            // If there's no explicit break, fall through to the next case
            if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
                // Last case falls through to the end, others to the next case
                let target = if i < case_blocks.len() - 1 {
                    case_blocks[i + 1]
                } else {
                    end_block
                };
                
                self.builder().build_unconditional_branch(target)
                    .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
            }
        }
        
        // Compile default block
        self.builder().position_at_end(default_block);
        
        // Compile default case statements
        if let Some(default_case) = &switch_stmt.default {
            let context = super::LoopContext {
                name: "switch".to_string(),
                break_block: end_block,
                continue_block: end_block, // No continue in switch
            };
            self.push_loop_context(context);
            
            for stmt in &default_case.statements {
                self.compile_statement(&**stmt)?;
            }
            
            self.pop_loop_context();
        }
        
        // Branch to end if not terminated
        if self.builder().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder().build_unconditional_branch(end_block)
                .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
        }
        
        // Set insertion point to end block
        self.builder().position_at_end(end_block);
        
        Ok(())
    }
    
    /// Helper function to compile integer-based switch statements
    fn compile_integer_switch_cases(
        &mut self,
        switch_stmt: &SwitchStatement,
        switch_instr: inkwell::values::InstructionValue<'ctx>,
        switch_end: BasicBlock<'ctx>
    ) -> Result<(), Error> {
        // For simplicity, just complete and return
        Ok(())
    }
}