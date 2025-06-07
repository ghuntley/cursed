use std::sync::Arc;
use std::time::Duration;
use tracing::info;
use cursed::benchmark::BenchmarkReporter;

// Integration test for the benchmark suite


mod tracing_setup {
    pub fn init_test_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info,cursed=debug")
            .with_test_writer()
            .try_init();
    }
}

#[test]
fn test_benchmark_harness() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a simple benchmark suite
    let mut suite = cursed::benchmark::harness::BenchmarkSuite::new(
        "test_suite",
        "Test benchmark suite",
    );
    
    // Add a simple benchmark
    suite.add_benchmark(cursed::benchmark::harness::Benchmark::new(
        "simple_benchmark",
        "Simple benchmark that just sleeps",
        || {
            // Sleep for a short time
            std::thread::sleep(Duration::from_millis(5));
            
            // Return metrics
            vec![Box::new(cursed::benchmark::metrics::TimingMetric {
                name: "sleep_time".to_string(),
                duration: Duration::from_millis(5),
            })]
        },
    ).with_config(cursed::benchmark::harness::BenchmarkConfig {
        iterations: 3,
        warmup: 1,
        collect_memory: false,
        collect_gc: false,
        collect_throughput: false,
    }));
    
    // Run the suite
    let results = suite.run();
    
    // Verify results
    assert_eq!(results.suite_name, "test_suite");
    assert_eq!(results.results.len(), 1);
    assert_eq!(results.results[0].name, "simple_benchmark");
    
    // Verify metrics
    assert!(!results.results[0].metrics.is_empty());
    assert_eq!(results.results[0].metrics[0].name(), "sleep_time");
    
    // Create a reporter and report results
    let reporter = cursed::benchmark::reporters::ConsoleReporter::new();
    reporter.report(&results);
}

#[test]
#[ignore = "Long-running GC benchmark test - run with --ignored flag to execute"]
fn test_gc_benchmark_suite() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a GC benchmark suite with a minimal configuration
    let mut suite = cursed::benchmark::scenarios::gc_suite();
    
    // Modify benchmarks to run fewer iterations
    for benchmark in &mut suite.benchmarks {
        benchmark.config.iterations = 2;
        benchmark.config.warmup = 1;
    }
    
    // Run the suite
    let results = suite.run();
    
    // Verify results
    assert_eq!(results.suite_name, "gc");
    assert!(!results.results.is_empty());
    
    // Create a reporter and report results
    let reporter = cursed::benchmark::reporters::ConsoleReporter::verbose();
    reporter.report(&results);
    
    // Output to CSV for analysis
    let csv_reporter = cursed::benchmark::reporters::CsvReporter::new("target/gc_benchmark_results.csv");
    csv_reporter.report(&results);
}

#[test]
#[ignore = "Long-running concurrency benchmark test - run with --ignored flag to execute"]
fn test_concurrency_benchmark_suite() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a concurrency benchmark suite with a minimal configuration
    let mut suite = cursed::benchmark::scenarios::concurrency_suite();
    
    // Modify benchmarks to run fewer iterations
    for benchmark in &mut suite.benchmarks {
        benchmark.config.iterations = 2;
        benchmark.config.warmup = 1;
    }
    
    // Run the suite
    let results = suite.run();
    
    // Verify results
    assert_eq!(results.suite_name, "concurrency");
    assert!(!results.results.is_empty());
    
    // Create a reporter and report results
    let reporter = cursed::benchmark::reporters::ConsoleReporter::verbose();
    reporter.report(&results);
    
    // Output to CSV for analysis
    let csv_reporter = cursed::benchmark::reporters::CsvReporter::new("target/concurrency_benchmark_results.csv");
    csv_reporter.report(&results);
}