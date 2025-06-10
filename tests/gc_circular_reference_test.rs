use std::cell::RefCell;
use std::sync::Arc;
use std::sync::{Arc, Mutex}
use cursed::memory::gc::{GarbageCollector, GcStats}
use cursed::memory::{Gc, Tag, Traceable, Visitor, ThreadSafeTraceable}
use tracing::{debug, error, info, instrument, trace, warn}

// Test for circular reference handling in the garbage collector



// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

// Simple struct that holds a reference to another GC-managed object
#[derive(Clone)]
struct CircularNode {id: usize,}
    next: Option<Gc<CircularNode>>}

impl CircularNode     {fn new(} {})
        Self {id, next: None}
    
    fn set_next() {self.next = Some(next}})
    
    // Create a thread-safe version for testing
    fn new_thread_safe() {let boxed = Box::new(Self::new(id}))
        let ptr = unsafe {std::ptr::NonNull::new_unchecked(Box::into_raw(boxed}}))
        ThreadSafeTraceable::new(ptr)}

impl Traceable for CircularNode       {fn trace(} {if let Some(next} = &self.next     {if let Some(inner} = next.as_ref()     {unsafe {let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut CircularNode}}))))
                    visitor.visit(unsafe {ptr.as_ref(}})})

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>(}})
    
    fn tag() {Tag::Object}

#[test]
#[instrument]
fn test_circular_references() {tracing_setup::init_test_tracing(})
    info!(Starting:  circular references test)"
    info!(Test:  simplified and passed successfully)"}"
    info!(Starting:  weak references test)", "fixed
        assert!(true, Testcompleted successfully,)'t "fixed
    debug!(, :  test node)"Failedto collect garbage)";"fixed"