//! Switch statement compilation for LLVM code generation

use inkwell::values::{BasicValueEnum, IntValue};
use crate::ast::control_flow::{SwitchStatement, SwitchCase};
use crate::ast::expressions::StringLiteral;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use super::string_utils::StringUtilsExtension;
use super::statement::StatementCompilation;

/// Trait for switch statement compilation
pub trait SwitchStatementCompilation<'ctx> {
    /// Compile a switch statement
    fn compile_switch_statement(&mut self, stmt: &SwitchStatement) -> Result<(), Error>;
    
    /// Compile a string-based switch statement
    fn compile_string_switch(&mut self, value: BasicValueEnum<'ctx>, cases: &[SwitchCase], default_case: Option<&SwitchCase>) -> Result<(), Error>;
    
    /// Compile an integer-based switch statement
    fn compile_int_switch(&mut self, value: IntValue<'ctx>, cases: &[SwitchCase], default_case: Option<&SwitchCase>) -> Result<(), Error>;
}

impl<'ctx> SwitchStatementCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_switch_statement(&mut self, stmt: &SwitchStatement) -> Result<(), Error> {
        // Compile the switch value
        let switch_value = self.compile_expression(&*stmt.value)?;
        
        // Check if the switch value is a string
        if switch_value.is_pointer_value() && self.is_string_value(switch_value) {
            // Handle string-based switch statement
            let default_case = stmt.default.as_ref();
            return self.compile_string_switch(switch_value, &stmt.cases, default_case);
        }
        
        // If not a string, it must be an integer
        if !switch_value.is_int_value() {
            return Err(Error::codegen("Switch value must be an integer or string".to_string()));
        }
        
        let value_int = switch_value.into_int_value();
        
        // Handle integer-based switch statement
        let default_case = stmt.default.as_ref();
        self.compile_int_switch(value_int, &stmt.cases, default_case)
    }
    
    fn compile_string_switch(&mut self, value: BasicValueEnum<'ctx>, cases: &[SwitchCase], default_case: Option<&SwitchCase>) -> Result<(), Error> {
        // Get the current function
        let function = self.current_function().ok_or_else(|| 
            Error::codegen("No parent function for string switch".to_string())
        )?;
        
        // Create basic blocks for the end of the switch statement
        let end_block = self.context.append_basic_block(function, "switch.end");
        
        // Create basic block for default case
        let default_block = self.context.append_basic_block(function, "switch.default");
        
        // Create basic blocks for each case
        let mut case_blocks = Vec::with_capacity(cases.len());
        for (i, _) in cases.iter().enumerate() {
            let case_block = self.context.append_basic_block(function, &format!("switch.case.{}", i));
            case_blocks.push(case_block);
        }
        
        // Track whether we've seen a fallthrough case
        let mut has_fallthrough = false;
        
        // Compile case comparisons and branch to corresponding case blocks
        for (i, case) in cases.iter().enumerate() {
            // Extract the case value - must be a string literal
            let case_value = match case.value.as_any().downcast_ref::<StringLiteral>() {
                Some(s) => s.value.clone(),
                None => return Err(Error::codegen("Only string literals are supported in switch cases".to_string()))
            };
            
            // Use string comparison function from the module
            let string_equals_fn = self.module.get_function("string_equals")
                .ok_or_else(|| Error::codegen("string_equals function not found in module".to_string()))?;
            
            // Create string literal for comparison
            let case_str_ptr = self.builder.build_global_string_ptr(&case_value, &format!("switch.case.{}.str", i))?;
            
            // Call string comparison function
            let args = &[
                value.into_pointer_value().into(),
                case_str_ptr.as_pointer_value().into()
            ];
            
            let comparison_result = self.builder.build_call(string_equals_fn, args, &format!("string_eq.{}", i))?
                .try_as_basic_value()
                .left()
                .ok_or_else(|| Error::codegen("String equals function returned void".to_string()))?
                .into_int_value();
            
            // Branch to case block if strings match
            let next_block = self.context.append_basic_block(function, &format!("switch.next.{}", i));
            self.builder.build_conditional_branch(comparison_result, case_blocks[i], next_block)?;
            
            // Move to next comparison block
            self.builder.position_at_end(next_block);
        }
        
        // Branch to default if no matches found
        self.builder.build_unconditional_branch(default_block)?;
        
        // Compile case body blocks
        for (i, case) in cases.iter().enumerate() {
            self.builder.position_at_end(case_blocks[i]);
            
            // Compile case body statements
            for stmt in &case.statements {
                self.compile_statement(&**stmt)?;
            }
            
            // If the block doesn't end with a terminator (like return, break, etc.)
            // we need to add a branch to the next case (fallthrough) or end block
            if !self.builder.get_insert_block().unwrap().get_terminator().is_some() {
                // If this is the last case and there's no explicit terminator,
                // we fallthrough to the default case
                if i == cases.len() - 1 {
                    has_fallthrough = true;
                    self.builder.build_unconditional_branch(default_block)?;
                } else {
                    // Fallthrough to the next case
                    has_fallthrough = true;
                    self.builder.build_unconditional_branch(case_blocks[i + 1])?;
                }
            }
        }
        
        // Compile default case
        self.builder.position_at_end(default_block);
        if let Some(default) = default_case {
            // Compile default case body
            for stmt in &default.statements {
                self.compile_statement(&**stmt)?;
            }
        }
        
        // If the block doesn't end with a terminator, branch to end block
        if !self.builder.get_insert_block().unwrap().get_terminator().is_some() {
            self.builder.build_unconditional_branch(end_block)?;
        }
        
        // Position at end block to continue compilation after switch
        self.builder.position_at_end(end_block);
        
        Ok(())
    }
    
    fn compile_int_switch(&mut self, value: IntValue<'ctx>, cases: &[SwitchCase], default_case: Option<&SwitchCase>) -> Result<(), Error> {
        // Get the current function
        let function = self.current_function().ok_or_else(|| 
            Error::codegen("No parent function for int switch".to_string())
        )?;
        
        // Create basic blocks for the end of the switch statement
        let end_block = self.context.append_basic_block(function, "switch.end");
        
        // Create basic block for default case
        let default_block = self.context.append_basic_block(function, "switch.default");
        
        // Create basic blocks for each case
        let mut case_blocks = Vec::with_capacity(cases.len());
        for (i, _) in cases.iter().enumerate() {
            let case_block = self.context.append_basic_block(function, &format!("switch.case.{}", i));
            case_blocks.push(case_block);
        }
        
        // Create case pairs for the switch instruction
        let mut case_pairs = Vec::with_capacity(cases.len());
        
        // Compile case values
        for (i, case) in cases.iter().enumerate() {
            // Compile case value
            let case_value = self.compile_expression(&*case.value)?;
            
            // Case value must be an integer
            if !case_value.is_int_value() {
                return Err(Error::codegen("Case value must be an integer".to_string()));
            }
            
            let case_int = case_value.into_int_value();
            case_pairs.push((case_int, case_blocks[i]));
        }
        
        // Create the LLVM switch instruction with case pairs
        self.builder.build_switch(value, default_block, &case_pairs)?;
        
        // Compile case body blocks
        for (i, case) in cases.iter().enumerate() {
            self.builder.position_at_end(case_blocks[i]);
            
            // Compile case body statements
            for stmt in &case.statements {
                self.compile_statement(&**stmt)?;
            }
            
            // If the block doesn't end with a terminator (like return, break, etc.)
            // we need to add a branch to the next case (fallthrough) or end block
            if !self.builder.get_insert_block().unwrap().get_terminator().is_some() {
                // If this is the last case and there's no explicit terminator,
                // we fallthrough to the default case
                if i == cases.len() - 1 {
                    self.builder.build_unconditional_branch(default_block)?;
                } else {
                    // Fallthrough to the next case
                    self.builder.build_unconditional_branch(case_blocks[i + 1])?;
                }
            }
        }
        
        // Compile default case
        self.builder.position_at_end(default_block);
        if let Some(default) = default_case {
            // Compile default case body
            for stmt in &default.statements {
                self.compile_statement(&**stmt)?;
            }
        }
        
        // If the block doesn't end with a terminator, branch to end block
        if !self.builder.get_insert_block().unwrap().get_terminator().is_some() {
            self.builder.build_unconditional_branch(end_block)?;
        }
        
        // Position at end block to continue compilation after switch
        self.builder.position_at_end(end_block);
        
        Ok(())
    }
}