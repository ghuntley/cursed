use std::collections::HashMap;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::core::interface_type_checker::InterfaceTypeChecker;
use cursed::error::Error;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::dynamic_dispatch::InterfaceManager;
use cursed::codegen::llvm::enhanced_dynamic_dispatch::EnhancedDynamicDispatch;
use cursed::codegen::llvm::integrated_interface_operations::IntegratedInterfaceOperations;
use inkwell::values::BasicValueEnum;

// Integration test for enhanced interface operations
//
// This test verifies the improvements to interface operations including:
// 1. Enhanced dynamic dispatch with better error handling
// 2. Integrated type assertions
// 3. Improved interface value creation
// 4. Better integration with the type checker


#[path = "common/mod.rs"]
mod common;

/// Test fixture with interfaces and implementing types
fn setup_test_hierarchy() -> Result<TypeChecker, Error> {
    let mut type_checker = TypeChecker::new();
    
    // Register a Serializable interface
    type_checker.register_interface(
        "Serializable",
        vec![
            ("to_json".to_string(), vec![], Some(Type::Tea)),
            ("from_json".to_string(), vec![Type::Tea], Some(Type::Lit)),
        ],
        Vec::new(),
    );
    
    // Register a Person struct that implements Serializable
    let person_fields = HashMap::from([
        ("name".to_string(), Type::Tea),
        ("age".to_string(), Type::Normie),
        ("email".to_string(), Type::Tea),
    ]);
    
    type_checker.register_struct("Person", person_fields, Vec::new());
    
    // Register Person methods
    let person_methods = vec![
        ("to_json".to_string(), vec![], Some(Type::Tea)),
        ("from_json".to_string(), vec![Type::Tea], Some(Type::Lit)),
        ("get_name".to_string(), vec![], Some(Type::Tea)),
    ];
    
    for (method_name, param_types, return_type) in person_methods.clone() {
        type_checker.register_struct_method("Person", &method_name, param_types, return_type)?;
    }
    
    // Register a Config struct that implements Serializable
    let config_fields = HashMap::from([
        ("settings".to_string(), Type::Tea),
        ("version".to_string(), Type::Normie),
    ]);
    
    type_checker.register_struct("Config", config_fields, Vec::new());
    
    // Register Config methods
    let config_methods = vec![
        ("to_json".to_string(), vec![], Some(Type::Tea)),
        ("from_json".to_string(), vec![Type::Tea], Some(Type::Lit)),
        ("get_version".to_string(), vec![], Some(Type::Normie)),
    ];
    
    for (method_name, param_types, return_type) in config_methods.clone() {
        type_checker.register_struct_method("Config", &method_name, param_types, return_type)?;
    }
    
    Ok(type_checker)
}

/// Test interface implementation checking
#[test]
fn test_interface_implementation_checking() -> Result<(), Error> {
    common::tracing::setup();
    tracing::info!("Starting interface implementation checking test");
    
    let mut type_checker = setup_test_hierarchy()?;
    
    // Verify that Person implements Serializable
    let person_type = Type::Struct("Person".to_string(), Vec::new());
    let serializable_type = Type::Interface("Serializable".to_string(), Vec::new());
    
    let person_implements = type_checker.check_interface_implementation(&person_type, &serializable_type)?;
    assert!(person_implements, "Person should implement Serializable");
    
    // Verify that Config implements Serializable
    let config_type = Type::Struct("Config".to_string(), Vec::new());
    
    let config_implements = type_checker.check_interface_implementation(&config_type, &serializable_type)?;
    assert!(config_implements, "Config should implement Serializable");
    
    tracing::info!("Completed interface implementation checking test");
    Ok(())
}

