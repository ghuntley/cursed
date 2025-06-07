//! Test for interface registry visualization

use std::sync::Once;

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

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceRegistryExtensionWithVisualization;
use cursed::codegen::llvm::interface_registry_visualization_integration::InterfaceRegistryVisualizationIntegration;
use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension};
use cursed::core::interface_registry_visualization::InterfaceRegistryVisualization;
use cursed::error::Error;
use inkwell::context::Context;
use tracing::{debug, info};

#[test]
fn test_registry_visualization_extension_hierarchy() {
    init_tracing!();
    info!("Testing registry visualization extension hierarchy");
    
    // Create a registry and populate it with some test data
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    registry.register_extension("Dog", "Mammal").unwrap();
    registry.register_extension("Cat", "Mammal").unwrap();
    registry.register_extension("Mammal", "Animal").unwrap();
    registry.register_extension("Bird", "Animal").unwrap();
    registry.register_extension("Animal", "LivingThing").unwrap();
    
    // Test the visualization extension
    let hierarchy = registry.get_extension_hierarchy().unwrap();
    
    // Verify the hierarchy contains the expected relationships
    assert!(hierarchy.contains_key("Dog"));
    assert!(hierarchy.contains_key("Cat"));
    assert!(hierarchy.contains_key("Mammal"));
    assert!(hierarchy.contains_key("Bird"));
    assert!(hierarchy.contains_key("Animal"));
    
    // Verify specific relationships
    let mammal_extensions = hierarchy.get("Mammal").unwrap();
    assert!(mammal_extensions.contains("Animal"));
    
    let animal_extensions = hierarchy.get("Animal").unwrap();
    assert!(animal_extensions.contains("LivingThing"));
    
    // Test the direct extensions method
    let dog_extensions = registry.get_direct_extensions("Dog").unwrap().unwrap();
    assert_eq!(dog_extensions.len(), 1);
    assert!(dog_extensions.contains("Mammal"));
    
    // Test the direct implementors method
    let mammal_implementors = registry.get_direct_implementors("Mammal").unwrap().unwrap();
    assert_eq!(mammal_implementors.len(), 2);
    assert!(mammal_implementors.contains("Dog"));
    assert!(mammal_implementors.contains("Cat"));
    
    // Test getting all interfaces
    let all_interfaces = registry.get_all_interfaces().unwrap();
    assert_eq!(all_interfaces.len(), 5);
    assert!(all_interfaces.contains("Dog"));
    assert!(all_interfaces.contains("Cat"));
    assert!(all_interfaces.contains("Mammal"));
    assert!(all_interfaces.contains("Bird"));
    assert!(all_interfaces.contains("Animal"));
    assert!(all_interfaces.contains("LivingThing"));
}

#[test]
fn test_registry_visualization_error_propagation() {
    init_tracing!();
    info!("Testing registry visualization error propagation");
    
    // Create a context and code generator to test integration
    let context = Context::create();
    let file_path = PathBuf::from("test_visualization.csd");
    let code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Access the registry from the code generator
    let registry = &code_gen.registry_extensions;
    
    // Register some test interfaces
    registry.register_extension("Interface1", "BaseInterface").unwrap();
    registry.register_extension("Interface2", "BaseInterface").unwrap();
    
    // The registry should properly propagate errors when using the ? operator
    let hierarchy = registry.get_extension_hierarchy().unwrap();
    assert!(hierarchy.contains_key("Interface1"));
    assert!(hierarchy.contains_key("Interface2"));
    
    let extensions = registry.get_direct_extensions("Interface1").unwrap().unwrap();
    assert_eq!(extensions.len(), 1);
    assert!(extensions.contains("BaseInterface"));
    
    // Test error propagation format with proper context
    let non_existent = registry.get_direct_extensions("NonExistentInterface").unwrap();
    assert!(non_existent.is_none());
}

#[test]
fn test_registry_thread_safety() {
    init_tracing!();
    info!("Testing registry thread safety with visualization");
    
    use std::sync::Arc;
    use std::thread;
    
    // Create a shared registry
    let registry = Arc::new(ThreadSafeInterfaceExtensionRegistry::new());
    
    // Register some base interfaces
    registry.register_extension("BaseInterface1", "Root").unwrap();
    registry.register_extension("BaseInterface2", "Root").unwrap();
    
    // Create multiple threads that add and query interface relationships
    let mut handles = vec![];
    
    for i in 0..10 {
        let registry_clone = registry.clone();
        let handle = thread::spawn(move || {
            let interface_name = format!("TestInterface{}", i);
            let base_name = if i % 2 == 0 { "BaseInterface1" } else { "BaseInterface2" };
            
            // Register a new extension
            registry_clone.register_extension(&interface_name, base_name).unwrap();
            
            // Query the registry using visualization methods
            let hierarchy = registry_clone.get_extension_hierarchy().unwrap();
            assert!(hierarchy.contains_key(&interface_name));
            
            let extensions = registry_clone.get_direct_extensions(&interface_name).unwrap().unwrap();
            assert_eq!(extensions.len(), 1);
            assert!(extensions.contains(base_name));
            
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