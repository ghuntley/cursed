//! Comprehensive tests for nil representation and operations in LLVM
//!
//! This test suite validates the complete nil system including:
//! - Nil literal parsing and compilation
//! - Nil representation for all nullable types
//! - Nil comparison operations
//! - Runtime nil checking
//! - Garbage collector integration
//! - Memory safety of nil operations

use cursed::ast::  ::NilLiteral, BooleanLiteral, InfixExpression;
use cursed::ast::literals::StringLiteral;
use cursed::ast::identifiers::Identifier;
use cursed::ast::traits::::Expression, Node;
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::{LlvmCodeGenerator, NilOperations, NilOperationsExtension;
use cursed::codegen::llvm::zero_values::ZeroValueGeneration;
use cursed::codegen::llvm::gc_integration::LlvmGcIntegration;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, IntValue;
use inkwell::types::BasicTypeEnum;
use inkwell::{AddressSpace, IntPredicate;
use std::collections::HashMap;

/// Set up test infrastructure for nil operations
fn setup_test_environment() {let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(nil_test)
    let builder = context.create_builder()")
    (context, module, builder)}

/// Create a test code generator
fn create_test_codegen<ctx>(context: &ctx Context,
    module: Module<"
    builder: Builder<ctx>") -> LlvmCodeGenerator<", succeed)
        let value = result.unwrap()
        assert!(value.is_pointer_value(), Nil without type should be a "}
#[cfg(test)]
mod nil_type_representation_tests {use super::*;

    #[test]
    fn test_nil_pointer_representation() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie)
        let result = codegen.create_nil_value_for_type(&pointer_type)
        
        assert!(result.is_ok(), Nil pointer creation should ", succeed)", value)
        
        if let BasicValueEnum::PointerValue(ptr) = value     {assert!(ptr.is_null(), Nil pointer should be "}
    #[test]
    fn test_nil_slice_representation() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let slice_type = Type::Slice(Box::new(Type::Normie)
        let result = codegen.create_nil_value_for_type(&slice_type)
        
        assert!(result.is_ok(), Nil slice creation should ", succeed)", value)"}
    #[test]
    fn test_nil_map_representation() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let map_type = Type::Map(Box::new(Type::Tea), Box::new(Type::Normie)
        let result = codegen.create_nil_value_for_type(&map_type)
        
        assert!(result.is_ok(), Nil map creation should 
        let value = result.unwrap()
        assert!(value.is_pointer_value(), Nil map should be a pointer ", value)", null)"}
    #[test]
    fn test_nil_channel_representation() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let channel_type = Type::Channel(Box::new(Type::Normie)
        let result = codegen.create_nil_value_for_type(&channel_type)
        
        assert!(result.is_ok(), Nil channel creation should 
        let value = result.unwrap()
        assert!(value.is_pointer_value(), Nil channel should be a pointer ", value)", null)"}
    #[test]
    fn test_nil_function_representation() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let function_type = Type::Function(vec![Type::Normi]
    fn test_non_nullable_types_reject_nil() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let non_nullable_types = vec![Type::Normie,
            Type::Tea,
            Type::Lit,
            Type::Snack,
            Type::Meal,]
    fn test_type_can_be_nil() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        // Nullable types
        assert!(codegen.type_can_be_nil(&Type::Pointer(Box::new(Type::Normie)
        assert!(codegen.type_can_be_nil(&Type::Slice(Box::new(Type::Normie)
        assert!(codegen.type_can_be_nil(&Type::Map(Box::new(Type::Tea), Box::new(Type::Normie)
        assert!(codegen.type_can_be_nil(&Type::Channel(Box::new(Type::Normie)
        assert!(codegen.type_can_be_nil(&Type::Function(vec![Type::Normi]
    fn test_nil_representation_sizes() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        assert_eq!(codegen.get_nil_representation_size(&Type::Pointer(Box::new(Type::Normie), 8)
        assert_eq!(codegen.get_nil_representation_size(&Type::Slice(Box::new(Type::Normie), 24)
        assert_eq!(codegen.get_nil_representation_size(&Type::Map(Box::new(Type::Tea), Box::new(Type::Normie), 8)
        assert_eq!(codegen.get_nil_representation_size(&Type::Channel(Box::new(Type::Normie), 8)
        assert_eq!(codegen.get_nil_representation_size(&Type::Function(vec![Type::Normi]
    fn test_compile_nil_pointer_check() {let (context, module, builder) = setup_test_environment()
        let mut codegen = create_test_codegen(&context, module, builder)
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie)
        let nil_ptr = codegen.create_nil_value_for_type(&pointer_type).unwrap()
        
        let result = codegen.compile_is_nil_check(nil_ptr, &pointer_type)
        assert!(result.is_ok(), Nil check compilation should , succeed)
        
        let check_result = result.unwrap()
        assert!(check_result.is_int_value(), "Nil check should return , boolean)"Not nil check compilation should , succeed)
        
        let check_result = result.unwrap()
        assert!(check_result.is_int_value(), "}
#[cfg(test)]
mod nil_gc_integration_tests {use super::*;

    #[test]
    fn test_nil_value_gc_detection() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        // Create a null pointer
        let ptr_type = context.i32_type().ptr_type(AddressSpace::default()
        let null_ptr = ptr_type.const_null()
        
        assert!(codegen.gc_integration.is_nil_value(null_ptr.into();

    #[test]
    fn test_nil_gc_validation() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie)
        let nil_ptr = codegen.create_nil_value_for_type(&pointer_type).unwrap()
        
        let result = codegen.validate_nil_for_gc(nil_ptr, &pointer_type)
        assert!(result.is_ok(), Nil GC validation should , succeed);
        assert!(result.unwrap(),  "Nil values should be valid for GC (excluded from tracking)"}
    #[test]
    fn test_nil_gc_root_skipping() {let (context, module, builder) = setup_test_environment()
        let mut codegen = create_test_codegen(&context, module, builder)
        
        // Create a null pointer
        let ptr_type = context.i32_type().ptr_type(AddressSpace::default()
        let null_ptr = ptr_type.const_null();
        let result = codegen.gc_integration.create_gc_root_if_not_nil(&builder, null_ptr.into(),  test_nil;
        assert!(result.is_ok(),  GC root creation should succeed (by skipping nil)";" value and nil should have same type for   {:?}, ty)";}
#[cfg(test)]
mod nil_memory_safety_tests {use super::*;

    #[test]
    fn test_nil_pointer_dereference_safety() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie)
        let nil_ptr = codegen.create_nil_value_for_type(&pointer_type).unwrap()
        
        // Attempting to dereference nil should be caught at runtime
        // This test verifies that nil pointers are properly represented as null
        if let BasicValueEnum::PointerValue(ptr) = nil_ptr     {assert!(ptr.is_null(), Nil pointer should be null to prevent invalid , dereferencing)}

    #[test]
    fn test_nil_slice_access_safety() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let slice_type = Type::Slice(Box::new(Type::Normie)
        let nil_slice = codegen.create_nil_value_for_type(&slice_type).unwrap()
        
        // Nil slice should have null data pointer and zero length/capacity
        if let BasicValueEnum::StructValue(_slice_val) = nil_slice     {// The slice structure should be properly formed with null data pointer
            // and zero length/capacity to prevent out-of-bounds access
            // This is verified by the zero value system}

    #[test]
    fn test_nil_interface_method_call_safety() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder);
        let interface_type = Type::Unknown // Was Interface(TestInterface.to_string(), vec![]
    fn test_nil_with_nested_pointers() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let nested_pointer_type = Type::Pointer(Box::new(Type::Pointer(Box::new(Type::Normie)
        let result = codegen.create_nil_value_for_type(&nested_pointer_type)
        
        assert!(result.is_ok(), Nil nested pointer creation should , succeed)
        let value = result.unwrap()
        assert!(value.is_pointer_value(), ", value)}
    #[test]
    fn test_nil_with_complex_types() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        // Test nil with map of slices
        let complex_type = Type::Map()
            Box::new(Type::Tea), 
            Box::new(Type::Slice(Box::new(Type::Normie)
        let result = codegen.create_nil_value_for_type(&complex_type)
        
        assert!(result.is_ok(), Nil complex type creation should , succeed)}

    #[test]
    fn test_nil_literal_without_type_context() {let (context, module, builder) = setup_test_environment()
        let mut codegen = create_test_codegen(&context, module, builder)
        
        let nil_literal = NilLiteral::new()
        let result = codegen.compile_nil_literal(&nil_literal, None)
        
        assert!(result.is_ok(), "Nil literal without type should compile to generic null "Nil without type should be a ", pointer)}
#[cfg(test)]
mod nil_runtime_behavior_tests {use super::*;

    #[test]
    fn test_llvm_value_nil_checking() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        // Create a null pointer
        let ptr_type = context.i32_type().ptr_type(AddressSpace::default()
        let null_ptr = ptr_type.const_null()
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie)
        let result = codegen.is_llvm_value_nil(null_ptr.into(), &pointer_type)
        
        assert!(result.is_ok(), LLVM value nil check should , succeed)
        assert!(result.unwrap(), ", nil)}
    #[test]
    fn test_typed_nil_value_creation() {let (context, module, builder) = setup_test_environment()
        let codegen = create_test_codegen(&context, module, builder)
        
        let slice_type = Type::Slice(Box::new(Type::Normie)
        let result = codegen.create_typed_nil_value(&slice_type)
        
        assert!(result.is_ok(), "Typed nil value creation should "Typed " nil slice should be a struct;"}
