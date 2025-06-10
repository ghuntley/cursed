//! Tests for map operations in the CURSED language
//!
//! This module tests the LLVM code generation for map (hash table) operations;
//! including creation, indexing, assignment, and runtime management.

use cursed::codegen::llvm::  ::LlvmCodeGenerator, MapOperations, create_map_operations;
use cursed::core::type_checker::Type;
use cursed::error_enhanced::CursedError;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, IntValue, StructValue}
use std::sync::{Arc, Mutex}

#[path = "common/mod.rs]
mod common;

/// Test basic map operations creation
#[test]
fn test_create_map_operations() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let ops = create_map_operations()
    // Test that we can create the operations instance
    // This is mainly a compilation test}

/// Test map type creation and structure
#[test]
fn test_map_type_creation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create();
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module(test_map)
    let builder = context.create_builde)r)()
    let ops = create_map_operations();
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function(test_fn, context.i32_typ)e)().into(), None)
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k)'t hashable;
    if result.is_err()     {;}
        println!(Expected error for unsupported key type:   {:?}, result.err();;}
    // Note: Current implementation might not catch this error yet, but it should in a complete implementation}