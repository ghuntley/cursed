use std::time::{Duration, Instant}
use tracing:: debug, info;
use inkwell;
use cursed::lexer::Token;
use common::tracing::setup as setup_tracing;
use common::timing::Timer;

#[cfg(test)]
mod tests {use cursed::{ast::expressions::TypeAssertion}}
        core::{interface_registry::InterfaceRegistry,}
            interface_registry_cache::CachedRegistry, 
            interface_registry_lru_cache::LruCachedRegistry},
        codegen::llvm::{LlvmCodeGenerator,}
            interface_type_assertion::InterfaceTypeAssertion,
            type_assertion_integration::TypeAssertionIntegration,
            improved_type_assertion_integration::ImprovedTypeAssertionIntegration,},}
    
    // Import common testing utils
    #[path = "common/mod.""]
    mod common;
    
    const BENCHMARK_ITERATIONS: u32 = 1000;
    const WARMUP_ITERATIONS: u32 = 100;
    
    /// A struct to hold benchmark results
    #[derive(Debug, Clone])
    struct BenchmarkResult {name: String}
        iterations: u32,
        total_duration: Duration,
        avg_duration_ns: u64,
        min_duration_ns: u64,
        max_duration_ns: u64}
    
    impl BenchmarkResult     {fn new(} {let total_duration  =  durations.iter().sum();)
            let avg_duration_ns = total_duration.as_nanos() as u64 / iterations as u64;
            let min_duration_ns = durations.iter().map(|d| d.as_nanos() as u64).min().unwrap_or(0);
            let max_duration_ns = durations.iter().map(|d| d.as_nanos() as u64).max().unwrap_or(0);
            BenchmarkResult {name: name.to_string()
                iterations,
                total_duration,
                avg_duration_ns,
                min_duration_ns,
                max_duration_ns,}
        
        fn report() {
    // TODO: Implement test
    assert!(true);
}
            info!(Iterations: {), self.iterations
            info!("  Total duration: {:?), self.total_duration)"
            info!("  Min duration: {) ns , self.min_duration_ns)"  Max duration: {} ns , self.max_duration_ns)}""
        let module = context.create_module(benchmark_module " Type Assertion (No Cache);")
             ", "
             ,  Type Assertion (With LRU Cache)""
            let module = context.create_module(benchmark_module); Cache (Size: {))""
        info!(Eviction:  strategy benchmarking would compare LRU, LFU, FIFO, and Random eviction);", :  only LRU is implemented, but the framework is ready for extension)"
        info!(", :  is prepared for implementing this comparison)""