/// Comprehensive GC memory safety stress tests
/// Tests for race conditions, memory corruption, and Send/Sync violations

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use cursed::runtime::gc_write_barrier_safe::*;
use cursed::runtime::heap_optimizer_safe::*;
use cursed::runtime::concurrent_gc::*;
use cursed::memory::Tag;

#[test]
fn test_concurrent_write_barrier_stress() {
    // Initialize write barrier log
    initialize_write_barrier_log().unwrap();
    
    let thread_count = 16;
    let operations_per_thread = 1000;
    let barrier = Arc::new(Barrier::new(thread_count));
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            // Wait for all threads to be ready
            barrier.wait();
            
            // Perform write barrier operations
            for i in 0..operations_per_thread {
                let source_addr = (thread_id * 10000 + i * 100) as usize;
                let target_addr = source_addr + 64;
                
                record_write_barrier(source_addr, target_addr, 0).unwrap();
                
                // Simulate concurrent allocation/deallocation
                if i % 10 == 0 {
                    thread::sleep(Duration::from_nanos(100));
                }
            }
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Cleanup
    if let Some(log_arc) = get_write_barrier_log() {
        if let Ok(mut log) = log_arc.lock() {
            log.stop().unwrap();
        }
    }
}

#[test]
fn test_heap_optimizer_concurrent_allocation() {
    let config = SafeHeapOptimizerConfig {
        thread_local_buffers: true,
        tlab_size: 64 * 1024, // 64KB TLABs
        max_threads: 32,
        ..Default::default()
    };
    
    let optimizer = Arc::new(SafeHeapOptimizer::new(config).unwrap());
    optimizer.start().unwrap();
    
    let thread_count = 8;
    let allocations_per_thread = 500;
    let barrier = Arc::new(Barrier::new(thread_count));
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        let optimizer = Arc::clone(&optimizer);
        let barrier = Arc::clone(&barrier);
        
        thread::spawn(move || {
            // Wait for all threads to be ready
            barrier.wait();
            
            let mut allocated_ptrs = Vec::new();
            
            // Allocation phase
            for i in 0..allocations_per_thread {
                let size = 32 + (i % 256); // Variable sizes
                let ptr = optimizer.allocate(size, 8, Tag::Object).unwrap();
                assert!(!ptr.is_null());
                allocated_ptrs.push(ptr);
                
                // Trigger write barriers during allocation
                if i % 50 == 0 {
                    for j in 0..allocated_ptrs.len().min(10) {
                        let source = allocated_ptrs[j].as_ptr() as usize;
                        let target = if j + 1 < allocated_ptrs.len() {
                            allocated_ptrs[j + 1].as_ptr() as usize
                        } else {
                            source + 64
                        };
                        record_write_barrier(source, target, 0).unwrap();
                    }
                }
            }
            
            // Deallocation phase
            for ptr in allocated_ptrs {
                optimizer.deallocate(&ptr).unwrap();
            }
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    optimizer.stop().unwrap();
    
    // Verify statistics
    let stats = optimizer.get_stats().unwrap();
    assert_eq!(stats.total_allocations as usize, thread_count * allocations_per_thread);
    assert_eq!(stats.total_deallocations as usize, thread_count * allocations_per_thread);
    assert_eq!(stats.current_heap_usage, 0); // All deallocated
}

#[test]
fn test_gc_write_barrier_race_conditions() {
    // Test for race conditions in write barrier with rapid object updates
    let log = Arc::new(std::sync::Mutex::new(ThreadSafeWriteBarrierLog::new()));
    {
        let mut log_guard = log.lock().unwrap();
        log_guard.start_worker().unwrap();
    }
    
    let thread_count = 12;
    let object_count = 100;
    let update_cycles = 200;
    
    // Create shared objects (simulated as addresses)
    let objects: Vec<usize> = (0..object_count).map(|i| (i + 1) * 0x1000).collect();
    let objects_arc = Arc::new(objects);
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        let objects = Arc::clone(&objects_arc);
        let log = Arc::clone(&log);
        
        thread::spawn(move || {
            for cycle in 0..update_cycles {
                for i in 0..object_count {
                    let source_idx = i;
                    let target_idx = (i + 1 + cycle) % object_count;
                    
                    let source_addr = objects[source_idx];
                    let target_addr = objects[target_idx];
                    
                    // Record write barrier
                    if let Ok(log_guard) = log.lock() {
                        log_guard.record_write_barrier(source_addr, target_addr, 0).unwrap();
                    }
                    
                    // Simulate memory pressure
                    if cycle % 10 == 0 && thread_id == 0 {
                        thread::sleep(Duration::from_micros(10));
                    }
                }
            }
        })
    }).collect();
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Cleanup
    {
        let mut log_guard = log.lock().unwrap();
        log_guard.stop().unwrap();
    }
}

