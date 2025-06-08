//! Integration tests for string literal compilation using the new string struct type.
//!
//! These tests verify that string literals are properly compiled to LLVM instructions
//! that create the `{i64, i8*}` struct representing CURSED strings.

use std::path::PathBuf;
use cursed::ast::expressions::literals::StringLiteral;
use cursed::codegen::llvm::{LlvmCodeGenerator, CursedStringType, StringTypeUtils, BasicExpressionOperations, PointerTypeExtension};
use cursed::codegen::llvm::zero_values::ZeroValueGeneration;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

fn create_test_generator<'ctx>(context: &'ctx Context) -> LlvmCodeGenerator<'ctx> {
    // Use the correct constructor signature
    let mut generator = LlvmCodeGenerator::new(context, "test_string_literals", PathBuf::from("/tmp/test"));
    
    // Create a dummy function so the builder has a place to position itself
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);
    generator.set_current_function(function);
    
    generator
}

#[test]
fn test_string_literal_compilation() {
    let context = Context::create();
    let mut generator = create_test_generator(&context);
    
    // Create a simple string literal
    let string_literal = StringLiteral {
        token: "\"hello\"".to_string(),
        value: "hello".to_string(),
    };
    
    // Compile the string literal
    let result = generator.compile_string_literal(&string_literal);
    match &result {
        Ok(_) => {},
        Err(e) => println!("String literal compilation error: {:?}", e),
    }
    assert!(result.is_ok(), "String literal compilation should succeed");
    
    let value = result.unwrap();
    assert!(value.is_struct_value(), "String literal should produce a struct value");
    
    // Verify the struct has the correct type
    let struct_value = value.into_struct_value();
    let struct_type = struct_value.get_type();
    assert_eq!(struct_type.count_fields(), 2, "String struct should have 2 fields");
    
    // Verify field types
    let fields = struct_type.get_field_types();
    assert!(fields[0].is_int_type(), "First field should be integer (length)");
    assert!(fields[1].is_pointer_type(), "Second field should be pointer (data)");
}

#[test]
fn test_empty_string_literal() {
    let context = Context::create();
    let mut generator = create_test_generator(&context);
    
    // Create an empty string literal
    let empty_string = StringLiteral {
        token: "\"\"".to_string(),
        value: "".to_string(),
    };
    
    // Compile the empty string literal
    let result = generator.compile_string_literal(&empty_string);
    assert!(result.is_ok(), "Empty string literal compilation should succeed");
    
    let value = result.unwrap();
    assert!(value.is_struct_value(), "Empty string should produce a struct value");
}

#[test]
fn test_string_with_escapes() {
    let context = Context::create();
    let mut generator = create_test_generator(&context);
    
    // Create a string literal with escape sequences
    let escaped_string = StringLiteral {
        token: "\"hello\\nworld\"".to_string(),
        value: "hello\\nworld".to_string(),
    };
    
    // Compile the string literal with escapes
    let result = generator.compile_string_literal(&escaped_string);
    assert!(result.is_ok(), "String literal with escapes should compile");
    
    let value = result.unwrap();
    assert!(value.is_struct_value(), "String with escapes should produce a struct value");
}

#[test]
fn test_unicode_string_literal() {
    let context = Context::create();
    let mut generator = create_test_generator(&context);
    
    // Create a string literal with Unicode characters
    let unicode_string = StringLiteral {
        token: "\"café\"".to_string(),
        value: "café".to_string(),
    };
    
    // Compile the Unicode string literal
    let result = generator.compile_string_literal(&unicode_string);
    assert!(result.is_ok(), "Unicode string literal should compile");
    
    let value = result.unwrap();
    assert!(value.is_struct_value(), "Unicode string should produce a struct value");
}

#[test]
fn test_long_string_literal() {
    let context = Context::create();
    let mut generator = create_test_generator(&context);
    
    // Create a long string literal
    let long_string_content = "a".repeat(1000);
    let long_string = StringLiteral {
        token: format!("\"{}\"", long_string_content),
        value: long_string_content,
    };
    
    // Compile the long string literal
    let result = generator.compile_string_literal(&long_string);
    assert!(result.is_ok(), "Long string literal should compile");
    
    let value = result.unwrap();
    assert!(value.is_struct_value(), "Long string should produce a struct value");
}

