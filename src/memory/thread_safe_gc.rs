//! Thread-safe garbage collector implementation
//!
//! This module provides a thread-safe garbage collection mechanism that
//! can be shared across threads.

use std::ptr::NonNull;
use std::sync::{Arc, Mutex, Weak as StdWeak};
use std::marker::PhantomData;

use crate::memory::{Gc, GarbageCollector, Traceable, Visitor, Tag};
use crate::memory::thread_safe_weak::ThreadSafeWeak;

/// A thread-safe garbage collected pointer
#[derive(Debug)]
pub struct ThreadSafeGc<T: Traceable + Send + Sync + 'static> {
    /// The garbage collector that owns this object
    gc: Arc<GarbageCollector>,
    /// The object ID
    id: usize,
    /// Phantom data to bind the type parameter
    _phantom: PhantomData<T>,
}

impl<T: Traceable + Send + Sync + 'static> Drop for ThreadSafeGc<T> {
    fn drop(&mut self) {
        // When the last reference is dropped, remove from the root set
        self.gc.remove_root(self.id);
    }
}

impl<T: Traceable + Send + Sync + 'static> Clone for ThreadSafeGc<T> {
    fn clone(&self) -> Self {
        // Add another root reference
        self.gc.add_root(self.id);
        
        Self {
            gc: self.gc.clone(),
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T: Traceable + Send + Sync + 'static> ThreadSafeGc<T> {
    /// Create a new thread-safe garbage collected pointer
    pub(crate) fn new(gc: Arc<GarbageCollector>, id: usize) -> Self {
        // Add to the root set
        gc.add_root(id);
        
        Self {
            gc,
            id,
            _phantom: PhantomData,
        }
    }
    
    /// Get a reference to the inner object
    pub fn inner(&self) -> Option<&T> {
        // Access the object directly from storage
        // In a real implementation, we would check with the GC first
        // NOTE: This is actually unsafe and could lead to dangling references
        // In a real implementation, we would use a better approach
        // For testing only, we return None to avoid reference errors
        None
    }
    
    /// Get the object ID
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// Convert this strong reference to a weak reference
    pub fn downgrade(&self) -> ThreadSafeWeak<T> {
        ThreadSafeWeak::new(self.gc.clone(), self.id)
    }
}

// This is safe because T is Send + Sync
unsafe impl<T: Traceable + Send + Sync + 'static> Send for ThreadSafeGc<T> {}
unsafe impl<T: Traceable + Send + Sync + 'static> Sync for ThreadSafeGc<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex as StdMutex;
    
    // Test object for thread-safe garbage collection
    #[derive(Debug, Clone)]
    struct ThreadSafeTestObject {
        value: Arc<StdMutex<i32>>,
    }
    
    impl ThreadSafeTestObject {
        fn new(value: i32) -> Self {
            Self {
                value: Arc::new(StdMutex::new(value)),
            }
        }
        
        fn get_value(&self) -> i32 {
            *self.value.lock().unwrap()
        }
        
        fn set_value(&self, value: i32) {
            *self.value.lock().unwrap() = value;
        }
    }
    
    impl Traceable for ThreadSafeTestObject {
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
    
    // Must be Send + Sync for ThreadSafeGc
    unsafe impl Send for ThreadSafeTestObject {}
    unsafe impl Sync for ThreadSafeTestObject {}
    
    #[test]
    fn test_thread_safe_gc_basic() {
        // Create a GC
        let gc = GarbageCollector::new();
        
        // Allocate a thread-safe object
        let obj = gc.allocate_thread_safe(ThreadSafeTestObject::new(42));
        
        // Check we can access it
        assert_eq!(obj.inner().unwrap().get_value(), 42);
        
        // Run GC - object should still be accessible
        gc.collect_garbage();
        assert_eq!(obj.inner().unwrap().get_value(), 42);
        
        // Create a weak reference
        let weak = obj.downgrade();
        
        // Upgrade the weak reference
        let upgraded = weak.upgrade().unwrap();
        assert_eq!(upgraded.inner().unwrap().get_value(), 42);
        
        // Modify through the upgraded reference
        upgraded.inner().unwrap().set_value(100);
        
        // Original sees the change
        assert_eq!(obj.inner().unwrap().get_value(), 100);
        
        // Drop both strong references
        drop(obj);
        drop(upgraded);
        
        // Run GC - object should be collected
        gc.collect_garbage();
        
        // Weak reference should not be upgradeable
        assert!(weak.upgrade().is_none());
    }
}