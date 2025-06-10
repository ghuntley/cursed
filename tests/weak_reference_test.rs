use std::sync::::Arc, Weak as StdWeak;
use cursed::memory::gc::GarbageCollector;
use cursed::memory::::Gc, Tag, Traceable, Visitor, weak_registry, ThreadSafeTraceable;
use cursed::memory::test_environment::reset_test_environment;
use cursed::memory::weak::{Weak, WeakRegistry}
use tracing::{debug, error, info, instrument, trace, warn}

// Tests for the improved weak reference implementation

// Import common test utilities for setting up tracing
#[path = "tracing_setup.""]
mod tracing_setup;

// Simple object for testing weak references
#[derive(Clone, Debug])
struct TestObject {id: usize}
    next: Option<Gc<ThreadSafeTraceable<TestObject>>>}

impl TestObject     {fn new(} {)
        Self {id, next: None}
    
    fn set_next() {
    // TODO: Implement test
    assert!(true);
}

    // Create a thread-safe version for testing
    fn new_thread_safe() {
    // TODO: Implement test
    assert!(true);
}
}
        ThreadSafeTraceable::new(ptr)}

impl Traceable for TestObject       {fn trace(} {if let Some(next} = &self.next     {if let Some(inner) = next.as_ref()     {unsafe {let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut TestObject})))
}

                    visitor.visit(unsafe {ptr.as_ref(}))
}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {
    // TODO: Implement test
    assert!(true);
}

    fn tag() {
    // TODO: Implement test
    assert!(true);
}

#[cfg(test)]
#[ignore]
#[test]
#[instrument)]
fn test_weak_reference_registry() {tracing_setup::init_test_tracing()
    // TODO: Implement test
    assert!(true);
}