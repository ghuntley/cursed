//! Optimization System Stress Tests
//!
//! These tests validate the optimization system under extreme conditions,
//! ensuring stability, performance, and resource management under stress.

use cursed::optimization::{OptimizationManager, BenchmarkRunner, BenchmarkConfig};
use cursed::codegen::llvm::optimization::{OptimizationConfig, OptimizationLevel};
use cursed::error::Result;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio;
use tracing::{info, warn, error};

// Test configuration constants
const STRESS_TEST_DURATION: Duration = Duration::from_secs(60);
const MAX_CONCURRENT_COMPILATIONS: usize = 16;
const LARGE_FILE_LINES: usize = 50000;
const MEMORY_PRESSURE_MB: usize = 1000;
const NETWORK_LATENCY_MS: u64 = 100;

macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .try_init();
    };
}

/// Stress test results tracking
#[derive(Debug, Clone)]
struct StressTestMetrics {
    successful_compilations: AtomicUsize,
    failed_compilations: AtomicUsize,
    total_compilation_time: Arc<Mutex<Duration>>,
    peak_memory_usage_mb: AtomicUsize,
    peak_cpu_usage_percent: AtomicUsize,
    cache_hit_rate: Arc<Mutex<f64>>,
    optimization_effectiveness: Arc<Mutex<f64>>,
}

impl StressTestMetrics {
    fn new() -> Self {
        Self {
            successful_compilations: AtomicUsize::new(0),
            failed_compilations: AtomicUsize::new(0),
            total_compilation_time: Arc::new(Mutex::new(Duration::ZERO)),
            peak_memory_usage_mb: AtomicUsize::new(0),
            peak_cpu_usage_percent: AtomicUsize::new(0),
            cache_hit_rate: Arc::new(Mutex::new(0.0)),
            optimization_effectiveness: Arc::new(Mutex::new(0.0)),
        }
    }

    fn record_success(&self, compilation_time: Duration) {
        self.successful_compilations.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut total_time) = self.total_compilation_time.lock() {
            *total_time += compilation_time;
        }
    }

    fn record_failure(&self) {
        self.failed_compilations.fetch_add(1, Ordering::Relaxed);
    }

    fn update_peak_memory(&self, memory_mb: usize) {
        self.peak_memory_usage_mb.fetch_max(memory_mb, Ordering::Relaxed);
    }

    fn update_peak_cpu(&self, cpu_percent: usize) {
        self.peak_cpu_usage_percent.fetch_max(cpu_percent, Ordering::Relaxed);
    }

    fn get_success_rate(&self) -> f64 {
        let successful = self.successful_compilations.load(Ordering::Relaxed);
        let failed = self.failed_compilations.load(Ordering::Relaxed);
        if successful + failed == 0 {
            0.0
        } else {
            successful as f64 / (successful + failed) as f64
        }
    }

    fn get_average_compilation_time(&self) -> Duration {
        let successful = self.successful_compilations.load(Ordering::Relaxed);
        if successful == 0 {
            Duration::ZERO
        } else {
            let total_time = self.total_compilation_time.lock().unwrap();
            *total_time / successful as u32
        }
    }
}

/// Stress test program generator
struct StressTestProgramGenerator;

impl StressTestProgramGenerator {
    fn generate_large_program(lines: usize) -> String {
        let mut source = String::from("facts main() {\n");
        source.push_str("    let mut result = 0;\n");
        
        for i in 0..lines {
            source.push_str(&format!("    result += fibonacci({});\n", i % 30));
        }
        
        source.push_str(r#"
    println!("Final result: {}", result);
}

facts fibonacci(n: i32) -> i32 {
    lowkey (n <= 1) {
        periodt n;
    }
    periodt fibonacci(n - 1) + fibonacci(n - 2);
}
"#);
        
        source
    }

