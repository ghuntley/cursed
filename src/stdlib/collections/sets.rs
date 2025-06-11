/// Comprehensive Set implementations for CURSED
/// 
/// This module provides three types of sets:
/// - HashSet<T>: Fast hash-based set with O(1) average operations
/// - TreeSet<T>: Ordered set using balanced binary tree with O(log n) operations  
/// - BitSet: Space-efficient set for small integers with O(1) operations

use super::{CollectionsError, CollectionsResult};
use crate::stdlib::collections::iterators_simple::{SimpleIterator, SimpleIntoIterator, VecIterator};
use std::collections::{HashMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Display};
use std::iter::{Iterator, FromIterator};
use std::ops::{BitAnd, BitOr, BitXor, Sub};

/// Hash-based set implementation for fast operations
#[derive(Debug, Clone)]
pub struct HashSet<T> {
    inner: std::collections::HashSet<T>,
}

/// Ordered set implementation using balanced binary tree
#[derive(Debug, Clone)]
pub struct TreeSet<T> {
    inner: BTreeSet<T>,
}

/// Bit-based set for efficient storage of small integers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitSet {
    bits: Vec<u64>,
    max_bits: usize,
}

// ==================== HashSet Implementation ====================

impl<T> HashSet<T>
where
    T: Hash + Eq + Clone,
{
    /// Create a new empty HashSet
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashSet::new(),
        }
    }

    /// Create a new HashSet with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: std::collections::HashSet::with_capacity(capacity),
        }
    }

    /// Insert an element into the set
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }

    /// Remove an element from the set
    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    }

    /// Check if the set contains an element
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }

    /// Get the number of elements in the set
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear all elements from the set
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Get the capacity of the set
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

    /// Union with another set
    pub fn union(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = self.clone();
        for item in &other.inner {
            result.insert(item.clone());
        }
        result
    }

    /// Intersection with another set
    pub fn intersection(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = HashSet::new();
        for item in &self.inner {
            if other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    /// Difference from another set
    pub fn difference(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = HashSet::new();
        for item in &self.inner {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    /// Symmetric difference with another set
    pub fn symmetric_difference(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = HashSet::new();
        
        // Add elements from self that are not in other
        for item in &self.inner {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        
        // Add elements from other that are not in self
        for item in &other.inner {
            if !self.contains(item) {
                result.insert(item.clone());
            }
        }
        
        result
    }

    /// Check if this set is a subset of another
    pub fn is_subset(&self, other: &HashSet<T>) -> bool {
        self.inner.iter().all(|item| other.contains(item))
    }

    /// Check if this set is a superset of another
    pub fn is_superset(&self, other: &HashSet<T>) -> bool {
        other.is_subset(self)
    }

    /// Check if sets are disjoint (no common elements)
    pub fn is_disjoint(&self, other: &HashSet<T>) -> bool {
        self.inner.iter().all(|item| !other.contains(item))
    }

    /// Convert to vector
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().cloned().collect()
    }

    /// Create iterator over elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}

impl<T> Default for HashSet<T>
where
    T: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for HashSet<T>
where
    T: Hash + Eq + Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            inner: std::collections::HashSet::from_iter(iter),
        }
    }
}

// ==================== TreeSet Implementation ====================

impl<T> TreeSet<T>
where
    T: Ord + Clone,
{
    /// Create a new empty TreeSet
    pub fn new() -> Self {
        Self {
            inner: BTreeSet::new(),
        }
    }

    /// Insert an element into the set
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }

    /// Remove an element from the set
    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    }

    /// Check if the set contains an element
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }

    /// Get the number of elements in the set
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear all elements from the set
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Get the first (smallest) element
    pub fn first(&self) -> Option<&T> {
        self.inner.first()
    }

    /// Get the last (largest) element
    pub fn last(&self) -> Option<&T> {
        self.inner.last()
    }

    /// Remove and return the first element
    pub fn pop_first(&mut self) -> Option<T> {
        self.inner.pop_first()
    }

    /// Remove and return the last element
    pub fn pop_last(&mut self) -> Option<T> {
        self.inner.pop_last()
    }

    /// Union with another set
    pub fn union(&self, other: &TreeSet<T>) -> TreeSet<T> {
        let mut result = self.clone();
        for item in &other.inner {
            result.insert(item.clone());
        }
        result
    }

    /// Intersection with another set
    pub fn intersection(&self, other: &TreeSet<T>) -> TreeSet<T> {
        let mut result = TreeSet::new();
        for item in &self.inner {
            if other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    /// Difference from another set
    pub fn difference(&self, other: &TreeSet<T>) -> TreeSet<T> {
        let mut result = TreeSet::new();
        for item in &self.inner {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    /// Symmetric difference with another set
    pub fn symmetric_difference(&self, other: &TreeSet<T>) -> TreeSet<T> {
        let mut result = TreeSet::new();
        
        // Add elements from self that are not in other
        for item in &self.inner {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        
        // Add elements from other that are not in self
        for item in &other.inner {
            if !self.contains(item) {
                result.insert(item.clone());
            }
        }
        
        result
    }

    /// Check if this set is a subset of another
    pub fn is_subset(&self, other: &TreeSet<T>) -> bool {
        self.inner.iter().all(|item| other.contains(item))
    }

    /// Check if this set is a superset of another
    pub fn is_superset(&self, other: &TreeSet<T>) -> bool {
        other.is_subset(self)
    }

    /// Check if sets are disjoint (no common elements)
    pub fn is_disjoint(&self, other: &TreeSet<T>) -> bool {
        self.inner.iter().all(|item| !other.contains(item))
    }

    /// Convert to vector (preserves order)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().cloned().collect()
    }

    /// Create iterator over elements (in order)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    /// Get range of elements
    pub fn range<R>(&self, range: R) -> impl Iterator<Item = &T>
    where
        R: std::ops::RangeBounds<T>,
    {
        self.inner.range(range)
    }
}

impl<T> Default for TreeSet<T>
where
    T: Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for TreeSet<T>
where
    T: Ord + Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            inner: BTreeSet::from_iter(iter),
        }
    }
}

