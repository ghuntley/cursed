//! If expression compilation for LLVM code generation
//!
//! This module handles the compilation of if expressions
//! (if condition {} else {}) to LLVM IR.

use inkwell::values::BasicValueEnum;
use crate::ast::expressions::if_expression::IfExpression;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use super::statement::StatementCompilation;

/// Trait for if expression compilation
pub trait IfExpressionCompilation<'ctx> {
    /// Compile an if expression
    fn compile_if_expression(&mut self, expr: &IfExpression) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> IfExpressionCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, expr), level = "debug")]
    fn compile_if_expression(&mut self, expr: &IfExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Compiling if expression");
        
        // Get the current function
        
        // First check if we already have a current function
        let function = if let Some(f) = self.current_function() {
            f
        } else {
            // Create a dummy function for testing
            let i32_type = self.context().i32_type();
            let fn_type = i32_type.fn_type(&[], false);
            self.module().add_function("__if_expr_dummy", fn_type, None)
        };
        
        // Compile the condition
        let condition = self.compile_expression(&**expr.condition())?;
        
        // Ensure the condition is a boolean value
        let condition_value = if condition.is_int_value() {
            // For non-boolean integers, compare with zero to get a boolean
            let zero = self.context().i32_type().const_int(0, false);
            self.builder().build_int_compare(
                inkwell::IntPredicate::NE,
                condition.into_int_value(),
                zero,
                "if_cond"
            ).map_err(|e| Error::from_str(&format!("Failed to build condition comparison: {}", e)))?
        } else {
            return Err(Error::from_str("If condition must be a boolean or integer value"));
        };
        
        // Create blocks for then, else, and merge
        let then_block = self.context().append_basic_block(function, "if_then");
        let else_block = self.context().append_basic_block(function, "if_else");
        let merge_block = self.context().append_basic_block(function, "if_merge");
        
        // Branch based on the condition
        self.builder().build_conditional_branch(condition_value, then_block, else_block)
            .map_err(|e| Error::from_str(&format!("Failed to build conditional branch: {}", e)))?;
        
        // Build the then block
        self.builder().position_at_end(then_block);
        
        // Create variable scopes for then and else blocks
        use super::variables::VariableScope;
        self.push_scope(VariableScope::new());
        
        // Compile the consequence statements
        let mut then_value = None;
        let consequence = expr.consequence();
        for stmt in consequence {
            // For the last statement, we want to get its value as the block result
            if let Some(last_stmt) = consequence.last() {
                if std::ptr::eq(&**stmt as *const dyn crate::ast::traits::Statement, 
                                &**last_stmt as *const dyn crate::ast::traits::Statement) {
                    // If it's an expression statement, get its value
                    if let Some(expr_stmt) = stmt.as_any().downcast_ref::<crate::ast::statements::ExpressionStatement>() {
                        if let Some(last_expr) = &expr_stmt.expression {
                            then_value = Some(self.compile_expression(last_expr.as_ref())?); 
                        }
                    } else {
                        // For non-expression statements, compile normally
                        self.compile_statement(&**stmt)?;
                    }
                } else {
                    // Not the last statement, compile normally
                    self.compile_statement(&**stmt)?;
                }
            }
        }
        
        // Default value if no expression result
        let then_value = then_value.unwrap_or_else(|| {
            self.context().i32_type().const_int(0, false).into()
        });
        
        // Branch to the merge block
        let then_end_block = self.builder().get_insert_block().unwrap();
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch to merge: {}", e)))?;
        
        // Pop the variable scope for the then block
        self.pop_scope();
        
        // Build the else block
        self.builder().position_at_end(else_block);
        
        // Create a new scope for the else block
        self.push_scope(super::variables::VariableScope::new());
        
        // Compile the alternative statements
        let mut else_value = None;
        if let Some(alternative) = expr.alternative() {
            for stmt in alternative {
                // For the last statement, we want to get its value as the block result
                if let Some(last_stmt) = alternative.last() {
                    if std::ptr::eq(&**stmt as *const dyn crate::ast::traits::Statement, 
                                    &**last_stmt as *const dyn crate::ast::traits::Statement) {
                        // If it's an expression statement, get its value
                        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<crate::ast::statements::ExpressionStatement>() {
                            if let Some(last_expr) = &expr_stmt.expression {
                                else_value = Some(self.compile_expression(last_expr.as_ref())?);
                            }
                        } else {
                            // For non-expression statements, compile normally
                            self.compile_statement(&**stmt)?;
                        }
                    } else {
                        // Not the last statement, compile normally
                        self.compile_statement(&**stmt)?;
                    }
                }
            }
        }
        
        // Default value if no expression result
        let else_value = else_value.unwrap_or_else(|| {
            self.context().i32_type().const_int(0, false).into()
        });
        
        // Branch to the merge block
        let else_end_block = self.builder().get_insert_block().unwrap();
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::from_str(&format!("Failed to build branch to merge: {}", e)))?;
        
        // Pop the variable scope for the else block
        self.pop_scope();
        
        // Build the merge block
        self.builder().position_at_end(merge_block);
        
        // Create a PHI node to merge the results
        // Get the type for the PHI node, using the same type for both branches
        let then_type = then_value.get_type();
        let else_type = else_value.get_type();
        tracing::debug!("PHI node types - then: {:?}, else: {:?}", then_type, else_type);
        
        // Ensure that both branches produce values of compatible types
        // If they don't match exactly, we may need to perform type conversions
        let phi_type = if then_type == else_type {
            then_type
        } else {
            // Handle common type conversion cases
            match (then_value, else_value) {
                // Convert integer types if necessary
                (then, else_val) if then.is_int_value() && else_val.is_int_value() => {
                    let then_int = then.into_int_value();
                    let else_int = else_val.into_int_value();
                    
                    // Choose the wider type for the PHI node
                    let then_width = then_int.get_type().get_bit_width();
                    let else_width = else_int.get_type().get_bit_width();
                    
                    if then_width >= else_width {
                        then_type
                    } else {
                        else_type
                    }
                },
                // Handle other type conversion cases as needed
                // For now, default to the then_type if no conversion is defined
                _ => {
                    tracing::warn!("Branch types don't match: {:?} vs {:?}", then_type, else_type);
                    then_type
                }
            }
        };
        
        let phi = self.builder().build_phi(phi_type, "if_result")
            .map_err(|e| Error::from_str(&format!("Failed to build phi node: {}", e)))?;
        
        // Set up incoming values for the PHI node - we may need to convert types first
        // If the types don't match the phi_type, we need to insert conversion instructions
        let (then_value_converted, else_value_converted) = if then_type == else_type {
            // Same types, no conversion needed
            (then_value, else_value)
        } else {
            // Different types, may need conversion
            match (then_value, else_value) {
                // Convert integer types if necessary
                (then, else_val) if then.is_int_value() && else_val.is_int_value() => {
                    let then_int = then.into_int_value();
                    let else_int = else_val.into_int_value();
                    
                    // Determine which value needs conversion and convert it
                    let (then_conv, else_conv) = if phi_type == then_type {
                        // Convert else to match then
                        let builder_pos = self.builder().get_insert_block().unwrap();
                        self.builder().position_at_end(else_end_block);
                        let else_extended = if then_int.get_type().get_bit_width() > else_int.get_type().get_bit_width() {
                            // Sign extend or zero extend based on signedness (assuming signed here)
                            self.builder().build_int_s_extend(else_int, then_int.get_type(), "extend")
                                .map_err(|e| Error::from_str(&format!("Failed to extend integer: {}", e)))?    
                        } else {
                            // Truncate
                            self.builder().build_int_truncate(else_int, then_int.get_type(), "trunc")
                                .map_err(|e| Error::from_str(&format!("Failed to truncate integer: {}", e)))?    
                        };
                        self.builder().position_at_end(builder_pos);
                        (then.into(), else_extended.into())
                    } else {
                        // Convert then to match else
                        let builder_pos = self.builder().get_insert_block().unwrap();
                        self.builder().position_at_end(then_end_block);
                        let then_extended = if else_int.get_type().get_bit_width() > then_int.get_type().get_bit_width() {
                            // Sign extend or zero extend based on signedness (assuming signed here)
                            self.builder().build_int_s_extend(then_int, else_int.get_type(), "extend")
                                .map_err(|e| Error::from_str(&format!("Failed to extend integer: {}", e)))?    
                        } else {
                            // Truncate
                            self.builder().build_int_truncate(then_int, else_int.get_type(), "trunc")
                                .map_err(|e| Error::from_str(&format!("Failed to truncate integer: {}", e)))?    
                        };
                        self.builder().position_at_end(builder_pos);
                        (then_extended.into(), else_val.into())
                    };
                    (then_conv, else_conv)
                },
                // Handle float conversions if needed
                // ... (similar logic for other types)
                // If no conversion is needed or possible, use as-is
                _ => (then_value, else_value),
            }
        };
        
        phi.add_incoming(&[(&then_value_converted, then_end_block), (&else_value_converted, else_end_block)]);
        
        // Important: returning a BasicValueEnum that we know is valid
        tracing::debug!("PHI value: {:?}", phi.as_basic_value());
        
        Ok(phi.as_basic_value())
    }
}

// Extension methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile an if expression (wrapper function)
    pub fn compile_if_expr(&mut self, expr: &IfExpression) -> Result<BasicValueEnum<'ctx>, String> {
        match self.compile_if_expression(expr) {
            Ok(val) => Ok(val),
            Err(e) => Err(e.to_string())
        }
    }
}