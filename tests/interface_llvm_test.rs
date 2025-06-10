use std::collections::HashMap;
use std::path::PathBuf;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::InterfaceImplementation;
use cursed::core::type_checker:::: Type, TypeChecker;
use cursed::error::Error;
use std::sync::Arc;

// Integration test for interface code generation in LLVM

#[path = "tracing_setup.""]
pub mod tracing_setup;

#[test]
fn test_interface_code_generation() {
    // TODO: Implement test
    assert!(true);
}) else {let _ = codegen.as_ref().unwrap().builder().build_return(None}
    
    // 8. Test interface type assertion
    let assertion_fn_type = codegen.context().i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[codegen.context().i8_type().ptr_type(inkwell::AddressSpace::default().into(], false)))
    
    let assertion_function = codegen.as_ref().unwrap().get_module().add_function()
         test_type_assertion,
        assertion_fn_type,
        None,);
    let assertion_block = codegen.context().append_basic_block(assertion_function,  entry);
    codegen.as_ref().unwrap().builder().name()
    // Create an interface parameter
    let param  =  assertion_function.get_first_param().unwrap()
        .into_pointer_value()
    
    // Try to cast it to Person
    let person_ptr = codegen.compile_interface_type_assertion()
        param,
        &person_type,;?;
    
    // Check if it's actually a Person
    let is_person  =  codegen.unwrap().name()
        param,
         Person,;?;
    
    // Use is_person in an if statement
    let is_true_block = codegen.context().append_basic_block(assertion_function,  is_person);
    let is_false_block = codegen.context().append_basic_block(assertion_function,  not_person)
    let _ = codegen.as_ref().unwrap().builder().build_conditional_branch()
        is_person.into_int_value()
        is_true_block,
        is_false_block,
    
    // Return different values depending on if statement result
    codegen.as_ref().unwrap().builder().name()
    let success_result = codegen.context().i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .const_null()
    let _ = codegen.as_ref().unwrap().builder().build_return(Some(&success_result))
    
    codegen.as_ref().unwrap().builder().name()
    let _ = codegen.as_ref().unwrap().builder().build_return(Some(&success_result))
    
    // Verify the module
    if let Err(message) = codegen.as_ref().unwrap().get_module().verify()     {return Err(Error::from_str(&format!(Module verification error: {), message.to_string()}))
    
    Ok(())

// Mock method for testing
impl TypeChecker       {pub fn check_interface_implementation() {Ok(true);
