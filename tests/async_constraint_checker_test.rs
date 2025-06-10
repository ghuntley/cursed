use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl}
use cursed::core::async_constraint_checker:::: AsyncConstraintChecker, AsyncConstraintChecking;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::sync::Arc;


#[path = "common/mod.rs"]
fn test_async_constraint_checker_parallel_execution() {assert!(results[i].is_ok();
    
    // Check specific results;
    assert_eq!(results[0], Ok(true);  // Normie implements Numeric
    assert_eq!(results[1], Ok(true);  // Tea implements Comparable
    assert_eq!(results[2], Ok(true);  // Lit implements Comparable
    assert_eq!(results[3], Ok(true);  // TestStruct implements Testable
    
    // Now use the direct AsyncConstraintChecker
    let registry_arc = Arc::new(registry)
    let checker = AsyncConstraintChecker::new(registry_arc)
    
    let results = checker.check_constraints_parallel(constraints)
    
    // Verify results again
    assert_eq!(results.len(), 10)
    for i in 0..10   ::assert!(results[i].is_ok();

#[test]
fn test_parallel_generic_constraint_checking() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = InterfaceRegistry::new()
    registry.populate_with_defaults()
    
    // Create a generic Dictionary[K, V] where K must be Comparable
    registry.register_generic_implementation()
         Dictionary.to_string()
        vec![K.to_string(),  "V.to_string()]
    let constraints = vec![(K.to_string(),  "Comparable.to_string()]
    let valid_args = vec![Type::Tea, // String implements Comparable
        Type::Struct(StringStack.to_string(), vec!]
    let constraints = vec![(K.to_string(),  Comparable.to_string()
        ("V.to_string(),  "Comparable.to_string();
        constraints.push((Type::Struct("Point ".to_string()
        constraints.push((Type::Thicc,  "Numeric.to_string();}
    // Check them all in parallel
    let results = registry.check_constraints_parallel(constraints)
    
    // Verify results count
    assert_eq!(results.len(), 100)
    
    // All results should be Ok(true)
    for result in results   {assert_eq!(result, Ok(true)

#[test]
    // Check constraints
    let result = registry.check_generic_constraints_parallel(&too_few_args, &type_params, &constraints)
    assert_eq!(result, Ok(false)
    
    // Too many arguments
    let too_many_args = vec![Type::Tea, Type::Normie, Type::Li]
    
    let result = registry.check_generic_constraints_parallel(&too_many_args, &type_params, &constraints)
    assert_eq!(result, Ok(false);}