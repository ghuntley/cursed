use crate::error::CursedError;
/// Specialized sorting functions for common types
/// 
/// This module provides optimized sorting functions for:
/// - Integers (normie/i32)
/// - Floating point numbers (float64/f64)
/// - Strings (tea/String)
/// 
/// Also includes corresponding "AreSorted" check functions.

use super::SortaFreshResult;

/// Sorts a slice of ints in ascending order
/// slay SortInts(a []normie)
pub fn sort_ints(a: &mut [i32]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    a.sort_unstable();
    Ok(())
/// Sorts a slice of float64s in ascending order
/// slay SortFloat64s(a []float64)
pub fn sort_float64s(a: &mut [f64]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    // Handle NaN values by moving them to the end
    a.sort_by(|a, b| {
        match (a.is_nan(), b.is_nan()) {
        }
    });
    
    Ok(())
/// Sorts a slice of strings in ascending order
/// slay SortStrings(a []tea)
pub fn sort_strings(a: &mut [String]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    a.sort_unstable();
    Ok(())
/// Sorts a slice of string slices in ascending order
pub fn sort_str_slices(a: &mut [&str]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    a.sort_unstable();
    Ok(())
/// Reports whether the slice is sorted in ascending order
/// slay IntsAreSorted(a []normie) lit
pub fn ints_are_sorted(a: &[i32]) -> bool {
    if a.len() <= 1 {
        return true;
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
/// Reports whether the slice is sorted in ascending order
/// slay Float64sAreSorted(a []float64) lit
pub fn float64s_are_sorted(a: &[f64]) -> bool {
    if a.len() <= 1 {
        return true;
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
/// Reports whether the slice is sorted in ascending order
/// slay StringsAreSorted(a []tea) lit
pub fn strings_are_sorted(a: &[String]) -> bool {
    if a.len() <= 1 {
        return true;
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
/// Reports whether the slice of string slices is sorted in ascending order
pub fn str_slices_are_sorted(a: &[&str]) -> bool {
    if a.len() <= 1 {
        return true;
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
/// Reverse sorts a slice of ints in descending order
pub fn reverse_sort_ints(a: &mut [i32]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    a.sort_unstable_by(|a, b| b.cmp(a));
    Ok(())
/// Reverse sorts a slice of float64s in descending order
pub fn reverse_sort_float64s(a: &mut [f64]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    // Handle NaN values by moving them to the end
    a.sort_by(|a, b| {
        match (a.is_nan(), b.is_nan()) {
        }
    });
    
    Ok(())
/// Reverse sorts a slice of strings in descending order
pub fn reverse_sort_strings(a: &mut [String]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    a.sort_unstable_by(|a, b| b.cmp(a));
    Ok(())
/// Stable sorts a slice of ints in ascending order
pub fn stable_sort_ints(a: &mut [i32]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    a.sort();
    Ok(())
/// Stable sorts a slice of float64s in ascending order
pub fn stable_sort_float64s(a: &mut [f64]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    // Handle NaN values by moving them to the end
    a.sort_by(|a, b| {
        match (a.is_nan(), b.is_nan()) {
        }
    });
    
    Ok(())
/// Stable sorts a slice of strings in ascending order
pub fn stable_sort_strings(a: &mut [String]) -> SortaFreshResult<()> {
    if a.len() <= 1 {
        return Ok(());
    a.sort();
    Ok(())
/// Generic sort by key function
/// slay SortBy[T any, K cmp.Ordered](slice []T, key func(T) K)
pub fn sort_by<T, K, F>(slice: &mut [T], key: F) -> SortaFreshResult<()>
where
{
    if slice.len() <= 1 {
        return Ok(());
    slice.sort_by_key(key);
    Ok(())
/// Stable generic sort by key function
pub fn stable_sort_by<T, K, F>(slice: &mut [T], key: F) -> SortaFreshResult<()>
where
{
    if slice.len() <= 1 {
        return Ok(());
    slice.sort_by_key(key);
    Ok(())
/// Checks if a slice is sorted by a key function
/// slay IsSortedBy[T any, K cmp.Ordered](slice []T, key func(T) K) lit
pub fn is_sorted_by<T, K, F>(slice: &[T], key: F) -> bool
where
{
    if slice.len() <= 1 {
        return true;
    for i in 1..slice.len() {
        if key(&slice[i]) < key(&slice[i - 1]) {
            return false;
        }
    }
    true
/// Checks if a slice is sorted by a comparison function
/// slay IsSortedFunc[T any](slice []T, cmp func(a, b T) normie) lit
pub fn is_sorted_func<T, F>(slice: &[T], cmp: F) -> bool
where
{
    if slice.len() <= 1 {
        return true;
    for i in 1..slice.len() {
        if cmp(&slice[i], &slice[i - 1]) < 0 {
            return false;
        }
    }
    true