#[test]
fn test_string_type_consistency() {
    let context = Context::create();
    let generator = create_test_generator(&context);
    
    // Create multiple string types and verify they're consistent
    let string_type1 = CursedStringType::new(&context);
    let string_type2 = CursedStringType::new(&context);
    
    let llvm_type1 = string_type1.get_llvm_type();
    let llvm_type2 = string_type2.get_llvm_type();
    
    // Both should be the same type structure
    assert_eq!(llvm_type1.count_fields(), llvm_type2.count_fields());
    assert_eq!(llvm_type1.count_fields(), 2);
    
    // Verify type size is correct
    assert_eq!(string_type1.size_of(), 16); // 8 bytes for i64 + 8 bytes for pointer
    assert_eq!(string_type2.size_of(), 16);
}

#[test]
fn test_string_deduplication() {
    let context = Context::create();
    let mut generator = create_test_generator(&context);
    
    // Create multiple identical string literals
    let string1 = StringLiteral {
        token: "\"test\"".to_string(),
        value: "test".to_string(),
    };
    
    let string2 = StringLiteral {
        token: "\"test\"".to_string(),
        value: "test".to_string(),
    };
    
    // Compile both strings
    let result1 = generator.compile_string_literal(&string1);
    let result2 = generator.compile_string_literal(&string2);
    
    assert!(result1.is_ok(), "First string should compile");
    assert!(result2.is_ok(), "Second string should compile");
    
    // Both should produce valid struct values
    let value1 = result1.unwrap();
    let value2 = result2.unwrap();
    
    assert!(value1.is_struct_value(), "First string should be a struct");
    assert!(value2.is_struct_value(), "Second string should be a struct");
}

#[test]
fn test_string_type_utilities() {
    // Test the utility functions for string types
    assert_eq!(StringTypeUtils::string_type_size(), 16);
    
    use cursed::core::type_checker::Type;
    assert!(StringTypeUtils::is_string_type(&Type::Tea));
    assert!(!StringTypeUtils::is_string_type(&Type::Normie));
    assert!(!StringTypeUtils::is_string_type(&Type::Lit));
}

#[test]
fn test_zero_value_string() {
    let context = Context::create();
    let generator = create_test_generator(&context);
    
    // Test creating a zero value string
    
    let zero_string = generator.create_empty_string();
    assert!(zero_string.is_ok(), "Should be able to create empty string");
    
    let value = zero_string.unwrap();
    assert!(value.is_struct_value(), "Zero string should be a struct value");
}

#[test]
fn test_string_literal_in_expression_context() {
    let context = Context::create();
    let mut generator = create_test_generator(&context);
    
    // Create a string literal as part of expression compilation
    let string_literal = StringLiteral {
        token: "\"expression\"".to_string(),
        value: "expression".to_string(),
    };
    
    let result = generator.compile_basic_expression(&string_literal);
    assert!(result.is_ok(), "String literal in expression context should compile");
    
    let value = result.unwrap();
    assert!(value.is_struct_value(), "Expression string should be a struct value");
}

#[test]
fn test_string_memory_layout() {
    let context = Context::create();
    
    // Verify the memory layout matches our expectations
    let string_type = CursedStringType::new(&context);
    let llvm_type = string_type.get_llvm_type();
    
    // Should have exactly 2 fields
    assert_eq!(llvm_type.count_fields(), 2);
    
    let fields = llvm_type.get_field_types();
    
    // First field: i64 (length)
    assert!(fields[0].is_int_type());
    let int_type = fields[0].into_int_type();
    assert_eq!(int_type.get_bit_width(), 64);
    
    // Second field: i8* (data pointer)
    assert!(fields[1].is_pointer_type());
    let ptr_type = fields[1].into_pointer_type();
    let pointee_type = ptr_type.get_element_type();
    assert!(pointee_type.is_int_type());
    let pointee_int = pointee_type.into_int_type();
    assert_eq!(pointee_int.get_bit_width(), 8);
}

#[test]
fn test_module_integration() {
    let context = Context::create();
    let mut generator = create_test_generator(&context);
    
    // Compile several string literals to test module integration
    let strings = vec![
        "hello",
        "world",
        "test",
        "",
        "unicode: café",
        "escapes: \\n\\t\\r",
    ];
    
    for (i, content) in strings.iter().enumerate() {
        let string_literal = StringLiteral {
            token: format!("\"{}\"", content),
            value: content.to_string(),
        };
        
        let result = generator.compile_string_literal(&string_literal);
        assert!(result.is_ok(), "String {} should compile: {}", i, content);
    }
    
    // Verify the module contains all the global strings
    let module = generator.module();
    
    // Count globals (should include our string literals)
    let mut global_count = 0;
    let mut global = module.get_first_global();
    while let Some(g) = global {
        global_count += 1;
        global = g.get_next_global();
    }
    
    // Should have at least as many globals as string literals we created
    assert!(global_count >= strings.len(), "Module should contain global string constants");
}
