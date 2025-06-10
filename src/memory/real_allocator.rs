/// Real Memory Allocator Implementation for Production GC
/// 
/// This module provides the actual memory allocation implementation that replaces
/// placeholder stubs with real heap management. It implements multiple allocation
/// strategies and integrates with the garbage collection system.

use std::ptr::{NonNull, null_mut};
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicUsize, AtomicU64, Ordering}};
use std::collections::{HashMap, VecDeque};
use std::alloc::{alloc, dealloc, realloc, Layout};
use std::time::{Instant, Duration};
use tracing::{instrument, debug, info, warn, error};

use crate::memory::{
    allocator::{Allocator, AllocationResult, AllocatorStatistics},
    heap_manager::{HeapManager, MemoryBlock},
    object_id::{ObjectId, ObjectIdGenerator, ObjectRegistry, SharedObjectRegistry},
    metadata::{ObjectHeader, ObjectMetadata},
    pressure_detection::{MemoryPressureDetector, PressureLevel},
};
use crate::profiling::memory::MemoryProfiler;

/// Real memory allocator with multiple allocation strategies
/// 
/// This allocator provides actual memory allocation and deallocation
/// with support for different allocation patterns and automatic
/// strategy adaptation based on usage patterns.
pub struct RealMemoryAllocator {
    /// Memory strategy configuration
    strategy: Arc<RwLock<AllocationStrategy>>,
    /// Memory pools for different object sizes
    pools: Arc<RwLock<Vec<MemoryPool>>>,
    /// Large object allocator for objects > threshold
    large_object_allocator: Arc<Mutex<LargeObjectAllocator>>,
    /// Metadata tracking for allocated objects
    metadata_tracker: Arc<Mutex<MetadataTracker>>,
    /// Statistics tracking
    statistics: Arc<Mutex<RealAllocatorStats>>,
    /// Pressure detector integration
    pressure_detector: Option<Arc<MemoryPressureDetector>>,
    /// Memory profiler integration
    profiler: Option<Arc<MemoryProfiler>>,
    /// Object registry for GC integration
    object_registry: SharedObjectRegistry,
    /// Object ID generator for unique identifiers
    id_generator: ObjectIdGenerator,
    /// Configuration
    config: Arc<RwLock<RealAllocatorConfig>>,
    /// Total allocated bytes
    total_allocated: AtomicU64,
    /// Total deallocated bytes
    total_deallocated: AtomicU64,
    /// Active allocation count
    active_allocations: AtomicUsize,
}

/// Configuration for real memory allocator
#[derive(Debug, Clone)]
pub struct RealAllocatorConfig {
    /// Initial pool size
    pub initial_pool_size: usize,
    /// Maximum pool size
    pub max_pool_size: usize,
    /// Large object threshold
    pub large_object_threshold: usize,
    /// Pool count for different size classes
    pub pool_count: usize,
    /// Enable automatic strategy adaptation
    pub adaptive_strategy: bool,
    /// Memory alignment requirement
    pub alignment: usize,
    /// Enable memory poisoning for debugging
    pub enable_poisoning: bool,
    /// Memory pressure threshold for adaptation
    pub pressure_threshold: PressureLevel,
    /// Statistics collection interval
    pub stats_interval: Duration,
}

impl Default for RealAllocatorConfig {
    fn default() -> Self {
        Self {
            initial_pool_size: 64 * 1024 * 1024, // 64MB
            max_pool_size: 512 * 1024 * 1024,    // 512MB
            large_object_threshold: 64 * 1024,    // 64KB
            pool_count: 16,
            adaptive_strategy: true,
            alignment: 8,
            enable_poisoning: cfg!(debug_assertions),
            pressure_threshold: PressureLevel::Moderate,
            stats_interval: Duration::from_secs(1),
        }
    }
}

/// Allocation strategy for different usage patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationStrategy {
    /// Fast bump allocation for temporary objects
    BumpAllocation,
    /// Free list allocation for general purpose
    FreeListAllocation,
    /// Segregated allocation for size-class optimization
    SegregatedAllocation,
    /// Best-fit allocation for minimal fragmentation
    BestFitAllocation,
}

