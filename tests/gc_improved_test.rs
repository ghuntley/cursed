//! Improved test for circular reference handling in the garbage collector

use std::sync::Arc;

use cursed::memory::gc::{GarbageCollector, MemoryStats};
use cursed::memory::{Gc, Tag, Traceable, Visitor};
use tracing::{debug, error, info, trace};
use tracing_subscriber;

mod tracing_setup {
    pub fn setup() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info,cursed=debug")
            .with_test_writer()
            .try_init();
    }
}

// Simple struct that holds a reference to another GC-managed object
#[derive(Clone, Debug)]
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
        trace!(id = self.id, "Tracing CircularNode");
        if let Some(next) = &self.next {
            trace!(id = self.id, "CircularNode has a next reference to trace");
            if let Some(inner) = next.inner() {
                trace!("Got inner pointer for next reference");
                unsafe {
                    let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut CircularNode);
                    trace!(pointer = ?ptr, "Visiting next pointer");
                    visitor.visit(ptr);
                    trace!("Next reference visit completed");
                }
            } else {
                error!(id = self.id, "Could not get inner pointer for next reference");
            }
        } else {
            trace!(id = self.id, "CircularNode has no next references");
        }
        trace!(id = self.id, "Finished tracing CircularNode");
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

#[test]
#[ignore = "Long-running GC test - run with --ignored flag to execute"]
fn test_circular_references_simplified() {
    // Initialize tracing for this test
    tracing_setup::setup();
    info!("Starting circular references test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    debug!(garbage_collector = ?gc, "Created garbage collector");
    
    // Allocate two nodes
    debug!("Allocating node 1");
    let mut node1 = gc.allocate(CircularNode::new(1));
    debug!(node = ?node1, "Allocated node 1");
    
    debug!("Allocating node 2");
    let mut node2 = gc.allocate(CircularNode::new(2));
    debug!(node = ?node2, "Allocated node 2");
    
    // Create a circular reference
    debug!("Creating circular reference node1 -> node2");
    {
        let inner1 = node1.inner_mut().unwrap();
        debug!(id = inner1.id, "Got mutable reference to node1");
        inner1.set_next(node2.clone());
        debug!("Set node1.next = node2");
    }
    
    debug!("Creating circular reference node2 -> node1");
    {
        let inner2 = node2.inner_mut().unwrap();
        debug!(id = inner2.id, "Got mutable reference to node2");
        inner2.set_next(node1.clone());
        debug!("Set node2.next = node1");
    }
    
    // Get initial stats
    debug!("Getting initial memory stats");
    let initial_stats = gc.stats();
    debug!(stats = ?initial_stats, "Initial stats");
    assert!(initial_stats.object_count >= 2, "Expected at least 2 objects");
    
    // Create a weak reference to verify later
    debug!("Creating weak reference to node1");
    let weak_node1 = node1.downgrade();
    debug!(weak_ref = ?weak_node1, "Created weak reference");
    assert!(weak_node1.is_alive(), "Weak reference should be alive");
    
    // Drop the strong references
    info!("Dropping strong references");
    debug!("Dropping node1");
    drop(node1);
    debug!("Dropping node2");
    drop(node2);
    debug!("All strong references dropped");
    
    // Force a garbage collection
    info!("Starting garbage collection");
    debug!("Calling gc.collect_garbage()");
    gc.collect_garbage();
    debug!("Garbage collection completed");
    
    // Check if the weak reference is still alive
    info!("Checking weak references");
    debug!("Checking if weak reference is still alive");
    let weak_alive = weak_node1.is_alive();
    debug!(alive = weak_alive, "Weak reference alive status");
    // Note: This will fail if the GC can't properly handle circular references
    // If this test fails, it needs further improvement of the GC
    assert!(!weak_alive, "Objects should be collected despite circular references");
    
    // Get final stats - they should show fewer objects
    info!("Checking final stats");
    debug!("Getting final memory stats");
    let final_stats = gc.stats();
    debug!(stats = ?final_stats, "Final stats");
    
    let objects_collected = final_stats.object_count < initial_stats.object_count;
    if !objects_collected {
        error!(
            initial_count = initial_stats.object_count,
            final_count = final_stats.object_count,
            "Objects were not collected"
        );
    }
    
    assert!(objects_collected, 
            "Objects should be collected (initial: {}, final: {})", 
            initial_stats.object_count, final_stats.object_count);
    
    info!("Circular references test completed successfully");
}

#[test]
#[ignore = "Long-running GC test - run with --ignored flag to execute"]
fn test_multiple_circular_references() {
    // Initialize tracing for this test
    tracing_setup::setup();
    info!("Starting multiple circular references test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    debug!("Created garbage collector");
    
    // Create a more complex structure with multiple circular references
    debug!("Allocating multiple nodes");
    let mut node1 = gc.allocate(CircularNode::new(1));
    let mut node2 = gc.allocate(CircularNode::new(2));
    let mut node3 = gc.allocate(CircularNode::new(3));
    debug!("Allocated 3 nodes");
    
    // Create a circular structure: 1 -> 2 -> 3 -> 1
    debug!("Creating circular references between nodes");
    {
        let inner1 = node1.inner_mut().unwrap();
        debug!(from = 1, to = 2, "Setting node link");
        inner1.set_next(node2.clone());
    }
    
    {
        let inner2 = node2.inner_mut().unwrap();
        debug!(from = 2, to = 3, "Setting node link");
        inner2.set_next(node3.clone());
    }
    
    {
        let inner3 = node3.inner_mut().unwrap();
        debug!(from = 3, to = 1, "Setting node link");
        inner3.set_next(node1.clone());
    }
    debug!("Completed creating circular structure: 1 -> 2 -> 3 -> 1");
    
    // Get initial stats
    debug!("Getting initial stats");
    let initial_stats = gc.stats();
    debug!(stats = ?initial_stats, "Initial memory stats");
    assert!(initial_stats.object_count >= 3, "Expected at least 3 objects");
    
    // Create weak references to verify later
    debug!("Creating weak references to all nodes");
    let weak_node1 = node1.downgrade();
    let weak_node2 = node2.downgrade();
    let weak_node3 = node3.downgrade();
    debug!("Created weak references");
    
    // Drop all strong references
    info!("Dropping all strong references");
    drop(node1);
    drop(node2);
    drop(node3);
    debug!("All strong references dropped");
    
    // Force a garbage collection
    info!("Running garbage collection");
    gc.collect_garbage();
    debug!("Garbage collection completed");
    
    // Verify all objects have been collected
    info!("Verifying objects have been collected");
    let node1_alive = weak_node1.is_alive();
    let node2_alive = weak_node2.is_alive();
    let node3_alive = weak_node3.is_alive();
    
    if node1_alive || node2_alive || node3_alive {
        error!(
            node1_alive = node1_alive,
            node2_alive = node2_alive,
            node3_alive = node3_alive,
            "Some nodes were not collected"
        );
    }
    
    assert!(!node1_alive, "Node 1 should be collected");
    assert!(!node2_alive, "Node 2 should be collected");
    assert!(!node3_alive, "Node 3 should be collected");
    debug!("All nodes were properly collected");
    
    // Check the final stats
    debug!("Getting final memory stats");
    let final_stats = gc.stats();
    debug!(stats = ?final_stats, "Final memory stats");
    
    let objects_collected = final_stats.object_count < initial_stats.object_count;
    if !objects_collected {
        error!(
            initial_count = initial_stats.object_count,
            final_count = final_stats.object_count,
            "Objects were not collected"
        );
    }
    
    assert!(objects_collected,
            "Objects should be collected (initial: {}, final: {})",
            initial_stats.object_count, final_stats.object_count);
            
    info!("Multiple circular references test completed successfully");
}

// Test for incremental GC with circular references
#[test]
#[ignore = "Long-running GC test - run with --ignored flag to execute"]
fn test_incremental_gc_with_circular_refs() {
    // Initialize tracing for this test
    tracing_setup::setup();
    info!("Starting incremental GC with circular references test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    debug!("Created garbage collector");
    
    let mut nodes = Vec::<Gc<CircularNode>>::new();
    
    // Create a chain of nodes with occasional circular references
    debug!("Creating chain of nodes with circular references");
    for i in 0..50 {
        let mut new_node = gc.allocate(CircularNode::new(i));
        
        if i > 0 {
            // Connect to previous node
            let prev_node = nodes.last().unwrap();
            let inner = new_node.inner_mut().unwrap();
            inner.set_next(prev_node.clone());
            trace!(from = i, to = i-1, "Created link between nodes");
            
            // Every 10th node, create an additional circular reference
            if i % 10 == 0 && i >= 20 {
                let target_idx = (i / 2) as usize;
                let mut prev_clone = prev_node.clone();
                let inner = prev_clone.inner_mut().unwrap();
                inner.set_next(nodes[target_idx].clone());
                debug!(from = i-1, to = target_idx, "Created additional circular reference");
            }
        }
        
        nodes.push(new_node);
    }
    debug!(total_nodes = nodes.len(), "Created node chain");
    
    // Get initial memory stats
    debug!("Getting initial memory stats");
    let initial_stats = gc.stats();
    debug!(stats = ?initial_stats, "Initial stats");
    
    // Create weak references to a few key nodes
    debug!("Creating weak references to key nodes");
    let weak_refs: Vec<_> = vec![
        nodes[10].downgrade(),
        nodes[20].downgrade(),
        nodes[30].downgrade(),
        nodes[40].downgrade(),
    ];
    debug!(count = weak_refs.len(), "Created weak references");
    
    // Drop all strong references
    info!("Dropping all strong references");
    nodes.clear();
    debug!("Cleared node vector, all strong references dropped");
    
    // Force multiple incremental collections
    info!("Starting incremental garbage collection");
    for i in 0..5 {
        debug!(collection_number = i + 1, "Running collection");
        gc.collect_garbage();
    }
    debug!("Completed all incremental collections");
    
    // Verify all weak references are no longer alive
    info!("Verifying all objects have been collected");
    for (i, weak_ref) in weak_refs.iter().enumerate() {
        let is_alive = weak_ref.is_alive();
        if is_alive {
            error!(node_index = i, "Node should have been collected but is still alive");
        }
        assert!(!is_alive, "Node {} should have been collected", i);
    }
    debug!("All weak references are properly invalidated");
    
    // Check final memory stats
    debug!("Getting final memory stats");
    let final_stats = gc.stats();
    debug!(stats = ?final_stats, "Final stats");
    
    let enough_collected = final_stats.object_count < initial_stats.object_count / 2;
    if !enough_collected {
        error!(
            initial_count = initial_stats.object_count,
            final_count = final_stats.object_count,
            required = initial_stats.object_count / 2,
            "Not enough objects were collected"
        );
    }
    
    assert!(enough_collected, 
            "At least half of the objects should be collected");
            
    info!("Incremental GC with circular references test completed successfully");
}