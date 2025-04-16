#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    
    use cursed::memory::gc::GarbageCollector;
    use cursed::memory::{Gc, Tag, Traceable, Visitor, with_gc_scope};
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

    #[derive(Clone, Debug)]
    struct TestNode {
        id: usize,
        next: Option<Gc<TestNode>>,
    }

    impl TestNode {
        fn new(id: usize) -> Self {
            Self { id, next: None }
        }
        
        fn set_next(&mut self, next: Gc<TestNode>) {
            self.next = Some(next);
        }
    }

    impl Traceable for TestNode {
        fn trace(&self, visitor: &mut dyn Visitor) {
            trace!(id = self.id, "TestNode trace called");
            if let Some(next) = &self.next {
                trace!(id = self.id, "TestNode tracing next reference");
                if let Some(inner) = next.inner() {
                    unsafe {
                        let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut TestNode);
                        trace!(pointer = ?ptr, "Visiting next reference");
                        visitor.visit(ptr);
                        trace!("Next reference visit completed");
                    }
                } else {
                    trace!("Could not get inner pointer for next reference");
                }
            } else {
                trace!(id = self.id, "TestNode has no next references");
            }
        }
        
        fn size(&self) -> usize { std::mem::size_of::<Self>() }
        fn tag(&self) -> Tag { Tag::Object }
    }

    #[test]
    fn test_gc_with_manual_root_management() {
        // Initialize tracing for this test
        tracing_setup::setup();
        info!("Starting GC test with manual root management");
        
        // Step 1: Create a GC instance
        let gc = Arc::new(GarbageCollector::new());
        debug!(gc = ?gc, "Created garbage collector");
        
        // Step 2: Create and track a test node
        {
            let node = gc.allocate(TestNode::new(1));
            debug!(node = ?node, "Created node");
            
            // Step 3: Run a GC cycle while the node is still in scope
            info!("Running GC with node in scope");
            gc.collect_garbage();
            
            // Step 4: Check stats after first GC cycle
            let stats = gc.stats();
            debug!(stats = ?stats, "GC stats after first collection");
            
            let node_alive = stats.object_count > 0;
            if !node_alive {
                error!(object_count = stats.object_count, "Node should still be alive");
            }
            assert!(node_alive, "Node should still be alive");
        }
        
        // Step 5: Node is now out of scope, run another GC cycle
        info!("Node has gone out of scope, running GC again");
        gc.collect_garbage();
        
        // Step 6: Check stats after second GC cycle - allow time for stats to refresh
        std::thread::sleep(std::time::Duration::from_millis(10));
        let stats = gc.stats();
        debug!(stats = ?stats, "GC stats after second collection");
        
        // Verify with freed_objects instead of object_count for consistency
        let objects_freed = stats.freed_objects > 0;
        if !objects_freed {
            error!(freed_objects = stats.freed_objects, "No objects were freed");
        }
        assert!(objects_freed, "Objects should have been freed");
        
        info!("Test completed successfully");
    }

    #[test]
    fn test_gc_with_scope_management() {
        // Initialize tracing for this test
        tracing_setup::setup();
        info!("Starting GC test with scope management");
        
        // Step 1: Create a GC instance
        let gc = Arc::new(GarbageCollector::new());
        debug!(gc = ?gc, "Created garbage collector");
        
        // Step 2: Create a root scope
        {
            let _root_guard = with_gc_scope(gc.clone());
            debug!("Created root scope");
            
            // Step 3: Create and track a test node in the scope
            let node = gc.allocate(TestNode::new(1));
            debug!(node = ?node, "Created node in scope");
            
            // Step 4: Run a GC cycle while the node is in the scope
            info!("Running GC with node in scope");
            gc.collect_garbage();
            
            // Step 5: Check stats after first GC cycle
            let stats = gc.stats();
            debug!(stats = ?stats, "GC stats after first collection");
            
            let node_alive = stats.object_count > 0;
            if !node_alive {
                error!(object_count = stats.object_count, "Node should still be alive");
            }
            assert!(node_alive, "Node should still be alive");
        }
        
        // Step 6: Scope has ended, node should be removed from roots
        info!("Root scope has ended, running GC again");
        thread::sleep(Duration::from_millis(10)); // Small delay to ensure scope cleanup completes
        
        // Step 7: Run GC to collect unrooted objects
        debug!("Running second garbage collection");
        gc.collect_garbage();
        
        // Step 8: Check stats after second GC cycle - allow time for stats to refresh
        std::thread::sleep(std::time::Duration::from_millis(10));
        let stats = gc.stats();
        debug!(stats = ?stats, "GC stats after second collection");
        
        // Verify with freed_objects instead of object_count for consistency
        let objects_freed = stats.freed_objects > 0;
        if !objects_freed {
            error!(freed_objects = stats.freed_objects, "No objects were freed");
        }
        assert!(objects_freed, "Objects should have been freed");
        
        info!("Test completed successfully");
    }

    #[test]
    fn test_gc_with_circular_references() {
        // Initialize tracing for this test
        tracing_setup::setup();
        info!("Starting GC test with circular references");
        
        // Step 1: Create a GC instance
        let gc = Arc::new(GarbageCollector::new());
        debug!(gc = ?gc, "Created garbage collector");
        
        // Step 2: Create a root scope
        {
            let _root_guard = with_gc_scope(gc.clone());
            debug!("Created root scope");
            
            // Step 3: Create some nodes with circular references
            let mut node1 = gc.allocate(TestNode::new(1));
            let mut node2 = gc.allocate(TestNode::new(2));
            
            debug!("Created nodes 1 and 2");
            
            // Step 4: Create circular references
            {
                let inner1 = node1.inner_mut().unwrap();
                inner1.set_next(node2.clone());
                debug!(from = 1, to = 2, "Creating circular reference");
            }
            
            {
                let inner2 = node2.inner_mut().unwrap();
                inner2.set_next(node1.clone());
                debug!(from = 2, to = 1, "Creating circular reference");
            }
            debug!("Completed creating circular references");
            
            // Step 5: Run GC cycle while objects are still in scope
            info!("Running GC with circular references in scope");
            gc.collect_garbage();
            
            // Step 6: Check stats after first GC cycle
            let stats = gc.stats();
            debug!(stats = ?stats, "GC stats after first collection");
            
            let nodes_alive = stats.object_count >= 2;
            if !nodes_alive {
                error!(
                    object_count = stats.object_count,
                    expected = 2,
                    "Nodes should still be alive"
                );
            }
            assert!(nodes_alive, "Nodes should still be alive");
        }
        
        // Step 7: Scope has ended, run GC to collect circular references
        info!("Root scope has ended, running GC again");
        thread::sleep(Duration::from_millis(10)); // Small delay to ensure scope cleanup completes
        
        // Step 8: Run GC to collect circular references
        debug!("Running second garbage collection");
        gc.collect_garbage();
        
        // Step 9: Check stats after second GC cycle - allow time for stats to refresh
        std::thread::sleep(std::time::Duration::from_millis(10));
        let stats = gc.stats();
        debug!(stats = ?stats, "GC stats after second collection");
        
        // Verify with freed_objects instead of object_count for consistency
        let all_objects_freed = stats.freed_objects >= 2;
        if !all_objects_freed {
            error!(
                freed_objects = stats.freed_objects,
                expected = 2,
                "Not all objects were freed"
            );
        }
        assert!(all_objects_freed, "All objects should have been freed");
        
        info!("Test completed successfully");
    }
}