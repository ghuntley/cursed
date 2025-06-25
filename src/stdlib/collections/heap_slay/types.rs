// Basic heap implementations and convenience types
//
// This module provides concrete implementations of the heap Interface
// for common data types, including integers, strings, and priority queues.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
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

/// A utility for sorting arrays using heap sort algorithm
/// Provides efficient O(n log n) sorting with O(1) space complexity
#[derive(Debug, Clone)]
pub struct HeapSorter<T> {
    comparator: fn(&T, &T) -> std::cmp::Ordering,
}

impl<T> HeapSorter<T> {
    /// Create a new heap sorter with a custom comparator
    pub fn new(comparator: fn(&T, &T) -> std::cmp::Ordering) -> Self {
        Self { comparator }
    }
    
    /// Sort a mutable slice using heap sort
    pub fn sort(&self, data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        
        // Build max heap
        for i in (0..data.len() / 2).rev() {
            self.heapify_down(data, i, data.len());
        }
        
        // Extract elements from heap
        for i in (1..data.len()).rev() {
            data.swap(0, i);
            self.heapify_down(data, 0, i);
        }
    }
    
    /// Sort and return a new vector
    pub fn sort_vec(&self, mut data: Vec<T>) -> Vec<T> {
        self.sort(&mut data);
        data
    }
    
    fn heapify_down(&self, data: &mut [T], start: usize, end: usize) {
        let mut parent = start;
        
        while 2 * parent + 1 < end {
            let left_child = 2 * parent + 1;
            let right_child = left_child + 1;
            let mut largest = parent;
            
            if (self.comparator)(&data[left_child], &data[largest]) == std::cmp::Ordering::Greater {
                largest = left_child;
            }
            
            if right_child < end && (self.comparator)(&data[right_child], &data[largest]) == std::cmp::Ordering::Greater {
                largest = right_child;
            }
            
            if largest == parent {
                break;
            }
            
            data.swap(parent, largest);
            parent = largest;
        }
    }
}

impl<T: Ord> Default for HeapSorter<T> {
    fn default() -> Self {
        Self::new(|a, b| a.cmp(b))
    }
}

/// A generic binary heap implementation
/// Can be configured as min-heap or max-heap based on comparator
#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
    comparator: fn(&T, &T) -> std::cmp::Ordering,
}

impl<T> BinaryHeap<T> {
    /// Create a new binary heap with custom comparator
    pub fn new(comparator: fn(&T, &T) -> std::cmp::Ordering) -> Self {
        Self {
            data: Vec::new(),
            comparator,
        }
    }
    
    /// Create a binary heap from existing data
    pub fn from_vec(data: Vec<T>, comparator: fn(&T, &T) -> std::cmp::Ordering) -> Self {
        let mut heap = Self { data, comparator };
        heap.heapify();
        heap
    }
    
    /// Push an element into the heap
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.bubble_up(self.data.len() - 1);
    }
    
    /// Pop the top element from the heap
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        
        if self.data.len() == 1 {
            return self.data.pop();
        }
        
        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        let result = self.data.pop();
        self.bubble_down(0);
        result
    }
    
    /// Peek at the top element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
    
    /// Get the number of elements in the heap
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Clear all elements from the heap
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Get capacity of underlying storage
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    
    /// Reserve capacity for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    
    fn heapify(&mut self) {
        if self.data.len() <= 1 {
            return;
        }
        
        for i in (0..self.data.len() / 2).rev() {
            self.bubble_down(i);
        }
    }
    
    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;
            
            if (self.comparator)(&self.data[index], &self.data[parent_index]) != std::cmp::Ordering::Greater {
                break;
            }
            
            self.data.swap(index, parent_index);
            index = parent_index;
        }
    }
    
    fn bubble_down(&mut self, mut index: usize) {
        let len = self.data.len();
        
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            let mut largest = index;
            
            if left_child < len && (self.comparator)(&self.data[left_child], &self.data[largest]) == std::cmp::Ordering::Greater {
                largest = left_child;
            }
            
            if right_child < len && (self.comparator)(&self.data[right_child], &self.data[largest]) == std::cmp::Ordering::Greater {
                largest = right_child;
            }
            
            if largest == index {
                break;
            }
            
            self.data.swap(index, largest);
            index = largest;
        }
    }
}

