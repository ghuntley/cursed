// Block Allocator for CURSED Memory Management
//
// A block allocator manages memory in fixed-size blocks.
// It supports individual deallocations and better memory locality.

use std::alloc::{self, Layout};
use std::collections::HashMap;
use std::ptr::{NonNull, null_mut};
use std::iter::Iterator;
use crate::error::Error;
use crate::prelude::{VecExt, RawPtrExt, ArrayExt};
use super::allocator::Allocator;
use super::{align_up, MIN_ALIGNMENT};
use std::ops::Deref;
use std::slice;
use std::mem;
use std::collections::HashSet;
use std::error::Error as StdError;
use crate::memory::allocator::{AllocatorBase, AllocatorExt};
use super::tagged::{Tag, TaggedPtr, TAG_MASK, TAG_SHIFT, PTR_MASK, NonNullExt, TaggedPtrExt, TaggedPtrConstructor};
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

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
#[derive(Debug, Clone)]
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
    /// Creates a new memory block with the specified block size and slot size
    ///
    /// # Arguments
    ///
    /// * `block_size` - The total size of the memory block in bytes
    /// * `slot_size` - The size of each slot in bytes
    ///
    /// # Returns
    ///
    /// A Result containing the newly created memory block, or an error if allocation failed
    pub fn new(block_size: usize, slot_size: usize) -> Result<Self, Error> {
        // Ensure block_size is a multiple of slot_size
        if block_size % slot_size != 0 {
            return Err(Error::Memory("Block size must be a multiple of slot size".to_string()));
        }

        // Calculate number of slots
        let slots = block_size / slot_size;
        if slots == 0 {
            return Err(Error::Memory("Block size too small for the specified slot size".to_string()));
        }

        // Allocate memory for the block
        let layout = Layout::from_size_align(block_size, MIN_ALIGNMENT)
            .map_err(|_| Error::Memory("Invalid memory layout".to_string()))?;
        
        let ptr = unsafe {
            let ptr = alloc::alloc(layout);
            if ptr.is_null() {
                return Err(Error::Memory("Failed to allocate memory for block".to_string()));
            }
            NonNull::new_unchecked(ptr)
        };

        // Initialize bitmap to track free slots (all slots initially free)
        let bitmap_size = (slots + 7) / 8; // Round up to nearest byte
        let bitmap = vec![0u8; bitmap_size];

        Ok(Self {
            ptr,
            size: block_size,
            slot_size,
            slots,
            bitmap,
            free_slots: slots,
        })
    }

    /// Checks if the given slot is allocated
    pub fn is_allocated(&self, slot: usize) -> bool {
        if slot >= self.slots {
            return false;
        }
        
        let byte_index = slot / 8;
        let bit_index = slot % 8;
        let mask = 1 << bit_index;
        
        (self.bitmap[byte_index] & mask) != 0
    }

    /// Marks a slot as allocated
    pub fn mark_allocated(&mut self, slot: usize) {
        if slot < self.slots {
            let byte_index = slot / 8;
            let bit_index = slot % 8;
            let mask = 1 << bit_index;
            
            self.bitmap[byte_index] |= mask;
            self.free_slots -= 1;
        }
    }

    /// Marks a slot as free
    pub fn mark_free(&mut self, slot: usize) {
        if slot < self.slots {
            let byte_index = slot / 8;
            let bit_index = slot % 8;
            let mask = 1 << bit_index;
            
            self.bitmap[byte_index] &= !mask;
            self.free_slots += 1;
        }
    }

    /// Checks if the given pointer is contained within this memory block
    pub fn contains_ptr(&self, ptr: *mut u8) -> bool {
        let self_ptr = self.ptr.as_ptr();
        let ptr_val = ptr as usize;
        let self_val = self_ptr as usize;
        
        // Check if ptr is within the bounds of this block
        ptr_val >= self_val && ptr_val < (self_val + self.size)
    }
    
    /// Gets the slot index for a given pointer
    pub fn get_slot_index(&self, ptr: NonNull<u8>) -> Option<usize> {
        let base_ptr = self.ptr.as_ptr();
        let ptr_val = ptr.as_ptr() as usize;
        let base_val = base_ptr as usize;
        
        if !self.contains_ptr(ptr.as_ptr()) {
            return None;
        }
        
        // Calculate offset from base and determine slot
        let offset = ptr_val - base_val;
        let slot = offset / self.slot_size;
        
        if slot < self.slots {
            Some(slot)
        } else {
            None
        }
    }
    
    /// Get the pointer for a given slot
    pub fn get_slot_ptr(&self, slot: usize) -> Option<NonNull<u8>> {
        if slot >= self.slots {
            return None;
        }

        let raw_ptr = self.ptr.as_ptr();
        let offset = slot * self.slot_size;
        
        // Calculate the pointer to the slot
        let ptr = unsafe {
            let ptr = raw_ptr.wrapping_offset(offset as isize);
            NonNull::new_unchecked(ptr) 
        };
        
        Some(ptr)
    }
    
    /// Get the size of each slot in this block
    pub fn get_slot_size(&self) -> usize {
        self.slot_size
    }

    /// Check if a slot is free
    pub fn is_slot_free(&self, slot: usize) -> bool {
        !self.is_allocated(slot)
    }

    /// Find a block with a free slot
    pub fn find_free_slot(&self) -> Option<usize> {
        if self.free_slots == 0 {
            return None;
        }
        
        // Check the bitmap to find a free slot
        for i in 0..self.slots {
            let byte_index = i / 8;
            let bit_index = i % 8;
            
            if byte_index < self.bitmap.len() {
                let byte = self.bitmap[byte_index];
                let is_allocated = (byte & (1 << bit_index)) != 0;
                
                if !is_allocated {
                    return Some(i);
                }
            }
        }
        
        None
    }

    /// Checks if a pointer is allocated in this block
    pub fn is_pointer_allocated(&self, ptr: *mut u8) -> bool {
        if !self.contains_ptr(ptr) {
            return false;
        }
        
        if let Some(slot) = self.get_slot_index(NonNull::new(ptr).unwrap()) {
            return self.is_allocated(slot);
        }
        
        false
    }

    /// Get the size of this memory block
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the number of slots in this memory block
    pub fn slots(&self) -> usize {
        self.slots
    }

    /// Get the size of each slot in this memory block
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }

    /// Get the number of free slots in this memory block
    pub fn free_slots(&self) -> usize {
        self.free_slots
    }

    /// Get the base pointer of this memory block
    pub fn ptr(&self) -> NonNull<u8> {
        self.ptr
    }

    /// Get the bitmap of this memory block
    pub fn bitmap(&self) -> &[u8] {
        &self.bitmap
    }

    /// Get the bitmap of this memory block as mutable
    pub fn bitmap_mut(&mut self) -> &mut [u8] {
        &mut self.bitmap
    }
}

