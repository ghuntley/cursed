//! Implementation of FloatConversion trait for LlvmCodeGenerator
//!
//! This module provides the concrete implementation of comprehensive float conversions
//! for the CURSED LLVM code generator, integrating with the existing infrastructure.

use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use super::float_conversions::FloatConversion;
use inkwell::builder::Builder;
use inkwell::context::Context;
use tracing::{debug, instrument};

impl<'ctx> FloatConversion<'ctx> for LlvmCodeGenerator<'ctx> {
    fn context(&self) -> &'ctx Context {
        &self.context
    }

    fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }
}

/// Enhanced type conversion system for comprehensive float conversions
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Enhanced compile_type_conversion with comprehensive float handling
    #[instrument(skip(self))]
    pub fn compile_type_conversion_enhanced(
        &mut self,
        type_conv: &crate::ast::expressions::TypeConversionExpression,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        debug!(
            target_type = type_conv.type_name.as_str(),
            "Starting enhanced type conversion"
        );

        // Get the value to convert
        let value = self.compile_expression(type_conv.expression.as_ref())
            .map_err(|e| format!("Failed to compile expression: {}", e))?;
        let target_type_name = &type_conv.type_name;

        // Handle float conversions with comprehensive IEEE 754 support
        match target_type_name.as_str() {
            // Float types - use enhanced conversion system
            "snack" => self.convert_to_snack(value),
            "meal" => self.convert_to_meal(value),
            
            // Integer types with float source support
            "smol" => self.convert_to_smol_enhanced(value),
            "mid" => self.convert_to_mid_enhanced(value), 
            "normie" => self.convert_to_normie_enhanced(value),
            "thicc" => self.convert_to_thicc_enhanced(value),
            
            // Boolean conversion with float support
            "lit" => self.convert_to_bool_enhanced(value),
            
            // Fallback for other types - simplified implementation  
            _ => Err(format!("Type conversion to {} not yet implemented in enhanced system", target_type_name)),
        }
    }

    /// Convert any value to snack (f32) with comprehensive handling
    #[instrument(skip(self))]
    fn convert_to_snack(
        &mut self,
        value: inkwell::values::BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        if value.is_float_value() {
            let float_val = value.into_float_value();
            let target_type = self.context.f32_type();
            self.convert_float_to_float(float_val, target_type, "snack")
        } else if value.is_int_value() {
            let int_val = value.into_int_value();
            let target_type = self.context.f32_type();
            // Most CURSED integer types are signed
            let is_signed = !matches!(target_type, _) || true; // Default to signed
            self.convert_int_to_float(int_val, target_type, "snack", is_signed)
        } else {
            Err(format!("Cannot convert {:?} to snack", value.get_type()))
        }
    }

    /// Convert any value to meal (f64) with comprehensive handling  
    #[instrument(skip(self))]
    fn convert_to_meal(
        &mut self,
        value: inkwell::values::BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        if value.is_float_value() {
            let float_val = value.into_float_value();
            let target_type = self.context.f64_type();
            self.convert_float_to_float(float_val, target_type, "meal")
        } else if value.is_int_value() {
            let int_val = value.into_int_value();
            let target_type = self.context.f64_type();
            // Most CURSED integer types are signed
            let is_signed = true;
            self.convert_int_to_float(int_val, target_type, "meal", is_signed)
        } else {
            Err(format!("Cannot convert {:?} to meal", value.get_type()))
        }
    }

    /// Convert any value to smol (i8) with float source support
    #[instrument(skip(self))]
    fn convert_to_smol_enhanced(
        &mut self,
        value: inkwell::values::BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        if value.is_float_value() {
            let float_val = value.into_float_value();
            let target_type = self.context.i8_type();
            self.convert_float_to_int(float_val, target_type, "smol", true)
        } else if value.is_int_value() {
            // Use existing integer conversion logic
            let int_val = value.into_int_value();
            let i8_type = self.context.i8_type();
            
            if int_val.get_type().get_bit_width() > 8 {
                let truncated = self.builder.build_int_truncate(
                    int_val,
                    i8_type,
                    "to_smol"
                ).map_err(|e| format!("Failed to truncate to smol: {:?}", e))?;
                Ok(truncated.into())
            } else {
                let extended = self.builder.build_int_s_extend(
                    int_val,
                    i8_type,
                    "to_smol"
                ).map_err(|e| format!("Failed to extend to smol: {:?}", e))?;
                Ok(extended.into())
            }
        } else {
            Err(format!("Cannot convert {:?} to smol", value.get_type()))
        }
    }

    /// Convert any value to mid (i16) with float source support  
    #[instrument(skip(self))]
    fn convert_to_mid_enhanced(
        &mut self,
        value: inkwell::values::BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        if value.is_float_value() {
            let float_val = value.into_float_value();
            let target_type = self.context.i16_type();
            self.convert_float_to_int(float_val, target_type, "mid", true)
        } else if value.is_int_value() {
            let int_val = value.into_int_value();
            let i16_type = self.context.i16_type();
            
            let bit_width = int_val.get_type().get_bit_width();
            if bit_width > 16 {
                let truncated = self.builder.build_int_truncate(
                    int_val,
                    i16_type,
                    "to_mid"
                ).map_err(|e| format!("Failed to truncate to mid: {:?}", e))?;
                Ok(truncated.into())
            } else if bit_width < 16 {
                let extended = self.builder.build_int_s_extend(
                    int_val,
                    i16_type,
                    "to_mid"
                ).map_err(|e| format!("Failed to extend to mid: {:?}", e))?;
                Ok(extended.into())
            } else {
                Ok(int_val.into())
            }
        } else {
            Err(format!("Cannot convert {:?} to mid", value.get_type()))
        }
    }

    /// Convert any value to normie (i32) with float source support
    #[instrument(skip(self))]
    fn convert_to_normie_enhanced(
        &mut self,
        value: inkwell::values::BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        if value.is_float_value() {
            let float_val = value.into_float_value();
            let target_type = self.context.i32_type();
            self.convert_float_to_int(float_val, target_type, "normie", true)
        } else if value.is_int_value() {
            let int_val = value.into_int_value();
            let i32_type = self.context.i32_type();
            
            let bit_width = int_val.get_type().get_bit_width();
            if bit_width > 32 {
                let truncated = self.builder.build_int_truncate(
                    int_val,
                    i32_type,
                    "to_normie"
                ).map_err(|e| format!("Failed to truncate to normie: {:?}", e))?;
                Ok(truncated.into())
            } else if bit_width < 32 {
                let extended = self.builder.build_int_s_extend(
                    int_val,
                    i32_type,
                    "to_normie"
                ).map_err(|e| format!("Failed to extend to normie: {:?}", e))?;
                Ok(extended.into())
            } else {
                Ok(int_val.into())
            }
        } else {
            Err(format!("Cannot convert {:?} to normie", value.get_type()))
        }
    }

    /// Convert any value to thicc (i64) with float source support
    #[instrument(skip(self))]
    fn convert_to_thicc_enhanced(
        &mut self,
        value: inkwell::values::BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        if value.is_float_value() {
            let float_val = value.into_float_value();
            let target_type = self.context.i64_type();
            self.convert_float_to_int(float_val, target_type, "thicc", true)
        } else if value.is_int_value() {
            let int_val = value.into_int_value();
            let i64_type = self.context.i64_type();
            
            if int_val.get_type().get_bit_width() < 64 {
                let extended = self.builder.build_int_s_extend(
                    int_val,
                    i64_type,
                    "to_thicc"
                ).map_err(|e| format!("Failed to extend to thicc: {:?}", e))?;
                Ok(extended.into())
            } else {
                Ok(int_val.into())
            }
        } else {
            Err(format!("Cannot convert {:?} to thicc", value.get_type()))
        }
    }

    /// Convert any value to lit (bool) with comprehensive float support
    #[instrument(skip(self))]
    fn convert_to_bool_enhanced(
        &mut self,
        value: inkwell::values::BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        if value.is_float_value() {
            let float_val = value.into_float_value();
            self.convert_float_to_bool(float_val)
        } else if value.is_int_value() {
            let int_val = value.into_int_value();
            let zero = int_val.get_type().const_zero();
            
            let is_non_zero = self.builder.build_int_compare(
                inkwell::IntPredicate::NE,
                int_val,
                zero,
                "int_to_bool"
            ).map_err(|e| format!("Failed to compare integer with zero: {:?}", e))?;
            
            // Convert i1 to i8 for consistency
            let result = self.builder.build_int_z_extend(
                is_non_zero,
                self.context.i8_type(),
                "bool_result"
            ).map_err(|e| format!("Failed to extend boolean: {:?}", e))?;
            
            Ok(result.into())
        } else {
            Err(format!("Cannot convert {:?} to lit", value.get_type()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use inkwell::module::Module;

    #[test]
    fn test_float_conversion_implementation() {
        let context = Context::create();
        let module = context.create_module("test");
        let generator = LlvmCodeGenerator::new(&context, "test", std::path::PathBuf::from("test.cursed"));
        
        // Test that the implementation compiles
        assert!(generator.context() == &context);
    }
}
