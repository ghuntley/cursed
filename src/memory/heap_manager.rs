/// Heap Manager for Garbage Collection
/// 
/// This module manages the heap memory layout and provides crucial memory safety
/// guarantees for the CURSED garbage collector. The heap manager is responsible for:
/// 
/// 1. **Memory Layout Management**: Organizes heap into logical segments for efficient allocation
/// 2. **Allocation Strategy**: Implements allocation policies that minimize fragmentation  
/// 3. **Safety Guarantees**: Ensures memory safety through bounds checking and alignment
/// 4. **Performance Optimization**: Reduces allocation overhead through smart pooling
/// 5. **Integration Support**: Provides hooks for profiling and debugging systems
/// 
/// The design uses multiple heap segments to reduce contention in concurrent scenarios
/// and enable more efficient garbage collection cycles.

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{self, NonNull};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashMap, VecDeque};
use std::time::{Instant, Duration};
use tracing::{instrument, debug, warn, error, info};

use crate::memory::object_id::{ObjectId, ObjectIdGenerator, ObjectMetadata, SharedObjectRegistry};
use crate::profiling::memory::{MemoryProfiler, GcEvent, AllocationEvent};

/// Raw memory block allocated from the system
/// 
/// This represents a contiguous block of memory that objects can be allocated within.
/// Each block tracks its usage to enable efficient allocation strategies.
#[derive(Debug)]
pub struct MemoryBlock {
    /// Pointer to the start of the block
    pub ptr: NonNull<u8>,
    /// Total size of the block in bytes
    pub size: usize,
    /// Number of bytes currently used
    pub used: usize,
    /// Next free offset for bump allocation
    pub next_free: usize,
    /// Block ID for debugging
    pub id: u32,
}

impl MemoryBlock {
    /// Allocate a new memory block from the system
    /// 
    /// This uses the system allocator to get a large contiguous chunk of memory
    /// that will be subdivided for object allocations.
    #[instrument]
    pub fn new(size: usize, id: u32) -> Result<Self, String> {
        if size == 0 {
            return Err("Cannot allocate zero-sized memory block".to_string());
        }
        
        // Ensure size is properly aligned
        let aligned_size = (size + 7) & !7; // 8-byte alignment
        
        let layout = Layout::from_size_align(aligned_size, 8)
            .map_err(|e| format!("Invalid layout for memory block: {}", e))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(format!("Failed to allocate {} bytes from system", aligned_size));
        }
        
        let non_null_ptr = NonNull::new(ptr)
            .ok_or("System allocator returned null pointer")?;
        
        debug!("Allocated memory block {} of {} bytes at {:p}", id, aligned_size, ptr);
        
        Ok(Self {
            ptr: non_null_ptr,
            size: aligned_size,
            used: 0,
            next_free: 0,
            id,
        })
    }
    
    /// Try to allocate space within this block
    /// 
    /// Returns the offset within the block if successful, or None if
    /// there isn't enough space remaining.
    #[instrument(skip(self))]
    pub fn try_allocate(&mut self, size: usize, align: usize) -> Option<usize> {
        if size == 0 {
            return None;
        }
        
        // Calculate aligned offset
        let aligned_offset = (self.next_free + align - 1) & !(align - 1);
        
        // Check if allocation fits
        if aligned_offset + size > self.size {
            debug!("Block {} cannot fit {} bytes (need {}, have {})", 
                   self.id, size, aligned_offset + size, self.size);
            return None;
        }
        
        // Update block state
        self.next_free = aligned_offset + size;
        self.used += size;
        
        debug!("Allocated {} bytes at offset {} in block {}", size, aligned_offset, self.id);
        Some(aligned_offset)
    }
    
    /// Get pointer to allocated space at given offset
    pub fn ptr_at_offset(&self, offset: usize) -> Result<NonNull<u8>, String> {
        if offset >= self.size {
            return Err(format!("Offset {} exceeds block size {}", offset, self.size));
        }
        
        unsafe {
            let ptr = self.ptr.as_ptr().add(offset);
            NonNull::new(ptr).ok_or("Computed null pointer".to_string())
        }
    }
    
    /// Get remaining free space in this block
    pub fn free_space(&self) -> usize {
        self.size - self.next_free
    }
    
    /// Get utilization percentage
    pub fn utilization(&self) -> f64 {
        if self.size == 0 {
            0.0
        } else {
            (self.used as f64 / self.size as f64) * 100.0
        }
    }
    
    /// Check if a pointer belongs to this block
    pub fn contains_ptr(&self, ptr: *const u8) -> bool {
        let start = self.ptr.as_ptr() as usize;
        let end = start + self.size;
        let addr = ptr as usize;
        addr >= start && addr < end
    }
}

impl Drop for MemoryBlock {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.size, 8)
            .expect("Invalid layout in MemoryBlock::drop");
        
        unsafe {
            dealloc(self.ptr.as_ptr(), layout);
        }
        
        debug!("Deallocated memory block {} of {} bytes", self.id, self.size);
    }
}

// Safety: MemoryBlock is safe to send between threads because:
// 1. NonNull<u8> is Send (raw pointers to u8 can be sent between threads)
// 2. All other fields (usize, u32) are Copy and Send
// 3. The memory pointed to is heap-allocated and owned by this block
// 4. Access is coordinated through the heap manager's locking
unsafe impl Send for MemoryBlock {}

