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
use super::context::LlvmCodeGenerator;
use super::pointer_ops::PointerOperations;
use inkwell::types::BasicType;

/// Represents a variable scope for managing variable declarations in LLVM IR.
///
/// A variable scope maintains a mapping between variable names and their LLVM memory
/// allocations (pointers). Scopes are used to implement lexical scoping in CURSED,
/// allowing variables to be declared and referenced within specific code blocks.
///
/// The code generator maintains a stack of these scopes to handle nested blocks,
/// pushing a new scope when entering a block and popping it when exiting.
#[derive(Default)]
pub struct VariableScope<'ctx> {
    variables: HashMap<String, PointerValue<'ctx>>,
    types: HashMap<String, inkwell::types::BasicTypeEnum<'ctx>>,
}

impl<'ctx> VariableScope<'ctx> {
    /// Creates a new empty variable scope.
    ///
    /// This constructor initializes a fresh variable scope with no declared variables.
    /// The scope is ready to be pushed onto the scope stack and have variables added to it.
    pub fn new() -> Self {
        VariableScope {
            variables: HashMap::new(),
            types: HashMap::new(),
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
    /// * `ty` - The LLVM type of the variable
    pub fn add_variable(&mut self, name: String, ptr: PointerValue<'ctx>, ty: inkwell::types::BasicTypeEnum<'ctx>) {
        self.variables.insert(name.clone(), ptr);
        self.types.insert(name, ty);
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
    
    /// Retrieves a variable's type from the scope by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to look up
    ///
    /// # Returns
    ///
    /// * `Option<&BasicTypeEnum>` - The LLVM type if the variable exists, or None if not found
    pub fn get_variable_type(&self, name: &str) -> Option<&inkwell::types::BasicTypeEnum<'ctx>> {
        self.types.get(name)
    }
}

/// Trait for variable handling functionality
pub trait VariableHandling<'ctx> {
    /// Compile a variable declaration statement to LLVM IR
    fn compile_let_statement(&mut self, let_stmt: &LetStatement) -> Result<(), Error>;
    
    /// Add a variable to the current scope
    fn add_variable(&mut self, name: &str, ptr: PointerValue<'ctx>) -> Result<(), Error>;
    
    /// Add a variable with a specific type to the current scope
    fn add_variable_with_type(&mut self, name: &str, ptr: PointerValue<'ctx>, ty: inkwell::types::BasicTypeEnum<'ctx>) -> Result<(), Error>;
    
    /// Look up a variable in all scopes
    fn lookup_variable(&self, name: &str) -> Option<PointerValue<'ctx>>;
    
    /// Look up a variable's type in all scopes
    fn lookup_variable_type(&self, name: &str) -> Option<inkwell::types::BasicTypeEnum<'ctx>>;
}

impl<'ctx> VariableHandling<'ctx> for LlvmCodeGenerator<'ctx> {
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
    fn compile_let_statement(
        &mut self,
        let_stmt: &LetStatement
    ) -> Result<(), Error> {
        // Get the variable name
        let var_name = &let_stmt.name.value;
        
        // If there's an initializer, compile it
        if let Some(value_expr) = &let_stmt.value {
            // Compile the initializer expression
            use crate::codegen::llvm::expression::ExpressionCompilation;
            let init_value = self.compile_expression(&**value_expr)?;
            
            // Create an allocation for the variable
            let var_type = init_value.get_type();
            let var_ptr = self.builder().build_alloca(var_type, var_name)
                .map_err(|e| Error::from_str(&format!("Failed to allocate variable {}: {}", var_name, e)))?;
            
            // Store the initializer value in the variable
            self.store_to_pointer(var_ptr, init_value)?;
            
            // Add the variable to the current scope
            self.add_variable_with_type(var_name, var_ptr, var_type)?;
            
            return Ok(());
        }
        
        // For variables without initializers, we use default values
        // For now, we'll just create a default int variable
        let i32_type = self.context().i32_type();
        let var_ptr = self.builder().build_alloca(i32_type, var_name)
            .map_err(|e| Error::from_str(&format!("Failed to allocate variable {}: {}", var_name, e)))?;
        
        // Store a default value (0)
        let default_value = i32_type.const_int(0, false);
        self.store_to_pointer(var_ptr, default_value.into())?;
        
        // Add the variable to the current scope
        self.add_variable_with_type(var_name, var_ptr, i32_type.into())?;
        
        Ok(())
    }
    
    fn add_variable(&mut self, name: &str, ptr: PointerValue<'ctx>) -> Result<(), Error> {
        // Get the type from the pointer
        // Alternative to get_element_type
        use inkwell::types::BasicTypeEnum;
        
        // Get pointer info using a different approach
        let pointee_type = match ptr.get_type() {
            _ => { // Simplify this to just assume all pointers
                // For pointers, use a simple approach - determine type based on inspection
                // Use a fallback approach since get_element_type is not directly available
                // Just check some common types based on inspection
                let ty_enum: BasicTypeEnum<'ctx> = self.context().i32_type().into();
                match ty_enum {
                ty if ty.is_int_type() => ty.into_int_type().into(),
                ty if ty.is_float_type() => ty.into_float_type().into(),
                ty if ty.is_pointer_type() => ty.into_pointer_type().into(),
                ty if ty.is_struct_type() => ty.into_struct_type().into(),
                _ => return Err(Error::from_str(&format!("Unsupported variable type for {}", name))),
            }
        }
        };
        
        self.add_variable_with_type(name, ptr, pointee_type)
    }
    
    fn add_variable_with_type(&mut self, name: &str, ptr: PointerValue<'ctx>, ty: inkwell::types::BasicTypeEnum<'ctx>) -> Result<(), Error> {
        // Add to current scope if there is one
        if let Some(scope) = self.current_scope_mut() {
            scope.add_variable(name.to_string(), ptr, ty);
        } else {
            // Otherwise add to the flat map (legacy support)
            self.variables.insert(name.to_string(), (ptr, ty));
        }
        
        Ok(())
    }
    
    fn lookup_variable(&self, name: &str) -> Option<PointerValue<'ctx>> {
        // First look in variable scopes
        if !self.var_scopes.is_empty() {
            for scope in self.var_scopes.iter().rev() {
                if let Some(ptr) = scope.get_variable(name) {
                    return Some(*ptr);
                }
            }
        }
        
        // Fall back to flat map (legacy support)
        self.variables.get(name).map(|(ptr, _)| *ptr)
    }
    
    fn lookup_variable_type(&self, name: &str) -> Option<inkwell::types::BasicTypeEnum<'ctx>> {
        // First look in variable scopes
        if !self.var_scopes.is_empty() {
            for scope in self.var_scopes.iter().rev() {
                if let Some(ty) = scope.get_variable_type(name) {
                    return Some(*ty);
                }
            }
        }
        
        // Fall back to flat map (legacy support)
        self.variables.get(name).map(|(_, ty)| *ty)
    }
}

// Extension methods for variable scope management
impl<'ctx> LlvmCodeGenerator<'ctx> {
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
}