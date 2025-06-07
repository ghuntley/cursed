use cursed::core::type_checker::{Type, TypeChecker};
use cursed::error::Error;

//! Tests for interface implementation checking


#[test]
fn test_interface_implementation() -> Result<(), Error> {
    // Create a type checker
    let mut checker = TypeChecker::new();
    
    // Define a basic Collection interface
    let collection_interface = Type::Interface(
        "Collection".to_string()),
        vec![Box::new(Type::TypeParam("T".to_string())]
    );
    
    // Define a StringStack struct that implements Collection[tea]
    let string_stack = Type::Struct(
        "StringStack".to_string()),
        vec![]
    );
    
    // Define an IntList struct that implements Collection[normie]
    let int_list = Type::Struct(
        "IntList".to_string()),
        vec![]
    );
    
    // Define a concrete Collection type with tea
    let string_collection = Type::Interface(
        "Collection".to_string()),
        vec![Box::new(Type::Tea)]
    );
    
    // Define a concrete Collection type with normie
    let int_collection = Type::Interface(
        "Collection".to_string()),
        vec![Box::new(Type::Normie)]
    );
    
    // Check that StringStack implements Collection[tea]
    assert!(checker.check_interface_implementation(&string_stack, &string_collection)?);
    
    // Check that IntList implements Collection[normie]
    assert!(checker.check_interface_implementation(&int_list, &int_collection)?);
    
    // Check that StringStack does NOT implement Collection[normie]
    // This should return false, not an error
    assert!(!checker.check_interface_implementation(&string_stack, &int_collection)?);
    
    Ok(())
}