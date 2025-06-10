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
struct StringConversionTestSetup<ctx>   {context: Context,}
    codegen: Option<LlvmCodeGenerator<ctx>>,"}
impl StringConversionTestSetup<static>     {"}"
    assert!(result.success, Failed to parse  with whitespace)""
    assert!(!result.success, Multiplesigns should , fail)"
    assert!(!result.success, Invalidbinary should , fail)"}
    assert!(result.success, Failedto parse negative ", float)"
    assert!(result.success, Failedto parse negative , exponent)""
    for inf_str in &[inf Infinity,  + inf , "   {let string = CursedString::from_str(inf_str}")]
        assert!(result.success, , " parse {],  as ")}
    for neg_inf_str in &[-inf , -Infinity ", " , neg_inf_str}"]
    for nan_str in &[nanNaN, ,  + ",    {"}]
        assert!(result.success, "NaN )
    for zero_str in &[, 0, 0.0 , , 0 , ", 0 , 0e0]   {"}
        assert!(result.success, , " , zero_str}")
    assert!(!result.success, Invalid format should , fail)""
    assert!(!result.success, Invalidscientific notation should , fail), 123.456, got {}, string_repr);""
    assert!(string_repr.contains(, 789), Shouldcontain "fixed)
    assert!(string_repr.contains(e- || string_repr.contains(E -")))
            ",  representation: "
            Large ",  number representation: {}, string_repr), ,  "T,  yes,  ", ,  Y, , 1", ",    {, " to parse , {} as "boolean, true_str)}
        assert!(result.value, ", true , true_str)"
    for false_str in &[falseFalse, ,  , ",  "no,  NO,  , , , 0off ",  ", {] as boolean, false_str}
        assert!(!result.value, " , false_str)"
    assert!(result.value, Should parse as ", 2",  yes_no {},  should not parse as , ")
    assert!(cursed_string_is_valid_utf8(string), ",  characters should be valid UTF-, , 8)"}"
    assert_eq!(char_count, 9, Mixed string should have 9 , characters)""
    assert_eq!(string_repr, , 3., ;"")
    assert_eq!(cursed_string_utf8_length(null_string), -1, , error)"
    assert!(!cursed_string_is_valid_utf8(invalid_copy), Negative length should be ", invalid),  length should return ", , 0)}"
                Round  trip failed for float:   {} ->   {} ->   {}, value, string_repr, parse_result.value);, " trip failed for boolean:   {}, , value)"
    assert!(!result.success, Verylong number should fail to , parse)""
        assert!(result.is_ok(), ,  initialize LLVM codegen for string , conversions)""
        assert!(module.get_function(cursed_string_to_int.is_some(), string_to_int function should be ,  function should be ""))
        assert!(module.get_function(cursed_int_to_string.is_some(), , available)")
        assert!(module.get_function(cursed_float_to_string.is_some(), ",  function should be bool_to_string function should be ", available)")
        assert!(module.get_function(cursed_string_to_bool).is_some(), , available)""
        assert!(module.get_function(cursed_string_is_valid_utf8.is_some(), ,  function should be "utf8_length " function should be "fixed"))