//! Stack data structures for CURSED collections
//!
//! This module provides comprehensive stack implementations:
//! - Stack: Generic stack with push/pop operations
//! - FixedStack: Stack with fixed capacity
//! - ThreadSafeStack: Thread-safe stack with synchronization
//! - StackWithMin: Stack that tracks minimum element

use crate::stdlib::collections::{CollectionsResult, CollectionsError};
use std::sync::{Arc, Mutex};

/// Generic stack data structure with LIFO (Last-In-First-Out) operations
#[derive(Debug, Clone)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Create a new stack with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Push an element onto the stack
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// Pop an element from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Peek at the top element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Get the number of elements in the stack
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear all elements from the stack
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the current capacity of the stack
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixed-capacity stack that prevents overflow
#[derive(Debug, Clone)]
pub struct FixedStack<T> {
    data: Vec<T>,
    max_capacity: usize,
}

impl<T> FixedStack<T> {
    /// Create a new fixed stack with specified capacity
    pub fn new(capacity: usize) -> CollectionsResult<Self> {
        if capacity == 0 {
            return Err(CollectionsError::InvalidCapacity { capacity });
        }
        
        Ok(Self {
            data: Vec::with_capacity(capacity),
            max_capacity: capacity,
        })
    }

    /// Push an element onto the stack (fails if at capacity)
    pub fn push(&mut self, item: T) -> CollectionsResult<()> {
        if self.data.len() >= self.max_capacity {
            return Err(CollectionsError::InvalidOperation {
                operation: "push".to_string(),
                reason: "Stack at maximum capacity".to_string(),
            });
        }
        self.data.push(item);
        Ok(())
    }

    /// Pop an element from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Peek at the top element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Get the number of elements in the stack
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Check if the stack is at full capacity
    pub fn is_full(&self) -> bool {
        self.data.len() >= self.max_capacity
    }

    /// Get the maximum capacity of the stack
    pub fn capacity(&self) -> usize {
        self.max_capacity
    }

    /// Clear all elements from the stack
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

/// Thread-safe stack using Arc<Mutex<>>
#[derive(Debug, Clone)]
pub struct ThreadSafeStack<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> ThreadSafeStack<T> {
    /// Create a new thread-safe stack
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Push an element onto the stack
    pub fn push(&self, item: T) -> CollectionsResult<()> {
        match self.data.lock() {
            Ok(mut stack) => {
                stack.push(item);
                Ok(())
            }
            Err(_) => Err(CollectionsError::InvalidOperation {
                operation: "push".to_string(),
                reason: "Failed to acquire lock".to_string(),
            }),
        }
    }

    /// Pop an element from the stack
    pub fn pop(&self) -> CollectionsResult<Option<T>> {
        match self.data.lock() {
            Ok(mut stack) => Ok(stack.pop()),
            Err(_) => Err(CollectionsError::InvalidOperation {
                operation: "pop".to_string(),
                reason: "Failed to acquire lock".to_string(),
            }),
        }
    }

    /// Get the number of elements in the stack
    pub fn len(&self) -> CollectionsResult<usize> {
        match self.data.lock() {
            Ok(stack) => Ok(stack.len()),
            Err(_) => Err(CollectionsError::InvalidOperation {
                operation: "len".to_string(),
                reason: "Failed to acquire lock".to_string(),
            }),
        }
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> CollectionsResult<bool> {
        match self.data.lock() {
            Ok(stack) => Ok(stack.is_empty()),
            Err(_) => Err(CollectionsError::InvalidOperation {
                operation: "is_empty".to_string(),
                reason: "Failed to acquire lock".to_string(),
            }),
        }
    }
}

impl<T> Default for ThreadSafeStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Stack that tracks the minimum element for O(1) min operations
#[derive(Debug, Clone)]
pub struct StackWithMin<T: Clone + Ord> {
    data: Vec<T>,
    min_stack: Vec<T>,
}

impl<T: Clone + Ord> StackWithMin<T> {
    /// Create a new stack with min tracking
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    /// Push an element onto the stack
    pub fn push(&mut self, item: T) {
        // Update min stack
        if self.min_stack.is_empty() || item <= *self.min_stack.last().unwrap() {
            self.min_stack.push(item.clone());
        }
        self.data.push(item);
    }

    /// Pop an element from the stack
    pub fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.data.pop() {
            // Update min stack if we're removing the minimum
            if let Some(min_item) = self.min_stack.last() {
                if item == *min_item {
                    self.min_stack.pop();
                }
            }
            Some(item)
        } else {
            None
        }
    }

    /// Peek at the top element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Get the minimum element in O(1) time
    pub fn min(&self) -> Option<&T> {
        self.min_stack.last()
    }

    /// Get the number of elements in the stack
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear all elements from the stack
    pub fn clear(&mut self) {
        self.data.clear();
        self.min_stack.clear();
    }
}

impl<T: Clone + Ord> Default for StackWithMin<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize stack processing
pub fn init_stacks() -> CollectionsResult<()> {
    let mut stack = Stack::new();
    stack.push("test");
    let result = stack.pop();
    if result != Some("test") {
        return Err(CollectionsError::InvalidOperation {
            operation: "init_test".to_string(),
            reason: "Stack test failed".to_string(),
        });
    }
    println!("📚 Stack processing initialized");
    Ok(())
}

/// Test stack functionality
pub fn test_stacks() -> CollectionsResult<()> {
    // Test basic stack
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    assert_eq!(stack.len(), 3);
    assert_eq!(stack.peek(), Some(&3));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.len(), 1);
    
    // Test fixed stack
    let mut fixed_stack = FixedStack::new(2)?;
    fixed_stack.push(10)?;
    fixed_stack.push(20)?;
    
    assert!(fixed_stack.is_full());
    assert!(fixed_stack.push(30).is_err()); // Should fail
    
    // Test stack with min
    let mut min_stack = StackWithMin::new();
    min_stack.push(5);
    min_stack.push(3);
    min_stack.push(7);
    min_stack.push(1);
    
    assert_eq!(min_stack.min(), Some(&1));
    min_stack.pop(); // Remove 1
    assert_eq!(min_stack.min(), Some(&3));
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stack_operations() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_fixed_stack() {
        let mut stack = FixedStack::new(2).unwrap();
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert!(stack.is_full());
        assert!(stack.push(3).is_err());
    }

    #[test]
    fn test_thread_safe_stack() {
        let stack = ThreadSafeStack::new();
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        
        assert_eq!(stack.len().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), Some(2));
        assert_eq!(stack.pop().unwrap(), Some(1));
        assert!(stack.is_empty().unwrap());
    }

    #[test]
    fn test_stack_with_min() {
        let mut stack = StackWithMin::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);
        stack.push(1);
        
        assert_eq!(stack.min(), Some(&1));
        stack.pop();
        assert_eq!(stack.min(), Some(&3));
        stack.pop();
        assert_eq!(stack.min(), Some(&3));
        stack.pop();
        assert_eq!(stack.min(), Some(&5));
    }
}
