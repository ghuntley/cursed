/// Core synchronization primitives for CURSED
/// 
/// This module provides thread-safe synchronization primitives including:
/// - Thread spawning and management
/// - Mutexes and RwLocks
/// - Atomic operations
/// - Barriers and condition variables
/// - Semaphores

// use crate::stdlib::sync::error::{SyncError, SyncResult, thread_error, lock_error, timeout_error};
use crate::error::CursedError;
use std::sync::{
// };

use std::thread::{self, ThreadId as StdThreadId, JoinHandle as StdJoinHandle};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, fence};

// Global statistics for monitoring
static ACTIVE_THREAD_COUNT: StdAtomicUsize = StdAtomicUsize::new(0);
static LOCK_CONTENTION_COUNT: AtomicU64 = AtomicU64::new(0);
static TOTAL_WAIT_TIME_NANOS: AtomicU64 = AtomicU64::new(0);

//==============================================================================
// Thread Management
//==============================================================================

/// Thread identifier
pub type ThreadId = StdThreadId;

/// Thread handle for managing spawned threads
pub struct Thread {
impl Thread {
    /// Get the thread ID
    pub fn id(&self) -> ThreadId {
        self.id
    /// Get the thread name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    /// Check if the thread has finished
    pub fn is_finished(&self) -> bool {
        self.handle.as_ref().map_or(true, |h| h.is_finished())
    }
}

/// Join handle for waiting on thread completion
pub struct JoinHandle<T> {
impl<T> JoinHandle<T> {
    /// Wait for the thread to complete and return its result
    pub fn join(self) -> SyncResult<T> {
        ACTIVE_THREAD_COUNT.fetch_sub(1, StdOrdering::Relaxed);
        self.handle.join().map_err(|_| thread_error("Thread panicked"))
    /// Get the thread ID
    pub fn thread_id(&self) -> ThreadId {
        self.thread_id
    /// Check if the thread has finished
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}

/// Builder for configuring thread spawning
pub struct ThreadBuilder {
impl ThreadBuilder {
    /// Create a new thread builder
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set the thread name
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    /// Set the stack size
    pub fn stack_size(mut self, size: usize) -> Self {
        self.stack_size = Some(size);
        self
    /// Spawn a thread with the configured settings
    pub fn spawn<F, T>(self, f: F) -> SyncResult<JoinHandle<T>>
    where
    {
        let mut builder = thread::Builder::new();
        
        if let Some(name) = self.name {
            builder = builder.name(name);
        if let Some(size) = self.stack_size {
            builder = builder.stack_size(size);
        let handle = builder.spawn(f)
            .map_err(|e| thread_error(&format!("Failed to spawn thread: {}", e)))?;
        
        let thread_id = handle.thread().id();
        ACTIVE_THREAD_COUNT.fetch_add(1, StdOrdering::Relaxed);

        Ok(JoinHandle { handle, thread_id })
    }
}

impl Default for ThreadBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawn a new thread
pub fn spawn<F, T>(f: F) -> SyncResult<JoinHandle<T>>
where
{
    ThreadBuilder::new().spawn(f)
/// Spawn a named thread
pub fn spawn_named<F, T>(name: &str, f: F) -> SyncResult<JoinHandle<T>>
where
{
    ThreadBuilder::new().name(name.to_string()).spawn(f)
/// Get the current thread ID
pub fn current_thread_id() -> ThreadId {
    thread::current().id()
/// Get the current thread name
pub fn current_thread_name() -> Option<String> {
    thread::current().name().map(|s| s.to_string())
/// Sleep for the specified duration
pub fn sleep(duration: Duration) {
    thread::sleep(duration);
/// Yield execution to other threads
pub fn yield_now() {
        // TODO: implement
    }
    thread::yield_now();
/// Park the current thread
pub fn park() {
        // TODO: implement
    }
    thread::park();
/// Park the current thread with a timeout
pub fn park_timeout(timeout: Duration) {
    thread::park_timeout(timeout);
/// Unpark a thread
pub fn unpark(thread: &Thread) {
    if let Some(handle) = &thread.handle {
        handle.thread().unpark();
    }
}

//==============================================================================
// Mutex
//==============================================================================

/// A mutual exclusion primitive useful for protecting shared data
pub struct Mutex<T> {
impl<T> Mutex<T> {
    /// Create a new mutex
    pub fn new(data: T) -> Self {
        Self {
        }
    }

    /// Create a new named mutex for debugging
    pub fn named(data: T, name: &str) -> Self {
        Self {
        }
    }

    /// Lock the mutex, blocking until available
    pub fn lock(&self) -> SyncResult<MutexGuard<T>> {
        let start = Instant::now();
        
        match self.inner.lock() {
            Ok(guard) => {
                let wait_time = start.elapsed();
                if wait_time > Duration::from_millis(1) {
                    LOCK_CONTENTION_COUNT.fetch_add(1, StdOrdering::Relaxed);
                    TOTAL_WAIT_TIME_NANOS.fetch_add(wait_time.as_nanos() as u64, StdOrdering::Relaxed);
                }
                Ok(MutexGuard { guard })
            }
            Err(_) => Err(lock_error("mutex", "lock"))
        }
    }

    /// Try to lock the mutex without blocking
    pub fn try_lock(&self) -> SyncResult<MutexGuard<T>> {
        match self.inner.try_lock() {
            Err(_) => Err(lock_error("mutex", "try_lock"))
        }
    }

    /// Get the name of the mutex
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// A guard that provides access to the data protected by a Mutex
pub struct MutexGuard<'a, T> {
impl<'a, T> std::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T> std::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

//==============================================================================
// RwLock
//==============================================================================

/// A reader-writer lock
pub struct RwLock<T> {
impl<T> RwLock<T> {
    /// Create a new RwLock
    pub fn new(data: T) -> Self {
        Self {
        }
    }

    /// Create a new named RwLock for debugging
    pub fn named(data: T, name: &str) -> Self {
        Self {
        }
    }

    /// Acquire a read lock
    pub fn read(&self) -> SyncResult<RwLockReadGuard<T>> {
        let start = Instant::now();
        
        match self.inner.read() {
            Ok(guard) => {
                let wait_time = start.elapsed();
                if wait_time > Duration::from_millis(1) {
                    LOCK_CONTENTION_COUNT.fetch_add(1, StdOrdering::Relaxed);
                    TOTAL_WAIT_TIME_NANOS.fetch_add(wait_time.as_nanos() as u64, StdOrdering::Relaxed);
                }
                Ok(RwLockReadGuard { guard })
            }
            Err(_) => Err(lock_error("rwlock", "read"))
        }
    }

    /// Try to acquire a read lock without blocking
    pub fn try_read(&self) -> SyncResult<RwLockReadGuard<T>> {
        match self.inner.try_read() {
            Err(_) => Err(lock_error("rwlock", "try_read"))
        }
    }

    /// Acquire a write lock
    pub fn write(&self) -> SyncResult<RwLockWriteGuard<T>> {
        let start = Instant::now();
        
        match self.inner.write() {
            Ok(guard) => {
                let wait_time = start.elapsed();
                if wait_time > Duration::from_millis(1) {
                    LOCK_CONTENTION_COUNT.fetch_add(1, StdOrdering::Relaxed);
                    TOTAL_WAIT_TIME_NANOS.fetch_add(wait_time.as_nanos() as u64, StdOrdering::Relaxed);
                }
                Ok(RwLockWriteGuard { guard })
            }
            Err(_) => Err(lock_error("rwlock", "write"))
        }
    }

    /// Try to acquire a write lock without blocking
    pub fn try_write(&self) -> SyncResult<RwLockWriteGuard<T>> {
        match self.inner.try_write() {
            Err(_) => Err(lock_error("rwlock", "try_write"))
        }
    }

    /// Get the name of the RwLock
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// Read guard for RwLock
pub struct RwLockReadGuard<'a, T> {
impl<'a, T> std::ops::Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

/// Write guard for RwLock  
pub struct RwLockWriteGuard<'a, T> {
impl<'a, T> std::ops::Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T> std::ops::DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

//==============================================================================
// Atomic Operations
//==============================================================================

pub use std::sync::atomic::Ordering;

/// Atomic boolean
pub struct AtomicBool {
impl AtomicBool {
    /// Create a new atomic boolean
    pub fn new(value: bool) -> Self {
        Self { inner: StdAtomicBool::new(value) }
    }

    /// Load the value
    pub fn load(&self, ordering: Ordering) -> bool {
        self.inner.load(ordering)
    /// Store a value
    pub fn store(&self, value: bool, ordering: Ordering) {
        self.inner.store(value, ordering)
    /// Swap values
    pub fn swap(&self, value: bool, ordering: Ordering) -> bool {
        self.inner.swap(value, ordering)
    /// Compare and swap
    pub fn compare_and_swap(&self, current: bool, new: bool, ordering: Ordering) -> bool {
        self.inner.compare_and_swap(current, new, ordering)
    /// Fetch and update with boolean AND
    pub fn fetch_and(&self, value: bool, ordering: Ordering) -> bool {
        self.inner.fetch_and(value, ordering)
    /// Fetch and update with boolean OR
    pub fn fetch_or(&self, value: bool, ordering: Ordering) -> bool {
        self.inner.fetch_or(value, ordering)
    /// Fetch and update with boolean XOR
    pub fn fetch_xor(&self, value: bool, ordering: Ordering) -> bool {
        self.inner.fetch_xor(value, ordering)
    }
}

/// Atomic 32-bit integer
pub struct AtomicI32 {
impl AtomicI32 {
    /// Create a new atomic i32
    pub fn new(value: i32) -> Self {
        Self { inner: StdAtomicI32::new(value) }
    }

    /// Load the value
    pub fn load(&self, ordering: Ordering) -> i32 {
        self.inner.load(ordering)
    /// Store a value
    pub fn store(&self, value: i32, ordering: Ordering) {
        self.inner.store(value, ordering)
    /// Swap values
    pub fn swap(&self, value: i32, ordering: Ordering) -> i32 {
        self.inner.swap(value, ordering)
    /// Compare and swap
    pub fn compare_and_swap(&self, current: i32, new: i32, ordering: Ordering) -> i32 {
        self.inner.compare_and_swap(current, new, ordering)
    /// Fetch and add
    pub fn fetch_add(&self, value: i32, ordering: Ordering) -> i32 {
        self.inner.fetch_add(value, ordering)
    /// Fetch and subtract
    pub fn fetch_sub(&self, value: i32, ordering: Ordering) -> i32 {
        self.inner.fetch_sub(value, ordering)
    /// Fetch and bitwise AND
    pub fn fetch_and(&self, value: i32, ordering: Ordering) -> i32 {
        self.inner.fetch_and(value, ordering)
    /// Fetch and bitwise OR
    pub fn fetch_or(&self, value: i32, ordering: Ordering) -> i32 {
        self.inner.fetch_or(value, ordering)
    /// Fetch and bitwise XOR
    pub fn fetch_xor(&self, value: i32, ordering: Ordering) -> i32 {
        self.inner.fetch_xor(value, ordering)
    /// Increment by 1 and return previous value
    pub fn fetch_increment(&self) -> i32 {
        self.fetch_add(1, Ordering::SeqCst)
    /// Decrement by 1 and return previous value  
    pub fn fetch_decrement(&self) -> i32 {
        self.fetch_sub(1, Ordering::SeqCst)
    }
}

/// Atomic 64-bit integer
pub struct AtomicI64 {
impl AtomicI64 {
    /// Create a new atomic i64
    pub fn new(value: i64) -> Self {
        Self { inner: StdAtomicI64::new(value) }
    }

    /// Load the value
    pub fn load(&self, ordering: Ordering) -> i64 {
        self.inner.load(ordering)
    /// Store a value
    pub fn store(&self, value: i64, ordering: Ordering) {
        self.inner.store(value, ordering)
    /// Swap values
    pub fn swap(&self, value: i64, ordering: Ordering) -> i64 {
        self.inner.swap(value, ordering)
    /// Compare and swap
    pub fn compare_and_swap(&self, current: i64, new: i64, ordering: Ordering) -> i64 {
        self.inner.compare_and_swap(current, new, ordering)
    /// Fetch and add
    pub fn fetch_add(&self, value: i64, ordering: Ordering) -> i64 {
        self.inner.fetch_add(value, ordering)
    /// Fetch and subtract
    pub fn fetch_sub(&self, value: i64, ordering: Ordering) -> i64 {
        self.inner.fetch_sub(value, ordering)
    /// Fetch and bitwise AND
    pub fn fetch_and(&self, value: i64, ordering: Ordering) -> i64 {
        self.inner.fetch_and(value, ordering)
    /// Fetch and bitwise OR
    pub fn fetch_or(&self, value: i64, ordering: Ordering) -> i64 {
        self.inner.fetch_or(value, ordering)
    /// Fetch and bitwise XOR
    pub fn fetch_xor(&self, value: i64, ordering: Ordering) -> i64 {
        self.inner.fetch_xor(value, ordering)
    /// Increment by 1 and return previous value
    pub fn fetch_increment(&self) -> i64 {
        self.fetch_add(1, Ordering::SeqCst)
    /// Decrement by 1 and return previous value
    pub fn fetch_decrement(&self) -> i64 {
        self.fetch_sub(1, Ordering::SeqCst)
    }
}

/// Atomic usize
pub struct AtomicUsize {
impl AtomicUsize {
    /// Create a new atomic usize
    pub fn new(value: usize) -> Self {
        Self { inner: StdAtomicUsize::new(value) }
    }

    /// Load the value
    pub fn load(&self, ordering: Ordering) -> usize {
        self.inner.load(ordering)
    /// Store a value
    pub fn store(&self, value: usize, ordering: Ordering) {
        self.inner.store(value, ordering)
    /// Swap values
    pub fn swap(&self, value: usize, ordering: Ordering) -> usize {
        self.inner.swap(value, ordering)
    /// Compare and swap
    pub fn compare_and_swap(&self, current: usize, new: usize, ordering: Ordering) -> usize {
        self.inner.compare_and_swap(current, new, ordering)
    /// Fetch and add
    pub fn fetch_add(&self, value: usize, ordering: Ordering) -> usize {
        self.inner.fetch_add(value, ordering)
    /// Fetch and subtract
    pub fn fetch_sub(&self, value: usize, ordering: Ordering) -> usize {
        self.inner.fetch_sub(value, ordering)
    /// Fetch and bitwise AND
    pub fn fetch_and(&self, value: usize, ordering: Ordering) -> usize {
        self.inner.fetch_and(value, ordering)
    /// Fetch and bitwise OR
    pub fn fetch_or(&self, value: usize, ordering: Ordering) -> usize {
        self.inner.fetch_or(value, ordering)
    /// Fetch and bitwise XOR
    pub fn fetch_xor(&self, value: usize, ordering: Ordering) -> usize {
        self.inner.fetch_xor(value, ordering)
    /// Increment by 1 and return previous value
    pub fn fetch_increment(&self) -> usize {
        self.fetch_add(1, Ordering::SeqCst)
    /// Decrement by 1 and return previous value
    pub fn fetch_decrement(&self) -> usize {
        self.fetch_sub(1, Ordering::SeqCst)
    }
}

/// Atomic pointer
pub struct AtomicPtr<T> {
impl<T> AtomicPtr<T> {
    /// Create a new atomic pointer
    pub fn new(ptr: *mut T) -> Self {
        Self { inner: StdAtomicPtr::new(ptr) }
    }

    /// Load the pointer
    pub fn load(&self, ordering: Ordering) -> *mut T {
        self.inner.load(ordering)
    /// Store a pointer
    pub fn store(&self, ptr: *mut T, ordering: Ordering) {
        self.inner.store(ptr, ordering)
    /// Swap pointers
    pub fn swap(&self, ptr: *mut T, ordering: Ordering) -> *mut T {
        self.inner.swap(ptr, ordering)
    /// Compare and swap
    pub fn compare_and_swap(&self, current: *mut T, new: *mut T, ordering: Ordering) -> *mut T {
        self.inner.compare_and_swap(current, new, ordering)
    }
}

/// Memory fence
pub fn memory_fence(ordering: Ordering) {
    std::sync::atomic::fence(ordering);
/// Compiler fence
pub fn compiler_fence(ordering: Ordering) {
    std::sync::atomic::compiler_fence(ordering);
//==============================================================================
// Condition Variable
//==============================================================================

/// A condition variable
pub struct CondVar {
impl CondVar {
    /// Create a new condition variable
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new named condition variable
    pub fn named(name: &str) -> Self {
        Self {
        }
    }

    /// Wait on the condition variable
    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> SyncResult<MutexGuard<'a, T>> {
        match self.inner.wait(guard.guard) {
            Err(_) => Err(lock_error("condvar", "wait"))
        }
    }

    /// Wait on the condition variable with a timeout
    pub fn wait_timeout<'a, T>(&self, guard: MutexGuard<'a, T>, timeout: Duration) -> SyncResult<(MutexGuard<'a, T>, bool)> {
        match self.inner.wait_timeout(guard.guard, timeout) {
            Err(_) => Err(timeout_error("condvar wait", timeout))
        }
    }

    /// Notify one waiting thread
    pub fn notify_one(&self) {
        self.inner.notify_one();
    /// Notify all waiting threads
    pub fn notify_all(&self) {
        self.inner.notify_all();
    /// Get the name of the condition variable
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Default for CondVar {
    fn default() -> Self {
        Self::new()
    }
}

//==============================================================================
// Barrier
//==============================================================================

/// A barrier for synchronizing multiple threads
pub struct Barrier {
impl Barrier {
    /// Create a new barrier
    pub fn new(n: usize) -> Self {
        Self {
        }
    }

    /// Create a new named barrier
    pub fn named(n: usize, name: &str) -> Self {
        Self {
        }
    }

    /// Wait at the barrier
    pub fn wait(&self) -> BarrierWaitResult {
        BarrierWaitResult {
        }
    }

    /// Get the name of the barrier
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// Result of waiting at a barrier
pub struct BarrierWaitResult {
impl BarrierWaitResult {
    /// Returns true if this thread was the last to reach the barrier
    pub fn is_leader(&self) -> bool {
        self.result.is_leader()
    }
}

//==============================================================================
// Semaphore
//==============================================================================

/// A counting semaphore
pub struct Semaphore {
impl Semaphore {
    /// Create a new semaphore with the given number of permits
    pub fn new(permits: usize) -> Self {
        Self {
        }
    }

    /// Create a new named semaphore
    pub fn named(permits: usize, name: &str) -> Self {
        Self {
        }
    }

    /// Acquire a permit
    pub fn acquire(&self) -> SyncResult<SemaphoreGuard> {
        loop {
            let current = self.permits.load(Ordering::Acquire);
            if current > 0 {
                if self.permits.compare_and_swap(current, current - 1, Ordering::Release) == current {
                    return Ok(SemaphoreGuard {
                    });
                }
            } else {
                // Wait for permits to become available
                let _guard = self.mutex.lock().map_err(|_| lock_error("semaphore", "acquire"))?;
                let _guard = self.waiters.wait(_guard).map_err(|_| lock_error("semaphore", "wait"))?;
            }
        }
    /// Try to acquire a permit without blocking
    pub fn try_acquire(&self) -> SyncResult<Option<SemaphoreGuard>> {
        let current = self.permits.load(Ordering::Acquire);
        if current > 0 {
            if self.permits.compare_and_swap(current, current - 1, Ordering::Release) == current {
                return Ok(Some(SemaphoreGuard {
                }));
            }
        }
        Ok(None)
    /// Acquire a permit with a timeout
    pub fn acquire_timeout(&self, timeout: Duration) -> SyncResult<Option<SemaphoreGuard>> {
        let start = Instant::now();
        
        loop {
            let current = self.permits.load(Ordering::Acquire);
            if current > 0 {
                if self.permits.compare_and_swap(current, current - 1, Ordering::Release) == current {
                    return Ok(Some(SemaphoreGuard {
                    }));
                }
            } else {
                if start.elapsed() >= timeout {
                    return Ok(None);
                // Wait with timeout
                let guard = self.mutex.lock().map_err(|_| lock_error("semaphore", "acquire_timeout"))?;
                let remaining = timeout.saturating_sub(start.elapsed());
                let _ = self.waiters.wait_timeout(guard, remaining).map_err(|_| timeout_error("semaphore acquire", timeout))?;
            }
        }
    /// Release a permit
    fn release(&self) {
        self.permits.fetch_add(1, Ordering::Release);
        self.waiters.notify_one();
    /// Get available permits
    pub fn available_permits(&self) -> usize {
        self.permits.load(Ordering::Acquire)
    /// Get the name of the semaphore
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// RAII guard for semaphore permits
pub struct SemaphoreGuard<'a> {
impl<'a> Drop for SemaphoreGuard<'a> {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

//==============================================================================
// Once and Lazy Initialization
//==============================================================================

/// A synchronization primitive which can be used to run one-time initialization
pub struct Once {
impl Once {
    /// Create a new Once
    pub const fn new() -> Self {
        Self {
        }
    }

    /// Perform initialization exactly once
    pub fn call_once<F>(&self, f: F)
    where
    {
        self.inner.call_once(f);
    /// Check if initialization has been completed
    pub fn is_completed(&self) -> bool {
        self.inner.is_completed()
    }
}

/// A thread-safe cell which can be written to only once
pub struct OnceCell<T> {
impl<T> OnceCell<T> {
    /// Create a new empty OnceCell
    pub const fn new() -> Self {
        Self {
        }
    }

    /// Get the value, initializing it if necessary
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
    {
        self.inner.get_or_init(f)
    /// Try to get the value without initializing
    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    /// Set the value if it hasn't been set already
    pub fn set(&self, value: T) -> Result<(), T> {
        self.inner.set(value)
    }
}

impl<T> Default for OnceCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A value which is initialized on the first access
pub struct Lazy<T> {
impl<T> Lazy<T> {
    /// Create a new lazy value
    pub const fn new(init: fn() -> T) -> Self {
        Self {
        }
    }

    /// Get the value, initializing if necessary
    pub fn get(&self) -> &T {
        self.cell.get_or_init(self.init)
    }
}

impl<T> std::ops::Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

//==============================================================================
// Statistics and Monitoring
//==============================================================================

/// Get the number of active threads
pub fn get_active_thread_count() -> usize {
    ACTIVE_THREAD_COUNT.load(StdOrdering::Relaxed)
/// Get lock contention statistics
// pub fn get_lock_contention_stats() -> crate::stdlib::sync::LockContentionStats {
    let contentions = LOCK_CONTENTION_COUNT.load(StdOrdering::Relaxed);
    let total_wait_time = TOTAL_WAIT_TIME_NANOS.load(StdOrdering::Relaxed);
    
    let avg_wait_time = if contentions > 0 {
        total_wait_time / contentions
    } else {
        0

//     crate::stdlib::sync::LockContentionStats {
        rwlock_contentions: contentions, // Simplified for now
    }
}

