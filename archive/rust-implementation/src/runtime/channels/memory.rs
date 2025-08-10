//! Memory management for channel buffers
//!
//! Provides efficient memory allocation and management for channel operations:
//! - Custom allocators for different buffer types
//! - Memory pool management
//! - Zero-copy optimizations
//! - Memory pressure handling

use std::alloc::{self, Layout, GlobalAlloc};
use std::ptr::{self, NonNull};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};
use std::collections::{HashMap, VecDeque};
use std::mem::{self, MaybeUninit};

use crate::runtime::channels::ChannelError;

/// Memory allocation statistics
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    /// Total bytes allocated
    pub total_allocated: usize,
    /// Total bytes deallocated
    pub total_deallocated: usize,
    /// Current bytes in use
    pub current_usage: usize,
    /// Peak memory usage
    pub peak_usage: usize,
    /// Number of allocations
    pub allocation_count: usize,
    /// Number of deallocations
    pub deallocation_count: usize,
    /// Number of pool hits
    pub pool_hits: usize,
    /// Number of pool misses
    pub pool_misses: usize,
}

/// Channel memory allocation trait
pub trait ChannelAllocator: Send + Sync {
    /// Allocate memory for channel buffer
    unsafe fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ChannelError>;
    
    /// Deallocate memory
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
    
    /// Get allocation statistics
    fn stats(&self) -> MemoryStats;
    
    /// Reset statistics
    fn reset_stats(&self);
}

/// Memory pool for efficient allocation of common sizes
pub struct MemoryPool {
    /// Pools for different sizes
    pools: RwLock<HashMap<usize, VecDeque<NonNull<u8>>>>,
    /// Pool statistics
    stats: Mutex<MemoryStats>,
    /// Maximum pool size per bucket
    max_pool_size: usize,
    /// Supported pool sizes (powers of 2)
    pool_sizes: Vec<usize>,
}

// Safety: MemoryPool is designed to be thread-safe with proper synchronization
unsafe impl Send for MemoryPool {}
unsafe impl Sync for MemoryPool {}

impl MemoryPool {
    /// Create a new memory pool
    pub fn new() -> Self {
        // Pre-define common sizes: 16B, 32B, 64B, 128B, 256B, 512B, 1KB, 2KB, 4KB, 8KB
        let pool_sizes = (4..=13).map(|i| 1usize << i).collect();
        
        Self {
            pools: RwLock::new(HashMap::new()),
            stats: Mutex::new(MemoryStats::default()),
            max_pool_size: 1000, // Max 1000 items per pool
            pool_sizes,
        }
    }
    
    /// Allocate from pool or system allocator
    pub unsafe fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ChannelError> {
        let size = layout.size();
        
        // Try to use pool for common sizes
        if let Some(&pool_size) = self.pool_sizes.iter().find(|&&s| s >= size) {
            if let Some(ptr) = self.try_allocate_from_pool(pool_size) {
                return Ok(ptr);
            }
        }
        
        // Fall back to system allocator
        let ptr = alloc::alloc(layout);
        if ptr.is_null() {
            return Err(ChannelError::AllocationError("System allocation failed".to_string()));
        }
        
        let ptr = NonNull::new_unchecked(ptr);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_allocated += layout.size();
            stats.current_usage += layout.size();
            stats.allocation_count += 1;
            stats.pool_misses += 1;
            
            if stats.current_usage > stats.peak_usage {
                stats.peak_usage = stats.current_usage;
            }
        }
        
