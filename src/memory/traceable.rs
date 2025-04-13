//! Traceable trait for garbage collection
//!
//! This module defines the Traceable trait that objects must implement
//! to be managed by the garbage collector.

use std::ptr::NonNull;

/// Tag identifying the type of an object
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Integer,
    Float,
    Boolean,
    String,
    Array,
    Map,
    Function,
    Object,
    Null,
}

/// Visitor for traversing object graphs during garbage collection
pub trait Visitor {
    /// Visit a traceable object
    fn visit(&mut self, ptr: NonNull<dyn Traceable>);
    
    /// Visit a traceable object with context information
    fn visit_with_context(&mut self, ptr: NonNull<dyn Traceable>, context: &str);
    
    /// Visit an object by its raw address
    fn visit_ptr(&mut self, ptr: usize, tag: Tag);
}

/// Trait for objects that can be traced by the garbage collector
pub trait Traceable {
    /// Trace all references from this object
    fn trace(&self, visitor: &mut dyn Visitor);
    
    /// Get the size of this object in bytes
    fn size(&self) -> usize;
    
    /// Get the type tag for this object
    fn tag(&self) -> Tag;
    
    /// Called before the object is collected
    /// Objects can perform cleanup operations here
    fn finalize(&mut self) {
        // Default implementation does nothing
    }
}