// ==================== BitSet Implementation ====================

impl BitSet {
    /// Create a new BitSet with specified maximum number of bits
    pub fn new(max_bits: usize) -> Self {
        let num_chunks = (max_bits + 63) / 64;
        Self {
            bits: vec![0u64; num_chunks],
            max_bits,
        }
    }

    /// Create a BitSet with all bits set to false
    pub fn zeros(max_bits: usize) -> Self {
        Self::new(max_bits)
    }

    /// Create a BitSet with all bits set to true
    pub fn ones(max_bits: usize) -> Self {
        let mut bitset = Self::new(max_bits);
        bitset.set_all();
        bitset
    }

    /// Set a bit to true
    pub fn set(&mut self, index: usize) -> CollectionsResult<()> {
        if index >= self.max_bits {
            return Err(CollectionsError::InvalidBitIndex {
                index,
                max_bits: self.max_bits,
            });
        }

        let chunk_index = index / 64;
        let bit_index = index % 64;
        self.bits[chunk_index] |= 1u64 << bit_index;
        Ok(())
    }

    /// Clear a bit (set to false)
    pub fn clear(&mut self, index: usize) -> CollectionsResult<()> {
        if index >= self.max_bits {
            return Err(CollectionsError::InvalidBitIndex {
                index,
                max_bits: self.max_bits,
            });
        }

        let chunk_index = index / 64;
        let bit_index = index % 64;
        self.bits[chunk_index] &= !(1u64 << bit_index);
        Ok(())
    }

    /// Toggle a bit
    pub fn toggle(&mut self, index: usize) -> CollectionsResult<()> {
        if index >= self.max_bits {
            return Err(CollectionsError::InvalidBitIndex {
                index,
                max_bits: self.max_bits,
            });
        }

        let chunk_index = index / 64;
        let bit_index = index % 64;
        self.bits[chunk_index] ^= 1u64 << bit_index;
        Ok(())
    }

