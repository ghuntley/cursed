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

        fn size(&self) -> usize {
            std::mem::size_of::<TestObject>()
        }

        fn tag(&self) -> Tag {
            Tag::Object
        }
    }

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
        });
        debug!(ptr = ?obj.ptr(), "Allocated test object");

        debug!("About to access object inner value");
        let inner = obj.inner();
        debug!(has_inner = inner.is_some(), "Got inner value");
        
        match &inner {
            Some(inner_obj) => {
                info!(value = inner_obj.value, "Inner object value");
                assert_eq!(inner_obj.value, 42);
                
                debug!("Checking inner object next field");
                assert!(inner_obj.next.is_none())
            },
            None => {
                error!("FAILED! Object inner is None");
                panic!("Object inner is None");
            }
        }

        // Force a garbage collection
        info!("About to run garbage collection");
        mm.collect_garbage();
        info!("Garbage collection completed");

        // Object should still be accessible after collection
        debug!("About to access object after GC");
        let inner_after_gc = obj.inner();
        debug!(inner_present = inner_after_gc.is_some(), "Inner after GC");
        
        match &inner_after_gc {
            Some(inner_obj) => {
                debug!(value = inner_obj.value, "Inner object value after GC");
                assert_eq!(inner_obj.value, 42);
            },
            None => {
                error!("Object inner after GC is None");
                panic!("Object inner after GC is None");
            }
        }
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

        let gc_obj = mm.allocate(obj1);

        // Check the linked structure
        assert_eq!(gc_obj.inner().unwrap().value, 1);
        assert_eq!(gc_obj.inner().unwrap().next.as_ref().unwrap().value, 2);
        assert_eq!(
            gc_obj
                .inner()
                .unwrap()
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
        mm.collect_garbage();

        // Objects should still be accessible
        assert_eq!(gc_obj.inner().unwrap().value, 1);
        assert_eq!(gc_obj.inner().unwrap().next.as_ref().unwrap().value, 2);
        assert_eq!(
            gc_obj
                .inner()
                .unwrap()
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
            });
            debug!("Creating second temp object");
            let _obj2 = mm.allocate(TestObject {
                value: 2,
                next: None,
            });

            // Objects exist here, verify stats
            let stats_before = mm.stats();
            debug!(live_objects = stats_before.live_objects, "Stats before GC");
            assert!(stats_before.live_objects >= 2);
            
            debug!("About to end scope, which will drop the Gc pointers");
        }
        debug!("Scope ended, GC pointers should be dropped");

        // Objects should be garbage collected
        info!("About to run garbage collection");
        mm.collect_garbage();
        info!("Garbage collection completed");

        let stats_after = mm.stats();
        debug!(live_objects = stats_after.live_objects, "Stats after GC");
        
        let condition_met = stats_after.live_objects < 2;
        if !condition_met {
            error!(live_objects = stats_after.live_objects, "Expected fewer than 2 live objects after GC");
        }
        
        assert!(stats_after.live_objects < 2, "Expected fewer than 2 live objects after GC, but found {}", stats_after.live_objects);
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
            }));
        }
        debug!(object_count = objects.len(), "Objects allocated");

        // Force a garbage collection
        info!("Running first garbage collection");
        mm.collect_garbage();
        debug!("First garbage collection completed");

        // Objects should still be accessible
        debug!("Verifying all objects are still accessible");
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.inner().unwrap().value, i as i64);
        }
        debug!("All objects verified after first GC");

        // Drop half the objects
        debug!("Dropping half the objects");
        objects.truncate(500);
        debug!(remaining_objects = objects.len(), "Objects remaining after truncation");

        // Force another garbage collection
        info!("Running second garbage collection");
        mm.collect_garbage();
        debug!("Second garbage collection completed");

        // Remaining objects should still be accessible
        debug!("Verifying remaining objects are still accessible");
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.inner().unwrap().value, i as i64);
        }
        debug!("All remaining objects verified after second GC");
        
        info!("Test stress completed successfully");
    }
}
