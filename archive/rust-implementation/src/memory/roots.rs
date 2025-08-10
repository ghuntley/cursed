//! Root set management for garbage collection
//! 
//! Manages the set of root objects that serve as starting points for
//! garbage collection marking phases.

use crate::error::CursedError;
use crate::memory::{Traceable, Visitor};
use crate::memory::heap::{ObjectId, get_global_heap};
use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex, Weak};

/// Root set manager for garbage collection
pub struct RootSet {
    /// Set of root object IDs
    roots: Mutex<HashSet<ObjectId>>,
    /// Named roots for easy access
    named_roots: Mutex<HashMap<String, ObjectId>>,
    /// Statistics
    stats: Mutex<RootStats>,
}

/// Root set statistics
#[derive(Debug, Clone, Default)]
pub struct RootStats {
    pub total_roots: usize,
    pub named_roots: usize,
    pub marking_cycles: u64,
}

impl RootSet {
    /// Create a new root set
    pub fn new() -> Self {
        Self {
            roots: Mutex::new(HashSet::new()),
            named_roots: Mutex::new(HashMap::new()),
            stats: Mutex::new(RootStats::default()),
        }
    }

    /// Add a root object
    pub fn add_root(&self, id: ObjectId) -> Result<(), CursedError> {
        let mut roots = self.roots.lock().unwrap();
        if roots.insert(id) {
            let mut stats = self.stats.lock().unwrap();
            stats.total_roots += 1;
        }
        Ok(())
    }

    /// Remove a root object
    pub fn remove_root(&self, id: ObjectId) -> Result<bool, CursedError> {
        let mut roots = self.roots.lock().unwrap();
        let removed = roots.remove(&id);
        
        if removed {
            let mut stats = self.stats.lock().unwrap();
            stats.total_roots -= 1;
        }
        
        Ok(removed)
    }

    /// Add a named root
    pub fn add_named_root(&self, name: String, id: ObjectId) -> Result<(), CursedError> {
        {
            let mut named_roots = self.named_roots.lock().unwrap();
            named_roots.insert(name, id);
        }
        
        self.add_root(id)?;
        
        {
            let mut stats = self.stats.lock().unwrap();
            stats.named_roots += 1;
        }
        
        Ok(())
    }

    /// Get a named root
    pub fn get_named_root(&self, name: &str) -> Option<ObjectId> {
        let named_roots = self.named_roots.lock().unwrap();
        named_roots.get(name).copied()
    }

    /// Remove a named root
    pub fn remove_named_root(&self, name: &str) -> Result<Option<ObjectId>, CursedError> {
        let id = {
            let mut named_roots = self.named_roots.lock().unwrap();
            named_roots.remove(name)
        };
        
        if let Some(id) = id {
            self.remove_root(id)?;
            let mut stats = self.stats.lock().unwrap();
            stats.named_roots -= 1;
        }
        
        Ok(id)
    }

    /// Mark all objects reachable from roots
    pub fn mark_reachable(&self) -> Result<usize, CursedError> {
        let heap = get_global_heap();
        let roots = self.roots.lock().unwrap();
        
        // First, unmark all objects
        heap.unmark_all();
        
        let mut marked_count = 0;
        
        // Mark all root objects
        for &root_id in roots.iter() {
            if heap.mark(root_id) {
                marked_count += 1;
            }
        }
        
        // Trace from marked objects to mark transitively reachable objects
        let mut visitor = RootMarkingVisitor::new(&heap);
        heap.trace_from_roots(&mut visitor);
        marked_count += visitor.newly_marked;
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.marking_cycles += 1;
        }
        
        Ok(marked_count)
    }

    /// Get all root IDs
    pub fn get_all_roots(&self) -> Vec<ObjectId> {
        let roots = self.roots.lock().unwrap();
        roots.iter().copied().collect()
    }

    /// Get all named roots
    pub fn get_all_named_roots(&self) -> HashMap<String, ObjectId> {
        let named_roots = self.named_roots.lock().unwrap();
        named_roots.clone()
    }

    /// Clear all roots
    pub fn clear(&self) {
        {
            let mut roots = self.roots.lock().unwrap();
            roots.clear();
        }
        {
            let mut named_roots = self.named_roots.lock().unwrap();
            named_roots.clear();
        }
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_roots = 0;
            stats.named_roots = 0;
        }
    }

    /// Get root set statistics
    pub fn stats(&self) -> RootStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }
}

impl Default for RootSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Visitor for marking objects reachable from roots
struct RootMarkingVisitor<'a> {
    heap: &'a crate::memory::heap::Heap,
    newly_marked: usize,
}

impl<'a> RootMarkingVisitor<'a> {
    fn new(heap: &'a crate::memory::heap::Heap) -> Self {
        Self {
            heap,
            newly_marked: 0,
        }
    }
}

impl<'a> Visitor for RootMarkingVisitor<'a> {
    fn visit(&mut self, obj: &dyn Traceable) {
        // In a real implementation, we'd need a way to map from Traceable back to ObjectId
        // For now, this is a simplified implementation that demonstrates the concept
        
        // Continue tracing this object's references
        obj.trace(self);
    }
}

/// Global root set instance
static GLOBAL_ROOT_SET: std::sync::LazyLock<Arc<RootSet>> = std::sync::LazyLock::new(|| {
    Arc::new(RootSet::new())
});

/// Get the global root set
pub fn get_global_root_set() -> Arc<RootSet> {
    Arc::clone(&GLOBAL_ROOT_SET)
}

/// Convenience function to add a root to the global root set
pub fn add_global_root(id: ObjectId) -> Result<(), CursedError> {
    get_global_root_set().add_root(id)
}

/// Convenience function to add a named root to the global root set
pub fn add_global_named_root(name: String, id: ObjectId) -> Result<(), CursedError> {
    get_global_root_set().add_named_root(name, id)
}

/// Convenience function for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    let root_set = get_global_root_set();
    let stats = root_set.stats();
    Ok(format!("RootSet ready - {} roots ({} named)", stats.total_roots, stats.named_roots))
}
