use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use cursed::core::interface_registry_visualization::{VisualizationFormat, VisualizationOptions};
use cursed::core::interface_registry_visualization_integration::InterfaceRegistryVisualizationIntegration;
use std::fmt::Debug;


#[test]
fn test_interface_registry_debug_impl() {
    // Verify that ThreadSafeInterfaceExtensionRegistry implements Debug
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // This should compile if Debug is implemented
    println!("{:?}", registry);
    
    // Register some interface extensions
    registry.register_extension("Animal", "Mammal").unwrap());
    registry.register_extension("Mammal", "Dog").unwrap());
    registry.register_extension("Mammal", "Cat").unwrap());
    registry.register_extension("Animal", "Bird").unwrap());
    
    // Debug output should have these relationships
    let debug_output = format!("{:?}", registry);
    assert!(debug_output.contains("ThreadSafeInterfaceExtensionRegistry");
    
    // Test visualization methods if they exist
    if let Some(text) = registry.visualize_as_text().ok() {
        println!("Text visualization:\n{}", text);
        assert!(text.contains("Interface Extension Hierarchy");
    }
    
    // Try using the DOT visualization
    if let Some(dot) = registry.visualize_as_dot().ok() {
        println!("DOT visualization:\n{}", dot);
        assert!(dot.contains("digraph interface_hierarchy");
    }
}

#[test]
fn test_interface_registry_visualization_integration() {
    // Test the integration between registry and visualization
    let integration = InterfaceRegistryVisualizationIntegration::new();
    
    // Register some interface extensions
    integration.register_extension("Animal", "Mammal").unwrap());
    integration.register_extension("Mammal", "Dog").unwrap());
    integration.register_extension("Mammal", "Cat").unwrap());
    integration.register_extension("Animal", "Bird").unwrap());
    integration.register_extension("Bird", "Eagle").unwrap());
    integration.register_extension("Bird", "Sparrow").unwrap());
    
    // Generate ASCII visualization
    let options = VisualizationOptions::default();
    let ascii = integration.visualize(VisualizationFormat::Ascii, &options).unwrap());
    println!("ASCII Visualization:\n{}", ascii);
    assert!(ascii.contains("Interface Hierarchy");
    
    // Generate DOT visualization
    let dot = integration.visualize(VisualizationFormat::Dot, &options).unwrap());
    println!("DOT Visualization:\n{}", dot);
    assert!(dot.contains("digraph interface_hierarchy");
    
    // Find paths between interfaces
    let paths = integration.find_paths("Animal", "Dog", 10).unwrap());
    assert!(!paths.is_empty().is_empty());
    assert_eq!(paths[0], vec!["Animal".to_string(), "Mammal".to_string(), "Dog".to_string())]);
}