//! Integration test for the DiamondInheritanceHandler implementation
//!
//! This test verifies that the diamond inheritance pattern handler can correctly
//! detect, visualize, and report diamond inheritance patterns in interface type assertions.

use std::collections::HashMap;

#[path = "common.rs"]
mod common;
use common::tracing::init_tracing;
use common::timing::Timer;

use inkwell::context::Context;
use tracing::{debug, info};

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::DiamondInheritanceHandler;
use cursed::codegen::llvm::interface_registry::{InterfaceTypeRegistry, InterfaceImplementation};
use cursed::codegen::llvm::InterfaceTypeRegistryExtensionCheckingAccess;

/// Test registry setup helper that creates a diamond inheritance pattern
fn setup_test_registry() -> InterfaceTypeRegistry<'static> {
    let mut registry = InterfaceTypeRegistry::new();
    
    // Register types for our diamond pattern
    registry.register_type(1, "Player", false).unwrap();
    registry.register_type(2, "GameObject", true).unwrap();
    registry.register_type(3, "Movable", true).unwrap();
    registry.register_type(4, "Drawable", true).unwrap();
    registry.register_type(5, "AnimatedObject", true).unwrap();
    
    // Set up inheritance relationships for the diamond pattern
    // GameObject is the base interface (top of diamond)
    // Movable and Drawable both extend GameObject (sides of diamond)
    // AnimatedObject extends both Movable and Drawable (bottom of diamond)
    // Player implements AnimatedObject (concrete type below diamond)
    
    // Movable extends GameObject
    registry.register_extension(3, 2).unwrap();
    
    // Drawable extends GameObject
    registry.register_extension(4, 2).unwrap();
    
    // AnimatedObject extends both Movable and Drawable
    registry.register_extension(5, 3).unwrap();
    registry.register_extension(5, 4).unwrap();
    
    // Player implements all interfaces in the diamond
    registry.register_implementation(1, 2).unwrap(); // Player implements GameObject
    registry.register_implementation(1, 3).unwrap(); // Player implements Movable
    registry.register_implementation(1, 4).unwrap(); // Player implements Drawable
    registry.register_implementation(1, 5).unwrap(); // Player implements AnimatedObject
    
    registry
}

#[test]
fn test_diamond_inheritance_handler_detection() {
    init_tracing!();
    info!(test_case = "diamond_inheritance_handler_detection", "Starting test");
    
    let _timer = Timer::new("diamond_inheritance_handler_detection");
    
    // Create a code generator with our test registry
    let context = Context::create();
    let mut code_gen = LlvmCodeGenerator::new(&context);
    
    // Set up the registry
    let registry = setup_test_registry();
    code_gen.internal_fields.insert(
        "interface_registry".to_string(),
        Box::new(registry)
    );
    
    // Test detection of diamond pattern
    let result = code_gen.detect_diamond_inheritance("Player", "GameObject");
    assert!(result.is_ok(), "Diamond detection failed: {:?}", result.err());
    
    // We should have found a diamond pattern
    let diamond_info = result.unwrap();
    assert!(diamond_info.is_some(), "No diamond pattern detected when one should exist");
    
    let info = diamond_info.unwrap();
    assert_eq!(info.concrete_type, "Player");
    assert_eq!(info.interface_type, "GameObject");
    assert!(info.left_path == "Movable" || info.left_path == "Drawable");
    assert!(info.right_path == "Movable" || info.right_path == "Drawable");
    assert_eq!(info.common_base, "GameObject");
    
    // Test visualization of the diamond pattern
    let visualization = code_gen.visualize_diamond_inheritance("Player", "GameObject", &diamond_info);
    assert!(visualization.is_ok(), "Visualization failed: {:?}", visualization.err());
    
    let viz_text = visualization.unwrap();
    debug!("Diamond visualization: {}", viz_text);
    
    // Verify visualization contains expected content
    assert!(viz_text.contains("Diamond Inheritance Pattern"));
    assert!(viz_text.contains("GameObject"));
    assert!(viz_text.contains("Movable"));
    assert!(viz_text.contains("Drawable"));
    assert!(viz_text.contains("Player"));
    assert!(viz_text.contains("All inheritance paths:"));
    
    // Test finding all diamond patterns for Player
    let all_patterns = code_gen.find_all_diamond_patterns("Player");
    assert!(all_patterns.is_ok(), "Finding all diamond patterns failed: {:?}", all_patterns.err());
    
    let patterns = all_patterns.unwrap();
    assert!(!patterns.is_empty(), "No diamond patterns found when at least one should exist");
    
    // Test has_diamond_inheritance
    let has_diamond = code_gen.has_diamond_inheritance("Player");
    assert!(has_diamond.is_ok(), "has_diamond_inheritance failed: {:?}", has_diamond.err());
    assert!(has_diamond.unwrap(), "Player should have diamond inheritance");
    
    // Test generating a full report
    let report = code_gen.generate_diamond_inheritance_report();
    assert!(report.is_ok(), "Report generation failed: {:?}", report.err());
    
    let report_text = report.unwrap();
    debug!("Diamond inheritance report: {}", report_text);
    assert!(report_text.contains("Diamond Inheritance Pattern Report"));
    assert!(report_text.contains("Player"));
    
    info!(test_case = "diamond_inheritance_handler_detection", "Test completed successfully");
}