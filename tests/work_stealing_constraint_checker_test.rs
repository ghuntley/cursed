use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use cursed::core::async_constraint_checker::{AsyncConstraintChecker, AsyncConstraintChecking};
use cursed::core::async_constraint_checker_work_stealing::{WorkStealingConstraintChecker, WorkStealingConstraintChecking};
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::sync::Arc;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;


#[path = "common.rs"]
mod common;

#[test]
fn test_work_stealing_vs_standard_performance() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register additional interface implementations for testing
    for i in 0..20 {
        let struct_name = format!("TestStruct{}", i);
        registry.register_implementation(
            Type::Struct(struct_name.clone(), vec![]),
            "Testable".to_string()
        );
        
        if i % 2 == 0 {
            registry.register_implementation(
                Type::Struct(struct_name.clone(), vec![]),
                "Container".to_string()
            );
        }
        
        if i % 3 == 0 {
            registry.register_implementation(
                Type::Struct(struct_name.clone(), vec![]),
                "Comparable".to_string()
            );
        }
    }
    
    // Create a more complex set of constraints
    let mut constraints = vec![];
    
    // Add many constraints to ensure parallelism is needed
    for i in 0..20 {
        let struct_name = format!("TestStruct{}", i);
        constraints.push((Type::Struct(struct_name.clone(), vec![]), "Testable".to_string());
        constraints.push((Type::Normie, "Numeric".to_string());
        constraints.push((Type::Tea, "Comparable".to_string());
        
        if i % 2 == 0 {
            constraints.push((Type::Struct(struct_name.clone(), vec![]), "Container".to_string());
        }
        
        if i % 3 == 0 {
            constraints.push((Type::Struct(struct_name.clone(), vec![]), "Comparable".to_string());
        }
    }
    
    // Add duplicates to test caching
    for i in 0..10 {
        constraints.push((Type::Normie, "Numeric".to_string());
        constraints.push((Type::Tea, "Comparable".to_string());
    }
    
    // Test baseline: Use standard async constraint checker
    let registry_arc = Arc::new(registry.clone();
    let standard_checker = AsyncConstraintChecker::new(Arc::clone(&registry_arc);
    
    let standard_start = std::time::Instant::now();
    let standard_results = standard_checker.check_constraints_parallel(constraints.clone();
    let standard_duration = standard_start.elapsed();
    
    println!("Standard checker processed {} constraints in {:?}", standard_results.len(), standard_duration);
    
    // Test work stealing implementation
    let work_stealing_checker = WorkStealingConstraintChecker::new(Arc::clone(&registry_arc);
    
    let ws_start = std::time::Instant::now();
    let ws_results = work_stealing_checker.check_constraints_parallel(constraints.clone();
    let ws_duration = ws_start.elapsed();
    
    println!("Work stealing checker processed {} constraints in {:?}", ws_results.len(), ws_duration);
    
    // Get and display work stealing statistics
    let stats = work_stealing_checker.get_stats();
    println!("Work stealing stats:");
    println!("  - Total steals: {}", stats.total_steals);
    println!("  - Max steals by a worker: {}", stats.max_steals);
    println!("  - Average steals per worker: {:.2}", stats.avg_steals_per_worker);
    println!("  - Workload imbalance: {:.2}", stats.workload_imbalance);
    println!("  - Cache hit rate: {:.2}", stats.cache_hit_rate);
    
    // Verify both implementations produce the same results
    assert_eq!(standard_results.len(), ws_results.len());
    for i in 0..standard_results.len() {
        assert_eq!(standard_results[i], ws_results[i]);
    }
    
    // Test the extension trait implementation
    let extension_start = std::time::Instant::now();
    let extension_results = registry.check_constraints_work_stealing(constraints.clone();
    let extension_duration = extension_start.elapsed();
    
    println!("Extension trait processed {} constraints in {:?}", extension_results.len(), extension_duration);
    
    // Verify extension trait gives the same results
    assert_eq!(standard_results.len(), extension_results.len());
    for i in 0..standard_results.len() {
        assert_eq!(standard_results[i], extension_results[i]);
    }
}

#[test]
fn test_work_stealing_unbalanced_workload() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Create an unbalanced workload with some "slow" constraints
    struct SlowChecker {
        registry: InterfaceRegistry,
    }
    
    impl SlowChecker {
        fn new(registry: InterfaceRegistry) -> Self {
            Self { registry }
        }
        
        fn check_implementation(&self, type_: &Type, interface_name: &str) -> Result<bool, Error> {
            // Simulate slow checks for certain types
            if interface_name == "SlowInterface" {
                // Simulate a long-running check
                std::thread::sleep(Duration::from_millis(50);
                Ok(true)
            } else {
                // Normal speed check
                self.registry.check_implementation(type_, interface_name)
            }
        }
    }
    
    // Register the slow interface implementations
    for i in 0..5 {
        let struct_name = format!("SlowStruct{}", i);
        registry.register_implementation(
            Type::Struct(struct_name.clone(), vec![]),
            "SlowInterface".to_string()
        );
    }
    
    // Create a mixed workload with fast and slow checks
    let mut constraints = vec![];
    
    // Add many fast checks
    for _ in 0..50 {
        constraints.push((Type::Normie, "Numeric".to_string());
        constraints.push((Type::Tea, "Comparable".to_string());
        constraints.push((Type::Lit, "Comparable".to_string());
    }
    
    // Add a few slow checks
    for i in 0..5 {
        let struct_name = format!("SlowStruct{}", i);
        constraints.push((Type::Struct(struct_name.clone(), vec![]), "SlowInterface".to_string());
    }
    
    // Randomize the order of constraints
    constraints.shuffle(&mut thread_rng();
    
    let registry_arc = Arc::new(registry);
    
    // Test standard vs work stealing implementation
    let standard_checker = AsyncConstraintChecker::new(Arc::clone(&registry_arc);
    let work_stealing_checker = WorkStealingConstraintChecker::new(Arc::clone(&registry_arc);
    
    // Test standard implementation
    println!("Testing standard implementation with unbalanced workload...");
    let standard_start = std::time::Instant::now();
    let _ = standard_checker.check_constraints_parallel(constraints.clone();
    let standard_duration = standard_start.elapsed();
    
    // Test work stealing implementation
    println!("Testing work stealing implementation with unbalanced workload...");
    let ws_start = std::time::Instant::now();
    let _ = work_stealing_checker.check_constraints_parallel(constraints.clone();
    let ws_duration = ws_start.elapsed();
    
    println!("Standard implementation took: {:?}", standard_duration);
    println!("Work stealing implementation took: {:?}", ws_duration);
    
    // Get work stealing statistics
    let stats = work_stealing_checker.get_stats();
    println!("Work stealing stats for unbalanced workload:");
    println!("  - Total steals: {}", stats.total_steals);
    println!("  - Max steals by a worker: {}", stats.max_steals);
    println!("  - Average steals per worker: {:.2}", stats.avg_steals_per_worker);
    println!("  - Workload imbalance: {:.2}", stats.workload_imbalance);
    println!("  - Tasks per worker: {:?}", stats.tasks_per_worker);
}

#[test]
fn test_generic_constraint_checking_with_work_stealing() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Register generic types with constraints
    registry.register_generic_implementation(
        "Collection".to_string(),
        vec!["T".to_string())],
        "Container".to_string(),
        vec![("T".to_string(), "Comparable".to_string())]
    );
    
    registry.register_generic_implementation(
        "Map".to_string(),
        vec!["K".to_string(), "V".to_string())],
        "Container".to_string(),
        vec![
            ("K".to_string(), "Comparable".to_string(),
            ("V".to_string(), "Serializable".to_string()
        ]
    );
    
    registry.register_generic_implementation(
        "ComplexStructure".to_string(),
        vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string())],
        "Serializable".to_string(),
        vec![
            ("A".to_string(), "Comparable".to_string(),
            ("B".to_string(), "Container".to_string(),
            ("C".to_string(), "Numeric".to_string(),
            ("D".to_string(), "Serializable".to_string()
        ]
    );
    
    // Register the Serializable interface for testing
    registry.register_implementation(Type::Tea, "Serializable".to_string());
    registry.register_implementation(Type::Lit, "Serializable".to_string());
    registry.register_implementation(Type::Normie, "Serializable".to_string());
    
    // Test with Collection<String>
    let collection_type_args = vec![Type::Tea]; // String implements Comparable
    let collection_type_params = vec!["T".to_string())];
    let collection_constraints = vec![("T".to_string(), "Comparable".to_string())];
    
    // Test standard implementation
    let registry_arc = Arc::new(registry.clone();
    let standard_checker = AsyncConstraintChecker::new(Arc::clone(&registry_arc);
    
    let standard_result = standard_checker.check_generic_constraints_parallel(
        &collection_type_args,
        &collection_type_params,
        &collection_constraints
    );
    
    // Test work stealing implementation
    let work_stealing_checker = WorkStealingConstraintChecker::new(Arc::clone(&registry_arc);
    
    let ws_result = work_stealing_checker.check_generic_constraints_parallel(
        &collection_type_args,
        &collection_type_params,
        &collection_constraints
    );
    
    // Both should succeed
    assert_eq!(standard_result, Ok(true);
    assert_eq!(ws_result, Ok(true);
    
    // Test with Map<String, Int>
    let map_type_args = vec![Type::Tea, Type::Normie]; // String implements Comparable, Int implements Serializable
    let map_type_params = vec!["K".to_string(), "V".to_string())];
    let map_constraints = vec![
        ("K".to_string(), "Comparable".to_string(),
        ("V".to_string(), "Serializable".to_string()
    ];
    
    // Test extension trait with work stealing
    let ws_extension_result = registry.check_generic_constraints_work_stealing(
        &map_type_args,
        &map_type_params,
        &map_constraints
    );
    
    assert_eq!(ws_extension_result, Ok(true);
    
    // Test with ComplexStructure<String, Map<String, Int>, Int, String>
    let complex_type_args = vec![
        Type::Tea, // A: String implements Comparable
        Type::Struct("Map".to_string(), vec![Type::Tea, Type::Normie]), // B: Map<String, Int> implements Container
        Type::Normie, // C: Int implements Numeric
        Type::Tea, // D: String implements Serializable
    ];
    let complex_type_params = vec![
        "A".to_string(), 
        "B".to_string(), 
        "C".to_string(), 
        "D".to_string()
    ];
    let complex_constraints = vec![
        ("A".to_string(), "Comparable".to_string(),
        ("B".to_string(), "Container".to_string(),
        ("C".to_string(), "Numeric".to_string(),
        ("D".to_string(), "Serializable".to_string()
    ];
    
    // Test with the more complex structure
    let complex_result = work_stealing_checker.check_generic_constraints_parallel(
        &complex_type_args,
        &complex_type_params,
        &complex_constraints
    );
    
    assert_eq!(complex_result, Ok(true);
    
    // Test with invalid type arguments
    let invalid_type_args = vec![
        Type::Normie, // Int does not implement Comparable
        Type::Struct("Map".to_string(), vec![Type::Tea, Type::Normie]),
        Type::Normie,
        Type::Tea,
    ];
    
    let invalid_result = work_stealing_checker.check_generic_constraints_parallel(
        &invalid_type_args,
        &complex_type_params,
        &complex_constraints
    );
    
    assert_eq!(invalid_result, Ok(false);
}