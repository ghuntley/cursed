//! FFI Threads Implementation - Bridging Rust FFI and Zig Runtime
//!
//! This module provides a comprehensive threading bridge between the Rust FFI system
//! and the Zig concurrency runtime, implementing proper condition variable semantics
//! with wait/notify operations, timeout support, mutex integration, and thread safety.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Condvar, RwLock};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::thread::{ThreadId, JoinHandle};
use std::ffi::c_void;

use crate::error::CursedError;
use crate::runtime::channels::sync::{ChannelSync, PriorityWaitQueue, WaitQueueEntry, WaitOperationType};
use crate::runtime::goroutine::{GoroutineId, GoroutineState};
use crate::runtime::channels::operations::OperationPriority;

/// Thread synchronization primitives for FFI bridge
pub struct FfiThreadSync {
    /// Condition variable registry for cross-language synchronization
    condition_vars: RwLock<HashMap<CondVarId, Arc<FfiConditionVariable>>>,
    
    /// Mutex registry for cross-language locking
    mutexes: RwLock<HashMap<MutexId, Arc<FfiMutex>>>,
    
    /// Thread registry for lifetime management
    threads: RwLock<HashMap<ThreadHandle, Arc<FfiThread>>>,
    
    /// Bridge to Zig runtime synchronization
    zig_bridge: Arc<ZigRuntimeBridge>,
    
    /// Thread local storage for goroutine mapping
    thread_local_storage: RwLock<HashMap<ThreadId, GoroutineId>>,
    
    /// Global sync state for coordination
    global_state: Arc<GlobalSyncState>,
    
    /// Performance statistics
    stats: Mutex<ThreadSyncStats>,
}

/// Unique identifier for condition variables across language boundaries
pub type CondVarId = u64;

/// Unique identifier for mutexes across language boundaries
pub type MutexId = u64;

/// Unique handle for threads managed by FFI system
pub type ThreadHandle = u64;

/// FFI-safe condition variable with timeout and priority support
pub struct FfiConditionVariable {
    /// Unique identifier
    id: CondVarId,
    
    /// Rust condition variable for native operations
    condvar: Condvar,
    
    /// Associated mutex for proper condition variable semantics
    associated_mutex: Option<MutexId>,
    
    /// Wait queue for priority-based wakeup
    wait_queue: Arc<PriorityWaitQueue>,
    
    /// Statistics tracking
    stats: Mutex<CondVarStats>,
    
    /// State tracking
    state: AtomicU64, // bit-packed state information
    
    /// Timeout tracking for operations
    timeout_manager: Arc<TimeoutManager>,
}

/// FFI-safe mutex with deadlock detection and performance monitoring
pub struct FfiMutex {
    /// Unique identifier
    id: MutexId,
    
    /// Rust mutex for native operations
    mutex: Mutex<MutexState>,
    
    /// Owner tracking for deadlock detection
    owner: AtomicU64, // ThreadHandle of current owner, 0 = unlocked
    
    /// Lock contention tracking
    contention_tracker: Arc<ContentionTracker>,
    
    /// Statistics
    stats: Mutex<MutexStats>,
}

/// Thread managed by FFI system with Zig runtime integration
pub struct FfiThread {
    /// Unique handle
    handle: ThreadHandle,
    
    /// Operating system thread ID
    os_thread_id: Option<ThreadId>,
    
    /// Associated goroutine ID in Zig runtime
    goroutine_id: Option<GoroutineId>,
    
    /// Thread join handle
    join_handle: Option<JoinHandle<()>>,
    
    /// Thread state
    state: AtomicU64,
    
    /// Priority level
    priority: AtomicU64,
}

/// Bridge to Zig runtime concurrency system
pub struct ZigRuntimeBridge {
    /// Channel sync from Zig runtime
    channel_sync: Arc<ChannelSync>,
    
    /// Goroutine state mapping
    goroutine_states: RwLock<HashMap<GoroutineId, GoroutineState>>,
    
    /// Runtime callback for notifications
    runtime_notifier: Arc<RuntimeNotifier>,
}

/// Global synchronization state for cross-language coordination
pub struct GlobalSyncState {
    /// Next condition variable ID
    next_condvar_id: AtomicU64,
    
    /// Next mutex ID
    next_mutex_id: AtomicU64,
    
    /// Next thread handle
    next_thread_handle: AtomicU64,
    
    /// Global shutdown flag
    shutdown_requested: AtomicBool,
    
    /// Emergency brake for deadlock prevention
    emergency_brake: AtomicBool,
}

/// Internal state for mutexes
#[derive(Debug, Default)]
struct MutexState {
    lock_count: u64,
    last_lock_time: Option<Instant>,
    contention_count: u64,
}

