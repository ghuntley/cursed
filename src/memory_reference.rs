// CURSED Memory Management Reference
// This file contains a reference implementation for memory management in the CURSED language.

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use crate::object::Object;

/// Memory management for the CURSED language
pub struct Memory {
    /// All allocated objects
    objects: Vec<Arc<RwLock<Object>>>,
    /// Objects that are visible to the garbage collector
    roots: Vec<Arc<RwLock<Object>>>,
    /// Object allocation count for statistics
    allocation_count: usize,
    /// Next scheduled garbage collection threshold
    next_gc: usize,
    /// Configuration: GC after this many allocations
    gc_threshold: usize,

}

impl Memory {
    /// Create a new memory manager
    #[tracing::instrument(level = "debug")]
    pub fn new() -> Self {
        tracing::debug!("Creating new Memory manager");
        let gc_threshold = 1000; // Default threshold
        Self {
            objects: Vec::new(),
            roots: Vec::new(),
            allocation_count: 0,
            next_gc: gc_threshold,
            gc_threshold,

        }
    }
    
    /// Create a new memory manager with custom garbage collection threshold
    #[tracing::instrument(level = "debug")]
    pub fn with_gc_threshold(threshold: usize) -> Self {
        tracing::debug!(threshold = threshold, "Creating Memory manager with custom GC threshold");
        let mut mem = Self::new();
        mem.gc_threshold = threshold;
        mem.next_gc = threshold;
        mem
    }
    

    
    /// Allocate a new object in memory
    pub fn allocate(&mut self, obj: Object) -> Arc<RwLock<Object>> {
        let arc_obj = Arc::new(RwLock::new(obj));
        self.objects.push(Arc::clone(&arc_obj));
        
        self.allocation_count += 1;
        if self.allocation_count >= self.next_gc {
            self.collect_garbage();
        }
        
        arc_obj
    }
    
    /// Add an object to the root set (protected from GC)
    pub fn add_root(&mut self, obj: Arc<RwLock<Object>>) {
        if !self.roots.iter().any(|root| Arc::ptr_eq(root, &obj)) {
            self.roots.push(Arc::clone(&obj));
        }
    }
    
    /// Remove an object from the root set
    pub fn remove_root(&mut self, obj: &Arc<RwLock<Object>>) {
        self.roots.retain(|root| !Arc::ptr_eq(root, obj));
    }
    
    /// Mark and sweep garbage collection
    #[tracing::instrument(skip(self), fields(object_count = self.objects.len(), roots_count = self.roots.len()), level = "info")]
    pub fn collect_garbage(&mut self) {
        tracing::info!("Starting garbage collection");

        
        // Mark phase: mark all reachable objects
        let mut marked = HashMap::new();
        self.mark_roots(&mut marked);
        
        // Sweep phase: remove unmarked objects
        self.objects.retain(|obj| {
            let is_marked = marked.contains_key(&(Arc::as_ptr(obj) as usize));
            is_marked
        });
        
        // Update next GC threshold
        self.allocation_count = 0;
        self.next_gc = self.objects.len() + self.gc_threshold;
        
        let remaining_objects = self.objects.len();
        tracing::info!(remaining_objects = remaining_objects, next_gc_at = self.next_gc, "Garbage collection completed");
        

    }
    
    /// Mark all objects reachable from roots
    fn mark_roots(&self, marked: &mut HashMap<usize, bool>) {
        for root in &self.roots {
            self.mark_object(root, marked);
        }
    }
    
    /// Mark an object and its children
    fn mark_object(&self, obj: &Arc<RwLock<Object>>, marked: &mut HashMap<usize, bool>) {
        let ptr = Arc::as_ptr(obj) as usize;
        
        // If already marked, return to avoid cycles
        if marked.contains_key(&ptr) {
            return;
        }
        
        // Mark this object
        marked.insert(ptr, true);
        
        // Mark children based on object type
        match &*obj.read().unwrap() {
            Object::Array(elements) => {
                for elem in elements {
                    if let Some(arc_elem) = elem.as_arc_rwlock() {
                        self.mark_object(arc_elem, marked);
                    }
                }
            },
            Object::Hash(pairs) => {
                for (key, value) in pairs {
                    if let Some(arc_key) = key.as_arc_rwlock() {
                        self.mark_object(arc_key, marked);
                    }
                    if let Some(arc_value) = value.as_arc_rwlock() {
                        self.mark_object(arc_value, marked);
                    }
                }
            },
            Object::Function { body, env, .. } => {
                if let Some(arc_env) = env.as_arc_rwlock() {
                    self.mark_object(arc_env, marked);
                }
            },
            Object::Closure { .. } => {
                // Mark captured variables in the closure
                if let Object::Closure { function, free } = &*obj.read().unwrap() {
                    if let Some(arc_fn) = function.as_arc_rwlock() {
                        self.mark_object(arc_fn, marked);
                    }
                    for free_var in free {
                        if let Some(arc_var) = free_var.as_arc_rwlock() {
                            self.mark_object(arc_var, marked);
                        }
                    }
                }
            },
            Object::Environment(env) => {
                // Mark all values in the environment
                for (_, value) in env.store.read().unwrap().iter() {
                    if let Some(arc_value) = value.as_arc_rwlock() {
                        self.mark_object(arc_value, marked);
                    }
                }
                // Mark outer environment if it exists
                if let Some(outer) = &env.outer {
                    if let Some(arc_outer) = outer.as_arc_rwlock() {
                        self.mark_object(arc_outer, marked);
                    }
                }
            },
            _ => {
                // Other object types don't have references to track
            }
        }
    }
    
    /// Get statistics about memory usage
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            object_count: self.objects.len(),
            root_count: self.roots.len(),
            allocation_count: self.allocation_count,
            next_gc: self.next_gc,
        }
    }
    
    /// Force garbage collection regardless of threshold
    pub fn force_gc(&mut self) {
        self.collect_garbage();
    }
}

/// Memory usage statistics
pub struct MemoryStats {
    /// Number of objects in memory
    pub object_count: usize,
    /// Number of root objects
    pub root_count: usize,
    /// Allocations since last GC
    pub allocation_count: usize,
    /// Next GC threshold
    pub next_gc: usize,
}

/// Extension trait for Object to support memory management
pub trait AsArcRwLock {
    fn as_arc_rwlock(&self) -> Option<&Arc<RwLock<Object>>>;
}

impl AsArcRwLock for Object {
    fn as_arc_rwlock(&self) -> Option<&Arc<RwLock<Object>>> {
        match self {
            Object::Reference(arc) => Some(arc),
            _ => None,
        }
    }
}

// For the reference implementation, we add a Reference variant to Object
// This would be defined in the actual object.rs file
impl Object {
    // This would be defined in the actual object implementation
    pub fn reference(obj: Arc<RwLock<Object>>) -> Self {
        Object::Reference(obj)
    }
}

// This enum variant would be added to the real Object enum
#[derive(Debug, Clone)]
pub enum ObjectVariant {
    Reference(Arc<RwLock<Object>>),
} 