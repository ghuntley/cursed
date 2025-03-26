// CURSED Memory Management Module
//
// This module provides custom memory allocation for the CURSED language VM.
// It implements several allocation strategies:
// 1. Bump allocation for fast allocation
// 2. Block-based memory management for better locality
// 3. Tagged pointers for efficient memory representation

use std::alloc::Layout;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::cell::RefCell;
use std::rc::Rc;
use crate::error::Error;
use std::fmt;
use crate::prelude::RawPtrExt;
use crate::memory::allocator::AllocatorBase;
use std::mem;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::iter::Iterator;
use std::collections::HashMap;
use std::ops::RangeBounds;
use crate::memory::bump::BumpAllocator;
use crate::memory::gc::GarbageCollector;
use crate::memory::block::BlockAllocator;
use crate::memory::block::BlockStats;
use crate::memory::bump::BumpStats;
use crate::memory::gc::GcStats;

/// Get the system's memory page size
#[inline]
pub fn page_size() -> usize {
    // Use 4KB as a reasonable default
    4096
}

// Module exports
pub mod allocator;
pub mod bump;
pub mod block;
pub mod tagged;
pub mod gc;
mod error;

#[cfg(test)]
mod tests;

// Re-export the public API
// pub use allocator::Allocator;

// Re-export tagged pointer functionality
pub use tagged::TaggedPtr;
pub use tagged::Tag;
pub use tagged::NonNullExt;
pub use tagged::{TaggedPtrExt, TaggedPtrExtMut, TaggedPtrConstructor};

// Re-export block allocator functionality
// pub use block::BlockAllocator;
pub use block::MemoryBlock;
pub use block::{BlockAllocatorExt, MemoryBlockExt, MemoryBlockExtMut};

// Re-export garbage collector functionality 
// pub use gc::GarbageCollector;
pub use gc::GarbageCollectorExt;

// Re-export bump allocator functionality
// pub use bump::BumpAllocator;

/// Minimum alignment for all allocations
pub const MIN_ALIGNMENT: usize = 8;

/// Default block size for allocators
pub const DEFAULT_BLOCK_SIZE: usize = 4 * 1024; // 4KB

// Import memory-related errors
pub use error::MemoryError;

// Re-export the main types that should be used from outside the memory module
pub use block::BlockAllocatorStats;
pub use bump::BumpAllocatorStats;
pub use tagged::{TAG_BITS, TAG_MASK, TAG_SHIFT, PTR_MASK};
pub use gc::{Traceable, Visitor, Gc, Trace};

/// Align a value up to the next multiple of `align`
pub fn align_up(value: usize, align: usize) -> usize {
    // Ensure alignment is a power of 2
    debug_assert!(is_power_of_two(align), "Alignment must be a power of 2");
    
    // To align up, we add (align - 1) and then mask off the lower bits
    (value + (align - 1)) & !(align - 1)
}

/// Create a new garbage collector
pub fn create_garbage_collector(heap_size: usize) -> Result<gc::GarbageCollector, error::MemoryError> {
    // Create a garbage collector with the specified heap size
    use crate::error;
    use crate::memory::gc;
    
    gc::GarbageCollectorCompanion::with_heap_size(heap_size).map_err(|e| error::MemoryError::AllocationFailed(format!("Failed to create garbage collector: {:?}", e)))
}

/// Create a new block allocator
pub fn create_block_allocator(heap_size: usize) -> Result<block::BlockAllocator, error::MemoryError> {
    block::BlockAllocatorCompanion::new(heap_size).map_err(|e| error::MemoryError::AllocationFailed(format!("Failed to create block allocator: {:?}", e)))
}

/// Create a new bump allocator
pub fn create_bump_allocator(heap_size: usize) -> Result<bump::BumpAllocator, error::MemoryError> {
    bump::BumpAllocatorCompanion::new(heap_size).map_err(|e| error::MemoryError::AllocationFailed(format!("Failed to create bump allocator: {:?}", e)))
}

/// A wrapper around allocated memory that tracks its allocator
pub struct Allocated<T> {
    ptr: std::ptr::NonNull<T>,
    allocator: Rc<dyn AllocatorBase>,
}

impl<T> Allocated<T> {
    /// Create a new allocated value
    pub fn new(ptr: std::ptr::NonNull<T>, allocator: Rc<dyn AllocatorBase>) -> Self {
        Self { ptr, allocator }
    }

    /// Get a reference to the allocated value
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr.as_ptr() }
    }

    /// Get a mutable reference to the allocated value
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.as_ptr() }
    }

    /// Get the underlying pointer
    pub fn as_ptr(&self) -> std::ptr::NonNull<T> {
        self.ptr
    }

    /// Get the allocator used for this allocation
    pub fn allocator(&self) -> &Rc<dyn AllocatorBase> {
        &self.allocator
    }
}

