use std::sync::atomic::{AtomicI32, AtomicI64, AtomicU32, AtomicU64, AtomicBool, AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};
use std::ptr;
use crate::error::Result as CursedResult;
use super::{MemoryOrder, AtomicResult, atomic_error};

/// Atomic 32-bit signed integer
/// Fields are not directly accessible to ensure atomic operations
#[derive(Debug)]
pub struct Int32 {
    inner: AtomicI32,
}

impl Int32 {
    /// Create a new atomic Int32 with initial value
    pub fn new(value: i32) -> Self {
        Self {
            inner: AtomicI32::new(value),
        }
    }

    /// Load the current value
    pub fn load(&self) -> i32 {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    /// Load the current value with specified memory ordering
    pub fn load_ordered(&self, order: MemoryOrder) -> i32 {
        self.inner.load(order.to_std_ordering())
    }

    /// Store a new value
    pub fn store(&self, val: i32) {
        self.store_ordered(val, MemoryOrder::SequentiallyConsistent)
    }

    /// Store a new value with specified memory ordering
    pub fn store_ordered(&self, val: i32, order: MemoryOrder) {
        self.inner.store(val, order.to_std_ordering())
    }

    /// Add a delta to the current value and return the previous value
    pub fn add(&self, delta: i32) -> i32 {
        self.add_ordered(delta, MemoryOrder::SequentiallyConsistent)
    }

    /// Add a delta with specified memory ordering
    pub fn add_ordered(&self, delta: i32, order: MemoryOrder) -> i32 {
        self.inner.fetch_add(delta, order.to_std_ordering())
    }

    /// Swap the current value with a new value and return the previous value
    pub fn swap(&self, new: i32) -> i32 {
        self.swap_ordered(new, MemoryOrder::SequentiallyConsistent)
    }

    /// Swap with specified memory ordering
    pub fn swap_ordered(&self, new: i32, order: MemoryOrder) -> i32 {
        self.inner.swap(new, order.to_std_ordering())
    }

    /// Compare and swap: if current value equals old, replace with new
    /// Returns true if the swap occurred
    pub fn compare_and_swap(&self, old: i32, new: i32) -> bool {
        self.compare_and_swap_ordered(old, new, MemoryOrder::SequentiallyConsistent)
    }

    /// Compare and swap with specified memory ordering
    pub fn compare_and_swap_ordered(&self, old: i32, new: i32, order: MemoryOrder) -> bool {
        let failure_order = order.cas_failure_ordering();
        self.inner.compare_exchange(old, new, order.to_std_ordering(), failure_order.to_std_ordering()).is_ok()
    }
}

impl Default for Int32 {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Atomic 64-bit signed integer
#[derive(Debug)]
pub struct Int64 {
    inner: AtomicI64,
}

impl Int64 {
    pub fn new(value: i64) -> Self {
        Self {
            inner: AtomicI64::new(value),
        }
    }

