use cursed::benchmark::language_comparison;
use cursed::benchmark::metrics::*;
use tracing::{debug, info};

// Integration tests for language comparison benchmarks

#[path = "tracing_setup.rs"]
mod tracing_setup;


#[test]
fn test_binary_trees_benchmark() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    let algorithm = language_comparison::Algorithm::BinaryTrees;
    let languages = vec![
        language_comparison::Language::Cursed,
        language_comparison::Language::Rust,
    ];
    
    for lang in languages {
        if language_comparison::ensure_language_benchmarks(lang) {
            info!(language = ?lang, algorithm = ?algorithm, "Testing benchmark implementation");
            let (duration, _) = language_comparison::run_language_benchmark(lang, algorithm);
            assert!(duration.as_millis() > 0, "Benchmark should take some time to run");
            info!(language = ?lang, time_ms = ?duration.as_millis(), "Benchmark completed");
        } else {
            info!(language = ?lang, "Language not available, skipping test");
        }
    }
}

#[test]
fn test_n_bodies_benchmark() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    let algorithm = language_comparison::Algorithm::NBodies;
    let languages = vec![
        language_comparison::Language::Cursed,
        language_comparison::Language::Rust,
    ];
    
    for lang in languages {
        if language_comparison::ensure_language_benchmarks(lang) {
            info!(language = ?lang, algorithm = ?algorithm, "Testing benchmark implementation");
            let (duration, _) = language_comparison::run_language_benchmark(lang, algorithm);
            assert!(duration.as_millis() > 0, "Benchmark should take some time to run");
            info!(language = ?lang, time_ms = ?duration.as_millis(), "Benchmark completed");
        } else {
            info!(language = ?lang, "Language not available, skipping test");
        }
    }
}

#[test]
fn test_language_comparison_suite() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    let mut suite = language_comparison::language_comparison_suite();
    
    // Limit to just one iteration for testing
    for benchmark in &mut suite.benchmarks {
        benchmark.config.iterations = 1;
        benchmark.config.warmup = 0;
    }
    
    // Run the suite
    let results = suite.run();
    
    // Ensure we got results
    assert_eq!(results.suite_name, "language_comparison");
    assert!(!results.results.is_empty().is_empty());
}