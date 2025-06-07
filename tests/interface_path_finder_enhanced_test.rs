use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_path_finder_enhanced::{InterfaceInheritancePath, EnhancedInterfacePathFinder};
use cursed::codegen::llvm::{InterfaceTypeRegistryAccess, InterfaceRegistryVisualizationIntegration};
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use cursed::error::Error;
use inkwell::context::Context;

//! # Enhanced Interface Path Finder Tests
//!
//! This module tests the enhanced implementation of the interface path finder with
//! proper error handling, visualizations, and registry integration.



mod common;

#[test]
fn test_interface_path_finder_enhanced() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.csd"));
    
    // Set up some test interfaces in the registry
    // Reader -> FileReader -> JSONFileReader
    // Reader -> NetworkReader
    // Serializable -> JSONSerializable
    codegen.register_type_in_registry(1001, "Reader");
    codegen.register_type_in_registry(1002, "FileReader");
    codegen.register_type_in_registry(1003, "JSONFileReader");
    codegen.register_type_in_registry(1004, "NetworkReader");
    codegen.register_type_in_registry(1005, "Serializable");
    codegen.register_type_in_registry(1006, "JSONSerializable");
    
    // Now we need to manually set up the inheritance relationships
    // since the registry doesn't have this capability yet
    // This is done through a test hook in the LlvmCodeGenerator
    setup_test_inheritance_relationships(&mut codegen);
    
    // Test finding a simple path
    let path = codegen.find_interface_path_enhanced("JSONFileReader", "Reader").unwrap();
    assert_eq!(path.path(), &vec!["JSONFileReader".to_string()), "FileReader".to_string()), "Reader".to_string())]);
    
    // Test finding path between unrelated interfaces
    let result = codegen.find_interface_path_enhanced("JSONFileReader", "Serializable");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No path found"));
    
    // Test finding path with non-existent interface
    let result = codegen.find_interface_path_enhanced("NonExistentInterface", "Reader");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("does not exist"));
    
    // Test finding alternative paths
    let paths = codegen.find_alternative_paths_enhanced("JSONFileReader", "Reader", 5).unwrap();
    assert_eq!(paths.len(), 1); // Only one path exists in our test setup
    
    // Test checking extension relationship
    assert!(codegen.check_extension_relationship_enhanced_test("JSONFileReader", "Reader").unwrap());
    assert!(!codegen.check_extension_relationship_enhanced_test("JSONFileReader", "Serializable").unwrap());
    
    // Test reversed inheritance detection
    let (reversed, message) = codegen.detect_reversed_inheritance_enhanced_test("Reader", "JSONFileReader").unwrap();
    assert!(reversed);
    assert!(message.contains("Reversed inheritance detected"));
    
    // Test hierarchy visualization
    let hierarchy = codegen.visualize_interface_hierarchy("Reader", 2).unwrap();
    assert!(hierarchy.contains("Interface Hierarchy for 'Reader'"));
    assert!(hierarchy.contains("FileReader"));
    assert!(hierarchy.contains("NetworkReader"));
    
    // Test DOT graph generation
    let dot_graph = codegen.generate_interface_hierarchy_dot_graph().unwrap();
    assert!(dot_graph.contains("digraph interface_hierarchy"));
    assert!(dot_graph.contains("\"FileReader\" -> \"Reader\""));
    assert!(dot_graph.contains("\"JSONFileReader\" -> \"FileReader\""));
}

/// Test hook to set up inheritance relationships for testing
fn setup_test_inheritance_relationships(codegen: &mut LlvmCodeGenerator) {
    // This function would ideally use internal APIs to set up the inheritance
    // relationships in the registry. Since we don't have access to that yet,
    // we're using reflection to insert test data.
    
    // For a real implementation, this would be done through proper registry APIs
    // or by extending the InterfaceTypeRegistry with methods to record relationships
    
    // Create a test inheritance map in codegen
    let mut test_inheritance_map = std::collections::HashMap::new();
    
    // Set up FileReader extends Reader
    let mut reader_extensions = std::collections::HashSet::new();
    reader_extensions.insert("FileReader".to_string());
    reader_extensions.insert("NetworkReader".to_string());
    test_inheritance_map.insert("Reader".to_string()), reader_extensions);
    
    // Set up JSONFileReader extends FileReader
    let mut filereader_extensions = std::collections::HashSet::new();
    filereader_extensions.insert("JSONFileReader".to_string());
    test_inheritance_map.insert("FileReader".to_string()), filereader_extensions);
    
    // Set up JSONSerializable extends Serializable
    let mut serializable_extensions = std::collections::HashSet::new();
    serializable_extensions.insert("JSONSerializable".to_string());
    test_inheritance_map.insert("Serializable".to_string()), serializable_extensions);
    
    // Store this in the code generator for testing
    // This would be implemented differently in a real system
    codegen.test_inheritance_map = Some(test_inheritance_map);
}