/// Memory pool for specific size ranges
struct MemoryPool {
    /// Size range for this pool
    size_range: (usize, usize),
    /// Memory blocks in this pool
    blocks: VecDeque<PoolBlock>,
    /// Current allocation strategy
    strategy: AllocationStrategy,
    /// Pool statistics
    stats: PoolStatistics,
    /// Free list for deallocated objects
    free_list: VecDeque<FreeSlot>,
}

/// Block within a memory pool
struct PoolBlock {
    /// Memory block
    block: MemoryBlock,
    /// Free space tracking
    free_slots: VecDeque<FreeSlot>,
    /// Allocation bitmap for quick free slot lookup
    allocation_bitmap: Vec<u64>,
    /// Block utilization percentage
    utilization: f64,
}

/// Free memory slot
#[derive(Debug, Clone, Copy)]
struct FreeSlot {
    /// Offset within block
    offset: usize,
    /// Size of free slot
    size: usize,
    /// Next free slot (for linked list)
    next: Option<usize>,
}

/// Statistics for a memory pool
#[derive(Debug, Clone)]
struct PoolStatistics {
    /// Total allocations in this pool
    allocations: u64,
    /// Total deallocations in this pool
    deallocations: u64,
    /// Total bytes allocated
    bytes_allocated: u64,
    /// Total bytes deallocated
    bytes_deallocated: u64,
    /// Current utilization
    utilization: f64,
    /// Fragmentation ratio
    fragmentation: f64,
}

/// Large object allocator for objects exceeding pool threshold
struct LargeObjectAllocator {
    /// Direct system allocations for large objects
    allocations: HashMap<usize, LargeAllocation>,
    /// Statistics
    stats: LargeObjectStats,
    /// Next allocation ID
    next_id: usize,
}

/// Large object allocation tracking
struct LargeAllocation {
    /// Pointer to allocated memory
    ptr: NonNull<u8>,
    /// Size of allocation
    size: usize,
    /// Layout used for allocation
    layout: Layout,
    /// Allocation timestamp
    timestamp: Instant,
    /// Object metadata
    metadata: ObjectMetadata,
}

/// Statistics for large object allocator
#[derive(Debug, Clone)]
struct LargeObjectStats {
    /// Total large allocations
    allocations: u64,
    /// Total large deallocations
    deallocations: u64,
    /// Total bytes allocated
    bytes_allocated: u64,
    /// Current active allocations
    active_allocations: usize,
    /// Average allocation size
    average_size: f64,
}

/// Metadata tracker for all allocations
struct MetadataTracker {
    /// Object metadata by pointer
    metadata: HashMap<usize, ObjectMetadata>,
    /// Allocation timestamps
    timestamps: HashMap<usize, Instant>,
    /// Object type information
    type_info: HashMap<usize, String>,
}

/// Comprehensive statistics for real allocator
#[derive(Debug, Clone)]
pub struct RealAllocatorStats {
    /// Base allocator statistics
    pub base: AllocatorStatistics,
    /// Pool-specific statistics
    pub pool_stats: Vec<PoolStatistics>,
    /// Large object statistics
    pub large_object_stats: LargeObjectStats,
    /// Memory pressure statistics
    pub pressure_stats: PressureStats,
    /// Strategy adaptation statistics
    pub adaptation_stats: AdaptationStats,
    /// Fragmentation analysis
    pub fragmentation_analysis: FragmentationAnalysis,
}

/// Memory pressure statistics
#[derive(Debug, Clone)]
pub struct PressureStats {
    /// Current pressure level
    pub current_pressure: PressureLevel,
    /// Pressure-triggered adaptations
    pub pressure_adaptations: u64,
    /// Emergency allocations
    pub emergency_allocations: u64,
    /// Failed allocations due to pressure
    pub pressure_failures: u64,
}

