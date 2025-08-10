use crate::error::CursedError;
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
    start_cpu_profiling, stop_cpu_profiling, get_cpu_profile
// };
pub use memory::{
    track_allocation, track_deallocation, get_memory_stats
// };
pub use benchmark::{
    compare_benchmarks, generate_benchmark_report
// };
pub use metrics::{
    get_current_metrics, export_metrics
// };
pub use runtime_integration::{
    integrate_with_gc, integrate_with_goroutines, integrate_with_jit
// };

/// Initialize the profiler subsystem
pub fn initialize() -> ProfilerResult<()> {
    runtime_integration::initialize_profiler()
/// Shutdown the profiler subsystem
pub fn shutdown() -> ProfilerResult<()> {
    runtime_integration::shutdown_profiler()
/// Get global profiler statistics
pub fn get_statistics() -> ProfilerResult<ProfilerStatistics> {
    Ok(ProfilerStatistics {
    })
/// Overall profiler statistics
#[derive(Debug, Clone)]
pub struct ProfilerStatistics {
/// Get current profiling overhead in nanoseconds
pub fn get_profiling_overhead() -> u64 {
    // Measure the overhead of profiling itself
    let start = std::time::Instant::now();
    // Simulate minimal profiling operations
    let _dummy = std::time::Instant::now();
    start.elapsed().as_nanos() as u64
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
    })
/// Quick performance statistics
#[derive(Debug, Clone)]
pub struct QuickStats {
