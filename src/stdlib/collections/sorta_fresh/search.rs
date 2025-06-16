/// Search functions for SortaFresh
/// 
/// This module provides binary search functionality for sorted data:
/// - Generic Search function
/// - Specialized search functions for common types
/// - Insertion point finding
/// - Advanced search utilities

use super::SortaFreshResult;
use crate::stdlib::collections::CollectionsError;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_basic() {
        let result = search(5, |i| i >= 3);
        assert_eq!(result, 3);
        
        let result = search(5, |i| i >= 0);
        assert_eq!(result, 0);
        
        let result = search(5, |i| i >= 10);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_search_ints() {
        let data = vec![1, 3, 5, 7, 9, 11];
        
        assert_eq!(search_ints(&data, 5), 2); // Exact match
        assert_eq!(search_ints(&data, 6), 3); // Should insert at position 3
        assert_eq!(search_ints(&data, 0), 0); // Before all elements
        assert_eq!(search_ints(&data, 15), 6); // After all elements
        
        // Empty slice
        assert_eq!(search_ints(&[], 5), 0);
    }

    #[test]
    fn test_search_float64s() {
        let data = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        
        assert_eq!(search_float64s(&data, 3.3), 2); // Exact match
        assert_eq!(search_float64s(&data, 3.5), 3); // Should insert at position 3
        assert_eq!(search_float64s(&data, 0.5), 0); // Before all elements
        assert_eq!(search_float64s(&data, 6.0), 5); // After all elements
        
        // Test with NaN
        let data_with_nan = vec![1.0, 2.0, f64::NAN, f64::NAN];
        assert_eq!(search_float64s(&data_with_nan, f64::NAN), 4); // NaN input returns end
        assert_eq!(search_float64s(&data_with_nan, 1.5), 1); // Normal search still works
    }

    #[test]
    fn test_search_strings() {
        let data = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
        ];
        
        assert_eq!(search_strings(&data, "banana"), 1); // Exact match
        assert_eq!(search_strings(&data, "blueberry"), 2); // Should insert at position 2
        assert_eq!(search_strings(&data, "aardvark"), 0); // Before all elements
        assert_eq!(search_strings(&data, "zebra"), 4); // After all elements
    }

    #[test]
    fn test_binary_search() {
        let data = vec![1, 3, 5, 7, 9, 11];
        
        let (index, found) = binary_search(&data, &5, |a, b| (*a).cmp(b));
        assert_eq!(index, 2);
        assert!(found);
        
        let (index, found) = binary_search(&data, &6, |a, b| (*a).cmp(b));
        assert_eq!(index, 3);
        assert!(!found);
        
        // Empty slice
        let (index, found) = binary_search(&[], &5, |a, b| (*a).cmp(b));
        assert_eq!(index, 0);
        assert!(!found);
    }

    #[test]
    fn test_insertion_point() {
        let data = vec![1, 3, 5, 7, 9, 11];
        
        assert_eq!(insertion_point(&data, &5, |a, b| (*a).cmp(b)), 2); // Exact match position
        assert_eq!(insertion_point(&data, &6, |a, b| (*a).cmp(b)), 3); // Insert position
        assert_eq!(insertion_point(&data, &0, |a, b| (*a).cmp(b)), 0); // Before all
        assert_eq!(insertion_point(&data, &15, |a, b| (*a).cmp(b)), 6); // After all
    }

    #[test]
    fn test_binary_search_by_key() {
        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            age: i32,
        }
        
        let people = vec![
            Person { name: "Alice".to_string(), age: 20 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Charlie".to_string(), age: 30 },
        ];
        
        let (index, found) = binary_search_by_key(&people, &25, |p| p.age);
        assert_eq!(index, 1);
        assert!(found);
        
        let (index, found) = binary_search_by_key(&people, &27, |p| p.age);
        assert_eq!(index, 2);
        assert!(!found);
    }

    #[test]
    fn test_equal_range() {
        let data = vec![1, 2, 2, 2, 3, 4, 5];
        let (lower, upper) = equal_range(&data, &2, |a, b| (*a).cmp(b));
        
        assert_eq!(lower, 1); // First occurrence of 2
        assert_eq!(upper, 4); // First element after 2s
        
        // Test with non-existent element
        let (lower, upper) = equal_range(&data, &6, |a, b| (*a).cmp(b));
        assert_eq!(lower, upper); // Should be the same (insertion point)
    }

    #[test]
    fn test_search_first() {
        let data = vec![1, 3, 5, 7, 9, 11];
        
        assert_eq!(search_first(&data, |&x| x > 5), Some(3)); // First element > 5 is at index 3
        assert_eq!(search_first(&data, |&x| x > 15), None); // No element > 15
        assert_eq!(search_first(&data, |&x| x >= 1), Some(0)); // First element >= 1 is at index 0
    }

    #[test]
    fn test_search_last() {
        let data = vec![1, 3, 5, 7, 9, 11];
        
        assert_eq!(search_last(&data, |&x| x < 9), Some(2)); // Last element < 9 is at index 2
        assert_eq!(search_last(&data, |&x| x < 0), None); // No element < 0
        assert_eq!(search_last(&data, |&x| x <= 11), Some(5)); // Last element <= 11 is at index 5
    }
}
