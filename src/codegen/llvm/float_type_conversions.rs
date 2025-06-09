//! Basic working float type conversions for CURSED language
//!
//! This module provides a simplified but working implementation of IEEE 754-compliant
//! float conversions that can be extended as the codebase stabilizes.

use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FloatValue, IntValue};
use inkwell::types::{BasicType, FloatType, IntType};
use inkwell::{FloatPredicate, IntPredicate};
use tracing::{debug, info, instrument};

/// Basic float conversion utilities
pub struct FloatTypeConverter<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
}

impl<'ctx> FloatTypeConverter<'ctx> {
    /// Create a new float type converter
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        Self { context, builder }
    }

    /// Convert float to integer with bounds checking
    #[instrument(skip(self))]
    pub fn convert_float_to_int(
        &self,
        float_value: FloatValue<'ctx>,
        target_bit_width: u32,
        is_signed: bool,
    ) -> Result<IntValue<'ctx>, String> {
        debug!(
            float_type = ?float_value.get_type(),
            target_bits = target_bit_width,
            is_signed = is_signed,
            "Converting float to integer"
        );

        let target_type = match target_bit_width {
            8 => self.context.i8_type(),
            16 => self.context.i16_type(),
            32 => self.context.i32_type(),
            64 => self.context.i64_type(),
            _ => return Err(format!("Unsupported integer bit width: {}", target_bit_width)),
        };

        // Check for special values first
        let is_nan = self.is_nan(float_value)?;
        let is_infinite = self.is_infinite(float_value)?;

        // Apply bounds checking
        let clamped_value = self.apply_bounds_checking(float_value, target_bit_width, is_signed)?;

        // Convert to integer
        let result = if is_signed {
            self.builder
                .build_float_to_signed_int(
                    clamped_value,
                    target_type,
                    &format!("float_to_i{}", target_bit_width),
                )
                .map_err(|e| format!("Failed to convert float to signed int: {:?}", e))?
        } else {
            self.builder
                .build_float_to_unsigned_int(
                    clamped_value,
                    target_type,
                    &format!("float_to_u{}", target_bit_width),
                )
                .map_err(|e| format!("Failed to convert float to unsigned int: {:?}", e))?
        };

        info!("Float to integer conversion completed successfully");
        Ok(result)
    }

    /// Convert between float types (f32 ↔ f64)
    #[instrument(skip(self))]
    pub fn convert_float_to_float(
        &self,
        float_value: FloatValue<'ctx>,
        target_is_f64: bool,
    ) -> Result<FloatValue<'ctx>, String> {
        debug!(
            source_type = ?float_value.get_type(),
            target_is_f64 = target_is_f64,
            "Converting between float types"
        );

        let source_is_f64 = float_value.get_type() == self.context.f64_type();
        
        if source_is_f64 == target_is_f64 {
            debug!("No conversion needed - types are identical");
            return Ok(float_value);
        }

        let result = if target_is_f64 {
            // f32 -> f64 (extension)
            self.builder
                .build_float_ext(float_value, self.context.f64_type(), "f32_to_f64")
                .map_err(|e| format!("Failed to extend float: {:?}", e))?
        } else {
            // f64 -> f32 (truncation)
            self.builder
                .build_float_trunc(float_value, self.context.f32_type(), "f64_to_f32")
                .map_err(|e| format!("Failed to truncate float: {:?}", e))?
        };

        info!("Float to float conversion completed");
        Ok(result)
    }

    /// Convert float to boolean (0.0 = false, non-zero = true)
    #[instrument(skip(self))]
    pub fn convert_float_to_bool(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        debug!(float_type = ?float_value.get_type(), "Converting float to boolean");

        let zero_constant = float_value.get_type().const_zero();
        
        // Check for NaN - NaN should be false
        let is_nan = self.is_nan(float_value)?;
        
        // Check if equal to zero (ordered comparison - false for NaN)
        let is_zero = self.builder
            .build_float_compare(
                FloatPredicate::OEQ,
                float_value,
                zero_constant,
                "is_zero",
            )
            .map_err(|e| format!("Failed to compare float with zero: {:?}", e))?;

        // Check for negative zero
        let neg_zero = float_value.get_type().const_float(-0.0);
        let is_neg_zero = self.builder
            .build_float_compare(
                FloatPredicate::OEQ,
                float_value,
                neg_zero,
                "is_neg_zero",
            )
            .map_err(|e| format!("Failed to check for negative zero: {:?}", e))?;

        // Result is false if: NaN OR zero OR negative zero
        let is_false_nan_or_zero = self.builder
            .build_or(is_nan, is_zero, "false_nan_or_zero")
            .map_err(|e| format!("Failed to combine NaN and zero checks: {:?}", e))?;

        let is_false = self.builder
            .build_or(is_false_nan_or_zero, is_neg_zero, "is_false")
            .map_err(|e| format!("Failed to combine all false conditions: {:?}", e))?;

        // Invert to get the boolean result
        let result = self.builder
            .build_not(is_false, "float_to_bool")
            .map_err(|e| format!("Failed to invert boolean: {:?}", e))?;

        debug!("Float to boolean conversion completed");
        Ok(result)
    }

    /// Convert integer to float
    #[instrument(skip(self))]
    pub fn convert_int_to_float(
        &self,
        int_value: IntValue<'ctx>,
        target_is_f64: bool,
        is_signed: bool,
    ) -> Result<FloatValue<'ctx>, String> {
        debug!(
            int_type = ?int_value.get_type(),
            target_is_f64 = target_is_f64,
            is_signed = is_signed,
            "Converting integer to float"
        );

        let target_type = if target_is_f64 {
            self.context.f64_type()
        } else {
            self.context.f32_type()
        };

        let result = if is_signed {
            self.builder
                .build_signed_int_to_float(
                    int_value,
                    target_type,
                    &format!("signed_int_to_{}", if target_is_f64 { "f64" } else { "f32" }),
                )
                .map_err(|e| format!("Failed to convert signed int to float: {:?}", e))?
        } else {
            self.builder
                .build_unsigned_int_to_float(
                    int_value,
                    target_type,
                    &format!("unsigned_int_to_{}", if target_is_f64 { "f64" } else { "f32" }),
                )
                .map_err(|e| format!("Failed to convert unsigned int to float: {:?}", e))?
        };

        info!("Integer to float conversion completed");
        Ok(result)
    }

    /// Check if a float value is NaN
    fn is_nan(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        // NaN is the only value that is not equal to itself
        let is_nan = self.builder
            .build_float_compare(
                FloatPredicate::UNO, // Unordered (true for NaN)
                float_value,
                float_value,
                "is_nan",
            )
            .map_err(|e| format!("Failed to check for NaN: {:?}", e))?;

        Ok(is_nan)
    }

    /// Check if a float value is infinite
    fn is_infinite(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        let float_type = float_value.get_type();
        let infinity = float_type.const_float(f64::INFINITY);
        let neg_infinity = float_type.const_float(f64::NEG_INFINITY);

        let is_pos_inf = self.builder
            .build_float_compare(
                FloatPredicate::OEQ,
                float_value,
                infinity,
                "is_pos_inf",
            )
            .map_err(|e| format!("Failed to check for positive infinity: {:?}", e))?;

        let is_neg_inf = self.builder
            .build_float_compare(
                FloatPredicate::OEQ,
                float_value,
                neg_infinity,
                "is_neg_inf",
            )
            .map_err(|e| format!("Failed to check for negative infinity: {:?}", e))?;

        let is_infinite = self.builder
            .build_or(is_pos_inf, is_neg_inf, "is_infinite")
            .map_err(|e| format!("Failed to combine infinity checks: {:?}", e))?;

        Ok(is_infinite)
    }

    /// Apply bounds checking for float-to-int conversions
    fn apply_bounds_checking(
        &self,
        float_value: FloatValue<'ctx>,
        target_bit_width: u32,
        is_signed: bool,
    ) -> Result<FloatValue<'ctx>, String> {
        let (min_value, max_value) = self.get_int_type_limits(target_bit_width, is_signed);

        debug!(
            target_bits = target_bit_width,
            min_value = min_value,
            max_value = max_value,
            is_signed = is_signed,
            "Applying bounds checking"
        );

        let float_type = float_value.get_type();
        let min_const = float_type.const_float(min_value);
        let max_const = float_type.const_float(max_value);

        // Clamp the value to the valid range
        let below_min = self.builder
            .build_float_compare(FloatPredicate::OLT, float_value, min_const, "below_min")
            .map_err(|e| format!("Failed to compare with minimum: {:?}", e))?;

        let above_max = self.builder
            .build_float_compare(FloatPredicate::OGT, float_value, max_const, "above_max")
            .map_err(|e| format!("Failed to compare with maximum: {:?}", e))?;

        let value_or_min = self.builder
            .build_select(below_min, min_const, float_value, "value_or_min")
            .map_err(|e| format!("Failed to select minimum value: {:?}", e))?
            .into_float_value();

        let final_value = self.builder
            .build_select(above_max, max_const, value_or_min, "bounds_checked")
            .map_err(|e| format!("Failed to select maximum value: {:?}", e))?
            .into_float_value();

        info!("Bounds checking applied successfully");
        Ok(final_value)
    }

    /// Get integer type limits for bounds checking
    fn get_int_type_limits(&self, bit_width: u32, is_signed: bool) -> (f64, f64) {
        if is_signed {
            let max_signed = (1u128 << (bit_width - 1)) - 1;
            let min_signed = -(1i128 << (bit_width - 1));
            (min_signed as f64, max_signed as f64)
        } else {
            let max_unsigned = (1u128 << bit_width) - 1;
            (0.0, max_unsigned as f64)
        }
    }

    /// Get the context
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    /// Get the builder
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }
}

