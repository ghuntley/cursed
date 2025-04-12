// Block allocator implementation
use crate::memory::allocator::{Allocator, AllocatorBase};
use std::ptr::NonNull;

/// A simple block allocator that manages fixed-size blocks of memory
pub struct BlockAllocator {
    blocks: Vec<Block>,
}

/// A block of memory
struct Block {
    ptr: NonNull<u8>,
    size: usize,
    used: bool,
}

impl BlockAllocator {
    /// Create a new block allocator
    pub fn new() -> Self {
        BlockAllocator { blocks: Vec::new() }
    }

    /// Create a new block allocator with an initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        BlockAllocator {
            blocks: Vec::with_capacity(capacity),
        }
    }

    /// Allocate a new block from the system allocator
    unsafe fn allocate_block(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            None
        } else {
            let ptr = NonNull::new_unchecked(ptr);
            self.blocks.push(Block {
                ptr,
                size,
                used: true,
            });
            Some(ptr)
        }
    }

    /// Find a free block that fits the requested size
    fn find_free_block(&mut self, size: usize) -> Option<usize> {
        self.blocks
            .iter()
            .enumerate()
            .find(|(_, block)| !block.used && block.size >= size)
            .map(|(index, _)| index)
    }

    /// Mark a block as used
    fn mark_used(&mut self, index: usize) {
        self.blocks[index].used = true;
    }

    /// Mark a block as free
    fn mark_free(&mut self, ptr: NonNull<u8>) {
        if let Some(index) = self.blocks.iter().position(|block| block.ptr == ptr) {
            self.blocks[index].used = false;
        }
    }
}

impl AllocatorBase for BlockAllocator {
    unsafe fn allocate(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        if let Some(index) = self.find_free_block(size) {
            self.mark_used(index);
            Some(self.blocks[index].ptr)
        } else {
            self.allocate_block(size, align)
        }
    }

    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, _size: usize, _align: usize) {
        self.mark_free(ptr);
    }

    unsafe fn reallocate(
        &mut self,
        ptr: NonNull<u8>,
        old_size: usize,
        new_size: usize,
        align: usize,
    ) -> Option<NonNull<u8>> {
        // Simple implementation: allocate new block, copy data, free old block
        let new_ptr = self.allocate(new_size, align)?;
        std::ptr::copy_nonoverlapping(
            ptr.as_ptr(),
            new_ptr.as_ptr(),
            std::cmp::min(old_size, new_size),
        );
        self.deallocate(ptr, old_size, align);
        Some(new_ptr)
    }
}

impl Allocator for BlockAllocator {}