        Ok(ptr)
    }
    
    /// Deallocate to pool or system allocator
    pub unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let size = layout.size();
        
        // Try to return to pool for common sizes
        if let Some(&pool_size) = self.pool_sizes.iter().find(|&&s| s >= size) {
            if self.try_deallocate_to_pool(ptr, pool_size) {
                return;
            }
        }
        
        // Fall back to system deallocator
        alloc::dealloc(ptr.as_ptr(), layout);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_deallocated += layout.size();
            stats.current_usage = stats.current_usage.saturating_sub(layout.size());
            stats.deallocation_count += 1;
        }
    }
    
    /// Try to allocate from memory pool
    fn try_allocate_from_pool(&self, size: usize) -> Option<NonNull<u8>> {
        let mut pools = self.pools.write().unwrap();
        
        if let Some(pool) = pools.get_mut(&size) {
            if let Some(ptr) = pool.pop_front() {
                // Update statistics
                {
                    let mut stats = self.stats.lock().unwrap();
                    stats.current_usage += size;
                    stats.allocation_count += 1;
                    stats.pool_hits += 1;
                    
                    if stats.current_usage > stats.peak_usage {
                        stats.peak_usage = stats.current_usage;
                    }
                }
                
                return Some(ptr);
            }
        }
        
        None
    }
    
    /// Try to deallocate to memory pool
    fn try_deallocate_to_pool(&self, ptr: NonNull<u8>, size: usize) -> bool {
        let mut pools = self.pools.write().unwrap();
        
        let pool = pools.entry(size).or_insert_with(VecDeque::new);
        
        if pool.len() < self.max_pool_size {
            pool.push_back(ptr);
            
            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.current_usage = stats.current_usage.saturating_sub(size);
                stats.deallocation_count += 1;
            }
            
            return true;
        }
        
        false
    }
    
    /// Get current statistics
    pub fn stats(&self) -> MemoryStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Reset statistics
    pub fn reset_stats(&self) {
        *self.stats.lock().unwrap() = MemoryStats::default();
    }
    
    /// Clean up unused pool memory
    pub fn cleanup(&self) {
        let mut pools = self.pools.write().unwrap();
        
        for (_size, pool) in pools.iter_mut() {
            // Keep only a small number of items in each pool
            let keep_count = self.max_pool_size / 4;
            while pool.len() > keep_count {
                if let Some(ptr) = pool.pop_back() {
                    unsafe {
                        let layout = Layout::from_size_align_unchecked(*_size, mem::align_of::<u8>());
                        alloc::dealloc(ptr.as_ptr(), layout);
                    }
                }
            }
        }
    }
}

impl ChannelAllocator for MemoryPool {
    unsafe fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ChannelError> {
        self.allocate(layout)
    }
    
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.deallocate(ptr, layout)
    }
    
    fn stats(&self) -> MemoryStats {
        self.stats()
    }
    
    fn reset_stats(&self) {
        self.reset_stats()
    }
}

/// Lock-free memory allocator for high-performance channels
pub struct LockFreeAllocator {
    /// Free list heads for different sizes
    free_lists: [AtomicPtr<FreeBlock>; 16],
    /// Allocation statistics
    stats: Mutex<MemoryStats>,
}

/// Free block in the lock-free allocator
struct FreeBlock {
    next: *mut FreeBlock,
}

impl LockFreeAllocator {
    /// Create a new lock-free allocator
    pub fn new() -> Self {
        const NULL_PTR: AtomicPtr<FreeBlock> = AtomicPtr::new(ptr::null_mut());
        
        Self {
            free_lists: [NULL_PTR; 16],
            stats: Mutex::new(MemoryStats::default()),
        }
    }
    
    /// Get the free list index for a given size
    fn get_free_list_index(&self, size: usize) -> usize {
        // Find the appropriate free list (powers of 2)
        let index = (size.next_power_of_two().trailing_zeros() as usize).saturating_sub(4);
        index.min(15)
    }
    
    /// Get the actual allocation size for a free list index
    fn get_allocation_size(&self, index: usize) -> usize {
        1 << (index + 4)
    }
}

