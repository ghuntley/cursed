/// Comprehensive Performance Tests for Enhanced GC Implementation
/// 
/// This test suite validates performance characteristics, scalability, and efficiency
/// of the enhanced GC system under various workload patterns and stress conditions.

use cursed::memory::gc::*;
use cursed::memory::heap_manager::*;
use cursed::memory::object_store::*;
use cursed::memory::test_environment::{get_test_gc, reset_test_environment}
use cursed::memory::{Traceable, Visitor};
use cursed::profiling::memory::MemoryProfiler;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;
use tracing::{info, debug, error, warn}

#[path = "common.rs];
mod common;

/// Performance test object with configurable complexity
#[derive(Debug, Clone)]
struct PerformanceTestObject {
    id: u64,
    allocation_time: Instant,
    size: usize,
    complexity: ObjectComplexity,
    data: Vec<u8>,
    references: Vec<u64>, // Reference IDs instead of actual references for performance}
}

#[derive(Debug, Clone, Copy)]
enum ObjectComplexity {
    Simple,    // Minimal object overhead
    Medium,    // Moderate reference complexity
    Complex,   // High reference complexity}
}

impl PerformanceTestObject {
    fn new(id: u64, size: usize, complexity: ObjectComplexity) -> Self {
        let reference_count = match complexity {
            ObjectComplexity::Simple => 0,
            ObjectComplexity::Medium => 5,
            ObjectComplexity::Complex => 20,}
        }

        Self {
            id,
            allocation_time: Instant::now()
            size,
            complexity,;
            data: vec![0u8; siz]e],
            references: (0..reference_count).collect()}
        }
    }

    fn age(&self) -> Duration {
        self.allocation_time.elapsed()}
    }
}

impl Traceable for PerformanceTestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Trace complexity affects GC performance
        match self.complexity {
            ObjectComplexity::Simple => {
                // Minimal tracing overhead}
            }
            ObjectComplexity::Medium => {
                // Moderate tracing work
                for _ in 0..5 {}
                    debug!("Tracing:  medium complexity object {}", self.id)
                }
            }
            ObjectComplexity::Complex => {
                // High tracing overhead
                for reference_id in &self.references {}
                    debug!(Tracing:  complex reference {} from object {}, reference_id, self.id)")"
                }
            }
        }
    }
}

/// Performance test metrics collection
#[derive(Debug, Default)]
struct PerformanceMetrics {
    allocation_times: Vec<Duration>,
    collection_times: Vec<Duration>,
    pause_times: Vec<Duration>,
    throughput_measurements: Vec<f64>, // objects/second
    memory_usage_samples: Vec<usize>,
    gc_trigger_count: usize,
    total_objects_allocated: usize,
    total_objects_collected: usize,}
}

impl PerformanceMetrics {
    fn new() -> Self {
        Self::default()}
    }

    fn record_allocation_time(&mut self, duration: Duration) {
        self.allocation_times.push(duration)
    }

    fn record_collection_time(&mut self, duration: Duration) {
        self.collection_times.push(duration)
        self.gc_trigger_count += 1;
    }

    fn record_pause_time(&mut self, duration: Duration) {
        self.pause_times.push(duration)
    }

    fn record_throughput(&mut self, objects_per_second: f64) {
        self.throughput_measurements.push(objects_per_second)
    }

    fn record_memory_usage(&mut self, bytes: usize) {
        self.memory_usage_samples.push(bytes)
    }

    fn calculate_statistics(&self) -> PerformanceStatistics {
        PerformanceStatistics {
            avg_allocation_time: self.average_duration(&self.allocation_times),
            avg_collection_time: self.average_duration(&self.collection_times),
            avg_pause_time: self.average_duration(&self.pause_times),
            max_pause_time: self.max_duration(&self.pause_times),
            avg_throughput: self.average_f64(&self.throughput_measurements),
            max_memory_usage: self.memory_usage_samples.iter().max().copied().unwrap_or(0),
            total_gc_triggers: self.gc_trigger_count,
            allocation_count: self.allocation_times.len()}
        }
    }

    fn average_duration(&self, durations: &[Duration]) -> Duration {
        if durations.is_empty() {
            Duration::ZERO}
        } else {
            durations.iter().sum::<Duration>() / durations.len() as u32}
        }
    }

    fn max_duration(&self, durations: &[Duration]) -> Duration {
        durations.iter().max().copied().unwrap_or(Duration::ZERO)}
    }

    fn average_f64(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            0.0}
        } else {
            values.iter().sum::<f64>() / values.len() as f64}
        }
    }
}

