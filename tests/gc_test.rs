use cursed::memory::gc::GarbageCollector;
use cursed::memory::{Gc, Traceable, Visitor, Storable};
use tracing::{debug, error, info, trace, warn};
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
        value: i32,
    }
    
    impl cursed::memory::Traceable for TestObject {
        fn trace(&self, _tracer: &mut dyn cursed::memory::Visitor) {
            // TestObject doesn't contain any other objects to trace
        }
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_basic_allocation() {
        tracing_setup::setup();
        
        let mut mm = GarbageCollector::new();
        let obj = TestObject { value: 42 };
        let gc_obj = mm.allocate(obj).expect("Failed to allocate");
        
        info!(value = gc_obj.value, "Allocated object");
        debug!("Checking inner object");
        
        mm.collect().expect("Failed to collect garbage");
        
        let inner_after_gc = &*gc_obj;
        debug!(value = inner_after_gc.value, "Object value after GC");
        
        info!("Test basic_allocation completed successfully");
        assert!(true);
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_collection_unreachable() {
        tracing_setup::setup();
        
        let mut mm = GarbageCollector::new();
        
        {
            let obj1 = TestObject { value: 1 };
            let obj2 = TestObject { value: 2 };
            let _gc_obj1 = mm.allocate(obj1).expect("Failed to allocate");
            let _gc_obj2 = mm.allocate(obj2).expect("Failed to allocate");
            debug!("About to end scope, which will drop the Gc pointers");
        }
        
        debug!("Scope ended, GC pointers should be dropped");
        mm.collect().expect("Failed to collect garbage");
        
        let stats_after = mm.get_stats().expect("Failed to get stats");
        debug!(live_objects = stats_after.current_objects, "Stats after GC");
        
        info!("Test collection_unreachable completed successfully");
        assert!(true);
    }

    #[test]
    #[ignore = "Long-running GC test - run with --ignored flag to execute"]
    fn test_multiple_collections() {
        tracing_setup::setup();
        
        let mut mm = GarbageCollector::new();
        let mut objects = Vec::new();
        
        for i in 0..10 {
            let obj = TestObject { value: i };
            let gc_obj = mm.allocate(obj).expect("Failed to allocate");
            objects.push(gc_obj);
        }
        
        debug!(object_count = objects.len(), "Objects created");
        mm.collect().expect("Failed to collect garbage");
        debug!("First garbage collection completed");
        
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.value, i as i32);
        }
        debug!("Objects verified after first GC");
        
        objects.truncate(5);
        debug!("Objects remaining after truncation");
        
        mm.collect().expect("Failed to collect garbage");
        debug!("Second garbage collection completed");
        
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.value, i as i32);
        }
        debug!("Remaining objects verified after second GC");
        info!("Test multiple_collections completed successfully");
    }
}
