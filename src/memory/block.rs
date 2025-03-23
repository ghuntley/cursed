// Block Allocator for CURSED Memory Management
//
// A block allocator manages memory in fixed-size blocks.
// It supports individual deallocations and better memory locality.

use std::alloc::{self, Layout};
use std::collections::HashMap;
use std::ptr::{NonNull, null_mut};
use std::iter::Iterator;
use crate::error::Error;
use crate::prelude::VecExt;
use super::allocator::Allocator;
use super::{align_up, MIN_ALIGNMENT};
use std::ops::Deref;
use std::slice;
use std::mem;
use std::collections::HashSet;

/// Default block size for the allocator (4KB)
const DEFAULT_BLOCK_SIZE: usize = 4 * 1024;

/// Statistics for block allocator
#[derive(Debug, Clone, Default)]
pub struct BlockAllocatorStats {
    /// Total number of allocations
    pub total_allocated: usize,
    /// Total number of deallocations
    pub total_freed: usize,
    /// Number of blocks allocated
    pub blocks_allocated: usize,
    /// Number of free slots available
    pub free_slots: usize,
    /// Total size of memory managed
    pub total_size: usize,
}

/// A block of memory
pub struct MemoryBlock {
    /// The memory block pointer
    ptr: NonNull<u8>,
    /// The size of the block
    size: usize,
    /// The size of each allocation slot in the block
    slot_size: usize,
    /// Number of slots in the block
    slots: usize,
    /// Bitmap of allocated regions (1 = allocated, 0 = free)
    bitmap: Vec<u8>,
    /// Number of free slots in the block
    free_slots: usize,
}

impl MemoryBlock {
    /// Create a new memory block with the given size and slot size
    pub fn new(block_size: usize, slot_size: usize) -> Result<Self, Error> {
        // Ensure block_size is at least as big as the slot size
        if block_size < slot_size {
            return Err(Error::Runtime("Block size too small for slots".to_string()));
        }
        
        // Allocate the memory block
        let layout = Layout::from_size_align(block_size, MIN_ALIGNMENT)
            .map_err(|_| Error::Runtime("Invalid memory layout".to_string()))?;
        
        let ptr = unsafe {
            std::alloc::alloc(layout)
        };
        
        if ptr == std::ptr::null_mut() {
            return Err(Error::Runtime("Failed to allocate memory block".to_string()));
        }
        
        let ptr = NonNull::new(ptr).unwrap();
        
        // Calculate number of slots in this block
        let num_slots = block_size / slot_size;
        
        // Create bitmap for tracking allocations
        let bitmap_size = (num_slots + 7) / 8; // Ceiling division to ensure enough bits
        let mut bitmap = Vec::with_capacity(bitmap_size);
        bitmap.resize(bitmap_size, 0);
        
        Ok(Self {
            ptr,
            size: block_size,
            slot_size,
            slots: num_slots,
            bitmap,
            free_slots: num_slots,
        })
    }
    
    /// Check if a slot is allocated
    pub fn is_allocated(&self, slot: usize) -> bool {
        if slot >= self.slots {
            return false;
        }
        
        let byte_index = slot / 8;
        let bit_index = slot % 8;
        let mask = 1 << bit_index;
        
        (self.bitmap[byte_index] & mask) != 0
    }
    
    /// Mark a slot as allocated
    pub fn mark_allocated(&mut self, slot: usize) -> Result<NonNull<u8>, Error> {
        if slot >= self.slots {
            return Err(Error::Runtime("Invalid slot".to_string()));
        }
        
        if self.is_allocated(slot) {
            return Err(Error::Runtime("Slot already allocated".to_string()));
        }
        
        let byte_index = slot / 8;
        let bit_index = slot % 8;
        
        self.bitmap[byte_index] |= 1 << bit_index;
        self.free_slots -= 1;
        
        self.get_slot_ptr(slot)
    }
    
