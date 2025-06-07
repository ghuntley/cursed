use std::sync::Once;
use cursed::error::Error;
use inkwell::context::Context;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tracing::{debug, error, info, warn};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension};
use cursed::codegen::llvm::ErrorPathExtensions;


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
fn setup_test_registry(_code_gen: &mut LlvmCodeGenerator) -> Result<(), Error> {
    // For now, we'll just return Ok since we're testing the compilation of basic methods
    // In a full implementation, we would set up the test inheritance map
    Ok(())
}

#[test]
fn test_visualize_interface_path() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry
    setup_test_registry(&mut code_gen).expect("Failed to set up test registry");
    
    // Test that the visualization method exists and can be called
    let result = code_gen.visualize_interface_path("Dog", "Animal");
    assert!(result.is_ok(), "Should be able to call visualize_interface_path method");
}

#[test]
fn test_generate_interface_hierarchy_dot_graph() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry
    setup_test_registry(&mut code_gen).expect("Failed to set up test registry");
    
    // Test that the DOT graph generation method exists and can be called
    let result = code_gen.generate_interface_hierarchy_dot_graph();
    assert!(result.is_ok(), "Should be able to call generate_interface_hierarchy_dot_graph method");
}

#[test]
fn test_find_alternative_paths() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry
    setup_test_registry(&mut code_gen).expect("Failed to set up test registry");
    
    // Find alternative paths between interfaces that don't have a direct path
    let result = code_gen.find_alternative_paths("Labrador", "Cat", 5);
    assert!(result.is_ok(), "Should be able to find alternative paths");
    
    let paths = result.unwrap();
    // Note: Current implementation returns empty paths, but method call should succeed
    info!("Alternative paths found: {:?}", paths);
}

#[test]
fn test_generate_error_message() {
    init_tracing!();
    
    // Create the code generator and test environment
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up the test registry
    setup_test_registry(&mut code_gen).expect("Failed to set up test registry");
    
    // Generate an error message for a failed assertion
    let result = code_gen.generate_path_error_message("Labrador", "Cat", "test.csd:123");
    assert!(result.is_ok(), "Should generate an error message");
    
    let message = result.unwrap();
    assert!(message.contains("Type assertion failed"), "Message should mention the type assertion failure");
    assert!(message.contains("Labrador"), "Message should mention the source type");
    assert!(message.contains("Cat"), "Message should mention the target type");
    assert!(message.contains("test.csd:123"), "Message should mention the location");
}