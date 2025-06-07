use std::collections::{HashMap, HashSet};
use cursed::codegen::llvm::interface_type_registry::InterfaceTypeRegistry;
use cursed::error::Error;

//! # Interface Registry Extension Visualization Tests
//!
//! This module tests the enhanced implementation of interface registry extension visualization
//! with proper path finding, hierarchy visualization, and cycle detection.


mod common;

/// Initialize tracing for tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        common::tracing::setup()
    };
}

/// Test visualizing a simple interface hierarchy
#[test]
fn test_interface_hierarchy_visualization() {
    init_tracing!();
    
    // Create a test registry with interface hierarchy
    let mut registry = InterfaceTypeRegistry::new();
    registry.register_type(1001, "Reader".to_string());
    registry.register_type(1002, "FileReader".to_string());
    registry.register_type(1003, "JSONFileReader".to_string());
    registry.register_type(1004, "NetworkReader".to_string());
    registry.register_type(1005, "Writer".to_string());
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = std::sync::Arc::new(
        cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
    );
    
    // Update the registry to use the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone());
    registry.register_type(1001, "Reader".to_string());
    registry.register_type(1002, "FileReader".to_string());
    registry.register_type(1003, "JSONFileReader".to_string());
    registry.register_type(1004, "NetworkReader".to_string());
    registry.register_type(1005, "Writer".to_string());
    
    // Add relationship data to the extension registry
    extension_registry.register_extension("FileReader", "Reader").unwrap();
    extension_registry.register_extension("JSONFileReader", "FileReader").unwrap();
    extension_registry.register_extension("NetworkReader", "Reader").unwrap();
    
    // Test visualizing the hierarchy
    let visualization = registry.visualize_interface_hierarchy(1001).unwrap();
    
    // Check that the visualization contains all expected interfaces
    assert!(visualization.contains("Reader"), "Visualization should include Reader");
    assert!(visualization.contains("FileReader"), "Visualization should include FileReader");
    assert!(visualization.contains("JSONFileReader"), "Visualization should include JSONFileReader");
    assert!(visualization.contains("NetworkReader"), "Visualization should include NetworkReader");
    
    // Writer should not be in the hierarchy
    assert!(!visualization.contains("Writer"), "Writer is not part of Reader hierarchy and should not be included");
    
    // Print the visualization for manual verification
    println!("Interface Hierarchy Visualization:\n{}", visualization);
}

/// Test visualizing a path between interfaces
#[test]
fn test_inheritance_path_visualization() {
    init_tracing!();
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = std::sync::Arc::new(
        cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
    );
    
    // Create a test registry with the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone());
    
    // Register interfaces
    registry.register_type(1001, "Reader".to_string());
    registry.register_type(1002, "FileReader".to_string());
    registry.register_type(1003, "JSONFileReader".to_string());
    registry.register_type(1004, "NetworkReader".to_string());
    registry.register_type(1005, "Writer".to_string());
    
    // Add relationship data to the extension registry
    extension_registry.register_extension("FileReader", "Reader").unwrap();
    extension_registry.register_extension("JSONFileReader", "FileReader").unwrap();
    extension_registry.register_extension("NetworkReader", "Reader").unwrap();
    
    // Test visualizing the path from JSONFileReader to Reader
    let path_visualization = registry.visualize_inheritance_path(1003, 1001).unwrap();
    
    // Check that the path contains all expected interfaces
    assert!(path_visualization.contains("JSONFileReader"), "Path should include JSONFileReader");
    assert!(path_visualization.contains("FileReader"), "Path should include FileReader");
    assert!(path_visualization.contains("Reader"), "Path should include Reader");
    
    // Print the path visualization for manual verification
    println!("Inheritance Path Visualization:\n{}", path_visualization);
    
    // Test with non-existent path (Writer to Reader)
    let no_path_visualization = registry.visualize_inheritance_path(1005, 1001).unwrap();
    
    // Check that the visualization indicates no path
    assert!(no_path_visualization.contains("No inheritance path"), "Visualization should indicate no path exists");
    
    // Check for solution suggestions
    assert!(no_path_visualization.contains("Possible solutions"), "Visualization should suggest solutions");
    
    // Print the no-path visualization for manual verification
    println!("No Path Visualization:\n{}", no_path_visualization);
    
    // Test detecting reversed relationships
    let reversed_visualization = registry.visualize_inheritance_path(1001, 1003).unwrap();
    
    // Check that the visualization detects the reversed relationship
    assert!(reversed_visualization.contains("REVERSED path"), "Visualization should detect the reversed relationship");
    
    // Check for fix suggestion
    assert!(reversed_visualization.contains("Possible fix"), "Visualization should suggest a fix for the reversed relationship");
    
    // Print the reversed visualization for manual verification
    println!("Reversed Relationship Visualization:\n{}", reversed_visualization);
}

