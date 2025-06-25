use crate::error::CursedError;
/// Core sorting functions for SortaFresh
/// 
/// This module implements the fundamental sorting operations:
/// - Sort: Basic sorting in ascending order
/// - Reverse: Sorting in reverse order
/// - IsSorted: Check if data is already sorted
/// - Stable: Stable sorting (preserves equal element order)
/// - Shuffle: Randomize element order

use super::{Sortable, ReverseSortable, SortaFreshResult};
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Sorts data in ascending order as determined by the Less method
/// slay Sort(data Sortable)
pub fn sort<T: Sortable>(data: &mut T) -> SortaFreshResult<()> {
    if data.is_empty() {
        return Ok(());
    let len = data.len();
    if len <= 1 {
        return Ok(());
    quick_sort_impl(data, 0, len - 1)?;
    Ok(())
/// Sorts data in reverse order
/// slay Reverse(data Sortable)
pub fn reverse_sort<T: Sortable>(data: &mut T) -> SortaFreshResult<()> {
    if data.is_empty() {
        return Ok(());
    let len = data.len();
    if len <= 1 {
        return Ok(());
    // Use reverse comparison for sorting
    reverse_quick_sort_impl(data, 0, len - 1)?;
    Ok(())
/// Reports whether data is sorted in ascending order
/// slay IsSorted(data Sortable) lit
pub fn is_sorted<T: Sortable>(data: &T) -> bool {
    let len = data.len();
    if len <= 1 {
        return true;
    for i in 0..len - 1 {
        if data.less(i + 1, i) {
            return false;
        }
    }
    true
/// Performs a stable sort on data
/// slay Stable(data Sortable)
pub fn stable_sort<T: Sortable>(data: &mut T) -> SortaFreshResult<()> {
    if data.is_empty() {
        return Ok(());
    let len = data.len();
    if len <= 1 {
        return Ok(());
    merge_sort_impl(data, 0, len - 1)?;
    Ok(())
/// Randomizes the order of elements in data
/// slay Shuffle(data Sortable)
pub fn shuffle<T: Sortable>(data: &mut T) -> SortaFreshResult<()> {
    let len = data.len();
    if len <= 1 {
        return Ok(());
    let mut rng = thread_rng();
    
    // Fisher-Yates shuffle implementation
    for i in (1..len).rev() {
        let j = rng.gen_range(0..=i);
        if i != j {
            data.swap(i, j);
        }
    }
    
    Ok(())
/// Internal quicksort implementation
fn quick_sort_impl<T: Sortable>(data: &mut T, low: i32, high: i32) -> SortaFreshResult<()> {
    if low < high {
        let pivot = partition(data, low, high)?;
        quick_sort_impl(data, low, pivot - 1)?;
        quick_sort_impl(data, pivot + 1, high)?;
    }
    Ok(())
/// Internal reverse quicksort implementation
fn reverse_quick_sort_impl<T: Sortable>(data: &mut T, low: i32, high: i32) -> SortaFreshResult<()> {
    if low < high {
        let pivot = reverse_partition(data, low, high)?;
        reverse_quick_sort_impl(data, low, pivot - 1)?;
        reverse_quick_sort_impl(data, pivot + 1, high)?;
    }
    Ok(())
/// Partition function for quicksort
fn partition<T: Sortable>(data: &mut T, low: i32, high: i32) -> SortaFreshResult<i32> {
    let mut i = low - 1;
    
    for j in low..high {
        if data.less(j, high) {
            i += 1;
            data.swap(i, j);
        }
    }
    
    data.swap(i + 1, high);
    Ok(i + 1)
/// Reverse partition function for quicksort
fn reverse_partition<T: Sortable>(data: &mut T, low: i32, high: i32) -> SortaFreshResult<i32> {
    let mut i = low - 1;
    
    for j in low..high {
        if data.less(high, j) { // Reverse comparison
            i += 1;
            data.swap(i, j);
        }
    }
    
    data.swap(i + 1, high);
    Ok(i + 1)
/// Internal merge sort implementation for stable sorting
fn merge_sort_impl<T: Sortable>(data: &mut T, left: i32, right: i32) -> SortaFreshResult<()> {
    if left < right {
        let mid = left + (right - left) / 2;
        
        merge_sort_impl(data, left, mid)?;
        merge_sort_impl(data, mid + 1, right)?;
        merge(data, left, mid, right)?;
    }
    Ok(())
/// Merge function for merge sort - implements stable merging
fn merge<T: Sortable>(data: &mut T, left: i32, mid: i32, right: i32) -> SortaFreshResult<()> {
    // For a proper merge sort implementation on Sortable trait,
    // we would need auxiliary storage. For now, we'll use insertion sort
    // on small ranges as a stable alternative.
    insertion_sort_range(data, left, right)
/// Insertion sort for a range - always stable
fn insertion_sort_range<T: Sortable>(data: &mut T, start: i32, end: i32) -> SortaFreshResult<()> {
    for i in start + 1..=end {
        let mut j = i;
        while j > start && data.less(j, j - 1) {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
    Ok(())
/// Generic sort function for slices with custom comparison
/// slay SortSlice[T any](slice []T, less func(i, j normie) lit)
pub fn sort_slice<T, F>(slice: &mut [T], less: F) -> SortaFreshResult<()>
where
{
    if slice.len() <= 1 {
        return Ok(());
    slice.sort_by(|a, b| {
        if less(a, b) {
            std::cmp::Ordering::Less
        } else if less(b, a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    
    Ok(())
/// Stable sort function for slices with custom comparison
/// slay StableSortSlice[T any](slice []T, less func(i, j normie) lit)
pub fn stable_sort_slice<T, F>(slice: &mut [T], less: F) -> SortaFreshResult<()>
where
{
    if slice.len() <= 1 {
        return Ok(());
    slice.sort_by(|a, b| {
        if less(a, b) {
            std::cmp::Ordering::Less
        } else if less(b, a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    
    Ok(())
/// Sort function with comparison function
/// slay SortFunc[T any](slice []T, cmp func(a, b T) normie)
pub fn sort_func<T, F>(slice: &mut [T], cmp: F) -> SortaFreshResult<()>
where
{
    if slice.len() <= 1 {
        return Ok(());
    slice.sort_by(|a, b| {
        match cmp(a, b) {
        }
    });
    
    Ok(())
/// Stable sort function with comparison function
/// slay StableSortFunc[T any](slice []T, cmp func(a, b T) normie)
pub fn stable_sort_func<T, F>(slice: &mut [T], cmp: F) -> SortaFreshResult<()>
where
{
    if slice.len() <= 1 {
        return Ok(());
    slice.stable_sort_by(|a, b| {
        match cmp(a, b) {
        }
    });
    
    Ok(())