#[test]
fn test_memory_corruption_detection() {
    // Test to detect memory corruption under concurrent GC
    let config = SafeHeapOptimizerConfig {
        thread_local_buffers: true,
        large_object_threshold: 1024,
        ..Default::default()
    };
    
    let optimizer = Arc::new(SafeHeapOptimizer::new(config).unwrap());
    optimizer.start().unwrap();
    
    let thread_count = 6;
    let test_duration = Duration::from_millis(500);
    let barrier = Arc::new(Barrier::new(thread_count));
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        let optimizer = Arc::clone(&optimizer);
        let barrier = Arc::clone(&barrier);
        
        thread::spawn(move || {
            barrier.wait();
            
            let start_time = std::time::Instant::now();
            let mut allocated_objects = Vec::new();
            let mut allocation_counter = 0;
            
            while start_time.elapsed() < test_duration {
                // Allocate objects of varying sizes
                let size = match thread_id % 4 {
                    0 => 64,      // Small objects
                    1 => 256,     // Medium objects
                    2 => 1024,    // Large objects
                    _ => 4096,    // Very large objects
                };
                
                if let Ok(ptr) = optimizer.allocate(size, 8, Tag::Object) {
                    // Write pattern to detect corruption
                    if let Some(raw_ptr) = ptr.as_non_null() {
                        unsafe {
                            let slice = std::slice::from_raw_parts_mut(raw_ptr.as_ptr(), size);
                            for (i, byte) in slice.iter_mut().enumerate() {
                                *byte = ((allocation_counter + i) % 256) as u8;
                            }
                        }
                    }
                    
                    allocated_objects.push((ptr, allocation_counter));
                    allocation_counter += 1;
                    
                    // Periodically verify and deallocate old objects
                    if allocated_objects.len() > 50 {
                        let (old_ptr, old_counter) = allocated_objects.remove(0);
                        
                        // Verify pattern before deallocation
                        if let Some(raw_ptr) = old_ptr.as_non_null() {
                            unsafe {
                                let slice = std::slice::from_raw_parts(raw_ptr.as_ptr(), old_ptr.size());
                                for (i, &byte) in slice.iter().enumerate() {
                                    let expected = ((old_counter + i) % 256) as u8;
                                    assert_eq!(byte, expected, 
                                        "Memory corruption detected at offset {} in object allocated at counter {}",
                                        i, old_counter);
                                }
                            }
                        }
                        
                        optimizer.deallocate(&old_ptr).unwrap();
                    }
                    
                    // Trigger write barriers for inter-object references
                    if allocated_objects.len() > 1 {
                        let idx1 = allocation_counter % allocated_objects.len();
                        let idx2 = (allocation_counter + 1) % allocated_objects.len();
                        
                        let source_addr = allocated_objects[idx1].0.as_ptr() as usize;
                        let target_addr = allocated_objects[idx2].0.as_ptr() as usize;
                        
                        record_write_barrier(source_addr, target_addr, 0).unwrap();
                    }
                }
                
                // Occasional yield to allow GC
                if allocation_counter % 20 == 0 {
                    thread::yield_now();
                }
            }
            
            // Final cleanup and verification
            for (ptr, counter) in allocated_objects {
                // Final verification
                if let Some(raw_ptr) = ptr.as_non_null() {
                    unsafe {
                        let slice = std::slice::from_raw_parts(raw_ptr.as_ptr(), ptr.size());
                        for (i, &byte) in slice.iter().enumerate() {
                            let expected = ((counter + i) % 256) as u8;
                            assert_eq!(byte, expected, 
                                "Memory corruption detected during cleanup at offset {} in object {}",
                                i, counter);
                        }
                    }
                }
                
                optimizer.deallocate(&ptr).unwrap();
            }
        })
    }).collect();
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    optimizer.stop().unwrap();
}

