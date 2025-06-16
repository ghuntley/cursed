use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::ptr;
use crate::error::Result as CursedResult;
use super::{atomic_error, MemoryOrder};

/// Node for the atomic queue
struct QueueNode<T> {
    data: Option<T>,
    next: AtomicPtr<QueueNode<T>>,
}

impl<T> QueueNode<T> {
    fn new(data: T) -> Box<Self> {
        Box::new(Self {
            data: Some(data),
            next: AtomicPtr::new(ptr::null_mut()),
        })
    }

    fn empty() -> Box<Self> {
        Box::new(Self {
            data: None,
            next: AtomicPtr::new(ptr::null_mut()),
        })
    }
}

/// Lock-free atomic queue implementation
/// Thread-safe queue supporting concurrent push and pop operations
#[derive(Debug)]
pub struct Queue<T> {
    head: AtomicPtr<QueueNode<T>>,
    tail: AtomicPtr<QueueNode<T>>,
    size: AtomicUsize,
}

impl<T> Queue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        let dummy = Box::into_raw(QueueNode::empty());
        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
            size: AtomicUsize::new(0),
        }
    }

    /// Push an item to the tail of the queue
    pub fn push(&self, item: T) {
        let new_node = Box::into_raw(QueueNode::new(item));
        
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };
            
            if tail == self.tail.load(Ordering::Acquire) {
                if next.is_null() {
                    // Try to link the new node at the end of the list
                    if unsafe { (*tail).next.compare_exchange_weak(
                        ptr::null_mut(),
                        new_node,
                        Ordering::Release,
                        Ordering::Relaxed
                    ).is_ok() } {
                        // Successfully linked new node, now swing tail to new node
                        let _ = self.tail.compare_exchange_weak(
                            tail,
                            new_node,
                            Ordering::Release,
                            Ordering::Relaxed
                        );
                        break;
                    }
                } else {
                    // Tail was lagging, try to advance it
                    let _ = self.tail.compare_exchange_weak(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed
                    );
                }
            }
        }
        
        self.size.fetch_add(1, Ordering::Relaxed);
    }

    /// Pop an item from the head of the queue
    /// Returns None if the queue is empty
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };
            
            if head == self.head.load(Ordering::Acquire) {
                if head == tail {
                    if next.is_null() {
                        // Queue is empty
                        return None;
                    }
                    // Tail is lagging, advance it
                    let _ = self.tail.compare_exchange_weak(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed
                    );
                } else {
                    if next.is_null() {
                        continue;
                    }
                    
                    // Read data before CAS to avoid data race
                    let data = unsafe { (*next).data.take() };
                    
                    // Try to swing head to next node
                    if self.head.compare_exchange_weak(
                        head,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed
                    ).is_ok() {
                        // Successfully dequeued
                        unsafe { Box::from_raw(head) }; // Deallocate old head
                        self.size.fetch_sub(1, Ordering::Relaxed);
                        return data;
                    }
                }
            }
        }
    }

    /// Get the current size of the queue
    /// Note: This is an approximation in concurrent scenarios
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    /// Check if the queue is empty
    /// Note: This is an approximation in concurrent scenarios
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Peek at the front item without removing it
    /// Returns None if the queue is empty
    pub fn peek(&self) -> Option<T> 
    where 
        T: Clone,
    {
        let head = self.head.load(Ordering::Acquire);
        let next = unsafe { (*head).next.load(Ordering::Acquire) };
        
        if next.is_null() {
            None
        } else {
            unsafe { (*next).data.as_ref().cloned() }
        }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        // Clean up all remaining nodes
        while self.pop().is_some() {}
        
        // Clean up the dummy head node
        let head = self.head.load(Ordering::Relaxed);
        if !head.is_null() {
            unsafe { Box::from_raw(head) };
        }
    }
}

unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Send> Sync for Queue<T> {}

/// Atomic stack implementation
/// Thread-safe stack supporting concurrent push and pop operations
#[derive(Debug)]
pub struct Stack<T> {
    head: AtomicPtr<StackNode<T>>,
    size: AtomicUsize,
}

