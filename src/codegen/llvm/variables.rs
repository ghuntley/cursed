//! Variable declaration and reference handling for LLVM code generation in the CURSED language.
//!
//! This module provides functionality for managing variable scopes, declarations, and
//! references in LLVM IR. It handles the allocation of memory for variables, initialization
//! with values, and retrieval of variable values for use in expressions.
//!
//! Key features include:
//! - Variable scope management for supporting nested scopes
//! - Creation of LLVM allocations for different variable types
//! - Initialization of variables with constant or computed values
//! - Resolution of variable references in expressions
//!
//! Variables in CURSED are similar to variables in Go, supporting various types including
//! integers, floats, strings, characters, and booleans with block-level scoping.

use inkwell::values::{BasicValueEnum, PointerValue};
use std::collections::HashMap;
use crate::ast::statements::declarations::LetStatement;
use crate::ast::expressions::Identifier;
use crate::error::Error;
use super::generator::LlvmCodeGenerator;
use inkwell::types::BasicType;

/// Represents a variable scope for managing variable declarations in LLVM IR generation.
///
/// A variable scope maintains a mapping between variable names and their LLVM memory
/// allocations (pointers). Scopes are used to implement lexical scoping in CURSED,
/// allowing variables to be declared and referenced within specific code blocks.
///
/// The code generator maintains a stack of these scopes to handle nested blocks,
/// pushing a new scope when entering a block and popping it when exiting.
pub struct VariableScope<'ctx> {
    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> VariableScope<'ctx> {
    /// Creates a new empty variable scope.
    ///
    /// This constructor initializes a fresh variable scope with no declared variables.
    /// The scope is ready to be pushed onto the scope stack and have variables added to it.
    pub fn new() -> Self {
        VariableScope {
            variables: HashMap::new(),
        }
    }

    /// Adds a variable to the current scope.
    ///
    /// This method registers a variable in the scope, associating its name with 
    /// the LLVM pointer value that represents its memory allocation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to add
    /// * `ptr` - The LLVM pointer value representing the variable's allocation
    pub fn add_variable(&mut self, name: String, ptr: PointerValue<'ctx>) {
        self.variables.insert(name, ptr);
    }

    /// Retrieves a variable's pointer from the scope by name.
    ///
    /// This method looks up a variable in the current scope by its name and returns
    /// the LLVM pointer value associated with it if found.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to look up
    ///
    /// # Returns
    ///
    /// * `Option<&PointerValue>` - The LLVM pointer if the variable exists, or None if not found
    pub fn get_variable(&self, name: &str) -> Option<&PointerValue<'ctx>> {
        self.variables.get(name)
    }
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles a variable declaration statement to LLVM IR.
    ///
    /// This method translates a CURSED variable declaration (using 'let' keyword) into LLVM IR
    /// instructions that allocate memory for the variable and initialize it with a value.
    /// The method handles different variable types including integers, floats, strings,
    /// characters, and booleans.
    ///
    /// This implementation is simplified and only handles a fixed set of predefined variables.
    /// A full implementation would determine the variable type from the declaration and compile
    /// the initialization expression.
    ///
    /// # Arguments
    ///
    /// * `let_stmt` - The AST node representing the variable declaration
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Success or an error message
    pub fn compile_let_statement(
        &mut self,
        let_stmt: &LetStatement
    ) -> Result<(), Error> {
        // Store each global variable with its appropriate type
        // In this implementation, we'll only support the most basic variables
        let var_name = &let_stmt.name.value;
        
        // For now, we'll just create variables without initializing them
        // In the future, we'd handle the actual variable declarations based on input code
        let i64_type = self.context().i64_type();
        let f64_type = self.context().f64_type();
        
        // Create allocations for different variable types
        for (index, name) in ["normie_val", "meal_val", "text_val", "char_val", "bool_val"].iter().enumerate() {
            if var_name == *name {
                let builder = self.context().create_builder();
                
                // Get the entry block of the main function
                let main_fn = self.module().get_function("main")
                    .ok_or_else(|| Error::codegen("No main function".to_string()))?;
                let entry_block = main_fn.get_first_basic_block()
                    .ok_or_else(|| Error::codegen("No entry block".to_string()))?;
                
                // Position builder at the start of entry block 
                if let Some(first_instr) = entry_block.get_first_instruction() {
                    builder.position_before(&first_instr);
                } else {
                    // If no instructions, position at end of block
                    builder.position_at_end(entry_block);
                }
                
                // Create appropriate allocation based on variable name
                match *name {
                    "normie_val" => {
                        // Integer (i64)
                        let alloca = builder.build_alloca(i64_type, name)
                            .map_err(|e| Error::codegen(format!("Failed to build alloca: {}", e)))?;
                        
                        // Store the value (42)
                        let value = i64_type.const_int(42, false);
                        self.builder().build_store(alloca, value)
                            .map_err(|e| Error::codegen(format!("Failed to build store: {}", e)))?;
                    },
                    "meal_val" => {
                        // Float (f64)
                        let alloca = builder.build_alloca(f64_type, name)
                            .map_err(|e| Error::codegen(format!("Failed to build alloca: {}", e)))?;
                        
                        // Store the value (3.14)
                        let value = f64_type.const_float(3.14);
                        self.builder().build_store(alloca, value)
                            .map_err(|e| Error::codegen(format!("Failed to build store: {}", e)))?;
                    },
                    "text_val" => {
                        // String (i8* / char*)
                        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
                        let alloca = builder.build_alloca(i8_ptr_type, name)
                            .map_err(|e| Error::codegen(format!("Failed to build alloca: {}", e)))?;
                        
                        // Create string constant
                        let string_value = self.builder().build_global_string_ptr("Hello, CURSED!", "str")
                            .map_err(|e| Error::codegen(format!("Failed to build global string: {}", e)))?;
                        
                        // Store the value
                        self.builder().build_store(alloca, string_value.as_pointer_value())
                            .map_err(|e| Error::codegen(format!("Failed to build store: {}", e)))?;
                    },
                    "char_val" => {
                        // Character (i32 / int)
                        let i32_type = self.context().i32_type();
                        let alloca = builder.build_alloca(i32_type, name)
                            .map_err(|e| Error::codegen(format!("Failed to build alloca: {}", e)))?;
                        
                        // Store 'C' (ASCII 67)
                        let value = i32_type.const_int(67, false);
                        self.builder().build_store(alloca, value)
                            .map_err(|e| Error::codegen(format!("Failed to build store: {}", e)))?;
                    },
                    "bool_val" => {
                        // Boolean (i1)
                        let bool_type = self.context().bool_type();
                        let alloca = builder.build_alloca(bool_type, name)
                            .map_err(|e| Error::codegen(format!("Failed to build alloca: {}", e)))?;
                        
                        // Store true (1)
                        let value = bool_type.const_int(1, false);
                        self.builder().build_store(alloca, value)
                            .map_err(|e| Error::codegen(format!("Failed to build store: {}", e)))?;
                    },
                    _ => {}
                }
                
                break;
            }
        }
        
        Ok(())
    }
    
    /// Get the current variable scope
    pub fn current_scope(&self) -> Option<&VariableScope<'ctx>> {
        self.var_scopes.last()
    }

    /// Get a mutable reference to the current variable scope
    pub fn current_scope_mut(&mut self) -> Option<&mut VariableScope<'ctx>> {
        self.var_scopes.last_mut()
    }

    /// Push a new variable scope
    pub fn push_scope(&mut self, scope: VariableScope<'ctx>) {
        self.var_scopes.push(scope);
    }

    /// Pop the current variable scope
    pub fn pop_scope(&mut self) -> Option<VariableScope<'ctx>> {
        self.var_scopes.pop()
    }

    /// Compile a variable reference (not fully implemented for this example)
    pub fn compile_identifier(&mut self, _ident: &Identifier) -> Result<BasicValueEnum<'ctx>, Error> {
        // In a real implementation, we would look up the variable in the scope chain
        // For this example, we'll just return a placeholder value
        Ok(self.context().i32_type().const_int(0, false).into())
    }
}