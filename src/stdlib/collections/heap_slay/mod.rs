// # heap_slay (container/heap)
//
// The `heap_slay` module provides an implementation of the heap (priority queue) data structure.
// It supports insertion, removal, and extraction of the minimum or maximum element in logarithmic time.
// This implementation uses the collab approach, making it flexible for various element types.
//
// ## Core Features
//
// - **Interface-based Design**: Any type implementing the `Interface` collab can use heap operations
// - **Logarithmic Time Complexity**: O(log n) for all major operations (Push, Pop, Remove)
// - **Flexible Ordering**: Support for both min and max heaps through custom Less functions
// - **Memory Efficient**: In-place operations that maintain heap property
// - **Type Safety**: Strongly typed implementations for common use cases
//
// ## Usage Examples
//
// ```cursed
// import "stdlib::collections::heap_slay";
//
// // Basic IntHeap example
// let mut h = IntHeap::new();
// heap_slay::push(&mut h, 3);
// heap_slay::push(&mut h, 1);
// heap_slay::push(&mut h, 5);
// let min = heap_slay::pop(&mut h); // Returns 1
//
// // Priority queue example
// let mut pq = PriorityQueue::new();
// let item = Item { value: "task", priority: 5, index: 0 };
// heap_slay::push(&mut pq, item);
// ```

pub mod core;
pub mod types;

// Re-export core functionality
pub use core::{
    Interface,
    init,
    push,
    pop,
    remove,
    fix,
    is_heap,
};

// Re-export convenience types
pub use types::{
    IntHeap,
    StringHeap,
    Item,
    PriorityQueue,
    HeapSorter,
    BinaryHeap,
    MinHeap,
    MaxHeap,
    HeapIterator,
    HeapError,
};

use crate::error::{CursedError, CursedError};

/// Result type for heap operations
pub type Heapcrate::error::Result<T> = std::result::Result<T>;

/// Create a heap-specific error
pub fn heap_error(message: &str) -> CursedError {
    CursedError::General(format!("heap_slay: {}", message))
}

/// Validate heap index bounds
pub fn validate_index(index: i32, len: i32) -> HeapResult<()> {
    if index < 0 || index >= len {
        return Err(heap_error(&format!(
            "index {} out of bounds for heap of length {}",
            index, len
        )));
    }
    Ok(())
}

/// Validate that heap is not empty for pop operations
pub fn validate_not_empty(len: i32) -> HeapResult<()> {
    if len == 0 {
        return Err(heap_error("cannot pop from empty heap"));
    }
    Ok(())
}

