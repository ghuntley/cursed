//! Comprehensive tests for string type conversions
//!
//! Tests cover all string conversion scenarios including:
//! - String to numeric conversions with error handling
//! - Numeric to string conversions
//! - Boolean conversions
//! - UTF-8 handling
//! - Edge cases and error conditions
//! - Memory safety and GC integration

use cursed::codegen::llvm::{LlvmCodeGenerator, StringConversions, StringConversionUtils};
use cursed::runtime::string_conversions::*;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::sync::Arc;

/// Test framework setup for string conversion tests
struct StringConversionTestSetup<'ctx> {
    context: Context,
    codegen: Option<LlvmCodeGenerator<'ctx>>,
}

impl StringConversionTestSetup<'static> {
    fn new() -> Self {
        Self {
            context: Context::create(),
            codegen: None,
        }
    }
    
    fn initialize_codegen(&mut self) -> Result<&mut LlvmCodeGenerator<'static>, Error> {
        if self.codegen.is_none() {
            let codegen = LlvmCodeGenerator::new(
                &self.context,
                "string_conversion_test".to_string(),
                false,
                false,
            )?;
            self.codegen = Some(codegen);
        }
        
        if let Some(ref mut codegen) = self.codegen {
            // Initialize string conversion runtime
            StringConversionUtils::initialize_runtime(codegen)?;
            StringConversionUtils::create_builtin_functions(codegen)?;
            Ok(codegen)
        } else {
            Err(Error::from_str("Failed to initialize codegen"))
        }
    }
}

#[test]
fn test_string_to_integer_conversions() {
    // Test basic decimal conversion
    let string = CursedString::from_str("123");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Failed to parse '123'");
    assert_eq!(result.value, 123);
    
    // Test negative numbers
    let string = CursedString::from_str("-456");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Failed to parse '-456'");
    assert_eq!(result.value, -456);
    
    // Test hexadecimal
    let string = CursedString::from_str("0xFF");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Failed to parse hexadecimal '0xFF'");
    assert_eq!(result.value, 255);
    
    // Test binary
    let string = CursedString::from_str("0b1010");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Failed to parse binary '0b1010'");
    assert_eq!(result.value, 10);
    
    // Test octal
    let string = CursedString::from_str("0o77");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Failed to parse octal '0o77'");
    assert_eq!(result.value, 63);
    
    // Test with whitespace
    let string = CursedString::from_str("  42  ");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Failed to parse '  42  ' with whitespace");
    assert_eq!(result.value, 42);
    
    // Test large numbers
    let string = CursedString::from_str("9223372036854775807"); // i64::MAX
    let result = cursed_string_to_int(string);
    assert!(result.success, "Failed to parse i64::MAX");
    assert_eq!(result.value, i64::MAX);
    
    // Test zero
    let string = CursedString::from_str("0");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Failed to parse '0'");
    assert_eq!(result.value, 0);
}

#[test]
fn test_string_to_integer_error_cases() {
    // Test empty string
    let string = CursedString::from_str("");
    let result = cursed_string_to_int(string);
    assert!(!result.success, "Empty string should fail");
    
    // Test whitespace only
    let string = CursedString::from_str("   ");
    let result = cursed_string_to_int(string);
    assert!(!result.success, "Whitespace-only string should fail");
    
    // Test invalid characters
    let string = CursedString::from_str("123abc");
    let result = cursed_string_to_int(string);
    assert!(!result.success, "String with letters should fail");
    
    // Test multiple signs
    let string = CursedString::from_str("--123");
    let result = cursed_string_to_int(string);
    assert!(!result.success, "Multiple signs should fail");
    
    // Test overflow (too large for i64)
    let string = CursedString::from_str("99999999999999999999999999999");
    let result = cursed_string_to_int(string);
    assert!(!result.success, "Overflow should fail");
    
    // Test invalid hex
    let string = CursedString::from_str("0xGG");
    let result = cursed_string_to_int(string);
    assert!(!result.success, "Invalid hex should fail");
    
    // Test invalid binary
    let string = CursedString::from_str("0b123");
    let result = cursed_string_to_int(string);
    assert!(!result.success, "Invalid binary should fail");
}

