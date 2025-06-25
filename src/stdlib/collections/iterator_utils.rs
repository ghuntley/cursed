/// Iterator utilities and additional functionality for CURSED collections
/// 
/// This module provides:
/// - Collection methods: collect, partition, group_by
/// - Aggregation: sum, product, min, max, count
/// - Advanced utilities: flat_map, windows, chunks
/// - Parallel iteration support (basic framework)
/// - CursedError handling and short-circuiting iterators

use super::{CollectionsError, CollectionsResult, Iterator, IntoIterator};
use std::collections::{HashMap, BTreeMap};
use std::hash::Hash;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};
use std::thread;

/// Utility functions for iterator operations
pub struct IteratorUtils;

impl IteratorUtils {
    /// Partition an iterator into two collections based on a predicate
    pub fn partition<I, T, P>(iter: I, predicate: P) -> (Vec<T>, Vec<T>)
    where
        I: Iterator<T>,
        P: Fn(&T) -> bool,
    {
        let mut true_vec = Vec::new();
        let mut false_vec = Vec::new();
        let mut iterator = iter;
        
        while let Some(item) = iterator.next() {
            if predicate(&item) {
                true_vec.push(item);
            } else {
                false_vec.push(item);
            }
        }
        
        (true_vec, false_vec)
    }
    
    /// Group elements by a key function
    pub fn group_by<I, T, K, F>(iter: I, key_fn: F) -> HashMap<K, Vec<T>>
    where
        I: Iterator<T>,
        K: Hash + Eq,
        F: Fn(&T) -> K,
    {
        let mut groups = HashMap::new();
        let mut iterator = iter;
        
        while let Some(item) = iterator.next() {
            let key = key_fn(&item);
            groups.entry(key).or_insert_with(Vec::new).push(item);
        }
        
        groups
    }
    
    /// Group elements by a key function using ordered map
    pub fn group_by_ordered<I, T, K, F>(iter: I, key_fn: F) -> BTreeMap<K, Vec<T>>
    where
        I: Iterator<T>,
        K: Ord,
        F: Fn(&T) -> K,
    {
        let mut groups = BTreeMap::new();
        let mut iterator = iter;
        
        while let Some(item) = iterator.next() {
            let key = key_fn(&item);
            groups.entry(key).or_insert_with(Vec::new).push(item);
        }
        
        groups
    }
    
    /// Compute sum of numeric elements
    pub fn sum<I, T>(iter: I) -> T
    where
        I: Iterator<T>,
        T: std::ops::Add<Output = T> + Default,
    {
        let mut iterator = iter;
        iterator.fold(T::default(), |acc, x| acc + x)
    }
    
    /// Compute product of numeric elements
    pub fn product<I, T>(iter: I) -> T
    where
        I: Iterator<T>,
        T: std::ops::Mul<Output = T> + From<i32>,
    {
        let mut iterator = iter;
        iterator.fold(T::from(1), |acc, x| acc * x)
    }
    
    /// Find minimum element with custom comparison
    pub fn min_by<I, T, F>(iter: I, compare: F) -> Option<T>
    where
        I: Iterator<T>,
        F: Fn(&T, &T) -> Ordering,
    {
        let mut iterator = iter;
        let first = iterator.next()?;
        Some(iterator.fold(first, |acc, x| {
            if compare(&x, &acc) == Ordering::Less {
                x
            } else {
                acc
            }
        }))
    }
    
    /// Find maximum element with custom comparison
    pub fn max_by<I, T, F>(iter: I, compare: F) -> Option<T>
    where
        I: Iterator<T>,
        F: Fn(&T, &T) -> Ordering,
    {
        let mut iterator = iter;
        let first = iterator.next()?;
        Some(iterator.fold(first, |acc, x| {
            if compare(&x, &acc) == Ordering::Greater {
                x
            } else {
                acc
            }
        }))
    }
    
    /// Find minimum element by key
    pub fn min_by_key<I, T, K, F>(iter: I, key_fn: F) -> Option<T>
    where
        I: Iterator<T>,
        K: Ord,
        F: Fn(&T) -> K,
    {
        Self::min_by(iter, |a, b| key_fn(a).cmp(&key_fn(b)))
    }
    
