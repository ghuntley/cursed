//! Object storage system for the garbage collector
//!
//! This module provides a central storage system for objects managed by the garbage collector,
//! with support for storing dependencies between objects for improved finalization ordering.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::ptr::NonNull;
use std::any::Any;
use once_cell::sync::Lazy;

use crate::memory::{Traceable, Tag};
use crate::debug_println;

/// A wrapper around an object that can be stored in the object storage
#[derive(Debug)]
pub struct StorageWrapper {
    /// The actual object data
    object: NonNull<dyn Traceable>,
    /// Object dependencies (for finalization ordering)
    dependencies: HashSet<usize>,
}

// Implement Send and Sync for StorageWrapper
// This is needed because NonNull<dyn Traceable> is not Send or Sync by default
// This is safe because we control access through the RwLock
unsafe impl Send for StorageWrapper {}
unsafe impl Sync for StorageWrapper {}

impl StorageWrapper {
    /// Create a new wrapper around an object
    pub fn new(object: NonNull<dyn Traceable>) -> Self {
        Self {
            object,
            dependencies: HashSet::new(),
        }
    }
    
    /// Get the wrapped object
    pub fn object(&self) -> NonNull<dyn Traceable> {
        self.object
    }
    
    /// Add a dependency on another object
    pub fn add_dependency(&mut self, id: usize) {
        self.dependencies.insert(id);
    }
    
    /// Get the object's dependencies
    pub fn dependencies(&self) -> &HashSet<usize> {
        &self.dependencies
    }
}

/// Central storage for all garbage-collected objects
#[derive(Debug, Default)]
pub struct ObjectStorage {
    /// Map of object ID to storage wrapper
    objects: HashMap<usize, StorageWrapper>,
    /// Counter for generating unique object IDs
    next_id: usize,
}

impl ObjectStorage {
    /// Create a new, empty object storage
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 1, // Start at 1, reserve 0 for null/invalid
        }
    }
    
    /// Store an object and return its ID
    pub fn store<T: Traceable + 'static>(&mut self, object: Box<T>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        // Convert to raw pointer
        let raw_ptr = Box::into_raw(object);
        let nn_ptr = unsafe { NonNull::new_unchecked(raw_ptr as *mut dyn Traceable) };
        
        // Create wrapper and store
        let wrapper = StorageWrapper::new(nn_ptr);
        self.objects.insert(id, wrapper);
        
        debug_println!("Stored object with ID {} (type: {:?})", id, unsafe { nn_ptr.as_ref().tag() });
        id
    }
    
    /// Store an object at a specific ID (used for consistency with GC)
    pub fn store_at_id<T: Traceable + 'static>(&mut self, object: Box<T>, id: usize) -> usize {
        // Convert to raw pointer
        let raw_ptr = Box::into_raw(object);
        let nn_ptr = unsafe { NonNull::new_unchecked(raw_ptr as *mut dyn Traceable) };
        
        // Create wrapper and store at the specified ID
        let wrapper = StorageWrapper::new(nn_ptr);
        self.objects.insert(id, wrapper);
        
        // Update next_id if necessary to avoid future collisions
        if id >= self.next_id {
            self.next_id = id + 1;
        }
        
        debug_println!("Stored object at specified ID {} (type: {:?})", id, unsafe { nn_ptr.as_ref().tag() });
        id
    }
    
    /// Get an object by ID
    pub fn get<T: Traceable + 'static>(&self, id: usize) -> Option<&T> {
        self.objects.get(&id).map(|wrapper| {
            let traceable_ptr = wrapper.object();
            unsafe {
                // This is unsafe but needed to cast from the trait object to the concrete type
                // In a production implementation, we would use proper type information
                let obj = traceable_ptr.as_ptr() as *const T;
                &*obj
            }
        })
    }
    
    /// Get the storage wrapper for an object by ID
    pub fn get_wrapper(&self, id: usize) -> Option<&StorageWrapper> {
        self.objects.get(&id)
    }
    
    /// Get a reference to a traceable object by ID (needed for thread-safe GC)
    pub fn get_traceable_ref<T: Traceable + 'static>(&self, id: usize) -> Option<&T> {
        self.objects.get(&id).map(|wrapper| {
            let traceable_ptr = wrapper.object();
            unsafe {
                // This is unsafe but needed to cast from the trait object to the concrete type
                let obj = traceable_ptr.as_ptr() as *const T;
                &*obj
            }
        })
    }
    
    /// Check if an object exists
    pub fn contains(&self, id: usize) -> bool {
        self.objects.contains_key(&id)
    }
    
    /// Remove an object
    pub fn remove(&mut self, id: usize) -> Option<NonNull<dyn Traceable>> {
        self.objects.remove(&id).map(|wrapper| wrapper.object())
    }
    
    /// Add a dependency between objects
    pub fn add_dependency(&mut self, from_id: usize, to_id: usize) {
        if let Some(wrapper) = self.objects.get_mut(&from_id) {
            wrapper.add_dependency(to_id);
        }
    }
    
    /// Get all object IDs
    pub fn all_ids(&self) -> Vec<usize> {
        self.objects.keys().copied().collect()
    }
    
    /// Get number of stored objects
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

