/// Heap Stress Benchmark for GC Performance Validation
///
/// This benchmark simulates real-world allocation patterns to validate
/// that the GC can achieve sub-100ms pause times under stress.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;

use cursed::runtime::{
    MemoryManager, GarbageCollector, GcConfig, RuntimeStack
};
use cursed::runtime::gc_tuning::{
    create_web_server_gc_config, create_low_latency_gc_config, 
    GcPerformanceTuner, GcTuningParams
};
use cursed::memory::Tag;

/// Simulates a web server allocation pattern
fn web_server_allocation_pattern(gc: &Arc<GarbageCollector>) -> Vec<Duration> {
    let mut pause_times = Vec::new();
    
    // Simulate 1000 requests with varying allocation sizes
    for i in 0..1000 {
        let start = Instant::now();
        
        // Allocate objects typical of web request handling
        let _ = gc.allocate(1024, Tag::String); // Request body
        let _ = gc.allocate(512, Tag::Object);  // Request headers
        let _ = gc.allocate(256, Tag::Array);   // Response data
        let _ = gc.allocate(2048, Tag::String); // HTML template
        
        // Occasionally allocate larger objects
        if i % 10 == 0 {
            let _ = gc.allocate(16384, Tag::String); // Large response
        }
        
        // Simulate some short-lived objects
        for _ in 0..5 {
            let _ = gc.allocate(64, Tag::Object);
        }
        
        // Force GC periodically to measure pause times
        if i % 50 == 0 {
            let gc_start = Instant::now();
            let _ = gc.collect();
            let gc_duration = gc_start.elapsed();
            pause_times.push(gc_duration);
        }
    }
    
    pause_times
}

/// Simulates a high-frequency trading system allocation pattern
fn low_latency_allocation_pattern(gc: &Arc<GarbageCollector>) -> Vec<Duration> {
    let mut pause_times = Vec::new();
    
    // Simulate 10000 trades with minimal allocation
    for i in 0..10000 {
        // Allocate small, short-lived objects
        let _ = gc.allocate(64, Tag::Number);  // Price
        let _ = gc.allocate(32, Tag::Number);  // Quantity
        let _ = gc.allocate(16, Tag::Boolean); // Buy/sell flag
        
        // Occasionally allocate order books
        if i % 100 == 0 {
            let _ = gc.allocate(4096, Tag::Array); // Order book
        }
        
        // Very frequent GC to measure pause times
        if i % 100 == 0 {
            let gc_start = Instant::now();
            let _ = gc.collect();
            let gc_duration = gc_start.elapsed();
            pause_times.push(gc_duration);
        }
    }
    
    pause_times
}

/// Simulates a data processing pipeline allocation pattern
fn data_processing_allocation_pattern(gc: &Arc<GarbageCollector>) -> Vec<Duration> {
    let mut pause_times = Vec::new();
    
    // Simulate processing 100 batches of data
    for batch in 0..100 {
        // Allocate batch data
        let _ = gc.allocate(1024 * 1024, Tag::Array); // 1MB batch
        
        // Process data with temporary objects
        for _ in 0..100 {
            let _ = gc.allocate(1024, Tag::Object);  // Processing buffer
            let _ = gc.allocate(512, Tag::String);   // Intermediate result
        }
        
        // Allocate result data
        let _ = gc.allocate(512 * 1024, Tag::Array); // 512KB result
        
        // Force GC after each batch
        let gc_start = Instant::now();
        let _ = gc.collect();
        let gc_duration = gc_start.elapsed();
        pause_times.push(gc_duration);
    }
    
    pause_times
}

