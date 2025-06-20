//! Basic heap implementations and convenience types
//!
//! This module provides concrete implementations of the heap Interface
//! for common data types, including integers, strings, and priority queues.

use crate::stdlib::value::Value;
use super::core::Interface;
use super::{HeapResult, heap_error};

/// A heap implementation for integers (normie).
/// This is a min-heap by default, where the smallest element is at the root.
///
/// # Examples
/// ```cursed
/// let mut heap = IntHeap::new();
/// heap.push_value(3);
/// heap.push_value(1);
/// heap.push_value(4);
/// 
/// heap_slay::init(&mut heap);
/// let min = heap_slay::pop(&mut heap); // Returns 1
/// ```
#[derive(Debug, Clone)]
pub struct IntHeap {
    data: Vec<i32>,
}

impl IntHeap {
    /// Create a new empty integer heap
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    
    /// Create an integer heap from a vector of values
    pub fn from(data: Vec<i32>) -> Self {
        Self { data }
    }
    
    /// Push an integer value directly (convenience method)
    pub fn push_value(&mut self, value: i32) {
        self.data.push(value);
    }
    
    /// Get the underlying data as a slice
    pub fn as_slice(&self) -> &[i32] {
        &self.data
    }
    
    /// Get the underlying data as a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [i32] {
        &mut self.data
    }
    
    /// Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get the capacity of the underlying vector
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    
    /// Reserve capacity for at least additional more elements
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
}

impl Interface for IntHeap {
    fn len(&self) -> i32 {
        self.data.len() as i32
    }
    
    fn less(&self, i: i32, j: i32) -> bool {
        self.data[i as usize] < self.data[j as usize]
    }
    
    fn swap(&mut self, i: i32, j: i32) {
        self.data.swap(i as usize, j as usize);
    }
    
    fn push(&mut self, x: Value) {
        match x {
            Value::Integer(val) => self.data.push(val),
            _ => {}, // Silently ignore non-integer values
        }
    }
    
    fn pop(&mut self) -> Option<Value> {
        self.data.pop().map(Value::Integer)
    }
}

impl Default for IntHeap {
    fn default() -> Self {
        Self::new()
    }
}

/// A heap implementation for strings (tea).
/// This is a min-heap by default, where strings are ordered lexicographically.
///
/// # Examples
/// ```cursed
/// let mut heap = StringHeap::new();
/// heap.push_value("world".to_string());
/// heap.push_value("hello".to_string());
/// heap.push_value("cursed".to_string());
/// 
/// heap_slay::init(&mut heap);
/// let min = heap_slay::pop(&mut heap); // Returns "cursed"
/// ```
#[derive(Debug, Clone)]
pub struct StringHeap {
    data: Vec<String>,
}

impl StringHeap {
    /// Create a new empty string heap
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    
    /// Create a string heap from a vector of strings
    pub fn from(data: Vec<String>) -> Self {
        Self { data }
    }
    
    /// Push a string value directly (convenience method)
    pub fn push_value(&mut self, value: String) {
        self.data.push(value);
    }
    
    /// Get the underlying data as a slice
    pub fn as_slice(&self) -> &[String] {
        &self.data
    }
    
    /// Get the underlying data as a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [String] {
        &mut self.data
    }
    
    /// Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get the capacity of the underlying vector
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    
    /// Reserve capacity for at least additional more elements
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
}

impl Interface for StringHeap {
    fn len(&self) -> i32 {
        self.data.len() as i32
    }
    
    fn less(&self, i: i32, j: i32) -> bool {
        self.data[i as usize] < self.data[j as usize]
    }
    
    fn swap(&mut self, i: i32, j: i32) {
        self.data.swap(i as usize, j as usize);
    }
    
    fn push(&mut self, x: Value) {
        match x {
            Value::String(val) => self.data.push(val),
            _ => {}, // Silently ignore non-string values
        }
    }
    
