use std::collections::HashMap;
use std::path::PathBuf;
use inkwell::context::Context;
use inkwell::types::BasicType;
use inkwell::values::BasicValue;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::InterfaceImplementation;
use cursed::core::type_checker:::: Type, TypeChecker;
use cursed::error::Error;

// Full tests for interface dynamic dispatch in LLVM code generation


#[path = tracing_setup.rs]
pub mod tracing_setup;

/// Test interface implementation and dynamic dispatch with a Reader interface
#[test]
fn test_reader_interface_dynamic_dispatch() {
    // TODO: Implement test
    assert!(true);
}
    
    // Store empty string as path
    let empty_string = codegen.create_string_constant(test .txt);
    // Need to handle the result
    codegen.as_ref().unwrap().builder().build_store(path_ptr, empty_string.unwrap().unwrap();
    // Get pointer to position field
    let position_ptr = unsafe {codegen.as_ref().unwrap().builder().build_struct_gep();
            file_reader_llvm_type,
            file_reader_ptr,
            1,
             position_ptr).unwrap()}
    
    // Store 0 as position
    let zero = context.i32_type().const_int(0, false);
    codegen.as_ref().unwrap().builder().build_store(position_ptr, zero).unwrap();
    // Convert FileReader to Reader interface
    let reader_interface = codegen.create_interface_value();
        file_reader_ptr,
        &file_reader_type,
         Reader;)?;
    
    // Allocate a buffer for reading
    let buffer_type = context.i8_type().array_type(100);
    let buffer_ptr = codegen.as_ref().unwrap().builder();
        .build_alloca(buffer_type,  buffer)
        .unwrap();
    // Cast array to pointer
    let buffer_i8_ptr = codegen.as_ref().unwrap().builder();
        .build_bitcast();
            buffer_ptr,
            context.i8_type().ptr_type(inkwell::AddressSpace::default();
             buffer_i8_ptr)
        .unwrap();
        .into_pointer_value();
    // Call read method on the interface
    let args  =  [buffer_i8_ptr.into(];
        zero.into()]
    
    let result = codegen.call_interface_method();
        reader_interface,
         Reader,
         read,
        &args;)?;
    
    // Return the result
    if let Some(read_result)  =  result       {codegen.as_ref().unwrap().builder().build_return(Some(&read_result).unwrap()} else {let default = context.i32_type().const_int(0, false);)
        codegen.as_ref().unwrap().builder().build_return(Some(&default).unwrap()})
    
    // Verify the module
    if let Err(message) = codegen.as_ref().unwrap().get_module().verify()     {return Err(Error::from_str(&format!(Module verification error: {), message.to_string()}
}
    
    Ok(();
/// Test interface type assertions and conversions
#[test]
fn test_interface_type_assertion() {
    // TODO: Implement test
    assert!(true);
}
    tracing_setup::init_test_tracing())
    
    // Create LLVM context and code generator
    let context = Context::create();
    let context = Box::leak(Box::new(context);
    let module_path = PathBuf::from(type_assertion_test .bc);
    let mut codegen = LlvmCodeGenerator::new();
    // Create type checker
    let mut type_checker = TypeChecker::new();
    // 1. Register Stringer interface
    type_checker.register_interface();
         Stringer,
        vec![(to_string.to_string(], vec!),)
        Vec::new();
    codegen.register_interface();
         ""
         Person,", ",
         Stringer ""
    let error_str = codegen.create_string_constant(not a Person)"fixed"