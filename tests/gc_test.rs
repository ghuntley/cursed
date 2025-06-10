use cursed::memory::gc::GarbageCollector;
use cursed::memory::{Gc, Tag, Traceable, Visitor};
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;

#[cfg(test)]
mod tests {
    use super::*;
    
    mod tracing_setup {
        pub fn setup() {
            let _ = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug")
                .with_test_writer()
                .try_init();
        }
    }

    #[derive(Debug, Clone)]
    struct TestObject {
        value: i64,
        next: Option<Box<TestObject>>,
    }

    impl Traceable for TestObject {
        fn trace(&self, visitor: &mut dyn Visitor) {
            if let Some(ref next) = self.next {
                next.trace(visitor);
            }
        }
    }

    unsafe impl Send for TestObject {}
    unsafe impl Sync for TestObject {}

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_basic_allocation() {
        // Initialize tracing for this test
        tracing_setup::setup();
        info!("Starting test_basic_allocation");
        let mm = GarbageCollector::new();
        debug!("Created GarbageCollector instance");
        
        debug!("About to allocate test object");
        let obj = mm.allocate(TestObject {
            value: 42,
            next: None,
        }).expect("Failed to allocate");
        debug!(ptr = ?obj.object_id(), "Allocated test object");

        debug!("About to access object inner value");
        let inner_obj = obj.as_ref();
        info!(value = inner_obj.value, "Inner object value");
        assert_eq!(inner_obj.value, 42);
        
        debug!("Checking inner object next field");
        assert!(inner_obj.next.is_none());

        // Force a garbage collection
        info!("About to run garbage collection");
        mm.collect().expect("Failed to collect garbage");
        info!("Garbage collection completed");

        // Object should still be accessible after collection
        debug!("About to access object after GC");
        let inner_after_gc = obj.as_ref();
        debug!(value = inner_after_gc.value, "Inner object value after GC");
        assert_eq!(inner_after_gc.value, 42);
        info!("Test basic_allocation completed successfully");
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_linked_objects() {
        // Initialize tracing for this test
        tracing_setup::setup();
        info!("Starting test_linked_objects");
        let mm = GarbageCollector::new();

        let obj3 = TestObject {
            value: 3,
            next: None,
        };
        let obj2 = TestObject {
            value: 2,
            next: Some(Box::new(obj3)),
        };
        let obj1 = TestObject {
            value: 1,
            next: Some(Box::new(obj2)),
        };

        let gc_obj = mm.allocate(obj1).expect("Failed to allocate");

        // Check the linked structure
        assert_eq!(gc_obj.value, 1);
        assert_eq!(gc_obj.as_ref().next.as_ref().unwrap().value, 2);
        assert_eq!(
            gc_obj
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .value,
            3
        );

        // Force a garbage collection
        mm.collect().expect("Failed to collect garbage");

        // Objects should still be accessible
        assert_eq!(gc_obj.value, 1);
        assert_eq!(gc_obj.as_ref().next.as_ref().unwrap().value, 2);
        assert_eq!(
            gc_obj
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .value,
            3
        );
        
        info!("Test linked_objects completed successfully");
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_collection_unreachable() {
        // Initialize tracing for this test
        tracing_setup::setup();
        info!("Starting test_collection_unreachable");
        let mm = GarbageCollector::new();
        debug!("Created GarbageCollector instance");

        // Creating a scope where objects are allocated but not kept
        {
            debug!("Creating first temp object");
            let _obj1 = mm.allocate(TestObject {
                value: 1,
                next: None,
            }).expect("Failed to allocate");
            debug!("Creating second temp object");
            let _obj2 = mm.allocate(TestObject {
                value: 2,
                next: None,
            }).expect("Failed to allocate");

            // Objects exist here, verify stats
            let stats_before = mm.stats();
            debug!(live_objects = stats_before.current_objects, "Stats before GC");
            assert!(stats_before.current_objects >= 2);
            
            debug!("About to end scope, which will drop the Gc pointers");
        }
        debug!("Scope ended, GC pointers should be dropped");

        // Objects should be garbage collected
        info!("About to run garbage collection");
        mm.collect().expect("Failed to collect garbage");
        info!("Garbage collection completed");

        let stats_after = mm.stats();
        debug!(live_objects = stats_after.current_objects, "Stats after GC");
        
        let condition_met = stats_after.current_objects < 2;
        if !condition_met {
            error!(live_objects = stats_after.current_objects, "Expected fewer than 2 live objects after GC");
        }
        
        assert!(stats_after.current_objects < 2, "Expected fewer than 2 live objects after GC, but found {}", stats_after.current_objects);
        info!("Test collection_unreachable completed successfully");
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_stress() {
        // Initialize tracing for this test
        tracing_setup::setup();
        info!("Starting test_stress");
        let mm = GarbageCollector::new();

        // Allocate a bunch of objects
        debug!("Allocating 1000 test objects");
        let mut objects = Vec::new();
        for i in 0..1000 {
            objects.push(mm.allocate(TestObject {
                value: i,
                next: None,
            }).expect("Failed to allocate object"));
        }
        debug!(object_count = objects.len(), "Objects allocated");

        // Force a garbage collection
        info!("Running first garbage collection");
        mm.collect().expect("Failed to collect garbage");
        debug!("First garbage collection completed");

        // Objects should still be accessible
        debug!("Verifying all objects are still accessible");
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.value, i as i64);
        }
        debug!("All objects verified after first GC");

        // Drop half the objects
        debug!("Dropping half the objects");
        objects.truncate(500);
        debug!(remaining_objects = objects.len(), "Objects remaining after truncation");

        // Force another garbage collection
        info!("Running second garbage collection");
        mm.collect().expect("Failed to collect garbage");
        debug!("Second garbage collection completed");

        // Remaining objects should still be accessible
        debug!("Verifying remaining objects are still accessible");
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.value, i as i64);
        }
        debug!("All remaining objects verified after second GC");
        
        info!("Test stress completed successfully");
    }
}
