use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use cursed::core::async_constraint_checker::{AsyncConstraintChecker, AsyncConstraintChecking};
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::sync::Arc;


#[path = "common.rs"]
mod common;

#[test]
fn test_async_constraint_checker_parallel_execution() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Add additional test implementations
    registry.register_implementation(
        Type::Struct("TestStruct".to_string(), vec![]),
        "Testable".to_string()
    );
    registry.register_implementation(
        Type::Struct("TestCase".to_string(), vec![]),
        "Testable".to_string()
    );
    registry.register_implementation(
        Type::Struct("TestSuite".to_string(), vec![]),
        "Container".to_string()
    );
    
    // Create many constraints to check
    let mut constraints = vec![];
    
    // Add 10 constraints to ensure parallelism
    constraints.push((Type::Normie, "Numeric".to_string());
    constraints.push((Type::Tea, "Comparable".to_string());
    constraints.push((Type::Lit, "Comparable".to_string());
    constraints.push((Type::Struct("TestStruct".to_string(), vec![]), "Testable".to_string());
    constraints.push((Type::Struct("TestCase".to_string(), vec![]), "Testable".to_string());
    constraints.push((Type::Struct("TestSuite".to_string(), vec![]), "Container".to_string());
    constraints.push((Type::Struct("Point".to_string(), vec![]), "Comparable".to_string());
    constraints.push((Type::Thicc, "Numeric".to_string());
    constraints.push((Type::Snack, "Numeric".to_string());
    constraints.push((Type::Meal, "Numeric".to_string());
    
    // Use the AsyncConstraintChecking trait
    let results = registry.check_constraints_parallel(constraints.clone();
    
    // Verify all results match expected outcomes
    assert_eq!(results.len(), 10);
    for i in 0..10 {
        assert!(results[i].is_ok());
    }
    
    // Check specific results
    assert_eq!(results[0], Ok(true);  // Normie implements Numeric
    assert_eq!(results[1], Ok(true);  // Tea implements Comparable
    assert_eq!(results[2], Ok(true);  // Lit implements Comparable
    assert_eq!(results[3], Ok(true);  // TestStruct implements Testable
    
    // Now use the direct AsyncConstraintChecker
    let registry_arc = Arc::new(registry);
    let checker = AsyncConstraintChecker::new(registry_arc);
    
    let results = checker.check_constraints_parallel(constraints);
    
    // Verify results again
    assert_eq!(results.len(), 10);
    for i in 0..10 {
        assert!(results[i].is_ok());
    }
}

#[test]
fn test_parallel_generic_constraint_checking() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create a generic Dictionary[K, V] where K must be Comparable
    registry.register_generic_implementation(
        "Dictionary".to_string(),
        vec!["K".to_string(), "V".to_string())],
        "Map".to_string(),
        vec![("K".to_string(), "Comparable".to_string())]
    );
    
    // Create type arguments
    let valid_args = vec![Type::Tea, Type::Normie]; // String and Int
    let type_params = vec!["K".to_string(), "V".to_string())];
    let constraints = vec![("K".to_string(), "Comparable".to_string())];
    
    // Check valid constraints in parallel
    let result = registry.check_generic_constraints_parallel(&valid_args, &type_params, &constraints);
    assert_eq!(result, Ok(true);
    
    // Now try with a type that doesn't implement Comparable
    let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
    let invalid_args = vec![non_comparable, Type::Normie];
    
    let result = registry.check_generic_constraints_parallel(&invalid_args, &type_params, &constraints);
    assert_eq!(result, Ok(false);
}

#[test]
fn test_concurrent_complex_constraint_checking() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register a complex generic type with multiple constraints
    // SortedMap[K, V] where K implements Comparable and V implements Container
    registry.register_generic_implementation(
        "SortedMap".to_string(),
        vec!["K".to_string(), "V".to_string())],
        "Map".to_string(),
        vec![
            ("K".to_string(), "Comparable".to_string(),
            ("V".to_string(), "Container".to_string()
        ]
    );
    
    // Create type params and valid args
    let type_params = vec!["K".to_string(), "V".to_string())];
    let valid_args = vec![
        Type::Tea, // String implements Comparable
        Type::Struct("StringStack".to_string(), vec![]) // StringStack implements Container
    ];
    let constraints = vec![
        ("K".to_string(), "Comparable".to_string(),
        ("V".to_string(), "Container".to_string()
    ];
    
    // Check valid constraints in parallel
    let result = registry.check_generic_constraints_parallel(&valid_args, &type_params, &constraints);
    assert_eq!(result, Ok(true);
    
    // Now try with partially invalid args
    let invalid_args = vec![
        Type::Tea, // String implements Comparable
        Type::Normie // Int doesn't implement Container
    ];
    
    let result = registry.check_generic_constraints_parallel(&invalid_args, &type_params, &constraints);
    assert_eq!(result, Ok(false);
}

#[test]
fn test_stress_parallel_checking() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create 100 constraints to check in parallel
    let mut constraints = vec![];
    
    // Add repeated constraints to simulate heavy load
    for _ in 0..20 {
        constraints.push((Type::Normie, "Numeric".to_string());
        constraints.push((Type::Tea, "Comparable".to_string());
        constraints.push((Type::Lit, "Comparable".to_string());
        constraints.push((Type::Struct("Point".to_string(), vec![]), "Comparable".to_string());
        constraints.push((Type::Thicc, "Numeric".to_string());
    }
    
    // Check them all in parallel
    let results = registry.check_constraints_parallel(constraints);
    
    // Verify results count
    assert_eq!(results.len(), 100);
    
    // All results should be Ok(true)
    for result in results {
        assert_eq!(result, Ok(true);
    }
}

#[test]
fn test_parallel_mismatched_type_params() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create a generic type with constraints
    let type_params = vec!["T".to_string(), "U".to_string())];
    let constraints = vec![("T".to_string(), "Comparable".to_string())];
    
    // Provide wrong number of type arguments
    let too_few_args = vec![Type::Tea];
    
    // Check constraints
    let result = registry.check_generic_constraints_parallel(&too_few_args, &type_params, &constraints);
    assert_eq!(result, Ok(false);
    
    // Too many arguments
    let too_many_args = vec![Type::Tea, Type::Normie, Type::Lit];
    
    let result = registry.check_generic_constraints_parallel(&too_many_args, &type_params, &constraints);
    assert_eq!(result, Ok(false);
}