    /// Check if a bit is set
    pub fn get(&self, index: usize) -> CollectionsResult<bool> {
        if index >= self.max_bits {
            return Err(CollectionsError::InvalidBitIndex {
                index,
                max_bits: self.max_bits,
            });
        }

        let chunk_index = index / 64;
        let bit_index = index % 64;
        Ok((self.bits[chunk_index] & (1u64 << bit_index)) != 0)
    }

    /// Insert a bit (same as set)
    pub fn insert(&mut self, index: usize) -> CollectionsResult<bool> {
        let was_set = self.get(index)?;
        self.set(index)?;
        Ok(!was_set)
    }

    /// Remove a bit (same as clear)
    pub fn remove(&mut self, index: usize) -> CollectionsResult<bool> {
        let was_set = self.get(index)?;
        self.clear(index)?;
        Ok(was_set)
    }

    /// Check if the BitSet contains a bit
    pub fn contains(&self, index: usize) -> bool {
        self.get(index).unwrap_or(false)
    }

    /// Set all bits to true
    pub fn set_all(&mut self) {
        for chunk in &mut self.bits {
            *chunk = u64::MAX;
        }
        // Clear bits beyond max_bits
        if self.max_bits % 64 != 0 {
            let last_chunk_index = self.bits.len() - 1;
            let valid_bits = self.max_bits % 64;
            let mask = (1u64 << valid_bits) - 1;
            self.bits[last_chunk_index] &= mask;
        }
    }

    /// Clear all bits
    pub fn clear_all(&mut self) {
        for chunk in &mut self.bits {
            *chunk = 0;
        }
    }

    /// Count the number of set bits
    pub fn count(&self) -> usize {
        self.bits.iter().map(|chunk| chunk.count_ones() as usize).sum()
    }

    /// Count the number of unset bits
    pub fn count_zeros(&self) -> usize {
        self.max_bits - self.count()
    }

    /// Check if any bit is set
    pub fn any(&self) -> bool {
        self.bits.iter().any(|&chunk| chunk != 0)
    }

    /// Check if all bits are set
    pub fn all(&self) -> bool {
        self.count() == self.max_bits
    }

    /// Check if no bits are set
    pub fn none(&self) -> bool {
        !self.any()
    }

    /// Get the maximum number of bits
    pub fn len(&self) -> usize {
        self.max_bits
    }

    /// Check if the BitSet is empty (has no capacity)
    pub fn is_empty(&self) -> bool {
        self.max_bits == 0
    }

    /// Union with another BitSet
    pub fn union(&self, other: &BitSet) -> CollectionsResult<BitSet> {
        if self.max_bits != other.max_bits {
            return Err(CollectionsError::TypeMismatch {
                expected: format!("BitSet with {} bits", self.max_bits),
                found: format!("BitSet with {} bits", other.max_bits),
            });
        }

        let mut result = self.clone();
        for (i, &other_chunk) in other.bits.iter().enumerate() {
            result.bits[i] |= other_chunk;
        }
        Ok(result)
    }

    /// Intersection with another BitSet
    pub fn intersection(&self, other: &BitSet) -> CollectionsResult<BitSet> {
        if self.max_bits != other.max_bits {
            return Err(CollectionsError::TypeMismatch {
                expected: format!("BitSet with {} bits", self.max_bits),
                found: format!("BitSet with {} bits", other.max_bits),
            });
        }

        let mut result = self.clone();
        for (i, &other_chunk) in other.bits.iter().enumerate() {
            result.bits[i] &= other_chunk;
        }
        Ok(result)
    }

    /// Difference from another BitSet
    pub fn difference(&self, other: &BitSet) -> CollectionsResult<BitSet> {
        if self.max_bits != other.max_bits {
            return Err(CollectionsError::TypeMismatch {
                expected: format!("BitSet with {} bits", self.max_bits),
                found: format!("BitSet with {} bits", other.max_bits),
            });
        }

        let mut result = self.clone();
        for (i, &other_chunk) in other.bits.iter().enumerate() {
            result.bits[i] &= !other_chunk;
        }
        Ok(result)
    }

