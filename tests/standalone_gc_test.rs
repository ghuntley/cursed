//! Tests for the improved garbage collector implementation with proper weak reference support

use std::sync::Arc;

use cursed::memory::gc::{GarbageCollector, MemoryStats};
use cursed::memory::{Gc, Tag, Traceable, Visitor, with_gc_scope};

// Test object with proper finalization and tracing
#[derive(Clone, Debug)]
struct TestObject {
    id: usize,
    value: String,
    next: Option<Gc<TestObject>>,
    has_been_finalized: bool,
}

impl TestObject {
    fn new(id: usize, value: impl Into<String>) -> Self {
        Self { 
            id, 
            value: value.into(), 
            next: None, 
            has_been_finalized: false 
        }
    }
    
    fn set_next(&mut self, next: Gc<TestObject>) {
        self.next = Some(next);
    }
}

impl Traceable for TestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(next) = &self.next {
            if let Some(inner) = next.inner() {
                unsafe {
                    let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut TestObject);
                    visitor.visit(ptr);
                }
            }
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
    
    fn finalize(&mut self) {
        println!("Finalizing TestObject id={} value={}", self.id, self.value);
        self.has_been_finalized = true;
    }
}

#[test]
fn test_weak_reference_gc_connection() {
    println!("u{2193} Starting weak reference GC connection test u{2193}");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a scope for root tracking
    let _scope_guard = with_gc_scope(gc.clone());
    
    // Create a test object
    let test_obj = gc.allocate(TestObject::new(1, "test value"));
    
    // Create a weak reference
    let weak_ref = test_obj.downgrade();
    
    // Verify the weak reference is alive
    assert!(weak_ref.is_alive(), "Weak reference should be alive");
    
    // Drop the strong reference
    drop(test_obj);
    
    // Force garbage collection
    gc.collect_garbage();
    
    // Verify the weak reference is no longer alive
    // This should now work with our improved implementation
    assert!(!weak_ref.is_alive(), "Weak reference should no longer be alive after collection");
    
    println!("u{2193} Completed weak reference GC connection test u{2193}");
}

#[test]
fn test_object_finalization() {
    println!("u{2193} Starting object finalization test u{2193}");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a scope for root tracking
    let _scope_guard = with_gc_scope(gc.clone());
    
    // Create a test object that we can track finalization on
    let mut test_obj = gc.allocate(TestObject::new(1, "finalization test"));
    
    // Ensure we can access the object before finalization
    assert_eq!(test_obj.inner().unwrap().value, "finalization test");
    assert_eq!(test_obj.inner().unwrap().has_been_finalized, false);
    
    // Create a weak reference to track the object
    let weak_ref = test_obj.downgrade();
    
    // Drop the strong reference to make the object eligible for collection
    drop(test_obj);
    
    // Force garbage collection which should trigger finalization
    gc.collect_garbage();
    
    // The object should be finalized and collected
    assert!(!weak_ref.is_alive(), "Object should be collected after finalization");
    
    println!("u{2193} Completed object finalization test u{2193}");
}

#[test]
fn test_circular_references_with_finalization() {
    println!("u{2193} Starting circular references with finalization test u{2193}");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a scope for root tracking
    let _scope_guard = with_gc_scope(gc.clone());
    
    // Create a cycle: obj1 -> obj2 -> obj3 -> obj1
    let mut obj1 = gc.allocate(TestObject::new(1, "circular1"));
    let mut obj2 = gc.allocate(TestObject::new(2, "circular2"));
    let mut obj3 = gc.allocate(TestObject::new(3, "circular3"));
    
    // Create circular references
    obj1.inner_mut().unwrap().set_next(obj2.clone());
    obj2.inner_mut().unwrap().set_next(obj3.clone());
    obj3.inner_mut().unwrap().set_next(obj1.clone());
    
    // Get initial stats
    let initial_stats = gc.stats();
    println!("Initial stats: {:?}", initial_stats);
    assert!(initial_stats.object_count >= 3, "Should have at least 3 objects");
    
    // Create weak references to track the objects
    let weak1 = obj1.downgrade();
    let weak2 = obj2.downgrade();
    let weak3 = obj3.downgrade();
    
    // Verify all weak references are alive
    assert!(weak1.is_alive(), "weak1 should be alive");
    assert!(weak2.is_alive(), "weak2 should be alive");
    assert!(weak3.is_alive(), "weak3 should be alive");
    
    // Drop all strong references
    drop(obj1);
    drop(obj2);
    drop(obj3);
    
    // Force garbage collection
    gc.collect_garbage();
    
    // Check final stats
    let final_stats = gc.stats();
    println!("Final stats: {:?}", final_stats);
    
    // With our improved collector, the circular references should be collected
    assert!(!weak1.is_alive(), "weak1 should not be alive after collection");
    assert!(!weak2.is_alive(), "weak2 should not be alive after collection");
    assert!(!weak3.is_alive(), "weak3 should not be alive after collection");
    
    println!("u{2193} Completed circular references with finalization test u{2193}");
}

#[test]
fn test_incremental_collection_with_finalization() {
    println!("u{2193} Starting incremental collection with finalization test u{2193}");
    
    // Create a garbage collector with incremental collection enabled
    let gc = Arc::new(GarbageCollector::with_options(cursed::memory::gc::GcOptions {
        initial_heap_size: 4096,
        allocation_threshold: 10,           // Trigger collection after 10 allocations
        incremental_step_size: 2,           // Process 2 objects per step
        incremental_time_budget_ms: 10,     // 10ms per incremental step
        verbose: true,
    }));
    
    // Create a scope for root tracking
    let _scope_guard = with_gc_scope(gc.clone());
    
    // Create a set of objects with connections
    let mut objects = Vec::new();
    for i in 0..20 {
        objects.push(gc.allocate(TestObject::new(i, format!("object-{}", i))));
    }
    
    // Create some connections (including cycles)
    for i in 0..objects.len() {
        // Connect each object to the next one in a ring structure
        let next_idx = (i + 1) % objects.len();
        objects[i].inner_mut().unwrap().set_next(objects[next_idx].clone());
    }
    
    // Create weak references to track object lifetime
    let weak_refs: Vec<_> = objects.iter().map(|obj| obj.downgrade()).collect();
    
    // Drop all strong references
    drop(objects);
    
    // Run multiple incremental collection steps
    for i in 0..15 {
        println!("Running incremental collection step {}", i+1);
        gc.collect_garbage_incremental();
    }
    
    // Check if all objects are eventually collected
    let mut alive_count = 0;
    for (i, weak) in weak_refs.iter().enumerate() {
        if weak.is_alive() {
            alive_count += 1;
            println!("Object {} is still alive", i);
        }
    }
    
    println!("{} objects still alive after incremental collection", alive_count);
    
    // Run a final full collection to ensure everything is collected
    gc.collect_garbage();
    
    // Verify all objects are now collected
    let mut final_alive_count = 0;
    for weak in &weak_refs {
        if weak.is_alive() {
            final_alive_count += 1;
        }
    }
    
    println!("{} objects still alive after full collection", final_alive_count);
    assert_eq!(final_alive_count, 0, "All objects should be collected");
    
    // Check final stats
    let final_stats = gc.stats();
    println!("Final stats: {:?}", final_stats);
    
    println!("u{2193} Completed incremental collection with finalization test u{2193}");
}