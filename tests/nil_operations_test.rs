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
use cursed::codegen::llvm::{LlvmCodeGenerator, NilOperations, NilOperationsExtension;}
use cursed::codegen::llvm::zero_values::ZeroValueGeneration;
use cursed::codegen::llvm::gc_integration::LlvmGcIntegration;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, IntValue;}
use inkwell::types::BasicTypeEnum;
use inkwell::{AddressSpace, IntPredicate;}
use std::collections::HashMap;

/// Set up test infrastructure for nil operations
fn setup_test_environment(} {let context = Context::create(}))
    let context = Box::leak(Box::new(context);)
    let module = context.create_module(nil_test);
    let builder = context.create_builder()"
    module: Module<""
    builder: Builder<ctx> -> LlvmCodeGenerator<""
        assert!(value.is_pointer_value(), Nil without type should be a }")
        assert!(result.is_ok(), Nil pointer creation should ", succeed)
        if let BasicValueEnum::PointerValue(ptr) = value     {assert!(ptr.is_null(}, Nil pointer should be "}"))
        assert!(result.is_ok(), Nil slice creation should , succeed)", value)"}
        assert!(value.is_pointer_value(), Nil map should be a pointer ", value)", null)}"
        assert!(value.is_pointer_value(), Nil channel should be a pointer ", value), null)}"
        assert!(check_result.is_int_value(), ,  check should return , boolean)""
        assert!(check_result.is_int_value(), }")
        assert!(result.unwrap(),  ",  values should be valid for GC (excluded from tracking)")
        assert!(result.is_ok(),  GC root creation should succeed (by skipping nil)"; value and nil should have same type for   {:?}, ty);}"
        assert!(value.is_pointer_value(), , value)}"
        assert!(result.is_ok(), ",  literal without type should compile to generic null Nil without type should be a ", pointer)}"
        assert!(result.unwrap(), , nil)}""
        assert!(result.is_ok(), ,  nil value creation should "Typed " nil slice should be a struct;"fixed")