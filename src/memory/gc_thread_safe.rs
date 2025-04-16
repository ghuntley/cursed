//! Implementation of thread-safe GC provider for the garbage collector
//!
//! This module provides the implementation of the ThreadSafeGcProvider trait
//! for the GarbageCollector, allowing it to allocate thread-safe objects.

use std::sync::{Arc, Mutex};
use tracing::{debug, info, error, warn};

use crate::memory::{GarbageCollector, Traceable, ThreadSafeGc, ThreadSafeGcProvider};
use crate::memory::thread_safe_gc_new::{ThreadSafeGcRegistry};

impl ThreadSafeGcProvider for GarbageCollector {
    fn allocate_thread_safe<T: Traceable + Clone + Send + Sync + 'static>(&self, value: T) -> ThreadSafeGc<T> {
        debug!(type_name = %std::any::type_name::<T>(), "Allocating thread-safe object");
        
        // Box the value first to get a stable pointer that we'll use consistently as the ID
        let boxed = Box::new(value.clone());
        let ptr = Box::into_raw(boxed);
        let obj_id = ptr as usize;
        
        // Store the object in the global object storage with the chosen ID
        let storage = crate::memory::object_storage::global_object_storage();
        if let Ok(mut storage_lock) = storage.write() {
            // Store the object with the specific ID we chose
            storage_lock.store_at_id(Box::new(value.clone()), obj_id);
            debug!(obj_id = obj_id, "Stored thread-safe object in global storage");
        } else {
            error!("Failed to lock storage for storing thread-safe object");
        }
        
        // Store in GC internal structures (we can't access the private method)
        // but we can simulate the effect by storing in the object storage
        
        // Create and return the thread-safe GC pointer
        ThreadSafeGc::new(Arc::new(self.clone()), obj_id)
    }
    
    fn allocate_thread_safe_with_registry<T: Traceable + Clone + Send + Sync + 'static>(
        &self, 
        value: T, 
        registry: Arc<Mutex<ThreadSafeGcRegistry>>
    ) -> ThreadSafeGc<T> {
        debug!(type_name = %std::any::type_name::<T>(), "Allocating thread-safe object with registry");
        
        // Box the value first to get a stable pointer that we'll use consistently as the ID
        let boxed = Box::new(value.clone());
        let ptr = Box::into_raw(boxed);
        let obj_id = ptr as usize;
        
        // Register the object with the provided registry
        if let Ok(mut registry_guard) = registry.lock() {
            registry_guard.register(obj_id);
        } else {
            error!("Failed to lock registry for registering thread-safe object");
        }
        
        // Store the object in the global object storage with the chosen ID
        let storage = crate::memory::object_storage::global_object_storage();
        if let Ok(mut storage_lock) = storage.write() {
            // Store the object with the specific ID we chose
            storage_lock.store_at_id(Box::new(value.clone()), obj_id);
            debug!(obj_id = obj_id, "Stored thread-safe object in global storage");
        } else {
            error!("Failed to lock storage for storing thread-safe object");
        }
        
        // Store in GC internal structures (we can't access the private method)
        // but we can simulate the effect by storing in the object storage
        
        // Create and return the thread-safe GC pointer
        ThreadSafeGc::new(Arc::new(self.clone()), obj_id)
    }
}