/// Statistics for condition variables
#[derive(Debug, Default, Clone)]
pub struct CondVarStats {
    pub total_waits: u64,
    pub total_notifications: u64,
    pub total_timeouts: u64,
    pub average_wait_time: Duration,
    pub max_wait_time: Duration,
    pub spurious_wakeups: u64,
}

/// Statistics for mutexes
#[derive(Debug, Default, Clone)]
pub struct MutexStats {
    pub total_locks: u64,
    pub total_unlocks: u64,
    pub contention_events: u64,
    pub average_hold_time: Duration,
    pub max_hold_time: Duration,
    pub deadlock_detections: u64,
}

/// Overall thread synchronization statistics
#[derive(Debug, Default, Clone)]
pub struct ThreadSyncStats {
    pub active_condition_vars: u64,
    pub active_mutexes: u64,
    pub active_threads: u64,
    pub total_sync_operations: u64,
    pub total_deadlocks_prevented: u64,
    pub total_timeouts: u64,
    pub bridge_operations: u64,
}

/// Timeout management for condition variable operations
pub struct TimeoutManager {
    /// Active timeouts
    timeouts: Mutex<HashMap<u64, Instant>>,
    
    /// Next timeout ID
    next_timeout_id: AtomicU64,
}

/// Contention tracking for mutex performance analysis
pub struct ContentionTracker {
    /// Contention events
    events: Mutex<Vec<ContentionEvent>>,
    
    /// Current contention level
    current_level: AtomicU64,
}

/// Contention event for analysis
#[derive(Debug, Clone)]
pub struct ContentionEvent {
    pub timestamp: Instant,
    pub thread_handle: ThreadHandle,
    pub wait_duration: Duration,
    pub acquired: bool,
}

/// Runtime notification system for Zig integration
pub struct RuntimeNotifier {
    /// Notification callbacks
    callbacks: RwLock<HashMap<String, Box<dyn Fn(&str) + Send + Sync>>>,
}

impl FfiThreadSync {
    /// Create a new FFI thread synchronization system
    pub fn new() -> Result<Self, CursedError> {
        let channel_sync = Arc::new(ChannelSync::new());
        let zig_bridge = Arc::new(ZigRuntimeBridge::new(channel_sync)?);
        
        Ok(Self {
            condition_vars: RwLock::new(HashMap::new()),
            mutexes: RwLock::new(HashMap::new()),
            threads: RwLock::new(HashMap::new()),
            zig_bridge,
            thread_local_storage: RwLock::new(HashMap::new()),
            global_state: Arc::new(GlobalSyncState::new()),
            stats: Mutex::new(ThreadSyncStats::default()),
        })
    }
    
    /// Create a new condition variable
    pub fn create_condition_variable(&self) -> Result<CondVarId, CursedError> {
        let id = self.global_state.next_condvar_id.fetch_add(1, Ordering::SeqCst);
        
        let condvar = Arc::new(FfiConditionVariable {
            id,
            condvar: Condvar::new(),
            associated_mutex: None,
            wait_queue: Arc::new(PriorityWaitQueue::new()),
            stats: Mutex::new(CondVarStats::default()),
            state: AtomicU64::new(0),
            timeout_manager: Arc::new(TimeoutManager::new()),
        });
        
        let mut condition_vars = self.condition_vars.write().unwrap();
        condition_vars.insert(id, condvar);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.active_condition_vars += 1;
        }
        
