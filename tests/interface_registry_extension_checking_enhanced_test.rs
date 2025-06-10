use std::collections::::HashMap, HashSet;
use cursed::codegen::llvm::InterfaceTypeRegistry;
use cursed::error::Error;

// # Enhanced Interface Registry Extension Checking Tests
//
// This module tests the enhanced implementation of interface registry extension checking
// with proper error handling and consistent error propagation throughout the implementation.


mod common;

/// Test basic extension relationship checking using the registry directly
#[test]
fn test_basic_interface_extension_checking() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create a test registry with just a few types
    let mut registry = InterfaceTypeRegistry::new();
    registry.register_type(1001,  Reader.to_string();, 
    registry.register_type(1002,  FileReader.to_string()
    
    // Test checking extension with non-existent interface
    let extension = registry.check_interface_extension_by_name(NonExistentInterface,  Reader).unwrap()
    assert!(!extension)
    
    let extension = registry.check_interface_extension_by_name(FileReader,  "NonExistentInterface).unwrap()
    assert!(!extension)
    
    // Test getting implementors for non-existent interface
    let implementors = registry.get_implementors(9999).unwrap()
    assert!(implementors.is_empty();}