impl Drop for MemoryBlock {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.size, MIN_ALIGNMENT)
            .expect("Invalid layout during MemoryBlock drop");
            
        unsafe {
            // Check for null pointer before deallocating
            let ptr = self.ptr.as_ptr();
            if !ptr.is_null() {
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
#[derive(Debug)]
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
    /// Available slot sizes
    slot_sizes: Vec<usize>,
    /// Statistics for block allocator
    stats: BlockAllocatorStats,
    /// Maximum size of the allocator
    max_size: usize,
    /// The start of the allocated memory
    start: NonNull<u8>,
    /// The number of blocks
    num_blocks: usize,
}

impl BlockAllocator {
    /// Create a new block allocator
    pub fn new(heap_size: usize) -> Result<Self, Error> {
        // Validate heap size
        if heap_size == 0 {
            return Err(Error::Memory("Heap size cannot be zero".to_string()));
        }

        // Define slot sizes (powers of 2)
        let slot_sizes = vec![8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];
        
        // Allocate memory for the heap
        let layout = Layout::from_size_align(heap_size, DEFAULT_ALIGNMENT)
            .map_err(|e| Error::Memory(format!("Invalid layout for heap: {}", e)))?;
            
        let start = unsafe {
            NonNull::new(std::alloc::alloc(layout))
                .ok_or_else(|| Error::Memory("Failed to allocate memory for heap".to_string()))?
        };
        
        Ok(Self {
            blocks: Vec::new(),
            block_size: DEFAULT_BLOCK_SIZE,
            ptr_map: HashMap::new(),
            total_size: heap_size,
            total_allocated: 0,
            total_freed: 0,
            free_blocks: Vec::new(),
            slot_sizes,
            stats: BlockAllocatorStats::default(),
            max_size: heap_size,
            start,
            num_blocks: 0,
        })
    }

    /// Get the current memory usage
    pub fn current(&self) -> usize {
        self.stats.used
    }

    /// Get the block allocator statistics
    pub fn stats(&self) -> &BlockStats {
        &self.stats
    }

    /// Find a block with a free slot of the specified size
    pub fn find_block_with_free_slot(&self, slot_size: usize) -> Option<usize> {
        // Look for a block with the right slot size and at least one free slot
        for (index, block) in self.blocks.iter().enumerate() {
            if block.slot_size == slot_size && block.free_slots > 0 {
                return Some(index);
            }
        }
        
        // If no existing blocks with free slots, check our free blocks list
        for &block_idx in &self.free_blocks {
            if block_idx < self.blocks.len() {
                let block = &self.blocks[block_idx];
                if block.slot_size == slot_size {
                    return Some(block_idx);
                }
            }
        }
        
        None
    }
    
    /// Get the appropriate slot size for the requested size
    pub fn get_slot_size(&self) -> usize {
        // Default implementation returns the first slot size
        self.slot_sizes.first().copied().unwrap_or(MIN_ALIGNMENT)
    }

    /// Get the appropriate slot size for the requested size
    pub fn get_slot_size_for(&self, size: usize) -> usize {
        // Ensure we have at least the minimum alignment
        let aligned_size = if size < MIN_ALIGNMENT {
            MIN_ALIGNMENT
        } else {
            // Round up to the nearest alignment boundary
            align_up(size, MIN_ALIGNMENT)
        };
        
        // Find the smallest slot size that can accommodate the aligned size
        for &slot_size in &self.slot_sizes {
            if slot_size >= aligned_size {
                return slot_size;
            }
        }
        
        // If no slot size fits, round up to the next power of 2
        let mut power = MIN_ALIGNMENT;
        while power < aligned_size {
            power *= 2;
        }
        
        power
    }

    /// Get statistics about allocations
    ///
    /// # Returns
    ///
    /// The current allocation statistics
    pub fn stats(&self) -> BlockAllocatorStats {
        self.stats.clone()
    }

    /// Get the memory capacity in bytes
    ///
    /// # Returns
    ///
    /// The total memory capacity in bytes
    pub fn memory_capacity(&self) -> usize {
        self.total_size
    }
    
    /// Get the current memory usage of the allocator
    pub fn memory_usage(&self) -> usize {
        // Calculate the memory in use
        self.total_allocated - self.total_freed
    }

    /// Get the total allocated memory (includes memory in blocks that may have free slots)
    ///
    /// # Returns
    ///
    /// The total allocated memory in bytes
    pub fn allocated_memory(&self) -> usize {
        self.total_size - (self.free_blocks.len() * DEFAULT_BLOCK_SIZE)
    }

    /// Calculate the number of free slots in all blocks
    ///
    /// # Returns
    ///
    /// The total number of free slots
    pub fn count_free_slots(&self) -> usize {
        self.blocks.iter().map(|block| block.free_slots).sum()
    }

    /// Get the blocks in this allocator
    ///
    /// # Returns
    ///
    /// A reference to the blocks vector
    pub fn blocks(&self) -> &[MemoryBlock] {
        &self.blocks
    }

    /// Get the free blocks in this allocator
    ///
    /// # Returns
    ///
    /// A reference to the free blocks vector
    pub fn free_blocks(&self) -> &[usize] {
        &self.free_blocks
    }

    /// Get the pointer map in this allocator
    ///
    /// # Returns
    ///
    /// A reference to the pointer map
    pub fn ptr_map(&self) -> &HashMap<usize, (usize, usize)> {
        &self.ptr_map
    }

    /// Find a block with a free slot, or create one if needed
    pub fn find_or_create_block_with_free_slot(&mut self, slot_size: usize) -> Option<usize> {
        // First try to find an existing block with a free slot
        if let Some(block_index) = self.find_block_with_free_slot(slot_size) {
            return Some(block_index);
        }
        
        // If no existing block has a free slot, create a new one
        // Implementation depends on how blocks are created in your system
        None
    }

    pub fn capacity(&self) -> usize {
        self.total_size
    }
}

impl AllocatorBase for BlockAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, Error> {
        let size = layout.size();
        let align = layout.align();
        
        // Determine the slot size needed
        let slot_size = self.get_slot_size_for(size);
        
        // Find a block with a free slot
        if let Some(block_index) = self.find_block_with_free_slot(slot_size) {
            let mut blocks = self.blocks.clone();
            let block = &mut blocks[block_index];
            
            // Find a free slot in the block
            if let Some(slot) = block.find_free_slot() {
                // Mark the slot as allocated
                block.mark_allocated(slot);
                
                // Calculate the pointer to the allocated memory
                if let Some(ptr) = block.get_slot_ptr(slot) {
                    // Update the pointer map
                    let mut ptr_map = self.ptr_map.clone();
                    ptr_map.insert(ptr.as_ptr() as usize, (block_index, slot));
                    
                    return Ok(ptr);
                }
            }
        }
        
        // If we get here, we couldn't allocate memory
        Err(Error::Memory("Out of memory in block allocator".to_string()))
    }
    
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        // Get the pointer address
        let addr = ptr.as_ptr() as usize;
        
        // Look up the block and slot
        if let Some(&(block_index, slot)) = self.ptr_map.get(&addr) {
            let mut blocks = self.blocks.clone();
            if block_index < blocks.len() {
                // Mark the slot as free
                blocks[block_index].mark_free(slot);
                
                // Remove from the pointer map
                let mut ptr_map = self.ptr_map.clone();
                ptr_map.remove(&addr);
            }
        }
    }
    
    fn reset(&mut self) {
        // Clear all allocations
        self.blocks = Vec::new();
        self.ptr_map = HashMap::new();
        self.free_blocks = Vec::new();
        self.total_allocated = 0;
        self.total_freed = 0;
        self.stats = BlockAllocatorStats::default();
    }
    
    fn memory_capacity(&self) -> usize {
        self.total_size
    }
    
    fn memory_usage(&self) -> usize {
        let mut used = 0;
        for block in &self.blocks {
            let allocated_slots = block.slots - block.free_slots;
            used += allocated_slots * block.slot_size;
        }
        used
    }
}

