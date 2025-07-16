use std::collections::HashMap;
use cursed::runtime::interface_dispatch::{
    InterfaceDispatchRegistry, InterfaceMethod, InterfaceValue,
    initialize_interface_dispatch, register_global_interface,
    register_global_implementation, create_global_interface_value,
};

#[test]
fn test_interface_runtime_method_resolution() {
    // Initialize the global interface dispatch system
    assert!(initialize_interface_dispatch().is_ok());
    
    // 1. Register a simple interface
    let shape_methods = vec![
        InterfaceMethod {
            name: "area".to_string(),
            param_types: vec![],
            return_type: Some("f64".to_string()),
            method_index: 0,
        },
        InterfaceMethod {
            name: "perimeter".to_string(),
            param_types: vec![],
            return_type: Some("f64".to_string()),
            method_index: 1,
        },
    ];
    
    assert!(register_global_interface("Shape".to_string(), shape_methods).is_ok());
    
    // 2. Register a concrete implementation (Rectangle)
    let mut rectangle_implementations = HashMap::new();
    rectangle_implementations.insert("area".to_string(), 0x1000); // Mock function pointer
    rectangle_implementations.insert("perimeter".to_string(), 0x2000); // Mock function pointer
    
    assert!(register_global_implementation(
        "Shape".to_string(),
        "Rectangle".to_string(),
        rectangle_implementations,
    ).is_ok());
    
    // 3. Create an interface value
    let rectangle_data_ptr = 0x5000; // Mock data pointer
    let interface_value = create_global_interface_value(
        "Rectangle",
        "Shape",
        rectangle_data_ptr,
    ).expect("Should create interface value");
    
    // 4. Test method resolution
    let area_method = interface_value.get_method("area");
    let perimeter_method = interface_value.get_method("perimeter");
    let nonexistent_method = interface_value.get_method("nonexistent");
    
    assert!(area_method.is_some(), "Area method should be found");
    assert!(perimeter_method.is_some(), "Perimeter method should be found");
    assert!(nonexistent_method.is_none(), "Nonexistent method should not be found");
    
    // 5. Test vtable structure
    let vtable = &interface_value.vtable;
    assert_eq!(vtable.interface_name, "Shape");
    assert_eq!(vtable.concrete_type, "Rectangle");
    assert_eq!(vtable.methods.len(), 2);
    
    // 6. Validate method indices
    assert_eq!(vtable.get_method_index("area"), Some(0));
    assert_eq!(vtable.get_method_index("perimeter"), Some(1));
    assert_eq!(vtable.get_method_index("nonexistent"), None);
}

#[test]
fn test_multiple_interface_implementation() {
    // This test should run after interface_runtime_method_resolution or reinitialize
    let _ = initialize_interface_dispatch();
    
    // Register Shape interface if not already registered
    let shape_methods = vec![
        InterfaceMethod {
            name: "area".to_string(),
            param_types: vec![],
            return_type: Some("f64".to_string()),
            method_index: 0,
        },
    ];
    let _ = register_global_interface("Shape2".to_string(), shape_methods);
    
    // Register Drawable interface
    let drawable_methods = vec![
        InterfaceMethod {
            name: "draw".to_string(),
            param_types: vec![],
            return_type: None,
            method_index: 0,
        },
    ];
    
    assert!(register_global_interface("Drawable".to_string(), drawable_methods).is_ok());
    
    // Register implementations
    let mut shape_implementations = HashMap::new();
    shape_implementations.insert("area".to_string(), 0x1000);
    
    let mut drawable_implementations = HashMap::new();
    drawable_implementations.insert("draw".to_string(), 0x3000);
    
    assert!(register_global_implementation(
        "Shape2".to_string(),
        "Rectangle2".to_string(),
        shape_implementations,
    ).is_ok());
    
    assert!(register_global_implementation(
        "Drawable".to_string(),
        "Rectangle2".to_string(),
        drawable_implementations,
    ).is_ok());
    
    // Create interface values
    let data_ptr = 0x5000;
    let shape_interface = create_global_interface_value("Rectangle2", "Shape2", data_ptr);
    let drawable_interface = create_global_interface_value("Rectangle2", "Drawable", data_ptr);
    
    assert!(shape_interface.is_ok());
    assert!(drawable_interface.is_ok());
    
    let shape_val = shape_interface.unwrap();
    let drawable_val = drawable_interface.unwrap();
    
    // Validate both interfaces for same concrete type
    assert_eq!(shape_val.concrete_type, drawable_val.concrete_type);
    assert_ne!(shape_val.interface_name, drawable_val.interface_name);
    
    // Test method resolution for each interface
    assert!(shape_val.get_method("area").is_some());
    assert!(shape_val.get_method("draw").is_none());
    
    assert!(drawable_val.get_method("draw").is_some());
    assert!(drawable_val.get_method("area").is_none());
}

#[test]
fn test_interface_dispatch_registry_direct() {
    let mut registry = InterfaceDispatchRegistry::new();
    
    // Test interface registration
    let methods = vec![
        InterfaceMethod {
            name: "test_method".to_string(),
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
            method_index: 0,
        }
    ];
    
    assert!(registry.register_interface("TestInterface".to_string(), methods).is_ok());
    
    // Test implementation registration
    let mut implementations = HashMap::new();
    implementations.insert("test_method".to_string(), 0x1234);
    
    assert!(registry.register_implementation(
        "TestInterface".to_string(),
        "TestType".to_string(),
        implementations
    ).is_ok());
    
    // Test interface compliance
    assert!(registry.implements_interface("TestType", "TestInterface"));
    assert!(!registry.implements_interface("NonExistentType", "TestInterface"));
    
    // Test vtable retrieval
    let vtable = registry.get_vtable("TestInterface", "TestType");
    assert!(vtable.is_some());
    
    let vtable = vtable.unwrap();
    assert_eq!(vtable.interface_name, "TestInterface");
    assert_eq!(vtable.concrete_type, "TestType");
    assert_eq!(vtable.methods.len(), 1);
    
    // Test method lookup
    assert!(vtable.get_method("test_method").is_some());
    assert!(vtable.get_method("nonexistent").is_none());
}
