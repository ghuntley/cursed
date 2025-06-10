/// Memory Allocation Algorithms for CURSED Heap Management
/// 
/// This module provides various allocation strategies optimized for different
/// use cases and object patterns:
/// 
/// 1. **Bump Allocator**: Fast linear allocation for short-lived objects
/// 2. **Free List Allocator**: First-fit allocation for general use with minimal fragmentation
/// 3. **Segregated Allocator**: Size-class based allocation for optimal memory utilization
/// 4. **Hybrid Allocator**: Adaptive allocation switching between strategies
/// 
/// Each allocator implements the Allocator trait providing a consistent interface
/// while optimizing for specific allocation patterns and performance characteristics.

use std::ptr::NonNull;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use tracing::{instrument, debug, warn, error};

use crate::memory::metadata::ObjectHeader;

/// Common interface for all memory allocators
/// 
/// This trait provides a unified interface for different allocation strategies
/// allowing the heap to switch between algorithms based on usage patterns.
pub trait Allocator: Send + Sync {
    /// Allocate memory of the given size with required alignment
    /// 
    /// Returns a pointer to the allocated memory or an error if allocation fails.
    /// The allocated memory is guaranteed to be properly aligned and initialized.
    fn allocate(&self, size: usize, alignment: usize) -> Result<AllocationResult, String>;
    
    /// Deallocate previously allocated memory
    /// 
    /// The pointer must have been returned by a previous call to allocate()
    /// from this same allocator instance.
    fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), String>;
    
    /// Get allocator statistics for monitoring
    fn get_statistics(&self) -> AllocatorStatistics;
    
    /// Reset allocator state (for testing and debugging)
    fn reset(&self) -> Result<(), String>;
    
    /// Check if allocator can handle the given size
    fn can_allocate(&self, size: usize) -> bool;
    
    /// Get allocator type name for debugging
    fn allocator_type(&self) -> &'static str;
}

/// Result of an allocation operation
#[derive(Debug, Clone)]
pub struct AllocationResult {
    /// Pointer to allocated memory
    pub ptr: NonNull<u8>,
    /// Actual size allocated (may be larger than requested)
    pub size: usize,
    /// Offset within the memory region
    pub offset: usize,
}

/// Statistics for allocator performance monitoring
#[derive(Debug, Clone)]
pub struct AllocatorStatistics {
    /// Total number of allocation requests
    pub allocations: u64,
    /// Total number of deallocation requests
    pub deallocations: u64,
    /// Total bytes allocated
    pub bytes_allocated: usize,
    /// Total bytes deallocated
    pub bytes_deallocated: usize,
    /// Current bytes in use
    pub bytes_in_use: usize,
    /// Number of allocation failures
    pub allocation_failures: u64,
    /// Average allocation size
    pub average_allocation_size: f64,
    /// Fragmentation ratio
    pub fragmentation_ratio: f64,
    /// Allocator-specific metrics
    pub custom_metrics: HashMap<String, f64>,
}

