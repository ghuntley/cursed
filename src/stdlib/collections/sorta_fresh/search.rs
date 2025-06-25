use crate::error::CursedError;
/// Search functions for SortaFresh
/// 
/// This module provides binary search functionality for sorted data:
/// - Generic Search function
/// - Specialized search functions for common types
/// - Insertion point finding
/// - Advanced search utilities

use super::SortaFreshResult;
use std::cmp::Ordering;

/// Uses binary search to find the index in a sorted data structure
/// slay Search(n int, f func(normie) lit) int
pub fn search<F>(n: i32, f: F) -> i32
where
    F: Fn(i32) -> bool,
{
    if n <= 0 {
        return 0;
    }
    
    let mut left = 0i32;
    let mut right = n;
    
    // Binary search for the first index where f(index) returns true
    while left < right {
        let mid = left + (right - left) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    
    left
}

/// Searches for x in a sorted slice of ints and returns the index
/// slay SearchInts(a []int, x normie) int
pub fn search_ints(a: &[i32], x: i32) -> i32 {
    if a.is_empty() {
        return 0;
    }
    
    let len = a.len() as i32;
    search(len, |i| {
        if let Some(idx) = i32_to_usize(i) {
            if idx < a.len() {
                return a[idx] >= x;
            }
        }
        true
    })
}

/// Searches for x in a sorted slice of float64s and returns the index
/// slay SearchFloat64s(a []float64, x float64) int
pub fn search_float64s(a: &[f64], x: f64) -> i32 {
    if a.is_empty() {
        return 0;
    }
    
    // Handle NaN input
    if x.is_nan() {
        return a.len() as i32; // NaN should be at the end
    }
    
    let len = a.len() as i32;
    search(len, |i| {
        if let Some(idx) = i32_to_usize(i) {
            if idx < a.len() {
                let val = a[idx];
                if val.is_nan() {
                    return true; // NaN values are at the end, so they're >= x
                }
                return val >= x;
            }
        }
        true
    })
}

/// Searches for x in a sorted slice of strings and returns the index
/// slay SearchStrings(a []tea, x tea) int
pub fn search_strings(a: &[String], x: &str) -> i32 {
    if a.is_empty() {
        return 0;
    }
    
    let len = a.len() as i32;
    search(len, |i| {
        if let Some(idx) = i32_to_usize(i) {
            if idx < a.len() {
                return a[idx].as_str() >= x;
            }
        }
        true
    })
}

/// Searches for x in a sorted slice of string slices and returns the index
pub fn search_str_slices(a: &[&str], x: &str) -> i32 {
    if a.is_empty() {
        return 0;
    }
    
    let len = a.len() as i32;
    search(len, |i| {
        if let Some(idx) = i32_to_usize(i) {
            if idx < a.len() {
                return a[idx] >= x;
            }
        }
        true
    })
}

/// Performs a binary search for value in sorted data
/// slay BinarySearch[T any](data []T, value T, cmp func(a, b T) normie) (index int, found lit)
pub fn binary_search<T, F>(data: &[T], value: &T, cmp: F) -> (i32, bool)
where
    F: Fn(&T, &T) -> i32,
{
    if data.is_empty() {
        return (0, false);
    }
    
    let mut left = 0;
    let mut right = data.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match cmp(&data[mid], value) {
            x if x < 0 => left = mid + 1,
            x if x > 0 => right = mid,
            _ => return (mid as i32, true), // Found exact match
        }
    }
    
    (left as i32, false)
}

/// Returns where value should be inserted to maintain order
/// slay InsertionPoint[T any](data []T, value T, cmp func(a, b T) normie) int
pub fn insertion_point<T, F>(data: &[T], value: &T, cmp: F) -> i32
where
    F: Fn(&T, &T) -> i32,
{
    if data.is_empty() {
        return 0;
    }
    
    let mut left = 0;
    let mut right = data.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if cmp(&data[mid], value) < 0 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    left as i32
}

/// Binary search with a key extraction function
pub fn binary_search_by_key<T, K, F>(data: &[T], key: &K, extract_key: F) -> (i32, bool)
where
    K: Ord,
    F: Fn(&T) -> K,
{
    if data.is_empty() {
        return (0, false);
    }
    
    match data.binary_search_by_key(key, extract_key) {
        Ok(index) => (index as i32, true),
        Err(index) => (index as i32, false),
    }
}

/// Binary search with a custom comparison function
pub fn binary_search_by<T, F>(data: &[T], f: F) -> (i32, bool)
where
    F: FnMut(&T) -> Ordering,
{
    if data.is_empty() {
        return (0, false);
    }
    
    match data.binary_search_by(f) {
        Ok(index) => (index as i32, true),
        Err(index) => (index as i32, false),
    }
}

/// Find the range of equal elements in sorted data
pub fn equal_range<T, F>(data: &[T], value: &T, cmp: F) -> (i32, i32)
where
    F: Fn(&T, &T) -> i32 + Clone,
{
    if data.is_empty() {
        return (0, 0);
    }
    
    // Find first occurrence
    let lower = insertion_point(data, value, cmp.clone());
    
    // Find first element greater than value
    let upper_cmp = |a: &T, b: &T| {
        let result = cmp(a, b);
        if result <= 0 { -1 } else { 1 }
    };
    
    let mut left = lower as usize;
    let mut right = data.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if upper_cmp(&data[mid], value) <= 0 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    (lower, left as i32)
}

/// Search for the first element that satisfies a predicate
pub fn search_first<T, P>(data: &[T], predicate: P) -> Option<i32>
where
    P: Fn(&T) -> bool,
{
    for (i, item) in data.iter().enumerate() {
        if predicate(item) {
            return Some(i as i32);
        }
    }
    None
}

/// Search for the last element that satisfies a predicate
pub fn search_last<T, P>(data: &[T], predicate: P) -> Option<i32>
where
    P: Fn(&T) -> bool,
{
    for (i, item) in data.iter().enumerate().rev() {
        if predicate(item) {
            return Some(i as i32);
        }
    }
    None
}

/// Helper function to convert i32 to usize safely
fn i32_to_usize(val: i32) -> Option<usize> {
    if val >= 0 {
        Some(val as usize)
    } else {
        None
    }
}

