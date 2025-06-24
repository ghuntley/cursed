use crate::error::Error;
/// Specialized sorting functions for common types
/// 
/// This module provides optimized sorting functions for:
/// - Integers (normie/i32)
/// - Floating point numbers (float64/f64)
/// - Strings (tea/String)
/// 
/// Also includes corresponding "AreSorted" check functions.

use super::SortaFreshResult;
use crate::stdlib::collections::CollectionsError;

/// Sorts a slice of ints in ascending order
/// slay SortInts(a []normie)
pub fn sort_ints(a: &mut [i32]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    a.sort_unstable();
    Ok(())
}

/// Sorts a slice of float64s in ascending order
/// slay SortFloat64s(a []float64)
pub fn sort_float64s(a: &mut [f64]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    // Handle NaN values by moving them to the end
    a.sort_by(|a, b| {
        match (a.is_nan(), b.is_nan()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
        }
    });
    
    Ok(())
}

/// Sorts a slice of strings in ascending order
/// slay SortStrings(a []tea)
pub fn sort_strings(a: &mut [String]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    a.sort_unstable();
    Ok(())
}

/// Sorts a slice of string slices in ascending order
pub fn sort_str_slices(a: &mut [&str]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    a.sort_unstable();
    Ok(())
}

/// Reports whether the slice is sorted in ascending order
/// slay IntsAreSorted(a []normie) lit
pub fn ints_are_sorted(a: &[i32]) -> bool {
    if a.len() <= 1 {
        return true;
    }
    
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
}

/// Reports whether the slice is sorted in ascending order
/// slay Float64sAreSorted(a []float64) lit
pub fn float64s_are_sorted(a: &[f64]) -> bool {
    if a.len() <= 1 {
        return true;
    }
    
    for i in 1..a.len() {
        match (a[i - 1].is_nan(), a[i].is_nan()) {
            (true, _) => continue, // NaN values are at the end, so continue
            (false, true) => return false, // Non-NaN followed by NaN is not sorted
            (false, false) => {
                if a[i] < a[i - 1] {
                    return false;
                }
            }
        }
    }
    true
}

/// Reports whether the slice is sorted in ascending order
/// slay StringsAreSorted(a []tea) lit
pub fn strings_are_sorted(a: &[String]) -> bool {
    if a.len() <= 1 {
        return true;
    }
    
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
}

/// Reports whether the slice of string slices is sorted in ascending order
pub fn str_slices_are_sorted(a: &[&str]) -> bool {
    if a.len() <= 1 {
        return true;
    }
    
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
}

/// Reverse sorts a slice of ints in descending order
pub fn reverse_sort_ints(a: &mut [i32]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    a.sort_unstable_by(|a, b| b.cmp(a));
    Ok(())
}

/// Reverse sorts a slice of float64s in descending order
pub fn reverse_sort_float64s(a: &mut [f64]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    // Handle NaN values by moving them to the end
    a.sort_by(|a, b| {
        match (a.is_nan(), b.is_nan()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal),
        }
    });
    
    Ok(())
}

/// Reverse sorts a slice of strings in descending order
pub fn reverse_sort_strings(a: &mut [String]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    a.sort_unstable_by(|a, b| b.cmp(a));
    Ok(())
}

/// Stable sorts a slice of ints in ascending order
pub fn stable_sort_ints(a: &mut [i32]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    a.sort();
    Ok(())
}

/// Stable sorts a slice of float64s in ascending order
pub fn stable_sort_float64s(a: &mut [f64]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    // Handle NaN values by moving them to the end
    a.sort_by(|a, b| {
        match (a.is_nan(), b.is_nan()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
        }
    });
    
    Ok(())
}

/// Stable sorts a slice of strings in ascending order
pub fn stable_sort_strings(a: &mut [String]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    }
    
    a.sort();
    Ok(())
}

/// Generic sort by key function
/// slay SortBy[T any, K cmp.Ordered](slice []T, key func(T) K)
pub fn sort_by<T, K, F>(slice: &mut [T], key: F) -> SortaFreshResult<()>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    if slice.len() <= 1 {
        return Ok(());
    }
    
    slice.sort_by_key(key);
    Ok(())
}

/// Stable generic sort by key function
pub fn stable_sort_by<T, K, F>(slice: &mut [T], key: F) -> SortaFreshResult<()>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    if slice.len() <= 1 {
        return Ok(());
    }
    
    slice.sort_by_key(key);
    Ok(())
}

/// Checks if a slice is sorted by a key function
/// slay IsSortedBy[T any, K cmp.Ordered](slice []T, key func(T) K) lit
pub fn is_sorted_by<T, K, F>(slice: &[T], key: F) -> bool
where
    K: Ord,
    F: Fn(&T) -> K,
{
    if slice.len() <= 1 {
        return true;
    }
    
    for i in 1..slice.len() {
        if key(&slice[i]) < key(&slice[i - 1]) {
            return false;
        }
    }
    true
}

