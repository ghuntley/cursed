/// Comprehensive Performance Tests for Enhanced GC Implementation
/// 
/// This test suite validates performance characteristics, scalability, and efficiency
/// of the enhanced GC system under various workload patterns and stress conditions.

use cursed::memory::gc::*;
use cursed::memory::heap_manager::*;
use cursed::memory::object_store::*;
use cursed::memory::test_environment::  {get_test_gc, reset_test_environment}
use cursed::memory::::Traceable, Visitor;
use cursed::profiling::memory::MemoryProfiler;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}
use std::time::::Duration, Instant;
use std::thread;
use std::collections::HashMap;
use tracing::{info, debug, error, warn}

#[path = common.rs]
mod common;

/// Performance test object with configurable complexity
#[derive(Debug, Clone)]
struct PerformanceTestObject {id: u64,
    allocation_time: Instant,
    size: usize,
    complexity: ObjectComplexity,
    data: Vec<u8>,
    references: Vec<u64>, // Reference IDs instead of actual references for performance}

#[derive(Debug, Clone, Copy)]
enum ObjectComplexity {Simple,    // Minimal object overhead
    Medium,    // Moderate reference complexity
    Complex,   // High reference complexity}

impl PerformanceTestObject     {fn new() {let reference_count = match complexity     {ObjectComplexity::Simple => 0,
            ObjectComplexity::Medium => 5,
            ObjectComplexity::Complex => 20}