#[test]
fn test_string_to_float_conversions() {
    // Test basic float
    let string = CursedString::from_str("123.456");
    let result = cursed_string_to_float(string);
    assert!(result.success, "Failed to parse '123.456'");
    assert!((result.value - 123.456).abs() < f64::EPSILON);
    
    // Test negative float
    let string = CursedString::from_str("-789.012");
    let result = cursed_string_to_float(string);
    assert!(result.success, "Failed to parse negative float");
    assert!((result.value + 789.012).abs() < f64::EPSILON);
    
    // Test scientific notation
    let string = CursedString::from_str("1.23e10");
    let result = cursed_string_to_float(string);
    assert!(result.success, "Failed to parse scientific notation");
    assert_eq!(result.value, 1.23e10);
    
    // Test negative exponent
    let string = CursedString::from_str("5.67e-8");
    let result = cursed_string_to_float(string);
    assert!(result.success, "Failed to parse negative exponent");
    assert_eq!(result.value, 5.67e-8);
    
    // Test infinity
    for inf_str in &["inf", "Infinity", "+inf", "+Infinity"] {
        let string = CursedString::from_str(inf_str);
        let result = cursed_string_to_float(string);
        assert!(result.success, "Failed to parse '{}' as infinity", inf_str);
        assert!(result.value.is_infinite() && result.value.is_sign_positive());
    }
    
    // Test negative infinity
    for neg_inf_str in &["-inf", "-Infinity"] {
        let string = CursedString::from_str(neg_inf_str);
        let result = cursed_string_to_float(string);
        assert!(result.success, "Failed to parse '{}' as negative infinity", neg_inf_str);
        assert!(result.value.is_infinite() && result.value.is_sign_negative());
    }
    
    // Test NaN
    for nan_str in &["nan", "NaN", "+nan", "-nan"] {
        let string = CursedString::from_str(nan_str);
        let result = cursed_string_to_float(string);
        assert!(result.success, "Failed to parse '{}' as NaN", nan_str);
        assert!(result.value.is_nan());
    }
    
    // Test zero variations
    for zero_str in &["0", "0.0", "+0", "-0", "0e0"] {
        let string = CursedString::from_str(zero_str);
        let result = cursed_string_to_float(string);
        assert!(result.success, "Failed to parse '{}' as zero", zero_str);
        assert_eq!(result.value.abs(), 0.0);
    }
}

#[test]
fn test_string_to_float_error_cases() {
    // Test empty string
    let string = CursedString::from_str("");
    let result = cursed_string_to_float(string);
    assert!(!result.success, "Empty string should fail");
    
    // Test invalid format
    let string = CursedString::from_str("not_a_float");
    let result = cursed_string_to_float(string);
    assert!(!result.success, "Invalid format should fail");
    
    // Test multiple decimal points
    let string = CursedString::from_str("12.34.56");
    let result = cursed_string_to_float(string);
    assert!(!result.success, "Multiple decimal points should fail");
    
    // Test invalid scientific notation
    let string = CursedString::from_str("1.23ee10");
    let result = cursed_string_to_float(string);
    assert!(!result.success, "Invalid scientific notation should fail");
}

#[test]
fn test_integer_to_string_conversions() {
    // Test positive integer
    let result = cursed_int_to_string(123);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "123");
    
    // Test negative integer
    let result = cursed_int_to_string(-456);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "-456");
    
    // Test zero
    let result = cursed_int_to_string(0);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "0");
    
    // Test large numbers
    let result = cursed_int_to_string(i64::MAX);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "9223372036854775807");
    
    let result = cursed_int_to_string(i64::MIN);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "-9223372036854775808");
}