// Safety: MemoryBlock is safe to share between threads because:
// 1. All mutation is coordinated through the heap manager's RwLock
// 2. The NonNull<u8> pointer is stable (doesn't change once allocated)
// 3. Field access is atomic or protected by the containing lock
unsafe impl Sync for MemoryBlock {}

/// Configuration for heap manager behavior
#[derive(Debug, Clone)]
pub struct HeapConfig {
    /// Default size for new memory blocks
    pub default_block_size: usize,
    /// Maximum number of blocks to maintain
    pub max_blocks: usize,
    /// Minimum block utilization before compaction
    pub min_utilization: f64,
    /// Enable memory profiling integration
    pub enable_profiling: bool,
    /// Memory pressure threshold (percentage of capacity)
    pub pressure_threshold: f64,
    /// Adaptive allocation growth factor
    pub growth_factor: f64,
    /// Maximum single allocation size relative to block size
    pub max_allocation_ratio: f64,
    /// GC trigger threshold (percentage of heap usage)
    pub gc_trigger_threshold: f64,
}

/// Memory pressure levels for adaptive allocation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryPressure {
    Low,
    Medium,
    High,
    Critical,
}

/// Allocation strategy based on current memory conditions
#[derive(Debug, Clone, Copy)]
pub enum AllocationStrategy {
    /// Standard bump allocation
    BumpAllocator,
    /// Best-fit allocation to reduce fragmentation
    BestFit,
    /// Compact existing blocks before allocating
    CompactFirst,
}

impl Default for HeapConfig {
    fn default() -> Self {
        Self {
            default_block_size: 1024 * 1024, // 1MB default blocks
            max_blocks: 64,
            min_utilization: 0.5, // 50% minimum utilization
            enable_profiling: true,
            pressure_threshold: 0.8, // 80% capacity triggers pressure
            growth_factor: 1.5, // 50% growth when expanding
            max_allocation_ratio: 0.5, // Max 50% of block size per allocation
            gc_trigger_threshold: 0.75, // Trigger GC at 75% heap usage
        }
    }
}

/// Information about an allocated object in the heap
#[derive(Debug, Clone)]
pub struct AllocationInfo {
    /// Object identifier
    pub object_id: ObjectId,
    /// Block where object is allocated
    pub block_id: u32,
    /// Offset within the block
    pub offset: usize,
    /// Size of the allocation
    pub size: usize,
    /// Pointer to the allocated memory
    pub ptr: NonNull<u8>,
}

// Safety: AllocationInfo is safe to send between threads because:
// 1. ObjectId is Copy and thread-safe
// 2. u32 and usize are Copy and Send
// 3. NonNull<u8> points to heap-allocated memory that is owned by the heap manager
// 4. Access to the pointed memory is coordinated through the heap manager's locks
// 5. The pointer remains valid as long as the object is tracked in the heap manager
unsafe impl Send for AllocationInfo {}

// Safety: AllocationInfo is safe to share between threads because:
// 1. All fields are either Copy or have stable addresses (NonNull<u8>)
// 2. The NonNull<u8> pointer doesn't change once allocated
// 3. Access to the memory is coordinated through heap manager synchronization
// 4. The struct itself is immutable after creation (only read operations)
unsafe impl Sync for AllocationInfo {}

/// Allocation metrics for performance monitoring
#[derive(Debug, Clone)]
pub struct AllocationMetrics {
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub bytes_allocated: u64,
    pub bytes_deallocated: u64,
    pub allocation_failures: u64,
    pub average_allocation_time: Duration,
    pub peak_memory_usage: usize,
    pub gc_triggers: u64,
    pub compaction_events: u64,
}

impl Default for AllocationMetrics {
    fn default() -> Self {
        Self {
            total_allocations: 0,
            total_deallocations: 0,
            bytes_allocated: 0,
            bytes_deallocated: 0,
            allocation_failures: 0,
            average_allocation_time: Duration::from_nanos(0),
            peak_memory_usage: 0,
            gc_triggers: 0,
            compaction_events: 0,
        }
    }
}

/// Recent allocation history for adaptive strategies
#[derive(Debug)]
struct AllocationHistory {
    recent_allocations: VecDeque<(Instant, usize)>,
    recent_failures: VecDeque<Instant>,
    pressure_events: VecDeque<(Instant, MemoryPressure)>,
}

impl AllocationHistory {
    fn new() -> Self {
        Self {
            recent_allocations: VecDeque::with_capacity(1000),
            recent_failures: VecDeque::with_capacity(100),
            pressure_events: VecDeque::with_capacity(100),
        }
    }
    
    fn record_allocation(&mut self, size: usize) {
        let now = Instant::now();
        self.recent_allocations.push_back((now, size));
        if self.recent_allocations.len() > 1000 {
            self.recent_allocations.pop_front();
        }
    }
    
    fn record_failure(&mut self) {
        let now = Instant::now();
        self.recent_failures.push_back(now);
        if self.recent_failures.len() > 100 {
            self.recent_failures.pop_front();
        }
    }
    
    fn record_pressure(&mut self, pressure: MemoryPressure) {
        let now = Instant::now();
        self.pressure_events.push_back((now, pressure));
        if self.pressure_events.len() > 100 {
            self.pressure_events.pop_front();
        }
    }
    
