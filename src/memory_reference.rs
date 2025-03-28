// CURSED Memory Management Reference
// This file contains a reference implementation for memory management in the CURSED language.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::object::Object;

/// Memory management for the CURSED language
pub struct Memory {
    /// All allocated objects
    objects: Vec<Rc<RefCell<Object>>>,
    /// Objects that are visible to the garbage collector
    roots: Vec<Rc<RefCell<Object>>>,
    /// Object allocation count for statistics
    allocation_count: usize,
    /// Next scheduled garbage collection threshold
    next_gc: usize,
    /// Configuration: GC after this many allocations
    gc_threshold: usize,
    /// Debug mode, prints GC information
    debug: bool,
}

impl Memory {
    /// Create a new memory manager
    pub fn new() -> Self {
        let gc_threshold = 1000; // Default threshold
        Self {
            objects: Vec::new(),
            roots: Vec::new(),
            allocation_count: 0,
            next_gc: gc_threshold,
            gc_threshold,
            debug: false,
        }
    }
    
    /// Create a new memory manager with custom garbage collection threshold
    pub fn with_gc_threshold(threshold: usize) -> Self {
        let mut mem = Self::new();
        mem.gc_threshold = threshold;
        mem.next_gc = threshold;
        mem
    }
    
    /// Enable debug mode for GC operations
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
    
    /// Allocate a new object in memory
    pub fn allocate(&mut self, obj: Object) -> Rc<RefCell<Object>> {
        let rc_obj = Rc::new(RefCell::new(obj));
        self.objects.push(Rc::clone(&rc_obj));
        
        self.allocation_count += 1;
        if self.allocation_count >= self.next_gc {
            self.collect_garbage();
        }
        
        rc_obj
    }
    
    /// Add an object to the root set (protected from GC)
    pub fn add_root(&mut self, obj: Rc<RefCell<Object>>) {
        if !self.roots.iter().any(|root| Rc::ptr_eq(root, &obj)) {
            self.roots.push(Rc::clone(&obj));
        }
    }
    
    /// Remove an object from the root set
    pub fn remove_root(&mut self, obj: &Rc<RefCell<Object>>) {
        self.roots.retain(|root| !Rc::ptr_eq(root, obj));
    }
    
    /// Mark and sweep garbage collection
    pub fn collect_garbage(&mut self) {
        if self.debug {
            println!("Starting garbage collection...");
            println!("Objects before GC: {}", self.objects.len());
        }
        
        // Mark phase: mark all reachable objects
        let mut marked = HashMap::new();
        self.mark_roots(&mut marked);
        
        // Sweep phase: remove unmarked objects
        self.objects.retain(|obj| {
            let is_marked = marked.contains_key(&(Rc::as_ptr(obj) as usize));
            is_marked
        });
        
        // Update next GC threshold
        self.allocation_count = 0;
        self.next_gc = self.objects.len() + self.gc_threshold;
        
        if self.debug {
            println!("Objects after GC: {}", self.objects.len());
            println!("Next GC at {} allocations", self.next_gc);
        }
    }
    
    /// Mark all objects reachable from roots
    fn mark_roots(&self, marked: &mut HashMap<usize, bool>) {
        for root in &self.roots {
            self.mark_object(root, marked);
        }
    }
    
    /// Mark an object and its children
    fn mark_object(&self, obj: &Rc<RefCell<Object>>, marked: &mut HashMap<usize, bool>) {
        let ptr = Rc::as_ptr(obj) as usize;
        
        // If already marked, return to avoid cycles
        if marked.contains_key(&ptr) {
            return;
        }
        
        // Mark this object
        marked.insert(ptr, true);
        
        // Mark children based on object type
        match &*obj.borrow() {
            Object::Array(elements) => {
                for elem in elements {
                    if let Some(rc_elem) = elem.as_rc_refcell() {
                        self.mark_object(rc_elem, marked);
                    }
                }
            },
            Object::Hash(pairs) => {
                for (key, value) in pairs {
                    if let Some(rc_key) = key.as_rc_refcell() {
                        self.mark_object(rc_key, marked);
                    }
                    if let Some(rc_value) = value.as_rc_refcell() {
                        self.mark_object(rc_value, marked);
                    }
                }
            },
            Object::Function { body, env, .. } => {
                if let Some(rc_env) = env.as_rc_refcell() {
                    self.mark_object(rc_env, marked);
                }
            },
            Object::Closure { .. } => {
                // Mark captured variables in the closure
                if let Object::Closure { function, free } = &*obj.borrow() {
                    if let Some(rc_fn) = function.as_rc_refcell() {
                        self.mark_object(rc_fn, marked);
                    }
                    for free_var in free {
                        if let Some(rc_var) = free_var.as_rc_refcell() {
                            self.mark_object(rc_var, marked);
                        }
                    }
                }
            },
            Object::Environment(env) => {
                // Mark all values in the environment
                for (_, value) in env.store.borrow().iter() {
                    if let Some(rc_value) = value.as_rc_refcell() {
                        self.mark_object(rc_value, marked);
                    }
                }
                // Mark outer environment if it exists
                if let Some(outer) = &env.outer {
                    if let Some(rc_outer) = outer.as_rc_refcell() {
                        self.mark_object(rc_outer, marked);
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
pub trait AsRcRefCell {
    fn as_rc_refcell(&self) -> Option<&Rc<RefCell<Object>>>;
}

impl AsRcRefCell for Object {
    fn as_rc_refcell(&self) -> Option<&Rc<RefCell<Object>>> {
        match self {
            Object::Reference(rc) => Some(rc),
            _ => None,
        }
    }
}

// For the reference implementation, we add a Reference variant to Object
// This would be defined in the actual object.rs file
impl Object {
    // This would be defined in the actual object implementation
    pub fn reference(obj: Rc<RefCell<Object>>) -> Self {
        Object::Reference(obj)
    }
}

// This enum variant would be added to the real Object enum
#[derive(Debug, Clone)]
pub enum ObjectVariant {
    Reference(Rc<RefCell<Object>>),
} 