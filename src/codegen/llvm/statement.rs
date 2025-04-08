//! LLVM code generation for statements
//! This module handles compiling AST statements to LLVM IR

use inkwell::IntPredicate;
use crate::ast::*;
use crate::lexer::Token;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles a single AST Statement node.
    pub fn compile_statement(&mut self, statement: &dyn crate::ast::Statement) -> Result<(), String> {
        if let Some(expr_stmt) = statement.as_any().downcast_ref::<crate::ast::statements::expressions::ExpressionStatement>() {
            self.compile_expression_statement(expr_stmt)
        } else if let Some(squad_stmt) = statement.as_any().downcast_ref::<crate::ast::declarations::SquadStatement>() {
            self.compile_squad_statement(squad_stmt)
        } else if let Some(let_stmt) = statement.as_any().downcast_ref::<crate::ast::statements::declarations::LetStatement>() {
            self.compile_let_statement(let_stmt)
        /*} else if let Some(facts_stmt) = statement.as_any().downcast_ref::<FactsStatement>() {
            self.compile_facts_statement(facts_stmt)*/
        } else if let Some(return_stmt) = statement.as_any().downcast_ref::<crate::ast::statements::declarations::ReturnStatement>() {
            self.compile_return_statement(return_stmt)
        } else if let Some(if_stmt) = statement.as_any().downcast_ref::<crate::ast::control_flow::IfStatement>() {
            self.compile_if_statement(if_stmt)
        } else if let Some(while_stmt) = statement.as_any().downcast_ref::<crate::ast::control_flow::WhileStatement>() {
            self.compile_while_statement(while_stmt)
        } else if let Some(break_stmt) = statement.as_any().downcast_ref::<crate::ast::control_flow::BreakStatement>() {
            self.compile_break_statement(break_stmt)
        } else if let Some(import_stmt) = statement.as_any().downcast_ref::<crate::ast::statements::declarations::ImportStatement>() {
            self.compile_import_statement(import_stmt)
        /*} else if let Some(later_stmt) = statement.as_any().downcast_ref::<LaterStatement>() {
            self.compile_later_statement(later_stmt)*/
        } else {
             Err(format!("Unsupported statement type: {}", statement.string()))
        }
    }
    
    // Expression statements simply evaluate the expression and discard the result
    fn compile_expression_statement(&mut self, expr_stmt: &crate::ast::statements::expressions::ExpressionStatement) -> Result<(), String> {
        if let Some(expr) = &expr_stmt.expression {
            // Compile the expression, potentially generating instructions
            let _ = self.compile_expression(expr.as_ref())?;
        }
        
        Ok(())
    }
    
    // Let statements declare variables
    fn compile_let_statement(&mut self, let_stmt: &crate::ast::statements::declarations::LetStatement) -> Result<(), String> {
        let var_name = &let_stmt.name.value;

        // Compile the initializer expression
        let rhs_val = match &let_stmt.value {
            Some(expr) => self.compile_expression(expr.as_ref())?,
            None => {
                return Err(format!("Variable '{}' must be initialized", var_name));
            }
        };

        // Determine the LLVM type based on the type annotation
        let llvm_type = if let Some(type_token) = &let_stmt.type_annotation {
            // Use the specific integer type
            if matches!(type_token, Token::Smol | Token::Mid | Token::Normie | Token::Thicc) {
                // Get the appropriate integer type
                let int_type = match type_token {
                    Token::Smol => self.context.i8_type().into(),
                    Token::Mid => self.context.i16_type().into(),
                    Token::Normie => self.context.i32_type().into(),
                    Token::Thicc => self.context.i64_type().into(),
                    _ => unreachable!(),
                };
                
                int_type
            } else {
                // Unsupported type annotation, use the RHS type
                rhs_val.get_type()
            }
        } else {
            // No type annotation, use the RHS type
            rhs_val.get_type()
        };

        // Allocate memory on the stack in the entry block
        let alloca = self.create_entry_block_alloca(llvm_type, var_name);

        // Store the initial value, with potential truncation/extension based on the target type
        if llvm_type.is_int_type() && rhs_val.is_int_value() {
            let int_type = llvm_type.into_int_type();
            let rhs_int = rhs_val.into_int_value();
            
            // Check if we need to truncate or extend
            let converted_int = if rhs_int.get_type().get_bit_width() != int_type.get_bit_width() {
                if rhs_int.get_type().get_bit_width() > int_type.get_bit_width() {
                    // Truncate
                    self.builder.build_int_truncate(rhs_int, int_type, "truncated").unwrap()
                } else {
                    // Sign extend (assuming signed integers)
                    self.builder.build_int_s_extend(rhs_int, int_type, "extended").unwrap()
                }
            } else {
                // Same bit width, no conversion needed
                rhs_int
            };
            
            // Store the value
            self.builder.build_store(alloca, converted_int).unwrap();
        } else {
            // For non-integer types or types that match, just store directly
            self.builder.build_store(alloca, rhs_val).unwrap();
        }

        // Store (Pointer, Type) tuple
        self.variables.insert(var_name.clone(), (alloca, llvm_type));

        Ok(())
    }
    
    // Facts statements declare constants
    fn compile_facts_statement(&mut self, facts_stmt: &crate::ast::statements::declarations::FactsStatement) -> Result<(), String> {
        // FactsStatement is a placeholder with minimal implementation
        // Using a fixed name for compatibility
        let const_name = "facts-placeholder".to_string();

        // Compile the constant value expression
        let rhs_val = self.compile_expression(facts_stmt.value.as_ref())?;
        let llvm_basic_type = rhs_val.get_type();

        // For constants, we create an alloca but mark it internally as immutable
        // Note: LLVM IR doesn't have a true constant concept for local variables
        // The immutability will be enforced at the language level by the parser/semantic analyzer
        let alloca = self.create_entry_block_alloca(llvm_basic_type, &const_name);

        // Store the constant value
        self.builder.build_store(alloca, rhs_val).unwrap();

        // Add to the variables hashmap but we'll track it as a constant internally
        // In a more sophisticated implementation, we might have a separate hashmap for constants
        self.variables.insert(const_name.clone(), (alloca, llvm_basic_type));

        Ok(())
    }
    
    // Return statements
    fn compile_return_statement(&mut self, return_stmt: &crate::ast::statements::declarations::ReturnStatement) -> Result<(), String> {
        // Ensure we're in a function
        if self.current_function.is_none() {
            return Err("Return statement outside of function context".to_string());
        }
        
        let function = self.current_function.unwrap();
        let return_type = function.get_type().get_return_type().unwrap();
        
        // Handle return with a value
        if let Some(return_value) = &return_stmt.return_value {
            let value = self.compile_expression(return_value.as_ref())?;
            
            // Check if the value type matches the function's return type
            if value.get_type() != return_type {
                // For now, only handle i64 to i32 conversion (common for main function)
                if value.is_int_value() && return_type.is_int_type() {
                    let int_val = value.into_int_value();
                    let return_int_type = return_type.into_int_type();
                    let truncated = self.builder.build_int_truncate(
                        int_val, 
                        return_int_type, 
                        "truncated"
                    ).unwrap();
                    self.builder.build_return(Some(&truncated)).unwrap();
                } else {
                    return Err(format!(
                        "Return type mismatch: function returns {:?} but got {:?}",
                        return_type, value.get_type()
                    ));
                }
            } else {
                self.builder.build_return(Some(&value)).unwrap();
            }
        } else {
            // Handle return without a value (void return)
            self.builder.build_return(None).unwrap();
        }
        
        Ok(())
    }
    
    // If statements
    fn compile_if_statement(&mut self, if_stmt: &crate::ast::control_flow::IfStatement) -> Result<(), String> {
        // Compile the condition expression
        let condition_value = self.compile_expression(if_stmt.condition.as_ref())?;
        
        // Convert to a boolean value if needed
        let condition_bool = if condition_value.is_int_value() {
            let int_val = condition_value.into_int_value();
            let zero = int_val.get_type().const_zero();
            self.builder.build_int_compare(IntPredicate::NE, int_val, zero, "if_cond").unwrap()
        } else {
            return Err("If condition must be a boolean or integer value".to_string());
        };
        
        // Get the current function
        let function = self.current_function.expect("If statement outside of function");
        
        // Create the blocks for the then, else, and merge points
        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.append_basic_block(function, "else");
        let merge_block = self.context.append_basic_block(function, "if_merge");
        
        // Branch based on the condition
        self.builder.build_conditional_branch(condition_bool, then_block, else_block).unwrap();
        
        // Emit the 'then' block
        self.builder.position_at_end(then_block);
        self.compile_statement(&if_stmt.consequence)?;
        
        // Branch to the merge block if we haven't already branched elsewhere
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_block).unwrap();
        }
        
        // Emit the 'else' block if it exists
        self.builder.position_at_end(else_block);
        if let Some(alt) = &if_stmt.alternative {
            self.compile_statement(alt)?;
        }
        
        // Branch to the merge block if we haven't already branched elsewhere
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_block).unwrap();
        }
        
        // Set the insertion point to the merge block for subsequent code
        self.builder.position_at_end(merge_block);
        
        Ok(())
    }
    
    // While statements
    fn compile_while_statement(&mut self, while_stmt: &crate::ast::control_flow::WhileStatement) -> Result<(), String> {
        // Get the current function
        let function = self.current_function.expect("While statement outside of function");
        
        // Create the required blocks
        let condition_block = self.context.append_basic_block(function, "while_cond");
        let loop_block = self.context.append_basic_block(function, "while_body");
        let exit_block = self.context.append_basic_block(function, "while_exit");
        
        // Track this loop's exit block for break statements
        self.loop_exit_blocks.push(exit_block);
        
        // Branch to the condition block first
        self.builder.build_unconditional_branch(condition_block).unwrap();
        
        // Compile the condition
        self.builder.position_at_end(condition_block);
        let condition_value = self.compile_expression(while_stmt.condition.as_ref())?;
        
        // Convert to a boolean value if needed
        let condition_bool = if condition_value.is_int_value() {
            let int_val = condition_value.into_int_value();
            let zero = int_val.get_type().const_zero();
            self.builder.build_int_compare(IntPredicate::NE, int_val, zero, "while_cond").unwrap()
        } else {
            return Err("While condition must be a boolean or integer value".to_string());
        };
        
        // Branch based on the condition
        self.builder.build_conditional_branch(condition_bool, loop_block, exit_block).unwrap();
        
        // Compile the loop body
        self.builder.position_at_end(loop_block);
        self.compile_statement(&while_stmt.body)?;
        
        // Jump back to the condition if we haven't exited otherwise
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(condition_block).unwrap();
        }
        
        // Position at the exit block for subsequent code
        self.builder.position_at_end(exit_block);
        
        // Remove this loop's exit block from the stack
        self.loop_exit_blocks.pop();
        
        Ok(())
    }
    
    // Break statements
    fn compile_break_statement(&mut self, _break_stmt: &crate::ast::control_flow::BreakStatement) -> Result<(), String> {
        // Check if we're inside a loop
        if let Some(exit_block) = self.loop_exit_blocks.last() {
            // Branch to the loop's exit block
            self.builder.build_unconditional_branch(*exit_block).unwrap();
            Ok(())
        } else {
            Err("Break statement outside of loop".to_string())
        }
    }
    
    // Import statements
    fn compile_import_statement(&mut self, import_stmt: &crate::ast::statements::declarations::ImportStatement) -> Result<(), String> {
        // For now, just acknowledge the import statement.
        // TODO: Implement actual module loading and symbol resolution.
        println!("Processing import statement for path: {}", import_stmt.path.value);
        if let Some(alias) = &import_stmt.alias {
            println!("  -> with alias: {}", alias.value);
        }
        // Currently, this does nothing semantically.
        Ok(())
    }
    
    // Later (defer) statements
    fn compile_later_statement(&mut self, _later_stmt: &LaterStatement) -> Result<(), String> {
        // This is a placeholder for later statement compilation
        // TODO: Implement deferred execution
        Err("Later (defer) statements not implemented yet".to_string())
    }
}