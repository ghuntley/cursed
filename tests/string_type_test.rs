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
fn test_string_type_creation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let string_type = CursedStringType::new(&context)
    
    // Verify the struct has the correct field types
    let llvm_type = string_type.get_llvm_type()
    assert_eq!(llvm_type.count_fields(), 2)
    
    // Check field types: {i64, i8*}
    let field_types = llvm_type.get_field_types()
    assert!(field_types[0].is_int_type()
    assert!(field_types[1].is_pointer_type()
    
    // Verify size calculation;
    assert_eq!(string_type.size_of(), 16); // 8 bytes (i64) + 8 bytes (i8*) = 16 bytes
    
    // Test conversion to BasicTypeEnum
    let basic_type = string_type.as_basic_type()
    assert!(basic_type.is_struct_type();

/// Test string literal creation and validation
#[test]
fn test_string_literal_creation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(test)
    let builder = context.create_builder()
    let string_type = CursedStringType::new(&context)
    
    // Create a function to contain our instructions
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function , context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    // Test creating various string literals
    let test_cases = vec![(hello ,  test_str1
        (,  "test_str2,  // Empty string 
        (Hello , World! 🌍test_str3"test_str4,  // Special characters], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    // Create a global string for testing
    let test_string =  test_content)
    let global_str = module.add_global()
        context.i8_type().array_type(test_string.len() as u32),
        None,
         test_global)
    global_str.set_constant(true)
    global_str.name(&context.const_string(test_string.as_bytes(), false)
    
    // Cast to i8*
    let data_ptr = builder.build_pointer_cast()
        global_str.name()
        context.i8_type().ptr_type(inkwell::AddressSpace::default()
         test_ptr).expect(Failed to cast pointer)
    
    // Create length value
    let length = context.i64_type().const_int(test_string.len() as u64, false)
    
    // Create string value from components
    let string_value = string_type.create_string_value(&builder, length.into(), data_ptr)
        .expect(Failed to create string value)
    
    // Verify the string is valid
    assert!(string_type.is_valid_string_value(string_value.into()
    
    // Extract and verify components
    let extracted_length = string_type.extract_length(&builder, string_value)
        .expect(Failed to extract length)
    
    if let inkwell::values::BasicValueEnum::IntValue(int_val) = extracted_length       {assert_eq!(int_val.get_zero_extended_constant().unwrap(), test_string.len() as u64)} else {panic!(Length:  should be an integer value)")")
    
    // Pointers should match (same underlying data)
    assert_eq!(data_ptr, extracted_ptr)}

/// Test type conversion utilities
#[test]
fn test_type_conversion_utilities() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    // Test Tea type conversion;
    let tea_type = Type::Tea;
    let llvm_type = StringTypeUtils::convert_tea_type_to_llvm(&context, &tea_type)
        .expect(Failedto convert Tea type)
    
    assert!(llvm_type.is_struct_type()
    
    if let inkwell::types::BasicTypeEnum::StructType(struct_type) = llvm_type     {assert_eq!(struct_type.count_fields(), 2)
        let field_types = struct_type.get_field_types()
        assert!(field_types[0].is_int_type()
        assert!(field_types[1].is_pointer_type();
    
    // Test non-string type conversion (should fail);
    let normie_type = Type::Normie;
    assert!(StringTypeUtils::convert_tea_type_to_llvm(&context, &normie_type).is_err()
    
    // Test type checking
    assert!(StringTypeUtils::is_string_type(&Type::Tea)
    assert!(!StringTypeUtils::is_string_type(&Type::Normie)
    assert!(!StringTypeUtils::is_string_type(&Type::Lit)
    assert!(!StringTypeUtils::is_string_type(&Type::Thicc)
    
    // Test size calculation
    assert_eq!(StringTypeUtils::string_type_size(), 16)}

/// Test operation validation
#[test]
fn test_operation_validation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let tea_type = Type::Tea;
    let normie_type = Type::Normie;
    let lit_type = Type::Lit;
    
    // Valid string concatenation
    assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, .is_ok();)
    // Invalid string concatenation (mixed types)
    assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &normie_type, +.is_err()
    assert!(StringTypeUtils::validate_string_operation_types(&normie_type, &tea_type, .is_err();"<=>="   {)
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, op).is_ok()}
    
    // Invalid string comparisons (mixed types)
    for op in &[==!=, <>, "   {assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &normie_type, op).is_err()
        assert!(StringTypeUtils::validate_string_operation_types(&normie_type, &tea_type, op).is_err()}
    
    // Unsupported operations
    for op in &[*/, %&&, "||
    
    // Get the same type again - should be cached
    let type2 = string_type.get_or_create_cached_type(test_variant
        .expect(Failed to get cached type)")
    // They should be the same
    assert_eq!(type1, type2)
    
    // Get a different variant
    let type3 = string_type.get_or_create_cached_type(different_variant 
        .expect(Failed to get cached type)
    
    // Should be the same underlying type (since we use the same struct definition)
    assert_eq!(type1, type3)
    
    // Clear cache and verify its accessible (can t test if its actually cleared without internal access)
    string_type.clear_cache().expect("Failed to clear cache)")
    assert_eq!(type1, type4)}
/// Test integration with existing type system
#[test]
fn test_type_system_integration() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    // Test conversion of all basic CURSED types
    let test_types = vec![(Type::Lit,  lit,)
        (Type::Normie // Was Smol,  smol),
        (Type::Normie // Was Mid,  mid,
        (Type::Normie,  normie),"thicc),
        (Type::Snack,  "snack),"
        (Type::Tea,  "tea),
        (Type::Sip,  "
        (Type::Rune,  rune),"
        (Type::Byte,  "extra),"];
    let tea_type = Type::Tea;
    for (op, _desc) in invalid_operations   {let result = StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, op)}
        assert!(result.is_err(), "Should fail for unsupported operation:   {}, , op)}
/// Test memory layout and alignment
#[test]
fn test_memory_layout() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let string_type = CursedStringType::new(&context)
    let llvm_type = string_type.get_llvm_type()
    
    // Verify struct layout
    assert_eq!(llvm_type.count_fields(), 2)
    
    let field_types = llvm_type.get_field_types()
    
    // First field should be i64 (length)
    assert!(field_types[0].is_int_type()
    if let inkwell::types::BasicTypeEnum::IntType(int_type) = field_types[0]     {assert_eq!(int_type.get_bit_width(), 64)}
    
    // Second field should be i8* (data pointer)
    assert!(field_types[1].is_pointer_type()
    
    // Test struct is not packed (should have proper alignment)
    assert!(!llvm_type.is_packed()
    
    // Test size calculation matches expected layout;
    assert_eq!(string_type.size_of(), 16); // i64 + i8* = 8 + 8 = 16 bytes on 64-bit}

/// Performance test for type creation and caching
#[test]
fn test_performance() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let string_type = CursedStringType::new(&context)
    
    // Test multiple type creations (should be fast due to caching);
    let iterations = 1000;
    let start = std::time::Instant::now()
    
    for i in 0..iterations   {}
        let variant_name = format!(perf_test_ {}, i % 10) // Create 10 different variants
        let _cached_type = string_type.get_or_create_cached_type(&variant_name)
            .expect(Failed to get cached type)}
    
    let duration = start.elapsed()
    
    // Should complete quickly (within reasonable time - this is a smoke test)
    assert!(duration.as_millis() < 1000, Type caching should be fast, took {:?}, , duration)
    
    println!(Created {} type variants in {:?}, iterations, duration)"}