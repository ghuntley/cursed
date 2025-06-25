/// Memory Management System for CURSED
/// 
/// This module provides a comprehensive garbage collection system with the following components:
/// 
/// 1. **Object Identification**: Unique ID system for tracking objects throughout their lifecycle
/// 2. **Heap Management**: Low-level memory allocation and layout management
/// 3. **Object Store**: High-level object storage with type safety and lifecycle management
/// 4. **Garbage Collection**: Advanced generational collector with multiple strategies
/// 5. **Root Set Management**: Comprehensive root tracking for reachability analysis
/// 6. **Generational Collection**: Young and old generation collection with promotion
/// 7. **Incremental Collection**: Low-latency collection with write barriers
/// 8. **Cycle Detection**: Detection and collection of circular references
/// 9. **Collection Triggers**: Smart heuristics for when to trigger collection
/// 
/// The system ensures memory safety through automatic garbage collection while providing
/// excellent performance through optimized allocation strategies and minimal collection overhead.

pub mod object_id;
pub mod heap_manager;
pub mod object_store;
pub mod gc;
pub mod gc_types;
pub mod root_set;
pub mod heap;
pub mod allocator;
pub mod regions;
pub mod metadata;

// New generational GC components
pub mod generational;
pub mod mark_sweep;
pub mod copying;
pub mod incremental;
pub mod cycle_detection;
pub mod collection_triggers;
pub mod roots;

// Production GC system
pub mod production_gc;
pub mod simple_production_gc;
pub mod real_allocator;
pub mod pressure_detection;

// Adaptive GC system
pub mod adaptive_gc;

// Real heap management with proper memory algorithms
pub mod real_heap_manager;

// Enhanced garbage collector with real heap integration
pub mod enhanced_gc;

// Integration example demonstrating real heap usage
pub mod integration_example;

// Test module for real heap management

/// Visitor pattern interface for garbage collection traversal
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
/// Object type tags for runtime type information
/// 
/// These tags help the garbage collector understand object layout
/// and provide debugging information during collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Regular object instance
    /// Array or vector of objects
    /// Function or closure object
    /// String object
    /// Number (integer or float)
    /// Boolean value
    /// Nil/null value
    /// Interface object
    /// Channel for goroutine communication
    /// Custom user-defined type
impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
// Primitive types that don't contain references
impl Traceable for i8 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for i16 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for i32 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for i64 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for u8 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for u16 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for u32 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for u64 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for f32 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for f64 {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for bool {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for String {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

impl Traceable for &str {
    fn trace(&self, _visitor: &mut dyn Visitor) {}
}

/// Specialized Traceable implementations for container types
impl<T: Traceable> Traceable for Vec<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for item in self.iter() {
            item.trace(visitor);
        }
    }
impl<T: Traceable> Traceable for Option<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(ref item) = self {
            item.trace(visitor);
        }
    }
impl<T: Traceable, E: Traceable> Traceable for Result<T, E> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        match self {
        }
    }
/// Memory utilities and helper functions
pub mod utils {
    use super::*;
    
    /// Get the size of a type in bytes
    pub fn size_of<T>() -> usize {
        std::mem::size_of::<T>()
    /// Get the alignment requirement of a type
    pub fn align_of<T>() -> usize {
        std::mem::align_of::<T>()
    /// Check if a pointer is properly aligned for type T
    pub fn is_aligned<T>(ptr: *const T) -> bool {
        (ptr as usize) % std::mem::align_of::<T>() == 0
    /// Round up size to next alignment boundary
    pub fn align_size(size: usize, align: usize) -> usize {
        (size + align - 1) & !(align - 1)
    /// Calculate fragmentation ratio
    pub fn fragmentation_ratio(total_free: usize, largest_free: usize) -> f64 {
        if total_free == 0 {
            0.0
        } else {
            1.0 - (largest_free as f64 / total_free as f64)
        }
    }