    /// Symmetric difference with another BitSet
    pub fn symmetric_difference(&self, other: &BitSet) -> CollectionsResult<BitSet> {
        if self.max_bits != other.max_bits {
            return Err(CollectionsError::TypeMismatch {
                expected: format!("BitSet with {} bits", self.max_bits),
                found: format!("BitSet with {} bits", other.max_bits),
            });
        }

        let mut result = self.clone();
        for (i, &other_chunk) in other.bits.iter().enumerate() {
            result.bits[i] ^= other_chunk;
        }
        Ok(result)
    }

    /// Complement (flip all bits)
    pub fn complement(&self) -> BitSet {
        let mut result = self.clone();
        for chunk in &mut result.bits {
            *chunk = !*chunk;
        }
        // Clear bits beyond max_bits
        if self.max_bits % 64 != 0 {
            let last_chunk_index = result.bits.len() - 1;
            let valid_bits = self.max_bits % 64;
            let mask = (1u64 << valid_bits) - 1;
            result.bits[last_chunk_index] &= mask;
        }
        result
    }

    /// Check if this BitSet is a subset of another
    pub fn is_subset(&self, other: &BitSet) -> bool {
        if self.max_bits != other.max_bits {
            return false;
        }
        
        for (i, &self_chunk) in self.bits.iter().enumerate() {
            if (self_chunk & other.bits[i]) != self_chunk {
                return false;
            }
        }
        true
    }

    /// Check if this BitSet is a superset of another
    pub fn is_superset(&self, other: &BitSet) -> bool {
        other.is_subset(self)
    }

    /// Check if BitSets are disjoint (no common set bits)
    pub fn is_disjoint(&self, other: &BitSet) -> bool {
        if self.max_bits != other.max_bits {
            return true;
        }
        
        for (i, &self_chunk) in self.bits.iter().enumerate() {
            if (self_chunk & other.bits[i]) != 0 {
                return false;
            }
        }
        true
    }

    /// Convert to vector of set bit indices
    pub fn to_vec(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for (chunk_index, &chunk) in self.bits.iter().enumerate() {
            let mut temp_chunk = chunk;
            let base_index = chunk_index * 64;
            
            while temp_chunk != 0 {
                let bit_index = temp_chunk.trailing_zeros() as usize;
                let actual_index = base_index + bit_index;
                if actual_index < self.max_bits {
                    result.push(actual_index);
                }
                temp_chunk &= temp_chunk - 1; // Clear the lowest set bit
            }
        }
        result
    }

    /// Create iterator over set bit indices
    pub fn iter(&self) -> BitSetIterator {
        BitSetIterator {
            bitset: self,
            current_index: 0,
        }
    }
}

/// Iterator for BitSet that yields set bit indices
pub struct BitSetIterator<'a> {
    bitset: &'a BitSet,
    current_index: usize,
}

impl<'a> Iterator for BitSetIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.bitset.max_bits {
            if self.bitset.get(self.current_index).unwrap_or(false) {
                let result = self.current_index;
                self.current_index += 1;
                return Some(result);
            }
            self.current_index += 1;
        }
        None
    }
}

// ==================== Convenience Functions ====================

/// Create a new HashSet from a vector
pub fn hash_set_from_vec<T>(vec: Vec<T>) -> HashSet<T>
where
    T: Hash + Eq + Clone,
{
    HashSet::from_iter(vec)
}

/// Create a new TreeSet from a vector
pub fn tree_set_from_vec<T>(vec: Vec<T>) -> TreeSet<T>
where
    T: Ord + Clone,
{
    TreeSet::from_iter(vec)
}

/// Create a new BitSet from a vector of indices
pub fn bit_set_from_vec(indices: Vec<usize>, max_bits: usize) -> CollectionsResult<BitSet> {
    let mut bitset = BitSet::new(max_bits);
    for index in indices {
        bitset.set(index)?;
    }
    Ok(bitset)
}

