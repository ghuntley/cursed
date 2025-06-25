/// Concurrent collections for thread-safe data structures
/// 
/// This module provides thread-safe collections including:
/// - ConcurrentHashMap and ConcurrentVec
/// - Channels for message passing
/// - Lock-free data structures

// use crate::stdlib::sync::error::{SyncError, SyncResult, channel_error, timeout_error};
// use crate::stdlib::sync::primitives::{Mutex, RwLock, AtomicUsize, AtomicBool, Ordering};
use std::collections::HashMap;
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};
use std::hash::Hash;
use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicUsize as StdAtomicUsize, Ordering as StdOrdering};
use std::ptr;
use crate::error::CursedError;

// Global channel statistics
static CHANNEL_COUNT: StdAtomicUsize = StdAtomicUsize::new(0);
static MESSAGES_SENT: AtomicU64 = AtomicU64::new(0);
static MESSAGES_RECEIVED: AtomicU64 = AtomicU64::new(0);

//==============================================================================
// ConcurrentHashMap
//==============================================================================

/// A thread-safe hash map
pub struct ConcurrentHashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    inner: RwLock<HashMap<K, V>>,
    size: AtomicUsize,
    name: Option<String>,
}

impl<K, V> ConcurrentHashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new concurrent hash map
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
            size: AtomicUsize::new(0),
            name: None,
        }
    }

    /// Create a new named concurrent hash map
    pub fn named(name: &str) -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
            size: AtomicUsize::new(0),
            name: Some(name.to_string()),
        }
    }

    /// Create with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: RwLock::new(HashMap::with_capacity(capacity)),
            size: AtomicUsize::new(0),
            name: None,
        }
    }

    /// Insert a key-value pair
    pub fn insert(&self, key: K, value: V) -> SyncResult<Option<V>> {
        let mut map = self.inner.write()?;
        let old_value = map.insert(key, value);
        
        if old_value.is_none() {
            self.size.fetch_add(1, Ordering::Relaxed);
        }
        
        Ok(old_value)
    }

    /// Get a value by key
    pub fn get(&self, key: &K) -> SyncResult<Option<V>> {
        let map = self.inner.read()?;
        Ok(map.get(key).cloned())
    }

    /// Check if a key exists
    pub fn contains_key(&self, key: &K) -> SyncResult<bool> {
        let map = self.inner.read()?;
        Ok(map.contains_key(key))
    }

    /// Remove a key-value pair
    pub fn remove(&self, key: &K) -> SyncResult<Option<V>> {
        let mut map = self.inner.write()?;
        let old_value = map.remove(key);
        
        if old_value.is_some() {
            self.size.fetch_sub(1, Ordering::Relaxed);
        }
        
        Ok(old_value)
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear all entries
    pub fn clear(&self) -> SyncResult<()> {
        let mut map = self.inner.write()?;
        map.clear();
        self.size.store(0, Ordering::Relaxed);
        Ok(())
    }

    /// Get all keys
    pub fn keys(&self) -> SyncResult<Vec<K>> {
        let map = self.inner.read()?;
        Ok(map.keys().cloned().collect())
    }

    /// Get all values
    pub fn values(&self) -> SyncResult<Vec<V>> {
        let map = self.inner.read()?;
        Ok(map.values().cloned().collect())
    }

    /// Execute a function with read access to the map
    pub fn with_read<F, R>(&self, f: F) -> SyncResult<R>
    where
        F: FnOnce(&HashMap<K, V>) -> R,
    {
        let map = self.inner.read()?;
        Ok(f(&*map))
    }

    /// Execute a function with write access to the map
    pub fn with_write<F, R>(&self, f: F) -> SyncResult<R>
    where
        F: FnOnce(&mut HashMap<K, V>) -> R,
    {
        let mut map = self.inner.write()?;
        let result = f(&mut *map);
        self.size.store(map.len(), Ordering::Relaxed);
        Ok(result)
    }

    /// Get the name of the concurrent hash map
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<K, V> Default for ConcurrentHashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

//==============================================================================
// ConcurrentVec
//==============================================================================

/// A thread-safe vector
pub struct ConcurrentVec<T>
where
    T: Clone,
{
    inner: RwLock<Vec<T>>,
    name: Option<String>,
}

impl<T> ConcurrentVec<T>
where
    T: Clone,
{
    /// Create a new concurrent vector
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Vec::new()),
            name: None,
        }
    }

    /// Create a new named concurrent vector
    pub fn named(name: &str) -> Self {
        Self {
            inner: RwLock::new(Vec::new()),
            name: Some(name.to_string()),
        }
    }

    /// Create with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: RwLock::new(Vec::with_capacity(capacity)),
            name: None,
        }
    }

    /// Push an element to the end
    pub fn push(&self, item: T) -> SyncResult<()> {
        let mut vec = self.inner.write()?;
        vec.push(item);
        Ok(())
    }

    /// Pop an element from the end
    pub fn pop(&self) -> SyncResult<Option<T>> {
        let mut vec = self.inner.write()?;
        Ok(vec.pop())
    }

    /// Get an element by index
    pub fn get(&self, index: usize) -> SyncResult<Option<T>> {
        let vec = self.inner.read()?;
        Ok(vec.get(index).cloned())
    }

    /// Set an element at index
    pub fn set(&self, index: usize, item: T) -> SyncResult<Option<T>> {
        let mut vec = self.inner.write()?;
        if index < vec.len() {
            let old = vec[index].clone();
            vec[index] = item;
            Ok(Some(old))
        } else {
            Ok(None)
        }
    }

    /// Insert an element at index
    pub fn insert(&self, index: usize, item: T) -> SyncResult<()> {
        let mut vec = self.inner.write()?;
        if index <= vec.len() {
            vec.insert(index, item);
            Ok(())
        } else {
            Err(SyncError::General {
                message: format!("Index {} out of bounds for vector of length {}", index, vec.len()),
            })
        }
    }

    /// Remove an element at index
    pub fn remove(&self, index: usize) -> SyncResult<T> {
        let mut vec = self.inner.write()?;
        if index < vec.len() {
            Ok(vec.remove(index))
        } else {
            Err(SyncError::General {
                message: format!("Index {} out of bounds for vector of length {}", index, vec.len()),
            })
        }
    }

    /// Get the length
    pub fn len(&self) -> SyncResult<usize> {
        let vec = self.inner.read()?;
        Ok(vec.len())
    }

    /// Check if empty
    pub fn is_empty(&self) -> SyncResult<bool> {
        let vec = self.inner.read()?;
        Ok(vec.is_empty())
    }

    /// Clear all elements
    pub fn clear(&self) -> SyncResult<()> {
        let mut vec = self.inner.write()?;
        vec.clear();
        Ok(())
    }

    /// Execute a function with read access to the vector
    pub fn with_read<F, R>(&self, f: F) -> SyncResult<R>
    where
        F: FnOnce(&Vec<T>) -> R,
    {
        let vec = self.inner.read()?;
        Ok(f(&*vec))
    }

    /// Execute a function with write access to the vector
    pub fn with_write<F, R>(&self, f: F) -> SyncResult<R>
    where
        F: FnOnce(&mut Vec<T>) -> R,
    {
        let mut vec = self.inner.write()?;
        Ok(f(&mut *vec))
    }

    /// Get the name of the concurrent vector
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<T> Default for ConcurrentVec<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

