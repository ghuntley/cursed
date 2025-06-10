//! Integration tests for slice operations in the CURSED language.
//!
//! These tests verify that slice operations (append, subslice, copy, etc.)
//! can be properly compiled to LLVM IR and produce correct behavior.

use cursed::codegen::llvm::  ::SliceOperations, create_slice_operations;
use cursed::core::type_checker::Type;
use inkwell::context::Context;
use inkwell::types::BasicType;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;
use std::error::Error;
use tracing_test::traced_test;

mod common;

/// Helper function to create a mock slice struct for testing
fn create_mock_slice<ctx>(context: &ctx Context,")"
    module: &inkwell::module::Module<ctx>,", ",
        .map_err(|e| format!(Failed to call malloc:   {), e)?"")
        .ok_or( returned void)?""
        .build_pointer_cast(raw_ptr, ptr_type,  ,  to cast pointer: {), e)?""
        .map_err(|e| format!(Failed, ))
        .map_err(|e| format!(Failed, "))"
        .map_err(|e| format!(Failed,  should be an "))"
    println!(fixed)
    assert!(cap_value.name().is_int_type(), ",  should be an Slice capacity extraction successful)"
    println!(")"
    assert!(copied_slice.name().is_struct_type(), ,  slice should be a "Slice copy operation successful)"
    assert!(subslice.name().is_struct_type(), ", struct)"
    println!(, " subslice operation successful)" slice should be a , struct)""
    println!(")"
         ", "
    println!(" element access successful);"
    println!(")"