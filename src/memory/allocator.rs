// Memory Allocator Trait
//
// This module defines the Allocator trait that all allocators in the CURSED memory
// management system must implement.

use std::alloc::Layout;
use std::ptr::NonNull;
use crate::error::Error;

/// A trait for memory allocators
pub trait Allocator {
    /// Allocate memory with the given layout
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, Error>;
    
    /// Deallocate memory that was allocated with this allocator
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout);
    
    /// Reset the allocator, deallocating all memory
    fn reset(&mut self) -> Result<(), Error>;
} 