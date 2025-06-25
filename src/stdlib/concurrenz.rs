/// ConcurrenZ - Synchronization primitives with Gen Z flair 🚀
/// 
/// This module provides comprehensive concurrency and synchronization operations using
/// CURSED language conventions and Gen Z naming. All primitives work with CURSED types
/// and provide the foundation for concurrent applications.
/// 
/// # Why ConcurrenZ matters:
/// - Essential for building scalable concurrent applications
/// - Provides thread-safe operations with CURSED semantics
/// - Includes modern concurrency patterns with Gen Z naming
/// - Optimized for performance while maintaining safety

// use crate::stdlib::sync::{self, SyncError, SyncResult};
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex as StdMutex, RwLock as StdRwLock};
use std::time::Duration;

/// CursedError type for ConcurrenZ operations
pub type ConcurrenzError = SyncError;

/// Result type for ConcurrenZ operations  
pub type ConcurrenzResult<T> = SyncResult<T>;

// ================================
// MUTEX OPERATIONS (LOCK VIBES)
// ================================

/// Mutex for thread-safe exclusive access (lock vibes)
/// 
/// # Examples
/// ```cursed
/// facts mutex = MutexVibes::new(42);
/// facts guard = mutex.lock_it();
/// *guard = 100;
/// ```
#[derive(Debug)]
pub struct MutexVibes<T> {
    inner: Arc<StdMutex<T>>,
}

impl<T> Clone for MutexVibes<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> MutexVibes<T> {
    /// Create new mutex (new lock vibes)
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(StdMutex::new(value)),
        }
    }
    
    /// Lock the mutex (acquire lock vibes)
    pub fn lock_it(&self) -> ConcurrenzResult<std::sync::MutexGuard<T>> {
        self.inner.lock().map_err(|_| ConcurrenzError::LockError("Mutex poisoned".to_string()))
    }
    
    /// Try to lock the mutex (attempt lock vibes)
    pub fn try_lock_it(&self) -> ConcurrenzResult<std::sync::MutexGuard<T>> {
        self.inner.try_lock().map_err(|e| match e {
            std::sync::TryLockError::Poisoned(_) => ConcurrenzError::LockError("Mutex poisoned".to_string()),
            std::sync::TryLockError::WouldBlock => ConcurrenzError::TimeoutError("Mutex would block".to_string()),
        })
    }
}

/// Create new mutex with vibes (convenience function)
/// 
/// # Examples
/// ```cursed
/// facts mutex = new_mutex_vibes(42);
/// ```
pub fn new_mutex_vibes<T>(value: T) -> MutexVibes<T> {
    MutexVibes::new(value)
}

// ================================
// READ-WRITE LOCK OPERATIONS (RWLOCK VIBES)
// ================================

/// Read-Write Lock for concurrent read access (shared vibes)
/// 
/// # Examples
/// ```cursed
/// facts rwlock = RwLockVibes::new(42);
/// facts read_guard = rwlock.read_it();
/// facts write_guard = rwlock.write_it();
/// ```
#[derive(Debug)]
pub struct RwLockVibes<T> {
    inner: Arc<StdRwLock<T>>,
}

impl<T> Clone for RwLockVibes<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> RwLockVibes<T> {
    /// Create new read-write lock (new shared lock vibes)
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(StdRwLock::new(value)),
        }
    }
    
    /// Acquire read lock (shared read vibes)
    pub fn read_it(&self) -> ConcurrenzResult<std::sync::RwLockReadGuard<T>> {
        self.inner.read().map_err(|_| ConcurrenzError::LockError("RwLock poisoned for read".to_string()))
    }
    
    /// Acquire write lock (exclusive write vibes)
    pub fn write_it(&self) -> ConcurrenzResult<std::sync::RwLockWriteGuard<T>> {
        self.inner.write().map_err(|_| ConcurrenzError::LockError("RwLock poisoned for write".to_string()))
    }
    
    /// Try to acquire read lock (attempt read vibes)
    pub fn try_read_it(&self) -> ConcurrenzResult<std::sync::RwLockReadGuard<T>> {
        self.inner.try_read().map_err(|e| match e {
            std::sync::TryLockError::Poisoned(_) => ConcurrenzError::LockError("RwLock poisoned for read".to_string()),
            std::sync::TryLockError::WouldBlock => ConcurrenzError::TimeoutError("RwLock would block for read".to_string()),
        })
    }
    
    /// Try to acquire write lock (attempt write vibes)
    pub fn try_write_it(&self) -> ConcurrenzResult<std::sync::RwLockWriteGuard<T>> {
        self.inner.try_write().map_err(|e| match e {
            std::sync::TryLockError::Poisoned(_) => ConcurrenzError::LockError("RwLock poisoned for write".to_string()),
            std::sync::TryLockError::WouldBlock => ConcurrenzError::TimeoutError("RwLock would block for write".to_string()),
        })
    }
}

