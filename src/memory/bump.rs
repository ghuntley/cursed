// Bump Allocator for CURSED Memory Management
//
// A bump allocator quickly allocates memory by simply incrementing a pointer.
// It doesn't support individual deallocations, but can reset all memory at once.

use std::vec::Vec;
use std::alloc::{self, Layout};
use std::ptr::NonNull;
use std::borrow::BorrowMut;
use std::iter::Iterator;
use std::ops::Deref;
use crate::error::Error;
use crate::prelude::VecExt;
use super::allocator::Allocator;
use super::{align_up, MIN_ALIGNMENT};
use std::cell::RefCell;
use num_traits::CheckedAdd;

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

/// A bump allocator for fast allocation
pub struct BumpAllocator {
    /// The current memory block
    current_block: Option<NonNull<u8>>,
    /// The current position in the block
    current_pos: usize,
    /// The capacity of the current block
    capacity: usize,
    /// The block size to use for allocations
    block_size: usize,
    /// The blocks that have been allocated
    blocks: Vec<(NonNull<u8>, usize)>,
    /// Number of resets performed
    reset_count: usize,
    /// Total bytes allocated
    total_allocated: usize,
}

impl BumpAllocator {
    /// Create a new bump allocator with the given size
    pub fn new(size: usize) -> Self {
        Self {
            current_block: None,
            current_pos: 0,
            capacity: 0,
            block_size: size,
            blocks: Vec::new(),
            reset_count: 0,
            total_allocated: 0,
        }
    }
    
    /// Allocate memory using the bump allocator
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, Error> {
        let size = layout.size();
        let align = layout.align().max(MIN_ALIGNMENT);
        
        // Calculate the aligned position
        let aligned_pos = align_up(self.current_pos, layout.align());
        if aligned_pos + layout.size() <= self.capacity {
            // There's enough space in the current block
            let raw_ptr = match self.current_block {
                Some(ptr) => ptr.as_ptr(),
                None => return Err(Error::Runtime("No current block".to_string())),
            };
            self.current_pos = aligned_pos + layout.size();
            
            // Get a pointer to the aligned position
            let ptr = unsafe { 
                raw_ptr.add(aligned_pos)
            };
            
            // Convert to NonNull and return
            return NonNull::new(ptr)
                .ok_or_else(|| Error::Runtime("Failed to get aligned pointer".to_string()));
        }
        
        // Not enough space, allocate a new block
        let size = size.max(self.block_size);
        
        // Allocate a new block of memory
        let layout = Layout::from_size_align(size, MIN_ALIGNMENT)
            .map_err(|_| Error::Runtime("Invalid layout for bump allocator block".to_string()))?;
            
        let ptr = unsafe {
            std::alloc::alloc(layout)
        };
        
        if ptr == std::ptr::null_mut() {
            return Err(Error::Runtime("Failed to allocate memory for bump allocator".to_string()));
        }
        
        // Update allocator state
        let ptr_nonnull = NonNull::new(ptr)
            .ok_or_else(|| Error::Runtime("Failed to create NonNull pointer".to_string()))?;
            
        self.current_block = Some(ptr_nonnull);
        self.current_pos = 0;
        self.capacity = size;
        
        // Calculate aligned position in the new block
        let aligned_start = align_up(0, layout.align());
        
        // Update position and get the aligned pointer
        self.current_pos = aligned_start + layout.size();
        let aligned_ptr = unsafe {
            ptr.add(aligned_start)
        };
        
        // Convert to NonNull and return
        NonNull::new(aligned_ptr)
            .ok_or_else(|| Error::Runtime("Failed to get aligned pointer".to_string()))
    }

    /// Reset the allocator to its initial state
    pub fn reset(&mut self) -> Result<(), Error> {
        // Implement the method directly here instead of recursively calling
        if let Some(ptr) = self.current_block {
            // Deallocate the current block if it exists
            unsafe {
                let layout = Layout::from_size_align(self.capacity, MIN_ALIGNMENT)
                    .map_err(|_| Error::Runtime("Invalid memory layout during reset".to_string()))?;
                
                std::alloc::dealloc(ptr.as_ptr(), layout);
            }
        }
        
        // Reset all state
        self.current_block = None;
        self.current_pos = 0;
        self.capacity = 0;
        
        Ok(())
    }

    /// Get statistics about the allocator
    pub fn stats(&self) -> BumpAllocatorStats {
        BumpAllocatorStats {
            total_size: self.capacity,
            total_allocated: self.total_allocated,
            bytes_in_use: self.current_pos,
            reset_count: self.reset_count,
        }
    }
}

impl Allocator for BumpAllocator {
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, Error> {
        let size = layout.size();
        let align = layout.align().max(MIN_ALIGNMENT);
        
        // Calculate the aligned position
        let aligned_pos = align_up(self.current_pos, layout.align());
        if aligned_pos + layout.size() <= self.capacity {
            // There's enough space in the current block
            let raw_ptr = match self.current_block {
                Some(ptr) => ptr.as_ptr(),
                None => return Err(Error::Runtime("No current block".to_string())),
            };
            self.current_pos = aligned_pos + layout.size();
            
            // Get a pointer to the aligned position
            let ptr = unsafe { 
                raw_ptr.add(aligned_pos)
            };
            
            // Convert to NonNull and return
            return NonNull::new(ptr)
                .ok_or_else(|| Error::Runtime("Failed to get aligned pointer".to_string()));
        }
        
        // Not enough space, allocate a new block
        let size = size.max(self.block_size);
        
        // Allocate a new block of memory
        let layout = Layout::from_size_align(size, MIN_ALIGNMENT)
            .map_err(|_| Error::Runtime("Invalid layout for bump allocator block".to_string()))?;
            
        let ptr = unsafe {
            std::alloc::alloc(layout)
        };
        
        if ptr == std::ptr::null_mut() {
            return Err(Error::Runtime("Failed to allocate memory for bump allocator".to_string()));
        }
        
        // Update allocator state
        let ptr_nonnull = NonNull::new(ptr)
            .ok_or_else(|| Error::Runtime("Failed to create NonNull pointer".to_string()))?;
            
        self.current_block = Some(ptr_nonnull);
        self.current_pos = 0;
        self.capacity = size;
        
        // Calculate aligned position in the new block
        let aligned_start = align_up(0, layout.align());
        
        // Update position and get the aligned pointer
        self.current_pos = aligned_start + layout.size();
        let aligned_ptr = unsafe {
            ptr.add(aligned_start)
        };
        
        // Convert to NonNull and return
        NonNull::new(aligned_ptr)
            .ok_or_else(|| Error::Runtime("Failed to get aligned pointer".to_string()))
    }
    
    unsafe fn deallocate(&mut self, _ptr: NonNull<u8>, _layout: Layout) {
        // Bump allocator doesn't support individual deallocations
        // Memory is freed only when reset() is called or when the allocator is dropped
    }
    
    fn reset(&mut self) -> Result<(), Error> {
        // Implement the method directly here instead of recursively calling
        if let Some(ptr) = self.current_block {
            // Deallocate the current block if it exists
            unsafe {
                let layout = Layout::from_size_align(self.capacity, MIN_ALIGNMENT)
                    .map_err(|_| Error::Runtime("Invalid memory layout during reset".to_string()))?;
                
                std::alloc::dealloc(ptr.as_ptr(), layout);
            }
        }
        
        // Reset all state
        self.current_block = None;
        self.current_pos = 0;
        self.capacity = 0;
        
        Ok(())
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        self.reset();
    }
} 