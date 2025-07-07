//! Inkwell-based Statement Generator for CURSED
//! 
//! This module provides type-safe statement compilation using the inkwell
//! LLVM bindings instead of string-based IR generation.

use crate::ast::{Statement, Expression, Type};
use crate::error::CursedError;
use crate::codegen::llvm::inkwell_expression_compiler::InkwellExpressionCompiler;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, BasicValue, IntValue, FloatValue, PointerValue, FunctionValue};
use inkwell::types::{BasicTypeEnum, BasicType, IntType, FloatType};
use inkwell::basic_block::BasicBlock;
use inkwell::{AddressSpace, IntPredicate, FloatPredicate};
use std::collections::HashMap;

/// Inkwell-based statement generator for CURSED statements to LLVM IR
pub struct InkwellStatementGenerator<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    
    /// LLVM module
    module: Module<'ctx>,
    
    /// LLVM IR builder
    builder: Builder<'ctx>,
    
    /// Expression compiler
    expression_compiler: InkwellExpressionCompiler<'ctx>,
    
    /// Current function context
    current_function: Option<FunctionValue<'ctx>>,
    
    /// Variable storage mapping (variable name -> alloca pointer)
    variables: HashMap<String, PointerValue<'ctx>>,
    
    /// Loop label stack for break/continue
    loop_stack: Vec<LoopContext<'ctx>>,
}

/// Context for loop constructs (for break/continue support)
#[derive(Debug, Clone)]
struct LoopContext<'ctx> {
    /// Continue block (loop header)
    continue_block: BasicBlock<'ctx>,
    /// Break block (loop exit)
    break_block: BasicBlock<'ctx>,
    /// Optional loop label
    label: Option<String>,
}

impl<'ctx> InkwellStatementGenerator<'ctx> {
    /// Create a new inkwell statement generator
    pub fn new(
        context: &'ctx Context,
        module_name: &str,
    ) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let expression_compiler = InkwellExpressionCompiler::new(context, &builder);
        
