use std::sync::Arc;
use std::thread;
use std::time::Duration;
use cursed::memory::gc::GarbageCollector;
use cursed::memory::{Gc, Traceable, Visitor, Storable};
use tracing::{debug, error, info, trace};
use tracing_subscriber;

#[cfg(test)]
mod tests {
    use super::*;
    
    mod tracing_setup {
        pub fn setup() {
            let _ = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug")
                .init();
        }
    }

    #[derive(Debug, Clone)]
    struct TestObject {
        id: u32,
    }

    impl Traceable for TestObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // Simple object, no references to trace
        }
    }

    #[test]
    fn test_simple_allocation() {
        tracing_setup::setup();
        
        let mut gc = GarbageCollector::new();
        
        let obj = TestObject { id: 1 };
        let gc_obj = gc.allocate(obj).expect("Failed to allocate");
        
        let inner = gc_obj.as_ref();
        debug!(id = inner.id, "Object has ID");
        assert_eq!(inner.id, 1);
        
        gc.collect().expect("Failed to collect garbage");
        
        let inner_after = gc_obj.as_ref();
        assert_eq!(inner_after.id, 1);
        
        info!("Test completed successfully");
    }

    #[test]
    fn test_simple_collection() {
        tracing_setup::setup();
        
        let mut gc = GarbageCollector::new();
        
        {
            let obj = TestObject { id: 2 };
            let _gc_obj = gc.allocate(obj).expect("Failed to allocate");
        }
        
        gc.collect().expect("Failed to collect garbage");
        info!("Test completed successfully");
    }
}
