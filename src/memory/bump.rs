// Bump Allocator for CURSED Memory Management
//
// A bump allocator quickly allocates memory by simply incrementing a pointer.
// It doesn't support individual deallocations, but can reset all memory at once.

use std::vec::Vec;
use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};
use std::borrow::BorrowMut;
use std::iter::Iterator;
use std::ops::Deref;
use crate::error::Error;
use crate::prelude::VecExt;
use crate::prelude::RawPtrExt;
use super::allocator::Allocator;
use super::{align_up, MIN_ALIGNMENT};
use std::cell::RefCell;
use num_traits::CheckedAdd;
use crate::prelude::ptr_is_null;
use crate::prelude::ptr_wrapping_offset;
use std::fmt::{self, Debug, Formatter};
use crate::memory::allocator::AllocatorBase;
use crate::page_size;
use std::marker::PhantomData;
use super::tagged::{Tag, TaggedPtr, TaggedPtrConstructor};
use std::cell::Cell;
use std::rc::Rc;

/// Statistics for bump allocator
#[derive(Debug, Clone, Default)]
pub struct BumpAllocatorStats {
    /// Total size of memory managed
    pub total_size: usize,
    /// Total bytes allocated
    pub total_allocated: usize,
    /// Current bytes in use
    pub bytes_in_use: usize,
    /// Number of resets performed
    pub reset_count: usize,
}

/// Default size for the bump allocator (1MB)
const DEFAULT_BUMP_SIZE: usize = 1024 * 1024;

/// A simple bump allocator that allocates memory sequentially
#[derive(Debug)]
pub struct BumpAllocator {
    /// The memory pointer
    memory: Option<NonNull<u8>>,
    /// The total size of the allocated memory
    size: usize,
    /// The current offset from the start
    offset_cell: RefCell<usize>,
    /// The previous allocation size and layout
    prev_size_cell: RefCell<Option<(usize, Layout)>>,
    /// The total capacity of the allocator in bytes
    capacity: usize,
    /// The current memory usage in bytes
    current: usize,
    /// The peak memory usage in bytes
    peak: usize,
    /// The total number of allocations
    total_allocated: usize,
    /// The total number of deallocations
    total_freed: usize,
    /// The current allocation statistics
    stats: BumpAllocatorStats,
}

impl BumpAllocator {
    /// Create a new bump allocator
    pub fn new(heap_size: usize) -> Result<Self, Error> {
        // Ensure heap size is a power of 2
        if !heap_size.is_power_of_two() {
            return Err(Error::InvalidHeapSize(heap_size));
        }

        let layout = Layout::from_size_align(heap_size, 8).unwrap();
        let ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(ptr).unwrap();
        
        Ok(Self {
            memory: Some(ptr),
            size: heap_size,
            offset_cell: RefCell::new(0),
            prev_size_cell: RefCell::new(None),
            capacity: heap_size,
            current: 0,
            peak: 0,
            total_allocated: 0,
            total_freed: 0,
            stats: BumpAllocatorStats {
                total_size: heap_size,
                total_allocated: 0,
                bytes_in_use: 0,
                reset_count: 0,
            },
        })
    }

    /// Align the offset to the requested alignment
    pub fn align_offset(&self, offset: usize, align: usize) -> usize {
        align_up(offset, align)
    }

    /// Calculate the offset needed to align an address to the given alignment
    fn align_offset_crate(&self, offset: usize, align: usize) -> usize {
        // Use crate-level align_up function
        crate::memory::align_up(offset, align) - offset
    }

    /// Allocate a region of memory for the bump allocator
    fn allocate_memory_region(size: usize) -> Result<NonNull<u8>, Error> {
        let layout = Layout::from_size_align(size, MIN_ALIGNMENT)
            .map_err(|_| Error::Memory("Invalid memory layout".to_string()))?;
        
        let ptr = unsafe {
            let ptr = alloc::alloc(layout);
            if ptr.is_null() {
                return Err(Error::Memory("Failed to allocate memory for bump allocator".to_string()));
            }
            NonNull::new_unchecked(ptr)
        };
        
        Ok(ptr)
    }

