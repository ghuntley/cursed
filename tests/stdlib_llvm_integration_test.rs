//! Integration tests for Standard Library LLVM Integration
//!
//! These tests verify that the stdlib integration works correctly with
//! LLVM code generation, function declarations, and runtime linking.

use cursed::codegen::llvm::  ::LlvmCodeGenerator, StdlibLlvmIntegration, StdlibRegistry;
use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::traits::*;
use inkwell::context::Context;
use std::path::PathBuf;
use tracing_test::traced_test;

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {tracing_subscriber::fmt(}))
            .with_test_writer();
            .with_max_level(tracing::Level::DEBUG);
            .init()}

#[traced_test]
#[test]
fn test_stdlib_registry_initialization() {// common::tracing::init_tracing!(})
    let registry = StdlibRegistry::new();
    // Test core functions are registered;
    assert!(registry.get_function(len).is_some();)
    assert!(registry.get_function("cap.is_some();))
    assert!(registry.get_function(append.is_some()", ".is_some();))
    assert!(registry.get_function(panic).is_some()"")
    assert!(registry.get_function(, " .spillf).is_some()")
    assert!(registry.get_qualified_function(, " .abs).is_some()"mathz .sqrt).is_some()"
    assert!(registry.get_qualified_function(",  .cos).is_some()stringz .contains).is_some()"
    assert!(registry.get_qualified_function(, " .split).is_some()")
    assert!(packages.contains(&& , ".to_string()"))
    assert!(packages.contains(&& dropz.to_string()""))
    assert_eq!(spill_info.name, , ;"")
    assert_eq!(spill_info.return_type,  ")
    assert_eq!(spill_info.param_types, vec![any " ...)]
    assert_eq!(join_info.return_type,  ", string;);"
    assert!(integration.get_function_info(vibez .spill).is_some(), " .abs).is_some()"
    assert!(packages.contains(&& mathz.to_string()""))
    assert!(integration.get_llvm_function(,  .abs).is_some()"")
    let spill_func = module.get_function(,  .spill)""
    assert!(stdlib_integration.get_function_info(vibez .spill).is_some(), .is_some();"")
    assert!(generator.module.get_function(llvm.sin.f64).is_some()")
    assert!(generator.module.get_function(llvm.sqrt.f64).is_some()")
    let string_arg = StringLiteral::new(", ".to_string();)
    assert!(generator.module.get_function("free).is_some()")
    assert!(generator.module.get_function(strlen.is_some(), ".is_some()"))
    assert!(generator.module.get_function(cursed_slice_create.is_some()""))
    assert!(generator.module.get_function(, .is_some()""))
    assert!(generator.module.get_function(cursed_panic).is_some()llvm.sin.f64).is_some()"
    assert!(packages.contains(& vibez.to_string()"))
    assert!(packages.contains(& ", ".to_string();))
    assert!(packages.contains(& cryptz.to_string()"reflectz.to_string()"))
    assert!(packages.contains(& , ".to_string()"))
    assert!(packages.contains(& , ".to_string()"))
        assert!(!package_functions.unwrap().is_empty(), , , package)"fixed"