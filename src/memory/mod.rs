//! Memory management module for the CURSED language
//!
//! This module provides garbage collection and memory management for CURSED.
//! It includes both regular (non-thread-safe) and thread-safe garbage collection.

/// Enable debug logging for memory operations
pub const DEBUG_MEMORY: bool = true;

/// Print debug info if debugging is enabled
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if $crate::memory::DEBUG_MEMORY {
            println!("[GC_DEBUG] {}", format!($($arg)*));
        }
    };
}

use std::ptr::NonNull;

// Public modules
pub mod gc;
pub mod thread_safe_gc;
pub mod thread_safe_weak;
pub mod weak;
pub mod weak_registry;
pub mod object_storage;
pub mod finalization_order;
pub mod test_environment;
pub mod deadlock_detector;
pub mod mark_sweep;

// Re-exports
pub use object_storage::{ObjectStorage, StorageWrapper, global_object_storage, register_dependency, store, contains};
pub use finalization_order::{finalize_objects_ordered, calculate_finalization_order};

// Add a convenience function for working with GC scopes (for tests)
pub fn with_gc_scope<R>(gc: R) -> R {
    // Initialize the test environment
    test_environment::reset_test_environment();
    // Return the GC
    gc
}

// Re-export types
pub use gc::GarbageCollector;
pub use thread_safe_gc::ThreadSafeGc;
pub use thread_safe_weak::ThreadSafeWeak;
pub use weak::Weak;
pub use weak_registry::{GlobalWeakRegistry, global_registry};

// For testing (re-exported)
pub use test_environment::{reset_test_environment, get_test_gc};

/// Object tags for the garbage collector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    /// Generic object
    Object,
    /// Function object
    Function,
    /// Array object
    Array,
    /// Hash/map object
    Hash,
    /// String object
    String,
    /// Environment object
    Environment,
    /// Integer object
    Int,
    /// Float object
    Float,
    /// Boolean object
    Boolean,
    /// Map object
    Map,
    /// Null object
    Null,
}

/// Trait for objects that can be traced by the garbage collector
pub trait Traceable {
    /// Trace all references contained in this object
    fn trace(&self, visitor: &mut dyn Visitor);
    
    /// Get the memory size of this object
    fn size(&self) -> usize;
    
    /// Get the object tag
    fn tag(&self) -> Tag;
    
    /// Called when the object is being finalized during garbage collection
    /// Default implementation does nothing
    fn finalize(&mut self) {}
}

/// Visitor for traversing object graphs
pub trait Visitor {
    /// Visit a traceable object
    fn visit(&mut self, ptr: NonNull<dyn Traceable>);
    
    /// Visit a traceable object with context information
    fn visit_with_context(&mut self, ptr: NonNull<dyn Traceable>, context: &str) {
        self.visit(ptr);
    }
    
    /// Visit an object by its memory address
    fn visit_ptr(&mut self, addr: usize, tag: Tag) {
        // Default implementation does nothing
    }
}

/// Extended visitor that supports thread-safe objects
/// This is separated from the base Visitor to maintain object safety
pub trait ThreadSafeVisitor: Visitor {
    /// Visit a thread-safe GC object
    fn visit_thread_safe<T: Traceable + Send + Sync + 'static>(&mut self, ptr: &ThreadSafeGc<T>);
}

/// A thread-safe version of Traceable that can be shared across threads
#[derive(Debug)]
pub struct ThreadSafeTraceable<T: Traceable>(NonNull<T>);

impl<T: Traceable> ThreadSafeTraceable<T> {
    /// Create a new thread-safe traceable object
    pub fn new(ptr: NonNull<T>) -> Self {
        Self(ptr)
    }
    
    /// Get the inner object
    pub fn inner(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
    
    /// Get a mutable reference to the inner object
    pub unsafe fn inner_mut(&mut self) -> &mut T {
        self.0.as_mut()
    }
}

impl<T: Traceable> Traceable for ThreadSafeTraceable<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        unsafe { self.0.as_ref().trace(visitor) }
    }
    
    fn size(&self) -> usize {
        unsafe { self.0.as_ref().size() }
    }
    
    fn tag(&self) -> Tag {
        unsafe { self.0.as_ref().tag() }
    }
}

// Thread-safe traceable is Send + Sync if T is Traceable
unsafe impl<T: Traceable + Send> Send for ThreadSafeTraceable<T> {}
unsafe impl<T: Traceable + Sync> Sync for ThreadSafeTraceable<T> {}

/// A garbage collected pointer (non-thread-safe version)
#[derive(Debug)]
pub struct Gc<T: Traceable + 'static> {
    /// The garbage collector that owns this object
    gc: Arc<GarbageCollector>,
    /// The object ID
    id: usize,
    /// Phantom data to bind the type parameter
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Traceable + 'static> Drop for Gc<T> {
    fn drop(&mut self) {
        // When the last reference is dropped, remove from the root set
        self.gc.remove_root(self.id);
    }
}

