//! CURSED Memory Allocator
//!
//! This module provides a custom memory allocator that integrates with CURSED's
//! garbage collection system. It provides thread-safe allocation/deallocation
//! with tracking and statistics for debugging.

use crate::error::{CursedError, Result};
use crate::memory::{Tag, Traceable, MemoryStats};
use std::alloc::{alloc, dealloc, Layout, GlobalAlloc, System};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicU64, Ordering}};
use std::collections::HashMap;
use std::ptr::{self, NonNull};
use std::time::Instant;

/// Minimum alignment for CURSED objects (64-bit aligned)
const MIN_ALIGNMENT: usize = 8;

/// Maximum allocation size (1GB)
const MAX_ALLOCATION_SIZE: usize = 1024 * 1024 * 1024;

/// Size classes for efficient allocation (powers of 2)
const SIZE_CLASSES: [usize; 16] = [
    16, 32, 64, 128, 256, 512, 1024, 2048,
    4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288
];

/// Allocation metadata stored alongside allocated objects
#[derive(Debug, Clone)]
pub struct AllocationMetadata {
    pub size: usize,
    pub tag: Tag,
    pub allocation_id: u64,
    pub timestamp: Instant,
    pub source_location: Option<String>,
    pub layout: Layout,
}

/// Thread-safe memory allocator for CURSED
pub struct Allocator {
    /// Allocation tracking
    allocations: Arc<Mutex<HashMap<usize, AllocationMetadata>>>,
    /// Statistics
    stats: Arc<AllocatorStats>,
    /// Next allocation ID
    next_id: AtomicU64,
    /// GC integration callback
    gc_notify: Option<Arc<dyn Fn(usize, bool) + Send + Sync>>,
}

/// Thread-safe allocation statistics
#[derive(Debug)]
pub struct AllocatorStats {
    pub total_allocated: AtomicUsize,
    pub total_freed: AtomicUsize,
    pub current_usage: AtomicUsize,
    pub allocation_count: AtomicU64,
    pub free_count: AtomicU64,
    pub peak_usage: AtomicUsize,
}

impl Default for AllocatorStats {
    fn default() -> Self {
        Self {
            total_allocated: AtomicUsize::new(0),
            total_freed: AtomicUsize::new(0),
            current_usage: AtomicUsize::new(0),
            allocation_count: AtomicU64::new(0),
            free_count: AtomicU64::new(0),
            peak_usage: AtomicUsize::new(0),
        }
    }
}

impl Allocator {
    /// Create a new CURSED allocator
    pub fn new() -> Self {
        Self {
            allocations: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(AllocatorStats::default()),
            next_id: AtomicU64::new(1),
            gc_notify: None,
        }
    }

    /// Create allocator with GC integration
    pub fn with_gc_callback<F>(callback: F) -> Self 
    where
        F: Fn(usize, bool) + Send + Sync + 'static,
    {
        Self {
            allocations: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(AllocatorStats::default()),
            next_id: AtomicU64::new(1),
            gc_notify: Some(Arc::new(callback)),
        }
    }

    /// Allocate memory for a CURSED object
    pub fn allocate(&self, size: usize, tag: Tag) -> Result<NonNull<u8>> {
        self.allocate_with_location(size, tag, None)
    }

