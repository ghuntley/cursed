//! Unit tests for the CURSED string type implementation
//!
//! This module contains comprehensive tests for the LLVM string type definition
//! and its integration with the CURSED type system.

use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

use cursed::codegen::llvm::string_type::{CursedStringType, StringTypeUtils};
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::types::{convert_type, create_basic_type, get_type_size, is_string_type, validate_type_compatibility};

mod common;

/// Test basic string type creation and validation
#[test]
fn test_string_type_creation() {
    common::tracing::setup();
    
    let context = Context::create();
    let string_type = CursedStringType::new(&context);
    
    // Verify the struct has the correct field types
    let llvm_type = string_type.get_llvm_type();
    assert_eq!(llvm_type.count_fields(), 2);
    
    // Check field types: {i64, i8*}
    let field_types = llvm_type.get_field_types();
    assert!(field_types[0].is_int_type());
    assert!(field_types[1].is_pointer_type());
    
    // Verify size calculation
    assert_eq!(string_type.size_of(), 16); // 8 bytes (i64) + 8 bytes (i8*) = 16 bytes
    
    // Test conversion to BasicTypeEnum
    let basic_type = string_type.as_basic_type();
    assert!(basic_type.is_struct_type());
}

/// Test string literal creation and validation
#[test]
fn test_string_literal_creation() {
    common::tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let string_type = CursedStringType::new(&context);
    
    // Create a function to contain our instructions
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Test creating various string literals
    let test_cases = vec![
        ("hello", "test_str1"),
        ("", "test_str2"),  // Empty string
        ("Hello, World! 🌍", "test_str3"),  // Unicode
        ("Line 1\nLine 2\tTabbed", "test_str4"),  // Special characters
    ];
    
    for (literal_value, name) in test_cases {
        let literal = string_type.create_string_literal(&builder, &module, literal_value, name)
            .expect(&format!("Failed to create string literal: '{}'", literal_value));
        
        // Verify the literal is valid
        assert!(string_type.is_valid_string_value(literal.into()));
        
        // Extract length and verify it's correct
        let length = string_type.extract_length(&builder, literal)
            .expect("Failed to extract length");
        
        if let inkwell::values::BasicValueEnum::IntValue(int_val) = length {
            assert_eq!(int_val.get_zero_extended_constant().unwrap(), literal_value.len() as u64);
        } else {
            panic!("Length should be an integer value");
        }
        
        // Extract data pointer and verify it's not null
        let data_ptr = string_type.extract_data_ptr(&builder, literal)
            .expect("Failed to extract data pointer");
        
        // Data pointer should be valid (we can't easily test the actual content in this context)
        assert!(!data_ptr.is_null());
    }
}

/// Test empty string creation
#[test]
fn test_empty_string_creation() {
    common::tracing::setup();
    
    let context = Context::create();
    let builder = context.create_builder();
    let string_type = CursedStringType::new(&context);
    
    // Create a function to contain our instructions
    let module = context.create_module("test");
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create an empty string
    let empty_string = string_type.create_empty_string(&builder)
        .expect("Failed to create empty string");
    
    // Verify it's valid
    assert!(string_type.is_valid_string_value(empty_string.into()));
    
    // Extract length and verify it's zero
    let length = string_type.extract_length(&builder, empty_string)
        .expect("Failed to extract length");
    
    if let inkwell::values::BasicValueEnum::IntValue(int_val) = length {
        assert_eq!(int_val.get_zero_extended_constant().unwrap(), 0);
    } else {
        panic!("Length should be an integer value");
    }
    
    // Extract data pointer (should be null for empty string)
    let data_ptr = string_type.extract_data_ptr(&builder, empty_string)
        .expect("Failed to extract data pointer");
    
    assert!(data_ptr.is_null());
}

/// Test string value creation from components
#[test]
fn test_string_value_creation() {
    common::tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let string_type = CursedStringType::new(&context);
    
    // Create a function to contain our instructions
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create a global string for testing
    let test_string = "test_content";
    let global_str = module.add_global(
        context.i8_type().array_type(test_string.len() as u32),
        None,
        "test_global"
    );
    global_str.set_constant(true);
    global_str.set_initializer(&context.const_string(test_string.as_bytes(), false));
    
    // Cast to i8*
    let data_ptr = builder.build_pointer_cast(
        global_str.as_pointer_value(),
        context.i8_type().ptr_type(inkwell::AddressSpace::default()),
        "test_ptr"
    ).expect("Failed to cast pointer");
    
    // Create length value
    let length = context.i64_type().const_int(test_string.len() as u64, false);
    
    // Create string value from components
    let string_value = string_type.create_string_value(&builder, length.into(), data_ptr)
        .expect("Failed to create string value");
    
    // Verify the string is valid
    assert!(string_type.is_valid_string_value(string_value.into()));
    
    // Extract and verify components
    let extracted_length = string_type.extract_length(&builder, string_value)
        .expect("Failed to extract length");
    
    if let inkwell::values::BasicValueEnum::IntValue(int_val) = extracted_length {
        assert_eq!(int_val.get_zero_extended_constant().unwrap(), test_string.len() as u64);
    } else {
        panic!("Length should be an integer value");
    }
    
    let extracted_ptr = string_type.extract_data_ptr(&builder, string_value)
        .expect("Failed to extract data pointer");
    
    // Pointers should match (same underlying data)
    assert_eq!(data_ptr, extracted_ptr);
}

