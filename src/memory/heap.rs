/// Advanced Heap Memory Management System
/// 
/// This module provides the core heap implementation for the CURSED garbage collector
/// with advanced memory management features including:
/// 
/// 1. **Multi-Generation Heap**: Young and old generations for efficient collection
/// 2. **Large Object Space**: Dedicated space for large allocations
/// 3. **Memory Regions**: Segregated heap regions for different object types
/// 4. **Allocation Strategies**: Multiple allocation algorithms optimized for different use cases
/// 5. **Memory Alignment**: Strict alignment guarantees for all object types
/// 6. **Statistics Tracking**: Comprehensive monitoring of heap utilization and performance

use std::sync::{Arc, RwLock, Mutex};
use std::ptr::NonNull;
use std::collections::HashMap;
use tracing::{instrument, debug, info, warn, error};

use crate::memory::allocator::{Allocator, BumpAllocator, FreeListAllocator, SegregatedAllocator, AllocationResult};
use crate::memory::regions::{HeapRegion, YoungGeneration, OldGeneration, LargeObjectSpace, RegionManager};
use crate::memory::metadata::{ObjectHeader, ObjectMetadata, MetadataManager};
use crate::memory::object_id::{ObjectId, SharedObjectRegistry};
use crate::profiling::memory::MemoryProfiler;

/// Configuration for heap behavior and performance tuning
#[derive(Debug, Clone)]
pub struct HeapConfiguration {
    /// Initial heap size in bytes
    pub initial_heap_size: usize,
    /// Maximum heap size in bytes
    pub max_heap_size: usize,
    /// Young generation size ratio (0.0 - 1.0)
    pub young_gen_ratio: f64,
    /// Large object threshold in bytes
    pub large_object_threshold: usize,
    /// Enable generational collection
    pub generational_gc: bool,
    /// Memory alignment requirement
    pub alignment: usize,
    /// Allocation algorithm preference
    pub allocation_strategy: AllocationStrategy,
    /// Enable memory statistics tracking
    pub track_statistics: bool,
}

impl Default for HeapConfiguration {
    fn default() -> Self {
        Self {
            initial_heap_size: 16 * 1024 * 1024, // 16MB
            max_heap_size: 1024 * 1024 * 1024,  // 1GB
            young_gen_ratio: 0.3,                // 30% for young generation
            large_object_threshold: 85 * 1024,   // 85KB threshold
            generational_gc: true,
            alignment: 8,                         // 8-byte alignment
            allocation_strategy: AllocationStrategy::Hybrid,
            track_statistics: true,
        }
    }
}

/// Allocation strategy preference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationStrategy {
    /// Bump allocation for fast allocation
    Bump,
    /// Free list for minimal fragmentation
    FreeList,
    /// Segregated lists for different sizes
    Segregated,
    /// Hybrid approach adapting to usage patterns
    Hybrid,
}

/// Main heap structure coordinating all memory management
/// 
/// The heap provides a unified interface for memory allocation while
/// delegating to specialized allocators and regions based on object
/// characteristics and allocation patterns.
pub struct Heap {
    /// Heap configuration
    config: HeapConfiguration,
    /// Region manager for heap space organization
    region_manager: Arc<RwLock<RegionManager>>,
    /// Metadata manager for object headers
    metadata_manager: Arc<Mutex<MetadataManager>>,
    /// Object registry for GC integration
    object_registry: SharedObjectRegistry,
    /// Memory profiler (optional)
    profiler: Option<Arc<MemoryProfiler>>,
    /// Heap statistics
    statistics: Arc<Mutex<HeapStatistics>>,
    /// Current allocation strategy
    current_strategy: Mutex<AllocationStrategy>,
}