    fn pop(&mut self) -> Option<Value> {
        self.data.pop().map(Value::String)
    }
}

impl Default for StringHeap {
    fn default() -> Self {
        Self::new()
    }
}

/// An item in a priority queue with a value, priority, and index.
/// The index is used internally by the priority queue for efficient updates.
///
/// # Examples
/// ```cursed
/// let item = Item {
///     value: Value::String("important task".to_string()),
///     priority: 10,
///     index: 0,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Item {
    /// The actual value stored in the item
    pub value: Value,
    /// The priority of the item (higher values = higher priority)
    pub priority: i32,
    /// The current index of the item in the heap (maintained internally)
    pub index: i32,
}

impl Item {
    /// Create a new item with the given value and priority
    pub fn new(value: Value, priority: i32) -> Self {
        Self {
            value,
            priority,
            index: -1, // Will be set when added to heap
        }
    }
    
    /// Create a new item with string value and priority
    pub fn with_string(value: String, priority: i32) -> Self {
        Self::new(Value::String(value), priority)
    }
    
    /// Create a new item with integer value and priority
    pub fn with_integer(value: i32, priority: i32) -> Self {
        Self::new(Value::Integer(value), priority)
    }
    
    /// Update the priority of this item
    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
    
    /// Update the value of this item
    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }
    
    /// Check if this item is currently in a heap (index >= 0)
    pub fn is_in_heap(&self) -> bool {
        self.index >= 0
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.value == other.value
    }
}

impl Eq for Item {}

/// A priority queue implementation using a max-heap.
/// Items with higher priority values are returned first.
///
/// # Examples
/// ```cursed
/// let mut pq = PriorityQueue::new();
/// pq.push_item(Item::with_string("low priority".to_string(), 1));
/// pq.push_item(Item::with_string("high priority".to_string(), 10));
/// 
/// heap_slay::init(&mut pq);
/// let highest = heap_slay::pop(&mut pq); // Returns "high priority" item
/// ```
#[derive(Debug, Clone)]
pub struct PriorityQueue {
    data: Vec<Item>,
}

impl PriorityQueue {
    /// Create a new empty priority queue
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    
    /// Create a priority queue from a vector of items
    pub fn from(data: Vec<Item>) -> Self {
        Self { data }
    }
    
    /// Push an item directly (convenience method)
    pub fn push_item(&mut self, mut item: Item) {
        item.index = self.data.len() as i32;
        self.data.push(item);
    }
    
    /// Update the priority and value of an item in the queue
    /// The item must already be in the queue (index >= 0)
    pub fn update(&mut self, item: &mut Item, value: Value, priority: i32) -> HeapResult<()> {
        if item.index < 0 || item.index >= self.data.len() as i32 {
            return Err(heap_error("item not in queue or invalid index"));
        }
        
        item.value = value.clone();
        item.priority = priority;
        
        // Update the item in the heap
        let idx = item.index as usize;
        self.data[idx].value = value;
        self.data[idx].priority = priority;
        
        // Re-establish heap property
        super::core::fix(self, item.index)?;
        
        Ok(())
    }
    
    /// Get the underlying data as a slice
    pub fn as_slice(&self) -> &[Item] {
        &self.data
    }
    
    /// Get the underlying data as a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [Item] {
        &mut self.data
    }
    
    /// Check if the priority queue is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Peek at the highest priority item without removing it
    pub fn peek(&self) -> Option<&Item> {
        self.data.get(0)
    }
    
    /// Get the capacity of the underlying vector
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    
    /// Reserve capacity for at least additional more elements
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
}

impl Interface for PriorityQueue {
    fn len(&self) -> i32 {
        self.data.len() as i32
    }
    
    fn less(&self, i: i32, j: i32) -> bool {
        // Max-heap: higher priority values should come first
        self.data[i as usize].priority > self.data[j as usize].priority
    }
    
