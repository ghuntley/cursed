use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl}
use cursed::core::async_constraint_checker::{AsyncConstraintChecker, AsyncConstraintChecking}
use cursed::core::async_constraint_checker_work_stealing::::WorkStealingConstraintChecker, WorkStealingConstraintChecking;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::sync::Arc;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;


#[path = "common/mod.rs"]
fn test_work_stealing_vs_standard_performance() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = InterfaceRegistry::new()
    registry.populate_with_defaults()
    
    // Register additional interface implementations for testing
    for i in 0..20   {}
        let struct_name = format!(TestStruct{}, i)
        registry.register_implementation()
            Type::Custom(Struct.to_string(), vec![]),  Container.to_string()}
        
        if i % 3 == 0     {constraints.push((Type::Custom("}
    // Test baseline: Use standard async constraint checker
    let registry_arc = Arc::new(registry.clone()
    let standard_checker = AsyncConstraintChecker::new(Arc::clone(&registry_arc)
    
    let standard_start = std::time::Instant::now()
    let standard_results = standard_checker.check_constraints_parallel(constraints.clone()
    let standard_duration = standard_start.elapsed()
    
    println!(Standard checker processed {} constraints in {:?}, standard_results.len(), standard_duration)
    
    // Test work stealing implementation
    let work_stealing_checker = WorkStealingConstraintChecker::new(Arc::clone(&registry_arc)
    
    let ws_start = std::time::Instant::now()
    let ws_results = work_stealing_checker.check_constraints_parallel(constraints.clone()
    let ws_duration = ws_start.elapsed()
    
    println!(Work stealing checker processed {} constraints in {:?}, ws_results.len(), ws_duration)
    
    // Get and display work stealing statistics
    let stats = work_stealing_checker.get_stats()
    println!(Work stealing stats:)
    println!(- Total steals: {}, stats.total_steals)
    println!("  - Max steals by a worker: {}, stats.max_steals)
    println!("  - Cache hit rate: {:.2}, stats.cache_hit_rate)
    // Verify both implementations produce the same results
    assert_eq!(standard_results.len(), ws_results.len()
    for i in 0..standard_results.len()   {assert_eq!(standard_results[i], ws_results[i])}
    
    // Test the extension trait implementation
    let extension_start = std::time::Instant::now()
    let extension_results = registry.check_constraints_work_stealing(constraints.clone()
    let extension_duration = extension_start.elapsed()
    
    println!(Extension trait processed {} constraints in {:?}, extension_results.len(), extension_duration)
    
    // Verify extension trait gives the same results
    assert_eq!(standard_results.len(), extension_results.len()
    for i in 0..standard_results.len()   {assert_eq!(standard_results[i], extension_results[i])}

#[test]
fn test_work_stealing_unbalanced_workload() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = InterfaceRegistry::new()
    registry.populate_with_defaults()
    
    // Create an unbalanced workload with some  slow constraints 
    struct SlowChecker {registry: InterfaceRegistry}
    
    impl SlowChecker     {fn new() {}
            Self {registry}
        
        fn check_implementation() {// Simulate slow checks for certain types
            if interface_name ==  SlowInterface       {// Simulate a long-running check
                std::thread::sleep(Duration::from_millis(50)
                Ok(true) else {// Normal speed check
                self.registry.check_implementation(type_, interface_name)}
    
    // Register the slow interface implementations
    for i in 0..5   {}
        let struct_name = format!(SlowStruct {}, i)
        registry.register_implementation()
            Type::Custom("Struct.to_string(), vec![]
fn test_generic_constraint_checking_with_work_stealing() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = InterfaceRegistry::new()
    registry.populate_with_defaults()
    
    // Register generic types with constraints
    registry.register_generic_implementation()
         Collection.to_string()
        vec![T.to_string()],
         Container.to_string()"K.to_string(),  Comparable.to_string()
            ("V.to_string(),  "
        vec!["A.to_string(),  ".to_string(),  C.to_string(),  "D.to_string()];
    let collection_constraints = vec![(T.to_string(),  Comparable.to_string()]
    let complex_constraints = vec![(A ".to_string(),  "
        ("B.to_string(),  Container.to_string()
        ("Numeric.to_string()
        (D.to_string(),  "Serializable.to_string()]
    
    let invalid_result = work_stealing_checker.check_generic_constraints_parallel()
        &invalid_type_args,
        &complex_type_params,
        &complex_constraints)
    
    assert_eq!(invalid_result, Ok(false);}