impl Heap {
    /// Create a new heap with the given configuration
    #[instrument]
    pub fn new(config: HeapConfiguration, object_registry: SharedObjectRegistry) -> Result<Self, String> {
        info!("Creating heap with {} MB initial size, {} MB max size", 
              config.initial_heap_size / (1024 * 1024),
              config.max_heap_size / (1024 * 1024));
        
        let region_manager = Arc::new(RwLock::new(
            RegionManager::new(&config)?
        ));
        
        let metadata_manager = Arc::new(Mutex::new(
            MetadataManager::new(config.alignment)?
        ));
        
        let statistics = Arc::new(Mutex::new(HeapStatistics::new()));
        
        let allocation_strategy = config.allocation_strategy;
        
        Ok(Self {
            config,
            region_manager,
            metadata_manager,
            object_registry,
            profiler: None,
            statistics,
            current_strategy: Mutex::new(allocation_strategy),
        })
    }
    
    /// Set memory profiler for heap monitoring
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        info!("Enabling memory profiling for heap");
        self.profiler = Some(profiler);
    }
    
    /// Allocate memory for an object of the given size and type
    #[instrument(skip(self))]
    pub fn allocate(&self, size: usize, alignment: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        if size == 0 {
            return Err("Cannot allocate zero bytes".to_string());
        }
        
        let actual_alignment = alignment.max(self.config.alignment);
        
        debug!("Allocating {} bytes for {} with {}-byte alignment", 
               size, type_name, actual_alignment);
        
        // Determine allocation region based on size and configuration
        let allocation_result = if size >= self.config.large_object_threshold {
            self.allocate_large_object(size, actual_alignment, type_name)
        } else if self.config.generational_gc {
            self.allocate_in_young_generation(size, actual_alignment, type_name)
        } else {
            self.allocate_in_main_heap(size, actual_alignment, type_name)
        }?;
        
        // Generate object ID
        let object_id = crate::memory::object_id::ObjectIdGenerator::new().next();
        
        // Update statistics
        if self.config.track_statistics {
            self.update_allocation_statistics(size, type_name)?;
        }
        
        // Profile the allocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_allocation(size, allocation_result.ptr.as_ptr() as u64, Vec::new());
        }
        
        debug!("Successfully allocated object {} at {:p}", object_id, allocation_result.ptr.as_ptr());
        Ok((object_id, allocation_result.ptr))
    }
    
    /// Deallocate an object
    #[instrument(skip(self))]
    pub fn deallocate(&self, object_id: ObjectId, ptr: NonNull<u8>, size: usize) -> Result<(), String> {
        debug!("Deallocating object {} at {:p}", object_id, ptr.as_ptr());
        
        // Determine which region owns this object
        let region_manager = self.region_manager.read()
            .map_err(|_| "Failed to acquire region manager read lock")?;
        
        if let Some(region) = region_manager.find_region_for_pointer(ptr.as_ptr()) {
            region.deallocate(ptr, size)?;
        } else {
            return Err(format!("Object {:p} not found in any heap region", ptr.as_ptr()));
        }
        
        // Update statistics
        if self.config.track_statistics {
            self.update_deallocation_statistics(size, "unknown")?;
        }
        
        // Profile the deallocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_deallocation(ptr.as_ptr() as u64, Vec::new());
        }
        
        debug!("Successfully deallocated object {}", object_id);
        Ok(())
    }
    
    /// Allocate in young generation
    fn allocate_in_young_generation(&self, size: usize, alignment: usize, type_name: &str) 
        -> Result<AllocationResult, String> {
        
        let region_manager = self.region_manager.read()
            .map_err(|_| "Failed to acquire region manager read lock")?;
        
        if let Some(young_gen) = region_manager.young_generation() {
            young_gen.allocate(size, alignment, type_name)
        } else {
            // Fallback to main heap if no young generation
            drop(region_manager);
            self.allocate_in_main_heap(size, alignment, type_name)
        }
    }
    
    /// Allocate in main heap (old generation or unified heap)
    fn allocate_in_main_heap(&self, size: usize, alignment: usize, type_name: &str) 
        -> Result<AllocationResult, String> {
        
        let region_manager = self.region_manager.read()
            .map_err(|_| "Failed to acquire region manager read lock")?;
        
        if let Some(old_gen) = region_manager.old_generation() {
            old_gen.allocate(size, alignment, type_name)
        } else {
            // Use primary region if no generational collection
            region_manager.primary_region().allocate(size, alignment, type_name)
        }
    }
    
    /// Allocate large object in dedicated space
    fn allocate_large_object(&self, size: usize, alignment: usize, type_name: &str) 
        -> Result<AllocationResult, String> {
        
        let region_manager = self.region_manager.read()
            .map_err(|_| "Failed to acquire region manager read lock")?;
        
        if let Some(large_space) = region_manager.large_object_space() {
            large_space.allocate(size, alignment, type_name)
        } else {
            // Fallback to main heap if no large object space
            drop(region_manager);
            self.allocate_in_main_heap(size, alignment, type_name)
        }
    }
    

    
    /// Update allocation statistics
    fn update_allocation_statistics(&self, size: usize, type_name: &str) -> Result<(), String> {
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        stats.record_allocation(size, type_name);
        Ok(())
    }
    
    /// Update deallocation statistics  
    fn update_deallocation_statistics(&self, size: usize, type_name: &str) -> Result<(), String> {
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        stats.record_deallocation(size, type_name);
        Ok(())
    }
    
    /// Get comprehensive heap statistics
    pub fn get_statistics(&self) -> Result<HeapStatistics, String> {
        let stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        let region_manager = self.region_manager.read()
            .map_err(|_| "Failed to acquire region manager read lock")?;
        
        let mut heap_stats = stats.clone();
        heap_stats.merge_region_statistics(&region_manager)?;
        
        Ok(heap_stats)
    }
    
    /// Perform garbage collection preparation
    /// 
    /// This prepares the heap for a garbage collection cycle by
    /// updating metadata and preparing regions for sweep phase.
    pub fn prepare_for_collection(&self) -> Result<(), String> {
        info!("Preparing heap for garbage collection");
        
        let region_manager = self.region_manager.read()
            .map_err(|_| "Failed to acquire region manager read lock")?;
        
        region_manager.prepare_for_collection()?;
        
        debug!("Heap prepared for garbage collection");
        Ok(())
    }
    
    /// Complete garbage collection cleanup
    /// 
    /// This performs post-collection cleanup including compaction
    /// and statistics updates.
    pub fn complete_collection(&self, collected_objects: usize, collected_bytes: usize) -> Result<(), String> {
        info!("Completing garbage collection: {} objects, {} bytes", 
              collected_objects, collected_bytes);
        
        // Update collection statistics
        if self.config.track_statistics {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_collection(collected_objects, collected_bytes);
        }
        
        // Trigger region compaction if needed
        let region_manager = self.region_manager.read()
            .map_err(|_| "Failed to acquire region manager read lock")?;
        region_manager.complete_collection()?;
        
        debug!("Garbage collection cleanup completed");
        Ok(())
    }
    
    /// Check if pointer is within heap bounds
    pub fn contains_pointer(&self, ptr: *const u8) -> bool {
        if let Ok(region_manager) = self.region_manager.read() {
            region_manager.contains_pointer(ptr)
        } else {
            false
        }
    }
    
    /// Get object metadata for a pointer
    pub fn get_object_metadata(&self, ptr: NonNull<u8>) -> Result<ObjectMetadata, String> {
        let metadata_manager = self.metadata_manager.lock()
            .map_err(|_| "Failed to acquire metadata manager lock")?;
        
        metadata_manager.get_metadata(ptr)
    }
    
    /// Adapt allocation strategy based on usage patterns
    pub fn adapt_allocation_strategy(&self) -> Result<(), String> {
        if self.config.allocation_strategy != AllocationStrategy::Hybrid {
            return Ok(()); // Only adapt if using hybrid strategy
        }
        
        let stats = self.get_statistics()?;
        let new_strategy = self.determine_optimal_strategy(&stats);
        
        let mut current_strategy = self.current_strategy.lock()
            .map_err(|_| "Failed to acquire strategy lock")?;
        
        if *current_strategy != new_strategy {
            info!("Adapting allocation strategy from {:?} to {:?}", *current_strategy, new_strategy);
            *current_strategy = new_strategy;
            
            // Update region allocators
            let region_manager = self.region_manager.read()
                .map_err(|_| "Failed to acquire region manager read lock")?;
            region_manager.update_allocation_strategy(new_strategy)?;
        }
        
        Ok(())
    }
    
    /// Determine optimal allocation strategy based on statistics
    fn determine_optimal_strategy(&self, stats: &HeapStatistics) -> AllocationStrategy {
        // Simple heuristics for strategy selection
        if stats.fragmentation_ratio > 0.4 {
            AllocationStrategy::FreeList // Reduce fragmentation
        } else if stats.average_allocation_size < 64.0 {
            AllocationStrategy::Segregated // Optimize for small objects
        } else {
            AllocationStrategy::Bump // Fast allocation for larger objects
        }
    }
}



