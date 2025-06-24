use crate::error::Error;
/// Comprehensive Iterator system for CURSED collections
/// 
/// This module provides:
/// - Core Iterator<T> trait with fundamental iteration methods
/// - IntoIterator<T> trait for converting collections to iterators
/// - Iterator adapters: map, filter, fold, reduce, enumerate, zip, chain
/// - Specialized iterators: Range, Chain, Filter, Map, Take, Skip
/// - Lazy evaluation and efficient memory usage
/// - Integration with existing collections (sets, queues, stacks)

use super::{CollectionsError, CollectionsResult};
use std::fmt::{Debug, Display};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::hash::Hash;
use std::marker::PhantomData;

/// Core Iterator trait providing fundamental iteration functionality
pub trait Iterator<T> {
    /// Advance the iterator and return the next value
    fn next(&mut self) -> Option<T>;
    
    /// Returns the bounds on the remaining length of the iterator
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
    
    /// Consumes the iterator and collects all elements into a Vec
    fn collect(self) -> Vec<T>
    where
        Self: Sized,
    {
        let mut result = Vec::new();
        let mut iter = self;
        while let Some(item) = iter.next() {
            result.push(item);
        }
        result
    }
    
    /// Count the number of elements in the iterator
    fn count(self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        let mut iter = self;
        while iter.next().is_some() {
            count += 1;
        }
        count
    }
    
    /// Find the first element matching a predicate
    fn find<P>(self, predicate: P) -> Option<T>
    where
        Self: Sized,
        P: FnMut(&T) -> bool,
    {
        let mut iter = self;
        let mut pred = predicate;
        while let Some(item) = iter.next() {
            if pred(&item) {
                return Some(item);
            }
        }
        None
    }
    
    /// Tests if any element matches a predicate
    fn any<P>(self, predicate: P) -> bool
    where
        Self: Sized,
        P: FnMut(&T) -> bool,
    {
        self.find(predicate).is_some()
    }
    
    /// Tests if all elements match a predicate
    fn all<P>(self, predicate: P) -> bool
    where
        Self: Sized,
        P: FnMut(&T) -> bool,
    {
        let mut iter = self;
        let mut pred = predicate;
        while let Some(item) = iter.next() {
            if !pred(&item) {
                return false;
            }
        }
        true
    }
    
    /// Transform each element using a function
    fn map<U, F>(self, f: F) -> MapIterator<T, U>
    where
        Self: Sized,
        T: Clone,
        F: FnMut(T) -> U + 'static,
    {
        MapIterator::new(self, f)
    }
    
    /// Filter elements using a predicate
    fn filter<P>(self, predicate: P) -> FilterIterator<Self, P>
    where
        Self: Sized,
        P: FnMut(&T) -> bool,
    {
        FilterIterator::new(self, predicate)
    }
    
    /// Take only the first n elements
    fn take(self, n: usize) -> TakeIterator<Self>
    where
        Self: Sized,
    {
        TakeIterator::new(self, n)
    }
    
    /// Skip the first n elements
    fn skip(self, n: usize) -> SkipIterator<Self>
    where
        Self: Sized,
    {
        SkipIterator::new(self, n)
    }
    
    /// Add indices to elements
    fn enumerate(self) -> EnumerateIterator<Self>
    where
        Self: Sized,
    {
        EnumerateIterator::new(self)
    }
    
    /// Chain with another iterator
    fn chain<I>(self, other: I) -> ChainIterator<Self, I>
    where
        Self: Sized,
        I: Iterator<T>,
    {
        ChainIterator::new(self, other)
    }
    
    /// Zip with another iterator
    fn zip<I, U>(self, other: I) -> ZipIterator<Self, I>
    where
        Self: Sized,
        I: Iterator<U>,
    {
        ZipIterator::new(self, other)
    }
    
