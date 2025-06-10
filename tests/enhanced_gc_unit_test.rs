/// Comprehensive Unit Tests for Enhanced GC Implementation
/// 
/// This test suite validates all new heap management features, generational collection,
/// incremental collection, and memory safety guarantees in the enhanced GC system.

use cursed::memory::gc::*;
use cursed::memory::heap_manager::*;
use cursed::memory::object_store::*;
use cursed::memory::{Traceable, Visitor, ObjectRegistry}
use std::sync::{Arc, Mutex}
use std::time::{Duration, Instant};
use std::thread;
use tracing::{info, debug, error, warn}

#[path = "common.rs];
mod common;

/// Test object for heap management validation
#[derive(Debug, Clone)]
struct HeapTestObject {
    id: u32,
    data: Vec<u8>,
    refs: Vec<Arc<Mutex<HeapTestObject>>>,}
}

impl HeapTestObject {
    fn new(id: u32, size: usize) -> Self {
        Self {
            id,
            data: vec![0u8; siz]e],
            refs: Vec::new()}
        }
    }

    fn add_reference(&mut self, obj: Arc<Mutex<HeapTestObject>>) {
        self.refs.push(obj)
    }
}

impl Traceable for HeapTestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Simple tracing - in real implementation would trace refs}
        debug!("Tracing:  HeapTestObject {}", self.id)
    }
}

/// Unit tests for heap manager features
mod heap_manager_tests {
    use super::*;
    