//==============================================================================
// Channels
//==============================================================================

/// CursedError types for channel operations
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelError {
    /// Channel is closed
    Closed,
    /// Send operation would block
    WouldBlock,
    /// Receive operation would block
    Empty,
    /// Operation timed out
    Timeout,
    /// Invalid operation
    InvalidOperation(String),
}

// impl std::fmt::Display for ChannelError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ChannelError::Closed => write!(f, "Channel is closed"),
//             ChannelError::WouldBlock => write!(f, "Operation would block"),
//             ChannelError::Empty => write!(f, "Channel is empty"),
//             ChannelError::Timeout => write!(f, "Operation timed out"),
//             ChannelError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for ChannelError {}
// 
/// Channel sender
pub struct ChannelSender<T> {
    sender: mpsc::Sender<T>,
    is_bounded: bool,
    capacity: Option<usize>,
}

impl<T> ChannelSender<T> {
    /// Send a message
    pub fn send(&self, msg: T) -> crate::error::Result<()> {
        match self.sender.send(msg) {
            Ok(()) => {
                MESSAGES_SENT.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(_) => Err(ChannelError::Closed),
        }
    }

    /// Check if the channel is bounded
    pub fn is_bounded(&self) -> bool {
        self.is_bounded
    }

    /// Get the capacity (if bounded)
    pub fn capacity(&self) -> Option<usize> {
        self.capacity
    }
}

impl<T> Clone for ChannelSender<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            is_bounded: self.is_bounded,
            capacity: self.capacity,
        }
    }
}

