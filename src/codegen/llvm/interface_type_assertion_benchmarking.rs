//! # Interface Type Assertion Performance Benchmarking
//!
//! This module provides comprehensive performance benchmarking for interface type assertions,
//! allowing precise measurement of different assertion patterns and optimization strategies.
//!
//! ## Features
//!
//! - High precision timing for interface type assertions
//! - Support for different hierarchy patterns (simple, nested, diamond)
//! - Statistical analysis of performance patterns
//! - Integration with tracing for structured logging
//! - Memory usage tracking for different interface patterns
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Basic usage
//! let mut benchmark = TypeAssertionBenchmark::new("my_benchmark");
//! benchmark.start();
//! // ... perform type assertion ...
//! benchmark.stop();
//! benchmark.report(); // Logs benchmark results
//!
//! // Comparative benchmarking
//! let mut benchmark_suite = TypeAssertionBenchmarkSuite::new();
//! benchmark_suite.benchmark("simple", || {
//!     // simple type assertion code
//! });
//! benchmark_suite.benchmark("complex", || {
//!     // complex type assertion code
//! });
//! benchmark_suite.report_comparisons();
//! ```

use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{debug, info, trace};
use crate::ast::expressions::TypeAssertion;
use crate::error::Error;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;

/// Different interface hierarchy patterns for benchmark classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HierarchyPattern {
    /// Direct interface implementation (Interface -> Struct)
    Simple,
    /// One level of interface nesting (Interface A -> Interface B -> Struct)
    Nested,
    /// Diamond inheritance pattern (Interface A, B -> Interface C -> Struct)
    Diamond,
    /// Deep nested interface hierarchy with multiple levels
    DeepNested,
}

impl HierarchyPattern {
    /// Convert a pattern to a string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            HierarchyPattern::Simple => "Simple",
            HierarchyPattern::Nested => "Nested",
            HierarchyPattern::Diamond => "Diamond",
            HierarchyPattern::DeepNested => "Deep Nested",
        }
    }
    
    /// Attempt to detect the pattern from a type assertion
    pub fn from_type_assertion(type_assertion: &TypeAssertion) -> Self {
        // In a real implementation, this would analyze the type hierarchy
        // For now, we'll use a simple heuristic based on the type name
        let type_name = &type_assertion.type_name;
        
        if type_name.contains("Diamond") {
            HierarchyPattern::Diamond
        } else if type_name.contains("DeepNested") {
            HierarchyPattern::DeepNested
        } else if type_name.contains("Nested") {
            HierarchyPattern::Nested
        } else {
            HierarchyPattern::Simple
        }
    }
}

/// Detailed statistics for benchmarking results
#[derive(Debug, Clone)]
pub struct BenchmarkStats {
    /// Name of the benchmark
    pub name: String,
    /// Number of iterations measured
    pub iterations: usize,
    /// Total duration of all iterations
    pub total_duration: Duration,
    /// Minimum duration observed
    pub min_duration: Duration,
    /// Maximum duration observed
    pub max_duration: Duration,
    /// Average duration
    pub avg_duration: Duration,
    /// Standard deviation of durations
    pub std_deviation: f64,
    /// Hierarchy pattern being benchmarked
    pub pattern: HierarchyPattern,
}

impl BenchmarkStats {
    /// Create a new benchmark statistics object from collected measurements
    pub fn new(name: &str, durations: &[Duration], pattern: HierarchyPattern) -> Self {
        let iterations = durations.len();
        let total_duration: Duration = durations.iter().sum();
        
        let min_duration = durations.iter()
            .min()
            .cloned()
            .unwrap_or_else(|| Duration::new(0, 0));
            
        let max_duration = durations.iter()
            .max()
            .cloned()
            .unwrap_or_else(|| Duration::new(0, 0));
            
        let avg_nanos = if iterations > 0 {
            total_duration.as_nanos() as f64 / iterations as f64
        } else {
            0.0
        };
        
        let avg_duration = Duration::from_nanos(avg_nanos as u64);
        
        // Calculate standard deviation
        let variance = if iterations > 1 {
            let mean_nanos = avg_duration.as_nanos() as f64;
            let sum_squared_diff: f64 = durations.iter()
                .map(|d| {
                    let diff = d.as_nanos() as f64 - mean_nanos;
                    diff * diff
                })
                .sum();
            sum_squared_diff / (iterations - 1) as f64
        } else {
            0.0
        };
        
        let std_deviation = variance.sqrt();
        
        BenchmarkStats {
            name: name.to_string(),
            iterations,
            total_duration,
            min_duration,
            max_duration,
            avg_duration,
            std_deviation,
            pattern,
        }
    }
    
    /// Report the benchmark statistics using structured logging
    pub fn report(&self) {
        info!(
            benchmark = %self.name,
            pattern = %self.pattern.as_str(),
            iterations = %self.iterations,
            total_ms = %self.total_duration.as_secs_f64() * 1000.0,
            avg_us = %self.avg_duration.as_micros(),
            min_us = %self.min_duration.as_micros(),
            max_us = %self.max_duration.as_micros(),
            std_dev_us = %self.std_deviation / 1000.0,
            "Completed benchmark"
        );
    }
    