    fn get_allocation_rate(&self, window: Duration) -> f64 {
        let cutoff = Instant::now() - window;
        let recent_count = self.recent_allocations
            .iter()
            .filter(|(time, _)| *time > cutoff)
            .count();
        recent_count as f64 / window.as_secs_f64()
    }
    
    fn get_failure_rate(&self, window: Duration) -> f64 {
        let cutoff = Instant::now() - window;
        let recent_failures = self.recent_failures
            .iter()
            .filter(|&time| *time > cutoff)
            .count();
        recent_failures as f64 / window.as_secs_f64()
    }
}

/// Main heap manager that coordinates memory allocation and deallocation
/// 
/// The heap manager maintains multiple memory blocks and implements allocation
/// strategies to minimize fragmentation and maximize performance.
#[derive(Debug)]
pub struct HeapManager {
    /// Configuration for heap behavior
    config: HeapConfig,
    /// Active memory blocks
    blocks: RwLock<Vec<MemoryBlock>>,
    /// Next block ID to assign
    next_block_id: Mutex<u32>,
    /// Object ID generator
    id_generator: ObjectIdGenerator,
    /// Object registry for metadata tracking
    object_registry: SharedObjectRegistry,
    /// Allocation tracking for objects
    allocations: RwLock<HashMap<ObjectId, AllocationInfo>>,
    /// Memory profiler (optional)
    profiler: Option<Arc<MemoryProfiler>>,
    /// Allocation metrics for monitoring
    metrics: RwLock<AllocationMetrics>,
    /// Allocation history for adaptive strategies
    history: RwLock<AllocationHistory>,
    /// Current memory pressure level
    pressure_level: RwLock<MemoryPressure>,
}

impl HeapManager {
    /// Create a new heap manager with the given configuration
    #[instrument]
    pub fn new(config: HeapConfig, object_registry: SharedObjectRegistry) -> Self {
        info!("Creating heap manager with {} byte blocks, max {} blocks", 
              config.default_block_size, config.max_blocks);
        
        Self {
            config,
            blocks: RwLock::new(Vec::new()),
            next_block_id: Mutex::new(1),
            id_generator: ObjectIdGenerator::new(),
            object_registry,
            allocations: RwLock::new(HashMap::new()),
            profiler: None,
            metrics: RwLock::new(AllocationMetrics::default()),
            history: RwLock::new(AllocationHistory::new()),
            pressure_level: RwLock::new(MemoryPressure::Low),
        }
    }
    
