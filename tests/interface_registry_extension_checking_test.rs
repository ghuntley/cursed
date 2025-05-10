//! # Interface Registry Extension Checking Tests
//!
//! This module tests the implementation of the InterfaceTypeRegistryExtensionChecking
//! trait for reliable inheritance verification in interface type assertions.

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_path_finder_enhanced::InterfaceTypeRegistryExtensionChecking;
use cursed::error::Error;

use inkwell::context::Context;

mod common;

#[test]
fn test_interface_registry_extension_checking() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module").unwrap();
    
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
    
    // Set up inheritance relationships for testing
    setup_test_inheritance_relationships(&mut codegen);
    
    // Using the enhanced interface path finder for relationship detection
    // Test transitive relationship - this should work with the enhanced path finder
    assert!(codegen.check_extension_relationship_enhanced("JSONFileReader", "Reader").unwrap());
    
    // Test unrelated interfaces
    assert!(!codegen.check_extension_relationship_enhanced("JSONFileReader", "Serializable").unwrap());
    
    // Test with non-existent interface
    assert!(!codegen.check_extension_relationship_enhanced("NonExistentInterface", "Reader").unwrap());
    
    // Test checking for reversed relationships
    assert!(!codegen.check_extension_relationship_enhanced("Reader", "JSONFileReader").unwrap());
    let (reversed, _) = codegen.detect_reversed_inheritance_enhanced("Reader", "JSONFileReader").unwrap();
    assert!(reversed);
    
    // Test visualization of interface hierarchies
    let hierarchy = codegen.visualize_interface_hierarchy("Reader", 2).unwrap();
    assert!(hierarchy.contains("Interface Hierarchy for 'Reader'"));
    assert!(hierarchy.contains("FileReader"));
    assert!(hierarchy.contains("NetworkReader"));
    assert!(hierarchy.contains("JSONFileReader"));
}

/// Test that path finding works even with partial relationships in the registry
#[test]
fn test_partial_extension_relationships() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module").unwrap();
    
    // Set up just a few test interfaces
    codegen.register_type_in_registry(1001, "Animal");
    codegen.register_type_in_registry(1002, "Mammal");
    codegen.register_type_in_registry(1003, "Dog");
    
    // Set up minimal inheritance relationships
    let mut test_inheritance_map = std::collections::HashMap::new();
    let mut animal_extensions = std::collections::HashSet::new();
    animal_extensions.insert("Mammal".to_string());
    test_inheritance_map.insert("Animal".to_string(), animal_extensions);
    
    let mut mammal_extensions = std::collections::HashSet::new();
    mammal_extensions.insert("Dog".to_string());
    test_inheritance_map.insert("Mammal".to_string(), mammal_extensions);
    
    codegen.test_inheritance_map = Some(test_inheritance_map);
    
    // Path finding for direct and indirect relationships
    let direct_path = codegen.find_interface_path_enhanced("Mammal", "Animal").unwrap();
    assert_eq!(direct_path.path(), &vec!["Mammal".to_string(), "Animal".to_string()]);
    
    // Test indirect relationship via path finding (Dog -> Mammal -> Animal)
    let indirect_path = codegen.find_interface_path_enhanced("Dog", "Animal").unwrap();
    assert_eq!(indirect_path.path(), &vec!["Dog".to_string(), "Mammal".to_string(), "Animal".to_string()]);
    
    // Check that the extension relationship check uses the enhanced path finder internally
    assert!(codegen.check_extension_relationship_enhanced("Dog", "Animal").unwrap());
}

/// Test hook to set up inheritance relationships for testing
fn setup_test_inheritance_relationships(codegen: &mut LlvmCodeGenerator) {
    // Create a test inheritance map in codegen
    let mut test_inheritance_map = std::collections::HashMap::new();
    
    // Set up FileReader extends Reader
    let mut reader_extensions = std::collections::HashSet::new();
    reader_extensions.insert("FileReader".to_string());
    reader_extensions.insert("NetworkReader".to_string());
    test_inheritance_map.insert("Reader".to_string(), reader_extensions);
    
    // Set up JSONFileReader extends FileReader
    let mut filereader_extensions = std::collections::HashSet::new();
    filereader_extensions.insert("JSONFileReader".to_string());
    test_inheritance_map.insert("FileReader".to_string(), filereader_extensions);
    
    // Set up JSONSerializable extends Serializable
    let mut serializable_extensions = std::collections::HashSet::new();
    serializable_extensions.insert("JSONSerializable".to_string());
    test_inheritance_map.insert("Serializable".to_string(), serializable_extensions);
    
    // Store this in the code generator for testing
    codegen.test_inheritance_map = Some(test_inheritance_map);
}