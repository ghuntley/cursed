#[cfg(test)]
mod tests {
    use cursed::memory::gc::GarbageCollector;
    use cursed::memory::{Gc, Tag, Traceable, Visitor};
    use std::cell::RefCell;

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
        let mm = GarbageCollector::new();
        let obj = mm.allocate(TestObject {
            value: 42,
            next: None,
        });

        assert_eq!(obj.inner().unwrap().value, 42);
        assert!(obj.inner().unwrap().next.is_none());

        // Force a garbage collection
        mm.collect_garbage();

        // Object should still be accessible after collection
        assert_eq!(obj.inner().unwrap().value, 42);
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_linked_objects() {
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
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_collection_unreachable() {
        let mm = GarbageCollector::new();

        // Creating a scope where objects are allocated but not kept
        {
            let _obj1 = mm.allocate(TestObject {
                value: 1,
                next: None,
            });
            let _obj2 = mm.allocate(TestObject {
                value: 2,
                next: None,
            });

            // Objects exist here, verify stats
            let stats_before = mm.stats();
            assert!(stats_before.live_objects >= 2);
        }

        // Objects should be garbage collected
        mm.collect_garbage();

        let stats_after = mm.stats();
        assert!(stats_after.live_objects < 2);
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_stress() {
        let mm = GarbageCollector::new();

        // Allocate a bunch of objects
        let mut objects = Vec::new();
        for i in 0..1000 {
            objects.push(mm.allocate(TestObject {
                value: i,
                next: None,
            }));
        }

        // Force a garbage collection
        mm.collect_garbage();

        // Objects should still be accessible
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.inner().unwrap().value, i as i64);
        }

        // Drop half the objects
        objects.truncate(500);

        // Force another garbage collection
        mm.collect_garbage();

        // Remaining objects should still be accessible
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.inner().unwrap().value, i as i64);
        }
    }
}
