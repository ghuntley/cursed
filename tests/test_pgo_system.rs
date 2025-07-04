//! Integration test for PGO system

use cursed::optimization::pgo::{PgoSystem, ExecutionContext};
use std::path::Path;

#[test]
fn test_pgo_basic_functionality() {
    // Initialize PGO system
    let mut pgo_system = PgoSystem::new().expect("Failed to create PGO system");
    
    // Initialize for collection
    let output_path = Path::new("target/test-pgo");
    pgo_system.initialize_collection(&output_path).expect("Failed to initialize collection");
    
    // Create execution context
    let exec_context = ExecutionContext {
        args: vec!["test".to_string(), "program".to_string()],
        input_files: vec!["test.csd".to_string()],
        ..Default::default()
    };
    
    // Collect profile data
    let profile_data = pgo_system.collect_profile_data(&exec_context)
        .expect("Failed to collect profile data");
    
    println!("DEBUG: Profile data - samples: {}, functions: {}", 
             profile_data.total_samples, profile_data.total_functions);
    println!("DEBUG: Sufficient for optimization: {}", 
             profile_data.is_sufficient_for_optimization());
    
    assert!(profile_data.total_functions > 0, "Should have collected some function data");
    
    // Store profile data
    pgo_system.store_profile_data(&profile_data)
        .expect("Failed to store profile data");
    
    // Get optimization recommendations
    let recommendations = pgo_system.get_optimization_recommendations(&profile_data)
        .expect("Failed to get recommendations");
    
    assert!(!recommendations.is_empty(), "Should have some recommendations");
    
    // Validate effectiveness
    let validation = pgo_system.validate_optimization_effectiveness()
        .expect("Failed to validate effectiveness");
    
    assert!(validation.contains("Profile validation"), "Should contain validation info");
    
    println!("PGO system test completed successfully!");
    println!("Collected profile data: {} samples, {} functions", 
             profile_data.total_samples, profile_data.total_functions);
    println!("Optimization recommendations: {}", recommendations.len());
    println!("Validation: {}", validation);
}

#[test]
fn test_pgo_profile_analysis() {
    let mut pgo_system = PgoSystem::new().expect("Failed to create PGO system");
    
    // Initialize for collection
    let output_path = std::path::Path::new("target/test-pgo-analysis");
    pgo_system.initialize_collection(&output_path).expect("Failed to initialize collection");
    
    // Create mock execution context with multiple files
    let exec_context = ExecutionContext {
        args: vec!["cursed".to_string(), "compile".to_string(), "--optimize".to_string()],
        input_files: vec![
            "main.csd".to_string(),
            "utils.csd".to_string(), 
            "parser.csd".to_string()
        ],
        ..Default::default()
    };
    
    // Collect profile data
    let profile_data = pgo_system.collect_profile_data(&exec_context)
        .expect("Failed to collect profile data");
    
    // Verify profile data structure  
    println!("DEBUG: Main function count: {}", profile_data.get_function_count("main"));
    println!("DEBUG: Total functions: {}", profile_data.total_functions);
    println!("DEBUG: All counters: {:?}", profile_data.counters.keys().collect::<Vec<_>>());
    
    assert!(profile_data.get_function_count("main") > 0, "Should have main function data");
    assert!(profile_data.total_functions >= 3, "Should have at least main + input files");
    
    // Test hot functions analysis
    let hot_functions = profile_data.get_hot_functions(10);
    assert!(!hot_functions.is_empty(), "Should identify hot functions");
    
    println!("Hot functions identified: {:?}", hot_functions);
}
