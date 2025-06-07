use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::async_constraint_checker::{AsyncConstraintChecker, AsyncConstraintChecking};
use cursed::core::type_checker::Type;
use std::sync::Arc;


#[path = "../common.rs"]
mod common;

#[test]
fn test_dynamic_worker_sizing() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    let registry_arc = Arc::new(registry);
    
    // Create a checker with custom worker configuration
    let checker = AsyncConstraintChecker::with_worker_config(
        registry_arc,
        2, // min_workers
        6, // max_workers
        0.5, // scaling_factor
    );
    
    // Verify the configuration was applied correctly
    assert_eq!(checker.min_workers, 2);
    assert_eq!(checker.max_workers, 6);
    assert_eq!(checker.scaling_factor, 0.5);
    
    // Create constraints to check
    let constraints = vec![
        (Type::Normie, "Numeric".to_string()),
        (Type::Tea, "Comparable".to_string()),
        (Type::Thicc, "Numeric".to_string()),
        (Type::Lit, "Comparable".to_string()),
    ];
    
    // Check constraints and verify results
    let results = checker.check_constraints_parallel(constraints);
    assert_eq!(results.len(), 4);
    
    // All of these should be true
    for result in &results {
        assert_eq!(*result, Ok(true));
    }
    
    // Check that stats were updated
    let stats = checker.get_detailed_stats();
    assert_eq!(stats.tasks_processed, 4);
    assert!(stats.avg_task_time_ms > 0.0);
    assert_eq!(stats.min_workers, 2);
    assert_eq!(stats.max_workers, 6);
    assert_eq!(stats.scaling_factor, 0.5);
}

#[test]
fn test_extension_trait_with_worker_config() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Use the extension trait to create a custom worker configuration
    let constraints = vec![
        (Type::Normie, "Numeric".to_string()),
        (Type::Tea, "Comparable".to_string()),
        (Type::Thicc, "Numeric".to_string()),
        (Type::Lit, "Comparable".to_string()),
    ];
    
    // Test the check_constraints_with_config method
    let results = registry.check_constraints_with_config(
        constraints.clone(),
        1,  // min_workers
        4,  // max_workers
        0.8 // scaling_factor
    );
    
    assert_eq!(results.len(), 4);
    for result in &results {
        assert_eq!(*result, Ok(true));
    }
    
    // Test the with_worker_config method
    let checker = registry.with_worker_config(2, 8, 0.7);
    let results = checker.check_constraints_parallel(constraints);
    
    assert_eq!(results.len(), 4);
    for result in &results {
        assert_eq!(*result, Ok(true));
    }
    
    // Get and verify stats
    let stats = checker.get_detailed_stats();
    assert_eq!(stats.min_workers, 2);
    assert_eq!(stats.max_workers, 8);
    assert_eq!(stats.scaling_factor, 0.7);
}

#[test]
fn test_cpu_detection_and_scaling() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    let registry_arc = Arc::new(registry);
    
    // Create a checker with default configuration
    let checker = AsyncConstraintChecker::new(registry_arc);
    
    // Get stats and verify CPU detection
    let stats = checker.get_detailed_stats();
    assert!(stats.available_cores > 0);
    
    // Check that max_workers is limited by available cores
    assert!(checker.max_workers <= stats.available_cores);
    
    // Create constraints to check
    let constraints = vec![
        (Type::Normie, "Numeric".to_string()),
        (Type::Tea, "Comparable".to_string()),
        (Type::Thicc, "Numeric".to_string()),
        (Type::Lit, "Comparable".to_string()),
    ];
    
    // Check constraints
    let results = checker.check_constraints_parallel(constraints);
    assert_eq!(results.len(), 4);
    
    // All of these should be true
    for result in &results {
        assert_eq!(*result, Ok(true));
    }
    
    // Check that optimal workers was calculated
    let stats = checker.get_detailed_stats();
    assert!(stats.tasks_processed == 4);
}