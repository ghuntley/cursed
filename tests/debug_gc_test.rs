#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    
    use cursed::memory::gc::GarbageCollector;
    use cursed::memory::{Gc, Tag, Traceable, Visitor, with_gc_scope};

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
            println!("TestNode({})::trace called", self.id);
            if let Some(next) = &self.next {
                println!("TestNode({})::trace tracing next reference", self.id);
                if let Some(inner) = next.inner() {
                    unsafe {
                        let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut TestNode);
                        visitor.visit(ptr);
                    }
                }
            }
        }
        
        fn size(&self) -> usize { std::mem::size_of::<Self>() }
        fn tag(&self) -> Tag { Tag::Object }
    }

    #[test]
    fn test_gc_with_manual_root_management() {
        println!("\n===== Starting GC test with manual root management =====\n");
        
        // Step 1: Create a GC instance
        let gc = Arc::new(GarbageCollector::new());
        println!("Created GC: {:?}", gc);
        
        // Step 2: Create and track a test node
        {
            let node = gc.allocate(TestNode::new(1));
            println!("Created node: {:?}", node);
            
            // Step 3: Run a GC cycle while the node is still in scope
            println!("\n----- Running GC with node in scope -----\n");
            gc.collect_garbage();
            
            // Step 4: Check stats after first GC cycle
            let stats = gc.stats();
            println!("GC stats after first collection: {:?}", stats);
            assert!(stats.object_count > 0, "Node should still be alive");
        }
        
        // Step 5: Node is now out of scope, run another GC cycle
        println!("\n----- Node has gone out of scope, running GC again -----\n");
        gc.collect_garbage();
        
        // Step 6: Check stats after second GC cycle
        let stats = gc.stats();
        println!("GC stats after second collection: {:?}", stats);
        assert_eq!(stats.object_count, 0, "All objects should be collected");
        
        println!("\n===== Test completed successfully =====\n");
    }

    #[test]
    fn test_gc_with_scope_management() {
        println!("\n===== Starting GC test with scope management =====\n");
        
        // Step 1: Create a GC instance
        let gc = Arc::new(GarbageCollector::new());
        println!("Created GC: {:?}", gc);
        
        // Step 2: Create a root scope
        {
            let _root_guard = with_gc_scope(gc.clone());
            println!("Created root scope");
            
            // Step 3: Create and track a test node in the scope
            let node = gc.allocate(TestNode::new(1));
            println!("Created node in scope: {:?}", node);
            
            // Step 4: Run a GC cycle while the node is in the scope
            println!("\n----- Running GC with node in scope -----\n");
            gc.collect_garbage();
            
            // Step 5: Check stats after first GC cycle
            let stats = gc.stats();
            println!("GC stats after first collection: {:?}", stats);
            assert!(stats.object_count > 0, "Node should still be alive");
        }
        
        // Step 6: Scope has ended, node should be removed from roots
        println!("\n----- Root scope has ended, running GC again -----\n");
        thread::sleep(Duration::from_millis(10)); // Small delay to ensure scope cleanup completes
        
        // Step 7: Run GC to collect unrooted objects
        gc.collect_garbage();
        
        // Step 8: Check stats after second GC cycle
        let stats = gc.stats();
        println!("GC stats after second collection: {:?}", stats);
        assert_eq!(stats.object_count, 0, "All objects should be collected");
        
        println!("\n===== Test completed successfully =====\n");
    }

    #[test]
    fn test_gc_with_circular_references() {
        println!("\n===== Starting GC test with circular references =====\n");
        
        // Step 1: Create a GC instance
        let gc = Arc::new(GarbageCollector::new());
        println!("Created GC: {:?}", gc);
        
        // Step 2: Create a root scope
        {
            let _root_guard = with_gc_scope(gc.clone());
            println!("Created root scope");
            
            // Step 3: Create some nodes with circular references
            let mut node1 = gc.allocate(TestNode::new(1));
            let mut node2 = gc.allocate(TestNode::new(2));
            
            println!("Created nodes 1 and 2");
            
            // Step 4: Create circular references
            {
                let inner1 = node1.inner_mut().unwrap();
                inner1.set_next(node2.clone());
                println!("Set node1.next = node2");
            }
            
            {
                let inner2 = node2.inner_mut().unwrap();
                inner2.set_next(node1.clone());
                println!("Set node2.next = node1");
            }
            
            // Step 5: Run GC cycle while objects are still in scope
            println!("\n----- Running GC with circular references in scope -----\n");
            gc.collect_garbage();
            
            // Step 6: Check stats after first GC cycle
            let stats = gc.stats();
            println!("GC stats after first collection: {:?}", stats);
            assert!(stats.object_count >= 2, "Nodes should still be alive");
        }
        
        // Step 7: Scope has ended, run GC to collect circular references
        println!("\n----- Root scope has ended, running GC again -----\n");
        thread::sleep(Duration::from_millis(10)); // Small delay to ensure scope cleanup completes
        
        // Step 8: Run GC to collect circular references
        gc.collect_garbage();
        
        // Step 9: Check stats after second GC cycle
        let stats = gc.stats();
        println!("GC stats after second collection: {:?}", stats);
        assert_eq!(stats.object_count, 0, "All objects should be collected");
        
        println!("\n===== Test completed successfully =====\n");
    }
}