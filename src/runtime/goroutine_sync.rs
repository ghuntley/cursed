//! Goroutine synchronization primitives for the CURSED language runtime
//!
//! This module provides thread-safe synchronization utilities specifically designed
//! for use with goroutines, including WaitGroup, Mutex, atomic operations, and
//! condition variables. These primitives enable safe coordination between goroutines
//! and prevent race conditions and deadlocks.

use std::sync::{Arc, Mutex as StdMutex, Condvar, atomic::{AtomicUsize, AtomicBool, AtomicI64, Ordering}};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::thread::{self, Thread, ThreadId};
use std::sync::mpsc::{self, Sender, Receiver};
use tracing::{instrument, debug, warn, error, info};
use crate::runtime::goroutine::{get_global_scheduler, GoroutineScheduler};

/// Errors that can occur during synchronization operations
#[derive(Debug, Clone)]
pub enum SyncError {
    /// Operation timed out
    Timeout,
    /// Operation was cancelled
    Cancelled,
    /// Lock acquisition failed
    LockFailed(String),
    /// Invalid state for operation
    InvalidState(String),
    /// Resource was already closed
    Closed,
    /// Deadlock detected
    Deadlock,
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncError::Timeout => write!(f, "Operation timed out"),
            SyncError::Cancelled => write!(f, "Operation was cancelled"),
            SyncError::LockFailed(msg) => write!(f, "Lock acquisition failed: {}", msg),
            SyncError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            SyncError::Closed => write!(f, "Resource was already closed"),
            SyncError::Deadlock => write!(f, "Deadlock detected"),
        }
    }
}

impl std::error::Error for SyncError {}

/// Result type for synchronization operations
pub type SyncResult<T> = Result<T, SyncError>;

/// WaitGroup implementation for goroutine coordination
/// 
/// Equivalent to Go's sync.WaitGroup, allows goroutines to wait for
/// a collection of other goroutines to complete.
#[derive(Debug)]
pub struct WaitGroup {
    counter: AtomicI64,
    condvar: Condvar,
    mutex: StdMutex<()>,
    closed: AtomicBool,
}

impl WaitGroup {
    /// Create a new WaitGroup
    pub fn new() -> Self {
        Self {
            counter: AtomicI64::new(0),
            condvar: Condvar::new(),
            mutex: StdMutex::new(()),
            closed: AtomicBool::new(false),
        }
    }

    /// Add delta to the WaitGroup counter
    /// 
    /// If the counter becomes zero, all goroutines blocked on Wait() are released.
    /// If the counter goes negative, this method panics.
    #[instrument(level = "debug", skip(self))]
    pub fn add(&self, delta: i64) -> SyncResult<()> {
        if self.closed.load(Ordering::Acquire) {
            return Err(SyncError::Closed);
        }

        let old_count = self.counter.fetch_add(delta, Ordering::SeqCst);
        let new_count = old_count + delta;

        debug!(old_count = old_count, delta = delta, new_count = new_count, "WaitGroup add");

        if new_count < 0 {
            // Reset counter to prevent corruption
            self.counter.store(0, Ordering::SeqCst);
            return Err(SyncError::InvalidState("WaitGroup counter went negative".to_string()));
        }

        if new_count == 0 {
            // Wake up all waiting goroutines
            self.condvar.notify_all();
            debug!("WaitGroup reached zero, notifying all waiters");
        }

        Ok(())
    }

    /// Increment the WaitGroup counter by one
    pub fn add_one(&self) -> SyncResult<()> {
        self.add(1)
    }

    /// Decrement the WaitGroup counter by one
    pub fn done(&self) -> SyncResult<()> {
        self.add(-1)
    }

    /// Wait for the WaitGroup counter to reach zero
    #[instrument(level = "debug", skip(self))]
    pub fn wait(&self) -> SyncResult<()> {
        let guard = self.mutex.lock()
            .map_err(|e| SyncError::LockFailed(format!("WaitGroup mutex poisoned: {}", e)))?;

        let mut guard = guard;
        while self.counter.load(Ordering::Acquire) > 0 {
            if self.closed.load(Ordering::Acquire) {
                return Err(SyncError::Closed);
            }
            
            guard = self.condvar.wait(guard)
                .map_err(|e| SyncError::LockFailed(format!("WaitGroup condvar poisoned: {}", e)))?;
        }

        debug!("WaitGroup wait completed");
        Ok(())
    }

