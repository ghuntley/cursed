//! Enhanced goroutine runtime scheduler for the CURSED language
//!
//! This module provides a comprehensive goroutine scheduler that manages
//! goroutine lifecycles, thread pool management, and garbage collection
//! integration for efficient concurrent execution.

use std::sync::{Arc, Mutex, RwLock, Condvar, atomic::{AtomicU64, AtomicUsize, AtomicBool, Ordering}};
use std::thread::{self, JoinHandle};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::ffi::c_void;
use crate::memory::{GarbageCollector, ThreadSafeGc, Traceable};
use crate::object_thread_safe::ThreadSafeObject;
use tracing::{instrument, debug, info, warn, error, span, Level};
use serde::{Serialize, Deserialize};

/// Unique identifier for goroutines
pub type GoroutineId = u64;

/// A function pointer type for goroutine functions
pub type GoroutineFunction = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

/// Goroutine lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoroutineState {
    /// Just created, not yet scheduled
    Created,
    /// Waiting to be executed
    Runnable,
    /// Currently executing on a thread
    Running,
    /// Blocked on I/O or synchronization
    Blocked,
    /// Execution completed successfully
    Terminated,
    /// Execution completed with panic
    Panicked,
}

/// Goroutine metadata and state information
#[derive(Debug)]
pub struct GoroutineMetadata {
    /// Unique identifier
    pub id: GoroutineId,
    /// Current state
    pub state: GoroutineState,
    /// Creation timestamp
    pub created_at: Instant,
    /// Start execution timestamp
    pub started_at: Option<Instant>,
    /// Completion timestamp
    pub completed_at: Option<Instant>,
    /// Function pointer
    pub func: GoroutineFunction,
    /// Data pointer (stored as usize for Send/Sync safety)
    pub data: usize,
    /// Thread handle if running
    pub handle: Option<JoinHandle<GoroutineResult>>,
    /// References to GC objects owned by this goroutine
    pub gc_references: Vec<usize>,
}

/// Result of goroutine execution
#[derive(Debug)]
pub enum GoroutineResult {
    /// Successful completion with optional result pointer
    Success(Option<*mut c_void>),
    /// Panic during execution
    Panic(String),
}

/// Thread pool configuration
#[derive(Debug, Clone)]
pub struct ThreadPoolConfig {
    /// Minimum number of threads in the pool
    pub min_threads: usize,
    /// Maximum number of threads in the pool
    pub max_threads: usize,
    /// Maximum time a worker thread can be idle before termination
    pub max_idle_time: Duration,
    /// Maximum queue size for pending goroutines
    pub max_queue_size: usize,
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        
        Self {
            min_threads: 1,
            max_threads: cpu_count * 2,
            max_idle_time: Duration::from_secs(60),
            max_queue_size: 10000,
        }
    }
}

/// Statistics about scheduler performance
#[derive(Debug, Default)]
pub struct SchedulerStatistics {
    /// Total goroutines created
    pub total_created: AtomicU64,
    /// Total goroutines completed successfully
    pub total_completed: AtomicU64,
    /// Total goroutines that panicked
    pub total_panicked: AtomicU64,
    /// Current number of active goroutines
    pub active_count: AtomicUsize,
    /// Current number of queued goroutines
    pub queued_count: AtomicUsize,
    /// Current number of worker threads
    pub worker_count: AtomicUsize,
    /// Total execution time of all goroutines
    pub total_execution_time: AtomicU64,
    /// Average execution time (in nanoseconds)
    pub avg_execution_time: AtomicU64,
}

/// Worker thread that executes goroutines
struct WorkerThread {
    /// Thread handle
    handle: JoinHandle<()>,
    /// Worker thread ID
    id: usize,
    /// Last activity timestamp
    last_active: Arc<RwLock<Instant>>,
}