/// Test that the error messages are formatted correctly and contain useful information
#[test]
fn test_interface_path_finder_enhanced_error_messages() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.csd"));
    
    // Set up some test interfaces in the registry
    codegen.register_type_in_registry(1001, "Reader");
    codegen.register_type_in_registry(1002, "FileReader");
    
    // Set up inheritance: FileReader extends Reader
    let mut test_inheritance_map = std::collections::HashMap::new();
    let mut reader_extensions = std::collections::HashSet::new();
    reader_extensions.insert("FileReader".to_string());
    test_inheritance_map.insert("Reader".to_string()), reader_extensions);
    codegen.test_inheritance_map = Some(test_inheritance_map);
    
    // Test error message for non-existent interface
    let err = codegen.find_interface_path_enhanced("NonExistentInterface", "Reader").unwrap_err();
    assert!(err.to_string().contains("does not exist in the registry"));
    
    // Test error message for no path
    let err = codegen.find_interface_path_enhanced("Reader", "FileReader").unwrap_err();
    assert!(err.to_string().contains("No path found"));
    assert!(err.to_string().contains("Did you mean to assert as the other way around"));
    
    // Test error message for alternative paths
    let err = codegen.find_alternative_paths_enhanced("Reader", "FileReader", 5).unwrap_err();
    assert!(err.to_string().contains("No alternative paths found"));
    
    // Test reversed inheritance message
    let (reversed, message) = codegen.detect_reversed_inheritance_enhanced_test("Reader", "FileReader").unwrap();
    assert!(reversed);
    assert!(message.contains("Reversed inheritance detected"));
    assert!(message.contains("The actual inheritance path is"));
}

/// Test the path visualization functionality
#[test]
fn test_interface_inheritance_path_visualization() {
    // Test the InterfaceInheritancePath struct directly
    let path = InterfaceInheritancePath::new(
        vec!["Child".to_string()), "Parent".to_string()), "GrandParent".to_string())],
        "Child".to_string()),
        "GrandParent".to_string()
    );
    
    // Test string representation
    assert_eq!(path.to_string_representation(), "Child -> Parent -> GrandParent");
    
    // Test visual representation
    let visual = path.to_visual_representation();
    assert!(visual.contains("Interface Inheritance Path:"));
    assert!(visual.contains("Child"));
    assert!(visual.contains("Parent"));
    assert!(visual.contains("GrandParent"));
    
    // Test empty path
    let empty_path = InterfaceInheritancePath::new(
        vec![],
        "Source".to_string()),
        "Target".to_string()
    );
    assert_eq!(empty_path.to_string_representation(), "No path from 'Source' to 'Target'.");
    assert_eq!(empty_path.is_empty(), true);
    assert_eq!(empty_path.len(), 0);
}

/// Test the DOT graph generation
#[test]
fn test_interface_hierarchy_dot_graph() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.csd"));
    
    // Set up some test interfaces in the registry
    codegen.register_type_in_registry(1001, "Animal");
    codegen.register_type_in_registry(1002, "Mammal");
    codegen.register_type_in_registry(1003, "Bird");
    codegen.register_type_in_registry(1004, "Dog");
    codegen.register_type_in_registry(1005, "Cat");
    codegen.register_type_in_registry(1006, "Eagle");
    
    // Set up inheritance relationships
    let mut test_inheritance_map = std::collections::HashMap::new();
    
    // Animal is the root
    let mut animal_extensions = std::collections::HashSet::new();
    animal_extensions.insert("Mammal".to_string());
    animal_extensions.insert("Bird".to_string());
    test_inheritance_map.insert("Animal".to_string()), animal_extensions);
    
    // Mammal extensions
    let mut mammal_extensions = std::collections::HashSet::new();
    mammal_extensions.insert("Dog".to_string());
    mammal_extensions.insert("Cat".to_string());
    test_inheritance_map.insert("Mammal".to_string()), mammal_extensions);
    
    // Bird extensions
    let mut bird_extensions = std::collections::HashSet::new();
    bird_extensions.insert("Eagle".to_string());
    test_inheritance_map.insert("Bird".to_string()), bird_extensions);
    
    codegen.test_inheritance_map = Some(test_inheritance_map);
    
    // Generate the DOT graph
    let dot_graph = codegen.generate_interface_hierarchy_dot_graph().unwrap();
    
    // Verify DOT graph content
    assert!(dot_graph.contains("digraph interface_hierarchy"));
    assert!(dot_graph.contains("rankdir=BT")); // Bottom to top direction
    
    // Check nodes
    assert!(dot_graph.contains("\"Animal\" [label=\"Animal\"]"));
    assert!(dot_graph.contains("\"Mammal\" [label=\"Mammal\"]"));
    assert!(dot_graph.contains("\"Bird\" [label=\"Bird\"]"));
    assert!(dot_graph.contains("\"Dog\" [label=\"Dog\"]"));
    assert!(dot_graph.contains("\"Cat\" [label=\"Cat\"]"));
    assert!(dot_graph.contains("\"Eagle\" [label=\"Eagle\"]"));
    
    // Check edges
    assert!(dot_graph.contains("\"Mammal\" -> \"Animal\""));
    assert!(dot_graph.contains("\"Bird\" -> \"Animal\""));
    assert!(dot_graph.contains("\"Dog\" -> \"Mammal\""));
    assert!(dot_graph.contains("\"Cat\" -> \"Mammal\""));
    assert!(dot_graph.contains("\"Eagle\" -> \"Bird\""));
}