struct StackNode<T> {
    data: T,
    next: *mut StackNode<T>,
}

impl<T> Stack<T> {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            size: AtomicUsize::new(0),
        }
    }

    /// Push an item onto the stack
    pub fn push(&self, item: T) {
        let new_node = Box::into_raw(Box::new(StackNode {
            data: item,
            next: ptr::null_mut(),
        }));
        
        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe { (*new_node).next = head };
            
            if self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed
            ).is_ok() {
                break;
            }
        }
        
        self.size.fetch_add(1, Ordering::Relaxed);
    }

    /// Pop an item from the stack
    /// Returns None if the stack is empty
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }
            
            let next = unsafe { (*head).next };
            
            if self.head.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed
            ).is_ok() {
                let node = unsafe { Box::from_raw(head) };
                self.size.fetch_sub(1, Ordering::Relaxed);
                return Some(node.data);
            }
        }
    }

    /// Get the current size of the stack
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed).is_null()
    }

    /// Peek at the top item without removing it
    pub fn peek(&self) -> Option<&T> {
        let head = self.head.load(Ordering::Acquire);
        if head.is_null() {
            None
        } else {
            Some(unsafe { &(*head).data })
        }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

unsafe impl<T: Send> Send for Stack<T> {}
unsafe impl<T: Send> Sync for Stack<T> {}

/// Thread-safe atomic counter with additional operations
#[derive(Debug)]
pub struct Counter {
    value: AtomicUsize,
    min_value: AtomicUsize,
    max_value: AtomicUsize,
}

impl Counter {
    /// Create a new counter with initial value
    pub fn new(initial: usize) -> Self {
        Self {
            value: AtomicUsize::new(initial),
            min_value: AtomicUsize::new(initial),
            max_value: AtomicUsize::new(initial),
        }
    }

    /// Get the current value
    pub fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }

    /// Increment the counter and return the new value
    pub fn increment(&self) -> usize {
        let new_value = self.value.fetch_add(1, Ordering::SeqCst) + 1;
        self.update_max(new_value);
        new_value
    }

    /// Decrement the counter and return the new value
    pub fn decrement(&self) -> usize {
        let new_value = self.value.fetch_sub(1, Ordering::SeqCst).saturating_sub(1);
        self.update_min(new_value);
        new_value
    }

    /// Add a value and return the new total
    pub fn add(&self, value: usize) -> usize {
        let new_value = self.value.fetch_add(value, Ordering::SeqCst) + value;
        self.update_max(new_value);
        new_value
    }

    /// Subtract a value and return the new total
    pub fn subtract(&self, value: usize) -> usize {
        let new_value = self.value.fetch_sub(value, Ordering::SeqCst).saturating_sub(value);
        self.update_min(new_value);
        new_value
    }

    /// Set the counter to a specific value
    pub fn set(&self, value: usize) {
        self.value.store(value, Ordering::SeqCst);
        self.update_min(value);
        self.update_max(value);
    }

    /// Reset the counter to 0
    pub fn reset(&self) {
        self.set(0);
    }

    /// Get the minimum value seen
    pub fn min(&self) -> usize {
        self.min_value.load(Ordering::Relaxed)
    }

    /// Get the maximum value seen
    pub fn max(&self) -> usize {
        self.max_value.load(Ordering::Relaxed)
    }

    /// Compare and swap the value
    pub fn compare_and_swap(&self, expected: usize, new: usize) -> Result<usize, usize> {
        match self.value.compare_exchange(expected, new, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(old) => {
                self.update_min(new);
                self.update_max(new);
                Ok(old)
            }
            Err(actual) => Err(actual),
        }
    }

    fn update_min(&self, value: usize) {
        loop {
            let current_min = self.min_value.load(Ordering::Relaxed);
            if value >= current_min {
                break;
            }
            if self.min_value.compare_exchange_weak(
                current_min,
                value,
                Ordering::Relaxed,
                Ordering::Relaxed
            ).is_ok() {
                break;
            }
        }
    }

    fn update_max(&self, value: usize) {
        loop {
            let current_max = self.max_value.load(Ordering::Relaxed);
            if value <= current_max {
                break;
            }
            if self.max_value.compare_exchange_weak(
                current_max,
                value,
                Ordering::Relaxed,
                Ordering::Relaxed
            ).is_ok() {
                break;
            }
        }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Create a new atomic queue
pub fn new_queue<T>() -> Queue<T> {
    Queue::new()
}

/// Create a new atomic stack
pub fn new_stack<T>() -> Stack<T> {
    Stack::new()
}

/// Create a new atomic counter
pub fn new_counter(initial: usize) -> Counter {
    Counter::new(initial)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_queue_basic() {
        let queue = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        
        queue.push(1);
        queue.push(2);
        queue.push(3);
        
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 3);
        
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
        
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_concurrent() {
        let queue = Arc::new(Queue::new());
        let mut handles = vec![];
        
        // Producers
        for i in 0..4 {
            let q = Arc::clone(&queue);
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    q.push(i * 100 + j);
                }
            });
            handles.push(handle);
        }
        
        // Consumers
        let items = Arc::new(Mutex::new(Vec::new()));
        for _ in 0..4 {
            let q = Arc::clone(&queue);
            let items_ref = Arc::clone(&items);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    while let Some(item) = q.pop() {
                        if let Ok(mut items) = items_ref.lock() {
                            items.push(item);
                            break;
                        }
                    }
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let final_items = items.lock().unwrap();
        assert_eq!(final_items.len(), 400);
    }

    #[test]
    fn test_stack_basic() {
        let stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 3);
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_concurrent() {
        let stack = Arc::new(Stack::new());
        let mut handles = vec![];
        
        // Push items from multiple threads
        for i in 0..4 {
            let s = Arc::clone(&stack);
            let handle = thread::spawn(move || {
                for j in 0..50 {
                    s.push(i * 50 + j);
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(stack.len(), 200);
        
        // Pop all items
        let mut count = 0;
        while stack.pop().is_some() {
            count += 1;
        }
        assert_eq!(count, 200);
    }

    #[test]
    fn test_counter_basic() {
        let counter = Counter::new(10);
        assert_eq!(counter.get(), 10);
        assert_eq!(counter.min(), 10);
        assert_eq!(counter.max(), 10);
        
        assert_eq!(counter.increment(), 11);
        assert_eq!(counter.min(), 10);
        assert_eq!(counter.max(), 11);
        
        assert_eq!(counter.add(5), 16);
        assert_eq!(counter.max(), 16);
        
        assert_eq!(counter.decrement(), 15);
        assert_eq!(counter.subtract(3), 12);
        assert_eq!(counter.min(), 10);
        assert_eq!(counter.max(), 16);
    }

    #[test]
    fn test_counter_concurrent() {
        let counter = Arc::new(Counter::new(0));
        let mut handles = vec![];
        
        // Multiple threads incrementing
        for _ in 0..8 {
            let c = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    c.increment();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.get(), 800);
        assert_eq!(counter.max(), 800);
    }

    #[test]
    fn test_counter_compare_and_swap() {
        let counter = Counter::new(5);
        
        // Successful CAS
        assert_eq!(counter.compare_and_swap(5, 10), Ok(5));
        assert_eq!(counter.get(), 10);
        
        // Failed CAS
        assert_eq!(counter.compare_and_swap(5, 15), Err(10));
        assert_eq!(counter.get(), 10);
    }

    #[test]
    fn test_queue_peek() {
        let queue = Queue::new();
        queue.push(42);
        queue.push(24);
        
        assert_eq!(queue.peek(), Some(42));
        assert_eq!(queue.peek(), Some(42)); // Should not consume
        
        assert_eq!(queue.pop(), Some(42));
        assert_eq!(queue.peek(), Some(24));
    }

    #[test]
    fn test_stack_peek() {
        let stack = Stack::new();
        stack.push(42);
        stack.push(24);
        
        assert_eq!(stack.peek(), Some(&24));
        assert_eq!(stack.peek(), Some(&24)); // Should not consume
        
        assert_eq!(stack.pop(), Some(24));
        assert_eq!(stack.peek(), Some(&42));
    }
}
