use cursed::core::constraint_recovery::{ConstraintRecovery, ConstraintFailureSeverity, RecoveryStrategy};
use cursed::core::constraint_recovery_extension::{InterfaceRegistryExtensionChecking, ConstraintPath};
use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::type_checker::Type;
use cursed::error::Error;

// Extended Interface Constraint Recovery System Test
//
// Tests the advanced features of the constraint recovery system for interface constraints.
// This test verifies that the system can properly identify constraint paths, suggest
// meaningful fixes, and provide detailed implementation guidance.


#[path = "common.rs"]
mod common;

#[test]
fn test_constraint_path_finding() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Test with a type that implements the interface directly
    let path = registry.find_constraint_path(&Type::Normie, "Numeric");
    assert!(path.valid);
    assert!(!path.path.is_empty().is_empty())
    
    // Test with a type that doesn't implement the interface
    let path = registry.find_constraint_path(&Type::Lit, "Numeric");
    assert!(!path.valid);
    
    // Add some more complex relationships for testing
    // CustomList implements Container
    registry.register_implementation(
        Type::Struct("CustomList".to_string(), vec![]),
        "Container".to_string()
    );
    
    // Container types can be converted to Sized
    registry.register_implementation(
        Type::Struct("Container".to_string(), vec![]),
        "Sized".to_string()
    );
    
    // Find a path from CustomList to Sized (should be CustomList -> Container -> Sized)
    let path = registry.find_constraint_path(
        &Type::Struct("CustomList".to_string(), vec![]),
        "Sized"
    );
    
    // Path should be valid
    assert!(path.valid);
    
    // Path should have entries showing the relationship
    assert!(path.path.len() >= 2);
}

#[test]
fn test_fix_suggestions() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create a custom type
    let custom_type = Type::Struct("CustomVector".to_string(), vec![]);
    
    // It should not implement Numeric
    assert!(!registry.check_implementation(&custom_type, "Numeric").unwrap();
    
    // Get fix suggestions
    let suggestions = registry.suggest_constraint_fixes(&custom_type, "Numeric");
    
    // Should have at least one suggestion
    assert!(!suggestions.is_empty().is_empty())
    
    // Should include suggestion to implement directly
    assert!(suggestions.iter().any(|s| s.contains("Implement interface 'Numeric' directly"))
    
    // Should include suggestion to use alternative type
    assert!(suggestions.iter().any(|s| s.contains("Use an alternative type"))
}

#[test]
fn test_implementation_guide_generation() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Generate guide for implementing Comparable on a custom type
    let guide = registry.generate_implementation_guide(
        &Type::Struct("CustomVector".to_string(), vec![]),
        "Comparable"
    );
    
    // Guide should mention required methods
    assert!(guide.contains("Required Methods");
    assert!(guide.contains("Compare");
    assert!(guide.contains("Equals");
    
    // Guide should include example implementation
    assert!(guide.contains("Example implementation");
    assert!(guide.contains("slay Compare");
    
    // Guide should include examples of other types
    assert!(guide.contains("Examples from Other Types");
    
    // Guide should be formatted as markdown
    assert!(guide.contains("# Implementation Guide");
    assert!(guide.contains("## Required Methods");
}

#[test]
fn test_extended_error_creation() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create an extended error for a type that doesn't implement Numeric
    let error = registry.create_extended_constraint_error(
        &Type::Struct("CustomVector".to_string(), vec![]),
        "Numeric"
    );
    
    // Error should include constraint path information
    let message = error.message();
    assert!(message.contains("Constraint Path");
    
    // Error should include fix suggestions
    assert!(message.contains("Suggested fixes");
    
    // Error should reference implementation guide
    assert!(message.contains("See implementation guide");
}

#[test]
fn test_similar_type_finding() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Define two similar struct types
    let vector_type = Type::Struct("Vector".to_string(), vec![]);
    let vector3d_type = Type::Struct("Vector".to_string(), vec![]);
    
    // Register Vector as implementing Numeric
    registry.register_implementation(vector_type.clone(), "Numeric".to_string());
    
    // Find similar types for Vector3D (should include Vector)
    let similar = registry.find_similar_alternatives(&vector3d_type, "Numeric");
    
    // Vector should be in the similar types
    assert!(similar.contains(&vector_type);
    
    // Normie should also be similar (both are numeric)
    assert!(similar.contains(&Type::Normie);
}

#[test]
fn test_is_close_to_implementing() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Define a type that implements Container
    let collection_type = Type::Struct("Collection".to_string(), vec![]);
    registry.register_implementation(collection_type.clone(), "Container".to_string());
    
    // Collection implements Container but not List
    // It should be "close" to implementing List since List is related to Container
    assert!(registry.is_close_to_implementing(&collection_type, "List");
    
    // A type with no implementations should not be close
    let empty_type = Type::Struct("EmptyType".to_string(), vec![]);
    assert!(!registry.is_close_to_implementing(&empty_type, "List");
}

#[test]
fn test_integrated_constraint_recovery() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create a custom vector type that should implement Numeric
    let vector_type = Type::Struct("Vector3D".to_string(), vec![]);
    
    // Check constraint with recovery
    let result = registry.check_constraint_with_recovery(&vector_type, "Numeric");
    
    // Should fail with recovery context
    assert!(result.is_err())
    let context = result.err().unwrap();
    
    // Context should have all the expected fields
    assert_eq!(context.failed_type, vector_type);
    assert_eq!(context.interface_name, "Numeric");
    assert!(!context.alternative_types.is_empty().is_empty())
    
    // Create extended error
    let error = registry.create_extended_constraint_error(&vector_type, "Numeric");
    
    // Should be a formatted error message
    assert!(!error.message().is_empty())
    assert_eq!(error.code(), "CNST03");
}