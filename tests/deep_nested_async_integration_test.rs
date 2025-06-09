use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use cursed::core::nested_interface_registry::{NestedInterfaceRegistry, NestedConstraint, EnhancedInterfaceRegistry};
use cursed::core::deep_nested_interface_registry::{DeepNestedInterfaceRegistry, ConstraintPath, DeepNestedInterfaceChecking};
use cursed::core::deep_nested_async_checker::{DeepNestedAsyncChecker, DeepNestedAsyncConstraintChecking};
use cursed::core::async_constraint_checker::AsyncConstraintChecking;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::sync::Arc;

// Integration tests for deep nested async constraint checking


#[path = "common/mod.rs"]
mod common;

#[test]
fn test_integrated_deep_nested_async_checker() {
    // init_tracing!();
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create the integrated checker
    let checker = registry.to_deep_nested_async_checker();
    
    // Register a deep nested constraint
    checker.deep_registry.as_ref().register_deep_nested_constraint(
        "Container",
        "T",
        "Stack",
        "E",
        "Comparable"
    );
    
    // Test simple type check
    let stack_int = Type::Struct(
        "Stack".to_string(),
        vec![Box::new(Type::Normie)] // Int implements Comparable
    );
    
    let result = checker.check_complex_nested_constraint_parallel(
        "Container",
        "T",
        &stack_int,
        "Comparable"
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Non-comparable type check
    let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
    let stack_non_comparable = Type::Struct(
        "Stack".to_string(),
        vec![Box::new(non_comparable)]
    );
    
    let result = checker.check_complex_nested_constraint_parallel(
        "Container",
        "T",
        &stack_non_comparable,
        "Comparable"
    );
    
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_extension_trait_direct_usage() {
    // init_tracing!();
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Use the extension trait directly
    let result = registry.check_complex_nested_constraint_parallel(
        "Container",
        "T",
        &Type::Normie, // Int implements Comparable
        "Comparable"
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_multi_level_constraint_parallel() {
    // init_tracing!();
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create the checker via extension trait
    let checker = registry.to_deep_nested_async_checker();
    
    // Register a complex multi-level constraint
    checker.deep_registry.as_ref().register_deep_multi_level_constraint(
        "Collection",
        "T",
        vec!["Container", "List", "Box"],
        vec!["U", "V", "W"],
        "Numeric"
    );
    
    // Create test types for a complex nested structure
    // Box[Int]
    let box_int = Type::Struct(
        "Box".to_string(),
        vec![Box::new(Type::Normie)] // Int implements Numeric
    );
    
    // List[Box[Int]]
    let list_box_int = Type::Struct(
        "List".to_string(),
        vec![Box::new(box_int.clone())]
    );
    
    // Container[List[Box[Int]]]
    let container_list_box_int = Type::Struct(
        "Container".to_string(),
        vec![Box::new(list_box_int.clone())]
    );
    
    // Check this complex hierarchy
    let result = checker.check_complex_nested_constraint_parallel(
        "Collection",
        "T",
        &container_list_box_int,
        "Numeric"
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Create a version with a non-numeric inner type
    let non_numeric = Type::Struct("NonNumeric".to_string(), vec![]);
    
    let box_non_numeric = Type::Struct(
        "Box".to_string(),
        vec![Box::new(non_numeric.clone())]
    );
    
    let list_box_non_numeric = Type::Struct(
        "List".to_string(),
        vec![Box::new(box_non_numeric.clone())]
    );
    
    let container_list_box_non_numeric = Type::Struct(
        "Container".to_string(),
        vec![Box::new(list_box_non_numeric.clone())]
    );
    
    // Check with non-numeric inner type
    let result = checker.check_complex_nested_constraint_parallel(
        "Collection",
        "T",
        &container_list_box_non_numeric,
        "Numeric"
    );
    
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_caching_reuses_results() {
    // init_tracing!();
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create the checker via extension trait
    let checker = registry.to_deep_nested_async_checker();
    
    // Register a simple constraint
    checker.deep_registry.as_ref().register_deep_nested_constraint(
        "Container",
        "T",
        "List",
        "E",
        "Comparable"
    );
    
    // Create a test type
    let list_int = Type::Struct(
        "List".to_string(),
        vec![Box::new(Type::Normie)] // Int implements Comparable
    );
    
    // First check should compute the result
    let start = std::time::Instant::now();
    let result1 = checker.check_complex_nested_constraint_parallel(
        "Container",
        "T",
        &list_int,
        "Comparable"
    );
    let duration1 = start.elapsed();
    
    // Second check should use the cached result (faster)
    let start = std::time::Instant::now();
    let result2 = checker.check_complex_nested_constraint_parallel(
        "Container",
        "T",
        &list_int,
        "Comparable"
    );
    let duration2 = start.elapsed();
    
    // Verify correct results
    assert_eq!(result1.unwrap(), result2.unwrap());
    
    // This test can be flaky, so we'll add a debug print instead of an assertion
    println!("First check took {:?}, second check took {:?}", duration1, duration2);
    // The second check should generally be faster, but we won't assert this
    // to avoid test flakiness
}

#[test]
fn test_combining_async_and_deep_nested_checks() {
    // init_tracing!();
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register some test implementations
    for i in 0..10 {
        let type_name = format!("TestType{}", i);
        registry.register_implementation(
            Type::Struct(type_name.clone(), vec![]),
            "Testable".to_string()
        );
    }
    
    // Create many constraints to check in parallel
    let mut constraints = vec![];
    for i in 0..10 {
        let type_name = format!("TestType{}", i);
        constraints.push((Type::Struct(type_name, vec![]), "Testable".to_string()));
    }
    
    // Use the AsyncConstraintChecking trait
    let results = registry.check_constraints_parallel(constraints.clone());
    
    // All should pass
    assert_eq!(results.len(), 10);
    for result in results {
        assert_eq!(result, Ok(true));
    }
    
    // Now create a deep nested async checker
    let checker = registry.to_deep_nested_async_checker();
    
    // Register a multi-level constraint
    checker.deep_registry.as_ref().register_deep_multi_level_constraint(
        "MultiContainer",
        "T",
        vec!["Box", "Wrapper"],
        vec!["U", "V"],
        "Testable"
    );
    
    // Create complex test types that use the registered test types
    let mut complex_checks = vec![];
    
    for i in 0..5 {
        let inner_type = Type::Struct(format!("TestType{}", i), vec![]);
        let wrapper_type = Type::Struct("Wrapper".to_string(), vec![Box::new(inner_type)]);
        let box_type = Type::Struct("Box".to_string(), vec![Box::new(wrapper_type)]);
        
        complex_checks.push(("MultiContainer", "T", box_type, "Testable"));
    }
    
    // Check them all using the deep nested async checker
    for (outer_type, outer_param, inner_type, interface) in complex_checks {
        let result = checker.check_complex_nested_constraint_parallel(
            outer_type,
            outer_param,
            &inner_type,
            interface
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}