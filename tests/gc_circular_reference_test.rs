//! Test for circular reference handling in the garbage collector

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cursed::memory::gc::{GarbageCollector, MemoryStats};
use cursed::memory::{Gc, Tag, Traceable, Visitor};

// Simple struct that holds a reference to another GC-managed object
#[derive(Clone)]
struct CircularNode {
    id: usize,
    next: Option<Gc<CircularNode>>,
}

impl CircularNode {
    fn new(id: usize) -> Self {
        Self { id, next: None }
    }
    
    fn set_next(&mut self, next: Gc<CircularNode>) {
        self.next = Some(next);
    }
}

impl Traceable for CircularNode {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(next) = &self.next {
            if let Some(inner) = next.inner() {
                unsafe {
                    let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut CircularNode);
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
}

#[test]
fn test_circular_references() {
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Allocate objects with circular references
    let mut node1 = gc.allocate(CircularNode::new(1));
    let mut node2 = gc.allocate(CircularNode::new(2));
    let mut node3 = gc.allocate(CircularNode::new(3));
    
    // Set up circular references: 1 -> 2 -> 3 -> 1
    if let Some(node1_inner) = node1.inner_mut() {
        node1_inner.set_next(node2.clone());
    }
    
    if let Some(node2_inner) = node2.inner_mut() {
        node2_inner.set_next(node3.clone());
    }
    
    if let Some(node3_inner) = node3.inner_mut() {
        node3_inner.set_next(node1.clone());
    }
    
    // Get initial stats
    let initial_stats = gc.stats();
    assert_eq!(initial_stats.object_count, 3, "Should have 3 objects initially");
    
    // Let nodes go out of scope, which should drop the Gc references
    drop(node1);
    drop(node2);
    drop(node3);
    
    // Force a garbage collection
    gc.collect_garbage();
    
    // Get updated stats
    let final_stats = gc.stats();
    
    // Objects should be collected since there are no external references
    // left despite the circular references between them
    assert_eq!(final_stats.object_count, 0, "All objects should be collected");
    assert!(final_stats.freed_objects >= 3, "At least 3 objects should be freed");
}

#[test]
fn test_weak_references() {
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a scope to test object cleanup
    {
        // Allocate an object
        let node = gc.allocate(CircularNode::new(42));
        
        // Create a weak reference to it
        let weak_node = node.downgrade();
        
        // Weak reference should be alive
        assert!(weak_node.is_alive(), "Weak reference should be alive");
        
        // Upgrading should work
        let upgraded = weak_node.upgrade();
        assert!(upgraded.is_some(), "Should be able to upgrade weak reference");
        
        // Let the strong reference go out of scope
        drop(node);
        drop(upgraded);
        
        // Force a garbage collection
        gc.collect_garbage();
        
        // Now the weak reference should not be alive
        assert!(!weak_node.is_alive(), "Weak reference should not be alive after collection");
        
        // Upgrading should fail
        let upgraded_after_collection = weak_node.upgrade();
        assert!(upgraded_after_collection.is_none(), "Should not be able to upgrade after collection");
    }
    
    // Get final stats
    let final_stats = gc.stats();
    assert_eq!(final_stats.object_count, 0, "All objects should be collected");
}

// Test for memory leaks by creating and dropping many objects with circular references - fixed with weak refs
#[test]
fn test_no_memory_leaks() {
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create many nodes with circular references
    for _ in 0..100 {
        let mut node1 = gc.allocate(CircularNode::new(1));
        let mut node2 = gc.allocate(CircularNode::new(2));
        
        // Create a circular reference
        if let Some(node1_inner) = node1.inner_mut() {
            node1_inner.set_next(node2.clone());
        }
        
        if let Some(node2_inner) = node2.inner_mut() {
            node2_inner.set_next(node1.clone());
        }
        
        // Let the nodes go out of scope, which should drop the Gc references
        drop(node1);
        drop(node2);
        
        // Force a garbage collection
        gc.collect_garbage();
    }
    
    // Get final stats
    let final_stats = gc.stats();
    assert_eq!(final_stats.object_count, 0, "All objects should be collected");
}