impl Allocator for BlockAllocator {}

impl Drop for BlockAllocator {
    fn drop(&mut self) {
        self.reset();
    }
}

impl Clone for BlockAllocator {
    fn clone(&self) -> Self {
        Self {
            blocks: self.blocks.clone(),
            block_size: self.block_size,
            ptr_map: self.ptr_map.clone(),
            total_size: self.total_size,
            total_allocated: self.total_allocated,
            total_freed: self.total_freed,
            free_blocks: self.free_blocks.clone(),
            slot_sizes: self.slot_sizes.clone(),
            stats: self.stats.clone(),
            max_size: self.max_size,
            start: self.start,
            num_blocks: self.num_blocks,
        }
    }
}

// The method wrapping_offset is implemented for raw pointers
//
// This change fixes the error for raw_ptr.wrapping_offset in find_free_slot()
unsafe impl<T> RawPtrExt<T> for *mut T {
    fn wrapping_offset(self, offset: isize) -> *mut T {
        unsafe { self.wrapping_offset(offset) }
    }
}

/// Extension trait for MemoryBlock references
pub trait MemoryBlockExt {
    /// Get the size of each slot in this block
    fn get_slot_size(&self) -> usize;
    
    /// Check if a slot is allocated
    fn is_allocated(&self, slot: usize) -> bool;
    