    /// Wait for the WaitGroup counter to reach zero with a timeout
    #[instrument(level = "debug", skip(self, timeout))]
    pub fn wait_timeout(&self, timeout: Duration) -> SyncResult<()> {
        let guard = self.mutex.lock()
            .map_err(|e| SyncError::LockFailed(format!("WaitGroup mutex poisoned: {}", e)))?;

        let start = Instant::now();
        let mut guard = guard;
        
        while self.counter.load(Ordering::Acquire) > 0 {
            if self.closed.load(Ordering::Acquire) {
                return Err(SyncError::Closed);
            }

            let elapsed = start.elapsed();
            if elapsed >= timeout {
                debug!(elapsed = ?elapsed, timeout = ?timeout, "WaitGroup wait timed out");
                return Err(SyncError::Timeout);
            }

            let remaining = timeout - elapsed;
            let (new_guard, timeout_result) = self.condvar.wait_timeout(guard, remaining)
                .map_err(|e| SyncError::LockFailed(format!("WaitGroup condvar poisoned: {}", e)))?;
            
            guard = new_guard;
            
            if timeout_result.timed_out() {
                debug!("WaitGroup wait timed out");
                return Err(SyncError::Timeout);
            }
        }

        debug!("WaitGroup wait completed within timeout");
        Ok(())
    }

    /// Get the current counter value
    pub fn count(&self) -> i64 {
        self.counter.load(Ordering::Acquire)
    }

    /// Close the WaitGroup, causing all waiting operations to fail
    pub fn close(&self) {
        self.closed.store(true, Ordering::Release);
        self.condvar.notify_all();
        debug!("WaitGroup closed");
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe mutex implementation for goroutines
///
/// Provides exclusive access to shared data with timeout and try-lock capabilities
#[derive(Debug)]
pub struct GoroutineMutex<T> {
    data: StdMutex<T>,
    owner: AtomicUsize, // ThreadId as usize for atomic operations
    lock_time: AtomicI64, // Timestamp when lock was acquired
}

impl<T> GoroutineMutex<T> {
    /// Create a new GoroutineMutex
    pub fn new(data: T) -> Self {
        Self {
            data: StdMutex::new(data),
            owner: AtomicUsize::new(0),
            lock_time: AtomicI64::new(0),
        }
    }

    /// Lock the mutex, blocking until available
    #[instrument(level = "debug", skip(self))]
    pub fn lock(&self) -> SyncResult<GoroutineMutexGuard<T>> {
        let start = Instant::now();
        
        match self.data.lock() {
            Ok(guard) => {
                let thread_id = current_thread_id_as_usize();
                self.owner.store(thread_id, Ordering::Release);
                self.lock_time.store(start.elapsed().as_nanos() as i64, Ordering::Release);
                
                debug!(thread_id = thread_id, "Mutex locked");
                Ok(GoroutineMutexGuard {
                    guard,
                    mutex: self,
                })
            }
            Err(_) => Err(SyncError::LockFailed("Mutex is poisoned".to_string()))
        }
    }

    /// Try to lock the mutex without blocking
    #[instrument(level = "debug", skip(self))]
    pub fn try_lock(&self) -> SyncResult<GoroutineMutexGuard<T>> {
        match self.data.try_lock() {
            Ok(guard) => {
                let thread_id = current_thread_id_as_usize();
                self.owner.store(thread_id, Ordering::Release);
                self.lock_time.store(Instant::now().elapsed().as_nanos() as i64, Ordering::Release);
                
                debug!(thread_id = thread_id, "Mutex try_locked");
                Ok(GoroutineMutexGuard {
                    guard,
                    mutex: self,
                })
            }
            Err(std::sync::TryLockError::WouldBlock) => {
                debug!("Mutex try_lock would block");
                Err(SyncError::LockFailed("Would block".to_string()))
            }
            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(SyncError::LockFailed("Mutex is poisoned".to_string()))
            }
        }
    }

    /// Get the current owner thread ID (0 if unlocked)
    pub fn owner(&self) -> usize {
        self.owner.load(Ordering::Acquire)
    }

    /// Check if the current thread owns the mutex
    pub fn is_owned_by_current_thread(&self) -> bool {
        self.owner() == current_thread_id_as_usize()
    }
}

/// RAII guard for GoroutineMutex
pub struct GoroutineMutexGuard<'a, T> {
    guard: std::sync::MutexGuard<'a, T>,
    mutex: &'a GoroutineMutex<T>,
}

