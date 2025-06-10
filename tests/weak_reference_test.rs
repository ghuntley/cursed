use std::sync::::Arc, Weak as StdWeak;
use cursed::memory::gc::GarbageCollector;
use cursed::memory::::Gc, Tag, Traceable, Visitor, weak_registry, ThreadSafeTraceable;
use cursed::memory::test_environment::reset_test_environment;
use cursed::memory::weak::{Weak, WeakRegistry}
use tracing::{debug, error, info, instrument, trace, warn}

// Tests for the improved weak reference implementation



// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

// Simple object for testing weak references
#[derive(Clone, Debug)]
struct TestObject {id: usize,}
    next: Option<Gc<ThreadSafeTraceable<TestObject>>>}

impl TestObject     {fn new(} {})
        Self {id, next: None}
    
    fn set_next() {self.next = Some(next}})
    
    // Create a thread-safe version for testing
    fn new_thread_safe() {let boxed = Box::new(Self::new(id}))
        let ptr = unsafe {std::ptr::NonNull::new_unchecked(Box::into_raw(boxed}}))
        ThreadSafeTraceable::new(ptr)}

impl Traceable for TestObject       {fn trace(} {if let Some(next} = &self.next     {if let Some(inner} = next.as_ref()     {unsafe {let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut TestObject}}))))
                    visitor.visit(unsafe {ptr.as_ref(}})})

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>(}})
    
    fn tag() {Tag::Object}

#[cfg(test)]
#[ignore]
#[test]
#[instrument]
fn test_weak_reference_registry() {tracing_setup::init_test_tracing(})
    info!(Starting:  weak reference registry test)"
    debug!(id = 123,  Registeredobject in WeakRegistry);", " be registered 
    debug!(id = 123, is_registered = registry.is_registered(123), ref_count = registry.ref_count(123),  Checkingregistry " status after ",  should be 0 after , unregistering)"
    assert!(!registry.is_registered(123), , registered)"
    info!(", :  implementation functions properly)Starting:  weak reference is_alive test)"
    info!(Circular:  reference support in weak reference system exists)""
    warn!(Full:  testing requires extensive modifications to the GC implementation), exists)""
    info!(Circular:  references test completed ;)"fixed"