    /// Find a free slot in the block
    fn find_free_slot(&self) -> Option<usize>;
    
    /// Check if a slot is free
    fn is_slot_free(&self, slot: usize) -> bool;
    
    /// Checks if a pointer is contained within this memory block
    fn contains_ptr(&self, ptr: *mut u8) -> bool;
    
    /// Gets the slot index for a given pointer
    fn get_slot_index(&self, ptr: NonNull<u8>) -> Option<usize>;
    
    /// Checks if a pointer is allocated in this block
    fn is_pointer_allocated(&self, ptr: *mut u8) -> bool;
}

impl MemoryBlockExt for MemoryBlock {
    fn get_slot_size(&self) -> usize {
        self.slot_size
    }
    
    fn is_allocated(&self, slot: usize) -> bool {
        if slot >= self.slots {
            return false;
        }
        
        let byte_index = slot / 8;
        let bit_index = slot % 8;
        let mask = 1 << bit_index;
        
        (self.bitmap[byte_index] & mask) != 0
    }
    
    fn find_free_slot(&self) -> Option<usize> {
        if self.free_slots == 0 {
            return None;
        }
        
        // Check the bitmap to find a free slot
        for i in 0..self.slots {
            let byte_index = i / 8;
            let bit_index = i % 8;
            
            if byte_index < self.bitmap.len() {
                let byte = self.bitmap[byte_index];
                let is_allocated = (byte & (1 << bit_index)) != 0;
                
                if !is_allocated {
                    return Some(i);
                }
            }
        }
        
        None
    }
    
