use std::sync::{Arc, RwLock, Mutex};
use cursed::memory::gc::{GarbageCollector, MemoryStats};
use cursed::memory::{Gc, Tag, Traceable, Visitor};
use tracing::{debug, error, info, trace, warn};
use std::time::{Duration, Instant};
use super::*;

// Comprehensive test for improved handling of circular references in the garbage collector



// Set up proper tracing for the test
mod common {
    pub mod tracing {
        pub fn setup() {
            let _ = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug")
                .with_test_writer()
                .try_init();
        }
    }
    
    pub mod timing {
        
        pub struct Timer {
            name: String,
            start: Instant,
        }
        
        impl Timer {
            pub fn new(name: &str) -> Self {
                Self {
                    name: name.to_string(),
                    start: Instant::now(),
                }
            }
        }
        
        impl Drop for Timer {
            fn drop(&mut self) {
                let elapsed = self.start.elapsed();
                info!(operation = self.name, duration_ms = elapsed.as_millis(), "Operation timing");
            }
        }
    }
}

// Macro to initialize tracing for tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

// Complex nested structure to test circular references
mod test_objects {
    
    // A node in a complex graph structure
    #[derive(Debug, Clone)]
    pub struct GraphNode {
        pub id: usize,
        pub name: String,
        pub edges: Arc<RwLock<Vec<Gc<GraphNode>>>>,
        pub was_finalized: Arc<Mutex<bool>>,
    }
    
    impl GraphNode {
        pub fn new(id: usize, name: &str) -> Self {
            Self {
                id,
                name: name.to_string(),
                edges: Arc::new(RwLock::new(Vec::new())),
                was_finalized: Arc::new(Mutex::new(false)),
            }
        }
        
        pub fn add_edge(&self, edge: Gc<GraphNode>) {
            if let Ok(mut edges) = self.edges.write() {
                edges.push(edge);
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire write lock on edges");
            }
        }
        
        pub fn get_edges(&self) -> Vec<Gc<GraphNode>> {
            if let Ok(edges) = self.edges.read() {
                edges.clone()
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire read lock on edges");
                Vec::new()
            }
        }
        
        pub fn was_finalized(&self) -> bool {
            if let Ok(lock) = self.was_finalized.lock() {
                *lock
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire lock on was_finalized");
                false
            }
        }
    }
    
    impl Traceable for GraphNode {
        fn trace(&self, visitor: &mut dyn Visitor) {
            trace!(id = self.id, name = ?self.name, "Tracing GraphNode");
            
            if let Ok(edges) = self.edges.read() {
                for edge in edges.iter() {
                    visitor.visit_ptr(edge.id(), Tag::Object);
                }
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire read lock on edges during trace");
            }
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
        
        fn finalize(&mut self) {
            info!(id = self.id, name = ?self.name, "Finalizing GraphNode");
            if let Ok(mut finalized) = self.was_finalized.lock() {
                *finalized = true;
            } else {
                error!(id = self.id, name = ?self.name, "Failed to set finalized flag during finalization");
            }
        }
    }
    
    // A hierarchical structure to test more complex circular references
    #[derive(Debug, Clone)]
    pub struct TreeNode {
        pub id: usize,
        pub name: String,
        pub parent: Arc<RwLock<Option<Gc<TreeNode>>>>,
        pub children: Arc<RwLock<Vec<Gc<TreeNode>>>>,
        pub was_finalized: Arc<Mutex<bool>>,
    }
    
    impl TreeNode {
        pub fn new(id: usize, name: &str) -> Self {
            Self {
                id,
                name: name.to_string(),
                parent: Arc::new(RwLock::new(None)),
                children: Arc::new(RwLock::new(Vec::new())),
                was_finalized: Arc::new(Mutex::new(false)),
            }
        }
        
        pub fn set_parent(&self, parent: Gc<TreeNode>) {
            if let Ok(mut p) = self.parent.write() {
                *p = Some(parent);
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire write lock on parent");
            }
        }
        
        pub fn add_child(&self, child: Gc<TreeNode>) {
            if let Ok(mut children) = self.children.write() {
                children.push(child);
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire write lock on children");
            }
        }
        
        pub fn get_parent(&self) -> Option<Gc<TreeNode>> {
            if let Ok(parent) = self.parent.read() {
                parent.clone()
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire read lock on parent");
                None
            }
        }
        
        pub fn get_children(&self) -> Vec<Gc<TreeNode>> {
            if let Ok(children) = self.children.read() {
                children.clone()
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire read lock on children");
                Vec::new()
            }
        }
        
        pub fn was_finalized(&self) -> bool {
            if let Ok(lock) = self.was_finalized.lock() {
                *lock
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire lock on was_finalized");
                false
            }
        }
    }
    