    /// Fold (reduce) the iterator with an accumulator
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, T) -> B,
    {
        let mut iter = self;
        let mut acc = init;
        let mut folder = f;
        while let Some(item) = iter.next() {
            acc = folder(acc, item);
        }
        acc
    }
    
    /// Reduce the iterator to a single value
    fn reduce<F>(self, f: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(T, T) -> T,
    {
        let mut iter = self;
        let first = iter.next()?;
        Some(iter.fold(first, f))
    }
    
    /// Step by a given amount
    fn step_by(self, step: usize) -> StepByIterator<Self>
    where
        Self: Sized,
    {
        StepByIterator::new(self, step)
    }
    
    /// Cycle the iterator infinitely
    fn cycle(self) -> CycleIterator<Self>
    where
        Self: Sized + Clone,
    {
        CycleIterator::new(self)
    }
    
    /// Get the minimum element
    fn min(self) -> Option<T>
    where
        Self: Sized,
        T: Ord,
    {
        self.reduce(|a, b| if a <= b { a } else { b })
    }
    
    /// Get the maximum element
    fn max(self) -> Option<T>
    where
        Self: Sized,
        T: Ord,
    {
        self.reduce(|a, b| if a >= b { a } else { b })
    }
}

/// Trait for types that can be converted into an iterator
pub trait IntoIterator<T> {
    type Iterator: Iterator<T>;
    
    /// Convert into an iterator
    fn into_iter(self) -> Self::Iterator;
}

// ==================== Range Iterator ====================

/// Iterator over numeric ranges
#[derive(Debug, Clone)]
pub struct RangeIterator<T> {
    start: T,
    end: T,
    step: T,
    current: T,
    done: bool,
}

impl<T> RangeIterator<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T>,
{
    /// Create a new range iterator
    pub fn new(start: T, end: T, step: T) -> Self {
        Self {
            start,
            end,
            step,
            current: start,
            done: false,
        }
    }
    
    /// Create a range from start to end (exclusive) with step 1
    pub fn range(start: T, end: T) -> Self
    where
        T: From<i32>,
    {
        Self::new(start, end, T::from(1))
    }
}

impl<T> Iterator<T> for RangeIterator<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T>,
{
    fn next(&mut self) -> Option<T> {
        if self.done || self.current >= self.end {
            return None;
        }
        
        let result = self.current;
        self.current = self.current + self.step;
        
        if self.current >= self.end {
            self.done = true;
        }
        
        Some(result)
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            (0, Some(0))
        } else {
            // This is a simplified estimation
            (0, None)
        }
    }
}

// ==================== Map Iterator ====================

/// Iterator that transforms elements using a function
pub struct MapIterator<T, U> {
    items: Vec<T>,
    index: usize,
    f: Box<dyn FnMut(T) -> U>,
}

impl<T, U> MapIterator<T, U> {
    pub fn new<I, F>(iter: I, f: F) -> Self
    where
        I: Iterator<T>,
        F: FnMut(T) -> U + 'static,
    {
        Self {
            items: iter.collect(),
            index: 0,
            f: Box::new(f),
        }
    }
}

impl<T, U> Iterator<U> for MapIterator<T, U>
where
    T: Clone,
{
    fn next(&mut self) -> Option<U> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some((self.f)(item))
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.items.len() - self.index;
        (remaining, Some(remaining))
    }
}

// ==================== Filter Iterator ====================

/// Iterator that filters elements using a predicate
#[derive(Debug, Clone)]
pub struct FilterIterator<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> FilterIterator<I, P> {
    pub fn new(iter: I, predicate: P) -> Self {
        Self { iter, predicate }
    }
}

impl<I, T, P> Iterator<T> for FilterIterator<I, P>
where
    I: Iterator<T>,
    P: FnMut(&T) -> bool,
{
    fn next(&mut self) -> Option<T> {
        while let Some(item) = self.iter.next() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }
        None
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}

// ==================== Take Iterator ====================

/// Iterator that yields only the first n elements
#[derive(Debug, Clone)]
pub struct TakeIterator<I> {
    iter: I,
    n: usize,
}

impl<I> TakeIterator<I> {
    pub fn new(iter: I, n: usize) -> Self {
        Self { iter, n }
    }
}

impl<I, T> Iterator<T> for TakeIterator<I>
where
    I: Iterator<T>,
{
    fn next(&mut self) -> Option<T> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            self.iter.next()
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (
            std::cmp::min(lower, self.n),
            upper.map(|u| std::cmp::min(u, self.n)),
        )
    }
}

// ==================== Skip Iterator ====================

/// Iterator that skips the first n elements
#[derive(Debug, Clone)]
pub struct SkipIterator<I> {
    iter: I,
    n: usize,
    skipped: bool,
}

impl<I> SkipIterator<I> {
    pub fn new(iter: I, n: usize) -> Self {
        Self { iter, n, skipped: false }
    }
}

