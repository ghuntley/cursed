//! LLVM code generation for method declarations and calls in CURSED
//!
//! This module handles compilation of method declarations (methods with receivers)
//! and method calls into LLVM IR.

use crate::ast::declarations::{MethodDeclaration, Receiver};
use crate::ast::expressions::MethodCall;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::statement::StatementCompilation;
use crate::codegen::llvm::variables::VariableHandling;
use crate::error::Error;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue, BasicMetadataValueEnum};
use inkwell::types::{BasicTypeEnum, FunctionType, BasicType, BasicMetadataTypeEnum};
use inkwell::AddressSpace;
use std::collections::HashMap;
use tracing::{debug, error, info, instrument};

/// Trait for compiling method declarations and calls
pub trait MethodCompilation<'ctx> {
    /// Compile a method declaration into LLVM IR
    fn compile_method_declaration(&mut self, method: &MethodDeclaration) -> Result<FunctionValue<'ctx>, Error>;
    
    /// Compile a method call into LLVM IR
    fn compile_method_call(&mut self, method_call: &MethodCall) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
    
    /// Register a method implementation for a type
    fn register_method_implementation(
        &mut self,
        receiver_type: &str,
        method_name: &str,
        function: FunctionValue<'ctx>,
    ) -> Result<(), Error>;
    
    /// Look up a method implementation for a type
    fn lookup_method_implementation(
        &self,
        receiver_type: &str,
        method_name: &str,
    ) -> Option<FunctionValue<'ctx>>;
}

impl<'ctx> MethodCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, method), level = "debug")]
    fn compile_method_declaration(&mut self, method: &MethodDeclaration) -> Result<FunctionValue<'ctx>, Error> {
        info!("Compiling method declaration: {}", method.name.value);
        
        // Generate the mangled method name
        let receiver_type_name = self.extract_type_name(&method.receiver)?;
        let method_name = format!("{}_{}", receiver_type_name, method.name.value);
        
        debug!("Method mangled name: {}", method_name);
        
        // Build parameter types (receiver + regular parameters)
        let mut param_types: Vec<BasicMetadataTypeEnum> = Vec::new();
        
        // Add receiver as first parameter (simplified - use i32 for now)
        param_types.push(self.context.i32_type().into());
        
        // Add regular parameters (simplified - use i32 for now)
        for _ in &method.parameters {
            param_types.push(self.context.i32_type().into());
        }
        
        // Determine return type (simplified - use i32 for now)
        let return_type = self.context.i32_type();
        
        // Create function type
        let fn_type = return_type.fn_type(&param_types, false);
        
        // Create function
        let function = self.module.add_function(&method_name, fn_type, None);
        
        // Set up function parameters with names
        let params = function.get_params();
        if !params.is_empty() {
            // First parameter is receiver
            params[0].set_name(&method.receiver.name.value);
            
            // Set names for other parameters (simplified)
            for (i, _param_decl) in method.parameters.iter().enumerate() {
                if i + 1 < params.len() {
                    params[i + 1].set_name(&format!("param_{}", i));
                }
            }
        }
        
        // Create entry basic block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        
        // Store the current function for return statements
        let previous_function = self.current_function.take();
        self.current_function = Some(function);
        
        // Create new scope for method body
        self.enter_scope();
        
        // Compile method body
        let result = self.compile_statement(&method.body);
        
        // Exit scope
        self.exit_scope();
        
        // Restore previous function
        self.current_function = previous_function;
        
        match result {
            Ok(_) => {
                // Add default return value
                let default_return = return_type.const_int(0, false);
                self.builder.build_return(Some(&default_return))?;
                
                // Register the method implementation
                self.register_method_implementation(&receiver_type_name, &method.name.value, function)?;
                
                info!("Successfully compiled method: {}", method_name);
                Ok(function)
            },
            Err(e) => {
                error!("Failed to compile method {}: {}", method_name, e);
                Err(e)
            }
        }
    }
    
    #[instrument(skip(self, method_call), level = "debug")]
    fn compile_method_call(&mut self, method_call: &MethodCall) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        info!("Compiling method call: {}", method_call.method.value);
        
        // Compile the receiver expression
        let receiver_value = self.compile_expression(method_call.receiver.as_ref())?;
        
        // Determine receiver type
        let receiver_type_name = self.determine_value_type_name(&receiver_value)?;
        
        debug!("Method call receiver type: {}", receiver_type_name);
        
        // Look up method implementation
        let method_function = match self.lookup_method_implementation(&receiver_type_name, &method_call.method.value) {
            Some(func) => func,
            None => {
                return Err(Error::from_str(&format!(
                    "Method '{}' not found for type '{}'",
                    method_call.method.value, receiver_type_name
                )));
            }
        };
        
        // Compile arguments
        let mut args: Vec<BasicMetadataValueEnum> = vec![receiver_value.into()]; // Receiver is first argument
        
        for arg_expr in &method_call.arguments {
            let arg_value = self.compile_expression(&**arg_expr)?;
            args.push(arg_value.into());
        }
        
        // Call the method
        let call_result = self.builder.build_call(method_function, &args, "method_call")?;
        
        info!("Successfully compiled method call: {}", method_call.method.value);
        
        Ok(call_result.try_as_basic_value().left())
    }
    
    fn register_method_implementation(
        &mut self,
        receiver_type: &str,
        method_name: &str,
        function: FunctionValue<'ctx>,
    ) -> Result<(), Error> {
        debug!("Registering method implementation: {}::{}", receiver_type, method_name);
        
        // Initialize method registry if it doesn't exist
        if self.method_registry.is_none() {
            self.method_registry = Some(HashMap::new());
        }
        
        if let Some(registry) = &mut self.method_registry {
            let type_methods = registry.entry(receiver_type.to_string())
                .or_insert_with(HashMap::new);
            type_methods.insert(method_name.to_string(), function);
        }
        
        Ok(())
    }
    
    fn lookup_method_implementation(
        &self,
        receiver_type: &str,
        method_name: &str,
    ) -> Option<FunctionValue<'ctx>> {
        if let Some(registry) = &self.method_registry {
            if let Some(type_methods) = registry.get(receiver_type) {
                return type_methods.get(method_name).copied();
            }
        }
        None
    }
}

/// Helper methods for method compilation
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Extract type name from receiver
    fn extract_type_name(&self, receiver: &Receiver) -> Result<String, Error> {
        // For now, assume the receiver type is a simple identifier
        // In a full implementation, this would handle more complex type expressions
        Ok(receiver.type_expr.string())
    }
    
    /// Determine the type name of a value
    fn determine_value_type_name(&self, _value: &BasicValueEnum<'ctx>) -> Result<String, Error> {
        // This would need to be implemented based on the value type
        // For now, return a placeholder
        Ok("Unknown".to_string())
    }
}
