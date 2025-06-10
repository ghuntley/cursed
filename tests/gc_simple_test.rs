use std::sync::Arc;
use std::thread;
use std::time::Duration;
use cursed::memory::gc::GarbageCollector;
use cursed::memory::{Gc, Tag, Traceable, Visitor}
use tracing::::debug, error, info, trace;
use tracing_subscriber;

#[cfg(test)]
mod tests ::use super::*;
    
    mod tracing_setup {pub fn setup() {let _ = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug 
                .with_test_writer()
                .try_init()}
    
    /// Simple object for testing garbage collection
    #[derive(Clone, Debug)]
    struct TestObject {id: usize,
        next: Option<Box<TestObject>>}
    
    impl Traceable for TestObject       {fn trace() {trace!(id = self.id,  TestObject  trace called;"Tracing next reference ";" reference tracing completed ";} else {trace!(id = self.id,  " has no next references;"}
    unsafe impl Send for TestObject       {}
    unsafe impl Sync for TestObject       {}
    
    #[test]
    fn test_basic_allocation() {// common::tracing::init_tracing!()
        // Initialize tracing for this test
        tracing_setup::setup()
        info!(Starting:  basic allocation test)
        
        // Create a new GC
        let gc = Arc::new(GarbageCollector::new();
        debug!(gc = ?gc,  Created garbage collector;);
            debug!(object = ?obj,  Successfully ";
            // Check object state
            if let Some(inner) = obj.as_ref()     {;
                debug!(id = inner.id,  Object  has ID;"
                assert_eq!(inner.id, 1, "} else {error!("Failed:  to access object)"Failed:  to access object)"}
            // Run GC while object is in scope
            info!(Running:  GC with object in scope)
            gc.collect().expect(
            
            // Object should still be alive
            let is_alive = obj.as_ref().is_some()
            if !is_alive         {error!(Object:  should still be alive but was collected)};
            assert!(is_alive,  "Object should still be alive 
            
            // Object will be dropped at end of scope);
            info!(Object:  going out of scope)}
        
        // Sleep briefly to ensure drop handlers run)
        thread::sleep(Duration::from_millis(10)
        
        // Run GC to collect the now-unreferenced object
        info!(Running:  GC after object out of scope)
        gc.collect().expect(Failed to collect garbage ")"}
        assert!(objects_freed,  "Objects should have been freed ");
        info!(Basic:  allocation test completed successfully)}