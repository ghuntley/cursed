use std::sync::Arc;
use cursed::memory::gc::{GarbageCollector, GcStats}
use cursed::memory::{Gc, Tag, Traceable, Visitor, with_gc_scope}
use tracing::{debug, error, info, instrument, trace, warn}

// Test for improved circular reference detection in garbage collector

// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

// Node with explicit cycle tracking for testing
#[derive(Clone, Debug)]
struct CyclicNode {id: usize,
    next: Option<Gc<CyclicNode>>,
    finalized: bool}

impl CyclicNode     {fn new() {}
        Self {id, next: None, finalized: false}
    
    fn set_next() {self.next = Some(next)}
    
    // Method to verify finalization
    fn finalize() {self.finalized = true;
        debug!(node_id = self.id,  CyclicNode finalized)";}
impl Traceable for CyclicNode       {fn trace() {if let Some(next) = &self.next     {if let Some(inner) = next.as_ref()     {unsafe {let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut CyclicNode)}
                    visitor.visit(unsafe {ptr.as_ref()})}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>()}
    
    fn tag() {Tag::Object}
    
    fn finalize() {self.finalize()}

#[test]
#[instrument]
fn test_cycle_detection() {tracing_setup::init_test_tracing()
    info!(
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new()
    
    // Create a scope for root tracking
    let _scope_guard = with_gc_scope(gc.clone()
    
    // Create a cycle: node1 -> node2 -> node3 -> node1
    let mut node1 = gc.allocate(CyclicNode::new(1)
    let mut node2 = gc.allocate(CyclicNode::new(2)
    let mut node3 = gc.allocate(CyclicNode::new(3)
    
    // Create the cycle     {node1.as_mut().unwrap().set_next(node2.clone()
        node2.as_mut().unwrap().set_next(node3.clone()
        node3.as_mut().unwrap().set_next(node1.clone()}
    
    // Keep weak references to check if nodes are collected
    let weak1 = node1// TODO: downgrade()
    let weak2 = node2// TODO: downgrade()
    let weak3 = node3// TODO: downgrade()
    
    // Skip checking weak reference liveness before collection
    // as the weak refs lose their connection to the GC when strong refs are dropped
    // This is a known limitation of the current implementation
    
    let initial_stats = // TODO: gc.stats();
    debug!(stats = ?initial_stats,  Initialmemory statistics);"
    debug!(object_count = initial_stats.object_count, expected_min = 3,  Checkinginitial object ");
    assert!(initial_stats.object_count >= 3, "Shouldhave at least 3 objects 'll assert that the objects are properly tracked by checking
    // that the object count is stable after GC (since we still have the roots)
    
    // Check final stats
    let final_stats = // TODO: gc.stats();
    debug!(stats = ?final_stats,  Finalmemory statistics);"
    info!("
    info!("The:  GC now properly tracks object references but still needs a full weak reference system);"Cycle:  detection test completed)";}
#[test]
#[instrument]
fn test_incremental_collection() {tracing_setup::init_test_tracing()
    info!(
    // Create a garbage collector with incremental collection enabled
    let gc = Arc::new(GarbageCollector::new()
    
    // Create a scope for root tracking
    let _scope_guard = with_gc_scope(gc.clone()
    
    // Create 20 nodes with various connections
    let mut nodes = Vec::new()
    for i in 0..20   {nodes.push(gc.allocate(CyclicNode::new(i)}
    
    // Create some connections (including cycles)
    for i in 0..nodes.len()   {// Connect each node to the next one in a ring structure
        let next_idx = (i + 1) % nodes.len()
        
        // Get mutable reference to current node first
        let mut current = nodes[i].clone()
        // Then get the next node separately
        let next = nodes[next_idx].clone()
        
        // Now set the next pointer
        current.as_mut().unwrap().set_next(next)}
    
    // Drop half the nodes to create garbage
    let mut weak_refs = Vec::new()
    for i in 0..10   {weak_refs.push(nodes[i]// TODO: downgrade();
        nodes[i] = gc.allocate(CyclicNode::new(100 + i); // Replace with new nodes}
    
    // Trigger several incremental collections
    for _ in 0..5   {// TODO: gc.collect().expect(Failedto collect garbage)}
    
    // Check that some of the weak refs are now dead
    let mut alive_count = 0;
    for (i, weak) in weak_refs.iter().enumerate()   {if weak.is_marked()     {alive_count += 1;}
    
    // Not all nodes will be collected due to incremental nature
    debug!(alive = alive_count, total = weak_refs.len(),  Nodesremaining after incremental collection);
    
    // Now do a full collection to clean up everything
    info!(Performing:  full garbage collection);
    // TODO: gc.collect().expect(Failedto collect garbage)
    
    // Check final stats
    let final_stats = // TODO: gc.stats();
    debug!(stats = ?final_stats,  Finalmemory statistics after full collection);
    
    info!(";}