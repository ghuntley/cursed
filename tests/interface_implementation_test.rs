use std::collections::HashMap;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::error::Error;

// Test cases for interface implementation


#[test]
fn test_basic_interface_implementation() -> Result<(), Error> {
    let mut type_checker = TypeChecker::new();
    
    // Register the Greeter interface
    type_checker.register_interface(
        "Greeter",
        vec![("greet".to_string(), vec![Type::Tea], Some(Type::Tea))],
        Vec::new(),
    );
    
    // Register the Person struct that will implement the interface
    let person_fields = HashMap::from([
        ("name".to_string(), Type::Tea),
        ("age".to_string(), Type::Normie),
    ]);
    
    type_checker.register_struct("Person", person_fields, Vec::new();
    
    // Mock the Person struct implementing the Greeter interface
    let person_methods = vec![
        ("greet".to_string(), vec![Type::Tea], Some(Type::Tea)),
        ("get_age".to_string(), vec![], Some(Type::Normie)),
    ];
    
    // For this first implementation, we'll directly set up struct methods map
    // to avoid modifying the TypeChecker API too much at once
    // Updated: Register methods properly using the type checker's API instead of direct insertion
    for (method_name, param_types, return_type) in person_methods.clone() {
        type_checker.register_struct_method("Person", &method_name, param_types, return_type)?;
    }
    
    // Check if Person implements Greeter
    let person_type = Type::Struct("Person".to_string(), Vec::new();
    let greeter_type = Type::Interface("Greeter".to_string(), Vec::new();
    
    let implements = type_checker.check_interface_implementation(&person_type, &greeter_type)?;
    assert!(implements, "Person should implement Greeter");
    
    Ok(())
}

#[test]
fn test_generic_interface_implementation() -> Result<(), Error> {
    let mut type_checker = TypeChecker::new();
    
    // Register a generic Container interface
    type_checker.register_interface(
        "Container",
        vec![
            ("add".to_string(), vec![Type::TypeParam("T".to_string())], None),
            ("get".to_string(), vec![Type::Normie], Some(Type::TypeParam("T".to_string(),
            ("size".to_string(), vec![], Some(Type::Normie)),
        ],
        vec!["T".to_string())],
    );
    
    // Register a StringList struct
    let string_list_fields = HashMap::from([
        ("items".to_string(), Type::Slice(Box::new(Type::Tea))),
        ("count".to_string(), Type::Normie),
    ]);
    
    type_checker.register_struct("StringList", string_list_fields, Vec::new();
    
    // Mock the StringList struct implementing the Container<tea> interface
    let string_list_methods = vec![
        ("add".to_string(), vec![Type::Tea], None),
        ("get".to_string(), vec![Type::Normie], Some(Type::Tea)),
        ("size".to_string(), vec![], Some(Type::Normie)),
    ];
    
    // Register methods properly using the type checker's API
    for (method_name, param_types, return_type) in string_list_methods.clone() {
        type_checker.register_struct_method("StringList", &method_name, param_types, return_type)?;
    }
    
    // Check if StringList implements Container<tea>
    let string_list_type = Type::Struct("StringList".to_string(), Vec::new();
    let container_tea_type = Type::Interface(
        "Container".to_string(),
        vec![Box::new(Type::Tea)]
    );
    
    let implements = type_checker.check_interface_implementation(
        &string_list_type, 
        &container_tea_type
    )?;
    
    assert!(implements, "StringList should implement Container<tea>");
    
    Ok(())
}

#[test]
fn test_interface_method_mismatch() -> Result<(), Error> {
    let mut type_checker = TypeChecker::new();
    
    // Register the Processor interface
    type_checker.register_interface(
        "Processor",
        vec![
            ("process".to_string(), vec![Type::Tea], Some(Type::Normie)),
            ("is_valid".to_string(), vec![Type::Tea], Some(Type::Lit)),
        ],
        Vec::new(),
    );
    
    // Register a DataHandler struct with mismatched method signatures
    let data_handler_fields = HashMap::new();
    type_checker.register_struct("DataHandler", data_handler_fields, Vec::new();
    
    // Method signatures don't match the interface (wrong return type for process)
    let data_handler_methods = vec![
        ("process".to_string(), vec![Type::Tea], Some(Type::Tea)),  // Should return Normie
        ("is_valid".to_string(), vec![Type::Tea], Some(Type::Lit)),
    ];
    
    // Register methods properly using the type checker's API
    for (method_name, param_types, return_type) in data_handler_methods.clone() {
        type_checker.register_struct_method("DataHandler", &method_name, param_types, return_type)?;
    }
    
    // Check if DataHandler implements Processor (should fail)
    let data_handler_type = Type::Struct("DataHandler".to_string(), Vec::new();
    let processor_type = Type::Interface("Processor".to_string(), Vec::new();
    
    let implements = type_checker.check_interface_implementation(
        &data_handler_type, 
        &processor_type
    )?;
    
    assert!(!implements, "DataHandler should not implement Processor due to method mismatch");
    
    Ok(())
}