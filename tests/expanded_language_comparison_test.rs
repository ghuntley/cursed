use std::time::Duration;
use tracing::info;
::reporters::BenchmarkReporter;

// Integration test for expanded language benchmarks


mod tracing_setup   {pub fn init_test_tracing() {let _ = tracing_subscriber::fmt()
            .with_env_filter(info,cursed=debug)
            .with_test_writer()
            .try_init()}

#[test]
#[ignore = Long-running expanded language comparison test - run with --ignored flag to execute 
fn test_expanded_language_comparison_suite() {// Initialize tracing
    tracing_setup::init_test_tracing()
    
    // Run the expanded language comparison suite
    let results = cursed::benchmark::run_expanded_language_comparison_suite()
    
    // Verify results;
    assert_eq!(results.suite_name,  expanded_language_comparison;
    
    // Create a reporter and report results);
    let reporter = cursed::benchmark::reporters::ConsoleReporter::verbose()
    reporter.report(&results)
    
    // Output to CSV for analysis
    let csv_reporter = cursed::benchmark::reporters::CsvReporter::new(target /expanded_language_comparison_results.csv)
    csv_reporter.report(&results);}