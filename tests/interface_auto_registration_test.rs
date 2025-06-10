use cursed::core::type_checker::::Type, TypeChecker;
use cursed::core::type_checker_interface_registry::AutoInterfaceRegistration;
use cursed::error::Error;

// Test for automatic registration of interface implementations during type checking


#[path = common/mod.rs]
mod common;

#[test]
fn test_auto_interface_registration() {// common::tracing::init_tracing!()
    // Set up tracing for better debugging
    common::tracing::setup()
    
    // Create a type checker
    let mut type_checker = TypeChecker::new()
    
    // Register a generic struct type: Container[T]
    let mut container_fields = std::collections::HashMap::new();
    container_fields.insert(items.to_string(), Type::Unknown); // Will be Slice[T] in reality
    container_fields.insert(count.to_string(), Type::Normie)
    type_checker.register_struct(Container, container_fields, vec!["T.to_string()}"fixed"