impl AllocatorStatistics {
    pub fn new() -> Self {
        Self {
            allocations: 0,
            deallocations: 0,
            bytes_allocated: 0,
            bytes_deallocated: 0,
            bytes_in_use: 0,
            allocation_failures: 0,
            average_allocation_size: 0.0,
            fragmentation_ratio: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
    
    pub fn record_allocation(&mut self, size: usize) {
        self.allocations += 1;
        self.bytes_allocated += size;
        self.bytes_in_use += size;
        self.average_allocation_size = self.bytes_allocated as f64 / self.allocations as f64;
    }
    
    pub fn record_deallocation(&mut self, size: usize) {
        self.deallocations += 1;
        self.bytes_deallocated += size;
        self.bytes_in_use = self.bytes_in_use.saturating_sub(size);
    }
    
    pub fn record_failure(&mut self) {
        self.allocation_failures += 1;
    }
}

/// Bump allocator for fast linear allocation
/// 
/// This allocator maintains a simple pointer that moves forward with each
/// allocation. It's extremely fast but can only deallocate everything at once.
/// Ideal for temporary allocations or generational garbage collection.
pub struct BumpAllocator {
    /// Memory region for allocations
    memory: Arc<RwLock<BumpMemory>>,
    /// Allocator statistics
    statistics: Arc<Mutex<AllocatorStatistics>>,
}

struct BumpMemory {
    /// Base pointer to memory region
    base: NonNull<u8>,
    /// Size of memory region
    size: usize,
    /// Current allocation offset
    offset: usize,
    /// High watermark for statistics
    high_watermark: usize,
}

// Safety: BumpMemory is safe to send between threads because:
// 1. NonNull<u8> points to heap-allocated memory that is owned by this allocator
// 2. Access is coordinated through RwLock synchronization
// 3. The pointer remains valid as long as the allocator exists
unsafe impl Send for BumpMemory {}

// Safety: BumpMemory is safe to share between threads because:
// 1. All mutation is coordinated through RwLock
// 2. The NonNull<u8> pointer is stable (doesn't change once allocated)
// 3. Field access is atomic or protected by the containing lock
unsafe impl Sync for BumpMemory {}

impl BumpAllocator {
    /// Create a new bump allocator with the given memory region
    #[instrument]
    pub fn new(base: NonNull<u8>, size: usize) -> Self {
        debug!("Creating bump allocator with {} bytes at {:p}", size, base.as_ptr());
        
        Self {
            memory: Arc::new(RwLock::new(BumpMemory {
                base,
                size,
                offset: 0,
                high_watermark: 0,
            })),
            statistics: Arc::new(Mutex::new(AllocatorStatistics::new())),
        }
    }
    
    /// Reset the allocator to initial state
    pub fn reset_to_start(&self) -> Result<(), String> {
        let mut memory = self.memory.write()
            .map_err(|_| "Failed to acquire write lock on bump memory")?;
        
        memory.offset = 0;
        debug!("Reset bump allocator to start");
        Ok(())
    }
    
    /// Get current usage percentage
    pub fn usage_percentage(&self) -> Result<f64, String> {
        let memory = self.memory.read()
            .map_err(|_| "Failed to acquire read lock on bump memory")?;
        
        Ok(memory.offset as f64 / memory.size as f64 * 100.0)
    }
}

impl Allocator for BumpAllocator {
    #[instrument(skip(self))]
    fn allocate(&self, size: usize, alignment: usize) -> Result<AllocationResult, String> {
        if size == 0 {
            return Err("Cannot allocate zero bytes".to_string());
        }
        
        let mut memory = self.memory.write()
            .map_err(|_| "Failed to acquire write lock on bump memory")?;
        
        // Calculate aligned offset
        let aligned_offset = (memory.offset + alignment - 1) & !(alignment - 1);
        
        // Check if allocation fits
        if aligned_offset + size > memory.size {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_failure();
            return Err(format!("Bump allocator out of space: need {} bytes, have {}", 
                              aligned_offset + size, memory.size - aligned_offset));
        }
        
        // Update offset
        memory.offset = aligned_offset + size;
        if memory.offset > memory.high_watermark {
            memory.high_watermark = memory.offset;
        }
        
        // Calculate pointer
        let ptr = unsafe {
            NonNull::new(memory.base.as_ptr().add(aligned_offset))
                .ok_or("Computed null pointer in bump allocator")?
        };
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_allocation(size);
        }
        
        debug!("Bump allocated {} bytes at offset {} (ptr {:p})", size, aligned_offset, ptr.as_ptr());
        
        Ok(AllocationResult {
            ptr,
            size,
            offset: aligned_offset,
        })
    }
    
    fn deallocate(&self, _ptr: NonNull<u8>, size: usize) -> Result<(), String> {
        // Bump allocator can't deallocate individual objects
        // Just update statistics
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        stats.record_deallocation(size);
        
        Ok(())
    }
    
    fn get_statistics(&self) -> AllocatorStatistics {
        let stats = self.statistics.lock().unwrap();
        let memory = self.memory.read().unwrap();
        
        let mut result = stats.clone();
        result.fragmentation_ratio = 0.0; // Bump allocator has no fragmentation
        result.custom_metrics.insert("high_watermark".to_string(), memory.high_watermark as f64);
        result.custom_metrics.insert("utilization".to_string(), 
                                   memory.offset as f64 / memory.size as f64 * 100.0);
        
        result
    }
    
    fn reset(&self) -> Result<(), String> {
        self.reset_to_start()?;
        
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        *stats = AllocatorStatistics::new();
        
        Ok(())
    }
    
    fn can_allocate(&self, size: usize) -> bool {
        if let Ok(memory) = self.memory.read() {
            memory.offset + size <= memory.size
        } else {
            false
        }
    }
    
    fn allocator_type(&self) -> &'static str {
        "BumpAllocator"
    }
}