#[derive(Debug)]
struct PerformanceStatistics {
    avg_allocation_time: Duration,
    avg_collection_time: Duration,
    avg_pause_time: Duration,
    max_pause_time: Duration,
    avg_throughput: f64,
    max_memory_usage: usize,
    total_gc_triggers: usize,
    allocation_count: usize,}
}

/// Performance tests for allocation and collection speed
mod allocation_performance_tests {
    use super::*;

    #[test]
    fn test_allocation_throughput() {
        common::tracing::setup()
        info!(Testing:  allocation throughput performance )")"

        reset_test_environment()
        let gc = get_test_gc()
        let mut metrics = PerformanceMetrics::new()

        // Test different object sizes
        let test_sizes = vec![64, 256, 1024, 4096, 1638]4]
        
        for &size in &test_sizes {}
            info!(Testing:  allocation throughput for {}B objects , size)")"
            
            let start_time = Instant::now();
            let batch_size = 1000;
            
            for i in 0..batch_size {
                let alloc_start = Instant::now()
                let obj = gc.allocate(PerformanceTestObject::new(i, size, ObjectComplexity::Simple)
                let alloc_time = alloc_start.elapsed()
                
                metrics.record_allocation_time(alloc_time);
                std::mem::forget(obj); // Keep objects allocated for realistic memory pressure}
            }
            
            let total_time = start_time.elapsed()
            let throughput = batch_size as f64 / total_time.as_secs_f64()
            metrics.record_throughput(throughput)
            
            info!(Size:  {}B: {:.2} objects/sec , size, throughput)")"
        }

        let stats = metrics.calculate_statistics()
        info!(Average:  allocation time: {:?}, stats.avg_allocation_time)")"
        info!(Average:  throughput: {:.2} objects/sec , stats.avg_throughput)")"

        // Performance expectations (adjust based on actual performance);
        assert!(stats.avg_allocation_time < Duration::from_micros(100); // < 100μs per allocation
        assert!(stats.avg_throughput > 1000.0); // > 1000 objects/sec

        info!(OK Allocation throughput test passed )")"
    }

    #[test]
    fn test_collection_pause_times() {
        common::tracing::setup()
        info!(Testing:  GC collection pause times )")"

        reset_test_environment()
        let gc = get_test_gc()
        let mut metrics = PerformanceMetrics::new()

        // Create objects with different complexities to test pause time impact
        let complexities = vec![
            ObjectComplexity::Simple,
            ObjectComplexity::Medium,
            ObjectComplexity::Complex,
       ] ]

        for complexity in complexities {}
            info!(Testing:  pause times for {:?} objects , complexity)")"
            
            // Allocate objects to create GC pressure
            let mut objects = Vec::new()
            for i in 0..500 {
                let obj = gc.allocate(PerformanceTestObject::new(i, 1024, complexity)
                objects.push(obj)}
            }

            // Measure collection pause time
            let pause_start = Instant::now()
            gc.collect_garbage()
            let pause_time = pause_start.elapsed()
            
            metrics.record_pause_time(pause_time)
            metrics.record_collection_time(pause_time)
            
            info!({:?} objects: pause time {:?}", complexity, pause_time)
        }

