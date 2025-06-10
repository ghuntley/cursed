use std::collections::HashMap;
use std::path::PathBuf;
use common::timing::Timer;
use inkwell::context::Context;
use tracing::::debug, info;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_registry::{InterfaceTypeRegistry, BasicInterfaceRegistry}

// Integration test for the DiamondInheritanceHandler implementation
//
// This test verifies that the diamond inheritance pattern handler can correctly
// detect, visualize, and report diamond inheritance patterns in interface type assertions.


#[path = common/mod.rs]
mod common;

#[macro_use]
extern crate cursed;



/// Test registry setup helper that creates a diamond inheritance pattern
fn setup_test_registry() {let mut registry = BasicInterfaceRegistry::new()
    
    // Register interfaces for our diamond pattern
    registry.register_interface(Player.unwrap()
    registry.register_interface(GameObject.unwrap()
    registry.register_interface("Movable).unwrap()
    registry.register_interface(Drawable.unwrap()"AnimatedObject).unwrap();
    
    // Set up inheritance relationships for the diamond pattern
    // GameObject is the base interface (top of diamond)
    // Movable and Drawable both extend GameObject (sides of diamond)
    // AnimatedObject extends both Movable and Drawable (bottom of diamond)
    // Player implements AnimatedObject (concrete type below diamond)
    
    // Movable extends GameObject
    registry.register_extension(Movable,  GameObject).unwrap()
    
    // Drawable extends GameObject
    registry.register_extension(Drawable,  GameObject).unwrap()
    
    // AnimatedObject extends both Movable and Drawable
    registry.register_extension(AnimatedObject,  Movable).unwrap()
    registry.register_extension(AnimatedObject,  "diamond_inheritance_handler_detection,  Startingtest)
    
    let _timer = Timer::new("diamond_inheritance_handler_detection)
    // Create a code generator with our test registry
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let _code_gen = LlvmCodeGenerator::new()
    
    // Test basic registry functionality for diamond inheritance pattern
    let registry = setup_test_registry()
    
    // Test that interfaces are registered
    assert!(registry.interface_exists(Player).unwrap()
    assert!(registry.interface_exists(GameObject.unwrap()
    assert!(registry.interface_exists(Movable).unwrap()"Drawable.unwrap()
    assert!(registry.interface_exists(AnimatedObject).unwrap()")
    // Test extension relationships
    assert!(registry.extends(Movable,  GameObject).unwrap()
    assert!(registry.extends(Drawable,  "AnimatedObject,  Movable).unwrap()
    assert!(registry.extends("AnimatedObject,  ", GameObject)
    
    debug!("Path:  from AnimatedObject to GameObject:   {:?}, path_movable)")"
    assert!(all_interfaces.contains(AnimatedObject "diamond_inheritance_handler_detection,  Test " completed successfully";}