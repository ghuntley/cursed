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

// Import required test utilities and framework
use cursed::error::Error;
use inkwell::context::Context;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tracing::{debug, error, info, warn};

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;

// Helper function to set up a test interface registry with complex hierarchies
fn setup_test_registry(registry: &ThreadSafeInterfaceExtensionRegistry) -> Result<(), Error> {
    // Set up a simple animal hierarchy
    registry.register_extension("Dog", "Animal")?;
    registry.register_extension("Cat", "Animal")?;
    registry.register_extension("Bird", "Animal")?;
    
    // Set up a more complex hierarchy with multiple levels
    registry.register_extension("Labrador", "Dog")?;
    registry.register_extension("GoldenRetriever", "Dog")?;
    registry.register_extension("SiberianHusky", "Dog")?;
    
    registry.register_extension("Siamese", "Cat")?;
    registry.register_extension("Persian", "Cat")?;
    
    registry.register_extension("Eagle", "Bird")?;
    registry.register_extension("Sparrow", "Bird")?;
    
    // Create a hierarchy for testing path visualization
    registry.register_extension("Renderer", "Component")?;
    registry.register_extension("AnimatedRenderer", "Renderer")?;
    registry.register_extension("InteractiveRenderer", "Renderer")?;
    registry.register_extension("AdvancedRenderer", "AnimatedRenderer")?;
    registry.register_extension("AdvancedRenderer", "InteractiveRenderer")?;
    
    // Add a more complex hierarchy for UI components
    registry.register_extension("UIComponent", "Component")?;
    registry.register_extension("Button", "UIComponent")?;
    registry.register_extension("TextField", "UIComponent")?;
    registry.register_extension("Panel", "UIComponent")?;
    registry.register_extension("Dialog", "Panel")?;
    registry.register_extension("AnimatedButton", "Button")?;
    registry.register_extension("AnimatedButton", "AnimatedRenderer")?;
    registry.register_extension("InteractiveTextField", "TextField")?;
    registry.register_extension("InteractiveTextField", "InteractiveRenderer")?;
    
    Ok(())
}

#[test]
fn test_generate_interface_hierarchy_dot() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization_integration.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry with a complex hierarchy
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Generate the DOT graph
    let result = code_gen.generate_interface_hierarchy_dot();
    assert!(result.is_ok(), "Should generate DOT graph without errors");
    
    // Validate DOT content
    let dot = result.unwrap();
    assert!(dot.contains("digraph interface_hierarchy"), "Should have the correct graph name");
    assert!(dot.contains("\"Dog\" -> \"Animal\""), "Should show Dog -> Animal relationship");
    assert!(dot.contains("\"Labrador\" -> \"Dog\""), "Should show Labrador -> Dog relationship");
    assert!(dot.contains("\"AnimatedButton\" -> \"Button\""), "Should show AnimatedButton -> Button relationship");
    assert!(dot.contains("\"AnimatedButton\" -> \"AnimatedRenderer\""), "Should show AnimatedButton -> AnimatedRenderer relationship");
}

#[test]
fn test_complex_path_finding() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization_integration.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry with a complex hierarchy
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Test finding a complex path through multiple interfaces
    let result = code_gen.find_interface_path("AnimatedButton", "Component");
    assert!(result.is_ok(), "Should find a path from AnimatedButton to Component");
    
    let path = result.unwrap();
    assert!(path.len() >= 3, "Path should have at least 3 nodes for AnimatedButton -> Button -> UIComponent -> Component");
    assert_eq!(path[0], "AnimatedButton");
    assert!(path.contains(&"Component".to_string()), "Path should contain Component as the final node");
}

#[test]
fn test_error_message_with_alternatives() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization_integration.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry with a complex hierarchy
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Generate an error message for interfaces with no direct path but multiple alternatives
    let result = code_gen.generate_path_error_message("Labrador", "Sparrow", "test.csd:123");
    assert!(result.is_ok(), "Should generate an error message");
    
    let message = result.unwrap();
    assert!(message.contains("Type assertion error"), "Message should mention the type assertion error");
    assert!(message.contains("Labrador"), "Message should mention the source type");
    assert!(message.contains("Sparrow"), "Message should mention the target type");
    assert!(message.contains("Alternative paths"), "Message should suggest alternative paths");
    assert!(message.contains("Animal"), "Message should mention the common ancestor Animal");
}

#[test]
fn test_interface_path_visualization() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization_integration.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry with a complex hierarchy
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Generate a visual representation of the path
    let result = code_gen.visualize_interface_path("Labrador", "Animal");
    assert!(result.is_ok(), "Should generate path visualization without errors");
    
    let visualization = result.unwrap();
    assert!(visualization.contains("Interface Inheritance Path:"), "Should have a title");
    assert!(visualization.contains("[Labrador]"), "Should show Labrador in the path");
    assert!(visualization.contains("[Dog]"), "Should show Dog in the path");
    assert!(visualization.contains("[Animal]"), "Should show Animal in the path");
    assert!(visualization.contains("digraph path"), "Should include DOT representation");
}

#[test]
fn test_diamond_inheritance_alternatives() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization_integration.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry with a complex hierarchy
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Test finding alternative paths in a diamond inheritance pattern
    let result = code_gen.find_alternative_paths("AdvancedRenderer", "Component", 5);
    assert!(result.is_ok(), "Should find alternative paths");
    
    let paths = result.unwrap();
    assert!(paths.len() >= 2, "Should find at least 2 alternative paths through the diamond inheritance");
    
    // At least one path should go through AnimatedRenderer
    let through_animated = paths.iter().any(|path| {
        path.contains(&"AnimatedRenderer".to_string()) && path.contains(&"Renderer".to_string())
    });
    assert!(through_animated, "Should find a path through AnimatedRenderer and Renderer");
    
    // Another path should go through InteractiveRenderer
    let through_interactive = paths.iter().any(|path| {
        path.contains(&"InteractiveRenderer".to_string()) && path.contains(&"Renderer".to_string())
    });
    assert!(through_interactive, "Should find a path through InteractiveRenderer and Renderer");
}