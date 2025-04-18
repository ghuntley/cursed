//! Test for interface type conversions and method dispatch

use std::collections::HashMap;
use cursed::core::type_checker::{Type, TypeChecker};
// No need for AST expressions
use cursed::error::Error;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::dynamic_dispatch::InterfaceManager;

#[path = "tracing_setup.rs"]
mod tracing_setup;

/// Test fixture to set up a common interface and implementation
fn setup_interface_types() -> Result<TypeChecker, Error> {
    let mut type_checker = TypeChecker::new();
    
    // Register a Printer interface
    type_checker.register_interface(
        "Printer",
        vec![
            ("print".to_string(), vec![Type::Tea], None),
            ("get_name".to_string(), vec![], Some(Type::Tea)),
        ],
        Vec::new(),
    );
    
    // Register ConsoleLogger struct that will implement the interface
    let logger_fields = HashMap::from([
        ("name".to_string(), Type::Tea),
        ("verbose".to_string(), Type::Lit),
    ]);
    
    type_checker.register_struct("ConsoleLogger", logger_fields, Vec::new());
    
    // Register methods for ConsoleLogger
    let logger_methods = vec![
        ("print".to_string(), vec![Type::Tea], None),
        ("get_name".to_string(), vec![], Some(Type::Tea)),
        ("set_verbose".to_string(), vec![Type::Lit], None),
    ];
    
    for (method_name, param_types, return_type) in logger_methods.clone() {
        type_checker.register_struct_method("ConsoleLogger", &method_name, param_types, return_type)?;
    }
    
    Ok(type_checker)
}

#[test]
fn test_interface_type_conversion() -> Result<(), Error> {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting interface type conversion test");
    
    let mut type_checker = setup_interface_types()?;
    
    // Test type conversion: ConsoleLogger -> Printer interface
    let logger_type = Type::Struct("ConsoleLogger".to_string(), Vec::new());
    let printer_type = Type::Interface("Printer".to_string(), Vec::new());
    
    // Check if ConsoleLogger implements Printer
    let implements = type_checker.check_interface_implementation(&logger_type, &printer_type)?;
    assert!(implements, "ConsoleLogger should implement Printer interface");
    
    // Test assignment compatibility
    let can_assign = type_checker.can_assign_to_interface(&logger_type, &printer_type)?;
    assert!(can_assign, "Should be able to assign ConsoleLogger to Printer interface");
    
    tracing::info!("Completed interface type conversion test");
    Ok(())
}

#[test]
fn test_interface_method_dispatch() -> Result<(), Error> {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting interface method dispatch test");
    
    // Create LLVM context for codegen
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create interface manager
    let mut interface_manager = InterfaceManager::new();
    
    // Register interface with manager
    interface_manager.register_interface(
        &context,
        "Printer",
        vec![
            ("print".to_string(), vec![Type::Tea], None),
            ("get_name".to_string(), vec![], Some(Type::Tea)),
        ],
        Vec::new(),
    )?;
    
    // Mock the implementation methods
    let function_type = context.void_type().fn_type(
        &[context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()],
        false,
    );
    
    let print_fn = module.add_function("ConsoleLogger_print", function_type, None);
    let get_name_fn = module.add_function(
        "ConsoleLogger_get_name", 
        context.i8_type().ptr_type(inkwell::AddressSpace::default())
            .fn_type(&[], false), 
        None
    );
    
    // Create a mapping of method names to function values
    let mut implementation_methods = HashMap::new();
    implementation_methods.insert("print".to_string(), print_fn);
    implementation_methods.insert("get_name".to_string(), get_name_fn);
    
    // Create vtable for this implementation
    let logger_type = Type::Struct("ConsoleLogger".to_string(), Vec::new());
    interface_manager.create_vtable_for_implementation(
        &context,
        &module,
        "Printer",
        &logger_type,
        implementation_methods,
    )?;
    
    // Verify the vtable was created properly
    let vtable_impl = interface_manager.get_vtable_impl("Printer", "ConsoleLogger");
    assert!(vtable_impl.is_some(), "VTable implementation should exist");
    
    tracing::info!("Completed interface method dispatch test");
    Ok(())
}