    /// Reset the allocator to the beginning of the memory region
    pub fn reset_to_mark(&self, mark: usize) -> Result<(), Error> {
        let mut offset = self.offset_cell.borrow_mut();
        if mark > *offset {
            return Err(Error::Memory("Invalid mark for reset".to_string()));
        }
        
        *offset = mark;
        Ok(())
    }
    
    /// Get the current offset (can be used as a mark)
    pub fn get_offset(&self) -> usize {
        *self.offset_cell.borrow()
    }
    
    /// Initialize the memory region if not already done
    pub fn initialize(&mut self) -> Result<(), Error> {
        if self.memory.is_none() {
            let layout = Layout::from_size_align(self.size, MIN_ALIGNMENT)
                .map_err(|_| Error::Memory("Invalid memory layout".to_string()))?;
            
            let ptr = unsafe {
                let ptr = alloc::alloc(layout);
                if ptr.is_null() {
                    return Err(Error::Memory("Failed to allocate memory for bump allocator".to_string()));
                }
                NonNull::new_unchecked(ptr)
            };
            
            self.memory = Some(ptr);
        }
        
        Ok(())
    }

    /// Get the current memory usage in bytes
    ///
    /// # Returns
    ///
    /// The current memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        self.current
    }

    /// Get the peak memory usage in bytes
    ///
    /// # Returns
    ///
    /// The peak memory usage in bytes
    pub fn peak_memory_usage(&self) -> usize {
        self.peak
    }

    /// Get the total memory capacity in bytes
    ///
    /// # Returns
    ///
    /// The total memory capacity in bytes
    pub fn memory_capacity(&self) -> usize {
        self.capacity
    }

    /// Get the total number of allocations
    ///
    /// # Returns
    ///
    /// The total number of allocations
    pub fn total_allocated(&self) -> usize {
        self.total_allocated
    }

    /// Get the total number of deallocations
    ///
    /// # Returns
    ///
    /// The total number of deallocations
    pub fn total_freed(&self) -> usize {
        self.total_freed
    }

    /// Get the current allocation statistics
    ///
    /// # Returns
    ///
    /// The current allocation statistics
    pub fn stats(&self) -> BumpAllocatorStats {
        self.stats.clone()
    }

    /// Get the current allocation pointer
    ///
    /// # Returns
    ///
    /// The current allocation pointer
    pub fn current(&self) -> usize {
        *self.offset_cell.borrow()
    }

    /// Get the peak allocation pointer
    ///
    /// # Returns
    ///
    /// The peak allocation pointer
    pub fn peak(&self) -> usize {
        self.peak
    }

    /// Get the total capacity
    ///
    /// # Returns
    ///
    /// The total capacity in bytes
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Check if there's enough space for an allocation
    pub fn has_space(&self, size: usize, align: usize) -> bool {
        let aligned_offset = self.align_offset(*self.offset_cell.borrow(), align);
        aligned_offset + size <= self.size
    }

    /// Get the remaining memory
    pub fn remaining_memory(&self) -> usize {
        self.size - *self.offset_cell.borrow()
    }

    /// Get the current memory usage of the allocator
    pub fn memory_usage(&self) -> usize {
        // Current memory usage is the difference between the heap size and remaining space
        self.size - self.remaining
    }
}

impl AllocatorBase for BumpAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, Error> {
        // Initialize memory if needed
        if self.memory.is_none() {
            // We can't call initialize directly since it requires &mut self
            // but we're behind an immutable reference
            return Err(Error::Memory("Allocator not initialized".to_string()));
        }
        
        let size = layout.size();
        let align = layout.align();
        
        // Calculate the aligned position
        let mut offset = self.offset_cell.borrow_mut();
        let aligned_offset = self.align_offset(*offset, align);
        
