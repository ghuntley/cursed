//! Unit tests for CURSED lookin_glass reflection library

use cursed::stdlib::lookin_glass::{self as lookin, TypeInfo, TypeKind, FieldInfo, MethodInfo, ParameterInfo};

#[test]
fn test_reflection_initialization() {
    lookin::initialize();
    
    let stats = lookin::get_reflection_statistics();
    // After initialization, basic types should be registered
    assert!(stats.types_created > 0 || lookin::registered_types().len() > 0);
}

#[test]
fn test_builtin_type_lookup() {
    lookin::initialize();
    
    // Test looking up built-in types
    let int_type = lookin::lookup_type("i64");
    assert!(int_type.is_some());
    
    let int_info = int_type.unwrap();
    assert_eq!(int_info.name, "i64");
    assert_eq!(int_info.kind, TypeKind::Primitive);
    assert_eq!(int_info.size, 8);
    
    // Test string type
    let string_type = lookin::lookup_type("string");
    assert!(string_type.is_some());
    
    let string_info = string_type.unwrap();
    assert_eq!(string_info.name, "string");
    assert_eq!(string_info.kind, TypeKind::Primitive);
}

#[test]
fn test_custom_type_registration() {
    lookin::initialize();
    
    let field1 = FieldInfo {
        name: "id".to_string(),
        type_name: "i64".to_string(),
        offset: 0,
        size: 8,
    };
    
    let field2 = FieldInfo {
        name: "name".to_string(),
        type_name: "string".to_string(),
        offset: 8,
        size: 24,
    };
    
    let method1 = MethodInfo {
        name: "get_id".to_string(),
        return_type: "i64".to_string(),
        parameters: vec![],
        is_static: false,
    };
    
    let custom_type = TypeInfo {
        name: "User".to_string(),
        size: 32,
        alignment: 8,
        kind: TypeKind::Struct,
        fields: vec![field1, field2],
        methods: vec![method1],
    };
    
    // Register the custom type
    assert!(lookin::register_type("User".to_string(), custom_type).is_ok());
    
    // Look it up
    let retrieved_type = lookin::lookup_type("User");
    assert!(retrieved_type.is_some());
    
    let user_info = retrieved_type.unwrap();
    assert_eq!(user_info.name, "User");
    assert_eq!(user_info.kind, TypeKind::Struct);
    assert_eq!(user_info.fields.len(), 2);
    assert_eq!(user_info.methods.len(), 1);
    assert_eq!(user_info.fields[0].name, "id");
    assert_eq!(user_info.methods[0].name, "get_id");
}

#[test]
fn test_registered_types_list() {
    lookin::initialize();
    
    let types = lookin::registered_types();
    assert!(!types.is_empty());
    
    // Should include basic types
    assert!(types.contains(&"bool".to_string()));
    assert!(types.contains(&"i64".to_string()));
    assert!(types.contains(&"string".to_string()));
}

#[test]
fn test_statistics_tracking() {
    lookin::initialize();
    lookin::reset_reflection_statistics();
    
    let initial_stats = lookin::get_reflection_statistics();
    let initial_lookups = initial_stats.type_lookups;
    
    // Perform some operations
    lookin::lookup_type("i64");
    lookin::lookup_type("string");
    lookin::track_value_conversion();
    lookin::track_method_invocation();
    
    let final_stats = lookin::get_reflection_statistics();
    assert!(final_stats.type_lookups > initial_lookups);
    assert_eq!(final_stats.value_conversions, 1);
    assert_eq!(final_stats.method_invocations, 1);
}

#[test]
fn test_type_kind_variants() {
    // Test that all TypeKind variants are available
    let kinds = vec![
        TypeKind::Primitive,
        TypeKind::Struct,
        TypeKind::Enum,
        TypeKind::Array,
        TypeKind::Pointer,
        TypeKind::Function,
        TypeKind::Interface,
    ];
    
    for kind in kinds {
        // Just ensure they can be created and compared
        assert_eq!(kind, kind.clone());
    }
}

#[test]
fn test_nonexistent_type_lookup() {
    lookin::initialize();
    
    let result = lookin::lookup_type("NonexistentType");
    assert!(result.is_none());
}

#[test]
fn test_parameter_info_creation() {
    let param = ParameterInfo {
        name: "value".to_string(),
        type_name: "i32".to_string(),
    };
    
    assert_eq!(param.name, "value");
    assert_eq!(param.type_name, "i32");
}
