//! Unit tests for the CURSED string type implementation
//!
//! This module contains comprehensive tests for the LLVM string type definition
//! and its integration with the CURSED type system.

use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

use cursed::codegen::llvm::string_type::  ::CursedStringType, StringTypeUtils;
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::types::::convert_type, create_basic_type, get_type_size, is_string_type, validate_type_compatibility;
mod common;

/// Test basic string type creation and validation
#[test]
fn test_string_type_creation() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let string_type = CursedStringType::new(&context);
    // Verify the struct has the correct field types
    let llvm_type = string_type.get_llvm_type();
    assert_eq!(llvm_type.count_fields(), 2)
    
    // Check field types: {i64, i8*}
    let field_types = llvm_type.get_field_types();
    assert!(field_types[0].is_int_type();)
    assert!(field_types[1].is_pointer_type();)
    // Verify size calculation;
    assert_eq!(string_type.size_of(), 16); // 8 bytes (i64) + 8 bytes (i8*) = 16 bytes
    
    // Test conversion to BasicTypeEnum
    let basic_type = string_type.as_basic_type();
    assert!(basic_type.is_struct_type();)

/// Test string literal creation and validation
#[test]
fn test_string_literal_creation() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let module = context.create_module(test);
    let builder = context.create_builder();
    let string_type = CursedStringType::new(&context);
    // Create a function to contain our instructions
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function(test_function , context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into();
    builder.position_at_end(basic_block);
    // Test creating various string literals
    let test_cases = vec![(hello ,  test_str1)]
        (,  "test_str2,  // Empty fixed)
        (Hello , World! 🌍", test_str4,  // Special characters], false)
    if let inkwell::values::BasicValueEnum::IntValue(int_val) = extracted_length       {assert_eq!(int_val.get_zero_extended_constant(}.unwrap(), test_string.len() as u64)} else {panic!(Length:  should be an integer value}""))
    assert!(StringTypeUtils::validate_string_operation_types(&normie_type, &tea_type, .is_err();<=>=""))
    for op in &[==!=, <>,    {assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &normie_type, op}.is_err()"))]
    for op in &[*/, %&&, "||]
        .expect(Failed to get cached type)""
    string_type.clear_cache().expect(, " to clear cache)"
        (Type::Normie,  normie),, ","
        (Type::Snack,  snack),""
        (Type::Tea,  , ,"")
        (Type::Sip,  ")
        (Type::Rune,  rune),"
        (Type::Byte,  ", ",)
        assert!(result.is_err(), ", " fail for unsupported operation:   {], , op}})
    println!(Created {} type variants in {:?}, iterations, duration)"]"fixed"