/// Channel receiver
pub struct ChannelReceiver<T> {
    receiver: mpsc::Receiver<T>,
    is_bounded: bool,
    capacity: Option<usize>,
}

impl<T> ChannelReceiver<T> {
    /// Receive a message (blocking)
    pub fn recv(&self) -> crate::error::Result<()> {
        match self.receiver.recv() {
            Ok(msg) => {
                MESSAGES_RECEIVED.fetch_add(1, Ordering::Relaxed);
                Ok(msg)
            }
            Err(_) => Err(ChannelError::Closed),
        }
    }

    /// Try to receive a message (non-blocking)
    pub fn try_recv(&self) -> crate::error::Result<()> {
        match self.receiver.try_recv() {
            Ok(msg) => {
                MESSAGES_RECEIVED.fetch_add(1, Ordering::Relaxed);
                Ok(msg)
            }
            Err(mpsc::TryRecvError::Empty) => Err(ChannelError::Empty),
            Err(mpsc::TryRecvError::Disconnected) => Err(ChannelError::Closed),
        }
    }

    /// Receive with timeout
    pub fn recv_timeout(&self, timeout: Duration) -> crate::error::Result<()> {
        match self.receiver.recv_timeout(timeout) {
            Ok(msg) => {
                MESSAGES_RECEIVED.fetch_add(1, Ordering::Relaxed);
                Ok(msg)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => Err(ChannelError::Timeout),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(ChannelError::Closed),
        }
    }

    /// Check if the channel is bounded
    pub fn is_bounded(&self) -> bool {
        self.is_bounded
    }

    /// Get the capacity (if bounded)
    pub fn capacity(&self) -> Option<usize> {
        self.capacity
    }

    /// Create an iterator over received messages
    pub fn iter(&self) -> ChannelIterator<T> {
        ChannelIterator {
            receiver: self,
        }
    }
}

/// Iterator over channel messages
pub struct ChannelIterator<'a, T> {
    receiver: &'a ChannelReceiver<T>,
}

impl<'a, T> Iterator for ChannelIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}

/// Create an unbounded channel
pub fn channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    let (sender, receiver) = mpsc::channel();
    CHANNEL_COUNT.fetch_add(1, Ordering::Relaxed);
    
    (
        ChannelSender {
            sender,
            is_bounded: false,
            capacity: None,
        },
        ChannelReceiver {
            receiver,
            is_bounded: false,
            capacity: None,
        },
    )
}

/// Create a bounded channel with the specified capacity
pub fn bounded_channel<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    // Note: This is simplified - real implementation would need proper bounded channel
    let (sender, receiver) = mpsc::channel();
    CHANNEL_COUNT.fetch_add(1, Ordering::Relaxed);
    
    (
        ChannelSender {
            sender,
            is_bounded: true,
            capacity: Some(capacity),
        },
        ChannelReceiver {
            receiver,
            is_bounded: true,
            capacity: Some(capacity),
        },
    )
}

/// Create an unbounded channel (alias for channel)
pub fn unbounded_channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    channel()
}

/// Select operation for multiple channels (simplified implementation)
pub fn select_channel<T>(receivers: &[&ChannelReceiver<T>]) -> crate::error::Result<()> {
    // Simple round-robin implementation
    for (index, receiver) in receivers.iter().enumerate() {
        if let Ok(msg) = receiver.try_recv() {
            return Ok((index, msg));
        }
    }
    Err(ChannelError::Empty)
}

