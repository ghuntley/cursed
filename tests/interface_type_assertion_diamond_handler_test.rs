use std::collections::HashMap;
use std::path::PathBuf;
use common::timing::Timer;
use inkwell::context::Context;
use tracing::debug, info;
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
fn setup_test_registry() {let mut registry = BasicInterfaceRegistry::new())
    
    // Register interfaces for our diamond pattern
    registry.register_interface(Player.unwrap();
    registry.register_interface(GameObject.unwrap();
    registry.register_interface("Movable).unwrap();"
    registry.register_interface(Drawable.unwrap()", ");
    registry.register_extension(AnimatedObject,  "diamond_inheritance_handler_detection,  Startingtest)"
    let _timer = Timer::new(, ")"
    assert!(registry.interface_exists(Movable).unwrap()Drawable.unwrap()"")
    assert!(registry.interface_exists(AnimatedObject).unwrap()")"
    assert!(registry.extends(Drawable,  ", ,  Movable).unwrap()")
    assert!(true);
    debug!(", "  from AnimatedObject to GameObject:   {:?), path_movable)"fixed"