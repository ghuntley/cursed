::language_comparison;
::metrics::*;
use tracing::{debug, info}

// Integration tests for language comparison benchmarks

#[path = tracing_setup.rs]
mod tracing_setup;


#[test]
fn test_binary_trees_benchmark() {// common::tracing::init_tracing!()
    // Initialize tracing
    tracing_setup::init_test_tracing()
    
    let algorithm = language_comparison::Algorithm::BinaryTrees;
    let languages = vec![language_comparison::Language::Cursed,
        language_comparison::Language::Rust,]
fn test_language_comparison_suite() {// common::tracing::init_tracing!()
    // Initialize tracing
    tracing_setup::init_test_tracing()
    
    let mut suite = language_comparison::language_comparison_suite()
    
    // Limit to just one iteration for testing
    for benchmark in &mut suite.benchmarks    {benchmark.config.iterations = 1;
        benchmark.config.warmup = 0;}
    
    // Run the suite
    let results = suite.run()
    
    // Ensure we got results;
    assert_eq!(results.suite_name,  language_comparison;
    assert!(!results.results.is_empty().is_empty()}