    /// Allocate memory with source location tracking
    pub fn allocate_with_location(
        &self, 
        size: usize, 
        tag: Tag, 
        location: Option<String>
    ) -> Result<NonNull<u8>> {
        if size == 0 {
            return Err(CursedError::runtime_error("Cannot allocate zero bytes"));
        }

        if size > MAX_ALLOCATION_SIZE {
            return Err(CursedError::runtime_error(&format!(
                "Allocation size {} exceeds maximum {}", size, MAX_ALLOCATION_SIZE
            )));
        }

        // Calculate layout with proper alignment
        let layout = self.calculate_layout(size)?;
        
        // Perform the actual allocation
        let ptr = unsafe {
            let raw_ptr = alloc(layout);
            if raw_ptr.is_null() {
                return Err(CursedError::runtime_error(&format!(
                    "Failed to allocate {} bytes", size
                )));
            }
            NonNull::new_unchecked(raw_ptr)
        };

        // Create allocation metadata
        let allocation_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let metadata = AllocationMetadata {
            size,
            tag,
            allocation_id,
            timestamp: Instant::now(),
            source_location: location,
            layout,
        };

        // Track the allocation
        if let Ok(mut allocations) = self.allocations.lock() {
            allocations.insert(ptr.as_ptr() as usize, metadata);
        }

        // Update statistics
        self.stats.total_allocated.fetch_add(size, Ordering::SeqCst);
        self.stats.allocation_count.fetch_add(1, Ordering::SeqCst);
        let current_usage = self.stats.current_usage.fetch_add(size, Ordering::SeqCst) + size;
        
        // Update peak usage
        let mut peak = self.stats.peak_usage.load(Ordering::SeqCst);
        while current_usage > peak {
            match self.stats.peak_usage.compare_exchange_weak(
                peak, current_usage, Ordering::SeqCst, Ordering::SeqCst
            ) {
                Ok(_) => break,
                Err(x) => peak = x,
            }
        }

        // Notify GC if callback is set
        if let Some(ref callback) = self.gc_notify {
            callback(current_usage, false);
        }

        Ok(ptr)
    }

    /// Deallocate memory
    pub fn deallocate(&self, ptr: NonNull<u8>) -> Result<()> {
        let ptr_addr = ptr.as_ptr() as usize;

        // Get allocation metadata
        let metadata = {
            if let Ok(mut allocations) = self.allocations.lock() {
                allocations.remove(&ptr_addr)
            } else {
                return Err(CursedError::runtime_error("Failed to acquire allocation lock"));
            }
        };

        let metadata = metadata.ok_or_else(|| {
            CursedError::runtime_error(&format!(
                "Attempting to deallocate untracked pointer: {:p}", ptr.as_ptr()
            ))
        })?;

        // Perform the actual deallocation
        unsafe {
            dealloc(ptr.as_ptr(), metadata.layout);
        }

        // Update statistics
        self.stats.total_freed.fetch_add(metadata.size, Ordering::SeqCst);
        self.stats.free_count.fetch_add(1, Ordering::SeqCst);
        self.stats.current_usage.fetch_sub(metadata.size, Ordering::SeqCst);

        // Notify GC if callback is set
        if let Some(ref callback) = self.gc_notify {
            let current_usage = self.stats.current_usage.load(Ordering::SeqCst);
            callback(current_usage, true);
        }

        Ok(())
    }