/// Checks if a slice is sorted by a comparison function
/// slay IsSortedFunc[T any](slice []T, cmp func(a, b T) normie) lit
pub fn is_sorted_func<T, F>(slice: &[T], cmp: F) -> bool
where
    F: Fn(&T, &T) -> i32,
{
    if slice.len() <= 1 {
        return true;
    }
    
    for i in 1..slice.len() {
        if cmp(&slice[i], &slice[i - 1]) < 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_ints() {
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        sort_ints(&mut data).unwrap();
        assert_eq!(data, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_sort_ints_empty() {
        let mut data: Vec<i32> = vec![];
        sort_ints(&mut data).unwrap();
        assert_eq!(data, vec![]);
    }

    #[test]
    fn test_sort_ints_single() {
        let mut data = vec![42];
        sort_ints(&mut data).unwrap();
        assert_eq!(data, vec![42]);
    }

    #[test]
    fn test_sort_float64s() {
        let mut data = vec![3.14, 1.41, 4.0, 1.73, 5.0, 9.8, 2.71, 6.28];
        sort_float64s(&mut data).unwrap();
        assert_eq!(data, vec![1.41, 1.73, 2.71, 3.14, 4.0, 5.0, 6.28, 9.8]);
    }

    #[test]
    fn test_sort_float64s_with_nan() {
        let mut data = vec![3.14, f64::NAN, 1.41, f64::NAN, 2.71];
        sort_float64s(&mut data).unwrap();
        
        // NaN values should be at the end
        assert_eq!(data[0], 1.41);
        assert_eq!(data[1], 2.71);
        assert_eq!(data[2], 3.14);
        assert!(data[3].is_nan());
        assert!(data[4].is_nan());
    }

    #[test]
    fn test_sort_strings() {
        let mut data = vec![
            "cherry".to_string(),
            "apple".to_string(), 
            "banana".to_string(),
            "date".to_string(),
        ];
        sort_strings(&mut data).unwrap();
        assert_eq!(data, vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
        ]);
    }

    #[test]
    fn test_ints_are_sorted() {
        assert!(ints_are_sorted(&[1, 2, 3, 4, 5]));
        assert!(ints_are_sorted(&[1, 1, 2, 2, 3]));
        assert!(!ints_are_sorted(&[1, 3, 2, 4, 5]));
        assert!(ints_are_sorted(&[42])); // Single element
        assert!(ints_are_sorted(&[])); // Empty
    }

    #[test]
    fn test_float64s_are_sorted() {
        assert!(float64s_are_sorted(&[1.0, 2.0, 3.0, 4.0, 5.0]));
        assert!(float64s_are_sorted(&[1.0, 1.0, 2.0, 2.0, 3.0]));
        assert!(!float64s_are_sorted(&[1.0, 3.0, 2.0, 4.0, 5.0]));
        
        // Test with NaN at the end (should be considered sorted)
        assert!(float64s_are_sorted(&[1.0, 2.0, 3.0, f64::NAN, f64::NAN]));
        assert!(!float64s_are_sorted(&[1.0, f64::NAN, 3.0])); // NaN in middle
    }

    #[test]
    fn test_strings_are_sorted() {
        let data = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
        ];
        assert!(strings_are_sorted(&data));
        
        let unsorted = vec![
            "cherry".to_string(),
            "apple".to_string(),
            "banana".to_string(),
        ];
        assert!(!strings_are_sorted(&unsorted));
    }

    #[test]
    fn test_reverse_sort_ints() {
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        reverse_sort_ints(&mut data).unwrap();
        assert_eq!(data, vec![9, 6, 5, 4, 3, 2, 1, 1]);
    }

    #[test]
    fn test_stable_sort_maintains_order() {
        #[derive(Debug, PartialEq)]
        struct Item {
            key: i32,
            id: char,
        }
        
        let mut data = vec![
            Item { key: 2, id: 'a' },
            Item { key: 1, id: 'b' },
            Item { key: 2, id: 'c' },
            Item { key: 1, id: 'd' },
        ];
        
        sort_by(&mut data, |item| item.key).unwrap();
        
        // Should maintain relative order of equal elements
        assert_eq!(data[0].key, 1);
        assert_eq!(data[1].key, 1);
        assert_eq!(data[2].key, 2);
        assert_eq!(data[3].key, 2);
    }

    #[test]
    fn test_is_sorted_by() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: i32,
        }
        
        let people = vec![
            Person { name: "Alice".to_string(), age: 20 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Charlie".to_string(), age: 30 },
        ];
        
        assert!(is_sorted_by(&people, |p| p.age));
        assert!(is_sorted_by(&people, |p| &p.name));
    }

    #[test]
    fn test_is_sorted_func() {
        let data = vec![1, 2, 3, 4, 5];
        assert!(is_sorted_func(&data, |a, b| (a - b)));
        
        let unsorted = vec![1, 3, 2, 4, 5];
        assert!(!is_sorted_func(&unsorted, |a, b| (a - b)));
    }
}
