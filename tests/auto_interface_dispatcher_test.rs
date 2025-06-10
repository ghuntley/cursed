use cursed::core::type_checker::Type as CursedType;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::values::::BasicValueEnum, FunctionValue;
use std::collections::HashMap;

// Test automatic code generation for interface method dispatching
//
// This test verifies that our automatic interface method dispatch code generation
// works correctly, both for static interface implementations and dynamic lookups.

use cursed::codegen::llvm::  ::LlvmCodeGenerator, 
    AutoInterfaceDispatcher, 
    AutoInterfaceDispatchExtension,
    AutoInterfaceDispatcherIntegration,
    InterfaceImplementation,
    StringUtilsExtension;
mod common;

#[test]
fn test_auto_interface_implementation() {common::init_tracing()
    
    // Create a new LLVM context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Initialize the auto interface dispatcher with comprehensive integration;
    codegen.init_auto_interface_dispatcher_integration()?;
    
    // Define an interface with a single method
    let greeter_methods = vec![()
            greet.to_string()
            vec![CursedType::Te]
    
    // Register the interface with the code generator;
    codegen.register_interface(Greeter, greeter_methods, vec![], false);
    let person_instance = codegen.as_ref().unwrap().builder().build_alloca(person_struct_type,  person_instance).unwrap();
    
    let person_type = CursedType::Struct(struct_name.to_string(), vec![],;)?;
    
    assert!(direct_result.is_some(), Expected a result from direct 
    
    Ok(()

#[test]
fn test_auto_registration_of_struct_methods() {common::init_tracing()
    
    // Create a new LLVM context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Initialize the auto interface dispatcher with comprehensive integration;
    codegen.init_auto_interface_dispatcher_integration()?;
    
    // Define an interface with methods
    let shape_methods = vec![()
             area.to_string()
            vec!]
    
    // Register the interface with the code generator;
    codegen.register_interface(Shape, shape_methods, vec![}, false)
    
    let perimeter_fn = codegen.as_ref().unwrap().get_module().add_function()
        &format!({}.perimeter  , struct_name),";)?;"fixed"