        Self {
            context,
            module,
            builder,
            expression_compiler,
            current_function: None,
            variables: HashMap::new(),
            loop_stack: Vec::new(),
        }
    }

    /// Get the LLVM module
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Set the current function context
    pub fn set_current_function(&mut self, function: FunctionValue<'ctx>) {
        self.current_function = Some(function);
        self.expression_compiler.set_current_function(function);
    }

    /// Clear the current function context
    pub fn clear_current_function(&mut self) {
        self.current_function = None;
        self.variables.clear();
        self.expression_compiler.clear_scope();
        self.loop_stack.clear();
    }

    /// Add a variable to the scope
    pub fn add_variable(&mut self, name: String, alloca: PointerValue<'ctx>) {
        self.variables.insert(name.clone(), alloca);
        self.expression_compiler.add_variable(name, alloca);
    }

    /// Compile a statement to LLVM IR
    pub fn compile_statement(&mut self, statement: &Statement) -> Result<(), CursedError> {
        match statement {
            Statement::Expression(expr) => {
                // Compile expression and ignore result
                self.expression_compiler.compile_expression(expr)?;
                Ok(())
            },
            Statement::Let(let_stmt) => {
                self.compile_let_statement(let_stmt)
            },
            Statement::Assignment(assign_stmt) => {
                self.compile_assignment_statement(assign_stmt)
            },
            Statement::ShortDeclaration(short_decl) => {
                self.compile_short_declaration(short_decl)
            },
            Statement::Return(return_stmt) => {
                self.compile_return_statement(return_stmt)
            },
            Statement::If(if_stmt) => {
                self.compile_if_statement(if_stmt)
            },
            Statement::While(while_stmt) => {
                self.compile_while_statement(while_stmt)
            },
            Statement::For(for_stmt) => {
                self.compile_for_statement(for_stmt)
            },
            Statement::Break(break_stmt) => {
                self.compile_break_statement(break_stmt)
            },
            Statement::Continue(continue_stmt) => {
                self.compile_continue_statement(continue_stmt)
            },
            Statement::Increment(inc_stmt) => {
                self.compile_increment_statement(inc_stmt)
            },
            Statement::Decrement(dec_stmt) => {
                self.compile_decrement_statement(dec_stmt)
            },
            _ => {
                Err(CursedError::CompilerError(format!("Unsupported statement type: {:?}", statement)))
            }
        }
    }

    /// Compile let statement (variable declaration with initialization)
    fn compile_let_statement(&mut self, let_stmt: &crate::ast::LetStatement) -> Result<(), CursedError> {
        match &let_stmt.target {
            crate::ast::LetTarget::Single(name) => {
                // Compile the initialization value
                let init_value = self.expression_compiler.compile_expression(&let_stmt.value)?;
                
                // Determine the variable type
                let var_type = if let Some(type_annotation) = &let_stmt.var_type {
                    self.convert_cursed_type_to_llvm(type_annotation)?
                } else {
                    // Infer type from the initialization value
                    self.infer_type_from_value(init_value)?
                };
                
                // Create alloca for the variable
                let alloca = self.builder.build_alloca(var_type, name)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to allocate variable '{}': {:?}", name, e)))?;
                
                // Store the initialization value
                self.builder.build_store(alloca, init_value)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to store initial value for '{}': {:?}", name, e)))?;
                
                // Register the variable
                self.variables.insert(name.clone(), alloca);
                self.expression_compiler.add_variable(name.clone(), alloca);
                
                Ok(())
            },
            crate::ast::LetTarget::Tuple(names) => {
                // Handle tuple destructuring
                let tuple_value = self.expression_compiler.compile_expression(&let_stmt.value)?;
                
                // For now, assume it's a struct-like tuple and extract fields
                for (index, var_name) in names.iter().enumerate() {
                    // Create alloca for each variable
                    let var_type = self.context.i32_type(); // Default to i32 for now
                    let alloca = self.builder.build_alloca(var_type, var_name)
                        .map_err(|e| CursedError::CompilerError(format!("Failed to allocate tuple variable '{}': {:?}", var_name, e)))?;
                    
                    // Extract the field value (simplified - real implementation would handle tuple types properly)
                    let field_value = self.context.i32_type().const_int(index as u64, false); // Placeholder
                    
                    self.builder.build_store(alloca, field_value)
                        .map_err(|e| CursedError::CompilerError(format!("Failed to store tuple field for '{}': {:?}", var_name, e)))?;
                    
                    // Register the variable
                    self.variables.insert(var_name.clone(), alloca);
                    self.expression_compiler.add_variable(var_name.clone(), alloca);
                }
                
                Ok(())
            }
        }
    }

    /// Compile assignment statement
    fn compile_assignment_statement(&mut self, assign_stmt: &crate::ast::AssignmentStatement) -> Result<(), CursedError> {
        match &assign_stmt.target {
            crate::ast::AssignmentTarget::Single(name) => {
                // Get the variable
                let var_ptr = self.variables.get(name)
                    .ok_or_else(|| CursedError::CompilerError(format!("Undefined variable in assignment: {}", name)))?
                    .clone();
                
                // Compile the new value
                let new_value = self.expression_compiler.compile_expression(&assign_stmt.value)?;
                
                // Store the new value
                self.builder.build_store(var_ptr, new_value)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to store assignment value for '{}': {:?}", name, e)))?;
                
                Ok(())
            },
            crate::ast::AssignmentTarget::Tuple(names) => {
                // Handle tuple destructuring assignment
                let tuple_value = self.expression_compiler.compile_expression(&assign_stmt.value)?;
                
                for (index, var_name) in names.iter().enumerate() {
                    let var_ptr = self.variables.get(var_name)
                        .ok_or_else(|| CursedError::CompilerError(format!("Undefined variable in tuple assignment: {}", var_name)))?
                        .clone();
                    
                    // Extract field value (simplified)
                    let field_value = self.context.i32_type().const_int(index as u64, false); // Placeholder
                    
                    self.builder.build_store(var_ptr, field_value)
                        .map_err(|e| CursedError::CompilerError(format!("Failed to store tuple assignment for '{}': {:?}", var_name, e)))?;
                }
                
                Ok(())
            }
        }
    }

    /// Compile short declaration (:= operator)
    fn compile_short_declaration(&mut self, short_decl: &crate::ast::ShortDeclarationStatement) -> Result<(), CursedError> {
        match &short_decl.target {
            crate::ast::ShortDeclarationTarget::Single(name) => {
                // Compile the initialization value
                let init_value = self.expression_compiler.compile_expression(&short_decl.value)?;
                
                // Infer type from the value
                let var_type = self.infer_type_from_value(init_value)?;
                
                // Create alloca for the variable
                let alloca = self.builder.build_alloca(var_type, name)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to allocate short declaration variable '{}': {:?}", name, e)))?;
                
                // Store the initialization value
                self.builder.build_store(alloca, init_value)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to store short declaration value for '{}': {:?}", name, e)))?;
                
                // Register the variable
                self.variables.insert(name.clone(), alloca);
                self.expression_compiler.add_variable(name.clone(), alloca);
                
                Ok(())
            },
            crate::ast::ShortDeclarationTarget::Tuple(names) => {
                // Handle tuple destructuring short declaration
                let tuple_value = self.expression_compiler.compile_expression(&short_decl.value)?;
                
                for (index, var_name) in names.iter().enumerate() {
                    // Create alloca for each variable
                    let var_type = self.context.i32_type(); // Default to i32 for now
                    let alloca = self.builder.build_alloca(var_type, var_name)
                        .map_err(|e| CursedError::CompilerError(format!("Failed to allocate tuple short declaration variable '{}': {:?}", var_name, e)))?;
                    
                    // Extract field value (simplified)
                    let field_value = self.context.i32_type().const_int(index as u64, false); // Placeholder
                    
                    self.builder.build_store(alloca, field_value)
                        .map_err(|e| CursedError::CompilerError(format!("Failed to store tuple short declaration for '{}': {:?}", var_name, e)))?;
                    
                    // Register the variable
                    self.variables.insert(var_name.clone(), alloca);
                    self.expression_compiler.add_variable(var_name.clone(), alloca);
                }
                
                Ok(())
            }
        }
    }

    /// Compile return statement
    fn compile_return_statement(&mut self, return_stmt: &crate::ast::ReturnStatement) -> Result<(), CursedError> {
        if let Some(value_expr) = &return_stmt.value {
            // Return with value
            let return_value = self.expression_compiler.compile_expression(value_expr)?;
            self.builder.build_return(Some(&return_value))
                .map_err(|e| CursedError::CompilerError(format!("Failed to build return instruction: {:?}", e)))?;
        } else {
            // Return void
            self.builder.build_return(None)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build void return instruction: {:?}", e)))?;
        }
        
        Ok(())
    }

    /// Compile if statement with proper basic block handling
    fn compile_if_statement(&mut self, if_stmt: &crate::ast::IfStatement) -> Result<(), CursedError> {
        let current_function = self.current_function
            .ok_or_else(|| CursedError::CompilerError("No current function for if statement".to_string()))?;

        // Compile condition
        let condition_val = self.expression_compiler.compile_expression(&if_stmt.condition)?;
        
        // Convert condition to i1 if necessary
        let condition_bool = match condition_val {
            BasicValueEnum::IntValue(int_val) => {
                // Compare with zero to get boolean
                let zero = self.context.i32_type().const_zero();
                self.builder.build_int_compare(IntPredicate::NE, int_val, zero, "cond")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build condition comparison: {:?}", e)))?
            },
            _ => return Err(CursedError::CompilerError("Condition must evaluate to an integer".to_string()))
        };

        // Create basic blocks
        let then_block = self.context.append_basic_block(current_function, "then");
        let else_block = self.context.append_basic_block(current_function, "else");
        let merge_block = self.context.append_basic_block(current_function, "merge");

        // Build conditional branch
        self.builder.build_conditional_branch(condition_bool, then_block, else_block)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build conditional branch: {:?}", e)))?;

        // Generate then block
        self.builder.position_at_end(then_block);
        for stmt in &if_stmt.then_branch {
            self.compile_statement(stmt)?;
        }
        
        // Only add branch if block doesn't already have terminator
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(merge_block)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build then branch: {:?}", e)))?;
        }

        // Generate else block
        self.builder.position_at_end(else_block);
        if let Some(else_branch) = &if_stmt.else_branch {
            for stmt in else_branch {
                self.compile_statement(stmt)?;
            }
        }
        
        // Only add branch if block doesn't already have terminator
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(merge_block)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build else branch: {:?}", e)))?;
        }

        // Continue at merge block
        self.builder.position_at_end(merge_block);

        Ok(())
    }

    /// Compile while statement with proper loop structure
    fn compile_while_statement(&mut self, while_stmt: &crate::ast::WhileStatement) -> Result<(), CursedError> {
        let current_function = self.current_function
            .ok_or_else(|| CursedError::CompilerError("No current function for while statement".to_string()))?;

        // Create basic blocks
        let loop_header = self.context.append_basic_block(current_function, "while_header");
        let loop_body = self.context.append_basic_block(current_function, "while_body");
        let loop_exit = self.context.append_basic_block(current_function, "while_exit");

        // Push loop context for break/continue
        self.loop_stack.push(LoopContext {
            continue_block: loop_header,
            break_block: loop_exit,
            label: None,
        });

        // Jump to loop header
        self.builder.build_unconditional_branch(loop_header)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build initial loop branch: {:?}", e)))?;

        // Generate loop header (condition check)
        self.builder.position_at_end(loop_header);
        let condition_val = self.expression_compiler.compile_expression(&while_stmt.condition)?;
        
        let condition_bool = match condition_val {
            BasicValueEnum::IntValue(int_val) => {
                let zero = self.context.i32_type().const_zero();
                self.builder.build_int_compare(IntPredicate::NE, int_val, zero, "while_cond")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build while condition: {:?}", e)))?
            },
            _ => return Err(CursedError::CompilerError("While condition must be an integer".to_string()))
        };

        self.builder.build_conditional_branch(condition_bool, loop_body, loop_exit)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build while conditional branch: {:?}", e)))?;

        // Generate loop body
        self.builder.position_at_end(loop_body);
        for stmt in &while_stmt.body {
            self.compile_statement(stmt)?;
        }
        
        // Only add branch if block doesn't already have terminator
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(loop_header)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build while body branch: {:?}", e)))?;
        }

        // Pop loop context
        self.loop_stack.pop();

        // Continue at loop exit
        self.builder.position_at_end(loop_exit);

        Ok(())
    }

    /// Compile for statement (simplified implementation)
    fn compile_for_statement(&mut self, for_stmt: &crate::ast::ForStatement) -> Result<(), CursedError> {
        let current_function = self.current_function
            .ok_or_else(|| CursedError::CompilerError("No current function for for statement".to_string()))?;

        // Compile initialization if present
        if let Some(init_stmt) = &for_stmt.init {
            self.compile_statement(init_stmt)?;
        }

        // Create basic blocks
        let loop_header = self.context.append_basic_block(current_function, "for_header");
        let loop_body = self.context.append_basic_block(current_function, "for_body");
        let loop_update = self.context.append_basic_block(current_function, "for_update");
        let loop_exit = self.context.append_basic_block(current_function, "for_exit");

        // Push loop context
        self.loop_stack.push(LoopContext {
            continue_block: loop_update,
            break_block: loop_exit,
            label: None,
        });

        // Jump to loop header
        self.builder.build_unconditional_branch(loop_header)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build initial for branch: {:?}", e)))?;

        // Generate loop header (condition check)
        self.builder.position_at_end(loop_header);
        if let Some(condition_expr) = &for_stmt.condition {
            let condition_val = self.expression_compiler.compile_expression(condition_expr)?;
            let condition_bool = match condition_val {
                BasicValueEnum::IntValue(int_val) => {
                    let zero = self.context.i32_type().const_zero();
                    self.builder.build_int_compare(IntPredicate::NE, int_val, zero, "for_cond")
                        .map_err(|e| CursedError::CompilerError(format!("Failed to build for condition: {:?}", e)))?
                },
                _ => return Err(CursedError::CompilerError("For condition must be an integer".to_string()))
            };
            
            self.builder.build_conditional_branch(condition_bool, loop_body, loop_exit)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build for conditional branch: {:?}", e)))?;
        } else {
            // No condition - infinite loop
            self.builder.build_unconditional_branch(loop_body)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build unconditional for branch: {:?}", e)))?;
        }

        // Generate loop body
        self.builder.position_at_end(loop_body);
        for stmt in &for_stmt.body {
            self.compile_statement(stmt)?;
        }
        
        // Only add branch if block doesn't already have terminator
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(loop_update)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build for body branch: {:?}", e)))?;
        }

        // Generate update block
        self.builder.position_at_end(loop_update);
        if let Some(update_expr) = &for_stmt.update {
            // For update, we need to treat the expression as a statement
            self.expression_compiler.compile_expression(update_expr)?;
        }
        
        // Jump back to header
        self.builder.build_unconditional_branch(loop_header)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build for update branch: {:?}", e)))?;

        // Pop loop context
        self.loop_stack.pop();

        // Continue at loop exit
        self.builder.position_at_end(loop_exit);

        Ok(())
    }

    /// Compile break statement
    fn compile_break_statement(&mut self, _break_stmt: &crate::ast::BreakStatement) -> Result<(), CursedError> {
        let loop_context = self.loop_stack.last()
            .ok_or_else(|| CursedError::CompilerError("Break statement outside of loop".to_string()))?;

        self.builder.build_unconditional_branch(loop_context.break_block)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build break branch: {:?}", e)))?;

        Ok(())
    }

    /// Compile continue statement
    fn compile_continue_statement(&mut self, _continue_stmt: &crate::ast::ContinueStatement) -> Result<(), CursedError> {
        let loop_context = self.loop_stack.last()
            .ok_or_else(|| CursedError::CompilerError("Continue statement outside of loop".to_string()))?;

        self.builder.build_unconditional_branch(loop_context.continue_block)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build continue branch: {:?}", e)))?;

        Ok(())
    }

    /// Compile increment statement
    fn compile_increment_statement(&mut self, inc_stmt: &crate::ast::IncrementStatement) -> Result<(), CursedError> {
        // Create an increment expression and compile it
        let inc_expr = crate::ast::IncrementExpression {
            variable: inc_stmt.variable.clone(),
            is_prefix: inc_stmt.is_prefix,
        };
        
        self.expression_compiler.compile_increment_expression(&inc_expr)?;
        Ok(())
    }

    /// Compile decrement statement
    fn compile_decrement_statement(&mut self, dec_stmt: &crate::ast::DecrementStatement) -> Result<(), CursedError> {
        // Create a decrement expression and compile it
        let dec_expr = crate::ast::DecrementExpression {
            variable: dec_stmt.variable.clone(),
            is_prefix: dec_stmt.is_prefix,
        };
        
        self.expression_compiler.compile_decrement_expression(&dec_expr)?;
        Ok(())
    }

    /// Convert CURSED type to LLVM type
    fn convert_cursed_type_to_llvm(&self, cursed_type: &Type) -> Result<BasicTypeEnum<'ctx>, CursedError> {
        match cursed_type {
            Type::Integer | Type::Normie => Ok(self.context.i32_type().into()),
            Type::Float => Ok(self.context.f64_type().into()),
            Type::String | Type::Tea => Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into()),
            Type::Boolean | Type::Lit => Ok(self.context.bool_type().into()),
            Type::Sip => Ok(self.context.i8_type().into()),
            Type::Smol => Ok(self.context.i8_type().into()),
            Type::Mid => Ok(self.context.i16_type().into()),
            Type::Thicc => Ok(self.context.i64_type().into()),
            Type::Snack => Ok(self.context.f32_type().into()),
            Type::Meal => Ok(self.context.f64_type().into()),
            Type::Byte => Ok(self.context.i8_type().into()),
            Type::Rune => Ok(self.context.i32_type().into()),
            _ => Err(CursedError::CompilerError(format!("Unsupported type conversion: {:?}", cursed_type)))
        }
    }

    /// Infer LLVM type from a value
    fn infer_type_from_value(&self, value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, CursedError> {
        match value {
            BasicValueEnum::IntValue(int_val) => Ok(int_val.get_type().into()),
            BasicValueEnum::FloatValue(float_val) => Ok(float_val.get_type().into()),
            BasicValueEnum::PointerValue(ptr_val) => Ok(ptr_val.get_type().into()),
            BasicValueEnum::ArrayValue(array_val) => Ok(array_val.get_type().into()),
            BasicValueEnum::StructValue(struct_val) => Ok(struct_val.get_type().into()),
            BasicValueEnum::VectorValue(vec_val) => Ok(vec_val.get_type().into()),
        }
    }

    /// Check if the current basic block has a terminator instruction
    fn has_terminator(&self) -> bool {
        if let Some(block) = self.builder.get_insert_block() {
            block.get_terminator().is_some()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_create_inkwell_statement_generator() {
        let context = Context::create();
        let generator = InkwellStatementGenerator::new(&context, "test_module");
        
        // Basic smoke test - generator should be created successfully
        assert_eq!(generator.variables.len(), 0);
        assert_eq!(generator.loop_stack.len(), 0);
    }

    #[test]
    fn test_module_creation() {
        let context = Context::create();
        let generator = InkwellStatementGenerator::new(&context, "test_module");
        
        // Module should be created with the correct name
        let module_name = generator.module().get_name().to_str().unwrap();
        assert_eq!(module_name, "test_module");
    }
}
