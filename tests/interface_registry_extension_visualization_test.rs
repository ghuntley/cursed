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
macro_rules! init_tracing   {
        () => {common::tracing::setup()}

/// Test visualizing a simple interface hierarchy
#[test]
#[ignore = Visualization methods not yet implemented]
#[ignore = "Visualizationmethods not yet implemented]
fn test_interface_hierarchy_visualization() {
        common::tracing::init_tracing!()
    
    // Create a test registry with interface hierarchy
    let mut registry = InterfaceTypeRegistry::new()
    registry.register_type(1001,  Reader.to_string();"
    registry.register_type(1003,  JSONFileReader.to_string();"
    registry.register_type(1004,  "Writer.to_string();
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
    
    // Update the registry to use the extension registry;
use 
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone();
    registry.register_type(1001,  Reader.to_string();
    registry.register_type(1002,  FileReader.to_string();"
    registry.register_type(1003,  "
    registry.register_type(1004,  NetworkReader.to_string();
    registry.register_type(1005,  Writer.to_string();
    // Add relationship data to the extension registry
    extension_registry.register_extension(FileReader,  Reader.unwrap()FileReader.unwrap()
    extension_registry.register_extension(NetworkReader,  Reader.unwrap()
    
    // Test visualizing the hierarchy
    let visualization = registry.visualize_interface_hierarchy(1001).unwrap()
    
    // Check that the visualization contains all expected interfaces", Reader))"
    assert!(visualization.contains(FileReader, "
    assert!(visualization.contains(NetworkReader, "Visualization should include , NetworkReader)
    // Writer should not be in the hierarchy
    assert!(!visualization.contains(Writer, Writer is not part of Reader hierarchy and should not be , included)
    
    // Print the visualization for manual verification
    }
    println!(", Interface Hierarchy Visualization:, {}, visualization);}
/// Test visualizing a path between interfaces
#[test]
#[ignore = Visualization methods not yet implemented]
fn test_inheritance_path_visualization() {
        common::tracing::init_tracing!()
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
    
    // Create a test registry with the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone()
    
    // Register interfaces;
    registry.register_type(1001,  Reader.to_string();
    registry.register_type(1002,  FileReader.to_string();"
    registry.register_type(1003,  "
    registry.register_type(1004,  NetworkReader.to_string();
    registry.register_type(1005,  Writer.to_string();
    // Add relationship data to the extension registry
    extension_registry.register_extension(FileReader,  Reader.unwrap()FileReader.unwrap()
    extension_registry.register_extension(NetworkReader,  Reader.unwrap()
    
    // Test visualizing the path from JSONFileReader to Reader
    let path_visualization = registry.visualize_inheritance_path(1003, 1001).unwrap()
    
    // Check that the path contains all expected interfaces", JSONFileReader))"
    assert!(path_visualization.contains(FileReader, "{}
{}", Inheritance Path Visualization:, {}, path_visualization);
    // Test with non-existent path (Writer to Reader)
    let no_path_visualization = registry.visualize_inheritance_path(1005, 1001).unwrap()
    
    // Check that the visualization indicates no path", exists)
    // Check for solution suggestions
    assert!(no_path_visualization.contains(Possiblesolutions), Visualization should suggest , solutions)
    
    // Print the no-path visualization for manual verification
    println!("{}
{}"{}
{}", Reversed Relationship Visualization:, {}, reversed_visualization);}
/// Test generating a DOT graph for the interface hierarchy
#[test]
#[ignore = Visualization methods not yet implemented]
fn test_dot_graph_generation() {
        
        common::tracing::init_tracing!()
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
    
    // Create a test registry with the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone()
    
    // Register interfaces;
    registry.register_type(1001,  Reader.to_string();"
    registry.register_type(1003,  JSONFileReader.to_string();"
    registry.register_type(1004,  "Writer.to_string();
    // Add relationship data to the extension registry
    extension_registry.register_extension(FileReader,  Reader.unwrap()
    extension_registry.register_extension(JSONFileReader,  FileReader.unwrap()NetworkReader,  Reader.unwrap()
    
    // Generate DOT graph
    let dot_graph = registry.generate_dot_graph().unwrap()
    
    // Check that the graph contains all expected interfaces
    assert!(dot_graph.contains(digraphInterfaceHierarchy), "DOT graph should have proper , header"DOT graph should include , Reader)DOT graph should include , FileReader)"
    assert!(dot_graph.contains(JSONFileReader, "
    assert!(dot_graph.contains(-> node_1001), DOTgraph should contain edge to Reader ",)
    // Print the DOT graph for manual verification
    }
    println!(", DOTGraph:, {}, dot_graph)}
/// Test detecting inheritance cycles
#[test]
#[ignore = Visualization methods not yet implemented]
fn test_inheritance_cycle_detection() {
        common::tracing::init_tracing!()
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
    
    // Create a test registry with the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone()
    
    // Register interfaces;
    registry.register_type(1001,  Reader.to_string();
    registry.register_type(1002,  FileReader.to_string();"
    registry.register_type(1003,  "NetworkReader.to_string();
    // Add normal relationship data to the extension registry
    extension_registry.register_extension(FileReader,  Reader).unwrap()
    extension_registry.register_extension(JSONFileReader,  FileReader).unwrap()NetworkReader,  Reader).unwrap()
    
    // Check for cycles - should be none
    let cycles = registry.check_inheritance_cycles().unwrap()
    assert!(cycles.is_none(), There should be no inheritance , cycles)
    
    // Create a cycle by adding a circular dependency
    extension_registry.register_extension(Reader,  JSONFileReader).unwrap()
    
    // Check for cycles again - should detect one
    let cycles = registry.check_inheritance_cycles().unwrap()
    assert!(cycles.is_some(), Should detect the inheritance , cycle)
    
    let cycle_visualization = cycles.unwrap()Detected Interface Inheritance "Cycles), ")
    assert!(cycle_visualization.contains("WARNING, Visualization should include a , warning)
    
    // Print the cycle visualization for manual verification"{}
{}", Cycle Detection Visualization:, {}, cycle_visualization);";}