#[test]
fn test_float_to_string_conversions() {
    // Test basic float
    let result = cursed_float_to_string(123.456);
    let string_repr = result.as_str().unwrap();
    assert!(string_repr.contains("123.456"), "Float representation should contain '123.456', got '{}'", string_repr);
    
    // Test negative float
    let result = cursed_float_to_string(-789.012);
    let string_repr = result.as_str().unwrap();
    assert!(string_repr.starts_with('-'), "Negative float should start with '-'");
    assert!(string_repr.contains("789"), "Should contain integer part");
    
    // Test zero
    let result = cursed_float_to_string(0.0);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "0");
    
    // Test infinity
    let result = cursed_float_to_string(f64::INFINITY);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "Infinity");
    
    // Test negative infinity
    let result = cursed_float_to_string(f64::NEG_INFINITY);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "-Infinity");
    
    // Test NaN
    let result = cursed_float_to_string(f64::NAN);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "NaN");
    
    // Test very small number
    let result = cursed_float_to_string(1e-10);
    let string_repr = result.as_str().unwrap();
    assert!(string_repr.contains("e-") || string_repr.contains("E-") || string_repr == "0.0000000001", 
           "Small number representation: '{}'", string_repr);
    
    // Test very large number
    let result = cursed_float_to_string(1e20);
    let string_repr = result.as_str().unwrap();
    assert!(string_repr.contains("e") || string_repr.contains("E") || string_repr.len() > 10,
           "Large number representation: '{}'", string_repr);
}

#[test]
fn test_boolean_conversions() {
    // Test bool to string
    let result = cursed_bool_to_string(true);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "true");
    
    let result = cursed_bool_to_string(false);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "false");
    
    // Test string to bool - true variants
    for true_str in &["true", "True", "TRUE", "t", "T", "yes", "YES", "y", "Y", "1", "on", "ON"] {
        let string = CursedString::from_str(true_str);
        let result = cursed_string_to_bool(string);
        assert!(result.success, "Failed to parse '{}' as boolean", true_str);
        assert!(result.value, "'{}' should parse as true", true_str);
    }
    
    // Test string to bool - false variants
    for false_str in &["false", "False", "FALSE", "f", "F", "no", "NO", "n", "N", "0", "off", "OFF"] {
        let string = CursedString::from_str(false_str);
        let result = cursed_string_to_bool(string);
        assert!(result.success, "Failed to parse '{}' as boolean", false_str);
        assert!(!result.value, "'{}' should parse as false", false_str);
    }
    
    // Test string to bool - with whitespace
    let string = CursedString::from_str("  true  ");
    let result = cursed_string_to_bool(string);
    assert!(result.success, "Failed to parse '  true  ' with whitespace");
    assert!(result.value, "Should parse as true");
    
    // Test string to bool - invalid
    for invalid_str in &["maybe", "tru", "fals", "2", "yes_no", ""] {
        let string = CursedString::from_str(invalid_str);
        let result = cursed_string_to_bool(string);
        assert!(!result.success, "'{}' should not parse as boolean", invalid_str);
    }
}

#[test]
fn test_utf8_validation() {
    // Test valid ASCII
    let string = CursedString::from_str("Hello, World!");
    assert!(cursed_string_is_valid_utf8(string), "ASCII should be valid UTF-8");
    
    // Test valid UTF-8 with Unicode
    let string = CursedString::from_str("Hello, 世界! 🌍");
    assert!(cursed_string_is_valid_utf8(string), "Unicode string should be valid UTF-8");
    
    // Test empty string
    let string = CursedString::from_str("");
    assert!(cursed_string_is_valid_utf8(string), "Empty string should be valid UTF-8");
    
    // Test string with various Unicode characters
    let string = CursedString::from_str("Café naïve résumé façade");
    assert!(cursed_string_is_valid_utf8(string), "Accented characters should be valid UTF-8");
    
    // Test emojis and symbols
    let string = CursedString::from_str("🚀 ✨ 🎉 → ∞ ≈ ≠");
    assert!(cursed_string_is_valid_utf8(string), "Emojis and symbols should be valid UTF-8");
}

