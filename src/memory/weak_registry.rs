//! Global registry for weak references
//!
//! This module provides a global registry for weak references to ensure they
//! can maintain connections to the garbage collector even after all strong
//! references are dropped. This is essential for proper cycle detection and cleanup.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak as StdWeak};
use once_cell::sync::Lazy;

use crate::memory::gc::GarbageCollector;

/// Global weak reference registry to maintain GC connections
/// This allows weak references to check liveness even after strong refs are dropped
#[derive(Debug, Default)]
pub struct GlobalWeakRegistry {
    // Map of object addresses to their GC instance
    references: HashMap<usize, StdWeak<GarbageCollector>>,
    // Reference counts to know when to remove from registry
    ref_counts: HashMap<usize, usize>,
}

impl GlobalWeakRegistry {
    /// Register a weak reference
    pub fn register(&mut self, addr: usize, gc: StdWeak<GarbageCollector>) {
        self.references.insert(addr, gc);
        *self.ref_counts.entry(addr).or_insert(0) += 1;
    }
    
    /// Unregister a weak reference
    pub fn unregister(&mut self, addr: usize) {
        if let Some(count) = self.ref_counts.get_mut(&addr) {
            *count -= 1;
            if *count == 0 {
                self.references.remove(&addr);
                self.ref_counts.remove(&addr);
            }
        }
    }
    
    /// Get the GC for a particular object address
    pub fn get_gc(&self, addr: usize) -> Option<Arc<GarbageCollector>> {
        self.references.get(&addr).and_then(|weak| weak.upgrade())
    }
    
    /// Check if an address is registered
    pub fn is_registered(&self, addr: usize) -> bool {
        self.references.contains_key(&addr)
    }
    
    /// Get the reference count for an address
    pub fn ref_count(&self, addr: usize) -> usize {
        self.ref_counts.get(&addr).copied().unwrap_or(0)
    }
}

/// Get the global weak reference registry
/// This is a static Mutex to allow global access while ensuring thread safety
static GLOBAL_REGISTRY: Lazy<Mutex<GlobalWeakRegistry>> = 
    Lazy::new(|| Mutex::new(GlobalWeakRegistry::default()));

/// Get a reference to the global registry
pub fn global_registry() -> &'static Mutex<GlobalWeakRegistry> {
    &GLOBAL_REGISTRY
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_global_registry() {
        let gc = Arc::new(GarbageCollector::new());
        let weak_gc = Arc::downgrade(&gc);
        
        let obj_addr = 0xDEADBEEF;
        
        // Register a reference
        {
            let mut registry = global_registry().lock().unwrap();
            registry.register(obj_addr, weak_gc.clone());
            assert_eq!(registry.ref_count(obj_addr), 1);
        }
        
        // Get the GC
        {
            let registry = global_registry().lock().unwrap();
            let retrieved_gc = registry.get_gc(obj_addr).unwrap();
            // We should be able to get the GC back
            assert!(Arc::ptr_eq(&gc, &retrieved_gc));
        }
        
        // Register again, count should increase
        {
            let mut registry = global_registry().lock().unwrap();
            registry.register(obj_addr, weak_gc);
            assert_eq!(registry.ref_count(obj_addr), 2);
        }
        
        // Unregister once, should still be registered
        {
            let mut registry = global_registry().lock().unwrap();
            registry.unregister(obj_addr);
            assert_eq!(registry.ref_count(obj_addr), 1);
            assert!(registry.is_registered(obj_addr));
        }
        
        // Unregister again, should be removed
        {
            let mut registry = global_registry().lock().unwrap();
            registry.unregister(obj_addr);
            assert_eq!(registry.ref_count(obj_addr), 0);
            assert!(!registry.is_registered(obj_addr));
        }
    }
}