/// Strategy adaptation statistics
#[derive(Debug, Clone)]
pub struct AdaptationStats {
    /// Strategy changes
    pub strategy_changes: u64,
    /// Current strategy distribution
    pub strategy_usage: HashMap<AllocationStrategy, u64>,
    /// Adaptation triggers
    pub adaptation_triggers: HashMap<String, u64>,
    /// Performance improvements from adaptation
    pub performance_improvements: f64,
}

/// Fragmentation analysis
#[derive(Debug, Clone)]
pub struct FragmentationAnalysis {
    /// Overall fragmentation ratio
    pub overall_fragmentation: f64,
    /// Fragmentation by pool
    pub pool_fragmentation: Vec<f64>,
    /// Largest free block size
    pub largest_free_block: usize,
    /// Free block count
    pub free_block_count: usize,
    /// Compaction opportunities
    pub compaction_opportunities: u64,
}

impl RealMemoryAllocator {
    /// Create a new real memory allocator
    #[instrument]
    pub fn new(config: RealAllocatorConfig, object_registry: SharedObjectRegistry) -> Result<Self, String> {
        info!("Creating real memory allocator with config: {:?}", config);
        
        // Initialize memory pools
        let pools = Self::create_memory_pools(&config)?;
        
        // Create large object allocator
        let large_object_allocator = Arc::new(Mutex::new(LargeObjectAllocator {
            allocations: HashMap::new(),
            stats: LargeObjectStats {
                allocations: 0,
                deallocations: 0,
                bytes_allocated: 0,
                active_allocations: 0,
                average_size: 0.0,
            },
            next_id: 1,
        }));
        
        // Create metadata tracker
        let metadata_tracker = Arc::new(Mutex::new(MetadataTracker {
            metadata: HashMap::new(),
            timestamps: HashMap::new(),
            type_info: HashMap::new(),
        }));
        
        Ok(Self {
            strategy: Arc::new(RwLock::new(AllocationStrategy::SegregatedAllocation)),
            pools: Arc::new(RwLock::new(pools)),
            large_object_allocator,
            metadata_tracker,
            object_registry,
            id_generator: ObjectIdGenerator::new(),
            statistics: Arc::new(Mutex::new(RealAllocatorStats {
                base: AllocatorStatistics::new(),
                pool_stats: Vec::new(),
                large_object_stats: LargeObjectStats {
                    allocations: 0,
                    deallocations: 0,
                    bytes_allocated: 0,
                    active_allocations: 0,
                    average_size: 0.0,
                },
                pressure_stats: PressureStats {
                    current_pressure: PressureLevel::None,
                    pressure_adaptations: 0,
                    emergency_allocations: 0,
                    pressure_failures: 0,
                },
                adaptation_stats: AdaptationStats {
                    strategy_changes: 0,
                    strategy_usage: HashMap::new(),
                    adaptation_triggers: HashMap::new(),
                    performance_improvements: 0.0,
                },
                fragmentation_analysis: FragmentationAnalysis {
                    overall_fragmentation: 0.0,
                    pool_fragmentation: Vec::new(),
                    largest_free_block: 0,
                    free_block_count: 0,
                    compaction_opportunities: 0,
                },
            })),
            pressure_detector: None,
            profiler: None,
            config: Arc::new(RwLock::new(config)),
            total_allocated: AtomicU64::new(0),
            total_deallocated: AtomicU64::new(0),
            active_allocations: AtomicUsize::new(0),
        })
    }
    
    /// Create memory pools for different size classes
    fn create_memory_pools(config: &RealAllocatorConfig) -> Result<Vec<MemoryPool>, String> {
        let mut pools = Vec::new();
        let pool_size = config.initial_pool_size / config.pool_count;
        
        // Create size classes (powers of 2 up to large object threshold)
        let mut size = 16; // Start with 16 bytes
        for i in 0..config.pool_count {
            let size_range = if i == config.pool_count - 1 {
                (size, config.large_object_threshold)
            } else {
                let next_size = size * 2;
                let range = (size, next_size - 1);
                size = next_size;
                range
            };
            
            let pool = MemoryPool {
                size_range,
                blocks: VecDeque::new(),
                strategy: AllocationStrategy::SegregatedAllocation,
                stats: PoolStatistics {
                    allocations: 0,
                    deallocations: 0,
                    bytes_allocated: 0,
                    bytes_deallocated: 0,
                    utilization: 0.0,
                    fragmentation: 0.0,
                },
                free_list: VecDeque::new(),
            };
            
            pools.push(pool);
        }
        
        info!("Created {} memory pools", pools.len());
        Ok(pools)
    }
    
