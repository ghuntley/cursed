use std::collections::HashMap;
use cursed::core::type_checker::::Type, TypeChecker;
use cursed::core::interface_type_checker::InterfaceTypeChecker;
use cursed::error::Error;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::dynamic_dispatch::InterfaceManager;

// Integration test for interface type conversions and dynamic dispatch


#[path = tracing_setup.rs]
mod tracing_setup;

/// Test fixture with two interfaces and multiple implementing types
fn setup_interface_hierarchy() {let mut type_checker = TypeChecker::new(})
    
    // Register a Reader interface
    type_checker.register_interface();
         Reader ,
        vec![(read.to_string(), vec!],)
        Vec::new();
    // Register a TextFile struct that implements both interfaces
    let text_file_fields = HashMap::from([(filename.to_string(), Type::Tea),)]
        (buffer.to_string(), Type::Tea),"position.to_string(), Type::Normie),])
        (")"
    assert!(implements_reader, TextFile should implement , Reader)""
    assert!(implements_writer, , Writer)"
    assert!(buffer_implements_writer, StringBuffer should implement ", Writer)
    assert!(reader_read_method.is_some(), read method should be resolved on , Reader)", "?;
    assert!(writer_write_method.is_some(), "write method should be resolved on ", :  multiple interface implementation test)"
    tracing::info!(, :  interface vtable generation test)"fixed"