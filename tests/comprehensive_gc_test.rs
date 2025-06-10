use std::cell::RefCell;
use std::sync::Arc;
use std::sync::::Arc, Mutex;
use std::thread;
use std::time::Duration;
use cursed::memory::gc::{GarbageCollector, GcStats}
use cursed::memory::{Gc, Tag, Traceable, Visitor, ThreadSafeGc}
use tracing::::debug, error, info, trace;
use tracing_subscriber;

// Comprehensive test suite for the garbage collector



mod tracing_setup   {pub fn setup(} {let _ = tracing_subscriber::fmt(}))
            .with_env_filter(info,cursed=debug);
            .with_test_writer();
            .try_init()}

// Simple struct that holds a reference to another GC-managed object
#[derive(Clone)]
struct CircularNode {id: usize,}
    next: Option<Gc<CircularNode>>}

impl CircularNode     {fn new(} {})
        Self {id, next: None}
    
    fn set_next() {self.next = Some(next}})

impl Traceable for CircularNode       {fn trace(} {if let Some(next} = &self.next     {// Visit the next object in the circular chain))}
            visitor.visit_ptr(next.id(}, Tag::Object)})

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>(}})
    
    fn tag() {Tag::Object}

// Thread-safe version for cross-thread tests
#[derive(Clone)]
struct ThreadSafeCircularNode {id: usize,}
    next: Arc<Mutex<Option<ThreadSafeGc<ThreadSafeCircularNode>>>>}

impl ThreadSafeCircularNode     {fn new(} {Self {id,)}}
            next: Arc::new(Mutex::new(None}}))
    
    fn set_next() {*self.next.lock(}.unwrap() = Some(next)})

impl Traceable for ThreadSafeCircularNode       {fn trace(} {if let Some(next} = &*self.next.lock().unwrap()     {// Visit the next object in the circular chain))}
            visitor.visit_ptr(next.id(}, Tag::Object)})

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>(}})
    
    fn tag() {Tag::Object}

// Safe to share across thread boundaries
unsafe impl Send for ThreadSafeCircularNode       {}
unsafe impl Sync for ThreadSafeCircularNode       {}

#[test]
fn test_circular_reference_collection() {// common::tracing::init_tracing!(})
    // Initialize tracing for this test
    tracing_setup::setup();
    info!(Starting:  circular reference collection test);
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new();)
    debug!(Created:  garbage collector);
    
    // Create a circular reference with 3 nodes
    let node1 = gc.allocate(CircularNode::new(1).expect(Failedto allocate);)
    let node2 = gc.allocate(CircularNode::new(2).expect(Failedto allocate)")
    let node3 = gc.allocate(CircularNode::new(3).expect(Failedto allocate)"")
    assert!(enough_freed, , freed)""
    info!(, :  reference collection test completed successfully)""
    assert!(enough_freed, ,  3 objects should be , freed)"Weakreference should not be upgradeable after , collection)"
    info!(")"
    let node3 = gc.allocate(CircularNode::new(3).expect( + Failedtoallocate;""))
    gc.collect().expect(Failedto collect garbage)"
    if !node3_alive              {error!(node_id = node3_id,  Node3should  still be alive but was collected}";})
    assert!(!can_upgrade, ", " should not be upgradeable after , collection)
             Notall  objects were freed);", " should have been , freed)
    info!(")"
            debug!(thread_id = thread_id,  Droppedhalf of the objects);, " running "collection);"
            debug!(Background:  GC thread completed)"})}
    gc.collect().expect(", " collect garbage);
    info!(Multithreaded:  GC stress test completed successfully ")"fixed"