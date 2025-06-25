/// SortaFresh - A fresh approach to sorting in CURSED
/// 
/// This module provides comprehensive sorting functionality including:
/// - Core interfaces (Sortable, SortableSearch)
/// - Basic sorting algorithms (Sort, Reverse, IsSorted, Stable, Shuffle)
/// - Specialized sorting for common types (ints, floats, strings)
/// - Generic sorting functions with custom comparators
/// - Binary search functionality
/// - Advanced sorting features

use std::cmp::Ordering;
use std::fmt::Debug;
use crate::error::CursedError;

pub mod core;
pub mod specialized;
pub mod search;

pub use core::*;
pub use specialized::*;
pub use search::*;

/// Result type for SortaFresh operations  
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Core interface for sortable collections
/// Equivalent to `Sortable` interface in the spec
pub trait Sortable {
    /// Returns the number of elements in the collection
    fn len(&self) -> i32;
    
    /// Reports whether element i should sort before element j
    fn less(&self, i: i32, j: i32) -> bool;
    
    /// Swaps elements i and j
    fn swap(&mut self, i: i32, j: i32);
    
    /// Helper method to check if collection is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Extended interface that adds search capability to Sortable
/// Equivalent to `SortableSearch` interface in the spec
pub trait SortableSearch: Sortable {
    /// Returns the smallest index i such that a[i] >= value
    /// Returns the length if no such index exists
    fn search(&self, value: &dyn std::any::Any) -> i32;
/// Implementation of Sortable for Vec<T> where T: Ord
impl<T: Ord> Sortable for Vec<T> {
    fn len(&self) -> i32 {
        self.len() as i32
    fn less(&self, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 || i >= self.len() as i32 || j >= self.len() as i32 {
            false
        } else {
            self[i as usize] < self[j as usize]
        }
    }
    
    fn swap(&mut self, i: i32, j: i32) {
        if i >= 0 && j >= 0 && i < self.len() as i32 && j < self.len() as i32 {
            self.swap(i as usize, j as usize);
        }
    }
/// Implementation of Sortable for slices
impl<T: Ord> Sortable for [T] {
    fn len(&self) -> i32 {
        self.len() as i32
    fn less(&self, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 || i >= self.len() as i32 || j >= self.len() as i32 {
            false
        } else {
            self[i as usize] < self[j as usize]
        }
    }
    
    fn swap(&mut self, i: i32, j: i32) {
        if i >= 0 && j >= 0 && i < self.len() as i32 && j < self.len() as i32 {
            self.swap(i as usize, j as usize);
        }
    }
/// Wrapper for custom comparison functions
pub struct CustomSortable<T, F> 
where 
{
impl<T, F> CustomSortable<T, F> 
where 
{
    pub fn new(mut data: Vec<T>, less_fn: F) -> Self {
        Self { data, less_fn }
    }
    
    pub fn into_vec(self) -> Vec<T> {
        self.data
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
}

impl<T, F> Sortable for CustomSortable<T, F> 
where 
{
    fn len(&self) -> i32 {
        self.data.len() as i32
    fn less(&self, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 || i >= self.data.len() as i32 || j >= self.data.len() as i32 {
            false
        } else {
            (self.less_fn)(&self.data[i as usize], &self.data[j as usize])
        }
    }
    
    fn swap(&mut self, i: i32, j: i32) {
        if i >= 0 && j >= 0 && i < self.data.len() as i32 && j < self.data.len() as i32 {
            self.data.swap(i as usize, j as usize);
        }
    }
/// Wrapper for reverse sorting
pub struct ReverseSortable<T: Sortable> {
impl<T: Sortable> ReverseSortable<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: Sortable> Sortable for ReverseSortable<T> {
    fn len(&self) -> i32 {
        self.inner.len()
    fn less(&self, i: i32, j: i32) -> bool {
        self.inner.less(j, i) // Reverse the comparison
    fn swap(&mut self, i: i32, j: i32) {
        self.inner.swap(i, j)
    }
}

/// Helper function to validate indices
pub(crate) fn validate_indices(len: i32, i: i32, j: i32) -> bool {
    i >= 0 && j >= 0 && i < len && j < len
/// Helper function to convert usize to i32 safely
pub(crate) fn usize_to_i32(val: usize) -> i32 {
    if val > i32::MAX as usize {
        i32::MAX
    } else {
        val as i32
    }
}

/// Helper function to convert i32 to usize safely
pub(crate) fn i32_to_usize(val: i32) -> Option<usize> {
    if val >= 0 {
        Some(val as usize)
    } else {
        None
    }
}

