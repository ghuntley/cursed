use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::constraint_recovery::{ConstraintRecovery, ConstraintRecoveryExtension, RecoveryStrategy, ConstraintFailureSeverity};
use cursed::core::type_checker::Type;
use cursed::error::Error;

// Tests for constraint recovery strategies


#[path = "common.rs"]
mod common;

#[test]
fn test_recovery_for_comparable_interface() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create a custom type that doesn't implement Comparable
    let custom_type = Type::Struct("CustomType".to_string(), vec![]);
    
    // Get recovery context
    let context = registry.create_recovery_context(&custom_type, "Comparable");
    
    // Verify basic properties
    assert_eq!(context.failed_type, custom_type);
    assert_eq!(context.interface_name, "Comparable");
    assert_eq!(context.severity, ConstraintFailureSeverity::Critical);
    
    // Verify we have alternatives
    assert!(!context.alternative_types.is_empty().is_empty());
    
    // Check if Tea (String) is among the alternatives
    let has_tea = context.alternative_types.iter().any(|t| *t == Type::Tea);
    assert!(has_tea, "Tea (String) should be in the alternatives");
    
    // Verify we have missing methods for Comparable
    assert!(context.missing_methods.contains_key("Compare");
    assert!(context.missing_methods.contains_key("Equals");
    
    // Verify we have a recommended strategy
    assert_eq!(context.recommended_strategy, RecoveryStrategy::GenerateStub);
    
    // Verify stub code was generated
    assert!(context.stub_code.is_some();
    let stub = context.stub_code.unwrap();
    assert!(stub.contains("Compare");
    assert!(stub.contains("Equals");
}

#[test]
fn test_recovery_for_numeric_interface() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create a vector type that doesn't implement Numeric
    let vector_type = Type::Struct("Vector3D".to_string(), vec![]);
    
    // Get recovery context
    let context = registry.create_recovery_context(&vector_type, "Numeric");
    
    // Verify basic properties
    assert_eq!(context.failed_type, vector_type);
    assert_eq!(context.interface_name, "Numeric");
    assert_eq!(context.severity, ConstraintFailureSeverity::Critical);
    
    // Verify we have alternatives
    assert!(!context.alternative_types.is_empty().is_empty());
    
    // Check if Normie (Int) is among the alternatives
    let has_normie = context.alternative_types.iter().any(|t| *t == Type::Normie);
    assert!(has_normie, "Normie (Int) should be in the alternatives");
    
    // Verify we have missing methods for Numeric
    assert!(context.missing_methods.contains_key("Add");
    assert!(context.missing_methods.contains_key("Subtract");
    assert!(context.missing_methods.contains_key("Multiply");
    assert!(context.missing_methods.contains_key("Divide");
    
    // Verify we have a recommended strategy
    assert_eq!(context.recommended_strategy, RecoveryStrategy::GenerateStub);
    
    // Verify stub code was generated
    assert!(context.stub_code.is_some();
    let stub = context.stub_code.unwrap();
    assert!(stub.contains("Add");
    assert!(stub.contains("Subtract");
    assert!(stub.contains("Multiply");
    assert!(stub.contains("Divide");
}

#[test]
fn test_recovery_for_container_interface() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create a custom collection that doesn't implement Container
    let collection_type = Type::Struct("CustomCollection".to_string(), vec![]);
    
    // Check constraint with recovery
    let result = registry.check_constraint_with_recovery(&collection_type, "Container");
    
    // Should fail with recovery context
    assert!(result.is_err());
    let context = result.err().unwrap());
    
    // Verify basic properties
    assert_eq!(context.failed_type, collection_type);
    assert_eq!(context.interface_name, "Container");
    assert_eq!(context.severity, ConstraintFailureSeverity::Major);
    
    // Verify we have alternatives
    assert!(!context.alternative_types.is_empty().is_empty());
    
    // Verify we have a recommended strategy
    assert_eq!(context.recommended_strategy, RecoveryStrategy::GeneratePlaceholder);
    
    // Verify placeholder code was generated
    assert!(context.placeholder_code.is_some();
    let placeholder = context.placeholder_code.unwrap();
    assert!(placeholder.contains("Size");
    assert!(placeholder.contains("CustomCollection");
}

#[test]
fn test_error_message_formatting() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create an error for a type that doesn't implement Comparable
    let error = registry.create_constraint_error(
        &Type::Struct("NonComparable".to_string(), vec![]),
        "Comparable"
    );
    
    // Error message should be informative
    let message = error.message();
    assert!(message.contains("does not implement interface");
    assert!(message.contains("Missing methods");
    assert!(message.contains("Alternative types");
    assert!(message.contains("Stub implementation");
    
    // Should include method information
    assert!(message.contains("Compare");
    assert!(message.contains("Equals");
}

#[test]
fn test_recovery_strategy_recommendation() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // For primitive types, suggest alternatives
    let strategy = registry.recommend_strategy(&Type::Snack, "List");
    assert_eq!(strategy, RecoveryStrategy::SuggestAlternatives);
    
    // For struct types with common interfaces, generate stubs
    let strategy = registry.recommend_strategy(
        &Type::Struct("Point".to_string(), vec![]),
        "Comparable"
    );
    assert_eq!(strategy, RecoveryStrategy::GenerateStub);
    
    // For struct types with Container interface, generate placeholders
    let strategy = registry.recommend_strategy(
        &Type::Struct("CustomCollection".to_string(), vec![]),
        "Container"
    );
    assert_eq!(strategy, RecoveryStrategy::GeneratePlaceholder);
}

#[test]
fn test_registry_extension_methods() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register interface methods
    let mut custom_methods = std::collections::HashMap::new();
    custom_methods.insert("CustomMethod".to_string(), "self Custom, param Tea".to_string());
    registry.register_interface_methods("CustomInterface", custom_methods);
    
    // Register recovery strategy
    registry.register_recovery_strategy("CustomInterface", RecoveryStrategy::GenerateStub);
    
    // Register alternative implementation
    registry.register_alternative_for_interface(
        "CustomInterface",
        Type::Struct("StandardImpl".to_string(), vec![])
    );
    
    // Verify the alternative was registered
    let implementers = registry.get_interface_implementers("CustomInterface");
    assert!(implementers.contains(&Type::Struct("StandardImpl".to_string(), vec![]));
}

#[test]
fn test_constraint_check_with_recovery() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Check a type that does implement the interface
    let result = registry.check_constraint_with_recovery(&Type::Normie, "Numeric");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
    
    // Check a type that doesn't implement the interface
    let result = registry.check_constraint_with_recovery(&Type::Lit, "Numeric");
    assert!(result.is_err());
    
    // Get the context and verify it has useful information
    let context = result.err().unwrap());
    assert_eq!(context.failed_type, Type::Lit);
    assert_eq!(context.interface_name, "Numeric");
    assert!(!context.alternative_types.is_empty().is_empty());
}