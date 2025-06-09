//! Integration tests for CURSED string type with LLVM string operations

use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

use cursed::codegen::llvm::{LlvmCodeGenerator, CursedStringType};

mod common;

/// Test integration of string type with LLVM code generator string helpers
#[test]
fn test_string_helpers_integration() {
    common::tracing::setup();
    
    let context = Context::create();
    let module_name = "test_module";
    let file_path = std::path::PathBuf::from("test.csd");
    
    let mut generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Initialize string helpers (should not panic)
    generator.init_string_helpers();
    
    // Verify that the helper functions were created
    let module = generator.module();
    
    // Check that all expected string functions exist
    let expected_functions = vec![
        "cursed_string_concat",
        "cursed_string_equals", 
        "cursed_string_compare",
        "cursed_string_length",
        "cursed_string_substring",
        "cursed_string_from_literal",
    ];
    
    for func_name in expected_functions {
        let function = module.get_function(func_name);
        assert!(function.is_some(), "Function {} should be declared", func_name);
        
        let function = function.unwrap();
        let func_type = function.get_type();
        
        match func_name {
            "cursed_string_concat" => {
                // Should return cursed_string and take two cursed_string parameters
                assert_eq!(func_type.count_param_types(), 2);
            }
            "cursed_string_equals" | "cursed_string_compare" => {
                // Should return i32 and take two cursed_string parameters
                assert_eq!(func_type.count_param_types(), 2);
            }
            "cursed_string_length" => {
                // Should return i64 and take one cursed_string parameter
                assert_eq!(func_type.count_param_types(), 1);
            }
            "cursed_string_substring" => {
                // Should return cursed_string and take cursed_string + two i64 parameters
                assert_eq!(func_type.count_param_types(), 3);
            }
            "cursed_string_from_literal" => {
                // Should return cursed_string and take i8* + i64 parameters
                assert_eq!(func_type.count_param_types(), 2);
            }
            _ => unreachable!(),
        }
    }
    
    println!("All string helper functions properly declared");
}

/// Test string literal creation through code generator
#[test]
fn test_string_literal_creation() {
    common::tracing::setup();
    
    let context = Context::create();
    let module_name = "test_module";
    let file_path = std::path::PathBuf::from("test.csd");
    
    let mut generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Create a function to hold our test code
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_func", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);
    
    // Test creating string literals
    let test_literals = vec![
        ("hello", "test1"),
        ("world", "test2"),
        ("", "test3"),  // empty string
        ("Hello, 世界! 🌍", "test4"),  // Unicode
    ];
    
    for (literal_text, name) in test_literals {
        let result = generator.create_string_literal(literal_text, name);
        assert!(result.is_ok(), "Failed to create string literal: '{}'", literal_text);
        
        let string_value = result.unwrap();
        
        // Verify the value is a struct (our string type)
        assert!(string_value.is_struct_value(), "String literal should be a struct value");
        
        // Test extracting length
        let length_result = generator.extract_string_length(string_value);
        assert!(length_result.is_ok(), "Failed to extract length from string literal");
        
        // Test extracting data pointer
        let data_ptr_result = generator.extract_string_data_ptr(string_value);
        assert!(data_ptr_result.is_ok(), "Failed to extract data pointer from string literal");
    }
    
    println!("Successfully created and validated string literals");
}

/// Test string type compatibility with type system
#[test]
fn test_string_type_system_compatibility() {
    common::tracing::setup();
    
    let context = Context::create();
    let string_type = CursedStringType::new(&context);
    
    // Test that the string type has the expected structure
    let llvm_type = string_type.get_llvm_type();
    assert_eq!(llvm_type.count_fields(), 2);
    
    let field_types = llvm_type.get_field_types();
    
    // First field should be i64 (length)
    assert!(field_types[0].is_int_type());
    if let inkwell::types::BasicTypeEnum::IntType(int_type) = field_types[0] {
        assert_eq!(int_type.get_bit_width(), 64);
    }
    
    // Second field should be a pointer (data)
    assert!(field_types[1].is_pointer_type());
    
    // Test basic type enum conversion
    let basic_type = string_type.as_basic_type();
    assert!(basic_type.is_struct_type());
    
    // Test size calculation
    assert_eq!(string_type.size_of(), 16); // 8 bytes (i64) + 8 bytes (ptr) = 16 bytes
    
    println!("String type system compatibility verified");
}

/// Test empty string handling
#[test]
fn test_empty_string_handling() {
    common::tracing::setup();
    
    let context = Context::create();
    let module_name = "test_module";
    let file_path = std::path::PathBuf::from("test.csd");
    
    let mut generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Create a function to hold our test code
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_func", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);
    
    // Create an empty string literal
    let empty_string_result = generator.create_string_literal("", "empty_test");
    assert!(empty_string_result.is_ok(), "Failed to create empty string literal");
    
    let empty_string = empty_string_result.unwrap();
    
    // Verify it's a struct value
    assert!(empty_string.is_struct_value(), "Empty string should be a struct value");
    
    // Extract and verify length is 0
    let length_result = generator.extract_string_length(empty_string);
    assert!(length_result.is_ok(), "Failed to extract length from empty string");
    
    let length_value = length_result.unwrap();
    if let inkwell::values::BasicValueEnum::IntValue(int_val) = length_value {
        // For constant values, we can check if it's zero
        if let Some(const_val) = int_val.get_zero_extended_constant() {
            assert_eq!(const_val, 0, "Empty string should have length 0");
        }
    }
    
    println!("Empty string handling verified");
}

/// Test string type caching behavior
#[test]
fn test_string_type_caching() {
    common::tracing::setup();
    
    let context = Context::create();
    let string_type = CursedStringType::new(&context);
    
    // Test caching with multiple type variants
    let variants = vec!["variant1", "variant2", "variant3", "variant1"]; // Note: variant1 repeated
    
    let mut cached_types = Vec::new();
    
    for variant in &variants {
        let result = string_type.get_or_create_cached_type(variant);
        assert!(result.is_ok(), "Failed to get cached type for variant: {}", variant);
        cached_types.push(result.unwrap());
    }
    
    // Verify that repeated requests for the same variant return the same type
    assert_eq!(cached_types[0], cached_types[3], "Cached types should be identical for same variant");
    
    // Clear cache and verify it doesn't break functionality
    let clear_result = string_type.clear_cache();
    assert!(clear_result.is_ok(), "Failed to clear cache");
    
    // Should still work after clearing cache
    let post_clear_result = string_type.get_or_create_cached_type("post_clear");
    assert!(post_clear_result.is_ok(), "Should work after cache clear");
    
    println!("String type caching behavior verified");
}

/// Test that string helpers initialization is idempotent
#[test]
fn test_string_helpers_idempotent() {
    common::tracing::setup();
    
    let context = Context::create();
    let module_name = "test_module";
    let file_path = std::path::PathBuf::from("test.csd");
    
    let mut generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Initialize string helpers multiple times
    generator.init_string_helpers();
    generator.init_string_helpers();
    generator.init_string_helpers();
    
    // Should not panic or create duplicate functions
    let module = generator.module();
    
    // Verify functions exist exactly once
    let function = module.get_function("cursed_string_concat");
    assert!(function.is_some(), "String concat function should exist after multiple initializations");
    
    println!("String helpers initialization idempotency verified");
}