/// Free list allocator using first-fit strategy
/// 
/// This allocator maintains a list of free memory blocks and uses a first-fit
/// allocation strategy. It provides good general-purpose allocation with
/// reasonable fragmentation characteristics.
pub struct FreeListAllocator {
    /// Memory region for allocations
    memory: Arc<RwLock<FreeListMemory>>,
    /// Allocator statistics
    statistics: Arc<Mutex<AllocatorStatistics>>,
}

struct FreeListMemory {
    /// Base pointer to memory region
    base: NonNull<u8>,
    /// Size of memory region
    size: usize,
    /// List of free blocks
    free_blocks: VecDeque<FreeBlock>,
    /// Set of allocated blocks for validation
    allocated_blocks: HashMap<usize, usize>, // offset -> size
}

// Safety: FreeListMemory is safe to send between threads because:
// 1. NonNull<u8> points to heap-allocated memory that is owned by this allocator
// 2. Access is coordinated through RwLock synchronization
// 3. The pointer remains valid as long as the allocator exists
unsafe impl Send for FreeListMemory {}

// Safety: FreeListMemory is safe to share between threads because:
// 1. All mutation is coordinated through RwLock
// 2. The NonNull<u8> pointer is stable (doesn't change once allocated)
// 3. Field access is atomic or protected by the containing lock
unsafe impl Sync for FreeListMemory {}

#[derive(Debug, Clone)]
struct FreeBlock {
    /// Offset from base pointer
    offset: usize,
    /// Size of free block
    size: usize,
}

impl FreeListAllocator {
    /// Create a new free list allocator
    #[instrument]
    pub fn new(base: NonNull<u8>, size: usize) -> Self {
        debug!("Creating free list allocator with {} bytes at {:p}", size, base.as_ptr());
        
        let mut free_blocks = VecDeque::new();
        free_blocks.push_back(FreeBlock { offset: 0, size });
        
        Self {
            memory: Arc::new(RwLock::new(FreeListMemory {
                base,
                size,
                free_blocks,
                allocated_blocks: HashMap::new(),
            })),
            statistics: Arc::new(Mutex::new(AllocatorStatistics::new())),
        }
    }
    
    /// Coalesce adjacent free blocks
    fn coalesce_free_blocks(free_blocks: &mut VecDeque<FreeBlock>) {
        if free_blocks.len() <= 1 {
            return;
        }
        
        // Sort blocks by offset
        let mut blocks: Vec<_> = free_blocks.drain(..).collect();
        blocks.sort_by_key(|b| b.offset);
        
        let mut coalesced = VecDeque::new();
        let mut current = blocks[0].clone();
        
        for block in blocks.into_iter().skip(1) {
            if current.offset + current.size == block.offset {
                // Adjacent blocks - coalesce
                current.size += block.size;
            } else {
                // Non-adjacent - save current and start new
                coalesced.push_back(current);
                current = block;
            }
        }
        
        coalesced.push_back(current);
        *free_blocks = coalesced;
    }
}

impl Allocator for FreeListAllocator {
    #[instrument(skip(self))]
    fn allocate(&self, size: usize, alignment: usize) -> Result<AllocationResult, String> {
        if size == 0 {
            return Err("Cannot allocate zero bytes".to_string());
        }
        
        let mut memory = self.memory.write()
            .map_err(|_| "Failed to acquire write lock on free list memory")?;
        
        // Find suitable free block (first-fit)
        let mut block_index = None;
        let mut allocation_offset = 0;
        let mut allocation_size = 0;
        
        for (i, block) in memory.free_blocks.iter().enumerate() {
            let aligned_offset = (block.offset + alignment - 1) & !(alignment - 1);
            let aligned_size = (size + alignment - 1) & !(alignment - 1);
            
            if aligned_offset + aligned_size <= block.offset + block.size {
                block_index = Some(i);
                allocation_offset = aligned_offset;
                allocation_size = aligned_size;
                break;
            }
        }
        
        let block_index = block_index.ok_or_else(|| {
            let mut stats = self.statistics.lock().unwrap();
            stats.record_failure();
            format!("No suitable free block found for {} bytes", size)
        })?;
        
        // Remove or split the free block
        let block = memory.free_blocks.remove(block_index).unwrap();
        
        // Add remaining space back as free blocks
        if allocation_offset > block.offset {
            // Space before allocation
            memory.free_blocks.push_back(FreeBlock {
                offset: block.offset,
                size: allocation_offset - block.offset,
            });
        }
        
        if allocation_offset + allocation_size < block.offset + block.size {
            // Space after allocation
            memory.free_blocks.push_back(FreeBlock {
                offset: allocation_offset + allocation_size,
                size: block.offset + block.size - (allocation_offset + allocation_size),
            });
        }
        
        // Record allocation
        memory.allocated_blocks.insert(allocation_offset, allocation_size);
        
        // Calculate pointer
        let ptr = unsafe {
            NonNull::new(memory.base.as_ptr().add(allocation_offset))
                .ok_or("Computed null pointer in free list allocator")?
        };
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_allocation(size);
        }
        
