/// Comprehensive Interface Method Dispatch Tests for CURSED
/// 
/// This module tests the complete interface dispatch system including:
/// - Method index lookup and resolution
/// - Runtime interface dispatch tables
/// - Error handling for method resolution failures
/// - Interface compliance checking
/// - Dynamic method dispatch correctness

use cursed::runtime::interface_dispatch::{
    InterfaceDispatchRegistry, InterfaceVTable, InterfaceValue, 
    initialize_interface_dispatch, get_global_dispatch_registry,
    register_global_interface, register_global_implementation,
    create_global_interface_value
};
use cursed::error::CursedError;
use cursed::runtime::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_interface_dispatch_registry_lookup() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize interface dispatch system
    initialize_interface_dispatch()?;
    
    // Create a test interface
    let interface_methods = vec![
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "test_method".to_string(),
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: 0,
        },
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "another_method".to_string(),
            param_types: vec!["str".to_string()],
            return_type: Some("str".to_string()),
            method_index: 1,
        },
    ];
    
    register_global_interface("TestInterface".to_string(), interface_methods)?;
    
    // Test that the interface was registered correctly by checking the global registry
    let registry = get_global_dispatch_registry()?;
    let registry_lock = registry.lock().unwrap();
    assert!(registry_lock.interface_methods.contains_key("TestInterface"));
    
    let methods = registry_lock.interface_methods.get("TestInterface").unwrap();
    assert_eq!(methods.len(), 2);
    assert_eq!(methods[0].name, "test_method");
    assert_eq!(methods[1].name, "another_method");
    
    Ok(())
}

#[test]
fn test_interface_vtable_creation() -> Result<(), CursedError> {
    // Test creation of interface vtables
    let mut vtable = InterfaceVTable::new(
        "TestInterface".to_string(),
        "TestStruct".to_string(),
    );
    
    // Add methods to vtable
    let method1 = cursed::runtime::interface_dispatch::VTableEntry {
        method_name: "test_method".to_string(),
        function_ptr: 0x1000,
        param_types: vec!["i32".to_string()],
        return_type: Some("i32".to_string()),
    };
    
    let method2 = cursed::runtime::interface_dispatch::VTableEntry {
        method_name: "another_method".to_string(),
        function_ptr: 0x2000,
        param_types: vec!["str".to_string()],
        return_type: Some("str".to_string()),
    };
    
    vtable.add_method(method1)?;
    vtable.add_method(method2)?;
    
    // Test method lookup
    let method = vtable.get_method("test_method");
    assert!(method.is_some());
    assert_eq!(method.unwrap().function_ptr, 0x1000);
    
    let method = vtable.get_method("nonexistent_method");
    assert!(method.is_none());
    
    // Test method index lookup
    let index = vtable.get_method_index("test_method");
    assert_eq!(index, Some(0));
    
    let index = vtable.get_method_index("another_method");
    assert_eq!(index, Some(1));
    
    Ok(())
}

#[test]
fn test_interface_dispatch_registry() -> Result<(), CursedError> {
    // Test the interface dispatch registry
    let mut registry = InterfaceDispatchRegistry::new();
    
    // Register interface
    let interface_methods = vec![
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "method1".to_string(),
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: 0,
        },
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "method2".to_string(),
            param_types: vec!["str".to_string()],
            return_type: Some("str".to_string()),
            method_index: 1,
        },
    ];
    
    registry.register_interface("TestInterface".to_string(), interface_methods)?;
    
    // Register implementation
    let mut method_implementations = HashMap::new();
    method_implementations.insert("method1".to_string(), 0x1000);
    method_implementations.insert("method2".to_string(), 0x2000);
    
    registry.register_implementation(
        "TestInterface".to_string(),
        "TestStruct".to_string(),
        method_implementations,
    )?;
    
    // Test getting vtable
    let vtable = registry.get_vtable("TestInterface", "TestStruct");
    assert!(vtable.is_some());
    
    let vtable = vtable.unwrap();
    assert_eq!(vtable.interface_name, "TestInterface");
    assert_eq!(vtable.concrete_type, "TestStruct");
    
    // Test method lookup in vtable
    let method = vtable.get_method("method1");
    assert!(method.is_some());
    assert_eq!(method.unwrap().function_ptr, 0x1000);
    
    Ok(())
}

#[test]
fn test_interface_value_creation() -> Result<(), CursedError> {
    // Test interface value creation and method dispatch
    let mut vtable = InterfaceVTable::new(
        "TestInterface".to_string(),
        "TestStruct".to_string(),
    );
    
    let method = cursed::runtime::interface_dispatch::VTableEntry {
        method_name: "test_method".to_string(),
        function_ptr: 0x1000,
        param_types: vec!["i32".to_string()],
        return_type: Some("i32".to_string()),
    };
    
    vtable.add_method(method)?;
    
    // Create interface value
    let interface_value = InterfaceValue::new(
        Arc::new(vtable),
        0x3000, // data pointer
        "TestInterface".to_string(),
        "TestStruct".to_string(),
    );
    
    // Test method lookup
    let method = interface_value.get_method("test_method");
    assert!(method.is_some());
    assert_eq!(method.unwrap().function_ptr, 0x1000);
    
    let method = interface_value.get_method("nonexistent_method");
    assert!(method.is_none());
    
    Ok(())
}

