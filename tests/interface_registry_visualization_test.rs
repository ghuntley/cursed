use std::sync::Once;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension};
use cursed::error::Error;
use tracing::{debug, info};
use std::sync::Arc;
use std::thread;

// Test for interface registry visualization


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
fn test_registry_visualization_extension_hierarchy() {
    init_tracing!();
    info!("Testing registry visualization extension hierarchy");
    
    // Create a registry and populate it with some test data
    let mut registry = ThreadSafeInterfaceExtensionRegistry::new();
    registry.register_extension("Dog", "Mammal").unwrap();
    registry.register_extension("Cat", "Mammal").unwrap();
    registry.register_extension("Mammal", "Animal").unwrap();
    registry.register_extension("Bird", "Animal").unwrap();
    registry.register_extension("Animal", "LivingThing").unwrap();
    
    // Test the visualization extension
    let hierarchy = registry.get_extension_hierarchy().unwrap();
    
    // Verify the hierarchy contains the expected relationships
    assert!(hierarchy.contains_key("Dog"))
    assert!(hierarchy.contains_key("Cat"))
    assert!(hierarchy.contains_key("Mammal"))
    assert!(hierarchy.contains_key("Bird"))
    assert!(hierarchy.contains_key("Animal"))
    
    // Verify specific relationships
    let mammal_extensions = hierarchy.get("Mammal").unwrap();
    assert!(mammal_extensions.contains(&"Animal".to_string())
    
    let animal_extensions = hierarchy.get("Animal").unwrap();
    assert!(animal_extensions.contains(&"LivingThing".to_string())
    
    // Test the direct extensions method
    let dog_extensions = registry.get_direct_extensions("Dog").unwrap().unwrap();
    assert_eq!(dog_extensions.len(), 1);
    assert!(dog_extensions.contains(&"Mammal".to_string())
    
    // Test the direct implementors method
    let mammal_implementors = registry.get_direct_implementors("Mammal").unwrap().unwrap();
    assert_eq!(mammal_implementors.len(), 2);
    assert!(mammal_implementors.contains(&"Dog".to_string()));
    assert!(mammal_implementors.contains(&"Cat".to_string()));
    
    // Test getting all interfaces
    let all_interfaces = registry.get_all_interfaces().unwrap();
    assert_eq!(all_interfaces.len(), 5);
    assert!(all_interfaces.contains("Dog"))
    assert!(all_interfaces.contains("Cat"))
    assert!(all_interfaces.contains("Mammal"))
    assert!(all_interfaces.contains("Bird"))
    assert!(all_interfaces.contains("Animal"))
    assert!(all_interfaces.contains("LivingThing"))
}

#[test]
fn test_registry_visualization_error_propagation() {
    init_tracing!();
    info!("Testing registry visualization error propagation");
    
    // Create a simple registry for testing
    let mut registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Register some test interfaces
    registry.register_extension("Interface1", "BaseInterface").unwrap();
    registry.register_extension("Interface2", "BaseInterface").unwrap();
    
    // The registry should properly propagate errors when using the ? operator
    let hierarchy = registry.get_extension_hierarchy().unwrap();
    assert!(hierarchy.contains_key("Interface1"));
    assert!(hierarchy.contains_key("Interface2"));
    
    let extensions = registry.get_direct_extensions("Interface1").unwrap().unwrap();
    assert_eq!(extensions.len(), 1);
    assert!(extensions.contains(&"BaseInterface".to_string()));
    
    // Test error propagation format with proper context
    let non_existent = registry.get_direct_extensions("NonExistentInterface").unwrap();
    assert!(non_existent.is_none());
}

#[test]
fn test_registry_thread_safety() {
    init_tracing!();
    info!("Testing registry thread safety with visualization");
    
    
    // Create a shared registry
    let mut registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Register some base interfaces and test interfaces
    registry.register_extension("BaseInterface1", "Root").unwrap();
    registry.register_extension("BaseInterface2", "Root").unwrap();
    
    // Pre-register all test interfaces to avoid mutation in threads
    for i in 0..10 {
        let interface_name = format!("TestInterface{}", i);
        let base_name = if i % 2 == 0 { "BaseInterface1" } else { "BaseInterface2" };
        registry.register_extension(&interface_name, base_name).unwrap();
    }
    
    // Now wrap it in Arc for sharing across threads (no more mutations needed)
    let registry = Arc::new(registry);
    
    // Create multiple threads that query interface relationships
    let mut handles = vec![];
    
    for i in 0..10 {
        let registry_clone = registry.clone();
        let handle = thread::spawn(move || {
            let interface_name = format!("TestInterface{}", i);
            let base_name = if i % 2 == 0 { "BaseInterface1" } else { "BaseInterface2" };
            
            // Query the registry using visualization methods
            let hierarchy = registry_clone.get_extension_hierarchy().unwrap();
            assert!(hierarchy.contains_key(&interface_name));
            
            let extensions = registry_clone.get_direct_extensions(&interface_name).unwrap().unwrap();
            assert_eq!(extensions.len(), 1);
            assert!(extensions.contains(&base_name.to_string()));
            
            // Return success indicator
            true
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        assert!(handle.join().unwrap());
    }
    
    // Verify final state with all interfaces
    let all_interfaces = registry.get_all_interfaces().unwrap();
    assert_eq!(all_interfaces.len(), 13); // 10 test interfaces + 2 base interfaces + Root
}