#[test]
fn test_send_sync_violations() {
    // Test that our types properly implement Send and Sync
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    
    // These should compile without errors
    assert_send::<SafePointer>();
    assert_sync::<SafePointer>();
    assert_send::<SafeThreadLocalBuffer>();
    assert_sync::<SafeThreadLocalBuffer>();
    assert_send::<SafeHeapOptimizer>();
    assert_sync::<SafeHeapOptimizer>();
    
    // Test actual cross-thread usage
    let config = SafeHeapOptimizerConfig::default();
    let optimizer = Arc::new(SafeHeapOptimizer::new(config).unwrap());
    optimizer.start().unwrap();
    
    let ptr = optimizer.allocate(64, 8, Tag::Object).unwrap();
    
    // Send pointer to another thread
    let handle = thread::spawn(move || {
        assert!(!ptr.is_null());
        assert_eq!(ptr.size(), 64);
        ptr
    });
    
    let returned_ptr = handle.join().unwrap();
    optimizer.deallocate(&returned_ptr).unwrap();
    optimizer.stop().unwrap();
}

#[test]
fn test_high_frequency_allocation_deallocation() {
    // Stress test with very high frequency allocation/deallocation
    let config = SafeHeapOptimizerConfig {
        tlab_size: 32 * 1024, // Smaller TLABs for more frequent allocation
        ..Default::default()
    };
    
    let optimizer = Arc::new(SafeHeapOptimizer::new(config).unwrap());
    optimizer.start().unwrap();
    
    let thread_count = 4;
    let operations_per_thread = 10000;
    let barrier = Arc::new(Barrier::new(thread_count));
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        let optimizer = Arc::clone(&optimizer);
        let barrier = Arc::clone(&barrier);
        
        thread::spawn(move || {
            barrier.wait();
            
            for i in 0..operations_per_thread {
                // Rapid allocation/deallocation cycles
                let size = 16 + (i % 128);
                let ptr = optimizer.allocate(size, 8, Tag::Object).unwrap();
                
                // Immediate deallocation to stress the allocator
                optimizer.deallocate(&ptr).unwrap();
                
                // Trigger write barriers occasionally
                if i % 100 == 0 {
                    record_write_barrier(
                        (thread_id * 1000 + i) as usize,
                        (thread_id * 1000 + i + 1) as usize,
                        0
                    ).unwrap();
                }
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    optimizer.stop().unwrap();
    
    let stats = optimizer.get_stats().unwrap();
    assert_eq!(stats.total_allocations as usize, thread_count * operations_per_thread);
    assert_eq!(stats.total_deallocations as usize, thread_count * operations_per_thread);
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_allocation_sizes_property(size in 1usize..10240) {
            let config = SafeHeapOptimizerConfig::default();
            let optimizer = SafeHeapOptimizer::new(config).unwrap();
            optimizer.start().unwrap();
            
            let ptr = optimizer.allocate(size, 8, Tag::Object).unwrap();
            assert!(!ptr.is_null());
            assert!(ptr.size() >= size); // May be aligned up
            
            optimizer.deallocate(&ptr).unwrap();
            optimizer.stop().unwrap();
        }
        
        #[test]
        fn test_write_barrier_addresses_property(
            source in 0x1000usize..0x100000,
            target in 0x1000usize..0x100000,
            offset in 0usize..1024
        ) {
            // Property: write barrier should handle any valid address range
            let result = record_write_barrier(source, target, offset);
            prop_assert!(result.is_ok());
        }
    }
}