impl<T> Drop for Allocated<T> {
    fn drop(&mut self) {
        unsafe {
            self.allocator.deallocate(self.ptr.cast(), std::alloc::Layout::new::<T>());
        }
    }
}

/// A memory manager that provides different allocation strategies
#[derive(Debug)]
pub struct MemoryManager {
    /// Bump allocator for temporary allocations
    pub bump_allocator: RefCell<bump::BumpAllocator>,
    /// Block allocator for fixed-size allocations
    pub block_allocator: RefCell<block::BlockAllocator>,
    /// Garbage collector
    pub gc: RefCell<gc::GarbageCollector>,
    /// Total memory size
    pub total_size: usize,
    /// Heap size
    pub heap_size: usize,
    /// GC size
    pub gc_size: usize,
}

impl MemoryManager {
    /// Create a new memory manager with the specified total size
    pub fn new(heap_size: usize) -> Result<Self, error::MemoryError> {
        Ok(Self {
            gc: RefCell::new(create_garbage_collector(heap_size)?),
            block_allocator: RefCell::new(create_block_allocator(heap_size)?),
            bump_allocator: RefCell::new(create_bump_allocator(heap_size)?),
            total_size: heap_size,
            heap_size,
            gc_size: heap_size / 4,
        })
    }
    
    /// Get the stats for this memory manager
    pub fn stats(&self) -> MemoryStats {
        // Calculate total in use directly to avoid the get_total_in_use method
        let bump_usage = self.bump_allocator.borrow().memory_usage();
        let block_usage = crate::memory::block::BlockAllocatorExt::memory_usage(&*self.block_allocator.borrow());
        let gc_usage = crate::memory::gc::GarbageCollectorExt::memory_usage(&*self.gc.borrow());
        let total_in_use = bump_usage + block_usage + gc_usage;
        
        MemoryStats {
            bump_stats: self.bump_allocator.borrow().stats.clone(),
            block_stats: self.block_allocator.borrow().stats.clone(),
            gc_stats: self.gc.borrow().stats(),
            total_managed: self.total_size,
            total_in_use,
        }
    }
    
    /// Get the total amount of memory in use
    pub fn get_total_in_use(&self) -> usize {
        // Sum up memory in use from all allocators
        self.bump_allocator.borrow().memory_usage() + 
        crate::memory::block::BlockAllocatorExt::memory_usage(&*self.block_allocator.borrow()) + 
        // Explicitly specify which memory_usage to use
        crate::memory::gc::GarbageCollectorExt::memory_usage(&*self.gc.borrow())
    }
    
    /// Get the total amount of memory managed
    pub fn get_total_managed(&self) -> usize {
        self.total_size
    }
    