/// Enhanced goroutine scheduler with thread pool management
pub struct GoroutineScheduler {
    /// Configuration
    config: ThreadPoolConfig,
    /// Counter for generating unique goroutine IDs
    next_id: AtomicU64,
    /// Counter for generating unique worker IDs
    next_worker_id: AtomicUsize,
    /// Active goroutines metadata
    goroutines: RwLock<HashMap<GoroutineId, GoroutineMetadata>>,
    /// Queue of goroutines waiting to be executed
    work_queue: Mutex<VecDeque<GoroutineId>>,
    /// Condition variable for work queue notifications
    work_available: Condvar,
    /// Worker threads
    workers: RwLock<HashMap<usize, WorkerThread>>,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Garbage collector reference
    gc: Arc<GarbageCollector>,
    /// Statistics
    stats: SchedulerStatistics,
}

impl GoroutineScheduler {
    /// Create a new enhanced goroutine scheduler
    pub fn new(config: ThreadPoolConfig, gc: Arc<GarbageCollector>) -> Self {
        let scheduler = Self {
            config,
            next_id: AtomicU64::new(1),
            next_worker_id: AtomicUsize::new(1),
            goroutines: RwLock::new(HashMap::new()),
            work_queue: Mutex::new(VecDeque::new()),
            work_available: Condvar::new(),
            workers: RwLock::new(HashMap::new()),
            shutdown: AtomicBool::new(false),
            gc,
            stats: SchedulerStatistics::default(),
        };
        
        // Start minimum number of worker threads
        for _ in 0..scheduler.config.min_threads {
            scheduler.spawn_worker();
        }
        
        scheduler
    }

    /// Create a new scheduler with default configuration
    pub fn with_defaults(gc: Arc<GarbageCollector>) -> Self {
        Self::new(ThreadPoolConfig::default(), gc)
    }

    /// Spawn a new goroutine
    #[instrument(level = "debug", skip(self, func, data))]
    pub fn spawn_goroutine(&self, func: GoroutineFunction, data: *mut c_void) -> GoroutineId {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let created_at = Instant::now();
        
        debug!(goroutine_id = id, "Creating new goroutine");
        
        // Create goroutine metadata
        let metadata = GoroutineMetadata {
            id,
            state: GoroutineState::Created,
            created_at,
            started_at: None,
            completed_at: None,
            func,
            data: data as usize,
            handle: None,
            gc_references: Vec::new(),
        };
        
        // Store metadata
        {
            let mut goroutines = self.goroutines.write().unwrap();
            goroutines.insert(id, metadata);
        }
        
        // Add to work queue
        {
            let mut queue = self.work_queue.lock().unwrap();
            if queue.len() >= self.config.max_queue_size {
                warn!(goroutine_id = id, "Work queue at capacity, goroutine may be delayed");
            }
            queue.push_back(id);
            self.stats.queued_count.fetch_add(1, Ordering::Relaxed);
        }
        
        // Update goroutine state to runnable
        self.update_goroutine_state(id, GoroutineState::Runnable);
        
        // Notify workers
        self.work_available.notify_one();
        
        // Update statistics
        self.stats.total_created.fetch_add(1, Ordering::Relaxed);
        
        // Check if we need more workers
        self.maybe_spawn_worker();
        
        id
    }

