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
// use crate::profiling::memory::{MemoryProfiler, GcEvent, AllocationEvent};
use crate::error::CursedError;

/// Raw memory block allocated from the system
/// 
/// This represents a contiguous block of memory that objects can be allocated within.
/// Each block tracks its usage to enable efficient allocation strategies.
#[derive(Debug)]
pub struct MemoryBlock {
    /// Pointer to the start of the block
    /// Total size of the block in bytes
    /// Number of bytes currently used
    /// Next free offset for bump allocation
    /// Block ID for debugging
impl MemoryBlock {
    /// Allocate a new memory block from the system
    /// 
    /// This uses the system allocator to get a large contiguous chunk of memory
    /// that will be subdivided for object allocations.
    #[instrument]
    pub fn new(size: usize, id: u32) -> Result<Self, String> {
        if size == 0 {
            return Err("Cannot allocate zero-sized memory block".to_string());
        // Ensure size is properly aligned
        let aligned_size = (size + 7) & !7; // 8-byte alignment
        
        let layout = Layout::from_size_align(aligned_size, 8)
            .map_err(|e| format!("Invalid layout for memory block: {}", e))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(format!("Failed to allocate {} bytes from system", aligned_size));
        let non_null_ptr = NonNull::new(ptr)
            .ok_or("System allocator returned null pointer")?;
        
        debug!("Allocated memory block {} of {} bytes at {:p}", id, aligned_size, ptr);
        
        Ok(Self {
        })
    /// Try to allocate space within this block
    /// 
    /// Returns the offset within the block if successful, or None if
    /// there isn't enough space remaining.
    #[instrument(skip(self))]
    pub fn try_allocate(&mut self, size: usize, align: usize) -> Option<usize> {
        if size == 0 {
            return None;
        // Calculate aligned offset
        let aligned_offset = (self.next_free + align - 1) & !(align - 1);
        
        // Check if allocation fits
        if aligned_offset + size > self.size {
                   self.id, size, aligned_offset + size, self.size);
            return None;
        // Update block state
        self.next_free = aligned_offset + size;
        self.used += size;
        
        debug!("Allocated {} bytes at offset {} in block {}", size, aligned_offset, self.id);
        Some(aligned_offset)
    /// Get pointer to allocated space at given offset
    pub fn ptr_at_offset(&self, offset: usize) -> Result<NonNull<u8>, String> {
        if offset >= self.size {
            return Err(format!("Offset {} exceeds block size {}", offset, self.size));
        unsafe {
            let ptr = self.ptr.as_ptr().add(offset);
            NonNull::new(ptr).ok_or("Computed null pointer".to_string())
        }
    }
    
    /// Get remaining free space in this block
    pub fn free_space(&self) -> usize {
        self.size - self.next_free
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
        debug!("Deallocated memory block {} of {} bytes", self.id, self.size);
    }
}

// Safety: MemoryBlock is safe to send between threads because:
// 1. NonNull<u8> is Send (raw pointers to u8 can be sent between threads)
// 2. All other fields (usize, u32) are Copy and Send
// 3. The memory pointed to is heap-allocated and owned by this block
// 4. Access is coordinated through the heap manager's locking
// Safety: MemoryBlock is safe to share between threads because:
// 1. All mutation is coordinated through the heap manager's RwLock
// 2. The NonNull<u8> pointer is stable (doesn't change once allocated)
// 3. Field access is atomic or protected by the containing lock
/// Configuration for heap manager behavior
#[derive(Debug, Clone)]
pub struct HeapConfig {
    /// Default size for new memory blocks
    /// Maximum number of blocks to maintain
    /// Minimum block utilization before compaction
    /// Enable memory profiling integration
    /// Memory pressure threshold (percentage of capacity)
    /// Adaptive allocation growth factor
    /// Maximum single allocation size relative to block size
    /// GC trigger threshold (percentage of heap usage)
/// Memory pressure levels for adaptive allocation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryPressure {
/// Allocation strategy based on current memory conditions
#[derive(Debug, Clone, Copy)]
pub enum AllocationStrategy {
    /// Standard bump allocation
    /// Best-fit allocation to reduce fragmentation
    /// Compact existing blocks before allocating
impl Default for HeapConfig {
    fn default() -> Self {
        Self {
            default_block_size: 1024 * 1024, // 1MB default blocks
            min_utilization: 0.5, // 50% minimum utilization
            pressure_threshold: 0.8, // 80% capacity triggers pressure
            growth_factor: 1.5, // 50% growth when expanding
            max_allocation_ratio: 0.5, // Max 50% of block size per allocation
            gc_trigger_threshold: 0.75, // Trigger GC at 75% heap usage
        }
    }
/// Information about an allocated object in the heap
#[derive(Debug, Clone)]
pub struct AllocationInfo {
    /// Object identifier
    /// Block where object is allocated
    /// Offset within the block
    /// Size of the allocation
    /// Pointer to the allocated memory
// Safety: AllocationInfo is safe to send between threads because:
// 1. ObjectId is Copy and thread-safe
// 2. u32 and usize are Copy and Send
// 3. NonNull<u8> points to heap-allocated memory that is owned by the heap manager
// 4. Access to the pointed memory is coordinated through the heap manager's locks
// 5. The pointer remains valid as long as the object is tracked in the heap manager
// Safety: AllocationInfo is safe to share between threads because:
// 1. All fields are either Copy or have stable addresses (NonNull<u8>)
// 2. The NonNull<u8> pointer doesn't change once allocated
// 3. Access to the memory is coordinated through heap manager synchronization
// 4. The struct itself is immutable after creation (only read operations)
/// Allocation metrics for performance monitoring
#[derive(Debug, Clone)]
pub struct AllocationMetrics {
impl Default for AllocationMetrics {
    fn default() -> Self {
        Self {
        }
    }
/// Recent allocation history for adaptive strategies
#[derive(Debug)]
struct AllocationHistory {
impl AllocationHistory {
    fn new() -> Self {
        Self {
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
    /// Active memory blocks
    /// Next block ID to assign
    /// Object ID generator
    /// Object registry for metadata tracking
    /// Allocation tracking for objects
    /// Memory profiler (optional)
    /// Allocation metrics for monitoring
    /// Allocation history for adaptive strategies
    /// Current memory pressure level
impl HeapManager {
    /// Create a new heap manager with the given configuration
    #[instrument]
    pub fn new(config: HeapConfig, object_registry: SharedObjectRegistry) -> Self {
              config.default_block_size, config.max_blocks);
        
        Self {
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
            AllocationStrategy::BestFit => {
                self.allocate_with_best_fit_strategy(object_id, size, align, type_name)
            AllocationStrategy::CompactFirst => {
                self.allocate_with_compaction_strategy(object_id, size, align, type_name)
        
        // Update metrics
        let allocation_time = start_time.elapsed();
        self.update_allocation_metrics(size, allocation_time, result.is_ok())?;
        
        match result {
            Ok((id, ptr)) => {
                if let Ok(mut history) = self.history.write() {
                    history.record_allocation(size);
                }
                Ok((id, ptr))
            Err(e) => {
                if let Ok(mut history) = self.history.write() {
                    history.record_failure();
                }
                Err(e)
            }
        }
    /// Determine the best allocation strategy based on current conditions
    fn determine_allocation_strategy(&self, size: usize) -> Result<AllocationStrategy, String> {
        let pressure = *self.pressure_level.read()
            .map_err(|_| "Failed to read pressure level")?;
        
        // Check if allocation is too large for normal strategy
        if size > (self.config.default_block_size as f64 * self.config.max_allocation_ratio) as usize {
            return Ok(AllocationStrategy::BestFit);
        match pressure {
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
        }
    }
    
    /// Allocate using bump allocation strategy
                                  align: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        // Try to allocate in existing blocks first
        if let Some((block_id, offset, ptr)) = self.try_allocate_in_existing_blocks(size, align)? {
            return self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name);
        // Need to create a new block
        let (block_id, offset, ptr) = self.allocate_new_block(size, align)?;
        self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name)
    /// Allocate using best-fit strategy to reduce fragmentation
                                      align: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        // First try best-fit allocation in existing blocks
        if let Some((block_id, offset, ptr)) = self.try_best_fit_allocation(size, align)? {
            return self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name);
        // Fall back to creating new block
        let (block_id, offset, ptr) = self.allocate_new_block(size, align)?;
        self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name)
    /// Allocate with compaction strategy
                                        align: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        // Try compaction first
        self.compact_blocks()?;
        
        // Try allocation after compaction
        if let Some((block_id, offset, ptr)) = self.try_allocate_in_existing_blocks(size, align)? {
            return self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name);
        // Still need new block
        let (block_id, offset, ptr) = self.allocate_new_block(size, align)?;
        self.finalize_allocation(object_id, block_id, offset, size, ptr, type_name)
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
        if let Some(idx) = best_block_idx {
            if let Some(offset) = blocks[idx].try_allocate(size, align) {
                let ptr = blocks[idx].ptr_at_offset(offset)?;
                return Ok(Some((blocks[idx].id, offset, ptr)));
            }
        }
        
        Ok(None)
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
        
        // Update pressure level
        if let Ok(mut pressure) = self.pressure_level.write() {
            if *pressure != new_pressure {
                      *pressure, new_pressure, utilization * 100.0);
                *pressure = new_pressure;
                
                // Record pressure event
                if let Ok(mut history) = self.history.write() {
                    history.record_pressure(new_pressure);
                }
            }
        // Trigger GC if needed
        if utilization >= self.config.gc_trigger_threshold {
            self.trigger_gc_internal("memory_pressure")?;
        Ok(())
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
    /// Compact blocks to reduce fragmentation
    fn compact_blocks(&self) -> Result<(), String> {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.compaction_events += 1;
        debug!("Starting block compaction");
        
        // For now, just log the compaction - actual implementation would need
        // to move objects between blocks which requires GC coordination
        warn!("Block compaction requested but not fully implemented - requires GC integration");
        
        Ok(())
    /// Internal GC trigger with metrics tracking
    fn trigger_gc_internal(&self, reason: &str) -> Result<(), String> {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.gc_triggers += 1;
        info!("Triggering GC due to: {}", reason);
        
        // Profile the GC event
        if let Some(profiler) = &self.profiler {
//             use crate::profiling::memory::{GcEvent, GcType};
            let gc_event = GcEvent {
            let _ = profiler.track_gc_event(gc_event);
        // For now, just log - actual GC integration would happen here
        debug!("GC triggered: {}", reason);
        
        Ok(())
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
    /// Allocate a new memory block
    fn allocate_new_block(&self, min_size: usize, align: usize) 
        -> Result<(u32, usize, NonNull<u8>), String> {
        
        let block_id = {
            let mut next_id = self.next_block_id.lock()
                .map_err(|_| "Failed to acquire lock on next_block_id")?;
            let id = *next_id;
            *next_id += 1;
            id
        
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
            blocks.push(new_block);
        info!("Created new memory block {} of {} bytes", block_id, block_size);
        Ok((block_id, offset, ptr))
    /// Finalize allocation by recording metadata and profiling info
                          size: usize, ptr: NonNull<u8>, type_name: &str) 
        -> Result<(ObjectId, NonNull<u8>), String> {
        
        // Register object metadata
        let metadata = ObjectMetadata::new(object_id, size, type_name.to_string());
        self.object_registry.register(metadata)?;
        
        // Track allocation info
        let alloc_info = AllocationInfo {
        
        {
            let mut allocations = self.allocations.write()
                .map_err(|_| "Failed to acquire write lock on allocations")?;
            allocations.insert(object_id, alloc_info);
        // Profile the allocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_allocation(size, ptr.as_ptr() as u64, Vec::new());
        debug!("Finalized allocation for object {} at {:p}", object_id, ptr.as_ptr());
        Ok((object_id, ptr))
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
        
        // Update deallocation metrics
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.total_deallocations += 1;
            metrics.bytes_deallocated += alloc_info.size as u64;
        // Unregister from object registry
        self.object_registry.unregister(object_id)?;
        
        // Profile the deallocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_deallocation(alloc_info.ptr.as_ptr() as u64, Vec::new());
        // Check if block can be consolidated or freed
        self.check_block_consolidation(alloc_info.block_id)?;
        
        debug!("Successfully deallocated object {}", object_id);
        Ok(())
    /// Check if blocks can be consolidated after deallocation
    fn check_block_consolidation(&self, block_id: u32) -> Result<(), String> {
        let blocks = self.blocks.read()
            .map_err(|_| "Failed to acquire read lock on blocks")?;
        
        if let Some(block) = blocks.iter().find(|b| b.id == block_id) {
            let utilization = block.utilization();
            if utilization < self.config.min_utilization * 100.0 {
                       block_id, utilization);
                // In a full implementation, we would trigger consolidation here
            }
        }
        
        Ok(())
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
        
        let active_objects = allocations.len();
        let object_registry_count = self.object_registry.object_count()
            .unwrap_or(0);
        
        Ok(HeapStats {
        })
    /// Calculate fragmentation ratio
    fn calculate_fragmentation(blocks: &[MemoryBlock]) -> f64 {
        if blocks.is_empty() {
            return 0.0;
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
    /// Get current thread ID for profiling
    fn get_current_thread_id() -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish()
    /// Get allocation info for an object
    pub fn get_allocation_info(&self, object_id: ObjectId) -> Result<Option<AllocationInfo>, String> {
        let allocations = self.allocations.read()
            .map_err(|_| "Failed to acquire read lock on allocations")?;
        Ok(allocations.get(&object_id).cloned())
    /// Get the complete allocation map for pointer-to-ObjectId resolution
    pub fn get_allocation_map(&self) -> Result<HashMap<ObjectId, AllocationInfo>, String> {
        let allocations = self.allocations.read()
            .map_err(|_| "Failed to acquire read lock on allocations")?;
        Ok(allocations.clone())
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
    /// Get allocation metrics
    pub fn get_allocation_metrics(&self) -> Result<AllocationMetrics, String> {
        self.metrics.read()
            .map(|m| m.clone())
            .map_err(|_| "Failed to read allocation metrics".to_string())
    /// Get allocation history summary
    pub fn get_allocation_history_summary(&self, window: Duration) -> Result<(f64, f64), String> {
        let history = self.history.read()
            .map_err(|_| "Failed to read allocation history")?;
        
        let allocation_rate = history.get_allocation_rate(window);
        let failure_rate = history.get_failure_rate(window);
        
        Ok((allocation_rate, failure_rate))
    /// Force memory pressure check (for testing or external triggers)
    pub fn force_pressure_check(&self) -> Result<MemoryPressure, String> {
        self.check_memory_pressure()?;
        Ok(self.get_memory_pressure())
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
        self.config = new_config;
        info!("Heap configuration updated successfully");
        Ok(())
    /// Get detailed block information for debugging
    pub fn get_block_details(&self) -> Result<Vec<(u32, usize, usize, f64)>, String> {
        let blocks = self.blocks.read()
            .map_err(|_| "Failed to acquire read lock on blocks")?;
        
        Ok(blocks.iter().map(|block| {
            (block.id, block.size, block.used, block.utilization())
        }).collect())
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
                                  block.id, block.used, block.size));
            }
            if block.next_free > block.size {
                                  block.id, block.next_free, block.size));
            }
        }
        
        // Check allocation consistency
        for (object_id, alloc_info) in allocations.iter() {
            if !blocks.iter().any(|b| b.id == alloc_info.block_id) {
                                  object_id, alloc_info.block_id));
            if let Some(block) = blocks.iter().find(|b| b.id == alloc_info.block_id) {
                if alloc_info.offset + alloc_info.size > block.size {
                                      object_id, alloc_info.block_id));
                }
            }
        Ok(issues)
    }
}

/// Heap statistics for monitoring and debugging
#[derive(Debug, Clone)]
pub struct HeapStats {
impl std::fmt::Display for HeapStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            self.metrics.average_allocation_time.as_micros()
        )
    }
}