    impl Traceable for TreeNode {
        fn trace(&self, visitor: &mut dyn Visitor) {
            trace!(id = self.id, name = ?self.name, "Tracing TreeNode");
            
            // Trace parent reference
            if let Ok(parent) = self.parent.read() {
                if let Some(ref p) = *parent {
                    visitor.visit_ptr(p.id(), Tag::Object);
                }
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire read lock on parent during trace");
            }
            
            // Trace children references
            if let Ok(children) = self.children.read() {
                for child in children.iter() {
                    visitor.visit_ptr(child.id(), Tag::Object);
                }
            } else {
                error!(id = self.id, name = ?self.name, "Failed to acquire read lock on children during trace");
            }
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
        
        fn finalize(&mut self) {
            info!(id = self.id, name = ?self.name, "Finalizing TreeNode");
            if let Ok(mut finalized) = self.was_finalized.lock() {
                *finalized = true;
            } else {
                error!(id = self.id, name = ?self.name, "Failed to set finalized flag during finalization");
            }
        }
    }
}

#[test]
fn test_simple_circular_reference() {
    // Initialize tracing for this test
    init_tracing!();
    info!("Starting simple circular reference test");
    
    let _timer = common::timing::Timer::new("simple_circular_reference_test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new();
    debug!("Created garbage collector");
    
    // Create two nodes with a circular reference
    let node1 = gc.allocate(test_objects::GraphNode::new(1, "Node 1");
    let node2 = gc.allocate(test_objects::GraphNode::new(2, "Node 2");
    
    debug!("Allocated two nodes");
    
    // Create mutual references
    if let Some(inner1) = node1.inner() {
        inner1.add_edge(node2.clone();
        debug!("Added edge from Node 1 to Node 2");
    }
    
    if let Some(inner2) = node2.inner() {
        inner2.add_edge(node1.clone();
        debug!("Added edge from Node 2 to Node 1");
    }
    
    // Create weak references to check later
    let weak1 = node1.downgrade();
    let weak2 = node2.downgrade();
    
    // Get initial stats
    let initial_stats = gc.stats();
    info!(objects = initial_stats.object_count, "Initial object count");
    
    // Drop strong references
    debug!("Dropping strong references");
    drop(node1);
    drop(node2);
    
    // Force garbage collection
    debug!("Running garbage collection");
    gc.collect_garbage();
    
    // Check weak references
    let node1_alive = weak1.upgrade().is_some());
    let node2_alive = weak2.upgrade().is_some());
    
    info!(node1_alive, node2_alive, "Weak references status after collection");
    
    // Both nodes should be collected despite the circular reference
    assert!(!node1_alive, "Node 1 should be collected");
    assert!(!node2_alive, "Node 2 should be collected");
    
    // Check final stats
    let final_stats = gc.stats();
    info!(objects = final_stats.object_count, "Final object count");
    
    assert!(final_stats.object_count < initial_stats.object_count, 
            "Objects with circular references should be collected");
            
    info!("Simple circular reference test completed successfully");
}

#[test]
fn test_complex_circular_reference_graph() {
    // Initialize tracing for this test
    init_tracing!();
    info!("Starting complex circular reference graph test");
    
    let _timer = common::timing::Timer::new("complex_circular_reference_graph_test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new();
    
    // Create a more complex graph with multiple circular references
    // Graph structure:
    // Node A → Node B → Node C
    // ↑      ↓       ↙
    // └──────Node D ←┘
    
    let node_a = gc.allocate(test_objects::GraphNode::new(1, "Node A");
    let node_b = gc.allocate(test_objects::GraphNode::new(2, "Node B");
    let node_c = gc.allocate(test_objects::GraphNode::new(3, "Node C");
    let node_d = gc.allocate(test_objects::GraphNode::new(4, "Node D");
    
    debug!("Allocated four nodes");
    
    // Create the graph connections
    if let Some(inner_a) = node_a.inner() {
        inner_a.add_edge(node_b.clone();
        debug!("Added edge from Node A to Node B");
    }
    
    if let Some(inner_b) = node_b.inner() {
        inner_b.add_edge(node_c.clone();
        inner_b.add_edge(node_d.clone();
        debug!("Added edges from Node B to Node C and Node D");
    }
    
    if let Some(inner_c) = node_c.inner() {
        inner_c.add_edge(node_d.clone();
        debug!("Added edge from Node C to Node D");
    }
    
    if let Some(inner_d) = node_d.inner() {
        inner_d.add_edge(node_a.clone();
        debug!("Added edge from Node D to Node A, creating a cycle");
    }
    
    // Create weak references to check later
    let weak_refs = vec![
        node_a.downgrade(),
        node_b.downgrade(),
        node_c.downgrade(),
        node_d.downgrade(),
    ];
    
    // Get initial stats
    let initial_stats = gc.stats();
    info!(objects = initial_stats.object_count, "Initial object count");
    
    // Drop strong references
    debug!("Dropping strong references");
    drop(node_a);
    drop(node_b);
    drop(node_c);
    drop(node_d);
    
    // Force garbage collection
    debug!("Running garbage collection");
    gc.collect_garbage();
    
    // Give GC some time to complete background work
    std::thread::sleep(std::time::Duration::from_millis(100);
    
    // Check weak references
    let alive_count = weak_refs.iter().filter(|weak| weak.upgrade().is_some()).count());
    
    info!(alive_count, "Nodes still alive after collection");
    
    // All nodes should be collected despite the circular references
    assert_eq!(alive_count, 0, "All nodes should be collected despite circular references");
    
    // Check final stats
    let final_stats = gc.stats();
    info!(objects = final_stats.object_count, "Final object count");
    
    assert!(final_stats.object_count < initial_stats.object_count, 
            "Objects with circular references should be collected");
            
    info!("Complex circular reference graph test completed successfully");
}

#[test]
fn test_tree_with_parent_child_circular_references() {
    // Initialize tracing for this test
    init_tracing!();
    info!("Starting tree with parent-child circular references test");
    
    let _timer = common::timing::Timer::new("tree_circular_reference_test");
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new();
    
    // Create a tree structure with parent-child circular references
    let root = gc.allocate(test_objects::TreeNode::new(1, "Root");
    let child1 = gc.allocate(test_objects::TreeNode::new(2, "Child 1");
    let child2 = gc.allocate(test_objects::TreeNode::new(3, "Child 2");
    let grandchild1 = gc.allocate(test_objects::TreeNode::new(4, "Grandchild 1");
    let grandchild2 = gc.allocate(test_objects::TreeNode::new(5, "Grandchild 2");
    
    debug!("Allocated five tree nodes");
    
    // Set up the tree structure
    // Add children to parent nodes
    if let Some(inner_root) = root.inner() {
        inner_root.add_child(child1.clone();
        inner_root.add_child(child2.clone();
        debug!("Added Child 1 and Child 2 to Root");
    }
    
    if let Some(inner_child1) = child1.inner() {
        inner_child1.add_child(grandchild1.clone();
        debug!("Added Grandchild 1 to Child 1");
    }
    
    if let Some(inner_child2) = child2.inner() {
        inner_child2.add_child(grandchild2.clone();
        debug!("Added Grandchild 2 to Child 2");
    }
    
    // Set parent references to create circular references
    if let Some(inner_child1) = child1.inner() {
        inner_child1.set_parent(root.clone();
        debug!("Set Root as parent of Child 1");
    }
    
    if let Some(inner_child2) = child2.inner() {
        inner_child2.set_parent(root.clone();
        debug!("Set Root as parent of Child 2");
    }
    
    if let Some(inner_grandchild1) = grandchild1.inner() {
        inner_grandchild1.set_parent(child1.clone();
        debug!("Set Child 1 as parent of Grandchild 1");
    }
    
    if let Some(inner_grandchild2) = grandchild2.inner() {
        inner_grandchild2.set_parent(child2.clone();
        debug!("Set Child 2 as parent of Grandchild 2");
    }
    
    // Create weak references to check later
    let weak_refs = vec![
        root.downgrade(),
        child1.downgrade(),
        child2.downgrade(),
        grandchild1.downgrade(),
        grandchild2.downgrade(),
    ];
    
    // Get initial stats
    let initial_stats = gc.stats();
    info!(objects = initial_stats.object_count, "Initial object count");
    
    // Verify the circular references are correctly set up
    if let Some(inner_root) = root.inner() {
        let children = inner_root.get_children();
        assert_eq!(children.len(), 2, "Root should have 2 children");
        
        for child in children {
            if let Some(inner_child) = child.inner() {
                let parent = inner_child.get_parent();
                assert!(parent.is_some(), "Child should have a parent");
                
                if let Some(p) = parent {
                    if let Some(inner_parent) = p.inner() {
                        assert_eq!(inner_parent.id, inner_root.id, "Child's parent should be the root");
                    }
                }
            }
        }
    }
    
    // Drop strong references
    debug!("Dropping strong references");
    drop(root);
    drop(child1);
    drop(child2);
    drop(grandchild1);
    drop(grandchild2);
    
    // Force garbage collection
    debug!("Running garbage collection");
    gc.collect_garbage();
    
    // Give GC some time to complete background work
    std::thread::sleep(std::time::Duration::from_millis(100);
    
    // Check weak references
    let alive_count = weak_refs.iter().filter(|weak| weak.upgrade().is_some()).count());
    
    info!(alive_count, "Nodes still alive after collection");
    
    // All nodes should be collected despite the circular parent-child references
    assert_eq!(alive_count, 0, "All tree nodes should be collected despite circular parent-child references");
    
    // Check final stats
    let final_stats = gc.stats();
    info!(objects = final_stats.object_count, "Final object count");
    
    assert!(final_stats.object_count < initial_stats.object_count, 
            "Tree objects with circular references should be collected");
            
    info!("Tree with parent-child circular references test completed successfully");
}