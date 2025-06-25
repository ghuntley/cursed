use std::sync::{Arc, Mutex};
use std::marker::PhantomData;
use crate::error::Result as CursedResult;
use super::{MemoryOrder, AtomicResult, atomic_error};

/// Generic atomic value container
/// Provides type-safe atomic operations for any type T
/// Fields are not directly accessible to ensure atomic operations
#[derive(Debug)]
pub struct Value<T> {
impl<T> Value<T> 
where 
{
    /// Create a new atomic Value with initial value
    pub fn new(value: T) -> Self {
        Self {
        }
    }

    /// Create a new empty atomic Value
    pub fn new_empty() -> Self {
        Self {
        }
    }

    /// Load the current value
    /// Returns None if the value is not set or the lock is poisoned
    pub fn load(&self) -> Option<T> {
        match self.inner.lock() {
            Err(_) => None, // Return None on poison
        }
    }

    /// Load the current value, blocking until available
    /// This is the same as load() for now, but could be extended for wait semantics
    pub fn load_blocking(&self) -> Option<T> {
        self.load()
    /// Store a new value
    /// Returns false if the lock is poisoned
    pub fn store(&self, val: T) -> bool {
        match self.inner.lock() {
            Ok(mut guard) => {
                *guard = Some(val);
                true
            }
        }
    }

    /// Store a new value and return the previous value
    /// Returns None if there was no previous value or the lock is poisoned
    pub fn swap(&self, new: T) -> Option<T> {
        match self.inner.lock() {
            Ok(mut guard) => {
                let old = guard.take();
                *guard = Some(new);
                old
            }
        }
    }

    /// Compare and swap: if current value equals old, replace with new
    /// Returns true if the swap occurred
    pub fn compare_and_swap(&self, old: T, new: T) -> bool 
    where 
    {
        match self.inner.lock() {
            Ok(mut guard) => {
                if let Some(ref current) = *guard {
                    if *current == old {
                        *guard = Some(new);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    /// Compare and swap with a predicate function
    /// If the predicate returns true for the current value, replace with new
    pub fn compare_and_swap_with<F>(&self, predicate: F, new: T) -> bool 
    where 
    {
        match self.inner.lock() {
            Ok(mut guard) => {
                if let Some(ref current) = *guard {
                    if predicate(current) {
                        *guard = Some(new);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    /// Try to update the value with a function
    /// The function receives the current value and should return the new value
    /// Returns the old value if successful, None if failed
    pub fn update<F>(&self, updater: F) -> Option<T> 
    where 
    {
        match self.inner.lock() {
            Ok(mut guard) => {
                if let Some(current) = guard.take() {
                    let old = current.clone();
                    let new = updater(current);
                    *guard = Some(new);
                    Some(old)
                } else {
                    None
                }
            }
        }
    }

    /// Get a shared reference to the inner value
    /// This is not atomic but can be useful for read-only access
    /// The function receives a reference to the current value
    pub fn with<F, R>(&self, f: F) -> Option<R> 
    where 
    {
        match self.inner.lock() {
            Ok(guard) => {
                if let Some(ref value) = *guard {
                    Some(f(value))
                } else {
                    None
                }
            }
        }
    }

    /// Check if the value is set (not None)
    pub fn is_set(&self) -> bool {
        match self.inner.lock() {
        }
    }

    /// Clear the value (set to None)
    pub fn clear(&self) -> Option<T> {
        match self.inner.lock() {
        }
    }

    /// Try to set the value only if it's currently None
    /// Returns true if the value was set, false if it was already set or lock failed
    pub fn try_set(&self, value: T) -> bool {
        match self.inner.lock() {
            Ok(mut guard) => {
                if guard.is_none() {
                    *guard = Some(value);
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Get the Arc for sharing across threads
    /// This allows multiple threads to access the same atomic value
    pub fn get_arc(&self) -> Arc<Mutex<Option<T>>> {
        Arc::clone(&self.inner)
    }
}

impl<T> Clone for Value<T> 
where 
{
    fn clone(&self) -> Self {
        Self {
        }
    }
impl<T> Default for Value<T> 
where 
{
    fn default() -> Self {
        Self::new_empty()
    }
}

// Safe to send across threads
unsafe impl<T> Send for Value<T> where T: Send + Sync {}
unsafe impl<T> Sync for Value<T> where T: Send + Sync {}

/// Specialized atomic value for strings
/// Provides optimized operations for string types
pub type AtomicString = Value<String>;

/// Specialized atomic value for vectors
/// Provides optimized operations for vector types
pub type AtomicVec<T> = Value<Vec<T>>;

/// Specialized atomic value for hash maps
pub type AtomicHashMap<K, V> = Value<std::collections::HashMap<K, V>>;

/// Helper functions for creating common atomic values
impl AtomicString {
    /// Create a new atomic string with initial value
    pub fn from_str(s: &str) -> Self {
        Value::new(s.to_string())
    /// Get the length of the string atomically
    pub fn len(&self) -> Option<usize> {
        self.with(|s| s.len())
    /// Check if the string is empty atomically
    pub fn is_empty(&self) -> bool {
        self.with(|s| s.is_empty()).unwrap_or(true)
    /// Append to the string atomically
    pub fn push_str(&self, s: &str) -> bool {
        match self.inner.lock() {
            Ok(mut guard) => {
                if let Some(ref mut string) = *guard {
                    string.push_str(s);
                    true
                } else {
                    false
                }
            }
        }
    }
impl<T> AtomicVec<T> 
where 
{
    /// Create a new atomic vector
    pub fn new_vec() -> Self {
        Value::new(Vec::new())
    /// Get the length of the vector atomically
    pub fn len(&self) -> Option<usize> {
        self.with(|v| v.len())
    /// Check if the vector is empty atomically
    pub fn is_empty(&self) -> bool {
        self.with(|v| v.is_empty()).unwrap_or(true)
    /// Push an element to the vector atomically
    pub fn push(&self, item: T) -> bool {
        match self.inner.lock() {
            Ok(mut guard) => {
                if let Some(ref mut vec) = *guard {
                    vec.push(item);
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Pop an element from the vector atomically
    pub fn pop(&self) -> Option<T> {
        match self.inner.lock() {
            Ok(mut guard) => {
                if let Some(ref mut vec) = *guard {
                    vec.pop()
                } else {
                    None
                }
            }
        }
    }
