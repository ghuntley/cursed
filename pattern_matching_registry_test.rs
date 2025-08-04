// Test the enum variant registry implementation
use cursed::pattern_matching::{EnumVariantRegistry, EnumPattern};
use cursed::pattern_matching::Pattern;

#[test]
fn test_enum_variant_registry() {
    let mut registry = EnumVariantRegistry::new();
    
    // Register Color enum with variants
    registry.register_enum(
        "Color".to_string(),
        vec!["Red".to_string(), "Green".to_string(), "Blue".to_string(), "Custom".to_string()]
    );
    
    // Test variant index lookup
    assert_eq!(registry.get_variant_index("Color", "Red"), Some(0));
    assert_eq!(registry.get_variant_index("Color", "Green"), Some(1));
    assert_eq!(registry.get_variant_index("Color", "Blue"), Some(2));
    assert_eq!(registry.get_variant_index("Color", "Custom"), Some(3));
    
    // Test unknown variant
    assert_eq!(registry.get_variant_index("Color", "Unknown"), None);
    assert_eq!(registry.get_variant_index("UnknownEnum", "Red"), None);
}

#[test] 
fn test_multiple_enums() {
    let mut registry = EnumVariantRegistry::new();
    
    // Register multiple enums
    registry.register_enum(
        "Status".to_string(),
        vec!["Success".to_string(), "Error".to_string(), "Pending".to_string()]
    );
    
    registry.register_enum(
        "Direction".to_string(), 
        vec!["North".to_string(), "South".to_string(), "East".to_string(), "West".to_string()]
    );
    
    // Test Status enum indices
    assert_eq!(registry.get_variant_index("Status", "Success"), Some(0));
    assert_eq!(registry.get_variant_index("Status", "Error"), Some(1));
    assert_eq!(registry.get_variant_index("Status", "Pending"), Some(2));
    
    // Test Direction enum indices  
    assert_eq!(registry.get_variant_index("Direction", "North"), Some(0));
    assert_eq!(registry.get_variant_index("Direction", "South"), Some(1));
    assert_eq!(registry.get_variant_index("Direction", "East"), Some(2));
    assert_eq!(registry.get_variant_index("Direction", "West"), Some(3));
    
    // Test cross-enum queries don't work
    assert_eq!(registry.get_variant_index("Status", "North"), None);
    assert_eq!(registry.get_variant_index("Direction", "Success"), None);
}

#[test]
fn test_enum_pattern_compilation() {
    use std::collections::HashMap;
    
    let mut registry = EnumVariantRegistry::new();
    registry.register_enum(
        "Option".to_string(),
        vec!["None".to_string(), "Some".to_string()]
    );
    
    let mut ir_code = String::new();
    let mut register_counter = 1;
    let mut label_counter = 1;
    
    let mut compiler = PatternCompiler::new(
        &mut ir_code,
        &mut register_counter,
        &mut label_counter,
        &registry
    );
    
    let enum_pattern = EnumPattern {
        enum_name: "Option".to_string(),
        variant_name: "Some".to_string(),
        patterns: vec![],
    };
    
    let mut bindings = HashMap::new();
    
    // This should now work without hardcoded variant index
    let result = compiler.compile_enum_pattern(
        "%value",
        &enum_pattern,
        "success_label",
        "fail_label",
        &mut bindings
    );
    
    assert!(result.is_ok());
    
    // Check that the IR contains the correct variant index (1 for "Some")
    assert!(ir_code.contains("icmp eq i32"));
    assert!(ir_code.contains(", 1")); // Should use variant index 1, not 0
}
