//! Expression compilation for LLVM code generation
//!
//! This module handles the compilation of expressions in the CURSED language
//! to LLVM IR.

use inkwell::values::BasicValueEnum;
use crate::ast::traits::Expression;
use crate::ast::expressions::{Identifier, CallExpression, DotExpression, TypeAssertion};
use crate::ast::pointer::types::PointerType;
use crate::ast::pointer::operations::PointerDereference;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::dot_expressions::DotExpressionCompilation;
use super::pointer_ops::PointerOperations;
use super::basic_expressions::BasicExpressionOperations;
use super::function_monomorphization::FunctionMonomorphization;
use super::variables::VariableHandling;
use super::if_expression::IfExpressionCompilation;
use super::type_assertion::InterfaceTypeAssertion;
use super::interface_type_assertion_errors::TypeAssertionErrorHandler;
use super::type_assertion_integration::TypeAssertionIntegration;
use super::type_assertion_implementation::IntegratedTypeAssertion;

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
    #[tracing::instrument(skip(self, expr), fields(expr_type = std::any::type_name_of_val(expr), expr_str = expr.string()), level = "debug")]
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, Error> {
        // Try to handle specific expression types
        let any = expr.as_any();
        
        tracing::debug!("Compiling expression");
        println!("DEBUG Expression Compilation: Type={}, String={}", 
                 std::any::type_name_of_val(expr), expr.string());
        
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
        
        // Handle dot expressions (object.property)
        if let Some(dot_expr) = any.downcast_ref::<DotExpression>() {
            println!("DEBUG: Found dot expression in compile_expression: {}.{}", 
                     dot_expr.object.string(), dot_expr.property);
            return self.compile_dot_expression(dot_expr);
        }
        
        // Handle type assertion expressions (value.(Type))
        if let Some(type_assertion) = any.downcast_ref::<TypeAssertion>() {
            tracing::debug!("Found type assertion expression: {}.({})", 
                     type_assertion.expression.string(), type_assertion.type_name);
            
            // Use the fully integrated type assertion implementation with error handling
            return self.compile_integrated_type_assertion(type_assertion);
        }
        
        // Handle if expressions
        if let Some(if_expr) = any.downcast_ref::<crate::ast::expressions::if_expression::IfExpression>() {
            return self.compile_if_expression(if_expr);
        }
        
        // Handle assignment expressions
        if let Some(assign_expr) = any.downcast_ref::<crate::ast::expressions::AssignmentExpression>() {
            let result = self.compile_assignment_expr(assign_expr)?
                .ok_or_else(|| Error::from_str("Assignment failed"))?;
            return Ok(result);
        }
        
        // Fall back to basic expressions (literals, arithmetic operations)
        self.compile_basic_expression(expr)
    }
    
    fn compile_identifier(&mut self, ident: &Identifier) -> Result<BasicValueEnum<'ctx>, Error> {
        println!("DEBUG: Compiling identifier: {}", ident.value);
        
        // Check if this is actually a dot expression that was parsed as an identifier
        if ident.value.contains(".") {
            let parts: Vec<&str> = ident.value.split(".").collect();
            if parts.len() == 2 {
                // Create a DotExpression on the fly
                let object_ident = Identifier {
                    token: parts[0].to_string(),
                    value: parts[0].to_string(),
                };
                
                let dot_expr = DotExpression {
                    token: ".".to_string(),
                    object: Box::new(object_ident),
                    property: parts[1].to_string(),
                };
                
                println!("DEBUG: Converting identifier to dot expression: {}.{}", 
                         parts[0], parts[1]);
                return self.compile_dot_expression(&dot_expr);
            }
        }
        
        // First try to look up in the variable scope
        if !self.var_scopes.is_empty() {
            for scope in self.var_scopes.iter().rev() {
                if let Some(ptr) = scope.get_variable(&ident.value) {
                    // Found the variable in this scope
                    let load_name = format!("{}_load", ident.value);
                    
                    // Load based on the variable's type if available
                    if let Some(var_type) = scope.get_variable_type(&ident.value) {
                        println!("DEBUG: Variable '{}' found in scope with type {:?}", ident.value, var_type);
                        
                        // Special handling for float types
                        if var_type.is_float_type() {
                            println!("DEBUG: Loading '{}' as float from scope", ident.value);
                            let float_type = var_type.into_float_type();
                            let result = self.builder().build_load(float_type, *ptr, &load_name)
                                .map_err(|e| Error::from_str(&format!("Failed to load float variable {}: {}", ident.value, e)))?;
                            return Ok(result);
                        }
                    } else {
                        println!("DEBUG: Variable '{}' found in scope but type unknown", ident.value);
                    }
                    
                    // Default to standard pointer load for other types
                    return self.load_from_pointer(*ptr, &load_name);
                }
            }
        }
        
        // Legacy implementation: look in the flat variables map
        if let Some((ptr, ty)) = self.variables.get(&ident.value) {
            // Found the variable, load its value
            println!("DEBUG: Variable '{}' found in global map with type {:?}", ident.value, ty);
            let load_name = format!("{}_load", ident.value);
            
            // Create appropriate typed load based on the variable's type
            let result = if ty.is_float_type() {
                // Explicitly load as float
                println!("DEBUG: Loading '{}' as float", ident.value);
                let float_type = ty.into_float_type();
                self.builder().build_load(float_type, *ptr, &load_name)
                    .map_err(|e| Error::from_str(&format!("Failed to load float variable {}: {}", ident.value, e)))?                
            } else {
                // Use general load
                self.load_from_pointer(*ptr, &load_name)?
            };
            
            return Ok(result);
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
        
        // Try to use our dot expression hook for any supported function
        if let Some(hook_result) = super::hook_dot_expressions::dot_expression_call_hook(self, call)? {
        println!("DEBUG: Used dot expression hook for direct call");
        return Ok(hook_result);
        }
        
        // Special case for dot expressions like vibez.spill (fallback)
        if let Some(dot_expr) = callee_expr.as_any().downcast_ref::<DotExpression>() {
        println!("DEBUG CALL: Found DotExpression callee: {}.{}", dot_expr.object.string(), dot_expr.property);
        
        // Special case for vibez.spill
        if dot_expr.object.string() == "vibez" && dot_expr.property == "spill" && call.arguments.len() == 1 {
        println!("DEBUG CALL: Special handling for vibez.spill");
        
        // Compile the string argument
        let arg = self.compile_expression(&*call.arguments[0])?;
        
        // Get the puts function and call it
        if let Some(puts_fn) = self.module().get_function("puts") {
        println!("DEBUG CALL: Found puts function for vibez.spill");
        
        println!("DEBUG CALL: Arg type: {}", if arg.is_int_value() { "int" } 
                                     else if arg.is_pointer_value() { "pointer" } 
                                     else { "other" });
                                         
        // For puts, we need to ensure the argument is a pointer to a null-terminated string
        // For string literals, we should already have a global string constant
        let arg_ptr = if arg.is_pointer_value() {
            // Already a pointer, use it directly
            arg.into_pointer_value()
        } else {
        // Something else, convert to string and create a global
            return Err(Error::from_str("vibez.spill argument must be a string"));
        };
        
            // Call puts with the string argument
                let result = self.builder().build_call(puts_fn, &[arg_ptr.into()], "vibez_spill_call")
                        .map_err(|e| Error::from_str(&format!("Failed to build vibez.spill call: {}", e)))?;
                    
                    // Return the argument value as the result
                    return Ok(arg);
                }
            }
        }
        
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