    pub fn load(&self) -> i64 {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    pub fn load_ordered(&self, order: MemoryOrder) -> i64 {
        self.inner.load(order.to_std_ordering())
    }

    pub fn store(&self, val: i64) {
        self.store_ordered(val, MemoryOrder::SequentiallyConsistent)
    }

    pub fn store_ordered(&self, val: i64, order: MemoryOrder) {
        self.inner.store(val, order.to_std_ordering())
    }

    pub fn add(&self, delta: i64) -> i64 {
        self.add_ordered(delta, MemoryOrder::SequentiallyConsistent)
    }

    pub fn add_ordered(&self, delta: i64, order: MemoryOrder) -> i64 {
        self.inner.fetch_add(delta, order.to_std_ordering())
    }

    pub fn swap(&self, new: i64) -> i64 {
        self.swap_ordered(new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn swap_ordered(&self, new: i64, order: MemoryOrder) -> i64 {
        self.inner.swap(new, order.to_std_ordering())
    }

    pub fn compare_and_swap(&self, old: i64, new: i64) -> bool {
        self.compare_and_swap_ordered(old, new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn compare_and_swap_ordered(&self, old: i64, new: i64, order: MemoryOrder) -> bool {
        let failure_order = order.cas_failure_ordering();
        self.inner.compare_exchange(old, new, order.to_std_ordering(), failure_order.to_std_ordering()).is_ok()
    }
}

impl Default for Int64 {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Atomic 32-bit unsigned integer
#[derive(Debug)]
pub struct Uint32 {
    inner: AtomicU32,
}

impl Uint32 {
    pub fn new(value: u32) -> Self {
        Self {
            inner: AtomicU32::new(value),
        }
    }

    pub fn load(&self) -> u32 {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    pub fn load_ordered(&self, order: MemoryOrder) -> u32 {
        self.inner.load(order.to_std_ordering())
    }

    pub fn store(&self, val: u32) {
        self.store_ordered(val, MemoryOrder::SequentiallyConsistent)
    }

    pub fn store_ordered(&self, val: u32, order: MemoryOrder) {
        self.inner.store(val, order.to_std_ordering())
    }

    pub fn add(&self, delta: u32) -> u32 {
        self.add_ordered(delta, MemoryOrder::SequentiallyConsistent)
    }

    pub fn add_ordered(&self, delta: u32, order: MemoryOrder) -> u32 {
        self.inner.fetch_add(delta, order.to_std_ordering())
    }

    pub fn swap(&self, new: u32) -> u32 {
        self.swap_ordered(new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn swap_ordered(&self, new: u32, order: MemoryOrder) -> u32 {
        self.inner.swap(new, order.to_std_ordering())
    }

    pub fn compare_and_swap(&self, old: u32, new: u32) -> bool {
        self.compare_and_swap_ordered(old, new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn compare_and_swap_ordered(&self, old: u32, new: u32, order: MemoryOrder) -> bool {
        let failure_order = order.cas_failure_ordering();
        self.inner.compare_exchange(old, new, order.to_std_ordering(), failure_order.to_std_ordering()).is_ok()
    }
}

impl Default for Uint32 {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Atomic 64-bit unsigned integer
#[derive(Debug)]
pub struct Uint64 {
    inner: AtomicU64,
}

impl Uint64 {
    pub fn new(value: u64) -> Self {
        Self {
            inner: AtomicU64::new(value),
        }
    }

    pub fn load(&self) -> u64 {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    pub fn load_ordered(&self, order: MemoryOrder) -> u64 {
        self.inner.load(order.to_std_ordering())
    }

    pub fn store(&self, val: u64) {
        self.store_ordered(val, MemoryOrder::SequentiallyConsistent)
    }

    pub fn store_ordered(&self, val: u64, order: MemoryOrder) {
        self.inner.store(val, order.to_std_ordering())
    }

    pub fn add(&self, delta: u64) -> u64 {
        self.add_ordered(delta, MemoryOrder::SequentiallyConsistent)
    }

    pub fn add_ordered(&self, delta: u64, order: MemoryOrder) -> u64 {
        self.inner.fetch_add(delta, order.to_std_ordering())
    }

    pub fn swap(&self, new: u64) -> u64 {
        self.swap_ordered(new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn swap_ordered(&self, new: u64, order: MemoryOrder) -> u64 {
        self.inner.swap(new, order.to_std_ordering())
    }

    pub fn compare_and_swap(&self, old: u64, new: u64) -> bool {
        self.compare_and_swap_ordered(old, new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn compare_and_swap_ordered(&self, old: u64, new: u64, order: MemoryOrder) -> bool {
        let failure_order = order.cas_failure_ordering();
        self.inner.compare_exchange(old, new, order.to_std_ordering(), failure_order.to_std_ordering()).is_ok()
    }
}

impl Default for Uint64 {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Atomic boolean
#[derive(Debug)]
pub struct Bool {
    inner: AtomicBool,
}

impl Bool {
    pub fn new(value: bool) -> Self {
        Self {
            inner: AtomicBool::new(value),
        }
    }

    pub fn load(&self) -> bool {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    pub fn load_ordered(&self, order: MemoryOrder) -> bool {
        self.inner.load(order.to_std_ordering())
    }

    pub fn store(&self, val: bool) {
        self.store_ordered(val, MemoryOrder::SequentiallyConsistent)
    }

    pub fn store_ordered(&self, val: bool, order: MemoryOrder) {
        self.inner.store(val, order.to_std_ordering())
    }

    pub fn swap(&self, new: bool) -> bool {
        self.swap_ordered(new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn swap_ordered(&self, new: bool, order: MemoryOrder) -> bool {
        self.inner.swap(new, order.to_std_ordering())
    }

    pub fn compare_and_swap(&self, old: bool, new: bool) -> bool {
        self.compare_and_swap_ordered(old, new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn compare_and_swap_ordered(&self, old: bool, new: bool, order: MemoryOrder) -> bool {
        let failure_order = order.cas_failure_ordering();
        self.inner.compare_exchange(old, new, order.to_std_ordering(), failure_order.to_std_ordering()).is_ok()
    }
}

impl Default for Bool {
    fn default() -> Self {
        Self::new(false)
    }
}

/// Atomic 32-bit floating point
/// Note: Not all platforms support atomic float operations directly
#[derive(Debug)]
pub struct Float32 {
    inner: AtomicU32, // Store as bits for atomic operations
}

impl Float32 {
    pub fn new(value: f32) -> Self {
        Self {
            inner: AtomicU32::new(value.to_bits()),
        }
    }

    pub fn load(&self) -> f32 {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    pub fn load_ordered(&self, order: MemoryOrder) -> f32 {
        f32::from_bits(self.inner.load(order.to_std_ordering()))
    }

    pub fn store(&self, val: f32) {
        self.store_ordered(val, MemoryOrder::SequentiallyConsistent)
    }

    pub fn store_ordered(&self, val: f32, order: MemoryOrder) {
        self.inner.store(val.to_bits(), order.to_std_ordering())
    }

    pub fn swap(&self, new: f32) -> f32 {
        self.swap_ordered(new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn swap_ordered(&self, new: f32, order: MemoryOrder) -> f32 {
        f32::from_bits(self.inner.swap(new.to_bits(), order.to_std_ordering()))
    }

    pub fn compare_and_swap(&self, old: f32, new: f32) -> bool {
        self.compare_and_swap_ordered(old, new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn compare_and_swap_ordered(&self, old: f32, new: f32, order: MemoryOrder) -> bool {
        let failure_order = order.cas_failure_ordering();
        self.inner.compare_exchange(old.to_bits(), new.to_bits(), 
                                   order.to_std_ordering(), failure_order.to_std_ordering()).is_ok()
    }
}

impl Default for Float32 {
    fn default() -> Self {
        Self::new(0.0)
    }
}

/// Atomic 64-bit floating point
#[derive(Debug)]
pub struct Float64 {
    inner: AtomicU64, // Store as bits for atomic operations
}

impl Float64 {
    pub fn new(value: f64) -> Self {
        Self {
            inner: AtomicU64::new(value.to_bits()),
        }
    }

    pub fn load(&self) -> f64 {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    pub fn load_ordered(&self, order: MemoryOrder) -> f64 {
        f64::from_bits(self.inner.load(order.to_std_ordering()))
    }

    pub fn store(&self, val: f64) {
        self.store_ordered(val, MemoryOrder::SequentiallyConsistent)
    }

    pub fn store_ordered(&self, val: f64, order: MemoryOrder) {
        self.inner.store(val.to_bits(), order.to_std_ordering())
    }

    pub fn swap(&self, new: f64) -> f64 {
        self.swap_ordered(new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn swap_ordered(&self, new: f64, order: MemoryOrder) -> f64 {
        f64::from_bits(self.inner.swap(new.to_bits(), order.to_std_ordering()))
    }

    pub fn compare_and_swap(&self, old: f64, new: f64) -> bool {
        self.compare_and_swap_ordered(old, new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn compare_and_swap_ordered(&self, old: f64, new: f64, order: MemoryOrder) -> bool {
        let failure_order = order.cas_failure_ordering();
        self.inner.compare_exchange(old.to_bits(), new.to_bits(), 
                                   order.to_std_ordering(), failure_order.to_std_ordering()).is_ok()
    }
}

impl Default for Float64 {
    fn default() -> Self {
        Self::new(0.0)
    }
}

/// Atomic string
/// Uses Arc<Mutex<String>> for thread-safe string operations
#[derive(Debug)]
pub struct String {
    inner: Arc<Mutex<std::string::String>>,
}

impl String {
    pub fn new(value: std::string::String) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
        }
    }

    pub fn load(&self) -> std::string::String {
        match self.inner.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => std::string::String::new(), // Return empty string on poison
        }
    }

    pub fn store(&self, val: std::string::String) {
        if let Ok(mut guard) = self.inner.lock() {
            *guard = val;
        }
    }

    pub fn swap(&self, new: std::string::String) -> std::string::String {
        match self.inner.lock() {
            Ok(mut guard) => {
                let old = guard.clone();
                *guard = new;
                old
            }
            Err(_) => std::string::String::new(),
        }
    }

    pub fn compare_and_swap(&self, old: std::string::String, new: std::string::String) -> bool {
        match self.inner.lock() {
            Ok(mut guard) => {
                if *guard == old {
                    *guard = new;
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new(std::string::String::new())
    }
}

/// Atomic pointer
#[derive(Debug)]
pub struct Pointer<T> {
    inner: AtomicPtr<T>,
}

impl<T> Pointer<T> {
    pub fn new(value: *mut T) -> Self {
        Self {
            inner: AtomicPtr::new(value),
        }
    }

    pub fn load(&self) -> *mut T {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    pub fn load_ordered(&self, order: MemoryOrder) -> *mut T {
        self.inner.load(order.to_std_ordering())
    }

    pub fn store(&self, val: *mut T) {
        self.store_ordered(val, MemoryOrder::SequentiallyConsistent)
    }

    pub fn store_ordered(&self, val: *mut T, order: MemoryOrder) {
        self.inner.store(val, order.to_std_ordering())
    }

    pub fn swap(&self, new: *mut T) -> *mut T {
        self.swap_ordered(new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn swap_ordered(&self, new: *mut T, order: MemoryOrder) -> *mut T {
        self.inner.swap(new, order.to_std_ordering())
    }

    pub fn compare_and_swap(&self, old: *mut T, new: *mut T) -> bool {
        self.compare_and_swap_ordered(old, new, MemoryOrder::SequentiallyConsistent)
    }

    pub fn compare_and_swap_ordered(&self, old: *mut T, new: *mut T, order: MemoryOrder) -> bool {
        let failure_order = order.cas_failure_ordering();
        self.inner.compare_exchange(old, new, order.to_std_ordering(), failure_order.to_std_ordering()).is_ok()
    }
}

impl<T> Default for Pointer<T> {
    fn default() -> Self {
        Self::new(ptr::null_mut())
    }
}

unsafe impl<T> Send for Pointer<T> {}
unsafe impl<T> Sync for Pointer<T> {}

