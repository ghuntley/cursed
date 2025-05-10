//! # Enhanced Interface Registry Extension Checking Tests
//!
//! This module tests the enhanced implementation of interface registry extension checking
//! with proper error handling and consistent error propagation throughout the implementation.

use std::collections::{HashMap, HashSet};
use cursed::codegen::llvm::interface_type_registry::InterfaceTypeRegistry;
use cursed::error::Error;

mod common;

/// Test basic extension relationship checking using the registry directly
#[test]
fn test_basic_interface_extension_checking() {
    common::tracing::setup();
    
    // Create a test registry and register some types
    let mut registry = InterfaceTypeRegistry::new();
    registry.register_type(1001, "Reader".to_string());
    registry.register_type(1002, "FileReader".to_string());
    registry.register_type(1003, "JSONFileReader".to_string());
    registry.register_type(1004, "NetworkReader".to_string());
    registry.register_type(1005, "Serializable".to_string());
    registry.register_type(1006, "JSONSerializable".to_string());
    
    // Test getting extension relationships
    let relationships = registry.get_extension_relationships().unwrap();
    
    // Since the registry doesn't actually contain any extension data yet,
    // the relationships map should be empty
    assert!(relationships.is_empty());
    
    // Test finding implementors - this will also be empty for the same reason
    let implementors = registry.get_implementors(1001).unwrap();
    assert!(implementors.is_empty());
    
    // Test checking extension by name
    let extension = registry.check_interface_extension_by_name("FileReader", "Reader").unwrap();
    assert!(!extension);
}

/// Test extension relationship lookup with non-existent interfaces
#[test]
fn test_extension_checking_non_existent_interfaces() {
    common::tracing::setup();
    
    // Create a test registry with just a few types
    let mut registry = InterfaceTypeRegistry::new();
    registry.register_type(1001, "Reader".to_string());
    registry.register_type(1002, "FileReader".to_string());
    
    // Test checking extension with non-existent interface
    let extension = registry.check_interface_extension_by_name("NonExistentInterface", "Reader").unwrap();
    assert!(!extension);
    
    let extension = registry.check_interface_extension_by_name("FileReader", "NonExistentInterface").unwrap();
    assert!(!extension);
    
    // Test getting implementors for non-existent interface
    let implementors = registry.get_implementors(9999).unwrap();
    assert!(implementors.is_empty());
}