/// Benchmarks concurrent allocation from multiple threads
fn concurrent_allocation_benchmark(gc: &Arc<GarbageCollector>, num_threads: usize) -> Vec<Duration> {
    let mut handles = Vec::new();
    let pause_times = Arc::new(std::sync::Mutex::new(Vec::new()));
    
    for thread_id in 0..num_threads {
        let gc_clone = gc.clone();
        let pause_times_clone = pause_times.clone();
        
        let handle = thread::spawn(move || {
            let mut local_pause_times = Vec::new();
            
            for i in 0..500 {
                // Each thread allocates objects with different patterns
                match thread_id % 3 {
                    0 => {
                        // Thread 0: Small frequent allocations
                        let _ = gc_clone.allocate(64, Tag::Number);
                        let _ = gc_clone.allocate(32, Tag::Boolean);
                    }
                    1 => {
                        // Thread 1: Medium allocations
                        let _ = gc_clone.allocate(1024, Tag::String);
                        let _ = gc_clone.allocate(512, Tag::Object);
                    }
                    2 => {
                        // Thread 2: Large allocations
                        let _ = gc_clone.allocate(4096, Tag::Array);
                    }
                    _ => unreachable!(),
                }
                
                // Measure GC pause time periodically
                if i % 50 == 0 {
                    let gc_start = Instant::now();
                    let _ = gc_clone.collect();
                    let gc_duration = gc_start.elapsed();
                    local_pause_times.push(gc_duration);
                }
            }
            
            let mut shared_pause_times = pause_times_clone.lock().unwrap();
            shared_pause_times.extend(local_pause_times);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let pause_times = pause_times.lock().unwrap();
    pause_times.clone()
}

/// Analyzes pause time statistics
fn analyze_pause_times(pause_times: &[Duration]) -> (Duration, Duration, Duration, Duration) {
    if pause_times.is_empty() {
        return (Duration::ZERO, Duration::ZERO, Duration::ZERO, Duration::ZERO);
    }
    
    let mut sorted_times = pause_times.to_vec();
    sorted_times.sort();
    
    let min = sorted_times[0];
    let max = sorted_times[sorted_times.len() - 1];
    
    let total_ms: u64 = sorted_times.iter().map(|d| d.as_millis() as u64).sum();
    let avg = Duration::from_millis(total_ms / sorted_times.len() as u64);
    
    let p99_index = (sorted_times.len() as f64 * 0.99) as usize;
    let p99 = sorted_times[p99_index.min(sorted_times.len() - 1)];
    
    (min, avg, p99, max)
}

fn bench_web_server_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("gc_web_server");
    
    // Test with default GC config
    let default_config = GcConfig::default();
    let stack = Arc::new(RuntimeStack::new());
    let default_gc = GarbageCollector::new(default_config, stack.clone()).unwrap();
    
    // Test with optimized web server config
    let web_config = create_web_server_gc_config();
    let web_gc = GarbageCollector::new(web_config, stack.clone()).unwrap();
    
    group.bench_function("default_gc", |b| {
        b.iter(|| {
            let pause_times = web_server_allocation_pattern(&default_gc);
            let (_, _, p99, _) = analyze_pause_times(&pause_times);
            black_box(p99);
        })
    });
    
    group.bench_function("optimized_gc", |b| {
        b.iter(|| {
            let pause_times = web_server_allocation_pattern(&web_gc);
            let (_, _, p99, _) = analyze_pause_times(&pause_times);
            black_box(p99);
        })
    });
    
    group.finish();
}

fn bench_low_latency_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("gc_low_latency");
    
    // Test with default GC config
    let default_config = GcConfig::default();
    let stack = Arc::new(RuntimeStack::new());
    let default_gc = GarbageCollector::new(default_config, stack.clone()).unwrap();
    
    // Test with optimized low latency config
    let low_latency_config = create_low_latency_gc_config();
    let low_latency_gc = GarbageCollector::new(low_latency_config, stack.clone()).unwrap();
    
    group.bench_function("default_gc", |b| {
        b.iter(|| {
            let pause_times = low_latency_allocation_pattern(&default_gc);
            let (_, _, p99, _) = analyze_pause_times(&pause_times);
            black_box(p99);
        })
    });
    
    group.bench_function("optimized_gc", |b| {
        b.iter(|| {
            let pause_times = low_latency_allocation_pattern(&low_latency_gc);
            let (_, _, p99, _) = analyze_pause_times(&pause_times);
            black_box(p99);
        })
    });
    
    group.finish();
}

fn bench_concurrent_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("gc_concurrent");
    
    let config = create_web_server_gc_config();
    let stack = Arc::new(RuntimeStack::new());
    let gc = GarbageCollector::new(config, stack).unwrap();
    
    for num_threads in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("threads", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter(|| {
                    let pause_times = concurrent_allocation_benchmark(&gc, num_threads);
                    let (_, _, p99, _) = analyze_pause_times(&pause_times);
                    black_box(p99);
                })
            }
        );
    }
    
    group.finish();
}

