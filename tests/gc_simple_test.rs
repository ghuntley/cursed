use std::sync::Arc;
use std::thread;
use std::time::Duration;
use cursed::memory::gc::GarbageCollector;
use cursed::memory::{Gc, Tag, Traceable, Visitor};
use tracing::{debug, error, info, trace};
use tracing_subscriber;

#[cfg(test)]
mod tests {
    
    
    mod tracing_setup {
        pub fn setup() {
            let _ = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug")
                .with_test_writer()
                .try_init();
        }
    }
    
    /// Simple object for testing garbage collection
    #[derive(Clone, Debug)]
    struct TestObject {
        id: usize,
        next: Option<Box<TestObject>>,
    }
    
    impl Traceable for TestObject {
        fn trace(&self, visitor: &mut dyn Visitor) {
            trace!(id = self.id, "TestObject trace called");
            if let Some(ref next) = self.next {
                trace!(id = self.id, "Tracing next reference");
                next.trace(visitor);
                trace!(id = self.id, "Next reference tracing completed");
            } else {
                trace!(id = self.id, "TestObject has no next references");
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
    // init_tracing!();
        // Initialize tracing for this test
        tracing_setup::setup();
        info!("Starting basic allocation test");
        
        // Create a new GC
        let gc = Arc::new(GarbageCollector::new());
        debug!(gc = ?gc, "Created garbage collector");
        
        // Create a scope for allocation
        {
            // Allocate an object
            debug!("Allocating test object");
            let obj = gc.allocate(TestObject {
                id: 1,
                next: None,
            });
            debug!(object = ?obj, "Successfully allocated object");
            
            // Check object state
            if let Some(inner) = obj.inner() {
                debug!(id = inner.id, "Object has ID");
                assert_eq!(inner.id, 1, "Object id should be 1");
            } else {
                error!("Failed to access object");
                panic!("Failed to access object");
            }
            
            // Run GC while object is in scope
            info!("Running GC with object in scope");
            gc.collect_garbage();
            
            // Object should still be alive
            let is_alive = obj.inner().is_some();
            if !is_alive {
                error!("Object should still be alive but was collected");
            }
            assert!(is_alive, "Object should still be alive");
            
            // Object will be dropped at end of scope
            info!("Object going out of scope");
        }
        
        // Sleep briefly to ensure drop handlers run
        thread::sleep(Duration::from_millis(10));
        
        // Run GC to collect the now-unreferenced object
        info!("Running GC after object out of scope");
        gc.collect_garbage();
        
        // Check final GC stats - allow time for stats to refresh
        thread::sleep(Duration::from_millis(10));
        let stats = gc.stats();
        debug!(stats = ?stats, "Final GC stats");
        
        // Since the object is gone, we should have collected it - check freed objects count
        let objects_freed = stats.freed_objects > 0;
        if !objects_freed {
            error!(freed_objects = stats.freed_objects, "No objects were freed");
        }
        assert!(objects_freed, "Objects should have been freed");
        
        info!("Basic allocation test completed successfully");
    }
}