/// A min-heap implementation (smallest element at top)
#[derive(Debug, Clone)]
pub struct MinHeap<T>(BinaryHeap<T>);

impl<T: Ord> MinHeap<T> {
    /// Create a new min-heap
    pub fn new() -> Self {
        Self(BinaryHeap::new(|a, b| b.cmp(a))) // Reverse order for min-heap
    }
    
    /// Create a min-heap from existing data
    pub fn from_vec(data: Vec<T>) -> Self {
        Self(BinaryHeap::from_vec(data, |a, b| b.cmp(a)))
    }
    
    /// Push an element into the min-heap
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
    
    /// Pop the minimum element from the heap
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    
    /// Peek at the minimum element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.0.peek()
    }
    
    /// Get the number of elements in the heap
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    /// Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    /// Clear all elements from the heap
    pub fn clear(&mut self) {
        self.0.clear();
    }
    
    /// Get capacity of underlying storage
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    
    /// Reserve capacity for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
}

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A max-heap implementation (largest element at top)
#[derive(Debug, Clone)]
pub struct MaxHeap<T>(BinaryHeap<T>);

impl<T: Ord> MaxHeap<T> {
    /// Create a new max-heap
    pub fn new() -> Self {
        Self(BinaryHeap::new(|a, b| a.cmp(b))) // Normal order for max-heap
    }
    
    /// Create a max-heap from existing data
    pub fn from_vec(data: Vec<T>) -> Self {
        Self(BinaryHeap::from_vec(data, |a, b| a.cmp(b)))
    }
    
    /// Push an element into the max-heap
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
    
    /// Pop the maximum element from the heap
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    
    /// Peek at the maximum element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.0.peek()
    }
    
    /// Get the number of elements in the heap
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    /// Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    /// Clear all elements from the heap
    pub fn clear(&mut self) {
        self.0.clear();
    }
    
    /// Get capacity of underlying storage
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    
    /// Reserve capacity for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
}

impl<T: Ord> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for heap structures
#[derive(Debug)]
pub struct HeapIterator<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> HeapIterator<T> {
    /// Create a new heap iterator from data
    pub fn new(data: Vec<T>) -> Self {
        Self { data, index: 0 }
    }
    
    /// Create an iterator from a BinaryHeap (consumes the heap)
    pub fn from_binary_heap(heap: BinaryHeap<T>) -> Self {
        Self::new(heap.data)
    }
    
    /// Create an iterator from a MinHeap (consumes the heap)
    pub fn from_min_heap(heap: MinHeap<T>) -> Self {
        Self::new(heap.0.data)
    }
    
    /// Create an iterator from a MaxHeap (consumes the heap)
    pub fn from_max_heap(heap: MaxHeap<T>) -> Self {
        Self::new(heap.0.data)
    }
    
    /// Get the remaining length
    pub fn len(&self) -> usize {
        self.data.len() - self.index
    }
    
    /// Check if iterator is empty
    pub fn is_empty(&self) -> bool {
        self.index >= self.data.len()
    }
}

impl<T> Iterator for HeapIterator<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = unsafe { std::ptr::read(&self.data[self.index]) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len();
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for HeapIterator<T> {}

impl<T> Drop for HeapIterator<T> {
    fn drop(&mut self) {
        // Properly drop remaining elements
        while self.index < self.data.len() {
            unsafe { std::ptr::drop_in_place(&mut self.data[self.index]) };
            self.index += 1;
        }
    }
}

/// Heap-specific error type (alias for HeapResult errors)
pub type HeapError = crate::error::CursedError;

