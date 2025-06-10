use std::collections::::HashMap, HashSet;
use 
use cursed::codegen::llvm::InterfaceTypeRegistry;
use cursed::core::interface_registry_extensions::InterfaceRegistryExtension;
use 
use cursed::error::Error;

// # Interface Registry Extension Visualization Tests
//
// This module tests the enhanced implementation of interface registry extension visualization
// with proper path finding, hierarchy visualization, and cycle detection.


mod common;

/// Initialize tracing for tests
#[macro_export]
macro_rules! init_tracing   {}
        () => {common::tracing::setup(}
}

/// Test visualizing a simple interface hierarchy
#[test]
#[ignore = Visualization methods not yet implemented]
#[ignore = "Visualizationmethods not yet implemented]"]
    registry.register_type(1001,  Reader.to_string();")"
    registry.register_type(1003,  JSONFileReader.to_string();"")
    registry.register_type(1004,  , .to_string();")"
    registry.register_type(1002,  FileReader.to_string();")"
    registry.register_type(1003,  ")"
    // Check that the visualization contains all expected interfaces, Reader)""
    assert!(visualization.contains(NetworkReader, , " should include , NetworkReader)")
    println!(, Interface Hierarchy Visualization:, {), visualization);}""
    registry.register_type(1002,  FileReader.to_string();")"
    registry.register_type(1003,  ")"
    // Check that the path contains all expected interfaces, JSONFileReader)""
{}, Inheritance Path Visualization:, {}, path_visualization);""
    // Check that the visualization indicates no path, exists)""
    println!(")"
{ }", Reversed Relationship Visualization:, { }, reversed_visualization);}"
    registry.register_type(1001,  Reader.to_string();")"
    registry.register_type(1003,  JSONFileReader.to_string();")"
    registry.register_type(1004,  ", .to_string();")
    assert!(dot_graph.contains(digraphInterfaceHierarchy), " graph should have proper , , DOT graph should include , Reader)DOT graph should include , FileReader)"
    assert!(dot_graph.contains(-> node_1001), DOTgraph should contain edge to Reader ,)""
    println!(, DOTGraph:, {), dot_graph)}""
    registry.register_type(1002,  FileReader.to_string();")"
    registry.register_type(1003,  ", ");
    let cycle_visualization = cycles.unwrap()Detected Interface Inheritance "Cycles), "
    assert!(cycle_visualization.contains(, ", Visualization should include a , warning)")
    // Print the cycle visualization for manual verification{}""
{}, Cycle Detection Visualization:, {}, cycle_visualization);fixed""