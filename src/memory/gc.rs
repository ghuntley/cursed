// Garbage collector implementation
use std::collections::HashSet;
use std::rc::Rc;
/// Alias for Traceable with explicit size information
pub trait Trace: Traceable {
    
}

/// Trait for objects that can be traced by the garbage collector
pub trait Traceable {
    fn trace(&self, visitor: &mut dyn Visitor);
    fn size(&self) -> usize;
}

/// Trait for visitors that traverse object graphs
pub trait Visitor {
    fn visit(&mut self, obj: &dyn Traceable);
    fn visit_ptr(&mut self, ptr: usize, tag: crate::memory::tagged::Tag);
}

/// A garbage collector implementation
pub struct GarbageCollector {
    marked: HashSet<usize>, // Using addresses as identifiers
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new() -> Self {
        GarbageCollector {
            marked: HashSet::new(),
        }
    }
    
    /// Mark an object as reachable
    pub fn mark(&mut self, obj: &dyn Traceable) {
        let thin_ptr = obj as *const _ as *const ();
        let addr = thin_ptr as usize;
        if !self.marked.contains(&addr) {
            self.marked.insert(addr);
            obj.trace(self);
        }
    }
    
    /// Clear all marks
    pub fn reset(&mut self) {
        self.marked.clear();
    }
}

impl Visitor for GarbageCollector {
    fn visit(&mut self, obj: &dyn Traceable) {
        self.mark(obj);
    }
    
    fn visit_ptr(&mut self, ptr: usize, _tag: crate::memory::tagged::Tag) {
        self.marked.insert(ptr);
    }
}

/// A garbage-collected reference
pub struct Gc<T: Traceable + 'static> {
    inner: Rc<T>,
}

impl<T: Traceable + 'static> Gc<T> {
    /// Create a new garbage-collected reference
    pub fn new(value: T) -> Self {
        Gc {
            inner: Rc::new(value),
        }
    }
    
    /// Get a reference to the inner value
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

// Implement Traceable for primitive types
impl Traceable for i64 {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Primitive types don't have pointers to trace
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}

impl Traceable for f64 {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Primitive types don't have pointers to trace
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<f64>()
    }
}

impl Traceable for bool {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Primitive types don't have pointers to trace
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<bool>()
    }
}

impl Traceable for String {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // String doesn't have pointers to other traceable objects
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<String>() + self.capacity()
    }
}

impl<T: Traceable> Traceable for Vec<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for item in self {
            item.trace(visitor);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Vec<T>>() + (self.capacity() * std::mem::size_of::<T>())
    }
}

// Implement Trace for primitive types
impl Trace for i64 {}
impl Trace for f64 {}
impl Trace for bool {}
impl Trace for String {}
impl<T: Traceable + Trace> Trace for Vec<T> {} 