    /// Generate a machine-readable report
    pub fn as_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("iterations".to_string(), self.iterations as f64);
        metrics.insert("total_ms".to_string(), self.total_duration.as_secs_f64() * 1000.0);
        metrics.insert("avg_us".to_string(), self.avg_duration.as_micros() as f64);
        metrics.insert("min_us".to_string(), self.min_duration.as_micros() as f64);
        metrics.insert("max_us".to_string(), self.max_duration.as_micros() as f64);
        metrics.insert("std_dev_us".to_string(), self.std_deviation / 1000.0);
        metrics
    }
}

/// A benchmark timer for a single measurement
pub struct TypeAssertionBenchmark {
    /// Name of the benchmark
    name: String,
    /// Start time
    start: Option<Instant>,
    /// Recorded durations
    durations: Vec<Duration>,
    /// Hierarchy pattern being benchmarked
    pattern: HierarchyPattern,
}

impl TypeAssertionBenchmark {
    /// Create a new benchmark with the given name and pattern
    pub fn new(name: &str, pattern: HierarchyPattern) -> Self {
        Self {
            name: name.to_string(),
            start: None,
            durations: Vec::new(),
            pattern,
        }
    }
    
    /// Start the benchmark timer
    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }
    
    /// Stop the benchmark timer and record the duration
    pub fn stop(&mut self) -> Option<Duration> {
        self.start.map(|start| {
            let duration = start.elapsed();
            self.durations.push(duration);
            self.start = None;
            duration
        })
    }
    
    /// Run a single benchmark iteration with the given function
    pub fn benchmark<F>(&mut self, f: F) -> Duration
    where
        F: FnOnce() -> (),
    {
        self.start();
        f();
        self.stop().unwrap_or_else(|| Duration::from_secs(0))
    }
    
    /// Run multiple benchmark iterations and collect statistics
    pub fn benchmark_iterations<F>(&mut self, iterations: usize, f: F) -> BenchmarkStats
    where
        F: Fn() -> (),
    {
        for _ in 0..iterations {
            self.benchmark(&f);
        }
        
        self.compute_stats()
    }
    
    /// Compute statistics from collected measurements
    pub fn compute_stats(&self) -> BenchmarkStats {
        BenchmarkStats::new(&self.name, &self.durations, self.pattern)
    }
    
    /// Report the benchmark results
    pub fn report(&self) {
        let stats = self.compute_stats();
        stats.report();
    }
}

/// A benchmark suite for running multiple related benchmarks
pub struct TypeAssertionBenchmarkSuite {
    /// Benchmarks in the suite
    pub benchmarks: Vec<BenchmarkStats>,
}

impl TypeAssertionBenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new() -> Self {
        Self {
            benchmarks: Vec::new(),
        }
    }
    
    /// Add a benchmark to the suite
    pub fn add_benchmark(&mut self, benchmark: BenchmarkStats) {
        self.benchmarks.push(benchmark);
    }
    
    /// Run a benchmark and add it to the suite
    pub fn benchmark<F>(&mut self, name: &str, pattern: HierarchyPattern, iterations: usize, f: F) 
    where
        F: Fn() -> (),
    {
        let mut benchmark = TypeAssertionBenchmark::new(name, pattern);
        let stats = benchmark.benchmark_iterations(iterations, f);
        self.add_benchmark(stats);
    }
    
    /// Report all benchmarks in the suite
    pub fn report_all(&self) {
        for benchmark in &self.benchmarks {
            benchmark.report();
        }
    }
    
    /// Report comparative analysis between benchmarks
    pub fn report_comparisons(&self) {
        if self.benchmarks.len() <= 1 {
            info!("Not enough benchmarks for comparison");
            return;
        }
        
        // Sort benchmarks by average duration
        let mut sorted_benchmarks = self.benchmarks.clone();
        sorted_benchmarks.sort_by(|a, b| a.avg_duration.cmp(&b.avg_duration));
        
        // Use the fastest as baseline
        let baseline = &sorted_benchmarks[0];
        
        info!(
            baseline = %baseline.name,
            baseline_avg_us = %baseline.avg_duration.as_micros(),
            "Benchmark comparisons (baseline: {})",
            baseline.name
        );
        
        for benchmark in &sorted_benchmarks[1..] {
            let ratio = benchmark.avg_duration.as_nanos() as f64 / 
                       baseline.avg_duration.as_nanos() as f64;
                       
            info!(
                benchmark = %benchmark.name,
                pattern = %benchmark.pattern.as_str(),
                avg_us = %benchmark.avg_duration.as_micros(),
                comparison = %format!("{:.2}x slower", ratio),
                "Comparison to baseline"
            );
        }
    }
    
    /// Report comparisons by hierarchy pattern
    pub fn report_pattern_comparisons(&self) {
        // Group benchmarks by pattern
        let mut pattern_groups: HashMap<HierarchyPattern, Vec<&BenchmarkStats>> = HashMap::new();
        
        for benchmark in &self.benchmarks {
            pattern_groups
                .entry(benchmark.pattern)
                .or_default()
                .push(benchmark);
        }
        
        // Report stats for each pattern group
        for (pattern, benchmarks) in pattern_groups {
            let avg_duration_ns: f64 = benchmarks.iter()
                .map(|b| b.avg_duration.as_nanos() as f64)
                .sum::<f64>() / benchmarks.len() as f64;
                
            info!(
                pattern = %pattern.as_str(),
                benchmark_count = %benchmarks.len(),
                avg_us = %(avg_duration_ns / 1000.0),
                "Pattern comparison"
            );
        }
    }
}

