//! Weak references implementation

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::memory::{Gc, Traceable};
use crate::memory::gc::GarbageCollector;

/// Weak reference to a garbage-collected object
#[derive(Debug)]
pub struct Weak<T: Traceable + Clone + 'static> {
    ptr: NonNull<T>,
    gc: Arc<GarbageCollector>,
    _marker: PhantomData<T>,
}

impl<T: Traceable + Clone + 'static> Weak<T> {
    /// Create a new weak reference from a pointer
    pub fn new(ptr: NonNull<T>, gc: Arc<GarbageCollector>) -> Self {
        Self {
            ptr,
            gc,
            _marker: PhantomData,
        }
    }
    
    /// Check if the referenced object still exists
    pub fn is_alive(&self) -> bool {
        let addr = self.ptr.as_ptr() as usize;
        self.gc.is_alive(addr)
    }
    
    /// Try to upgrade to a strong reference
    pub fn upgrade(&self) -> Option<Gc<T>> {
        if self.is_alive() {
            Some(Gc::new(self.ptr, self.gc.as_ref().clone()))
        } else {
            None
        }
    }
}

impl<T: Traceable + Clone + 'static> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            gc: self.gc.clone(),
            _marker: PhantomData,
        }
    }
}