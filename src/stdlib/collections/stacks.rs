use crate::error::CursedError;
/// Comprehensive Stack implementations for CURSED
/// 
/// This module provides four types of stacks:
/// - Stack<T>: LIFO stack with dynamic resizing and O(1) operations
/// - FixedStack<T>: Fixed-capacity stack for memory-constrained environments
/// - ThreadSafeStack<T>: Concurrent stack with lock-based synchronization
/// - StackWithMin<T>: Stack that tracks minimum element in O(1)

use super::{CollectionsError, CollectionsResult};
use std::cmp::{Ord, Ordering};
use std::fmt::{Debug, Display};
use std::iter::{Iterator, FromIterator};
use std::mem;
use std::sync::{Arc, Mutex, MutexGuard};

/// LIFO stack implementation with dynamic resizing
#[derive(Debug, Clone)]
pub struct Stack<T> {
/// Fixed-capacity stack for memory-constrained environments
#[derive(Debug, Clone)]
pub struct FixedStack<T> {
/// Thread-safe stack implementation using mutexes
#[derive(Debug)]
pub struct ThreadSafeStack<T> {
/// Stack that tracks minimum element in O(1) time
#[derive(Debug, Clone)]
pub struct StackWithMin<T> {
// ==================== Stack Implementation ====================

impl<T> Stack<T> {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new stack with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Push an element onto the stack
    pub fn push(&mut self, item: T) -> CollectionsResult<()> {
        self.inner.push(item);
        Ok(())
    /// Pop an element from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    /// Peek at the top element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.inner.last()
    /// Peek at the top element mutably without removing it
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.inner.last_mut()
    /// Get the number of elements in the stack
    pub fn len(&self) -> usize {
        self.inner.len()
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    /// Get the current capacity of the stack
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    /// Clear all elements from the stack
    pub fn clear(&mut self) {
        self.inner.clear();
    /// Reserve capacity for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    /// Shrink the capacity to fit the current number of elements
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    /// Push multiple elements onto the stack
    pub fn push_many<I>(&mut self, items: I) -> CollectionsResult<()>
    where
    {
        self.inner.extend(items);
        Ok(())
    /// Pop multiple elements from the stack
    pub fn pop_many(&mut self, count: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(count.min(self.len()));
        for _ in 0..count {
            if let Some(item) = self.pop() {
                result.push(item);
            } else {
                break;
            }
        }
        result
    /// Peek at multiple elements from the top
    pub fn peek_many(&self, count: usize) -> Vec<&T> {
        let actual_count = count.min(self.len());
        let start_index = self.len().saturating_sub(actual_count);
        self.inner[start_index..].iter().rev().collect()
    /// Get an iterator over stack elements (from top to bottom)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().rev()
    /// Convert stack to vector (top element becomes last)
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    /// Create stack from vector (last element becomes top)
    pub fn from_vec(mut vec: Vec<T>) -> Self {
        Self { inner: vec }
    }

    /// Duplicate the top element
    pub fn dup(&mut self) -> CollectionsResult<()>
    where
    {
        if let Some(top) = self.peek() {
            let item = top.clone();
            let _ = self.push(item);
            Ok(())
        } else {
            Err(CollectionsError::IndexOutOfBounds { index: 0, size: 0 })
        }
    }

    /// Swap the top two elements
    pub fn swap(&mut self) -> CollectionsResult<()> {
        if self.len() < 2 {
            return Err(CollectionsError::IndexOutOfBounds { 
                size: self.len() 
            });
        }
        let len = self.inner.len();
        self.inner.swap(len - 1, len - 2);
        Ok(())
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}

impl<T> Into<Vec<T>> for Stack<T> {
    fn into(self) -> Vec<T> {
        self.into_vec()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
        }
    }
impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::iter::Rev<std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter().rev()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Rev<std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter().rev()
    }
}

// ==================== FixedStack Implementation ====================

impl<T> FixedStack<T> {
    /// Create a new fixed-capacity stack
    pub fn new(capacity: usize) -> CollectionsResult<Self> {
        if capacity == 0 {
            return Err(CollectionsError::InvalidCapacity { capacity });
        }
        Ok(Self {
        })
    /// Push an element onto the stack
    pub fn push(&mut self, item: T) -> CollectionsResult<()> {
        if self.is_full() {
            return Err(CollectionsError::InsufficientMemory { 
                requested: std::mem::size_of::<T>() 
            });
        }
        self.data.push(item);
        Ok(())
    /// Pop an element from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    /// Peek at the top element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    /// Peek at the top element mutably without removing it
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    /// Get the number of elements in the stack
    pub fn len(&self) -> usize {
        self.data.len()
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }
    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    /// Check if the stack is full
    pub fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    /// Get the capacity of the stack
    pub fn capacity(&self) -> usize {
        self.capacity
    /// Get the remaining capacity
    pub fn remaining_capacity(&self) -> usize {
        self.capacity - self.len()
    /// Clear all elements from the stack
    pub fn clear(&mut self) {
        self.data.clear();
    /// Push multiple elements, stopping when full
    pub fn push_many<I>(&mut self, items: I) -> CollectionsResult<usize>
    where
    {
        let mut count = 0;
        for item in items {
            if self.is_full() {
                break;
            }
            self.push(item)?;
            count += 1;
        }
        Ok(count)
    /// Pop multiple elements from the stack
    pub fn pop_many(&mut self, count: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(count.min(self.len()));
        for _ in 0..count {
            if let Some(item) = self.pop() {
                result.push(item);
            } else {
                break;
            }
        }
        result
    /// Get an iterator over stack elements (from top to bottom)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().rev()
    }
}

impl<T> Default for FixedStack<T> {
    fn default() -> Self {
        Self::new(1024).unwrap()
    }
}

// ==================== ThreadSafeStack Implementation ====================

impl<T> ThreadSafeStack<T> {
    /// Create a new thread-safe stack
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new thread-safe stack with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Push an element onto the stack
    pub fn push(&self, item: T) -> CollectionsResult<()> {
        let mut guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        guard.push(item);
        Ok(())
    /// Pop an element from the stack
    pub fn pop(&self) -> CollectionsResult<Option<T>> {
        let mut guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        Ok(guard.pop())
    /// Peek at the top element without removing it
    pub fn peek<F, R>(&self, f: F) -> CollectionsResult<Option<R>>
    where
    {
        let guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        Ok(guard.last().map(f))
    /// Get the number of elements in the stack
    pub fn len(&self) -> CollectionsResult<usize> {
        let guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        Ok(guard.len())
    /// Check if the stack is empty
    pub fn is_empty(&self) -> CollectionsResult<bool> {
        let guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        Ok(guard.is_empty())
    /// Clear all elements from the stack
    pub fn clear(&self) -> CollectionsResult<()> {
        let mut guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        guard.clear();
        Ok(())
    /// Push multiple elements onto the stack
    pub fn push_many<I>(&self, items: I) -> CollectionsResult<()>
    where
    {
        let mut guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        guard.extend(items);
        Ok(())
    /// Pop multiple elements from the stack
    pub fn pop_many(&self, count: usize) -> CollectionsResult<Vec<T>> {
        let mut guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        
        let mut result = Vec::with_capacity(count.min(guard.len()));
        for _ in 0..count {
            if let Some(item) = guard.pop() {
                result.push(item);
            } else {
                break;
            }
        }
        Ok(result)
    /// Get a snapshot of the stack as a vector
    pub fn snapshot(&self) -> CollectionsResult<Vec<T>>
    where
    {
        let guard = self.inner.lock().map_err(|_| {
            CollectionsError::OperationNotSupported {
            }
        })?;
        Ok(guard.clone())
    }
}

impl<T> Default for ThreadSafeStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for ThreadSafeStack<T>
where
{
    fn clone(&self) -> Self {
        if let Ok(snapshot) = self.snapshot() {
            Self {
            }
        } else {
            Self::new()
        }
    }
// ==================== StackWithMin Implementation ====================

impl<T> StackWithMin<T>
where
{
    /// Create a new stack that tracks minimum elements
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new stack with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Push an element onto the stack
    pub fn push(&mut self, item: T) {
        // Push to main stack first
        self.main_stack.push(item.clone());
        
        // Update minimum stack - push if this is new minimum or equal to current minimum
        let should_push_min = self.min_stack.last()
            .map(|min| item <= *min)
            .unwrap_or(true);
        
        if should_push_min {
            self.min_stack.push(item);
        }
    }

    /// Pop an element from the stack
    pub fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.main_stack.pop() {
            // Update minimum stack if necessary
            if let Some(min) = self.min_stack.last() {
                if item == *min {
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
        self.main_stack.last()
    /// Get the minimum element in O(1) time
    pub fn min(&self) -> Option<&T> {
        self.min_stack.last()
    /// Get the number of elements in the stack
    pub fn len(&self) -> usize {
        self.main_stack.len()
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }
    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.main_stack.is_empty()
    /// Clear all elements from the stack
    pub fn clear(&mut self) {
        self.main_stack.clear();
        self.min_stack.clear();
    /// Get the current capacity of the stack
    pub fn capacity(&self) -> usize {
        self.main_stack.capacity()
    /// Reserve capacity for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.main_stack.reserve(additional);
        self.min_stack.reserve(additional);
    /// Push multiple elements onto the stack
    pub fn push_many<I>(&mut self, items: I)
    where
    {
        for item in items {
            let _ = self.push(item);
        }
    }

    /// Pop multiple elements from the stack
    pub fn pop_many(&mut self, count: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(count.min(self.len()));
        for _ in 0..count {
            if let Some(item) = self.pop() {
                result.push(item);
            } else {
                break;
            }
        }
        result
    /// Get an iterator over stack elements (from top to bottom)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.main_stack.iter().rev()
    }
}

impl<T> Default for StackWithMin<T>
where
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for StackWithMin<T>
where
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Self::new();
        stack.push_many(iter);
        stack
    }
}

// ==================== Display Implementations ====================

impl<T: Display> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stack[")?;
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl<T: Display> Display for FixedStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FixedStack[{}/{}](", self.len(), self.capacity())?;
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, ")")
    }
}

impl<T: Display + Clone> Display for ThreadSafeStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(snapshot) = self.snapshot() {
            write!(f, "ThreadSafeStack[")?;
            for (i, item) in snapshot.iter().rev().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        } else {
            write!(f, "ThreadSafeStack[<poisoned>]")
        }
    }
impl<T: Display + Ord + Clone> Display for StackWithMin<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StackWithMin[")?;
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        if let Some(min) = self.min() {
            write!(f, "] (min: {})", min)
        } else {
            write!(f, "] (min: none)")
        }
    }
