use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::deep_nested_constraint_checker::DeepNestedConstraintChecking;
use cursed::core::type_checker::Type;
use std::collections::HashMap;

// Tests for deep nested constraint checking
// 
// This test verifies that the constraint checker can handle deeply nested generic types
// with multiple constraints.


#[path = "common/mod.rs"]
mod common;

#[test]
fn test_simple_constraint_checking() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register a Map[K, V] generic type with K: Comparable constraint
    let map_constraints = vec![("K".to_string(), "Comparable".to_string())];
    registry.register_generic_implementation(
        "Map".to_string(),
        vec!["K".to_string(), "V".to_string()],
        "Container".to_string(),
        map_constraints
    );
    
    // Create a constraint map
    let mut constraint_map = HashMap::new();
    constraint_map.insert("K".to_string(), vec!["Comparable".to_string()]);
    
    // Check with valid arguments
    let type_args = vec![Type::Tea, Type::Normie]; // String and Int
    let result = registry.check_nested_generic_constraints(
        "Map",
        &type_args,
        &constraint_map
    );
    
    assert_eq!(result, Ok(true));
    
    // Check with invalid arguments
    let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
    let type_args = vec![non_comparable, Type::Normie];
    let result = registry.check_nested_generic_constraints(
        "Map",
        &type_args,
        &constraint_map
    );
    
    assert_eq!(result, Ok(false));
}

#[test]
fn test_nested_generic_constraint_checking() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register nested generics: Map[K, List[V]] with K: Comparable, V: Container
    registry.register_generic_implementation(
        "Map".to_string(),
        vec!["K".to_string(), "V".to_string()],
        "Container".to_string(),
        vec![("K".to_string(), "Comparable".to_string())]
    );
    
    registry.register_generic_implementation(
        "List".to_string(),
        vec!["T".to_string()],
        "Container".to_string(),
        vec![("T".to_string(), "Container".to_string())]
    );
    
    // Create a constraint map
    let mut constraint_map = HashMap::new();
    constraint_map.insert("K".to_string(), vec!["Comparable".to_string()]);
    constraint_map.insert("T".to_string(), vec!["Container".to_string()]);
    
    // Create a List[V] as the second argument
    let list_type = Type::Generic(
        "List".to_string(),
        vec![Type::Struct("Array".to_string(), vec![])] // Array implements Container
    );
    
    // Check with valid arguments
    let type_args = vec![Type::Tea, list_type]; // String and List[Array]
    let result = registry.check_nested_generic_constraints(
        "Map",
        &type_args,
        &constraint_map
    );
    
    assert_eq!(result, Ok(true));
    
    // Check with invalid nested argument
    let bad_list_type = Type::Generic(
        "List".to_string(),
        vec![Type::Lit] // Lit doesn't implement Container
    );
    
    let type_args = vec![Type::Tea, bad_list_type];
    let result = registry.check_nested_generic_constraints_with_details(
        "Map",
        &type_args,
        &constraint_map
    );
    
    // Verify the result correctly identifies the nested failure
    assert!(result.is_ok());
    let details = result.unwrap();
    assert_eq!(details.satisfied, false);
    
    // The failure path should include the nested List generic
    if let Some(path) = details.failure_path {
        let path_str = path.format();
        assert!(path_str.contains("List"));
        assert!(path_str.contains("Lit"));
    } else {
        panic!("Expected failure path");
    }
}

#[test]
fn test_multiple_constraints_per_type_parameter() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register Collection[T] with T: Container + Comparable (multiple constraints)
    registry.register_generic_implementation(
        "Collection".to_string(),
        vec!["T".to_string()],
        "Container".to_string(),
        vec![("T".to_string(), "Container".to_string()), ("T".to_string(), "Comparable".to_string())]
    );
    
    // Create a constraint map with multiple constraints per parameter
    let mut constraint_map = HashMap::new();
    constraint_map.insert("T".to_string(), vec!["Container".to_string(), "Comparable".to_string()]);
    
    // Create valid type that implements both constraints
    let array_type = Type::Struct("Array".to_string(), vec![]); // Implements both Container and Comparable
    
    // Check with valid arguments
    let type_args = vec![array_type];
    let result = registry.check_nested_generic_constraints(
        "Collection",
        &type_args,
        &constraint_map
    );
    
    assert_eq!(result, Ok(true));
    
    // Create invalid type that only implements one constraint
    let set_type = Type::Struct("CustomSet".to_string(), vec![]); // Only implements Container
    registry.register_implementation(set_type.clone(), "Container".to_string());
    
    // Check with partially valid arguments
    let type_args = vec![set_type];
    let result = registry.check_nested_generic_constraints(
        "Collection",
        &type_args,
        &constraint_map
    );
    
    assert_eq!(result, Ok(false));
}

