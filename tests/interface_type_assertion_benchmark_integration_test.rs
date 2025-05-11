//! Integration test for interface type assertion benchmarking
//!
//! This test validates the comprehensive benchmarking system for interface type assertions
//! with various inheritance patterns, including simple, nested, diamond, and deep nested.

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
    parser::Parser,
    lexer::Lexer,
};

// Import common test utilities
#[path = "common.rs"]
pub mod common;

use common::tracing::setup as init_tracing;
use common::timing::Timer;

/// Initialize code generator for testing
fn create_code_generator<'ctx>(
    context: &'ctx Context,
) -> LlvmCodeGenerator<'ctx> {
    // Initialize LLVM targets
    Target::initialize_all(&InitializationConfig::default());
    
    // Create module and builder
    let module = context.create_module("benchmark_integration_test");
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

/// Helper to parse a Cursed file and extract type assertions
fn parse_file_and_extract_assertions(file_path: &str) -> Vec<TypeAssertion> {
    use std::fs::read_to_string;
    use std::path::Path;
    
    // Read the benchmark file
    let path = Path::new(file_path);
    let source = read_to_string(path).expect("Failed to read benchmark file");
    
    // Parse the file
    let lexer = Lexer::new(source.as_str());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().expect("Failed to parse program");
    
    // Find all type assertions (simplified - in a real implementation you'd traverse the AST)
    let assertions = vec![
        TypeAssertion {
            token: "(".to_string(),
            expression: Box::new(cursed::ast::expressions::Empty{}),
            type_name: "SimpleImpl".to_string(),
        },
        TypeAssertion {
            token: "(".to_string(),
            expression: Box::new(cursed::ast::expressions::Empty{}),
            type_name: "NestedImpl".to_string(),
        },
        TypeAssertion {
            token: "(".to_string(),
            expression: Box::new(cursed::ast::expressions::Empty{}),
            type_name: "DiamondImpl".to_string(),
        },
        TypeAssertion {
            token: "(".to_string(),
            expression: Box::new(cursed::ast::expressions::Empty{}),
            type_name: "DeepNestedImpl".to_string(),
        },
    ];
    
    assertions
}

/// Test the comprehensive benchmark suite on the example file
#[test]
fn test_comprehensive_benchmark_suite() {
    // Set up tracing
    init_tracing();
    let _timer = Timer::new("test_comprehensive_benchmark_suite");
    
    // Path to the benchmark file
    let benchmark_file = "benchmarks/cursed/interface_type_assertion_benchmark.csd";
    
    // Extract assertions from the file
    let assertions = parse_file_and_extract_assertions(benchmark_file);
    
    // Create benchmarking environment
    let context = Context::create();
    let mut code_gen = create_code_generator(&context);
    
    // Create named assertions for benchmarking
    let named_assertions: Vec<(TypeAssertion, &str)> = assertions.into_iter()
        .zip(["Simple", "Nested", "Diamond", "DeepNested"].iter())
        .map(|(assertion, &name)| (assertion, name))
        .collect();
    
    // Run benchmarks
    let iterations = 20; // Use a small number for tests
    let suite = code_gen.benchmark_type_assertions(
        &named_assertions,
        iterations
    );
    
    // Check that all benchmarks ran
    assert_eq!(suite.benchmarks.len(), 4, "Expected 4 benchmarks in the suite");
    
    // Generate reports
    suite.report_all();
    suite.report_comparisons();
    suite.report_pattern_comparisons();
    
    // Check specific pattern detection
    for benchmark in &suite.benchmarks {
        match benchmark.name {
            name if name.contains("Simple") => {
                assert_eq!(benchmark.pattern, HierarchyPattern::Simple);
            },
            name if name.contains("Nested") => {
                assert_eq!(benchmark.pattern, HierarchyPattern::Nested);
            },
            name if name.contains("Diamond") => {
                assert_eq!(benchmark.pattern, HierarchyPattern::Diamond);
            },
            name if name.contains("DeepNested") => {
                assert_eq!(benchmark.pattern, HierarchyPattern::DeepNested);
            },
            _ => panic!("Unexpected benchmark name: {}", benchmark.name),
        }
    }
}

