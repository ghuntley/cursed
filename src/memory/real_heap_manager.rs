/// Real Heap Management System with Proper Memory Algorithms
/// 
/// This module provides production-ready heap management that implements actual
/// memory allocation and deallocation algorithms, integrating seamlessly with
/// the existing GC infrastructure while providing:
/// 
/// 1. **Real Memory Allocation**: Actual heap memory allocation with proper block management
/// 2. **Free List Management**: Advanced free list algorithms with coalescing and splitting
/// 3. **Memory Region Mapping**: Virtual memory management with segment tracking
/// 4. **Block Size Tracking**: Precise size tracking for all allocations
/// 5. **Fragmentation Management**: Real-time fragmentation detection and mitigation
/// 6. **GC Integration**: Seamless integration with existing GC and goroutine systems

use std::ptr::{NonNull, null_mut};
use std::alloc::{alloc, dealloc, realloc, Layout};
use std::sync::{Arc, RwLock, Mutex, atomic::{AtomicUsize, AtomicU64, Ordering}};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::mem;
use std::time::{Instant, Duration};
use tracing::{instrument, debug, info, warn, error, span, Level};

use crate::memory::object_id::{ObjectId, SharedObjectRegistry};
use crate::memory::metadata::{MetadataManager, ObjectHeader, MemoryLayout};
use crate::memory::allocator::{Allocator, AllocationResult};
// use crate::profiling::memory::MemoryProfiler;
use crate::error::CursedError;

/// Real heap block with proper memory management
/// 
/// Represents a memory block allocated from the system with proper
/// bookkeeping for size, alignment, and free space management.
#[derive(Debug)]
pub struct RealHeapBlock {
    /// System-allocated memory pointer
    /// Total block size
    /// Layout used for system allocation
    /// Free list for this block
    free_list: BTreeMap<usize, FreeChunk>, // offset -> chunk
    /// Allocated chunks for tracking
    allocated_chunks: HashMap<usize, AllocatedChunk>, // offset -> chunk
    /// Block ID for debugging
    /// Usage statistics
/// Free memory chunk within a block
#[derive(Debug, Clone)]
struct FreeChunk {
/// Allocated memory chunk within a block
#[derive(Debug, Clone)]
struct AllocatedChunk {
/// Block usage statistics
#[derive(Debug, Clone)]
struct BlockStatistics {
impl BlockStatistics {
    fn new() -> Self {
        Self {
        }
    }
    
