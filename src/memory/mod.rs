//! Memory management for the CURSED language

pub mod gc;
pub mod weak;
pub mod strategy;
pub mod tagged;
pub mod bump;
pub mod block;
pub mod allocator;
pub mod memory_visitor;
pub mod channel;

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::memory::gc::GarbageCollector;

/// Tag for different types of objects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Int,
    Float,
    Boolean,
    String,
    Array,
    Map,
    Function,
    Object,
    Null,
}

/// Trait for objects that can be traced by the garbage collector
pub trait Traceable: 'static {
    /// Trace all references in this object
    fn trace(&self, visitor: &mut dyn Visitor);
    
    /// Get the size of this object in bytes
    fn size(&self) -> usize;
    
    /// Get the type tag for this object
    fn tag(&self) -> Tag;
}

/// Visitor for traversing object graphs during garbage collection
pub trait Visitor {
    /// Visit a traceable object
    fn visit(&mut self, ptr: NonNull<dyn Traceable>);
    
    /// Visit with context information (for debugging)
    fn visit_with_context(&mut self, ptr: NonNull<dyn Traceable>, context: &str);
    
    /// Visit a pointer by its raw address
    fn visit_ptr(&mut self, ptr: usize, tag: Tag);
}

/// Garbage-collected reference to an object
#[derive(Debug)]
pub struct Gc<T: Traceable + Clone + 'static> {
    ptr: NonNull<T>,
    gc: Arc<GarbageCollector>,
    _marker: PhantomData<T>,
}

impl<T: Traceable + Clone + 'static> Gc<T> {
    /// Create a new Gc reference
    pub fn new(ptr: NonNull<T>, gc: GarbageCollector) -> Self {
        let addr = ptr.as_ptr() as usize;
        gc.add_root(addr);
        
        Self {
            ptr,
            gc: Arc::new(gc),
            _marker: PhantomData,
        }
    }
    
    /// Get a reference to the inner value
    pub fn inner(&self) -> Option<&T> {
        unsafe { Some(&*self.ptr.as_ptr()) }
    }
    
    /// Get a mutable reference to the inner value
    pub fn inner_mut(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut *self.ptr.as_ptr()) }
    }
    
    /// Get the raw pointer
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
    
    /// Create a weak reference to this object
    pub fn downgrade(&self) -> weak::Weak<T> {
        weak::Weak::new(self.ptr, self.gc.clone())
    }
}

impl<T: Traceable + Clone + 'static> Clone for Gc<T> {
    fn clone(&self) -> Self {
        // Register as a new root when cloned
        let addr = self.ptr.as_ptr() as usize;
        self.gc.add_root(addr);
        
        Self {
            ptr: self.ptr,
            gc: self.gc.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: Traceable + Clone + 'static> Drop for Gc<T> {
    fn drop(&mut self) {
        // Remove from roots when dropped
        let addr = self.ptr.as_ptr() as usize;
        self.gc.remove_root(addr);
    }
}