#[test]
fn test_deeply_recursive_constraint_checking() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register multiple levels of nested generics
    // Map[K, Tree[T, Map[T, List[V]]]] with multiple constraints
    
    // First register the base generics
    registry.register_generic_implementation(
        "Map".to_string(),
        vec!["K".to_string(), "V".to_string()],
        "Container".to_string(),
        vec![("K".to_string(), "Comparable".to_string())]
    );
    
    registry.register_generic_implementation(
        "List".to_string(),
        vec!["T".to_string()],
        "Container".to_string(),
        vec![("T".to_string(), "Container".to_string())]
    );
    
    registry.register_generic_implementation(
        "Tree".to_string(),
        vec!["K".to_string(), "V".to_string()],
        "Container".to_string(),
        vec![("K".to_string(), "Comparable".to_string()), ("V".to_string(), "Container".to_string())]
    );
    
    // Create a constraint map
    let mut constraint_map = HashMap::new();
    constraint_map.insert("K".to_string(), vec!["Comparable".to_string()]);
    constraint_map.insert("T".to_string(), vec!["Comparable".to_string(), "Container".to_string()]);
    constraint_map.insert("V".to_string(), vec!["Container".to_string()]);
    
    // Now build a complex nested structure that should be valid
    // List[Array]
    let list_type = Type::Generic(
        "List".to_string(),
        vec![Type::Struct("Array".to_string(), vec![])]
    );
    
    // Inner Map[String, List[Array]]
    let inner_map = Type::Generic(
        "Map".to_string(),
        vec![Type::Tea, list_type]
    );
    
    // Tree[String, Map[String, List[Array]]]
    let tree_type = Type::Generic(
        "Tree".to_string(),
        vec![Type::Tea, inner_map]
    );
    
    // Outer Map[String, Tree[String, Map[String, List[Array]]]]
    let outer_map_args = vec![Type::Tea, tree_type];
    
    // Check the complex structure
    let result = registry.check_nested_generic_constraints(
        "Map",
        &outer_map_args,
        &constraint_map
    );
    
    assert_eq!(result, Ok(true));
    
    // Now introduce an invalid constraint deep in the structure
    // Replace Array with a type that doesn't implement Container
    let bad_list_type = Type::Generic(
        "List".to_string(),
        vec![Type::Lit] // Lit doesn't implement Container
    );
    
    let bad_inner_map = Type::Generic(
        "Map".to_string(),
        vec![Type::Tea, bad_list_type]
    );
    
    let bad_tree_type = Type::Generic(
        "Tree".to_string(),
        vec![Type::Tea, bad_inner_map]
    );
    
    let bad_outer_map_args = vec![Type::Tea, bad_tree_type];
    
    // Check the invalid structure with detailed results
    let result = registry.check_nested_generic_constraints_with_details(
        "Map",
        &bad_outer_map_args,
        &constraint_map
    );
    
    // Verify the result correctly identifies the deeply nested failure
    assert!(result.is_ok());
    let details = result.unwrap();
    assert_eq!(details.satisfied, false);
    
    // Check that we have the correct failure path that points to the exact issue
    if let Some(path) = details.failure_path {
        let path_str = path.format();
        println!("Failure path: {}", path_str);
        assert!(path_str.contains("List"));
        assert!(path_str.contains("Lit"));
        assert!(path_str.contains("Container"));
    } else {
        panic!("Expected failure path");
    }
}