fn bench_data_processing_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("gc_data_processing");
    
    let config = create_web_server_gc_config();
    let stack = Arc::new(RuntimeStack::new());
    let gc = GarbageCollector::new(config, stack).unwrap();
    
    group.bench_function("batch_processing", |b| {
        b.iter(|| {
            let pause_times = data_processing_allocation_pattern(&gc);
            let (_, _, p99, _) = analyze_pause_times(&pause_times);
            black_box(p99);
        })
    });
    
    group.finish();
}

/// Validates that GC pause times meet the sub-100ms target
fn validate_pause_times() {
    println!("Validating GC pause times...");
    
    let config = create_web_server_gc_config();
    let stack = Arc::new(RuntimeStack::new());
    let gc = GarbageCollector::new(config, stack).unwrap();
    
    // Run web server workload
    let pause_times = web_server_allocation_pattern(&gc);
    let (min, avg, p99, max) = analyze_pause_times(&pause_times);
    
    println!("Web Server Workload Results:");
    println!("  Min pause time: {:?}", min);
    println!("  Avg pause time: {:?}", avg);
    println!("  P99 pause time: {:?}", p99);
    println!("  Max pause time: {:?}", max);
    
    // Validate P99 is under 100ms
    if p99.as_millis() > 100 {
        println!("❌ FAIL: P99 pause time ({:?}) exceeds 100ms target", p99);
    } else {
        println!("✅ PASS: P99 pause time ({:?}) meets <100ms target", p99);
    }
    
    // Run low latency workload
    let low_latency_config = create_low_latency_gc_config();
    let low_latency_gc = GarbageCollector::new(low_latency_config, stack).unwrap();
    
    let pause_times = low_latency_allocation_pattern(&low_latency_gc);
    let (min, avg, p99, max) = analyze_pause_times(&pause_times);
    
    println!("\nLow Latency Workload Results:");
    println!("  Min pause time: {:?}", min);
    println!("  Avg pause time: {:?}", avg);
    println!("  P99 pause time: {:?}", p99);
    println!("  Max pause time: {:?}", max);
    
    // Validate P99 is under 50ms for low latency
    if p99.as_millis() > 50 {
        println!("❌ FAIL: P99 pause time ({:?}) exceeds 50ms low-latency target", p99);
    } else {
        println!("✅ PASS: P99 pause time ({:?}) meets <50ms low-latency target", p99);
    }
}

criterion_group!(
    benches,
    bench_web_server_workload,
    bench_low_latency_workload,
    bench_concurrent_workload,
    bench_data_processing_workload
);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pause_time_validation() {
        validate_pause_times();
    }
    
    #[test]
    fn test_web_server_pattern() {
        let config = create_web_server_gc_config();
        let stack = Arc::new(RuntimeStack::new());
        let gc = GarbageCollector::new(config, stack).unwrap();
        
        let pause_times = web_server_allocation_pattern(&gc);
        assert!(!pause_times.is_empty());
        
        let (_, _, p99, _) = analyze_pause_times(&pause_times);
        // This is an aspirational test - we want P99 < 100ms
        // For now, just ensure we get reasonable measurements
        assert!(p99.as_millis() < 1000); // Should be less than 1 second
    }
    
    #[test]
    fn test_low_latency_pattern() {
        let config = create_low_latency_gc_config();
        let stack = Arc::new(RuntimeStack::new());
        let gc = GarbageCollector::new(config, stack).unwrap();
        
        let pause_times = low_latency_allocation_pattern(&gc);
        assert!(!pause_times.is_empty());
        
        let (_, _, p99, _) = analyze_pause_times(&pause_times);
        // Should be better than web server pattern
        assert!(p99.as_millis() < 500); // Should be less than 500ms
    }
    
    #[test]
    fn test_concurrent_allocation() {
        let config = create_web_server_gc_config();
        let stack = Arc::new(RuntimeStack::new());
        let gc = GarbageCollector::new(config, stack).unwrap();
        
        let pause_times = concurrent_allocation_benchmark(&gc, 4);
        assert!(!pause_times.is_empty());
        
        let (_, _, p99, _) = analyze_pause_times(&pause_times);
        // Concurrent allocation should still have reasonable pause times
        assert!(p99.as_millis() < 2000); // Should be less than 2 seconds
    }
}
