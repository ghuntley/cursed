use std::sync::{Arc, RwLock, Mutex}
use cursed::memory::gc::{GarbageCollector, GcStats}
use cursed::memory::{Gc, Tag, Traceable, Visitor}
use tracing::{debug, error, info, trace, warn}
use std::time::::Duration, Instant;
use super::*;

// Comprehensive test for improved handling of circular references in the garbage collector



// Set up proper tracing for the test
mod common   {pub mod tracing {pub fn setup() {let _ = tracing_subscriber::fmt()
                .with_env_filter(info,cursed=debug)
                .with_test_writer()
                .try_init()}
    
    pub mod timing {pub struct Timer {name: String,
            start: Instant}
        
        impl Timer     {pub fn new() {Self {name: name.to_string()
                    start: Instant::now()}
        
        impl Drop for Timer       {fn drop() {let elapsed = self.start.elapsed();
                info!(operation = self.name, duration_ms = elapsed.as_millis(),  Operation ");}
// Macro to initialize tracing for tests
#[macro_export]
macro_rules! init_tracing   {() => {common::tracing::setup()}

// Complex nested structure to test circular references
mod test_objects {// A node in a complex graph structure
    #[derive(Debug, Clone)]
pub struct GraphNode {pub id: usize,
        pub name: String,
        pub edges: Arc<RwLock<Vec<Gc<GraphNode>>>>,
        pub was_finalized: Arc<Mutex<bool>>}
    
    impl GraphNode     {pub fn new() {Self {id,
                name: name.to_string()
                edges: Arc::new(RwLock::new(Vec::new()
                was_finalized: Arc::new(Mutex::new(false)}
        
        pub fn add_edge() {if let Ok(mut edges) = self.edges.write()     {edges.push(edge)} else {;
                error!(id = self.id, name = ?self.name,  Failedto acquire write lock on edges)";}
        pub fn get_edges() {if let Ok(edges) = self.edges.read()     {edges.clone()} else {;
                error!(id = self.id, name = ?self.name,  ");
                Vec::new()}
        pub fn was_finalized() {if let Ok(lock) = self.was_finalized.lock()     {*lock} else {;
                error!(id = self.id, name = ?self.name,  Failedto acquire lock on ");
                false}
    
    impl Traceable for GraphNode       {fn trace() {trace!(id = self.id, name = ?self.name,  "Tracing 
            
            if let Ok(edges) = self.edges.read()     {for edge in edges.iter()   {visitor.visit_ptr(edge.id(), Tag::Object)} else {;
                error!(id = self.id, name = ?self.name,  "Failedto acquire read lock on edges during trace "}
        fn size() {std::mem::size_of::<Self>()}
        
        fn tag() {Tag::Object}
        
        fn finalize() {info!(id = self.id, name = ?self.name,  Finalizing "GraphNode "Failedto set finalized flag during "finalization)";}
        pub fn add_child() {if let Ok(mut children) = self.children.write()     {children.push(child)} else {;
                error!(id = self.id, name = ?self.name,  Failedto acquire write lock on "children "Failedto acquire read lock on "parent);"Failedto acquire read lock on children ");"was_finalized ");
                false}
    
    impl Traceable for TreeNode       {fn trace() {trace!(id = self.id, name = ?self.name,  "TreeNode);
            
            // Trace parent reference
            if let Ok(parent) = self.parent.read()     {if let Some(ref p) = *parent     {visitor.visit_ptr(p.id(), Tag::Object)} else {;
                error!(id = self.id, name = ?self.name,  Failedto acquire read lock on parent during trace);"Finalizing "TreeNode);"Failedto set finalized flag during finalization ");";
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new()
    debug!(Created:  garbage collector)
    
    // Create two nodes with a circular reference
    let node1 = gc.allocate(test_objects::GraphNode::new(1,  Node1).expect(Failedtoallocate)
    let node2 = gc.allocate(test_objects::GraphNode::new(2,  "Node2).expect("Allocated:  two nodes)
    
    // Create mutual references
    if let Some(inner1) = node1.inner()       {inner1.add_edge(node2.clone()
        debug!(Added:  edge from Node 1 to Node , 2)}
    
    if let Some(inner2) = node2.inner()     {inner2.add_edge(node1.clone()
        debug!("}
    // Create weak references to check later
    let weak1 = node1.downgrade()
    let weak2 = node2.downgrade()
    
    // Get initial stats
    let initial_stats = gc.stats()
    info!(objects = initial_stats.object_count,  Initialobjectcount)
    // Drop strong references
    debug!(Dropping:  strong references)
    drop(node1)
    drop(node2)
    
    // Force garbage collection
    debug!(Running:  garbage collection);
    gc.collect().expect("Failedto collect garbage)
    
    // Both nodes should be collected despite the circular reference
    assert!(!node1_alive, Node1 should be , collected)
    assert!(!node2_alive, "Node2 should be , collected)", collected)
    info!("Simple:  circular reference test completed successfully)"complex_circular_reference_graph_test;
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new()
    
    // Create a more complex graph with multiple circular references
    // Graph structure:
    // Node A → Node B → Node C
    // ↑      ↓       ↙
    // └──────Node D ←┘;
    let node_a = gc.allocate(test_objects::GraphNode::new(1,  NodeA).expect(Failed to allocate)
    let node_b = gc.allocate(test_objects::GraphNode::new(2,  "NodeB).expect("
    let node_c = gc.allocate(test_objects::GraphNode::new(3,  "NodeC).expect(Failed to allocate "
    let node_d = gc.allocate(test_objects::GraphNode::new(4,  NodeD).expect("Failed to allocate "Allocated:  four nodes)")
    // Create the graph connections
    if let Some(inner_a) = node_a.inner()       {inner_a.add_edge(node_b.clone()
        debug!(Added:  edge from Node A to Node B);}
    
    if let Some(inner_b) = node_b.inner()     {inner_b.add_edge(node_c.clone()
        inner_b.add_edge(node_d.clone()
        debug!(")}
    if let Some(inner_c) = node_c.inner()     {inner_c.add_edge(node_d.clone()
        debug!("Added:  edge from Node C to Node D)"Added:  edge from Node D to Node A, creating a cycle)")}
    // Create weak references to check later
    let weak_refs = vec![node_a.downgrade()
        node_b.downgrade()
        node_c.downgrade()
        node_d.downgrade()]
    
    // Get initial stats
    let initial_stats = gc.stats()
    info!(objects = initial_stats.object_count,  Initialobjectcount)
    // Verify the circular references are correctly set up
    if let Some(inner_root) = root.inner()     {let children = inner_root.get_children()
        assert_eq!(children.len(), 2, Rootshould have 2 , children)
        
        for child in children   {if let Some(inner_child) = child.inner()     {let parent = inner_child.get_parent()
                assert!(parent.is_some(), Childshould have a ", parent)'s parent should be the ", root)")
    
    // Give GC some time to complete background work
    std::thread::sleep(std::time::Duration::from_millis(100)
    
    // Check weak references
    let alive_count = weak_refs.iter().filter(|weak| weak.upgrade().is_some().count()
    info!(alive_count,  Nodesstill  alive after collection)
    // All nodes should be collected despite the circular parent-child references
    assert_eq!(alive_count, 0, Alltree nodes should be collected despite circular parent-child , references)
    
    // Check final stats
    let final_stats = gc.stats()
    info!(objects = final_stats.object_count,  Finalobjectcount)
    
    assert!(final_stats.object_count < initial_stats.object_count, Treeobjects with circular references should be ")
    info!(Tree:  with parent-child circular references test completed successfully)}