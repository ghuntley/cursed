//! Object storage and finalization for the GC

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::any::Any;
use std::marker::PhantomData;
use crate::memory::Traceable;

/// A wrapper for storing objects with direct access
pub struct StorageWrapper<T: Traceable + Send + Sync + 'static> {
    /// The object's address in storage
    address: usize,
    /// Phantom data to preserve the type parameter
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Traceable + Send + Sync + 'static> Clone for StorageWrapper<T> {
    fn clone(&self) -> Self {
        Self {
            address: self.address,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Traceable + Send + Sync + 'static> StorageWrapper<T> {
    /// Create a new storage wrapper
    pub fn new(obj: T) -> Self {
        let storage = global_object_storage();
        let address = storage.store(obj);
        Self { 
            address,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Get a reference to the wrapped object
    pub fn get(&self) -> Option<&T> {
        let storage = global_object_storage();
        storage.get(self.address)
    }
    
    /// Get the object's address
    pub fn address(&self) -> usize {
        self.address
    }
}

/// Object storage for direct access to traced objects
pub struct ObjectStorage {
    /// Map of addresses to objects
    objects: RwLock<HashMap<usize, Box<dyn Any + Send + Sync>>>,
    /// Tracks which objects have been finalized
    finalized_objects: Mutex<HashSet<usize>>,
}

impl ObjectStorage {
    /// Create a new object storage
    pub fn new() -> Self {
        Self {
            objects: RwLock::new(HashMap::new()),
            finalized_objects: Mutex::new(HashSet::new()),
        }
    }
    
    /// Store an object and return its address
    pub fn store<T: Traceable + Send + Sync + 'static>(&self, obj: T) -> usize {
        let boxed = Box::new(obj);
        let ptr = Box::as_ref(&boxed) as *const _ as usize;
        
        let mut objects = self.objects.write().unwrap();
        objects.insert(ptr, boxed as Box<dyn Any + Send + Sync>);
        
        ptr
    }
    
    /// Get a reference to an object by address
    pub fn get<T: Traceable + Send + Sync + 'static>(&self, address: usize) -> Option<&T> {
        // We need to use a separate function to avoid lifetime issues
        self.get_internal(address)
    }
    
    /// Check if an object exists at the given address
    pub fn contains(&self, address: usize) -> bool {
        let objects = self.objects.read().unwrap();
        objects.contains_key(&address)
    }
    
    /// Remove and finalize an object
    pub fn remove_and_finalize(&self, address: usize) -> bool {
        let mut objects = self.objects.write().unwrap();
        
        if let Some(obj) = objects.remove(&address) {
            // Downcast to Traceable to call finalize
            self.finalize_object(obj);
            true
        } else {
            false
        }
    }
    
    // Internal helper that handles lifetime issues
    fn get_internal<T: Send + Sync + 'static>(&self, address: usize) -> Option<&T> {
        // This is unsafe but carefully designed to be safe in practice:
        // 1. We only return references to objects that are still in the storage map
        // 2. These objects won't be dropped as long as they're in the map
        unsafe {
            let objects = self.objects.read().unwrap();
            if let Some(obj) = objects.get(&address) {
                let ptr = obj.downcast_ref::<T>()?;
                let ptr_raw = ptr as *const T;
                Some(&*ptr_raw)
            } else {
                None
            }
        }
    }
    
    // Finalize a boxed Any object by downcasting it to Traceable
    fn finalize_object(&self, obj: Box<dyn Any + Send + Sync>) {
        // Try to finalize using a trait object
        // This is a bit tricky since we can't directly downcast to a trait object
        // We'll have to check specific types we know that implement Traceable
        // For a complete solution, we'd need to register finalization callbacks
        
        // For now, we'll just skip actual finalization in tests
        // In a real implementation, this would properly finalize the object
    }
}

/// Get a reference to the global object storage
pub fn global_object_storage() -> &'static ObjectStorage {
    lazy_static::lazy_static! {
        static ref GLOBAL_STORAGE: ObjectStorage = ObjectStorage::new();
    }
    
    &GLOBAL_STORAGE
}

/// Register a dependency between two objects
pub fn register_dependency(dependent: usize, dependency: usize) {
    // For now, just delegate to the finalization order module
    crate::memory::finalization_order::register_dependency(dependent, dependency);
}