/// Trait extension to allow Any downcasting for Traceable objects
trait TraceableAsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Traceable + 'static> TraceableAsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Global object storage for the garbage collector
static GLOBAL_OBJECT_STORAGE: Lazy<RwLock<ObjectStorage>> = 
    Lazy::new(|| RwLock::new(ObjectStorage::new()));

/// Get the global object storage
pub fn global_object_storage() -> &'static RwLock<ObjectStorage> {
    &GLOBAL_OBJECT_STORAGE
}

/// Helper function to store an object in the global storage (test only)
pub fn store<T: Traceable + 'static>(obj: T) -> usize {
    if let Ok(mut storage) = GLOBAL_OBJECT_STORAGE.write() {
        storage.store(Box::new(obj))
    } else {
        panic!("Failed to acquire write lock on global object storage")
    }
}

/// Helper function to check if an object exists in the global storage (test only)
pub fn contains(id: usize) -> bool {
    if let Ok(storage) = GLOBAL_OBJECT_STORAGE.read() {
        storage.contains(id)
    } else {
        false
    }
}

/// Register a dependency between two objects (for finalization ordering)
pub fn register_dependency(from_id: usize, to_id: usize) {
    if let Ok(mut storage) = GLOBAL_OBJECT_STORAGE.write() {
        storage.add_dependency(from_id, to_id);
    } else {
        debug_println!("Warning: Could not register dependency from {} to {}", from_id, to_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug)]
    struct TestObject {
        value: i32,
    }
    
    impl TestObject {
        fn new(value: i32) -> Self {
            Self { value }
        }
    }
    
    impl Traceable for TestObject {
        fn trace(&self, _visitor: &mut dyn crate::memory::Visitor) {
            // No references to trace
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
    }
    
    #[test]
    fn test_object_storage_basic() {
        let mut storage = ObjectStorage::new();
        
        // Store an object
        let obj1 = Box::new(TestObject::new(42));
        let id1 = storage.store(obj1);
        
        // Store another object
        let obj2 = Box::new(TestObject::new(100));
        let id2 = storage.store(obj2);
        
        // Check we can retrieve them
        let retrieved1 = storage.get::<TestObject>(id1).unwrap();
        let retrieved2 = storage.get::<TestObject>(id2).unwrap();
        
        assert_eq!(retrieved1.value, 42);
        assert_eq!(retrieved2.value, 100);
        
        // Add a dependency
        storage.add_dependency(id1, id2);
        
        // Check dependency
        let wrapper1 = storage.objects.get(&id1).unwrap();
        assert!(wrapper1.dependencies().contains(&id2));
        
        // Clean up
        let ptr1 = storage.remove(id1).unwrap();
        let ptr2 = storage.remove(id2).unwrap();
        
        unsafe {
            let _boxed1 = Box::from_raw(ptr1.as_ptr().cast::<TestObject>());
            let _boxed2 = Box::from_raw(ptr2.as_ptr().cast::<TestObject>());
        }
    }
}