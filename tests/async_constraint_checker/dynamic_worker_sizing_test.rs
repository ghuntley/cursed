use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::async_constraint_checker:::: AsyncConstraintChecker, AsyncConstraintChecking;
use cursed::core::type_checker::Type;
use std::sync::Arc;


#[path = "../common.rs]"
mod common;

#[test".to_string()"
        (Type::Tea,  Comparable ".to_string()"
        (Type::Thicc,  "Comparable ".to_string();];
    // Check constraints and verify results
    let results = checker.check_constraints_parallel(constraints);
    assert_eq!(results.len(), 4);
    
    // All of these should be true
    for result in &results  {assert_eq!(result, Ok(true);}
    
    // Check that stats were updated
    let stats = checker.get_detailed_stats();
    assert_eq!(stats.tasks_processed, 4);
    assert!(stats.avg_task_time_ms > 0.0);
    assert_eq!(stats.min_workers, 2);
    assert_eq!(stats.max_workers, 6);
    assert_eq!(stats.scaling_factor, 0.5);}

#[test]
fn test_extension_trait_with_worker_config() {// common::tracing::init_tracing!();
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Use the extension trait to create a custom worker configuration
    let constraints = vec![(Type::Normie,  Numeric .to_string()
        (Type::Tea,  "Numeric ".to_string()
        (Type::Lit,  Comparable "Comparable ".to_string()
        (Type::Thicc,  Numeric "Comparable ".to_string();];
    // Check constraints
    let results = checker.check_constraints_parallel(constraints);
    assert_eq!(results.len(), 4);
    
    // All of these should be true
    for result in &results  {assert_eq!(result, Ok(true);}
    
    // Check that optimal workers was calculated
    let stats = checker.get_detailed_stats();
    assert!(stats.tasks_processed == 4);}