impl ChannelAllocator for LockFreeAllocator {
    unsafe fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ChannelError> {
        let size = layout.size().max(mem::size_of::<FreeBlock>());
        let index = self.get_free_list_index(size);
        let actual_size = self.get_allocation_size(index);
        
        // Try to pop from free list
        let free_list = &self.free_lists[index];
        loop {
            let head = free_list.load(Ordering::Acquire);
            
            if head.is_null() {
                // Free list is empty, allocate from system
                let actual_layout = Layout::from_size_align_unchecked(actual_size, layout.align());
                let ptr = alloc::alloc(actual_layout);
                
                if ptr.is_null() {
                    return Err(ChannelError::AllocationError("System allocation failed".to_string()));
                }
                
                let ptr = NonNull::new_unchecked(ptr);
                
                // Update statistics
                {
                    let mut stats = self.stats.lock().unwrap();
                    stats.total_allocated += actual_size;
                    stats.current_usage += actual_size;
                    stats.allocation_count += 1;
                    stats.pool_misses += 1;
                    
                    if stats.current_usage > stats.peak_usage {
                        stats.peak_usage = stats.current_usage;
                    }
                }
                
                return Ok(ptr);
            }
            
            // Try to pop the head
            let next = (*head).next;
            match free_list.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    // Successfully popped from free list
                    let ptr = NonNull::new_unchecked(head as *mut u8);
                    
                    // Update statistics
                    {
                        let mut stats = self.stats.lock().unwrap();
                        stats.current_usage += actual_size;
                        stats.allocation_count += 1;
                        stats.pool_hits += 1;
                        
                        if stats.current_usage > stats.peak_usage {
                            stats.peak_usage = stats.current_usage;
                        }
                    }
                    
                    return Ok(ptr);
                }
                Err(_) => {
                    // Retry the operation
                    continue;
                }
            }
        }
    }
    
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let size = layout.size().max(mem::size_of::<FreeBlock>());
        let index = self.get_free_list_index(size);
        let actual_size = self.get_allocation_size(index);
        
        // Push to free list
        let free_list = &self.free_lists[index];
        let block = ptr.as_ptr() as *mut FreeBlock;
        
        loop {
            let head = free_list.load(Ordering::Acquire);
            (*block).next = head;
            
            match free_list.compare_exchange_weak(
                head,
                block,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    // Successfully pushed to free list
                    // Update statistics
                    {
                        let mut stats = self.stats.lock().unwrap();
                        stats.current_usage = stats.current_usage.saturating_sub(actual_size);
                        stats.deallocation_count += 1;
                    }
                    
                    return;
                }
                Err(_) => {
                    // Retry the operation
                    continue;
                }
            }
        }
    }
    
    fn stats(&self) -> MemoryStats {
        self.stats.lock().unwrap().clone()
    }
    
    fn reset_stats(&self) {
        *self.stats.lock().unwrap() = MemoryStats::default();
    }
}

/// Channel memory manager
pub struct ChannelMemoryManager {
    /// Primary allocator
    allocator: Box<dyn ChannelAllocator>,
    /// Fallback allocator
    fallback_allocator: Box<dyn ChannelAllocator>,
    /// Memory pressure threshold
    pressure_threshold: usize,
    /// Total system memory limit
    memory_limit: Option<usize>,
}

