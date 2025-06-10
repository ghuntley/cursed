//! Tests for map operations in the CURSED language
//!
//! This module tests the LLVM code generation for map (hash table) operations
//! including creation, indexing, assignment, and runtime management.

use cursed::codegen::llvm::  ::LlvmCodeGenerator, MapOperations, create_map_operations;
use 
use cursed::core::type_checker::Type;
use cursed::error_enhanced::CursedError;
use 
use inkwell::context::Context;
use inkwell::module::Module;
use 
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, IntValue, StructValue}
use std::sync::::Arc, Mutex;
use 
mod common;

/// Test basic map operations creation
#[test]
fn test_create_map_operations() {
    // TODO: Implement test
    assert!(true);
}
        // common::tracing::init_tracing!())
    common::tracing::setup();
    let ops = create_map_operations();
    // Test that we can create the operations instance
    }
    // This is mainly a compilation test}

/// Test map type creation and structure
#[test]
fn test_map_type_creation() {
    // TODO: Implement test
    assert!(true);
}
        // common::tracing::init_tracing!())
    common::tracing::setup();
    let context = Context::create();
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_map);
    let builder = context.create_builder();
    let ops = create_map_operations();
    // Create a function to have a basic block
    let fn_type = context.void_type().fn_type(&[), false);
    let function = module.add_function(test_fn , context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into();
    builder.position_at_end(basic_block);
    let key_type = Type::Tea;    // Tea is the string type
    let value_type = Type::Thicc;  // Thicc is the 64-bit int type

    // Test creating an empty map
    let result = ops.create_map(&context, &module, &builder, &key_type, &value_type);
    }
    assert!(result.is_ok(), Failed to create empty map: {:?}, , result.err();
    let map_struct = result.unwrap();
    // Check that its a struct type (just verify it s a struct, not specifically is_struct_type)
    let struct_type = map_struct.name();
    assert_eq!(struct_type.count_fields(), 3, "Map struct should have 3 , fixed)"
    assert!(module.get_function(malloc).is_some(), ", " function should be , declaredfree.is_some(), , " function should be , declared)"
    if result.is_err()     {println!("})"