impl<T: Traceable + 'static> Clone for Gc<T> {
    fn clone(&self) -> Self {
        // Add another root reference
        self.gc.add_root(self.id);
        
        Self {
            gc: self.gc.clone(),
            id: self.id,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Traceable + 'static> Gc<T> {
    /// Create a new garbage collected pointer
    pub(crate) fn new(gc: Arc<GarbageCollector>, id: usize) -> Self {
        // Add to the root set
        gc.add_root(id);
        
        Self {
            gc,
            id,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Get a reference to the inner object
    pub fn inner(&self) -> Option<&T> {
        // Access the object directly from the pointer
        // In a real implementation, we would check with the GC first
        // NOTE: This is actually unsafe and could lead to dangling references
        // In a real implementation, we would use a better approach
        // For testing only, we return None to avoid reference errors
        None
    }
    
    /// Get a mutable reference to the inner object
    pub fn inner_mut(&self) -> Option<&mut T> {
        // This is unsafe in a real implementation, as it would break Rust's borrowing rules
        // For testing purposes, we just return None
        None
    }
    
    /// Get the raw pointer value (for testing only)
    pub fn as_ptr(&self) -> *const T {
        std::ptr::null()
    }
    
    /// Get the object ID
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// Convert this strong reference to a weak reference
    pub fn downgrade(&self) -> Weak<T> where T: Clone + Send + Sync {
        Weak::new(NonNull::<T>::dangling(), self.gc.clone())
    }
}

/// Extension trait for Traceable to get as Any
trait TraceableAsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: Traceable + 'static> TraceableAsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Import std::sync::Arc for GC
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};
    
    #[derive(Debug, Clone)]
    struct TestObject {
        value: i32,
        next: Arc<Mutex<Option<Gc<TestObject>>>>,
    }
    
    impl TestObject {
        fn new(value: i32) -> Self {
            Self {
                value,
                next: Arc::new(Mutex::new(None)),
            }
        }
        
        fn set_next(&self, next: Gc<TestObject>) {
            if let Ok(mut guard) = self.next.lock() {
                *guard = Some(next);
            }
        }
    }
    
    impl Traceable for TestObject {
        fn trace(&self, visitor: &mut dyn Visitor) {
            if let Ok(guard) = self.next.lock() {
                if let Some(next) = &*guard {
                    unsafe {
                        // This would typically point to an actual object instance
                        // but for testing purposes we're skipping this part
                        // visitor.visit(NonNull::new_unchecked(...));
                    }
                }
            }
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
    }
    
    #[test]
    fn test_gc_basic() {
        let gc = GarbageCollector::new();
        
        // Allocate an object
        let obj = gc.allocate(TestObject::new(42));
        
        // Check we can access it
        assert_eq!(obj.inner().unwrap().value, 42);
        
        // Run GC - object should still be accessible
        gc.collect_garbage();
        assert_eq!(obj.inner().unwrap().value, 42);
        
        // Create a weak reference
        let weak = obj.downgrade();
        
        // Drop the strong reference
        drop(obj);
        
        // Run GC - object should be collected
        gc.collect_garbage();
        
        // Weak reference should not be upgradeable
        assert!(weak.upgrade().is_none());
    }
    
    #[test]
    fn test_thread_safe_gc_basic() {
        // Create a thread-safe object
        #[derive(Debug, Clone)]
        struct ThreadSafeTestObject {
            value: Arc<Mutex<i32>>,
        }
        
        impl ThreadSafeTestObject {
            fn new(value: i32) -> Self {
                Self {
                    value: Arc::new(Mutex::new(value)),
                }
            }
            
            fn get_value(&self) -> i32 {
                *self.value.lock().unwrap()
            }
            
            fn set_value(&self, value: i32) {
                *self.value.lock().unwrap() = value;
            }
        }
        
        impl Traceable for ThreadSafeTestObject {
            fn trace(&self, _visitor: &mut dyn Visitor) {
                // No references to trace
            }
            
            fn size(&self) -> usize {
                std::mem::size_of::<Self>()
            }
            
            fn tag(&self) -> Tag {
                Tag::Object
            }
        }
        
        // Must be Send + Sync for ThreadSafeGc
        unsafe impl Send for ThreadSafeTestObject {}
        unsafe impl Sync for ThreadSafeTestObject {}
        
        // Create a GC
        let gc = GarbageCollector::new();
        
        // Allocate a thread-safe object
        let obj = gc.allocate_thread_safe(ThreadSafeTestObject::new(42));
        
        // Check we can access it
        assert_eq!(obj.inner().unwrap().get_value(), 42);
        
        // Run GC - object should still be accessible
        gc.collect_garbage();
        assert_eq!(obj.inner().unwrap().get_value(), 42);
        
        // Create a weak reference
        let weak = obj.downgrade();
        
        // Upgrade the weak reference
        let upgraded = weak.upgrade().unwrap();
        assert_eq!(upgraded.inner().unwrap().get_value(), 42);
        
        // Modify through the upgraded reference
        upgraded.inner().unwrap().set_value(100);
        
        // Original sees the change
        assert_eq!(obj.inner().unwrap().get_value(), 100);
        
        // Drop both strong references
        drop(obj);
        drop(upgraded);
        
        // Run GC - object should be collected
        gc.collect_garbage();
        
        // Weak reference should not be upgradeable
        assert!(weak.upgrade().is_none());
    }
}