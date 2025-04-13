//! Weak references implementation

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::{Arc, Weak as StdWeak};

use crate::memory::gc::GarbageCollector;
use crate::memory::{Gc, Traceable};

/// Weak reference to a garbage-collected object
#[derive(Debug)]
pub struct Weak<T: Traceable + Clone + 'static> {
    ptr: NonNull<T>,
    gc: StdWeak<GarbageCollector>,
    _marker: PhantomData<T>,
}

impl<T: Traceable + Clone + 'static> Weak<T> {
    /// Create a new weak reference from a pointer
    pub fn new(ptr: NonNull<T>, gc: Arc<GarbageCollector>) -> Self {
        Self {
            ptr,
            gc: Arc::downgrade(&gc),
            _marker: PhantomData,
        }
    }

    /// Check if the referenced object still exists
    pub fn is_alive(&self) -> bool {
        if let Some(gc) = self.gc.upgrade() {
            let addr = self.ptr.as_ptr() as usize;
            gc.is_alive(addr)
        } else {
            // If GC is gone, the object is not alive
            false
        }
    }

    /// Try to upgrade to a strong reference
    pub fn upgrade(&self) -> Option<Gc<T>> {
        if let Some(gc) = self.gc.upgrade() {
            if self.is_alive() {
                Some(Gc::new(self.ptr, gc))
            } else {
                None
            }
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