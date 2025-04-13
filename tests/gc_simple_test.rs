#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    
    use cursed::memory::gc::GarbageCollector;
    use cursed::memory::{Gc, Tag, Traceable, Visitor};
    
    /// Simple object for testing garbage collection
    #[derive(Clone, Debug)]
    struct TestObject {
        id: usize,
        next: Option<Box<TestObject>>,
    }
    
    impl Traceable for TestObject {
        fn trace(&self, visitor: &mut dyn Visitor) {
            println!("TestObject({})::trace called", self.id);
            if let Some(ref next) = self.next {
                next.trace(visitor);
            }
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<TestObject>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
    }
    
    #[test]
    fn test_basic_allocation() {
        println!("\n==== Starting test_basic_allocation ====");
        
        // Create a new GC
        let gc = Arc::new(GarbageCollector::new());
        println!("Created GC: {:?}", gc);
        
        // Create a scope for allocation
        {
            // Allocate an object
            println!("Allocating object...");
            let obj = gc.allocate(TestObject {
                id: 1,
                next: None,
            });
            println!("Successfully allocated object: {:?}", obj);
            
            // Check object state
            if let Some(inner) = obj.inner() {
                println!("Object has id: {}", inner.id);
                assert_eq!(inner.id, 1, "Object id should be 1");
            } else {
                panic!("Failed to access object");
            }
            
            // Run GC while object is in scope
            println!("\n--- Running GC with object in scope ---");
            gc.collect_garbage();
            
            // Object should still be alive
            assert!(obj.inner().is_some(), "Object should still be alive");
            
            // Object will be dropped at end of scope
            println!("\n--- Object going out of scope ---");
        }
        
        // Sleep briefly to ensure drop handlers run
        thread::sleep(Duration::from_millis(10));
        
        // Run GC to collect the now-unreferenced object
        println!("\n--- Running GC after object out of scope ---");
        gc.collect_garbage();
        
        // Check final GC stats
        let stats = gc.stats();
        println!("Final stats: {:?}", stats);
        
        // Since the object is gone, we should have collected it
        assert_eq!(stats.object_count, 0, "All objects should be collected");
        
        println!("\n==== test_basic_allocation completed successfully ====");
    }
}