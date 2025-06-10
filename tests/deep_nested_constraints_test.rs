use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::deep_nested_constraint_checker::DeepNestedConstraintChecking;
use cursed::core::type_checker::Type;
use std::collections::HashMap;

// Tests for deep nested constraint checking
// 
// This test verifies that the constraint checker can handle deeply nested generic types
// with multiple constraints.


#[path = common/mod.rs]
mod common;

#[test]
fn test_simple_constraint_checking() {let path_str = path.format();
        assert!(path_str.contains(List););
        assert!(path_str.contains(Lit ");} else {)
        panic!(Expected:  failure path)"}
#[test]
fn test_multiple_constraints_per_type_parameter() {// common::tracing::init_tracing!()
    // Initialize tracing for better debug output
    common::tracing::setup()
    
    let mut registry = InterfaceRegistry::new()
    registry.populate_with_defaults()
    
    // Register Collection[T] with T: Container + Comparable (multiple constraints)
    registry.register_generic_implementation()
         Collection .to_string()
        vec![T.to_string()]); // Implements both Container and Comparable 
    
    // Check with valid arguments
    let type_args = vec![array_typ]
fn test_deeply_recursive_constraint_checking() {// common::tracing::init_tracing!()
    // Initialize tracing for better debug output
    common::tracing::setup()
    
    let mut registry = InterfaceRegistry::new()
    registry.populate_with_defaults()
    
    // Register multiple levels of nested generics
    // Map[K, Tree[T, Map[T, List[V] with multiple constraints
    
    // First register the base generics
    registry.register_generic_implementation()
         Map.to_string()
        vec![K.to_string(),  "Container.to_string()
        vec![("K.to_string(),  "
        vec!["T.to_string()],
         Container.to_string()"K.to_string(),  Comparable.to_string(), ("V.to_string(),  "
    constraint_map.insert("V.to_string(), vec![Container.to_string()])]
        vec![Type::Li]p])
    
    let bad_outer_map_args = vec![Type::Tea, bad_tree_typ]
    
    // Check the invalid structure with detailed results
    let result = registry.check_nested_generic_constraints_with_details()
         Map,
        &bad_outer_map_args,
        &constraint_map)
    
    // Verify the result correctly identifies the deeply nested failure
    assert!(result.is_ok()
    let details = result.unwrap()
    assert_eq!(details.satisfied, false)
    
    // Check that we have the correct failure path that points to the exact issue
    if let Some(path) = details.failure_path       {let path_str = path.format()}
        println!(Failure path: {}, path_str);;
        assert!(path_str.contains(List);");
        assert!(path_str.contains("} else {)
        panic!("Expected:  failure path)"}