    /// Mark a slot as free
    pub fn mark_free(&mut self, slot: usize) {
        if slot >= self.slots {
            return;
        }
        
        let byte_index = slot / 8;
        let bit_index = slot % 8;
        let mask = !(1 << bit_index);
        
        self.bitmap[byte_index] &= mask;
        self.free_slots += 1;
    }
    
    /// Find a free slot in the block
    pub fn find_free_slot(&self) -> Option<usize> {
        if self.free_slots == 0 {
            return None;
        }
        
        for (byte_index, &byte) in self.bitmap.iter().enumerate() {
            if byte != 0xFF {  // Not all bits are set
                for bit_index in 0..8 {
                    let slot = byte_index * 8 + bit_index;
                    if slot < self.slots && !self.is_allocated(slot) {
                        return Some(slot);
                    }
                }
            }
        }
        
        None
    }
    
    /// Get the pointer to a slot
    pub fn get_slot_ptr(&self, slot: usize) -> Result<NonNull<u8>, Error> {
        if slot >= self.slots {
            return Err(Error::Runtime(format!("Invalid slot: {}", slot)));
        }
        
        let offset = slot * self.slot_size;
        let raw_ptr = self.ptr.as_ptr();
        let slot_ptr = unsafe {
            raw_ptr.add(offset)
        };
        
        // Convert to NonNull
        NonNull::new(slot_ptr)
            .ok_or_else(|| Error::Runtime("Slot pointer is null".to_string()))
    }
    
    /// Check if this block contains the pointer
    pub fn contains_ptr(&self, ptr: NonNull<u8>) -> bool {
        let ptr_addr = ptr.as_ptr() as usize;
        let start_addr = self.ptr.as_ptr() as usize;
        let end_addr = start_addr + self.size;
        
        ptr_addr >= start_addr && ptr_addr < end_addr
    }
    
    /// Get the slot index for a pointer
    pub fn ptr_to_slot(&self, ptr: NonNull<u8>) -> Option<usize> {
        if !self.contains_ptr(ptr) {
            return None;
        }
        
        let offset = (ptr.as_ptr() as usize) - (self.ptr.as_ptr() as usize);
        Some(offset / self.slot_size)
    }
    
    /// Get a pointer to a specific offset within the block
    pub fn get_ptr_at_offset(&self, offset: usize) -> Option<NonNull<u8>> {
        if offset < self.size {
            let raw_ptr = self.ptr.as_ptr();
            let offset_ptr = unsafe {
                raw_ptr.add(offset)
            };
            NonNull::new(offset_ptr)
        } else {
            None
        }
    }
}

impl Drop for MemoryBlock {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.size, MIN_ALIGNMENT)
            .expect("Invalid layout during MemoryBlock drop");
            
        unsafe {
            // Check for null pointer before deallocating
            let ptr = self.ptr.as_ptr();
            if ptr != std::ptr::null_mut() {
                alloc::dealloc(ptr, layout);
            }
        }
    }
}

/// The block sizes to use, in bytes
const BLOCK_SIZES: [usize; 4] = [
    4 * 1024,      // 4 KB
    16 * 1024,     // 16 KB
    64 * 1024,     // 64 KB
    256 * 1024,    // 256 KB
];

/// The slot sizes to use for each block size, in bytes
const SLOT_SIZES: [usize; 4] = [
    16,     // For objects <= 16 bytes
    64,     // For objects <= 64 bytes
    256,    // For objects <= 256 bytes
    1024,   // For objects <= 1024 bytes
];

/// A block allocator for efficient memory management
pub struct BlockAllocator {
    /// Blocks for each size class
    blocks: Vec<MemoryBlock>,
    /// Block size to use for new allocations
    block_size: usize,
    /// Map from pointers to (block index, slot index)
    ptr_map: HashMap<usize, (usize, usize)>,
    /// Total allocated size
    pub total_size: usize,
    /// Total allocated count
    total_allocated: usize,
    /// Total freed count
    total_freed: usize,
    /// Free blocks
    free_blocks: Vec<usize>,
}

