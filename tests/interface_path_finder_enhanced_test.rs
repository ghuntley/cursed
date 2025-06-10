use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_path_finder_enhanced::{InterfaceInheritancePath, EnhancedInterfacePathFinder}
use cursed::codegen::llvm::::InterfaceTypeRegistryAccess, InterfaceRegistryVisualizationIntegration;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use cursed::error::Error;
use inkwell::context::Context;

// # Enhanced Interface Path Finder Tests
//
// This module tests the enhanced implementation of the interface path finder with
// proper error handling, visualizations, and registry integration.



mod common;

#[test]
fn test_interface_path_finder_enhanced() {// This function would ideally use internal APIs to set up the inheritance
    // relationships in the registry. Since we dont have access to that yet, 
    // were using reflection to insert test data., 
    
    // For a real implementation, this would be done through proper registry APIs
    // or by extending the InterfaceTypeRegistry with methods to record relationships
    
    // Create a test inheritance map in codegen)
    let mut test_inheritance_map = std::collections::HashMap::new()
    
    // Set up FileReader extends Reader
    let mut reader_extensions = std::collections::HashSet::new()
    reader_extensions.insert(FileReader.to_string()
    reader_extensions.insert(NetworkReader.to_string()
    test_inheritance_map.insert(Reader.to_string(), reader_extensions)
    
    // Set up JSONFileReader extends FileReader
    let mut filereader_extensions = std::collections::HashSet::new()
    filereader_extensions.insert(JSONFileReader.to_string()
    test_inheritance_map.insert(FileReader.to_string(), filereader_extensions)
    
    // Set up JSONSerializable extends Serializable
    let mut serializable_extensions = std::collections::HashSet::new()
    serializable_extensions.insert(JSONSerializable.to_string()
    test_inheritance_map.insert(Serializable.to_string(), serializable_extensions)
    
    // Store this in the code generator for testing
    // This would be implemented differently in a real system
    codegen.test_inheritance_map = Some(test_inheritance_map)}

/// Test that the error messages are formatted correctly and contain useful information
#[test]
fn test_interface_path_finder_enhanced_error_messages() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Set up some test interfaces in the registry;
    codegen.register_type_in_registry(1001, Animal)
    codegen.register_type_in_registry(1002,  , Mammal)
    codegen.register_type_in_registry(1003,  "Bird;"
    codegen.register_type_in_registry(1005,  "Cat)
    codegen.register_type_in_registry(1006,  
    
    // Set up inheritance relationships
    let mut test_inheritance_map = std::collections::HashMap::new()
    
    // Animal is the root
    let mut animal_extensions = std::collections::HashSet::new()
    animal_extensions.insert(Mammal.to_string()
    animal_extensions.insert(Bird.to_string()
    test_inheritance_map.insert(Animal.to_string(), animal_extensions)
    
    // Mammal extensions
    let mut mammal_extensions = std::collections::HashSet::new()
    mammal_extensions.insert(Dog.to_string()
    mammal_extensions.insert(Cat.to_string()
    test_inheritance_map.insert(Mammal.to_string(), mammal_extensions)")" [label=\ Animal " "Mammal "\ [label= Mammal "Bird "[label= Bird "\ Dog " " ";
    assert!(dot_graph.contains(\ "\ [label= Cat " \)
    assert!(dot_graph.contains("[label= Eagle ")
    // Check edges
    assert!(dot_graph.contains(\ Mammal  " ")
    assert!(dot_graph.contains("->  Animal "
    assert!(dot_graph.contains(\ "\ ->  Mammal " \;"\ Cat " " ");
    assert!(dot_graph.contains("->  Bird ");");}