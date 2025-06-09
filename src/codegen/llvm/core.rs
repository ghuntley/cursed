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
use crate::core::type_checker::Type;
use inkwell::values::BasicValueEnum;
use super::context::LlvmCodeGenerator;
use super::type_conversions::{IntegerTypeConversions, ConversionMatrix};
use super::type_conversion_system::{TypeConversionSystem, ConversionConfig};
use super::float_conversions::FloatConversion;
use tracing::{debug, instrument, info, warn};

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles a type conversion expression to LLVM IR using comprehensive conversion system.
    ///
    /// This method handles explicit type conversions in CURSED code using the 'as' keyword.
    /// It supports conversions between all CURSED numeric types with proper overflow/underflow
    /// handling and LLVM intrinsics:
    ///
    /// Integer types:
    /// - "smol": 8-bit integer (i8)
    /// - "mid": 16-bit integer (i16)
    /// - "normie": 32-bit integer (i32)
    /// - "thicc": 64-bit integer (i64)
    /// - "lit": boolean type
    ///
    /// Floating-point types:
    /// - "snack": 32-bit float (f32)
    /// - "meal": 64-bit float (f64)
    ///
    /// Features:
    /// - Proper sign extension/truncation for integer conversions
    /// - Overflow checking for narrowing conversions
    /// - Signed/unsigned float to integer conversions
    /// - Integer to boolean conversions (0 = false, non-zero = true)
    /// - Boolean to integer conversions (false = 0, true = 1)
    ///
    /// # Arguments
    ///
    /// * `type_conv` - The AST node representing the type conversion expression
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The LLVM value resulting from the conversion, or an error message
    #[instrument(skip(self), fields(target_type = %type_conv.type_name))]
    pub fn compile_type_conversion(&mut self, type_conv: &TypeConversionExpression) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Compiling type conversion to {}", type_conv.type_name);

        // Get the value to convert
        let value = self.compile_expression(type_conv.expression.as_ref())?;
        
        // Convert target type name to Type enum
        let target_type = self.string_to_type(&type_conv.type_name)?;
        
        // Determine source type from the LLVM value
        let source_type = self.infer_source_type(&value)?;
        
        debug!("Converting from {:?} to {:?}", source_type, target_type);

        // Use comprehensive conversion system
        match (&source_type, &target_type) {
            // Integer to integer conversions
            (Type::Smol | Type::Mid | Type::Normie | Type::Thicc, 
             Type::Smol | Type::Mid | Type::Normie | Type::Thicc) => {
                if value.is_int_value() {
                    let result = self.convert_integer_to_integer(
                        value.into_int_value(), 
                        &source_type, 
                        &target_type
                    )?;
                    Ok(result.into())
                } else {
                    Err(format!("Expected integer value for integer conversion"))
                }
            },

            // Integer to float conversions
            (Type::Smol | Type::Mid | Type::Normie | Type::Thicc, 
             Type::Snack | Type::Meal) => {
                if value.is_int_value() {
                    let result = self.convert_integer_to_float(
                        value.into_int_value(), 
                        &source_type, 
                        &target_type,
                        true  // Signed conversion
                    )?;
                    Ok(result.into())
                } else {
                    Err(format!("Expected integer value for integer to float conversion"))
                }
            },

            // Float to integer conversions
            (Type::Snack | Type::Meal, 
             Type::Smol | Type::Mid | Type::Normie | Type::Thicc) => {
                if value.is_float_value() {
                    let result = self.convert_float_to_integer(
                        value.into_float_value(), 
                        &source_type, 
                        &target_type,
                        true  // Signed conversion
                    )?;
                    Ok(result.into())
                } else {
                    Err(format!("Expected float value for float to integer conversion"))
                }
            },

            // Float to float conversions
            (Type::Snack | Type::Meal, Type::Snack | Type::Meal) => {
                if value.is_float_value() {
                    let result = self.convert_float_to_float(
                        value.into_float_value(), 
                        &source_type, 
                        &target_type
                    )?;
                    Ok(result.into())
                } else {
                    Err(format!("Expected float value for float conversion"))
                }
            },

            // Integer to boolean conversions  
            (Type::Smol | Type::Mid | Type::Normie | Type::Thicc, Type::Lit) => {
                if value.is_int_value() {
                    let result = self.convert_integer_to_bool(value.into_int_value())?;
                    Ok(result.into())
                } else {
                    Err(format!("Expected integer value for integer to boolean conversion"))
                }
            },

            // Boolean to integer conversions
            (Type::Lit, Type::Smol | Type::Mid | Type::Normie | Type::Thicc) => {
                if value.is_int_value() {
                    let result = self.convert_bool_to_integer(value.into_int_value(), &target_type)?;
                    Ok(result.into())
                } else {
                    Err(format!("Expected boolean value for boolean to integer conversion"))
                }
            },

            // Same type - no conversion needed
            (source, target) if source == target => {
                debug!("No conversion needed - same types");
                Ok(value)
            },

            // Unsupported conversions
            _ => Err(format!("Unsupported type conversion from {:?} to {:?}", source_type, target_type))
        }
    }

    /// Convert string type name to Type enum
    fn string_to_type(&self, type_name: &str) -> Result<Type, String> {
        match type_name {
            "smol" => Ok(Type::Smol),
            "mid" => Ok(Type::Mid),
            "normie" => Ok(Type::Normie),
            "thicc" => Ok(Type::Thicc),
            "snack" => Ok(Type::Snack),
            "meal" => Ok(Type::Meal),
            "lit" => Ok(Type::Lit),
            _ => Err(format!("Unknown type name: {}", type_name)),
        }
    }

    /// Infer source type from LLVM value
    fn infer_source_type(&self, value: &BasicValueEnum<'ctx>) -> Result<Type, String> {
        if let Ok(int_value) = value.try_into() {
            let int_value: inkwell::values::IntValue = int_value;
            match int_value.get_type().get_bit_width() {
                1 => Ok(Type::Lit),     // Boolean
                8 => Ok(Type::Smol),    // i8
                16 => Ok(Type::Mid),    // i16
                32 => Ok(Type::Normie), // i32
                64 => Ok(Type::Thicc),  // i64
                _ => Err(format!("Unknown integer bit width: {}", int_value.get_type().get_bit_width())),
            }
        } else if let Ok(float_value) = value.try_into() {
            let float_value: inkwell::values::FloatValue = float_value;
            if float_value.get_type().is_f32_type() {
                Ok(Type::Snack) // f32
            } else if float_value.get_type().is_f64_type() {
                Ok(Type::Meal)  // f64
            } else {
                Err(format!("Unknown float type"))
            }
        } else {
            Err(format!("Cannot infer type from LLVM value"))
        }
    }

    /// Convert between float types
    fn convert_float_to_float(
        &mut self,
        value: inkwell::values::FloatValue<'ctx>,
        source_type: &Type,
        target_type: &Type,
    ) -> Result<inkwell::values::FloatValue<'ctx>, String> {
        let target_llvm_type = self.get_llvm_float_type(target_type)?;

        let result = match (source_type, target_type) {
            (Type::Meal, Type::Snack) => {
                // f64 to f32 - truncation
                self.builder.build_float_trunc(
                    value,
                    target_llvm_type,
                    "float_trunc"
                ).map_err(|e| format!("Failed to truncate float: {:?}", e))?
            },
            (Type::Snack, Type::Meal) => {
                // f32 to f64 - extension
                self.builder.build_float_ext(
                    value,
                    target_llvm_type,
                    "float_ext"
                ).map_err(|e| format!("Failed to extend float: {:?}", e))?
            },
            _ => value, // Same type
        };

        Ok(result)
    }
}