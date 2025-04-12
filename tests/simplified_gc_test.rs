//! A simplified test for the garbage collector

#[cfg(test)]
mod tests {
    use cursed::memory::gc::GarbageCollector;
    use cursed::memory::{Tag, Traceable, Visitor};
    use std::sync::Arc;

    // A simple object type for testing
    #[derive(Debug, Clone)]
    struct SimpleObject {
        value: i64,
    }

    impl Traceable for SimpleObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // Simple object has no references to trace
        }

        fn size(&self) -> usize {
            std::mem::size_of::<SimpleObject>()
        }

        fn tag(&self) -> Tag {
            Tag::Object
        }
    }

    #[test]
    fn test_simple_allocation() {
        let mm = GarbageCollector::new();
        let obj = mm.allocate(SimpleObject { value: 42 });

        assert_eq!(obj.inner().unwrap().value, 42);
    }
}
