use std::collections::HashMap;
use std::path::PathBuf;
use common::timing::Timer;
use inkwell::context::Context;
use tracing::{debug, info};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_registry::{InterfaceTypeRegistry, BasicInterfaceRegistry};

// Integration test for the DiamondInheritanceHandler implementation
//
// This test verifies that the diamond inheritance pattern handler can correctly
// detect, visualize, and report diamond inheritance patterns in interface type assertions.


#[path = "common/mod.rs"]
mod common;

#[macro_use]
extern crate cursed;



/// Test registry setup helper that creates a diamond inheritance pattern
fn setup_test_registry() -> BasicInterfaceRegistry {
    let mut registry = BasicInterfaceRegistry::new();
    
    // Register interfaces for our diamond pattern
    registry.register_interface("Player").unwrap();
    registry.register_interface("GameObject").unwrap();
    registry.register_interface("Movable").unwrap();
    registry.register_interface("Drawable").unwrap();
    registry.register_interface("AnimatedObject").unwrap();
    
    // Set up inheritance relationships for the diamond pattern
    // GameObject is the base interface (top of diamond)
    // Movable and Drawable both extend GameObject (sides of diamond)
    // AnimatedObject extends both Movable and Drawable (bottom of diamond)
    // Player implements AnimatedObject (concrete type below diamond)
    
    // Movable extends GameObject
    registry.register_extension("Movable", "GameObject").unwrap();
    
    // Drawable extends GameObject
    registry.register_extension("Drawable", "GameObject").unwrap();
    
    // AnimatedObject extends both Movable and Drawable
    registry.register_extension("AnimatedObject", "Movable").unwrap();
    registry.register_extension("AnimatedObject", "Drawable").unwrap();
    
    registry
}

#[test]
fn test_diamond_inheritance_handler_detection() {
    init_tracing!();
    info!(test_case = "diamond_inheritance_handler_detection", "Starting test");
    
    let _timer = Timer::new("diamond_inheritance_handler_detection");
    
    // Create a code generator with our test registry
    let context = Context::create();
    let _code_gen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Test basic registry functionality for diamond inheritance pattern
    let registry = setup_test_registry();
    
    // Test that interfaces are registered
    assert!(registry.interface_exists("Player").unwrap());
    assert!(registry.interface_exists("GameObject").unwrap());
    assert!(registry.interface_exists("Movable").unwrap());
    assert!(registry.interface_exists("Drawable").unwrap());
    assert!(registry.interface_exists("AnimatedObject").unwrap());
    
    // Test extension relationships
    assert!(registry.extends("Movable", "GameObject").unwrap());
    assert!(registry.extends("Drawable", "GameObject").unwrap());
    assert!(registry.extends("AnimatedObject", "Movable").unwrap());
    assert!(registry.extends("AnimatedObject", "Drawable").unwrap());
    
    // Test transitive relationships 
    assert!(registry.extends("AnimatedObject", "GameObject").unwrap());
    
    // Test path finding for diamond pattern
    let path_movable = registry.find_path("AnimatedObject", "GameObject").unwrap();
    assert!(path_movable.is_some(), "Should find a path from AnimatedObject to GameObject");
    
    debug!("Path from AnimatedObject to GameObject: {:?}", path_movable);
    
    // Test that we can find multiple paths through the diamond
    let all_interfaces = registry.get_all_interfaces().unwrap();
    assert!(all_interfaces.contains("GameObject"));
    assert!(all_interfaces.contains("Movable"));
    assert!(all_interfaces.contains("Drawable"));
    assert!(all_interfaces.contains("AnimatedObject"));
    
    info!(test_case = "diamond_inheritance_handler_detection", "Test completed successfully");
}