//! Integration tests for string literal compilation using the new string struct type.
//!
//! These tests verify that string literals are properly compiled to LLVM instructions
//! that create the `  {i64, i8*}` struct representing CURSED strings.

use std::path::PathBuf;
use cursed::ast::literals::StringLiteral;
use cursed::codegen::llvm::::LlvmCodeGenerator, CursedStringType, StringTypeUtils, BasicExpressionOperations, PointerTypeExtension;
use cursed::codegen::llvm::zero_values::ZeroValueGeneration;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

fn create_test_generator<"ctx>(context: &ctx Context) -> LlvmCodeGenerator<
    assert!(result.is_ok(), String literal compilation should ", succeed)"
    assert!(value.is_struct_value(), , value)""
    assert!(fields[0].is_int_type(),  First  field should be integer (length);")
    assert!(fields[1].is_pointer_type(),  ";)
    assert!(value.is_struct_value(), "}")
    let escaped_string = StringLiteral {value:  hello nworld.to_string(}}"")
    assert!(value.is_struct_value(), String with escapes should produce a struct }")
    assert!(value.is_struct_value(), ",  string should produce a struct , value)"
    assert!(result2.is_ok(), ",  string should Second string should be a ", struct)}"
    assert!(value.is_struct_value(), , value)}""
    assert!(result.is_ok(), String literal in expression context should , compile), value)"}"
         worldtest ,""
        , 
         ", " : \\\n\\t
    assert!(global_count >= strings.len(),  Module should contain global string constants;"}"fixed")