use crate::error::Error;
/// Performance monitoring and profiling tools for CURSED
pub mod error;
pub mod cpu;
pub mod memory;
pub mod benchmark;
pub mod metrics;
pub mod runtime_integration;

// Re-export all public types and functions
pub use error::{ProfilerError, ProfilerResult};
pub use cpu::{
    CpuProfiler, CpuProfile, CpuSample, FunctionProfile, CallGraph, 
    ProfileData, SamplingConfig, ProfilerConfig,
    start_cpu_profiling, stop_cpu_profiling, get_cpu_profile
};
pub use memory::{
    MemoryProfiler, MemoryProfile, AllocationProfile, AllocationSite,
    MemoryStats, HeapProfile, GcProfile, MemoryTracker,
    start_memory_profiling, stop_memory_profiling, get_memory_profile,
    track_allocation, track_deallocation, get_memory_stats
};
pub use benchmark::{
    Benchmark, BenchmarkResult, BenchmarkSuite, BenchmarkConfig,
    BenchmarkRunner, BenchmarkReport, ComparisonResult,
    benchmark_function, benchmark_with_setup, run_benchmark_suite,
    compare_benchmarks, generate_benchmark_report
};
pub use metrics::{
    PerformanceMetrics, MetricsCollector, MetricType, MetricValue,
    CounterMetric, GaugeMetric, HistogramMetric, TimerMetric,
    collect_metrics, start_metrics_collection, stop_metrics_collection,
    get_current_metrics, export_metrics
};
pub use runtime_integration::{
    ProfilerRuntime, RuntimeProfiler, IntegrationConfig,
    initialize_profiler, shutdown_profiler, get_profiler_runtime,
    integrate_with_gc, integrate_with_goroutines, integrate_with_jit
};

/// Initialize the profiler subsystem
pub fn initialize() -> ProfilerResult<()> {
    runtime_integration::initialize_profiler()
}

/// Shutdown the profiler subsystem
pub fn shutdown() -> ProfilerResult<()> {
    runtime_integration::shutdown_profiler()
}

/// Get global profiler statistics
pub fn get_statistics() -> ProfilerResult<ProfilerStatistics> {
    Ok(ProfilerStatistics {
        cpu_profiles_created: cpu::get_profile_count(),
        memory_profiles_created: memory::get_profile_count(),
        benchmarks_run: benchmark::get_benchmark_count(),
        metrics_collected: metrics::get_metrics_count(),
        total_samples: cpu::get_total_samples() + memory::get_total_samples(),
        profiling_overhead_ns: get_profiling_overhead(),
    })
}

/// Overall profiler statistics
#[derive(Debug, Clone)]
pub struct ProfilerStatistics {
    pub cpu_profiles_created: u64,
    pub memory_profiles_created: u64,
    pub benchmarks_run: u64,
    pub metrics_collected: u64,
    pub total_samples: u64,
    pub profiling_overhead_ns: u64,
}

/// Get current profiling overhead in nanoseconds
pub fn get_profiling_overhead() -> u64 {
    // Measure the overhead of profiling itself
    let start = std::time::Instant::now();
    // Simulate minimal profiling operations
    let _dummy = std::time::Instant::now();
    start.elapsed().as_nanos() as u64
}

/// Quick performance check - returns basic system performance metrics
pub fn quick_performance_check() -> ProfilerResult<QuickStats> {
    let start = std::time::Instant::now();
    
    // CPU performance test
    let cpu_start = std::time::Instant::now();
    let mut sum = 0i64;
    for i in 0..1000000 {
        sum += i;
    }
    let cpu_time = cpu_start.elapsed();
    
    // Memory allocation test
    let mem_start = std::time::Instant::now();
    let _vec: Vec<i32> = (0..10000).collect();
    let mem_time = mem_start.elapsed();
    
    let total_time = start.elapsed();
    
    Ok(QuickStats {
        total_time_ns: total_time.as_nanos() as u64,
        cpu_performance_ns: cpu_time.as_nanos() as u64,
        memory_performance_ns: mem_time.as_nanos() as u64,
        dummy_computation_result: sum,
    })
}

/// Quick performance statistics
#[derive(Debug, Clone)]
pub struct QuickStats {
    pub total_time_ns: u64,
    pub cpu_performance_ns: u64,
    pub memory_performance_ns: u64,
    pub dummy_computation_result: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_initialization() {
        let result = initialize();
        assert!(result.is_ok());
        
        let stats = get_statistics();
        assert!(stats.is_ok());
        
        let shutdown_result = shutdown();
        assert!(shutdown_result.is_ok());
    }

    #[test]
    fn test_quick_performance_check() {
        let stats = quick_performance_check();
        assert!(stats.is_ok());
        
        let stats = stats.unwrap();
        assert!(stats.total_time_ns > 0);
        assert!(stats.cpu_performance_ns > 0);
        assert!(stats.memory_performance_ns > 0);
    }

    #[test]
    fn test_profiling_overhead() {
        let overhead = get_profiling_overhead();
        assert!(overhead > 0);
        // Overhead should be reasonable (less than 1ms)
        assert!(overhead < 1_000_000);
    }
}