    /// Reallocate memory (resize existing allocation)
    pub fn reallocate(&self, ptr: NonNull<u8>, new_size: usize) -> Result<NonNull<u8>> {
        if new_size == 0 {
            self.deallocate(ptr)?;
            return Err(CursedError::runtime_error("Cannot reallocate to zero size"));
        }

        let ptr_addr = ptr.as_ptr() as usize;

        // Get existing metadata
        let old_metadata = {
            if let Ok(allocations) = self.allocations.lock() {
                allocations.get(&ptr_addr).cloned()
            } else {
                return Err(CursedError::runtime_error("Failed to acquire allocation lock"));
            }
        };

        let old_metadata = old_metadata.ok_or_else(|| {
            CursedError::runtime_error("Attempting to reallocate untracked pointer")
        })?;

        // If sizes are the same, no-op
        if old_metadata.size == new_size {
            return Ok(ptr);
        }

        // Allocate new memory
        let new_ptr = self.allocate_with_location(
            new_size, 
            old_metadata.tag, 
            old_metadata.source_location.clone()
        )?;

        // Copy existing data
        let copy_size = old_metadata.size.min(new_size);
        unsafe {
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), copy_size);
        }

        // Deallocate old memory
        self.deallocate(ptr)?;

        Ok(new_ptr)
    }

    /// Get allocation metadata for a pointer
    pub fn get_metadata(&self, ptr: NonNull<u8>) -> Option<AllocationMetadata> {
        let ptr_addr = ptr.as_ptr() as usize;
        if let Ok(allocations) = self.allocations.lock() {
            allocations.get(&ptr_addr).cloned()
        } else {
            None
        }
    }

    /// Get current memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            total_allocated: self.stats.total_allocated.load(Ordering::SeqCst),
            total_freed: self.stats.total_freed.load(Ordering::SeqCst),
            current_usage: self.stats.current_usage.load(Ordering::SeqCst),
            gc_collections: 0, // Updated by GC system
            gc_time_ms: 0,     // Updated by GC system
        }
    }

    /// Get detailed allocator statistics
    pub fn get_detailed_stats(&self) -> DetailedAllocatorStats {
        DetailedAllocatorStats {
            total_allocated: self.stats.total_allocated.load(Ordering::SeqCst),
            total_freed: self.stats.total_freed.load(Ordering::SeqCst),
            current_usage: self.stats.current_usage.load(Ordering::SeqCst),
            allocation_count: self.stats.allocation_count.load(Ordering::SeqCst),
            free_count: self.stats.free_count.load(Ordering::SeqCst),
            peak_usage: self.stats.peak_usage.load(Ordering::SeqCst),
            active_allocations: if let Ok(allocations) = self.allocations.lock() {
                allocations.len()
            } else {
                0
            },
        }
    }

    /// Check for memory leaks
    pub fn check_leaks(&self) -> Vec<AllocationMetadata> {
        if let Ok(allocations) = self.allocations.lock() {
            allocations.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Clear all allocation tracking (for testing)
    pub fn clear_tracking(&self) {
        if let Ok(mut allocations) = self.allocations.lock() {
            allocations.clear();
        }
    }

    /// Calculate layout for allocation size
    fn calculate_layout(&self, size: usize) -> Result<Layout> {
        // Round up to next size class for efficiency
        let actual_size = self.round_to_size_class(size);
        
        Layout::from_size_align(actual_size, MIN_ALIGNMENT)
            .map_err(|e| CursedError::runtime_error(&format!(
                "Invalid layout for size {}: {}", size, e
            )))
    }

    /// Round size up to the nearest size class
    fn round_to_size_class(&self, size: usize) -> usize {
        for &class_size in &SIZE_CLASSES {
            if size <= class_size {
                return class_size;
            }
        }
        // For very large allocations, just round up to page boundary
        let page_size = 4096;
        ((size + page_size - 1) / page_size) * page_size
    }
}

/// Detailed allocator statistics
#[derive(Debug, Clone)]
pub struct DetailedAllocatorStats {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub current_usage: usize,
    pub allocation_count: u64,
    pub free_count: u64,
    pub peak_usage: usize,
    pub active_allocations: usize,
}

impl Default for Allocator {
    fn default() -> Self {
        Self::new()
    }
}

/// Global allocator instance
static GLOBAL_ALLOCATOR: once_cell::sync::Lazy<Allocator> = 
    once_cell::sync::Lazy::new(|| Allocator::new());

/// Get the global allocator instance
pub fn global_allocator() -> &'static Allocator {
    &GLOBAL_ALLOCATOR
}

/// Convenience function to allocate memory with the global allocator
pub fn allocate(size: usize, tag: Tag) -> Result<NonNull<u8>> {
    global_allocator().allocate(size, tag)
}

/// Convenience function to deallocate memory with the global allocator
pub fn deallocate(ptr: NonNull<u8>) -> Result<()> {
    global_allocator().deallocate(ptr)
}

/// Convenience function to get global memory statistics
pub fn get_memory_stats() -> MemoryStats {
    global_allocator().get_stats()
}

