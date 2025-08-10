//! Queue data structures for CURSED collections
//!
//! This module provides comprehensive queue implementations:
//! - Queue: Basic FIFO queue with enqueue/dequeue operations
//! - Deque: Double-ended queue supporting operations at both ends
//! - PriorityQueue: Queue that maintains elements in priority order
//! - CircularQueue: Fixed-size queue with circular buffer implementation

use crate::stdlib::collections::{CollectionsResult, CollectionsError};
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Ordering;

/// Basic queue data structure with FIFO (First-In-First-Out) operations
#[derive(Debug, Clone)]
pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    /// Create a new queue with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
        }
    }

    /// Add an element to the back of the queue
    pub fn enqueue(&mut self, item: T) {
        self.data.push_back(item);
    }

    /// Remove and return the front element from the queue
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Peek at the front element without removing it
    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Peek at the back element without removing it
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Get the number of elements in the queue
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear all elements from the queue
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the current capacity of the queue
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Double-ended queue supporting operations at both ends
#[derive(Debug, Clone)]
pub struct Deque<T> {
    data: VecDeque<T>,
}

impl<T> Deque<T> {
    /// Create a new empty deque
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    /// Create a new deque with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
        }
    }

    /// Add an element to the front of the deque
    pub fn push_front(&mut self, item: T) {
        self.data.push_front(item);
    }

    /// Add an element to the back of the deque
    pub fn push_back(&mut self, item: T) {
        self.data.push_back(item);
    }

    /// Remove and return the front element from the deque
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Remove and return the back element from the deque
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Peek at the front element without removing it
    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Peek at the back element without removing it
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Get the number of elements in the deque
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the deque is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear all elements from the deque
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get element at index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Insert element at index
    pub fn insert(&mut self, index: usize, item: T) -> CollectionsResult<()> {
        if index > self.data.len() {
            return Err(CollectionsError::IndexOutOfBounds { 
                index, 
                size: self.data.len() 
            });
        }
        self.data.insert(index, item);
        Ok(())
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Element with priority for priority queue
#[derive(Debug, Clone)]
pub struct PriorityItem<T> {
    pub item: T,
    pub priority: i32,
}

impl<T> PriorityItem<T> {
    /// Create a new priority item
    pub fn new(item: T, priority: i32) -> Self {
        Self { item, priority }
    }
}

impl<T> PartialEq for PriorityItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for PriorityItem<T> {}

impl<T> Ord for PriorityItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<T> PartialOrd for PriorityItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Priority queue that maintains elements in priority order (higher priority first)
#[derive(Debug, Clone)]
pub struct PriorityQueue<T> {
    heap: BinaryHeap<PriorityItem<T>>,
}

impl<T> PriorityQueue<T> {
    /// Create a new empty priority queue
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    /// Create a new priority queue with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    /// Add an element with priority to the queue
    pub fn enqueue(&mut self, item: T, priority: i32) {
        self.heap.push(PriorityItem::new(item, priority));
    }

    /// Remove and return the highest priority element
    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.item)
    }

    /// Peek at the highest priority element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|item| &item.item)
    }

    /// Get the number of elements in the queue
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Clear all elements from the queue
    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Circular queue with fixed capacity
#[derive(Debug, Clone)]
pub struct CircularQueue<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> CircularQueue<T> {
    /// Create a new circular queue with specified capacity
    pub fn new(capacity: usize) -> CollectionsResult<Self> {
        if capacity == 0 {
            return Err(CollectionsError::InvalidCapacity { capacity });
        }
        
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);
        
        Ok(Self {
            data,
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        })
    }

    /// Add an element to the queue (fails if full)
    pub fn enqueue(&mut self, item: T) -> CollectionsResult<()> {
        if self.is_full() {
            return Err(CollectionsError::InvalidOperation {
                operation: "enqueue".to_string(),
                reason: "Queue at maximum capacity".to_string(),
            });
        }
        
        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    /// Remove and return the front element from the queue
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        let item = self.data[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        item
    }

    /// Peek at the front element without removing it
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.head].as_ref()
        }
    }

    /// Get the number of elements in the queue
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Check if the queue is full
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    /// Get the maximum capacity of the queue
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear all elements from the queue
    pub fn clear(&mut self) {
        for item in &mut self.data {
            *item = None;
        }
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }
}

