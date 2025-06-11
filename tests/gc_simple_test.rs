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
                .try_init();
        }
    }

    #[derive(Debug)]
    struct TestObject {
        id: u32,
        data: String,
    }

    impl Traceable for TestObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // Simple object with no references to trace
        }
    }

    // No need to implement Storable manually - it's automatically implemented for Traceable + Send + Sync + 'static

    #[test]
    fn simple_gc_allocation_test() {
        tracing_setup::setup();
        
        let mut gc = GarbageCollector::new();
        let obj = TestObject {
            id: 42,
            data: "test data".to_string(),
        };
        
        let gc_obj = gc.allocate(obj).expect("Failed to allocate");
        debug!(id = gc_obj.id, "Object has ID");
        gc.collect().expect("Failed to collect garbage");
        info!("Test completed successfully");
    }

    #[test]
    fn simple_gc_multiple_objects_test() {
        tracing_setup::setup();
        
        let mut gc = GarbageCollector::new();
        
        for i in 0..10 {
            let obj = TestObject {
                id: i,
                data: format!("test data {}", i),
            };
            let _gc_obj = gc.allocate(obj).expect("Failed to allocate");
        }
        
        gc.collect().expect("Failed to collect garbage");
        info!("Test completed successfully");
    }
}
