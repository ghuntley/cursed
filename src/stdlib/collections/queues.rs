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
    inner: VecDeque<T>,
}

/// Double-ended queue implementation for O(1) operations at both ends
#[derive(Debug, Clone)]
pub struct Deque<T> {
    inner: VecDeque<T>,
}

/// Priority queue implementation using binary heap
#[derive(Debug, Clone)]
pub struct PriorityQueue<T> {
    inner: BinaryHeap<PriorityQueueItem<T>>,
    is_min_heap: bool,
}

/// Internal wrapper for heap items to handle min/max heap behavior
#[derive(Debug, Clone, PartialEq, Eq)]
enum PriorityQueueItem<T> {
    Max(T),
    Min(Reverse<T>),
}

impl<T: Ord> Ord for PriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PriorityQueueItem::Max(a), PriorityQueueItem::Max(b)) => a.cmp(b),
            (PriorityQueueItem::Min(a), PriorityQueueItem::Min(b)) => a.cmp(b),
            _ => panic!("Cannot compare different heap types"),
        }
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
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

/// Thread-safe queue wrapper
#[derive(Debug, Clone)]
pub struct ThreadSafeQueue<T> {
    inner: Arc<Mutex<Queue<T>>>,
}

/// Thread-safe deque wrapper
#[derive(Debug, Clone)]
pub struct ThreadSafeDeque<T> {
    inner: Arc<Mutex<Deque<T>>>,
}

// ==================== Queue Implementation ====================

impl<T> Queue<T>
where
    T: Clone,
{
    /// Create a new empty Queue
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    /// Create a new Queue with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
        }
    }

    /// Add an element to the back of the queue (enqueue)
    pub fn enqueue(&mut self, item: T) -> CollectionsResult<()> {
        self.inner.push_back(item);
        Ok(())
    }

    /// Remove and return the front element (dequeue)
    pub fn dequeue(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    /// Peek at the front element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.inner.front()
    }

    /// Peek at the back element without removing it
    pub fn peek_back(&self) -> Option<&T> {
        self.inner.back()
    }

    /// Get the number of elements in the queue
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear all elements from the queue
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Get the capacity of the queue
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserve space for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Shrink the capacity to fit the current size
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    /// Convert to vector (preserves order)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().cloned().collect()
    }

    /// Create iterator over elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    /// Drain elements with a predicate
    pub fn drain_filter<P>(&mut self, mut predicate: P) -> Vec<T>
    where
        P: FnMut(&T) -> bool,
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
        }

        drained
    }

    /// Bulk enqueue from iterator
    pub fn enqueue_all<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = T>,
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
    }

    /// Peek at multiple elements from front
    pub fn peek_many(&self, n: usize) -> Vec<&T> {
        self.inner.iter().take(n).collect()
    }
}

impl<T> Default for Queue<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for Queue<T>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            inner: VecDeque::from_iter(iter),
        }
    }
}

// ==================== Deque Implementation ====================

impl<T> Deque<T>
where
    T: Clone,
{
    /// Create a new empty Deque
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    /// Create a new Deque with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
        }
    }

    /// Add an element to the front of the deque
    pub fn push_front(&mut self, item: T) -> CollectionsResult<()> {
        self.inner.push_front(item);
        Ok(())
    }

    /// Add an element to the back of the deque
    pub fn push_back(&mut self, item: T) -> CollectionsResult<()> {
        self.inner.push_back(item);
        Ok(())
    }

    /// Remove and return the front element
    pub fn pop_front(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    /// Remove and return the back element
    pub fn pop_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    }

    /// Peek at the front element without removing it
    pub fn front(&self) -> Option<&T> {
        self.inner.front()
    }

    /// Peek at the back element without removing it
    pub fn back(&self) -> Option<&T> {
        self.inner.back()
    }

    /// Get element at index
    pub fn get(&self, index: usize) -> CollectionsResult<&T> {
        self.inner.get(index).ok_or(CollectionsError::IndexOutOfBounds {
            index,
            size: self.len(),
        })
    }

    /// Get mutable element at index
    pub fn get_mut(&mut self, index: usize) -> CollectionsResult<&mut T> {
        let len = self.len();
        self.inner.get_mut(index).ok_or(CollectionsError::IndexOutOfBounds {
            index,
            size: len,
        })
    }

    /// Insert element at index
    pub fn insert(&mut self, index: usize, item: T) -> CollectionsResult<()> {
        if index > self.len() {
            return Err(CollectionsError::IndexOutOfBounds {
                index,
                size: self.len(),
            });
        }
        self.inner.insert(index, item);
        Ok(())
    }

    /// Remove element at index
    pub fn remove(&mut self, index: usize) -> CollectionsResult<T> {
        if index >= self.len() {
            return Err(CollectionsError::IndexOutOfBounds {
                index,
                size: self.len(),
            });
        }
        Ok(self.inner.remove(index).unwrap())
    }

    /// Get the number of elements in the deque
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }

    /// Check if the deque is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear all elements from the deque
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Get the capacity of the deque
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserve space for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Shrink the capacity to fit the current size
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    /// Convert to vector (preserves order)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().cloned().collect()
    }

    /// Create iterator over elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

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
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for Deque<T>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            inner: VecDeque::from_iter(iter),
        }
    }
}