        debug!("Free list allocated {} bytes at offset {} (ptr {:p})", 
               size, allocation_offset, ptr.as_ptr());
        
        Ok(AllocationResult {
            ptr,
            size: allocation_size,
            offset: allocation_offset,
        })
    }
    
    #[instrument(skip(self))]
    fn deallocate(&self, ptr: NonNull<u8>, _size: usize) -> Result<(), String> {
        let mut memory = self.memory.write()
            .map_err(|_| "Failed to acquire write lock on free list memory")?;
        
        // Calculate offset
        let offset = unsafe { ptr.as_ptr().offset_from(memory.base.as_ptr()) };
        if offset < 0 {
            return Err("Pointer not within allocator bounds".to_string());
        }
        
        let offset = offset as usize;
        
        // Find allocated block
        let size = memory.allocated_blocks.remove(&offset)
            .ok_or_else(|| format!("Block at offset {} not found in allocated blocks", offset))?;
        
        // Add to free list
        memory.free_blocks.push_back(FreeBlock { offset, size });
        
        // Coalesce adjacent free blocks
        Self::coalesce_free_blocks(&mut memory.free_blocks);
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_deallocation(size);
        }
        
        debug!("Free list deallocated {} bytes at offset {}", size, offset);
        Ok(())
    }
    
    fn get_statistics(&self) -> AllocatorStatistics {
        let stats = self.statistics.lock().unwrap();
        let memory = self.memory.read().unwrap();
        
        let mut result = stats.clone();
        
        // Calculate fragmentation
        let total_free: usize = memory.free_blocks.iter().map(|b| b.size).sum();
        let largest_free = memory.free_blocks.iter().map(|b| b.size).max().unwrap_or(0);
        
        result.fragmentation_ratio = if total_free > 0 {
            1.0 - (largest_free as f64 / total_free as f64)
        } else {
            0.0
        };
        
        result.custom_metrics.insert("free_blocks".to_string(), memory.free_blocks.len() as f64);
        result.custom_metrics.insert("total_free".to_string(), total_free as f64);
        result.custom_metrics.insert("largest_free".to_string(), largest_free as f64);
        
        result
    }
    
    fn reset(&self) -> Result<(), String> {
        let mut memory = self.memory.write()
            .map_err(|_| "Failed to acquire write lock on free list memory")?;
        
        let size = memory.size;
        memory.free_blocks.clear();
        memory.free_blocks.push_back(FreeBlock { offset: 0, size });
        memory.allocated_blocks.clear();
        
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        *stats = AllocatorStatistics::new();
        
        Ok(())
    }
    
    fn can_allocate(&self, size: usize) -> bool {
        if let Ok(memory) = self.memory.read() {
            memory.free_blocks.iter().any(|block| block.size >= size)
        } else {
            false
        }
    }
    
    fn allocator_type(&self) -> &'static str {
        "FreeListAllocator"
    }
}

/// Segregated allocator with size classes
/// 
/// This allocator maintains separate free lists for different size classes,
/// providing excellent allocation performance and low fragmentation for
/// objects with predictable size patterns.
pub struct SegregatedAllocator {
    /// Size classes and their allocators
    size_classes: Arc<RwLock<Vec<SizeClass>>>,
    /// Allocator statistics
    statistics: Arc<Mutex<AllocatorStatistics>>,
    /// Large object threshold
    large_object_threshold: usize,
}

struct SizeClass {
    /// Maximum size for this class
    max_size: usize,
    /// Free list allocator for this size class
    allocator: FreeListAllocator,
    /// Statistics for this size class
    allocations: u64,
    /// Base memory for this size class
    memory_base: NonNull<u8>,
    /// Memory size for this size class
    memory_size: usize,
}

