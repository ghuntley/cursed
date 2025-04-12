// Bump allocator implementation
use crate::memory::allocator::{Allocator, AllocatorBase};
use std::cell::Cell;
use std::ptr::NonNull;

/// A bump allocator that allocates memory linearly and only frees all at once
#[derive(Debug)]
pub struct BumpAllocator {
    memory: NonNull<u8>,
    capacity: usize,
    current: Cell<usize>,
}

impl BumpAllocator {
    /// Create a new bump allocator with the given capacity
    pub fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::from_size_align(capacity, 8)
            .expect("Invalid layout for bump allocator");

        let memory = unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            NonNull::new_unchecked(ptr)
        };

        BumpAllocator {
            memory,
            capacity,
            current: Cell::new(0),
        }
    }

    /// Reset the allocator, effectively freeing all allocations
    pub fn reset(&mut self) {
        self.current.set(0);
    }

    /// Get the current offset into the memory block
    pub fn offset(&self) -> usize {
        self.current.get()
    }

    /// Get the remaining capacity
    pub fn remaining(&self) -> usize {
        self.capacity - self.current.get()
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        // Free the entire memory block when the allocator is dropped
        let layout = std::alloc::Layout::from_size_align(self.capacity, 8)
            .expect("Invalid layout for bump allocator");
        unsafe {
            std::alloc::dealloc(self.memory.as_ptr(), layout);
        }
    }
}

impl AllocatorBase for BumpAllocator {
    unsafe fn allocate(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        // Calculate the aligned offset
        let offset = self.current.get();
        let aligned_offset = (offset + align - 1) & !(align - 1);

        // Check if we have enough space
        if aligned_offset + size > self.capacity {
            return None;
        }

        // Update the current offset
        self.current.set(aligned_offset + size);

        // Return the pointer to the allocated memory
        Some(NonNull::new_unchecked(
            self.memory.as_ptr().add(aligned_offset),
        ))
    }

    unsafe fn deallocate(&mut self, _ptr: NonNull<u8>, _size: usize, _align: usize) {
        // Bump allocator doesn't support individual deallocation
        // Memory is only freed when reset() is called or the allocator is dropped
    }

    unsafe fn reallocate(
        &mut self,
        ptr: NonNull<u8>,
        old_size: usize,
        new_size: usize,
        align: usize,
    ) -> Option<NonNull<u8>> {
        // For a bump allocator, we can only append to the current allocation if it's the last one
        let current_ptr = self.memory.as_ptr().add(self.current.get() - old_size);
        if current_ptr == ptr.as_ptr() && new_size > old_size {
            // This is the last allocation, we can just extend it if there's enough space
            let additional = new_size - old_size;
            if self.current.get() + additional <= self.capacity {
                self.current.set(self.current.get() + additional);
                return Some(ptr);
            }
        }

        // Otherwise, we need to allocate a new block and copy the data
        let new_ptr = self.allocate(new_size, align)?;
        std::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), old_size);
        Some(new_ptr)
    }
}

impl Allocator for BumpAllocator {}
