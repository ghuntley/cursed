use std::collections::HashMap;
use std::path::PathBuf;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::InterfaceImplementation;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::error::Error;
use std::sync::Arc;

// Integration test for interface code generation in LLVM


#[path = "tracing_setup.rs"]
pub mod tracing_setup;

#[test]
fn test_interface_code_generation() -> Result<(), Error> {
    // Set up tracing
    tracing_setup::init_test_tracing();
    
    // Create a new LLVM context and code generator
    let context = Context::create();
    let module_path = PathBuf::from("interface_test.bc");
    let mut codegen = LlvmCodeGenerator::new(&context, "interface_test_module", module_path);
    
    // Create a type checker
    let mut type_checker = TypeChecker::new();
    
    // 1. Register the Printable interface
    type_checker.register_interface(
        "Printable",
        vec![("to_string".to_string(), vec![], Some(Type::Tea))],
        Vec::new(),
    );
    
    // Register the interface with the code generator as well
    codegen.register_interface(
        "Printable",
        vec![("to_string".to_string(), vec![], Some(Type::Tea))],
        Vec::new(),
    )?;
    
    // 2. Register a Person struct
    let person_fields = HashMap::from([
        ("name".to_string(), Type::Tea),
        ("age".to_string(), Type::Normie),
    ]);
    
    type_checker.register_struct("Person", person_fields, Vec::new());
    
    // 3. Register a to_string method for Person
    type_checker.register_struct_method(
        "Person", 
        "to_string", 
        vec![], 
        Some(Type::Tea)
    )?;
    
    // 4. Verify that Person implements Printable
    let person_type = Type::Struct("Person".to_string(), Vec::new());
    let printable_type = Type::Interface("Printable".to_string(), Vec::new());
    
    let implements = type_checker.check_interface_implementation(&person_type, &printable_type)?;
    assert!(implements, "Person should implement Printable");
    
    // 5. Define the to_string implementation for Person with LLVM
    // Generate LLVM struct type for Person
    let person_llvm_type = codegen.context().struct_type(
        &[
            codegen.context().i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // name: tea
            codegen.context().i32_type().into(), // age: normie
        ],
        false,
    );
    
    // Create a function for Person.to_string
    let fn_type = codegen.context().i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[
            person_llvm_type.ptr_type(inkwell::AddressSpace::default()).into()
        ], false);
    
    let function = codegen.module().add_function(
        "Person.to_string",
        fn_type,
        None,
    );
    
    // 6. Register Person as implementing Printable
    let mut person_methods = HashMap::new();
    person_methods.insert("to_string".to_string(), function);
    
    codegen.register_interface_implementation(
        "Person",
        "Printable",
        person_methods,
    )?;
    
    // 7. Create interface value and call method (for testing dynamic dispatch)
    let entry_block = codegen.context().append_basic_block(function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Allocate a Person struct
    let person_ptr = codegen.builder().build_alloca(person_llvm_type, "person").unwrap();
    
    // Create an interface value from the Person
    let interface_value = codegen.create_interface_value(
        person_ptr,
        &person_type,
        "Printable",
    )?;
    
    // Call the to_string method on the interface
    let method_result = codegen.call_interface_method(
        interface_value,
        "Printable",
        "to_string",
        &[],
    )?;
    
    // Return the result
    if let Some(result) = method_result {
        let _ = codegen.builder().build_return(Some(&result));
    } else {
        let _ = codegen.builder().build_return(None);
    }
    
    // 8. Test interface type assertion
    let assertion_fn_type = codegen.context().i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[
            codegen.context().i8_type().ptr_type(inkwell::AddressSpace::default()).into()
        ], false);
    
    let assertion_function = codegen.module().add_function(
        "test_type_assertion", 
        assertion_fn_type,
        None,
    );
    
    let assertion_block = codegen.context().append_basic_block(assertion_function, "entry");
    codegen.builder().position_at_end(assertion_block);
    
    // Create an interface parameter
    let param = assertion_function.get_first_param().unwrap()
        .into_pointer_value();
    
    // Try to cast it to Person
    let person_ptr = codegen.compile_interface_type_assertion(
        param,
        &person_type,
    )?;
    
    // Check if it's actually a Person
    let is_person = codegen.check_instance_of(
        param,
        "Person",
    )?;
    
    // Use is_person in an if statement
    let is_true_block = codegen.context().append_basic_block(assertion_function, "is_person");
    let is_false_block = codegen.context().append_basic_block(assertion_function, "not_person");
    
    let _ = codegen.builder().build_conditional_branch(
        is_person.into_int_value(),
        is_true_block,
        is_false_block,
    );
    
    // Return different values depending on if statement result
    codegen.builder().position_at_end(is_true_block);
    let success_result = codegen.context().i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .const_null();
    let _ = codegen.builder().build_return(Some(&success_result));
    
    codegen.builder().position_at_end(is_false_block);
    let _ = codegen.builder().build_return(Some(&success_result));
    
    // Verify the module
    if let Err(message) = codegen.module().verify() {
        return Err(Error::from_str(&format!("Module verification error: {}", message.to_string())));
    }
    
    Ok(())
}