impl BlockAllocator {
    /// Create a new block allocator with the given total size
    pub fn new(total_size: usize) -> Self {
        Self {
            blocks: Vec::new(),
            block_size: DEFAULT_BLOCK_SIZE,
            ptr_map: HashMap::new(),
            total_size: 0, // Will be incremented as blocks are allocated
            total_allocated: 0,
            total_freed: 0,
            free_blocks: Vec::new(),
        }
    }
    
    /// Get the appropriate slot size for an allocation
    pub fn get_slot_size(&self, size: usize) -> usize {
        // Round up to MIN_ALIGNMENT
        let aligned_size = align_up(size, MIN_ALIGNMENT);
        
        // Find the smallest slot size that can fit this allocation
        for &slot_size in &SLOT_SIZES {
            if slot_size >= aligned_size {
                return slot_size;
            }
        }
        
        // If no predefined slot size fits, use the aligned size
        aligned_size
    }

    /// Get the block size to use for the given slot size
    pub fn get_block_size(&self, slot_size: usize) -> usize {
        // Find an appropriate block size based on the slot size
        for i in 0..4 {  // SLOT_SIZES has 4 elements
            if SLOT_SIZES[i] >= slot_size {
                return BLOCK_SIZES[i];
            }
        }
        
        // Default to the configured block size
        self.block_size
    }

    /// Find a block with a free slot for the given slot size
    pub fn find_block_with_free_slot(&self, slot_size: usize) -> Option<usize> {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.slot_size == slot_size && block.free_slots > 0 {
                return Some(i);
            }
        }
        None
    }
    
    /// Allocate a new memory block for the given slot size
    pub fn allocate_block(&mut self, slot_size: usize) -> Result<usize, Error> {
        // Get the appropriate block size for this slot size
        let block_size = self.get_block_size(slot_size);
        
        // Create a new memory block
        let block = MemoryBlock::new(block_size, slot_size)?;
        
        // Update total size
        self.total_size += block_size;
        
        // Add the block to our list
        self.blocks.push(block);
        
        // Return the index of the new block
        Ok(self.blocks.len() - 1)
    }
    
    /// Get the total capacity of the allocator
    pub fn capacity(&self) -> usize {
        self.total_size
    }
    
    /// Get statistics about allocations
    pub fn stats(&self) -> BlockAllocatorStats {
        BlockAllocatorStats {
            total_allocated: self.total_allocated,
            total_freed: self.total_freed,
            blocks_allocated: self.blocks.len(),
            free_slots: self.blocks.iter().map(|block| block.free_slots).sum(),
            total_size: self.total_size,
        }
    }
}

impl Allocator for BlockAllocator {
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, Error> {
        let size = layout.size().max(layout.align());
        let slot_size = self.get_slot_size(size);
        
        // Try to find a block with a free slot
        let block_index = if let Some(index) = self.find_block_with_free_slot(slot_size) {
            index
        } else {
            // Allocate a new block
            self.allocate_block(slot_size)?
        };
        
        // Find a free slot in the block
        let slot = self.blocks[block_index].find_free_slot()
            .ok_or_else(|| Error::Runtime("No free slots in block".to_string()))?;
        
        // Mark the slot as allocated
        let ptr = self.blocks[block_index].mark_allocated(slot)?;
        
        // Store the pointer mapping
        self.ptr_map.insert(ptr.as_ptr() as usize, (block_index, slot));
        
        self.total_allocated += 1;
        
        Ok(ptr)
    }
    
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, _layout: Layout) {
        if let Some((block_index, slot)) = self.ptr_map.remove(&(ptr.as_ptr() as usize)) {
            if block_index < self.blocks.len() {
                self.blocks[block_index].mark_free(slot);
                self.total_freed += 1;
            }
        }
    }
    
    fn reset(&mut self) -> Result<(), Error> {
        // Free all blocks
        self.blocks.clear();
        self.ptr_map.clear();
        self.total_size = 0;
        self.total_allocated = 0;
        self.total_freed = 0;
        
        Ok(())
    }
}

impl Drop for BlockAllocator {
    fn drop(&mut self) {
        self.reset();
    }
} 