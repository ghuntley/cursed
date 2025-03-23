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

// Module exports
pub mod allocator;
pub mod bump;
pub mod block;
pub mod tagged;
pub mod gc;

#[cfg(test)]
mod tests;

// Re-export the public API
pub use allocator::Allocator;
pub use block::BlockAllocator;
pub use bump::BumpAllocator;

// Re-export GC-related items with clear visibility
pub use gc::GarbageCollector;
pub use gc::Gc;
pub use gc::Traceable;
pub use gc::Visitor;

// Re-export tagged pointer functionality
pub use tagged::TaggedPtr;
pub use tagged::Tag;
pub use tagged::NonNullExt;

/// Minimum alignment for all allocations
pub const MIN_ALIGNMENT: usize = 8;

/// Default block size for allocators
pub const DEFAULT_BLOCK_SIZE: usize = 4 * 1024; // 4KB

/// Align a value up to the specified alignment
pub fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

/// A memory allocated object with a reference to its allocator
pub struct Allocated<T> {
    /// Pointer to the object
    ptr: NonNull<T>,
    
    /// Layout of the allocated memory
    layout: Layout,
    
    /// The allocator that allocated this memory
    allocator: Rc<dyn Allocator>,
    
    /// Phantom data to ensure type safety
    _phantom: PhantomData<T>,
}

impl<T> Allocated<T> {
    /// Create a new allocated object
    pub fn new(ptr: NonNull<u8>, layout: Layout, allocator: Rc<dyn Allocator>) -> Self {
        let ptr = ptr.cast();
        Self {
            ptr,
            layout,
            allocator,
            _phantom: PhantomData,
        }
    }
    
    /// Get a reference to the allocated object
    pub fn get(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
    
    /// Get a mutable reference to the allocated object
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
    
    /// Get the pointer to the allocated object
    pub fn ptr(&self) -> NonNull<T> {
        self.ptr
    }
}

impl<T> Drop for Allocated<T> {
    fn drop(&mut self) {
        let mut allocator = self.allocator.clone();
        unsafe {
            let alloc_ref = Rc::get_mut(&mut allocator).unwrap();
            alloc_ref.deallocate(self.ptr.cast(), self.layout);
        }
    }
}

/// Memory management error type
#[derive(Debug, Clone)]
pub enum MemoryError {
    /// Out of memory error
    OutOfMemory,
    /// Invalid layout error
    InvalidLayout,
    /// Invalid pointer error
    InvalidPointer,
    /// Null pointer error
    NullPointer,
    /// Already allocated error
    AlreadyAllocated,
    /// Invalid block size
    InvalidBlockSize,
    /// Invalid slot
    InvalidSlot,
    /// Garbage collector error
    GCError(String),
    /// Bump allocator error
    BumpError(String),
    /// Block allocator error
    BlockError(String),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::OutOfMemory => write!(f, "Out of memory"),
            MemoryError::InvalidLayout => write!(f, "Invalid memory layout"),
            MemoryError::InvalidPointer => write!(f, "Invalid pointer"),
            MemoryError::NullPointer => write!(f, "Null pointer"),
            MemoryError::AlreadyAllocated => write!(f, "Memory already allocated"),
            MemoryError::InvalidBlockSize => write!(f, "Invalid block size"),
            MemoryError::InvalidSlot => write!(f, "Invalid slot"),
            MemoryError::GCError(msg) => write!(f, "Garbage collector error: {}", msg),
            MemoryError::BumpError(msg) => write!(f, "Bump allocator error: {}", msg),
            MemoryError::BlockError(msg) => write!(f, "Block allocator error: {}", msg),
        }
    }
}

impl From<MemoryError> for Error {
    fn from(error: MemoryError) -> Self {
        Error::Runtime(error.to_string())
    }
}

/// A memory manager that provides different allocation strategies
pub struct MemoryManager {
    /// Bump allocator for fast, stack-like allocations
    bump_allocator: Rc<RefCell<BumpAllocator>>,
    
    /// Block allocator for individual object allocations
    block_allocator: Rc<RefCell<BlockAllocator>>,
    
    /// Garbage collector for managed objects
    pub garbage_collector: Rc<RefCell<GarbageCollector>>,
}

impl MemoryManager {
    /// Create a new memory manager with the default configuration
    pub fn new(total_size: usize) -> Result<Self, Error> {
        Self::new_with_sizes(
            total_size / 4, // Bump allocator gets 1/4
            total_size / 4, // Block allocator gets 1/4
            total_size / 2, // GC gets 1/2
        )
    }
    
    /// Create a new memory manager with specific allocator sizes
    pub fn new_with_sizes(
        bump_size: usize,
        block_size: usize,
        gc_size: usize
    ) -> Result<Self, Error> {
        // Create allocators
        let bump_allocator = BumpAllocator::new(bump_size);
        let block_allocator = BlockAllocator::new(block_size);
        let garbage_collector = GarbageCollector::new(gc_size);
        
        Ok(Self {
            bump_allocator: Rc::new(RefCell::new(bump_allocator)),
            block_allocator: Rc::new(RefCell::new(block_allocator)),
            garbage_collector: Rc::new(RefCell::new(garbage_collector)),
        })
    }
    
    /// Allocate memory with the default allocator
    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, Error> {
        // Try the block allocator first for individual objects
        match self.block_allocator.borrow_mut().allocate(layout) {
            Ok(ptr) => Ok(ptr),
            Err(_) => {
                // Try the bump allocator as backup
                self.bump_allocator.borrow_mut().allocate(layout)
            }
        }
    }
    
    /// Collect garbage
    pub fn collect_garbage<T>(&mut self, roots: &[Rc<T>]) -> Result<(), Error> {
        // For simplicity, we'll just run a collection cycle
        let mut gc = self.garbage_collector.borrow_mut();
        
        // Run collection cycle
        if gc.should_collect() {
            gc.run_collection();
        }
        
        Ok(())
    }
}

// Implementation moved to src/memory/tests.rs
// #[cfg(test)]
// mod tests { ... }

// No need for a VecExt trait here, removed in favor of the one in prelude
// trait VecExt<T> { ... } 