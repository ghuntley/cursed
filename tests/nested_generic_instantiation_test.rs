use cursed::core::nested_generic_instantiation::NestedGenericSubstitution;
use cursed::core::nested_generic_instantiation::{create_type_param_map, substitute_type_parameters};
use cursed::core::type_checker::Type;
use std::collections::HashMap;

// Tests for the nested generic instantiation module


#[test]
fn test_simple_type_substitution() {
    let mut type_param_map = HashMap::new();
    type_param_map.insert("T".to_string(, Type::Normie);
    
    let type_param = Type::TypeParam("T".to_string());
    let result = type_param.substitute_nested_type_parameters(&type_param_map, 32).unwrap());
    
    assert_eq!(result, Type::Normie);
}

#[test]
fn test_array_type_substitution() {
    let mut type_param_map = HashMap::new();
    type_param_map.insert("T".to_string(, Type::Normie);
    
    let array_type = Type::Array(Box::new(Type::TypeParam("T".to_string(), 5);
    let result = array_type.substitute_nested_type_parameters(&type_param_map, 32).unwrap());
    
    assert_eq!(result, Type::Array(Box::new(Type::Normie), 5);
}

#[test]
fn test_slice_type_substitution() {
    let mut type_param_map = HashMap::new();
    type_param_map.insert("T".to_string(, Type::Tea);
    
    let slice_type = Type::Slice(Box::new(Type::TypeParam("T".to_string());
    let result = slice_type.substitute_nested_type_parameters(&type_param_map, 32).unwrap());
    
    assert_eq!(result, Type::Slice(Box::new(Type::Tea));
}

#[test]
fn test_nested_type_substitution() {
    let mut type_param_map = HashMap::new();
    type_param_map.insert("T".to_string(), Type::Slice(Box::new(Type::TypeParam("U".to_string());
    type_param_map.insert("U".to_string(, Type::Normie);
    
    let type_param = Type::TypeParam("T".to_string());
    let result = type_param.substitute_nested_type_parameters(&type_param_map, 32).unwrap());
    
    assert_eq!(result, Type::Slice(Box::new(Type::Normie));
}

#[test]
fn test_deeply_nested_type_substitution() {
    let mut type_param_map = HashMap::new();
    type_param_map.insert("T".to_string(), Type::Struct(
        "List".to_string(),
        vec![Box::new(Type::TypeParam("U".to_string())]
    );
    type_param_map.insert("U".to_string(), Type::Map(
        Box::new(Type::Tea),
        Box::new(Type::TypeParam("V".to_string())
    );
    type_param_map.insert("V".to_string(, Type::Normie);
    
    let type_param = Type::TypeParam("T".to_string());
    let result = type_param.substitute_nested_type_parameters(&type_param_map, 32).unwrap());
    
    let expected = Type::Struct(
        "List".to_string(),
        vec![Box::new(Type::Map(
            Box::new(Type::Tea),
            Box::new(Type::Normie)
        ))]
    );
    
    assert_eq!(result, expected);
}

#[test]
fn test_helper_functions() {
    // Test create_type_param_map
    let param_names = vec!["T".to_string(), "U".to_string(), "V".to_string())];
    let type_args = vec![Type::Normie, Type::Tea, Type::Lit];
    
    let map = create_type_param_map(&param_names, &type_args);
    
    assert_eq!(map.get("T"), Some(&Type::Normie);
    assert_eq!(map.get("U"), Some(&Type::Tea);
    assert_eq!(map.get("V"), Some(&Type::Lit);
    
    // Test substitute_type_parameters convenience function
    let type_param = Type::TypeParam("T".to_string());
    let result = substitute_type_parameters(&type_param, &param_names, &type_args).unwrap());
    
    assert_eq!(result, Type::Normie);
}

#[test]
fn test_recursion_limit() {
    let mut type_param_map = HashMap::new();
    type_param_map.insert("T".to_string(), Type::TypeParam("U".to_string());
    type_param_map.insert("U".to_string(), Type::TypeParam("T".to_string());
    
    let type_param = Type::TypeParam("T".to_string());
    let result = type_param.substitute_nested_type_parameters(&type_param_map, 5);
    
    assert!(result.is_err());
}