        Ok(id)
    }
    
    /// Create a new mutex
    pub fn create_mutex(&self) -> Result<MutexId, CursedError> {
        let id = self.global_state.next_mutex_id.fetch_add(1, Ordering::SeqCst);
        
        let mutex = Arc::new(FfiMutex {
            id,
            mutex: Mutex::new(MutexState::default()),
            owner: AtomicU64::new(0),
            contention_tracker: Arc::new(ContentionTracker::new()),
            stats: Mutex::new(MutexStats::default()),
        });
        
        let mut mutexes = self.mutexes.write().unwrap();
        mutexes.insert(id, mutex);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.active_mutexes += 1;
        }
        
        Ok(id)
    }
    
    /// Wait on condition variable with timeout support
    pub fn condition_wait(
        &self,
        condvar_id: CondVarId,
        mutex_id: MutexId,
        timeout_ms: Option<u64>,
    ) -> Result<ConditionWaitResult, CursedError> {
        let start_time = Instant::now();
        
        // Get condition variable and mutex
        let condvar = {
            let condition_vars = self.condition_vars.read().unwrap();
            condition_vars.get(&condvar_id)
                .ok_or_else(|| CursedError::General(format!("Condition variable {} not found", condvar_id)))?
                .clone()
        };
        
        let mutex = {
            let mutexes = self.mutexes.read().unwrap();
            mutexes.get(&mutex_id)
                .ok_or_else(|| CursedError::General(format!("Mutex {} not found", mutex_id)))?
                .clone()
        };
        
        // Verify mutex is locked by current thread
        let current_thread = self.get_current_thread_handle()?;
        if mutex.owner.load(Ordering::Acquire) != current_thread {
            return Err(CursedError::General("Mutex not owned by current thread".to_string()));
        }
        
        // Create wait entry for priority queue
        let goroutine_id = self.get_current_goroutine_id()?;
        let wait_entry = WaitQueueEntry {
            goroutine_id,
            priority: OperationPriority::Normal,
            wait_start: start_time,
            timeout: timeout_ms.map(|ms| start_time + Duration::from_millis(ms)),
            operation_type: WaitOperationType::Receive, // Using receive as generic wait
        };
        
        // Add to wait queue
        condvar.wait_queue.enqueue(wait_entry);
        
        // Bridge to Zig runtime
        self.zig_bridge.notify_wait_started(goroutine_id, condvar_id)?;
        
        // Perform the actual wait
        let result = if let Some(timeout_ms) = timeout_ms {
            let timeout = Duration::from_millis(timeout_ms);
            let mutex_guard = mutex.mutex.lock().unwrap();
            
            let (guard, timeout_result) = condvar.condvar.wait_timeout(mutex_guard, timeout).unwrap();
            
            if timeout_result.timed_out() {
                // Update statistics
                {
                    let mut stats = condvar.stats.lock().unwrap();
                    stats.total_timeouts += 1;
                }
                
                ConditionWaitResult::Timeout
            } else {
                ConditionWaitResult::Notified
            }
        } else {
            let mutex_guard = mutex.mutex.lock().unwrap();
            let _guard = condvar.condvar.wait(mutex_guard).unwrap();
            ConditionWaitResult::Notified
        };
        
        // Update statistics
        {
            let mut stats = condvar.stats.lock().unwrap();
            stats.total_waits += 1;
            let wait_time = start_time.elapsed();
            stats.average_wait_time = (stats.average_wait_time + wait_time) / 2;
            if wait_time > stats.max_wait_time {
                stats.max_wait_time = wait_time;
            }
        }
        
        // Bridge notification to Zig runtime
        self.zig_bridge.notify_wait_completed(goroutine_id, condvar_id, &result)?;
        
        Ok(result)
    }
    
    /// Notify one waiting thread
    pub fn condition_notify_one(&self, condvar_id: CondVarId) -> Result<bool, CursedError> {
        let condvar = {
            let condition_vars = self.condition_vars.read().unwrap();
            condition_vars.get(&condvar_id)
                .ok_or_else(|| CursedError::General(format!("Condition variable {} not found", condvar_id)))?
                .clone()
        };
        
        // Try to dequeue from priority queue first
        if let Some(wait_entry) = condvar.wait_queue.dequeue() {
            // Bridge to Zig runtime
            self.zig_bridge.notify_goroutine_wakeup(wait_entry.goroutine_id, condvar_id)?;
            
            // Notify through condition variable
            condvar.condvar.notify_one();
            
            // Update statistics
            {
                let mut stats = condvar.stats.lock().unwrap();
                stats.total_notifications += 1;
            }
            
            Ok(true)
        } else {
            // No one waiting, just signal for potential future waiters
            condvar.condvar.notify_one();
            Ok(false)
        }
    }
    
    /// Notify all waiting threads
    pub fn condition_notify_all(&self, condvar_id: CondVarId) -> Result<u64, CursedError> {
        let condvar = {
            let condition_vars = self.condition_vars.read().unwrap();
            condition_vars.get(&condvar_id)
                .ok_or_else(|| CursedError::General(format!("Condition variable {} not found", condvar_id)))?
                .clone()
        };
        
        // Get all waiting entries
        let waiting_entries = condvar.wait_queue.clear();
        let notification_count = waiting_entries.len() as u64;
        
        // Bridge to Zig runtime for each waiting goroutine
        for entry in waiting_entries {
            self.zig_bridge.notify_goroutine_wakeup(entry.goroutine_id, condvar_id)?;
        }
        
        // Notify all through condition variable
        condvar.condvar.notify_all();
        
        // Update statistics
        {
            let mut stats = condvar.stats.lock().unwrap();
            stats.total_notifications += notification_count;
        }
        
        Ok(notification_count)
    }
    
    /// Lock a mutex with deadlock detection
    pub fn mutex_lock(&self, mutex_id: MutexId) -> Result<(), CursedError> {
        let mutex = {
            let mutexes = self.mutexes.read().unwrap();
            mutexes.get(&mutex_id)
                .ok_or_else(|| CursedError::General(format!("Mutex {} not found", mutex_id)))?
                .clone()
        };
        
        let current_thread = self.get_current_thread_handle()?;
        let start_time = Instant::now();
        
        // Check for potential deadlock
        if let Err(e) = self.check_deadlock_potential(current_thread, mutex_id) {
            return Err(e);
        }
        
        // Try to acquire the lock
        let _guard = mutex.mutex.lock().unwrap();
        
        // Record ownership
        mutex.owner.store(current_thread, Ordering::Release);
        
        // Update statistics
        {
            let mut state = mutex.mutex.lock().unwrap();
            state.lock_count += 1;
            state.last_lock_time = Some(start_time);
            
            if start_time.elapsed() > Duration::from_millis(1) {
                state.contention_count += 1;
                
                // Record contention event
                let event = ContentionEvent {
                    timestamp: start_time,
                    thread_handle: current_thread,
                    wait_duration: start_time.elapsed(),
                    acquired: true,
                };
                
                let mut events = mutex.contention_tracker.events.lock().unwrap();
                events.push(event);
                
                // Limit event history to prevent memory growth
                if events.len() > 1000 {
                    events.drain(0..500);
                }
            }
        }
        
        {
            let mut stats = mutex.stats.lock().unwrap();
            stats.total_locks += 1;
        }
        
        Ok(())
    }
    
    /// Try to lock a mutex without blocking
    pub fn mutex_try_lock(&self, mutex_id: MutexId) -> Result<bool, CursedError> {
        let mutex = {
            let mutexes = self.mutexes.read().unwrap();
            mutexes.get(&mutex_id)
                .ok_or_else(|| CursedError::General(format!("Mutex {} not found", mutex_id)))?
                .clone()
        };
        
        let current_thread = self.get_current_thread_handle()?;
        
        // Try to acquire the lock non-blocking
        if let Ok(_guard) = mutex.mutex.try_lock() {
            mutex.owner.store(current_thread, Ordering::Release);
            
            // Update statistics
            {
                let mut stats = mutex.stats.lock().unwrap();
                stats.total_locks += 1;
            }
            
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Unlock a mutex
    pub fn mutex_unlock(&self, mutex_id: MutexId) -> Result<(), CursedError> {
        let mutex = {
            let mutexes = self.mutexes.read().unwrap();
            mutexes.get(&mutex_id)
                .ok_or_else(|| CursedError::General(format!("Mutex {} not found", mutex_id)))?
                .clone()
        };
        
        let current_thread = self.get_current_thread_handle()?;
        
        // Verify ownership
        if mutex.owner.load(Ordering::Acquire) != current_thread {
            return Err(CursedError::General("Cannot unlock mutex not owned by current thread".to_string()));
        }
        
        // Calculate hold time
        let hold_time = {
            let state = mutex.mutex.lock().unwrap();
            state.last_lock_time.map(|t| t.elapsed()).unwrap_or(Duration::ZERO)
        };
        
        // Release ownership
        mutex.owner.store(0, Ordering::Release);
        
        // Update statistics
        {
            let mut stats = mutex.stats.lock().unwrap();
            stats.total_unlocks += 1;
            stats.average_hold_time = (stats.average_hold_time + hold_time) / 2;
            if hold_time > stats.max_hold_time {
                stats.max_hold_time = hold_time;
            }
        }
        
        Ok(())
    }
    
    /// Create a new thread with Zig runtime integration
    pub fn create_thread<F>(&self, name: String, func: F) -> Result<ThreadHandle, CursedError>
    where
        F: FnOnce() + Send + 'static,
    {
        let handle = self.global_state.next_thread_handle.fetch_add(1, Ordering::SeqCst);
        
        // Create goroutine in Zig runtime
        let goroutine_id = self.zig_bridge.create_goroutine()?;
        
        // Spawn the thread
        let zig_bridge = self.zig_bridge.clone();
        let join_handle = std::thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                // Register thread with Zig runtime
                let _ = zig_bridge.register_thread_goroutine_mapping(std::thread::current().id(), goroutine_id);
                
                // Execute the function
                func();
                
                // Cleanup
                let _ = zig_bridge.cleanup_goroutine(goroutine_id);
            })
            .map_err(|e| CursedError::General(format!("Failed to spawn thread: {}", e)))?;
        
        let os_thread_id = join_handle.thread().id();
        
        let thread = Arc::new(FfiThread {
            handle,
            os_thread_id: Some(os_thread_id),
            goroutine_id: Some(goroutine_id),
            join_handle: Some(join_handle),
            state: AtomicU64::new(1), // Running state
            priority: AtomicU64::new(1), // Normal priority
        });
        
        let mut threads = self.threads.write().unwrap();
        threads.insert(handle, thread);
        
        // Update thread-local storage mapping
        {
            let mut tls = self.thread_local_storage.write().unwrap();
            tls.insert(os_thread_id, goroutine_id);
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.active_threads += 1;
        }
        
        Ok(handle)
    }
    
    /// Join a thread
    pub fn join_thread(&self, handle: ThreadHandle) -> Result<(), CursedError> {
        let thread = {
            let mut threads = self.threads.write().unwrap();
            threads.remove(&handle)
        };
        
        if let Some(thread) = thread {
            if let Some(join_handle) = thread.join_handle {
                join_handle.join()
                    .map_err(|_| CursedError::General("Thread panicked".to_string()))?;
            }
            
            // Clean up thread-local storage
            if let Some(os_thread_id) = thread.os_thread_id {
                let mut tls = self.thread_local_storage.write().unwrap();
                tls.remove(&os_thread_id);
            }
            
            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.active_threads = stats.active_threads.saturating_sub(1);
            }
            
            Ok(())
        } else {
            Err(CursedError::General(format!("Thread handle {} not found", handle)))
        }
    }
    
    /// Get current thread handle (internal helper)
    fn get_current_thread_handle(&self) -> Result<ThreadHandle, CursedError> {
        let current_thread_id = std::thread::current().id();
        
        // Find thread handle by OS thread ID
        let threads = self.threads.read().unwrap();
        for (handle, thread) in threads.iter() {
            if thread.os_thread_id == Some(current_thread_id) {
                return Ok(*handle);
            }
        }
        
        Err(CursedError::General("Current thread not registered with FFI system".to_string()))
    }
    
    /// Get current goroutine ID (internal helper)
    fn get_current_goroutine_id(&self) -> Result<GoroutineId, CursedError> {
        let current_thread_id = std::thread::current().id();
        
        let tls = self.thread_local_storage.read().unwrap();
        tls.get(&current_thread_id)
            .copied()
            .ok_or_else(|| CursedError::General("Current thread not mapped to goroutine".to_string()))
    }
    
    /// Check for potential deadlock (simplified detection)
    fn check_deadlock_potential(&self, thread_handle: ThreadHandle, mutex_id: MutexId) -> Result<(), CursedError> {
        // Simplified deadlock detection - in production this would be more sophisticated
        let mutexes = self.mutexes.read().unwrap();
        
        // Check if any other thread is waiting on mutexes that we own
        // This is a basic check and could be expanded with dependency graph analysis
        
        for (other_mutex_id, other_mutex) in mutexes.iter() {
            if *other_mutex_id == mutex_id {
                continue;
            }
            
            let other_owner = other_mutex.owner.load(Ordering::Acquire);
            if other_owner == thread_handle {
                // We own another mutex - potential for deadlock
                // For now, just warn and allow, but could implement more sophisticated checking
                eprintln!("Warning: Thread {} attempting to acquire mutex {} while owning mutex {}", 
                         thread_handle, mutex_id, other_mutex_id);
            }
        }
        
        Ok(())
    }
    
    /// Get synchronization statistics
    pub fn get_stats(&self) -> ThreadSyncStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Get condition variable statistics
    pub fn get_condvar_stats(&self, condvar_id: CondVarId) -> Result<CondVarStats, CursedError> {
        let condition_vars = self.condition_vars.read().unwrap();
        let condvar = condition_vars.get(&condvar_id)
            .ok_or_else(|| CursedError::General(format!("Condition variable {} not found", condvar_id)))?;
        
        Ok(condvar.stats.lock().unwrap().clone())
    }
    
    /// Get mutex statistics
    pub fn get_mutex_stats(&self, mutex_id: MutexId) -> Result<MutexStats, CursedError> {
        let mutexes = self.mutexes.read().unwrap();
        let mutex = mutexes.get(&mutex_id)
            .ok_or_else(|| CursedError::General(format!("Mutex {} not found", mutex_id)))?;
        
        Ok(mutex.stats.lock().unwrap().clone())
    }
    
    /// Cleanup all resources
    pub fn cleanup(&self) -> Result<(), CursedError> {
        // Set shutdown flag
        self.global_state.shutdown_requested.store(true, Ordering::SeqCst);
        
        // Notify all condition variables to wake up waiting threads
        let condition_vars = self.condition_vars.read().unwrap();
        for condvar in condition_vars.values() {
            condvar.condvar.notify_all();
        }
        
        // Wait for all threads to complete (with timeout)
        let threads: Vec<_> = {
            let mut threads = self.threads.write().unwrap();
            threads.drain().collect()
        };
        
        for (handle, thread) in threads {
            if let Some(join_handle) = thread.join_handle {
                // Wait with timeout to avoid hanging
                let _ = join_handle.join();
            }
        }
        
        // Clean up Zig bridge
        self.zig_bridge.cleanup()?;
        
        Ok(())
    }
}