    fn is_slot_free(&self, slot: usize) -> bool {
        !self.is_allocated(slot)
    }
    
    fn contains_ptr(&self, ptr: *mut u8) -> bool {
        let self_ptr = self.ptr.as_ptr();
        let ptr_val = ptr as usize;
        let self_val = self_ptr as usize;
        
        // Check if ptr is within the bounds of this block
        ptr_val >= self_val && ptr_val < (self_val + self.size)
    }
    
    fn get_slot_index(&self, ptr: NonNull<u8>) -> Option<usize> {
        let base_ptr = self.ptr.as_ptr();
        let ptr_val = ptr.as_ptr() as usize;
        let base_val = base_ptr as usize;
        
        if !self.contains_ptr(ptr.as_ptr()) {
            return None;
        }
        
        // Calculate offset from base and determine slot
        let offset = ptr_val - base_val;
        let slot = offset / self.slot_size;
        
        if slot < self.slots {
            Some(slot)
        } else {
            None
        }
    }
    
    fn is_pointer_allocated(&self, ptr: *mut u8) -> bool {
        if !self.contains_ptr(ptr) {
            return false;
        }
        
        if let Some(slot) = self.get_slot_index(NonNull::new(ptr).unwrap()) {
            return self.is_allocated(slot);
        }
        
        false
    }
}

/// Extension trait for MemoryBlock mutable references
pub trait MemoryBlockExtMut: MemoryBlockExt {
    /// Marks a slot as allocated
    fn mark_allocated(&mut self, slot: usize);
    
    /// Marks a slot as free
    fn mark_free(&mut self, slot: usize);
    
    /// Get the pointer for a given slot
    fn get_slot_ptr(&self, slot: usize) -> Option<NonNull<u8>>;
}

impl MemoryBlockExtMut for MemoryBlock {
    fn mark_allocated(&mut self, slot: usize) {
        if slot < self.slots {
            let byte_index = slot / 8;
            let bit_index = slot % 8;
            let mask = 1 << bit_index;
            
            self.bitmap[byte_index] |= mask;
            self.free_slots -= 1;
        }
    }
    
    fn mark_free(&mut self, slot: usize) {
        if slot < self.slots {
            let byte_index = slot / 8;
            let bit_index = slot % 8;
            let mask = 1 << bit_index;
            
            self.bitmap[byte_index] &= !mask;
            self.free_slots += 1;
        }
    }
    
    fn get_slot_ptr(&self, slot: usize) -> Option<NonNull<u8>> {
        if slot >= self.slots {
            return None;
        }

        let raw_ptr = self.ptr.as_ptr();
        let offset = slot * self.slot_size;
        
        // Calculate the pointer to the slot
        let ptr = unsafe {
            let ptr = raw_ptr.add(offset);
            NonNull::new_unchecked(ptr) 
        };
        
        Some(ptr)
    }
}

/// Extension trait for BlockAllocator references
pub trait BlockAllocatorExt {
    /// Get the memory capacity of the allocator
    fn memory_capacity(&self) -> usize;
    
