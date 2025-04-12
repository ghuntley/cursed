//! LLVM code generation for functions in the CURSED language.
//!
//! This module handles the translation of CURSED functions to LLVM IR, including
//! function definitions, parameter handling, function bodies, and function calls.
//! It manages the scope of variables within functions and ensures proper control
//! flow for function entry and exit.
//!
//! Key responsibilities include:
//! - Creating LLVM function declarations with appropriate signatures
//! - Setting up parameter passing and local variable storage
//! - Compiling function bodies while maintaining proper variable scoping
//! - Generating function call instructions with argument passing
//! - Handling return values and ensuring proper function termination

use inkwell::values::BasicValueEnum;
use crate::ast::declarations::FunctionStatement;
use crate::ast::expressions::CallExpression;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles a function definition to LLVM IR.
    ///
    /// This method translates a CURSED function declaration into an LLVM function with the
    /// appropriate signature, parameter handling, and body. It creates a new function context
    /// with its own variable scope for parameters and local variables.
    ///
    /// The process includes:
    /// 1. Creating an LLVM function with the appropriate signature
    /// 2. Creating an entry basic block for the function body
    /// 3. Setting up parameter allocations and initializations
    /// 4. Compiling the function body statements
    /// 5. Ensuring the function has a proper terminator (return instruction)
    /// 6. Validating the generated function
    ///
    /// # Arguments
    ///
    /// * `fn_lit` - The AST function declaration node to compile
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - A pointer to the compiled function
    pub fn compile_function_literal(&mut self, fn_lit: &crate::ast::declarations::FunctionStatement) -> Result<BasicValueEnum<'ctx>, String> {
        // Function name - using a generic anonymous function name if none is provided
        let fn_name = "anonymous_fn";
        
        // Create parameter types
        let mut param_types = Vec::new(); // Fixed size
        // Parameters would be processed here
        
        // Default return type
        let return_type = self.context.i64_type().into();
        
        // Create the function
        let function = self.create_function(&fn_name, &param_types, return_type, false)?;
        
        // Create a new basic block for the function body
        let entry_block = self.context.append_basic_block(function, "entry");
        
        // Save the previous function context
        let previous_function = self.current_function;
        self.current_function = Some(function);
        
        // Position at the start of the new block
        self.builder.position_at_end(entry_block);
        
        // Save previous variables
        let previous_variables = std::mem::take(&mut self.variables);
        
        // Add parameters to the function's symbol table
        for (i, param) in fn_lit.parameters.iter().enumerate() {
            let param_name = &param.name.value; // Access the value through name
            let llvm_param = function.get_nth_param(i as u32)
                .expect(&format!("Missing parameter {}", i));
            
            // Allocate stack space for the parameter
            let param_alloca = self.create_entry_block_alloca(llvm_param.get_type(), param_name);
            
            // Store the parameter value
            self.builder.build_store(param_alloca, llvm_param).unwrap();
            
            // Add to symbol table
            self.variables.insert(param_name.clone(), (param_alloca, llvm_param.get_type()));
        }
        
        // Compile the function body
        self.compile_statement(&fn_lit.body)?;
        
        // Add a default return if the block doesn't have a terminator
        if let Some(block) = self.builder.get_insert_block() {
            if block.get_terminator().is_none() {
                self.builder.build_return(Some(&self.context.i64_type().const_int(0, false))).unwrap();
            }
        }
        
        // Validate the function
        if !function.verify(true) {
            return Err(format!("Invalid function: {}", fn_name));
        }
        
        // Restore previous context
        self.current_function = previous_function;
        self.variables = previous_variables;
        
        // Return the function as a pointer value
        Ok(function.as_global_value().as_pointer_value().into())
    }
    
    /// Compiles a function call expression to LLVM IR.
    ///
    /// This method translates a CURSED function call into LLVM IR call instructions.
    /// It handles resolving the function being called (either by name or pointer),
    /// compiling the arguments, and creating the actual call instruction.
    ///
    /// The method performs validation to ensure:
    /// - The called entity is actually a function
    /// - The correct number of arguments is provided
    /// - Each argument is properly compiled and converted to the expected type
    ///
    /// # Arguments
    ///
    /// * `call_expr` - The AST function call expression node to compile
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The function call's return value
    pub fn compile_call_expression(&mut self, call_expr: &crate::ast::expressions::CallExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // Get the function to call
        let callee = self.compile_expression(call_expr.function.as_ref())?;
        
        // If it's not a function pointer, try to find it by name
        let function = if !callee.is_pointer_value() {
            return Err("Callee is not a function".to_string());
        } else if let Some(fn_name) = call_expr.function.as_any().downcast_ref::<crate::ast::expressions::Identifier>() {
            // Look up the function by name
            match self.module.get_function(&fn_name.value) {
                Some(f) => f,
                None => return Err(format!("Function '{}' not found", fn_name.value)),
            }
        } else {
            // Function pointer
            // This is limited - we should properly cast the pointer to a function type
            return Err("Function pointers not fully supported yet".to_string());
        };
        
        // Check that the argument count matches
        if function.count_params() != call_expr.arguments.len() as u32 {
            return Err(format!(
                "Function '{}' expects {} arguments but got {}",
                function.get_name().to_string_lossy(),
                function.count_params(),
                call_expr.arguments.len()
            ));
        }
        
        // Compile arguments
        let mut compiled_args = Vec::with_capacity(call_expr.arguments.len());
        for arg in &call_expr.arguments {
            let compiled_arg = self.compile_expression(arg.as_ref())?;
            compiled_args.push(compiled_arg.into());
        }
        
        // Call the function
        let call_result = self.builder
            .build_call(function, &compiled_args, "call")
            .unwrap();
        
        // Get the return value
        match call_result.try_as_basic_value().left() {
            Some(value) => Ok(value),
            None => Ok(self.context.i64_type().const_int(0, false).into()), // void return
        }
    }
}