#[test]
fn test_interface_dispatch_error_handling() -> Result<(), CursedError> {
    // Test error handling for interface dispatch
    let mut registry = InterfaceDispatchRegistry::new();
    
    // Test registering interface with empty method name
    let interface_methods = vec![
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "".to_string(), // Empty name should cause error
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: 0,
        },
    ];
    
    let result = registry.register_interface("TestInterface".to_string(), interface_methods);
    assert!(result.is_err());
    
    // Test duplicate method registration
    let interface_methods = vec![
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "method1".to_string(),
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: 0,
        },
    ];
    
    registry.register_interface("TestInterface".to_string(), interface_methods)?;
    
    // Test duplicate method in vtable
    let mut vtable = InterfaceVTable::new(
        "TestInterface".to_string(),
        "TestStruct".to_string(),
    );
    
    let method = cursed::runtime::interface_dispatch::VTableEntry {
        method_name: "duplicate_method".to_string(),
        function_ptr: 0x1000,
        param_types: vec!["i32".to_string()],
        return_type: Some("i32".to_string()),
    };
    
    vtable.add_method(method.clone())?;
    
    // Adding the same method again should fail
    let result = vtable.add_method(method);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_global_interface_dispatch() -> Result<(), CursedError> {
    // Test global interface dispatch system
    initialize_interface_dispatch()?;
    
    // Register interface
    let interface_methods = vec![
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "global_method".to_string(),
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: 0,
        },
    ];
    
    register_global_interface("GlobalInterface".to_string(), interface_methods)?;
    
    // Register implementation
    let mut method_implementations = HashMap::new();
    method_implementations.insert("global_method".to_string(), 0x5000);
    
    register_global_implementation(
        "GlobalInterface".to_string(),
        "GlobalStruct".to_string(),
        method_implementations,
    )?;
    
    // Create interface value
    let interface_value = create_global_interface_value(
        "GlobalStruct",
        "GlobalInterface",
        0x6000, // data pointer
    )?;
    
    // Test that we can access the method through the interface value
    let method = interface_value.get_method("global_method");
    assert!(method.is_some());
    assert_eq!(method.unwrap().function_ptr, 0x5000);
    
    // Test that the interface value has correct metadata
    assert_eq!(interface_value.interface_name, "GlobalInterface");
    assert_eq!(interface_value.concrete_type, "GlobalStruct");
    assert_eq!(interface_value.data_ptr, 0x6000);
    
    Ok(())
}

#[test]
fn test_interface_compliance_checking() -> Result<(), CursedError> {
    // Test interface compliance checking
    let mut registry = InterfaceDispatchRegistry::new();
    
    // Register interface with multiple methods
    let interface_methods = vec![
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "method1".to_string(),
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: 0,
        },
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "method2".to_string(),
            param_types: vec!["str".to_string()],
            return_type: Some("str".to_string()),
            method_index: 1,
        },
    ];
    
    registry.register_interface("CompliantInterface".to_string(), interface_methods)?;
    
    // Test complete implementation (should succeed)
    let mut complete_implementation = HashMap::new();
    complete_implementation.insert("method1".to_string(), 0x1000);
    complete_implementation.insert("method2".to_string(), 0x2000);
    
    let result = registry.register_implementation(
        "CompliantInterface".to_string(),
        "CompliantStruct".to_string(),
        complete_implementation,
    );
    assert!(result.is_ok());
    
    // Test incomplete implementation (should fail)
    let mut incomplete_implementation = HashMap::new();
    incomplete_implementation.insert("method1".to_string(), 0x1000);
    // Missing method2
    
    let result = registry.register_implementation(
        "CompliantInterface".to_string(),
        "IncompliantStruct".to_string(),
        incomplete_implementation,
    );
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_interface_dispatch_performance() -> Result<(), CursedError> {
    // Test performance characteristics of interface dispatch
    initialize_interface_dispatch()?;
    
    // Create large interface with many methods
    let mut interface_methods = Vec::new();
    for i in 0..100 {
        interface_methods.push(cursed::runtime::interface_dispatch::InterfaceMethod {
            name: format!("method_{}", i),
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: i,
        });
    }
    
    register_global_interface("LargeInterface".to_string(), interface_methods)?;
    
    // Register implementation for all methods
    let mut method_implementations = HashMap::new();
    for i in 0..100 {
        method_implementations.insert(format!("method_{}", i), 0x1000 + i * 0x100);
    }
    
    register_global_implementation(
        "LargeInterface".to_string(),
        "LargeStruct".to_string(),
        method_implementations,
    )?;
    
    // Test method lookup performance
    let interface_value = create_global_interface_value(
        "LargeStruct",
        "LargeInterface",
        0x10000,
    )?;
    
    // Test various method lookups
    for i in &[0, 25, 50, 75, 99] {
        let method_name = format!("method_{}", i);
        let method = interface_value.get_method(&method_name);
        assert!(method.is_some());
        assert_eq!(method.unwrap().function_ptr, 0x1000 + i * 0x100);
    }
    
    Ok(())
}

#[test]
fn test_vtable_registry_integration() -> Result<(), CursedError> {
    // Test vtable registry integration with global registry
    initialize_interface_dispatch()?;
    
    // Register interface 
    let interface_methods = vec![
        cursed::runtime::interface_dispatch::InterfaceMethod {
            name: "registry_method".to_string(),
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: 0,
        },
    ];
    
    register_global_interface("RegistryInterface".to_string(), interface_methods)?;
    
    // Register implementation
    let mut method_implementations = HashMap::new();
    method_implementations.insert("registry_method".to_string(), 0x1000);
    
    register_global_implementation(
        "RegistryInterface".to_string(),
        "RegistryStruct".to_string(),
        method_implementations,
    )?;
    
    // Test that vtable was created correctly
    let registry = get_global_dispatch_registry()?;
    let registry_lock = registry.lock().unwrap();
    let vtable = registry_lock.get_vtable("RegistryInterface", "RegistryStruct");
    assert!(vtable.is_some());
    
    let vtable = vtable.unwrap();
    let method = vtable.get_method("registry_method");
    assert!(method.is_some());
    assert_eq!(method.unwrap().function_ptr, 0x1000);
    
    Ok(())
}
