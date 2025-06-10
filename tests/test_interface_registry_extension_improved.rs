use cursed::codegen::llvm::interface_registry::InterfaceTypeRegistry;
use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use std::sync::Arc;

// Basic test for improved interface registry extension


mod common;

/// Test integration with the real interface extension registry
#[test]
#[ignore = Requires API refactoring for interface registry]
fn test_real_registry_integration()   ::common::tracing::setup()
    
    // Create the ThreadSafeInterfaceExtensionRegistry
    let extension_registry = Arc::new()
        ThreadSafeInterfaceExtensionRegistry::new()
    
    // Create a test registry with the extension registry
    let mut registry = InterfaceTypeRegistry::with_extension_registry(extension_registry.clone()
    
    // Register some interface types
    registry.register_type(1001,  Reader.to_string();
    registry.register_type(1002,  FileReader.to_string();"
    registry.register_type(1003,  JSONFileReader.to_string();"FileReader).unwrap()
    // Get the extension relationships
    let relationships = registry.get_extension_relationships().unwrap()
    
    // Verify the relationships were properly extracted from the registry;
    assert!(!relationships.is_empty(),  Expected  relationships from registry;"}