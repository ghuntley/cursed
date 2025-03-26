// Memory Allocator Trait
//
// This module defines the Allocator trait that all allocators in the CURSED memory
// management system must implement.

use std::alloc::Layout;
use std::ptr::NonNull;
use crate::error::Error;
use std::fmt;

/// Base trait for allocators that can be used as trait objects
pub trait AllocatorBase {
    /// Allocate memory with the given layout
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, Error>;
    
    /// Deallocate memory with the given layout
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
    
    /// Reset the allocator to its initial state
    fn reset(&mut self);
    
    /// Get the total memory capacity
    fn memory_capacity(&self) -> usize;
    
    /// Get the current memory usage
    fn memory_usage(&self) -> usize;
}

/// Trait for allocators that can allocate specific types
pub trait Allocator: AllocatorBase {
    /// Allocate memory for a specific type
    fn allocate_type<T>(&self) -> Result<NonNull<T>, Error> {
        let layout = Layout::new::<T>();
        self.allocate(layout).map(|ptr| NonNull::new(ptr.as_ptr() as *mut T).unwrap())
    }
    
    /// Deallocate memory for a specific type
    unsafe fn deallocate_type<T>(&self, ptr: NonNull<T>) {
        let layout = Layout::new::<T>();
        self.deallocate(ptr.cast(), layout);
    }
    
    /// Allocate memory for a slice of a specific type
    fn allocate_slice<T>(&self, len: usize) -> Result<NonNull<[T]>, Error> {
        if len == 0 {
            return Err(Error::Memory("Cannot allocate a zero-length slice".to_string()));
        }
        
        let layout = match Layout::array::<T>(len) {
            Ok(layout) => layout,
            Err(_) => return Err(Error::Memory("Invalid layout for slice allocation".to_string())),
        };
        
        let ptr = self.allocate(layout)?;
        let ptr = ptr.as_ptr() as *mut T;
        let slice_ptr = std::ptr::slice_from_raw_parts_mut(ptr, len);
        
        Ok(unsafe { NonNull::new_unchecked(slice_ptr) })
    }
    
    /// Deallocate memory for a slice of a specific type
    unsafe fn deallocate_slice<T>(&self, ptr: NonNull<[T]>) {
        let ptr = ptr.as_ptr() as *mut T;
        let len = std::slice::from_raw_parts(ptr, 0).len();
        let layout = Layout::array::<T>(len).unwrap();
        
        self.deallocate(NonNull::new_unchecked(ptr as *mut u8), layout);
    }
}

/// Helper function to align a size to a power of 2
pub fn align_to_power_of_two(size: usize) -> usize {
    let mut aligned = size;
    aligned -= 1;
    aligned |= aligned >> 1;
    aligned |= aligned >> 2;
    aligned |= aligned >> 4;
    aligned |= aligned >> 8;
    aligned |= aligned >> 16;
    aligned |= aligned >> 32;
    aligned += 1;
    aligned
}

/// Helper function to check if a size is a power of 2
pub fn is_power_of_two(size: usize) -> bool {
    size > 0 && (size & (size - 1)) == 0
}

/// Helper function to align a size to a given alignment
pub fn align_up(size: usize, align: usize) -> usize {
    (size + align - 1) & !(align - 1)
}

/// Helper function to align a size down to a given alignment
pub fn align_down(size: usize, align: usize) -> usize {
    size & !(align - 1)
}

/// Helper function to check if a size is aligned to a given alignment
pub fn is_aligned(size: usize, align: usize) -> bool {
    size & (align - 1) == 0
}

/// Helper function to get the next power of 2
pub fn next_power_of_2(n: usize) -> usize {
    let mut n = n - 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n + 1
}

/// Helper function to get the previous power of 2
pub fn prev_power_of_2(n: usize) -> usize {
    let mut n = n;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n - (n >> 1)
} 