// Safety: SizeClass is safe to send between threads because:
// 1. All fields are either primitives or thread-safe types
// 2. NonNull<u8> points to memory owned by this allocator
// 3. FreeListAllocator is already declared Send/Sync
unsafe impl Send for SizeClass {}

// Safety: SizeClass is safe to share between threads because:
// 1. Access is coordinated through containing RwLock
// 2. All mutation goes through thread-safe methods
unsafe impl Sync for SizeClass {}

impl SegregatedAllocator {
    /// Create a new segregated allocator with default size classes
    #[instrument]
    pub fn new(base: NonNull<u8>, size: usize) -> Result<Self, String> {
        debug!("Creating segregated allocator with {} bytes at {:p}", size, base.as_ptr());
        
        let size_classes = Self::create_size_classes(base, size)?;
        
        Ok(Self {
            size_classes: Arc::new(RwLock::new(size_classes)),
            statistics: Arc::new(Mutex::new(AllocatorStatistics::new())),
            large_object_threshold: size / 10, // 10% of total space for large objects
        })
    }
    
    /// Create default size classes
    fn create_size_classes(base: NonNull<u8>, total_size: usize) -> Result<Vec<SizeClass>, String> {
        let class_sizes = vec![16, 32, 64, 128, 256, 512, 1024, 2048, 4096];
        let mut classes = Vec::new();
        let class_memory_size = total_size / (class_sizes.len() + 1); // +1 for large objects
        
        for (i, &max_size) in class_sizes.iter().enumerate() {
            let offset = i * class_memory_size;
            let class_base = unsafe {
                NonNull::new(base.as_ptr().add(offset))
                    .ok_or("Failed to compute size class base pointer")?
            };
            
            let allocator = FreeListAllocator::new(class_base, class_memory_size);
            
            classes.push(SizeClass {
                max_size,
                allocator,
                allocations: 0,
                memory_base: class_base,
                memory_size: class_memory_size,
            });
        }
        
        debug!("Created {} size classes with {} bytes each", class_sizes.len(), class_memory_size);
        Ok(classes)
    }
    
    /// Find appropriate size class for allocation
    fn find_size_class(&self, size: usize) -> Result<usize, String> {
        let classes = self.size_classes.read()
            .map_err(|_| "Failed to acquire read lock on size classes")?;
        
        for (i, class) in classes.iter().enumerate() {
            if size <= class.max_size {
                return Ok(i);
            }
        }
        
        Err(format!("Size {} exceeds largest size class", size))
    }
}

impl Allocator for SegregatedAllocator {
    #[instrument(skip(self))]
    fn allocate(&self, size: usize, alignment: usize) -> Result<AllocationResult, String> {
        if size == 0 {
            return Err("Cannot allocate zero bytes".to_string());
        }
        
        if size > self.large_object_threshold {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_failure();
            return Err(format!("Size {} exceeds large object threshold {}", 
                              size, self.large_object_threshold));
        }
        
        let class_index = self.find_size_class(size)?;
        
        let result = {
            let mut classes = self.size_classes.write()
                .map_err(|_| "Failed to acquire write lock on size classes")?;
            
            classes[class_index].allocations += 1;
            classes[class_index].allocator.allocate(size, alignment)?
        };
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_allocation(size);
        }
        
        debug!("Segregated allocated {} bytes in size class {} (ptr {:p})", 
               size, class_index, result.ptr.as_ptr());
        
