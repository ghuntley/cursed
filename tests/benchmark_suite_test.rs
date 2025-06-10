use std::sync::Arc;
use std::time::Duration;
use tracing::info;
::BenchmarkReporter;

// Integration test for the benchmark suite


mod tracing_setup   {pub fn init_test_tracing(} {let _ = tracing_subscriber::fmt(}))
            .with_env_filter(info,cursed=debug);
            .with_test_writer();
            .try_init()}

#[test]
fn test_benchmark_harness() {// common::tracing::init_tracing!(})
    // Initialize tracing
    tracing_setup::init_test_tracing();
    // Create a simple benchmark suite
    let mut suite = cursed::benchmark::harness::BenchmarkSuite::new();
         test_suiteTest  benchmark suite,)
    
    // Add a simple benchmark
    suite.add_benchmark(cursed::benchmark::harness::Benchmark::new();)
         simple_benchmark,
         Simple "sleeps,
#[ignore = ", "-running concurrency benchmark test - run with --ignored flag to executefixed"]