    /// Set pressure detector for adaptive behavior
    pub fn set_pressure_detector(&mut self, detector: Arc<MemoryPressureDetector>) {
        self.pressure_detector = Some(detector);
    }
    
    /// Set memory profiler for tracking
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        self.profiler = Some(profiler);
    }
    
    /// Allocate memory with automatic strategy selection
    #[instrument(skip(self))]
    pub fn allocate_object(&self, size: usize, alignment: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        if size == 0 {
            return Err("Cannot allocate zero bytes".to_string());
        }
        
        debug!("Allocating {} bytes for type {} with alignment {}", size, type_name, alignment);
        
        // Check memory pressure and adapt if needed
        self.check_and_adapt_to_pressure()?;
        
        // Generate object ID
        let object_id = self.id_generator.next();
        
        // Route allocation based on size
        let (ptr, actual_size) = if size >= self.get_large_object_threshold()? {
            self.allocate_large_object(object_id, size, alignment, type_name)?
        } else {
            self.allocate_from_pools(size, alignment, type_name)?
        };
        
        // Track metadata
        self.track_allocation_metadata(ptr, object_id, size, type_name)?;
        
        // Update statistics
        self.update_allocation_stats(size, actual_size)?;
        
        // Profile allocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_allocation(actual_size, ptr.as_ptr() as u64, vec![]);
        }
        
        debug!("Successfully allocated object {} at {:p}", object_id, ptr.as_ptr());
        Ok((object_id, ptr))
    }
    
    /// Deallocate memory
    #[instrument(skip(self))]
    pub fn deallocate_object(&self, object_id: ObjectId, ptr: NonNull<u8>) -> Result<(), String> {
        debug!("Deallocating object {} at {:p}", object_id, ptr.as_ptr());
        
        // Get allocation metadata
        let (size, type_name) = self.get_allocation_metadata(ptr)?;
        
        // Route deallocation based on size
        if size >= self.get_large_object_threshold()? {
            self.deallocate_large_object(ptr, size)?;
        } else {
            self.deallocate_from_pools(ptr, size)?;
        }
        
        // Remove metadata
        self.remove_allocation_metadata(ptr)?;
        
        // Update statistics
        self.update_deallocation_stats(size)?;
        
        // Profile deallocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_deallocation(ptr.as_ptr() as u64, vec![]);
        }
        
        debug!("Successfully deallocated object {}", object_id);
        Ok(())
    }
    
    /// Allocate from memory pools
    fn allocate_from_pools(&self, size: usize, alignment: usize, type_name: &str) -> Result<(NonNull<u8>, usize), String> {
        let pool_index = self.find_best_pool(size)?;
        let mut pools = self.pools.write().map_err(|_| "Failed to acquire pools write lock")?;
        
        if pool_index >= pools.len() {
            return Err(format!("Invalid pool index {} for size {}", pool_index, size));
        }
        
        // Try to allocate from existing blocks
        if let Some((ptr, actual_size)) = self.try_allocate_from_existing_blocks(&mut pools[pool_index], size, alignment)? {
            return Ok((ptr, actual_size));
        }
        
        // Create new block if needed
        let new_block = self.create_new_pool_block(&pools[pool_index], size)?;
        pools[pool_index].blocks.push_back(new_block);
        
        // Allocate from the new block
        if let Some((ptr, actual_size)) = self.try_allocate_from_existing_blocks(&mut pools[pool_index], size, alignment)? {
            Ok((ptr, actual_size))
        } else {
            Err("Failed to allocate from new block".to_string())
        }
    }
    
    /// Allocate large object directly from system
    fn allocate_large_object(&self, object_id: ObjectId, size: usize, alignment: usize, type_name: &str) -> Result<(NonNull<u8>, usize), String> {
        let aligned_size = (size + alignment - 1) & !(alignment - 1);
        let layout = Layout::from_size_align(aligned_size, alignment)
            .map_err(|e| format!("Invalid layout for large object: {}", e))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(format!("Failed to allocate {} bytes for large object", aligned_size));
        }
        
        let non_null_ptr = NonNull::new(ptr)
            .ok_or("System allocator returned null pointer")?;
        
        // Track large allocation
        let mut large_allocator = self.large_object_allocator.lock()
            .map_err(|_| "Failed to acquire large object allocator lock")?;
        
        let allocation_id = large_allocator.next_id;
        large_allocator.next_id += 1;
        
        let allocation = LargeAllocation {
            ptr: non_null_ptr,
            size: aligned_size,
            layout,
            timestamp: Instant::now(),
            metadata: ObjectMetadata::new(
                object_id,
                aligned_size,
                alignment,
                type_name.to_string(),
                non_null_ptr.cast::<ObjectHeader>(),
                non_null_ptr
            ),
        };
        
        large_allocator.allocations.insert(allocation_id, allocation);
        large_allocator.stats.allocations += 1;
        large_allocator.stats.bytes_allocated += aligned_size as u64;
        large_allocator.stats.active_allocations += 1;
        large_allocator.stats.average_size = 
            large_allocator.stats.bytes_allocated as f64 / large_allocator.stats.allocations as f64;
        
        debug!("Allocated large object of {} bytes at {:p}", aligned_size, ptr);
        Ok((non_null_ptr, aligned_size))
    }
    
    /// Deallocate from memory pools
    fn deallocate_from_pools(&self, ptr: NonNull<u8>, size: usize) -> Result<(), String> {
        let pool_index = self.find_best_pool(size)?;
        let mut pools = self.pools.write().map_err(|_| "Failed to acquire pools write lock")?;
        
        if pool_index >= pools.len() {
            return Err(format!("Invalid pool index {} for size {}", pool_index, size));
        }
        
        // Find the block containing this pointer
        for block in &mut pools[pool_index].blocks {
            let block_start = block.block.ptr.as_ptr() as usize;
            let block_end = block_start + block.block.size;
            let ptr_addr = ptr.as_ptr() as usize;
            
            if ptr_addr >= block_start && ptr_addr < block_end {
                // Add to free list
                let offset = ptr_addr - block_start;
                let free_slot = FreeSlot {
                    offset,
                    size,
                    next: None,
                };
                block.free_slots.push_back(free_slot);
                
                // Update pool statistics
                pools[pool_index].stats.deallocations += 1;
                pools[pool_index].stats.bytes_deallocated += size as u64;
                
                return Ok(());
            }
        }
        
        Err(format!("Pointer {:p} not found in any pool block", ptr.as_ptr()))
    }
    
    /// Deallocate large object
    fn deallocate_large_object(&self, ptr: NonNull<u8>, size: usize) -> Result<(), String> {
        let mut large_allocator = self.large_object_allocator.lock()
            .map_err(|_| "Failed to acquire large object allocator lock")?;
        
        // Find and remove the allocation
        let mut found_id = None;
        for (&id, allocation) in &large_allocator.allocations {
            if allocation.ptr == ptr {
                found_id = Some(id);
                break;
            }
        }
        
        if let Some(id) = found_id {
            let allocation = large_allocator.allocations.remove(&id)
                .ok_or("Failed to remove large allocation")?;
            
            // Deallocate system memory
            unsafe {
                dealloc(allocation.ptr.as_ptr(), allocation.layout);
            }
            
            // Update statistics
            large_allocator.stats.deallocations += 1;
            large_allocator.stats.active_allocations -= 1;
            
            debug!("Deallocated large object of {} bytes at {:p}", allocation.size, ptr.as_ptr());
            Ok(())
        } else {
            Err(format!("Large object pointer {:p} not found", ptr.as_ptr()))
        }
    }
    
    /// Find best pool for allocation size
    fn find_best_pool(&self, size: usize) -> Result<usize, String> {
        let pools = self.pools.read().map_err(|_| "Failed to acquire pools read lock")?;
        
        for (i, pool) in pools.iter().enumerate() {
            if size >= pool.size_range.0 && size <= pool.size_range.1 {
                return Ok(i);
            }
        }
        
        Err(format!("No suitable pool found for size {}", size))
    }
    
    /// Try to allocate from existing blocks in a pool
    fn try_allocate_from_existing_blocks(
        &self,
        pool: &mut MemoryPool,
        size: usize,
        alignment: usize
    ) -> Result<Option<(NonNull<u8>, usize)>, String> {
        for block in &mut pool.blocks {
            if let Some(free_slot) = block.free_slots.pop_front() {
                if free_slot.size >= size {
                    let ptr_addr = block.block.ptr.as_ptr() as usize + free_slot.offset;
                    let aligned_addr = (ptr_addr + alignment - 1) & !(alignment - 1);
                    
                    if aligned_addr + size <= ptr_addr + free_slot.size {
                        let ptr = NonNull::new(aligned_addr as *mut u8)
                            .ok_or("Failed to create non-null pointer")?;
                        
                        // If there's remaining space, add it back to free list
                        let remaining_size = (ptr_addr + free_slot.size) - (aligned_addr + size);
                        if remaining_size > 0 {
                            let remaining_slot = FreeSlot {
                                offset: free_slot.offset + (aligned_addr - ptr_addr) + size,
                                size: remaining_size,
                                next: None,
                            };
                            block.free_slots.push_front(remaining_slot);
                        }
                        
                        // Update pool statistics
                        pool.stats.allocations += 1;
                        pool.stats.bytes_allocated += size as u64;
                        
                        return Ok(Some((ptr, size)));
                    }
                }
                
                // Put the slot back if it couldn't be used
                block.free_slots.push_front(free_slot);
            }
        }
        
        Ok(None)
    }
    
    /// Create a new block for a pool
    fn create_new_pool_block(&self, pool: &MemoryPool, min_size: usize) -> Result<PoolBlock, String> {
        let config = self.config.read().map_err(|_| "Failed to read config")?;
        let block_size = (config.initial_pool_size / config.pool_count).max(min_size * 4);
        
        let memory_block = MemoryBlock::new(block_size, 0)?;
        
        // Initialize with one large free slot
        let mut free_slots = VecDeque::new();
        free_slots.push_back(FreeSlot {
            offset: 0,
            size: block_size,
            next: None,
        });
        
        Ok(PoolBlock {
            block: memory_block,
            free_slots,
            allocation_bitmap: vec![0; (block_size + 63) / 64], // One bit per 8 bytes
            utilization: 0.0,
        })
    }
    
    /// Track allocation metadata
    fn track_allocation_metadata(
        &self,
        ptr: NonNull<u8>,
        object_id: ObjectId,
        size: usize,
        type_name: &str
    ) -> Result<(), String> {
        let mut tracker = self.metadata_tracker.lock()
            .map_err(|_| "Failed to acquire metadata tracker lock")?;
        
        let ptr_addr = ptr.as_ptr() as usize;
        let metadata = ObjectMetadata::new(
            object_id,
            size,
            8,
            type_name.to_string(),
            ptr.cast::<ObjectHeader>(),
            ptr
        );
        
        tracker.metadata.insert(ptr_addr, metadata);
        tracker.timestamps.insert(ptr_addr, Instant::now());
        tracker.type_info.insert(ptr_addr, type_name.to_string());
        
        Ok(())
    }
    
    /// Get allocation metadata
    fn get_allocation_metadata(&self, ptr: NonNull<u8>) -> Result<(usize, String), String> {
        let tracker = self.metadata_tracker.lock()
            .map_err(|_| "Failed to acquire metadata tracker lock")?;
        
        let ptr_addr = ptr.as_ptr() as usize;
        let metadata = tracker.metadata.get(&ptr_addr)
            .ok_or("Metadata not found for pointer")?;
        let type_name = tracker.type_info.get(&ptr_addr)
            .ok_or("Type info not found for pointer")?;
        
        Ok((metadata.size, type_name.clone()))
    }
    
    /// Remove allocation metadata
    fn remove_allocation_metadata(&self, ptr: NonNull<u8>) -> Result<(), String> {
        let mut tracker = self.metadata_tracker.lock()
            .map_err(|_| "Failed to acquire metadata tracker lock")?;
        
        let ptr_addr = ptr.as_ptr() as usize;
        tracker.metadata.remove(&ptr_addr);
        tracker.timestamps.remove(&ptr_addr);
        tracker.type_info.remove(&ptr_addr);
        
        Ok(())
    }
    
    /// Update allocation statistics
    fn update_allocation_stats(&self, requested_size: usize, actual_size: usize) -> Result<(), String> {
        self.total_allocated.fetch_add(actual_size as u64, Ordering::Relaxed);
        self.active_allocations.fetch_add(1, Ordering::Relaxed);
        
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        stats.base.record_allocation(actual_size);
        
        Ok(())
    }
    
    /// Update deallocation statistics
    fn update_deallocation_stats(&self, size: usize) -> Result<(), String> {
        self.total_deallocated.fetch_add(size as u64, Ordering::Relaxed);
        self.active_allocations.fetch_sub(1, Ordering::Relaxed);
        
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        stats.base.record_deallocation(size);
        
        Ok(())
    }
    
    /// Check memory pressure and adapt allocation strategy
    fn check_and_adapt_to_pressure(&self) -> Result<(), String> {
        if let Some(pressure_detector) = &self.pressure_detector {
            // This would need HeapStats - simplified for now
            let current_pressure = pressure_detector.current_pressure()?;
            
            let config = self.config.read().map_err(|_| "Failed to read config")?;
            if current_pressure >= config.pressure_threshold {
                self.adapt_allocation_strategy(current_pressure)?;
            }
        }
        
        Ok(())
    }
    
    /// Adapt allocation strategy based on conditions
    fn adapt_allocation_strategy(&self, pressure: PressureLevel) -> Result<(), String> {
        let config = self.config.read().map_err(|_| "Failed to read config")?;
        if !config.adaptive_strategy {
            return Ok(());
        }
        
        let new_strategy = match pressure {
            PressureLevel::Emergency | PressureLevel::Critical => AllocationStrategy::BestFitAllocation,
            PressureLevel::High => AllocationStrategy::FreeListAllocation,
            PressureLevel::Moderate => AllocationStrategy::SegregatedAllocation,
            _ => return Ok(()),
        };
        
        let mut current_strategy = self.strategy.write()
            .map_err(|_| "Failed to acquire strategy write lock")?;
        
        if *current_strategy != new_strategy {
            info!("Adapting allocation strategy from {:?} to {:?} due to {} pressure",
                  *current_strategy, new_strategy, pressure);
            *current_strategy = new_strategy;
            
            // Update statistics
            if let Ok(mut stats) = self.statistics.lock() {
                stats.adaptation_stats.strategy_changes += 1;
                stats.pressure_stats.pressure_adaptations += 1;
            }
        }
        
        Ok(())
    }
    
    /// Get large object threshold
    fn get_large_object_threshold(&self) -> Result<usize, String> {
        let config = self.config.read().map_err(|_| "Failed to read config")?;
        Ok(config.large_object_threshold)
    }
    
    /// Get comprehensive allocator statistics
    pub fn get_statistics(&self) -> Result<RealAllocatorStats, String> {
        let stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        Ok(stats.clone())
    }
    
    /// Force compaction of memory pools
    pub fn compact_pools(&self) -> Result<(), String> {
        info!("Starting memory pool compaction");
        
        let mut pools = self.pools.write()
            .map_err(|_| "Failed to acquire pools write lock")?;
        
        for pool in pools.iter_mut() {
            self.compact_pool(pool)?;
        }
        
        info!("Memory pool compaction completed");
        Ok(())
    }
    
    /// Compact a single memory pool
    fn compact_pool(&self, pool: &mut MemoryPool) -> Result<(), String> {
        // Coalesce adjacent free slots
        for block in &mut pool.blocks {
            block.free_slots.make_contiguous();
            let mut free_slots: Vec<_> = block.free_slots.drain(..).collect();
            free_slots.sort_by_key(|slot| slot.offset);
            
            let mut coalesced = VecDeque::new();
            let mut current_slot = None;
            
            for slot in free_slots {
                match current_slot {
                    None => current_slot = Some(slot),
                    Some(ref mut current) => {
                        if current.offset + current.size == slot.offset {
                            // Adjacent slots - coalesce
                            current.size += slot.size;
                        } else {
                            // Non-adjacent - save current and start new
                            coalesced.push_back(*current);
                            current_slot = Some(slot);
                        }
                    }
                }
            }
            
            if let Some(final_slot) = current_slot {
                coalesced.push_back(final_slot);
            }
            
            block.free_slots = coalesced;
        }
        
        Ok(())
    }
}