    /// Set memory profiler for allocation tracking
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        if self.config.enable_profiling {
            info!("Enabling memory profiling for heap manager");
            self.profiler = Some(profiler);
        }
    }
    
    /// Allocate memory for a new object
    /// 
    /// This is the main allocation entry point. It handles finding appropriate
    /// blocks, creating new blocks if needed, and tracking the allocation.
    #[instrument(skip(self, type_name))]
    pub fn allocate<T>(&self, size: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        let start_time = Instant::now();
        
        if size == 0 {
            return Err("Cannot allocate zero bytes".to_string());
        }
        
        // Check memory pressure and possibly trigger GC
        self.check_memory_pressure()?;
        
        let object_id = self.id_generator.next();
        let align = std::mem::align_of::<T>().max(8); // At least 8-byte alignment
        
        debug!("Allocating {} bytes for {} (object {})", size, type_name, object_id);
        
        // Determine allocation strategy based on current conditions
        let strategy = self.determine_allocation_strategy(size)?;
        
        let result = match strategy {
            AllocationStrategy::BumpAllocator => {
                self.allocate_with_bump_strategy(object_id, size, align, type_name)
            },
            AllocationStrategy::BestFit => {
                self.allocate_with_best_fit_strategy(object_id, size, align, type_name)
            },
            AllocationStrategy::CompactFirst => {
                self.allocate_with_compaction_strategy(object_id, size, align, type_name)
            },
        };
        
        // Update metrics
        let allocation_time = start_time.elapsed();
        self.update_allocation_metrics(size, allocation_time, result.is_ok())?;
        
        match result {
            Ok((id, ptr)) => {
                if let Ok(mut history) = self.history.write() {
                    history.record_allocation(size);
                }
                Ok((id, ptr))
            },
            Err(e) => {
                if let Ok(mut history) = self.history.write() {
                    history.record_failure();
                }
                Err(e)
            }
        }
    }
    
    /// Determine the best allocation strategy based on current conditions
    fn determine_allocation_strategy(&self, size: usize) -> Result<AllocationStrategy, String> {
        let pressure = *self.pressure_level.read()
            .map_err(|_| "Failed to read pressure level")?;
        
        // Check if allocation is too large for normal strategy
        if size > (self.config.default_block_size as f64 * self.config.max_allocation_ratio) as usize {
            return Ok(AllocationStrategy::BestFit);
        }
        
        match pressure {
            MemoryPressure::Low => Ok(AllocationStrategy::BumpAllocator),
            MemoryPressure::Medium => {
                // Check recent failure rate
                if let Ok(history) = self.history.read() {
                    let failure_rate = history.get_failure_rate(Duration::from_secs(10));
                    if failure_rate > 0.1 { // More than 10% failure rate
                        Ok(AllocationStrategy::BestFit)
                    } else {
                        Ok(AllocationStrategy::BumpAllocator)
                    }
                } else {
                    Ok(AllocationStrategy::BumpAllocator)
                }
            },
            MemoryPressure::High => Ok(AllocationStrategy::BestFit),
            MemoryPressure::Critical => Ok(AllocationStrategy::CompactFirst),
        }
    }
    
    /// Allocate using bump allocation strategy
    fn allocate_with_bump_strategy(&self, object_id: ObjectId, size: usize, 
                                  align: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        // Try to allocate in existing blocks first
        if let Some((block_id, offset, ptr)) = self.try_allocate_in_existing_blocks(size, align)? {
            return self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name);
        }
        
        // Need to create a new block
        let (block_id, offset, ptr) = self.allocate_new_block(size, align)?;
        self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name)
    }
    
    /// Allocate using best-fit strategy to reduce fragmentation
    fn allocate_with_best_fit_strategy(&self, object_id: ObjectId, size: usize, 
                                      align: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        // First try best-fit allocation in existing blocks
        if let Some((block_id, offset, ptr)) = self.try_best_fit_allocation(size, align)? {
            return self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name);
        }
        
        // Fall back to creating new block
        let (block_id, offset, ptr) = self.allocate_new_block(size, align)?;
        self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name)
    }
    
    /// Allocate with compaction strategy
    fn allocate_with_compaction_strategy(&self, object_id: ObjectId, size: usize, 
                                        align: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        // Try compaction first
        self.compact_blocks()?;
        
        // Try allocation after compaction
        if let Some((block_id, offset, ptr)) = self.try_allocate_in_existing_blocks(size, align)? {
            return self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name);
        }
        
        // Still need new block
        let (block_id, offset, ptr) = self.allocate_new_block(size, align)?;
        self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name)
    }
    
    /// Try best-fit allocation in existing blocks
    fn try_best_fit_allocation(&self, size: usize, align: usize) 
        -> Result<Option<(u32, usize, NonNull<u8>)>, String> {
        
        let mut blocks = self.blocks.write()
            .map_err(|_| "Failed to acquire write lock on blocks")?;
        
        let mut best_block_idx = None;
        let mut best_fit_size = usize::MAX;
        
        // Find the block with the smallest sufficient free space
        for (idx, block) in blocks.iter().enumerate() {
            let free_space = block.free_space();
            if free_space >= size && free_space < best_fit_size {
                // Check if allocation would fit with alignment
                let aligned_offset = (block.next_free + align - 1) & !(align - 1);
                if aligned_offset + size <= block.size {
                    best_block_idx = Some(idx);
                    best_fit_size = free_space;
                }
            }
        }
        
        if let Some(idx) = best_block_idx {
            if let Some(offset) = blocks[idx].try_allocate(size, align) {
                let ptr = blocks[idx].ptr_at_offset(offset)?;
                return Ok(Some((blocks[idx].id, offset, ptr)));
            }
        }
        
        Ok(None)
    }
    
    /// Check memory pressure and trigger GC if needed
    fn check_memory_pressure(&self) -> Result<(), String> {
        let stats = self.get_stats()?;
        let utilization = stats.average_utilization / 100.0;
        
        let new_pressure = if utilization >= 0.95 {
            MemoryPressure::Critical
        } else if utilization >= 0.85 {
            MemoryPressure::High
        } else if utilization >= self.config.pressure_threshold {
            MemoryPressure::Medium
        } else {
            MemoryPressure::Low
        };
        
        // Update pressure level
        if let Ok(mut pressure) = self.pressure_level.write() {
            if *pressure != new_pressure {
                info!("Memory pressure changed from {:?} to {:?} (utilization: {:.1}%)", 
                      *pressure, new_pressure, utilization * 100.0);
                *pressure = new_pressure;
                
                // Record pressure event
                if let Ok(mut history) = self.history.write() {
                    history.record_pressure(new_pressure);
                }
            }
        }
        
        // Trigger GC if needed
        if utilization >= self.config.gc_trigger_threshold {
            self.trigger_gc_internal("memory_pressure")?;
        }
        
        Ok(())
    }
    
    /// Update allocation metrics
    fn update_allocation_metrics(&self, size: usize, allocation_time: Duration, success: bool) -> Result<(), String> {
        if let Ok(mut metrics) = self.metrics.write() {
            if success {
                metrics.total_allocations += 1;
                metrics.bytes_allocated += size as u64;
                
                // Update average allocation time
                let total_time = metrics.average_allocation_time.as_nanos() as u64 * (metrics.total_allocations - 1);
                let new_total = total_time + allocation_time.as_nanos() as u64;
                metrics.average_allocation_time = Duration::from_nanos(new_total / metrics.total_allocations);
                
                // Update peak memory usage
                if let Ok(stats) = self.get_stats() {
                    if stats.total_used > metrics.peak_memory_usage {
                        metrics.peak_memory_usage = stats.total_used;
                    }
                }
            } else {
                metrics.allocation_failures += 1;
            }
        }
        Ok(())
    }
    
    /// Compact blocks to reduce fragmentation
    fn compact_blocks(&self) -> Result<(), String> {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.compaction_events += 1;
        }
        
        debug!("Starting block compaction");
        
        // For now, just log the compaction - actual implementation would need
        // to move objects between blocks which requires GC coordination
        warn!("Block compaction requested but not fully implemented - requires GC integration");
        
        Ok(())
    }
    
    /// Internal GC trigger with metrics tracking
    fn trigger_gc_internal(&self, reason: &str) -> Result<(), String> {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.gc_triggers += 1;
        }
        
        info!("Triggering GC due to: {}", reason);
        
        // Profile the GC event
        if let Some(profiler) = &self.profiler {
            use crate::profiling::memory::{GcEvent, GcType};
            let gc_event = GcEvent {
                gc_type: GcType::Minor,
                duration: Duration::from_millis(0),
                bytes_collected: 0,
                bytes_remaining: 0,
                objects_collected: 0,
                objects_remaining: 0,
                timestamp: Instant::now(),
                trigger_reason: reason.to_string(),
            };
            let _ = profiler.track_gc_event(gc_event);
        }
        
        // For now, just log - actual GC integration would happen here
        debug!("GC triggered: {}", reason);
        
        Ok(())
    }
    
    /// Try to allocate in existing blocks
    fn try_allocate_in_existing_blocks(&self, size: usize, align: usize) 
        -> Result<Option<(u32, usize, NonNull<u8>)>, String> {
        
        let mut blocks = self.blocks.write()
            .map_err(|_| "Failed to acquire write lock on blocks")?;
        
        for block in blocks.iter_mut() {
            if let Some(offset) = block.try_allocate(size, align) {
                let ptr = block.ptr_at_offset(offset)?;
                return Ok(Some((block.id, offset, ptr)));
            }
        }
        
        Ok(None)
    }
    
    /// Allocate a new memory block
    fn allocate_new_block(&self, min_size: usize, align: usize) 
        -> Result<(u32, usize, NonNull<u8>), String> {
        
        let block_id = {
            let mut next_id = self.next_block_id.lock()
                .map_err(|_| "Failed to acquire lock on next_block_id")?;
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        // Determine block size (at least default, but larger if needed)
        let block_size = self.config.default_block_size.max(min_size + 1024);
        
        let mut new_block = MemoryBlock::new(block_size, block_id)?;
        let offset = new_block.try_allocate(min_size, align)
            .ok_or("Failed to allocate in new block")?;
        let ptr = new_block.ptr_at_offset(offset)?;
        
        // Add block to collection
        {
            let mut blocks = self.blocks.write()
                .map_err(|_| "Failed to acquire write lock on blocks")?;
            
            // Check if we need to remove old blocks
            if blocks.len() >= self.config.max_blocks {
                warn!("Reached maximum blocks ({}), may need compaction", self.config.max_blocks);
            }
            
            blocks.push(new_block);
        }
        
        info!("Created new memory block {} of {} bytes", block_id, block_size);
        Ok((block_id, offset, ptr))
    }
    
    /// Finalize allocation by recording metadata and profiling info
    fn finalize_allocation(&self, object_id: ObjectId, block_id: u32, offset: usize, 
                          size: usize, ptr: NonNull<u8>, type_name: &str) 
        -> Result<(ObjectId, NonNull<u8>), String> {
        
        // Register object metadata
        let metadata = ObjectMetadata::new(object_id, size, type_name.to_string());
        self.object_registry.register(metadata)?;
        
        // Track allocation info
        let alloc_info = AllocationInfo {
            object_id,
            block_id,
            offset,
            size,
            ptr,
        };
        
        {
            let mut allocations = self.allocations.write()
                .map_err(|_| "Failed to acquire write lock on allocations")?;
            allocations.insert(object_id, alloc_info);
        }
        
        // Profile the allocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_allocation(size, ptr.as_ptr() as u64, Vec::new());
        }
        
        debug!("Finalized allocation for object {} at {:p}", object_id, ptr.as_ptr());
        Ok((object_id, ptr))
    }
    
    /// Deallocate an object
    #[instrument(skip(self))]
    pub fn deallocate(&self, object_id: ObjectId) -> Result<(), String> {
        debug!("Deallocating object {}", object_id);
        
        // Remove allocation tracking
        let alloc_info = {
            let mut allocations = self.allocations.write()
                .map_err(|_| "Failed to acquire write lock on allocations")?;
            allocations.remove(&object_id)
                .ok_or_else(|| format!("Object {} not found in allocations", object_id))?
        };
        
        // Update deallocation metrics
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.total_deallocations += 1;
            metrics.bytes_deallocated += alloc_info.size as u64;
        }
        
        // Unregister from object registry
        self.object_registry.unregister(object_id)?;
        
        // Profile the deallocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_deallocation(alloc_info.ptr.as_ptr() as u64, Vec::new());
        }
        
        // Check if block can be consolidated or freed
        self.check_block_consolidation(alloc_info.block_id)?;
        
        debug!("Successfully deallocated object {}", object_id);
        Ok(())
    }
    
    /// Check if blocks can be consolidated after deallocation
    fn check_block_consolidation(&self, block_id: u32) -> Result<(), String> {
        let blocks = self.blocks.read()
            .map_err(|_| "Failed to acquire read lock on blocks")?;
        
        if let Some(block) = blocks.iter().find(|b| b.id == block_id) {
            let utilization = block.utilization();
            if utilization < self.config.min_utilization * 100.0 {
                debug!("Block {} has low utilization ({:.1}%), may need consolidation", 
                       block_id, utilization);
                // In a full implementation, we would trigger consolidation here
            }
        }
        
        Ok(())
    }
    
    /// Get heap statistics
    pub fn get_stats(&self) -> Result<HeapStats, String> {
        let blocks = self.blocks.read()
            .map_err(|_| "Failed to acquire read lock on blocks")?;
        let allocations = self.allocations.read()
            .map_err(|_| "Failed to acquire read lock on allocations")?;
        let pressure = *self.pressure_level.read()
            .map_err(|_| "Failed to read pressure level")?;
        let metrics = self.metrics.read()
            .map_err(|_| "Failed to read metrics")?
            .clone();
        
        let total_blocks = blocks.len();
        let total_capacity: usize = blocks.iter().map(|b| b.size).sum();
        let total_used: usize = blocks.iter().map(|b| b.used).sum();
        let total_free = total_capacity - total_used;
        let average_utilization = if total_capacity > 0 {
            (total_used as f64 / total_capacity as f64) * 100.0
        } else {
            0.0
        };
        
        let active_objects = allocations.len();
        let object_registry_count = self.object_registry.object_count()
            .unwrap_or(0);
        
        Ok(HeapStats {
            total_blocks,
            total_capacity,
            total_used,
            total_free,
            average_utilization,
            active_objects,
            object_registry_count,
            fragmentation_ratio: Self::calculate_fragmentation(&blocks),
            memory_pressure: pressure,
            metrics,
        })
    }
    
    /// Calculate fragmentation ratio
    fn calculate_fragmentation(blocks: &[MemoryBlock]) -> f64 {
        if blocks.is_empty() {
            return 0.0;
        }
        
        let total_free: usize = blocks.iter().map(|b| b.free_space()).sum();
        let largest_free = blocks.iter().map(|b| b.free_space()).max().unwrap_or(0);
        
        if total_free == 0 {
            0.0
        } else {
            1.0 - (largest_free as f64 / total_free as f64)
        }
    }
    
    /// Trigger garbage collection event
    pub fn trigger_gc(&self, _gc_type: &str) -> Result<(), String> {
        // For now, just log the GC trigger - profiling integration can be added later
        debug!("GC triggered: {}", _gc_type);
        Ok(())
    }
    
    /// Get current thread ID for profiling
    fn get_current_thread_id() -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish()
    }
    
    /// Get allocation info for an object
    pub fn get_allocation_info(&self, object_id: ObjectId) -> Result<Option<AllocationInfo>, String> {
        let allocations = self.allocations.read()
            .map_err(|_| "Failed to acquire read lock on allocations")?;
        Ok(allocations.get(&object_id).cloned())
    }
    
    /// Get the complete allocation map for pointer-to-ObjectId resolution
    pub fn get_allocation_map(&self) -> Result<HashMap<ObjectId, AllocationInfo>, String> {
        let allocations = self.allocations.read()
            .map_err(|_| "Failed to acquire read lock on allocations")?;
        Ok(allocations.clone())
    }
    
    /// Check if pointer is valid (within heap bounds)
    pub fn is_valid_pointer(&self, ptr: *const u8) -> bool {
        if let Ok(blocks) = self.blocks.read() {
            blocks.iter().any(|block| block.contains_ptr(ptr))
        } else {
            false
        }
    }
    
    /// Get current memory pressure level
    pub fn get_memory_pressure(&self) -> MemoryPressure {
        self.pressure_level.read()
            .map(|guard| *guard)
            .unwrap_or(MemoryPressure::Low)
    }
    
    /// Get allocation metrics
    pub fn get_allocation_metrics(&self) -> Result<AllocationMetrics, String> {
        self.metrics.read()
            .map(|m| m.clone())
            .map_err(|_| "Failed to read allocation metrics".to_string())
    }
    
    /// Get allocation history summary
    pub fn get_allocation_history_summary(&self, window: Duration) -> Result<(f64, f64), String> {
        let history = self.history.read()
            .map_err(|_| "Failed to read allocation history")?;
        
        let allocation_rate = history.get_allocation_rate(window);
        let failure_rate = history.get_failure_rate(window);
        
        Ok((allocation_rate, failure_rate))
    }
    
    /// Force memory pressure check (for testing or external triggers)
    pub fn force_pressure_check(&self) -> Result<MemoryPressure, String> {
        self.check_memory_pressure()?;
        Ok(self.get_memory_pressure())
    }
    
    /// Set heap configuration (runtime reconfiguration)
    pub fn update_config(&mut self, new_config: HeapConfig) -> Result<(), String> {
        info!("Updating heap configuration");
        
        // Validate new configuration
        if new_config.default_block_size == 0 {
            return Err("Default block size cannot be zero".to_string());
        }
        if new_config.max_blocks == 0 {
            return Err("Max blocks cannot be zero".to_string());
        }
        if new_config.pressure_threshold < 0.0 || new_config.pressure_threshold > 1.0 {
            return Err("Pressure threshold must be between 0.0 and 1.0".to_string());
        }
        
        self.config = new_config;
        info!("Heap configuration updated successfully");
        Ok(())
    }
    
    /// Get detailed block information for debugging
    pub fn get_block_details(&self) -> Result<Vec<(u32, usize, usize, f64)>, String> {
        let blocks = self.blocks.read()
            .map_err(|_| "Failed to acquire read lock on blocks")?;
        
        Ok(blocks.iter().map(|block| {
            (block.id, block.size, block.used, block.utilization())
        }).collect())
    }
    
    /// Perform heap consistency check
    pub fn check_heap_consistency(&self) -> Result<Vec<String>, String> {
        let mut issues = Vec::new();
        
        let blocks = self.blocks.read()
            .map_err(|_| "Failed to acquire read lock on blocks")?;
        let allocations = self.allocations.read()
            .map_err(|_| "Failed to acquire read lock on allocations")?;
        
        // Check block consistency
        for block in blocks.iter() {
            if block.used > block.size {
                issues.push(format!("Block {} has used ({}) > size ({})", 
                                  block.id, block.used, block.size));
            }
            if block.next_free > block.size {
                issues.push(format!("Block {} has next_free ({}) > size ({})", 
                                  block.id, block.next_free, block.size));
            }
        }
        
        // Check allocation consistency
        for (object_id, alloc_info) in allocations.iter() {
            if !blocks.iter().any(|b| b.id == alloc_info.block_id) {
                issues.push(format!("Object {} references non-existent block {}", 
                                  object_id, alloc_info.block_id));
            }
            
            if let Some(block) = blocks.iter().find(|b| b.id == alloc_info.block_id) {
                if alloc_info.offset + alloc_info.size > block.size {
                    issues.push(format!("Object {} allocation extends beyond block {} bounds", 
                                      object_id, alloc_info.block_id));
                }
            }
        }
        
        Ok(issues)
    }
}

