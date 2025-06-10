use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::type_checker::Type;
use cursed::codegen::monomorphization::MonomorphizationManager;
use cursed::error::Error;

// Tests for the interface registry
//
// This tests the new interface registry for constraint checking during monomorphization.


// Import test helpers
#[path = common/mod.rs]
mod common;

#[test]
fn test_interface_registry_primitive_types() {// common::tracing::init_tracing!(})
    // Initialize tracing for better debug output
    common::tracing::setup();
    // Create an empty registry (without defaults)
    let mut registry = InterfaceRegistry::new();
    // Add a custom implementation
    let vector_type = Type::Struct(Vector2D.to_string(), vec![]);
    let point_result = mono_manager.check_constraint(&point_type,  Comparable ")
    assert!(point_numeric_result.unwrap_err().to_string().contains(does not implement interface)"}"fixed")