    /// Find maximum element by key
    pub fn max_by_key<I, T, K, F>(iter: I, key_fn: F) -> Option<T>
    where
        I: Iterator<T>,
        K: Ord,
        F: Fn(&T) -> K,
    {
        Self::max_by(iter, |a, b| key_fn(a).cmp(&key_fn(b)))
    }
    
    /// Get the nth element (0-indexed)
    pub fn nth<I, T>(iter: I, n: usize) -> Option<T>
    where
        I: Iterator<T>,
    {
        let mut iterator = iter;
        for _ in 0..n {
            iterator.next()?;
        }
        iterator.next()
    }
    
    /// Get the position of the first element matching a predicate
    pub fn position<I, T, P>(iter: I, predicate: P) -> Option<usize>
    where
        I: Iterator<T>,
        P: Fn(&T) -> bool,
    {
        let mut iterator = iter;
        let mut pos = 0;
        while let Some(item) = iterator.next() {
            if predicate(&item) {
                return Some(pos);
            }
            pos += 1;
        }
        None
    }
    
    /// Get the position of the last element matching a predicate
    pub fn rposition<I, T, P>(iter: I, predicate: P) -> Option<usize>
    where
        I: Iterator<T>,
        P: Fn(&T) -> bool,
    {
        let mut iterator = iter;
        let mut last_pos = None;
        let mut pos = 0;
        while let Some(item) = iterator.next() {
            if predicate(&item) {
                last_pos = Some(pos);
            }
            pos += 1;
        }
        last_pos
    }
    
    /// Try to collect with error handling
    pub fn try_collect<I, T, E>(iter: I) -> Result<Vec<T>, E>
    where
        I: Iterator<Result<T, E>>,
    {
        let mut result = Vec::new();
        let mut iterator = iter;
        while let Some(item) = iterator.next() {
            result.push(item?);
        }
        Ok(result)
    }
    
    /// Unzip pairs into two separate collections
    pub fn unzip<I, A, B>(iter: I) -> (Vec<A>, Vec<B>)
    where
        I: Iterator<(A, B)>,
    {
        let mut as_vec = Vec::new();
        let mut bs_vec = Vec::new();
        let mut iterator = iter;
        
        while let Some((a, b)) = iterator.next() {
            as_vec.push(a);
            bs_vec.push(b);
        }
        
        (as_vec, bs_vec)
    }
}

// ==================== Advanced Iterator Adapters ====================

/// Iterator that flattens nested iterators
#[derive(Debug, Clone)]
pub struct FlatMapIterator<I, F, U> {
    outer: I,
    inner: Option<U>,
    f: F,
}

impl<I, F, U> FlatMapIterator<I, F, U> {
    pub fn new(iter: I, f: F) -> Self {
        Self {
            outer: iter,
            inner: None,
            f,
        }
    }
}

impl<I, F, T, U, V> Iterator<V> for FlatMapIterator<I, F, U>
where
    I: Iterator<T>,
    U: Iterator<V>,
    F: FnMut(T) -> U,
{
    fn next(&mut self) -> Option<V> {
        loop {
            if let Some(ref mut inner) = self.inner {
                if let Some(item) = inner.next() {
                    return Some(item);
                }
                self.inner = None;
            }
            
            match self.outer.next() {
                Some(outer_item) => {
                    self.inner = Some((self.f)(outer_item));
                }
                None => return None,
            }
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (outer_lower, outer_upper) = self.outer.size_hint();
        
        if let Some(ref inner) = self.inner {
            let (inner_lower, inner_upper) = inner.size_hint();
            (
                inner_lower,
                inner_upper.and_then(|inner| outer_upper.map(|outer| inner + outer)),
            )
        } else {
            (0, outer_upper)
        }
    }
}

/// Iterator that yields windows of elements
#[derive(Debug, Clone)]
pub struct WindowsIterator<T> {
    items: Vec<T>,
    window_size: usize,
    index: usize,
}

impl<T> WindowsIterator<T>
where
    T: Clone,
{
    pub fn new<I>(iter: I, window_size: usize) -> Self
    where
        I: Iterator<T>,
    {
        assert!(window_size > 0, "window size must be greater than 0");
        let items: Vec<T> = iter.collect();
        Self {
            items,
            window_size,
            index: 0,
        }
    }
}

impl<T> Iterator<Vec<T>> for WindowsIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<Vec<T>> {
        if self.index + self.window_size > self.items.len() {
            return None;
        }
        
        let window = self.items[self.index..self.index + self.window_size].to_vec();
        self.index += 1;
        Some(window)
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = if self.items.len() >= self.window_size {
            self.items.len() - self.window_size + 1 - self.index
        } else {
            0
        };
        (remaining, Some(remaining))
    }
}