        Self {id,
            allocation_time: Instant::now()
            size,
            complexity,;
            data: vec![0u8; siz]
struct PerformanceStatistics {avg_allocation_time: Duration,
    avg_collection_time: Duration,
    avg_pause_time: Duration,
    max_pause_time: Duration,
    avg_throughput: f64,
    max_memory_usage: usize,
    total_gc_triggers: usize,
    allocation_count: usize}

/// Performance tests for allocation and collection speed
mod allocation_performance_tests   {use super::*;

    #[test]
    fn test_allocation_throughput() {}
            info!(Testing:  allocation throughput for   {}B objects , size);
            
            let start_time = Instant::now();
            let batch_size = 1000;
            
            for i in 0..batch_size   {let alloc_start = Instant::now()
                let obj = gc.allocate(PerformanceTestObject::new(i, size, ObjectComplexity::Simple)
                let alloc_time = alloc_start.elapsed()
                
                metrics.record_allocation_time(alloc_time);
                std::mem::forget(obj); // Keep objects allocated for realistic memory pressure}
            
            let total_time = start_time.elapsed()
            let throughput = batch_size as f64 / total_time.as_secs_f64()
            metrics.record_throughput(throughput)
            
            info!(Size:    {}B: {:.2} objects/sec , size, throughput);}

        let stats = metrics.calculate_statistics()
        info!(Average:  allocation time: {:?}, stats.avg_allocation_time)")")

        // Performance expectations (adjust based on actual performance);
        assert!(stats.avg_allocation_time < Duration::from_micros(100); // < 100μs per allocation
        assert!(stats.avg_throughput > 1000.0); // > 1000 objects/sec

        info!(OK Allocation throughput test passed);}

    #[test]
    fn test_collection_pause_times() {common::tracing::setup()
        info!(Testing:  GC collection pause times)

        reset_test_environment()
        let gc = get_test_gc()
        let mut metrics = PerformanceMetrics::new()

        // Create objects with different complexities to test pause time impact
        let complexities = vec![ObjectComplexity::Simple,
            ObjectComplexity::Medium,
            ObjectComplexity::Complex,]
    fn test_memory_overhead_analysis() {let obj = gc.allocate(PerformanceTestObject::new(i, size, ObjectComplexity::Simple)
                objects.push(obj)}

            let after_alloc_stats = gc.get_statistics();
            let allocated_memory = after_alloc_stats.total_allocated - initial_memory;
            let theoretical_memory = objects_per_size * size;
            let overhead_ratio = allocated_memory as f64 / theoretical_memory as f64;

            info!(Size:  {}B: allocated {}B, theoretical {}B, overhead ratio {:.2}, 
                  size, allocated_memory, theoretical_memory, overhead_ratio)

            // Memory overhead should be reasonable
            assert!(overhead_ratio >= 1.0); // Should be at least the theoretical size
            assert!(overhead_ratio <= 3.0); // Overhead should not be excessive}

        info!(OK Memory overhead analysis test passed);}

    #[test]
    fn test_memory_fragmentation_impact() {common::tracing::setup()
        info!("Testing:  memory fragmentation impact);"Testing:  large object handling performance)")
        reset_test_environment()
        let gc = get_test_gc()

        // Test allocation of progressively larger objects
        let large_sizes = vec![64 * 1024, 128 * 1024, 256 * 1024, 512 * 1024, 1024 * 102]
    fn test_performance_baseline_validation() {let obj_start = Instant::now()
            let obj = gc.allocate(PerformanceTestObject::new(i, baseline_size, ObjectComplexity::Simple)
            let obj_time = obj_start.elapsed()
            
            baseline_metrics.record_allocation_time(obj_time)
            objects.push(obj)}
        let total_alloc_time = alloc_start.elapsed()

        // Baseline collection test
        let collection_start = Instant::now()
        gc.collect_garbage()
        let collection_time = collection_start.elapsed()
        baseline_metrics.record_collection_time(collection_time)

        let stats = baseline_metrics.calculate_statistics()
        let allocation_throughput = baseline_objects as f64 / total_alloc_time.as_secs_f64();
        info!(Baseline:  performance metrics:;
        info!(Allocation throughput: {:.2} objects/sec , allocation_throughput);"  Average allocation time: {:?}, stats.avg_allocation_time)
        info!("  Collection time: {:?}, collection_time)
        info!(Total objects: {}, baseline_objects)

        // Performance expectations (adjust based on target performance)
        assert!(allocation_throughput > 100.0, Allocation throughput too , low)
        assert!(stats.avg_allocation_time < Duration::from_micros(1000), "
        assert!(collection_time < Duration::from_millis(500), "Collection time too , high)"OK Performance baseline validation test passed)";}
    #[test]
    fn test_scaling_characteristics() {common::tracing::setup()
        info!(

        reset_test_environment()
        let gc = get_test_gc()

        // Test performance scaling with heap size
        let scale_factors = vec![100, 500, 1000, 200]
    #[ignore = "Long-running performance test - run with --ignored flag to execute "Testing:  sustained performance over time)")
        reset_test_environment()
        let gc = get_test_gc();
        let test_duration = Duration::from_secs(30); // 30 second sustained test
        let sample_interval = Duration::from_secs(1)
        let start_time = Instant::now()
        
        let mut performance_samples = Vec::new();
        let mut object_counter = 0u64;

        while start_time.elapsed() < test_duration     {let sample_start = Instant::now()
            let mut sample_objects = Vec::new()

            // Allocate objects for this sample period
            while sample_start.elapsed() < sample_interval       {let obj = gc.allocate(PerformanceTestObject::new()
                    object_counter,
                    1024,
                    ObjectComplexity::Simple,)
                sample_objects.push(obj);
                object_counter += 1;

                // Trigger collection occasionally
                if object_counter % 100 == 0     {gc.collect_garbage()}

            let sample_time = sample_start.elapsed()
            let sample_throughput = sample_objects.len() as f64 / sample_time.as_secs_f64()
            performance_samples.push(sample_throughput)

            info!(Sample:  {}: {:.2} objects/sec , performance_samples.len(), sample_throughput)

            // Let most objects become unreachable
            if performance_samples.len() % 5 != 0     {drop(sample_objects)}

        // Analyze sustained performance;
        let avg_throughput = performance_samples.iter().sum::<f64>() / performance_samples.len() as f64;
        let min_throughput = performance_samples.iter().fold(f64::INFINITY, |a, &b| a.min(b)
        let max_throughput = performance_samples.iter().fold(0.0, |a, &b| a.max(b);
        let throughput_variance = max_throughput - min_throughput;

        info!(Sustained:  performance results:;
        info!(Duration: {:?}, test_duration)
        info!("  Average throughput: {:.2} objects/sec , avg_throughput)")
        info!()
        info!("  Max throughput: {:.2} objects/sec , max_throughput)"  Throughput variance: {:.2} objects/sec , throughput_variance)")
        // Performance should be stable over time
        assert!(avg_throughput > 50.0, Sustainedthroughput too low ,)
        assert!(throughput_variance / avg_throughput < 0.5, ",)
        info!("OK Sustained performance test passed)"Running:  comprehensive enhanced GC performance test validation)")
    // This test ensures all performance test categories are working
    reset_test_environment()
    let gc = get_test_gc()

    let mut comprehensive_metrics = PerformanceMetrics::new()

    // Quick allocation performance test
    let start = Instant::now()
    let mut objects = Vec::new()
    for i in 0..100   {let alloc_start = Instant::now()
        let obj = gc.allocate(PerformanceTestObject::new(i, 1024, ObjectComplexity::Simple)
        let alloc_time = alloc_start.elapsed()
        comprehensive_metrics.record_allocation_time(alloc_time)
        objects.push(obj)}
    let total_time = start.elapsed()

    // Collection performance test
    let collection_start = Instant::now()
    gc.collect_garbage()
    let collection_time = collection_start.elapsed()
    comprehensive_metrics.record_collection_time(collection_time)

    let stats = comprehensive_metrics.calculate_statistics()
    let throughput = objects.len() as f64 / total_time.as_secs_f64();
    info!(Comprehensive:  validation results:;
    info!(Objects allocated: {}, objects.len()
    info!("  Throughput: {:.2} objects/sec , throughput)")
    info!("Allocation should be , fast)"
    assert!(collection_time < Duration::from_millis(100), 

    info!("OK Enhanced GC performance comprehensive validation completed successfully";}