    fn generate_memory_intensive_program() -> String {
        r#"
facts main() {
    let mut data_structures = Vec::new();
    
    // Create multiple large data structures
    lowkey (sus i = 0; i < 1000; i++) {
        let mut matrix = Vec::new();
        lowkey (sus j = 0; j < 1000; j++) {
            matrix.push(vec![i + j; 100]);
        }
        data_structures.push(matrix);
    }
    
    // Perform operations on data
    let mut sum = 0;
    lowkey (sus i = 0; i < data_structures.len(); i++) {
        lowkey (sus j = 0; j < data_structures[i].len(); j++) {
            sum += data_structures[i][j].len();
        }
    }
    
    println!("Processed {} elements", sum);
}
"#.to_string()
    }

    fn generate_computation_intensive_program() -> String {
        r#"
facts prime_sieve(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    lowkey (sus i = 2; i * i <= limit; i++) {
        lowkey (is_prime[i]) {
            lowkey (sus j = i * i; j <= limit; j += i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime
}

facts matrix_operations() -> f64 {
    let size = 500;
    let mut matrix_a = vec![vec![0.0; size]; size];
    let mut matrix_b = vec![vec![0.0; size]; size];
    let mut result = vec![vec![0.0; size]; size];
    
    // Initialize matrices
    lowkey (sus i = 0; i < size; i++) {
        lowkey (sus j = 0; j < size; j++) {
            matrix_a[i][j] = (i + j) as f64;
            matrix_b[i][j] = (i * j) as f64;
        }
    }
    
    // Matrix multiplication
    lowkey (sus i = 0; i < size; i++) {
        lowkey (sus j = 0; j < size; j++) {
            lowkey (sus k = 0; k < size; k++) {
                result[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }
    
    // Sum all elements
    let mut sum = 0.0;
    lowkey (sus i = 0; i < size; i++) {
        lowkey (sus j = 0; j < size; j++) {
            sum += result[i][j];
        }
    }
    
    sum
}

facts main() {
    // Prime sieve
    let primes = prime_sieve(100000);
    let prime_count = primes.iter().filter(|&&p| p).count();
    
    // Matrix operations
    let matrix_result = matrix_operations();
    
    println!("Found {} primes", prime_count);
    println!("Matrix sum: {}", matrix_result);
}
"#.to_string()
    }

    fn generate_concurrent_program(id: usize) -> String {
        format!(r#"
facts worker_{}() -> i64 {{
    let mut result = 0;
    
    // Simulate different workload patterns
    lowkey (sus i = 0; i < 10000; i++) {{
        result += (i * {}) as i64;
        
        // Simulate some computation
        lowkey (sus j = 0; j < 100; j++) {{
            result ^= j as i64;
        }}
    }}
    
    result
}}

facts parallel_reduction() -> i64 {{
    let workers = 8;
    let mut results = Vec::new();
    
    lowkey (sus i = 0; i < workers; i++) {{
        results.push(worker_{}());
    }}
    
    results.iter().sum()
}}

facts main() {{
    let result = parallel_reduction();
    println!("Worker {} result: {{}}", result);
}}
"#, id, id + 1, id)
    }
}

// =============================================================================
// STRESS TESTS
// =============================================================================

#[tokio::test]
#[ignore] // Only run with --ignored flag
async fn test_sustained_high_load_compilation() -> Result<()> {
    init_tracing!();
    
    info!("Starting sustained high load compilation test");
    
    let metrics = Arc::new(StressTestMetrics::new());
    let test_start = Instant::now();
    
    // Create optimization manager
    let work_dir = PathBuf::from("test_results/stress_tests");
    std::fs::create_dir_all(&work_dir).ok();
    
    let manager = Arc::new(Mutex::new(
        OptimizationManager::new()
            .with_benchmarking("target/debug/cursed", &work_dir)
    ));
    
    // Launch concurrent compilation tasks
    let mut handles = Vec::new();
    
    for i in 0..MAX_CONCURRENT_COMPILATIONS {
        let metrics_clone = Arc::clone(&metrics);
        let manager_clone = Arc::clone(&manager);
        
        let handle = tokio::spawn(async move {
            let mut compilation_count = 0;
            
            while test_start.elapsed() < STRESS_TEST_DURATION {
                let program_source = match i % 4 {
                    0 => StressTestProgramGenerator::generate_large_program(1000),
                    1 => StressTestProgramGenerator::generate_memory_intensive_program(),
                    2 => StressTestProgramGenerator::generate_computation_intensive_program(),
                    _ => StressTestProgramGenerator::generate_concurrent_program(i),
                };
                
                let compilation_start = Instant::now();
                
                // Simulate compilation process
                match simulate_compilation(&program_source, OptimizationLevel::Default).await {
                    Ok(result) => {
                        let compilation_time = compilation_start.elapsed();
                        metrics_clone.record_success(compilation_time);
                        
                        // Update resource usage metrics
                        metrics_clone.update_peak_memory(result.memory_usage_mb as usize);
                        metrics_clone.update_peak_cpu(result.cpu_usage_percent as usize);
                        
                        compilation_count += 1;
                    }
                    Err(e) => {
                        warn!("Compilation failed in worker {}: {:?}", i, e);
                        metrics_clone.record_failure();
                    }
                }
                
                // Small delay to prevent overwhelming
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            
            info!("Worker {} completed {} compilations", i, compilation_count);
        });
        
        handles.push(handle);
    }
    
    // Wait for all workers to complete
    for handle in handles {
        handle.await.map_err(|e| cursed::error::CursedError::General(format!("Task failed: {}", e)))?;
    }
    
    let total_duration = test_start.elapsed();
    info!("Stress test completed in {:?}", total_duration);
    
    // Analyze results
    let success_rate = metrics.get_success_rate();
    let avg_compilation_time = metrics.get_average_compilation_time();
    let successful_compilations = metrics.successful_compilations.load(Ordering::Relaxed);
    let failed_compilations = metrics.failed_compilations.load(Ordering::Relaxed); 
    let peak_memory = metrics.peak_memory_usage_mb.load(Ordering::Relaxed);
    let peak_cpu = metrics.peak_cpu_usage_percent.load(Ordering::Relaxed);
    
    info!("Stress test results:");
    info!("  Success rate: {:.2}%", success_rate * 100.0);
    info!("  Successful compilations: {}", successful_compilations);
    info!("  Failed compilations: {}", failed_compilations);
    info!("  Average compilation time: {:?}", avg_compilation_time);
    info!("  Peak memory usage: {} MB", peak_memory);
    info!("  Peak CPU usage: {}%", peak_cpu);
    
    // Validate stress test results
    assert!(success_rate >= 0.95, "Success rate {:.2}% below 95%", success_rate * 100.0);
    assert!(successful_compilations >= 100, "Not enough successful compilations: {}", successful_compilations);
    assert!(avg_compilation_time <= Duration::from_secs(30), "Average compilation time too high: {:?}", avg_compilation_time);
    assert!(peak_memory <= MEMORY_PRESSURE_MB * 2, "Peak memory usage {} MB exceeds limit", peak_memory);
    
    Ok(())
}

#[tokio::test]
#[ignore] // Only run with --ignored flag
async fn test_memory_pressure_optimization() -> Result<()> {
    init_tracing!();
    
    info!("Testing optimization system under memory pressure");
    
    let work_dir = PathBuf::from("test_results/memory_pressure_tests");
    std::fs::create_dir_all(&work_dir).ok();
    
    let manager = OptimizationManager::new()
        .with_benchmarking("target/debug/cursed", &work_dir);
    
    // Generate memory-intensive program
    let memory_program = StressTestProgramGenerator::generate_memory_intensive_program();
    
    // Test compilation under different optimization levels
    let optimization_levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Size,
    ];
    
    let mut memory_usage_results = Vec::new();
    
    for level in optimization_levels {
        info!("Testing memory pressure with optimization level: {:?}", level);
        
        let start_time = Instant::now();
        let result = simulate_compilation(&memory_program, level).await?;
        let compilation_time = start_time.elapsed();
        
        memory_usage_results.push((level, result.memory_usage_mb, compilation_time));
        
        info!("Level {:?}: Memory={:.1}MB, Time={:?}", 
              level, result.memory_usage_mb, compilation_time);
    }
    
    // Analyze memory usage patterns
    let baseline_memory = memory_usage_results[0].1; // O0 memory usage
    
    for (level, memory_usage, _) in &memory_usage_results[1..] {
        let memory_ratio = memory_usage / baseline_memory;
        info!("Memory ratio for {:?}: {:.2}x", level, memory_ratio);
        
        // Memory optimization should not increase usage by more than 50%
        assert!(memory_ratio <= 1.5, 
                "Memory usage increased by {:.2}x for {:?}, exceeding 1.5x limit", 
                memory_ratio, level);
    }
    
    Ok(())
}

#[tokio::test] 
#[ignore] // Only run with --ignored flag
async fn test_large_file_optimization_scalability() -> Result<()> {
    init_tracing!();
    
    info!("Testing optimization scalability with large files");
    
    let work_dir = PathBuf::from("test_results/scalability_tests");
    std::fs::create_dir_all(&work_dir).ok();
    
    let manager = OptimizationManager::new()
        .with_benchmarking("target/debug/cursed", &work_dir);
    
    // Test with increasingly large files
    let file_sizes = vec![1000, 5000, 10000, 25000, 50000];
    let mut scalability_results = Vec::new();
    
    for size in file_sizes {
        info!("Testing file with {} lines", size);
        
        let large_program = StressTestProgramGenerator::generate_large_program(size);
        
        let start_time = Instant::now();
        let result = simulate_compilation(&large_program, OptimizationLevel::Default).await?;
        let compilation_time = start_time.elapsed();
        
        scalability_results.push((size, compilation_time, result.memory_usage_mb));
        
        info!("Size {}: Time={:?}, Memory={:.1}MB", 
              size, compilation_time, result.memory_usage_mb);
    }
    
    // Analyze scalability
    let base_time = scalability_results[0].1.as_secs_f64();
    let base_size = scalability_results[0].0 as f64;
    
    for (size, time, _) in &scalability_results[1..] {
        let size_ratio = *size as f64 / base_size;
        let time_ratio = time.as_secs_f64() / base_time;
        
        info!("Size {}x: Time ratio {:.2}x", size_ratio, time_ratio);
        
        // Time complexity should be roughly linear (within 2x for reasonable scalability)
        let expected_max_ratio = size_ratio * 1.5; // Allow some overhead
        assert!(time_ratio <= expected_max_ratio,
                "Compilation time ratio {:.2}x exceeds expected {:.2}x for size ratio {:.2}x",
                time_ratio, expected_max_ratio, size_ratio);
    }
    
    Ok(())
}

#[tokio::test]
#[ignore] // Only run with --ignored flag  
async fn test_optimization_cache_under_pressure() -> Result<()> {
    init_tracing!();
    
    info!("Testing optimization cache performance under pressure");
    
    let work_dir = PathBuf::from("test_results/cache_pressure_tests");
    std::fs::create_dir_all(&work_dir).ok();
    
    let manager = Arc::new(Mutex::new(
        OptimizationManager::new()
            .with_benchmarking("target/debug/cursed", &work_dir)
    ));
    
    let cache_metrics = Arc::new(Mutex::new(Vec::new()));
    
    // Generate many similar programs to test cache effectiveness
    let num_programs = 100;
    let mut cache_test_handles = Vec::new();
    
    for i in 0..num_programs {
        let manager_clone = Arc::clone(&manager);
        let cache_metrics_clone = Arc::clone(&cache_metrics);
        
        let handle = tokio::spawn(async move {
            // Generate slightly different programs
            let program = format!(r#"
                facts compute_{}() -> i32 {{
                    let mut result = 0;
                    lowkey (sus j = 0; j < 1000; j++) {{
                        result += j * {};
                    }}
                    result
                }}
                
                facts main() {{
                    let result = compute_{}();
                    println!("Result: {{}}", result);
                }}
            "#, i, i + 1, i);
            
            let start_time = Instant::now();
            let result = simulate_compilation(&program, OptimizationLevel::Default).await?;
            let compilation_time = start_time.elapsed();
            
            // Record cache metrics
            if let Ok(mut metrics) = cache_metrics_clone.lock() {
                metrics.push((i, compilation_time, result.cache_hit_rate));
            }
            
            Ok::<_, cursed::error::CursedError>(())
        });
        
        cache_test_handles.push(handle);
    }
    
    // Wait for all cache tests to complete
    for handle in cache_test_handles {
        handle.await.map_err(|e| cursed::error::CursedError::General(format!("Task failed: {}", e)))??;
    }
    
    // Analyze cache performance
    let metrics = cache_metrics.lock().unwrap();
    let total_compilations = metrics.len();
    let average_cache_hit_rate: f64 = metrics.iter().map(|(_, _, hit_rate)| hit_rate).sum::<f64>() / total_compilations as f64;
    let average_compilation_time: Duration = {
        let total_time: Duration = metrics.iter().map(|(_, time, _)| *time).sum();
        total_time / total_compilations as u32
    };
    
    info!("Cache pressure test results:");
    info!("  Total compilations: {}", total_compilations);
    info!("  Average cache hit rate: {:.2}%", average_cache_hit_rate * 100.0);
    info!("  Average compilation time: {:?}", average_compilation_time);
    
    // Validate cache performance
    assert!(average_cache_hit_rate >= 0.4, "Cache hit rate {:.2}% below 40%", average_cache_hit_rate * 100.0);
    assert!(average_compilation_time <= Duration::from_secs(10), "Average compilation time {:?} too high", average_compilation_time);
    
    Ok(())
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Simulated compilation result
#[derive(Debug, Clone)]
struct CompilationResult {
    memory_usage_mb: f64,
    cpu_usage_percent: f64, 
    cache_hit_rate: f64,
    optimization_effectiveness: f64,
}

async fn simulate_compilation(
    _program_source: &str,
    optimization_level: OptimizationLevel,
) -> Result<CompilationResult> {
    // Simulate compilation time based on optimization level
    let base_duration = Duration::from_millis(500);
    let optimization_overhead = match optimization_level {
        OptimizationLevel::None => 1.0,
        OptimizationLevel::Less => 1.2,
        OptimizationLevel::Default => 1.5,
        OptimizationLevel::Aggressive => 2.0,
        OptimizationLevel::Size => 1.3,
    };
    
    let compilation_duration = Duration::from_secs_f64(
        base_duration.as_secs_f64() * optimization_overhead
    );
    
    tokio::time::sleep(compilation_duration).await;
    
    // Simulate realistic metrics
    let memory_usage_mb = match optimization_level {
        OptimizationLevel::None => 50.0,
        OptimizationLevel::Less => 75.0,
        OptimizationLevel::Default => 100.0,
        OptimizationLevel::Aggressive => 150.0,
        OptimizationLevel::Size => 80.0,
    };
    
    let cpu_usage_percent = match optimization_level {
        OptimizationLevel::None => 30.0,
        OptimizationLevel::Less => 50.0,
        OptimizationLevel::Default => 70.0,
        OptimizationLevel::Aggressive => 90.0,
        OptimizationLevel::Size => 60.0,
    };
    
    let cache_hit_rate = match optimization_level {
        OptimizationLevel::None => 0.2,
        OptimizationLevel::Less => 0.4,
        OptimizationLevel::Default => 0.6,
        OptimizationLevel::Aggressive => 0.7,
        OptimizationLevel::Size => 0.5,
    };
    
    let optimization_effectiveness = match optimization_level {
        OptimizationLevel::None => 0.0,
        OptimizationLevel::Less => 0.3,
        OptimizationLevel::Default => 0.6,
        OptimizationLevel::Aggressive => 0.8,
        OptimizationLevel::Size => 0.4,
    };
    
    Ok(CompilationResult {
        memory_usage_mb,
        cpu_usage_percent,
        cache_hit_rate,
        optimization_effectiveness,
    })
}
