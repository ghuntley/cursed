//! Basic expression handling for LLVM code generation

use inkwell::values::BasicValueEnum;
use crate::ast::traits::Expression;
use crate::ast::expressions::literals::{IntegerLiteral, FloatLiteral, BooleanLiteral, StringLiteral};
use crate::error::Error;
use super::generator::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a basic expression to LLVM IR
    pub fn compile_basic_expression(
        &mut self, 
        expr: &dyn Expression
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get access to the LLVM context through the module
        let context = self.module().get_context();
        
        // Handle different expression types
        let any = expr.as_any();
        
        // Integer literal
        if let Some(int_lit) = any.downcast_ref::<IntegerLiteral>() {
            let i32_type = context.i32_type();
            return Ok(i32_type.const_int(int_lit.value as u64, false).into());
        }
        
        // Float literal
        if let Some(float_lit) = any.downcast_ref::<FloatLiteral>() {
            let f64_type = context.f64_type();
            return Ok(f64_type.const_float(float_lit.value).into());
        }
        
        // Boolean literal
        if let Some(bool_lit) = any.downcast_ref::<BooleanLiteral>() {
            let i1_type = context.bool_type();
            return Ok(i1_type.const_int(if bool_lit.value { 1 } else { 0 }, false).into());
        }
        
        // String literal
        if let Some(str_lit) = any.downcast_ref::<StringLiteral>() {
            let result = self.builder().build_global_string_ptr(&str_lit.value, "str");
            match result {
                Ok(global_value) => {
                    return Ok(global_value.as_pointer_value().into());
                },
                Err(e) => {
                    return Err(Error::codegen(format!("Failed to build string: {}", e)));
                }
            }
        }
        
        // If we reach here, we don't know how to compile this expression
        Err(Error::codegen(
            format!("Unsupported expression type: {}", expr.string())
        ))
    }
}