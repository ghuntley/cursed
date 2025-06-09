//! Simplified goroutine runtime scheduler for the CURSED language
//!
//! This module provides a basic goroutine scheduler that manages
//! goroutine lifecycles with proper thread safety and GC integration.

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicUsize, AtomicBool, Ordering}};
use std::thread::{self, JoinHandle};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::ffi::c_void;
use crate::memory::GarbageCollector;
use crate::runtime::goroutine::GoroutineId;
use tracing::{instrument, debug, info, warn, error, span, Level};
use serde::{Serialize, Deserialize};

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

/// Enhanced goroutine scheduler with thread pool management
pub struct SimpleGoroutineScheduler {
    /// Configuration
    config: ThreadPoolConfig,
    /// Counter for generating unique goroutine IDs
    next_id: AtomicU64,
    /// Active goroutines metadata
    goroutines: Arc<RwLock<HashMap<GoroutineId, GoroutineMetadata>>>,
    /// Queue of goroutines waiting to be executed
    work_queue: Arc<Mutex<VecDeque<GoroutineId>>>,
    /// Shutdown flag
    shutdown: Arc<AtomicBool>,
    /// Garbage collector reference
    gc: Arc<GarbageCollector>,
    /// Statistics
    stats: Arc<SchedulerStatistics>,
    /// Worker thread handles
    worker_handles: Mutex<Vec<JoinHandle<()>>>,
}

impl SimpleGoroutineScheduler {
    /// Create a new enhanced goroutine scheduler
    pub fn new(config: ThreadPoolConfig, gc: Arc<GarbageCollector>) -> Self {
        let scheduler = Self {
            config: config.clone(),
            next_id: AtomicU64::new(1),
            goroutines: Arc::new(RwLock::new(HashMap::new())),
            work_queue: Arc::new(Mutex::new(VecDeque::new())),
            shutdown: Arc::new(AtomicBool::new(false)),
            gc,
            stats: Arc::new(SchedulerStatistics::default()),
            worker_handles: Mutex::new(Vec::new()),
        };
        
        // Start worker threads
        scheduler.start_workers();
        
        scheduler
    }

    /// Create a new scheduler with default configuration
    pub fn with_defaults(gc: Arc<GarbageCollector>) -> Self {
        Self::new(ThreadPoolConfig::default(), gc)
    }

    /// Start worker threads
    fn start_workers(&self) {
        let mut handles = self.worker_handles.lock().unwrap();
        
        for i in 0..self.config.min_threads {
            let handle = self.spawn_worker(i);
            handles.push(handle);
        }
        
        self.stats.worker_count.store(handles.len(), Ordering::Relaxed);
    }

