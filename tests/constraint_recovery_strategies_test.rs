use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::constraint_recovery::::ConstraintRecovery, ConstraintRecoveryExtension, RecoveryStrategy, ConstraintFailureSeverity;
use cursed::core::type_checker::Type;
use cursed::error::Error;

// Tests for constraint recovery strategies


#[path = common/mod.rs]
mod common;

#[test]
fn test_recovery_for_comparable_interface() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = InterfaceRegistry::new()
    registry.populate_with_defaults()
    
    // Create a vector type that doesnt implement Numeric)
    let vector_type = Type::Struct(Vector3D.to_string(), vec![}
fn test_registry_extension_methods() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = InterfaceRegistry::new()
    registry.populate_with_defaults()
    
    // Register interface methods
    let mut custom_methods = std::collections::HashMap::new()
    custom_methods.insert(CustomMethod.to_string(),  self  Custom, param Tea.to_string()
    registry.register_interface_methods("CustomInterface, custom_methods)"fixed"