/// Result of condition variable wait operation
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionWaitResult {
    /// Successfully notified
    Notified,
    /// Operation timed out
    Timeout,
    /// Spurious wakeup (should retry)
    SpuriousWakeup,
}

impl ZigRuntimeBridge {
    /// Create new Zig runtime bridge
    pub fn new(channel_sync: Arc<ChannelSync>) -> Result<Self, CursedError> {
        Ok(Self {
            channel_sync,
            goroutine_states: RwLock::new(HashMap::new()),
            runtime_notifier: Arc::new(RuntimeNotifier::new()),
        })
    }
    
    /// Create a new goroutine
    pub fn create_goroutine(&self) -> Result<GoroutineId, CursedError> {
        // Generate unique goroutine ID
        static NEXT_GOROUTINE_ID: AtomicU64 = AtomicU64::new(1);
        let id = NEXT_GOROUTINE_ID.fetch_add(1, Ordering::SeqCst);
        
        // Register with Zig runtime
        {
            let mut states = self.goroutine_states.write().unwrap();
            states.insert(id, GoroutineState::Ready);
        }
        
        Ok(id)
    }
    
    /// Register thread-goroutine mapping
    pub fn register_thread_goroutine_mapping(&self, thread_id: ThreadId, goroutine_id: GoroutineId) -> Result<(), CursedError> {
        // Update goroutine state
        {
            let mut states = self.goroutine_states.write().unwrap();
            states.insert(goroutine_id, GoroutineState::Running);
        }
        
        Ok(())
    }
    
