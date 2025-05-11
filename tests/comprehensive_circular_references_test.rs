//! Comprehensive test for circular reference handling in the garbage collector
//!
//! This test creates complex object networks with circular references and verifies
//! that the garbage collector properly identifies and collects these objects when
//! they become unreachable. It tests various scenarios including:
//! - Basic circular references (A -> B -> A)
//! - Multi-object cycles (A -> B -> C -> A)
//! - Nested cycles (A -> B -> A and B -> C -> B)
//! - Mixed reachable and unreachable cycles

use std::sync::{Arc, Mutex, RwLock};
use std::thread::sleep;
use std::time::Duration;

use cursed::memory::gc::{GarbageCollector, MemoryStats};
use cursed::memory::{Gc, Tag, Traceable, Visitor};
use tracing::{debug, error, info, trace, warn};

mod tracing_setup {
    pub fn setup() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info,cursed=debug")
            .with_test_writer()
            .try_init();
    }
}

/// A node in a circular reference graph
#[derive(Clone, Debug)]
struct CircNode {
    id: usize,
    // Use Arc+RwLock/Mutex for thread-safe interior mutability with Clone support
    references: Arc<RwLock<Vec<Option<Gc<CircNode>>>>>,
    // Flag to track if this node was finalized during collection
    was_finalized: Arc<Mutex<bool>>,
}

impl CircNode {
    fn new(id: usize) -> Self {
        Self {
            id,
            references: Arc::new(RwLock::new(Vec::new())),
            was_finalized: Arc::new(Mutex::new(false)),
        }
    }
    
    fn add_reference(&self, other: Gc<CircNode>) {
        self.references.write().unwrap().push(Some(other));
    }
    
    // Return a copy of our references for verification
    fn get_references(&self) -> Vec<Option<Gc<CircNode>>> {
        self.references.read().unwrap().clone()
    }
    
    fn was_finalized(&self) -> bool {
        *self.was_finalized.lock().unwrap()
    }
    
    fn reference_count(&self) -> usize {
        self.references.read().unwrap().len()
    }
    
    fn get_reference(&self, index: usize) -> Option<Gc<CircNode>> {
        let refs = self.references.read().unwrap();
        if index < refs.len() {
            refs[index].clone()
        } else {
            None
        }
    }
    
    // Helper to create a complete cycle
    fn create_cycle(gc: &Arc<GarbageCollector>, size: usize) -> Vec<Gc<CircNode>> {
        debug!("Creating cycle of {} nodes", size);
        let mut nodes = Vec::with_capacity(size);
        
        // First create all nodes
        for i in 0..size {
            nodes.push(gc.allocate(CircNode::new(i)));
        }
        
        // Then connect them in a cycle
        for i in 0..size {
            let next_idx = (i + 1) % size;
            {
                let node = nodes[i].inner_mut().unwrap();
                node.add_reference(nodes[next_idx].clone());
                debug!(from = i, to = next_idx, "Created reference");
            }
        }
        
        nodes
    }
}

impl Traceable for CircNode {
    fn trace(&self, visitor: &mut dyn Visitor) {
        trace!(id = self.id, "Tracing CircNode");
        
        // Trace all references
        let refs = self.references.read().unwrap();
        for (i, ref_opt) in refs.iter().enumerate() {
            if let Some(node_ref) = ref_opt {
                trace!(id = self.id, ref_idx = i, "CircNode has a reference to trace");
                if let Some(inner) = node_ref.inner() {
                    trace!("Got inner pointer for reference");
                    
                    // Create a pointer that the visitor can track
                    unsafe {
                        let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut CircNode);
                        trace!(pointer = ?ptr, "Visiting reference pointer");
                        visitor.visit(ptr);
                        trace!("Reference visit completed");
                    }
                } else {
                    error!(id = self.id, ref_idx = i, "Could not get inner pointer for reference");
                }
            }
        }
        
        trace!(id = self.id, "Finished tracing CircNode");
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
    
    fn finalize(&mut self) {
        info!(id = self.id, "Finalizing CircNode");
        if let Ok(mut finalized) = self.was_finalized.lock() {
            *finalized = true;
        }
    }
}

