//! Lock-free Work Stealing Deque Implementation
//!
//! This module provides a lock-free double-ended queue optimized for work stealing
//! between goroutine scheduler threads. It uses atomic operations to ensure
//! thread safety without locks.

use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::ptr::{self, NonNull};
use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;

/// Minimum deque capacity
const MIN_CAPACITY: usize = 32;

/// Node in the lock-free deque
struct Node<T> {
    data: Option<T>,
    next: AtomicPtr<Node<T>>,
    prev: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data: Some(data),
            next: AtomicPtr::new(ptr::null_mut()),
            prev: AtomicPtr::new(ptr::null_mut()),
        }
    }

    fn empty() -> Self {
        Self {
            data: None,
            next: AtomicPtr::new(ptr::null_mut()),
            prev: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

/// Lock-free work stealing deque
pub struct LockFreeDeque<T> {
    /// Head pointer for push/pop operations (owner thread)
    head: AtomicPtr<Node<T>>,
    /// Tail pointer for steal operations (other threads)
    tail: AtomicPtr<Node<T>>,
    /// Current size estimate
    size: AtomicUsize,
    /// Maximum capacity
    capacity: usize,
    /// Phantom data for type safety
    _phantom: PhantomData<T>,
}

unsafe impl<T: Send> Send for LockFreeDeque<T> {}
unsafe impl<T: Send> Sync for LockFreeDeque<T> {}

impl<T> LockFreeDeque<T> {
    /// Create a new lock-free deque with default capacity
    pub fn new() -> Self {
        Self::with_capacity(MIN_CAPACITY)
    }

    /// Create a new lock-free deque with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.max(MIN_CAPACITY);
        
        // Create sentinel node
        let sentinel = Box::into_raw(Box::new(Node::empty()));
        
        Self {
            head: AtomicPtr::new(sentinel),
            tail: AtomicPtr::new(sentinel),
            size: AtomicUsize::new(0),
            capacity,
            _phantom: PhantomData,
        }
    }

    /// Push an item to the head (owner thread only)
    pub fn push(&self, item: T) -> Result<(), T> {
        if self.len() >= self.capacity {
            return Err(item);
        }

        let new_node = Box::into_raw(Box::new(Node::new(item)));
        
        let mut backoff = 1;
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            
            unsafe {
                (*new_node).next.store(current_head, Ordering::Relaxed);
                (*current_head).prev.store(new_node, Ordering::Relaxed);
            }
            
            // Try to update head pointer
            if self.head.compare_exchange_weak(
                current_head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed
            ).is_ok() {
                self.size.fetch_add(1, Ordering::Relaxed);
                return Ok(());
            }
            
            // Reset node pointers on failure
            unsafe {
                (*new_node).next.store(ptr::null_mut(), Ordering::Relaxed);
            }
            
            // Exponential backoff to prevent busy waiting
            if backoff < 64 {
                for _ in 0..backoff {
                    std::hint::spin_loop();
                }
                backoff *= 2;
            } else {
                std::thread::yield_now();
                backoff = 1;
            }
        }
    }

    /// Pop an item from the head (owner thread only)
    pub fn pop(&self) -> Option<T> {
        let mut backoff = 1;
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            
            unsafe {
                let next = (*current_head).next.load(Ordering::Acquire);
                
                // Check if deque is empty
                if next.is_null() {
                    return None;
                }
                
                // Try to update head pointer
                if self.head.compare_exchange_weak(
                    current_head,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed
                ).is_ok() {
                    (*next).prev.store(ptr::null_mut(), Ordering::Relaxed);
                    
                    // Extract data and cleanup
                    let data = (*current_head).data.take();
                    let _ = Box::from_raw(current_head);
                    
                    if data.is_some() {
                        self.size.fetch_sub(1, Ordering::Relaxed);
                    }
                    
                    return data;
                }
            }
            
            // Exponential backoff to prevent busy waiting
            if backoff < 64 {
                for _ in 0..backoff {
                    std::hint::spin_loop();
                }
                backoff *= 2;
            } else {
                std::thread::yield_now();
                backoff = 1;
            }
        }
    }

    /// Steal an item from the tail (other threads)
    pub fn steal(&self) -> Option<T> {
        let mut backoff = 1;
        loop {
            let current_tail = self.tail.load(Ordering::Acquire);
            
            unsafe {
                let prev = (*current_tail).prev.load(Ordering::Acquire);
                
                // Check if deque is empty
                if prev.is_null() {
                    return None;
                }
                
                // Try to update tail pointer
                if self.tail.compare_exchange_weak(
                    current_tail,
                    prev,
                    Ordering::Release,
                    Ordering::Relaxed
                ).is_ok() {
                    (*prev).next.store(ptr::null_mut(), Ordering::Relaxed);
                    
                    // Extract data and cleanup
                    let data = (*current_tail).data.take();
                    let _ = Box::from_raw(current_tail);
                    
                    if data.is_some() {
                        self.size.fetch_sub(1, Ordering::Relaxed);
                    }
                    
                    return data;
                }
            }
            
            // Exponential backoff to prevent busy waiting
            if backoff < 64 {
                for _ in 0..backoff {
                    std::hint::spin_loop();
                }
                backoff *= 2;
            } else {
                std::thread::yield_now();
                backoff = 1;
            }
        }
    }

    /// Get approximate length
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    /// Check if deque is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Try to steal multiple items for batch processing
    pub fn steal_batch(&self, max_items: usize) -> Vec<T> {
        let mut stolen = Vec::with_capacity(max_items);
        
        for _ in 0..max_items {
            if let Some(item) = self.steal() {
                stolen.push(item);
            } else {
                break;
            }
        }
        
        stolen
    }

    /// Clear all items (owner thread only)
    pub fn clear(&self) {
        while self.pop().is_some() {}
    }
}