/// Union multiple HashSets
pub fn hash_set_union_multiple<T>(sets: Vec<&HashSet<T>>) -> HashSet<T>
where
    T: Hash + Eq + Clone,
{
    let mut result = HashSet::new();
    for set in sets {
        for item in set.iter() {
            result.insert(item.clone());
        }
    }
    result
}

/// Intersection of multiple HashSets
pub fn hash_set_intersection_multiple<T>(sets: Vec<&HashSet<T>>) -> HashSet<T>
where
    T: Hash + Eq + Clone,
{
    if sets.is_empty() {
        return HashSet::new();
    }

    let mut result = sets[0].clone();
    for set in &sets[1..] {
        result = result.intersection(set);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_set_basic_operations() {
        let mut set = HashSet::new();
        
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        
        assert!(set.insert("hello".to_string()));
        assert!(!set.insert("hello".to_string())); // Already exists
        assert!(set.insert("world".to_string()));
        
        assert_eq!(set.len(), 2);
        assert!(!set.is_empty());
        assert!(set.contains(&"hello".to_string()));
        assert!(set.contains(&"world".to_string()));
        assert!(!set.contains(&"foo".to_string()));
        
        assert!(set.remove(&"hello".to_string()));
        assert!(!set.remove(&"hello".to_string())); // Already removed
        assert_eq!(set.len(), 1);
        
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn test_tree_set_ordered_operations() {
        let mut set = TreeSet::new();
        
        set.insert(3);
        set.insert(1);
        set.insert(4);
        set.insert(1); // Duplicate
        set.insert(5);
        set.insert(9);
        
        assert_eq!(set.len(), 5);
        assert_eq!(set.first(), Some(&1));
        assert_eq!(set.last(), Some(&9));
        
        let ordered: Vec<_> = set.iter().cloned().collect();
        assert_eq!(ordered, vec![1, 3, 4, 5, 9]);
        
        assert_eq!(set.pop_first(), Some(1));
        assert_eq!(set.pop_last(), Some(9));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_bit_set_operations() {
        let mut bitset = BitSet::new(100);
        
        assert!(bitset.is_empty() == false); // Has capacity but no set bits
        assert_eq!(bitset.len(), 100);
        assert_eq!(bitset.count(), 0);
        assert!(bitset.none());
        assert!(!bitset.any());
        assert!(!bitset.all());
        
        bitset.set(5).unwrap();
        bitset.set(10).unwrap();
        bitset.set(15).unwrap();
        
        assert_eq!(bitset.count(), 3);
        assert!(bitset.any());
        assert!(!bitset.all());
        assert!(!bitset.none());
        
        assert!(bitset.get(5).unwrap());
        assert!(bitset.get(10).unwrap());
        assert!(bitset.get(15).unwrap());
        assert!(!bitset.get(20).unwrap());
        
        let indices: Vec<_> = bitset.iter().collect();
        assert_eq!(indices, vec![5, 10, 15]);
        
        bitset.clear(10).unwrap();
        assert!(!bitset.get(10).unwrap());
        assert_eq!(bitset.count(), 2);
    }

    #[test]
    fn test_set_operations() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        set1.insert(3);
        
        let mut set2 = HashSet::new();
        set2.insert(3);
        set2.insert(4);
        set2.insert(5);
        
        let union = set1.union(&set2);
        let union_vec = union.to_vec();
        assert_eq!(union_vec.len(), 5);
        
        let intersection = set1.intersection(&set2);
        let intersection_vec = intersection.to_vec();
        assert_eq!(intersection_vec, vec![3]);
        
        let difference = set1.difference(&set2);
        let mut diff_vec = difference.to_vec();
        diff_vec.sort();
        assert_eq!(diff_vec, vec![1, 2]);
        
        assert!(!set1.is_subset(&set2));
        assert!(!set1.is_superset(&set2));
        assert!(!set1.is_disjoint(&set2));
    }

    #[test]
    fn test_bit_set_set_operations() {
        let mut set1 = BitSet::new(64);
        set1.set(1).unwrap();
        set1.set(3).unwrap();
        set1.set(5).unwrap();
        
        let mut set2 = BitSet::new(64);
        set2.set(3).unwrap();
        set2.set(5).unwrap();
        set2.set(7).unwrap();
        
        let union = set1.union(&set2).unwrap();
        let union_indices: Vec<_> = union.iter().collect();
        assert_eq!(union_indices, vec![1, 3, 5, 7]);
        
        let intersection = set1.intersection(&set2).unwrap();
        let intersection_indices: Vec<_> = intersection.iter().collect();
        assert_eq!(intersection_indices, vec![3, 5]);
        
        let difference = set1.difference(&set2).unwrap();
        let difference_indices: Vec<_> = difference.iter().collect();
        assert_eq!(difference_indices, vec![1]);
        
        assert!(intersection.is_subset(&set1));
        assert!(set1.is_superset(&intersection));
    }

    #[test]
    fn test_bit_set_error_handling() {
        let mut bitset = BitSet::new(10);
        
        // Index out of bounds
        assert!(bitset.set(10).is_err());
        assert!(bitset.get(15).is_err());
        assert!(bitset.clear(20).is_err());
        
        // Valid operations
        assert!(bitset.set(9).is_ok());
        assert!(bitset.get(9).is_ok());
        assert!(bitset.clear(9).is_ok());
    }

    #[test]
    fn test_convenience_functions() {
        let vec = vec![1, 2, 3, 2, 1]; // Duplicates
        let hash_set = hash_set_from_vec(vec.clone());
        assert_eq!(hash_set.len(), 3);
        
        let tree_set = tree_set_from_vec(vec);
        assert_eq!(tree_set.len(), 3);
        assert_eq!(tree_set.to_vec(), vec![1, 2, 3]);
        
        let bit_set = bit_set_from_vec(vec![1, 3, 5], 10).unwrap();
        assert_eq!(bit_set.count(), 3);
        assert!(bit_set.contains(1));
        assert!(bit_set.contains(3));
        assert!(bit_set.contains(5));
    }

    #[test]
    fn test_into_iterator() {
        let mut hash_set = HashSet::new();
        hash_set.insert(1);
        hash_set.insert(2);
        hash_set.insert(3);
        
        let items: Vec<i32> = hash_set.simple_into_iter().collect();
        assert_eq!(items.len(), 3);
        assert!(items.contains(&1));
        assert!(items.contains(&2));
        assert!(items.contains(&3));
        
        let mut tree_set = TreeSet::new();
        tree_set.insert(3);
        tree_set.insert(1);
        tree_set.insert(2);
        
        let items: Vec<i32> = tree_set.simple_into_iter().collect();
        assert_eq!(items, vec![1, 2, 3]); // TreeSet maintains order
        
        let mut bit_set = BitSet::new(10);
        bit_set.set(1).unwrap();
        bit_set.set(3).unwrap();
        bit_set.set(5).unwrap();
        
        let items: Vec<usize> = bit_set.simple_into_iter().collect();
        assert_eq!(items, vec![1, 3, 5]);
    }
}

// ==================== IntoIterator Implementations ====================

impl<T> SimpleIntoIterator<T> for HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Iterator = VecIterator<T>;
    
    fn simple_into_iter(self) -> Self::Iterator {
        VecIterator::new(self.to_vec())
    }
}

impl<T> SimpleIntoIterator<T> for TreeSet<T>
where
    T: Ord + Clone,
{
    type Iterator = VecIterator<T>;
    
    fn simple_into_iter(self) -> Self::Iterator {
        VecIterator::new(self.to_vec())
    }
}

impl SimpleIntoIterator<usize> for BitSet {
    type Iterator = VecIterator<usize>;
    
    fn simple_into_iter(self) -> Self::Iterator {
        let mut items = Vec::new();
        for i in 0..self.max_bits {
            if self.contains(i) {
                items.push(i);
            }
        }
        VecIterator::new(items)
    }
}