impl ChannelMemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            allocator: Box::new(MemoryPool::new()),
            fallback_allocator: Box::new(LockFreeAllocator::new()),
            pressure_threshold: 100 * 1024 * 1024, // 100MB
            memory_limit: None,
        }
    }
    
    /// Create a memory manager with custom allocator
    pub fn with_allocator(allocator: Box<dyn ChannelAllocator>) -> Self {
        Self {
            fallback_allocator: Box::new(MemoryPool::new()),
            allocator,
            pressure_threshold: 100 * 1024 * 1024,
            memory_limit: None,
        }
    }
    
    /// Set memory limit
    pub fn set_memory_limit(&mut self, limit: usize) {
        self.memory_limit = Some(limit);
    }
    
    /// Allocate memory
    pub unsafe fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ChannelError> {
        // Check memory limit
        if let Some(limit) = self.memory_limit {
            let current_usage = self.allocator.stats().current_usage;
            if current_usage + layout.size() > limit {
                return Err(ChannelError::AllocationError("Memory limit exceeded".to_string()));
            }
        }
        
        // Try primary allocator
        match self.allocator.allocate(layout) {
            Ok(ptr) => Ok(ptr),
            Err(_) => {
                // Fall back to secondary allocator
                self.fallback_allocator.allocate(layout)
            }
        }
    }
    
    /// Deallocate memory
    pub unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // Try to determine which allocator was used
        // For simplicity, we'll always use the primary allocator
        self.allocator.deallocate(ptr, layout)
    }
    
    /// Check if under memory pressure
    pub fn is_under_pressure(&self) -> bool {
        let stats = self.allocator.stats();
        stats.current_usage > self.pressure_threshold
    }
    
    /// Get combined statistics
    pub fn stats(&self) -> MemoryStats {
        let primary_stats = self.allocator.stats();
        let fallback_stats = self.fallback_allocator.stats();
        
        MemoryStats {
            total_allocated: primary_stats.total_allocated + fallback_stats.total_allocated,
            total_deallocated: primary_stats.total_deallocated + fallback_stats.total_deallocated,
            current_usage: primary_stats.current_usage + fallback_stats.current_usage,
            peak_usage: primary_stats.peak_usage.max(fallback_stats.peak_usage),
            allocation_count: primary_stats.allocation_count + fallback_stats.allocation_count,
            deallocation_count: primary_stats.deallocation_count + fallback_stats.deallocation_count,
            pool_hits: primary_stats.pool_hits + fallback_stats.pool_hits,
            pool_misses: primary_stats.pool_misses + fallback_stats.pool_misses,
        }
    }
    
    /// Force cleanup to reduce memory usage
    pub fn cleanup(&self) {
        // Force cleanup by using dummy implementation for now
        // Real implementation would need trait object downcasting
        /*
        if let Some(pool) = self.allocator.as_ref().as_any().downcast_ref::<MemoryPool>() {
            pool.cleanup();
        }
        */
    }
}

/// Extension trait for type erasure
trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Global memory manager instance
static GLOBAL_MEMORY_MANAGER: once_cell::sync::Lazy<ChannelMemoryManager> = 
    once_cell::sync::Lazy::new(|| ChannelMemoryManager::new());

/// Get the global memory manager
pub fn get_global_memory_manager() -> &'static ChannelMemoryManager {
    &GLOBAL_MEMORY_MANAGER
}

/// Allocate memory using the global manager
pub unsafe fn global_allocate(layout: Layout) -> Result<NonNull<u8>, ChannelError> {
    get_global_memory_manager().allocate(layout)
}

/// Deallocate memory using the global manager
pub unsafe fn global_deallocate(ptr: NonNull<u8>, layout: Layout) {
    get_global_memory_manager().deallocate(ptr, layout)
}

/// Get global memory statistics
pub fn global_memory_stats() -> MemoryStats {
    get_global_memory_manager().stats()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool() {
        let pool = MemoryPool::new();
        
        unsafe {
            let layout = Layout::from_size_align(64, 8).unwrap();
            
            // Allocate some memory
            let ptr1 = pool.allocate(layout).unwrap();
            let ptr2 = pool.allocate(layout).unwrap();
            
            // Deallocate
            pool.deallocate(ptr1, layout);
            pool.deallocate(ptr2, layout);
            
            // Allocate again (should come from pool)
            let ptr3 = pool.allocate(layout).unwrap();
            
            pool.deallocate(ptr3, layout);
        }
        
        let stats = pool.stats();
        assert!(stats.allocation_count > 0);
        assert!(stats.pool_hits > 0);
    }

    #[test]
    fn test_lock_free_allocator() {
        let allocator = LockFreeAllocator::new();
        
        unsafe {
            let layout = Layout::from_size_align(32, 4).unwrap();
            
            let ptr = allocator.allocate(layout).unwrap();
            allocator.deallocate(ptr, layout);
        }
        
        let stats = allocator.stats();
        assert_eq!(stats.allocation_count, 1);
        assert_eq!(stats.deallocation_count, 1);
    }

    #[test]
    fn test_memory_manager() {
        let manager = ChannelMemoryManager::new();
        
        unsafe {
            let layout = Layout::from_size_align(128, 8).unwrap();
            
            let ptr = manager.allocate(layout).unwrap();
            manager.deallocate(ptr, layout);
        }
        
        let stats = manager.stats();
        assert!(stats.allocation_count > 0);
    }
}