impl<T> Drop for LockFreeDeque<T> {
    fn drop(&mut self) {
        // Clear all remaining items
        self.clear();
        
        // Clean up sentinel node
        let sentinel = self.head.load(Ordering::Relaxed);
        if !sentinel.is_null() {
            unsafe {
                let _ = Box::from_raw(sentinel);
            }
        }
    }
}

impl<T> Default for LockFreeDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Priority-aware lock-free deque for goroutine scheduling
pub struct PriorityLockFreeDeque<T> {
    /// High priority deque
    high_priority: LockFreeDeque<T>,
    /// Normal priority deque
    normal_priority: LockFreeDeque<T>,
    /// Low priority deque
    low_priority: LockFreeDeque<T>,
    /// Total size across all priorities
    total_size: AtomicUsize,
}

impl<T> PriorityLockFreeDeque<T> {
    /// Create a new priority deque
    pub fn new() -> Self {
        Self {
            high_priority: LockFreeDeque::new(),
            normal_priority: LockFreeDeque::new(),
            low_priority: LockFreeDeque::new(),
            total_size: AtomicUsize::new(0),
        }
    }

    /// Push item with specified priority
    pub fn push_with_priority(&self, item: T, priority: u8) -> Result<(), T> {
        let result = match priority {
            0..=2 => self.low_priority.push(item),
            3..=6 => self.normal_priority.push(item),
            7..=10 => self.high_priority.push(item),
            _ => self.normal_priority.push(item),
        };
        
        if result.is_ok() {
            self.total_size.fetch_add(1, Ordering::Relaxed);
        }
        
        result
    }

