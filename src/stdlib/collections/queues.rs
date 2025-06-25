use crate::error::CursedError;
/// Comprehensive Queue implementations for CURSED
/// 
/// This module provides four types of queues:
/// - Queue<T>: FIFO queue with dynamic resizing and O(1) amortized operations
/// - Deque<T>: Double-ended queue with O(1) operations at both ends
/// - PriorityQueue<T>: Binary heap-based priority queue with O(log n) operations
/// - CircularQueue<T>: Fixed-size circular buffer with O(1) operations

use super::{CollectionsError, CollectionsResult};
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::{Ord, Ordering, Reverse};
use std::fmt::{Debug, Display};
use std::iter::{Iterator, FromIterator};
use std::mem;
use std::sync::{Arc, Mutex};

/// FIFO queue implementation with dynamic resizing
#[derive(Debug, Clone)]
pub struct Queue<T> {
/// Double-ended queue implementation for O(1) operations at both ends
#[derive(Debug, Clone)]
pub struct Deque<T> {
/// Priority queue implementation using binary heap
#[derive(Debug, Clone)]
pub struct PriorityQueue<T> {
/// Internal wrapper for heap items to handle min/max heap behavior
#[derive(Debug, Clone, PartialEq, Eq)]
enum PriorityQueueItem<T> {
impl<T: Ord> Ord for PriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
        }
    }
impl<T: Ord> PartialOrd for PriorityQueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Fixed-size circular buffer queue
#[derive(Debug, Clone)]
pub struct CircularQueue<T> {
/// Thread-safe queue wrapper
#[derive(Debug, Clone)]
pub struct ThreadSafeQueue<T> {
/// Thread-safe deque wrapper
#[derive(Debug, Clone)]
pub struct ThreadSafeDeque<T> {
// ==================== Queue Implementation ====================

impl<T> Queue<T>
where
{
    /// Create a new empty Queue
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new Queue with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Add an element to the back of the queue (enqueue)
    pub fn enqueue(&mut self, item: T) -> CollectionsResult<()> {
        self.inner.push_back(item);
        Ok(())
    /// Remove and return the front element (dequeue)
    pub fn dequeue(&mut self) -> Option<T> {
        self.inner.pop_front()
    /// Peek at the front element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.inner.front()
    /// Peek at the back element without removing it
    pub fn peek_back(&self) -> Option<&T> {
        self.inner.back()
    /// Get the number of elements in the queue
    pub fn len(&self) -> usize {
        self.inner.len()
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    /// Clear all elements from the queue
    pub fn clear(&mut self) {
        self.inner.clear()
    /// Get the capacity of the queue
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    /// Reserve space for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    /// Shrink the capacity to fit the current size
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    /// Convert to vector (preserves order)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().cloned().collect()
    /// Create iterator over elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    /// Drain elements with a predicate
    pub fn drain_filter<P>(&mut self, mut predicate: P) -> Vec<T>
    where
    {
        let mut drained = Vec::new();
        let mut temp_queue = Queue::new();

        while let Some(item) = self.dequeue() {
            if predicate(&item) {
                drained.push(item);
            } else {
                let _ = temp_queue.enqueue(item);
            }
        }

        // Restore remaining elements
        while let Some(item) = temp_queue.dequeue() {
            let _ = self.enqueue(item);
        drained
    /// Bulk enqueue from iterator
    pub fn enqueue_all<I>(&mut self, items: I)
    where
    {
        for item in items {
            let _ = self.enqueue(item);
        }
    }

    /// Bulk dequeue up to n elements
    pub fn dequeue_many(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n.min(self.len()));
        for _ in 0..n {
            if let Some(item) = self.dequeue() {
                result.push(item);
            } else {
                break;
            }
        }
        result
    /// Peek at multiple elements from front
    pub fn peek_many(&self, n: usize) -> Vec<&T> {
        self.inner.iter().take(n).collect()
    }
}

impl<T> Default for Queue<T>
where
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for Queue<T>
where
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
        }
    }
// ==================== Deque Implementation ====================

impl<T> Deque<T>
where
{
    /// Create a new empty Deque
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new Deque with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Add an element to the front of the deque
    pub fn push_front(&mut self, item: T) -> CollectionsResult<()> {
        self.inner.push_front(item);
        Ok(())
    /// Add an element to the back of the deque
    pub fn push_back(&mut self, item: T) -> CollectionsResult<()> {
        self.inner.push_back(item);
        Ok(())
    /// Remove and return the front element
    pub fn pop_front(&mut self) -> Option<T> {
        self.inner.pop_front()
    /// Remove and return the back element
    pub fn pop_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    /// Peek at the front element without removing it
    pub fn front(&self) -> Option<&T> {
        self.inner.front()
    /// Peek at the back element without removing it
    pub fn back(&self) -> Option<&T> {
        self.inner.back()
    /// Get element at index
    pub fn get(&self, index: usize) -> CollectionsResult<&T> {
        self.inner.get(index).ok_or(CollectionsError::IndexOutOfBounds {
        })
    /// Get mutable element at index
    pub fn get_mut(&mut self, index: usize) -> CollectionsResult<&mut T> {
        let len = self.len();
        self.inner.get_mut(index).ok_or(CollectionsError::IndexOutOfBounds {
        })
    /// Insert element at index
    pub fn insert(&mut self, index: usize, item: T) -> CollectionsResult<()> {
        if index > self.len() {
            return Err(CollectionsError::IndexOutOfBounds {
            });
        }
        self.inner.insert(index, item);
        Ok(())
    /// Remove element at index
    pub fn remove(&mut self, index: usize) -> CollectionsResult<T> {
        if index >= self.len() {
            return Err(CollectionsError::IndexOutOfBounds {
            });
        }
        Ok(self.inner.remove(index).unwrap())
    /// Get the number of elements in the deque
    pub fn len(&self) -> usize {
        self.inner.len()
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    /// Check if the deque is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    /// Clear all elements from the deque
    pub fn clear(&mut self) {
        self.inner.clear()
    /// Get the capacity of the deque
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    /// Reserve space for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    /// Shrink the capacity to fit the current size
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    /// Convert to vector (preserves order)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().cloned().collect()
    /// Create iterator over elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    /// Rotate left by n positions
    pub fn rotate_left(&mut self, n: usize) {
        if !self.is_empty() {
            let len = self.len();
            let n = n % len;
            for _ in 0..n {
                if let Some(item) = self.pop_front() {
                    let _ = self.push_back(item);
                }
            }
        }
    }

    /// Rotate right by n positions
    pub fn rotate_right(&mut self, n: usize) {
        if !self.is_empty() {
            let len = self.len();
            let n = n % len;
            for _ in 0..n {
                if let Some(item) = self.pop_back() {
                    let _ = self.push_front(item);
                }
            }
        }
    }

    /// Swap elements at two indices
    pub fn swap(&mut self, a: usize, b: usize) -> CollectionsResult<()> {
        let len = self.len();
        if a >= len {
            return Err(CollectionsError::IndexOutOfBounds { index: a, size: len });
        }
        if b >= len {
            return Err(CollectionsError::IndexOutOfBounds { index: b, size: len });
        }
        self.inner.swap(a, b);
        Ok(())
    }
}

impl<T> Default for Deque<T>
where
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for Deque<T>
where
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
        }
    }
// ==================== PriorityQueue Implementation ====================

impl<T> PriorityQueue<T>
where
{
    /// Create a new empty max heap priority queue
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new empty min heap priority queue
    pub fn new_min() -> Self {
        Self {
        }
    }

    /// Create a new priority queue with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Create a new min heap priority queue with specified capacity
    pub fn with_capacity_min(capacity: usize) -> Self {
        Self {
        }
    }

    /// Add an element to the priority queue
    pub fn push(&mut self, item: T) -> CollectionsResult<()> {
        if self.is_min_heap {
            self.inner.push(PriorityQueueItem::Min(Reverse(item)));
        } else {
            self.inner.push(PriorityQueueItem::Max(item));
        }
        Ok(())
    /// Add an element with priority to the priority queue (alias for push for queue-like interface)
    pub fn enqueue(&mut self, priority: T, _item: T) -> CollectionsResult<()> {
        // For now, just use the priority as the item since PriorityQueue<T> doesn't support separate priority/item
        self.push(priority)
    /// Remove and return the highest priority element (alias for pop for queue-like interface)
    pub fn dequeue(&mut self) -> CollectionsResult<Option<T>> {
        Ok(self.pop())
    /// Remove and return the highest priority element
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop().map(|item| match item {
        })
    /// Peek at the highest priority element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.inner.peek().map(|item| match item {
        })
    /// Get the number of elements in the priority queue
    pub fn len(&self) -> usize {
        self.inner.len()
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    /// Check if the priority queue is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    /// Clear all elements from the priority queue
    pub fn clear(&mut self) {
        self.inner.clear()
    /// Get the capacity of the priority queue
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    /// Reserve space for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    /// Shrink the capacity to fit the current size
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    /// Check if this is a min heap
    pub fn is_min_heap(&self) -> bool {
        self.is_min_heap
    /// Convert to sorted vector (heap sort)
    pub fn to_sorted_vec(&self) -> Vec<T> {
        let mut clone = self.clone();
        let mut result = Vec::with_capacity(clone.len());
        while let Some(item) = clone.pop() {
            result.push(item);
        }
        result
    /// Convert to vector (no particular order)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().map(|item| match item {
        }).collect()
    /// Create iterator over elements (no particular order)
    pub fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.inner.iter().map(|item| match item {
        }))
    /// Drain elements with a predicate
    pub fn drain_filter<P>(&mut self, mut predicate: P) -> Vec<T>
    where
    {
        let mut drained = Vec::new();
        let mut temp_queue = PriorityQueue::new();
        temp_queue.is_min_heap = self.is_min_heap;

        while let Some(item) = self.pop() {
            if predicate(&item) {
                drained.push(item);
            } else {
                let _ = temp_queue.push(item);
            }
        }

        // Restore remaining elements
        while let Some(item) = temp_queue.pop() {
            let _ = self.push(item);
        drained
    /// Bulk push from iterator
    pub fn push_all<I>(&mut self, items: I)
    where
    {
        for item in items {
            let _ = self.push(item);
        }
    }

    /// Pop up to n elements
    pub fn pop_many(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n.min(self.len()));
        for _ in 0..n {
            if let Some(item) = self.pop() {
                result.push(item);
            } else {
                break;
            }
        }
        result
    }
}

impl<T> Default for PriorityQueue<T>
where
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for PriorityQueue<T>
where
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut pq = Self::new();
        pq.push_all(iter);
        pq
    }
}

// ==================== CircularQueue Implementation ====================

impl<T> CircularQueue<T>
where
{
    /// Create a new circular queue with specified capacity
    pub fn new(capacity: usize) -> CollectionsResult<Self> {
        if capacity == 0 {
            return Err(CollectionsError::InvalidCapacity { capacity });
        Ok(Self {
        })
    /// Add an element to the back of the queue
    pub fn enqueue(&mut self, item: T) -> CollectionsResult<()> {
        if self.is_full() {
            return Err(CollectionsError::InsufficientMemory {
            });
        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        Ok(())
    /// Remove and return the front element
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        item
    /// Peek at the front element without removing it
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.buffer[self.head].as_ref()
        }
    }

    /// Peek at the back element without removing it
    pub fn peek_back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            let back_index = if self.tail == 0 { self.capacity - 1 } else { self.tail - 1 };
            self.buffer[back_index].as_ref()
        }
    }

    /// Get the number of elements in the queue
    pub fn len(&self) -> usize {
        self.size
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    /// Check if the queue is full
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    /// Get the capacity of the queue
    pub fn capacity(&self) -> usize {
        self.capacity
    /// Clear all elements from the queue
    pub fn clear(&mut self) {
        for item in &mut self.buffer {
            *item = None;
        }
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    /// Convert to vector (preserves order)
    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size);
        let mut index = self.head;
        for _ in 0..self.size {
            if let Some(ref item) = self.buffer[index] {
                result.push(item.clone());
            }
            index = (index + 1) % self.capacity;
        }
        result
    /// Create iterator over elements
    pub fn iter(&self) -> CircularQueueIterator<T> {
        CircularQueueIterator {
        }
    }

    /// Force enqueue (overwrites oldest if full)
    pub fn force_enqueue(&mut self, item: T) -> Option<T> {
        if self.is_full() {
            let old_item = self.dequeue();
            self.enqueue(item).expect("Should succeed after dequeue");
            old_item
        } else {
            self.enqueue(item).expect("Should succeed when not full");
            None
        }
    }

    /// Get element at logical index (0 = front)
    pub fn get(&self, index: usize) -> CollectionsResult<&T> {
        if index >= self.size {
            return Err(CollectionsError::IndexOutOfBounds {
            });
        let actual_index = (self.head + index) % self.capacity;
        self.buffer[actual_index].as_ref().ok_or(CollectionsError::ElementNotFound {
        })
    }
}

/// Iterator for CircularQueue
pub struct CircularQueueIterator<'a, T> {
impl<'a, T> Iterator for CircularQueueIterator<'a, T>
where
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        let item = self.queue.buffer[self.current].as_ref();
        self.current = (self.current + 1) % self.queue.capacity;
        self.remaining -= 1;
        item
    }
}

// ==================== Thread-Safe Implementations ====================

impl<T> ThreadSafeQueue<T>
where
{
    /// Create a new thread-safe queue
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Thread-safe enqueue
    pub fn enqueue(&self, item: T) -> CollectionsResult<()> {
        let mut queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        let _ = queue.enqueue(item);
        Ok(())
    /// Thread-safe dequeue
    pub fn dequeue(&self) -> CollectionsResult<Option<T>> {
        let mut queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        Ok(queue.dequeue())
    /// Thread-safe peek
    pub fn peek<R>(&self, f: impl FnOnce(Option<&T>) -> R) -> CollectionsResult<R> {
        let queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        Ok(f(queue.peek()))
    /// Thread-safe len
    pub fn len(&self) -> CollectionsResult<usize> {
        let queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        Ok(queue.len())
    /// Thread-safe is_empty
    pub fn is_empty(&self) -> CollectionsResult<bool> {
        let queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        Ok(queue.is_empty())
    }
}

impl<T> ThreadSafeDeque<T>
where
{
    /// Create a new thread-safe deque
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Thread-safe push_front
    pub fn push_front(&self, item: T) -> CollectionsResult<()> {
        let mut deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        let _ = deque.push_front(item);
        Ok(())
    /// Thread-safe push_back
    pub fn push_back(&self, item: T) -> CollectionsResult<()> {
        let mut deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        let _ = deque.push_back(item);
        Ok(())
    /// Thread-safe pop_front
    pub fn pop_front(&self) -> CollectionsResult<Option<T>> {
        let mut deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        Ok(deque.pop_front())
    /// Thread-safe pop_back
    pub fn pop_back(&self) -> CollectionsResult<Option<T>> {
        let mut deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        Ok(deque.pop_back())
    /// Thread-safe len
    pub fn len(&self) -> CollectionsResult<usize> {
        let deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
        })?;
        Ok(deque.len())
    }
}

// ==================== Convenience Functions ====================

/// Create a new Queue from a vector
pub fn queue_from_vec<T>(vec: Vec<T>) -> Queue<T>
where
{
    Queue::from_iter(vec)
/// Create a new Deque from a vector
pub fn deque_from_vec<T>(vec: Vec<T>) -> Deque<T>
where
{
    Deque::from_iter(vec)
/// Create a new PriorityQueue from a vector
pub fn priority_queue_from_vec<T>(vec: Vec<T>) -> PriorityQueue<T>
where
{
    PriorityQueue::from_iter(vec)
/// Create a new min heap PriorityQueue from a vector
pub fn min_priority_queue_from_vec<T>(vec: Vec<T>) -> PriorityQueue<T>
where
{
    let mut pq = PriorityQueue::new_min();
    pq.push_all(vec);
    pq
/// Create a new CircularQueue from a vector
pub fn circular_queue_from_vec<T>(vec: Vec<T>) -> CollectionsResult<CircularQueue<T>>
where
{
    let capacity = vec.len().max(1);
    let mut cq = CircularQueue::new(capacity)?;
    for item in vec {
        cq.enqueue(item)?;
    }
    Ok(cq)
