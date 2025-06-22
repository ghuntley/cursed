/// Comprehensive collections library for CURSED
/// 
/// This module provides efficient collection types including:
/// - Sets: HashSet, TreeSet, BitSet for element uniqueness
/// - Queues: Queue, Deque, PriorityQueue, CircularQueue for FIFO/LIFO operations
/// - Stacks: Stack, FixedStack, ThreadSafeStack, StackWithMin for LIFO operations
/// - Heaps: Binary heap implementation with priority queue support (heap_slay)
/// - Iterators: Comprehensive iterator system with lazy evaluation and functional programming
/// - Iterator Utils: Advanced iterator operations, parallel processing, and utilities

pub mod sets;
pub mod queues;
pub mod stacks;
pub mod heap_slay;
pub mod iterators_simple;
pub mod advanced;
pub mod sorta_fresh;

// Re-export all public types and functions for easy access
pub use sets::*;
pub use queues::*;
pub use stacks::*;
// Use explicit imports from heap_slay to avoid conflicts
pub use heap_slay::{
    HeapSorter, BinaryHeap, MinHeap, MaxHeap, HeapIterator,
    PriorityQueue as HeapPriorityQueue, HeapError
};
pub use iterators_simple::*;
pub use advanced::*;
pub use sorta_fresh::*;

// Collections result type
pub type CollectionsResult<T> = Result<T, CollectionsError>;

/// Errors that can occur during collection operations
#[derive(Debug, Clone, PartialEq)]
pub enum CollectionsError {
    IndexOutOfBounds { index: usize, size: usize },
    ElementNotFound { element: String },
    InvalidCapacity { capacity: usize },
    InvalidRange { start: usize, end: usize },
    TypeMismatch { expected: String, found: String },
    OperationNotSupported { operation: String, collection_type: String },
    InsufficientMemory { requested: usize },
    InvalidBitIndex { index: usize, max_bits: usize },
    InvalidOperation { operation: String, reason: String },
}

impl std::fmt::Display for CollectionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollectionsError::IndexOutOfBounds { index, size } => {
                write!(f, "Index {} out of bounds for collection of size {}", index, size)
            }
            CollectionsError::ElementNotFound { element } => {
                write!(f, "Element not found: {}", element)
            }
            CollectionsError::InvalidCapacity { capacity } => {
                write!(f, "Invalid capacity: {}", capacity)
            }
            CollectionsError::InvalidRange { start, end } => {
                write!(f, "Invalid range {}..{}", start, end)
            }
            CollectionsError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            CollectionsError::OperationNotSupported { operation, collection_type } => {
                write!(f, "Operation '{}' not supported for {}", operation, collection_type)
            }
            CollectionsError::InsufficientMemory { requested } => {
                write!(f, "Insufficient memory: requested {} bytes", requested)
            }
            CollectionsError::InvalidBitIndex { index, max_bits } => {
                write!(f, "Bit index {} out of bounds (max: {})", index, max_bits)
            }
            CollectionsError::InvalidOperation { operation, reason } => {
                write!(f, "Invalid operation '{}': {}", operation, reason)
            }
        }
    }
}

impl std::error::Error for CollectionsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collections_error_display() {
        let error = CollectionsError::IndexOutOfBounds { index: 5, size: 3 };
        assert_eq!(error.to_string(), "Index 5 out of bounds for collection of size 3");

        let error = CollectionsError::ElementNotFound { element: "test".to_string() };
        assert_eq!(error.to_string(), "Element not found: test");

        let error = CollectionsError::InvalidCapacity { capacity: 0 };
        assert_eq!(error.to_string(), "Invalid capacity: 0");

        let error = CollectionsError::TypeMismatch { 
            expected: "String".to_string(), 
            found: "Number".to_string() 
        };
        assert_eq!(error.to_string(), "Type mismatch: expected String, found Number");
    }
}
