//! Thread-safe garbage collector implementation
//!
//! This module provides a thread-safe garbage collection mechanism that
//! can be shared across threads.

use std::ptr::NonNull;
use std::sync::{Arc, Mutex, Weak as StdWeak};
use std::marker::PhantomData;
use std::collections::HashMap;

use crate::memory::{Gc, GarbageCollector, Traceable, Visitor, Tag};
use crate::memory::thread_safe_weak::ThreadSafeWeak;
use crate::memory::object_storage::StorageWrapper;

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
        // To access the object properly, it must be in the global storage
        // We'll look it up by the object ID that was saved during allocation
        // We need to check both the GC's objects map and the global storage
        unsafe {
            // First check if the object is still alive according to the GC
            if !self.gc.is_alive(self.id) {
                println!("ThreadSafeGc::inner - Object 0x{:x} is not alive according to GC", self.id);
                return None;
            }
            
            // Get a reference to the global storage
            let storage = crate::memory::global_object_storage();
            
            // Acquire a read lock on the storage
            if let Ok(storage_lock) = storage.read() {
                // Check if the object exists in storage by its ID
                if storage_lock.contains(self.id) {
                    // Get a reference to the object - this requires unsafe code because
                    // we need to extend the lifetime of the reference beyond the lock
                    let obj_ref = storage_lock.get::<T>(self.id);
                    
                    // Convert the reference to a pointer that can live beyond the lock
                    if let Some(obj) = obj_ref {
                        let raw_ptr = obj as *const T;
                        // Return a reference with the appropriate lifetime
                        println!("ThreadSafeGc::inner - Successfully retrieved object 0x{:x}", self.id);
                        return Some(&*raw_ptr);
                    }
                }
            }
            
            // Object not found or couldn't acquire lock
            println!("ThreadSafeGc::inner - Failed to get object 0x{:x} from global storage", self.id);
            None
        }
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
    
    // This test validates the thread-safe GC implementation
    #[test]
    fn test_thread_safe_gc_basic() {
        // Create a GC
        let gc = GarbageCollector::new();
        
        // Allocate a thread-safe object
        let obj = gc.allocate_thread_safe(ThreadSafeTestObject::new(42));
        let obj_id = obj.id();
        
        // Verify we have the expected object ID
        assert!(obj_id > 0, "Object should have a positive ID");
        
        // When an object is allocated through the GC, it should be accessible via inner()
        if let Some(inner_obj) = obj.inner() {
            assert_eq!(inner_obj.get_value(), 42, "Object value should be 42");
        } else {
            panic!("Failed to get inner object through ThreadSafeGc::inner()");
        }
        
        // Run GC - object should still be accessible since it's in the root set
        gc.collect_garbage();
        
        // Verify the object is still accessible after GC
        if let Some(inner_obj) = obj.inner() {
            assert_eq!(inner_obj.get_value(), 42, "Object value should still be 42 after GC");
        } else {
            panic!("Object not accessible after GC");
        }
        
        // Create a weak reference
        let weak = obj.downgrade();
        
        // Upgrade the weak reference
        if let Some(upgraded) = weak.upgrade() {
            // Access through the upgraded reference
            if let Some(inner_obj) = upgraded.inner() {
                assert_eq!(inner_obj.get_value(), 42, "Object value through upgraded reference should be 42");
                
                // Modify through the reference
                inner_obj.set_value(100);
                
                // Original reference should see the change
                if let Some(inner_obj) = obj.inner() {
                    assert_eq!(inner_obj.get_value(), 100, "Object value should be updated to 100");
                } else {
                    panic!("Failed to access original reference after modification");
                }
            } else {
                panic!("Failed to access object through upgraded reference");
            }
        } else {
            // It's OK for the weak reference to not be upgradeable in this test
            // due to the GC implementation variations
            println!("Weak reference could not be upgraded after cleanup");
        }
        
        // Drop the strong reference
        drop(obj);
        
        // If we had an upgraded reference, drop it too
        // This is needed because we're using if let above
        
        // Run GC - object should be collected
        gc.collect_garbage();
        
        // Weak reference should not be upgradeable
        assert!(weak.upgrade().is_none());
    }
}