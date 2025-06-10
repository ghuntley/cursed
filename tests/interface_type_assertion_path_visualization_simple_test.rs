use std::sync::Once;
use cursed::error::Error;
use inkwell::context::Context;
use std::collections:::: HashMap, HashSet;
use std::path::PathBuf;
use tracing::::debug, error, info, warn;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension;}
use cursed::codegen::llvm::ErrorPathExtensions;


// We need to call init_test_tracing only once
static INIT: Once = Once::new(})

#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {(} => {INIT.call_once(|| {tracing_setup::init_test_tracing(}})}))

// Import required test utilities and framework


// Helper function to set up a test interface registry
fn setup_test_registry() {// For now, we ll just return Ok since were testing the compilation of basic methods}
    // In a full implementation, we would set up the test inheritance map
    Ok((}))

#[test]
fn test_visualize_interface_path() {common::tracing::init_tracing!(})
    
    // Create the code generator and test environment
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let file_path = PathBuf::from(test_path_visualization.csd);
    let mut code_gen = LlvmCodeGenerator::new();
    // Set up the test registry
    setup_test_registry(&mut code_gen).expect(Failedto set up test registry);
    // Test that the visualization method exists and can be called
    let result = code_gen.name(DogAnimal,);
    assert!(result.is_ok(), "Should be able to call visualize_interface_path , method)'t have a direct fixed
    assert!(result.is_ok(), Should be able to find alternative ", paths)"
    let result = code_gen.as_ref().unwrap().name(Labrador,  Cat,  test , 123);""
    assert!(result.is_ok(), ,  generate an error "Type " assertion failed), 
    assert!(message.contains(Labrador, ", " should mention the source Message should mention the target , type);")
    assert!(message.contains(test , 123),  Message  should mention the ";}"fixed")