/// Comprehensive heap statistics for monitoring and debugging
#[derive(Debug, Clone)]
pub struct HeapStatistics {
    /// Total bytes allocated
    pub total_allocated: usize,
    /// Total bytes deallocated
    pub total_deallocated: usize,
    /// Current heap usage
    pub current_usage: usize,
    /// Peak heap usage
    pub peak_usage: usize,
    /// Total number of allocations
    pub allocation_count: u64,
    /// Total number of deallocations
    pub deallocation_count: u64,
    /// Number of garbage collections
    pub collection_count: u64,
    /// Total bytes collected
    pub total_collected: usize,
    /// Average allocation size
    pub average_allocation_size: f64,
    /// Fragmentation ratio (0.0 = no fragmentation, 1.0 = maximum fragmentation)
    pub fragmentation_ratio: f64,
    /// Allocation statistics by type
    pub type_statistics: HashMap<String, TypeStatistics>,
    /// Heap utilization percentage
    pub utilization_percentage: f64,
}

impl HeapStatistics {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self {
            total_allocated: 0,
            total_deallocated: 0,
            current_usage: 0,
            peak_usage: 0,
            allocation_count: 0,
            deallocation_count: 0,
            collection_count: 0,
            total_collected: 0,
            average_allocation_size: 0.0,
            fragmentation_ratio: 0.0,
            type_statistics: HashMap::new(),
            utilization_percentage: 0.0,
        }
    }
    
    /// Record an allocation
    pub fn record_allocation(&mut self, size: usize, type_name: &str) {
        self.total_allocated += size;
        self.current_usage += size;
        self.allocation_count += 1;
        
        if self.current_usage > self.peak_usage {
            self.peak_usage = self.current_usage;
        }
        
        self.average_allocation_size = self.total_allocated as f64 / self.allocation_count as f64;
        
        // Update type statistics
        let type_stats = self.type_statistics.entry(type_name.to_string())
            .or_insert_with(TypeStatistics::new);
        type_stats.record_allocation(size);
    }
    
    /// Record a deallocation
    pub fn record_deallocation(&mut self, size: usize, type_name: &str) {
        self.total_deallocated += size;
        self.current_usage = self.current_usage.saturating_sub(size);
        self.deallocation_count += 1;
        
        // Update type statistics
        if let Some(type_stats) = self.type_statistics.get_mut(type_name) {
            type_stats.record_deallocation(size);
        }
    }
    
    /// Record a garbage collection
    pub fn record_collection(&mut self, objects_collected: usize, bytes_collected: usize) {
        self.collection_count += 1;
        self.total_collected += bytes_collected;
        self.current_usage = self.current_usage.saturating_sub(bytes_collected);
    }
    
    /// Merge statistics from region manager
    pub fn merge_region_statistics(&mut self, region_manager: &RegionManager) -> Result<(), String> {
        let region_stats = region_manager.get_statistics()?;
        
        self.fragmentation_ratio = region_stats.overall_fragmentation;
        self.utilization_percentage = region_stats.utilization_percentage;
        
        Ok(())
    }
}