/// Test enhanced dynamic dispatch
#[test]
fn test_enhanced_dynamic_dispatch() -> Result<(), Error> {
    common::tracing::setup();
    tracing::info!("Starting enhanced dynamic dispatch test");
    
    // Set up LLVM context
    let context = Context::create();
    
    // Create code generator
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_enhanced_dispatch", std::path::PathBuf::from("test.csd"));
    
    // Set up interface manager
    let interface_manager = InterfaceManager::new();
    code_gen.set_interface_manager(interface_manager);
    
    // Register Serializable interface
    code_gen.register_interface(
        "Serializable",
        vec![
            ("to_json".to_string(), vec![], Some(Type::Tea)),
            ("from_json".to_string(), vec![Type::Tea], Some(Type::Lit)),
        ],
        Vec::new(),
    )?;
    
    // Create a Person struct type
    let person_type = Type::Struct("Person".to_string(), Vec::new());
    
    // Create method function types
    let to_json_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()], false);
        
    let from_json_type = context.bool_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
        ], false);
    
    // Add method implementations
    let to_json_fn = code_gen.module().add_function("Person_to_json", to_json_type, None);
    let from_json_fn = code_gen.module().add_function("Person_from_json", from_json_type, None);
    
    // Register method implementations
    let mut methods = HashMap::new();
    methods.insert("to_json".to_string(), to_json_fn);
    methods.insert("from_json".to_string(), from_json_fn);
    
    // Register the implementation
    code_gen.register_interface_implementation("Person", "Serializable", methods)?;
    
    // Create a basic block for test code
    let void_fn_type = context.void_type().fn_type(&[], false);
    let test_fn = code_gen.module().add_function("test_function", void_fn_type, None);
    let entry_block = context.append_basic_block(test_fn, "entry");
    code_gen.builder().position_at_end(entry_block);
    
    // Create a dummy Person instance
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let person_ptr = code_gen.builder().build_alloca(i8_ptr_type, "person_ptr").unwrap();
    
    // Create interface value
    let interface_val = code_gen.create_interface_value(person_ptr, &person_type, "Serializable")?;
    
    // Test the enhanced dynamic dispatch
    let args: Vec<BasicValueEnum> = Vec::new();
    let result = code_gen.call_interface_method_enhanced(interface_val, "Serializable", "to_json", &args);
    
    // We're not actually executing the code, so we can't check the result value
    // but we can verify that the method call was generated without errors
    assert!(result.is_ok(), "Enhanced dynamic dispatch should succeed");
    
    // Test null check
    let null_ptr = i8_ptr_type.const_null();
    let is_null = code_gen.check_interface_null(null_ptr, "test operation")?;
    assert!(is_null, "Null pointer should be detected correctly");
    
    tracing::info!("Completed enhanced dynamic dispatch test");
    Ok(())
}

/// Test integrated interface operations
#[test]
fn test_integrated_interface_operations() -> Result<(), Error> {
    common::tracing::setup();
    tracing::info!("Starting integrated interface operations test");
    
    // Set up LLVM context
    let context = Context::create();
    
    // Create code generator
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_integrated_ops", std::path::PathBuf::from("test.csd"));
    
    // Set up interface manager
    let interface_manager = InterfaceManager::new();
    code_gen.set_interface_manager(interface_manager);
    
    // Register Serializable interface
    code_gen.register_interface(
        "Serializable",
        vec![
            ("to_json".to_string(), vec![], Some(Type::Tea)),
            ("from_json".to_string(), vec![Type::Tea], Some(Type::Lit)),
        ],
        Vec::new(),
    )?;
    
    // Create a Person struct type
    let person_type = Type::Struct("Person".to_string(), Vec::new());
    let config_type = Type::Struct("Config".to_string(), Vec::new());
    let serializable_type = Type::Interface("Serializable".to_string(), Vec::new());
    
    // Create method function types
    let to_json_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()], false);
        
    let from_json_type = context.bool_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
        ], false);
    
    // Add method implementations for Person
    let person_to_json_fn = code_gen.module().add_function("Person_to_json", to_json_type, None);
    let person_from_json_fn = code_gen.module().add_function("Person_from_json", from_json_type, None);
    
    // Register Person method implementations
    let mut person_methods = HashMap::new();
    person_methods.insert("to_json".to_string(), person_to_json_fn);
    person_methods.insert("from_json".to_string(), person_from_json_fn);
    
    // Register the Person implementation
    code_gen.register_interface_implementation("Person", "Serializable", person_methods)?;
    
    // Add method implementations for Config
    let config_to_json_fn = code_gen.module().add_function("Config_to_json", to_json_type, None);
    let config_from_json_fn = code_gen.module().add_function("Config_from_json", from_json_type, None);
    
    // Register Config method implementations
    let mut config_methods = HashMap::new();
    config_methods.insert("to_json".to_string(), config_to_json_fn);
    config_methods.insert("from_json".to_string(), config_from_json_fn);
    
    // Register the Config implementation
    code_gen.register_interface_implementation("Config", "Serializable", config_methods)?;
    
    // Create a basic block for test code
    let void_fn_type = context.void_type().fn_type(&[], false);
    let test_fn = code_gen.module().add_function("test_function", void_fn_type, None);
    let entry_block = context.append_basic_block(test_fn, "entry");
    code_gen.builder().position_at_end(entry_block);
    
    // Create dummy instances
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let person_ptr = code_gen.builder().build_alloca(i8_ptr_type, "person_ptr").unwrap();
    let config_ptr = code_gen.builder().build_alloca(i8_ptr_type, "config_ptr").unwrap();
    
    // Create interface values
    let serializable_person = code_gen.integrated_create_interface(person_ptr, &person_type, "Serializable")?;
    
    // Test type assertion on person
    let (person_result, person_success) = code_gen.integrated_type_assertion(
        serializable_person, 
        &person_type, 
        true
    )?;
    
    // Test implementation checking
    let person_implements = code_gen.integrated_check_implements_interface(&person_type, &serializable_type)?;
    assert!(person_implements, "Person should implement Serializable");
    
    let config_implements = code_gen.integrated_check_implements_interface(&config_type, &serializable_type)?;
    assert!(config_implements, "Config should implement Serializable");
    
    tracing::info!("Completed integrated interface operations test");
    Ok(())
}