// ==================== PriorityQueue Implementation ====================

impl<T> PriorityQueue<T>
where
    T: Ord + Clone,
{
    /// Create a new empty max heap priority queue
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
            is_min_heap: false,
        }
    }

    /// Create a new empty min heap priority queue
    pub fn new_min() -> Self {
        Self {
            inner: BinaryHeap::new(),
            is_min_heap: true,
        }
    }

    /// Create a new priority queue with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: BinaryHeap::with_capacity(capacity),
            is_min_heap: false,
        }
    }

    /// Create a new min heap priority queue with specified capacity
    pub fn with_capacity_min(capacity: usize) -> Self {
        Self {
            inner: BinaryHeap::with_capacity(capacity),
            is_min_heap: true,
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
    }

    /// Add an element with priority to the priority queue (alias for push for queue-like interface)
    pub fn enqueue(&mut self, priority: T, _item: T) -> CollectionsResult<()> {
        // For now, just use the priority as the item since PriorityQueue<T> doesn't support separate priority/item
        self.push(priority)
    }

    /// Remove and return the highest priority element (alias for pop for queue-like interface)
    pub fn dequeue(&mut self) -> CollectionsResult<Option<T>> {
        Ok(self.pop())
    }

    /// Remove and return the highest priority element
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop().map(|item| match item {
            PriorityQueueItem::Max(val) => val,
            PriorityQueueItem::Min(Reverse(val)) => val,
        })
    }

    /// Peek at the highest priority element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.inner.peek().map(|item| match item {
            PriorityQueueItem::Max(val) => val,
            PriorityQueueItem::Min(Reverse(val)) => val,
        })
    }

    /// Get the number of elements in the priority queue
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }

    /// Check if the priority queue is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear all elements from the priority queue
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Get the capacity of the priority queue
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserve space for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Shrink the capacity to fit the current size
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    /// Check if this is a min heap
    pub fn is_min_heap(&self) -> bool {
        self.is_min_heap
    }

    /// Convert to sorted vector (heap sort)
    pub fn to_sorted_vec(&self) -> Vec<T> {
        let mut clone = self.clone();
        let mut result = Vec::with_capacity(clone.len());
        while let Some(item) = clone.pop() {
            result.push(item);
        }
        result
    }

    /// Convert to vector (no particular order)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().map(|item| match item {
            PriorityQueueItem::Max(val) => val.clone(),
            PriorityQueueItem::Min(Reverse(val)) => val.clone(),
        }).collect()
    }

    /// Create iterator over elements (no particular order)
    pub fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.inner.iter().map(|item| match item {
            PriorityQueueItem::Max(val) => val,
            PriorityQueueItem::Min(Reverse(val)) => val,
        }))
    }

    /// Drain elements with a predicate
    pub fn drain_filter<P>(&mut self, mut predicate: P) -> Vec<T>
    where
        P: FnMut(&T) -> bool,
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
        }

        drained
    }

    /// Bulk push from iterator
    pub fn push_all<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = T>,
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
    T: Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for PriorityQueue<T>
