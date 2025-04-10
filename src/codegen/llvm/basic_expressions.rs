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
        
        // Import operator types
        use crate::ast::expressions::operators::{PrefixExpression, InfixExpression};
        
        // Prefix expressions
        if let Some(prefix) = any.downcast_ref::<PrefixExpression>() {
            let right = self.compile_expression(prefix.right.as_ref())?;
            
            match prefix.operator.as_str() {
                "-" => {
                    if right.is_int_value() {
                        let right_val = right.into_int_value();
                        let result = self.builder().build_int_neg(right_val, "neg");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build negation: {}", e)))?.into());
                    } else if right.is_float_value() {
                        let right_val = right.into_float_value();
                        let result = self.builder().build_float_neg(right_val, "fneg");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build float negation: {}", e)))?.into());
                    } else {
                        return Err(Error::codegen(format!("Cannot negate non-numeric value: {:?}", right)));
                    }
                },
                "!" => {
                    if right.is_int_value() {
                        let right_val = right.into_int_value();
                        let bool_type = self.context().bool_type();
                        // Compare to 0 to get a boolean value, then invert
                        let zero = bool_type.const_int(0, false);
                        let is_zero = self.builder().build_int_compare(inkwell::IntPredicate::EQ, right_val, zero, "is_zero");
                        let result = is_zero.map_err(|e| Error::codegen(format!("Failed to build comparison: {}", e)))?;
                        return Ok(result.into());
                    } else {
                        return Err(Error::codegen(format!("Cannot apply ! to non-boolean value: {:?}", right)));
                    }
                },
                _ => return Err(Error::codegen(format!("Unsupported prefix operator: {}", prefix.operator))),
            }
        }
        
        // Infix expressions
        if let Some(infix) = any.downcast_ref::<InfixExpression>() {
            let left = self.compile_expression(infix.left.as_ref())?;
            let right = self.compile_expression(infix.right.as_ref())?;
            
            // Arithmetic operations
            if left.is_int_value() && right.is_int_value() {
                let left_val = left.into_int_value();
                let right_val = right.into_int_value();
                
                match infix.operator.as_str() {
                    "+" => {
                        let result = self.builder().build_int_add(left_val, right_val, "add");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build addition: {}", e)))?.into());
                    },
                    "-" => {
                        let result = self.builder().build_int_sub(left_val, right_val, "sub");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build subtraction: {}", e)))?.into());
                    },
                    "*" => {
                        let result = self.builder().build_int_mul(left_val, right_val, "mul");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build multiplication: {}", e)))?.into());
                    },
                    "/" => {
                        let result = self.builder().build_int_signed_div(left_val, right_val, "div");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build division: {}", e)))?.into());
                    },
                    _ => return Err(Error::codegen(format!("Unsupported infix operator: {}", infix.operator))),
                }
            }
            // Float operations (we should handle mixed types in a real compiler)
            else if left.is_float_value() && right.is_float_value() {
                let left_val = left.into_float_value();
                let right_val = right.into_float_value();
                
                match infix.operator.as_str() {
                    "+" => {
                        let result = self.builder().build_float_add(left_val, right_val, "fadd");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build float addition: {}", e)))?.into());
                    },
                    "-" => {
                        let result = self.builder().build_float_sub(left_val, right_val, "fsub");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build float subtraction: {}", e)))?.into());
                    },
                    "*" => {
                        let result = self.builder().build_float_mul(left_val, right_val, "fmul");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build float multiplication: {}", e)))?.into());
                    },
                    "/" => {
                        let result = self.builder().build_float_div(left_val, right_val, "fdiv");
                        return Ok(result.map_err(|e| Error::codegen(format!("Failed to build float division: {}", e)))?.into());
                    },
                    _ => return Err(Error::codegen(format!("Unsupported float infix operator: {}", infix.operator))),
                }
            } else {
                return Err(Error::codegen(format!("Incompatible types for operator {}: {:?} and {:?}", infix.operator, left, right)));
            }
        }
        
        // If we reach here, we don't know how to compile this expression
        Err(Error::codegen(
            format!("Unsupported expression type: {}", expr.string())
        ))
    }
}