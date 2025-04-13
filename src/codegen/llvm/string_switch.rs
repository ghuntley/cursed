//! Code generation for string-based switch statements in CURSED.
//!
//! This module provides support for string comparisons in vibe_check statements.
//! When switch values are strings, this module generates the appropriate comparison
//! code and branching logic to implement the switch statement semantics.

// Note: This module is currently a stub for future implementation
// It will be expanded in later updates to fully support string-based switch statements

use super::LlvmCodeGenerator;
use crate::ast::Expression;
use crate::codegen::llvm::statement::StatementCompilation;
use crate::error::Error;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the current value of the string literal counter
    fn get_string_literal_count(&self) -> usize {
        // Use a default value if the field isn't available in our context
        // This makes the code more robust during development
        0
    }

    /// Increment the string literal counter
    fn increment_string_literal_count(&mut self) {
        // No-op if the field isn't available in our context
    }
    /// Checks if a value is a string type
    ///
    /// This is a basic check to determine if a value could be a string (char* pointer)
    pub fn is_string_type(&self, value: BasicValueEnum<'ctx>) -> bool {
        if !value.is_pointer_value() {
            return false;
        }

        // For now, we'll assume any pointer could be a string
        // In a more robust implementation, we'd track type information
        // to know definitively which pointers are strings
        true
    }

    /// Generates code for a string comparison
    ///
    /// This function creates the LLVM IR to compare two strings for equality.
    /// It generates a call to the strcmp function from the C standard library
    /// and converts the result to a boolean value.
    ///
    /// Returns an IntValue representing a boolean result (1 for equal, 0 for not equal)
    pub fn generate_string_comparison(
        &mut self,
        lhs: PointerValue<'ctx>,
        rhs: PointerValue<'ctx>,
    ) -> Result<IntValue<'ctx>, Error> {
        // Get or declare the strcmp function from the C standard library
        let strcmp_fn = self.get_or_declare_strcmp()?;

        // Call strcmp(lhs, rhs)
        let args = &[lhs.into(), rhs.into()];
        let call_site_value = match self.builder_mut().build_call(strcmp_fn, args, "strcmp_result") {
            Ok(val) => val,
            Err(e) => {
                return Err(Error::codegen(format!(
                "Failed to build strcmp call: {}",
                e
                )))
                }
        };

        let strcmp_result = call_site_value
            .try_as_basic_value()
            .left()
            .ok_or_else(|| Error::codegen("Failed to call strcmp".to_string()))?;

        // strcmp returns 0 when strings are equal, so we need to compare the result with 0
        let strcmp_result_int = strcmp_result.into_int_value();
        let zero = self.context.i32_type().const_zero();

        // Build equality comparison (result == 0)
        let equal = match self.builder_mut().build_int_compare(
            inkwell::IntPredicate::EQ,
            strcmp_result_int,
            zero,
            "strings_equal",
        ) {
            Ok(val) => val,
            Err(e) => return Err(Error::codegen(format!("Failed to build comparison: {}", e))),
        };

        Ok(equal)
    }

    /// Gets or declares the strcmp function from the C standard library
    fn get_or_declare_strcmp(&self) -> Result<inkwell::values::FunctionValue<'ctx>, Error> {
        // Check if strcmp has already been declared in this module
        if let Some(function) = self.module.get_function("strcmp") {
            return Ok(function);
        }

        // Create function type for strcmp: int strcmp(const char*, const char*)
        let i8_ptr_type = self
            .context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::default());
        let i32_type = self.context.i32_type();
        let strcmp_type = i32_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);

        // Add the strcmp function declaration to the module
        let strcmp_fn = self.module.add_function("strcmp", strcmp_type, None);

        Ok(strcmp_fn)
    }

    /// Create a constant string in the module and return a pointer to it
    pub fn create_string_constant(&mut self, value: &str) -> Result<PointerValue<'ctx>, Error> {
        // Create a string constant with null terminator
        let string_val = self.context.const_string(value.as_bytes(), true);

        // Generate a unique name for this string constant
        let string_id = self.get_string_literal_count();
        self.increment_string_literal_count();
        let global_str_name = format!("string_{}", string_id);

        // Create a global variable for the string constant
        let global_str = self
            .module
            .add_global(string_val.get_type(), None, &global_str_name);

        // Initialize the global with our string constant
        global_str.set_initializer(&string_val);

        // Cast the global to a char pointer (i8*)
        // Prepare type first to avoid borrowing context and builder simultaneously
        let ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let global_ptr = global_str.as_pointer_value();
        let name = format!("str_ptr_{}", string_id);
        
        let str_ptr = match self.builder_mut().build_pointer_cast(
            global_ptr,
            ptr_type,
            &name,
        ) {
            Ok(val) => val,
            Err(e) => {
                return Err(Error::codegen(format!(
                    "Failed to build pointer cast: {}",
                    e
                )))
            }
        };

        Ok(str_ptr)
    }

    /// Evaluate a constant expression to extract its string value
    pub fn evaluate_string_expr(
        &mut self,
        expr: &dyn Expression,
    ) -> Result<PointerValue<'ctx>, Error> {
        // For now, we only support string literals
        if let Some(string_lit) = expr.as_any().downcast_ref::<crate::ast::StringLiteral>() {
            return self.create_string_constant(&string_lit.value);
        }

        Err(Error::codegen(
            "Only string literals are supported in switch cases".to_string(),
        ))
    }

    /// Compiles a switch statement with string case values
    ///
    /// This method handles the case where the switch value is a string.
    /// It generates a series of string comparisons for each case and
    /// branches based on the results.
    pub fn compile_string_switch_statement(
        &mut self,
        switch_stmt: &crate::ast::control_flow::SwitchStatement,
        switch_value: PointerValue<'ctx>,
    ) -> Result<(), Error> {
        // For string-based switch statements, we need to:  
        // 1. Create a series of string comparison blocks for each case
        // 2. Chain them together with conditional branches
        // 3. Handle fallthrough and break statements
        
        // Get the current function
        let function = self.builder_mut().get_insert_block()
            .and_then(|block| block.get_parent())
            .ok_or_else(|| Error::codegen("No parent function for string switch".to_string()))?;
        
        // Create basic blocks for the end of the switch statement
        let end_block = self.context.append_basic_block(function, "switch.end");
        
        // Create a default block
        let default_block = self.context.append_basic_block(function, "switch.default");
        
        // Create a block for each case
        let mut case_blocks = Vec::with_capacity(switch_stmt.cases.len());
        for (i, _) in switch_stmt.cases.iter().enumerate() {
            let case_block = self.context.append_basic_block(function, &format!("switch.case.{}", i));
            case_blocks.push(case_block);
        }
        
        // Save the current block before we start branching
        let current_block = self.builder_mut().get_insert_block().unwrap();
        
        // Create a loop context for break statements within the switch
        let loop_ctx = super::LoopContext {
            name: "switch".to_string(),
            break_block: end_block,
            continue_block: end_block, // Continue doesn't make sense in a switch
        };
        self.push_loop_context(loop_ctx);
        
        // We'll need to create a chain of comparison blocks
        let mut comp_blocks = Vec::with_capacity(switch_stmt.cases.len());
        for (i, _) in switch_stmt.cases.iter().enumerate() {
            let comp_block = self.context.append_basic_block(function, &format!("switch.comp.{}", i));
            comp_blocks.push(comp_block);
        }
        
        // Branch from current block to first comparison block
        if !comp_blocks.is_empty() {
            self.builder_mut().build_unconditional_branch(comp_blocks[0])
                .map_err(|e| Error::codegen(format!("Failed to branch to first comparison: {}", e)))?;
        } else {
            // No cases, just branch to default
            self.builder_mut().build_unconditional_branch(default_block)
                .map_err(|e| Error::codegen(format!("Failed to branch to default: {}", e)))?;
        }
        
        // Build the comparison chain
        for (i, case) in switch_stmt.cases.iter().enumerate() {
            // Position at this comparison block
            self.builder_mut().position_at_end(comp_blocks[i]);
            
            // We need to check equality with the case value
            let mut equals = None;
            
            // Evaluate the case value
            let case_expr = &*case.value;
            // Evaluate the string expression
            let case_str = self.evaluate_string_expr(case_expr)?;
            let equal = self.generate_string_comparison(switch_value, case_str)?;
            equals = Some(equal);
            
            // We should always have a case expression by this point
            
            // If equal to any of this case's strings, branch to case block, otherwise continue
            let next_block = if i < comp_blocks.len() - 1 {
                comp_blocks[i + 1]
            } else {
                default_block
            };
            
            self.builder_mut().build_conditional_branch(equals.unwrap(), case_blocks[i], next_block)
                .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        }
        // Now build the case blocks - this is where the actual statements are executed
        for (i, case) in switch_stmt.cases.iter().enumerate() {
            // Build the case block
            self.builder_mut().position_at_end(case_blocks[i]);
            
            // Compile all statements in this case
            for stmt in &case.statements {
                self.compile_statement(&**stmt)?;
            }
            
            // If no terminator (no break), fall through to next case or end
            if self.builder_mut().get_insert_block().unwrap().get_terminator().is_none() {
                let target = if i < case_blocks.len() - 1 {
                    case_blocks[i + 1]
                } else {
                    end_block
                };
                
                self.builder_mut().build_unconditional_branch(target)
                    .map_err(|e| Error::codegen(format!("Failed to build fallthrough: {}", e)))?;
            }
        }
        
        // Default block
        self.builder_mut().position_at_end(default_block);
        
        // Compile default case statements if they exist
        if let Some(default_case) = &switch_stmt.default {
            for stmt in &default_case.statements {
                self.compile_statement(&**stmt)?;
            }
        }
        
        // Branch from default to end if not already terminated
        if self.builder_mut().get_insert_block().unwrap().get_terminator().is_none() {
            self.builder_mut().build_unconditional_branch(end_block)
                .map_err(|e| Error::codegen(format!("Failed to build default branch: {}", e)))?;
        }
        
        // Position at the end block for continued code generation
        self.builder_mut().position_at_end(end_block);
        
        // Pop the loop context
        self.pop_loop_context();
        
        Ok(())
    }
    
    // No need for the stub helper anymore since we now call compile_statement directly
}