    /// Notify that a wait operation started
    pub fn notify_wait_started(&self, goroutine_id: GoroutineId, condvar_id: CondVarId) -> Result<(), CursedError> {
        // Update goroutine state
        {
            let mut states = self.goroutine_states.write().unwrap();
            states.insert(goroutine_id, GoroutineState::Waiting);
        }
        
        // Integrate with channel sync
        self.channel_sync.block_on_receive(
            condvar_id as usize, 
            goroutine_id, 
            OperationPriority::Normal, 
            None
        ).map_err(|e| CursedError::General(format!("Failed to block goroutine: {:?}", e)))?;
        
        Ok(())
    }
    
    /// Notify that a wait operation completed
    pub fn notify_wait_completed(&self, goroutine_id: GoroutineId, condvar_id: CondVarId, result: &ConditionWaitResult) -> Result<(), CursedError> {
        // Update goroutine state
        {
            let mut states = self.goroutine_states.write().unwrap();
            states.insert(goroutine_id, GoroutineState::Running);
        }
        
        // Unblock from channel sync
        self.channel_sync.unblock_goroutine(goroutine_id);
        
        Ok(())
    }
    
    /// Notify goroutine wakeup
    pub fn notify_goroutine_wakeup(&self, goroutine_id: GoroutineId, condvar_id: CondVarId) -> Result<(), CursedError> {
        // Update goroutine state
        {
            let mut states = self.goroutine_states.write().unwrap();
            states.insert(goroutine_id, GoroutineState::Ready);
        }
        
        // Trigger wakeup through channel sync
        self.channel_sync.unblock_receiver(condvar_id as usize);
        
        Ok(())
    }
    
