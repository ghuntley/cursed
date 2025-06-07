use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::interface_registry_visualization_improved::ImprovedInterfaceRegistryVisualization;
use cursed::error::Error;



// Common setup code for registry with a test hierarchy
fn setup_test_registry() -> InterfaceRegistry {
    let registry = InterfaceRegistry::new();
    
    // Define a test hierarchy
    // Animal <- Mammal <- Dog
    //        <- Bird   <- Eagle
    //        <- Fish   <- Shark
    // Mammal also implements Pet
    // Dog also implements Pet
    
    // Register interfaces and their extensions
    registry.register_interface_extension("Dog", "Mammal").unwrap();
    registry.register_interface_extension("Mammal", "Animal").unwrap();
    registry.register_interface_extension("Dog", "Pet").unwrap();
    registry.register_interface_extension("Mammal", "Pet").unwrap();
    registry.register_interface_extension("Bird", "Animal").unwrap();
    registry.register_interface_extension("Eagle", "Bird").unwrap();
    registry.register_interface_extension("Fish", "Animal").unwrap();
    registry.register_interface_extension("Shark", "Fish").unwrap();
    
    registry
}

#[test]
fn test_get_extension_hierarchy() {
    let registry = setup_test_registry();
    
    let hierarchy = registry.get_extension_hierarchy().unwrap();
    
    // Check that Dog extends Mammal and Pet
    let dog_extensions = hierarchy.get("Dog").unwrap();
    assert!(dog_extensions.contains(&"Mammal".to_string())
    assert!(dog_extensions.contains(&"Pet".to_string())
    
    // Check that Mammal extends Animal and Pet
    let mammal_extensions = hierarchy.get("Mammal").unwrap();
    assert!(mammal_extensions.contains(&"Animal".to_string())
    assert!(mammal_extensions.contains(&"Pet".to_string())
    
    // Verify all expected extensions are present
    assert_eq!(hierarchy.len(), 7); // Dog, Mammal, Bird, Eagle, Fish, Shark, and Animal all have extensions
}

#[test]
fn test_does_extend() {
    let registry = setup_test_registry();
    
    // Direct extensions
    assert!(registry.does_extend("Dog", "Mammal").unwrap();
    assert!(registry.does_extend("Mammal", "Animal").unwrap();
    
    // Transitive extensions
    assert!(registry.does_extend("Dog", "Animal").unwrap();
    
    // Non-extensions
    assert!(!registry.does_extend("Dog", "Bird").unwrap();
    assert!(!registry.does_extend("Eagle", "Mammal").unwrap();
    
    // Self-extension (should always be true)
    assert!(registry.does_extend("Dog", "Dog").unwrap();
}

#[test]
fn test_find_interface_paths() {
    let registry = setup_test_registry();
    
    // Find path from Dog to Animal
    let paths = registry.find_interface_paths("Dog", "Animal", 10).unwrap();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec!["Dog", "Mammal", "Animal"]);
    
    // Find path from Dog to Pet
    let paths = registry.find_interface_paths("Dog", "Pet", 10).unwrap();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec!["Dog", "Pet"]);
    
    // No path should exist from Dog to Bird
    let paths = registry.find_interface_paths("Dog", "Bird", 10).unwrap();
    assert_eq!(paths.len(), 0);
    
    // Path from Dog to itself (should be just Dog)
    let paths = registry.find_interface_paths("Dog", "Dog", 10).unwrap();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec!["Dog"]);
}

#[test]
fn test_visualize_hierarchy_ascii() {
    let registry = setup_test_registry();
    
    let visualization = registry.visualize_hierarchy_ascii().unwrap();
    println!("{}", visualization);
    
    // Check that the visualization includes all interfaces
    assert!(visualization.contains("Animal");
    assert!(visualization.contains("Mammal");
    assert!(visualization.contains("Dog");
    assert!(visualization.contains("Bird");
    assert!(visualization.contains("Eagle");
    assert!(visualization.contains("Fish");
    assert!(visualization.contains("Shark");
}

#[test]
fn test_generate_detailed_error_message() {
    let registry = setup_test_registry();
    
    // Error for incompatible types
    let error_message = registry.generate_detailed_error_message(
        "Eagle", "Mammal", "test.csd:42"
    ).unwrap();
    
    // Check that the error message is detailed
    assert!(error_message.contains("Eagle");
    assert!(error_message.contains("Mammal");
    assert!(error_message.contains("test.csd:42");
    assert!(error_message.contains("No inheritance path");
    
    // Error for types with a valid path
    let error_message = registry.generate_detailed_error_message(
        "Dog", "Animal", "test.csd:42"
    ).unwrap();
    
    // Check that the error shows the path
    assert!(error_message.contains("Valid inheritance paths");
    assert!(error_message.contains("Dog -> Mammal -> Animal");
}

#[test]
fn test_detect_reversed_inheritance() {
    let registry = setup_test_registry();
    
    // Animal extends nothing, Mammal extends Animal (reversed)
    assert!(registry.detect_reversed_inheritance("Animal", "Mammal").unwrap();
    
    // Mammal extends Animal, Animal extends nothing (not reversed)
    assert!(!registry.detect_reversed_inheritance("Mammal", "Animal").unwrap();
    
    // Dog extends Mammal (correct), Mammal extends Dog (incorrect, reversed)
    assert!(!registry.detect_reversed_inheritance("Dog", "Mammal").unwrap();
    assert!(registry.detect_reversed_inheritance("Mammal", "Dog").unwrap();
    
    // Unrelated types should return false
    assert!(!registry.detect_reversed_inheritance("Eagle", "Dog").unwrap();
}

#[test]
fn test_generate_fix_suggestions() {
    let registry = setup_test_registry();
    
    // Get suggestions for reversed inheritance
    let suggestions = registry.generate_fix_suggestions("Animal", "Mammal").unwrap();
    
    // Should suggest using reversed assertion
    assert!(suggestions.iter().any(|s| s.contains("inheritance relationship is reversed"))
    
    // Get suggestions for types with common interfaces
    let suggestions = registry.generate_fix_suggestions("Dog", "Mammal").unwrap();
    
    // Should suggest common interfaces
    assert!(suggestions.iter().any(|s| s.contains("Pet"))
    
    // Get suggestions for unrelated types
    let suggestions = registry.generate_fix_suggestions("Eagle", "Dog").unwrap();
    
    // Should suggest implementing interface explicitly
    assert!(suggestions.iter().any(|s| s.contains("explicitly implement"))
}

#[test]
fn test_generate_interface_hierarchy_dot() {
    let registry = setup_test_registry();
    
    let dot_graph = registry.generate_interface_hierarchy_dot().unwrap();
    
    // Check that the DOT graph includes all interfaces
    assert!(dot_graph.contains("digraph InterfaceHierarchy");
    assert!(dot_graph.contains("Animal");
    assert!(dot_graph.contains("Mammal");
    assert!(dot_graph.contains("Dog");
    assert!(dot_graph.contains("extends");
}

#[test]
fn test_get_direct_implementors() {
    let registry = setup_test_registry();
    
    // Get direct implementors of Animal
    let implementors = registry.get_direct_implementors("Animal").unwrap().unwrap();
    assert!(implementors.contains(&"Mammal".to_string())
    assert!(implementors.contains(&"Bird".to_string())
    assert!(implementors.contains(&"Fish".to_string())
    assert_eq!(implementors.len(), 3);
    
    // Get direct implementors of Mammal
    let implementors = registry.get_direct_implementors("Mammal").unwrap().unwrap();
    assert!(implementors.contains(&"Dog".to_string())
    assert_eq!(implementors.len(), 1);
    
    // Get direct implementors of Pet
    let implementors = registry.get_direct_implementors("Pet").unwrap().unwrap();
    assert!(implementors.contains(&"Dog".to_string())
    assert!(implementors.contains(&"Mammal".to_string())
    assert_eq!(implementors.len(), 2);
}

#[test]
fn test_get_all_interfaces() {
    let registry = setup_test_registry();
    
    let interfaces = registry.get_all_interfaces().unwrap();
    
    // Check that all interfaces are included
    assert!(interfaces.contains("Animal");
    assert!(interfaces.contains("Mammal");
    assert!(interfaces.contains("Dog");
    assert!(interfaces.contains("Bird");
    assert!(interfaces.contains("Eagle");
    assert!(interfaces.contains("Fish");
    assert!(interfaces.contains("Shark");
    assert!(interfaces.contains("Pet");
}