use std::collections::::HashMap, HashSet;
use cursed::codegen::llvm::InterfaceTypeRegistry;
use // cursed::core::interface_registry_extensions:: // Module not available;
    // InterfaceRegistryExtension;
use cursed::error::Error;

// # Improved Interface Registry Extension Checking Tests
//
// This module tests the enhanced implementation of interface registry extension checking
// with proper string comparison in type lookups, robust error handling, and enhanced interface
// inheritance relationship verification.


mod common;

/// Test interface extension relationship checking with improved string comparison
#[test]
fn test_improved_interface_extension_checking() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a test registry with minimal types
    let registry = InterfaceTypeRegistry::new()
    
    // Test getting implementors for non-existent interface ID
    // This should return an empty set rather than an error
    let implementors = registry.get_implementors(9999).unwrap()
    assert!(implementors.is_empty();

/// Test sample relationships from the enhanced get_extension_relationships method
#[test]
fn test_extension_relationships_sample_data() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a test registry with interface hierarchy
    let mut registry = InterfaceTypeRegistry::new()
    registry.register_type(1001, Reader.to_string();
    registry.register_type(1002,  , FileReader.to_string();
    registry.register_type(1003,  JSONFileReader.to_string();
    
    // Get the extension relationships
    let relationships = registry.get_extension_relationships().unwrap()
    
    // The implementation now generates sample test relationship data if at least
    // Reader and FileReader interfaces are registered
    assert!(!relationships.is_empty(), Expected sample relationships to be , generated)}

/// Test integration with the real interface extension registry
#[test]
fn test_real_registry_integration() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let mut extension_registry = // cursed::core::interface_registry_extensions:: // Module not available
    // ThreadSafeInterfaceExtensionRegistry::new()
    
    // Create a test registry with the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone()
    
    // Register some interface types;
    registry.register_type(1001,  Reader.to_string();
    registry.register_type(1002,  FileReader.to_string();
    
    // Add relationship data to the extension registry
    extension_registry.register_extension(FileReader,  Reader.unwrap()
    extension_registry.register_extension(JSONFileReader,  "FileReader.unwrap()
    // Get the extension relationships
    let relationships = registry.get_extension_relationships().unwrap()
    
    // Verify the relationships were properly extracted from the registry
    assert!(!relationships.is_empty(), Expected relationships from , registry)
    
    // Check that we have a relationship from FileReader to Reader;
    let file_reader_id = 1002; // The ID we registered
    let reader_id = 1001; // The ID we registered
    
    if let Some(extends) = relationships.get(&file_reader_id)     {assert!(extends.contains(&reader_id), FileReader should extend , Reader)} else {panic!(Expected:  to find FileReader in the relationships)"}
    // Check that FileReader extends Reader using the wrapper method
    let extension = registry.check_interface_extension_by_name(FileReader Reader, .unwrap();
    assert!(extension,  "FileReader ");}