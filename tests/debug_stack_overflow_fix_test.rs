use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;

#[test]
fn test_debug_no_stack_overflow() {
    // Create a registry
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Test Debug formatting - this should not cause stack overflow
    let debug_output = format!("{:?}", *registry.read().unwrap());
    
    // Verify that the debug output contains expected information
    assert!(debug_output.contains("ThreadSafeInterfaceExtensionRegistry"));
    assert!(debug_output.contains("interface_count"));
    assert!(debug_output.contains("extensions_count"));
    assert!(debug_output.contains("implementers_count"));
    
    println!("Debug output: {}", debug_output);
}

#[test]
fn test_debug_nested_formatting() {
    // Create a registry
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Test nested Debug formatting (e.g., in a Vec with Arc reference)
    let vec_with_registry = vec![registry.clone()];
    let debug_output = format!("{:?}", vec_with_registry);
    
    // Should not cause stack overflow and should contain expected information
    assert!(debug_output.contains("RwLock"));
    
    println!("Nested debug output: {}", debug_output);
}
