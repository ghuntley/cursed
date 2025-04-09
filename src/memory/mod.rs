//! Memory management for the CURSED language

pub mod gc;
pub mod weak;
pub mod strategy;
pub mod tagged;

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
pub trait Traceable {
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
    fn visit_ptr(&mut self, ptr: usize, tag: Tag) {
        // Default implementation does nothing
    }
}

/// Garbage-collected reference to an object
#[derive(Debug)]
pub struct Gc<T: Traceable + Clone + 'static> {
    ptr: Option<NonNull<T>>,
    gc: Arc<GarbageCollector>,
    _marker: PhantomData<T>,
}

impl<T: Traceable + Clone + 'static> Gc<T> {
    /// Create a new empty Gc
    pub fn new_empty(gc: &GarbageCollector) -> Self {
        Self {
            ptr: None,
            gc: Arc::new(gc.clone()),
            _marker: PhantomData,
        }
    }
    
    /// Get a reference to the inner value
    pub fn inner(&self) -> Option<&T> {
        None
    }
    
    /// Check if the reference is null
    pub fn is_null(&self) -> bool {
        self.ptr.is_none()
    }
    
    /// Create a weak reference to this object
    pub fn downgrade(&self) -> weak::Weak<T> {
        match self.ptr {
            Some(ptr) => weak::Weak::new(ptr, self.gc.clone()),
            None => weak::Weak::new(NonNull::dangling(), self.gc.clone()),
        }
    }
}

impl<T: Traceable + Clone + 'static> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            gc: self.gc.clone(),
            _marker: PhantomData,
        }
    }
}