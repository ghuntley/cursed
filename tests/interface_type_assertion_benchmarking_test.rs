//! Test for the interface type assertion benchmarking capabilities
//!
//! This test validates the benchmarking system for interface type assertions,
//! demonstrating how to use it for performance analysis and optimization.

use std::time::Duration;
use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target, TargetMachine};

use cursed::{
    ast::expressions::TypeAssertion,
    codegen::llvm::LlvmCodeGenerator,
    codegen::llvm::interface_type_assertion::InterfaceTypeAssertion,
    codegen::llvm::interface_type_assertion_benchmarking::{TypeAssertionBenchmarking, HierarchyPattern, BenchmarkStats, TypeAssertionBenchmark},
    core::interface_registry_lru_cache::LruCachedRegistry,
};

// Import common test utilities
#[path = "common.rs"]
pub mod common;

use common::tracing::setup as init_tracing;
use common::timing::Timer;

// Constants for benchmark configuration
const WARMUP_ITERATIONS: usize = 5;
const BENCHMARK_ITERATIONS: usize = 10; // Reduced for tests, use higher values for actual benchmarks

/// Helper to create a test assertion
fn create_test_assertion(type_name: &str) -> TypeAssertion {
    TypeAssertion {
        token: "(".to_string(),
        expression: Box::new(cursed::ast::expressions::Empty{}),
        type_name: type_name.to_string(),
    }
}

/// Create a code generator for benchmarking
fn create_code_generator<'ctx>(
    context: &'ctx Context,
) -> LlvmCodeGenerator<'ctx> {
    // Initialize LLVM targets
    Target::initialize_all(&InitializationConfig::default());
    
    // Create module and builder
    let module = context.create_module("benchmark_test");
    let builder = context.create_builder();
    
    // Set up a target machine for the module
    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target.create_target_machine(
        &target_triple,
        "generic",
        "",
        inkwell::OptimizationLevel::Default,
        inkwell::targets::RelocMode::Default,
        inkwell::targets::CodeModel::Default,
    ).unwrap();
    
    // Set up the data layout
    let data_layout = target_machine.get_target_data().get_data_layout();
    module.set_data_layout(&data_layout);
    
    // Create a test function
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create a registry with LRU cache
    let registry = Box::new(LruCachedRegistry::new(100));
    
    // Create the code generator
    LlvmCodeGenerator::new(
        context,
        module,
        &builder,
        registry,
        None, // no type_registry needed for test
    )
}

/// Test the basic benchmarking functionality
#[test]
fn test_basic_benchmarking() {
    // Set up tracing
    init_tracing();
    let _timer = Timer::new("test_basic_benchmarking");
    
    // Create LLVM context
    let context = Context::create();
    
    // Create our code generator
    let mut code_gen = create_code_generator(&context);
    
    // Create a simple benchmark
    let mut benchmark = TypeAssertionBenchmark::new(
        "Simple Benchmark", 
        HierarchyPattern::Simple
    );
    
    // Run a simple operation to benchmark
    let _duration = benchmark.benchmark(|| {
        // Simulate work
        std::thread::sleep(Duration::from_micros(10));
    });
    
    // Check that we recorded a duration
    assert!(!benchmark.compute_stats().durations.is_empty());
    
    // Report the results
    benchmark.report();
}

/// Test benchmarking a full type assertion
#[test]
fn test_type_assertion_benchmarking() {
    // Set up tracing
    init_tracing();
    let _timer = Timer::new("test_type_assertion_benchmarking");
    
    // Create LLVM context
    let context = Context::create();
    
    // Create our code generator
    let mut code_gen = create_code_generator(&context);
    
    // Create a test type assertion
    let type_assertion = create_test_assertion("TestType");
    
    // Benchmark the type assertion
    let result = code_gen.compile_type_assertion_with_benchmarking(&type_assertion);
    
    // Check that we got a result and benchmark stats
    assert!(result.is_ok());
    let (_value, stats) = result.unwrap();
    
    // Report the stats
    stats.report();
}

/// Test benchmarking multiple different assertion patterns
#[test]
fn test_benchmark_suite() {
    // Set up tracing
    init_tracing();
    let _timer = Timer::new("test_benchmark_suite");
    
    // Create LLVM context
    let context = Context::create();
    
    // Create our code generator
    let mut code_gen = create_code_generator(&context);
    
    // Create different types of assertions
    let simple_assertion = create_test_assertion("SimpleType");
    let nested_assertion = create_test_assertion("NestedType");
    let diamond_assertion = create_test_assertion("DiamondType");
    let deep_nested_assertion = create_test_assertion("DeepNestedType");
    
    // Create a list of assertions to benchmark
    let assertions = vec![
        (simple_assertion, "Simple Assertion"),
        (nested_assertion, "Nested Assertion"),
        (diamond_assertion, "Diamond Assertion"),
        (deep_nested_assertion, "Deep Nested Assertion"),
    ];
    
    // Run the benchmarking suite
    let suite = code_gen.benchmark_type_assertions(
        &assertions,
        BENCHMARK_ITERATIONS
    );
    
    // Generate reports
    suite.report_all();
    suite.report_comparisons();
    suite.report_pattern_comparisons();
}

/// Test the BenchmarkStats functionality
#[test]
fn test_benchmark_stats() {
    // Set up tracing
    init_tracing();
    
    // Create some test durations
    let durations = vec![
        Duration::from_micros(100),
        Duration::from_micros(150),
        Duration::from_micros(120),
        Duration::from_micros(110),
        Duration::from_micros(130),
    ];
    
    // Create statistics
    let stats = BenchmarkStats::new(
        "Test Stats", 
        &durations, 
        HierarchyPattern::Simple
    );
    
    // Verify statistics
    assert_eq!(stats.iterations, 5);
    assert_eq!(stats.min_duration, Duration::from_micros(100));
    assert_eq!(stats.max_duration, Duration::from_micros(150));
    
    // Check average calculation
    let expected_avg = Duration::from_micros(122); // (100+150+120+110+130)/5 = 122
    assert_eq!(stats.avg_duration.as_micros(), expected_avg.as_micros());
    
    // Generate and check metrics
    let metrics = stats.as_metrics();
    assert_eq!(metrics.get("iterations").unwrap(), &5.0);
    
    // Report the statistics
    stats.report();
}

/// Test hierarchical pattern detection
#[test]
fn test_hierarchy_pattern_detection() {
    // Set up tracing
    init_tracing();
    
    // Create assertions with different type patterns
    let simple = create_test_assertion("SimpleType");
    let nested = create_test_assertion("NestedType");
    let diamond = create_test_assertion("DiamondType");
    let deep_nested = create_test_assertion("DeepNestedType");
    
    // Detect patterns
    assert_eq!(HierarchyPattern::from_type_assertion(&simple), HierarchyPattern::Simple);
    assert_eq!(HierarchyPattern::from_type_assertion(&nested), HierarchyPattern::Nested);
    assert_eq!(HierarchyPattern::from_type_assertion(&diamond), HierarchyPattern::Diamond);
    assert_eq!(HierarchyPattern::from_type_assertion(&deep_nested), HierarchyPattern::DeepNested);
}