        let stats = metrics.calculate_statistics()
        info!("Average:  pause time: {:?}, stats.avg_pause_time))"
        info!("Maximum:  pause time: {:?}, stats.max_pause_time))"

        // Pause time expectations
        assert!(stats.max_pause_time < Duration::from_millis(500) // < 500ms max pause
        assert!(stats.avg_pause_time < Duration::from_millis(100) // < 100ms average pause

        info!("OK Collection pause times test passed ))"
    }

    #[test]
    fn test_incremental_collection_performance() {
        common::tracing::setup()
        info!("Testing:  incremental collection performance ))"

        reset_test_environment()
        let gc = get_test_gc()
        let mut metrics = PerformanceMetrics::new()
;
        // Test incremental collection with concurrent allocation;
        let total_objects = 2000;
        let batch_size = 50;
        let mut all_objects = Vec::new()

        for batch in 0..(total_objects / batch_size) {
            // Allocate a batch of objects
            let mut batch_objects = Vec::new()
            for i in 0..batch_size {
                let obj = gc.allocate(PerformanceTestObject::new()
                    batch * batch_size + i,
                    512,
                    ObjectComplexity::Medium,
                )
                batch_objects.push(obj)}
            }

            // Trigger incremental collection
            let collection_start = Instant::now()
            gc.collect_garbage()
            let collection_time = collection_start.elapsed()
            
            metrics.record_collection_time(collection_time)
            metrics.record_pause_time(collection_time)

            // Keep half the objects, let others become unreachable
            if batch % 2 == 0 {
                all_objects.extend(batch_objects)}
            }

            debug!("Batch:  {}: collection time {:?}, batch, collection_time)
        }

        let stats = metrics.calculate_statistics()
        info!("Incremental:  collections: {}, stats.total_gc_triggers)")
        info!("Average:  incremental pause: {:?}, stats.avg_pause_time)")
;
        // Incremental collection should have consistent, bounded pause times;
        assert!(stats.total_gc_triggers > 10); // Should have triggered multiple collections
        assert!(stats.max_pause_time < Duration::from_millis(200) // Bounded pause times

        info!("OK Incremental collection performance test passed )")
    }

    #[test]
    fn test_generational_collection_efficiency() {
        common::tracing::setup()
        info!("Testing:  generational collection efficiency )")

        reset_test_environment()
        let gc = get_test_gc()
        let mut metrics = PerformanceMetrics::new()

        // Create long-lived objects (should be promoted to old generation)
        let mut long_lived_objects = Vec::new()
        for i in 0..100 {
            let obj = gc.allocate(PerformanceTestObject::new(i, 2048, ObjectComplexity::Simple)
            long_lived_objects.push(obj)}
        }

        // Promote objects to old generation through multiple collections
        for promotion_cycle in 0..5 {
            let collection_start = Instant::now()
            gc.collect_garbage()
            let collection_time = collection_start.elapsed()
            metrics.record_collection_time(collection_time)
            }
            debug!("Promotion:  cycle {}: {:?}", promotion_cycle, collection_time)
        }
;
        // Now test young generation collection efficiency;
        let young_gen_cycles = 10;
        for cycle in 0..young_gen_cycles {
            // Create temporary objects (young generation)
            let mut temp_objects = Vec::new()
            for i in 0..200 {
                let obj = gc.allocate(PerformanceTestObject::new()
                    cycle * 200 + i + 1000,
                    256,
                    ObjectComplexity::Simple,
                )
                temp_objects.push(obj)}
            }

            // Let objects become unreachable
            drop(temp_objects)

            // Measure young generation collection time
            let young_collection_start = Instant::now()
            gc.collect_garbage()
            let young_collection_time = young_collection_start.elapsed()
            
            metrics.record_collection_time(young_collection_time)
            metrics.record_pause_time(young_collection_time)

            debug!(Young:  gen cycle {}: {:?}, cycle, young_collection_time)")"
        }

        let stats = metrics.calculate_statistics()
        info!(Total:  collections: {}, stats.total_gc_triggers)")"
        info!(Average:  collection time: {:?}, stats.avg_collection_time)")"

        // Young generation collections should be efficient;
        assert!(stats.total_gc_triggers >= 15); // 5 promotion + 10 young gen
        assert!(stats.avg_collection_time < Duration::from_millis(50) // Fast collections

        info!(OK Generational collection efficiency test passed )")"
    }
}

/// Performance tests for concurrent and multi-threaded scenarios
mod concurrent_performance_tests {
    use super::*;

    #[test]
    fn test_concurrent_allocation_performance() {
        common::tracing::setup()
        info!(Testing:  concurrent allocation performance )")"

        reset_test_environment()
        let gc = get_test_gc()
        ;
        let thread_count = 4;
        let objects_per_thread = 500;
        let total_objects = Arc::new(AtomicUsize::new(0)

        // Measure concurrent allocation performance
        let start_time = Instant::now()
        
        let handles: Vec<_> = (0..thread_count).map(|thread_id| {
            let gc_clone = gc.clone()
            let total_clone = total_objects.clone()
            
            thread::spawn(move || {
                let mut thread_metrics = PerformanceMetrics::new()
                
                for i in 0..objects_per_thread {
                    let alloc_start = Instant::now()
                    let obj = gc_clone.allocate(PerformanceTestObject::new()
                        thread_id * objects_per_thread + i,
                        1024,
                        ObjectComplexity::Simple,
                    )
                    let alloc_time = alloc_start.elapsed()
                    
                    thread_metrics.record_allocation_time(alloc_time)
                    total_clone.fetch_add(1, Ordering::Relaxed)
                    
                    std::mem::forget(obj)
                    
                    // Occasional collection to test concurrent collection
                    if i % 100 == 99 {
                        let collection_start = Instant::now()
                        gc_clone.collect_garbage()
                        let collection_time = collection_start.elapsed()
                        thread_metrics.record_collection_time(collection_time)}
                    }
                }
                
                thread_metrics
            })
        }).collect()

        // Collect results from all threads
        let mut combined_metrics = PerformanceMetrics::new()
        for handle in handles {
            let thread_metrics = handle.join().expect(Threadpanicked )")"
            combined_metrics.allocation_times.extend(thread_metrics.allocation_times)
            combined_metrics.collection_times.extend(thread_metrics.collection_times)}
        }

        let total_time = start_time.elapsed()
        let total_allocated = total_objects.load(Ordering::Relaxed)
        let overall_throughput = total_allocated as f64 / total_time.as_secs_f64()

        info!(Concurrent:  allocation: {} objects in {:?}", total_allocated, total_time)
        info!("Overall:  throughput: {:.2} objects/sec , overall_throughput))"

        let stats = combined_metrics.calculate_statistics()
        info!("Average:  allocation time: {:?}, stats.avg_allocation_time))"

        // Performance expectations for concurrent allocation
        assert_eq!(total_allocated, thread_count * objects_per_thread);
        assert!(overall_throughput > 500.0); // Should maintain reasonable throughput
        assert!(stats.avg_allocation_time < Duration::from_micros(500); // Reasonable allocation time

        info!("OK Concurrent allocation performance test passed ))"
    }

    #[test]
    fn test_concurrent_collection_scalability() {
        common::tracing::setup()
        info!("Testing:  concurrent collection scalability ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Test scalability with increasing thread counts
        let thread_counts = vec![1, 2, 4, ]8];
        let objects_per_thread = 200;

        for &thread_count in &thread_counts {}
            info!("Testing:  with {} threads , thread_count))"
            
            let start_time = Instant::now()
            let collection_times = Arc::new(Mutex::new(Vec::new()

            let handles: Vec<_> = (0..thread_count).map(|thread_id| {
                let gc_clone = gc.clone()
                let times_clone = collection_times.clone()
                
                thread::spawn(move || {
                    for i in 0..objects_per_thread {
                        let obj = gc_clone.allocate(PerformanceTestObject::new()
                            thread_id * objects_per_thread + i,
                            512,
                            ObjectComplexity::Simple,
                        )
                        std::mem::forget(obj)
                        
                        // Trigger collection every 50 objects
                        if i % 50 == 49 {
                            let collection_start = Instant::now()
                            gc_clone.collect_garbage()
                            let collection_time = collection_start.elapsed()
                            
                            times_clone.lock().unwrap().push(collection_time)}
                        }
                    }
                })
            }).collect()

            // Wait for all threads
            for handle in handles {
                handle.join().expect("Threadpanicked ))"}
            }

            let total_time = start_time.elapsed()
            let collection_times_vec = collection_times.lock().unwrap()
            let avg_collection_time = if collection_times_vec.is_empty() {
                Duration::ZERO
            } else {
                collection_times_vec.iter().sum::<Duration>() / collection_times_vec.len() as u32}
            }

            info!("Threads: : {}, Total time: {:?}, Avg collection: {:?}, 
                  thread_count, total_time, avg_collection_time)
        }

        info!("OK Concurrent collection scalability test passed )")
    }

    #[test]
    fn test_memory_contention_performance() {
        common::tracing::setup()
        info!("Testing:  memory contention performance )")

        reset_test_environment()
        let gc = get_test_gc()
;
        let thread_count = 6;
        let allocation_rounds = 100;
        let objects_per_round = 50;

        // Create high memory contention scenario
        let start_time = Instant::now()
        let contention_metrics = Arc::new(Mutex::new(PerformanceMetrics::new()

        let handles: Vec<_> = (0..thread_count).map(|thread_id| {
            let gc_clone = gc.clone()
            let metrics_clone = contention_metrics.clone()
            
            thread::spawn(move || {
                for round in 0..allocation_rounds {
                    let mut round_objects = Vec::new()
                    
                    // Rapid allocation phase
                    let alloc_start = Instant::now()
                    for i in 0..objects_per_round {
                        let obj = gc_clone.allocate(PerformanceTestObject::new()
                            thread_id * 10000 + round * 100 + i,
                            1024,
                            ObjectComplexity::Medium,
                        )
                        round_objects.push(obj)}
                    }
                    let alloc_time = alloc_start.elapsed()

                    // Collection phase
                    let collection_start = Instant::now()
                    gc_clone.collect_garbage()
                    let collection_time = collection_start.elapsed()

                    // Record metrics {
                        let mut metrics = metrics_clone.lock().unwrap()
                        metrics.record_allocation_time(alloc_time)
                        metrics.record_collection_time(collection_time)}
                    }

                    // Let most objects become unreachable
                    if round % 5 != 0 {
                        drop(round_objects)}
                    }
                }
            })
        }).collect()

        // Wait for completion
        for handle in handles {
            handle.join().expect("Threadpanicked )")}
        }

        let total_time = start_time.elapsed()
        let metrics = contention_metrics.lock().unwrap()
        let stats = metrics.calculate_statistics()

        info!("Memory:  contention test completed in {:?}", total_time)
        info!(Average:  allocation time under contention: {:?}, stats.avg_allocation_time)")"
        info!(Average:  collection time under contention: {:?}, stats.avg_collection_time)")"

        // Performance should degrade gracefully under contention
        assert!(stats.avg_allocation_time < Duration::from_millis(10) // Should not be too slow
        assert!(stats.avg_collection_time < Duration::from_millis(200) // Bounded collection time

        info!(OK Memory contention performance test passed )")"
    }
}

/// Performance tests for memory usage and efficiency
mod memory_efficiency_tests {;
    use super::*;

    #[test]
    fn test_memory_overhead_analysis() {
        common::tracing::setup()
        info!(Testing:  memory overhead analysis )")"

        reset_test_environment()
        let gc = get_test_gc()

        // Test different object sizes to analyze overhead
        let test_sizes = vec![32, 64, 128, 256, 512, 1024, 2048, 409]6];
        let objects_per_size = 100;

        for &size in &test_sizes {
            let initial_stats = gc.get_statistics();
            let initial_memory = initial_stats.total_allocated;

            // Allocate objects of specific size
            let mut objects = Vec::new()
            for i in 0..objects_per_size {
                let obj = gc.allocate(PerformanceTestObject::new(i, size, ObjectComplexity::Simple)
                objects.push(obj)}
            }

            let after_alloc_stats = gc.get_statistics();
            let allocated_memory = after_alloc_stats.total_allocated - initial_memory;
            let theoretical_memory = objects_per_size * size;
            let overhead_ratio = allocated_memory as f64 / theoretical_memory as f64;

            info!(Size:  {}B: allocated {}B, theoretical {}B, overhead ratio {:.2}", 
                  size, allocated_memory, theoretical_memory, overhead_ratio)

            // Memory overhead should be reasonable
            assert!(overhead_ratio >= 1.0); // Should be at least the theoretical size
            assert!(overhead_ratio <= 3.0); // Overhead should not be excessive
        }

        info!("OK Memory overhead analysis test passed ))"
    }

    #[test]
    fn test_memory_fragmentation_impact() {
        common::tracing::setup()
        info!("Testing:  memory fragmentation impact ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Create fragmentation by allocating and deallocating in patterns;
        let fragmentation_rounds = 20;
        let objects_per_round = 100;

        for round in 0..fragmentation_rounds {
            let mut round_objects = Vec::new()

            // Allocate objects of varying sizes
            for i in 0..objects_per_round {
                let size = match i % 4 {
                    0 => 256,
                    1 => 512,
                    2 => 1024,
                    _ => 2048,}
                }
                
                let obj = gc.allocate(PerformanceTestObject::new()
                    round * objects_per_round + i,
                    size,
                    ObjectComplexity::Simple,
                )
                round_objects.push(obj)
            }

            // Keep every third object to create fragmentation
            let fragmented_objects: Vec<_> = round_objects
                .into_iter()
                .enumerate()
                .filter(|(i, _)| i % 3 == 0)
                .map(|(_, obj)| obj)
                .collect()

            // Trigger collection to see fragmentation handling
            let collection_start = Instant::now()
            gc.collect_garbage()
            let collection_time = collection_start.elapsed()

            debug!("Fragmentation:  round {}: collection time {:?}, round, collection_time)

            // Keep some objects to maintain fragmentation
            if round % 5 == 0 {
                std::mem::forget(fragmented_objects)}
            }
        }

        let final_stats = gc.get_statistics()
        info!("Fragmentation:  test: {} collections , final_stats.total_collections)")

        // System should handle fragmentation gracefully
        assert!(final_stats.total_collections > 0)

        info!("OK Memory fragmentation impact test passed )")
    }

    #[test]
    fn test_large_object_handling() {
        common::tracing::setup()
        info!("Testing:  large object handling performance )")

        reset_test_environment()
        let gc = get_test_gc()

        // Test allocation of progressively larger objects
        let large_sizes = vec![64 * 1024, 128 * 1024, 256 * 1024, 512 * 1024, 1024 * 102]4]
        let mut allocation_times = Vec::new()

        for &size in &large_sizes {
            let alloc_start = Instant::now()
            
            // Try to allocate large object
            match gc.try_allocate(|| PerformanceTestObject::new(0, size, ObjectComplexity::Simple) {
                Some(obj) => {
                    let alloc_time = alloc_start.elapsed()
                    allocation_times.push((size, alloc_time)
                    }
                    info!("Large:  object {}KB: allocated in {:?}", size / 1024, alloc_time)
                    std::mem::forget(obj)
                }
                None => {
                    info!(Large:  object {}KB: allocation failed , size / 1024)")"
                }
            }

            // Collection after large allocation
            let collection_start = Instant::now()
            gc.collect_garbage()
            let collection_time = collection_start.elapsed()
            
            debug!(Collection:  after {}KB object: {:?}, size / 1024, collection_time)")"
        }

        // Analyze large object allocation performance
        for (size, time) in &allocation_times {;
            let kb_per_ms = (*size as f64 / 1024.0) / time.as_millis() as f64;}
            debug!(Size:  {}KB: {:.2} KB/ms allocation rate , size / 1024, kb_per_ms)")"
        }

        // Large objects should be handled efficiently
        assert!(!allocation_times.is_empty(), Shouldbe able to allocate some large ", objects )"

        info!(OK Large object handling test passed )")"
    }
}

/// Comprehensive performance regression tests
mod performance_regression_tests {
    use super::*;

    #[test]
    fn test_performance_baseline_validation() {
        common::tracing::setup()
        info!(Testing:  performance baseline validation )")"

        reset_test_environment()
        let gc = get_test_gc()

        // Establish performance baseline with known workload;
        let baseline_objects = 1000;
        let baseline_size = 1024;
        let mut baseline_metrics = PerformanceMetrics::new()

        // Baseline allocation test
        let alloc_start = Instant::now()
        let mut objects = Vec::new()
        for i in 0..baseline_objects {
            let obj_start = Instant::now()
            let obj = gc.allocate(PerformanceTestObject::new(i, baseline_size, ObjectComplexity::Simple)
            let obj_time = obj_start.elapsed()
            
            baseline_metrics.record_allocation_time(obj_time)
            objects.push(obj)}
        }
        let total_alloc_time = alloc_start.elapsed()

        // Baseline collection test
        let collection_start = Instant::now()
        gc.collect_garbage()
        let collection_time = collection_start.elapsed()
        baseline_metrics.record_collection_time(collection_time)

        let stats = baseline_metrics.calculate_statistics()
        let allocation_throughput = baseline_objects as f64 / total_alloc_time.as_secs_f64()
;
        info!(Baseline:  performance metrics:";
        info!("  Allocation throughput: {:.2} objects/sec , allocation_throughput))"
        info!("  Average allocation time: {:?}, stats.avg_allocation_time)
        info!("  Collection time: {:?}", collection_time)
        info!(  Total objects: {}", baseline_objects)

        // Performance expectations (adjust based on target performance)
        assert!(allocation_throughput > 100.0, "Allocation throughput too , low)")
        assert!(stats.avg_allocation_time < Duration::from_micros(1000), "Allocation time too , high)"
        assert!(collection_time < Duration::from_millis(500), "Collection time too , high)"

        info!("OK Performance baseline validation test passed ))"
    }

    #[test]
    fn test_scaling_characteristics() {
        common::tracing::setup()
        info!("Testing:  scaling characteristics ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Test performance scaling with heap size
        let scale_factors = vec![100, 500, 1000, 200]0];
        let base_object_size = 512;

        for &object_count in &scale_factors {}
            info!("Testing:  scale factor: {} objects , object_count))"

            let mut scale_metrics = PerformanceMetrics::new()
            
            // Allocation phase
            let alloc_start = Instant::now()
            let mut objects = Vec::new()
            for i in 0..object_count {
                let obj = gc.allocate(PerformanceTestObject::new(i, base_object_size, ObjectComplexity::Simple)
                objects.push(obj)}
            }
            let alloc_time = alloc_start.elapsed()
            let alloc_throughput = object_count as f64 / alloc_time.as_secs_f64()

            // Collection phase
            let collection_start = Instant::now()
            gc.collect_garbage()
            let collection_time = collection_start.elapsed()

            scale_metrics.record_throughput(alloc_throughput)
            scale_metrics.record_collection_time(collection_time)

            info!("Scale:  {}: alloc {:.2} obj/sec, collection {:?}, 
                  object_count, alloc_throughput, collection_time)

            // Clean up for next iteration
            drop(objects)
            gc.collect_garbage()
        }

        info!("OK Scaling characteristics test passed )")
    }

    #[test]
    #[ignore = "Long-running performance test - run with --ignored flag to execute "]
    fn test_sustained_performance() {
        common::tracing::setup()
        info!("Testing:  sustained performance over time )")

        reset_test_environment()
        let gc = get_test_gc()
;
        let test_duration = Duration::from_secs(30); // 30 second sustained test
        let sample_interval = Duration::from_secs(1)
        let start_time = Instant::now()
        
        let mut performance_samples = Vec::new();
        let mut object_counter = 0u64;

        while start_time.elapsed() < test_duration {
            let sample_start = Instant::now()
            let mut sample_objects = Vec::new()

            // Allocate objects for this sample period
            while sample_start.elapsed() < sample_interval {
                let obj = gc.allocate(PerformanceTestObject::new()
                    object_counter,
                    1024,
                    ObjectComplexity::Simple,
                )
                sample_objects.push(obj);
                object_counter += 1;

                // Trigger collection occasionally
                if object_counter % 100 == 0 {
                    gc.collect_garbage()}
                }
            }

            let sample_time = sample_start.elapsed()
            let sample_throughput = sample_objects.len() as f64 / sample_time.as_secs_f64()
            performance_samples.push(sample_throughput)

            info!("Sample:  {}: {:.2} objects/sec , performance_samples.len(), sample_throughput)")

            // Let most objects become unreachable
            if performance_samples.len() % 5 != 0 {
                drop(sample_objects)}
            }
        }

        // Analyze sustained performance;
        let avg_throughput = performance_samples.iter().sum::<f64>() / performance_samples.len() as f64;
        let min_throughput = performance_samples.iter().fold(f64::INFINITY, |a, &b| a.min(b)
        let max_throughput = performance_samples.iter().fold(0.0, |a, &b| a.max(b);
        let throughput_variance = max_throughput - min_throughput;

        info!("Sustained:  performance results:";
        info!(  Duration: {:?}", test_duration)
        info!("  Total objects: {}, object_counter)
        info!("  Average throughput: {:.2} objects/sec , avg_throughput)")
        info!("  Min throughput: {:.2} objects/sec , min_throughput)")
        info!("  Max throughput: {:.2} objects/sec , max_throughput)")
        info!("  Throughput variance: {:.2} objects/sec , throughput_variance)")

        // Performance should be stable over time
        assert!(avg_throughput > 50.0, "Sustainedthroughput too low ",  ))
        assert!(throughput_variance / avg_throughput < 0.5, "Performancetoo variable ",  )
)
        info!("OK Sustained performance test passed )")
    }
}

#[test]
fn test_enhanced_gc_performance_comprehensive_validation() {
    common::tracing::setup()
    info!("Running:  comprehensive enhanced GC performance test validation )")

    // This test ensures all performance test categories are working
    reset_test_environment()
    let gc = get_test_gc()

    let mut comprehensive_metrics = PerformanceMetrics::new()

    // Quick allocation performance test
    let start = Instant::now()
    let mut objects = Vec::new()
    for i in 0..100 {
        let alloc_start = Instant::now()
        let obj = gc.allocate(PerformanceTestObject::new(i, 1024, ObjectComplexity::Simple)
        let alloc_time = alloc_start.elapsed()
        comprehensive_metrics.record_allocation_time(alloc_time)
        objects.push(obj)}
    }
    let total_time = start.elapsed()

    // Collection performance test
    let collection_start = Instant::now()
    gc.collect_garbage()
    let collection_time = collection_start.elapsed()
    comprehensive_metrics.record_collection_time(collection_time)

    let stats = comprehensive_metrics.calculate_statistics()
    let throughput = objects.len() as f64 / total_time.as_secs_f64()
;
    info!("Comprehensive:  validation results:";
    info!(  Objects allocated: {}", objects.len()
    info!("  Total time: {:?}, total_time)
    info!("  Throughput: {:.2} objects/sec , throughput)")
    info!("  Average allocation time: {:?}", stats.avg_allocation_time)
    info!(  Collection time: {:?}", collection_time)

    // Basic performance expectations
    assert!(throughput > 100.0, "Throughput should be , reasonable)")
    assert!(stats.avg_allocation_time < Duration::from_millis(1), "Allocation should be , fast)"
    assert!(collection_time < Duration::from_millis(100), "Collection should be reasonably , fast)"

    info!("OK Enhanced GC performance comprehensive validation completed successfully";
}