impl<I, T> Iterator<T> for SkipIterator<I>
where
    I: Iterator<T>,
{
    fn next(&mut self) -> Option<T> {
        if !self.skipped {
            for _ in 0..self.n {
                self.iter.next()?;
            }
            self.skipped = true;
        }
        self.iter.next()
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (
            lower.saturating_sub(self.n),
            upper.map(|u| u.saturating_sub(self.n)),
        )
    }
}

// ==================== Enumerate Iterator ====================

/// Iterator that yields (index, element) pairs
#[derive(Debug, Clone)]
pub struct EnumerateIterator<I> {
    iter: I,
    count: usize,
}

impl<I> EnumerateIterator<I> {
    pub fn new(iter: I) -> Self {
        Self { iter, count: 0 }
    }
}

impl<I, T> Iterator<(usize, T)> for EnumerateIterator<I>
where
    I: Iterator<T>,
{
    fn next(&mut self) -> Option<(usize, T)> {
        self.iter.next().map(|item| {
            let index = self.count;
            self.count += 1;
            (index, item)
        })
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// ==================== Chain Iterator ====================

/// Iterator that chains two iterators together
#[derive(Debug, Clone)]
pub struct ChainIterator<A, B> {
    first: Option<A>,
    second: B,
}

impl<A, B> ChainIterator<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self {
            first: Some(first),
            second,
        }
    }
}

impl<A, B, T> Iterator<T> for ChainIterator<A, B>
where
    A: Iterator<T>,
    B: Iterator<T>,
{
    fn next(&mut self) -> Option<T> {
        if let Some(ref mut first) = self.first {
            if let Some(item) = first.next() {
                return Some(item);
            }
            self.first = None;
        }
        self.second.next()
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (second_lower, second_upper) = self.second.size_hint();
        
        if let Some(ref first) = self.first {
            let (first_lower, first_upper) = first.size_hint();
            (
                first_lower.saturating_add(second_lower),
                match (first_upper, second_upper) {
                    (Some(a), Some(b)) => a.checked_add(b),
                    _ => None,
                },
            )
        } else {
            (second_lower, second_upper)
        }
    }
}

// ==================== Zip Iterator ====================

/// Iterator that pairs elements from two iterators
#[derive(Debug, Clone)]
pub struct ZipIterator<A, B> {
    a: A,
    b: B,
}

impl<A, B> ZipIterator<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B, T, U> Iterator<(T, U)> for ZipIterator<A, B>
where
    A: Iterator<T>,
    B: Iterator<U>,
{
    fn next(&mut self) -> Option<(T, U)> {
        match (self.a.next(), self.b.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper) = self.a.size_hint();
        let (b_lower, b_upper) = self.b.size_hint();
        
        (
            std::cmp::min(a_lower, b_lower),
            match (a_upper, b_upper) {
                (Some(a), Some(b)) => Some(std::cmp::min(a, b)),
                _ => None,
            },
        )
    }
}

// ==================== Step By Iterator ====================

/// Iterator that yields every nth element
#[derive(Debug, Clone)]
pub struct StepByIterator<I> {
    iter: I,
    step: usize,
    first_take: bool,
}

impl<I> StepByIterator<I> {
    pub fn new(iter: I, step: usize) -> Self {
        assert!(step != 0, "step size must be non-zero");
        Self {
            iter,
            step,
            first_take: true,
        }
    }
}

impl<I, T> Iterator<T> for StepByIterator<I>
where
    I: Iterator<T>,
{
    fn next(&mut self) -> Option<T> {
        if self.first_take {
            self.first_take = false;
            self.iter.next()
        } else {
            for _ in 0..self.step {
                self.iter.next()?;
            }
            self.iter.next()
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let step = self.step;
        
        // Conservative estimate
        (
            if lower == 0 { 0 } else { (lower - 1) / step + 1 },
            upper.map(|u| if u == 0 { 0 } else { (u - 1) / step + 1 }),
        )
    }
}

// ==================== Cycle Iterator ====================

/// Iterator that repeats elements infinitely
#[derive(Debug, Clone)]
pub struct CycleIterator<I> {
    original: I,
    current: I,
}

impl<I> CycleIterator<I>
where
    I: Clone,
{
    pub fn new(iter: I) -> Self {
        Self {
            current: iter.clone(),
            original: iter,
        }
    }
}

impl<I, T> Iterator<T> for CycleIterator<I>
where
    I: Iterator<T> + Clone,
{
    fn next(&mut self) -> Option<T> {
        match self.current.next() {
            Some(item) => Some(item),
            None => {
                self.current = self.original.clone();
                self.current.next()
            }
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, _) = self.current.size_hint();
        if lower == 0 {
            (0, None)
        } else {
            (usize::MAX, None)
        }
    }
}

// ==================== Vec Iterator ====================

/// Iterator for Vec<T>
#[derive(Debug, Clone)]
pub struct VecIterator<T> {
    vec: Vec<T>,
    index: usize,
}

impl<T> VecIterator<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { vec, index: 0 }
    }
}

