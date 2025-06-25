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
#[cfg(test)]
pub mod test_real_heap;

// Re-export main types for convenience
pub use gc::{GarbageCollector, Gc, WeakGc, CollectionStats, GcStats};
pub use gc_types::{CollectionAlgorithm, CollectionTrigger, EnhancedCollectionStats, HeapStats as GcHeapStats, AlgorithmStats, ComprehensiveGcStats};
pub use object_id::{ObjectId, ObjectIdGenerator, ObjectMetadata, ObjectRegistry, SharedObjectRegistry};
pub use heap_manager::{HeapManager, HeapConfig, HeapStats, AllocationInfo};
pub use object_store::{ObjectStore, ObjectHandle, Storable, StoredObjectInfo, ObjectStoreStats};
pub use root_set::{RootSetManager, RootType, RootInfo, RootSetStats, SharedRootSetManager};
pub use heap::{Heap, HeapConfiguration, HeapStatistics, AllocationStrategy};
pub use allocator::{Allocator, BumpAllocator, FreeListAllocator, SegregatedAllocator, AllocationResult, AllocatorStatistics};
pub use regions::{HeapRegion, YoungGeneration, OldGeneration, LargeObjectSpace, RegionManager, RegionType, RegionStatistics};
pub use metadata::{ObjectHeader, ObjectMetadata as ExtendedObjectMetadata, MetadataManager, MemoryLayout, MetadataStatistics};

// Re-export generational GC types
pub use generational::{GenerationalCollector, GenerationalConfig, GenerationalStats, Generation, CollectionStrategy};
pub use mark_sweep::{MarkSweepCollector, MarkSweepConfig, MarkSweepStats};
pub use copying::{CopyingCollector, CopyingConfig, CopyingStats};
pub use incremental::{IncrementalCollector, IncrementalConfig, IncrementalStats, IncrementalWorkType};
pub use cycle_detection::{CycleDetector, CycleDetectionConfig, CycleDetectionStats, CycleInfo, CycleDetectionAlgorithm};
pub use collection_triggers::{CollectionTriggerManager, TriggerConfig, TriggerStats, TriggerType, TriggerReason};
pub use roots::{RootSetManager as NewRootSetManager, RootType as NewRootType, RootInfo as NewRootInfo, RootSetStats as NewRootSetStats, StackScanConfig};

// Re-export production GC types
pub use production_gc::{ProductionGarbageCollector, ProductionGcConfig, ProductionGcStats};
pub use simple_production_gc::{SimpleProductionGarbageCollector, SimpleProductionGcConfig, SimpleProductionStats};
pub use real_allocator::{RealMemoryAllocator, RealAllocatorConfig, RealAllocatorStats, AllocationStrategy as RealAllocationStrategy};
pub use pressure_detection::{MemoryPressureDetector, PressureDetectionConfig, PressureLevel, SystemMemoryInfo};

// Re-export adaptive GC types
pub use adaptive_gc::{
    AdaptiveGarbageCollector, AdaptiveGcConfig, AdaptiveGcStats, AdaptiveStrategy, 
    BehaviorPattern, AdaptiveThresholds, PerformanceMetrics, TargetMetrics
};

// Re-export real heap management types
pub use real_heap_manager::{
    RealHeapManager, RealHeapConfig, RealHeapStatistics, RealHeapBlock
};

// Re-export enhanced GC types
pub use enhanced_gc::{
    EnhancedGarbageCollector, EnhancedGcStats
};

/// Core trait for objects that can be traced during garbage collection
/// 
/// This trait enables the garbage collector to traverse object references
/// and perform proper reachability analysis during collection cycles.
pub trait Traceable {
    /// Trace all object references for garbage collection
    /// 
    /// Implementations should call visitor.visit() for each object reference
    /// contained within this object. This enables the GC to follow object
    /// relationships during mark-and-sweep collection.
    fn trace(&self, visitor: &mut dyn Visitor);
}

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
}

impl<T: Traceable> Traceable for Option<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(ref item) = self {
            item.trace(visitor);
        }
    }
}

impl<T: Traceable, E: Traceable> Traceable for Result<T, E> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        match self {
            Ok(ref item) => item.trace(visitor),
            Err(ref error) => error.trace(visitor),
        }
    }
}

/// Memory utilities and helper functions
pub mod utils {
    use super::*;
    
    /// Get the size of a type in bytes
    pub fn size_of<T>() -> usize {
        std::mem::size_of::<T>()
    }
    
    /// Get the alignment requirement of a type
    pub fn align_of<T>() -> usize {
        std::mem::align_of::<T>()
    }
    
    /// Check if a pointer is properly aligned for type T
    pub fn is_aligned<T>(ptr: *const T) -> bool {
        (ptr as usize) % std::mem::align_of::<T>() == 0
    }
    
    /// Round up size to next alignment boundary
    pub fn align_size(size: usize, align: usize) -> usize {
        (size + align - 1) & !(align - 1)
    }
    
    /// Calculate fragmentation ratio
    pub fn fragmentation_ratio(total_free: usize, largest_free: usize) -> f64 {
        if total_free == 0 {
            0.0
        } else {
            1.0 - (largest_free as f64 / total_free as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Basic test struct for unit testing
    #[derive(Debug)]
    struct SimpleObject {
        value: i32,
    }
    
    impl Traceable for SimpleObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // No references to trace
        }
    }
    
    #[test]
    fn test_heap_manager_integration() {
        let config = HeapConfig::default();
        let registry = std::sync::Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        let (id, ptr) = heap.allocate::<i32>(4, "i32").unwrap();
        assert!(!id.is_null());
        assert!(heap.is_valid_pointer(ptr.as_ptr()));
        
        let stats = heap.get_stats().unwrap();
        assert!(stats.total_used > 0);
        assert_eq!(stats.active_objects, 1);
    }
    
    #[test]
    fn test_root_set_integration() {
        let root_manager = RootSetManager::new();
        let obj_id = ObjectId::new(123);
        
        // Add as global root
        root_manager.add_global_root(obj_id, Some("test object".to_string())).unwrap();
        assert!(root_manager.is_root(obj_id));
        
        let all_roots = root_manager.get_all_roots().unwrap();
        assert!(all_roots.contains(&obj_id));
        
        let stats = root_manager.get_stats().unwrap();
        assert_eq!(stats.global_roots, 1);
        assert_eq!(stats.total_roots, 1);
    }
    
    #[test]
    fn test_memory_utils() {
        assert_eq!(utils::size_of::<i32>(), 4);
        assert_eq!(utils::size_of::<i64>(), 8);
        
        assert_eq!(utils::align_size(5, 8), 8);
        assert_eq!(utils::align_size(8, 8), 8);
        assert_eq!(utils::align_size(9, 8), 16);
        
        assert_eq!(utils::fragmentation_ratio(100, 50), 0.5);
        assert_eq!(utils::fragmentation_ratio(0, 0), 0.0);
    }
}
