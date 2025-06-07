use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::interface_type_assertion_diamond_inheritance::DiamondInheritanceHandler;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use inkwell::context::Context;
use std::cell::RefCell;
use std::sync::Arc;
use tracing::{info, debug};
use common::tracing::setup as init_tracing;

#[cfg(test)]
mod tests {
    
    // Include common test utilities
    #[path = "common.rs"]
    mod common;
    
    /// Set up a mock type registry with a diamond inheritance pattern
    fn setup_diamond_inheritance_registry(code_gen: &mut LlvmCodeGenerator) {
        // Create the registry if it doesn't exist
        let registry = code_gen.ensure_interface_type_registry();
        
        // Register basic types
        let game_object_id = registry.register_type("GameObject", None).unwrap();
        let movable_id = registry.register_type("Movable", None).unwrap();
        let drawable_id = registry.register_type("Drawable", None).unwrap();
        let animated_object_id = registry.register_type("AnimatedObject", None).unwrap();
        let player_id = registry.register_type("Player", None).unwrap();
        
        // Set up inheritance relationships
        
        // Player implements all interfaces
        registry.register_implementation(player_id, game_object_id).unwrap();
        registry.register_implementation(player_id, movable_id).unwrap();
        registry.register_implementation(player_id, drawable_id).unwrap();
        registry.register_implementation(player_id, animated_object_id).unwrap();
        
        // Set up the diamond: AnimatedObject inherits from both Movable and Drawable
        registry.register_implementation(animated_object_id, movable_id).unwrap();
        registry.register_implementation(animated_object_id, drawable_id).unwrap();
        
        // Both Movable and Drawable inherit from GameObject
        registry.register_implementation(movable_id, game_object_id).unwrap();
        registry.register_implementation(drawable_id, game_object_id).unwrap();
    }
    
