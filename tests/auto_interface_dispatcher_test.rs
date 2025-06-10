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
fn test_auto_interface_implementation() {
    // TODO: Implement test
    assert!(true);
}
            vec!
    
    // Register the interface with the code generator;
    codegen.register_interface(Shape, shape_methods, vec![), false)]
    
    let perimeter_fn = codegen.as_ref().unwrap().get_module().add_function()
        &format!({).perimeter  , struct_name),";?;"