//! Test for improved circular reference detection in garbage collector

use std::sync::Arc;

use cursed::memory::gc::{GarbageCollector, MemoryStats};
use cursed::memory::{Gc, Tag, Traceable, Visitor, with_gc_scope};

// Node with explicit cycle tracking for testing
#[derive(Clone, Debug)]
struct CyclicNode {
    id: usize,
    next: Option<Gc<CyclicNode>>,
    finalized: bool,
}

impl CyclicNode {
    fn new(id: usize) -> Self {
        Self { id, next: None, finalized: false }
    }
    
    fn set_next(&mut self, next: Gc<CyclicNode>) {
        self.next = Some(next);
    }
    
    // Method to verify finalization
    fn finalize(&mut self) {
        self.finalized = true;
        println!("CyclicNode id={} finalized", self.id);
    }
}

impl Traceable for CyclicNode {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(next) = &self.next {
            if let Some(inner) = next.inner() {
                unsafe {
                    let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut CyclicNode);
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
        self.finalize();
    }
}

#[test]
fn test_cycle_detection() {
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a scope for root tracking
    let _scope_guard = with_gc_scope(gc.clone());
    
    // Create a cycle: node1 -> node2 -> node3 -> node1
    let node1 = gc.allocate(CyclicNode::new(1));
    let node2 = gc.allocate(CyclicNode::new(2));
    let node3 = gc.allocate(CyclicNode::new(3));
    
    // Create the cycle
    {
        node1.inner_mut().unwrap().set_next(node2.clone());
        node2.inner_mut().unwrap().set_next(node3.clone());
        node3.inner_mut().unwrap().set_next(node1.clone());
    }
    
    // Keep weak references to check if nodes are collected
    let weak1 = node1.downgrade();
    let weak2 = node2.downgrade();
    let weak3 = node3.downgrade();
    
    // Check initial state
    assert!(weak1.is_alive(), "Node 1 should be alive");
    assert!(weak2.is_alive(), "Node 2 should be alive");
    assert!(weak3.is_alive(), "Node 3 should be alive");
    
    let initial_stats = gc.stats();
    println!("Initial stats: {:?}", initial_stats);
    assert!(initial_stats.object_count >= 3, "Should have at least 3 objects");
    
    // Drop all strong references
    drop(node1);
    drop(node2);
    drop(node3);
    
    // Force garbage collection
    gc.collect_garbage();
    
    // Check if nodes were collected
    assert!(!weak1.is_alive(), "Node 1 should be collected");
    assert!(!weak2.is_alive(), "Node 2 should be collected");
    assert!(!weak3.is_alive(), "Node 3 should be collected");
    
    // Check final stats
    let final_stats = gc.stats();
    println!("Final stats: {:?}", final_stats);
    assert!(final_stats.object_count < initial_stats.object_count, 
           "Objects should have been collected");
}

#[test]
fn test_incremental_collection() {
    // Create a garbage collector with incremental collection enabled
    let gc = Arc::new(GarbageCollector::with_options(cursed::memory::gc::GcOptions {
        initial_heap_size: 4096,
        allocation_threshold: 10,            // Trigger collection after 10 allocations
        incremental_step_size: 2,            // Process 2 objects per step
        incremental_time_budget_ms: 5,       // 5ms per incremental step
        verbose: true,
    }));
    
    // Create a scope for root tracking
    let _scope_guard = with_gc_scope(gc.clone());
    
    // Create 20 nodes with various connections
    let mut nodes = Vec::new();
    for i in 0..20 {
        nodes.push(gc.allocate(CyclicNode::new(i)));
    }
    
    // Create some connections (including cycles)
    for i in 0..nodes.len() {
        // Connect each node to two others to create a complex graph
        let next_idx = (i + 1) % nodes.len();
        let random_idx = (i * 7) % nodes.len(); // Simple pseudo-random connection
        
        nodes[i].inner_mut().unwrap().set_next(nodes[next_idx].clone());
    }
    
    // Drop half the nodes to create garbage
    let mut weak_refs = Vec::new();
    for i in 0..10 {
        weak_refs.push(nodes[i].downgrade());
        nodes[i] = gc.allocate(CyclicNode::new(100 + i)); // Replace with new nodes
    }
    
    // Trigger several incremental collections
    for _ in 0..5 {
        gc.collect_garbage_incremental();
    }
    
    // Check that some of the weak refs are now dead
    let mut alive_count = 0;
    for (i, weak) in weak_refs.iter().enumerate() {
        if weak.is_alive() {
            alive_count += 1;
        }
    }
    
    // Not all nodes will be collected due to incremental nature
    println!("Alive nodes after incremental collection: {}/{}", alive_count, weak_refs.len());
    
    // Now do a full collection to clean up everything
    gc.collect_garbage();
    
    // Check final stats
    let final_stats = gc.stats();
    println!("Final stats after full collection: {:?}", final_stats);
}