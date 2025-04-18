//! Minimal test for interface implementation

use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::InterfaceImplementation;
use std::collections::HashMap;
use std::path::PathBuf;

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

#[test]
fn test_minimal_interface() {
    // Set up tracing
    tracing_setup::init_test_tracing();
    
    // Create a new LLVM context and code generator
    let context = Context::create();
    let module_path = PathBuf::from("minimal_interface_test.bc");
    let mut codegen = LlvmCodeGenerator::new(&context, "minimal_interface_test", module_path);
    
    // Test registering an interface
    let result = codegen.register_interface(
        "Stringer",
        vec![("to_string".to_string(), vec![], Some(cursed::core::type_checker::Type::Tea))],
        Vec::new(),
    );
    
    assert!(result.is_ok(), "Failed to register interface: {:?}", result.err());
    
    // Create a basic struct type
    let struct_type = context.struct_type(
        &[context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()],
        false
    );
    
    // Create a to_string method function
    let fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(
            &[struct_type.ptr_type(inkwell::AddressSpace::default()).into()],
            false
        );
    
    let function = codegen.module().add_function("Person.to_string", fn_type, None);
    
    // Register implementation
    let mut methods = HashMap::new();
    methods.insert("to_string".to_string(), function);
    
    let result = codegen.register_interface_implementation(
        "Person",
        "Stringer",
        methods,
    );
    
    assert!(result.is_ok(), "Failed to register implementation: {:?}", result.err());
}