    fn swap(&mut self, i: i32, j: i32) {
        let i_usize = i as usize;
        let j_usize = j as usize;
        
        // Update the index fields when swapping
        self.data[i_usize].index = j;
        self.data[j_usize].index = i;
        
        self.data.swap(i_usize, j_usize);
    }
    
    fn push(&mut self, x: Value) {
        // For generic push, create item with priority 0
        let mut item = Item::new(x, 0);
        item.index = self.data.len() as i32;
        self.data.push(item);
    }
    
    fn pop(&mut self) -> Option<Value> {
        if let Some(mut item) = self.data.pop() {
            item.index = -1; // Mark as no longer in heap
            Some(item.value)
        } else {
            None
        }
    }
}

impl Default for PriorityQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;
    use super::super::core::{init, push, pop, is_heap};

    #[test]
    fn test_int_heap_new() {
        let heap = IntHeap::new();
        assert_eq!(heap.len(), 0);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_int_heap_from() {
        let heap = IntHeap::from(vec![3, 1, 4, 1, 5]);
        assert_eq!(heap.len(), 5);
        assert!(!heap.is_empty());
    }

    #[test]
    fn test_int_heap_push_value() {
        let mut heap = IntHeap::new();
        heap.push_value(42);
        assert_eq!(heap.len(), 1);
        assert_eq!(heap.as_slice(), &[42]);
    }

    #[test]
    fn test_int_heap_operations() {
        let mut heap = IntHeap::new();
        
        // Test push and initialization
        push(&mut heap, Value::Integer(3));
        push(&mut heap, Value::Integer(1));
        push(&mut heap, Value::Integer(4));
        push(&mut heap, Value::Integer(1));
        push(&mut heap, Value::Integer(5));
        
        init(&mut heap);
        assert!(is_heap(&heap));
        
        // Test pop
        let min = pop(&mut heap).unwrap().unwrap();
        assert_eq!(min, Value::Integer(1));
        assert!(is_heap(&heap));
    }

    #[test]
    fn test_string_heap_new() {
        let heap = StringHeap::new();
        assert_eq!(heap.len(), 0);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_string_heap_operations() {
        let mut heap = StringHeap::new();
        
        push(&mut heap, Value::String("world".to_string()));
        push(&mut heap, Value::String("hello".to_string()));
        push(&mut heap, Value::String("cursed".to_string()));
        
        init(&mut heap);
        assert!(is_heap(&heap));
        
        let min = pop(&mut heap).unwrap().unwrap();
        assert_eq!(min, Value::String("cursed".to_string()));
        assert!(is_heap(&heap));
    }

    #[test]
    fn test_item_creation() {
        let item = Item::new(Value::String("test".to_string()), 5);
        assert_eq!(item.priority, 5);
        assert_eq!(item.value, Value::String("test".to_string()));
        assert_eq!(item.index, -1);
        assert!(!item.is_in_heap());
    }

    #[test]
    fn test_item_with_string() {
        let item = Item::with_string("test".to_string(), 10);
        assert_eq!(item.priority, 10);
        assert_eq!(item.value, Value::String("test".to_string()));
    }

    #[test]
    fn test_item_with_integer() {
        let item = Item::with_integer(42, 3);
        assert_eq!(item.priority, 3);
        assert_eq!(item.value, Value::Integer(42));
    }

    #[test]
    fn test_item_setters() {
        let mut item = Item::new(Value::Integer(1), 1);
        
        item.set_priority(10);
        assert_eq!(item.priority, 10);
        
        item.set_value(Value::String("new".to_string()));
        assert_eq!(item.value, Value::String("new".to_string()));
    }

    #[test]
    fn test_priority_queue_new() {
        let pq = PriorityQueue::new();
        assert_eq!(pq.len(), 0);
        assert!(pq.is_empty());
        assert!(pq.peek().is_none());
    }

    #[test]
    fn test_priority_queue_push_item() {
        let mut pq = PriorityQueue::new();
        let item = Item::with_string("test".to_string(), 5);
        
        pq.push_item(item);
        assert_eq!(pq.len(), 1);
        assert!(!pq.is_empty());
        
        let peeked = pq.peek().unwrap();
        assert_eq!(peeked.priority, 5);
        assert_eq!(peeked.index, 0);
    }

    #[test]
    fn test_priority_queue_operations() {
        let mut pq = PriorityQueue::new();
        
        pq.push_item(Item::with_string("low".to_string(), 1));
        pq.push_item(Item::with_string("high".to_string(), 10));
        pq.push_item(Item::with_string("medium".to_string(), 5));
        
        init(&mut pq);
        assert!(is_heap(&pq));
        
        // Should return highest priority first (max-heap)
        let highest = pop(&mut pq).unwrap().unwrap();
        assert_eq!(highest, Value::String("high".to_string()));
        assert!(is_heap(&pq));
        
        let medium = pop(&mut pq).unwrap().unwrap();
        assert_eq!(medium, Value::String("medium".to_string()));
        assert!(is_heap(&pq));
    }

    #[test]
    fn test_priority_queue_update() {
        let mut pq = PriorityQueue::new();
        let mut item = Item::with_string("test".to_string(), 5);
        
        pq.push_item(item.clone());
        init(&mut pq);
        
        // Update the item
        let result = pq.update(&mut item, Value::String("updated".to_string()), 10);
        assert!(result.is_ok());
        assert!(is_heap(&pq));
        
        // Check the updated values
        let peeked = pq.peek().unwrap();
        assert_eq!(peeked.priority, 10);
        assert_eq!(peeked.value, Value::String("updated".to_string()));
    }

    #[test]
    fn test_priority_queue_update_invalid_item() {
        let mut pq = PriorityQueue::new();
        let mut item = Item::with_string("test".to_string(), 5);
        // Don't add item to queue
        
        let result = pq.update(&mut item, Value::String("updated".to_string()), 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_priority_queue_max_heap_behavior() {
        let mut pq = PriorityQueue::new();
        
        // Add items in random order
        pq.push_item(Item::with_integer(1, 3));
        pq.push_item(Item::with_integer(2, 1));
        pq.push_item(Item::with_integer(3, 5));
        pq.push_item(Item::with_integer(4, 2));
        pq.push_item(Item::with_integer(5, 4));
        
        init(&mut pq);
        
        // Should pop in descending priority order
        let mut priorities = Vec::new();
        while pq.len() > 0 {
            if let Some(Value::Integer(val)) = pop(&mut pq).unwrap() {
                let priority = val; // In this test, value == priority
                priorities.push(priority);
            }
        }
        
        // Should be in descending order based on priority
        assert_eq!(priorities, vec![3, 5, 1, 4, 2]); // Values corresponding to priorities 5,4,3,2,1
    }

    #[test]
    fn test_heap_interface_consistency() {
        // Test that all heap types maintain Interface contract
        let mut int_heap = IntHeap::new();
        let mut string_heap = StringHeap::new();
        let mut pq = PriorityQueue::new();
        
        // All should start empty
        assert_eq!(int_heap.len(), 0);
        assert_eq!(string_heap.len(), 0);
        assert_eq!(pq.len(), 0);
        
        // All should handle push/pop
        int_heap.push(Value::Integer(42));
        string_heap.push(Value::String("test".to_string()));
        pq.push(Value::Integer(1));
        
        assert_eq!(int_heap.len(), 1);
        assert_eq!(string_heap.len(), 1);
        assert_eq!(pq.len(), 1);
        
        let int_val = int_heap.pop();
        let string_val = string_heap.pop();
        let pq_val = pq.pop();
        
        assert_eq!(int_val, Some(Value::Integer(42)));
        assert_eq!(string_val, Some(Value::String("test".to_string())));
        assert_eq!(pq_val, Some(Value::Integer(1)));
    }
}