    /// Allocate a block of memory
    pub fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, error::MemoryError> {
        let size = layout.size();
        let align = layout.align();

        // For small allocations, use the bump allocator
        if size <= 128 {
            if let Ok(ptr) = self.bump_allocator.borrow_mut().allocate(layout)
                .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut u8) }) {
                return Ok(ptr);
            }
        }

        // For medium-sized allocations, use the block allocator
        if size <= 4096 {
            if let Ok(ptr) = self.block_allocator.borrow_mut().allocate(layout)
                .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut u8) }) {
                return Ok(ptr);
            }
        }

        // For large allocations, or if other allocators failed, use system allocator
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                return Err(error::MemoryError::AllocationFailed(format!("Failed to allocate {} bytes", size)));
            }
            NonNull::new_unchecked(ptr)
        };

        Ok(ptr)
    }
    
    /// Collect garbage
    pub fn collect_garbage<T>(&mut self, _roots: &[Rc<T>]) -> Result<(), error::MemoryError> {
        // For simplicity, we'll just run a collection cycle
        let mut gc = self.gc.borrow_mut();
        
        // Run collection cycle
        if gc.should_collect() {
            gc.run_collection();
        }
        
        Ok(())
    }
    
    /// Allocate memory using the bump allocator
    pub fn allocate_bump(&self, size: usize) -> Result<NonNull<u8>, error::MemoryError> {
        let layout = Layout::from_size_align(size, MIN_ALIGNMENT)
            .map_err(|e| error::MemoryError::Other(format!("Invalid layout for bump allocation: {}", e)))?;
            
        // Convert the slice pointer to a raw pointer and the AllocError to an Error
        self.bump_allocator.borrow_mut().allocate(layout)
            .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut u8) })
            .map_err(|_| error::MemoryError::AllocationFailed("Bump allocation failed".to_string()))
    }
    
    /// Allocate memory using the block allocator
    pub fn allocate_block(&self, size: usize) -> Result<NonNull<u8>, error::MemoryError> {
        let layout = Layout::from_size_align(size, MIN_ALIGNMENT)
            .map_err(|e| error::MemoryError::Other(format!("Invalid layout for block allocation: {}", e)))?;
            
        // Convert the slice pointer to a raw pointer and the AllocError to an Error
        self.block_allocator.borrow_mut().allocate(layout)
            .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut u8) })
            .map_err(|_| error::MemoryError::AllocationFailed("Block allocation failed".to_string()))
    }
    
    /// Deallocate memory that was allocated with this memory manager
    pub fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), error::MemoryError> {
        let layout = Layout::from_size_align(size, MIN_ALIGNMENT)
            .map_err(|e| error::MemoryError::Other(format!("Invalid layout for deallocation: {}", e)))?;
        
        // Try to deallocate using all allocators
        // This is inefficient but safe - in a real implementation we would
        // track which allocator each pointer came from
        
        // Try bump allocator (no-op for bump allocator)
        unsafe {
            self.bump_allocator.borrow_mut().deallocate(ptr, layout);
        }
        
        // Try block allocator
        unsafe {
            self.block_allocator.borrow_mut().deallocate(ptr, layout);
        }
        
        // Try GC allocator
        // (No direct deallocation for GC - objects are collected during GC cycles)
        
        Ok(())
    }
    
    /// Reset all allocators in the memory manager
    pub fn reset(&self) -> Result<(), error::MemoryError> {
        // Reset the bump allocator
        self.bump_allocator.borrow_mut().reset();
        
        // Reset the block allocator
        self.block_allocator.borrow_mut().reset();
        
        // Reset the garbage collector
        // Note: This doesn't deallocate memory, it just resets collection stats
        // Actual deallocation happens during collection cycles
        
        Ok(())
    }
    
    /// Get memory statistics
    pub fn memory_stats(&self) -> MemoryStats {
        // Calculate total in use directly to avoid the get_total_in_use method
        let bump_usage = self.bump_allocator.borrow().memory_usage();
        let block_usage = crate::memory::block::BlockAllocatorExt::memory_usage(&*self.block_allocator.borrow());
        let gc_usage = crate::memory::gc::GarbageCollectorExt::memory_usage(&*self.gc.borrow());
        let total_in_use = bump_usage + block_usage + gc_usage;
        
        MemoryStats {
            bump_stats: self.bump_allocator.borrow().stats.clone(),
            block_stats: self.block_allocator.borrow().stats.clone(),
            gc_stats: self.gc.borrow().stats(),
            total_managed: self.total_size,
            total_in_use,
        }
    }
}

// Implementation moved to src/memory/tests.rs
// #[cfg(test)]
// mod tests { ... }

/// Comprehensive memory statistics across all allocators
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total memory managed (in bytes)
    pub total_managed: usize,
    /// Total memory currently in use (in bytes)
    pub total_in_use: usize,
    /// Memory usage by the bump allocator (in bytes)
    pub bump_stats: BumpAllocatorStats,
    /// Memory usage by the block allocator (in bytes)
    pub block_stats: BlockAllocatorStats,
    /// Memory usage by the garbage collector (in bytes)
    pub gc_stats: GcStats,
}

impl fmt::Display for MemoryStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Memory usage:")?;
        writeln!(f, "  Total in use: {} bytes", self.total_in_use)?;
        writeln!(f, "  Total managed: {} bytes", self.total_managed)?;
        writeln!(f, "  Bump allocator stats:")?;
        writeln!(f, "    Used: {} bytes", self.bump_stats.bytes_in_use)?;
        writeln!(f, "    Total: {} bytes", self.bump_stats.total_size)?;
        writeln!(f, "  Block allocator stats:")?;
        writeln!(f, "    Used: {} bytes", self.block_stats.total_allocated - self.block_stats.total_freed)?;
        writeln!(f, "    Total: {} bytes", self.block_stats.total_size)?;
        writeln!(f, "  GC stats:")?;
        writeln!(f, "    Collections: {}", self.gc_stats.collections)?;
        writeln!(f, "    Total allocated: {} bytes", self.gc_stats.total_allocated)?;
        writeln!(f, "    Current heap size: {} bytes", self.gc_stats.current_heap_size)?;
        write!(f, "    Max heap size: {} bytes", self.gc_stats.max_heap_size)
    }
}

/// Checks if a number is a power of two
pub fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

/// Calculate the next multiple of an alignment value
#[inline]
pub fn next_multiple_of(value: usize, align: usize) -> usize {
    debug_assert!(is_power_of_two(align), "Alignment must be a power of 2");
    
    // For power-of-2 alignments, we can use a faster bit operation
    if value & (align - 1) == 0 {
        // Already aligned
        value
    } else {
        // Calculate next aligned value: (value + align - 1) & !(align - 1)
        (value + align - 1) & !(align - 1)
    }
} 