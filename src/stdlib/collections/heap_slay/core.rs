// Core heap interface and algorithms
//
// This module provides the fundamental heap operations that work with any type
// implementing the `Interface` collab. The heap is maintained as a binary heap
// where each parent node satisfies the heap property relative to its children.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
use super::{HeapResult, validate_index, validate_not_empty};

/// The core heap interface that must be implemented by any type wanting to use heap operations.
/// This collab provides the essential operations needed to maintain heap invariants.
pub trait Interface {
    /// Returns the number of elements in the collection
    fn len(&self) -> i32;
    
    /// Reports whether element i should sort before element j.
    /// For a min-heap, this should return true if h[i] < h[j].
    /// For a max-heap, this should return true if h[i] > h[j].
    fn less(&self, i: i32, j: i32) -> bool;
    
    /// Swaps the elements with indexes i and j
    fn swap(&mut self, i: i32, j: i32);
    
    /// Adds an element to the collection
    fn push(&mut self, x: Value);
    
    /// Removes and returns the last element (highest index)
    fn pop(&mut self) -> Option<Value>;
}

/// Establish heap ordering in the given collection.
/// This function rearranges the elements to satisfy the heap property.
/// Time complexity: O(n)
///
/// # Arguments
/// * `h` - The collection to heapify
///
/// # Examples
/// ```cursed
/// let mut heap = IntHeap::from(vec![3, 1, 4, 1, 5]);
/// heap_slay::init(&mut heap);
/// // heap is now properly ordered
/// ```
pub fn init<H: Interface>(h: &mut H) {
    let n = h.len();
    // Start from the last non-leaf node and sift down
    for i in (0..n/2).rev() {
        down(h, i, n);
    }
}

/// Push an element onto the heap and restore ordering.
/// Time complexity: O(log n)
///
/// # Arguments
/// * `h` - The heap to push to
/// * `x` - The element to push
///
/// # Examples
/// ```cursed
/// let mut heap = IntHeap::new();
/// heap_slay::push(&mut heap, Value::Integer(42));
/// ```
pub fn push<H: Interface>(h: &mut H, x: Value) {
    h.push(x);
    up(h, h.len() - 1);
}

/// Pop the minimum element from the heap and restore ordering.
/// Returns None if the heap is empty.
/// Time complexity: O(log n)
///
/// # Arguments
/// * `h` - The heap to pop from
///
/// # Returns
/// The minimum element, or None if empty
///
/// # Examples
/// ```cursed
/// let mut heap = IntHeap::from(vec![1, 3, 2]);
/// heap_slay::init(&mut heap);
/// let min = heap_slay::pop(&mut heap); // Returns Some(Value::Integer(1))
/// ```
pub fn pop<H: Interface>(h: &mut H) -> HeapResult<Option<Value>> {
    let n = h.len();
    if n == 0 {
        return Ok(None);
    }
    
    h.swap(0, n - 1);
    let result = h.pop();
    
    if h.len() > 0 {
        down(h, 0, h.len());
    }
    
    Ok(result)
}

/// Remove the element at index i from the heap and restore ordering.
/// Time complexity: O(log n)
///
/// # Arguments
/// * `h` - The heap to remove from
/// * `i` - The index of the element to remove
///
/// # Returns
/// The removed element
///
/// # Errors
/// Returns an error if the index is out of bounds
///
/// # Examples
/// ```cursed
/// let mut heap = IntHeap::from(vec![1, 3, 2, 4]);
/// heap_slay::init(&mut heap);
/// let removed = heap_slay::remove(&mut heap, 1)?; // Remove element at index 1
/// ```
pub fn remove<H: Interface>(h: &mut H, i: i32) -> HeapResult<Value> {
    let n = h.len();
    validate_index(i, n)?;
    validate_not_empty(n)?;
    
    if i == n - 1 {
        // Removing last element, just pop
        return Ok(h.pop().unwrap());
    }
    
    h.swap(i, n - 1);
    let result = h.pop().unwrap();
    
    if i < h.len() {
        // Fix the heap property at position i
        fix(h, i)?;
    }
    
    Ok(result)
}

/// Fix re-establishes the heap ordering after element i's value has changed.
/// This function should be called when an element's priority changes.
/// Time complexity: O(log n)
///
/// # Arguments
/// * `h` - The heap to fix
/// * `i` - The index of the changed element
///
/// # Errors
/// Returns an error if the index is out of bounds
///
/// # Examples
/// ```cursed
/// // After modifying an element's value
/// heap_slay::fix(&mut heap, modified_index)?;
/// ```
pub fn fix<H: Interface>(h: &mut H, i: i32) -> HeapResult<()> {
    validate_index(i, h.len())?;
    
    if !down(h, i, h.len()) {
        up(h, i);
    }
    Ok(())
}

/// Check if a heap is properly ordered.
/// This function verifies that the heap property is satisfied.
/// Time complexity: O(n)
///
/// # Arguments
/// * `h` - The heap to check
///
/// # Returns
/// True if the heap property is satisfied, false otherwise
///
/// # Examples
/// ```cursed
/// let heap = IntHeap::from(vec![1, 2, 3, 4]);
/// assert!(heap_slay::is_heap(&heap));
/// ```
pub fn is_heap<H: Interface>(h: &H) -> bool {
    let n = h.len();
    for i in 0..n {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        
        // Check left child
        if left < n && h.less(left, i) {
            return false;
        }
        
        // Check right child
        if right < n && h.less(right, i) {
            return false;
        }
    }
    true
}

/// Move element at index i up the heap until heap property is satisfied.
/// Time complexity: O(log n)
fn up<H: Interface>(h: &mut H, mut i: i32) {
    while i > 0 {
        let parent = (i - 1) / 2;
        if !h.less(i, parent) {
            break;
        }
        h.swap(i, parent);
        i = parent;
    }
}

/// Move element at index i down the heap until heap property is satisfied.
/// Returns true if any swaps were made.
/// Time complexity: O(log n)
fn down<H: Interface>(h: &mut H, mut i: i32, n: i32) -> bool {
    let mut swapped = false;
    
    loop {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut smallest = i;
        
        // Find the smallest among parent, left child, right child
        if left < n && h.less(left, smallest) {
            smallest = left;
        }
        if right < n && h.less(right, smallest) {
            smallest = right;
        }
        
        if smallest == i {
            break;
        }
        
        h.swap(i, smallest);
        swapped = true;
        i = smallest;
    }
    
    swapped
}