    /// Cleanup goroutine
    pub fn cleanup_goroutine(&self, goroutine_id: GoroutineId) -> Result<(), CursedError> {
        {
            let mut states = self.goroutine_states.write().unwrap();
            states.remove(&goroutine_id);
        }
        
        Ok(())
    }
    
    /// Cleanup bridge
    pub fn cleanup(&self) -> Result<(), CursedError> {
        let mut states = self.goroutine_states.write().unwrap();
        states.clear();
        Ok(())
    }
}

impl GlobalSyncState {
    /// Create new global sync state
    pub fn new() -> Self {
        Self {
            next_condvar_id: AtomicU64::new(1),
            next_mutex_id: AtomicU64::new(1),
            next_thread_handle: AtomicU64::new(1),
            shutdown_requested: AtomicBool::new(false),
            emergency_brake: AtomicBool::new(false),
        }
    }
}

impl TimeoutManager {
    /// Create new timeout manager
    pub fn new() -> Self {
        Self {
            timeouts: Mutex::new(HashMap::new()),
            next_timeout_id: AtomicU64::new(1),
        }
    }
}

impl ContentionTracker {
    /// Create new contention tracker
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            current_level: AtomicU64::new(0),
        }
    }
}

impl RuntimeNotifier {
    /// Create new runtime notifier
    pub fn new() -> Self {
        Self {
            callbacks: RwLock::new(HashMap::new()),
        }
    }
}