where
    T: Ord + Clone,
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
    T: Clone,
{
    /// Create a new circular queue with specified capacity
    pub fn new(capacity: usize) -> CollectionsResult<Self> {
        if capacity == 0 {
            return Err(CollectionsError::InvalidCapacity { capacity });
        }

        Ok(Self {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        })
    }

    /// Add an element to the back of the queue
    pub fn enqueue(&mut self, item: T) -> CollectionsResult<()> {
        if self.is_full() {
            return Err(CollectionsError::InsufficientMemory {
                requested: self.capacity + 1,
            });
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    /// Remove and return the front element
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        item
    }

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
    }

    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Check if the queue is full
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    /// Get the capacity of the queue
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear all elements from the queue
    pub fn clear(&mut self) {
        for item in &mut self.buffer {
            *item = None;
        }
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }

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
    }

    /// Create iterator over elements
    pub fn iter(&self) -> CircularQueueIterator<T> {
        CircularQueueIterator {
            queue: self,
            current: self.head,
            remaining: self.size,
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
                index,
                size: self.size,
            });
        }

        let actual_index = (self.head + index) % self.capacity;
        self.buffer[actual_index].as_ref().ok_or(CollectionsError::ElementNotFound {
            element: format!("index {}", index),
        })
    }
}

/// Iterator for CircularQueue
pub struct CircularQueueIterator<'a, T> {
    queue: &'a CircularQueue<T>,
    current: usize,
    remaining: usize,
}

impl<'a, T> Iterator for CircularQueueIterator<'a, T>
where
    T: Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let item = self.queue.buffer[self.current].as_ref();
        self.current = (self.current + 1) % self.queue.capacity;
        self.remaining -= 1;
        item
    }
}

// ==================== Thread-Safe Implementations ====================

impl<T> ThreadSafeQueue<T>
where
    T: Clone + Send + Sync,
{
    /// Create a new thread-safe queue
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Queue::new())),
        }
    }

    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Queue::with_capacity(capacity))),
        }
    }

    /// Thread-safe enqueue
    pub fn enqueue(&self, item: T) -> CollectionsResult<()> {
        let mut queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "enqueue".to_string(),
            collection_type: "ThreadSafeQueue".to_string(),
        })?;
        let _ = queue.enqueue(item);
        Ok(())
    }

    /// Thread-safe dequeue
    pub fn dequeue(&self) -> CollectionsResult<Option<T>> {
        let mut queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "dequeue".to_string(),
            collection_type: "ThreadSafeQueue".to_string(),
        })?;
        Ok(queue.dequeue())
    }

    /// Thread-safe peek
    pub fn peek<R>(&self, f: impl FnOnce(Option<&T>) -> R) -> CollectionsResult<R> {
        let queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "peek".to_string(),
            collection_type: "ThreadSafeQueue".to_string(),
        })?;
        Ok(f(queue.peek()))
    }

    /// Thread-safe len
    pub fn len(&self) -> CollectionsResult<usize> {
        let queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "len".to_string(),
            collection_type: "ThreadSafeQueue".to_string(),
        })?;
        Ok(queue.len())
    }

    /// Thread-safe is_empty
    pub fn is_empty(&self) -> CollectionsResult<bool> {
        let queue = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "is_empty".to_string(),
            collection_type: "ThreadSafeQueue".to_string(),
        })?;
        Ok(queue.is_empty())
    }
}

impl<T> ThreadSafeDeque<T>
where
    T: Clone + Send + Sync,
{
    /// Create a new thread-safe deque
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Deque::new())),
        }
    }

    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Deque::with_capacity(capacity))),
        }
    }

    /// Thread-safe push_front
    pub fn push_front(&self, item: T) -> CollectionsResult<()> {
        let mut deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "push_front".to_string(),
            collection_type: "ThreadSafeDeque".to_string(),
        })?;
        let _ = deque.push_front(item);
        Ok(())
    }

    /// Thread-safe push_back
    pub fn push_back(&self, item: T) -> CollectionsResult<()> {
        let mut deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "push_back".to_string(),
            collection_type: "ThreadSafeDeque".to_string(),
        })?;
        let _ = deque.push_back(item);
        Ok(())
    }

    /// Thread-safe pop_front
    pub fn pop_front(&self) -> CollectionsResult<Option<T>> {
        let mut deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "pop_front".to_string(),
            collection_type: "ThreadSafeDeque".to_string(),
        })?;
        Ok(deque.pop_front())
    }

    /// Thread-safe pop_back
    pub fn pop_back(&self) -> CollectionsResult<Option<T>> {
        let mut deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "pop_back".to_string(),
            collection_type: "ThreadSafeDeque".to_string(),
        })?;
        Ok(deque.pop_back())
    }

    /// Thread-safe len
    pub fn len(&self) -> CollectionsResult<usize> {
        let deque = self.inner.lock().map_err(|_| CollectionsError::OperationNotSupported {
            operation: "len".to_string(),
            collection_type: "ThreadSafeDeque".to_string(),
        })?;
        Ok(deque.len())
    }
}

