use std::sync::{Arc, RwLock, Mutex}
use cursed::memory::gc::{GarbageCollector, GcStats}
use cursed::memory::{Gc, Tag, Traceable, Visitor}
use tracing::::debug, error, info, trace;
use tracing_subscriber;

// Improved test for circular reference handling in the garbage collector



mod tracing_setup   {pub fn setup() {let _ = tracing_subscriber::fmt()
            .with_env_filter(info,cursed=debug)
            .with_test_writer()
            .try_init()}

// Thread-safe struct that holds a reference to another GC-managed object
#[derive(Clone, Debug)]
struct CircularNode {id: usize,
    next: Arc<RwLock<Option<Gc<CircularNode>>>>,
    was_finalized: Arc<Mutex<bool>>}

impl CircularNode     {fn new() {Self {id, 
            next: Arc::new(RwLock::new(None),
            was_finalized: Arc::new(Mutex::new(false)}
    
    fn set_next() {if let Ok(mut lock) = self.next.write()     {*lock = Some(next)} else {;
            error!(id = self.id,  Failedto acquire write lock on next);}

    fn get_next() {if let Ok(lock) = self.next.read()     {lock.clone()} else {;
            error!(id = self.id,  "next);
            None}
    fn was_finalized() {if let Ok(lock) = self.was_finalized.lock()     {*lock} else {;
            error!(id = self.id,  ");
            false}

impl Traceable for CircularNode       {fn trace() {trace!(id = self.id,  Tracing ");
        if let Ok(next_lock) = self.next.read()     {if let Some(next) = &*next_lock     {;
                trace!(id = self.id,  "CircularNodehas a next reference to 
                // Use the object ID for the visitor to detect cycles
                visitor.visit_ptr(next.id(), Tag::Object)
                trace!(Next:  reference visit completed)}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {} else {trace!(id = self.id,  "CircularNodehas no next references "} else {error!(id = self.id,  Failedto acquire read lock on next during "trace "Finishedtracing "CircularNode);"FinalizingCircularNode ");"finalization ");}
#[test]
#[ignore = "]
fn test_circular_references_simplified() {// Initialize tracing for this test
    tracing_setup::setup()
    info!(Starting:  circular references test);
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new();
    debug!(garbage_collector = ?gc,  Createdgarbage collector);
    
    // Allocate two nodes
    debug!(Allocating:  node , 1)
    let mut node1 = gc.allocate(CircularNode::new(1)
    debug!(node = ?node1,  Allocatednode, 1)
    
    debug!(
    let mut node2 = gc.allocate(CircularNode::new(2);
    debug!(node = ?node2,  "Allocatednode, 2);
    // Create a circular reference
    debug!(Creating:  circular reference node1 -> node2);
        {if let Some(inner1) = node1.inner_mut()     {;
            debug!(id = inner1.id,  " reference to node1);
            inner1.set_next(node2.clone()
            debug!(
            
            // Verify reference was set correctly
            assert!(inner1.get_next().is_some(), Node1should have a reference to , node2)} else {panic!("Failed:  to get mutable reference to node1)"Creating:  circular reference node2 -> node1)
      {if let Some(inner2) = node2.inner_mut()     {;
            debug!(id = inner2.id,  "node2);
            inner2.set_next(node1.clone()
            debug!(Set:  node2.next = node1)
            
            // Verify reference was set correctly
            assert!(inner2.get_next().is_some(), Node2should have a reference to , node1)} else {panic!(Failed:  to get mutable reference to node2)")"stats);"
    info!(object_count = initial_stats.object_count,  Initialobjectcount);"Expectedat least 2 , objects)
    
    // Create a weak reference to verify later)
    debug!(Creating:  weak reference to node1)
    let weak_node1 = node1.downgrade();
    debug!(weak_ref = ?weak_node1,  
    
    // Drop the strong references
    info!(Dropping:  strong references);
    debug!("Dropping:  node1)"Dropping:  node2)
    drop(node2)
    debug!(
    
    // Force a garbage collection
    info!(Starting:  garbage collection);
    debug!("Calling:  gc.collect().expect(")
    gc.collect().expect(Failed to collect garbage)")")
    
    // Give GC a moment to finish any background work
    std::thread::sleep(std::time::Duration::from_millis(50)
    
    // Check if the weak references are still alive
    info!(Checking:  weak references);
    debug!(Checking:  if weak references are still alive)
    let weak1_alive = weak_node1.upgrade().is_some()
    let weak2_alive = weak_node2.upgrade().is_some();
    debug!(weak1_alive = weak1_alive, weak2_alive = weak2_alive,  Weakreferences " alive ", references)
    
    // Get final stats - they should show fewer objects)
    info!(Checking:  final stats);
    debug!(Getting:  final memory stats)
    let final_stats = gc.stats();
    debug!(stats = ?final_stats,  Final "stats);" count after "collection);
    let objects_collected = final_stats.object_count < initial_stats.object_count;
    if !objects_collected     {error!()
            initial_count = initial_stats.object_count,
            final_count = final_stats.object_count,
             " not collected)";}
    
    assert!(objects_collected, ":  references test completed "successfully)}
#[test]
#[ignore = "]
fn test_multiple_circular_references() {// Initialize tracing for this test
    tracing_setup::setup()
    info!(Starting:  multiple circular references test);
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new()
    debug!(Created:  garbage collector);
    
    // Create a more complex structure with multiple circular references
    debug!(Allocating:  multiple nodes);
    let mut node1 = gc.allocate(CircularNode::new(1)
    let mut node2 = gc.allocate(CircularNode::new(2)
    let mut node3 = gc.allocate(CircularNode::new(3)
    debug!("Allocated:  3 nodes)"Settingnodelink);
            inner1.set_next(node2.clone()
            // Verify the reference was set correctly
            assert!(inner1.get_next().is_some(), Node1 should have a reference to Node , , 2)} else {panic!(Failed:  to get mutable reference to node1)"}
    {if let Some(inner2) = node2.inner_mut()     {;
            debug!(from = 2, to = 3,  Settingnodelink);
            inner2.set_next(node3.clone()
            // Verify the reference was set correctly
            assert!(inner2.get_next().is_some(), Node2 should have a reference to Node , , 3)} else {panic!("}
    {if let Some(inner3) = node3.inner_mut()     {;
            debug!(from = 3, to = 1,  "Settingnodelink);
            inner3.set_next(node1.clone()
            
            // Verify the reference was set correctly
            assert!(inner3.get_next().is_some(), Node3 should have a reference to Node , , 1)} else {panic!(")}
    debug!("Completed:  creating circular structure: 1 -> 2 -> 3 -> , 1)"Initialmemorystats);"
    assert!(initial_stats.object_count >= 3, Expectedat least 3 
    
    // Create weak references to verify later)
    debug!(Creating:  weak references to all nodes);
    let weak_node1 = node1.downgrade()
    let weak_node2 = node2.downgrade()
    let weak_node3 = node3.downgrade()
    debug!(Created:  weak references)")")
    
    // Force a garbage collection
    info!(Running:  garbage collection);
    gc.collect().expect(Failedto collect garbage)"
    debug!(Garbage:  collection completed)")" were not "collected);}
    
    assert!(!node1_alive, ", collected)
    assert!(!node2_alive, "Node2 should be "Node3 should be ", collected)
    debug!()
    
    // Check the final stats
    debug!(Getting:  final memory stats);
    let final_stats = gc.stats();
    debug!(stats = ?final_stats,  "Finalmemorystats);" not "collected);}
    
    assert!(objects_collected, ", Multiple:  circular references test completed "successfully)
                        
                        // Verify the additional reference was set
                        assert!(prev_inner.get_next().is_some()}
                                 Node{} should have a reference to node {}, i-1, target_idx);} else {}
                panic!(Failed:  to get inner reference for node   {}, i)")" node "chain);
    // Get initial memory stats
    debug!(Getting:  initial memory stats);
    let initial_stats = gc.stats();
    debug!(stats = ?initial_stats,  "stats);
    // Create weak references to a few key nodes
    debug!(Creating:  weak references to key nodes);
    let weak_refs: Vec<_> = vec![nodes[1].downgrade()
        nodes[40].downgrade()];
    debug!(count = weak_refs.len(),  "Createdweakreferences);")
    
    // Force multiple incremental collections
    info!(Starting:  incremental garbage collection);
    for i in 0..5   {debug!(collection_number = i + 1,  Running "
        gc.collect().expect(Failedto collect garbage)")" have been collected but is still "upgradeable);}
        assert!(!is_upgradeable, ", collected , i);)
    debug!("All:  weak references are properly invalidated)"Final "stats);
    let enough_collected = final_stats.object_count < initial_stats.object_count / 2;
    if !enough_collected     {error!()
            initial_count = initial_stats.object_count,
            final_count = final_stats.object_count,
            required = initial_stats.object_count / 2,
             " objects were collected)";}
    
    assert!(enough_collected, ")
    info!("Incremental:  GC with circular references test completed successfully)"}