/// Convenience functions for CURSED type names
impl<'ctx> FloatTypeConverter<'ctx> {
    /// Convert to CURSED smol type (i8)
    pub fn to_smol(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.convert_float_to_int(float_value, 8, true)
    }

    /// Convert to CURSED mid type (i16)
    pub fn to_mid(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.convert_float_to_int(float_value, 16, true)
    }

    /// Convert to CURSED normie type (i32)
    pub fn to_normie(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.convert_float_to_int(float_value, 32, true)
    }

    /// Convert to CURSED thicc type (i64)
    pub fn to_thicc(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.convert_float_to_int(float_value, 64, true)
    }

    /// Convert to CURSED snack type (f32)
    pub fn to_snack(&self, value: BasicValueEnum<'ctx>) -> Result<FloatValue<'ctx>, String> {
        match value {
            BasicValueEnum::FloatValue(float_val) => {
                self.convert_float_to_float(float_val, false)
            },
            BasicValueEnum::IntValue(int_val) => {
                self.convert_int_to_float(int_val, false, true) // Assume signed
            },
            _ => Err("Cannot convert non-numeric value to snack".to_string())
        }
    }

    /// Convert to CURSED meal type (f64)
    pub fn to_meal(&self, value: BasicValueEnum<'ctx>) -> Result<FloatValue<'ctx>, String> {
        match value {
            BasicValueEnum::FloatValue(float_val) => {
                self.convert_float_to_float(float_val, true)
            },
            BasicValueEnum::IntValue(int_val) => {
                self.convert_int_to_float(int_val, true, true) // Assume signed
            },
            _ => Err("Cannot convert non-numeric value to meal".to_string())
        }
    }

    /// Convert to CURSED lit type (bool)
    pub fn to_lit(&self, value: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, String> {
        match value {
            BasicValueEnum::FloatValue(float_val) => {
                self.convert_float_to_bool(float_val)
            },
            BasicValueEnum::IntValue(int_val) => {
                // Integer to boolean: 0 = false, non-zero = true
                let zero = int_val.get_type().const_zero();
                let is_non_zero = self.builder
                    .build_int_compare(IntPredicate::NE, int_val, zero, "int_to_bool")
                    .map_err(|e| format!("Failed to compare integer with zero: {:?}", e))?;
                Ok(is_non_zero)
            },
            _ => Err("Cannot convert non-numeric value to lit".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_float_converter_creation() {
        let context = Context::create();
        let converter = FloatTypeConverter::new(&context);
        assert_eq!(converter.context() as *const _, &context as *const _);
    }

    #[test]
    fn test_int_type_limits() {
        let context = Context::create();
        let converter = FloatTypeConverter::new(&context);
        
        assert_eq!(converter.get_int_type_limits(8, true), (-128.0, 127.0));
        assert_eq!(converter.get_int_type_limits(8, false), (0.0, 255.0));
        assert_eq!(converter.get_int_type_limits(32, true), (-2147483648.0, 2147483647.0));
    }

    #[test]
    fn test_ieee754_constants() {
        assert!(f64::NAN.is_nan());
        assert!(f64::INFINITY.is_infinite());
        assert!(f64::NEG_INFINITY.is_infinite());
        assert_eq!(0.0f64.to_bits(), 0x0000000000000000u64);
        assert_eq!((-0.0f64).to_bits(), 0x8000000000000000u64);
    }
}
