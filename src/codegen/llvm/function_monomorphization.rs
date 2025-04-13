//! Function monomorphization for LLVM code generation
//!
//! This module handles the specialization of generic functions in LLVM code generation.
//! It creates concrete implementations of generic functions with specific type parameters.

use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::types::BasicTypeEnum;
use crate::ast::expressions::CallExpression;
use crate::ast::declarations::FunctionStatement;
use crate::core::type_checker::Type;
use crate::error::Error;
use super::context::LlvmCodeGenerator;

/// Trait for function monomorphization functionality
pub trait FunctionMonomorphization<'ctx> {
    /// Compile a generic function call expression
    fn compile_generic_call_expression(
        &mut self,
        call_expr: &CallExpression,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate a specialized version of a generic function with concrete type arguments
    fn generate_specialized_function(
        &mut self,
        generic_function: &FunctionStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<FunctionValue<'ctx>, Error>;
    
    /// Convert a type name to an LLVM type
    fn monomorphization_type_to_llvm_type(&self, type_name: &str) -> Result<BasicTypeEnum<'ctx>, Error>;
}

impl<'ctx> FunctionMonomorphization<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_generic_call_expression(
        &mut self,
        call_expr: &CallExpression,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the function name
        let function_name = &call_expr.function.string();
        println!("DEBUG: Compiling generic call expression for function: {}", function_name);

        // This is a simplified implementation that just returns a placeholder value
        // In a real implementation, we would:
        // 1. Extract type arguments
        // 2. Get the generic function declaration
        // 3. Generate a specialized version of the function
        // 4. Call the specialized function

        // For now, just return a dummy value
        Ok(self.context().i32_type().const_int(42, false).into())
    }

    fn generate_specialized_function(
        &mut self,
        generic_function: &FunctionStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<FunctionValue<'ctx>, Error> {
        println!("DEBUG: Generating specialized function: {} with {} type args", 
                specialized_name, type_args.len());
        
        // This is a simplified implementation that just creates a basic function
        // with the specialized name and returns 42

        // Create function type (i32 return, no params)
        let i32_type = self.context().i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        
        // Create the function
        let function = self.module().add_function(specialized_name, fn_type, None);
        
        // Create a basic block
        let basic_block = self.context().append_basic_block(function, "entry");
        
        // Set insertion point to the basic block
        self.builder().position_at_end(basic_block);
        
        // Return 42
        let ret_val = i32_type.const_int(42, false);
        self.builder().build_return(Some(&ret_val))
            .map_err(|e| Error::from_str(&format!("Failed to build return: {}", e)))?;
        
        Ok(function)
    }

    fn monomorphization_type_to_llvm_type(&self, type_name: &str) -> Result<BasicTypeEnum<'ctx>, Error> {
        match type_name {
            "normie" => Ok(self.context().i32_type().into()),
            "smol" => Ok(self.context().i8_type().into()),
            "mid" => Ok(self.context().i16_type().into()),
            "thicc" => Ok(self.context().i64_type().into()),
            "snack" => Ok(self.context().f32_type().into()),
            "meal" => Ok(self.context().f64_type().into()),
            "byte" => Ok(self.context().i8_type().into()),
            "rune" => Ok(self.context().i32_type().into()),
            _ => Err(Error::from_str(&format!("Unsupported type: {}", type_name))),
        }
    }
}

// Extension methods that don't need to be part of the trait
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Lookup a function with generic type parameters
    pub fn lookup_generic_function(&self, name: &str) -> Option<&FunctionStatement> {
        // This would normally look up the function in a symbol table
        // For now, return None to indicate no function was found
        None
    }
}