/// Initialize queue processing
pub fn init_queues() -> CollectionsResult<()> {
    let mut queue = Queue::new();
    queue.enqueue("test");
    let result = queue.dequeue();
    if result != Some("test") {
        return Err(CollectionsError::InvalidOperation {
            operation: "init_test".to_string(),
            reason: "Queue test failed".to_string(),
        });
    }
    println!("🚪 Queue processing initialized");
    Ok(())
}

/// Test queue functionality
pub fn test_queues() -> CollectionsResult<()> {
    // Test basic queue
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.front(), Some(&1));
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.len(), 1);
    
    // Test deque
    let mut deque = Deque::new();
    deque.push_back(1);
    deque.push_front(0);
    deque.push_back(2);
    
    assert_eq!(deque.len(), 3);
    assert_eq!(deque.pop_front(), Some(0));
    assert_eq!(deque.pop_back(), Some(2));
    assert_eq!(deque.pop_front(), Some(1));
    
    // Test priority queue
    let mut pq = PriorityQueue::new();
    pq.enqueue("low", 1);
    pq.enqueue("high", 10);
    pq.enqueue("medium", 5);
    
    assert_eq!(pq.dequeue(), Some("high"));
    assert_eq!(pq.dequeue(), Some("medium"));
    assert_eq!(pq.dequeue(), Some("low"));
    
    // Test circular queue
    let mut cq = CircularQueue::new(3)?;
    cq.enqueue(1)?;
    cq.enqueue(2)?;
    cq.enqueue(3)?;
    
    assert!(cq.is_full());
    assert!(cq.enqueue(4).is_err()); // Should fail
    
    assert_eq!(cq.dequeue(), Some(1));
    cq.enqueue(4)?; // Should work now
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_queue_operations() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.front(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_deque_operations() {
        let mut deque = Deque::new();
        
        deque.push_back(2);
        deque.push_front(1);
        deque.push_back(3);
        
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&3));
        
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert!(deque.is_empty());
    }

    #[test]
    fn test_priority_queue() {
        let mut pq = PriorityQueue::new();
        
        pq.enqueue("low", 1);
        pq.enqueue("high", 10);
        pq.enqueue("medium", 5);
        
        assert_eq!(pq.len(), 3);
        assert_eq!(pq.dequeue(), Some("high"));
        assert_eq!(pq.dequeue(), Some("medium"));
        assert_eq!(pq.dequeue(), Some("low"));
        assert!(pq.is_empty());
    }

    #[test]
    fn test_circular_queue() {
        let mut cq = CircularQueue::new(3).unwrap();
        
        assert!(cq.enqueue(1).is_ok());
        assert!(cq.enqueue(2).is_ok());
        assert!(cq.enqueue(3).is_ok());
        assert!(cq.is_full());
        assert!(cq.enqueue(4).is_err());
        
        assert_eq!(cq.dequeue(), Some(1));
        assert!(cq.enqueue(4).is_ok());
        
        assert_eq!(cq.dequeue(), Some(2));
        assert_eq!(cq.dequeue(), Some(3));
        assert_eq!(cq.dequeue(), Some(4));
        assert!(cq.is_empty());
    }

    #[test]
    fn test_circular_queue_wraparound() {
        let mut cq = CircularQueue::new(2).unwrap();
        
        cq.enqueue(1).unwrap();
        cq.enqueue(2).unwrap();
        
        assert_eq!(cq.dequeue(), Some(1));
        cq.enqueue(3).unwrap();
        
        assert_eq!(cq.dequeue(), Some(2));
        assert_eq!(cq.dequeue(), Some(3));
        assert!(cq.is_empty());
    }
}
