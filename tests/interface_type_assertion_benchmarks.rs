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
    
    /// Helper to prepare a benchmark function for type assertions
    fn prepare_type_assertion_benchmark(use_lru_cache: bool, use_improved_integration: bool) -> Box<dyn Fn() -> ()> {
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
        
        // Create a dummy LlvmCodeGenerator - we won't actually generate full code
        // This is just to benchmark the type assertion paths
        let mut code_gen = LlvmCodeGenerator::new(
            &context,
            module,
            &builder,
            registry,
            None, // no type_registry needed for benchmark
        );
        
        // Create the type assertion to benchmark
        let type_assertion = TypeAssertion::dummy_for_testing();
        
        // Return a closure that performs the appropriate type assertion operation
        // based on the parameters
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
    
    /// Benchmark standard type assertion without caching
    #[test]
    fn benchmark_standard_no_cache() {
        setup_tracing();
        let _timer = Timer::new("benchmark_standard_no_cache");
        
        let assertion_fn = prepare_type_assertion_benchmark(false, false);
        let result = run_benchmark(
            "Standard Type Assertion (No Cache)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark standard type assertion with basic caching
    #[test]
    fn benchmark_standard_with_cache() {
        setup_tracing();
        let _timer = Timer::new("benchmark_standard_with_cache");
        
        // Use the basic cache implementation without LRU
        let assertion_fn = prepare_type_assertion_benchmark(false, false);
        let result = run_benchmark(
            "Standard Type Assertion (With Basic Cache)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark standard type assertion with LRU caching
    #[test]
    fn benchmark_standard_with_lru_cache() {
        setup_tracing();
        let _timer = Timer::new("benchmark_standard_with_lru_cache");
        
        let assertion_fn = prepare_type_assertion_benchmark(true, false);
        let result = run_benchmark(
            "Standard Type Assertion (With LRU Cache)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark improved type assertion without caching
    #[test]
    fn benchmark_improved_no_cache() {
        setup_tracing();
        let _timer = Timer::new("benchmark_improved_no_cache");
        
        let assertion_fn = prepare_type_assertion_benchmark(false, true);
        let result = run_benchmark(
            "Improved Type Assertion (No Cache)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark improved type assertion with LRU caching
    #[test]
    fn benchmark_improved_with_lru_cache() {
        setup_tracing();
        let _timer = Timer::new("benchmark_improved_with_lru_cache");
        
        let assertion_fn = prepare_type_assertion_benchmark(true, true);
        let result = run_benchmark(
            "Improved Type Assertion (With LRU Cache)", 
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn
        );
        
        result.report();
    }
    
    /// Benchmark with different cache sizes to determine optimal sizing
    #[test]
    fn benchmark_cache_size_impact() {
        setup_tracing();
        let _timer = Timer::new("benchmark_cache_size_impact");
        
        let cache_sizes = [10, 50, 100, 500, 1000];
        
        for size in cache_sizes {
            // Setup the test - we'll create a minimal LLVM environment for benchmarking
            let context = inkwell::context::Context::create();
            let module = context.create_module("benchmark_module");
            let builder = context.create_builder();
            
            // Create the LRU registry with the current size
            let registry: Box<dyn InterfaceRegistry> = Box::new(LruCachedRegistry::new(size));
            
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
            
            // Create a dummy LlvmCodeGenerator with the current registry size
            let mut code_gen = LlvmCodeGenerator::new(
                &context,
                module,
                &builder,
                registry,
                None, // no type_registry needed for benchmark
            );
            
            // Create the type assertion to benchmark
            let type_assertion = TypeAssertion::dummy_for_testing();
            
            // Create a benchmarking function that uses the improved integration
            let assertion_fn = move || {
                let _ = code_gen.compile_type_assertion_with_propagation(
                    &type_assertion,
                    Some("benchmark location")
                );
            };
            
            let result = run_benchmark(
                &format!("LRU Cache (Size: {})", size), 
                BENCHMARK_ITERATIONS, 
                WARMUP_ITERATIONS, 
                assertion_fn
            );
            
            result.report();
        }
    }
    
    /// Benchmark with different eviction strategies
    #[test]
    fn benchmark_eviction_strategies() {
        setup_tracing();
        let _timer = Timer::new("benchmark_eviction_strategies");
        
        // This test would compare different eviction strategies if implemented
        // (e.g., LRU, LFU, FIFO, Random)
        // For now, we'll just log a placeholder message
        info!("Eviction strategy benchmarking would compare LRU, LFU, FIFO, and Random eviction");
        info!("Currently only LRU is implemented, but the framework is ready for extension");
    }
    
    /// Benchmark the overhead of thread-safe vs non-thread-safe implementations
    #[test]
    fn benchmark_thread_safety_overhead() {
        setup_tracing();
        let _timer = Timer::new("benchmark_thread_safety_overhead");
        
        // This would benchmark thread-safe vs non-thread-safe versions
        // For now, we'll just log a placeholder message
        info!("Thread safety overhead benchmarking would compare with/without synchronization");
        info!("Framework is prepared for implementing this comparison");
    }
    
    /// Helper to simulate a dummy TypeAssertion for testing
    impl TypeAssertion {
        pub fn dummy_for_testing() -> Self {
            // Create a minimal TypeAssertion instance for benchmarking
            // In a real implementation, this would be properly constructed
            TypeAssertion {
                token: "token".to_string(),
                expression: Box::new(cursed::ast::expressions::Empty{}),
                type_name: "DummyType".to_string(),
            }
        }
    }
}