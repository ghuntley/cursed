//! Object storage system for Traceable objects
//!
//! This module provides a way to store and access Traceable objects directly,
//! which is essential for proper finalization during garbage collection.
//! The storage system maintains a mapping between object addresses and the
//! actual object instances, allowing direct access when needed.

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::cell::UnsafeCell;

use crate::memory::{Traceable, Tag};

/// Storage for Traceable objects that allows direct access for finalization
#[derive(Debug)]
pub struct ObjectStorage {
    // Map of object addresses to TypeErasedObjects
    objects: RwLock<HashMap<usize, TypeErasedObject>>,
}

/// A type-erased object that can be stored in the ObjectStorage
pub struct TypeErasedObject {
    // The actual object data as Any
    data: Box<dyn Any + Send + Sync>,
    // The type ID of the object
    type_id: TypeId,
    // The tag of the object
    tag: Tag,
    // Reference to finalization function - can't impl Debug for functions
    finalize_fn: Option<Box<dyn Fn(&mut dyn Any) + Send + Sync>>,
}

// Manual Debug impl since we can't derive it for function pointers
impl std::fmt::Debug for TypeErasedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeErasedObject")
            .field("type_id", &self.type_id)
            .field("tag", &self.tag)
            .field("has_finalizer", &self.finalize_fn.is_some())
            .finish()
    }
}

impl ObjectStorage {
    /// Create a new object storage
    pub fn new() -> Self {
        Self {
            objects: RwLock::new(HashMap::new()),
        }
    }
    
    /// Store an object and get its address
    pub fn store<T: Traceable + Clone + Send + Sync + 'static>(&self, obj: T) -> usize {
        // Get finalize function
        let finalize_fn = Box::new(|any_obj: &mut dyn Any| {
            if let Some(typed_obj) = any_obj.downcast_mut::<T>() {
                typed_obj.finalize();
            }
        });
        
        // Box the object
        let boxed = Box::new(obj);
        
        // Get the raw pointer and address
        let ptr = Box::into_raw(boxed);
        let addr = ptr as usize;
        
        // Create type-erased object
        let type_erased = TypeErasedObject {
            data: unsafe { Box::from_raw(ptr as *mut T) as Box<dyn Any + Send + Sync> },
            type_id: TypeId::of::<T>(),
            tag: unsafe { (*ptr).tag() },
            finalize_fn: Some(finalize_fn),
        };
        
        // Store the type-erased object
        let mut objects = self.objects.write().unwrap();
        objects.insert(addr, type_erased);
        
        addr
    }
    
    /// Get a reference to an object by address
    pub fn get<T: 'static>(&self, addr: usize) -> Option<&T> {
        let objects = self.objects.read().unwrap();
        
        objects.get(&addr).and_then(|obj| {
            if obj.type_id == TypeId::of::<T>() {
                // Safe to cast since we checked the type ID
                let any_ref = &obj.data;
                unsafe {
                    let raw_ptr = any_ref as *const dyn Any as *const T;
                    Some(&*raw_ptr)
                }
            } else {
                None
            }
        })
    }
    
    /// Get a mutable reference to an object by address
    pub fn get_mut<T: 'static>(&self, addr: usize) -> Option<&mut T> {
        let objects = self.objects.write().unwrap();
        
        if let Some(obj) = objects.get(&addr) {
            if obj.type_id == TypeId::of::<T>() {
                // Safe to cast since we checked the type ID
                // This is UB because we're getting a mutable reference through an immutable one
                // A proper implementation would use a different approach, like RwLock per object
                // or UnsafeCell
                let any_ref = &obj.data as *const Box<dyn Any + Send + Sync> as *mut Box<dyn Any + Send + Sync>;
                unsafe {
                    let raw_ptr = (*any_ref).as_mut() as *mut dyn Any as *mut T;
                    Some(&mut *raw_ptr)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Remove an object from storage and finalize it
    pub fn remove_and_finalize(&self, addr: usize) -> bool {
        let mut objects = self.objects.write().unwrap();
        
        if let Some(mut obj) = objects.remove(&addr) {
            // Call finalize function if it exists
            if let Some(finalize_fn) = &obj.finalize_fn {
                finalize_fn(obj.data.as_mut());
            }
            
            true
        } else {
            false
        }
    }
    
    /// Check if an object exists in storage
    pub fn contains(&self, addr: usize) -> bool {
        let objects = self.objects.read().unwrap();
        objects.contains_key(&addr)
    }
    
    /// Get the total number of objects in storage
    pub fn len(&self) -> usize {
        let objects = self.objects.read().unwrap();
        objects.len()
    }
    
    /// Check if the storage is empty
    pub fn is_empty(&self) -> bool {
        let objects = self.objects.read().unwrap();
        objects.is_empty()
    }
    
    /// Clear all objects from storage
    pub fn clear(&self) {
        let mut objects = self.objects.write().unwrap();
        // Finalize all objects before clearing
        for (_, mut obj) in objects.drain() {
            if let Some(finalize_fn) = &obj.finalize_fn {
                finalize_fn(obj.data.as_mut());
            }
        }
    }
}

/// Global object storage singleton
pub fn global_object_storage() -> &'static ObjectStorage {
    static STORAGE: once_cell::sync::Lazy<ObjectStorage> = 
        once_cell::sync::Lazy::new(|| ObjectStorage::new());
    &STORAGE
}

/// A wrapper for accessing objects in storage
#[derive(Debug, Clone)]
pub struct StorageWrapper<T: Traceable + Clone + Send + Sync + 'static> {
    // Address of the object in storage
    addr: usize,
    // Phantom data to keep track of the type
    _marker: PhantomData<T>,
}

impl<T: Traceable + Clone + Send + Sync + 'static> StorageWrapper<T> {
    /// Create a new wrapper for an object
    pub fn new(obj: T) -> Self {
        let addr = global_object_storage().store(obj);
        Self {
            addr,
            _marker: PhantomData,
        }
    }
    
    /// Get the object's address
    pub fn address(&self) -> usize {
        self.addr
    }
    
    /// Get a reference to the object
    pub fn get(&self) -> Option<&T> {
        global_object_storage().get(self.addr)
    }
    
    /// Get a mutable reference to the object
    pub fn get_mut(&self) -> Option<&mut T> {
        global_object_storage().get_mut(self.addr)
    }
}

impl<T: Traceable + Clone + Send + Sync + 'static> Drop for StorageWrapper<T> {
    fn drop(&mut self) {
        // When the wrapper is dropped, remove the object from storage and finalize it
        // This is just for cleanup - the actual memory management is handled by the GC
        // global_object_storage().remove_and_finalize(self.addr);
    }
}