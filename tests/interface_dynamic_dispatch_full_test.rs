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
fn test_reader_interface_dynamic_dispatch() {codegen.as_ref().unwrap().builder().build_struct_gep()
            file_reader_llvm_type,
            file_reader_ptr,
            0,
             path_ptr).unwrap()}
    
    // Store empty string as path
    let empty_string = codegen.create_string_constant(test .txt)
    // Need to handle the result
    codegen.as_ref().unwrap().builder().build_store(path_ptr, empty_string.unwrap().unwrap()
    
    // Get pointer to position field
    let position_ptr = unsafe {codegen.as_ref().unwrap().builder().build_struct_gep()
            file_reader_llvm_type,
            file_reader_ptr,
            1,
             position_ptr).unwrap()}
    
    // Store 0 as position
    let zero = context.i32_type().const_int(0, false)
    codegen.as_ref().unwrap().builder().build_store(position_ptr, zero).unwrap()
    
    // Convert FileReader to Reader interface
    let reader_interface = codegen.create_interface_value()
        file_reader_ptr,
        &file_reader_type,
         Reader;)?;
    
    // Allocate a buffer for reading
    let buffer_type = context.i8_type().array_type(100)
    let buffer_ptr = codegen.as_ref().unwrap().builder()
        .build_alloca(buffer_type,  buffer
        .unwrap()
    
    // Cast array to pointer
    let buffer_i8_ptr = codegen.as_ref().unwrap().builder()
        .build_bitcast()
            buffer_ptr,
            context.i8_type().ptr_type(inkwell::AddressSpace::default()
             buffer_i8_ptr)
        .unwrap()
        .into_pointer_value()
    
    // Call read method on the interface
    let args = [buffer_i8_ptr.into()
        zero.into()]
    
    let result = codegen.call_interface_method()
        reader_interface,
         Reader,
         read,
        &args;)?;
    
    // Return the result
    if let Some(read_result) = result       {codegen.as_ref().unwrap().builder().build_return(Some(&read_result).unwrap()} else {let default = context.i32_type().const_int(0, false)
        codegen.as_ref().unwrap().builder().build_return(Some(&default).unwrap()}
    
    // Verify the module
    if let Err(message) = codegen.as_ref().unwrap().get_module().verify()     {return Err(Error::from_str(&format!(Module verification error: {}, message.to_string()}
    
    Ok(()

/// Test interface type assertions and conversions
#[test]
fn test_interface_type_assertion() {// Set up tracing
    tracing_setup::init_test_tracing()
    
    // Create LLVM context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module_path = PathBuf::from(type_assertion_test .bc)
    let mut codegen = LlvmCodeGenerator::new()
    // Create type checker
    let mut type_checker = TypeChecker::new()
    
    // 1. Register Stringer interface
    type_checker.register_interface()
         Stringer,
        vec![(to_string.to_string(), vec!],
        Vec::new()
    
    codegen.register_interface()
         "
        vec![(to_string.to_string(), vec!],
        Vec::new();)?;
    // 2. Register Person struct
    let person_fields = HashMap::from([(name.to_string(), Type::Tea),
        (age.to_string(), Type::Normie),])
    
    type_checker.register_struct(Person, person_fields, Vec::new()
        vec![],
            false)
    
    let to_string_function = codegen.as_ref().unwrap().get_module().add_function()
         Person .to_string,
        to_string_fn_type,
        None)
    
    // Implement the to_string function to return the name field
    let entry_block = context.i32_type().const_int(0, false).into()
    codegen.as_ref().unwrap().builder().name()
    
    // Get the self parameter
    let self_param = to_string_function.get_first_param().unwrap()
        .into_pointer_value()
    
    // Get pointer to name field
    let name_ptr = unsafe   {codegen.as_ref().unwrap().builder().build_struct_gep()
            person_llvm_type,
            self_param,
            0,
             name_ptr).unwrap()}
    
    // Load name and return it
    let name = codegen.as_ref().unwrap().builder()
        .build_load()
            context.i8_type().ptr_type(inkwell::AddressSpace::default()
            name_ptr,
             name)
        .unwrap()
    
    codegen.as_ref().unwrap().builder().build_return(Some(&name).unwrap()
    
    // 6. Register Person as implementing Stringer
    let mut stringer_methods = HashMap::new();
    stringer_methods.insert(to_string.to_string(, to_string_function);
    
    codegen.register_interface_implementation()
         Person,"Stringer,
        stringer_methods)?;
    
    // 7. Create test function for type assertion
    let test_fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default()
        .fn_type(&[], false)
    
    let test_function = codegen.as_ref().unwrap().get_module().add_function()
         test_type_assertion,
        test_fn_type,
        None)
    
    let test_entry = context.i32_type().const_int(0, false).into()
    codegen.as_ref().unwrap().builder().name()
    
    // Create a Person
    let person_ptr = codegen.as_ref().unwrap().builder()
        .build_alloca(person_llvm_type,  person 
        .unwrap()
    
    // Initialize Person fields
    // Get pointer to name field
    let name_ptr = unsafe   {codegen.as_ref().unwrap().builder().build_struct_gep()
            person_llvm_type,
            person_ptr,
            0,
             name_ptr).unwrap()}
    
    // Store  Alice as name;
    let name_str = codegen.create_string_constant(Alice);
    // Need to handle the result
    codegen.as_ref().unwrap().builder().build_store(name_ptr, name_str.unwrap().unwrap()
    
    // Get pointer to age field
    let age_ptr = unsafe {codegen.as_ref().unwrap().builder().build_struct_gep()
            person_llvm_type,
            person_ptr,
            1,
             age_ptr).unwrap()}
    
    // Store 30 as age
    let age = context.i32_type().const_int(30, false)
    codegen.as_ref().unwrap().builder().build_store(age_ptr, age).unwrap()
    
    // Convert Person to Stringer interface;
    let person_type = Type::Struct(Person.to_string(), Vec::new();
    let stringer_interface = codegen.create_interface_value()
        person_ptr,
        &person_type,
         Stringer "
    let error_str = codegen.create_string_constant(not a Person)")
    // Need to handle the result
    codegen.as_ref().unwrap().builder().build_return(Some(&error_str.unwrap().unwrap()
    
    // Verify the module
    if let Err(message) = codegen.as_ref().unwrap().get_module().verify()     {return Err(Error::from_str(&format!(Module verification error: {}, message.to_string()}
    
    Ok(()

// Mock method for testing
impl TypeChecker       {pub fn check_interface_implementation() {Ok(true);
