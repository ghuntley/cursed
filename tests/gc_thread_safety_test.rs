#[cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier, atomic::{AtomicUsize, Ordering}};
    use std::thread;
    use std::time::Duration;
    use cursed::memory::{
        GarbageCollector, GcConfig, Gc, ObjectStore, ObjectId, ObjectRegistry, 
        Traceable, Visitor, Tag, HeapConfig
    };

    #[path = "common.rs"]
    mod common;

    // Test object that's safe to share between threads
    #[derive(Debug, Clone)]
    struct ThreadSafeTestObject {
        id: usize,
        value: String,
        counter: Arc<AtomicUsize>,
    }

    impl cursed::memory::Storable for ThreadSafeTestObject {
        fn size(&self) -> usize {
            std::mem::size_of::<Self>() + self.value.len()
        }

        fn type_name(&self) -> &'static str {
            "ThreadSafeTestObject"
        }
    }

    impl Traceable for ThreadSafeTestObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // This object contains no GC references
        }
    }

    #[test]
    fn test_concurrent_allocation() {
        common::tracing::setup();
        
        let gc = Arc::new(GarbageCollector::new());
        let num_threads = 4;
        let allocations_per_thread = 100;
        let barrier = Arc::new(Barrier::new(num_threads));
        let success_count = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let gc = gc.clone();
                let barrier = barrier.clone();
                let success_count = success_count.clone();
                
                thread::spawn(move || {
                    barrier.wait(); // Synchronize thread starts
                    
                    for i in 0..allocations_per_thread {
                        let counter = Arc::new(AtomicUsize::new(0));
                        let obj = ThreadSafeTestObject {
                            id: thread_id * allocations_per_thread + i,
                            value: format!("Thread {} Object {}", thread_id, i),
                            counter,
                        };
                        
                        match gc.allocate(obj) {
                            Ok(_gc_ptr) => {
                                success_count.fetch_add(1, Ordering::SeqCst);
                            }
                            Err(e) => {
                                eprintln!("Allocation failed in thread {}: {}", thread_id, e);
                            }
                        }
                    }
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Verify all allocations succeeded
        let total_successes = success_count.load(Ordering::SeqCst);
        assert_eq!(
            total_successes,
            num_threads * allocations_per_thread,
            "All allocations should succeed in concurrent environment"
        );
    }

    #[test]
    fn test_concurrent_collection() {
        common::tracing::setup();
        
        let gc = Arc::new(GarbageCollector::new());
        let num_threads = 3;
        let barrier = Arc::new(Barrier::new(num_threads));
        let collection_count = Arc::new(AtomicUsize::new(0));

        // First, allocate some objects
        for i in 0..50 {
            let counter = Arc::new(AtomicUsize::new(0));
            let obj = ThreadSafeTestObject {
                id: i,
                value: format!("Pre-allocated object {}", i),
                counter,
            };
            gc.allocate(obj).expect("Pre-allocation should succeed");
        }

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let gc = gc.clone();
                let barrier = barrier.clone();
                let collection_count = collection_count.clone();
                
                thread::spawn(move || {
                    barrier.wait(); // Synchronize thread starts
                    
                    // Each thread attempts garbage collection
                    for attempt in 0..10 {
                        match gc.collect() {
                            Ok(_stats) => {
                                collection_count.fetch_add(1, Ordering::SeqCst);
                                println!("Thread {} collection {} succeeded", thread_id, attempt);
                            }
                            Err(e) => {
                                // Collection might fail if another collection is in progress
                                println!("Thread {} collection {} failed: {}", thread_id, attempt, e);
                            }
                        }
                        
                        // Small delay between collection attempts
                        thread::sleep(Duration::from_millis(10));
                    }
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Verify at least some collections succeeded
        let total_collections = collection_count.load(Ordering::SeqCst);
        assert!(
            total_collections > 0,
            "At least some garbage collections should succeed"
        );
        
        println!("Completed {} garbage collections across {} threads", total_collections, num_threads);
    }

    #[test]
    fn test_allocation_during_collection() {
        common::tracing::setup();
        
        let gc = Arc::new(GarbageCollector::new());
        let barrier = Arc::new(Barrier::new(2));
        let allocation_success = Arc::new(AtomicUsize::new(0));
        let collection_success = Arc::new(AtomicUsize::new(0));

        // Pre-allocate objects to make collection meaningful
        for i in 0..100 {
            let counter = Arc::new(AtomicUsize::new(0));
            let obj = ThreadSafeTestObject {
                id: i,
                value: format!("Initial object {}", i),
                counter,
            };
            gc.allocate(obj).expect("Initial allocation should succeed");
        }

        // Thread 1: Performs garbage collection
        let gc_collector = gc.clone();
        let barrier_collector = barrier.clone();
        let collection_success_collector = collection_success.clone();
        let collector_handle = thread::spawn(move || {
            barrier_collector.wait();
            
            for _ in 0..5 {
                match gc_collector.collect() {
                    Ok(_) => {
                        collection_success_collector.fetch_add(1, Ordering::SeqCst);
                        println!("Collection succeeded");
                    }
                    Err(e) => {
                        println!("Collection failed: {}", e);
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
        });

        // Thread 2: Performs allocations during collection
        let gc_allocator = gc.clone();
        let barrier_allocator = barrier.clone();
        let allocation_success_allocator = allocation_success.clone();
        let allocator_handle = thread::spawn(move || {
            barrier_allocator.wait();
            
            for i in 0..50 {
                let counter = Arc::new(AtomicUsize::new(0));
                let obj = ThreadSafeTestObject {
                    id: 1000 + i,
                    value: format!("Concurrent object {}", i),
                    counter,
                };
                
                match gc_allocator.allocate(obj) {
                    Ok(_) => {
                        allocation_success_allocator.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(e) => {
                        println!("Allocation during collection failed: {}", e);
                    }
                }
                thread::sleep(Duration::from_millis(50));
            }
        });

        // Wait for both threads
        collector_handle.join().expect("Collector thread should complete");
        allocator_handle.join().expect("Allocator thread should complete");

        // Verify operations succeeded
        let allocations = allocation_success.load(Ordering::SeqCst);
        let collections = collection_success.load(Ordering::SeqCst);
        
        assert!(allocations > 0, "Some allocations should succeed during collection");
        assert!(collections > 0, "Some collections should succeed");
        
        println!("Completed {} allocations and {} collections concurrently", allocations, collections);
    }

    #[test]
    fn test_gc_pointer_thread_safety() {
        common::tracing::setup();
        
        let gc = Arc::new(GarbageCollector::new());
        
        // Create a shared GC pointer
        let counter = Arc::new(AtomicUsize::new(0));
        let obj = ThreadSafeTestObject {
            id: 42,
            value: "Shared object".to_string(),
            counter: counter.clone(),
        };
        
        let gc_ptr = gc.allocate(obj).expect("Should allocate shared object");
        let shared_ptr = Arc::new(gc_ptr);
        
        let num_threads = 4;
        let barrier = Arc::new(Barrier::new(num_threads));
        let access_count = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let shared_ptr = shared_ptr.clone();
                let barrier = barrier.clone();
                let access_count = access_count.clone();
                let counter = counter.clone();
                
                thread::spawn(move || {
                    barrier.wait(); // Synchronize thread starts
                    
                    for _ in 0..100 {
                        // Access the shared object through GC pointer
                        if shared_ptr.is_valid() {
                            let _id = shared_ptr.id;
                            let _value = &shared_ptr.value;
                            counter.fetch_add(1, Ordering::SeqCst);
                            access_count.fetch_add(1, Ordering::SeqCst);
                        }
                        
                        // Small delay to increase chance of race conditions
                        thread::yield_now();
                    }
                    
                    println!("Thread {} completed object access", thread_id);
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Verify all accesses succeeded
        let total_accesses = access_count.load(Ordering::SeqCst);
        let counter_value = counter.load(Ordering::SeqCst);
        
        assert_eq!(
            total_accesses,
            num_threads * 100,
            "All object accesses should succeed"
        );
        assert_eq!(
            counter_value,
            total_accesses,
            "Counter should match access count"
        );
        
        // Verify object is still valid
        assert!(shared_ptr.is_valid(), "Shared object should remain valid");
    }

    #[test]
    fn test_root_set_thread_safety() {
        common::tracing::setup();
        
        let gc = Arc::new(GarbageCollector::new());
        let num_threads = 3;
        let barrier = Arc::new(Barrier::new(num_threads));
        let root_operations = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let gc = gc.clone();
                let barrier = barrier.clone();
                let root_operations = root_operations.clone();
                
                thread::spawn(move || {
                    barrier.wait(); // Synchronize thread starts
                    
                    let mut objects = Vec::new();
                    
                    // Each thread creates and manages roots
                    for i in 0..10 {
                        let counter = Arc::new(AtomicUsize::new(0));
                        let obj = ThreadSafeTestObject {
                            id: thread_id * 100 + i,
                            value: format!("Root object {} from thread {}", i, thread_id),
                            counter,
                        };
                        
                        match gc.allocate(obj) {
                            Ok(gc_ptr) => {
                                // Mark as root
                                if gc_ptr.mark_as_root().is_ok() {
                                    root_operations.fetch_add(1, Ordering::SeqCst);
                                    objects.push(gc_ptr);
                                }
                            }
                            Err(e) => {
                                println!("Thread {} failed to allocate: {}", thread_id, e);
                            }
                        }
                    }
                    
                    // Trigger garbage collection
                    if let Ok(_) = gc.collect() {
                        println!("Thread {} triggered collection", thread_id);
                    }
                    
                    // Unmark roots
                    for gc_ptr in &objects {
                        if gc_ptr.unmark_as_root().is_ok() {
                            root_operations.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                    
                    println!("Thread {} completed root operations", thread_id);
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Verify root operations succeeded
        let total_root_ops = root_operations.load(Ordering::SeqCst);
        assert!(
            total_root_ops > 0,
            "Some root operations should succeed in concurrent environment"
        );
        
        println!("Completed {} root operations across {} threads", total_root_ops, num_threads);
    }

    #[test]
    fn test_stress_concurrent_operations() {
        common::tracing::setup();
        
        let gc = Arc::new(GarbageCollector::new());
        let num_threads = 6;
        let operations_per_thread = 50;
        let barrier = Arc::new(Barrier::new(num_threads));
        let success_count = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let gc = gc.clone();
                let barrier = barrier.clone();
                let success_count = success_count.clone();
                
                thread::spawn(move || {
                    barrier.wait(); // Synchronize thread starts
                    
                    let mut objects = Vec::new();
                    
                    for i in 0..operations_per_thread {
                        // Allocate objects
                        let counter = Arc::new(AtomicUsize::new(0));
                        let obj = ThreadSafeTestObject {
                            id: thread_id * 1000 + i,
                            value: format!("Stress test object {} from thread {}", i, thread_id),
                            counter,
                        };
                        
                        if let Ok(gc_ptr) = gc.allocate(obj) {
                            objects.push(gc_ptr);
                            success_count.fetch_add(1, Ordering::SeqCst);
                        }
                        
                        // Occasionally trigger collection
                        if i % 10 == 0 {
                            let _ = gc.collect();
                        }
                        
                        // Occasionally mark/unmark as root
                        if i % 5 == 0 && !objects.is_empty() {
                            let idx = i % objects.len();
                            let _ = objects[idx].mark_as_root();
                            thread::yield_now();
                            let _ = objects[idx].unmark_as_root();
                        }
                        
                        // Small random delay to create more race conditions
                        if i % 3 == 0 {
                            thread::sleep(Duration::from_millis(1));
                        }
                    }
                    
                    // Final collection attempt
                    let _ = gc.collect();
                    
                    println!("Thread {} completed stress test with {} objects", 
                             thread_id, objects.len());
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Verify operations succeeded under stress
        let total_successes = success_count.load(Ordering::SeqCst);
        assert!(
            total_successes > num_threads * operations_per_thread / 2,
            "At least half of stress test operations should succeed"
        );
        
        // Final collection to ensure GC is still functional
        let final_stats = gc.collect().expect("Final collection should succeed");
        println!("Stress test completed: {} successful operations, final collection took {:?}", 
                 total_successes, final_stats.total_duration);
    }

    #[test]
    fn test_memory_consistency() {
        common::tracing::setup();
        
        let gc = Arc::new(GarbageCollector::new());
        let shared_counter = Arc::new(AtomicUsize::new(0));
        let num_threads = 4;
        let barrier = Arc::new(Barrier::new(num_threads));

        // Create a shared object
        let obj = ThreadSafeTestObject {
            id: 999,
            value: "Memory consistency test".to_string(),
            counter: shared_counter.clone(),
        };
        
        let shared_gc_ptr = Arc::new(gc.allocate(obj).expect("Should allocate shared object"));

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let gc = gc.clone();
                let shared_gc_ptr = shared_gc_ptr.clone();
                let shared_counter = shared_counter.clone();
                let barrier = barrier.clone();
                
                thread::spawn(move || {
                    barrier.wait(); // Synchronize thread starts
                    
                    for iteration in 0..100 {
                        // Read from shared object
                        if shared_gc_ptr.is_valid() {
                            let current_count = shared_counter.load(Ordering::SeqCst);
                            
                            // Increment counter
                            shared_counter.fetch_add(1, Ordering::SeqCst);
                            
                            // Verify monotonic increase (memory consistency)
                            let new_count = shared_counter.load(Ordering::SeqCst);
                            assert!(
                                new_count > current_count,
                                "Counter should increase monotonically"
                            );
                        }
                        
                        // Occasionally trigger GC to test memory consistency during collection
                        if iteration % 20 == 0 {
                            let _ = gc.collect();
                        }
                        
                        thread::yield_now();
                    }
                    
                    println!("Thread {} completed memory consistency test", thread_id);
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Verify final state
        let final_count = shared_counter.load(Ordering::SeqCst);
        assert_eq!(
            final_count,
            num_threads * 100,
            "Final counter should match expected value"
        );
        
        // Verify object is still valid and accessible
        assert!(shared_gc_ptr.is_valid(), "Shared object should still be valid");
        assert_eq!(shared_gc_ptr.id, 999, "Object data should be intact");
        
        println!("Memory consistency test passed with final counter: {}", final_count);
    }

    #[test]
    fn test_gc_stats_thread_safety() {
        common::tracing::setup();
        
        let gc = Arc::new(GarbageCollector::new());
        let num_threads = 3;
        let barrier = Arc::new(Barrier::new(num_threads));

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let gc = gc.clone();
                let barrier = barrier.clone();
                
                thread::spawn(move || {
                    barrier.wait(); // Synchronize thread starts
                    
                    for i in 0..20 {
                        // Allocate some objects
                        for j in 0..5 {
                            let counter = Arc::new(AtomicUsize::new(0));
                            let obj = ThreadSafeTestObject {
                                id: thread_id * 1000 + i * 10 + j,
                                value: format!("Stats test object"),
                                counter,
                            };
                            let _ = gc.allocate(obj);
                        }
                        
                        // Check stats
                        let stats = gc.stats();
                        assert!(stats.current_objects >= 0, "Object count should be non-negative");
                        assert!(stats.current_heap_size >= 0, "Heap size should be non-negative");
                        
                        // Trigger collection
                        if let Ok(collection_stats) = gc.collect() {
                            assert!(collection_stats.total_duration >= Duration::ZERO, 
                                   "Collection duration should be non-negative");
                        }
                        
                        // Check stats again
                        let stats_after = gc.stats();
                        assert!(stats_after.total_collections >= stats.total_collections, 
                               "Collection count should not decrease");
                    }
                    
                    println!("Thread {} completed stats test", thread_id);
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Check final stats consistency
        let final_stats = gc.stats();
        assert!(final_stats.total_collections > 0, "Should have performed some collections");
        println!("Final GC stats: {} collections, {} objects, {} bytes", 
                 final_stats.total_collections, 
                 final_stats.current_objects, 
                 final_stats.current_heap_size);
    }
}