impl<T> std::ops::Deref for GoroutineMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> std::ops::DerefMut for GoroutineMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl<T> Drop for GoroutineMutexGuard<'_, T> {
    fn drop(&mut self) {
        let thread_id = current_thread_id_as_usize();
        self.mutex.owner.store(0, Ordering::Release);
        self.mutex.lock_time.store(0, Ordering::Release);
        debug!(thread_id = thread_id, "Mutex unlocked");
    }
}

/// Atomic operations support for goroutine-safe data structures
#[derive(Debug)]
pub struct AtomicCounter {
    value: AtomicI64,
    operations: AtomicUsize, // Track number of operations for debugging
}

impl AtomicCounter {
    /// Create a new atomic counter
    pub fn new(initial: i64) -> Self {
        Self {
            value: AtomicI64::new(initial),
            operations: AtomicUsize::new(0),
        }
    }

    /// Get the current value
    pub fn get(&self) -> i64 {
        self.operations.fetch_add(1, Ordering::Relaxed);
        self.value.load(Ordering::Acquire)
    }

    /// Set a new value
    pub fn set(&self, value: i64) {
        self.operations.fetch_add(1, Ordering::Relaxed);
        self.value.store(value, Ordering::Release);
    }

    /// Atomically add to the value and return the new value
    pub fn add(&self, delta: i64) -> i64 {
        self.operations.fetch_add(1, Ordering::Relaxed);
        self.value.fetch_add(delta, Ordering::SeqCst) + delta
    }

    /// Atomically compare and swap
    pub fn compare_and_swap(&self, current: i64, new: i64) -> (i64, bool) {
        self.operations.fetch_add(1, Ordering::Relaxed);
        match self.value.compare_exchange(current, new, Ordering::SeqCst, Ordering::Acquire) {
            Ok(prev) => (prev, true),
            Err(prev) => (prev, false),
        }
    }

    /// Get the number of operations performed
    pub fn operation_count(&self) -> usize {
        self.operations.load(Ordering::Acquire)
    }
}

/// Condition variable implementation for goroutine blocking primitives
#[derive(Debug)]
pub struct GoroutineCondvar {
    condvar: Condvar,
    waiters: AtomicUsize,
    notifications: AtomicUsize,
}

impl GoroutineCondvar {
    /// Create a new condition variable
    pub fn new() -> Self {
        Self {
            condvar: Condvar::new(),
            waiters: AtomicUsize::new(0),
            notifications: AtomicUsize::new(0),
        }
    }

