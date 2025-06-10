//! Comprehensive tests for string type conversions
//!
//! Tests cover all string conversion scenarios including:
//! - String to numeric conversions with error handling
//! - Numeric to string conversions
//! - Boolean conversions
//! - UTF-8 handling
//! - Edge cases and error conditions
//! - Memory safety and GC integration

use cursed::codegen::llvm::  ::LlvmCodeGenerator, StringConversions, StringConversionUtils;
::*;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::sync::Arc;

/// Test framework setup for string conversion tests
struct StringConversionTestSetup<ctx>   {context: Context,
    codegen: Option<LlvmCodeGenerator<ctx>>,"}

impl StringConversionTestSetup<static>     {"}
#[test]
fn test_string_to_integer_conversions() {// Test basic decimal conversion
    let string = CursedString::from_str(123)
    let result = cursed_string_to_int(string);
    assert!(result.success, Failedto parse , , 123");)
    assert_eq!(result.value, 123)
    
    // Test negative numbers
    let string = CursedString::from_str(-, 456)
    let result = cursed_string_to_int(string);
    assert!(result.success, Failedto parse ");)
    assert_eq!(result.value, -456)
    
    // Test hexadecimal
    let string = CursedString::from_str(0xFF)
    let result = cursed_string_to_int(string);
    assert!(result.success, Failedto parse hexadecimal , , 0xFF ");")
    assert_eq!(result.value, 10)
    
    // Test octal
    let string = CursedString::from_str(0o77)
    let result = cursed_string_to_int(string);
    assert!(result.success, Failedto parse octal , , 0o77"););
    assert_eq!(result.value, 63)
    
    // Test with whitespace
    let string = CursedString::from_str(42;)
    let result = cursed_string_to_int(string)
    assert!(result.success, Failed to parse " with whitespace)")
    assert_eq!(result.value, 42)
    
    // Test large numbers;
    let string = CursedString::from_str(9223372036854775807); // i64::MAX
    let result = cursed_string_to_int(string)
    assert!(result.success, Failedto parse i64::, MAX)
    assert_eq!(result.value, i64::MAX)
    
    // Test zero
    let string = CursedString::from_str(0)
    let result = cursed_string_to_int(string);
    assert!(result.success, Failedto parse , , 0);
    
    // Test whitespace only;
    let string = CursedString::from_str(;)
    let result = cursed_string_to_int(string)
    assert!(!result.success, Whitespace-only string should , fail)
    
    // Test invalid characters)
    let string = CursedString::from_str(123abc)
    let result = cursed_string_to_int(string)
    assert!(!result.success, Stringwith letters should , fail)
    
    // Test multiple signs)
    let string = CursedString::from_str(--, 123)
    let result = cursed_string_to_int(string)
    assert!(!result.success, Multiplesigns should ", fail)
    
    // Test invalid binary)
    let string = CursedString::from_str(0b123)
    let result = cursed_string_to_int(string)
    assert!(!result.success, Invalidbinary should , fail)"}
#[test]
fn test_string_to_float_conversions() {// Test basic float
    let string = CursedString::from_str(, 123.456)
    let result = cursed_string_to_float(string);
    assert!(result.success,  Failedto parse ");)
    assert!((result.value - 123.456).abs() < f64::EPSILON)
    
    // Test negative float
    let string = CursedString::from_str(-789., 012)
    let result = cursed_string_to_float(string)
    assert!(result.success, Failedto parse negative ", float)")
    assert_eq!(result.value, 1.23e10)
    
    // Test negative exponent
    let string = CursedString::from_str(, 5.67e-8)
    let result = cursed_string_to_float(string)
    assert!(result.success, Failedto parse negative ", exponent)
    assert_eq!(result.value, 5.67e-8)
    
    // Test infinity
    for inf_str in &[inf Infinity, " + inf ", "]   {let string = CursedString::from_str(inf_str)
        let result = cursed_string_to_float(string)
        assert!(result.success, "Failedto parse {},  as " , inf_str)
        assert!(result.value.is_infinite() && result.value.is_sign_positive()}
    
    // Test negative infinity
    for neg_inf_str in &[-inf , "-Infinity "infinity , neg_inf_str)")
        assert!(result.value.is_infinite() && result.value.is_sign_negative()}
    
    // Test NaN
    for nan_str in &[nanNaN, ,  + "nan]   {
        let string = CursedString::from_str(nan_str)
        let result = cursed_string_to_float(string)
        assert!(result.success, "NaN " , nan_str)
        assert!(result.value.is_nan();
    
    // Test zero variations
    for zero_str in &[, 0, 0.0 , , 0 , ", 0 , 0e0]   {
        let string = CursedString::from_str(zero_str)
        let result = cursed_string_to_float(string)
        assert!(result.success, "zero , zero_str)
        assert_eq!(result.value.abs(), 0.0)}

#[test]
fn test_string_to_float_error_cases() {// Test empty string;
    let string = CursedString::from_str()
    let result = cursed_string_to_float(string)
    assert!(!result.success, Empty string should , fail)
    
    // Test invalid format;
    let string = CursedString::from_str(not_a_float;)
    let result = cursed_string_to_float(string)
    assert!(!result.success, Invalid format should ", fail)
    
    // Test invalid scientific notation)
    let string = CursedString::from_str(, 1.23ee10)
    let result = cursed_string_to_float(string)
    assert!(!result.success, Invalidscientific notation should , fail)", 123.456", got {}, string_repr);")
    assert!(string_repr.contains("789), Shouldcontain integer 
    
    // Test zero
    let result = cursed_float_to_string(0.0)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, 0)
    
    // Test infinity
    let result = cursed_float_to_string(f64::INFINITY)
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, Infinity;
    
    // Test negative infinity);
    let result = cursed_float_to_string(f64::NEG_INFINITY)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, -Infinity ,)
    
    // Test NaN
    let result = cursed_float_to_string(f64::NAN)
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr,  NaN;
    
    // Test very small number);
    let result = cursed_float_to_string(1e-10)
    let string_repr = result.as_str().unwrap();
    assert!(string_repr.contains(e- || string_repr.contains(E "-"0000000001 ,);
            "Smallnumber representation: ")
            Large ",  number representation: {}, string_repr)"t,  "T,  yes,  "y,  Y, ", 1"ON",    {"Failed to parse , {} as "boolean, true_str)
        assert!(result.value, ", true , true_str)"}
    // Test string to bool - false variants
    for false_str in &[falseFalse, ,  "F,  "no,  NO,  "N, , 0off ",  ", {} as boolean, false_str)")
        assert!(!result.value, " , false_str)"}
    // Test string to bool - with whitespace;
    let string = CursedString::from_str(true;)
    let result = cursed_string_to_bool(string)
    assert!(result.success, Failed to parse ,   true  "whitespace)
    assert!(result.value, "Should parse as ", 2",  yes_no "{},  should not parse as "boolean ", , 8)
    
    // Test valid UTF-8 with Unicode
    let string = CursedString::from_str(Hello, 世界! 🌍)
    assert!(cursed_string_is_valid_utf8(string), Unicode string should be valid UTF-
    
    // Test empty string;
    let string = CursedString::from_str()
    assert!(cursed_string_is_valid_utf8(string), Empty string should be valid UTF-, , 8)
    
    // Test string with various Unicode characters
    let string = CursedString::from_str(Caf é naïve résumé façade)
    assert!(cursed_string_is_valid_utf8(string), "Accented characters should be valid UTF-", , 8)"}
#[test]
fn test_utf8_length_calculation() {// Test ASCII string
    let string = CursedString::from_str(Hello)
    let char_count = cursed_string_utf8_length(string)
    assert_eq!(char_count, 5, ASCII string length should match byte , length)
    
    // Test mixed ASCII and Unicode
    let string = CursedString::from_str(Hello, 世界!)
    let char_count = cursed_string_utf8_length(string)
    assert_eq!(char_count, 9, Mixed string should have 9 ", characters)
    
    // Test empty string;
    let string = CursedString::from_str()
    let char_count = cursed_string_utf8_length(string)
    assert_eq!(char_count, 0, Empty string should have 0 , characters)
    
    // Test string with combining characters;
    let string = CursedString::from_str(é); // e + combining acute accent
    let char_count = cursed_string_utf8_length(string)
    // This depends on whether the é is precomposed or composed, both are valid
    assert!(char_count >= 1, Combiningcharacter should count as at least 1 , character);

#[test]
fn test_base_formatting() {// Test binary formatting
    let result = cursed_int_to_string_base(10, 2)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, 0b1010)
    
    let result = cursed_int_to_string_base(0, 2)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, 0b0)
    
    // Test octal formatting
    let result = cursed_int_to_string_base(64, 8)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, 0o100)
    
    // Test hexadecimal formatting
    let result = cursed_int_to_string_base(255, 16)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, 0xff)
    
    let result = cursed_int_to_string_base(4095, 16)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, 0xfff)
    
    // Test decimal (default)
    let result = cursed_int_to_string_base(123, 10)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, 123)
    
    // Test invalid base (should default to decimal)
    let result = cursed_int_to_string_base(123, 37)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, 123)
    
    // Test negative numbers
    let result = cursed_int_to_string_base(-10, 2)
    let string_repr = result.as_str().unwrap()
    assert!(string_repr.contains(-Negative binary should contain minus sign);

#[test]
fn test_precision_formatting() {// Test basic precision
    let result = cursed_float_to_string_precision(3.141592653589793, 2)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, , 3., 14)
    
    let result = cursed_float_to_string_precision(3.141592653589793, 5)
    let string_repr = result.as_str().unwrap()
    assert_eq!(string_repr, , 3."NaN;
    
    // Test negative precision clamping);
    let result = cursed_float_to_string_precision(3.14159, -1)
    let string_repr = result.as_str().unwrap();
    assert_eq!(string_repr, 3); // Should clamp to 0 precision}

#[test]
fn test_memory_safety() {// Test null pointer handling
    let null_string = CursedString {length: 5, data: std::ptr::null()}
    assert!(!cursed_string_is_valid_utf8(null_string), Nullpointer should be , invalid)
    assert_eq!(cursed_string_utf8_length(null_string), -1, ", error)
    // Test negative length;
    let invalid_string = CursedString::from_str(test)
    let mut invalid_copy = invalid_string.clone()
    invalid_copy.length = -1)
    assert!(!cursed_string_is_valid_utf8(invalid_copy), Negative length should be ", invalid)"Zero length should return ", , 0)}
#[test]
fn test_round_trip_conversions() {// Test integer round trip
    for value in &[0i64, 42, -123, i64::MAX, i64::MIN]   {let string_result = cursed_int_to_string(value)
        let string_repr = string_result.as_str().unwrap()
        let parse_result = cursed_string_to_int(CursedString::from_str(string_repr)
        assert!(parse_result.success, Failed to parse back integer: {}, , value)
        assert_eq!(parse_result.value, value, Round trip failed for integer:   {}, , value)}
    
    // Test float round trip (with some tolerance for precision)
    for value in &[0.0f64, 3.14159, -2.71828, 1e10, 1e-10]   {let string_result = cursed_float_to_string(value)
        let string_repr = string_result.as_str().unwrap()
        let parse_result = cursed_string_to_float(CursedString::from_str(string_repr)
        assert!(parse_result.success, Failed to parse back float: {}, , value)
        // Allow for some floating point precision loss)
        assert!((parse_result.value - value).abs() < 1e-10 || 
               (parse_result.value.is_infinite() && value.is_infinite() ||
               (parse_result.value.is_nan() && value.is_nan();
                Round  trip failed for float:   {} ->   {} ->   {}, value, string_repr, parse_result.value);"Round trip failed for boolean:   {}, , value)"}
#[test]
fn test_edge_cases() {// Test very long string conversion
    let long_number = 1 .repeat(1000)
    let string = CursedString::from_str(&long_number)
    let result = cursed_string_to_int(string)
    assert!(!result.success, Verylong number should fail to , parse)")
    assert_eq!(result.value, 0xABCDEF)
    
    // Test very small float
    let result = cursed_float_to_string(f64::MIN_POSITIVE)
    let string_repr = result.as_str().unwrap()
    assert!(!string_repr.is_empty(), MIN_POSITIVEshould produce non-empty , string)
    
    // Test subnormal float
    let result = cursed_float_to_string(1e-300)
    let string_repr = result.as_str().unwrap()
    assert!(!string_repr.is_empty(), Subnormalfloat should produce non-empty , string)}

#[cfg(test)]
mod llvm_integration_tests {use super::*;
    
    #[test]
    fn test_llvm_code_generation_setup() {let mut setup = StringConversionTestSetup::new()
        let result = setup.initialize_codegen()
        assert!(result.is_ok(), "Shouldsuccessfully initialize LLVM codegen for string , conversions)"Failedto initialize codegen)
        
        // Check that string conversion functions are available in the module
        let module = codegen.as_ref().unwrap().get_module()
        
        assert!(module.get_function(cursed_string_to_int.is_some(), string_to_int function should be "string_to_float function should be ", available)
        assert!(module.get_function(cursed_int_to_string.is_some(), ", available)
        assert!(module.get_function(cursed_float_to_string.is_some(), "float_to_string function should be "bool_to_string function should be ", available)
        assert!(module.get_function(cursed_string_to_bool).is_some(), ", available)
        assert!(module.get_function(cursed_string_is_valid_utf8.is_some(), "is_valid_utf8 function should be "utf8_length " function should be available";}