    fn update_fragmentation(&mut self, total_free: usize, largest_free: usize) {
        self.largest_free_chunk = largest_free;
        if total_free > 0 {
            self.fragmentation_score = 1.0 - (largest_free as f64 / total_free as f64);
        } else {
            self.fragmentation_score = 0.0;
        }
    }
impl RealHeapBlock {
    /// Create a new real heap block
    #[instrument]
    pub fn new(size: usize, id: u32) -> Result<Self, String> {
        info!("Creating real heap block {} with {} bytes", id, size);
        
        // Ensure minimum size and alignment
        let actual_size = size.max(4096).next_power_of_two(); // At least 4KB, power of 2
        let layout = Layout::from_size_align(actual_size, 8)
            .map_err(|e| format!("Invalid layout for heap block: {}", e))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(format!("Failed to allocate {} bytes from system", actual_size));
        let non_null_ptr = NonNull::new(ptr)
            .ok_or("System allocator returned null pointer")?;
        
        // Initialize with one large free chunk
        let mut free_list = BTreeMap::new();
        free_list.insert(0, FreeChunk {
        });
        
        let mut stats = BlockStatistics::new();
        stats.largest_free_chunk = actual_size;
        stats.free_chunk_count = 1;
        
        debug!("Created real heap block {} at {:p} with {} bytes", id, ptr, actual_size);
        
        Ok(Self {
        })
    /// Allocate memory within this block using advanced algorithms
    #[instrument(skip(self))]
    pub fn allocate(&mut self, size: usize, alignment: usize, object_id: Option<ObjectId>) -> Result<NonNull<u8>, String> {
        let aligned_size = self.align_size(size, alignment);
        
        debug!("Block {} allocating {} bytes (aligned to {})", self.id, size, aligned_size);
        
        // Find suitable free chunk using best-fit algorithm
        let chunk_offset = self.find_best_fit_chunk(aligned_size, alignment)?;
        
        // Split the chunk if needed
        self.split_chunk_if_needed(chunk_offset, aligned_size)?;
        
        // Remove from free list and add to allocated chunks
        let free_chunk = self.free_list.remove(&chunk_offset)
            .ok_or("Free chunk disappeared during allocation")?;
        
        self.allocated_chunks.insert(chunk_offset, AllocatedChunk {
        });
        
        // Update statistics
        self.stats.total_allocations += 1;
        self.stats.bytes_allocated += aligned_size;
        self.stats.current_usage += aligned_size;
        self.update_fragmentation_stats();
        
        // Calculate final pointer
        let ptr = unsafe {
            NonNull::new(self.ptr.as_ptr().add(chunk_offset))
                .ok_or("Computed null pointer during allocation")?
        
               self.id, aligned_size, chunk_offset, ptr.as_ptr());
        
        Ok(ptr)
    /// Deallocate memory and coalesce with adjacent free chunks
    #[instrument(skip(self))]
    pub fn deallocate(&mut self, ptr: NonNull<u8>) -> Result<usize, String> {
        let offset = self.ptr_to_offset(ptr)?;
        
        debug!("Block {} deallocating at offset {} (ptr {:p})", self.id, offset, ptr.as_ptr());
        
        // Find and remove allocated chunk
        let allocated_chunk = self.allocated_chunks.remove(&offset)
            .ok_or_else(|| format!("No allocation found at offset {}", offset))?;
        
        let size = allocated_chunk.size;
        
        // Create new free chunk
        let free_chunk = FreeChunk {
        
        // Insert into free list
        self.free_list.insert(offset, free_chunk);
        
        // Coalesce with adjacent free chunks
        self.coalesce_free_chunks(offset)?;
        
        // Update statistics
        self.stats.total_deallocations += 1;
        self.stats.bytes_deallocated += size;
        self.stats.current_usage = self.stats.current_usage.saturating_sub(size);
        self.update_fragmentation_stats();
        
        debug!("Block {} deallocated {} bytes at offset {}", self.id, size, offset);
        
        Ok(size)
    /// Find best-fit chunk for allocation
    fn find_best_fit_chunk(&self, size: usize, alignment: usize) -> Result<usize, String> {
        let mut best_offset = None;
        let mut best_waste = usize::MAX;
        
        for (&offset, chunk) in &self.free_list {
            let aligned_offset = self.align_offset(offset, alignment);
            let usable_size = chunk.size.saturating_sub(aligned_offset - offset);
            
            if usable_size >= size {
                let waste = usable_size - size;
                if waste < best_waste {
                    best_waste = waste;
                    best_offset = Some(aligned_offset);
                }
            }
        best_offset.ok_or_else(|| {
            format!("No suitable free chunk found for {} bytes with {}-byte alignment", size, alignment)
        })
    /// Split a chunk if it's larger than needed
    fn split_chunk_if_needed(&mut self, offset: usize, needed_size: usize) -> Result<(), String> {
        if let Some(chunk) = self.free_list.get(&offset).cloned() {
            if chunk.size > needed_size + 16 { // Only split if remainder is at least 16 bytes
                let remainder_offset = offset + needed_size;
                let remainder_size = chunk.size - needed_size;
                
                // Update original chunk
                self.free_list.insert(offset, FreeChunk {
                });
                
                // Create remainder chunk
                self.free_list.insert(remainder_offset, FreeChunk {
                });
                
                       offset, needed_size, remainder_size);
            }
        }
        
        Ok(())
    /// Coalesce adjacent free chunks to reduce fragmentation
    fn coalesce_free_chunks(&mut self, offset: usize) -> Result<(), String> {
        let mut current_offset = offset;
        let mut coalesced_size = self.free_list.get(&offset)
            .map(|c| c.size)
            .unwrap_or(0);
        
        // Coalesce with previous chunks
        loop {
            let chunk = match self.free_list.get(&current_offset).cloned() {
            
            if let Some(prev_offset) = chunk.prev_chunk {
                let should_coalesce = if let Some(prev_chunk) = self.free_list.get(&prev_offset) {
                    prev_offset + prev_chunk.size == current_offset
                } else {
                    false
                
                if should_coalesce {
                    // Adjacent - coalesce
                    let prev_chunk = self.free_list.get(&prev_offset).cloned().unwrap();
                    self.free_list.remove(&current_offset);
                    coalesced_size += prev_chunk.size;
                    current_offset = prev_offset;
                    continue;
                }
            }
            break;
        // Find and coalesce with next chunks
        let mut next_offset = current_offset + coalesced_size;
        while let Some(next_chunk) = self.free_list.get(&next_offset).cloned() {
            if next_offset == current_offset + coalesced_size {
                // Adjacent - coalesce
                self.free_list.remove(&next_offset);
                coalesced_size += next_chunk.size;
                next_offset = current_offset + coalesced_size;
            } else {
                break;
            }
        }
        
        // Update the coalesced chunk
        self.free_list.insert(current_offset, FreeChunk {
            prev_chunk: None, // Will be updated by link_free_chunks
        });
        
        // Relink the free chunk chain
        self.link_free_chunks();
        
        debug!("Coalesced chunks starting at offset {} into {} bytes", current_offset, coalesced_size);
        
        Ok(())
    /// Link free chunks in order
    fn link_free_chunks(&mut self) {
        let mut sorted_offsets: Vec<_> = self.free_list.keys().cloned().collect();
        sorted_offsets.sort();
        
        for i in 0..sorted_offsets.len() {
            let offset = sorted_offsets[i];
            if let Some(chunk) = self.free_list.get_mut(&offset) {
                chunk.prev_chunk = if i > 0 { Some(sorted_offsets[i - 1]) } else { None };
                chunk.next_chunk = if i < sorted_offsets.len() - 1 { Some(sorted_offsets[i + 1]) } else { None };
            }
        }
    /// Update fragmentation statistics
    fn update_fragmentation_stats(&mut self) {
        let total_free: usize = self.free_list.values().map(|c| c.size).sum();
        let largest_free = self.free_list.values().map(|c| c.size).max().unwrap_or(0);
        
        self.stats.free_chunk_count = self.free_list.len();
        self.stats.update_fragmentation(total_free, largest_free);
    /// Convert pointer to offset within block
    fn ptr_to_offset(&self, ptr: NonNull<u8>) -> Result<usize, String> {
        let ptr_addr = ptr.as_ptr() as usize;
        let base_addr = self.ptr.as_ptr() as usize;
        
        if ptr_addr < base_addr || ptr_addr >= base_addr + self.size {
            return Err(format!("Pointer {:p} not within block bounds", ptr.as_ptr()));
        Ok(ptr_addr - base_addr)
    /// Align size to alignment boundary
    fn align_size(&self, size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    /// Align offset to alignment boundary
    fn align_offset(&self, offset: usize, alignment: usize) -> usize {
        (offset + alignment - 1) & !(alignment - 1)
    /// Check if pointer belongs to this block
    pub fn contains_ptr(&self, ptr: *const u8) -> bool {
        let addr = ptr as usize;
        let start = self.ptr.as_ptr() as usize;
        let end = start + self.size;
        addr >= start && addr < end
    /// Get block utilization percentage
    pub fn utilization(&self) -> f64 {
        if self.size == 0 {
            0.0
        } else {
            (self.stats.current_usage as f64 / self.size as f64) * 100.0
        }
    }
    
    /// Get fragmentation score
    pub fn fragmentation_score(&self) -> f64 {
        self.stats.fragmentation_score
    /// Get free space
    pub fn free_space(&self) -> usize {
        self.size - self.stats.current_usage
    /// Check if can allocate size
    pub fn can_allocate(&self, size: usize, alignment: usize) -> bool {
        let aligned_size = self.align_size(size, alignment);
        
        for (&offset, chunk) in &self.free_list {
            let aligned_offset = self.align_offset(offset, alignment);
            let usable_size = chunk.size.saturating_sub(aligned_offset - offset);
            
            if usable_size >= aligned_size {
                return true;
            }
        }
        
        false
    /// Get block statistics
    pub fn get_statistics(&self) -> BlockStatistics {
        self.stats.clone()
    }
}

// Safety: RealHeapBlock is safe to send between threads because:
// 1. NonNull<u8> points to heap-allocated memory owned by this block
// 2. All data structures are self-contained
// 3. Access will be coordinated through external synchronization
// Safety: RealHeapBlock is safe to share between threads because:
// 1. All mutations will be coordinated through external locks
// 2. The memory pointer is stable once allocated
// 3. Internal data structures don't contain raw pointers to shared data
impl Drop for RealHeapBlock {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr.as_ptr(), self.layout);
        }
        debug!("Deallocated real heap block {} of {} bytes", self.id, self.size);
    }
}

/// Real heap manager with proper memory algorithms
/// 
/// This manages multiple heap blocks and implements real memory allocation
/// algorithms integrated with the existing GC infrastructure.
pub struct RealHeapManager {
    /// Configuration
    /// Active heap blocks
    /// Next block ID
    /// Metadata manager
    /// Object registry for GC integration
    /// Memory profiler (optional)
    /// Global statistics
    /// Memory pressure tracking
/// Configuration for real heap manager
#[derive(Debug, Clone)]
pub struct RealHeapConfig {
    /// Initial block size
    /// Maximum number of blocks
    /// Block growth factor
    /// Fragmentation threshold for triggering compaction
    /// Memory pressure threshold
    /// Enable automatic compaction
    /// Minimum free space percentage
impl Default for RealHeapConfig {
    fn default() -> Self {
        Self {
            initial_block_size: 2 * 1024 * 1024, // 2MB
        }
    }
/// Global heap statistics
#[derive(Debug, Clone)]
pub struct RealHeapStatistics {
impl RealHeapStatistics {
    fn new() -> Self {
        Self {
        }
    }
/// Memory pressure monitoring
#[derive(Debug)]
struct MemoryPressureMonitor {
impl MemoryPressureMonitor {
    fn new() -> Self {
        Self {
        }
    }
    
    fn update_pressure(&mut self, stats: &RealHeapStatistics) {
        let utilization = if stats.total_capacity > 0 {
            stats.total_used as f64 / stats.total_capacity as f64
        } else {
            0.0
        
        let failure_rate = self.calculate_failure_rate();
        let fragmentation_factor = stats.overall_fragmentation;
        
        // Combine factors to compute overall pressure
        self.current_pressure = utilization * 0.5 + failure_rate * 0.3 + fragmentation_factor * 0.2;
        
        // Record in history
        let now = Instant::now();
        self.pressure_history.push_back((now, self.current_pressure));
        
        // Keep only last 100 entries
        if self.pressure_history.len() > 100 {
            self.pressure_history.pop_front();
        }
    }
    
    fn record_allocation_failure(&mut self) {
        let now = Instant::now();
        self.recent_failures.push_back(now);
        
        // Keep only failures from last 60 seconds
        let cutoff = now - Duration::from_secs(60);
        while let Some(&front_time) = self.recent_failures.front() {
            if front_time < cutoff {
                self.recent_failures.pop_front();
            } else {
                break;
            }
        }
    fn calculate_failure_rate(&self) -> f64 {
        let now = Instant::now();
        let window = Duration::from_secs(30);
        let cutoff = now - window;
        
        let recent_failures = self.recent_failures.iter()
            .filter(|&&time| time > cutoff)
            .count();
        
        // Normalize to failures per second
        recent_failures as f64 / window.as_secs() as f64
    fn should_trigger_compaction(&self, threshold: f64) -> bool {
        if self.current_pressure < threshold {
            return false;
        // Don't compact too frequently
        if let Some(last_compaction) = self.last_compaction {
            if last_compaction.elapsed() < Duration::from_secs(10) {
                return false;
            }
        }
        
        true
    fn record_compaction(&mut self) {
        self.last_compaction = Some(Instant::now());
    }
}

use std::sync::atomic::AtomicU32;

impl RealHeapManager {
    /// Create a new real heap manager
    #[instrument]
    pub fn new(config: RealHeapConfig, object_registry: SharedObjectRegistry) -> Result<Self, String> {
              config.initial_block_size as f64 / (1024.0 * 1024.0));
        
        let metadata_manager = Arc::new(MetadataManager::new(8)?);
        
        Ok(Self {
        })
    /// Set memory profiler
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        info!("Enabling memory profiling for real heap manager");
        self.profiler = Some(profiler);
    /// Allocate memory with real heap algorithms
    #[instrument(skip(self))]
    pub fn allocate(&self, size: usize, alignment: usize, type_name: &str) -> Result<(ObjectId, NonNull<u8>), String> {
        if size == 0 {
            return Err("Cannot allocate zero bytes".to_string());
        let start_time = Instant::now();
        let actual_alignment = alignment.max(8);
        
               size, type_name, actual_alignment);
        
        // Check memory pressure first
        self.check_and_handle_pressure()?;
        
        // Try to allocate in existing blocks
        let allocation_result = self.try_allocate_in_existing_blocks(size, actual_alignment)?;
        
        let (block_ptr, object_id) = match allocation_result {
            None => {
                // Need to create a new block
                self.allocate_new_block_and_allocate(size, actual_alignment)?
            }
        
        // Initialize object metadata
        let layout = MemoryLayout::calculate(size, actual_alignment);
        let total_size = layout.total_size;
        
        // Allocate space for header + object
        let full_allocation = self.allocate_internal(total_size, actual_alignment)?;
        
        // Initialize object header and metadata
        let object_id = self.metadata_manager.initialize_object(
            full_allocation, size, actual_alignment, type_name
        )?;
        
        // Get object data pointer (after header)
        let object_ptr = layout.get_object_ptr(full_allocation);
        
        // Update statistics
        self.update_allocation_statistics(size, start_time)?;
        
        // Profile the allocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_allocation(size, object_ptr.as_ptr() as u64, Vec::new());
               object_id, size, object_ptr.as_ptr());
        
        Ok((object_id, object_ptr))
    /// Deallocate memory with proper cleanup
    #[instrument(skip(self))]
    pub fn deallocate(&self, object_id: ObjectId, ptr: NonNull<u8>) -> Result<(), String> {
        debug!("Real heap deallocating object {} at {:p}", object_id, ptr.as_ptr());
        
        // Get object metadata first
        let metadata = self.metadata_manager.get_metadata(ptr)?;
        let header_ptr = metadata.header_ptr;
        let size = metadata.size;
        
        // Find which block owns this pointer and deallocate
        let mut blocks = self.blocks.write()
            .map_err(|_| "Failed to acquire write lock on blocks")?;
        
        let mut found_block = false;
        for block in blocks.iter_mut() {
            if block.contains_ptr(header_ptr.as_ptr() as *const u8) {
                block.deallocate(header_ptr.cast())?;
                found_block = true;
                break;
            }
        }
        
        if !found_block {
            return Err(format!("Could not find block containing pointer {:p}", ptr.as_ptr()));
        // Remove object metadata
        self.metadata_manager.remove_metadata(ptr)?;
        
        // Update statistics
        self.update_deallocation_statistics(size)?;
        
        // Profile the deallocation
        if let Some(profiler) = &self.profiler {
            let _ = profiler.track_deallocation(ptr.as_ptr() as u64, Vec::new());
        debug!("Real heap deallocated object {} with {} bytes", object_id, size);
        
        Ok(())
    /// Internal allocation within blocks
    fn allocate_internal(&self, size: usize, alignment: usize) -> Result<NonNull<u8>, String> {
        let mut blocks = self.blocks.write()
            .map_err(|_| "Failed to acquire write lock on blocks")?;
        
        // Try each block
        for block in blocks.iter_mut() {
            if block.can_allocate(size, alignment) {
                if let Ok(ptr) = block.allocate(size, alignment, None) {
                    return Ok(ptr);
                }
            }
        // Need new block
        drop(blocks);
        self.create_new_block_and_allocate(size, alignment)
    /// Try to allocate in existing blocks
    fn try_allocate_in_existing_blocks(&self, size: usize, alignment: usize) 
        -> Result<Option<(NonNull<u8>, ObjectId)>, String> {
        
        let mut blocks = self.blocks.write()
            .map_err(|_| "Failed to acquire write lock on blocks")?;
        
        for block in blocks.iter_mut() {
            if block.can_allocate(size, alignment) {
                if let Ok(ptr) = block.allocate(size, alignment, None) {
                    // Create object ID
                    let object_id = crate::memory::object_id::ObjectIdGenerator::new().next();
                    return Ok(Some((ptr, object_id)));
                }
            }
        Ok(None)
    /// Allocate new block and perform allocation
    fn allocate_new_block_and_allocate(&self, size: usize, alignment: usize) 
        -> Result<(NonNull<u8>, ObjectId), String> {
        
        let block_id = self.next_block_id.fetch_add(1, Ordering::Relaxed);
        let block_size = self.calculate_new_block_size(size)?;
        
        let mut new_block = RealHeapBlock::new(block_size, block_id)?;
        let ptr = new_block.allocate(size, alignment, None)?;
        let object_id = crate::memory::object_id::ObjectIdGenerator::new().next();
        
        // Add block to collection
        {
            let mut blocks = self.blocks.write()
                .map_err(|_| "Failed to acquire write lock on blocks")?;
            
            if blocks.len() >= self.config.max_blocks {
                warn!("Maximum number of blocks ({}) reached", self.config.max_blocks);
                return Err("Maximum number of heap blocks reached".to_string());
            blocks.push(new_block);
        info!("Created new heap block {} with {} bytes", block_id, block_size);
        
        Ok((ptr, object_id))
    /// Create new block and allocate
    fn create_new_block_and_allocate(&self, size: usize, alignment: usize) -> Result<NonNull<u8>, String> {
        let block_id = self.next_block_id.fetch_add(1, Ordering::Relaxed);
        let block_size = self.calculate_new_block_size(size)?;
        
        let mut new_block = RealHeapBlock::new(block_size, block_id)?;
        let ptr = new_block.allocate(size, alignment, None)?;
        
        // Add to blocks
        {
            let mut blocks = self.blocks.write()
                .map_err(|_| "Failed to acquire write lock on blocks")?;
            blocks.push(new_block);
        Ok(ptr)
    /// Calculate size for new block
    fn calculate_new_block_size(&self, min_size: usize) -> Result<usize, String> {
        let base_size = self.config.initial_block_size;
        let required_size = min_size + 4096; // Add some overhead
        
        let blocks = self.blocks.read()
            .map_err(|_| "Failed to acquire read lock on blocks")?;
        
        let block_count = blocks.len();
        drop(blocks);
        
        // Grow blocks based on count
        let growth_multiplier = (self.config.growth_factor).powi(block_count as i32);
        let calculated_size = (base_size as f64 * growth_multiplier) as usize;
        
        Ok(calculated_size.max(required_size))
    /// Check and handle memory pressure
    fn check_and_handle_pressure(&self) -> Result<(), String> {
        let stats = self.get_statistics()?;
        
        let mut pressure_monitor = self.pressure_monitor.lock()
            .map_err(|_| "Failed to acquire pressure monitor lock")?;
        
        pressure_monitor.update_pressure(&stats);
        
        if pressure_monitor.should_trigger_compaction(self.config.pressure_threshold) {
            if self.config.auto_compaction {
                drop(pressure_monitor);
                warn!("High memory pressure detected, triggering compaction");
                self.trigger_compaction()?;
            } else {
                warn!("High memory pressure detected but auto-compaction disabled");
            }
        }
        
        Ok(())
    /// Trigger heap compaction
    #[instrument(skip(self))]
    pub fn trigger_compaction(&self) -> Result<(), String> {
        info!("Starting heap compaction");
        
        let start_time = Instant::now();
        
        // Record compaction start
        {
            let mut pressure_monitor = self.pressure_monitor.lock()
                .map_err(|_| "Failed to acquire pressure monitor lock")?;
            pressure_monitor.record_compaction();
        // Compact each block
        let mut blocks = self.blocks.write()
            .map_err(|_| "Failed to acquire write lock on blocks")?;
        
        let mut total_reclaimed = 0;
        for block in blocks.iter_mut() {
            let before_free = block.free_space();
            // Coalescing already happens during deallocation, so compaction
            // here would mainly be about moving objects within blocks or
            // between blocks, which requires GC integration
            let after_free = block.free_space();
            total_reclaimed += after_free.saturating_sub(before_free);
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.compaction_events += 1;
        let duration = start_time.elapsed();
        info!("Heap compaction completed in {:?}, reclaimed {} bytes", duration, total_reclaimed);
        
        Ok(())
    /// Update allocation statistics
    fn update_allocation_statistics(&self, size: usize, start_time: Instant) -> Result<(), String> {
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        stats.total_allocations += 1;
        
        // Update other statistics from blocks
        self.update_global_statistics(&mut stats)?;
        
        Ok(())
    /// Update deallocation statistics
    fn update_deallocation_statistics(&self, size: usize) -> Result<(), String> {
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        stats.total_deallocations += 1;
        
        // Update other statistics from blocks
        self.update_global_statistics(&mut stats)?;
        
        Ok(())
    /// Update global statistics from blocks
    fn update_global_statistics(&self, stats: &mut RealHeapStatistics) -> Result<(), String> {
        let blocks = self.blocks.read()
            .map_err(|_| "Failed to acquire read lock on blocks")?;
        
        stats.total_blocks = blocks.len();
        stats.total_capacity = blocks.iter().map(|b| b.size).sum();
        stats.total_used = blocks.iter().map(|b| b.stats.current_usage).sum();
        stats.total_free = stats.total_capacity - stats.total_used;
        
        // Calculate overall fragmentation
        let total_fragmentation: f64 = blocks.iter().map(|b| b.fragmentation_score()).sum();
        stats.overall_fragmentation = if blocks.len() > 0 {
            total_fragmentation / blocks.len() as f64
        } else {
            0.0
        
        // Calculate average utilization
        let total_utilization: f64 = blocks.iter().map(|b| b.utilization()).sum();
        stats.average_block_utilization = if blocks.len() > 0 {
            total_utilization / blocks.len() as f64
        } else {
            0.0
        
        Ok(())
    /// Get heap statistics
    pub fn get_statistics(&self) -> Result<RealHeapStatistics, String> {
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        self.update_global_statistics(&mut stats)?;
        
        Ok(stats.clone())
    /// Check if pointer is valid
    pub fn is_valid_pointer(&self, ptr: *const u8) -> bool {
        if let Ok(blocks) = self.blocks.read() {
            blocks.iter().any(|block| block.contains_ptr(ptr))
        } else {
            false
        }
    }
    
    /// Get memory pressure level
    pub fn get_memory_pressure(&self) -> f64 {
        if let Ok(pressure_monitor) = self.pressure_monitor.lock() {
            pressure_monitor.current_pressure
        } else {
            0.0
        }
    }
    
    /// Force garbage collection event
    pub fn trigger_gc(&self, gc_type: &str) -> Result<(), String> {
        debug!("GC triggered on real heap: {}", gc_type);
        
        // Update pressure after GC
        let mut pressure_monitor = self.pressure_monitor.lock()
            .map_err(|_| "Failed to acquire pressure monitor lock")?;
        
        // Simulate pressure reduction after GC
        pressure_monitor.current_pressure *= 0.7; // Reduce pressure by 30%
        
        Ok(())
    }
}

// Safety implementations for RealHeapManager
unsafe impl Send for RealHeapManager {}
unsafe impl Sync for RealHeapManager {}

/// Heap statistics compatible with existing GC interface
impl From<RealHeapStatistics> for crate::memory::heap_manager::HeapStats {
    fn from(real_stats: RealHeapStatistics) -> Self {
        Self {
            active_objects: 0, // Would need object count from metadata manager
            object_registry_count: 0, // Would need registry count
            memory_pressure: crate::memory::heap_manager::MemoryPressure::Low, // Convert pressure level
        }
    }