/// Try select operation for multiple channels
pub fn try_select_channel<T>(receivers: &[&ChannelReceiver<T>], timeout: Duration) -> crate::error::Result<()> {
    let start = Instant::now();
    
    while start.elapsed() < timeout {
        for (index, receiver) in receivers.iter().enumerate() {
            if let Ok(msg) = receiver.try_recv() {
                return Ok((index, msg));
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    
    Err(ChannelError::Timeout)
}

//==============================================================================
// Lock-Free Data Structures
//==============================================================================

/// A lock-free stack implementation
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    /// Create a new lock-free stack
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Push an element onto the stack
    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = head;
            }
            
            if self.head.compare_and_swap(head, new_node, Ordering::Release) == head {
                break;
            }
        }
    }

    /// Pop an element from the stack
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };
            
            if self.head.compare_and_swap(head, next, Ordering::Release) == head {
                let data = unsafe { Box::from_raw(head).data };
                return Some(data);
            }
        }
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Default for LockFreeStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

/// A lock-free queue implementation (simplified)
pub struct LockFreeQueue<T> {
    head: AtomicPtr<QueueNode<T>>,
    tail: AtomicPtr<QueueNode<T>>,
}

struct QueueNode<T> {
    data: Option<T>,
    next: AtomicPtr<QueueNode<T>>,
}

impl<T> LockFreeQueue<T> {
    /// Create a new lock-free queue
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(QueueNode {
            data: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    /// Enqueue an element
    pub fn enqueue(&self, data: T) {
        let new_node = Box::into_raw(Box::new(QueueNode {
            data: Some(data),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };
            
            if next.is_null() {
                if unsafe { (*tail).next.compare_and_swap(ptr::null_mut(), new_node, Ordering::Release) }.is_null() {
                    break;
                }
            } else {
                let _ = self.tail.compare_and_swap(tail, next, Ordering::Release);
            }
        }
        
        let tail = self.tail.load(Ordering::Acquire);
        let _ = self.tail.compare_and_swap(tail, new_node, Ordering::Release);
    }

    /// Dequeue an element
    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == tail {
                if next.is_null() {
                    return None;
                }
                let _ = self.tail.compare_and_swap(tail, next, Ordering::Release);
            } else {
                if next.is_null() {
                    continue;
                }
                
                let data = unsafe { (*next).data.take() };
                if self.head.compare_and_swap(head, next, Ordering::Release) == head {
                    unsafe { Box::from_raw(head) };
                    return data;
                }
            }
        }
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        head == tail && unsafe { (*head).next.load(Ordering::Acquire).is_null() }
    }
}

impl<T> Default for LockFreeQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LockFreeQueue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        unsafe {
            let head = self.head.load(Ordering::Acquire);
            if !head.is_null() {
                Box::from_raw(head);
            }
        }
    }
}

/// An atomic counter with additional operations
pub struct AtomicCounter {
    value: AtomicUsize,
    name: Option<String>,
}

impl AtomicCounter {
    /// Create a new atomic counter
    pub fn new(initial: usize) -> Self {
        Self {
            value: AtomicUsize::new(initial),
            name: None,
        }
    }

    /// Create a named atomic counter
    pub fn named(initial: usize, name: &str) -> Self {
        Self {
            value: AtomicUsize::new(initial),
            name: Some(name.to_string()),
        }
    }

    /// Get the current value
    pub fn get(&self) -> usize {
        self.value.load(Ordering::Acquire)
    }

    /// Set the value
    pub fn set(&self, value: usize) {
        self.value.store(value, Ordering::Release);
    }

    /// Increment and return the new value
    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::AcqRel) + 1
    }

    /// Decrement and return the new value
    pub fn decrement(&self) -> usize {
        self.value.fetch_sub(1, Ordering::AcqRel) - 1
    }

    /// Add a value and return the new total
    pub fn add(&self, amount: usize) -> usize {
        self.value.fetch_add(amount, Ordering::AcqRel) + amount
    }

    /// Subtract a value and return the new total
    pub fn subtract(&self, amount: usize) -> usize {
        self.value.fetch_sub(amount, Ordering::AcqRel) - amount
    }

    /// Compare and swap
    pub fn compare_and_swap(&self, current: usize, new: usize) -> usize {
        self.value.compare_and_swap(current, new, Ordering::AcqRel)
    }

    /// Reset to zero and return the previous value
    pub fn reset(&self) -> usize {
        self.value.swap(0, Ordering::AcqRel)
    }

    /// Get the name of the counter
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new(0)
    }
}

