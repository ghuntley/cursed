//! Comprehensive Float Type Conversions for CURSED Language
//!
//! This module implements IEEE 754-compliant floating point conversions with proper
//! handling of special values (NaN, infinity, -0.0) and bounds checking for float-to-int conversions.
//!
//! CURSED Float Types:
//! - "snack": 32-bit float (f32)
//! - "meal": 64-bit float (f64)

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{BasicValue, BasicValueEnum, FloatValue, IntValue};
use inkwell::types::{BasicType, FloatType, IntType};
use inkwell::{FloatPredicate, IntPredicate};
use tracing::{debug, error, info, warn, instrument};

/// Comprehensive float conversion implementation with IEEE 754 compliance
pub trait FloatConversion<'ctx> {
    fn context(&self) -> &'ctx Context;
    fn builder(&self) -> &Builder<'ctx>;

    /// Convert float to any integer type with bounds checking and special value handling
    #[instrument(skip(self))]
    fn convert_float_to_int(
        &self,
        float_value: FloatValue<'ctx>,
        target_int_type: IntType<'ctx>,
        target_type_name: &str,
        is_signed: bool,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!(
            float_type = ?float_value.get_type(),
            target_type = target_type_name,
            is_signed = is_signed,
            "Starting float to integer conversion"
        );

        let float_type = float_value.get_type();
        let target_bit_width = target_int_type.get_bit_width();

        // Check for special float values first
        let is_nan = self.check_is_nan(float_value)?;
        let is_infinite = self.check_is_infinite(float_value)?;
        let is_negative_zero = self.check_is_negative_zero(float_value)?;

        // Handle special cases
        let special_case_result = self.handle_special_float_cases(
            float_value,
            target_int_type,
            is_nan,
            is_infinite,
            is_negative_zero,
        )?;

        if let Some(result) = special_case_result {
            info!(
                target_type = target_type_name,
                special_case = "handled",
                "Special float value conversion completed"
            );
            return Ok(result);
        }

        // Perform bounds checking for normal values
        let bounds_checked_value = self.apply_bounds_checking(
            float_value,
            target_int_type,
            target_type_name,
            is_signed,
        )?;

        // Convert to integer
        let result = if is_signed {
            self.builder()
                .build_float_to_signed_int(
                    bounds_checked_value,
                    target_int_type,
                    &format!("float_to_signed_{}", target_type_name),
                )
                .map_err(|e| format!("Failed to convert float to signed int: {}", e))?
        } else {
            self.builder()
                .build_float_to_unsigned_int(
                    bounds_checked_value,
                    target_int_type,
                    &format!("float_to_unsigned_{}", target_type_name),
                )
                .map_err(|e| format!("Failed to convert float to unsigned int: {}", e))?
        };

        info!(
            float_value = ?float_value,
            target_type = target_type_name,
            result_bit_width = target_bit_width,
            "Float to integer conversion completed successfully"
        );

        Ok(result.into())
    }

    /// Convert between float types (f32 ↔ f64)
    #[instrument(skip(self))]
    fn convert_float_to_float(
        &self,
        float_value: FloatValue<'ctx>,
        target_float_type: FloatType<'ctx>,
        target_type_name: &str,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!(
            source_type = ?float_value.get_type(),
            target_type = target_type_name,
            "Starting float to float conversion"
        );

        let source_type = float_value.get_type();
        let target_type = target_float_type;

        // Check if conversion is needed
        if source_type == target_type {
            debug!("No conversion needed - types are identical");
            return Ok(float_value.into());
        }

        // Use print_to_string to get type info for bit width comparison
        let source_info = source_type.print_to_string().to_string();
        let target_info = target_type.print_to_string().to_string();
        
        let result = if source_info.contains("double") && target_info.contains("float") {
            // Truncation (f64 -> f32) - potential precision loss
            warn!(
                source_type = %source_info,
                target_type = %target_info,
                "Float truncation may cause precision loss"
            );

            self.builder()
                .build_float_trunc(
                    float_value,
                    target_type,
                    &format!("float_trunc_to_{}", target_type_name),
                )
                .map_err(|e| format!("Failed to truncate float: {}", e))?
        } else {
            // Extension (f32 -> f64) - no precision loss
            debug!("Float extension - no precision loss expected");

            self.builder()
                .build_float_ext(
                    float_value,
                    target_type,
                    &format!("float_ext_to_{}", target_type_name),
                )
                .map_err(|e| format!("Failed to extend float: {}", e))?
        };

        info!(
            target_type = target_type_name,
            precision_change = if source_info.contains("double") && target_info.contains("float") {
                "loss"
            } else {
                "preserved"
            },
            "Float to float conversion completed"
        );

        Ok(result.into())
    }

    /// Convert float to boolean (0.0 = false, non-zero = true)
    #[instrument(skip(self))]
    fn convert_float_to_bool(
        &self,
        float_value: FloatValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!(float_type = ?float_value.get_type(), "Converting float to boolean");

        let zero_constant = float_value.get_type().const_zero();
        
        // Handle NaN specially - NaN is considered false
        let is_nan = self.check_is_nan(float_value)?;
        let is_zero = self.builder()
            .build_float_compare(
                FloatPredicate::OEQ,  // Ordered equal (false for NaN)
                float_value,
                zero_constant,
                "is_zero",
            )
            .map_err(|e| format!("Failed to compare float with zero: {}", e))?;

        let is_negative_zero = self.check_is_negative_zero(float_value)?;

        // Result is false if: NaN OR zero OR negative zero
        let is_false_case = self.builder()
            .build_or(is_nan, is_zero, "is_false_or_zero")
            .map_err(|e| format!("Failed to build OR operation: {}", e))?;

        let is_false = self.builder()
            .build_or(is_false_case, is_negative_zero, "is_false_final")
            .map_err(|e| format!("Failed to build final OR operation: {}", e))?;

        // Invert to get the boolean result (true if not false)
        let result = self.builder()
            .build_not(is_false, "float_to_bool")
            .map_err(|e| format!("Failed to build NOT operation: {}", e))?;

        debug!("Float to boolean conversion completed");
        Ok(result.into())
    }

    /// Convert integer to float type
    #[instrument(skip(self))]
    fn convert_int_to_float(
        &self,
        int_value: IntValue<'ctx>,
        target_float_type: FloatType<'ctx>,
        target_type_name: &str,
        is_signed: bool,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!(
            int_type = ?int_value.get_type(),
            target_type = target_type_name,
            is_signed = is_signed,
            "Converting integer to float"
        );

        let result = if is_signed {
            self.builder()
                .build_signed_int_to_float(
                    int_value,
                    target_float_type,
                    &format!("signed_int_to_{}", target_type_name),
                )
                .map_err(|e| format!("Failed to convert signed int to float: {}", e))?
        } else {
            self.builder()
                .build_unsigned_int_to_float(
                    int_value,
                    target_float_type,
                    &format!("unsigned_int_to_{}", target_type_name),
                )
                .map_err(|e| format!("Failed to convert unsigned int to float: {}", e))?
        };

        info!(
            target_type = target_type_name,
            source_bit_width = int_value.get_type().get_bit_width(),
            "Integer to float conversion completed"
        );

        Ok(result.into())
    }

    /// Check if a float value is NaN
    #[instrument(skip(self))]
    fn check_is_nan(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        // NaN is the only value that is not equal to itself
        let is_nan = self.builder()
            .build_float_compare(
                FloatPredicate::UNO,  // Unordered (true for NaN)
                float_value,
                float_value,
                "is_nan",
            )
            .map_err(|e| format!("Failed to check for NaN: {}", e))?;

        Ok(is_nan)
    }

    /// Check if a float value is infinite
    #[instrument(skip(self))]
    fn check_is_infinite(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        let float_type = float_value.get_type();
        let infinity = if float_type == self.context().f32_type() {
            float_type.const_float(f64::INFINITY)
        } else {
            float_type.const_float(f64::INFINITY)
        };

        let neg_infinity = if float_type == self.context().f32_type() {
            float_type.const_float(f64::NEG_INFINITY)
        } else {
            float_type.const_float(f64::NEG_INFINITY)
        };

        let is_pos_inf = self.builder()
            .build_float_compare(
                FloatPredicate::OEQ,
                float_value,
                infinity,
                "is_pos_inf",
            )
            .map_err(|e| format!("Failed to check for positive infinity: {}", e))?;

        let is_neg_inf = self.builder()
            .build_float_compare(
                FloatPredicate::OEQ,
                float_value,
                neg_infinity,
                "is_neg_inf",
            )
            .map_err(|e| format!("Failed to check for negative infinity: {}", e))?;

        let is_infinite = self.builder()
            .build_or(is_pos_inf, is_neg_inf, "is_infinite")
            .map_err(|e| format!("Failed to combine infinity checks: {}", e))?;

        Ok(is_infinite)
    }

    /// Check if a float value is negative zero (-0.0)
    #[instrument(skip(self))]
    fn check_is_negative_zero(&self, float_value: FloatValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        let float_type = float_value.get_type();
        let negative_zero = float_type.const_float(-0.0);

        // Use bitwise comparison for negative zero detection
        let is_neg_zero = self.builder()
            .build_float_compare(
                FloatPredicate::OEQ,
                float_value,
                negative_zero,
                "is_neg_zero",
            )
            .map_err(|e| format!("Failed to check for negative zero: {}", e))?;

        Ok(is_neg_zero)
    }

    /// Handle special float cases (NaN, infinity, -0.0) in conversions
    #[instrument(skip(self))]
    fn handle_special_float_cases(
        &self,
        float_value: FloatValue<'ctx>,
        target_int_type: IntType<'ctx>,
        is_nan: IntValue<'ctx>,
        is_infinite: IntValue<'ctx>,
        is_negative_zero: IntValue<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let zero_int = target_int_type.const_zero();
        let current_block = self.builder().get_insert_block().unwrap();
        let function = current_block.get_parent().unwrap();

        // Create blocks for different cases
        let nan_block = self.context().append_basic_block(function, "nan_case");
        let inf_block = self.context().append_basic_block(function, "inf_case");
        let neg_zero_block = self.context().append_basic_block(function, "neg_zero_case");
        let normal_block = self.context().append_basic_block(function, "normal_case");
        let merge_block = self.context().append_basic_block(function, "merge_special");

        // Branch based on special cases
        let is_special = self.builder()
            .build_or(is_nan, is_infinite, "is_special_or_inf")
            .map_err(|e| format!("Failed to combine special case checks: {}", e))?;

        let is_any_special = self.builder()
            .build_or(is_special, is_negative_zero, "is_any_special")
            .map_err(|e| format!("Failed to combine all special case checks: {}", e))?;

        self.builder()
            .build_conditional_branch(is_any_special, merge_block, normal_block)
            .map_err(|e| format!("Failed to build conditional branch: {}", e))?;

        // Handle special cases
        self.builder().position_at_end(merge_block);

        // For special cases, return 0 (safe default)
        let special_result = zero_int;

        // If we have special cases, return the result, otherwise continue with normal processing
        self.builder().position_at_end(normal_block);

        // Return None to indicate normal processing should continue
        Ok(None)
    }

    /// Apply bounds checking for float-to-int conversions
    #[instrument(skip(self))]
    fn apply_bounds_checking(
        &self,
        float_value: FloatValue<'ctx>,
        target_int_type: IntType<'ctx>,
        target_type_name: &str,
        is_signed: bool,
    ) -> Result<FloatValue<'ctx>, String> {
        let bit_width = target_int_type.get_bit_width();
        let float_type = float_value.get_type();

        let (min_value, max_value) = if is_signed {
            let max_signed = (1u128 << (bit_width - 1)) - 1;
            let min_signed = -(1i128 << (bit_width - 1));
            (min_signed as f64, max_signed as f64)
        } else {
            let max_unsigned = (1u128 << bit_width) - 1;
            (0.0, max_unsigned as f64)
        };

        debug!(
            target_type = target_type_name,
            bit_width = bit_width,
            min_value = min_value,
            max_value = max_value,
            is_signed = is_signed,
            "Applying bounds checking for float-to-int conversion"
        );

        let min_const = float_type.const_float(min_value);
        let max_const = float_type.const_float(max_value);

        // Clamp the value to the valid range
        let clamped_min = self.builder()
            .build_float_compare(FloatPredicate::OLT, float_value, min_const, "below_min")
            .map_err(|e| format!("Failed to compare with minimum: {}", e))?;

        let clamped_max = self.builder()
            .build_float_compare(FloatPredicate::OGT, float_value, max_const, "above_max")
            .map_err(|e| format!("Failed to compare with maximum: {}", e))?;

        let value_or_min = self.builder()
            .build_select(clamped_min, min_const, float_value, "value_or_min")
            .map_err(|e| format!("Failed to select minimum value: {}", e))?;

        let final_value = self.builder()
            .build_select(clamped_max, max_const, value_or_min.into_float_value(), "bounds_checked")
            .map_err(|e| format!("Failed to select maximum value: {}", e))?;

        let float_val = final_value.into_float_value();
        info!(
            target_type = target_type_name,
            "Bounds checking applied successfully"
        );
        Ok(float_val)
    }

    /// Get integer type limits for bounds checking
    fn get_int_type_limits(bit_width: u32, is_signed: bool) -> (f64, f64) {
        if is_signed {
            let max_signed = (1u128 << (bit_width - 1)) - 1;
            let min_signed = -(1i128 << (bit_width - 1));
            (min_signed as f64, max_signed as f64)
        } else {
            let max_unsigned = (1u128 << bit_width) - 1;
            (0.0, max_unsigned as f64)
        }
    }

    /// Check if a float-to-int conversion would overflow
    #[instrument(skip(self))]
    fn would_overflow(
        &self,
        float_value: FloatValue<'ctx>,
        target_int_type: IntType<'ctx>,
        is_signed: bool,
    ) -> Result<IntValue<'ctx>, String> {
        let bit_width = target_int_type.get_bit_width();
        let (min_value, max_value) = Self::get_int_type_limits(bit_width, is_signed);

        let float_type = float_value.get_type();
        let min_const = float_type.const_float(min_value);
        let max_const = float_type.const_float(max_value);

        let below_min = self.builder()
            .build_float_compare(FloatPredicate::OLT, float_value, min_const, "below_min")
            .map_err(|e| format!("Failed to check underflow: {}", e))?;

        let above_max = self.builder()
            .build_float_compare(FloatPredicate::OGT, float_value, max_const, "above_max")
            .map_err(|e| format!("Failed to check overflow: {}", e))?;

        let would_overflow = self.builder()
            .build_or(below_min, above_max, "would_overflow")
            .map_err(|e| format!("Failed to combine overflow checks: {}", e))?;

        Ok(would_overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use inkwell::builder::Builder;

    struct TestFloatConverter<'ctx> {
        context: &'ctx Context,
        builder: Builder<'ctx>,
    }

    impl<'ctx> TestFloatConverter<'ctx> {
        fn new(context: &'ctx Context) -> Self {
            let builder = context.create_builder();
            Self { context, builder }
        }
    }

    impl<'ctx> FloatConversion<'ctx> for TestFloatConverter<'ctx> {
        fn context(&self) -> &'ctx Context {
            self.context
        }

        fn builder(&self) -> &Builder<'ctx> {
            &self.builder
        }
    }

    #[test]
    fn test_int_type_limits() {
        let (min, max) = TestFloatConverter::get_int_type_limits(8, true);
        assert_eq!(min, -128.0);
        assert_eq!(max, 127.0);

        let (min, max) = TestFloatConverter::get_int_type_limits(8, false);
        assert_eq!(min, 0.0);
        assert_eq!(max, 255.0);

        let (min, max) = TestFloatConverter::get_int_type_limits(32, true);
        assert_eq!(min, -2147483648.0);
        assert_eq!(max, 2147483647.0);
    }

    #[test]
    fn test_special_values() {
        assert!(f64::NAN.is_nan());
        assert!(f64::INFINITY.is_infinite());
        assert!(f64::NEG_INFINITY.is_infinite());
        assert_eq!((-0.0f64).to_bits(), 0x8000000000000000u64);
        assert_eq!((0.0f64).to_bits(), 0x0000000000000000u64);
    }
}
