use crate::error::CursedError;
/// Comprehensive Set implementations for CURSED
/// 
/// This module provides three types of sets:
/// - HashSet<T>: Fast hash-based set with O(1) average operations
/// - TreeSet<T>: Ordered set using balanced binary tree with O(log n) operations  
/// - BitSet: Space-efficient set for small integers with O(1) operations

use super::{CollectionsError, CollectionsResult};
// use crate::stdlib::collections::iterators_simple::{SimpleIterator, SimpleIntoIterator, VecIterator};
use std::collections::{HashMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Display};
use std::iter::{Iterator, FromIterator};
use std::ops::{BitAnd, BitOr, BitXor, Sub};

/// Hash-based set implementation for fast operations
#[derive(Debug, Clone)]
pub struct HashSet<T> {
/// Ordered set implementation using balanced binary tree
#[derive(Debug, Clone)]
pub struct TreeSet<T> {
/// Bit-based set for efficient storage of small integers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitSet {
// ==================== HashSet Implementation ====================

impl<T> HashSet<T>
where
{
    /// Create a new empty HashSet
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new HashSet with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
        }
    }

    /// Insert an element into the set
    pub fn insert(&mut self, value: T) -> CollectionsResult<bool> {
        Ok(self.inner.insert(value))
    /// Remove an element from the set
    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    /// Check if the set contains an element
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    /// Get the number of elements in the set
    pub fn len(&self) -> usize {
        self.inner.len()
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }
    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    /// Clear all elements from the set
    pub fn clear(&mut self) {
        self.inner.clear()
    /// Get the capacity of the set
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    /// Reserve space for additional elements
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    /// Shrink the capacity to fit the current size
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    /// Union with another set
    pub fn union(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = self.clone();
        for item in &other.inner {
            result.insert(item.clone());
        }
        result
    /// Intersection with another set
    pub fn intersection(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = HashSet::new();
        for item in &self.inner {
            if other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    /// Difference from another set
    pub fn difference(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = HashSet::new();
        for item in &self.inner {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
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
    /// Check if this set is a subset of another
    pub fn is_subset(&self, other: &HashSet<T>) -> bool {
        self.inner.iter().all(|item| other.contains(item))
    /// Check if this set is a superset of another
    pub fn is_superset(&self, other: &HashSet<T>) -> bool {
        other.is_subset(self)
    /// Check if sets are disjoint (no common elements)
    pub fn is_disjoint(&self, other: &HashSet<T>) -> bool {
        self.inner.iter().all(|item| !other.contains(item))
    /// Convert to vector
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().cloned().collect()
    /// Create iterator over elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}

impl<T> Default for HashSet<T>
where
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for HashSet<T>
where
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
        }
    }
// ==================== TreeSet Implementation ====================

impl<T> TreeSet<T>
where
{
    /// Create a new empty TreeSet
    pub fn new() -> Self {
        Self {
        }
    }

    /// Insert an element into the set
    pub fn insert(&mut self, value: T) -> CollectionsResult<bool> {
        Ok(self.inner.insert(value))
    /// Remove an element from the set
    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    /// Check if the set contains an element
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    /// Get the number of elements in the set
    pub fn len(&self) -> usize {
        self.inner.len()
    /// Alias for len() for compatibility
    pub fn size(&self) -> usize {
        self.len()
    }
    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    /// Clear all elements from the set
    pub fn clear(&mut self) {
        self.inner.clear()
    /// Get the first (smallest) element
    pub fn first(&self) -> Option<&T> {
        self.inner.first()
    /// Get the last (largest) element
    pub fn last(&self) -> Option<&T> {
        self.inner.last()
    /// Remove and return the first element
    pub fn pop_first(&mut self) -> Option<T> {
        self.inner.pop_first()
    /// Remove and return the last element
    pub fn pop_last(&mut self) -> Option<T> {
        self.inner.pop_last()
    /// Union with another set
    pub fn union(&self, other: &TreeSet<T>) -> TreeSet<T> {
        let mut result = self.clone();
        for item in &other.inner {
            result.insert(item.clone());
        }
        result
    /// Intersection with another set
    pub fn intersection(&self, other: &TreeSet<T>) -> TreeSet<T> {
        let mut result = TreeSet::new();
        for item in &self.inner {
            if other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    /// Difference from another set
    pub fn difference(&self, other: &TreeSet<T>) -> TreeSet<T> {
        let mut result = TreeSet::new();
        for item in &self.inner {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
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
    /// Check if this set is a subset of another
    pub fn is_subset(&self, other: &TreeSet<T>) -> bool {
        self.inner.iter().all(|item| other.contains(item))
    /// Check if this set is a superset of another
    pub fn is_superset(&self, other: &TreeSet<T>) -> bool {
        other.is_subset(self)
    /// Check if sets are disjoint (no common elements)
    pub fn is_disjoint(&self, other: &TreeSet<T>) -> bool {
        self.inner.iter().all(|item| !other.contains(item))
    /// Convert to vector (preserves order)
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().cloned().collect()
    /// Create iterator over elements (in order)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    /// Get range of elements
    pub fn range<R>(&self, range: R) -> impl Iterator<Item = &T>
    where
    {
        self.inner.range(range)
    }
}

impl<T> Default for TreeSet<T>
where
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for TreeSet<T>
where
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
        }
    }
// ==================== BitSet Implementation ====================

impl BitSet {
    /// Create a new BitSet with specified maximum number of bits
    pub fn new(max_bits: usize) -> Self {
        let num_chunks = (max_bits + 63) / 64;
        Self {
        }
    }

    /// Create a BitSet with all bits set to false
    pub fn zeros(max_bits: usize) -> Self {
        Self::new(max_bits)
    /// Create a BitSet with all bits set to true
    pub fn ones(max_bits: usize) -> Self {
        let mut bitset = Self::new(max_bits);
        bitset.set_all();
        bitset
    /// Set a bit to true
    pub fn set(&mut self, index: usize) -> CollectionsResult<()> {
        if index >= self.max_bits {
            return Err(CollectionsError::InvalidBitIndex {
            });
        let chunk_index = index / 64;
        let bit_index = index % 64;
        self.bits[chunk_index] |= 1u64 << bit_index;
        Ok(())
    /// Clear a bit (set to false)
    pub fn clear(&mut self, index: usize) -> CollectionsResult<()> {
        if index >= self.max_bits {
            return Err(CollectionsError::InvalidBitIndex {
            });
        let chunk_index = index / 64;
        let bit_index = index % 64;
        self.bits[chunk_index] &= !(1u64 << bit_index);
        Ok(())
    /// Toggle a bit
    pub fn toggle(&mut self, index: usize) -> CollectionsResult<()> {
        if index >= self.max_bits {
            return Err(CollectionsError::InvalidBitIndex {
            });
        let chunk_index = index / 64;
        let bit_index = index % 64;
        self.bits[chunk_index] ^= 1u64 << bit_index;
        Ok(())
    /// Check if a bit is set
    pub fn get(&self, index: usize) -> CollectionsResult<bool> {
        if index >= self.max_bits {
            return Err(CollectionsError::InvalidBitIndex {
            });
        let chunk_index = index / 64;
        let bit_index = index % 64;
        Ok((self.bits[chunk_index] & (1u64 << bit_index)) != 0)
    /// Insert a bit (same as set)
    pub fn insert(&mut self, index: usize) -> CollectionsResult<bool> {
        let was_set = self.get(index)?;
        self.set(index)?;
        Ok(!was_set)
    /// Remove a bit (same as clear)
    pub fn remove(&mut self, index: usize) -> CollectionsResult<bool> {
        let was_set = self.get(index)?;
        self.clear(index)?;
        Ok(was_set)
    /// Check if the BitSet contains a bit
    pub fn contains(&self, index: usize) -> bool {
        self.get(index).unwrap_or(false)
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
    /// Count the number of unset bits
    pub fn count_zeros(&self) -> usize {
        self.max_bits - self.count()
    /// Check if any bit is set
    pub fn any(&self) -> bool {
        self.bits.iter().any(|&chunk| chunk != 0)
    /// Check if all bits are set
    pub fn all(&self) -> bool {
        self.count() == self.max_bits
    /// Check if no bits are set
    pub fn none(&self) -> bool {
        !self.any()
    /// Get the maximum number of bits
    pub fn len(&self) -> usize {
        self.max_bits
    /// Check if the BitSet is empty (has no capacity)
    pub fn is_empty(&self) -> bool {
        self.max_bits == 0
    /// Union with another BitSet
    pub fn union(&self, other: &BitSet) -> CollectionsResult<BitSet> {
        if self.max_bits != other.max_bits {
            return Err(CollectionsError::TypeMismatch {
            });
        let mut result = self.clone();
        for (i, &other_chunk) in other.bits.iter().enumerate() {
            result.bits[i] |= other_chunk;
        }
        Ok(result)
    /// Intersection with another BitSet
    pub fn intersection(&self, other: &BitSet) -> CollectionsResult<BitSet> {
        if self.max_bits != other.max_bits {
            return Err(CollectionsError::TypeMismatch {
            });
        let mut result = self.clone();
        for (i, &other_chunk) in other.bits.iter().enumerate() {
            result.bits[i] &= other_chunk;
        }
        Ok(result)
    /// Difference from another BitSet
    pub fn difference(&self, other: &BitSet) -> CollectionsResult<BitSet> {
        if self.max_bits != other.max_bits {
            return Err(CollectionsError::TypeMismatch {
            });
        let mut result = self.clone();
        for (i, &other_chunk) in other.bits.iter().enumerate() {
            result.bits[i] &= !other_chunk;
        }
        Ok(result)
    /// Symmetric difference with another BitSet
    pub fn symmetric_difference(&self, other: &BitSet) -> CollectionsResult<BitSet> {
        if self.max_bits != other.max_bits {
            return Err(CollectionsError::TypeMismatch {
            });
        let mut result = self.clone();
        for (i, &other_chunk) in other.bits.iter().enumerate() {
            result.bits[i] ^= other_chunk;
        }
        Ok(result)
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
    /// Check if this BitSet is a subset of another
    pub fn is_subset(&self, other: &BitSet) -> bool {
        if self.max_bits != other.max_bits {
            return false;
        for (i, &self_chunk) in self.bits.iter().enumerate() {
            if (self_chunk & other.bits[i]) != self_chunk {
                return false;
            }
        }
        true
    /// Check if this BitSet is a superset of another
    pub fn is_superset(&self, other: &BitSet) -> bool {
        other.is_subset(self)
    /// Check if BitSets are disjoint (no common set bits)
    pub fn is_disjoint(&self, other: &BitSet) -> bool {
        if self.max_bits != other.max_bits {
            return true;
        for (i, &self_chunk) in self.bits.iter().enumerate() {
            if (self_chunk & other.bits[i]) != 0 {
                return false;
            }
        }
        true
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
    /// Create iterator over set bit indices
    pub fn iter(&self) -> BitSetIterator {
        BitSetIterator {
        }
    }
/// Iterator for BitSet that yields set bit indices
pub struct BitSetIterator<'a> {
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
{
    HashSet::from_iter(vec)
/// Create a new TreeSet from a vector
pub fn tree_set_from_vec<T>(vec: Vec<T>) -> TreeSet<T>
where
{
    TreeSet::from_iter(vec)
/// Create a new BitSet from a vector of indices
pub fn bit_set_from_vec(indices: Vec<usize>, max_bits: usize) -> CollectionsResult<BitSet> {
    let mut bitset = BitSet::new(max_bits);
    for index in indices {
        bitset.set(index)?;
    }
    Ok(bitset)
/// Union multiple HashSets
pub fn hash_set_union_multiple<T>(sets: Vec<&HashSet<T>>) -> HashSet<T>
where
{
    let mut result = HashSet::new();
    for set in sets {
        for item in set.iter() {
            result.insert(item.clone());
        }
    }
    result
/// Intersection of multiple HashSets
pub fn hash_set_intersection_multiple<T>(sets: Vec<&HashSet<T>>) -> HashSet<T>
where
{
    if sets.is_empty() {
        return HashSet::new();
    let mut result = sets[0].clone();
    for set in &sets[1..] {
        result = result.intersection(set);
    }
    result

impl<T> SimpleIntoIterator<T> for TreeSet<T>
where
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