    /// Get the current memory usage in bytes
    fn memory_usage(&self) -> usize;
    
    /// Get the blocks in this allocator
    fn blocks(&self) -> &[MemoryBlock];
    
    /// Get the free blocks in this allocator
    fn free_blocks(&self) -> &[usize];
    
    /// Get the pointer map in this allocator
    fn ptr_map(&self) -> &HashMap<usize, (usize, usize)>;
    
    /// Calculate the number of free slots in all blocks
    fn count_free_slots(&self) -> usize;
}

impl BlockAllocatorExt for BlockAllocator {
    fn memory_capacity(&self) -> usize {
        self.total_size
    }
    
    fn memory_usage(&self) -> usize {
        let mut used = 0;
        for block in &self.blocks {
            let allocated_slots = block.slots - block.free_slots;
            used += allocated_slots * block.slot_size;
        }
        used
    }
    
    fn blocks(&self) -> &[MemoryBlock] {
        &self.blocks
    }
    
    fn free_blocks(&self) -> &[usize] {
        &self.free_blocks
    }
    
    fn ptr_map(&self) -> &HashMap<usize, (usize, usize)> {
        &self.ptr_map
    }
    
    fn count_free_slots(&self) -> usize {
        self.blocks.iter().map(|block| block.free_slots).sum()
    }
}

// Implementation of TaggedPtrConstructor for BlockAllocator
impl TaggedPtrConstructor for BlockAllocator {
    type T = BlockAllocator;
    
    fn new(ptr: Option<NonNull<Self::T>>, _tag: Tag) -> TaggedPtr<Self::T> {
        TaggedPtr {
            ptr,
            tag: 0,
            _phantom: PhantomData,
        }
    }
}

// Implement MemoryBlockExt trait methods
impl MemoryBlockExt for BlockAllocator {
    fn get_slot_size(&self) -> usize {
        // Default implementation returns the first slot size
        self.slot_sizes.first().copied().unwrap_or(MIN_ALIGNMENT)
    }
    
    fn is_allocated(&self, slot: usize) -> bool {
        // This implementation is a placeholder since BlockAllocator doesn't directly implement this
        false
    }
    
    fn find_free_slot(&self) -> Option<usize> {
        // Look for any block with a free slot
        for (index, block) in self.blocks.iter().enumerate() {
            if block.free_slots > 0 {
                return Some(index);
            }
        }
        None
    }
    
    fn is_slot_free(&self, slot: usize) -> bool {
        // This implementation is a placeholder since BlockAllocator doesn't directly implement this
        false
    }
    
    fn contains_ptr(&self, ptr: *mut u8) -> bool {
        // Check if the pointer is within the total allocated memory range
        let ptr_addr = ptr as usize;
        let start_addr = self.start.as_ptr() as usize;
        let end_addr = start_addr + self.total_size;
        
        ptr_addr >= start_addr && ptr_addr < end_addr
    }
    
    fn get_slot_index(&self, ptr: NonNull<u8>) -> Option<usize> {
        // Convert to usize for easier comparison
        let ptr_addr = ptr.as_ptr() as usize;
        
        // Check if we have this pointer in our map
        self.ptr_map.get(&ptr_addr).map(|(_, slot_idx)| *slot_idx)
    }
    
    fn is_pointer_allocated(&self, ptr: *mut u8) -> bool {
        // Convert to usize for easier comparison
        let ptr_addr = ptr as usize;
        
        // Check if we have this pointer in our map
        self.ptr_map.contains_key(&ptr_addr)
    }
}

impl Block {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            is_free: true,
            next: None,
            prev: None,
        }
    }
}

pub struct BlockStats {
    pub total_size: usize,
    pub used_size: usize,
    pub free_blocks: usize,
    pub total_blocks: usize,
}

impl Default for BlockStats {
    fn default() -> Self {
        Self {
            total_size: 0,
            used_size: 0,
            free_blocks: 0,
            total_blocks: 0,
        }
    }
}

impl Clone for BlockStats {
    fn clone(&self) -> Self {
        Self {
            total_size: self.total_size,
            used_size: self.used_size,
            free_blocks: self.free_blocks,
            total_blocks: self.total_blocks,
        }
    }
} 