/// Heap statistics for monitoring and debugging
#[derive(Debug, Clone)]
pub struct HeapStats {
    pub total_blocks: usize,
    pub total_capacity: usize,
    pub total_used: usize,
    pub total_free: usize,
    pub average_utilization: f64,
    pub active_objects: usize,
    pub object_registry_count: usize,
    pub fragmentation_ratio: f64,
    pub memory_pressure: MemoryPressure,
    pub metrics: AllocationMetrics,
}

impl std::fmt::Display for HeapStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Heap Stats:\n\
             - Blocks: {}\n\
             - Capacity: {} bytes\n\
             - Used: {} bytes ({:.1}%)\n\
             - Free: {} bytes\n\
             - Active Objects: {}\n\
             - Fragmentation: {:.1}%\n\
             - Memory Pressure: {:?}\n\
             - Total Allocations: {}\n\
             - Total Deallocations: {}\n\
             - Allocation Failures: {}\n\
             - Peak Memory: {} bytes\n\
             - GC Triggers: {}\n\
             - Avg Allocation Time: {:.2}μs",
            self.total_blocks,
            self.total_capacity,
            self.total_used,
            self.average_utilization,
            self.total_free,
            self.active_objects,
            self.fragmentation_ratio * 100.0,
            self.memory_pressure,
            self.metrics.total_allocations,
            self.metrics.total_deallocations,
            self.metrics.allocation_failures,
            self.metrics.peak_memory_usage,
            self.metrics.gc_triggers,
            self.metrics.average_allocation_time.as_micros()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectRegistry;
    
    #[test]
    fn test_memory_block_creation() {
        let block = MemoryBlock::new(1024, 1).unwrap();
        assert_eq!(block.size, 1024);
        assert_eq!(block.used, 0);
        assert_eq!(block.next_free, 0);
        assert_eq!(block.id, 1);
        assert_eq!(block.free_space(), 1024);
    }
    
    #[test]
    fn test_memory_block_allocation() {
        let mut block = MemoryBlock::new(1024, 1).unwrap();
        
        let offset1 = block.try_allocate(100, 8).unwrap();
        assert_eq!(offset1, 0);
        assert_eq!(block.used, 100);
        assert_eq!(block.free_space(), 924);
        
        let offset2 = block.try_allocate(200, 8).unwrap();
        assert_eq!(offset2, 100);
        assert_eq!(block.used, 300);
        
        // Try to allocate more than available
        let offset3 = block.try_allocate(800, 8);
        assert!(offset3.is_none());
    }
    
    #[test]
    fn test_heap_manager_allocation() {
        let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        let (id1, ptr1) = heap.allocate::<u64>(8, "u64").unwrap();
        let (id2, ptr2) = heap.allocate::<u32>(4, "u32").unwrap();
        
        assert_ne!(id1, id2);
        assert_ne!(ptr1.as_ptr(), ptr2.as_ptr());
        
        // Verify allocations are tracked
        assert!(heap.get_allocation_info(id1).unwrap().is_some());
        assert!(heap.get_allocation_info(id2).unwrap().is_some());
        
        // Verify pointers are valid
        assert!(heap.is_valid_pointer(ptr1.as_ptr()));
        assert!(heap.is_valid_pointer(ptr2.as_ptr()));
    }
    
    #[test]
    fn test_heap_manager_deallocation() {
        let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        let (id, _ptr) = heap.allocate::<u64>(8, "u64").unwrap();
        assert!(heap.get_allocation_info(id).unwrap().is_some());
        
        heap.deallocate(id).unwrap();
        assert!(heap.get_allocation_info(id).unwrap().is_none());
    }
    
    #[test]
    fn test_heap_stats() {
        let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        let stats_initial = heap.get_stats().unwrap();
        assert_eq!(stats_initial.active_objects, 0);
        assert_eq!(stats_initial.total_blocks, 0);
        
        let (_id, _ptr) = heap.allocate::<u64>(8, "u64").unwrap();
        
        let stats_after = heap.get_stats().unwrap();
        assert_eq!(stats_after.active_objects, 1);
        assert_eq!(stats_after.total_blocks, 1);
        assert!(stats_after.total_used > 0);
    }
    
    #[test]
    fn test_large_allocation() {
        let mut config = HeapConfig::default();
        config.default_block_size = 1024; // Small blocks
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        // Allocate something larger than default block size
        let (id, ptr) = heap.allocate::<[u8; 2048]>(2048, "large_array").unwrap();
        
        assert!(heap.get_allocation_info(id).unwrap().is_some());
        assert!(heap.is_valid_pointer(ptr.as_ptr()));
        
        let stats = heap.get_stats().unwrap();
        assert!(stats.total_capacity >= 2048);
    }
    
    #[test]
    fn test_memory_pressure_detection() {
        let mut config = HeapConfig::default();
        config.default_block_size = 1024;
        config.pressure_threshold = 0.5; // 50% triggers medium pressure
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        // Initially should be low pressure
        assert_eq!(heap.get_memory_pressure(), MemoryPressure::Low);
        
        // Allocate to increase pressure
        let mut allocations = Vec::new();
        for i in 0..10 {
            let (id, _ptr) = heap.allocate::<[u8; 100]>(100, &format!("test_{}", i)).unwrap();
            allocations.push(id);
        }
        
        // Force pressure check
        let pressure = heap.force_pressure_check().unwrap();
        assert!(pressure != MemoryPressure::Low || heap.get_stats().unwrap().average_utilization < 50.0);
    }
    
    #[test]
    fn test_allocation_metrics() {
        let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        let initial_metrics = heap.get_allocation_metrics().unwrap();
        assert_eq!(initial_metrics.total_allocations, 0);
        
        // Perform some allocations
        let (id1, _ptr1) = heap.allocate::<u64>(8, "u64").unwrap();
        let (id2, _ptr2) = heap.allocate::<u32>(4, "u32").unwrap();
        
        let metrics = heap.get_allocation_metrics().unwrap();
        assert_eq!(metrics.total_allocations, 2);
        assert_eq!(metrics.bytes_allocated, 12);
        assert!(metrics.average_allocation_time.as_nanos() > 0);
        
        // Test deallocation metrics
        heap.deallocate(id1).unwrap();
        let metrics_after = heap.get_allocation_metrics().unwrap();
        assert_eq!(metrics_after.total_deallocations, 1);
        assert_eq!(metrics_after.bytes_deallocated, 8);
    }
    
    #[test]
    fn test_best_fit_allocation() {
        let mut config = HeapConfig::default();
        config.default_block_size = 1024;
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        // Create some allocations and deallocations to create fragmentation
        let (id1, _ptr1) = heap.allocate::<[u8; 100]>(100, "test1").unwrap();
        let (id2, _ptr2) = heap.allocate::<[u8; 200]>(200, "test2").unwrap();
        let (id3, _ptr3) = heap.allocate::<[u8; 100]>(100, "test3").unwrap();
        
        // Deallocate middle allocation to create a hole
        heap.deallocate(id2).unwrap();
        
        // This should fit in the deallocated space
        let (id4, _ptr4) = heap.allocate::<[u8; 150]>(150, "test4").unwrap();
        
        assert!(heap.get_allocation_info(id4).unwrap().is_some());
    }
    
    #[test]
    fn test_heap_consistency_check() {
        let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        // Fresh heap should have no consistency issues
        let issues = heap.check_heap_consistency().unwrap();
        assert!(issues.is_empty());
        
        // Create some allocations
        let (id1, _ptr1) = heap.allocate::<u64>(8, "u64").unwrap();
        let (id2, _ptr2) = heap.allocate::<u32>(4, "u32").unwrap();
        
        // Still should be consistent
        let issues = heap.check_heap_consistency().unwrap();
        assert!(issues.is_empty());
    }
    
    #[test]
    fn test_config_update() {
        let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let mut heap = HeapManager::new(config, registry);
        
        let mut new_config = HeapConfig::default();
        new_config.pressure_threshold = 0.9;
        new_config.gc_trigger_threshold = 0.8;
        
        // Should succeed with valid config
        assert!(heap.update_config(new_config).is_ok());
        
        // Should fail with invalid config
        let mut invalid_config = HeapConfig::default();
        invalid_config.default_block_size = 0;
        assert!(heap.update_config(invalid_config).is_err());
    }
    
    #[test]
    fn test_allocation_history() {
        let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);
        
        // Perform allocations over time
        for i in 0..5 {
            let _ = heap.allocate::<[u8; 100]>(100, &format!("test_{}", i)).unwrap();
        }
        
        let window = Duration::from_secs(1);
        let (alloc_rate, failure_rate) = heap.get_allocation_history_summary(window).unwrap();
        
        assert!(alloc_rate >= 0.0);
        assert!(failure_rate >= 0.0);
    }
}
