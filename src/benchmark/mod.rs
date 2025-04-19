//! Benchmark suite for the CURSED programming language
//!
//! This module provides benchmarking tools for measuring the performance
//! of various aspects of the CURSED implementation, with a focus on:
//! 1. Garbage collector performance
//! 2. Memory allocation patterns
//! 3. Concurrency primitives
//! 4. Parser and compiler performance

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tracing::{debug, error, info, trace, warn, instrument};

pub mod harness;
pub mod metrics;
pub mod reporters;
pub mod scenarios;

// Re-exports
pub use harness::{Benchmark, BenchmarkSuite, BenchmarkConfig};
pub use metrics::{Metric, MetricType, MetricValue, TimingMetric, MemoryMetric, ThroughputMetric};
pub use reporters::{BenchmarkReporter, ConsoleReporter, JsonReporter, CsvReporter};

/// Run a standard suite of benchmarks and return the results
#[instrument(skip_all, fields(suite_name = "standard"))]
pub fn run_standard_suite() -> harness::BenchmarkResults {
    info!("Running standard benchmark suite");
    let suite = scenarios::standard_suite();
    let reporter = ConsoleReporter::new();
    let results = suite.run();
    reporter.report(&results);
    results
}

/// Run a garbage collector focused benchmark suite
#[instrument(skip_all, fields(suite_name = "gc"))]
pub fn run_gc_suite() -> harness::BenchmarkResults {
    info!("Running garbage collector benchmark suite");
    let suite = scenarios::gc_suite();
    let reporter = ConsoleReporter::new();
    let results = suite.run();
    reporter.report(&results);
    results
}

/// Run a concurrency focused benchmark suite
#[instrument(skip_all, fields(suite_name = "concurrency"))]
pub fn run_concurrency_suite() -> harness::BenchmarkResults {
    info!("Running concurrency benchmark suite");
    let suite = scenarios::concurrency_suite();
    let reporter = ConsoleReporter::new();
    let results = suite.run();
    reporter.report(&results);
    results
}

/// Helper to run a benchmark and return timing information
#[instrument(skip_all, fields(name = %name))]
pub fn time_execution<F, T>(name: &str, f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    debug!(name = name, duration_ms = ?duration.as_millis(), "Timed execution");
    (result, duration)
}

/// Capture memory statistics before and after a function execution
#[instrument(skip_all, fields(name = %name))]
pub fn measure_memory<F, T>(name: &str, gc: &Arc<crate::memory::GarbageCollector>, f: F) -> (T, metrics::MemoryMetric)
where
    F: FnOnce() -> T,
{
    let before = gc.stats();
    let result = f();
    let after = gc.stats();
    
    let metric = metrics::MemoryMetric {
        name: name.to_string(),
        before_object_count: before.object_count,
        after_object_count: after.object_count,
        before_total_size: before.total_size,
        after_total_size: after.total_size,
        allocated: after.total_size - before.total_size + after.total_collected - before.total_collected,
        collected: after.total_collected - before.total_collected,
        collection_time_ms: after.total_gc_time_ms - before.total_gc_time_ms,
    };
    
    debug!(
        name = name,
        before_objects = before.object_count,
        after_objects = after.object_count,
        allocated_bytes = metric.allocated,
        collected_bytes = metric.collected,
        "Memory measurement"
    );
    
    (result, metric)
}