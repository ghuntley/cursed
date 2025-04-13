//! Expression compilation for LLVM code generation
//!
//! This module handles the compilation of expressions in the CURSED language
//! to LLVM IR.

use inkwell::values::BasicValueEnum;
use crate::ast::traits::Expression;
use crate::ast::expressions::{Identifier, CallExpression};
use crate::ast::pointer::types::PointerType;
use crate::ast::pointer::operations::PointerDereference;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::pointer_ops::PointerOperations;
use super::basic_expressions::BasicExpressionOperations;
use super::function_monomorphization::FunctionMonomorphization;
use super::variables::VariableHandling;

/// Trait for compiling expressions
pub trait ExpressionCompilation<'ctx> {
    /// Compile an expression to LLVM IR
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a specific expression type: identifier
    fn compile_identifier(&mut self, ident: &Identifier) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a specific expression type: function call
    fn compile_regular_call_expression(&mut self, call: &CallExpression) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> ExpressionCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, Error> {
        // Try to handle specific expression types
        let any = expr.as_any();
        
        // Handle identifier expressions (variable references)
        if let Some(ident) = any.downcast_ref::<Identifier>() {
            return self.compile_identifier(ident);
        }
        
        // Handle function call expressions
        if let Some(call) = any.downcast_ref::<CallExpression>() {
            return self.compile_regular_call_expression(call);
        }
        
        // Handle pointer type expressions (@T)
        if let Some(pointer_type) = any.downcast_ref::<PointerType>() {
            return self.compile_pointer_type(pointer_type);
        }
        
        // Handle pointer dereference expressions (@ptr)
        if let Some(pointer_deref) = any.downcast_ref::<PointerDereference>() {
            return self.compile_pointer_dereference(pointer_deref);
        }
        
        // Fall back to basic expressions (literals, arithmetic operations)
        self.compile_basic_expression(expr)
    }
    
    fn compile_identifier(&mut self, ident: &Identifier) -> Result<BasicValueEnum<'ctx>, Error> {
        println!("DEBUG: Compiling identifier: {}", ident.value);
        
        // First try to look up in the variable scope
        if !self.var_scopes.is_empty() {
            for scope in self.var_scopes.iter().rev() {
                if let Some(ptr) = scope.get_variable(&ident.value) {
                    // Found the variable in this scope
                    let load_name = format!("{}_load", ident.value);
                    
                    // Use load_from_pointer from the PointerOperations trait
                    return self.load_from_pointer(*ptr, &load_name);
                }
            }
        }
        
        // Legacy implementation: look in the flat variables map
        if let Some((ptr, ty)) = self.variables.get(&ident.value) {
            // Found the variable, load its value
            let load_name = format!("{}_load", ident.value);
            
            // Use load_from_pointer from the PointerOperations trait
            return self.load_from_pointer(*ptr, &load_name);
        }
        
        // Check if it's a function (variable can be used as function reference)
        if let Some(function) = self.module().get_function(&ident.value) {
            // As a function reference, we return the pointer
            println!("DEBUG: Found function '{}'", ident.value);
            return Ok(function.as_global_value().as_pointer_value().into());
        }
        
        // Variable not found
        Err(Error::from_str(&format!("Variable not found: {}", ident.value)))
    }
    
    fn compile_regular_call_expression(&mut self, call: &CallExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        // Special case for puts function with addition inside
        if let Some(ident) = call.function.as_any().downcast_ref::<Identifier>() {
            if ident.value == "puts" && call.arguments.len() == 1 {
                // Check if the argument is an infix expression with +
                if let Some(infix) = call.arguments[0].as_any().downcast_ref::<crate::ast::expressions::InfixExpression>() {
                    if infix.operator == "+" {
                        println!("DEBUG: Found puts with addition: {} + {}", infix.left.string(), infix.right.string());
                        
                        // Compile the left and right operands
                        let left = self.compile_expression(&*infix.left)?;
                        let right = self.compile_expression(&*infix.right)?;
                        
                        // Ensure we're dealing with integer values
                        if left.is_int_value() && right.is_int_value() {
                            let left_val = left.into_int_value();
                            let right_val = right.into_int_value();
                            
                            // Add the values
                            let sum = self.builder().build_int_add(left_val, right_val, "sum")
                                .map_err(|e| Error::from_str(&format!("Failed to add: {}", e)))?;
                            
                            // Call puts with the sum
                            if let Some(puts_fn) = self.module().get_function("puts") {
                                // Cast to i32 for puts
                                let sum_i32 = self.builder().build_int_cast(sum, self.context().i32_type(), "sum_i32")
                                    .map_err(|e| Error::from_str(&format!("Failed to cast to i32: {}", e)))?;
                                
                                let _ = self.builder().build_call(puts_fn, &[sum_i32.into()], "puts_call")
                                    .map_err(|e| Error::from_str(&format!("Failed to build puts call: {}", e)))?;
                                
                                // Return the sum as the result
                                return Ok(sum.into());
                            }
                        }
                    }
                }
            }
        }
        // Get the callee function
        // Access the function directly, since it's a required field
        // We need to use as_ref() to get &dyn Expression from Box<dyn Expression>
        let callee_expr = call.function.as_ref();
        
        // Evaluate the callee to get a function value
        let callee_value = self.compile_expression(callee_expr)?;
        
        // Compile all arguments
        let mut compiled_args = Vec::new();
        for arg in &call.arguments {
            let compiled_arg = self.compile_expression(arg.as_ref())?;
            compiled_args.push(compiled_arg);
        }
        
        // Determine the function's name for diagnostics
        let fn_name = if let Some(ident) = callee_expr.as_any().downcast_ref::<Identifier>() {
            ident.value.clone()
        } else {
            "anonymous_function".to_string()
        };
        
        // If it's a direct function reference (identifier), get the function value
        let function = if let Some(ident) = callee_expr.as_any().downcast_ref::<Identifier>() {
            // Special case for puts function
            if ident.value == "puts" && call.arguments.len() == 1 {
                println!("DEBUG: Handling puts function call");
                // Compile the argument
                let arg = self.compile_expression(&*call.arguments[0])?;
                
                // Call puts with the argument
                if let Some(puts_fn) = self.module().get_function("puts") {
                    // For printing the sum, convert the value back to i32
                    let arg_i32 = if arg.is_int_value() {
                        self.builder().build_int_cast(arg.into_int_value(), self.context().i32_type(), "i32_cast")
                            .map_err(|e| Error::from_str(&format!("Failed to cast to i32: {}", e)))?
                            .into()
                    } else {
                        arg
                    };
                    
                    let result = self.builder().build_call(puts_fn, &[arg_i32.into()], "puts_call")
                        .map_err(|e| Error::from_str(&format!("Failed to build puts call: {}", e)))?;
                        
                    // Use the argument value as the result of the puts call for debugging
                    return Ok(arg);
                } else {
                    return Err(Error::from_str("puts function not found in module"));
                }
            }
            
            // Regular function lookup
            self.module().get_function(&ident.value)
                .ok_or_else(|| Error::from_str(&format!("Function not found: {}", ident.value)))?
        } else if callee_value.is_pointer_value() {
            // For function pointers, we need to extract the function value
            let ptr_val = callee_value.into_pointer_value();
            
            // If it's a global function reference, try to convert it to a function
            // Simplify the approach: the ptr_val should be a function pointer
            if true { // We assume it's a function since it came from a call expression
                // Instead of try_into() which doesn't work here, get the function another way
                self.module().get_function(&fn_name)
                    .ok_or_else(|| Error::from_str(&format!("Failed to find function: {}", fn_name)))?
            } else {
                return Err(Error::from_str(&format!("Value is not a function: {}", fn_name)));
            }
        } else {
            return Err(Error::from_str(&format!("Callee is not a function: {}", fn_name)));
        };
        
        // Convert compiled arguments to BasicMetadataValueEnum
        let args: Vec<_> = compiled_args.iter().map(|arg| (*arg).into()).collect();
        
        // Build the call instruction
        let call_result = self.builder().build_call(
            function, 
            &args, 
            &format!("{}_result", fn_name)
        ).map_err(|e| Error::from_str(&format!("Failed to build call to {}: {}", fn_name, e)))?;
        
        // Handle void return type and other cases
        // The inkwell Either enum is in a base module now
        let left_value = call_result.try_as_basic_value().left();
        
        // Handle void return type and other cases
        match left_value {
            Some(value) => {
                Ok(value)
            },
            None => {
                // Function returns void, create a dummy value (nullptr)
                let void_ptr = self.context().i8_type().ptr_type(inkwell::AddressSpace::default())
                    .const_null();
                Ok(void_ptr.into())
            }
        }
    }
}

// Extension methods for LlvmCodeGenerator to handle expressions
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile an expression internally, delegating to the expression trait
    pub fn compile_expression_internal(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, String> {
        // This version wraps the trait version
        match self.compile_expression(expr) {
            Ok(val) => Ok(val),
            Err(e) => Err(e.to_string())
        }
    }
}