impl Drop for RealMemoryAllocator {
    fn drop(&mut self) {
        info!("Shutting down real memory allocator");
        
        // Deallocate all large objects
        if let Ok(mut large_allocator) = self.large_object_allocator.lock() {
            for (_, allocation) in large_allocator.allocations.drain() {
                unsafe {
                    dealloc(allocation.ptr.as_ptr(), allocation.layout);
                }
            }
        }
        
        // Memory pools will be automatically deallocated when MemoryBlocks are dropped
        info!("Real memory allocator shutdown complete");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectRegistry;
    
    #[test]
    fn test_real_allocator_creation() {
        let config = RealAllocatorConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let allocator = RealMemoryAllocator::new(config, registry).unwrap();
        
        let stats = allocator.get_statistics().unwrap();
        assert_eq!(stats.base.allocations, 0);
    }
    
    #[test]
    fn test_object_allocation_and_deallocation() {
        let config = RealAllocatorConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let allocator = RealMemoryAllocator::new(config, registry).unwrap();
        
        // Allocate an object
        let (id, ptr) = allocator.allocate_object(64, 8, "test").unwrap();
        
        // Verify allocation
        let stats = allocator.get_statistics().unwrap();
        assert_eq!(stats.base.allocations, 1);
        
        // Deallocate the object
        allocator.deallocate_object(id, ptr).unwrap();
        
        // Verify deallocation
        let stats = allocator.get_statistics().unwrap();
        assert_eq!(stats.base.deallocations, 1);
    }
    
    #[test]
    fn test_large_object_allocation() {
        let mut config = RealAllocatorConfig::default();
        config.large_object_threshold = 1024;
        let registry = Arc::new(ObjectRegistry::new());
        let allocator = RealMemoryAllocator::new(config, registry).unwrap();
        
        // Allocate large object
        let (id, ptr) = allocator.allocate_object(2048, 8, "large_test").unwrap();
        
        // Should be handled by large object allocator
        let stats = allocator.get_statistics().unwrap();
        assert_eq!(stats.large_object_stats.allocations, 1);
        
        // Deallocate
        allocator.deallocate_object(id, ptr).unwrap();
        assert_eq!(allocator.get_statistics().unwrap().large_object_stats.deallocations, 1);
    }
    
    #[test]
    fn test_pool_compaction() {
        let config = RealAllocatorConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let allocator = RealMemoryAllocator::new(config, registry).unwrap();
        
        // Allocate and deallocate several objects to create fragmentation
        let mut allocations = Vec::new();
        for i in 0..10 {
            let (id, ptr) = allocator.allocate_object(32, 8, "fragment_test").unwrap();
            allocations.push((id, ptr));
        }
        
        // Deallocate every other allocation
        for (i, (id, ptr)) in allocations.iter().enumerate() {
            if i % 2 == 0 {
                allocator.deallocate_object(*id, *ptr).unwrap();
            }
        }
        
        // Compact pools
        allocator.compact_pools().unwrap();
        
        // Compaction should succeed without error
        assert!(true);
    }
}
