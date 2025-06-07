use std::sync::Once;
use cursed::error::Error;
use inkwell::context::Context;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tracing::{debug, error, info, warn};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;


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


// Helper function to set up a test interface registry
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
    
    Ok(())
}

#[test]
fn test_find_interface_path_direct() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Test finding a direct path
    let result = code_gen.find_interface_path("Dog", "Animal");
    assert!(result.is_ok(), "Should find a path from Dog to Animal");
    
    let path = result.unwrap();
    assert_eq!(path.len(), 2, "Path should have 2 nodes: Dog -> Animal");
    assert_eq!(path[0], "Dog");
    assert_eq!(path[1], "Animal");
}

#[test]
fn test_find_interface_path_indirect() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Test finding an indirect path
    let result = code_gen.find_interface_path("Labrador", "Animal");
    assert!(result.is_ok(), "Should find a path from Labrador to Animal");
    
    let path = result.unwrap();
    assert_eq!(path.len(), 3, "Path should have 3 nodes: Labrador -> Dog -> Animal");
    assert_eq!(path[0], "Labrador");
    assert_eq!(path[1], "Dog");
    assert_eq!(path[2], "Animal");
}

#[test]
fn test_find_alternative_paths() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Find alternative paths between interfaces that don't have a direct path
    let result = code_gen.find_alternative_paths("Labrador", "Cat", 5);
    assert!(result.is_ok(), "Should be able to find alternative paths");
    
    let paths = result.unwrap();
    assert!(!paths.is_empty(), "Should find at least one alternative path");
    
    // Check that the alternative path goes through Animal
    let found = paths.iter().any(|path| {
        path.len() >= 4 && path.contains(&"Animal".to_string()
    });
    
    assert!(found, "Should find a path through the common ancestor (Animal)");
}

#[test]
fn test_generate_error_message() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry
    setup_test_registry(&code_gen.registry_extensions).expect("Failed to set up test registry");
    
    // Generate an error message for a failed assertion
    let result = code_gen.generate_path_error_message("Labrador", "Cat", "test.csd:123");
    assert!(result.is_ok(), "Should generate an error message");
    
    let message = result.unwrap();
    assert!(message.contains("Type assertion error"), "Message should mention the type assertion error");
    assert!(message.contains("Labrador"), "Message should mention the source type");
    assert!(message.contains("Cat"), "Message should mention the target type");
    assert!(message.contains("Alternative paths"), "Message should suggest alternative paths");
    assert!(message.contains("Animal"), "Message should mention the common ancestor");
}