    /// Wait on the condition variable with a mutex guard
    #[instrument(level = "debug", skip(self, guard))]
    pub fn wait<'a, T>(&self, guard: GoroutineMutexGuard<'a, T>) -> SyncResult<GoroutineMutexGuard<'a, T>> {
        self.waiters.fetch_add(1, Ordering::SeqCst);
        
        // Extract the underlying mutex guard
        let std_guard = std::mem::ManuallyDrop::new(guard);
        let mutex = std_guard.mutex;
        let inner_guard = unsafe {
            // SAFETY: We're extracting the guard from our wrapper to pass to std condvar
            std::ptr::read(&std_guard.guard as *const _)
        };

        debug!("Condition variable wait starting");
        
        match self.condvar.wait(inner_guard) {
            Ok(new_guard) => {
                self.waiters.fetch_sub(1, Ordering::SeqCst);
                debug!("Condition variable wait completed");
                Ok(GoroutineMutexGuard {
                    guard: new_guard,
                    mutex,
                })
            }
            Err(_) => {
                self.waiters.fetch_sub(1, Ordering::SeqCst);
                Err(SyncError::LockFailed("Condition variable wait failed".to_string()))
            }
        }
    }

    /// Wait on the condition variable with a timeout
    #[instrument(level = "debug", skip(self, guard, timeout))]
    pub fn wait_timeout<'a, T>(&self, guard: GoroutineMutexGuard<'a, T>, timeout: Duration) -> SyncResult<(GoroutineMutexGuard<'a, T>, bool)> {
        self.waiters.fetch_add(1, Ordering::SeqCst);
        
        // Extract the underlying mutex guard
        let std_guard = std::mem::ManuallyDrop::new(guard);
        let mutex = std_guard.mutex;
        let inner_guard = unsafe {
            // SAFETY: We're extracting the guard from our wrapper to pass to std condvar
            std::ptr::read(&std_guard.guard as *const _)
        };

        debug!(timeout = ?timeout, "Condition variable wait with timeout starting");
        
        match self.condvar.wait_timeout(inner_guard, timeout) {
            Ok((new_guard, timeout_result)) => {
                self.waiters.fetch_sub(1, Ordering::SeqCst);
                let timed_out = timeout_result.timed_out();
                debug!(timed_out = timed_out, "Condition variable wait with timeout completed");
                Ok((GoroutineMutexGuard {
                    guard: new_guard,
                    mutex,
                }, timed_out))
            }
            Err(_) => {
                self.waiters.fetch_sub(1, Ordering::SeqCst);
                Err(SyncError::LockFailed("Condition variable wait failed".to_string()))
            }
        }
    }

    /// Notify one waiting goroutine
    pub fn notify_one(&self) {
        self.notifications.fetch_add(1, Ordering::SeqCst);
        self.condvar.notify_one();
        debug!("Condition variable notified one waiter");
    }

    /// Notify all waiting goroutines
    pub fn notify_all(&self) {
        let waiters = self.waiters.load(Ordering::Acquire);
        self.notifications.fetch_add(waiters, Ordering::SeqCst);
        self.condvar.notify_all();
        debug!(waiters = waiters, "Condition variable notified all waiters");
    }

    /// Get the number of currently waiting goroutines
    pub fn waiter_count(&self) -> usize {
        self.waiters.load(Ordering::Acquire)
    }

    /// Get the total number of notifications sent
    pub fn notification_count(&self) -> usize {
        self.notifications.load(Ordering::Acquire)
    }
}

impl Default for GoroutineCondvar {
    fn default() -> Self {
        Self::new()
    }
}

/// Goroutine parking and unparking mechanisms for efficient blocking
#[derive(Debug)]
pub struct GoroutineParker {
    parked_goroutines: StdMutex<HashMap<ThreadId, Thread>>,
    park_count: AtomicUsize,
    unpark_count: AtomicUsize,
}

impl GoroutineParker {
    /// Create a new goroutine parker
    pub fn new() -> Self {
        Self {
            parked_goroutines: StdMutex::new(HashMap::new()),
            park_count: AtomicUsize::new(0),
            unpark_count: AtomicUsize::new(0),
        }
    }

    /// Park the current goroutine (block until unparked)
    #[instrument(level = "debug", skip(self))]
    pub fn park(&self) -> SyncResult<()> {
        let current_thread = thread::current();
        let thread_id = current_thread.id();

        debug!(thread_id = ?thread_id, "Parking goroutine");

        // Register the thread for unparking
        {
            let mut parked = self.parked_goroutines.lock()
                .map_err(|e| SyncError::LockFailed(format!("Parker mutex poisoned: {}", e)))?;
            parked.insert(thread_id, current_thread);
        }

        self.park_count.fetch_add(1, Ordering::SeqCst);

        // Park the thread
        thread::park();

        // Remove from parked list when unparked
        {
            let mut parked = self.parked_goroutines.lock()
                .map_err(|e| SyncError::LockFailed(format!("Parker mutex poisoned: {}", e)))?;
            parked.remove(&thread_id);
        }

        debug!(thread_id = ?thread_id, "Goroutine unparked");
        Ok(())
    }

