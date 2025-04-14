//! Thread-safe weak reference implementation for the GC

use std::sync::{Arc, Weak, Mutex, RwLock};
use std::time::Duration;
use std::marker::PhantomData;
use std::thread::current;

use crate::memory::gc::GarbageCollector;
use crate::memory::{ThreadSafeGc, Traceable, Visitor};
use crate::memory::weak_registry::{global_registry, GlobalWeakRegistry};
use crate::debug_println;

/// Timeout for lock acquisitions to prevent deadlocks
const LOCK_TIMEOUT: Duration = Duration::from_millis(100);

/// A weak reference to a ThreadSafeGc-managed object
/// This can be safely shared across thread boundaries
pub struct ThreadSafeWeak<T: Traceable + Send + Sync + 'static> {
    /// The object ID used in the weak reference registry
    id: usize,
    /// A weak reference to the garbage collector
    gc: Weak<GarbageCollector>,
    /// Reference to the global registry (to extend lifetime)
    registry: &'static GlobalWeakRegistry,
    /// Phantom data to bind the type parameter
    _phantom: PhantomData<T>,
}

// Implement Debug for ThreadSafeWeak
impl<T: Traceable + Send + Sync + 'static> std::fmt::Debug for ThreadSafeWeak<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThreadSafeWeak")
            .field("id", &self.id)
            .field("is_alive", &self.is_alive())
            .finish()
    }
}

impl<T: Traceable + Send + Sync + 'static> Clone for ThreadSafeWeak<T> {
    fn clone(&self) -> Self {
        // Register another reference in the registry
        let thread_id = format!("{:?}", current().id());
        debug_println!("[{}] Cloning ThreadSafeWeak for object id={}", thread_id, self.id);
        self.registry.register(self.id);
        
        Self {
            id: self.id,
            gc: self.gc.clone(),
            registry: self.registry,
            _phantom: PhantomData,
        }
    }
}

impl<T: Traceable + Send + Sync + 'static> Drop for ThreadSafeWeak<T> {
    fn drop(&mut self) {
        // Unregister this reference from the registry
        let thread_id = format!("{:?}", current().id());
        debug_println!("[{}] Dropping ThreadSafeWeak for object id={}", thread_id, self.id);
        self.registry.unregister(self.id);
    }
}

impl<T: Traceable + Send + Sync + 'static> ThreadSafeWeak<T> {
    /// Create a new weak reference from a ThreadSafeGc and an object ID
    pub(crate) fn new(gc: &Arc<GarbageCollector>, id: usize) -> Self {
        // Register this weak reference in the global registry
        let thread_id = format!("{:?}", current().id());
        debug_println!("[{}] Creating new ThreadSafeWeak for object id={}", thread_id, id);
        let registry = global_registry();
        registry.register(id);
        debug_println!("[{}] Registered weak reference for object id={} in global registry", thread_id, id);
        
        Self {
            id,
            gc: Arc::downgrade(gc),
            registry,
            _phantom: PhantomData,
        }
    }
    
    /// Attempt to upgrade to a strong reference
    pub fn upgrade(&self) -> Option<ThreadSafeGc<T>> {
        let thread_id = format!("{:?}", current().id());
        debug_println!("[{}] Attempting to upgrade ThreadSafeWeak for object id={}", thread_id, self.id);
        
        // First, check if the registry still has this object registered
        // Important to check registry before trying to upgrade GC to avoid race conditions
        if !self.registry.is_registered(self.id) {
            debug_println!("[{}] Object id={} not found in registry, cannot upgrade", thread_id, self.id);
            return None;
        }
        debug_println!("[{}] Object id={} found in registry", thread_id, self.id);
        
        // Then upgrade the GC reference using atomic CAS to handle race conditions
        let gc = match self.gc.upgrade() {
            Some(gc) => {
                debug_println!("[{}] Successfully upgraded GC reference for object id={}", thread_id, self.id);
                gc
            },
            None => {
                debug_println!("[{}] Failed to upgrade GC reference for object id={}", thread_id, self.id);
                return None;
            }
        };
        
        // Before creating a new strong reference, check the registry again
        // to ensure the object wasn't collected during our upgrade process
        if !self.registry.is_registered(self.id) {
            debug_println!("[{}] Object id={} was collected during upgrade, cannot create strong reference", thread_id, self.id);
            return None;
        }
        
        // Now use the GC's atomic upgrade operation which will verify the object still exists
        // and create a new strong reference in one atomic operation
        let result = gc.upgrade_from_weak::<T>(self.id);
        
        if result.is_some() {
            debug_println!("[{}] Successfully upgraded weak reference to strong reference for object id={}", thread_id, self.id);
        } else {
            debug_println!("[{}] Failed to upgrade weak reference to strong reference for object id={}", thread_id, self.id);
        }
        
        result
    }
    
    /// Check if the weak reference is still alive
    pub fn is_alive(&self) -> bool {
        let thread_id = format!("{:?}", current().id());
        debug_println!("[{}] Checking if weak reference for object id={} is alive", thread_id, self.id);
        
        // Fast path: first check if the registry still has this object registered
        // This is the most common reason for an object not being alive
        let is_registered = self.registry.is_registered(self.id);
        if !is_registered {
            debug_println!("[{}] Object id={} not registered in global registry", thread_id, self.id);
            return false;
        }
        
        // Then check if the GC is still alive
        if self.gc.upgrade().is_none() {
            debug_println!("[{}] GC no longer exists for object id={}", thread_id, self.id);
            return false;
        }
        
        debug_println!("[{}] Object id={} is alive (registered in global registry)", thread_id, self.id);
        true
    }
    
    /// Get the object ID
    pub fn id(&self) -> usize {
        self.id
    }
}

// Test-only functions
#[cfg(test)]
impl<T: Traceable + Send + Sync + 'static> ThreadSafeWeak<T> {
    /// Create a placeholder weak reference for testing
    pub fn new_for_test(id: usize) -> Self {
        // Register this weak reference in the global registry
        let registry = global_registry();
        registry.register(id);
        
        Self {
            id,
            gc: Weak::new(),
            registry,
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Tag;
    
    // Simple test object
    #[derive(Debug)]
    struct TestObject {
        value: Arc<RwLock<i32>>,
    }
    
    impl TestObject {
        fn new(value: i32) -> Self {
            Self {
                value: Arc::new(RwLock::new(value)),
            }
        }
        
        fn get_value(&self) -> i32 {
            *self.value.read().unwrap()
        }
    }
    
    impl Traceable for TestObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // No references to trace
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
    }
    
    // Must be Send + Sync for ThreadSafeWeak
    unsafe impl Send for TestObject {}
    unsafe impl Sync for TestObject {}
    
    #[test]
    fn test_thread_safe_weak_basics() {
        // Create a weak reference for testing
        let weak = ThreadSafeWeak::<TestObject>::new_for_test(12345);
        
        // Check basic properties
        assert_eq!(weak.id(), 12345);
        
        // Since this is a test-only weak reference with no real GC,
        // it shouldn't be considered alive
        assert!(!weak.is_alive());
        
        // Upgrading should fail
        assert!(weak.upgrade().is_none());
        
        // Create a clone
        let weak_clone = weak.clone();
        assert_eq!(weak_clone.id(), weak.id());
        
        // The registry should have 2 references
        assert_eq!(global_registry().ref_count(12345), 2);
        
        // Drop them both
        drop(weak);
        drop(weak_clone);
        
        // The registry should have 0 references
        assert_eq!(global_registry().ref_count(12345), 0);
    }
}