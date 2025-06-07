use std::time::{Duration, Instant};
use tracing::{debug, info};
use inkwell;
use cursed::lexer::Token;
use common::tracing::setup as setup_tracing;
use common::timing::Timer;

#[cfg(test)]
mod tests {
    
    use cursed::{
        ast::expressions::TypeAssertion,
        ast::types::{InterfaceType, StructType, Type},
        core::{
            interface_registry::InterfaceRegistry,
            interface_registry_cache::CachedRegistry, 
            interface_registry_lru_cache::LruCachedRegistry,
        },
        codegen::llvm::{
            LlvmCodeGenerator,
            interface_type_assertion::InterfaceTypeAssertion,
            type_assertion_integration::TypeAssertionIntegration,
            improved_type_assertion_integration::ImprovedTypeAssertionIntegration,
        },
    };
    
    // Import common testing utils
    #[path = "common/mod.rs"]
    mod common;
    
    const BENCHMARK_ITERATIONS: u32 = 1000;
    const WARMUP_ITERATIONS: u32 = 100;
    
    /// A struct to hold benchmark results
    #[derive(Debug, Clone)]
    struct BenchmarkResult {
        name: String,
        iterations: u32,
        total_duration: Duration,
        avg_duration_ns: u64,
        min_duration_ns: u64,
        max_duration_ns: u64,
    }
    
    impl BenchmarkResult {
        fn new(name: &str, iterations: u32, durations: Vec<Duration>) -> Self {
            let total_duration = durations.iter().sum();
            let avg_duration_ns = total_duration.as_nanos() as u64 / iterations as u64;
            let min_duration_ns = durations.iter().map(|d| d.as_nanos() as u64).min().unwrap_or(0);
            let max_duration_ns = durations.iter().map(|d| d.as_nanos() as u64).max().unwrap_or(0);
            
            BenchmarkResult {
                name: name.to_string(),
                iterations,
                total_duration,
                avg_duration_ns,
                min_duration_ns,
                max_duration_ns,
            }
        }
        
        fn report(&self) {
            info!("Benchmark: {}", self.name);
            info!("  Iterations: {}", self.iterations);
            info!("  Total duration: {:?}", self.total_duration);
            info!("  Avg duration: {} ns", self.avg_duration_ns);
            info!("  Min duration: {} ns", self.min_duration_ns);
            info!("  Max duration: {} ns", self.max_duration_ns);
        }
    }
    
    /// Run a benchmark with a given function and return results
    fn run_benchmark<F>(name: &str, iterations: u32, warmup: u32, f: F) -> BenchmarkResult 
    where
        F: Fn() -> (),
    {
        // Perform warmup iterations
        for _ in 0..warmup {
            f();
        }
        
        // Collect timing for each iteration
        let mut durations = Vec::with_capacity(iterations as usize);
        
        for _ in 0..iterations {
            let start = Instant::now();
            f();
            durations.push(start.elapsed());
        }
        
        BenchmarkResult::new(name, iterations, durations)
    }
    
    /// Define an enum for interface hierarchy benchmarking
    enum HierarchyType {
        Simple,      // Direct interface implementation
        Nested,      // One level of nesting (Interface A -> Interface B -> Struct)
        Diamond,     // Diamond inheritance (Interface A, B -> Interface C -> Struct)
        DeepNested,  // Deep nesting with multiple levels
    }
    