    /// Test detecting diamond inheritance patterns
    #[test]
    fn test_detect_diamond_inheritance() {
        init_tracing();
        info!("Testing diamond inheritance pattern detection");
        
        // Create a code generator with context
        let context = Context::create();
        let module = context.create_module("diamond_test");
        let builder = context.create_builder();
        
        let mut code_gen = LlvmCodeGenerator::new(&context, module, builder);
        
        // Set up the mock registry with diamond inheritance
        setup_diamond_inheritance_registry(&mut code_gen);
        
        // Test diamond detection from Player to GameObject
        let result = code_gen.detect_diamond_inheritance("Player", "GameObject");
        assert!(result.is_ok())
        
        let diamond_info = result.unwrap();
        assert!(diamond_info.is_some(), "Diamond inheritance should be detected");
        
        if let Some(info) = diamond_info {
            info!("Detected diamond inheritance with base: {}", info.base_type_name);
            assert_eq!(info.base_type_name, "GameObject", "Base type should be GameObject");
            assert!(info.paths.len() >= 2, "Should detect at least 2 paths");
            
            // Visualize the diamond pattern
            let visualization = code_gen.visualize_diamond_inheritance(
                "Player", "GameObject", &info
            ).unwrap();
            
            info!("Diamond visualization:\n{}", visualization);
            assert!(visualization.contains("Diamond Inheritance Pattern");
        }
    }
    
    /// Test finding multiple inheritance paths
    #[test]
    fn test_find_all_inheritance_paths() {
        init_tracing();
        info!("Testing finding all inheritance paths in diamond pattern");
        
        // Create a code generator with context
        let context = Context::create();
        let module = context.create_module("diamond_paths_test");
        let builder = context.create_builder();
        
        let mut code_gen = LlvmCodeGenerator::new(&context, module, builder);
        
        // Set up the mock registry with diamond inheritance
        setup_diamond_inheritance_registry(&mut code_gen);
        
        // Get type IDs
        let registry = code_gen.ensure_interface_type_registry();
        let player_id = registry.get_type_id("Player").unwrap();
        let game_object_id = registry.get_type_id("GameObject").unwrap();
        
        // Find all paths from Player to GameObject
        let paths = code_gen.find_all_inheritance_paths(player_id, game_object_id, 10).unwrap();
        
        info!("Found {} paths from Player to GameObject", paths.len());
        assert!(paths.len() >= 2, "Should find at least 2 paths in the diamond");
        
        // Verify path contents
        for (i, path) in paths.iter().enumerate() {
            let path_types: Vec<String> = path.iter()
                .map(|&id| registry.get_type_name(id).unwrap_or_else(|_| format!("Unknown(0x{:x})", id)))
                .collect();
                
            info!("Path {}: {}", i+1, path_types.join(" -> ");
            
            // Verify each path starts with Player and ends with GameObject
            assert_eq!(registry.get_type_name(path[0]).unwrap(), "Player");
            assert_eq!(registry.get_type_name(path[path.len()-1]).unwrap(), "GameObject");
        }
    }
    
    /// Test visualizing diamond inheritance patterns
    #[test]
    fn test_diamond_inheritance_visualization() {
        init_tracing();
        info!("Testing diamond inheritance visualization");
        
        // Create a code generator with context
        let context = Context::create();
        let module = context.create_module("diamond_viz_test");
        let builder = context.create_builder();
        
        let mut code_gen = LlvmCodeGenerator::new(&context, module, builder);
        
        // Set up the mock registry with diamond inheritance
        setup_diamond_inheritance_registry(&mut code_gen);
        
        // Test paths from Player to AnimatedObject
        let result = code_gen.detect_diamond_inheritance("Player", "AnimatedObject");
        assert!(result.is_ok())
        
        let diamond_info = result.unwrap();
        if let Some(info) = diamond_info {
            let visualization = code_gen.visualize_diamond_inheritance(
                "Player", "AnimatedObject", &info
            ).unwrap();
            
            info!("Diamond visualization for Player -> AnimatedObject:\n{}", visualization);
            
            // Check for expected patterns in the visualization
            assert!(visualization.contains("Diamond Inheritance Pattern");
            assert!(visualization.contains("Player");
            assert!(visualization.contains("AnimatedObject");
            
            // There should be method resolution order information
            assert!(visualization.contains("Method Resolution Order");
        } else {
            panic!("Failed to detect diamond inheritance from Player to AnimatedObject");
        }
    }
    
    /// Test the entire diamond inheritance handling system with a real example
    #[test]
    fn test_complete_diamond_inheritance_system() {
        init_tracing();
        info!("Testing complete diamond inheritance handling system");
        
        // Create a code generator with context
        let context = Context::create();
        let module = context.create_module("complete_diamond_test");
        let builder = context.create_builder();
        
        let mut code_gen = LlvmCodeGenerator::new(&context, module, builder);
        
        // Set up a more complex inheritance hierarchy
        let registry = code_gen.ensure_interface_type_registry();
        
        // Base interfaces
        let identifiable_id = registry.register_type("Identifiable", None).unwrap();
        let positionable_id = registry.register_type("Positionable", None).unwrap();
        let visual_id = registry.register_type("Visual", None).unwrap();
        
        // Mid-level interfaces
        let game_object_id = registry.register_type("GameObject", None).unwrap();
        let ui_element_id = registry.register_type("UIElement", None).unwrap();
        let physics_object_id = registry.register_type("PhysicsObject", None).unwrap();
        
        // High-level interfaces
        let character_id = registry.register_type("Character", None).unwrap();
        let button_id = registry.register_type("Button", None).unwrap();
        
        // Concrete types
        let player_id = registry.register_type("Player", None).unwrap();
        let menu_button_id = registry.register_type("MenuButton", None).unwrap();
        
        // Set up base inheritance
        registry.register_implementation(game_object_id, identifiable_id).unwrap();
        registry.register_implementation(game_object_id, positionable_id).unwrap();
        registry.register_implementation(game_object_id, visual_id).unwrap();
        
        registry.register_implementation(ui_element_id, identifiable_id).unwrap();
        registry.register_implementation(ui_element_id, positionable_id).unwrap();
        registry.register_implementation(ui_element_id, visual_id).unwrap();
        
        registry.register_implementation(physics_object_id, positionable_id).unwrap();
        
        // Mid-level inheritance
        registry.register_implementation(character_id, game_object_id).unwrap();
        registry.register_implementation(character_id, physics_object_id).unwrap();
        
        registry.register_implementation(button_id, ui_element_id).unwrap();
        
        // Concrete implementation
        registry.register_implementation(player_id, character_id).unwrap();
        registry.register_implementation(menu_button_id, button_id).unwrap();
        
        // Test for diamond patterns
        info!("Testing complex inheritance diamonds");
        
        // Player to Positionable should have diamond pattern
        let result = code_gen.detect_diamond_inheritance("Player", "Positionable");
        assert!(result.is_ok())
        
        if let Some(info) = result.unwrap() {
            info!("Detected diamond: Player -> Positionable through {} paths", info.paths.len());
            let viz = code_gen.visualize_diamond_inheritance("Player", "Positionable", &info).unwrap();
            debug!("Visualization:\n{}", viz);
            
            assert!(info.paths.len() >= 2, "Should have multiple paths in diamond");
        } else {
            panic!("Failed to detect diamond from Player to Positionable");
        }
        
        // A simpler, non-diamond path (MenuButton to UIElement) should not detect diamond
        let result = code_gen.detect_diamond_inheritance("MenuButton", "UIElement");
        assert!(result.is_ok())
        
        if let Some(info) = result.unwrap() {
            panic!("Incorrectly detected diamond from MenuButton to UIElement");
        } else {
            info!("Correctly did not detect diamond from MenuButton to UIElement");
        }
    }
}