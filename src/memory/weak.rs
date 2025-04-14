//! Regular weak reference implementation for non-thread-safe objects

use std::marker::PhantomData;
use std::sync::{Arc, Weak as StdWeak};
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use crate::memory::gc::GarbageCollector;
use crate::memory::{Gc, Traceable};

/// A weak reference to a GC-managed object
pub struct Weak<T: Traceable + 'static> {
    /// A weak reference to the garbage collector
    gc: StdWeak<GarbageCollector>,
    /// The object ID
    id: usize,
    /// Phantom data to bind the type parameter
    _phantom: PhantomData<T>,
}

impl<T: Traceable + 'static> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Self {
            gc: self.gc.clone(),
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T: Traceable + 'static> Weak<T> {
    /// Create a new weak reference
    pub(crate) fn new(gc: &Arc<GarbageCollector>, id: usize) -> Self {
        Self {
            gc: Arc::downgrade(gc),
            id,
            _phantom: PhantomData,
        }
    }
    
    /// Attempt to upgrade to a strong reference
    pub fn upgrade(&self) -> Option<Gc<T>> {
        // First try to upgrade the GC reference
        let gc = self.gc.upgrade()?;
        
        // Check if the object still exists
        if gc.get_object::<T>(self.id).is_none() {
            return None;
        }
        
        // Create a new strong reference
        Some(Gc::new(gc, self.id))
    }
    
    /// Check if the object is still alive
    pub fn is_alive(&self) -> bool {
        if let Some(gc) = self.gc.upgrade() {
            gc.get_object::<T>(self.id).is_some()
        } else {
            false
        }
    }
    
    /// Get the object ID
    pub fn id(&self) -> usize {
        self.id
    }
}

/// A registry for tracking weak references
pub struct WeakRegistry {
    /// Map of object IDs to reference counts
    entries: RwLock<HashMap<usize, usize>>,
    /// Map of object IDs to weak references to garbage collector
    gc_refs: RwLock<HashMap<usize, StdWeak<GarbageCollector>>>,
}

impl Default for WeakRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl WeakRegistry {
    /// Create a new weak reference registry
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            gc_refs: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register an object in the registry
    pub fn register(&self, id: usize, gc: StdWeak<GarbageCollector>) {
        let mut entries = self.entries.write().unwrap();
        let mut gc_refs = self.gc_refs.write().unwrap();
        
        // Increment reference count
        *entries.entry(id).or_insert(0) += 1;
        
        // Store the GC reference if not already present
        if !gc_refs.contains_key(&id) {
            gc_refs.insert(id, gc);
        }
    }
    
    /// Unregister an object from the registry
    pub fn unregister(&self, id: usize) {
        let mut entries = self.entries.write().unwrap();
        
        if let Some(count) = entries.get_mut(&id) {
            *count -= 1;
            if *count == 0 {
                entries.remove(&id);
                
                // Also remove the GC reference
                let mut gc_refs = self.gc_refs.write().unwrap();
                gc_refs.remove(&id);
            }
        }
    }
    
    /// Check if an object is registered
    pub fn is_registered(&self, id: usize) -> bool {
        let entries = self.entries.read().unwrap();
        entries.contains_key(&id)
    }
    
    /// Get the reference count for an object
    pub fn ref_count(&self, id: usize) -> usize {
        let entries = self.entries.read().unwrap();
        entries.get(&id).copied().unwrap_or(0)
    }
    
    /// Get the garbage collector for an object
    pub fn get_gc(&self, id: usize) -> Option<Arc<GarbageCollector>> {
        let gc_refs = self.gc_refs.read().unwrap();
        gc_refs.get(&id).and_then(|gc| gc.upgrade())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{Tag, Visitor};
    
    #[derive(Debug)]
    struct TestObject {
        value: i32,
    }
    
    impl Traceable for TestObject {
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
    
    #[test]
    fn test_weak_registry() {
        let registry = WeakRegistry::new();
        
        // Create a garbage collector
        let gc = Arc::new(GarbageCollector::new());
        let gc_weak = Arc::downgrade(&gc);
        
        // Register an object
        registry.register(123, gc_weak.clone());
        
        // Check registration
        assert!(registry.is_registered(123));
        assert_eq!(registry.ref_count(123), 1);
        
        // Register again
        registry.register(123, gc_weak.clone());
        assert_eq!(registry.ref_count(123), 2);
        
        // Unregister
        registry.unregister(123);
        assert_eq!(registry.ref_count(123), 1);
        
        // Get the GC
        let retrieved_gc = registry.get_gc(123);
        assert!(retrieved_gc.is_some());
        
        // Unregister the last reference
        registry.unregister(123);
        assert_eq!(registry.ref_count(123), 0);
        assert!(!registry.is_registered(123));
        
        // GC should no longer be retrievable
        let retrieved_gc = registry.get_gc(123);
        assert!(retrieved_gc.is_none());
    }
    
    #[test]
    fn test_weak_reference() {
        // Create a garbage collector
        let gc = Arc::new(GarbageCollector::new());
        
        // Allocate an object
        let obj = gc.allocate(TestObject { value: 42 });
        
        // Create a weak reference
        let weak = obj.downgrade();
        
        // Check properties
        assert_eq!(weak.id(), obj.id());
        assert!(weak.is_alive());
        
        // Upgrade the weak reference
        let upgraded = weak.upgrade().unwrap();
        assert_eq!(upgraded.inner().unwrap().value, 42);
        
        // Drop the original strong reference
        drop(obj);
        
        // The upgraded reference should still keep the object alive
        assert!(weak.is_alive());
        
        // Drop the upgraded reference
        drop(upgraded);
        
        // Force garbage collection
        gc.collect_garbage();
        
        // The weak reference should now be dead
        assert!(!weak.is_alive());
        assert!(weak.upgrade().is_none());
    }
}