    /// Park the current goroutine with a timeout
    #[instrument(level = "debug", skip(self, timeout))]
    pub fn park_timeout(&self, timeout: Duration) -> SyncResult<bool> {
        let current_thread = thread::current();
        let thread_id = current_thread.id();

        debug!(thread_id = ?thread_id, timeout = ?timeout, "Parking goroutine with timeout");

        // Register the thread for unparking
        {
            let mut parked = self.parked_goroutines.lock()
                .map_err(|e| SyncError::LockFailed(format!("Parker mutex poisoned: {}", e)))?;
            parked.insert(thread_id, current_thread);
        }

        self.park_count.fetch_add(1, Ordering::SeqCst);

        // Park the thread with timeout
        thread::park_timeout(timeout);

        // Check if we were actually unparked or timed out
        let was_unparked = {
            let mut parked = self.parked_goroutines.lock()
                .map_err(|e| SyncError::LockFailed(format!("Parker mutex poisoned: {}", e)))?;
            parked.remove(&thread_id).is_none() // If it's not in the map, we were unparked
        };

        debug!(thread_id = ?thread_id, was_unparked = was_unparked, "Goroutine park timeout completed");
        Ok(was_unparked)
    }

    /// Unpark a specific goroutine by thread ID
    #[instrument(level = "debug", skip(self, thread_id))]
    pub fn unpark(&self, thread_id: ThreadId) -> SyncResult<bool> {
        let thread_handle = {
            let mut parked = self.parked_goroutines.lock()
                .map_err(|e| SyncError::LockFailed(format!("Parker mutex poisoned: {}", e)))?;
            parked.remove(&thread_id)
        };

        if let Some(thread) = thread_handle {
            self.unpark_count.fetch_add(1, Ordering::SeqCst);
            thread.unpark();
            debug!(thread_id = ?thread_id, "Unparked goroutine");
            Ok(true)
        } else {
            debug!(thread_id = ?thread_id, "Goroutine not found for unparking");
            Ok(false)
        }
    }

    /// Unpark all currently parked goroutines
    pub fn unpark_all(&self) -> SyncResult<usize> {
        let parked_threads = {
            let mut parked = self.parked_goroutines.lock()
                .map_err(|e| SyncError::LockFailed(format!("Parker mutex poisoned: {}", e)))?;
            let threads: Vec<_> = parked.drain().collect();
            threads
        };

        let count = parked_threads.len();
        for (thread_id, thread) in parked_threads {
            self.unpark_count.fetch_add(1, Ordering::SeqCst);
            thread.unpark();
            debug!(thread_id = ?thread_id, "Unparked goroutine");
        }

        info!(count = count, "Unparked all goroutines");
        Ok(count)
    }

    /// Get the number of currently parked goroutines
    pub fn parked_count(&self) -> usize {
        self.parked_goroutines.lock()
            .map(|parked| parked.len())
            .unwrap_or(0)
    }

    /// Get statistics about parking operations
    pub fn stats(&self) -> (usize, usize, usize) {
        (
            self.park_count.load(Ordering::Acquire),
            self.unpark_count.load(Ordering::Acquire),
            self.parked_count(),
        )
    }
}

impl Default for GoroutineParker {
    fn default() -> Self {
        Self::new()
    }
}

/// Global goroutine parker instance
static GLOBAL_PARKER: std::sync::OnceLock<Arc<GoroutineParker>> = std::sync::OnceLock::new();

/// Get the global goroutine parker
pub fn get_global_parker() -> Arc<GoroutineParker> {
    GLOBAL_PARKER.get_or_init(|| Arc::new(GoroutineParker::new())).clone()
}

/// Utility function to convert ThreadId to usize for atomic operations
fn current_thread_id_as_usize() -> usize {
    // Note: This is a simple hash of the thread ID
    // In a real implementation, you might want a more sophisticated approach
    let thread_id = thread::current().id();
    let thread_id_debug = format!("{:?}", thread_id);
    
    // Extract the numeric part from "ThreadId(N)" format
    thread_id_debug.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0)
}

/// FFI exports for LLVM-generated code
/// 
/// These functions provide C-compatible interfaces for the synchronization primitives

/// Create a new WaitGroup
#[no_mangle]
pub extern "C" fn cursed_waitgroup_new() -> *mut WaitGroup {
    let wg = Box::new(WaitGroup::new());
    Box::into_raw(wg)
}

