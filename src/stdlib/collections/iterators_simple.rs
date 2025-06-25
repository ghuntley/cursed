use crate::error::CursedError;
/// Simplified Iterator system for CURSED collections
/// 
/// This module provides a basic iterator system that avoids complex type constraints
/// while still providing functional programming capabilities.

use super::{CollectionsError, CollectionsResult};
use std::fmt::Debug;

/// Simple Iterator trait for basic iteration
pub trait SimpleIterator<T> {
    /// Get the next item
    fn next(&mut self) -> Option<T>;
    
    /// Count remaining items
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
    
    /// Collect into Vec
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
    
    /// Find first matching element
    fn find<P>(self, mut predicate: P) -> Option<T>
    where
        Self: Sized,
        P: FnMut(&T) -> bool,
    {
        let mut iter = self;
        while let Some(item) = iter.next() {
            if predicate(&item) {
                return Some(item);
            }
        }
        None
    }
    
    /// Test if any element matches
    fn any<P>(self, predicate: P) -> bool
    where
        Self: Sized,
        P: FnMut(&T) -> bool,
    {
        self.find(predicate).is_some()
    }
    
    /// Test if all elements match
    fn all<P>(self, mut predicate: P) -> bool
    where
        Self: Sized,
        P: FnMut(&T) -> bool,
    {
        let mut iter = self;
        while let Some(item) = iter.next() {
            if !predicate(&item) {
                return false;
            }
        }
        true
    }
    
    /// Take first n elements
    fn take(self, n: usize) -> TakeIterator<T>
    where
        Self: Sized,
    {
        TakeIterator::new(self.collect(), n)
    }
    
    /// Skip first n elements
    fn skip(self, n: usize) -> SkipIterator<T>
    where
        Self: Sized,
    {
        SkipIterator::new(self.collect(), n)
    }
    
    /// Map elements using a function
    fn map<U, F>(self, f: F) -> MapIterator<U>
    where
        Self: Sized,
        F: Fn(T) -> U,
    {
        let items = self.collect();
        MapIterator::new(items.into_iter().map(f).collect())
    }
    
    /// Filter elements using a predicate
    fn filter<P>(self, predicate: P) -> FilterIterator<T>
    where
        Self: Sized,
        P: Fn(&T) -> bool,
    {
        let items = self.collect();
        FilterIterator::new(items.into_iter().filter(predicate).collect())
    }
    
    /// Fold/reduce with accumulator
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, T) -> B,
    {
        let mut acc = init;
        let mut iter = self;
        while let Some(item) = iter.next() {
            acc = f(acc, item);
        }
        acc
    }
    
    /// Reduce to single value
    fn reduce<F>(self, mut f: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(T, T) -> T,
    {
        let mut iter = self;
        let first = iter.next()?;
        Some(iter.fold(first, f))
    }
}

/// Simple vector iterator
#[derive(Debug, Clone)]
pub struct VecIterator<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> VecIterator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items, index: 0 }
    }
}

impl<T> SimpleIterator<T> for VecIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<T> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
}

/// Range iterator
#[derive(Debug, Clone)]
pub struct RangeIterator {
    current: i32,
    end: i32,
    step: i32,
}

impl RangeIterator {
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        Self {
            current: start,
            end,
            step,
        }
    }
}

impl SimpleIterator<i32> for RangeIterator {
    fn next(&mut self) -> Option<i32> {
        if self.current >= self.end {
            None
        } else {
            let value = self.current;
            self.current += self.step;
            Some(value)
        }
    }
}

/// Take iterator
#[derive(Debug, Clone)]
pub struct TakeIterator<T> {
    items: Vec<T>,
    index: usize,
    limit: usize,
}

impl<T> TakeIterator<T> {
    pub fn new(items: Vec<T>, limit: usize) -> Self {
        Self { items, index: 0, limit }
    }
}

impl<T> SimpleIterator<T> for TakeIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<T> {
        if self.index >= self.items.len() || self.index >= self.limit {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
}

/// Skip iterator
#[derive(Debug, Clone)]
pub struct SkipIterator<T> {
    items: Vec<T>,
    index: usize,
    skip_count: usize,
}

impl<T> SkipIterator<T> {
    pub fn new(items: Vec<T>, skip_count: usize) -> Self {
        Self { items, index: skip_count, skip_count }
    }
}

impl<T> SimpleIterator<T> for SkipIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<T> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
}

/// Map iterator
#[derive(Debug, Clone)]
pub struct MapIterator<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> MapIterator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items, index: 0 }
    }
}

impl<T> SimpleIterator<T> for MapIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<T> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
}

/// Filter iterator
#[derive(Debug, Clone)]
pub struct FilterIterator<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> FilterIterator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items, index: 0 }
    }
}

impl<T> SimpleIterator<T> for FilterIterator<T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<T> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        }
    }
}

/// IntoIterator trait for converting collections
pub trait SimpleIntoIterator<T> {
    type Iterator: SimpleIterator<T>;
    fn simple_into_iter(self) -> Self::Iterator;
}

impl<T> SimpleIntoIterator<T> for Vec<T>
where
    T: Clone,
{
    type Iterator = VecIterator<T>;
    
    fn simple_into_iter(self) -> Self::Iterator {
        VecIterator::new(self)
    }
}

/// Utility functions
pub fn simple_range(start: i32, end: i32) -> RangeIterator {
    RangeIterator::new(start, end, 1)
}

pub fn simple_range_step(start: i32, end: i32, step: i32) -> RangeIterator {
    RangeIterator::new(start, end, step)
}

/// Iterator utilities
pub struct SimpleIteratorUtils;

impl SimpleIteratorUtils {
    /// Sum numeric elements
    pub fn sum<I>(iter: I) -> i32
    where
        I: SimpleIterator<i32>,
    {
        iter.fold(0, |acc, x| acc + x)
    }
    
    /// Find min element
    pub fn min<I>(iter: I) -> Option<i32>
    where
        I: SimpleIterator<i32>,
    {
        iter.reduce(|acc, x| if x < acc { x } else { acc })
    }
    
    /// Find max element
    pub fn max<I>(iter: I) -> Option<i32>
    where
        I: SimpleIterator<i32>,
    {
        iter.reduce(|acc, x| if x > acc { x } else { acc })
    }
    
    /// Partition elements
    pub fn partition<I, T, P>(iter: I, mut predicate: P) -> (Vec<T>, Vec<T>)
    where
        I: SimpleIterator<T>,
        P: FnMut(&T) -> bool,
    {
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        let mut iterator = iter;
        
        while let Some(item) = iterator.next() {
            if predicate(&item) {
                true_items.push(item);
            } else {
                false_items.push(item);
            }
        }
        
        (true_items, false_items)
    }
}