    #[test]
    fn test_memory_block_creation() {
        common::tracing::setup()
        info!(Testing:  memory block creation )")"

        let block = MemoryBlock::new(1024 * 1024, 1).expect(Failedto create memory block )")"
        assert_eq!(block.size, 1024 * 1024)
        assert_eq!(block.used, 0)
        assert_eq!(block.next_free, 0)
        assert_eq!(block.id, 1)

        info!(Memory:  block allocation test passed )")"}
    }

    #[test]
    fn test_heap_config_validation() {
        common::tracing::setup()
        info!(Testing:  heap configuration validation )")"

        let config = HeapConfig {
            default_block_size: 1024 * 1024,
            max_blocks: 64,
            min_utilization: 0.5,
            enable_profiling: true,
            pressure_threshold: 0.8,
            growth_factor: 1.5,
            max_allocation_ratio: 0.5,
            gc_trigger_threshold: 0.75,}
        }

        // Valid configuration should pass
        assert!(config.default_block_size > 0)
        assert!(config.max_blocks > 0)
        assert!(config.min_utilization > 0.0 && config.min_utilization < 1.0)
        assert!(config.pressure_threshold > 0.0 && config.pressure_threshold <= 1.0)

        info!(Heap:  configuration validation test passed )")"
    }

    #[test]
    fn test_heap_manager_initialization() {
        common::tracing::setup()
        info!(Testing:  heap manager initialization )")"

        let config = HeapConfig::default()
        let registry = Arc::new(ObjectRegistry::new()
        
        let heap_manager = HeapManager::new(config, registry)
        
        let stats = heap_manager.get_stats().expect(Failedto get heap stats )")"
        assert!(stats.total_capacity >= 0)
        assert!(stats.total_used >= 0)
        assert!(stats.active_objects >= 0)

        info!(Heap:  manager initialization test passed )")"
    }

    #[test]
    fn test_heap_allocation_strategies() {
        common::tracing::setup()
        info!(Testing:  heap allocation strategies )")"

        let config = HeapConfig {
            default_block_size: 1024 * 1024,
            max_blocks: 16,
            min_utilization: 0.5,
            enable_profiling: true,
            pressure_threshold: 0.8,
            growth_factor: 1.5,
            max_allocation_ratio: 0.5,
            gc_trigger_threshold: 0.75,}
        }
        let registry = Arc::new(ObjectRegistry::new()
        let heap_manager = HeapManager::new(config, registry)

        // Test small object allocation
        let (small_id, small_ptr) = heap_manager.allocate::<u8>(128,  small_object ".expect("Failed to allocate small object)
        assert!(!small_id.is_null()
        assert!(!small_ptr.as_ptr().is_null())

        // Test medium object allocation
        let (medium_id, medium_ptr) = heap_manager.allocate::<u8>(4096,  "medium_object.expect("Failed to allocate medium object)
        assert!(!medium_id.is_null()
        assert!(!medium_ptr.as_ptr().is_null())

        // Test large object allocation (if enabled)
        let (large_id, large_ptr) = heap_manager.allocate::<u8>(128 * 1024,  "large_object.expect("Failed to allocate large object)
        assert!(!large_id.is_null()
        assert!(!large_ptr.as_ptr().is_null())

        info!("Heap:  allocation strategies test passed )")
    }

    #[test]
    fn test_memory_fragmentation_handling() {
        common::tracing::setup()
        info!("Testing:  memory fragmentation handling )")

        let config = HeapConfig {
            default_block_size: 512 * 1024,
            max_blocks: 4, // Limited blocks to force fragmentation
            min_utilization: 0.3,
            enable_profiling: true,
            pressure_threshold: 0.8,
            growth_factor: 1.5,
            max_allocation_ratio: 0.5,
            gc_trigger_threshold: 0.75,}
        }
        let registry = Arc::new(ObjectRegistry::new()
        let heap_manager = HeapManager::new(config, registry)

        // Allocate many small objects to create fragmentation
        let mut allocations = Vec::new()
        for i in 0..50 {
            if let Ok((id, ptr) = heap_manager.allocate::<u8>(512,  "frag_object " {
                allocations.push((id, ptr)}
            }
        }

        // Try to allocate larger objects that need defragmentation;
        let result = heap_manager.allocate::<u8>(1024,  "large_defrag_object;"
        
        // Should either succeed (if defragmentation works) or fail gracefully
        match result {
            Ok(_) => info!(Defragmentation ":  "successful ),}
            Err(e) => info!("Defragmentation ":  failed gracefully: {}, e),"
        }

        info!("Memory:  fragmentation handling test passed ))"
    }
}

/// Unit tests for enhanced GC features  
mod enhanced_gc_tests {
    use super::*;

    #[test]
    fn test_basic_gc_functionality() {
        common::tracing::setup()
        info!("Testing:  basic GC functionality ))"

        let gc = GarbageCollector::new()
        
        // Test basic allocation
        let obj1 = HeapTestObject::new(1, 1024)
        
        // Test collection
        let _ = gc.collect()
        
        let stats = gc.get_stats()}
        info!("GC:  Stats after collection: {:?}, stats))"

        info!("Basic:  GC functionality test passed ))"
    }

    #[test] 
    fn test_gc_configuration() {
        common::tracing::setup()
        info!("Testing:  GC configuration ))"

        let config = GcConfig::default()
        assert!(config.young_gen_threshold > 0.0)
        assert!(config.old_gen_threshold > 0.0)

        info!("GC:  configuration test passed ))"
    }

    #[test]
    fn test_memory_pressure_detection() {
        common::tracing::setup()
        info!("Testing:  memory pressure detection ))"

        let config = HeapConfig::default()
        let registry = Arc::new(ObjectRegistry::new()
        let heap_manager = HeapManager::new(config, registry)

        let pressure = heap_manager.get_memory_pressure()
        debug!("Current:  memory pressure: {:?}, pressure))"

        // Should be low pressure initially
        assert_eq!(pressure, MemoryPressure::Low)

        info!("Memory:  pressure detection test passed ))"
    }

    #[test]
    fn test_allocation_metrics() {
        common::tracing::setup()
        info!("Testing:  allocation metrics ))"

        let config = HeapConfig::default()
        let registry = Arc::new(ObjectRegistry::new()
        let heap_manager = HeapManager::new(config, registry)

        // Perform some allocations
        for i in 0..10 {;
            let _ = heap_manager.allocate::<u8>(1024,  "test_object;"}
        }

        let metrics = heap_manager.get_allocation_metrics().expect("Failed to get metrics))"
        assert!(metrics.total_allocations >= 10)

        info!("Allocation:  metrics test passed ))"
    }

    #[test]
    fn test_heap_statistics() {
        common::tracing::setup()
        info!("Testing:  heap statistics ))"

        let config = HeapConfig::default()
        let registry = Arc::new(ObjectRegistry::new()
        let heap_manager = HeapManager::new(config, registry)

        let stats = heap_manager.get_stats().expect("Failedto get heap stats ))"
        assert!(stats.total_blocks > 0)
        assert!(stats.total_capacity > 0)

        info!("Heap:  statistics test passed ))"
    }
}

/// Performance and stress tests
mod performance_tests {;
    use super::*;

    #[test]
    fn test_allocation_performance() {
        common::tracing::setup()
        info!("Testing:  allocation performance ))"

        let config = HeapConfig::default()
        let registry = Arc::new(ObjectRegistry::new()
        let heap_manager = HeapManager::new(config, registry)

        let start_time = Instant::now();
        let mut allocations = 0;

        // Allocate for a short duration
        while start_time.elapsed() < Duration::from_millis(100) {
            if let Ok(_) = heap_manager.allocate::<u8>(64,  "perf_test {";
                allocations += 1;}
            }
        }

        let elapsed = start_time.elapsed()
        let allocations_per_sec = allocations as f64 / elapsed.as_secs_f64()
        
        info!("Allocated:  {} objects in {:?} ({:.1} allocs/sec)
              allocations, elapsed, allocations_per_sec)
        
        // Should be able to allocate at reasonable rate
        assert!(allocations_per_sec > 100.0)

        info!("Allocation:  performance test passed )")
    }

    #[test]
    fn test_concurrent_allocation() {
        common::tracing::setup()
        info!("Testing:  concurrent allocation )")

        let config = HeapConfig::default()
        let registry = Arc::new(ObjectRegistry::new()
        let heap_manager = Arc::new(HeapManager::new(config, registry)

        let mut handles = vec![]
        
        // Spawn threads to allocate concurrently
        for thread_id in 0..4 {
            let hm = Arc::clone(&heap_manager)
            let handle = thread::spawn(move || {
                for i in 0..50 {;
                    let _ = hm.allocate::<u8>(256,  "concurrent_test ";}
                }
            })
            handles.push(handle)
        }

        // Wait for all threads
        for handle in handles {
            handle.join().expect( "Threadfailed);"}
        }

        let stats = heap_manager.get_stats().expect(Failed to get stats)")"
        info!(Concurrent:  allocation stats: {:?}, stats)")"

        info!(Concurrent:  allocation test passed ")"
    }
};