        // Check if we have enough space
        if aligned_offset + size <= self.size {
            // Get pointer to the aligned position
            let ptr = unsafe {
                let base_ptr = self.memory.unwrap().as_ptr();
                let ptr = base_ptr.add(aligned_offset);
                NonNull::new_unchecked(ptr)
            };
            
            // Update offset
            *offset = aligned_offset + size;
            *self.prev_size_cell.borrow_mut() = Some((size, layout));
            
            Ok(ptr)
        } else {
            Err(Error::Memory("Out of memory in bump allocator".to_string()))
        }
    }
    
    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // Bump allocators don't support individual deallocations
        // This is a no-op
    }
    
    fn reset(&mut self) {
        // Reset the offset to reuse memory
        *self.offset_cell.borrow_mut() = 0;
        *self.prev_size_cell.borrow_mut() = None;
    }

    fn memory_capacity(&self) -> usize {
        self.size
    }
    
    fn memory_usage(&self) -> usize {
        *self.offset_cell.borrow()
    }
}

impl Allocator for BumpAllocator {
    // The trait AllocatorBase is implemented above, and Allocator provides default implementations
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        // Free the entire memory block
        if let Some(ptr) = self.memory {
            unsafe {
                if !ptr_is_null(ptr.as_ptr()) {
                    let layout = Layout::from_size_align(self.size, MIN_ALIGNMENT).unwrap();
                    std::alloc::dealloc(ptr.as_ptr(), layout);
                }
            }
        }
    }
}

impl fmt::Display for BumpAllocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BumpAllocator(size={}, offset={})", self.size, *self.offset_cell.borrow())
    }
}

impl Default for BumpAllocator {
    fn default() -> Self {
        Self::new(DEFAULT_BUMP_SIZE)
    }
}

impl Clone for BumpAllocator {
    fn clone(&self) -> Self {
        Self {
            memory: None, // New instance will allocate its own memory
            size: self.size,
            offset_cell: RefCell::new(*self.offset_cell.borrow()),
            prev_size_cell: RefCell::new(*self.prev_size_cell.borrow()),
            capacity: self.capacity,
            current: self.current,
            peak: self.peak,
            total_allocated: self.total_allocated,
            total_freed: self.total_freed,
            stats: self.stats.clone(),
        }
    }
}

impl PartialEq for BumpAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && *self.offset_cell.borrow() == *other.offset_cell.borrow()
    }
}

impl Eq for BumpAllocator {}

// Add this helper function to allocate memory
/// Allocates memory with the given size
fn allocate(size: usize) -> Result<NonNull<u8>, Error> {
    let layout = Layout::from_size_align(size, MIN_ALIGNMENT)
        .map_err(|e| Error::from(format!("Invalid layout: {}", e)))?;
        
    let ptr = unsafe {
        let ptr = alloc::alloc(layout);
        if ptr.is_null() {
            return Err(Error::from(format!("Failed to allocate {} bytes", size)));
        }
        NonNull::new_unchecked(ptr)
    };
    
    Ok(ptr)
}

// Implementation of TaggedPtrConstructor for BumpAllocator
impl TaggedPtrConstructor for BumpAllocator {
    type T = BumpAllocator;
    
    fn new(ptr: Option<NonNull<Self::T>>, _tag: Tag) -> TaggedPtr<Self::T> {
        TaggedPtr {
            ptr,
            tag: 0,
            _phantom: PhantomData,
        }
    }
}

pub struct BumpStats {
    pub total_size: usize,
    pub used_size: usize,
    pub allocations: usize,
    pub deallocations: usize,
}

impl Default for BumpStats {
    fn default() -> Self {
        Self {
            total_size: 0,
            used_size: 0,
            allocations: 0,
            deallocations: 0,
        }
    }
}

impl Clone for BumpStats {
    fn clone(&self) -> Self {
        Self {
            total_size: self.total_size,
            used_size: self.used_size,
            allocations: self.allocations,
            deallocations: self.deallocations,
        }
    }
} 