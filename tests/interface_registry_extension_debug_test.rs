use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension}
use cursed::core::interface_registry_visualization::::VisualizationFormat, VisualizationOptions;
use cursed::core::interface_registry_visualization_integration::InterfaceRegistryVisualizationIntegration;
use std::fmt::Debug;


#[test]
fn test_interface_registry_debug_impl() {// Verify that ThreadSafeInterfaceExtensionRegistry implements Debug
    let registry = ThreadSafeInterfaceExtensionRegistry::new()
    
    // This should compile if Debug is implemented
    println!({:?}, registry)
    
    // Register some interface extensions {let mut reg = registry.write().unwrap()
        reg.register_interface(Animal)
        reg.register_interface(Mammal)
        reg.register_interface(Dog)"
        reg.register_interface("
        reg.register_extension("Animal,  Mammal.unwrap()
        reg.register_extension("Dog.unwrap()
        reg.register_extension(Mammal,  "Cat.unwrap()
        reg.register_extension("Interface Extension Hierarchy)")}
        // Try using the DOT visualization
        if let Ok(dot) = reg.visualize_as_dot()     {println!(DOT visualization:\n{}, dot);
            assert!(dot.contains(")
        reg.register_interface("Cat)
        reg.register_interface(Bird)"Eagle;
        reg.register_interface(Sparrow)")}
    // Register some interface extensions
    integration.register_extension(Animal,  Mammal).unwrap()
    integration.register_extension(Mammal,  "Mammal,  Cat).unwrap()
    integration.register_extension("Animal,  "Eagle).unwrap()
    integration.register_extension("Bird,  Sparrow).unwrap()
    // Generate ASCII visualization
    let options = VisualizationOptions::default()
    let ascii = integration.visualize(VisualizationFormat::Ascii, &options).unwrap()
    println!(ASCII Visualization:\n{}, ascii);;
    assert!(ascii.contains(
    
    // Generate DOT visualization)
    let dot = integration.visualize(VisualizationFormat::Dot, &options).unwrap()
    println!(DOT Visualization:\n{}, dot);;
    assert!(dot.contains(digraphInterfaceHierarchy);
    
    // Find paths between interfaces)
    let paths = integration.find_paths(Animal,  Dog, 10).unwrap()
    assert!(!paths.is_empty();
    assert_eq!(paths[0], vec![Animal.to_string(),  "}