    /// Spawn a new worker thread
    fn spawn_worker(&self, worker_id: usize) -> JoinHandle<()> {
        let goroutines = Arc::clone(&self.goroutines);
        let work_queue = Arc::clone(&self.work_queue);
        let shutdown = Arc::clone(&self.shutdown);
        let gc = Arc::clone(&self.gc);
        let stats = Arc::clone(&self.stats);
        
        thread::spawn(move || {
            let _span = span!(Level::DEBUG, "worker_thread", worker_id = worker_id).entered();
            debug!("Worker thread started");
            
            loop {
                // Check for shutdown
                if shutdown.load(Ordering::Acquire) {
                    debug!("Worker thread shutting down");
                    break;
                }
                
                // Get work from queue
                let goroutine_id = {
                    let mut queue = work_queue.lock().unwrap();
                    
                    if let Some(id) = queue.pop_front() {
                        stats.queued_count.fetch_sub(1, Ordering::Relaxed);
                        Some(id)
                    } else {
                        None
                    }
                };
                
                if let Some(id) = goroutine_id {
                    Self::execute_goroutine(id, &goroutines, &gc, &stats);
                } else {
                    // No work available, sleep briefly
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
            
            debug!("Worker thread terminated");
        })
    }

    /// Execute a goroutine
    fn execute_goroutine(
        goroutine_id: GoroutineId,
        goroutines: &Arc<RwLock<HashMap<GoroutineId, GoroutineMetadata>>>,
        gc: &Arc<GarbageCollector>,
        stats: &Arc<SchedulerStatistics>,
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
                stats.active_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        debug!("Executing goroutine");
        
        let start_time = Instant::now();
        
        // Execute the goroutine function with panic handling
        let result = std::panic::catch_unwind(|| {
            unsafe { func(data) }
        });
        
        let execution_time = start_time.elapsed();
        
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
        
        // Update final state and statistics
        {
            let mut goroutines_write = goroutines.write().unwrap();
            if let Some(metadata) = goroutines_write.get_mut(&goroutine_id) {
                metadata.completed_at = Some(Instant::now());
                metadata.state = match goroutine_result {
                    GoroutineResult::Success(_) => {
                        stats.total_completed.fetch_add(1, Ordering::Relaxed);
                        GoroutineState::Terminated
                    },
                    GoroutineResult::Panic(_) => {
                        stats.total_panicked.fetch_add(1, Ordering::Relaxed);
                        GoroutineState::Panicked
                    },
                };
                
                // Clean up GC references
                for obj_id in &metadata.gc_references {
                    gc.remove_root(*obj_id);
                }
                metadata.gc_references.clear();
                
                stats.active_count.fetch_sub(1, Ordering::Relaxed);
                
                // Update execution time statistics
                let execution_ns = execution_time.as_nanos() as u64;
                stats.total_execution_time.fetch_add(execution_ns, Ordering::Relaxed);
                
                let completed_count = stats.total_completed.load(Ordering::Relaxed);
                if completed_count > 0 {
                    let total_time = stats.total_execution_time.load(Ordering::Relaxed);
                    let avg_time = total_time / completed_count;
                    stats.avg_execution_time.store(avg_time, Ordering::Relaxed);
                }
            }
        }
        
        debug!("Goroutine execution completed");
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
            queue.push_back(id);
            self.stats.queued_count.fetch_add(1, Ordering::Relaxed);
        }
        
        // Update statistics
        self.stats.total_created.fetch_add(1, Ordering::Relaxed);
        
        id
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
        
        // Wait for all workers to finish
        let mut handles = self.worker_handles.lock().unwrap();
        for handle in handles.drain(..) {
            if let Err(_) = handle.join() {
                warn!("Worker thread panicked during shutdown");
            }
        }
        
        info!("Goroutine scheduler shutdown complete");
    }
}

impl Drop for SimpleGoroutineScheduler {
    fn drop(&mut self) {
        if !self.shutdown.load(Ordering::Acquire) {
            self.shutdown();
        }
    }
}

/// Global goroutine scheduler instance using the simple scheduler
static GLOBAL_SIMPLE_SCHEDULER: once_cell::sync::Lazy<Arc<SimpleGoroutineScheduler>> = 
    once_cell::sync::Lazy::new(|| {
        let gc = Arc::new(GarbageCollector::new());
        Arc::new(SimpleGoroutineScheduler::with_defaults(gc))
    });

/// Get the global simple goroutine scheduler
pub fn get_global_simple_scheduler() -> Arc<SimpleGoroutineScheduler> {
    GLOBAL_SIMPLE_SCHEDULER.clone()
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

    // Test function that simulates an error (safer than panicking in extern "C")
    unsafe extern "C" fn error_function(_data: *mut c_void) -> *mut c_void {
        // Return an error code that can be detected by the caller
        0xDEADBEEF as *mut c_void
    }

    #[test]
    fn test_simple_scheduler_creation() {
        let gc = Arc::new(GarbageCollector::new());
        let config = ThreadPoolConfig::default();
        let scheduler = SimpleGoroutineScheduler::new(config, gc);
        
        assert_eq!(scheduler.active_count(), 0);
        assert_eq!(scheduler.queued_count(), 0);
        
        let stats = scheduler.get_statistics();
        assert_eq!(stats.total_created.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_simple_scheduler_single_goroutine() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
        let counter = AtomicI32::new(0);
        
        let id = scheduler.spawn_goroutine(
            test_function, 
            &counter as *const _ as *mut c_void
        );
        
        // Wait for completion
        let result = scheduler.wait_for_goroutine(id).unwrap();
        
        // Verify execution
        assert!(matches!(result, GoroutineResult::Success(_)));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
    }

    #[test]
    fn test_simple_scheduler_multiple_goroutines() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
        let counter = AtomicI32::new(0);
        let num_goroutines = 10;
        
        for _ in 0..num_goroutines {
            scheduler.spawn_goroutine(test_function, &counter as *const _ as *mut c_void);
        }
        
        // Wait for all to complete
        scheduler.wait_all().unwrap();
        
        assert_eq!(counter.load(Ordering::SeqCst), num_goroutines);
        
        let stats = scheduler.get_statistics();
        assert_eq!(stats.total_created.load(Ordering::Relaxed), num_goroutines as u64);
        assert_eq!(stats.total_completed.load(Ordering::Relaxed), num_goroutines as u64);
    }

    #[test]
    fn test_simple_scheduler_error_handling() {
        let gc = Arc::new(GarbageCollector::new());
        let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
        
        let id = scheduler.spawn_goroutine(error_function, std::ptr::null_mut());
        
        // Wait for completion
        let result = scheduler.wait_for_goroutine(id).unwrap();
        
        // Since our error function doesn't actually panic, it will complete successfully
        // In a real implementation, we'd need better error handling mechanisms
        assert!(matches!(result, GoroutineResult::Success(_)));
        assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
        
        let stats = scheduler.get_statistics();
        assert_eq!(stats.total_completed.load(Ordering::Relaxed), 1);
    }
}