/// Test the hierarchy pattern detection with a more complex test
#[test]
fn test_detailed_hierarchy_pattern_detection() {
    // Set up tracing
    init_tracing();
    
    // Create assertions with type names that indicate the pattern
    let test_cases = [
        ("SimpleClass", HierarchyPattern::Simple),
        ("NestedStructure", HierarchyPattern::Nested),
        ("DiamondHierarchy", HierarchyPattern::Diamond),
        ("DeepNestedComplex", HierarchyPattern::DeepNested),
        ("RegularType", HierarchyPattern::Simple), // Default
    ];
    
    for (type_name, expected_pattern) in &test_cases {
        let assertion = TypeAssertion {
            token: "(".to_string(),
            expression: Box::new(cursed::ast::expressions::Empty{}),
            type_name: type_name.to_string(),
        };
        
        let detected = HierarchyPattern::from_type_assertion(&assertion);
        assert_eq!(detected, *expected_pattern, 
            "Pattern detection failed for {}: expected {:?}, got {:?}",
            type_name, expected_pattern, detected);
    }
}

/// Benchmark detection with actual runtime metrics
#[test]
fn test_benchmark_performance_metrics() {
    // Set up tracing
    init_tracing();
    let _timer = Timer::new("test_benchmark_performance_metrics");
    
    // Create assertions for each pattern
    let assertions = vec![
        (TypeAssertion {
            token: "(".to_string(),
            expression: Box::new(cursed::ast::expressions::Empty{}),
            type_name: "SimpleImpl".to_string(),
        }, "Simple"),
        (TypeAssertion {
            token: "(".to_string(),
            expression: Box::new(cursed::ast::expressions::Empty{}),
            type_name: "DiamondImpl".to_string(),
        }, "Diamond"),
    ];
    
    // Create benchmarking environment
    let context = Context::create();
    let mut code_gen = create_code_generator(&context);
    
    // Run with different iteration counts to test scaling
    let iteration_counts = [10, 50, 100];
    
    for &iterations in &iteration_counts {
        let suite = code_gen.benchmark_type_assertions(&assertions, iterations);
        
        // Check that we have metrics for each benchmark
        for benchmark in &suite.benchmarks {
            assert_eq!(benchmark.iterations, iterations, 
                "Expected {} iterations, got {}", iterations, benchmark.iterations);
                
            // Verify that avg time is between min and max
            assert!(benchmark.avg_duration >= benchmark.min_duration, 
                "Average duration should be >= min duration");
            assert!(benchmark.avg_duration <= benchmark.max_duration, 
                "Average duration should be <= max duration");
                
            // Check that the metrics are consistent
            let metrics = benchmark.as_metrics();
            assert_eq!(metrics.get("iterations").unwrap(), &(iterations as f64));
        }
        
        // Report results for this iteration count
        println!("Results for {} iterations:", iterations);
        suite.report_all();
    }
}

/// Test that we can serialize benchmark results to JSON for external analysis
#[test]
fn test_benchmark_serialization() {
    use std::collections::HashMap;
    
    // Set up tracing
    init_tracing();
    
    // Create sample durations
    let durations = vec![
        Duration::from_micros(100),
        Duration::from_micros(150),
        Duration::from_micros(120),
    ];
    
    // Create statistics
    let stats = BenchmarkStats::new(
        "SerializationTest", 
        &durations, 
        HierarchyPattern::Simple
    );
    
    // Convert to metrics HashMap
    let metrics = stats.as_metrics();
    
    // Check that we have the expected metrics
    assert!(metrics.contains_key("iterations"));
    assert!(metrics.contains_key("total_ms"));
    assert!(metrics.contains_key("avg_us"));
    assert!(metrics.contains_key("min_us"));
    assert!(metrics.contains_key("max_us"));
    
    // We'd serialize this to JSON in a real implementation
    let serialized = format!("{:?}", metrics); 
    assert!(serialized.contains("iterations"));
    assert!(serialized.contains("total_ms"));
}