/// Test type conversion utilities
#[test]
fn test_type_conversion_utilities() {
    common::tracing::setup();
    
    let context = Context::create();
    
    // Test Tea type conversion
    let tea_type = Type::Tea;
    let llvm_type = StringTypeUtils::convert_tea_type_to_llvm(&context, &tea_type)
        .expect("Failed to convert Tea type");
    
    assert!(llvm_type.is_struct_type());
    
    if let inkwell::types::BasicTypeEnum::StructType(struct_type) = llvm_type {
        assert_eq!(struct_type.count_fields(), 2);
        let field_types = struct_type.get_field_types();
        assert!(field_types[0].is_int_type());
        assert!(field_types[1].is_pointer_type());
    }
    
    // Test non-string type conversion (should fail)
    let normie_type = Type::Normie;
    assert!(StringTypeUtils::convert_tea_type_to_llvm(&context, &normie_type).is_err());
    
    // Test type checking
    assert!(StringTypeUtils::is_string_type(&Type::Tea));
    assert!(!StringTypeUtils::is_string_type(&Type::Normie));
    assert!(!StringTypeUtils::is_string_type(&Type::Lit));
    assert!(!StringTypeUtils::is_string_type(&Type::Thicc));
    
    // Test size calculation
    assert_eq!(StringTypeUtils::string_type_size(), 16);
}

/// Test operation validation
#[test]
fn test_operation_validation() {
    common::tracing::setup();
    
    let tea_type = Type::Tea;
    let normie_type = Type::Normie;
    let lit_type = Type::Lit;
    
    // Valid string concatenation
    assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, "+").is_ok());
    
    // Invalid string concatenation (mixed types)
    assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &normie_type, "+").is_err());
    assert!(StringTypeUtils::validate_string_operation_types(&normie_type, &tea_type, "+").is_err());
    
    // Valid string comparisons
    for op in &["==", "!=", "<", ">", "<=", ">="] {
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, op).is_ok());
    }
    
    // Invalid string comparisons (mixed types)
    for op in &["==", "!=", "<", ">", "<=", ">="] {
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &normie_type, op).is_err());
        assert!(StringTypeUtils::validate_string_operation_types(&normie_type, &tea_type, op).is_err());
    }
    
    // Unsupported operations
    for op in &["*", "/", "%", "&&", "||"] {
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, op).is_err());
    }
}

/// Test type caching functionality
#[test]
fn test_type_caching() {
    common::tracing::setup();
    
    let context = Context::create();
    let string_type = CursedStringType::new(&context);
    
    // Get a cached type
    let type1 = string_type.get_or_create_cached_type("test_variant")
        .expect("Failed to get cached type");
    
    // Get the same type again - should be cached
    let type2 = string_type.get_or_create_cached_type("test_variant")
        .expect("Failed to get cached type");
    
    // They should be the same
    assert_eq!(type1, type2);
    
    // Get a different variant
    let type3 = string_type.get_or_create_cached_type("different_variant")
        .expect("Failed to get cached type");
    
    // Should be the same underlying type (since we use the same struct definition)
    assert_eq!(type1, type3);
    
    // Clear cache and verify it's accessible (can't test if it's actually cleared without internal access)
    string_type.clear_cache().expect("Failed to clear cache");
    
    // Should still work after clearing cache
    let type4 = string_type.get_or_create_cached_type("after_clear")
        .expect("Failed to get cached type after clear");
    assert_eq!(type1, type4);
}

/// Test integration with existing type system
#[test]
fn test_type_system_integration() {
    common::tracing::setup();
    
    let context = Context::create();
    
    // Test convert_type function
    let tea_type = Type::Tea;
    let llvm_type = convert_type(&context, &tea_type)
        .expect("Failed to convert Tea type through type system");
    
    assert!(llvm_type.is_struct_type());
    
    // Test create_basic_type function
    let basic_type = create_basic_type(&context, "tea")
        .expect("Failed to create basic tea type");
    
    assert!(basic_type.is_struct_type());
    
    // Test type size function
    assert_eq!(get_type_size(&tea_type), 16);
    
    // Test is_string_type function
    assert!(is_string_type(&tea_type));
    assert!(!is_string_type(&Type::Normie));
    
    // Test validate_type_compatibility function
    assert!(validate_type_compatibility(&tea_type, &tea_type, "+").is_ok());
    assert!(validate_type_compatibility(&tea_type, &Type::Normie, "+").is_err());
}

