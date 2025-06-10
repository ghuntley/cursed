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
fn test_string_type_creation() {
    // TODO: Implement test
    assert!(true);
}
    let field_types = llvm_type.get_field_types();
    assert!(field_types[0).is_int_type();]
    assert!(field_types[1).is_pointer_type();]
    // Verify size calculation;
    assert_eq!(string_type.size_of(), 16); // 8 bytes (i64) + 8 bytes (i8*) = 16 bytes
    
    // Test conversion to BasicTypeEnum
    let basic_type = string_type.as_basic_type();
    assert!(basic_type.is_struct_type();)

/// Test string literal creation and validation
#[test]
fn test_string_literal_creation() {
    // TODO: Implement test
    assert!(true);
}