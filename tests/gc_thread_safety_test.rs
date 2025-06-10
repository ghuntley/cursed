/// Thread Safety Tests for Garbage Collection System
/// 
/// This test suite validates that the GC implementation is properly thread-safe
/// and can handle concurrent operations without data races or memory corruption.

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use cursed::memory::gc::{GarbageCollector, Gc};
use cursed::memory::object_store::Storable;
use cursed::memory::{Traceable, Visitor};

#[derive(Debug)]
struct ThreadSafeTestObject {
    id: u64,
    data: Vec<u8>,
    counter: std::sync::atomic::AtomicU64,
}

impl Traceable for ThreadSafeTestObject {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // No references to trace in this simple test object
    }

unsafe impl Send for TestObject {}
unsafe impl Sync for TestObject {}
}

impl ThreadSafeTestObject {
    fn new(id: u64, size: usize) -> Self {
        Self {
            id,
            data: vec![0u8; size],
            counter: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    fn increment(&self) {
        self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    
    fn get_count(&self) -> u64 {
        self.counter.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[test]
fn test_concurrent_allocation() {
    let gc = Arc::new(GarbageCollector::new();
    let num_threads = 8;
    let allocations_per_thread = 100;
    let barrier = Arc::new(Barrier::new(num_threads));
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let gc_clone = Arc::clone(&gc);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            // Wait for all threads to start
            barrier_clone.wait();
            
            let mut objects = Vec::new();
            
            // Allocate objects concurrently
            for i in 0..allocations_per_thread {
                let obj = ThreadSafeTestObject::new((thread_id * 1000 + i) as u64, 64);
                match gc_clone.allocate(obj) {
                    Ok(gc_ptr) => {
                        objects.push(gc_ptr);
                    }
                    Err(e) => {
                        panic!("Allocation failed: {}", e);
                    }
                }
            }
            
            // Verify all objects are accessible
            for (i, gc_ptr) in objects.iter().enumerate() {
                assert_eq!(gc_ptr.id, (thread_id * 1000 + i) as u64);
                assert!(gc_ptr.is_valid());
            }
            
            objects.len()
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut total_allocated = 0;
    for handle in handles {
        total_allocated += handle.join().expect("Thread panicked");
    }
    
    assert_eq!(total_allocated, num_threads * allocations_per_thread);
    
    // Verify GC stats
    let stats = gc.get_stats().expect("Failed to get GC stats");
    assert_eq!(stats.current_objects, total_allocated);
}

#[test]
fn test_concurrent_gc_collection() {
    let gc = Arc::new(GarbageCollector::new();
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads + 1)); // +1 for GC thread
    
    let mut handles = Vec::new();
    
    // Allocation threads
    for thread_id in 0..num_threads {
        let gc_clone = Arc::clone(&gc);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let mut objects = Vec::new();
            let mut iteration = 0;
            
            // Continuously allocate and occasionally drop objects
            while iteration < 50 {
                // Allocate some objects
                for i in 0..10 {
                    let obj = ThreadSafeTestObject::new((thread_id * 10000 + iteration * 100 + i) as u64, 128);
                    if let Ok(gc_ptr) = gc_clone.allocate(obj) {
                        objects.push(gc_ptr);
                    }
                }
                
                // Drop some objects to create garbage
                if objects.len() > 20 {
                    objects.truncate(10);
                }
                
                // Small delay to allow GC to run
                thread::sleep(Duration::from_millis(1));
                iteration += 1;
            }
            
            objects.len()
        });
        
        handles.push(handle);
    }
    
    // GC thread
    let gc_clone = Arc::clone(&gc);
    let barrier_clone = Arc::clone(&barrier);
    let gc_handle = thread::spawn(move || {
        barrier_clone.wait();
        
        let mut collections = 0;
        
        // Run GC periodically
        while collections < 10 {
            thread::sleep(Duration::from_millis(5));
            
            if gc_clone.should_collect() {
                match gc_clone.collect() {
                    Ok(stats) => {
                        collections += 1;
                        println!("GC cycle {}: collected {} objects in {:?}", 
                                collections, stats.objects_collected, stats.duration);
                    }
                    Err(e) => {
                        eprintln!("GC collection failed: {}", e);
                    }
                }
            }
        }
        
        collections
    });
    
    // Wait for allocation threads
    for handle in handles {
        handle.join().expect("Allocation thread panicked");
    }
    
    // Wait for GC thread
    let total_collections = gc_handle.join().expect("GC thread panicked");
    
    assert!(total_collections > 0, "No garbage collections performed");
    
    // Final stats check
    let final_stats = gc.get_stats().expect("Failed to get final stats");
    assert!(final_stats.total_collections > 0);
    assert!(final_stats.total_objects_collected >= 0);
}

#[test]
fn test_concurrent_reference_counting() {
    let gc = Arc::new(GarbageCollector::new();
    let barrier = Arc::new(Barrier::new(4));
    
    // Create a shared object
    let shared_obj = ThreadSafeTestObject::new(999, 256);
    let shared_gc_ptr = gc.allocate(shared_obj).expect("Failed to allocate shared object");
    let shared_gc_ptr = Arc::new(shared_gc_ptr);
    
    let mut handles = Vec::new();
    
    for thread_id in 0..4 {
        let gc_clone = Arc::clone(&gc);
        let shared_ptr_clone = Arc::clone(&shared_gc_ptr);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            // Each thread clones and uses the shared object
            for _ in 0..100 {
                let local_ptr = shared_ptr_clone.as_ref().clone();
                
                // Access the object
                local_ptr.increment();
                
                // Verify it's still valid
                assert!(local_ptr.is_valid());
                assert_eq!(local_ptr.id, 999);
                
                // Small delay to allow other threads to work
                thread::yield_now();
            }
            
            thread_id
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    // Verify the shared object received increments from all threads
    let final_count = shared_gc_ptr.get_count();
    assert_eq!(final_count, 400); // 4 threads * 100 increments each
    
    // Object should still be valid
    assert!(shared_gc_ptr.is_valid());
}

#[test]
fn test_concurrent_root_management() {
    let gc = Arc::new(GarbageCollector::new();
    let num_threads = 6;
    let barrier = Arc::new(Barrier::new(num_threads));
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let gc_clone = Arc::clone(&gc);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let mut roots = Vec::new();
            
            // Each thread creates some objects and marks them as roots
            for i in 0..20 {
                let obj = ThreadSafeTestObject::new((thread_id * 100 + i) as u64, 64);
                let gc_ptr = gc_clone.allocate(obj).expect("Allocation failed");
                
                // Mark every other object as a root
                if i % 2 == 0 {
                    gc_ptr.mark_as_root().expect("Failed to mark as root");
                    roots.push(gc_ptr);
                }
            }
            
            // Verify root objects are properly marked
            for root in &roots {
                assert!(root.is_valid());
            }
            
            // Unmark some roots
            for (i, root) in roots.iter().enumerate() {
                if i % 2 == 0 {
                    root.unmark_as_root().expect("Failed to unmark root");
                }
            }
            
            roots.len()
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    let mut total_roots = 0;
    for handle in handles {
        total_roots += handle.join().expect("Thread panicked");
    }
    
    assert!(total_roots > 0);
    
    // Get final root count from object store
    let root_objects = gc.object_store().get_root_objects().expect("Failed to get roots");
    
    // Should have roughly half the roots remaining (those not unmarked)
    assert!(root_objects.len() <= total_roots);
}

#[test]
fn test_gc_component_send_sync() {
    // These tests compile-time verify that our types implement Send + Sync
    
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}
    
    // GarbageCollector should be Send + Sync
    is_send::<GarbageCollector>();
    is_sync::<GarbageCollector>();
    
    // Gc<T> should be Send + Sync when T is Storable
    is_send::<Gc<ThreadSafeTestObject>>();
    is_sync::<Gc<ThreadSafeTestObject>>();
    
    // Arc<GarbageCollector> should definitely be Send + Sync
    is_send::<Arc<GarbageCollector>>();
    is_sync::<Arc<GarbageCollector>>();
}

#[test]
fn test_memory_safety_under_load() {
    let gc = Arc::new(GarbageCollector::new();
    let num_threads = 8;
    let barrier = Arc::new(Barrier::new(num_threads));
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let gc_clone = Arc::clone(&gc);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let mut all_objects = Vec::new();
            
            // High-intensity allocation and deallocation
            for round in 0..50 {
                let mut round_objects = Vec::new();
                
                // Allocate many objects
                for i in 0..20 {
                    let obj = ThreadSafeTestObject::new((thread_id * 100000 + round * 1000 + i) as u64, 128);
                    if let Ok(gc_ptr) = gc_clone.allocate(obj) {
                        round_objects.push(gc_ptr);
                    }
                }
                
                // Access all objects to ensure they're valid
                for gc_ptr in &round_objects {
                    assert!(gc_ptr.is_valid());
                    gc_ptr.increment();
                }
                
                // Keep some objects, drop others
                if round % 3 == 0 {
                    all_objects.extend(round_objects);
                } else {
                    // Objects will be dropped and become garbage
                }
                
                // Occasionally trigger GC
                if round % 10 == 0 && gc_clone.should_collect() {
                    let _ = gc_clone.collect();
                }
            }
            
            // Final verification
            for gc_ptr in &all_objects {
                assert!(gc_ptr.is_valid());
                assert!(gc_ptr.get_count() > 0);
            }
            
            all_objects.len()
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    let mut total_surviving = 0;
    for handle in handles {
        total_surviving += handle.join().expect("Thread panicked");
    }
    
    // Run a final GC to clean up
    let final_collection = gc.collect().expect("Final GC failed");
    println!("Final GC: collected {} objects", final_collection.objects_collected);
    
    // Verify system is still in a consistent state
    let final_stats = gc.get_stats().expect("Failed to get final stats");
    assert!(final_stats.current_objects <= total_surviving);
}