    /// Pop highest priority item available
    pub fn pop(&self) -> Option<T> {
        // Try high priority first
        if let Some(item) = self.high_priority.pop() {
            self.total_size.fetch_sub(1, Ordering::Relaxed);
            return Some(item);
        }
        
        // Then normal priority
        if let Some(item) = self.normal_priority.pop() {
            self.total_size.fetch_sub(1, Ordering::Relaxed);
            return Some(item);
        }
        
        // Finally low priority
        if let Some(item) = self.low_priority.pop() {
            self.total_size.fetch_sub(1, Ordering::Relaxed);
            return Some(item);
        }
        
        None
    }

    /// Steal from any priority level
    pub fn steal(&self) -> Option<T> {
        // Try to steal from high priority first
        if let Some(item) = self.high_priority.steal() {
            self.total_size.fetch_sub(1, Ordering::Relaxed);
            return Some(item);
        }
        
        // Then normal priority
        if let Some(item) = self.normal_priority.steal() {
            self.total_size.fetch_sub(1, Ordering::Relaxed);
            return Some(item);
        }
        
        // Finally low priority
        if let Some(item) = self.low_priority.steal() {
            self.total_size.fetch_sub(1, Ordering::Relaxed);
            return Some(item);
        }
        
        None
    }

    /// Get total length across all priorities
    pub fn len(&self) -> usize {
        self.total_size.load(Ordering::Relaxed)
    }

    /// Check if all priority queues are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get statistics for each priority level
    pub fn get_priority_stats(&self) -> (usize, usize, usize) {
        (
            self.high_priority.len(),
            self.normal_priority.len(),
            self.low_priority.len(),
        )
    }
}

impl<T> Default for PriorityLockFreeDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T: Send> Send for PriorityLockFreeDeque<T> {}
unsafe impl<T: Send> Sync for PriorityLockFreeDeque<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Arc;

    #[test]
    fn test_basic_operations() {
        let deque = LockFreeDeque::new();
        
        assert!(deque.is_empty());
        assert_eq!(deque.len(), 0);
        
        // Push some items
        assert!(deque.push(1).is_ok());
        assert!(deque.push(2).is_ok());
        assert!(deque.push(3).is_ok());
        
        assert_eq!(deque.len(), 3);
        assert!(!deque.is_empty());
        
        // Pop items
        assert_eq!(deque.pop(), Some(3));
        assert_eq!(deque.pop(), Some(2));
        assert_eq!(deque.pop(), Some(1));
        assert_eq!(deque.pop(), None);
        
        assert!(deque.is_empty());
    }

    #[test]
    fn test_work_stealing() {
        let deque = Arc::new(LockFreeDeque::new());
        
        // Push items from main thread
        for i in 0..10 {
            deque.push(i).unwrap();
        }
        
        let deque_clone = deque.clone();
        let handle = thread::spawn(move || {
            let mut stolen = Vec::new();
            while let Some(item) = deque_clone.steal() {
                stolen.push(item);
            }
            stolen
        });
        
        // Pop remaining items
        let mut popped = Vec::new();
        while let Some(item) = deque.pop() {
            popped.push(item);
        }
        
        let stolen = handle.join().unwrap();
        
        // Verify all items were processed
        let mut all_items = popped;
        all_items.extend(stolen);
        all_items.sort();
        
        assert_eq!(all_items, (0..10).collect::<Vec<_>>());
    }

    #[test]
    fn test_priority_deque() {
        let pq = PriorityLockFreeDeque::new();
        
        // Push items with different priorities
        pq.push_with_priority("low1", 1).unwrap();
        pq.push_with_priority("high1", 8).unwrap();
        pq.push_with_priority("normal1", 5).unwrap();
        pq.push_with_priority("high2", 9).unwrap();
        pq.push_with_priority("low2", 2).unwrap();
        
        // Should pop in priority order
        assert_eq!(pq.pop(), Some("high2"));
        assert_eq!(pq.pop(), Some("high1"));
        assert_eq!(pq.pop(), Some("normal1"));
        assert_eq!(pq.pop(), Some("low2"));
        assert_eq!(pq.pop(), Some("low1"));
        assert_eq!(pq.pop(), None);
    }
}
