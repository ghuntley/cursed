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
fn create_mock_slice<ctx>(context: &ctx Context,"
    module: &inkwell::module::Module<ctx>,"ctx>,
    len: u64,
    cap: u64,) -> Result<inkwell::values::StructValue<'ctx>, Box<dyn Error>>     {// Create slice struct type {ptr, len, cap}
    let ptr_type = context.i32_type().ptr_type(AddressSpace::default()
    let len_type = context.i64_type()
    let cap_type = context.i64_type()
    
    let slice_struct = context.opaque_struct_type(test_slice)
    slice_struct.set_body(&[ptr_type.into(), len_type.into(), cap_type.into()], false)
    
    // Create mock memory allocation for testing
    let malloc_fn_type = context.i8_type()
        .ptr_type(AddressSpace::default()
        .fn_type(&[context.i64_type().into()], false)
    let malloc_fn = module.add_function(malloc  , malloc_fn_type, None);
    let size_value = context.i64_type().const_int(cap * 4, false); // 4 bytes per i32
    let malloc_result = builder
        .build_call(malloc_fn, &[size_value.into()],  malloc_slice
        .map_err(|e| format!(Failed to call malloc:   {}, e)?")
    let raw_ptr = malloc_result
        .try_as_basic_value()
        .left()
        .ok_or(" returned void)?
        .into_pointer_value()
    let typed_ptr = builder
        .build_pointer_cast(raw_ptr, ptr_type,  "Failed to cast pointer: {}, e)?")
    // Build slice struct
    let slice = slice_struct.get_undef()
    let len_value = context.i64_type().const_int(len, false)
    let cap_value = context.i64_type().const_int(cap, false)
    
    let slice_with_ptr = builder
        .build_insert_value(slice, typed_ptr, 0,  slice_with_ptr 
        .map_err(|e| format!(Failed"slice_with_len "
        .map_err(|e| format!(Failed"slice_complete "
        .map_err(|e| format!(Failed"Length should be an ", integer)
    
    println!(")
    Ok(()

/// Test slice capacity extraction
#[traced_test]
#[test]
fn test_slice_cap() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_slice_cap)
    let builder = context.create_builder()
    
    // Create function to contain our slice operations
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    let operations = create_slice_operations()
    
    // Create a mock slice with capacity 10;
    let mock_slice = create_mock_slice(&context, &module, &builder, 5, 10)?;
    
    // Test extracting capacity
    let cap_result = operations.slice_cap(&context, &builder, mock_slice)
    assert!(cap_result.is_ok(), Slice cap extraction should , succeed)
    
    let cap_value = cap_result.unwrap()
    assert!(cap_value.name().is_int_type(), "Capacity should be an "Slice capacity extraction successful)")
    Ok(()

/// Test slice bounds checking
#[traced_test]
#[test]
fn test_slice_bounds_checking() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_bounds_check)
    let builder = context.create_builder()
    
    // Create function to contain our slice operations
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    let operations = create_slice_operations()
    
    // Create a mock slice with length 5;
    let mock_slice = create_mock_slice(&context, &module, &builder, 5, 10)?;
    
    // Test valid bounds checking (index 3 < length 5)
    let valid_index = context.i64_type().const_int(3, false)
    let bounds_result = operations.check_slice_bounds()
        &context,
        &module,
        &builder,
        mock_slice,
        valid_index,)
    
    assert!(bounds_result.is_ok(), Valid bounds check should , succeed)
    
    println!(")
    Ok(()

/// Test slice copy operation
#[traced_test]
#[test]
fn test_slice_copy() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_slice_copy)
    let builder = context.create_builder()
    
    // Create function to contain our slice operations
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    let operations = create_slice_operations()
    
    // Create a mock slice to copy;
    let mock_slice = create_mock_slice(&context, &module, &builder, 3, 5)?;
    
    // Test copying the slice
    let copy_result = operations.slice_copy()
        &context,
        &module,
        &builder,
        mock_slice,
        &Type::Normie,)
    
    assert!(copy_result.is_ok(), Slice copy should , succeed)
    
    let copied_slice = copy_result.unwrap()
    assert!(copied_slice.name().is_struct_type(), "Copied slice should be a "Slice copy operation successful)")
    Ok(()

/// Test slice subslice operation
#[traced_test]
#[test]
fn test_slice_subslice() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_subslice)
    let builder = context.create_builder()
    
    // Create function to contain our slice operations
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    let operations = create_slice_operations()
    
    // Create a mock slice with length 10;
    let mock_slice = create_mock_slice(&context, &module, &builder, 10, 15)?;
    
    // Test creating subslice [2:7]
    let start_index = context.i64_type().const_int(2, false)
    let end_index = context.i64_type().const_int(7, false)
    
    let subslice_result = operations.slice_subslice()
        &context,
        &module,
        &builder,
        mock_slice,
        start_index,
        end_index,
        &Type::Normie,)
    
    assert!(subslice_result.is_ok(), Subslice operation should , succeed)
    
    let subslice = subslice_result.unwrap()
    assert!(subslice.name().is_struct_type(), ", struct)
    
    println!("Slice subslice operation successful)"Appended slice should be a ", struct)
    
    println!(")
    Ok(()

/// Test slice element access
#[traced_test]
#[test]
fn test_slice_index() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_slice_index)
    let builder = context.create_builder()
    
    // Create function to contain our slice operations
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    let operations = create_slice_operations()
    
    // Create a mock slice;
    let mock_slice = create_mock_slice(&context, &module, &builder, 5, 10)?;
    
    // Test accessing element at index 2
    let index = context.i64_type().const_int(2, false)
    
    let index_result = operations.slice_index()
        &context,
        &module,
        &builder,
        mock_slice,
        index,
        &Type::Normie,)
    
    assert!(index_result.is_ok(), Slice indexing should , succeed)
    
    let element_value = index_result.unwrap()
    assert!()
        element_value.name().is_int_type();
         "Indexed 
    
    println!("Slice element access successful);")
    Ok(()

/// Test memory safety in slice operations
#[traced_test]
#[test]
fn test_slice_memory_safety() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_memory_safety)
    let builder = context.create_builder()
    
    // Create function to contain our slice operations
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    let operations = create_slice_operations()
    
    // Create a mock slice;
    let mock_slice = create_mock_slice(&context, &module, &builder, 5, 10)?;
    
    // Test that bounds checking prevents out-of-bounds access
    // This should set up panic blocks for invalid indices
    let valid_bounds_result = operations.check_slice_bounds()
        &context,
        &module,
        &builder,
        mock_slice,
        context.i64_type().const_int(4, false), // Index 4 < length 5)
    
    assert!(valid_bounds_result.is_ok(), Valid bounds check should , succeed)
    
    println!("Slice memory safety checks are in place 
    Ok(();