/// Create new read-write lock with vibes (convenience function)
/// 
/// # Examples
/// ```cursed
/// facts rwlock = new_rwlock_vibes(42);
/// ```
pub fn new_rwlock_vibes<T>(value: T) -> RwLockVibes<T> {
    RwLockVibes::new(value)
}

// ================================
// ATOMIC OPERATIONS (ATOMIC VIBES)
// ================================

/// Atomic boolean for lock-free operations (atomic truth vibes)
/// 
/// # Examples
/// ```cursed
/// facts atomic = AtomicBoolVibes::new(false);
/// atomic.store_it(true);
/// facts value = atomic.load_it();
/// ```
#[derive(Debug)]
pub struct AtomicBoolVibes {
    inner: std::sync::atomic::AtomicBool,
}

impl AtomicBoolVibes {
    /// Create new atomic boolean (new atomic truth vibes)
    pub fn new(value: bool) -> Self {
        Self {
            inner: std::sync::atomic::AtomicBool::new(value),
        }
    }
    
    /// Load atomic value (get truth vibes)
    pub fn load_it(&self) -> bool {
        self.inner.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Store atomic value (set truth vibes)
    pub fn store_it(&self, value: bool) {
        self.inner.store(value, std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Swap atomic value (exchange truth vibes)
    pub fn swap_it(&self, value: bool) -> bool {
        self.inner.swap(value, std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Compare and swap (conditional exchange vibes)
    pub fn compare_and_swap_it(&self, current: bool, new: bool) -> bool {
        self.inner.compare_and_swap(current, new, std::sync::atomic::Ordering::SeqCst)
    }
}

/// Atomic integer for lock-free operations (atomic number vibes)
/// 
/// # Examples
/// ```cursed
/// facts atomic = AtomicIntVibes::new(42);
/// atomic.store_it(100);
/// facts value = atomic.load_it();
/// facts old_value = atomic.fetch_add_it(10);
/// ```
#[derive(Debug)]
pub struct AtomicIntVibes {
    inner: std::sync::atomic::AtomicI32,
}

impl AtomicIntVibes {
    /// Create new atomic integer (new atomic number vibes)
    pub fn new(value: i32) -> Self {
        Self {
            inner: std::sync::atomic::AtomicI32::new(value),
        }
    }
    
    /// Load atomic value (get number vibes)
    pub fn load_it(&self) -> i32 {
        self.inner.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Store atomic value (set number vibes)
    pub fn store_it(&self, value: i32) {
        self.inner.store(value, std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Swap atomic value (exchange number vibes)
    pub fn swap_it(&self, value: i32) -> i32 {
        self.inner.swap(value, std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Compare and swap (conditional exchange vibes)
    pub fn compare_and_swap_it(&self, current: i32, new: i32) -> i32 {
        self.inner.compare_and_swap(current, new, std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Atomic add and return old value (add and fetch vibes)
    pub fn fetch_add_it(&self, value: i32) -> i32 {
        self.inner.fetch_add(value, std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Atomic subtract and return old value (sub and fetch vibes)
    pub fn fetch_sub_it(&self, value: i32) -> i32 {
        self.inner.fetch_sub(value, std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Atomic increment and return new value (inc vibes)
    pub fn increment_it(&self) -> i32 {
        self.fetch_add_it(1) + 1
    }
    
    /// Atomic decrement and return new value (dec vibes)
    pub fn decrement_it(&self) -> i32 {
        self.fetch_sub_it(1) - 1
    }
}

/// Create new atomic boolean with vibes (convenience function)
/// 
/// # Examples
/// ```cursed
/// facts atomic = new_atomic_bool_vibes(false);
/// ```
pub fn new_atomic_bool_vibes(value: bool) -> AtomicBoolVibes {
    AtomicBoolVibes::new(value)
}

/// Create new atomic integer with vibes (convenience function)
/// 
/// # Examples
/// ```cursed
/// facts atomic = new_atomic_int_vibes(42);
/// ```
pub fn new_atomic_int_vibes(value: i32) -> AtomicIntVibes {
    AtomicIntVibes::new(value)
}

// ================================
// CHANNEL OPERATIONS (MESSAGE VIBES)
// ================================

/// Channel sender for message passing (send vibes)
/// 
/// # Examples
/// ```cursed
/// facts (sender, receiver) = channel_vibes();
/// sender.send_it(42);
/// facts value = receiver.receive_it();
/// ```
#[derive(Debug, Clone)]
pub struct SenderVibes<T> {
    inner: std::sync::mpsc::Sender<T>,
}

impl<T> SenderVibes<T> {
    /// Send value through channel (send message vibes)
    pub fn send_it(&self, value: T) -> ConcurrenzResult<()> {
        self.inner.send(value).map_err(|_| ConcurrenzError::ChannelError("Channel receiver dropped".to_string()))
    }
    
    /// Try to send value through channel (attempt send vibes)
    pub fn try_send_it(&self, value: T) -> ConcurrenzResult<()> {
        match self.inner.try_send(value) {
            Ok(()) => Ok(()),
            Err(std::sync::mpsc::TrySendError::Full(_)) => Err(ConcurrenzError::TimeoutError("Channel full".to_string())),
            Err(std::sync::mpsc::TrySendError::Disconnected(_)) => Err(ConcurrenzError::ChannelError("Channel receiver dropped".to_string())),
        }
    }
}

/// Channel receiver for message passing (receive vibes)
/// 
/// # Examples
/// ```cursed
/// facts (sender, receiver) = channel_vibes();
/// sender.send_it(42);
/// facts value = receiver.receive_it();
/// ```
#[derive(Debug)]
pub struct ReceiverVibes<T> {
    inner: std::sync::mpsc::Receiver<T>,
}

impl<T> ReceiverVibes<T> {
    /// Receive value from channel (receive message vibes)
    pub fn receive_it(&self) -> ConcurrenzResult<T> {
        self.inner.recv().map_err(|_| ConcurrenzError::ChannelError("Channel sender dropped".to_string()))
    }
    
    /// Try to receive value from channel (attempt receive vibes)
    pub fn try_receive_it(&self) -> ConcurrenzResult<T> {
        match self.inner.try_recv() {
            Ok(value) => Ok(value),
            Err(std::sync::mpsc::TryRecvError::Empty) => Err(ConcurrenzError::TimeoutError("Channel empty".to_string())),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Err(ConcurrenzError::ChannelError("Channel sender dropped".to_string())),
        }
    }
    
    /// Receive with timeout (timed receive vibes)
    pub fn receive_timeout_it(&self, timeout: Duration) -> ConcurrenzResult<T> {
        match self.inner.recv_timeout(timeout) {
            Ok(value) => Ok(value),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Err(ConcurrenzError::TimeoutError("Receive timeout".to_string())),
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Err(ConcurrenzError::ChannelError("Channel sender dropped".to_string())),
        }
    }
    
    /// Get iterator over received values (iterate vibes)
    pub fn iter_it(&self) -> std::sync::mpsc::Iter<T> {
        self.inner.iter()
    }
}

/// Create unbounded channel for message passing (message tube vibes)
/// 
/// # Examples
/// ```cursed
/// facts (sender, receiver) = channel_vibes::<normie>();
/// sender.send_it(42);
/// facts value = receiver.receive_it(); // 42
/// ```
pub fn channel_vibes<T>() -> (SenderVibes<T>, ReceiverVibes<T>) {
    let (sender, receiver) = std::sync::mpsc::channel();
    (
        SenderVibes { inner: sender },
        ReceiverVibes { inner: receiver },
    )
}

// ================================
// THREAD OPERATIONS (THREAD VIBES)
// ================================

/// Thread handle for concurrent execution (execution vibes)
/// 
/// # Examples
/// ```cursed
/// facts handle = spawn_thread_vibes(|| {
///     // Do work
///     42
/// });
/// facts result = handle.join_it();
/// ```
#[derive(Debug)]
pub struct ThreadHandleVibes<T> {
    inner: Option<std::thread::JoinHandle<T>>,
}

impl<T> ThreadHandleVibes<T> {
    /// Join thread and get result (wait for completion vibes)
    pub fn join_it(mut self) -> ConcurrenzResult<T> {
        if let Some(handle) = self.inner.take() {
            handle.join().map_err(|_| ConcurrenzError::ThreadError("Thread panicked".to_string()))
        } else {
            Err(ConcurrenzError::ThreadError("Thread already joined".to_string()))
        }
    }
    
    /// Check if thread is finished (completion check vibes)
    pub fn is_finished(&self) -> bool {
        self.inner.as_ref().map_or(true, |h| h.is_finished())
    }
    
    /// Get thread ID (identify vibes)
    pub fn thread_id(&self) -> Option<std::thread::ThreadId> {
        self.inner.as_ref().map(|h| h.thread().id())
    }
}

/// Spawn new thread with vibes (create execution vibes)
/// 
/// # Examples
/// ```cursed
/// facts handle = spawn_thread_vibes(|| {
///     println!("Hello from thread!");
///     42
/// });
/// facts result = handle.join_it(); // 42
/// ```
pub fn spawn_thread_vibes<F, T>(f: F) -> ThreadHandleVibes<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let handle = std::thread::spawn(f);
    ThreadHandleVibes {
        inner: Some(handle),
    }
}

/// Spawn named thread with vibes (create named execution vibes)
/// 
/// # Examples
/// ```cursed
/// facts handle = spawn_named_thread_vibes("worker", || {
///     42
/// });
/// ```
pub fn spawn_named_thread_vibes<F, T>(name: &str, f: F) -> ThreadHandleVibes<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let handle = std::thread::Builder::new()
        .name(name.to_string())
        .spawn(f)
        .expect("Failed to spawn thread");
    ThreadHandleVibes {
        inner: Some(handle),
    }
}

/// Sleep current thread (pause vibes)
/// 
/// # Examples
/// ```cursed
/// sleep_vibes(Duration::from_millis(1000)); // Sleep 1 second
/// ```
pub fn sleep_vibes(duration: Duration) {
    std::thread::sleep(duration);
}

/// Yield current thread (give way vibes)
/// 
/// # Examples
/// ```cursed
/// yield_vibes(); // Let other threads run
/// ```
pub fn yield_vibes() {
        // TODO: implement
    }
    std::thread::yield_now();
}

/// Get current thread ID (who am I vibes)
/// 
/// # Examples
/// ```cursed
/// facts thread_id = current_thread_id_vibes();
/// ```
pub fn current_thread_id_vibes() -> std::thread::ThreadId {
    std::thread::current().id()
}

/// Get current thread name (what's my name vibes)
/// 
/// # Examples
/// ```cursed
/// facts thread_name = current_thread_name_vibes(); // Some("main") or None
/// ```
pub fn current_thread_name_vibes() -> Option<String> {
    std::thread::current().name().map(|s| s.to_string())
}

// ================================
// BARRIER OPERATIONS (SYNC POINT VIBES)
// ================================

/// Barrier for thread synchronization (sync point vibes)
/// 
/// # Examples
/// ```cursed
/// facts barrier = BarrierVibes::new(3);
/// // In each thread:
/// barrier.wait_it();
/// ```
#[derive(Debug, Clone)]
pub struct BarrierVibes {
    inner: Arc<std::sync::Barrier>,
}

impl BarrierVibes {
    /// Create new barrier (new sync point vibes)
    pub fn new(n: usize) -> Self {
        Self {
            inner: Arc::new(std::sync::Barrier::new(n)),
        }
    }
    
    /// Wait at barrier (sync wait vibes)
    pub fn wait_it(&self) -> std::sync::BarrierWaitResult {
        self.inner.wait()
    }
}

/// Create new barrier with vibes (convenience function)
/// 
/// # Examples
/// ```cursed
/// facts barrier = new_barrier_vibes(3);
/// ```
pub fn new_barrier_vibes(n: usize) -> BarrierVibes {
    BarrierVibes::new(n)
}

// ================================
// CONDITION VARIABLE OPERATIONS (NOTIFY VIBES)
// ================================

/// Condition variable for thread coordination (notify vibes)
/// 
/// # Examples
/// ```cursed
/// facts condvar = CondVarVibes::new();
/// facts mutex = MutexVibes::new(false);
/// // In waiter thread:
/// facts guard = mutex.lock_it();
/// condvar.wait_it(guard);
/// // In notifier thread:
/// condvar.notify_one_it();
/// ```
#[derive(Debug, Clone)]
pub struct CondVarVibes {
    inner: Arc<std::sync::Condvar>,
}

impl CondVarVibes {
    /// Create new condition variable (new notify vibes)
    pub fn new() -> Self {
        Self {
            inner: Arc::new(std::sync::Condvar::new()),
        }
    }
    
    /// Wait on condition variable (wait for notify vibes)
    pub fn wait_it<'a, T>(&self, guard: std::sync::MutexGuard<'a, T>) -> ConcurrenzResult<std::sync::MutexGuard<'a, T>> {
        self.inner.wait(guard).map_err(|_| ConcurrenzError::LockError("Mutex poisoned in condvar wait".to_string()))
    }
    
    /// Wait with timeout (timed wait vibes)
    pub fn wait_timeout_it<'a, T>(&self, guard: std::sync::MutexGuard<'a, T>, timeout: Duration) -> ConcurrenzResult<(std::sync::MutexGuard<'a, T>, std::sync::WaitTimeoutResult)> {
        self.inner.wait_timeout(guard, timeout).map_err(|_| ConcurrenzError::LockError("Mutex poisoned in condvar wait_timeout".to_string()))
    }
    
    /// Notify one waiting thread (wake one vibes)
    pub fn notify_one_it(&self) {
        self.inner.notify_one();
    }
    
    /// Notify all waiting threads (wake all vibes)
    pub fn notify_all_it(&self) {
        self.inner.notify_all();
    }
}

/// Create new condition variable with vibes (convenience function)
/// 
/// # Examples
/// ```cursed
/// facts condvar = new_condvar_vibes();
/// ```
pub fn new_condvar_vibes() -> CondVarVibes {
    CondVarVibes::new()
}

// ================================
// ONCE OPERATIONS (ONE TIME VIBES)
// ================================

/// Once for one-time initialization (one shot vibes)
/// 
/// # Examples
/// ```cursed
/// facts once = OnceVibes::new();
/// once.call_once_it(|| {
///     println!("This runs only once!");
/// });
/// ```
#[derive(Debug)]
pub struct OnceVibes {
    inner: std::sync::Once,
}

impl OnceVibes {
    /// Create new Once (new one shot vibes)
    pub fn new() -> Self {
        Self {
            inner: std::sync::Once::new(),
        }
    }
    
    /// Call closure exactly once (one time execution vibes)
    pub fn call_once_it<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        self.inner.call_once(f);
    }
    
    /// Check if Once has been called (completion check vibes)
    pub fn is_completed(&self) -> bool {
        self.inner.is_completed()
    }
}

/// Create new Once with vibes (convenience function)
/// 
/// # Examples
/// ```cursed
/// facts once = new_once_vibes();
/// ```
pub fn new_once_vibes() -> OnceVibes {
    OnceVibes::new()
}

// ================================
// UTILITY FUNCTIONS
// ================================

/// Get number of CPU cores (core count vibes)
/// 
/// # Examples
/// ```cursed
/// facts cores = num_cpus_vibes(); // 8
/// ```
pub fn num_cpus_vibes() -> usize {
    std::thread::available_parallelism().map_or(1, |p| p.get())
}

/// Park current thread (pause current vibes)
/// 
/// # Examples
/// ```cursed
/// park_vibes(); // Park until unparked
/// ```
pub fn park_vibes() {
        // TODO: implement
    }
    std::thread::park();
}

/// Park current thread with timeout (timed pause vibes)
/// 
/// # Examples
/// ```cursed
/// park_timeout_vibes(Duration::from_millis(1000));
/// ```
pub fn park_timeout_vibes(timeout: Duration) {
    std::thread::park_timeout(timeout);
}

/// Unpark specific thread (wake specific vibes)
/// 
/// # Examples
/// ```cursed
/// unpark_thread_vibes(thread_handle);
/// ```
pub fn unpark_thread_vibes(handle: &ThreadHandleVibes<()>) {
    if let Some(h) = &handle.inner {
        h.thread().unpark();
    }
}

/// Module initialization function
pub fn init_concurrenz() -> ConcurrenzResult<()> {
    // Initialize any global state for ConcurrenZ module
    Ok(())
}

/// Get module statistics and information
pub fn get_concurrenz_stats() -> HashMap<String, String> {
    let mut stats = HashMap::new();
    stats.insert("version".to_string(), "1.0.0".to_string());
    stats.insert("primitives".to_string(), "Mutex, RwLock, Atomic, Channel, Thread, Barrier, CondVar, Once".to_string());
    stats.insert("features".to_string(), "Thread-safe, Gen Z naming, comprehensive concurrency".to_string());
    stats.insert("cpu_cores".to_string(), num_cpus_vibes().to_string());
    stats
}

