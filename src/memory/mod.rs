//! Memory management and garbage collection for the CURSED language
//!
//! This module implements the memory management system and garbage collector
//! for CURSED. It provides safe memory allocation, automatic reclamation of
//! unreachable objects, and utilities for handling complex data structures.
//!
//! ## Components
//!
//! * `gc`: Core garbage collection implementation using a mark-and-sweep algorithm
//! * `weak`: Weak references to avoid reference cycles
//! * `container`: Specialized container implementations for better performance
//! * `strategy`: Different memory management strategies
//! * `allocator`: Memory allocation utilities
//! * `channel`: Thread-safe communication channels

pub mod allocator;
pub mod block;
pub mod bump;
pub mod channel;
pub mod container;
pub mod deadlock_detector;
pub mod finalization_order;
pub mod gc;
// We've improved mark_sweep.rs directly instead of creating a separate module
pub mod mark_sweep;
pub mod memory_visitor;
pub mod object_storage;
pub mod root;
pub mod scope;
pub mod strategy;
pub mod tagged;
pub mod test_environment;
pub mod weak;

// Re-export important types
pub use container::{ContainerType, Specializable, SpecializedVector};
pub use finalization_order::{finalization_graph, register_dependency, finalize_objects_ordered};
pub use root::{RootScope, RootScopeGuard, ROOT_MANAGER};
pub use gc::MarkState;
pub use object_storage::{global_object_storage, ObjectStorage, StorageWrapper};
pub use scope::{with_gc_scope, with_new_gc, with_gc_scope_fn, with_new_gc_fn};
pub use test_environment::{get_test_gc, is_test_environment, reset_test_environment};
pub use weak::weak_registry;

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::{Arc, Weak as StdWeak};

use crate::memory::gc::GarbageCollector;

/// Type tag for different memory-managed object types
///
/// These tags are used by the garbage collector to identify object types
/// during marking and sweeping. Each tag corresponds to a specific
/// type of object that can be managed by the garbage collector.
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
///
/// Objects implementing this trait can participate in garbage collection.
/// The `trace` method allows the GC to traverse object references during
/// the mark phase, while `size` and `tag` provide metadata needed for
/// memory management operations.
pub trait Traceable: 'static {
    /// Trace all references in this object
    fn trace(&self, visitor: &mut dyn Visitor);

    /// Get the size of this object in bytes
    fn size(&self) -> usize;

    /// Get the type tag for this object
    fn tag(&self) -> Tag;
    
    /// Optional finalization method called before the object is collected
    /// This can be used for cleanup operations like closing files or freeing resources
    fn finalize(&mut self) {
        // Default implementation does nothing
    }
}

/// Visitor for traversing object graphs during garbage collection
///
/// This trait defines the interface for objects that traverse the object graph
/// during garbage collection's mark phase. Implementations of this trait
/// visit each reachable object, marking it and its references as live.
pub trait Visitor {
    /// Visit a traceable object
    fn visit(&mut self, ptr: NonNull<dyn Traceable>);

    /// Visit with context information (for debugging)
    fn visit_with_context(&mut self, ptr: NonNull<dyn Traceable>, context: &str);

    /// Visit a pointer by its raw address
    fn visit_ptr(&mut self, ptr: usize, tag: Tag);
}

/// Smart pointer for garbage-collected objects
///
/// `Gc<T>` is a reference-counted smart pointer that provides safe access
/// to heap-allocated objects managed by the garbage collector. It automatically
/// registers and unregisters objects as roots when created and destroyed.
/// 
/// This implementation uses a Weak reference to the GarbageCollector to avoid
/// circular references and potential deadlocks during cleanup.
#[derive(Debug)]
pub struct Gc<T: Traceable + Clone + Send + Sync + 'static> {
    ptr: NonNull<T>,
    gc: StdWeak<GarbageCollector>,
    _marker: PhantomData<T>,
}

impl<T: Traceable + Clone + Send + Sync + 'static> Gc<T> {
    /// Create a new Gc reference
    pub fn new(ptr: NonNull<T>, gc: Arc<GarbageCollector>) -> Self {
        println!("Gc::new called for {}", std::any::type_name::<T>());
        let addr = ptr.as_ptr() as usize;
        println!("Adding root at address 0x{:x}", addr);
        
        // Try to add to current root scope first
        if !ROOT_MANAGER.lock().unwrap().add_root(addr) {
            // If no active scope, add directly to the GC
            gc.add_root(addr);
        }
        println!("Root added successfully");

        Self {
            ptr,
            gc: Arc::downgrade(&gc), // Use weak reference to avoid circular reference
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
        // Try to upgrade the weak reference to get a temporary strong reference
        if let Some(gc) = self.gc.upgrade() {
            weak::Weak::new(self.ptr, gc)
        } else {
            // This should never happen in practice, but just in case 
            // create a default GarbageCollector
            let gc = Arc::new(GarbageCollector::new());
            weak::Weak::new(self.ptr, gc)
        }
    }
    
    /// Get the GarbageCollector for this object
    fn gc(&self) -> Option<Arc<GarbageCollector>> {
        self.gc.upgrade()
    }
}

impl<T: Traceable + Clone + Send + Sync + 'static> Clone for Gc<T> {
    fn clone(&self) -> Self {
        // Try to upgrade the weak reference to get a temporary strong reference
        if let Some(gc) = self.gc.upgrade() {
            // Register as a new root when cloned
            let addr = self.ptr.as_ptr() as usize;
            gc.add_root(addr);

            Self {
                ptr: self.ptr,
                gc: Arc::downgrade(&gc),
                _marker: PhantomData,
            }
        } else {
            // This should never happen in practice, but just in case
            // create a default GarbageCollector
            let gc = Arc::new(GarbageCollector::new());
            let addr = self.ptr.as_ptr() as usize;
            gc.add_root(addr);
            
            Self {
                ptr: self.ptr,
                gc: Arc::downgrade(&gc),
                _marker: PhantomData,
            }
        }
    }
}

impl<T: Traceable + Clone + Send + Sync + 'static> Drop for Gc<T> {
    fn drop(&mut self) {
        // Try to upgrade the weak reference to get a temporary strong reference
        if let Some(gc) = self.gc.upgrade() {
            // Remove from roots when dropped
            let addr = self.ptr.as_ptr() as usize;
            
            // Try to remove from current root scope first
            if !ROOT_MANAGER.lock().unwrap().remove_root(addr) {
                // If no active scope, remove directly from GC
                gc.remove_root(addr);
            }
            
            println!("Gc::drop - Removed root 0x{:x}", addr);
        } else {
            println!("Gc::drop - GC no longer available, skipping root removal");
        }
        // If upgrade fails, the GC is already gone, so no need to remove root
    }
}