#[test]
fn test_utf8_length_calculation() {
    // Test ASCII string
    let string = CursedString::from_str("Hello");
    let char_count = cursed_string_utf8_length(string);
    assert_eq!(char_count, 5, "ASCII string length should match byte length");
    
    // Test Unicode string
    let string = CursedString::from_str("世界");
    let char_count = cursed_string_utf8_length(string);
    assert_eq!(char_count, 2, "Unicode string should have 2 characters, not 6 bytes");
    
    // Test mixed ASCII and Unicode
    let string = CursedString::from_str("Hello, 世界!");
    let char_count = cursed_string_utf8_length(string);
    assert_eq!(char_count, 9, "Mixed string should have 9 characters");
    
    // Test with emojis
    let string = CursedString::from_str("🌍🚀");
    let char_count = cursed_string_utf8_length(string);
    assert_eq!(char_count, 2, "Two emojis should count as 2 characters");
    
    // Test empty string
    let string = CursedString::from_str("");
    let char_count = cursed_string_utf8_length(string);
    assert_eq!(char_count, 0, "Empty string should have 0 characters");
    
    // Test string with combining characters
    let string = CursedString::from_str("é"); // e + combining acute accent
    let char_count = cursed_string_utf8_length(string);
    // This depends on whether the é is precomposed or composed, both are valid
    assert!(char_count >= 1, "Combining character should count as at least 1 character");
}

#[test]
fn test_base_formatting() {
    // Test binary formatting
    let result = cursed_int_to_string_base(10, 2);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "0b1010");
    
    let result = cursed_int_to_string_base(0, 2);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "0b0");
    
    // Test octal formatting
    let result = cursed_int_to_string_base(64, 8);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "0o100");
    
    // Test hexadecimal formatting
    let result = cursed_int_to_string_base(255, 16);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "0xff");
    
    let result = cursed_int_to_string_base(4095, 16);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "0xfff");
    
    // Test decimal (default)
    let result = cursed_int_to_string_base(123, 10);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "123");
    
    // Test invalid base (should default to decimal)
    let result = cursed_int_to_string_base(123, 37);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "123");
    
    // Test negative numbers
    let result = cursed_int_to_string_base(-10, 2);
    let string_repr = result.as_str().unwrap();
    assert!(string_repr.contains("-"), "Negative binary should contain minus sign");
}

#[test]
fn test_precision_formatting() {
    // Test basic precision
    let result = cursed_float_to_string_precision(3.141592653589793, 2);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "3.14");
    
    let result = cursed_float_to_string_precision(3.141592653589793, 5);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "3.14159");
    
    // Test zero precision
    let result = cursed_float_to_string_precision(3.7, 0);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "4"); // Should round
    
    // Test high precision
    let result = cursed_float_to_string_precision(1.23456789, 8);
    let string_repr = result.as_str().unwrap();
    assert!(string_repr.len() > 8, "High precision should produce longer string");
    
    // Test special values maintain their representation
    let result = cursed_float_to_string_precision(f64::INFINITY, 5);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "Infinity");
    
    let result = cursed_float_to_string_precision(f64::NAN, 3);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "NaN");
    
    // Test negative precision clamping
    let result = cursed_float_to_string_precision(3.14159, -1);
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, "3"); // Should clamp to 0 precision
}

#[test]
fn test_memory_safety() {
    // Test null pointer handling
    let null_string = CursedString { length: 5, data: std::ptr::null() };
    assert!(!cursed_string_is_valid_utf8(null_string), "Null pointer should be invalid");
    assert_eq!(cursed_string_utf8_length(null_string), -1, "Null pointer should return error");
    
    // Test negative length
    let invalid_string = CursedString::from_str("test");
    let mut invalid_copy = invalid_string.clone();
    invalid_copy.length = -1;
    assert!(!cursed_string_is_valid_utf8(invalid_copy), "Negative length should be invalid");
    
    // Test zero length
    let zero_string = CursedString { length: 0, data: std::ptr::null() };
    assert!(cursed_string_is_valid_utf8(zero_string), "Zero length should be valid (empty string)");
    assert_eq!(cursed_string_utf8_length(zero_string), 0, "Zero length should return 0");
}

