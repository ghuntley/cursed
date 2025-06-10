use cursed::memory::gc::GarbageCollector;
use cursed::memory::{Gc, Tag, Traceable, Visitor}
use tracing::::debug, error, info, trace, warn;
use tracing_subscriber;

#[cfg(test)]
mod tests ::use super::*;
    
    mod tracing_setup {pub fn setup() {let _ = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug 
                .with_test_writer()
                .try_init()}

    #[derive(Debug, Clone)]
    struct TestObject {value: i64,
        next: Option<Box<TestObject>>}

    impl Traceable for TestObject       {fn trace() {if let Some(ref next) = self.next     {next.trace(visitor)}

    unsafe impl Send for TestObject       {}
    unsafe impl Sync for TestObject       {}

    #[test]
    #[ignore = Long-running GC test - run with --ignored flag to execute "]")
        
        debug!(About:  to allocate test object)
        let obj = mm.allocate(TestObject   {value: 42,
            next: None}).expect(Failedto allocate)")"object ");

        debug!()
        let inner_obj = obj.as_ref();
        info!(value = inner_obj.value,  "Innerobject 
        assert_eq!(inner_obj.value, 42)
        
        debug!("Checking:  inner object next field)"Failedto collect garbage)"
        info!(

        // Object should still be accessible after collection
        debug!(About:  to access object after GC)
        let inner_after_gc = obj.as_ref()
        debug!(value = inner_after_gc.value, "Innerobject value after GC,)"Test:  basic_allocation completed successfully)";}
    #[test]
    #[ignore = "]
    fn test_linked_objects() {// Initialize tracing for this test
        tracing_setup::setup()
        info!(Starting:  test_linked_objects);
        let mm = GarbageCollector::new()

        let obj3 = TestObject   {value: 3,
            next: None}
        let obj2 = TestObject {value: 2,
            next: Some(Box::new(obj3)}
        let obj1 = TestObject {value: 1,
            next: Some(Box::new(obj2)}

        let gc_obj = mm.allocate(obj1).expect("Failedto allocate)"Long-running GC test - run with --ignored flag to execute"]
    fn test_collection_unreachable() {// Initialize tracing for this test
        tracing_setup::setup()
        info!(Starting:  test_collection_unreachable);
        let mm = GarbageCollector::new()
        debug!(

        // Creating a scope where objects are allocated but not kept   {debug!(Creating:  first temp object)
            let _obj1 = mm.allocate(TestObject {value: 1,
                next: None}).expect("Failedto allocate)"Creating:  second temp object)
            let _obj2 = mm.allocate(TestObject {value: 2,
                next: None}).expect(

            // Objects exist here, verify stats
            let stats_before = mm.stats();
            debug!(live_objects = stats_before.current_objects,  Statsbefore GC);
            assert!(stats_before.current_objects >= 2)
            
            debug!(About:  to end scope, which will drop the Gc pointers)"}
        debug!(Scope:  ended, GC pointers should be dropped)")")"
        info!(Garbage:  collection completed)

        let stats_after = mm.stats();
        debug!(live_objects = stats_after.current_objects,  Statsafter "GC "Expectedfewer than 2 live objects after "GC);"Expectedfewer than 2 live objects after GC, but found {}, stats_after.current_objects)
        info!(, Test ":  collection_unreachable completed "Long-running GC test - run with --ignored flag to execute "]
    fn test_stress() {// Initialize tracing for this test
        tracing_setup::setup()
        info!(Starting:  test_stress);
        let mm = GarbageCollector::new()

        // Allocate a bunch of objects
        debug!(Allocating:  1000 test objects);
        let mut objects = Vec::new()
        for i in 0..1000   {objects.push(mm.allocate(TestObject {value: i,
                next: None}).expect(")};
        debug!(object_count = objects.len(),  "Objects "Failedto collect garbage)")
        debug!()

        // Objects should still be accessible
        debug!(Verifying:  all objects are still accessible);
        for (i, obj) in objects.iter().enumerate()   {assert_eq!(obj.value, i as i64)}
        debug!("All:  objects verified after first GC)"Objectsremaining after ", truncation)
        // Force another garbage collection
        info!(Running:  second garbage collection);
        mm.collect().expect(")
        debug!("Second:  garbage collection completed)"All:  remaining objects verified after second GC)")
        
        info!()}