impl<T> Iterator<T> for VecIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<T> {
        if self.index >= self.vec.len() {
            None
        } else {
            let item = self.vec[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<T> IntoIterator<T> for Vec<T>
where
    T: Clone,
{
    type Iterator = VecIterator<T>;
    
    fn into_iter(self) -> Self::Iterator {
        VecIterator::new(self)
    }
}

// ==================== Utility Functions ====================

/// Create a range iterator from start to end (exclusive)
pub fn range<T>(start: T, end: T) -> RangeIterator<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T> + From<i32>,
{
    RangeIterator::range(start, end)
}

/// Create a range iterator with step
pub fn range_step<T>(start: T, end: T, step: T) -> RangeIterator<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T>,
{
    RangeIterator::new(start, end, step)
}

/// Repeat a value infinitely
pub fn repeat<T>(value: T) -> RepeatIterator<T>
where
    T: Clone,
{
    RepeatIterator::new(value)
}

/// Repeat a value a specified number of times
pub fn repeat_n<T>(value: T, n: usize) -> TakeIterator<RepeatIterator<T>>
where
    T: Clone,
{
    repeat(value).take(n)
}

// ==================== Repeat Iterator ====================

/// Iterator that repeats a single value infinitely
#[derive(Debug, Clone)]
pub struct RepeatIterator<T> {
    value: T,
}

impl<T> RepeatIterator<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Iterator<T> for RepeatIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<T> {
        Some(self.value.clone())
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_iterator() {
        let mut iter = range(0, 5);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_map_iterator() {
        let vec = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = vec.into_iter().map(|x| x * 2).collect();
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_filter_iterator() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        let result: Vec<i32> = vec.into_iter().filter(|&x| x % 2 == 0).collect();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_chain_iterator() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
        let result: Vec<i32> = vec1.into_iter().chain(vec2.into_iter()).collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_take_iterator() {
        let vec = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = vec.into_iter().take(3).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_skip_iterator() {
        let vec = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = vec.into_iter().skip(2).collect();
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_enumerate_iterator() {
        let vec = vec!["a", "b", "c"];
        let result: Vec<(usize, &str)> = vec.into_iter().enumerate().collect();
        assert_eq!(result, vec![(0, "a"), (1, "b"), (2, "c")]);
    }

    #[test]
    fn test_zip_iterator() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec!["a", "b", "c"];
        let result: Vec<(i32, &str)> = vec1.into_iter().zip(vec2.into_iter()).collect();
        assert_eq!(result, vec![(1, "a"), (2, "b"), (3, "c")]);
    }

    #[test]
    fn test_fold() {
        let vec = vec![1, 2, 3, 4, 5];
        let sum = vec.into_iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_reduce() {
        let vec = vec![1, 2, 3, 4, 5];
        let product = vec.into_iter().reduce(|acc, x| acc * x);
        assert_eq!(product, Some(120));
    }

    #[test]
    fn test_step_by() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: Vec<i32> = vec.into_iter().step_by(2).collect();
        assert_eq!(result, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_repeat() {
        let result: Vec<i32> = repeat(42).take(5).collect();
        assert_eq!(result, vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn test_min_max() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(vec.clone().into_iter().min(), Some(1));
        assert_eq!(vec.into_iter().max(), Some(9));
    }

    #[test]
    fn test_count() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.into_iter().count(), 5);
    }

    #[test]
    fn test_any_all() {
        let vec = vec![2, 4, 6, 8];
        assert!(vec.clone().into_iter().any(|x| x > 5));
        assert!(vec.into_iter().all(|x| x % 2 == 0));
    }
}