/// Iterator that yields chunks of elements
#[derive(Debug, Clone)]
pub struct ChunksIterator<T> {
    items: Vec<T>,
    chunk_size: usize,
    index: usize,
}

impl<T> ChunksIterator<T>
where
    T: Clone,
{
    pub fn new<I>(iter: I, chunk_size: usize) -> Self
    where
        I: Iterator<T>,
    {
        assert!(chunk_size > 0, "chunk size must be greater than 0");
        let items: Vec<T> = iter.collect();
        Self {
            items,
            chunk_size,
            index: 0,
        }
    }
}

impl<T> Iterator<Vec<T>> for ChunksIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<Vec<T>> {
        if self.index >= self.items.len() {
            return None;
        }
        
        let end = std::cmp::min(self.index + self.chunk_size, self.items.len());
        let chunk = self.items[self.index..end].to_vec();
        self.index = end;
        Some(chunk)
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_items = if self.index < self.items.len() {
            self.items.len() - self.index
        } else {
            0
        };
        let chunks = (remaining_items + self.chunk_size - 1) / self.chunk_size;
        (chunks, Some(chunks))
    }
}

/// Iterator with error handling and short-circuiting
#[derive(Debug, Clone)]
pub struct TryIterator<I, T, E> {
    iter: I,
    error: Option<E>,
    _phantom: std::marker::PhantomData<T>,
}

impl<I, T, E> TryIterator<I, T, E>
where
    I: Iterator<Result<T, E>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            error: None,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Get the first error encountered
    pub fn error(&self) -> Option<&E> {
        self.error.as_ref()
    }
    
    /// Check if an error was encountered
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