/// Add to WaitGroup counter
#[no_mangle]
pub extern "C" fn cursed_waitgroup_add(wg: *mut WaitGroup, delta: i64) -> i32 {
    if wg.is_null() { return 1; }
    
    let wg = unsafe { &*wg };
    match wg.add(delta) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

/// Wait for WaitGroup
#[no_mangle]
pub extern "C" fn cursed_waitgroup_wait(wg: *mut WaitGroup) -> i32 {
    if wg.is_null() { return 1; }
    
    let wg = unsafe { &*wg };
    match wg.wait() {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

/// Destroy a WaitGroup
#[no_mangle]
pub extern "C" fn cursed_waitgroup_destroy(wg: *mut WaitGroup) {
    if !wg.is_null() {
        unsafe { Box::from_raw(wg) };
    }
}

/// Park the current goroutine
#[no_mangle]
pub extern "C" fn cursed_goroutine_park() -> i32 {
    let parker = get_global_parker();
    match parker.park() {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

/// Unpark all goroutines
#[no_mangle]
pub extern "C" fn cursed_goroutine_unpark_all() -> usize {
    let parker = get_global_parker();
    parker.unpark_all().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_waitgroup_basic() {
        let wg = WaitGroup::new();
        
        // Test add and done
        wg.add_one().unwrap();
        assert_eq!(wg.count(), 1);
        
        wg.done().unwrap();
        assert_eq!(wg.count(), 0);
    }

    #[test]
    fn test_waitgroup_multiple_goroutines() {
        let wg = Arc::new(WaitGroup::new());
        let counter = Arc::new(AtomicUsize::new(0));
        
        // Spawn multiple threads
        let mut handles = Vec::new();
        for i in 0..5 {
            wg.add_one().unwrap();
            let wg_clone = Arc::clone(&wg);
            let counter_clone = Arc::clone(&counter);
            
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(10 * i));
                counter_clone.fetch_add(1, Ordering::SeqCst);
                wg_clone.done().unwrap();
            });
            handles.push(handle);
        }
        
        // Wait for all to complete
        wg.wait().unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 5);
        
        // Clean up
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_mutex_basic() {
        let mutex = GoroutineMutex::new(42);
        
        {
            let guard = mutex.lock().unwrap();
            assert_eq!(*guard, 42);
            assert!(mutex.is_owned_by_current_thread());
        }
        
        assert!(!mutex.is_owned_by_current_thread());
        assert_eq!(mutex.owner(), 0);
    }

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new(0);
        
        assert_eq!(counter.get(), 0);
        counter.set(10);
        assert_eq!(counter.get(), 10);
        
        let new_val = counter.add(5);
        assert_eq!(new_val, 15);
        assert_eq!(counter.get(), 15);
        
        let (old, success) = counter.compare_and_swap(15, 20);
        assert_eq!(old, 15);
        assert!(success);
        assert_eq!(counter.get(), 20);
        
        assert!(counter.operation_count() > 0);
    }

    #[test]
    fn test_condition_variable() {
        let mutex = Arc::new(GoroutineMutex::new(false));
        let condvar = Arc::new(GoroutineCondvar::new());
        
        let mutex_clone = Arc::clone(&mutex);
        let condvar_clone = Arc::clone(&condvar);
        
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            {
                let mut guard = mutex_clone.lock().unwrap();
                *guard = true;
            }
            condvar_clone.notify_one();
        });
        
        // Wait for the condition
        let mut guard = mutex.lock().unwrap();
        while !*guard {
            guard = condvar.wait(guard).unwrap();
        }
        
        assert!(*guard);
        handle.join().unwrap();
    }

    #[test]
    fn test_goroutine_parker() {
        let parker = GoroutineParker::new();
        let (tx, rx) = mpsc::channel();
        
        let handle = thread::spawn(move || {
            let parker_local = get_global_parker();
            
            // Signal we're about to park
            tx.send(thread::current().id()).unwrap();
            
            // Park and wait to be unparked
            parker_local.park().unwrap();
        });
        
        // Wait for the thread to be ready to park
        let thread_id = rx.recv().unwrap();
        
        // Wait until the thread is actually parked by checking the parker count
        let parker_global = get_global_parker();
        let initial_count = parker_global.parked_count();
        let mut attempts = 0;
        while parker_global.parked_count() == initial_count && attempts < 100 {
            thread::sleep(Duration::from_millis(1));
            attempts += 1;
        }
        
        // Unpark the thread
        let unparked = parker_global.unpark(thread_id).unwrap();
        assert!(unparked);
        
        handle.join().unwrap();
    }
}