#[test]
fn test_round_trip_conversions() {
    // Test integer round trip
    for value in &[0i64, 42, -123, i64::MAX, i64::MIN] {
        let string_result = cursed_int_to_string(*value);
        let string_repr = string_result.as_str().unwrap();
        let parse_result = cursed_string_to_int(CursedString::from_str(string_repr));
        assert!(parse_result.success, "Failed to parse back integer: {}", value);
        assert_eq!(parse_result.value, *value, "Round trip failed for integer: {}", value);
    }
    
    // Test float round trip (with some tolerance for precision)
    for value in &[0.0f64, 3.14159, -2.71828, 1e10, 1e-10] {
        let string_result = cursed_float_to_string(*value);
        let string_repr = string_result.as_str().unwrap();
        let parse_result = cursed_string_to_float(CursedString::from_str(string_repr));
        assert!(parse_result.success, "Failed to parse back float: {}", value);
        // Allow for some floating point precision loss
        assert!((parse_result.value - *value).abs() < 1e-10 || 
               (parse_result.value.is_infinite() && value.is_infinite()) ||
               (parse_result.value.is_nan() && value.is_nan()),
               "Round trip failed for float: {} -> {} -> {}", value, string_repr, parse_result.value);
    }
    
    // Test boolean round trip
    for value in &[true, false] {
        let string_result = cursed_bool_to_string(*value);
        let string_repr = string_result.as_str().unwrap();
        let parse_result = cursed_string_to_bool(CursedString::from_str(string_repr));
        assert!(parse_result.success, "Failed to parse back boolean: {}", value);
        assert_eq!(parse_result.value, *value, "Round trip failed for boolean: {}", value);
    }
}

#[test]
fn test_edge_cases() {
    // Test very long string conversion
    let long_number = "1".repeat(1000);
    let string = CursedString::from_str(&long_number);
    let result = cursed_string_to_int(string);
    assert!(!result.success, "Very long number should fail to parse");
    
    // Test string with many leading zeros
    let string = CursedString::from_str("000000000000123");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Leading zeros should parse correctly");
    assert_eq!(result.value, 123);
    
    // Test hex with mixed case
    let string = CursedString::from_str("0xAbCdEf");
    let result = cursed_string_to_int(string);
    assert!(result.success, "Mixed case hex should parse");
    assert_eq!(result.value, 0xABCDEF);
    
    // Test very small float
    let result = cursed_float_to_string(f64::MIN_POSITIVE);
    let string_repr = result.as_str().unwrap();
    assert!(!string_repr.is_empty(), "MIN_POSITIVE should produce non-empty string");
    
    // Test subnormal float
    let result = cursed_float_to_string(1e-300);
    let string_repr = result.as_str().unwrap();
    assert!(!string_repr.is_empty(), "Subnormal float should produce non-empty string");
}

#[cfg(test)]
mod llvm_integration_tests {
    use super::*;
    
    #[test]
    fn test_llvm_code_generation_setup() {
        let mut setup = StringConversionTestSetup::new();
        let result = setup.initialize_codegen();
        assert!(result.is_ok(), "Should successfully initialize LLVM codegen for string conversions");
    }
    
    #[test]
    fn test_runtime_function_availability() {
        let mut setup = StringConversionTestSetup::new();
        let codegen = setup.initialize_codegen().expect("Failed to initialize codegen");
        
        // Check that string conversion functions are available in the module
        let module = codegen.module();
        
        assert!(module.get_function("cursed_string_to_int").is_some(), "string_to_int function should be available");
        assert!(module.get_function("cursed_string_to_float").is_some(), "string_to_float function should be available");
        assert!(module.get_function("cursed_int_to_string").is_some(), "int_to_string function should be available");
        assert!(module.get_function("cursed_float_to_string").is_some(), "float_to_string function should be available");
        assert!(module.get_function("cursed_bool_to_string").is_some(), "bool_to_string function should be available");
        assert!(module.get_function("cursed_string_to_bool").is_some(), "string_to_bool function should be available");
        assert!(module.get_function("cursed_string_is_valid_utf8").is_some(), "is_valid_utf8 function should be available");
        assert!(module.get_function("cursed_string_utf8_length").is_some(), "utf8_length function should be available");
    }
}