impl<I, T, E> Iterator<T> for TryIterator<I, T, E>
where
    I: Iterator<Result<T, E>>,
{
    fn next(&mut self) -> Option<T> {
        if self.error.is_some() {
            return None;
        }
        
        match self.iter.next() {
            Some(Ok(item)) => Some(item),
            Some(Err(e)) => {
                self.error = Some(e);
                None
            }
            None => None,
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.error.is_some() {
            (0, Some(0))
        } else {
            self.iter.size_hint()
        }
    }
}

// ==================== Parallel Iterator Support ====================

/// Basic parallel iterator framework
pub struct ParallelIterator<I, T> {
    iter: I,
    thread_count: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<I, T> ParallelIterator<I, T>
where
    I: Iterator<T> + Send,
    T: Send + 'static,
{
    pub fn new(iter: I, thread_count: usize) -> Self {
        Self {
            iter,
            thread_count: std::cmp::max(1, thread_count),
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Parallel map operation
    pub fn map<U, F>(self, f: F) -> Vec<U>
    where
        F: Fn(T) -> U + Send + Sync + 'static,
        U: Send + 'static,
    {
        let items: Vec<T> = self.iter.collect();
        let chunk_size = (items.len() + self.thread_count - 1) / self.thread_count;
        let f = Arc::new(f);
        
        let mut handles = Vec::new();
        
        for chunk in items.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let f = Arc::clone(&f);
            
            let handle = thread::spawn(move || {
                chunk.into_iter().map(|item| f(item)).collect::<Vec<U>>()
            });
            
            handles.push(handle);
        }
        
        let mut result = Vec::new();
        for handle in handles {
            result.extend(handle.join().unwrap());
        }
        
        result
    }
    
    /// Parallel filter operation
    pub fn filter<F>(self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
        T: Clone,
    {
        let items: Vec<T> = self.iter.collect();
        let chunk_size = (items.len() + self.thread_count - 1) / self.thread_count;
        let predicate = Arc::new(predicate);
        
        let mut handles = Vec::new();
        
        for chunk in items.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let predicate = Arc::clone(&predicate);
            
            let handle = thread::spawn(move || {
                chunk
                    .iter()
                    .filter(|item| predicate(item))
                    .cloned()
                    .collect::<Vec<T>>()
            });
            
            handles.push(handle);
        }
        
        let mut result = Vec::new();
        for handle in handles {
            result.extend(handle.join().unwrap());
        }
        
        result
    }
    
    /// Parallel reduce operation
    pub fn reduce<F>(self, f: F) -> Option<T>
    where
        F: Fn(T, T) -> T + Send + Sync + 'static,
        T: Clone,
    {
        let items: Vec<T> = self.iter.collect();
        if items.is_empty() {
            return None;
        }
        
        let chunk_size = (items.len() + self.thread_count - 1) / self.thread_count;
        let f = Arc::new(f);
        
        let mut handles = Vec::new();
        
        for chunk in items.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let f = Arc::clone(&f);
            
            let handle = thread::spawn(move || {
                chunk.iter().cloned().reduce(|a, b| f(a, b))
            });
            
            handles.push(handle);
        }
        
        let partial_results: Vec<T> = handles
            .into_iter()
            .filter_map(|handle| handle.join().unwrap())
            .collect();
        
        partial_results.into_iter().reduce(|a, b| f(a, b))
    }
}

// ==================== Extension Traits ====================

/// Additional iterator methods
pub trait IteratorExt<T>: Iterator<T> + Sized {
    /// Flat map transformation
    fn flat_map<U, F, I>(self, f: F) -> FlatMapIterator<Self, F, I>
    where
        F: FnMut(T) -> I,
        I: Iterator<U>,
    {
        FlatMapIterator::new(self, f)
    }
    
    /// Create windows of elements
    fn windows(self, size: usize) -> WindowsIterator<T>
    where
        T: Clone,
    {
        WindowsIterator::new(self, size)
    }
    
    /// Create chunks of elements
    fn chunks(self, size: usize) -> ChunksIterator<T>
    where
        T: Clone,
    {
        ChunksIterator::new(self, size)
    }
    
    /// Convert to parallel iterator
    fn parallel(self, thread_count: usize) -> ParallelIterator<Self, T>
    where
        Self: Send,
        T: Send + 'static,
    {
        ParallelIterator::new(self, thread_count)
    }
    
    /// Convert to try iterator for error handling
    fn try_iter<E>(self) -> TryIterator<Self, T, E>
    where
        Self: Iterator<Result<T, E>>,
    {
        TryIterator::new(self)
    }
    
    /// Partition elements based on predicate
    fn partition<P>(self, predicate: P) -> (Vec<T>, Vec<T>)
    where
        P: Fn(&T) -> bool,
    {
        IteratorUtils::partition(self, predicate)
    }
    
    /// Group elements by key
    fn group_by<K, F>(self, key_fn: F) -> HashMap<K, Vec<T>>
    where
        K: Hash + Eq,
        F: Fn(&T) -> K,
    {
        IteratorUtils::group_by(self, key_fn)
    }
    
    /// Sum numeric elements
    fn sum_elements(self) -> T
    where
        T: std::ops::Add<Output = T> + Default,
    {
        IteratorUtils::sum(self)
    }
    
    /// Product of numeric elements
    fn product_elements(self) -> T
    where
        T: std::ops::Mul<Output = T> + From<i32>,
    {
        IteratorUtils::product(self)
    }
}

impl<I, T> IteratorExt<T> for I where I: Iterator<T> {}