#[test]
#[ignore = "Long-running GC test - run with --ignored flag to execute"]
fn test_simple_circular_reference_collection() {
    // Initialize tracing for this test
    tracing_setup::setup();
    info!("Starting simple circular reference collection test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    debug!(garbage_collector = ?gc, "Created garbage collector");
    
    // Create two nodes with a circular reference
    let mut node1 = gc.allocate(CircNode::new(1));
    let mut node2 = gc.allocate(CircNode::new(2));
    
    // Create circular references: node1 -> node2 -> node1
    {
        let inner1 = node1.inner_mut().unwrap();
        inner1.add_reference(node2.clone());
        debug!("Added reference from node1 to node2");
    }
    
    {
        let inner2 = node2.inner_mut().unwrap();
        inner2.add_reference(node1.clone());
        debug!("Added reference from node2 to node1");
    }
    
    // Get initial stats
    let initial_stats = gc.stats();
    debug!(stats = ?initial_stats, "Initial stats");
    assert!(initial_stats.object_count >= 2, "Expected at least 2 objects");
    
    // Create weak references to track object lifetime
    let weak1 = node1.downgrade();
    let weak2 = node2.downgrade();
    assert!(weak1.is_alive(), "Node 1 weak reference should be alive");
    assert!(weak2.is_alive(), "Node 2 weak reference should be alive");
    
    // Drop the strong references - this should make the objects unreachable
    info!("Dropping strong references");
    drop(node1);
    drop(node2);
    
    // Force garbage collection
    info!("Running garbage collection with cycle detection");
    gc.collect_garbage_with_cycles(); // Use cycle detection explicitly
    
    // Wait a short time to allow collection to complete any background tasks
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Verify the objects have been collected despite the circular references
    info!("Verifying objects were collected");
    let node1_alive = weak1.is_alive();
    let node2_alive = weak2.is_alive();
    
    if node1_alive || node2_alive {
        error!(
            node1_alive = node1_alive,
            node2_alive = node2_alive,
            "Some nodes were not collected - circular reference detection failed"
        );
    }
    
    assert!(!node1_alive, "Node 1 should have been collected");
    assert!(!node2_alive, "Node 2 should have been collected");
    
    // Verify final stats show fewer objects
    let final_stats = gc.stats();
    debug!(stats = ?final_stats, "Final stats");
    assert!(final_stats.object_count < initial_stats.object_count,
           "Objects should have been collected");
    
    info!("Simple circular reference collection test completed successfully");
}

#[test]
#[ignore = "Long-running GC test - run with --ignored flag to execute"]
fn test_multi_node_cycle_collection() {
    // Initialize tracing for this test
    tracing_setup::setup();
    info!("Starting multi-node cycle collection test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a cycle of nodes: n1 -> n2 -> n3 -> n4 -> n1
    let mut nodes = CircNode::create_cycle(&gc, 4);
    info!("Created cycle of 4 nodes");
    
    // Verify structure
    for i in 0..nodes.len() {
        let node = nodes[i].inner().unwrap();
        assert_eq!(node.reference_count(), 1, "Node {} should have 1 reference", i);
        
        let next_idx = (i + 1) % nodes.len();
        let ref_id = node.get_reference(0).unwrap().inner().unwrap().id;
        assert_eq!(ref_id, next_idx, "Node {} should reference node {}", i, next_idx);
    }
    
    // Get initial stats
    let initial_stats = gc.stats();
    debug!(stats = ?initial_stats, "Initial stats");
    assert!(initial_stats.object_count >= 4, "Expected at least 4 objects");
    
    // Create weak references to track object lifetime
    let weak_refs: Vec<_> = nodes.iter().map(|n| n.downgrade()).collect();
    for (i, weak) in weak_refs.iter().enumerate() {
        assert!(weak.is_alive(), "Node {} weak reference should be alive", i);
    }
    
    // Drop the strong references - this should make the objects unreachable
    info!("Dropping strong references");
    nodes.clear(); // This drops all the nodes
    
    // Force garbage collection
    info!("Running garbage collection with cycle detection");
    gc.collect_garbage_with_cycles(); // Use the cycle detection implementation
    
    // Give it a small amount of time to complete internal tasks
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Verify all objects have been collected despite the circular references
    info!("Verifying objects were collected");
    let mut all_collected = true;
    for (i, weak) in weak_refs.iter().enumerate() {
        let is_alive = weak.is_alive();
        if is_alive {
            error!(node = i, "Node should have been collected but is still alive");
            all_collected = false;
        }
    }
    
    assert!(all_collected, "All nodes in the cycle should have been collected");
    
    // Verify final stats show fewer objects
    let final_stats = gc.stats();
    debug!(stats = ?final_stats, "Final stats");
    assert!(final_stats.object_count < initial_stats.object_count,
           "Objects should have been collected");
    
    info!("Multi-node cycle collection test completed successfully");
}

#[test]
#[ignore = "Long-running GC test - run with --ignored flag to execute"]
fn test_complex_reference_graph() {
    // Initialize tracing for this test
    tracing_setup::setup();
    info!("Starting complex reference graph test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a more complex structure with multiple cycles and references
    // Group 1: A cycle that should be collected (unreachable)
    let unreachable_cycle = CircNode::create_cycle(&gc, 3);
    
    // Group 2: A standalone node that references itself (should be collected)
    let self_ref_node = gc.allocate(CircNode::new(100));
    {
        let inner = self_ref_node.inner_mut().unwrap();
        inner.add_reference(self_ref_node.clone());
        debug!("Created self-reference in node 100");
    }
    
    // Group 3: A cycle that has an external reference (should remain alive)
    let mut reachable_cycle = CircNode::create_cycle(&gc, 2);
    let reachable_root = reachable_cycle[0].clone(); // Keep this reference alive
    
    // Group 4: A complex structure with cross-linked cycles
    // This has a cycle: A->B->C->A
    // And another cycle: B->D->E->B
    let mut complex_group = Vec::new();
    for i in 0..5 {
        complex_group.push(gc.allocate(CircNode::new(200 + i)));
    }
    
    // Create the first cycle (A->B->C->A)
    {
        let a = complex_group[0].inner_mut().unwrap();
        a.add_reference(complex_group[1].clone()); // A->B
    }
    {
        let b = complex_group[1].inner_mut().unwrap();
        b.add_reference(complex_group[2].clone()); // B->C
    }
    {
        let c = complex_group[2].inner_mut().unwrap();
        c.add_reference(complex_group[0].clone()); // C->A
    }
    
    // Create the second cycle (B->D->E->B)
    {
        let b = complex_group[1].inner_mut().unwrap();
        b.add_reference(complex_group[3].clone()); // B->D
    }
    {
        let d = complex_group[3].inner_mut().unwrap();
        d.add_reference(complex_group[4].clone()); // D->E
    }
    {
        let e = complex_group[4].inner_mut().unwrap();
        e.add_reference(complex_group[1].clone()); // E->B
    }
    
    // Keep reference to the root of this complex structure (A)
    let complex_root = complex_group[0].clone();
    
    // Get initial stats
    let initial_stats = gc.stats();
    debug!(stats = ?initial_stats, "Initial stats");
    
    // Create weak references to all nodes
    let unreachable_weak: Vec<_> = unreachable_cycle.iter().map(|n| n.downgrade()).collect();
    let self_ref_weak = self_ref_node.downgrade();
    let reachable_weak: Vec<_> = reachable_cycle.iter().map(|n| n.downgrade()).collect();
    let complex_weak: Vec<_> = complex_group.iter().map(|n| n.downgrade()).collect();
    
    // Drop the strong references to groups that should be collected
    info!("Dropping strong references to unreachable objects");
    drop(unreachable_cycle);
    drop(self_ref_node);
    drop(complex_group);
    
    // Force garbage collection
    info!("Running garbage collection");
    gc.collect_garbage();
    
    // Verify unreachable objects have been collected
    info!("Verifying collection results");
    
    // Group 1: Unreachable cycle should be collected
    for (i, weak) in unreachable_weak.iter().enumerate() {
        assert!(!weak.is_alive(), "Unreachable cycle node {} should have been collected", i);
    }
    
    // Group 2: Self-referential node should be collected
    assert!(!self_ref_weak.is_alive(), "Self-referential node should have been collected");
    
    // Group 3: Reachable cycle should still be alive
    for (i, weak) in reachable_weak.iter().enumerate() {
        assert!(weak.is_alive(), "Reachable cycle node {} should still be alive", i);
    }
    
    // Group 4: Complex cross-linked cycles should still be alive (we kept a reference to the root)
    for (i, weak) in complex_weak.iter().enumerate() {
        assert!(weak.is_alive(), "Complex structure node {} should still be alive", i);
    }
    
    // Now drop the remaining strong references
    info!("Dropping remaining strong references");
    drop(reachable_root);
    drop(reachable_cycle);
    drop(complex_root);
    
    // Run garbage collection again
    info!("Running second garbage collection with cycle detection");
    gc.collect_garbage_with_cycles(); // Use cycle detection for better handling
    
    // Give it a small amount of time to complete internal tasks
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Now all objects should be collected
    info!("Verifying all objects are now collected");
    
    // Group 3: Reachable cycle should now be collected
    for (i, weak) in reachable_weak.iter().enumerate() {
        assert!(!weak.is_alive(), "Previously reachable cycle node {} should now be collected", i);
    }
    
    // Group 4: Complex structure should now be collected
    for (i, weak) in complex_weak.iter().enumerate() {
        assert!(!weak.is_alive(), "Complex structure node {} should now be collected", i);
    }
    
    // Final stats should show a significant reduction in object count
    let final_stats = gc.stats();
    debug!(initial = ?initial_stats, final = ?final_stats, "Memory stats comparison");
    
    // Confirm memory was reclaimed
    assert!(final_stats.object_count < initial_stats.object_count / 2,
           "At least half of all objects should have been collected");
    
    info!("Complex reference graph test completed successfully");
}

#[test]
#[ignore = "Long-running GC test - run with --ignored flag to execute"]
fn test_incremental_collection_with_cycles() {
    // Initialize tracing for this test
    tracing_setup::setup();
    info!("Starting incremental collection with cycles test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a large network of interconnected nodes with various cycle patterns
    let mut node_groups = Vec::new();
    let group_count = 5;
    let nodes_per_group = 10;
    
    info!("Creating {} groups with {} nodes each", group_count, nodes_per_group);
    
    // Create groups of nodes with internal cycles
    for g in 0..group_count {
        let group_base_id = g * 100;
        let mut group = Vec::new();
        
        // Create the nodes in this group
        for i in 0..nodes_per_group {
            let node_id = group_base_id + i;
            group.push(gc.allocate(CircNode::new(node_id)));
        }
        
        // Create a cycle within the group
        for i in 0..nodes_per_group {
            let next_idx = (i + 1) % nodes_per_group;
            {
                let node = group[i].inner_mut().unwrap();
                node.add_reference(group[next_idx].clone());
            }
            
            // Add some cross-references to make the graph more complex
            if i % 3 == 0 && i > 0 {
                let random_idx = (i * 7) % nodes_per_group; // Not truly random but creates complex patterns
                let node = group[i].inner_mut().unwrap();
                node.add_reference(group[random_idx].clone());
            }
        }
        
        node_groups.push(group);
    }
    
    // Create some cross-group references
    for g in 0..group_count {
        let next_group = (g + 1) % group_count;
        
        // Connect some nodes between groups
        for i in 0..3 { // Link 3 nodes between each group
            let from_idx = i * 3;
            let to_idx = (i + 1) * 2;
            
            if from_idx < nodes_per_group && to_idx < nodes_per_group {
                let node = node_groups[g][from_idx].inner_mut().unwrap();
                node.add_reference(node_groups[next_group][to_idx].clone());
            }
        }
    }
    
    // Get initial stats
    let initial_stats = gc.stats();
    info!(total_objects = initial_stats.object_count, "Created object network");
    
    // Create weak references to track some specific nodes
    let tracker_group = 2; // Track group 2
    let weak_refs: Vec<_> = node_groups[tracker_group].iter().map(|n| n.downgrade()).collect();
    
    // Keep references to half the groups
    let keep_groups = group_count / 2;
    let mut kept_groups = Vec::new();
    for g in 0..keep_groups {
        kept_groups.push(node_groups[g].clone());
    }
    
    // Drop references to the other groups
    for g in keep_groups..group_count {
        node_groups[g].clear();
    }
    node_groups.clear();
    
    // Run incremental collection multiple times
    info!("Running incremental garbage collection");
    for i in 0..10 {
        info!(iteration = i, "Running incremental collection");
        gc.collect_garbage_incremental();
        sleep(Duration::from_millis(20)); // Small delay between collections to allow processing
    }
    
    // Finish with a full cycle detection collection to ensure proper cycle detection
    info!("Finalizing with a full cycle detection collection");
    gc.collect_garbage_with_cycles();
    
    // Check if weak references to the dropped group are still alive
    if tracker_group < keep_groups {
        // These should be alive if we kept this group
        for (i, weak) in weak_refs.iter().enumerate() {
            assert!(weak.is_alive(), "Node {i} in kept group {tracker_group} should still be alive");
        }
    } else {
        // These should be collected if we dropped this group
        let mut all_collected = true;
        for (i, weak) in weak_refs.iter().enumerate() {
            if weak.is_alive() {
                warn!(i = i, "Node in dropped group is still alive");
                all_collected = false;
            }
        }
        assert!(all_collected, "All nodes in dropped group should be collected");
    }
    
    // Now drop all remaining references
    info!("Dropping all remaining references");
    kept_groups.clear();
    
    // Run incremental collection again
    info!("Running final garbage collection");
    for _ in 0..10 {
        gc.collect_garbage_incremental();
        sleep(Duration::from_millis(20)); // Slightly longer delay for more thorough processing
    }
    
    // Final full collection with cycle detection to ensure all cycles are properly handled
    info!("Finalizing with a full cycle detection collection");
    gc.collect_garbage_with_cycles();
    sleep(Duration::from_millis(50)); // Give some time for completion
    
    // Get final stats
    let final_stats = gc.stats();
    info!(initial = initial_stats.object_count, final = final_stats.object_count, "Final object counts");
    
    // All objects should be collected now
    let enough_collected = final_stats.object_count < initial_stats.object_count / 2;
    assert!(enough_collected, "At least half of all objects should be collected");
    
    info!("Incremental collection with cycles test completed successfully");
}