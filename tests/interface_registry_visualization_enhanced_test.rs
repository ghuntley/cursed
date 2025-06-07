use std::sync::Once;
use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use cursed::core::interface_registry_visualization_enhanced::EnhancedVisualizationIntegration;
use cursed::error::Error;
use tracing::{debug, info};

// Test for enhanced interface registry visualization integration


// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}


#[test]
fn test_enhanced_detailed_error_message() {
    init_tracing!();
    info!("Testing enhanced detailed error message");
    
    // Create a registry with test data
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    registry.register_extension("Dog", "Mammal").unwrap();
    registry.register_extension("Cat", "Mammal").unwrap();
    registry.register_extension("Mammal", "Animal").unwrap();
    registry.register_extension("Bird", "Animal").unwrap();
    registry.register_extension("Animal", "LivingThing").unwrap();
    
    // Test with inheritance relationship in the wrong direction
    let error_message = EnhancedVisualizationIntegration::generate_detailed_error_message(
        &registry,
        "Animal", 
        "Dog", 
        "test.csd:42:10"
    ).unwrap();
    
    // Verify error message contains expected information
    assert!(error_message.contains("Type Assertion Error");
    assert!(error_message.contains("test.csd:42:10");
    assert!(error_message.contains("Cannot assert type 'Animal' as 'Dog'!");
    assert!(error_message.contains("inheritance relationship is reversed");
    
    // Test with interfaces that have no relationship
    let error_message = EnhancedVisualizationIntegration::generate_detailed_error_message(
        &registry,
        "Dog", 
        "Bird", 
        "test.csd:42:10"
    ).unwrap();
    
    // Verify error message for unrelated interfaces
    assert!(error_message.contains("No inheritance relationship exists");
    assert!(error_message.contains("To fix this error");
}

#[test]
fn test_enhanced_ascii_hierarchy() {
    init_tracing!();
    info!("Testing enhanced ASCII hierarchy visualization");
    
    // Create a registry with test data
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    registry.register_extension("Dog", "Mammal").unwrap();
    registry.register_extension("Cat", "Mammal").unwrap();
    registry.register_extension("Mammal", "Animal").unwrap();
    registry.register_extension("Bird", "Animal").unwrap();
    registry.register_extension("Animal", "LivingThing").unwrap();
    
    // Test ASCII visualization
    let ascii = EnhancedVisualizationIntegration::generate_ascii_hierarchy(&registry).unwrap();
    
    // Verify output contains expected content
    assert!(ascii.contains("Interface Hierarchy Tree");
    assert!(ascii.contains("LivingThing");
    assert!(ascii.contains("Animal");
    assert!(ascii.contains("Mammal");
    assert!(ascii.contains("Dog");
    assert!(ascii.contains("Cat");
    assert!(ascii.contains("Bird");
}