    /// Helper to prepare a benchmark function for type assertions with different hierarchy types
    fn prepare_complex_hierarchy_benchmark(
        hierarchy_type: HierarchyType,
        use_lru_cache: bool,
        use_improved_integration: bool
    ) -> Box<dyn Fn() -> ()> {
        // Setup the test - we'll create a minimal LLVM environment for benchmarking
        let context = inkwell::context::Context::create();
        let module = context.create_module("benchmark_module");
        let builder = context.create_builder();
        
        // Create the appropriate registry based on cache type
        let registry: Box<dyn InterfaceRegistry> = if use_lru_cache {
            Box::new(LruCachedRegistry::new(100)) // Cache size of 100
        } else {
            Box::new(CachedRegistry::new())
        };
        
        // Create a target machine for the code generator
        let target_triple = inkwell::targets::TargetMachine::get_default_triple();
        inkwell::targets::Target::initialize_all(&inkwell::targets::InitializationConfig::default());
        let target = inkwell::targets::Target::from_triple(&target_triple).unwrap();
        let target_machine = target.create_target_machine(
            &target_triple,
            "generic",
            "",
            inkwell::OptimizationLevel::Default,
            inkwell::targets::RelocMode::Default,
            inkwell::targets::CodeModel::Default,
        ).unwrap();
        
        // Create a data layout for LLVM module
        let data_layout = target_machine.get_target_data().get_data_layout();
        module.set_data_layout(&data_layout);
        
        // Create a simplified test function to benchmark
        let void_type = context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let function = module.add_function("test_function", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create a dummy LlvmCodeGenerator
        let mut code_gen = LlvmCodeGenerator::new(
            &context,
            module,
            &builder,
            registry,
            None, // no type_registry needed for benchmark
        );
        
        // Create the appropriate type assertion based on hierarchy type
        let type_assertion = match hierarchy_type {
            HierarchyType::Simple => TypeAssertion::simple_for_testing(),
            HierarchyType::Nested => TypeAssertion::nested_for_testing(),
            HierarchyType::Diamond => TypeAssertion::diamond_for_testing(),
            HierarchyType::DeepNested => TypeAssertion::deep_nested_for_testing(),
        };
        
        // Return a closure that performs the appropriate type assertion operation
        Box::new(move || {
            if use_improved_integration {
                // Use the improved integration path
                let _ = code_gen.compile_type_assertion_with_propagation(
                    &type_assertion,
                    Some("benchmark location")
                );
            } else {
                // Use the standard integration path
                let _ = code_gen.compile_type_assertion_integrated(&type_assertion);
            }
        })
    }
    
    /// Benchmark simple interface type assertion with LRU cache
    #[test]
    fn benchmark_simple_hierarchy_lru() {
        setup_tracing();
        let _timer = Timer::new("benchmark_simple_hierarchy_lru");
        
        let assertion_fn = prepare_complex_hierarchy_benchmark(
            HierarchyType::Simple,
            true,  // use LRU cache
            true   // use improved integration
        );
        
        let result = run_benchmark(
            "Simple Interface Hierarchy (LRU)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark nested interface hierarchy with LRU cache
    #[test]
    fn benchmark_nested_hierarchy_lru() {
        setup_tracing();
        let _timer = Timer::new("benchmark_nested_hierarchy_lru");
        
        let assertion_fn = prepare_complex_hierarchy_benchmark(
            HierarchyType::Nested,
            true,  // use LRU cache
            true   // use improved integration
        );
        
        let result = run_benchmark(
            "Nested Interface Hierarchy (LRU)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark diamond inheritance pattern with LRU cache
    #[test]
    fn benchmark_diamond_hierarchy_lru() {
        setup_tracing();
        let _timer = Timer::new("benchmark_diamond_hierarchy_lru");
        
        let assertion_fn = prepare_complex_hierarchy_benchmark(
            HierarchyType::Diamond,
            true,  // use LRU cache
            true   // use improved integration
        );
        
        let result = run_benchmark(
            "Diamond Interface Hierarchy (LRU)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark deep nested interface hierarchy with LRU cache
    #[test]
    fn benchmark_deep_nested_hierarchy_lru() {
        setup_tracing();
        let _timer = Timer::new("benchmark_deep_nested_hierarchy_lru");
        
        let assertion_fn = prepare_complex_hierarchy_benchmark(
            HierarchyType::DeepNested,
            true,  // use LRU cache
            true   // use improved integration
        );
        
        let result = run_benchmark(
            "Deep Nested Interface Hierarchy (LRU)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark performance comparison of all hierarchy types
    #[test]
    fn benchmark_hierarchy_comparison() {
        setup_tracing();
        let _timer = Timer::new("benchmark_hierarchy_comparison");
        
        // Array of hierarchy types to benchmark
        let hierarchy_types = [
            (HierarchyType::Simple, "Simple"),
            (HierarchyType::Nested, "Nested"),
            (HierarchyType::Diamond, "Diamond"),
            (HierarchyType::DeepNested, "Deep Nested"),
        ];
        
        // Array of cache types to compare
        let cache_types = [
            (false, "No Cache"),
            (true, "LRU Cache"),
        ];
        
        // Run benchmarks for each combination
        for (hierarchy_type, hierarchy_name) in &hierarchy_types {
            for (use_lru, cache_name) in &cache_types {
                let assertion_fn = prepare_complex_hierarchy_benchmark(
                    hierarchy_type.clone(),
                    *use_lru,
                    true  // always use improved integration
                );
                
                let result = run_benchmark(
                    &format!("{} Hierarchy ({})", hierarchy_name, cache_name),
                    BENCHMARK_ITERATIONS,
                    WARMUP_ITERATIONS,
                    assertion_fn
                );
                
                result.report();
            }
        }
    }
    
    /// Benchmark with realistic scenario simulating common usage patterns
    #[test]
    fn benchmark_realistic_workload() {
        setup_tracing();
        let _timer = Timer::new("benchmark_realistic_workload");
        
        // Create a more complex workload simulation
        // This will create a mix of different assertion types
        let context = inkwell::context::Context::create();
        let module = context.create_module("benchmark_module");
        let builder = context.create_builder();
        
        // Use LRU cache for realistic scenario
        let registry: Box<dyn InterfaceRegistry> = Box::new(LruCachedRegistry::new(100));
        
        // Create a target machine for the code generator
        let target_triple = inkwell::targets::TargetMachine::get_default_triple();
        inkwell::targets::Target::initialize_all(&inkwell::targets::InitializationConfig::default());
        let target = inkwell::targets::Target::from_triple(&target_triple).unwrap();
        let target_machine = target.create_target_machine(
            &target_triple,
            "generic",
            "",
            inkwell::OptimizationLevel::Default,
            inkwell::targets::RelocMode::Default,
            inkwell::targets::CodeModel::Default,
        ).unwrap();
        
        // Create a data layout for LLVM module
        let data_layout = target_machine.get_target_data().get_data_layout();
        module.set_data_layout(&data_layout);
        
        // Create a simplified test function to benchmark
        let void_type = context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let function = module.add_function("test_function", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create a dummy LlvmCodeGenerator
        let mut code_gen = LlvmCodeGenerator::new(
            &context,
            module,
            &builder,
            registry,
            None, // no type_registry needed for benchmark
        );
        
        // Create assertions with different complexity
        let simple_assertion = TypeAssertion::simple_for_testing();
        let nested_assertion = TypeAssertion::nested_for_testing();
        let diamond_assertion = TypeAssertion::diamond_for_testing();
        let deep_nested_assertion = TypeAssertion::deep_nested_for_testing();
        
        // Create a workload simulation function that performs a mix of assertions
        let workload_fn = move || {
            // Realistic workload with 60% simple, 20% nested, 15% diamond, 5% deep nested
            for i in 0..100 {
                if i < 60 {
                    let _ = code_gen.compile_type_assertion_with_propagation(
                        &simple_assertion,
                        Some("benchmark location")
                    );
                } else if i < 80 {
                    let _ = code_gen.compile_type_assertion_with_propagation(
                        &nested_assertion,
                        Some("benchmark location")
                    );
                } else if i < 95 {
                    let _ = code_gen.compile_type_assertion_with_propagation(
                        &diamond_assertion,
                        Some("benchmark location")
                    );
                } else {
                    let _ = code_gen.compile_type_assertion_with_propagation(
                        &deep_nested_assertion,
                        Some("benchmark location")
                    );
                }
            }
        };
        
        // Run the benchmark
        let result = run_benchmark(
            "Realistic Workload Simulation",
            100,  // fewer iterations since each one does 100 assertions
            10,   // fewer warmup iterations
            workload_fn
        );
        
        result.report();
    }
    
    /// Helper to simulate a more complex TypeAssertion for testing
    impl TypeAssertion {
        pub fn simple_for_testing() -> Self {
            // Simple interface -> struct assertion
            TypeAssertion {
                token: "token".to_string(),
                expression: Box::new(cursed::ast::expressions::Empty{}),
                type_name: "SimpleType".to_string(),
            }
        }
        
        pub fn nested_for_testing() -> Self {
            // Nested interface (interface A -> interface B -> struct) assertion
            TypeAssertion {
                token: "token".to_string(),
                expression: Box::new(cursed::ast::expressions::Empty{}),
                type_name: "NestedType".to_string(),
            }
        }
        
        pub fn diamond_for_testing() -> Self {
            // Diamond inheritance (interface A, B -> interface C -> struct) assertion
            TypeAssertion {
                token: "token".to_string(),
                expression: Box::new(cursed::ast::expressions::Empty{}),
                type_name: "DiamondType".to_string(),
            }
        }
        
        pub fn deep_nested_for_testing() -> Self {
            // Deep nested interface hierarchy
            TypeAssertion {
                token: "token".to_string(),
                expression: Box::new(cursed::ast::expressions::Empty{}),
                type_name: "DeepNestedType".to_string(),
            }
        }
    }
}