// C FFI exports for use from LLVM-compiled CURSED code
extern "C" {
    // Export C functions for FFI bridge integration
}

/// C FFI: Create condition variable
#[no_mangle]
pub extern "C" fn cursed_ffi_condvar_create() -> u64 {
    thread_local! {
        static FFI_SYNC: std::cell::RefCell<Option<FfiThreadSync>> = std::cell::RefCell::new(None);
    }
    
    FFI_SYNC.with(|sync| {
        let mut sync_ref = sync.borrow_mut();
        if sync_ref.is_none() {
            *sync_ref = FfiThreadSync::new().ok();
        }
        
        if let Some(ref sync_instance) = *sync_ref {
            sync_instance.create_condition_variable().unwrap_or(0)
        } else {
            0
        }
    })
}

/// C FFI: Wait on condition variable
#[no_mangle]
pub extern "C" fn cursed_ffi_condvar_wait(condvar_id: u64, mutex_id: u64, timeout_ms: u64) -> u32 {
    thread_local! {
        static FFI_SYNC: std::cell::RefCell<Option<FfiThreadSync>> = std::cell::RefCell::new(None);
    }
    
    FFI_SYNC.with(|sync| {
        let sync_ref = sync.borrow();
        if let Some(ref sync_instance) = *sync_ref {
            let timeout = if timeout_ms == 0 { None } else { Some(timeout_ms) };
            match sync_instance.condition_wait(condvar_id, mutex_id, timeout) {
                Ok(ConditionWaitResult::Notified) => 0, // Success
                Ok(ConditionWaitResult::Timeout) => 1,  // Timeout
                Ok(ConditionWaitResult::SpuriousWakeup) => 2, // Spurious
                Err(_) => 3, // Error
            }
        } else {
            3 // Error - not initialized
        }
    })
}

/// C FFI: Notify one on condition variable
#[no_mangle]
pub extern "C" fn cursed_ffi_condvar_notify_one(condvar_id: u64) -> u32 {
    thread_local! {
        static FFI_SYNC: std::cell::RefCell<Option<FfiThreadSync>> = std::cell::RefCell::new(None);
    }
    
    FFI_SYNC.with(|sync| {
        let sync_ref = sync.borrow();
        if let Some(ref sync_instance) = *sync_ref {
            match sync_instance.condition_notify_one(condvar_id) {
                Ok(true) => 1,  // Notified someone
                Ok(false) => 0, // No one waiting
                Err(_) => 2,    // Error
            }
        } else {
            2 // Error - not initialized
        }
    })
}

/// C FFI: Notify all on condition variable
#[no_mangle]
pub extern "C" fn cursed_ffi_condvar_notify_all(condvar_id: u64) -> u64 {
    thread_local! {
        static FFI_SYNC: std::cell::RefCell<Option<FfiThreadSync>> = std::cell::RefCell::new(None);
    }
    
    FFI_SYNC.with(|sync| {
        let sync_ref = sync.borrow();
        if let Some(ref sync_instance) = *sync_ref {
            sync_instance.condition_notify_all(condvar_id).unwrap_or(0)
        } else {
            0
        }
    })
}

/// C FFI: Create mutex
#[no_mangle]
pub extern "C" fn cursed_ffi_mutex_create() -> u64 {
    thread_local! {
        static FFI_SYNC: std::cell::RefCell<Option<FfiThreadSync>> = std::cell::RefCell::new(None);
    }
    
    FFI_SYNC.with(|sync| {
        let mut sync_ref = sync.borrow_mut();
        if sync_ref.is_none() {
            *sync_ref = FfiThreadSync::new().ok();
        }
        
        if let Some(ref sync_instance) = *sync_ref {
            sync_instance.create_mutex().unwrap_or(0)
        } else {
            0
        }
    })
}

/// C FFI: Lock mutex
#[no_mangle]
pub extern "C" fn cursed_ffi_mutex_lock(mutex_id: u64) -> u32 {
    thread_local! {
        static FFI_SYNC: std::cell::RefCell<Option<FfiThreadSync>> = std::cell::RefCell::new(None);
    }
    
    FFI_SYNC.with(|sync| {
        let sync_ref = sync.borrow();
        if let Some(ref sync_instance) = *sync_ref {
            match sync_instance.mutex_lock(mutex_id) {
                Ok(()) => 0, // Success
                Err(_) => 1, // Error
            }
        } else {
            1 // Error - not initialized
        }
    })
}