/// Extension trait for LLVM code generator to add benchmarking capabilities
pub trait TypeAssertionBenchmarking<'ctx>: InterfaceTypeAssertion<'ctx> {
    /// Compile a type assertion with benchmarking
    fn compile_type_assertion_with_benchmarking(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<(inkwell::values::BasicValueEnum<'ctx>, BenchmarkStats), Error>;
    
    /// Run a benchmark suite for different assertion patterns
    fn benchmark_type_assertions(
        &mut self,
        assertion_types: &[(TypeAssertion, &str)],
        iterations: usize
    ) -> TypeAssertionBenchmarkSuite;
}

/// Implementation of benchmarking for LlvmCodeGenerator
impl<'ctx> TypeAssertionBenchmarking<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_type_assertion_with_benchmarking(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<(inkwell::values::BasicValueEnum<'ctx>, BenchmarkStats), Error> {
        // Detect hierarchy pattern
        let pattern = HierarchyPattern::from_type_assertion(type_assertion);
        
        // Create a benchmark
        let name = format!("TypeAssertion: {}", type_assertion.type_name);
        let mut benchmark = TypeAssertionBenchmark::new(&name, pattern);
        
        // Start timing
        benchmark.start();
        
        // Perform the type assertion
        let result = self.compile_type_assertion(type_assertion);
        
        // Stop timing
        benchmark.stop();
        
        // Compute statistics
        let stats = benchmark.compute_stats();
        
        // Log basic timing info
        trace!(
            type_name = %type_assertion.type_name,
            duration_us = %stats.avg_duration.as_micros(),
            pattern = %pattern.as_str(),
            "Type assertion compiled"
        );
        
        // Return the result with benchmark stats
        result.map(|val| (val, stats))
    }
    
    fn benchmark_type_assertions(
        &mut self,
        assertion_types: &[(TypeAssertion, &str)],
        iterations: usize
    ) -> TypeAssertionBenchmarkSuite {
        let mut suite = TypeAssertionBenchmarkSuite::new();
        
        for (type_assertion, name) in assertion_types {
            let pattern = HierarchyPattern::from_type_assertion(type_assertion);
            
            // Create a benchmark for this type assertion
            let mut benchmark = TypeAssertionBenchmark::new(name, pattern);
            
            // Run the benchmark
            for _ in 0..iterations {
                benchmark.start();
                let _ = self.compile_type_assertion(type_assertion);
                benchmark.stop();
            }
            
            // Add to suite
            suite.add_benchmark(benchmark.compute_stats());
        }
        
        suite
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use inkwell::context::Context;
    use crate::ast::expressions::{TypeAssertion, Identifier, Empty};
    use crate::ast::traits::Expression;
    use crate::InterfaceTypeRegistry;
use crate::core::interface_registry_lru_cache::LruCachedRegistry;
    
    /// Create a test type assertion
    fn create_test_assertion(type_name: &str) -> TypeAssertion {
        TypeAssertion {
            token: "(".to_string(),
            expression: Box::new(Empty{}),
            type_name: type_name.to_string(),
        }
    }
    
    /// Test the basic benchmark functionality
    #[test]
    fn test_basic_benchmarking() {
        // Set up the LLVM context and code generator
        let context = Context::create();
        let module = context.create_module("benchmark_test");
        let builder = context.create_builder();
        
        // Set up a registry
        let registry: Box<dyn InterfaceTypeRegistry> = Box::new(LruCachedRegistry::new(100));
        
        // Create a test function
        let void_type = context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let function = module.add_function("test_function", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create the code generator
        let mut code_gen = LlvmCodeGenerator::new(
            &context,
            module,
            &builder,
            registry,
            None, // no type_registry needed for test
        );
        
        // Create test type assertions
        let simple_assertion = create_test_assertion("SimpleType");
        let nested_assertion = create_test_assertion("NestedType");
        let diamond_assertion = create_test_assertion("DiamondType");
        
        // Test with benchmarking
        let assertions = vec![
            (simple_assertion, "Simple Assertion"),
            (nested_assertion, "Nested Assertion"),
            (diamond_assertion, "Diamond Assertion"),
        ];
        
        // Run the benchmark suite
        let suite = code_gen.benchmark_type_assertions(
            &assertions,
            10 // just a few iterations for the test
        );
        
        // Report results
        suite.report_all();
        suite.report_comparisons();
        suite.report_pattern_comparisons();
    }
}