/// Test string type with different CURSED types
#[test]
fn test_cursed_type_compatibility() {
    common::tracing::setup();
    
    let context = Context::create();
    
    // Test conversion of all basic CURSED types
    let test_types = vec![
        (Type::Lit, "lit"),
        (Type::Normie // Was Smol, "smol"),
        (Type::Normie // Was Mid, "mid"),
        (Type::Normie, "normie"),
        (Type::Thicc, "thicc"),
        (Type::Snack, "snack"),
        (Type::Meal, "meal"),
        (Type::Tea, "tea"),
        (Type::Sip, "sip"),
        (Type::Rune, "rune"),
        (Type::Byte, "byte"),
        (Type::Extra, "extra"),
    ];
    
    for (type_enum, type_name) in test_types {
        // Test convert_type
        let llvm_type = convert_type(&context, &type_enum);
        assert!(llvm_type.is_ok(), "Failed to convert type: {:?}", type_enum);
        
        // Test create_basic_type
        let basic_type = create_basic_type(&context, type_name);
        assert!(basic_type.is_ok(), "Failed to create basic type: {}", type_name);
        
        // The results should have the same structure (but may be different instances)
        let llvm_result = llvm_type.unwrap();
        let basic_result = basic_type.unwrap();
        
        // For struct types (tea, extra), both should be struct types with same field structure
        if type_name == "tea" || type_name == "extra" {
            assert!(llvm_result.is_struct_type());
            assert!(basic_result.is_struct_type());
            
            // Check that they have the same number of fields
            if let (inkwell::types::BasicTypeEnum::StructType(s1), 
                    inkwell::types::BasicTypeEnum::StructType(s2)) = (llvm_result, basic_result) {
                assert_eq!(s1.count_fields(), s2.count_fields());
            }
        } else {
            // For primitive types, they should be exactly equal
            assert_eq!(llvm_result, basic_result);
        }
        
        // Test size calculation
        let size = get_type_size(&type_enum);
        assert!(size > 0, "Type size should be positive for: {:?}", type_enum);
    }
}

/// Test error handling and edge cases
#[test]
fn test_error_handling() {
    common::tracing::setup();
    
    let context = Context::create();
    let string_type = CursedStringType::new(&context);
    
    // Test validation with invalid values
    let i32_value = context.i32_type().const_int(42, false);
    assert!(!string_type.is_valid_string_value(i32_value.into()));
    
    let f64_value = context.f64_type().const_float(3.14);
    assert!(!string_type.is_valid_string_value(f64_value.into()));
    
    // Test invalid type conversion
    let invalid_types = vec![
        Type::Unknown,
        Type::Unknown // Was Named("InvalidType".to_string()),
    ];
    
    for invalid_type in invalid_types {
        let result = StringTypeUtils::convert_tea_type_to_llvm(&context, &invalid_type);
        assert!(result.is_err(), "Should fail to convert invalid type: {:?}", invalid_type);
    }
    
    // Test invalid operations
    let invalid_operations = vec![
        ("*", "Multiplication not supported for strings"),
        ("/", "Division not supported for strings"),
        ("%", "Modulo not supported for strings"),
        ("**", "Exponentiation not supported for strings"),
        ("&&", "Logical AND not supported for strings"),
        ("||", "Logical OR not supported for strings"),
    ];
    
    let tea_type = Type::Tea;
    for (op, _desc) in invalid_operations {
        let result = StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, op);
        assert!(result.is_err(), "Should fail for unsupported operation: {}", op);
    }
}

/// Test memory layout and alignment
#[test]
fn test_memory_layout() {
    common::tracing::setup();
    
    let context = Context::create();
    let string_type = CursedStringType::new(&context);
    let llvm_type = string_type.get_llvm_type();
    
    // Verify struct layout
    assert_eq!(llvm_type.count_fields(), 2);
    
    let field_types = llvm_type.get_field_types();
    
    // First field should be i64 (length)
    assert!(field_types[0].is_int_type());
    if let inkwell::types::BasicTypeEnum::IntType(int_type) = field_types[0] {
        assert_eq!(int_type.get_bit_width(), 64);
    }
    
    // Second field should be i8* (data pointer)
    assert!(field_types[1].is_pointer_type());
    
    // Test struct is not packed (should have proper alignment)
    assert!(!llvm_type.is_packed());
    
    // Test size calculation matches expected layout
    assert_eq!(string_type.size_of(), 16); // i64 + i8* = 8 + 8 = 16 bytes on 64-bit
}

/// Performance test for type creation and caching
#[test]
fn test_performance() {
    common::tracing::setup();
    
    let context = Context::create();
    let string_type = CursedStringType::new(&context);
    
    // Test multiple type creations (should be fast due to caching)
    let iterations = 1000;
    let start = std::time::Instant::now();
    
    for i in 0..iterations {
        let variant_name = format!("perf_test_{}", i % 10); // Create 10 different variants
        let _cached_type = string_type.get_or_create_cached_type(&variant_name)
            .expect("Failed to get cached type");
    }
    
    let duration = start.elapsed();
    
    // Should complete quickly (within reasonable time - this is a smoke test)
    assert!(duration.as_millis() < 1000, "Type caching should be fast, took {:?}", duration);
    
    println!("Created {} type variants in {:?}", iterations, duration);
}