/// Allocate typed memory with proper CURSED object metadata
pub fn allocate_typed<T: Traceable>() -> Result<NonNull<T>> {
    let size = std::mem::size_of::<T>();
    let tag = if size == 0 {
        Tag::Nil
    } else {
        // Use a dummy instance to get the proper tag
        // This is safe because we're only calling get_tag, not trace
        unsafe {
            let dummy: T = std::mem::zeroed();
            let tag = dummy.get_tag();
            std::mem::forget(dummy);
            tag
        }
    };
    
    let ptr = global_allocator().allocate(size, tag)?;
    Ok(ptr.cast::<T>())
}

/// Deallocate typed memory
pub fn deallocate_typed<T>(ptr: NonNull<T>) -> Result<()> {
    global_allocator().deallocate(ptr.cast::<u8>())
}

/// Convenience function for compatibility
pub fn get_minimal_result() -> Result<String> {
    let stats = global_allocator().get_stats();
    Ok(format!("Allocator ready - {} bytes allocated", stats.current_usage))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_allocation() {
        let allocator = Allocator::new();
        
        let ptr = allocator.allocate(64, Tag::Object).unwrap();
        // NonNull pointer is guaranteed to be non-null by type system
        
        let stats = allocator.get_detailed_stats();
        assert_eq!(stats.allocation_count, 1);
        assert_eq!(stats.active_allocations, 1);
        
        allocator.deallocate(ptr).unwrap();
        
        let stats = allocator.get_detailed_stats();
        assert_eq!(stats.free_count, 1);
        assert_eq!(stats.active_allocations, 0);
    }

    #[test]
    fn test_reallocation() {
        let allocator = Allocator::new();
        
        let ptr = allocator.allocate(32, Tag::String).unwrap();
        let new_ptr = allocator.reallocate(ptr, 128).unwrap();
        
        let stats = allocator.get_detailed_stats();
        assert_eq!(stats.allocation_count, 2); // Original + new
        assert_eq!(stats.free_count, 1);       // Original freed
        assert_eq!(stats.active_allocations, 1);
        
        allocator.deallocate(new_ptr).unwrap();
    }

    #[test]
    fn test_size_class_rounding() {
        let allocator = Allocator::new();
        
        // Test that small sizes round up to size classes
        assert_eq!(allocator.round_to_size_class(10), 16);
        assert_eq!(allocator.round_to_size_class(33), 64);
        assert_eq!(allocator.round_to_size_class(1000), 1024);
    }

    #[test]
    fn test_zero_size_allocation() {
        let allocator = Allocator::new();
        
        let result = allocator.allocate(0, Tag::Object);
        assert!(result.is_err());
    }

    #[test]
    fn test_max_allocation_size() {
        let allocator = Allocator::new();
        
        let result = allocator.allocate(MAX_ALLOCATION_SIZE + 1, Tag::Object);
        assert!(result.is_err());
    }

    #[test]
    fn test_gc_callback() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();
        
        let allocator = Allocator::with_gc_callback(move |_usage, _freed| {
            called_clone.store(true, Ordering::SeqCst);
        });
        
        let ptr = allocator.allocate(64, Tag::Object).unwrap();
        assert!(called.load(Ordering::SeqCst));
        
        called.store(false, Ordering::SeqCst);
        allocator.deallocate(ptr).unwrap();
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_metadata_tracking() {
        let allocator = Allocator::new();
        
        let ptr = allocator.allocate_with_location(
            128, 
            Tag::Array, 
            Some("test.rs:42".to_string())
        ).unwrap();
        
        let metadata = allocator.get_metadata(ptr).unwrap();
        assert_eq!(metadata.size, 128);
        assert_eq!(metadata.tag, Tag::Array);
        assert_eq!(metadata.source_location, Some("test.rs:42".to_string()));
        
        allocator.deallocate(ptr).unwrap();
    }
}
