use std::collections::HashMap;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::core::interface_type_checker::InterfaceTypeChecker;
use cursed::error::Error;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::dynamic_dispatch::InterfaceManager;

// Integration test for interface type conversions and dynamic dispatch


#[path = "tracing_setup.rs"]
mod tracing_setup;

/// Test fixture with two interfaces and multiple implementing types
fn setup_interface_hierarchy() -> Result<TypeChecker, Error> {
    let mut type_checker = TypeChecker::new();
    
    // Register a Reader interface
    type_checker.register_interface(
        "Reader",
        vec![
            ("read".to_string(), vec![], Some(Type::Tea)),
            ("has_more".to_string(), vec![], Some(Type::Lit)),
        ],
        Vec::new(),
    );
    
    // Register a Writer interface
    type_checker.register_interface(
        "Writer",
        vec![
            ("write".to_string(), vec![Type::Tea], None),
            ("flush".to_string(), vec![], None),
        ],
        Vec::new(),
    );
    
    // Register a TextFile struct that implements both interfaces
    let text_file_fields = HashMap::from([
        ("filename".to_string(), Type::Tea),
        ("buffer".to_string(), Type::Tea),
        ("position".to_string(), Type::Normie),
    ]);
    
    type_checker.register_struct("TextFile", text_file_fields, Vec::new())?;
    
    // Register TextFile methods
    let text_file_methods = vec![
        ("read".to_string(), vec![], Some(Type::Tea)),
        ("has_more".to_string(), vec![], Some(Type::Lit)),
        ("write".to_string(), vec![Type::Tea], None),
        ("flush".to_string(), vec![], None),
        ("close".to_string(), vec![], None),
    ];
    
    for (method_name, param_types, return_type) in text_file_methods.clone() {
        type_checker.register_struct_method("TextFile", &method_name, param_types, return_type)?;
    }
    
    // Register a StringBuffer struct that implements Writer
    let buffer_fields = HashMap::from([
        ("data".to_string(), Type::Tea),
    ]);
    
    type_checker.register_struct("StringBuffer", buffer_fields, Vec::new())?;
    
    // Register StringBuffer methods
    let buffer_methods = vec![
        ("write".to_string(), vec![Type::Tea], None),
        ("flush".to_string(), vec![], None),
        ("get_contents".to_string(), vec![], Some(Type::Tea)),
    ];
    
    for (method_name, param_types, return_type) in buffer_methods.clone() {
        type_checker.register_struct_method("StringBuffer", &method_name, param_types, return_type)?;
    }
    
    Ok(type_checker)
}

#[test]
fn test_multiple_interface_implementations() -> Result<(), Error> {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting multiple interface implementation test");
    
    let mut type_checker = setup_interface_hierarchy()?;
    
    // Test if TextFile implements both Reader and Writer
    let text_file_type = Type::Struct("TextFile".to_string(), Vec::new());
    let reader_type = Type::Unknown // Was Interface("Reader".to_string(), Vec::new());
    let writer_type = Type::Unknown // Was Interface("Writer".to_string(), Vec::new());
    
    let implements_reader = type_checker.check_interface_implementation(&text_file_type, &reader_type)?;
    let implements_writer = type_checker.check_interface_implementation(&text_file_type, &writer_type)?;
    
    assert!(implements_reader, "TextFile should implement Reader");
    assert!(implements_writer, "TextFile should implement Writer");
    
    // Test if StringBuffer implements Writer but not Reader
    let buffer_type = Type::Struct("StringBuffer".to_string(), Vec::new());
    
    let buffer_implements_writer = type_checker.check_interface_implementation(&buffer_type, &writer_type)?;
    let buffer_implements_reader = type_checker.check_interface_implementation(&buffer_type, &reader_type)?;
    
    assert!(buffer_implements_writer, "StringBuffer should implement Writer");
    assert!(!buffer_implements_reader, "StringBuffer should not implement Reader");
    
    // Test method resolution
    let reader_read_method = type_checker.resolve_interface_method(&reader_type, "read")?;
    assert!(reader_read_method.is_some(), "read method should be resolved on Reader");
    
    let writer_write_method = type_checker.resolve_interface_method(&writer_type, "write")?;
    assert!(writer_write_method.is_some(), "write method should be resolved on Writer");
    
    tracing::info!("Completed multiple interface implementation test");
    Ok(())
}

#[test]
fn test_interface_vtable_generation() -> Result<(), Error> {
    tracing_setup::init_test_tracing();
    tracing::info!("Starting interface vtable generation test");
    
    // Create LLVM context
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create interface manager
    let mut interface_manager = InterfaceManager::new();
    
    // Register Writer interface
    interface_manager.register_interface(
        &context,
        "Writer",
        vec![
            ("write".to_string(), vec![Type::Tea], None),
            ("flush".to_string(), vec![], None),
        ],
        Vec::new(),
    )?;
    
    // Create method implementations for StringBuffer
    let write_fn_type = context.void_type().fn_type(
        &[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // self
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // string
        ],
        false,
    );
    
    let flush_fn_type = context.void_type().fn_type(
        &[context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()], // self
        false,
    );
    
    let write_fn = module.add_function("StringBuffer_write", write_fn_type, None);
    let flush_fn = module.add_function("StringBuffer_flush", flush_fn_type, None);
    
    // Register methods for the implementation
    let mut method_map = HashMap::new();
    method_map.insert("write".to_string(), write_fn);
    method_map.insert("flush".to_string(), flush_fn);
    
    // Create vtable implementation for StringBuffer -> Writer
    let buffer_type = Type::Struct("StringBuffer".to_string(), Vec::new());
    interface_manager.create_vtable_for_implementation(
        &context,
        &module,
        "Writer",
        &buffer_type,
        method_map,
    )?;
    
    // Verify vtable exists
    let vtable = interface_manager.get_vtable_impl("Writer", "StringBuffer");
    assert!(vtable.is_some(), "VTable should be created for StringBuffer implementing Writer");
    
    // End the vtable test here since we've verified the main components
    // A full test would involve creating a function and properly using the interface
    // but that requires more setup that would be redundant with other tests
    
    tracing::info!("VTable creation successful");
    
    tracing::info!("Completed interface vtable generation test");
    Ok(())
}