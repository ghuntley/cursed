//! Interface Type Assertion Benchmark Runner
//!
//! This binary runs comprehensive benchmarks for interface type assertions
//! measuring performance across different inheritance patterns and optimizations.

use std::time::Duration;
use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target, TargetMachine};

use cursed::{
    ast::expressions::TypeAssertion,
    codegen::llvm::LlvmCodeGenerator,
    codegen::llvm::interface_type_assertion::InterfaceTypeAssertion,
    codegen::llvm::interface_type_assertion_benchmarking::{
        TypeAssertionBenchmarking, 
        HierarchyPattern, 
        BenchmarkStats, 
        TypeAssertionBenchmark,
        TypeAssertionBenchmarkSuite
    },
    core::interface_registry_lru_cache::LruCachedRegistry,
};

/// Setup tracing for benchmarks
fn setup_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    
    // Configure tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    
    // Set as global subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

/// Initialize code generator for benchmarking
fn create_code_generator<'ctx>(
    context: &'ctx Context,
) -> LlvmCodeGenerator<'ctx> {
    // Initialize LLVM targets
    Target::initialize_all(&InitializationConfig::default());
    
    // Create module and builder
    let module = context.create_module("benchmark_runner");
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
    
    // Create a registry with LRU cache (increasing cache size for benchmarks)
    let registry = Box::new(LruCachedRegistry::new(1000));
    
    // Create the code generator
    LlvmCodeGenerator::new(
        context,
        module,
        &builder,
        registry,
        None, // no type_registry needed for test
    )
}

/// Create a type assertion for benchmarking
fn create_test_assertion(type_name: &str) -> TypeAssertion {
    TypeAssertion {
        token: "(".to_string(),
        expression: Box::new(cursed::ast::expressions::Empty{}),
        type_name: type_name.to_string(),
    }
}

/// Run benchmarks for all patterns with different optimization settings
fn run_comprehensive_benchmarks(iterations: usize) {
    // Create LLVM context
    let context = Context::create();
    
    // Create the code generator
    let mut code_gen = create_code_generator(&context);
    
    // Create test assertions for each inheritance pattern
    let simple_assertion = create_test_assertion("SimpleImpl");
    let nested_assertion = create_test_assertion("NestedImpl");
    let diamond_assertion = create_test_assertion("DiamondImpl");
    let deep_nested_assertion = create_test_assertion("DeepNestedImpl");
    
    // Create benchmark suite
    let named_assertions = vec![
        (simple_assertion, "Simple Inheritance"),
        (nested_assertion, "Nested Inheritance"),
        (diamond_assertion, "Diamond Inheritance"),
        (deep_nested_assertion, "Deep Nested Inheritance"),
    ];
    
    // Report benchmark configuration
    println!("\n==== INTERFACE TYPE ASSERTION BENCHMARK ====");
    println!("Running with {} iterations per pattern\n", iterations);
    
    // Run the benchmark suite
    let suite = code_gen.benchmark_type_assertions(
        &named_assertions,
        iterations
    );
    
    // Generate reports
    suite.report_all();
    suite.report_comparisons();
    suite.report_pattern_comparisons();
}

/// Run a more detailed benchmark focusing on a specific pattern
fn run_detailed_pattern_benchmark(pattern: HierarchyPattern, iterations: usize) {
    println!("\n==== DETAILED {} PATTERN BENCHMARK ====", pattern.as_str());
    println!("Running with {} iterations\n", iterations);
    
    // Create LLVM context
    let context = Context::create();
    
    // Create the code generator
    let mut code_gen = create_code_generator(&context);
    
    // Create a type name based on the pattern
    let type_name = match pattern {
        HierarchyPattern::Simple => "SimpleImpl",
        HierarchyPattern::Nested => "NestedImpl",
        HierarchyPattern::Diamond => "DiamondImpl",
        HierarchyPattern::DeepNested => "DeepNestedImpl",
    };
    
    // Create the type assertion
    let assertion = create_test_assertion(type_name);
    
    // Create benchmark
    let mut benchmark = TypeAssertionBenchmark::new(
        &format!("{} Pattern Detailed", pattern.as_str()),
        pattern
    );
    
    // Run the benchmarks with warm-up
    const WARMUP_ITERATIONS: usize = 10;
    
    // Warm-up phase
    println!("Warming up...");
    for _ in 0..WARMUP_ITERATIONS {
        let _ = code_gen.compile_type_assertion(&assertion);
    }
    
    // Benchmark phase
    println!("Running benchmark...");
    for _ in 0..iterations {
        benchmark.start();
        let _ = code_gen.compile_type_assertion(&assertion);
        benchmark.stop();
    }
    
    // Report results
    benchmark.report();
    
    // Show detailed statistics
    let stats = benchmark.compute_stats();
    println!("\nDetailed Statistics:");
    println!("  Iterations:   {}", stats.iterations);
    println!("  Total Time:   {:.2}ms", stats.total_duration.as_secs_f64() * 1000.0);
    println!("  Average Time: {}μs", stats.avg_duration.as_micros());
    println!("  Min Time:     {}μs", stats.min_duration.as_micros());
    println!("  Max Time:     {}μs", stats.max_duration.as_micros());
    println!("  Std Dev:      {:.2}μs", stats.std_deviation / 1000.0);
}

/// Main function
fn main() {
    // Setup tracing
    setup_tracing();
    
    // Configuration
    let default_iterations = 100;
    let detailed_iterations = 500;
    
    // Run comprehensive benchmarks
    run_comprehensive_benchmarks(default_iterations);
    
    // Run detailed benchmarks for each pattern
    run_detailed_pattern_benchmark(HierarchyPattern::Simple, detailed_iterations);
    run_detailed_pattern_benchmark(HierarchyPattern::Nested, detailed_iterations);
    run_detailed_pattern_benchmark(HierarchyPattern::Diamond, detailed_iterations);
    run_detailed_pattern_benchmark(HierarchyPattern::DeepNested, detailed_iterations);
    
    // Final summary
    println!("\n==== BENCHMARK COMPLETE ====");
}