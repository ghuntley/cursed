//! Core functionality for LLVM code generation in the CURSED language.
//!
//! This module contains essential utilities and helper functions for generating
//! LLVM IR from CURSED language constructs. It implements core operations such as
//! type conversions, which allow programs to explicitly cast between different
//! numeric types.
//!
//! The core functionality bridges between the CURSED language's unique type system
//! (with names like "smol", "thicc", "snack", and "meal") and LLVM's more
//! traditional types (i8, i64, f32, f64, etc.).

use crate::ast::*;
use inkwell::values::BasicValueEnum;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles a type conversion expression to LLVM IR.
    ///
    /// This method handles explicit type conversions in CURSED code using the 'as' keyword.
    /// It supports conversions between the language's numeric types:
    ///
    /// Integer types:
    /// - "smol": 8-bit integer (i8)
    /// - "mid": 16-bit integer (i16)
    /// - "normie": 32-bit integer (i32)
    /// - "thicc": 64-bit integer (i64)
    ///
    /// Floating-point types:
    /// - "snack": 32-bit float (f32)
    /// - "meal": 64-bit float (f64)
    ///
    /// # Arguments
    ///
    /// * `type_conv` - The AST node representing the type conversion expression
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The LLVM value resulting from the conversion, or an error message
    pub fn compile_type_conversion(&mut self, type_conv: &TypeConversionExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // Get the value to convert
        let value = self.compile_expression(type_conv.expression.as_ref())?;
        
        // Get the target type - type_name is already a string
        let target_type_name = &type_conv.type_name;
        
        // Handle conversion based on target type
        match target_type_name.as_str() {
            // Integer conversions
            "smol" => {
                if value.is_int_value() {
                    let i8_type = self.context.i8_type();
                    let truncated = self.builder.build_int_truncate(
                        value.into_int_value(), 
                        i8_type, 
                        "to_smol"
                    ).unwrap();
                    Ok(truncated.into())
                } else {
                    Err(format!("Cannot convert non-integer value to smol"))
                }
            },
            "mid" => {
                if value.is_int_value() {
                    let i16_type = self.context.i16_type();
                    let result = if value.get_type().into_int_type().get_bit_width() > 16 {
                        self.builder.build_int_truncate(
                            value.into_int_value(), 
                            i16_type, 
                            "to_mid"
                        ).unwrap()
                    } else {
                        self.builder.build_int_s_extend(
                            value.into_int_value(), 
                            i16_type, 
                            "to_mid"
                        ).unwrap()
                    };
                    Ok(result.into())
                } else {
                    Err(format!("Cannot convert non-integer value to mid"))
                }
            },
            "normie" => {
                if value.is_int_value() {
                    let i32_type = self.context.i32_type();
                    let result = if value.get_type().into_int_type().get_bit_width() > 32 {
                        self.builder.build_int_truncate(
                            value.into_int_value(), 
                            i32_type, 
                            "to_normie"
                        ).unwrap()
                    } else {
                        self.builder.build_int_s_extend(
                            value.into_int_value(), 
                            i32_type, 
                            "to_normie"
                        ).unwrap()
                    };
                    Ok(result.into())
                } else if value.is_float_value() {
                    // Float to int conversion
                    let i32_type = self.context.i32_type();
                    let result = self.builder.build_float_to_signed_int(
                        value.into_float_value(),
                        i32_type,
                        "float_to_normie"
                    ).unwrap();
                    Ok(result.into())
                } else {
                    Err(format!("Cannot convert value to normie"))
                }
            },
            "thicc" => {
                if value.is_int_value() {
                    let i64_type = self.context.i64_type();
                    let result = self.builder.build_int_s_extend(
                        value.into_int_value(),
                        i64_type,
                        "to_thicc"
                    ).unwrap();
                    Ok(result.into())
                } else if value.is_float_value() {
                    // Float to int conversion
                    let i64_type = self.context.i64_type();
                    let result = self.builder.build_float_to_signed_int(
                        value.into_float_value(),
                        i64_type,
                        "float_to_thicc"
                    ).unwrap();
                    Ok(result.into())
                } else {
                    Err(format!("Cannot convert value to thicc"))
                }
            },
            // Float conversions
            "snack" => {
                if value.is_float_value() {
                    let f32_type = self.context.f32_type();
                    let result = self.builder.build_float_trunc(
                        value.into_float_value(),
                        f32_type,
                        "to_snack"
                    ).unwrap();
                    Ok(result.into())
                } else if value.is_int_value() {
                    // Int to float conversion
                    let f32_type = self.context.f32_type();
                    let result = self.builder.build_signed_int_to_float(
                        value.into_int_value(),
                        f32_type,
                        "int_to_snack"
                    ).unwrap();
                    Ok(result.into())
                } else {
                    Err(format!("Cannot convert value to snack"))
                }
            },
            "meal" => {
                if value.is_float_value() {
                    let f64_type = self.context.f64_type();
                    let result = self.builder.build_float_ext(
                        value.into_float_value(),
                        f64_type,
                        "to_meal"
                    ).unwrap();
                    Ok(result.into())
                } else if value.is_int_value() {
                    // Int to float conversion
                    let f64_type = self.context.f64_type();
                    let result = self.builder.build_signed_int_to_float(
                        value.into_int_value(),
                        f64_type,
                        "int_to_meal"
                    ).unwrap();
                    Ok(result.into())
                } else {
                    Err(format!("Cannot convert value to meal"))
                }
            },
            // Other types
            _ => Err(format!("Unsupported type conversion to {}", target_type_name))
        }
    }
}