// ==================== Convenience Functions ====================

/// Create a new Queue from a vector
pub fn queue_from_vec<T>(vec: Vec<T>) -> Queue<T>
where
    T: Clone,
{
    Queue::from_iter(vec)
}

/// Create a new Deque from a vector
pub fn deque_from_vec<T>(vec: Vec<T>) -> Deque<T>
where
    T: Clone,
{
    Deque::from_iter(vec)
}

/// Create a new PriorityQueue from a vector
pub fn priority_queue_from_vec<T>(vec: Vec<T>) -> PriorityQueue<T>
where
    T: Ord + Clone,
{
    PriorityQueue::from_iter(vec)
}

/// Create a new min heap PriorityQueue from a vector
pub fn min_priority_queue_from_vec<T>(vec: Vec<T>) -> PriorityQueue<T>
where
    T: Ord + Clone,
{
    let mut pq = PriorityQueue::new_min();
    pq.push_all(vec);
    pq
}

/// Create a new CircularQueue from a vector
pub fn circular_queue_from_vec<T>(vec: Vec<T>) -> CollectionsResult<CircularQueue<T>>
where
    T: Clone,
{
    let capacity = vec.len().max(1);
    let mut cq = CircularQueue::new(capacity)?;
    for item in vec {
        cq.enqueue(item)?;
    }
    Ok(cq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_basic_operations() {
        let mut queue = Queue::new();
        
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        
        queue.enqueue("first".to_string());
        queue.enqueue("second".to_string());
        queue.enqueue("third".to_string());
        
        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(&"first".to_string()));
        
        assert_eq!(queue.dequeue(), Some("first".to_string()));
        assert_eq!(queue.dequeue(), Some("second".to_string()));
        assert_eq!(queue.len(), 1);
        
        assert_eq!(queue.dequeue(), Some("third".to_string()));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_deque_operations() {
        let mut deque = Deque::new();
        
        let _ = deque.push_back(1);
        let _ = deque.push_front(0);
        let _ = deque.push_back(2);
        
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.front(), Some(&0));
        assert_eq!(deque.back(), Some(&2));
        
        assert_eq!(deque.pop_front(), Some(0));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.len(), 1);
        
        assert_eq!(deque.get(0).unwrap(), &1);
        deque.insert(0, 10).unwrap();
        assert_eq!(deque.get(0).unwrap(), &10);
        assert_eq!(deque.get(1).unwrap(), &1);
    }

    #[test]
    fn test_priority_queue_max_heap() {
        let mut pq = PriorityQueue::new();
        
        pq.push(3);
        pq.push(1);
        pq.push(4);
        pq.push(1);
        pq.push(5);
        
        assert_eq!(pq.len(), 5);
        assert_eq!(pq.peek(), Some(&5));
        
        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.pop(), Some(4));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_priority_queue_min_heap() {
        let mut pq = PriorityQueue::new_min();
        
        pq.push(3);
        pq.push(1);
        pq.push(4);
        pq.push(1);
        pq.push(5);
        
        assert_eq!(pq.len(), 5);
        assert_eq!(pq.peek(), Some(&1));
        
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.pop(), Some(4));
        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_circular_queue_operations() {
        let mut cq = CircularQueue::new(3).unwrap();
        
        assert!(cq.is_empty());
        assert!(!cq.is_full());
        assert_eq!(cq.capacity(), 3);
        
        cq.enqueue(1).unwrap();
        cq.enqueue(2).unwrap();
        cq.enqueue(3).unwrap();
        
        assert!(cq.is_full());
        assert!(cq.enqueue(4).is_err()); // Should fail when full
        
        assert_eq!(cq.peek(), Some(&1));
        assert_eq!(cq.dequeue(), Some(1));
        assert_eq!(cq.dequeue(), Some(2));
        
        cq.enqueue(4).unwrap();
        cq.enqueue(5).unwrap();
        
        assert_eq!(cq.to_vec(), vec![3, 4, 5]);
    }

    #[test]
    fn test_circular_queue_force_enqueue() {
        let mut cq = CircularQueue::new(2).unwrap();
        
        cq.enqueue(1).unwrap();
        cq.enqueue(2).unwrap();
        
        // Force enqueue should remove oldest (1) and add 3
        let removed = cq.force_enqueue(3);
        assert_eq!(removed, Some(1));
        assert_eq!(cq.to_vec(), vec![2, 3]);
    }

    #[test]
    fn test_deque_rotation() {
        let mut deque = deque_from_vec(vec![1, 2, 3, 4, 5]);
        
        deque.rotate_left(2);
        assert_eq!(deque.to_vec(), vec![3, 4, 5, 1, 2]);
        
        deque.rotate_right(1);
        assert_eq!(deque.to_vec(), vec![2, 3, 4, 5, 1]);
    }

    #[test]
    fn test_queue_bulk_operations() {
        let mut queue = Queue::new();
        
        queue.enqueue_all(vec![1, 2, 3, 4, 5]);
        assert_eq!(queue.len(), 5);
        
        let first_three = queue.dequeue_many(3);
        assert_eq!(first_three, vec![1, 2, 3]);
        assert_eq!(queue.len(), 2);
        
        let peeked = queue.peek_many(2);
        assert_eq!(peeked, vec![&4, &5]);
    }

    #[test]
    fn test_priority_queue_drain_filter() {
        let mut pq = priority_queue_from_vec(vec![1, 2, 3, 4, 5, 6]);
        
        let evens = pq.drain_filter(|&x| x % 2 == 0);
        assert_eq!(evens.len(), 3); // 2, 4, 6
        assert_eq!(pq.len(), 3); // 1, 3, 5 remaining
        
        let sorted = pq.to_sorted_vec();
        assert_eq!(sorted, vec![5, 3, 1]); // Max heap order
    }

    #[test]
    fn test_convenience_functions() {
        let vec = vec![3, 1, 4, 1, 5];
        
        let queue = queue_from_vec(vec.clone());
        assert_eq!(queue.len(), 5);
        
        let deque = deque_from_vec(vec.clone());
        assert_eq!(deque.len(), 5);
        
        let pq = priority_queue_from_vec(vec.clone());
        assert_eq!(pq.len(), 5);
        assert_eq!(pq.peek(), Some(&5)); // Max element
        
        let min_pq = min_priority_queue_from_vec(vec.clone());
        assert_eq!(min_pq.len(), 5);
        assert_eq!(min_pq.peek(), Some(&1)); // Min element
        
        let cq = circular_queue_from_vec(vec).unwrap();
        assert_eq!(cq.len(), 5);
        assert_eq!(cq.capacity(), 5);
    }

    #[test]
    fn test_error_handling() {
        // CircularQueue capacity error
        assert!(CircularQueue::<i32>::new(0).is_err());
        
        // Index out of bounds
        let mut deque = Deque::<i32>::new();
        assert!(deque.get(0).is_err());
        assert!(deque.remove(0).is_err());
        
        // CircularQueue overflow
        let mut cq = CircularQueue::new(1).unwrap();
        cq.enqueue(1).unwrap();
        assert!(cq.enqueue(2).is_err());
    }

    #[test]
    fn test_circular_queue_iterator() {
        let mut cq = CircularQueue::new(5).unwrap();
        for i in 1..=3 {
            cq.enqueue(i).unwrap();
        }
        
        let collected: Vec<_> = cq.iter().cloned().collect();
        assert_eq!(collected, vec![1, 2, 3]);
        
        // Test with wrap-around
        cq.dequeue();
        cq.enqueue(4).unwrap();
        cq.enqueue(5).unwrap();
        
        let collected: Vec<_> = cq.iter().cloned().collect();
        assert_eq!(collected, vec![2, 3, 4, 5]);
    }
}