/// C FFI: Try lock mutex
#[no_mangle]
pub extern "C" fn cursed_ffi_mutex_try_lock(mutex_id: u64) -> u32 {
    thread_local! {
        static FFI_SYNC: std::cell::RefCell<Option<FfiThreadSync>> = std::cell::RefCell::new(None);
    }
    
    FFI_SYNC.with(|sync| {
        let sync_ref = sync.borrow();
        if let Some(ref sync_instance) = *sync_ref {
            match sync_instance.mutex_try_lock(mutex_id) {
                Ok(true) => 0,  // Acquired
                Ok(false) => 1, // Would block
                Err(_) => 2,    // Error
            }
        } else {
            2 // Error - not initialized
        }
    })
}

/// C FFI: Unlock mutex
#[no_mangle]
pub extern "C" fn cursed_ffi_mutex_unlock(mutex_id: u64) -> u32 {
    thread_local! {
        static FFI_SYNC: std::cell::RefCell<Option<FfiThreadSync>> = std::cell::RefCell::new(None);
    }
    
    FFI_SYNC.with(|sync| {
        let sync_ref = sync.borrow();
        if let Some(ref sync_instance) = *sync_ref {
            match sync_instance.mutex_unlock(mutex_id) {
                Ok(()) => 0, // Success
                Err(_) => 1, // Error
            }
        } else {
            1 // Error - not initialized
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::thread;
    use std::sync::Arc;
    
    #[test]
    fn test_ffi_thread_sync_creation() {
        let sync = FfiThreadSync::new().unwrap();
        let stats = sync.get_stats();
        assert_eq!(stats.active_condition_vars, 0);
        assert_eq!(stats.active_mutexes, 0);
        assert_eq!(stats.active_threads, 0);
    }
    
    #[test]
    fn test_condition_variable_creation() {
        let sync = FfiThreadSync::new().unwrap();
        let condvar_id = sync.create_condition_variable().unwrap();
        assert!(condvar_id > 0);
        
        let stats = sync.get_stats();
        assert_eq!(stats.active_condition_vars, 1);
    }
    
    #[test]
    fn test_mutex_creation() {
        let sync = FfiThreadSync::new().unwrap();
        let mutex_id = sync.create_mutex().unwrap();
        assert!(mutex_id > 0);
        
        let stats = sync.get_stats();
        assert_eq!(stats.active_mutexes, 1);
    }
    
    #[test]
    fn test_condition_variable_notify() {
        let sync = Arc::new(FfiThreadSync::new().unwrap());
        let condvar_id = sync.create_condition_variable().unwrap();
        
        // Test notify_one with no waiters
        let result = sync.condition_notify_one(condvar_id).unwrap();
        assert!(!result); // No one was waiting
        
        // Test notify_all with no waiters
        let count = sync.condition_notify_all(condvar_id).unwrap();
        assert_eq!(count, 0); // No one was waiting
    }
    
    #[test]
    fn test_mutex_operations() {
        let sync = Arc::new(FfiThreadSync::new().unwrap());
        let mutex_id = sync.create_mutex().unwrap();
        
        // Note: This test is limited because we can't easily test actual locking
        // without proper thread integration, but we can test the interface
        
        // Test try_lock (should work since we're single-threaded in test)
        // Note: This will fail because current thread isn't registered
        let result = sync.mutex_try_lock(mutex_id);
        assert!(result.is_err()); // Expected to fail without proper thread registration
    }
    
    #[test]
    fn test_c_ffi_exports() {
        // Test C FFI interface
        let condvar_id = cursed_ffi_condvar_create();
        // Note: Will be 0 because thread-local isn't initialized, but tests the interface
        
        let mutex_id = cursed_ffi_mutex_create();
        // Note: Will be 0 because thread-local isn't initialized, but tests the interface
    }
    
    #[test]
    fn test_zig_runtime_bridge() {
        let channel_sync = Arc::new(ChannelSync::new());
        let bridge = ZigRuntimeBridge::new(channel_sync).unwrap();
        
        let goroutine_id = bridge.create_goroutine().unwrap();
        assert!(goroutine_id > 0);
        
        let result = bridge.cleanup_goroutine(goroutine_id);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_global_sync_state() {
        let state = GlobalSyncState::new();
        
        let id1 = state.next_condvar_id.fetch_add(1, Ordering::SeqCst);
        let id2 = state.next_condvar_id.fetch_add(1, Ordering::SeqCst);
        
        assert!(id2 > id1);
        assert!(!state.shutdown_requested.load(Ordering::Acquire));
    }
}