/// Test generating a DOT graph for the interface hierarchy
#[test]
fn test_dot_graph_generation() {
    init_tracing!();
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = std::sync::Arc::new(
        cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
    );
    
    // Create a test registry with the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone());
    
    // Register interfaces
    registry.register_type(1001, "Reader".to_string());
    registry.register_type(1002, "FileReader".to_string());
    registry.register_type(1003, "JSONFileReader".to_string());
    registry.register_type(1004, "NetworkReader".to_string());
    registry.register_type(1005, "Writer".to_string());
    
    // Add relationship data to the extension registry
    extension_registry.register_extension("FileReader", "Reader").unwrap();
    extension_registry.register_extension("JSONFileReader", "FileReader").unwrap();
    extension_registry.register_extension("NetworkReader", "Reader").unwrap();
    
    // Generate DOT graph
    let dot_graph = registry.generate_dot_graph().unwrap();
    
    // Check that the graph contains all expected interfaces
    assert!(dot_graph.contains("digraph InterfaceHierarchy"), "DOT graph should have proper header");
    assert!(dot_graph.contains("Reader"), "DOT graph should include Reader");
    assert!(dot_graph.contains("FileReader"), "DOT graph should include FileReader");
    assert!(dot_graph.contains("JSONFileReader"), "DOT graph should include JSONFileReader");
    
    // Check for edges in the graph
    assert!(dot_graph.contains("-> node_1001"), "DOT graph should contain edge to Reader");
    
    // Print the DOT graph for manual verification
    println!("DOT Graph:\n{}", dot_graph);
}

/// Test detecting inheritance cycles
#[test]
fn test_inheritance_cycle_detection() {
    init_tracing!();
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = std::sync::Arc::new(
        cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
    );
    
    // Create a test registry with the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone());
    
    // Register interfaces
    registry.register_type(1001, "Reader".to_string());
    registry.register_type(1002, "FileReader".to_string());
    registry.register_type(1003, "JSONFileReader".to_string());
    registry.register_type(1004, "NetworkReader".to_string());
    
    // Add normal relationship data to the extension registry
    extension_registry.register_extension("FileReader", "Reader").unwrap();
    extension_registry.register_extension("JSONFileReader", "FileReader").unwrap();
    extension_registry.register_extension("NetworkReader", "Reader").unwrap();
    
    // Check for cycles - should be none
    let cycles = registry.check_inheritance_cycles().unwrap();
    assert!(cycles.is_none(), "There should be no inheritance cycles");
    
    // Create a cycle by adding a circular dependency
    extension_registry.register_extension("Reader", "JSONFileReader").unwrap();
    
    // Check for cycles again - should detect one
    let cycles = registry.check_inheritance_cycles().unwrap();
    assert!(cycles.is_some(), "Should detect the inheritance cycle");
    
    let cycle_visualization = cycles.unwrap();
    assert!(cycle_visualization.contains("Detected Interface Inheritance Cycles"), "Visualization should indicate detected cycles");
    assert!(cycle_visualization.contains("WARNING"), "Visualization should include a warning");
    
    // Print the cycle visualization for manual verification
    println!("Cycle Detection Visualization:\n{}", cycle_visualization);
}