use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::ptr;
use crate::error::Result as CursedResult;
use super::{atomic_error, MemoryOrder};

/// Node for the atomic queue
struct QueueNode<T> {
impl<T> QueueNode<T> {
    fn new(data: T) -> Box<Self> {
        Box::new(Self {
        })
    fn empty() -> Box<Self> {
        Box::new(Self {
        })
    }
}

/// Lock-free atomic queue implementation
/// Thread-safe queue supporting concurrent push and pop operations
#[derive(Debug)]
pub struct Queue<T> {
impl<T> Queue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        let dummy = Box::into_raw(QueueNode::empty());
        Self {
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
                        Ordering::Relaxed
                    ).is_ok() } {
                        // Successfully linked new node, now swing tail to new node
                        let _ = self.tail.compare_exchange_weak(
                            Ordering::Relaxed
                        );
                        break;
                    }
                } else {
                    // Tail was lagging, try to advance it
                    let _ = self.tail.compare_exchange_weak(
                        Ordering::Relaxed
                    );
                }
            }
        self.size.fetch_add(1, Ordering::Relaxed);
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
                        Ordering::Relaxed
                    );
                } else {
                    if next.is_null() {
                        continue;
                    // Read data before CAS to avoid data race
                    let data = unsafe { (*next).data.take() };
                    
                    // Try to swing head to next node
                    if self.head.compare_exchange_weak(
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
    /// Get the current size of the queue
    /// Note: This is an approximation in concurrent scenarios
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    /// Check if the queue is empty
    /// Note: This is an approximation in concurrent scenarios
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    /// Peek at the front item without removing it
    /// Returns None if the queue is empty
    pub fn peek(&self) -> Option<T> 
    where 
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
        // Clean up the dummy head node
        let head = self.head.load(Ordering::Relaxed);
        if !head.is_null() {
            unsafe { Box::from_raw(head) };
        }
    }
unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Send> Sync for Queue<T> {}

/// Atomic stack implementation
/// Thread-safe stack supporting concurrent push and pop operations
#[derive(Debug)]
pub struct Stack<T> {
struct StackNode<T> {
impl<T> Stack<T> {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self {
        }
    }

    /// Push an item onto the stack
    pub fn push(&self, item: T) {
        let new_node = Box::into_raw(Box::new(StackNode {
        }));
        
        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe { (*new_node).next = head };
            
            if self.head.compare_exchange_weak(
                Ordering::Relaxed
            ).is_ok() {
                break;
            }
        }
        
        self.size.fetch_add(1, Ordering::Relaxed);
    /// Pop an item from the stack
    /// Returns None if the stack is empty
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            let next = unsafe { (*head).next };
            
            if self.head.compare_exchange_weak(
                Ordering::Relaxed
            ).is_ok() {
                let node = unsafe { Box::from_raw(head) };
                self.size.fetch_sub(1, Ordering::Relaxed);
                return Some(node.data);
            }
        }
    /// Get the current size of the stack
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed).is_null()
    /// Peek at the top item without removing it
    pub fn peek(&self) -> Option<&T> {
        let head = self.head.load(Ordering::Acquire);
        if head.is_null() {
            None
        } else {
            Some(unsafe { &(*head).data })
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
unsafe impl<T: Send> Send for Stack<T> {}
unsafe impl<T: Send> Sync for Stack<T> {}

/// Thread-safe atomic counter with additional operations
#[derive(Debug)]
pub struct Counter {
impl Counter {
    /// Create a new counter with initial value
    pub fn new(initial: usize) -> Self {
        Self {
        }
    }

    /// Get the current value
    pub fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    /// Increment the counter and return the new value
    pub fn increment(&self) -> usize {
        let new_value = self.value.fetch_add(1, Ordering::SeqCst) + 1;
        self.update_max(new_value);
        new_value
    /// Decrement the counter and return the new value
    pub fn decrement(&self) -> usize {
        let new_value = self.value.fetch_sub(1, Ordering::SeqCst).saturating_sub(1);
        self.update_min(new_value);
        new_value
    /// Add a value and return the new total
    pub fn add(&self, value: usize) -> usize {
        let new_value = self.value.fetch_add(value, Ordering::SeqCst) + value;
        self.update_max(new_value);
        new_value
    /// Subtract a value and return the new total
    pub fn subtract(&self, value: usize) -> usize {
        let new_value = self.value.fetch_sub(value, Ordering::SeqCst).saturating_sub(value);
        self.update_min(new_value);
        new_value
    /// Set the counter to a specific value
    pub fn set(&self, value: usize) {
        self.value.store(value, Ordering::SeqCst);
        self.update_min(value);
        self.update_max(value);
    /// Reset the counter to 0
    pub fn reset(&self) {
        self.set(0);
    /// Get the minimum value seen
    pub fn min(&self) -> usize {
        self.min_value.load(Ordering::Relaxed)
    /// Get the maximum value seen
    pub fn max(&self) -> usize {
        self.max_value.load(Ordering::Relaxed)
    /// Compare and swap the value
    pub fn compare_and_swap(&self, expected: usize, new: usize) -> Result<usize, usize> {
        match self.value.compare_exchange(expected, new, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(old) => {
                self.update_min(new);
                self.update_max(new);
                Ok(old)
            }
        }
    }

    fn update_min(&self, value: usize) {
        loop {
            let current_min = self.min_value.load(Ordering::Relaxed);
            if value >= current_min {
                break;
            }
            if self.min_value.compare_exchange_weak(
                Ordering::Relaxed
            ).is_ok() {
                break;
            }
        }
    fn update_max(&self, value: usize) {
        loop {
            let current_max = self.max_value.load(Ordering::Relaxed);
            if value <= current_max {
                break;
            }
            if self.max_value.compare_exchange_weak(
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
/// Create a new atomic stack
pub fn new_stack<T>() -> Stack<T> {
    Stack::new()
/// Create a new atomic counter
pub fn new_counter(initial: usize) -> Counter {
    Counter::new(initial)
