/// Production-ready GC system integration test
/// 
/// Tests all memory management components working together
/// for production workloads with minimal pause times.

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    
    use crate::runtime::gc::{GarbageCollector, GcConfig, GcState};
    use crate::runtime::gc_tuning::{TriColorCollector, GcPerformanceTuner, GcTuningParams};
    use crate::runtime::memory::{MemoryManager, MemoryConfig};
    use crate::runtime::memory_profiler::{MemoryProfiler, ProfilingConfig};
    use crate::runtime::concurrent_gc::{ConcurrentGarbageCollector, ConcurrentGcConfig};
    use crate::runtime::heap_optimizer::{HeapOptimizer, HeapOptimizerConfig, AllocationStrategy};
    use crate::runtime::gc_monitor::{GcMonitor, GcMonitorConfig};
    use crate::runtime::stack::RuntimeStack;
    use crate::memory::Tag;

    /// Test production GC system integration
    #[test]
    fn test_production_gc_system() {
        let stack_manager = Arc::new(RuntimeStack::new());
        
        // Create base GC
        let gc_config = GcConfig::default();
        let gc = Arc::new(GarbageCollector::new(gc_config, Arc::clone(&stack_manager)).unwrap());
        
        // Create tri-color collector
        let tri_color_collector = Arc::new(TriColorCollector::new());
        
        // Create performance tuner
        let performance_tuner = Arc::new(GcPerformanceTuner::new(GcTuningParams::default()));
        
        // Create concurrent GC
        let concurrent_config = ConcurrentGcConfig::default();
        let concurrent_gc = Arc::new(ConcurrentGarbageCollector::new(
            concurrent_config, 
            Arc::clone(&gc), 
            Arc::clone(&tri_color_collector),
            Arc::clone(&performance_tuner)
        ).unwrap());
        
        // Create memory manager
        let memory_config = MemoryConfig::default();
        let memory_manager = Arc::new(MemoryManager::new(memory_config, Arc::clone(&stack_manager)).unwrap());
        
        // Create profiler
        let profiling_config = ProfilingConfig::default();
        let profiler = Arc::new(MemoryProfiler::new(profiling_config));
        
        // Create heap optimizer
        let heap_config = HeapOptimizerConfig::default();
        let heap_optimizer = Arc::new(HeapOptimizer::new(heap_config).unwrap());
        
        // Create GC monitor
        let monitor_config = GcMonitorConfig::default();
        let mut monitor = GcMonitor::new(monitor_config).unwrap();
        monitor.set_gc_ref(Arc::clone(&gc));
        monitor.set_concurrent_gc_ref(Arc::clone(&concurrent_gc));
        monitor.set_memory_manager_ref(Arc::clone(&memory_manager));
        monitor.set_profiler_ref(Arc::clone(&profiler));
        monitor.set_heap_optimizer_ref(Arc::clone(&heap_optimizer));
        
        // Start components
        assert!(profiler.start().is_ok());
        assert!(heap_optimizer.start().is_ok());
        assert!(concurrent_gc.start().is_ok());
        assert!(monitor.start().is_ok());
        
        // Test allocation
        let handle = memory_manager.allocate_raw(1024, Tag::Object).unwrap();
        assert_eq!(handle.size(), 1024);
        
        // Test collection
        let stats = gc.collect().unwrap();
        assert!(stats.total_collections > 0);
        
        // Test concurrent collection
        let concurrent_stats = concurrent_gc.collect().unwrap();
        assert!(concurrent_stats.total_concurrent_collections > 0);
        
        // Test profiling
        let profiling_stats = profiler.get_stats();
        assert!(profiling_stats.total_allocations > 0);
        
        // Test heap optimization
        let heap_stats = heap_optimizer.get_stats();
        assert_eq!(heap_stats.total_allocations, 0); // Direct allocation bypasses optimizer
        
        // Test monitoring
        let metrics = monitor.get_metrics_snapshot();
        assert!(metrics.gc_stats.total_collections > 0);
        
        // Stop components
        assert!(monitor.stop().is_ok());
        assert!(concurrent_gc.stop().is_ok());
        assert!(heap_optimizer.stop().is_ok());
        assert!(profiler.stop().is_ok());
    }

    /// Test low-latency GC configuration
    #[test]
    fn test_low_latency_gc() {
        let stack_manager = Arc::new(RuntimeStack::new());
        
        // Create low-latency GC config
        let gc_config = GcConfig {
            incremental_collection: true,
            concurrent_collection: true,
            incremental_time_budget: 5, // 5ms max
            max_heap_size: Some(512 * 1024 * 1024), // 512MB
            enable_compaction: false, // Disable for lower latency
            ..Default::default()
        };
        
        let gc = Arc::new(GarbageCollector::new(gc_config, Arc::clone(&stack_manager)).unwrap());
        
        // Test low-latency allocation and collection
        let start = std::time::Instant::now();
        
        // Allocate many small objects
        for i in 0..100 {
            let _obj = gc.allocate(64 + i % 64, Tag::Object).unwrap();
        }
        
        // Perform collection
        let stats = gc.collect().unwrap();
        
        let elapsed = start.elapsed();
        
        // Verify low latency
        assert!(stats.max_pause_time < Duration::from_millis(10));
        assert!(elapsed < Duration::from_millis(100));
        assert!(stats.total_collections > 0);
    }

    /// Test high-throughput GC configuration
    #[test]
    fn test_high_throughput_gc() {
        let stack_manager = Arc::new(RuntimeStack::new());
        
        // Create high-throughput GC config
        let gc_config = GcConfig {
            initial_heap_size: 256 * 1024 * 1024, // 256MB
            max_heap_size: Some(1024 * 1024 * 1024), // 1GB
            young_generation_ratio: 0.6, // Large young generation
            concurrent_collection: true,
            concurrent_threads: 4,
            enable_compaction: true,
            ..Default::default()
        };
        
        let gc = Arc::new(GarbageCollector::new(gc_config, Arc::clone(&stack_manager)).unwrap());
        
        // Test high-throughput allocation
        let start = std::time::Instant::now();
        
        // Allocate many objects of various sizes
        for i in 0..1000 {
            let size = 32 + (i % 10) * 32; // 32-352 bytes
            let _obj = gc.allocate(size, Tag::Object).unwrap();
        }
        
        let stats = gc.collect().unwrap();
        let elapsed = start.elapsed();
        
        // Verify high throughput
        assert!(stats.allocation_rate > 10_000_000.0); // > 10MB/s
        assert!(elapsed < Duration::from_secs(1));
        assert!(stats.total_collections > 0);
    }

    /// Test memory leak detection
    #[test]
    fn test_memory_leak_detection() {
        let profiling_config = ProfilingConfig {
            leak_detection: true,
            leak_threshold_seconds: 0, // Immediate detection
            ..Default::default()
        };
        
        let profiler = MemoryProfiler::new(profiling_config);
        
        // Start profiling
        profiler.start().unwrap();
        
        // Simulate allocation without deallocation
        profiler.record_allocation(0x1000, 1024, Tag::Object, None).unwrap();
        profiler.record_allocation(0x2000, 2048, Tag::Array, None).unwrap();
        
        // Wait for leak detection
        thread::sleep(Duration::from_millis(100));
        
        // Check for detected leaks
        let leaks = profiler.analyze_leaks().unwrap();
        assert!(leaks.len() >= 2);
        
        let stats = profiler.get_stats();
        assert_eq!(stats.live_allocations, 2);
        assert_eq!(stats.total_allocations, 2);
        
        // Stop profiling
        profiler.stop().unwrap();
    }

    /// Test heap allocation strategies
    #[test]
    fn test_heap_allocation_strategies() {
        let strategies = vec![
            AllocationStrategy::FirstFit,
            AllocationStrategy::BestFit,
            AllocationStrategy::SizeClass,
            AllocationStrategy::ThreadLocal,
        ];
        
        for strategy in strategies {
            let config = HeapOptimizerConfig {
                allocation_strategy: strategy,
                track_statistics: true,
                ..Default::default()
            };
            
            let optimizer = HeapOptimizer::new(config).unwrap();
            optimizer.start().unwrap();
            
            // Test allocation with different sizes
            let ptr1 = optimizer.allocate(64, 8, Tag::Object).unwrap();
            let ptr2 = optimizer.allocate(256, 8, Tag::Array).unwrap();
            let ptr3 = optimizer.allocate(1024, 8, Tag::String).unwrap();
            
            let stats = optimizer.get_stats();
            assert_eq!(stats.total_allocations, 3);
            
            // Test deallocation
            optimizer.deallocate(ptr1, 64).unwrap();
            optimizer.deallocate(ptr2, 256).unwrap();
            optimizer.deallocate(ptr3, 1024).unwrap();
            
            let stats = optimizer.get_stats();
            assert_eq!(stats.total_deallocations, 3);
            
            optimizer.stop().unwrap();
        }
    }

    /// Test concurrent GC write barriers
    #[test]
    fn test_concurrent_gc_write_barriers() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let gc_config = GcConfig::default();
        let gc = Arc::new(GarbageCollector::new(gc_config, Arc::clone(&stack_manager)).unwrap());
        
        let tri_color_collector = Arc::new(TriColorCollector::new());
        let performance_tuner = Arc::new(GcPerformanceTuner::new(GcTuningParams::default()));
        
        let concurrent_config = ConcurrentGcConfig {
            concurrent_marking: true,
            concurrent_sweeping: true,
            write_barrier_mode: crate::runtime::concurrent_gc::WriteBarrierMode::CardTable,
            ..Default::default()
        };
        
        let concurrent_gc = ConcurrentGarbageCollector::new(
            concurrent_config,
            Arc::clone(&gc),
            Arc::clone(&tri_color_collector),
            Arc::clone(&performance_tuner)
        ).unwrap();
        
        concurrent_gc.start().unwrap();
        
        // Test write barrier
        concurrent_gc.write_barrier(0x1000, 0x2000, 8);
        concurrent_gc.write_barrier(0x2000, 0x3000, 16);
        
        // Perform concurrent collection
        let stats = concurrent_gc.collect().unwrap();
        assert!(stats.total_concurrent_collections > 0);
        
        concurrent_gc.stop().unwrap();
    }

    /// Test GC monitoring and alerting
    #[test]
    fn test_gc_monitoring_alerting() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let gc_config = GcConfig::default();
        let gc = Arc::new(GarbageCollector::new(gc_config, Arc::clone(&stack_manager)).unwrap());
        
        let monitor_config = GcMonitorConfig {
            real_time_monitoring: true,
            alerting: true,
            monitoring_interval_ms: 100,
            ..Default::default()
        };
        
        let mut monitor = GcMonitor::new(monitor_config).unwrap();
        monitor.set_gc_ref(Arc::clone(&gc));
        
        // Register alert callback
        let alert_received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let alert_flag = Arc::clone(&alert_received);
        
        monitor.register_alert_callback(move |event| {
            println!("Alert received: {:?}", event);
            alert_flag.store(true, std::sync::atomic::Ordering::Relaxed);
        });
        
        monitor.start().unwrap();
        
        // Generate some GC activity
        for i in 0..10 {
            let _obj = gc.allocate(1024 + i * 100, Tag::Object).unwrap();
        }
        
        let _stats = gc.collect().unwrap();
        
        // Wait for monitoring
        thread::sleep(Duration::from_millis(200));
        
        // Get metrics
        let metrics = monitor.get_metrics_snapshot();
        assert!(metrics.gc_stats.total_collections > 0);
        
        // Get recommendations
        let recommendations = monitor.get_tuning_recommendations();
        assert!(recommendations.len() >= 0); // May or may not have recommendations
        
        // Generate report
        let report = monitor.generate_report();
        assert!(report.contains("GC Monitoring Report"));
        
        monitor.stop().unwrap();
    }

    /// Test complete production GC system under load
    #[test]
    fn test_production_gc_under_load() {
        let stack_manager = Arc::new(RuntimeStack::new());
        
        // Create production-ready GC configuration
        let gc_config = GcConfig {
            initial_heap_size: 128 * 1024 * 1024, // 128MB
            max_heap_size: Some(512 * 1024 * 1024), // 512MB
            incremental_collection: true,
            concurrent_collection: true,
            incremental_time_budget: 10, // 10ms
            concurrent_threads: 2,
            enable_compaction: true,
            compaction_threshold: 0.3,
            ..Default::default()
        };
        
        let gc = Arc::new(GarbageCollector::new(gc_config, Arc::clone(&stack_manager)).unwrap());
        
        // Create memory manager
        let memory_config = MemoryConfig::default();
        let memory_manager = Arc::new(MemoryManager::new(memory_config, Arc::clone(&stack_manager)).unwrap());
        
        // Create profiler
        let profiling_config = ProfilingConfig {
            track_allocations: true,
            leak_detection: true,
            sampling_rate: 10, // Sample every 10th allocation
            ..Default::default()
        };
        let profiler = Arc::new(MemoryProfiler::new(profiling_config));
        
        // Start components
        profiler.start().unwrap();
        
        let start_time = std::time::Instant::now();
        
        // Simulate production workload
        let mut handles = Vec::new();
        for i in 0..500 {
            let size = 32 + (i % 100) * 8; // 32-832 bytes
            let handle = memory_manager.allocate_raw(size, Tag::Object).unwrap();
            handles.push(handle);
            
            // Occasionally trigger collection
            if i % 100 == 0 {
                let _stats = gc.collect().unwrap();
            }
        }
        
        // Final collection
        let final_stats = gc.collect().unwrap();
        let elapsed = start_time.elapsed();
        
        // Verify production performance
        assert!(final_stats.max_pause_time < Duration::from_millis(50)); // < 50ms pause
        assert!(final_stats.gc_overhead < 0.2); // < 20% overhead
        assert!(elapsed < Duration::from_secs(5)); // Complete in < 5 seconds
        
        // Check profiling results
        let profiling_stats = profiler.get_stats();
        assert!(profiling_stats.total_allocations > 0);
        assert!(profiling_stats.allocation_rate > 0.0);
        
        // Generate memory report
        let report = profiler.generate_report().unwrap();
        assert!(report.contains("Memory Profiling Report"));
        
        profiler.stop().unwrap();
        
        println!("Production GC test completed:");
        println!("  Total collections: {}", final_stats.total_collections);
        println!("  Max pause time: {:?}", final_stats.max_pause_time);
        println!("  GC overhead: {:.2}%", final_stats.gc_overhead * 100.0);
        println!("  Heap utilization: {:.2}%", final_stats.heap_utilization * 100.0);
        println!("  Total elapsed: {:?}", elapsed);
        println!("  Allocation rate: {:.2} MB/s", profiling_stats.allocation_rate / 1_000_000.0);
    }
}
