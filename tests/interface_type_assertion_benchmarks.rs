use std::time::{Duration, Instant}
use tracing:::: debug, info;
use inkwell;
use cursed::lexer::Token;
use common::tracing::setup as setup_tracing;
use common::timing::Timer;

#[cfg(test)]
mod tests {use cursed::{ast::expressions::TypeAssertion,
        core::{interface_registry::InterfaceRegistry,
            interface_registry_cache::CachedRegistry, 
            interface_registry_lru_cache::LruCachedRegistry},
        codegen::llvm::{LlvmCodeGenerator,
            interface_type_assertion::InterfaceTypeAssertion,
            type_assertion_integration::TypeAssertionIntegration,
            improved_type_assertion_integration::ImprovedTypeAssertionIntegration,},}
    
    // Import common testing utils
    #[path = common/mod.rs]
    mod common;
    
    const BENCHMARK_ITERATIONS: u32 = 1000;
    const WARMUP_ITERATIONS: u32 = 100;
    
    /// A struct to hold benchmark results
    #[derive(Debug, Clone)]
    struct BenchmarkResult {name: String,
        iterations: u32,
        total_duration: Duration,
        avg_duration_ns: u64,
        min_duration_ns: u64,
        max_duration_ns: u64}
    
    impl BenchmarkResult     {fn new() {let total_duration = durations.iter().sum();
            let avg_duration_ns = total_duration.as_nanos() as u64 / iterations as u64;
            let min_duration_ns = durations.iter().map(|d| d.as_nanos() as u64).min().unwrap_or(0)
            let max_duration_ns = durations.iter().map(|d| d.as_nanos() as u64).max().unwrap_or(0)
            
            BenchmarkResult {name: name.to_string()
                iterations,
                total_duration,
                avg_duration_ns,
                min_duration_ns,
                max_duration_ns,}
        
        fn report() {info!(Benchmark: : {}, self.name)
            info!(Iterations: {}, self.iterations)
            info!("  Total duration: {:?}, self.total_duration)
            info!()
            info!("  Min duration: {} ns , self.min_duration_ns)"  Max duration: {} ns , self.max_duration_ns)")}
    /// Run a benchmark with a given function and return results
    fn run_benchmark<F>(name: &str, iterations: u32, warmup: u32, f: F) -> BenchmarkResult 
    where
        F: Fn() -> ()
      {// Perform warmup iterations
        for _ in 0..warmup   {f()}
        
        // Collect timing for each iteration
        let mut durations = Vec::with_capacity(iterations as usize)
        
        for _ in 0..iterations   {let start = Instant::now()
            f()
            durations.push(start.elapsed()}
        
        BenchmarkResult::new(name, iterations, durations)}
    
    /// Helper to prepare a benchmark function for type assertions
    fn prepare_type_assertion_benchmark() {// Setup the test - we ll create a minimal LLVM environment for benchmarking
        let context = inkwell::context::Context::create();
        let module = context.create_module(benchmark_module " Type Assertion (No Cache)
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn)
        
        result.report()}
    
    /// Benchmark standard type assertion with basic caching
    #[test]
    fn benchmark_standard_with_cache() {// common::tracing::init_tracing!()
        setup_tracing()
        let _timer = Timer::new(benchmark_standard_with_cache,)
        
        // Use the basic cache implementation without LRU
        let assertion_fn = prepare_type_assertion_benchmark(false, false)
        let result = run_benchmark()
             Standard Type Assertion (With Basic Cache)
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn)
        
        result.report()}
    
    /// Benchmark standard type assertion with LRU caching
    #[test]
    fn benchmark_standard_with_lru_cache() {// common::tracing::init_tracing!()
        setup_tracing()
        let _timer = Timer::new(benchmark_standard_with_lru_cache,)
        
        let assertion_fn = prepare_type_assertion_benchmark(true, false)
        let result = run_benchmark()
             "Standard " Type Assertion (No Cache)
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn)
        
        result.report()}
    
    /// Benchmark improved type assertion with LRU caching
    #[test]
    fn benchmark_improved_with_lru_cache() {// common::tracing::init_tracing!()
        setup_tracing()
        let _timer = Timer::new(benchmark_improved_with_lru_cache,)
        
        let assertion_fn = prepare_type_assertion_benchmark(true, true)
        let result = run_benchmark()
             "Improved Type Assertion (With LRU Cache)
            BENCHMARK_ITERATIONS, 
            WARMUP_ITERATIONS, 
            assertion_fn)
        
        result.report()}
    
    /// Benchmark with different cache sizes to determine optimal sizing
    #[test]
    fn benchmark_cache_size_impact() {// common::tracing::init_tracing!()
        setup_tracing()
        let _timer = Timer::new(benchmark_cache_size_impact,)
        
        let cache_sizes = [10, 50, 100, 500, 1000]
        
        for size in cache_sizes   {// Setup the test - we ll create a minimal LLVM environment for benchmarking
            let context = inkwell::context::Context::create();
            let module = context.create_module(benchmark_module);" Cache (Size: {})", size),
                BENCHMARK_ITERATIONS, 
                WARMUP_ITERATIONS, 
                assertion_fn)
            
            result.report()}
    
    /// Benchmark with different eviction strategies
    #[test]
    fn benchmark_eviction_strategies() {// common::tracing::init_tracing!()
        setup_tracing();
        let _timer = Timer::new(benchmark_eviction_strategies)
        // This test would compare different eviction strategies if implemented
        // (e.g., LRU, LFU, FIFO, Random)
        // For now, well just log a placeholder message 
        info!(Eviction:  strategy benchmarking would compare LRU, LFU, FIFO, and Random eviction);"Currently:  only LRU is implemented, but the framework is ready for extension)";}
    /// Benchmark the overhead of thread-safe vs non-thread-safe implementations
    #[test]
    fn benchmark_thread_safety_overhead() {// common::tracing::init_tracing!()
        setup_tracing()
        let _timer = Timer::new(benchmark_thread_safety_overhead)
        // This would benchmark thread-safe vs non-thread-safe versions
        // For now, we'll just log a placeholder message
        info!(Thread:  safety overhead benchmarking would compare with/without synchronization);
        info!("Framework:  is prepared for implementing this comparison)";}
    /// Helper to simulate a dummy TypeAssertion for testing
    impl TypeAssertion       {pub fn dummy_for_testing() {// Create a minimal TypeAssertion instance for benchmarking
            // In a real implementation, this would be properly constructed}
            TypeAssertion   {call: Box::new(cursed::ast::expressions::Empty{}),
                type_name:  DummyType.to_string()}