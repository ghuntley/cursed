// Memory Management Module for CURSED
//
// This module provides memory management functionality including:
// - Garbage collection system
// - Memory allocation and deallocation
// - Object tracing and marking
// - Reference counting and cycle detection
// - Memory safety and leak detection

// Enable advanced memory management modules
pub mod allocator;
pub mod gc;
pub mod enhanced_gc;
pub mod production_gc;
pub mod mark_sweep;
pub mod generational;
pub mod copying;
pub mod incremental;
pub mod heap;
pub mod roots;
pub mod metadata;

// Export main interfaces
pub use gc::{MinimalImplementation as GcMinimal, get_minimal_result};
pub use enhanced_gc::{MinimalImplementation as EnhancedGcMinimal};
pub use production_gc::{MinimalImplementation as ProductionGcMinimal};

use std::sync::Arc;

/// Trait for objects that can be traced by the garbage collector
pub trait Traceable {
    /// Trace all object references within this object
    /// 
    /// This method should call `visit` on the visitor for each object
    /// reference contained within this object. This enables the garbage
    /// collector to build a complete picture of object reachability.
    fn trace(&self, visitor: &mut dyn Visitor);

    /// Get the runtime type tag for this object
    fn get_tag(&self) -> Tag;

    /// Get the size of this object in bytes
    fn size(&self) -> usize;
}

/// Visitor trait for garbage collection traversal
/// 
/// The visitor is used during the mark phase of garbage collection to
/// traverse the object graph starting from root objects.
pub trait Visitor {
    /// Visit an object during GC traversal
    /// 
    /// This method is called for each object reference encountered during
    /// garbage collection. The visitor will mark the object as reachable
    /// and continue traversing its references.
    fn visit(&mut self, obj: &dyn Traceable);
}

/// Object type tags for runtime type information
/// 
/// These tags help the garbage collector understand object layout
/// and provide debugging information during collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Regular object instance
    Object,
    /// Array or vector of objects
    Array,
    /// Function or closure object
    Function,
    /// String object
    String,
    /// Number (integer or float)
    Number,
    /// Boolean value
    Boolean,
    /// Nil/null value
    Nil,
    /// Interface object
    Interface,
    /// Channel for goroutine communication
    Channel,
    /// Custom user-defined type
    Custom(u32),
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Object => write!(f, "Object"),
            Tag::Array => write!(f, "Array"),
            Tag::Function => write!(f, "Function"),
            Tag::String => write!(f, "String"),
            Tag::Number => write!(f, "Number"),
            Tag::Boolean => write!(f, "Boolean"),
            Tag::Nil => write!(f, "Nil"),
            Tag::Interface => write!(f, "Interface"),
            Tag::Channel => write!(f, "Channel"),
            Tag::Custom(id) => write!(f, "Custom({})", id),
        }
    }
}

// Primitive types that don't contain references
impl Traceable for i32 {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Primitives have no references to trace
    }

    fn get_tag(&self) -> Tag {
        Tag::Number
    }

    fn size(&self) -> usize {
        std::mem::size_of::<i32>()
    }
}

impl Traceable for i64 {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Primitives have no references to trace
    }

    fn get_tag(&self) -> Tag {
        Tag::Number
    }

    fn size(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}

impl Traceable for f64 {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Primitives have no references to trace
    }

    fn get_tag(&self) -> Tag {
        Tag::Number
    }

    fn size(&self) -> usize {
        std::mem::size_of::<f64>()
    }
}

impl Traceable for bool {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Primitives have no references to trace
    }

    fn get_tag(&self) -> Tag {
        Tag::Boolean
    }

    fn size(&self) -> usize {
        std::mem::size_of::<bool>()
    }
}

impl Traceable for String {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Strings don't contain object references
    }

    fn get_tag(&self) -> Tag {
        Tag::String
    }

    fn size(&self) -> usize {
        std::mem::size_of::<String>() + self.capacity()
    }
}

// Container types that may contain references
impl<T: Traceable> Traceable for Vec<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for item in self.iter() {
            item.trace(visitor);
        }
    }

    fn get_tag(&self) -> Tag {
        Tag::Array
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Vec<T>>() + (self.capacity() * std::mem::size_of::<T>())
    }
}

impl<T: Traceable> Traceable for Option<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(ref value) = self {
            value.trace(visitor);
        }
    }

    fn get_tag(&self) -> Tag {
        match self {
            Some(_) => Tag::Object,
            None => Tag::Nil,
        }
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Option<T>>()
    }
}

impl<T: Traceable, E: Traceable> Traceable for Result<T, E> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        match self {
            Ok(ref value) => value.trace(visitor),
            Err(ref error) => error.trace(visitor),
        }
    }

    fn get_tag(&self) -> Tag {
        Tag::Object
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Result<T, E>>()
    }
}

impl<T: Traceable> Traceable for Arc<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        (**self).trace(visitor);
    }

    fn get_tag(&self) -> Tag {
        (**self).get_tag()
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Arc<T>>() + (**self).size()
    }
}

/// Memory statistics for debugging and monitoring
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub current_usage: usize,
    pub gc_collections: u64,
    pub gc_time_ms: u64,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            total_allocated: 0,
            total_freed: 0,
            current_usage: 0,
            gc_collections: 0,
            gc_time_ms: 0,
        }
    }
}

/// Root object holder for garbage collection
pub struct GcRoot {
    objects: Vec<Box<dyn Traceable>>,
}

impl std::fmt::Debug for GcRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GcRoot")
            .field("objects_count", &self.objects.len())
            .finish()
    }
}

impl GcRoot {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_root<T: Traceable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }

    pub fn trace_all(&self, visitor: &mut dyn Visitor) {
        for obj in &self.objects {
            obj.trace(visitor);
        }
    }
}

impl Default for GcRoot {
    fn default() -> Self {
        Self::new()
    }
}
