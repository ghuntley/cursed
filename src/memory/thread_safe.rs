//! Thread-safe memory management utilities
//!
//! This module provides wrappers and implementations to ensure memory safety
//! when using objects across thread boundaries.

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

use crate::memory::{Tag, Traceable, Visitor};

/// Thread-safe wrapper for raw pointers
///
/// This type wraps a NonNull<T> and implements Send and Sync traits safely.
/// It's used to allow NonNull<T> to be safely shared across thread boundaries
/// even when T itself might not be Send or Sync.
#[derive(Debug)]
pub struct ThreadSafePointer<T: Traceable + 'static> {
    /// The raw pointer to the object
    ptr: NonNull<T>,
    /// Marker for variance
    _marker: PhantomData<T>,
}

// Custom implementation of Send and Sync that ensures thread safety
// We're explicitly allowing cross-thread use of this pointer because:
// 1. We never allow concurrent mutation of the pointed-to data
// 2. The GC handles synchronization of access to the data
// 3. This type is only used internally by the GC system
unsafe impl<T: Traceable + 'static> Send for ThreadSafePointer<T> {}
unsafe impl<T: Traceable + 'static> Sync for ThreadSafePointer<T> {}

impl<T: Traceable + 'static> ThreadSafePointer<T> {
    /// Create a new thread-safe pointer wrapping a NonNull<T>
    pub fn new(ptr: NonNull<T>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Get the raw pointer
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Get the address as usize
    pub fn address(&self) -> usize {
        self.ptr.as_ptr() as usize
    }

    /// Get a reference to the inner value
    ///
    /// # Safety
    /// This is unsafe because it dereferences a raw pointer.
    /// Caller must ensure the pointer is valid.
    pub unsafe fn inner(&self) -> &T {
        &*self.ptr.as_ptr()
    }

    /// Get a mutable reference to the inner value
    ///
    /// # Safety
    /// This is unsafe because it dereferences a raw pointer.
    /// Caller must ensure the pointer is valid and not concurrently accessed.
    pub unsafe fn inner_mut(&mut self) -> &mut T {
        &mut *self.ptr.as_ptr()
    }
}

impl<T: Traceable + 'static> Clone for ThreadSafePointer<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T: Traceable + 'static> Copy for ThreadSafePointer<T> {}

/// A thread-safe wrapper around a Traceable object
#[derive(Debug)]
pub struct ThreadSafeTraceable<T: Traceable + 'static> {
    /// The pointer to the object
    ptr: ThreadSafePointer<T>,
    /// Mutex for synchronized access
    lock: Arc<Mutex<()>>,
}

// Can be safely sent between threads
unsafe impl<T: Traceable + 'static> Send for ThreadSafeTraceable<T> {}
// Can be safely shared between threads
unsafe impl<T: Traceable + 'static> Sync for ThreadSafeTraceable<T> {}

impl<T: Traceable + 'static> ThreadSafeTraceable<T> {
    /// Create a new thread-safe traceable object
    pub fn new(ptr: NonNull<T>) -> Self {
        Self {
            ptr: ThreadSafePointer::new(ptr),
            lock: Arc::new(Mutex::new(())),
        }
    }

    /// Trace the object's references safely
    pub fn trace(&self, visitor: &mut dyn Visitor) {
        // Acquire lock to ensure exclusive access during tracing
        let _guard = self.lock.lock().unwrap();
        unsafe {
            self.ptr.inner().trace(visitor);
        }
    }

    /// Get the size of the object
    pub fn size(&self) -> usize {
        unsafe { self.ptr.inner().size() }
    }

    /// Get the tag of the object
    pub fn tag(&self) -> Tag {
        unsafe { self.ptr.inner().tag() }
    }

    /// Get the address of the object
    pub fn address(&self) -> usize {
        self.ptr.address()
    }
}

impl<T: Traceable + 'static> Clone for ThreadSafeTraceable<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            lock: Arc::clone(&self.lock),
        }
    }
}

// Implementation of Traceable for ThreadSafeTraceable
impl<T: Traceable + 'static> Traceable for ThreadSafeTraceable<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        self.trace(visitor); // Delegate to our method
    }

    fn size(&self) -> usize {
        self.size() // Delegate to our method
    }

    fn tag(&self) -> Tag {
        self.tag() // Delegate to our method
    }

    fn finalize(&mut self) {
        // Acquire lock for exclusive access during finalization
        let _guard = self.lock.lock().unwrap();
        unsafe {
            self.ptr.inner_mut().finalize();
        }
    }
}