//==============================================================================
// Additional Concurrent Collections
//==============================================================================

/// A concurrent queue using locks
pub struct ConcurrentQueue<T>
where
    T: Clone,
{
    inner: Mutex<std::collections::VecDeque<T>>,
    name: Option<String>,
}

impl<T> ConcurrentQueue<T>
where
    T: Clone,
{
    /// Create a new concurrent queue
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(std::collections::VecDeque::new()),
            name: None,
        }
    }

    /// Create a named concurrent queue
    pub fn named(name: &str) -> Self {
        Self {
            inner: Mutex::new(std::collections::VecDeque::new()),
            name: Some(name.to_string()),
        }
    }

    /// Enqueue an element
    pub fn enqueue(&self, item: T) -> SyncResult<()> {
        let mut queue = self.inner.lock()?;
        queue.push_back(item);
        Ok(())
    }

    /// Dequeue an element
    pub fn dequeue(&self) -> SyncResult<Option<T>> {
        let mut queue = self.inner.lock()?;
        Ok(queue.pop_front())
    }

    /// Peek at the front element
    pub fn peek(&self) -> SyncResult<Option<T>> {
        let queue = self.inner.lock()?;
        Ok(queue.front().cloned())
    }

    /// Get the length
    pub fn len(&self) -> SyncResult<usize> {
        let queue = self.inner.lock()?;
        Ok(queue.len())
    }

    /// Check if empty
    pub fn is_empty(&self) -> SyncResult<bool> {
        let queue = self.inner.lock()?;
        Ok(queue.is_empty())
    }

    /// Clear all elements
    pub fn clear(&self) -> SyncResult<()> {
        let mut queue = self.inner.lock()?;
        queue.clear();
        Ok(())
    }

    /// Get the name of the queue
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<T> Default for ConcurrentQueue<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A concurrent stack using locks
pub struct ConcurrentStack<T>
where
    T: Clone,
{
    inner: Mutex<Vec<T>>,
    name: Option<String>,
}

impl<T> ConcurrentStack<T>
where
    T: Clone,
{
    /// Create a new concurrent stack
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Vec::new()),
            name: None,
        }
    }

    /// Create a named concurrent stack
    pub fn named(name: &str) -> Self {
        Self {
            inner: Mutex::new(Vec::new()),
            name: Some(name.to_string()),
        }
    }

    /// Push an element
    pub fn push(&self, item: T) -> SyncResult<()> {
        let mut stack = self.inner.lock()?;
        stack.push(item);
        Ok(())
    }

    /// Pop an element
    pub fn pop(&self) -> SyncResult<Option<T>> {
        let mut stack = self.inner.lock()?;
        Ok(stack.pop())
    }

    /// Peek at the top element
    pub fn peek(&self) -> SyncResult<Option<T>> {
        let stack = self.inner.lock()?;
        Ok(stack.last().cloned())
    }

    /// Get the length
    pub fn len(&self) -> SyncResult<usize> {
        let stack = self.inner.lock()?;
        Ok(stack.len())
    }

    /// Check if empty
    pub fn is_empty(&self) -> SyncResult<bool> {
        let stack = self.inner.lock()?;
        Ok(stack.is_empty())
    }

    /// Clear all elements
    pub fn clear(&self) -> SyncResult<()> {
        let mut stack = self.inner.lock()?;
        stack.clear();
        Ok(())
    }

    /// Get the name of the stack
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<T> Default for ConcurrentStack<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

//==============================================================================
// Statistics
//==============================================================================

/// Get channel statistics  
// pub fn get_channel_statistics() -> crate::stdlib::sync::ChannelStatistics {
//     crate::stdlib::sync::ChannelStatistics {
        active_channels: CHANNEL_COUNT.load(StdOrdering::Relaxed),
        messages_sent: MESSAGES_SENT.load(StdOrdering::Relaxed),
        messages_received: MESSAGES_RECEIVED.load(StdOrdering::Relaxed),
        blocked_senders: 0, // Would need more complex tracking
        blocked_receivers: 0, // Would need more complex tracking
    }
}