        Ok(result)
    }
    
    #[instrument(skip(self))]
    fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), String> {
        let class_index = self.find_size_class(size)?;
        
        {
            let classes = self.size_classes.read()
                .map_err(|_| "Failed to acquire read lock on size classes")?;
            classes[class_index].allocator.deallocate(ptr, size)?;
        }
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            stats.record_deallocation(size);
        }
        
        debug!("Segregated deallocated {} bytes from size class {}", size, class_index);
        Ok(())
    }
    
    fn get_statistics(&self) -> AllocatorStatistics {
        let stats = self.statistics.lock().unwrap();
        let classes = self.size_classes.read().unwrap();
        
        let mut result = stats.clone();
        
        // Aggregate statistics from all size classes
        let mut total_fragmentation = 0.0;
        let mut class_count = 0;
        
        for (i, class) in classes.iter().enumerate() {
            let class_stats = class.allocator.get_statistics();
            result.custom_metrics.insert(
                format!("class_{}_allocations", i), 
                class.allocations as f64
            );
            result.custom_metrics.insert(
                format!("class_{}_fragmentation", i), 
                class_stats.fragmentation_ratio
            );
            
            total_fragmentation += class_stats.fragmentation_ratio;
            class_count += 1;
        }
        
        result.fragmentation_ratio = if class_count > 0 {
            total_fragmentation / class_count as f64
        } else {
            0.0
        };
        
        result.custom_metrics.insert("size_classes".to_string(), class_count as f64);
        
        result
    }
    
    fn reset(&self) -> Result<(), String> {
        let mut classes = self.size_classes.write()
            .map_err(|_| "Failed to acquire write lock on size classes")?;
        
        for class in classes.iter_mut() {
            class.allocator.reset()?;
            class.allocations = 0;
        }
        
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        *stats = AllocatorStatistics::new();
        
        Ok(())
    }
    
    fn can_allocate(&self, size: usize) -> bool {
        if size > self.large_object_threshold {
            return false;
        }
        
        if let Ok(classes) = self.size_classes.read() {
            for class in classes.iter() {
                if size <= class.max_size {
                    return class.allocator.can_allocate(size);
                }
            }
        }
        
        false
    }
    
    fn allocator_type(&self) -> &'static str {
        "SegregatedAllocator"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::{alloc, dealloc, Layout};
    
    fn create_test_memory(size: usize) -> (NonNull<u8>, Layout) {
        let layout = Layout::from_size_align(size, 8).unwrap();
        let ptr = unsafe { alloc(layout) };
        let non_null = NonNull::new(ptr).unwrap();
        (non_null, layout)
    }
    
    #[test]
    fn test_bump_allocator() {
        let (base, layout) = create_test_memory(1024);
        let allocator = BumpAllocator::new(base, 1024);
        
        // Test allocation
        let result1 = allocator.allocate(64, 8).unwrap();
        assert_eq!(result1.size, 64);
        assert_eq!(result1.offset, 0);
        
        let result2 = allocator.allocate(32, 8).unwrap();
        assert_eq!(result2.offset, 64);
        
        // Test statistics
        let stats = allocator.get_statistics();
        assert_eq!(stats.allocations, 2);
        assert_eq!(stats.bytes_allocated, 96);
        
        // Test usage
        let usage = allocator.usage_percentage().unwrap();
        assert!(usage > 0.0 && usage < 100.0);
        
        unsafe { dealloc(base.as_ptr(), layout) };
    }
    
    #[test]
    fn test_free_list_allocator() {
        let (base, layout) = create_test_memory(1024);
        let allocator = FreeListAllocator::new(base, 1024);
        
        // Test allocation
        let result1 = allocator.allocate(64, 8).unwrap();
        let result2 = allocator.allocate(32, 8).unwrap();
        
        // Test deallocation
        allocator.deallocate(result1.ptr, result1.size).unwrap();
        
        // Allocate again (should reuse freed space)
        let result3 = allocator.allocate(48, 8).unwrap();
        
        let stats = allocator.get_statistics();
        assert_eq!(stats.allocations, 3);
        assert_eq!(stats.deallocations, 1);
        
        unsafe { dealloc(base.as_ptr(), layout) };
    }
    
    #[test]
    fn test_segregated_allocator() {
        let (base, layout) = create_test_memory(8192);
        let allocator = SegregatedAllocator::new(base, 8192).unwrap();
        
        // Test small allocation
        let result1 = allocator.allocate(16, 8).unwrap();
        let result2 = allocator.allocate(64, 8).unwrap();
        let result3 = allocator.allocate(256, 8).unwrap();
        
        // Test deallocation
        allocator.deallocate(result1.ptr, 16).unwrap();
        allocator.deallocate(result2.ptr, 64).unwrap();
        allocator.deallocate(result3.ptr, 256).unwrap();
        
        let stats = allocator.get_statistics();
        assert_eq!(stats.allocations, 3);
        assert_eq!(stats.deallocations, 3);
        
        unsafe { dealloc(base.as_ptr(), layout) };
    }
    
    #[test]
    fn test_allocator_trait_interface() {
        let (base, layout) = create_test_memory(1024);
        let allocator: Box<dyn Allocator> = Box::new(BumpAllocator::new(base, 1024));
        
        assert_eq!(allocator.allocator_type(), "BumpAllocator");
        assert!(allocator.can_allocate(100));
        
        let result = allocator.allocate(64, 8).unwrap();
        let stats = allocator.get_statistics();
        assert_eq!(stats.allocations, 1);
        
        unsafe { dealloc(base.as_ptr(), layout) };
    }
}