/// Statistics for a specific object type
#[derive(Debug, Clone)]
pub struct TypeStatistics {
    /// Total allocations for this type
    pub allocation_count: u64,
    /// Total deallocations for this type
    pub deallocation_count: u64,
    /// Total bytes allocated for this type
    pub total_allocated: usize,
    /// Total bytes deallocated for this type
    pub total_deallocated: usize,
    /// Current live objects of this type
    pub live_objects: u64,
    /// Current bytes used by this type
    pub current_usage: usize,
}

impl TypeStatistics {
    pub fn new() -> Self {
        Self {
            allocation_count: 0,
            deallocation_count: 0,
            total_allocated: 0,
            total_deallocated: 0,
            live_objects: 0,
            current_usage: 0,
        }
    }
    
    pub fn record_allocation(&mut self, size: usize) {
        self.allocation_count += 1;
        self.total_allocated += size;
        self.live_objects += 1;
        self.current_usage += size;
    }
    
    pub fn record_deallocation(&mut self, size: usize) {
        self.deallocation_count += 1;
        self.total_deallocated += size;
        self.live_objects = self.live_objects.saturating_sub(1);
        self.current_usage = self.current_usage.saturating_sub(size);
    }
}

impl std::fmt::Display for HeapStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Heap Statistics:\n\
             - Current Usage: {:.2} MB ({:.1}%)\n\
             - Peak Usage: {:.2} MB\n\
             - Total Allocated: {:.2} MB\n\
             - Allocations: {} (avg {:.1} bytes)\n\
             - Collections: {} ({:.2} MB collected)\n\
             - Fragmentation: {:.1}%\n\
             - Live Objects: {}",
            self.current_usage as f64 / (1024.0 * 1024.0),
            self.utilization_percentage,
            self.peak_usage as f64 / (1024.0 * 1024.0),
            self.total_allocated as f64 / (1024.0 * 1024.0),
            self.allocation_count,
            self.average_allocation_size,
            self.collection_count,
            self.total_collected as f64 / (1024.0 * 1024.0),
            self.fragmentation_ratio * 100.0,
            self.allocation_count - self.deallocation_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectRegistry;
    
    #[test]
    fn test_heap_creation() {
        let config = HeapConfiguration::default();
        let registry = Arc::new(ObjectRegistry::new());
        
        let heap = Heap::new(config.clone(), registry).unwrap();
        let stats = heap.get_statistics().unwrap();
        
        assert_eq!(stats.allocation_count, 0);
        assert_eq!(stats.current_usage, 0);
    }
    
    #[test]
    fn test_allocation_and_deallocation() {
        let config = HeapConfiguration::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap = Heap::new(config, registry).unwrap();
        
        let (id, ptr) = heap.allocate(64, 8, "test").unwrap();
        
        let stats = heap.get_statistics().unwrap();
        assert_eq!(stats.allocation_count, 1);
        assert!(stats.current_usage > 0);
        
        heap.deallocate(id, ptr, 64).unwrap();
        
        let stats_after = heap.get_statistics().unwrap();
        assert_eq!(stats_after.deallocation_count, 1);
    }
    
    #[test]
    fn test_large_object_allocation() {
        let mut config = HeapConfiguration::default();
        config.large_object_threshold = 1024;
        let registry = Arc::new(ObjectRegistry::new());
        let heap = Heap::new(config, registry).unwrap();
        
        // Allocate large object
        let (id, ptr) = heap.allocate(2048, 8, "large_object").unwrap();
        assert!(heap.contains_pointer(ptr.as_ptr()));
        
        heap.deallocate(id, ptr, 2048).unwrap();
    }
    
    #[test]
    fn test_heap_statistics() {
        let config = HeapConfiguration::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap = Heap::new(config, registry).unwrap();
        
        // Multiple allocations
        let allocs = vec![
            heap.allocate(32, 8, "small").unwrap(),
            heap.allocate(64, 8, "medium").unwrap(),
            heap.allocate(128, 8, "large").unwrap(),
        ];
        
        let stats = heap.get_statistics().unwrap();
        assert_eq!(stats.allocation_count, 3);
        assert!(stats.average_allocation_size > 0.0);
        
        // Deallocate all
        let sizes = [32, 64, 128];
        for ((id, ptr), &size) in allocs.iter().zip(sizes.iter()) {
            heap.deallocate(*id, *ptr, size).unwrap();
        }
        
        let final_stats = heap.get_statistics().unwrap();
        assert_eq!(final_stats.deallocation_count, 3);
    }
}