    /// Update goroutine state
    fn update_goroutine_state(&self, id: GoroutineId, new_state: GoroutineState) {
        if let Ok(mut goroutines) = self.goroutines.write() {
            if let Some(metadata) = goroutines.get_mut(&id) {
                let old_state = metadata.state;
                metadata.state = new_state;
                
                debug!(
                    goroutine_id = id, 
                    old_state = ?old_state, 
                    new_state = ?new_state,
                    "Goroutine state transition"
                );
                
                // Update timestamps
                match new_state {
                    GoroutineState::Running => {
                        metadata.started_at = Some(Instant::now());
                        self.stats.active_count.fetch_add(1, Ordering::Relaxed);
                    },
                    GoroutineState::Terminated | GoroutineState::Panicked => {
                        metadata.completed_at = Some(Instant::now());
                        self.stats.active_count.fetch_sub(1, Ordering::Relaxed);
                        
                        // Update execution time statistics
                        if let Some(started_at) = metadata.started_at {
                            let execution_time = metadata.completed_at.unwrap().duration_since(started_at);
                            let execution_ns = execution_time.as_nanos() as u64;
                            
                            self.stats.total_execution_time.fetch_add(execution_ns, Ordering::Relaxed);
                            
                            // Update average execution time
                            let completed_count = match new_state {
                                GoroutineState::Terminated => {
                                    self.stats.total_completed.fetch_add(1, Ordering::Relaxed) + 1
                                },
                                GoroutineState::Panicked => {
                                    self.stats.total_panicked.fetch_add(1, Ordering::Relaxed);
                                    self.stats.total_completed.load(Ordering::Relaxed)
                                },
                                _ => unreachable!(),
                            };
                            
                            if completed_count > 0 {
                                let total_time = self.stats.total_execution_time.load(Ordering::Relaxed);
                                let avg_time = total_time / completed_count;
                                self.stats.avg_execution_time.store(avg_time, Ordering::Relaxed);
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    /// Check if we need to spawn a new worker thread
    fn maybe_spawn_worker(&self) {
        let queue_size = {
            let queue = self.work_queue.lock().unwrap();
            queue.len()
        };
        
        let worker_count = {
            let workers = self.workers.read().unwrap();
            workers.len()
        };
        
        // Spawn new worker if queue is growing and we haven't reached max
        if queue_size > worker_count * 2 && worker_count < self.config.max_threads {
            self.spawn_worker();
        }
    }

    /// Spawn a new worker thread
    fn spawn_worker(&self) {
        let worker_id = self.next_worker_id.fetch_add(1, Ordering::SeqCst);
        let last_active = Arc::new(RwLock::new(Instant::now()));
        let last_active_clone = last_active.clone();
        
        // We need to use unsafe pointers since we can't clone Mutex/Condvar/RwLock
        // This is safe because the scheduler outlives all worker threads
        let work_queue_ptr = &self.work_queue as *const Mutex<VecDeque<GoroutineId>>;
        let work_available_ptr = &self.work_available as *const Condvar;
        let goroutines_ptr = &self.goroutines as *const RwLock<HashMap<GoroutineId, GoroutineMetadata>>;
        let shutdown_ptr = &self.shutdown as *const AtomicBool;
        let queued_count_ptr = &self.stats.queued_count as *const AtomicUsize;
        let gc = self.gc.clone();
        
        debug!(worker_id = worker_id, "Spawning new worker thread");
        
        let handle = thread::spawn(move || {
            let _span = span!(Level::DEBUG, "worker_thread", worker_id = worker_id).entered();
            debug!("Worker thread started");
            
            // SAFETY: Pointers are valid for the lifetime of the scheduler
            let work_queue = unsafe { &*work_queue_ptr };
            let work_available = unsafe { &*work_available_ptr };
            let goroutines = unsafe { &*goroutines_ptr };
            let shutdown = unsafe { &*shutdown_ptr };
            let queued_count = unsafe { &*queued_count_ptr };
            
            loop {
                // Check for shutdown
                if shutdown.load(Ordering::Acquire) {
                    debug!("Worker thread shutting down");
                    break;
                }
                
                // Get work from queue
                let goroutine_id = {
                    let mut queue = work_queue.lock().unwrap();
                    
                    // Wait for work or timeout
                    if queue.is_empty() {
                        let result = work_available.wait_timeout(queue, Duration::from_secs(10)).unwrap();
                        queue = result.0;
                        
                        // Check timeout for worker retirement
                        if result.1.timed_out() {
                            // TODO: Implement worker retirement logic
                            continue;
                        }
                    }
                    
                    if let Some(id) = queue.pop_front() {
                        queued_count.fetch_sub(1, Ordering::Relaxed);
                        Some(id)
                    } else {
                        None
                    }
                };
                
                if let Some(id) = goroutine_id {
                    // Update last active time
                    {
                        let mut last_active = last_active_clone.write().unwrap();
                        *last_active = Instant::now();
                    }
                    
                    Self::execute_goroutine(id, goroutines, &gc);
                }
            }
            
            debug!("Worker thread terminated");
        });
        
        let worker = WorkerThread {
            handle,
            id: worker_id,
            last_active,
        };
        
        // Store worker
        {
            let mut workers = self.workers.write().unwrap();
            workers.insert(worker_id, worker);
            self.stats.worker_count.store(workers.len(), Ordering::Relaxed);
        }
        
        debug!(worker_id = worker_id, "Worker thread spawned successfully");
    }

    /// Execute a goroutine
    fn execute_goroutine(
        goroutine_id: GoroutineId,
        goroutines: &RwLock<HashMap<GoroutineId, GoroutineMetadata>>,
        gc: &Arc<GarbageCollector>,
    ) {
        let _span = span!(Level::DEBUG, "execute_goroutine", goroutine_id = goroutine_id).entered();
        
        // Get goroutine metadata
        let (func, data) = {
            let goroutines_read = goroutines.read().unwrap();
            if let Some(metadata) = goroutines_read.get(&goroutine_id) {
                (metadata.func, metadata.data as *mut c_void)
            } else {
                error!("Goroutine metadata not found");
                return;
            }
        };
        
        // Update state to running
        {
            let mut goroutines_write = goroutines.write().unwrap();
            if let Some(metadata) = goroutines_write.get_mut(&goroutine_id) {
                metadata.state = GoroutineState::Running;
                metadata.started_at = Some(Instant::now());
            }
        }
        
        debug!("Executing goroutine");
        
        // Execute the goroutine function with panic handling
        let result = std::panic::catch_unwind(|| {
            unsafe { func(data) }
        });
        
        let goroutine_result = match result {
            Ok(return_value) => {
                debug!("Goroutine completed successfully");
                GoroutineResult::Success(if return_value.is_null() { None } else { Some(return_value) })
            },
            Err(panic_info) => {
                let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_info.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };
                
                error!(panic_message = %panic_msg, "Goroutine panicked");
                GoroutineResult::Panic(panic_msg)
            }
        };
        
        // Update final state
        {
            let mut goroutines_write = goroutines.write().unwrap();
            if let Some(metadata) = goroutines_write.get_mut(&goroutine_id) {
                metadata.completed_at = Some(Instant::now());
                metadata.state = match goroutine_result {
                    GoroutineResult::Success(_) => GoroutineState::Terminated,
                    GoroutineResult::Panic(_) => GoroutineState::Panicked,
                };
                
                // Clean up GC references
                for obj_id in &metadata.gc_references {
                    gc.remove_root(*obj_id);
                }
                metadata.gc_references.clear();
            }
        }
        
        // Trigger GC collection periodically
        // Note: GC collection is triggered by the GC itself based on allocation patterns
        debug!("Goroutine completed, GC will collect when needed");
    }

    /// Register a GC object reference for a goroutine
    pub fn register_gc_reference(&self, goroutine_id: GoroutineId, object_id: usize) {
        if let Ok(mut goroutines) = self.goroutines.write() {
            if let Some(metadata) = goroutines.get_mut(&goroutine_id) {
                metadata.gc_references.push(object_id);
                self.gc.add_root(object_id);
                debug!(
                    goroutine_id = goroutine_id,
                    object_id = object_id,
                    "Registered GC reference for goroutine"
                );
            }
        }
    }

    /// Wait for a specific goroutine to complete
    #[instrument(level = "debug", skip(self))]
    pub fn wait_for_goroutine(&self, id: GoroutineId) -> Result<GoroutineResult, String> {
        debug!(goroutine_id = id, "Waiting for goroutine completion");
        
        // Poll for completion
        loop {
            let completed = {
                let goroutines = self.goroutines.read()
                    .map_err(|_| "Failed to acquire goroutines lock")?;
                
                if let Some(metadata) = goroutines.get(&id) {
                    matches!(metadata.state, GoroutineState::Terminated | GoroutineState::Panicked)
                } else {
                    return Err("Goroutine not found".to_string());
                }
            };
            
            if completed {
                // Get the result
                let result = {
                    let goroutines = self.goroutines.read().unwrap();
                    let metadata = goroutines.get(&id).unwrap();
                    
                    match metadata.state {
                        GoroutineState::Terminated => GoroutineResult::Success(None),
                        GoroutineState::Panicked => GoroutineResult::Panic("Goroutine panicked".to_string()),
                        _ => unreachable!(),
                    }
                };
                
                debug!(goroutine_id = id, "Goroutine wait completed");
                return Ok(result);
            }
            
            // Sleep briefly to avoid busy waiting
            thread::sleep(Duration::from_millis(1));
        }
    }

    /// Wait for all goroutines to complete
    #[instrument(level = "debug", skip(self))]
    pub fn wait_all(&self) -> Result<(), String> {
        debug!("Waiting for all goroutines to complete");
        
        loop {
            let active_ids: Vec<GoroutineId> = {
                let goroutines = self.goroutines.read()
                    .map_err(|_| "Failed to acquire goroutines lock")?;
                
                goroutines.iter()
                    .filter(|(_, metadata)| !matches!(
                        metadata.state, 
                        GoroutineState::Terminated | GoroutineState::Panicked
                    ))
                    .map(|(id, _)| *id)
                    .collect()
            };
            
            if active_ids.is_empty() {
                debug!("All goroutines completed");
                return Ok(());
            }
            
            debug!(active_count = active_ids.len(), "Waiting for active goroutines");
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Get goroutine metadata
    pub fn get_goroutine_metadata(&self, id: GoroutineId) -> Option<GoroutineState> {
        let goroutines = self.goroutines.read().ok()?;
        goroutines.get(&id).map(|metadata| metadata.state)
    }

    /// Get scheduler statistics
    pub fn get_statistics(&self) -> SchedulerStatistics {
        SchedulerStatistics {
            total_created: AtomicU64::new(self.stats.total_created.load(Ordering::Relaxed)),
            total_completed: AtomicU64::new(self.stats.total_completed.load(Ordering::Relaxed)),
            total_panicked: AtomicU64::new(self.stats.total_panicked.load(Ordering::Relaxed)),
            active_count: AtomicUsize::new(self.stats.active_count.load(Ordering::Relaxed)),
            queued_count: AtomicUsize::new(self.stats.queued_count.load(Ordering::Relaxed)),
            worker_count: AtomicUsize::new(self.stats.worker_count.load(Ordering::Relaxed)),
            total_execution_time: AtomicU64::new(self.stats.total_execution_time.load(Ordering::Relaxed)),
            avg_execution_time: AtomicU64::new(self.stats.avg_execution_time.load(Ordering::Relaxed)),
        }
    }

    /// Clean up completed goroutines
    #[instrument(level = "debug", skip(self))]
    pub fn cleanup_completed(&self) {
        let mut cleanup_count = 0;
        
        {
            let mut goroutines = self.goroutines.write().unwrap();
            goroutines.retain(|id, metadata| {
                let should_remove = matches!(
                    metadata.state,
                    GoroutineState::Terminated | GoroutineState::Panicked
                ) && metadata.completed_at.map_or(false, |t| t.elapsed() > Duration::from_secs(60));
                
                if should_remove {
                    cleanup_count += 1;
                    debug!(goroutine_id = id, "Cleaning up completed goroutine");
                }
                
                !should_remove
            });
        }
        
        if cleanup_count > 0 {
            info!(cleanup_count = cleanup_count, "Cleaned up completed goroutines");
        }
    }

    /// Get the number of active goroutines
    pub fn active_count(&self) -> usize {
        self.stats.active_count.load(Ordering::Relaxed)
    }

    /// Get the number of queued goroutines
    pub fn queued_count(&self) -> usize {
        self.stats.queued_count.load(Ordering::Relaxed)
    }

    /// Shutdown the scheduler
    #[instrument(level = "info", skip(self))]
    pub fn shutdown(&self) {
        info!("Shutting down goroutine scheduler");
        
        // Set shutdown flag
        self.shutdown.store(true, Ordering::Release);
        
        // Notify all workers
        self.work_available.notify_all();
        
        // Wait for all workers to finish
        let workers = {
            let mut workers = self.workers.write().unwrap();
            workers.drain().collect::<Vec<_>>()
        };
        
        for (worker_id, worker) in workers {
            debug!(worker_id = worker_id, "Waiting for worker thread to finish");
            if let Err(_) = worker.handle.join() {
                warn!(worker_id = worker_id, "Worker thread panicked during shutdown");
            }
        }
        
        info!("Goroutine scheduler shutdown complete");
    }
}

impl Drop for GoroutineScheduler {
    fn drop(&mut self) {
        if !self.shutdown.load(Ordering::Acquire) {
            self.shutdown();
        }
    }
}

/// Global goroutine scheduler instance
static GLOBAL_SCHEDULER: once_cell::sync::Lazy<Arc<GoroutineScheduler>> = 
    once_cell::sync::Lazy::new(|| {
        let gc = Arc::new(GarbageCollector::new());
        Arc::new(GoroutineScheduler::with_defaults(gc))
    });

/// Get the global goroutine scheduler
pub fn get_global_scheduler() -> Arc<GoroutineScheduler> {
    GLOBAL_SCHEDULER.clone()
}

/// Initialize the global scheduler with custom configuration
pub fn initialize_global_scheduler(config: ThreadPoolConfig, gc: Arc<GarbageCollector>) {
    once_cell::sync::Lazy::force(&GLOBAL_SCHEDULER);
    // Note: In a real implementation, we'd need a way to replace the global scheduler
    // For now, this serves as documentation of the intended API
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::time::Duration;

    // Test function for goroutines
    unsafe extern "C" fn test_function(data: *mut c_void) -> *mut c_void {
        let counter = data as *mut AtomicI32;
        (*counter).fetch_add(1, Ordering::SeqCst);
        std::ptr::null_mut()
    }

    // Test function that panics
    unsafe extern "C" fn panic_function(_data: *mut c_void) -> *mut c_void {
        panic!("Test panic");
    }

    // Test function that takes some time
    unsafe extern "C" fn slow_function(data: *mut c_void) -> *mut c_void {
        let duration_ms = data as usize;
        std::thread::sleep(Duration::from_millis(duration_ms as u64));
        std::ptr::null_mut()
    }

    #[test]
    fn test_scheduler_creation() {
        let gc = Arc::new(GarbageCollector::new());
        let config = ThreadPoolConfig::default();
        let scheduler = GoroutineScheduler::new(config, gc);
        
        assert_eq!(scheduler.active_count(), 0);
        assert_eq!(scheduler.queued_count(), 0);
        
        let stats = scheduler.get_statistics();
        assert_eq!(stats.total_created.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_basic_goroutine_execution() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = GoroutineScheduler::with_defaults(gc);
        let counter = AtomicI32::new(0);
        
        let id = scheduler.spawn_goroutine(test_function, &counter as *const _ as *mut c_void);
        
        // Wait for completion
        let result = scheduler.wait_for_goroutine(id).unwrap();
        assert!(matches!(result, GoroutineResult::Success(_)));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        
        // Check state
        assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
    }

    #[test]
    fn test_multiple_goroutines() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = GoroutineScheduler::with_defaults(gc);
        let counter = AtomicI32::new(0);
        let num_goroutines = 10;
        
        let mut ids = Vec::new();
        for _ in 0..num_goroutines {
            let id = scheduler.spawn_goroutine(test_function, &counter as *const _ as *mut c_void);
            ids.push(id);
        }
        
        // Wait for all
        scheduler.wait_all().unwrap();
        
        assert_eq!(counter.load(Ordering::SeqCst), num_goroutines);
        
        // Check all are completed
        for id in ids {
            assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
        }
    }

    #[test]
    fn test_goroutine_panic_handling() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = GoroutineScheduler::with_defaults(gc);
        
        let id = scheduler.spawn_goroutine(panic_function, std::ptr::null_mut());
        
        // Wait for completion
        let result = scheduler.wait_for_goroutine(id).unwrap();
        assert!(matches!(result, GoroutineResult::Panic(_)));
        
        // Check state
        assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Panicked));
        
        let stats = scheduler.get_statistics();
        assert_eq!(stats.total_panicked.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_scheduler_statistics() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = GoroutineScheduler::with_defaults(gc);
        let counter = AtomicI32::new(0);
        
        // Spawn some goroutines
        let mut ids = Vec::new();
        for _ in 0..5 {
            let id = scheduler.spawn_goroutine(test_function, &counter as *const _ as *mut c_void);
            ids.push(id);
        }
        
        // Wait for completion
        scheduler.wait_all().unwrap();
        
        let stats = scheduler.get_statistics();
        assert_eq!(stats.total_created.load(Ordering::Relaxed), 5);
        assert_eq!(stats.total_completed.load(Ordering::Relaxed), 5);
        assert_eq!(stats.total_panicked.load(Ordering::Relaxed), 0);
        assert!(stats.avg_execution_time.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn test_cleanup_completed() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = GoroutineScheduler::with_defaults(gc);
        let counter = AtomicI32::new(0);
        
        let id = scheduler.spawn_goroutine(test_function, &counter as *const _ as *mut c_void);
        scheduler.wait_for_goroutine(id).unwrap();
        
        // Manually mark as old for cleanup
        {
            let mut goroutines = scheduler.goroutines.write().unwrap();
            if let Some(metadata) = goroutines.get_mut(&id) {
                metadata.completed_at = Some(Instant::now() - Duration::from_secs(61));
            }
        }
        
        scheduler.cleanup_completed();
        
        // Should be cleaned up
        assert_eq!(scheduler.get_goroutine_metadata(id), None);
    }

    #[test]
    fn test_gc_integration() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = GoroutineScheduler::with_defaults(gc.clone());
        let counter = AtomicI32::new(0);
        
        let id = scheduler.spawn_goroutine(test_function, &counter as *const _ as *mut c_void);
        
        // Register a fake GC reference
        scheduler.register_gc_reference(id, 12345);
        
        // Wait for completion
        scheduler.wait_for_goroutine(id).unwrap();
        
        // Check that GC references were cleaned up
        let goroutines = scheduler.goroutines.read().unwrap();
        let metadata = goroutines.get(&id).unwrap();
        assert!(metadata.gc_references.is_empty());
    }

    #[test]
    fn test_thread_pool_scaling() {
        let config = ThreadPoolConfig {
            min_threads: 1,
            max_threads: 4,
            max_idle_time: Duration::from_secs(1),
            max_queue_size: 100,
        };
        
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = GoroutineScheduler::new(config, gc);
        
        // Initial worker count should be minimum
        let initial_workers = scheduler.stats.worker_count.load(Ordering::Relaxed);
        assert!(initial_workers >= 1);
        
        // Spawn many slow goroutines to trigger scaling
        for i in 0..10 {
            scheduler.spawn_goroutine(slow_function, (100 * (i + 1)) as *mut c_void);
        }
        
        // Give some time for workers to spawn
        std::thread::sleep(Duration::from_millis(200));
        
        let workers_after_load = scheduler.stats.worker_count.load(Ordering::Relaxed);
        assert!(workers_after_load >= initial_workers);
        
        // Wait for all to complete
        scheduler.wait_all().unwrap();
    }
}
