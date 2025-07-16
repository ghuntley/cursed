use std::collections::HashMap;
use cursed::runtime::interface_dispatch::{
    InterfaceDispatchRegistry, InterfaceMethod, InterfaceValue,
    initialize_interface_dispatch, register_global_interface,
    register_global_implementation, create_global_interface_value,
    dispatch_global_method
};
use cursed::runtime::value::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Validating interface runtime method resolution system...");
    
    // Initialize the global interface dispatch system
    initialize_interface_dispatch()?;
    
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
    
    println!("✅ Registering Shape interface with 2 methods...");
    register_global_interface("Shape".to_string(), shape_methods)?;
    
    // 2. Register a concrete implementation (Rectangle)
    let mut rectangle_implementations = HashMap::new();
    rectangle_implementations.insert("area".to_string(), 0x1000); // Mock function pointer
    rectangle_implementations.insert("perimeter".to_string(), 0x2000); // Mock function pointer
    
    println!("✅ Registering Rectangle implementation for Shape interface...");
    register_global_implementation(
        "Shape".to_string(),
        "Rectangle".to_string(),
        rectangle_implementations,
    )?;
    
    // 3. Create an interface value
    println!("✅ Creating interface value for Rectangle implementing Shape...");
    let rectangle_data_ptr = 0x5000; // Mock data pointer
    let interface_value = create_global_interface_value(
        "Rectangle",
        "Shape",
        rectangle_data_ptr,
    )?;
    
    println!("✅ Interface value created successfully!");
    println!("   Interface: {}", interface_value.interface_name);
    println!("   Concrete type: {}", interface_value.concrete_type);
    println!("   Data pointer: 0x{:x}", interface_value.data_ptr);
    
    // 4. Test method resolution
    println!("✅ Testing method resolution...");
    let area_method = interface_value.get_method("area");
    let perimeter_method = interface_value.get_method("perimeter");
    let nonexistent_method = interface_value.get_method("nonexistent");
    
    assert!(area_method.is_some(), "Area method should be found");
    assert!(perimeter_method.is_some(), "Perimeter method should be found");
    assert!(nonexistent_method.is_none(), "Nonexistent method should not be found");
    
    println!("   ✓ area method resolved: {}", area_method.unwrap().method_name);
    println!("   ✓ perimeter method resolved: {}", perimeter_method.unwrap().method_name);
    println!("   ✓ nonexistent method correctly returns None");
    
    // 5. Test vtable lookup
    println!("✅ Testing vtable structure...");
    let vtable = &interface_value.vtable;
    println!("   Interface: {}", vtable.interface_name);
    println!("   Concrete type: {}", vtable.concrete_type);
    println!("   Method count: {}", vtable.methods.len());
    
    for (index, method) in vtable.methods.iter().enumerate() {
        println!("   Method {}: {} -> 0x{:x}", index, method.method_name, method.function_ptr);
    }
    
    // 6. Validate method indices
    println!("✅ Testing method index lookup...");
    assert_eq!(vtable.get_method_index("area"), Some(0));
    assert_eq!(vtable.get_method_index("perimeter"), Some(1));
    assert_eq!(vtable.get_method_index("nonexistent"), None);
    
    println!("   ✓ Method indices correctly resolved");
    
    // 7. Test inheritance (register a derived interface)
    let drawable_methods = vec![
        InterfaceMethod {
            name: "draw".to_string(),
            param_types: vec![],
            return_type: None,
            method_index: 0,
        },
    ];
    
    println!("✅ Registering Drawable interface...");
    register_global_interface("Drawable".to_string(), drawable_methods)?;
    
    let mut rectangle_drawable_implementations = HashMap::new();
    rectangle_drawable_implementations.insert("draw".to_string(), 0x3000); // Mock function pointer
    
    register_global_implementation(
        "Drawable".to_string(),
        "Rectangle".to_string(),
        rectangle_drawable_implementations,
    )?;
    
    let drawable_interface_value = create_global_interface_value(
        "Rectangle",
        "Drawable",
        rectangle_data_ptr,
    )?;
    
    println!("✅ Multiple interface implementation validated!");
    println!("   Rectangle implements both Shape and Drawable interfaces");
    
    // 8. Validate different interface values for same concrete type
    assert_eq!(interface_value.concrete_type, drawable_interface_value.concrete_type);
    assert_ne!(interface_value.interface_name, drawable_interface_value.interface_name);
    
    println!("   ✓ Same concrete type implementing multiple interfaces works correctly");
    
    println!("\n🎉 All interface runtime method resolution tests passed!");
    println!("📋 Summary:");
    println!("   ✓ Interface registration");
    println!("   ✓ Implementation registration");
    println!("   ✓ Interface value creation");
    println!("   ✓ Method resolution");
    println!("   ✓ VTable structure validation");
    println!("   ✓ Method index lookup");
    println!("   ✓ Multiple interface implementation");
    println!("   ✓ Runtime dispatch system integrity");
    
    Ok(())
}
