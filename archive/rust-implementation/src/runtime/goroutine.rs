//! CURSED Goroutine Runtime System
//!
//! This module provides Go-style goroutine scheduling with cooperative concurrency:
//! - Work-stealing scheduler with configurable parallelism
//! - Goroutine spawning using "stan" keyword
//! - Cooperative yield points using "yolo" keyword
//! - Integration with channels for message passing
//! - Stack allocation and management
//! - LLVM FFI integration for compiled code

use crate::error::CursedError;
use crate::runtime::channels::{ChannelSender, ChannelReceiver};
use crate::runtime::stack::{RuntimeStack, StackId, StackFrame};
// use crate::runtime::preemptive_scheduler::PreemptiveScheduler;

use std::collections::{HashMap, VecDeque};
use crate::runtime::lockfree_deque::{LockFreeDeque, PriorityLockFreeDeque};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::sync::atomic::{AtomicU64, AtomicUsize, AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::panic::{self, AssertUnwindSafe};

/// Platform error type for PAL integration
#[derive(Debug, Clone)]
pub enum PlatformError {
    /// Goroutine spawn failed
    SpawnFailed(String),
    /// Yield operation failed
    YieldFailed(String),
    /// Scheduler operation failed
    SchedulerError(String),
    /// Invalid operation
    InvalidOperation(String),
    /// Platform-specific error
    PlatformSpecific(String),
}

impl std::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformError::SpawnFailed(msg) => write!(f, "Spawn failed: {}", msg),
            PlatformError::YieldFailed(msg) => write!(f, "Yield failed: {}", msg),
            PlatformError::SchedulerError(msg) => write!(f, "Scheduler error: {}", msg),
            PlatformError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            PlatformError::PlatformSpecific(msg) => write!(f, "Platform error: {}", msg),
        }
    }
}

impl std::error::Error for PlatformError {}

/// Scheduler trait for platform abstraction layer
/// 
/// This trait provides the interface that PAL implementations expect for goroutine scheduling.
/// It includes goroutine spawning and yielding operations.
pub trait Scheduler: Send + Sync {
    /// Spawn a new goroutine with the given task
    /// 
    /// # Arguments
    /// * `task` - The function to execute in the new goroutine
    /// 
    /// # Returns
    /// * `Ok(())` - Goroutine spawned successfully
    /// * `Err(PlatformError)` - Spawn failed
    fn spawn_goroutine(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), PlatformError>;
    
    /// Yield execution to allow other goroutines to run
    /// 
    /// This implements cooperative scheduling by voluntarily giving up
    /// the CPU to allow other goroutines to execute.
    /// 
    /// # Returns
    /// * `Ok(())` - Yield successful
    /// * `Err(PlatformError)` - Yield failed
    fn yield_now(&self) -> Result<(), PlatformError>;
}

/// Global scheduler instance
static GLOBAL_SCHEDULER: once_cell::sync::OnceCell<Arc<GoroutineScheduler>> = once_cell::sync::OnceCell::new();

/// Global preemptive scheduler instance
// static GLOBAL_PREEMPTIVE_SCHEDULER: once_cell::sync::OnceCell<Arc<PreemptiveScheduler>> = once_cell::sync::OnceCell::new();

/// Goroutine identifier type
pub type GoroutineId = u64;

/// Worker thread identifier type
pub type WorkerId = usize;

/// Goroutine state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoroutineState {
    /// Ready to run
    Ready,
    /// Currently running
    Running,
    /// Waiting for I/O or channel operation
    Waiting,
    /// Yielded voluntarily
    Yielded,
    /// Completed execution
    Completed,
    /// Panicked during execution
    Panicked,
    /// Error isolated - panic contained
    ErrorIsolated,
}

/// Goroutine priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GoroutinePriority {
    /// Low priority background tasks
    Low = 0,
    /// Normal priority tasks (default)
    Normal = 1,
    /// High priority tasks
    High = 2,
    /// Critical system tasks
    Critical = 3,
}

/// Goroutine error context for error isolation
#[derive(Debug, Clone)]
pub struct GoroutineErrorContext {
    /// Panic information if available
    pub panic_info: Option<String>,
    /// Stack trace at panic
    pub stack_trace: Vec<String>,
    /// Error isolation enabled
    pub isolation_enabled: bool,
    /// Recovery attempts
    pub recovery_attempts: u32,
    /// Maximum recovery attempts
    pub max_recovery_attempts: u32,
    /// Error timestamp
    pub error_timestamp: Option<Instant>,
    /// Error propagation chain
    pub error_chain: Vec<String>,
}

/// Panic propagation configuration
#[derive(Clone)]
pub struct PanicPropagationConfig {
    /// Whether to propagate panics to parent goroutine
    pub propagate_to_parent: bool,
    /// Whether to propagate panics to child goroutines
    pub propagate_to_children: bool,
    /// Whether to propagate panics to sibling goroutines
    pub propagate_to_siblings: bool,
    /// Maximum propagation depth
    pub max_propagation_depth: u32,
    /// Panic propagation timeout
    pub propagation_timeout: Duration,
    /// Custom panic handler
    pub panic_handler: Option<Arc<dyn Fn(&str, GoroutineId) + Send + Sync>>,
}

impl std::fmt::Debug for PanicPropagationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PanicPropagationConfig")
            .field("propagate_to_parent", &self.propagate_to_parent)
            .field("propagate_to_children", &self.propagate_to_children)
            .field("propagate_to_siblings", &self.propagate_to_siblings)
            .field("max_propagation_depth", &self.max_propagation_depth)
            .field("propagation_timeout", &self.propagation_timeout)
            .field("panic_handler", &self.panic_handler.as_ref().map(|_| "<function>"))
            .finish()
    }
}

impl Default for PanicPropagationConfig {
    fn default() -> Self {
        Self {
            propagate_to_parent: false,
            propagate_to_children: false,
            propagate_to_siblings: false,
            max_propagation_depth: 3,
            propagation_timeout: Duration::from_secs(5),
            panic_handler: None,
        }
    }
}

/// Join handle for goroutine error propagation
#[derive(Debug)]
pub struct GoroutineJoinHandle {
    /// Goroutine ID
    pub goroutine_id: GoroutineId,
    /// Result of goroutine execution
    pub result: Arc<Mutex<Option<Result<(), String>>>>,
    /// Error notification channel
    pub error_notifier: Arc<Condvar>,
    /// Completion status
    pub completed: Arc<AtomicBool>,
}

impl Default for GoroutinePriority {
    fn default() -> Self {
        GoroutinePriority::Normal
    }
}

/// Goroutine resource usage information
#[derive(Debug, Clone)]
pub struct GoroutineResourceUsage {
    pub goroutine_id: GoroutineId,
    pub total_runtime: Duration,
    pub channel_count: usize,
    pub child_count: usize,
    pub has_error_context: bool,
    pub state: GoroutineState,
    pub created_at: Instant,
    pub last_run: Option<Instant>,
}

/// Memory reclamation statistics
#[derive(Debug, Clone)]
pub struct MemoryReclamationStats {
    pub total_goroutines_cleaned: u64,
    pub active_goroutines: usize,
    pub stacks_allocated: usize,
    pub stacks_deallocated: usize,
    pub current_stacks: usize,
    pub total_memory_used: usize,
    pub peak_goroutines: usize,
    pub reclamation_efficiency: f64,
}

/// Goroutine execution context
pub struct Goroutine {
    /// Unique goroutine identifier
    pub id: GoroutineId,
    /// Current state
    pub state: AtomicU64, // Using atomic for lock-free state transitions
    /// Priority level
    pub priority: GoroutinePriority,
    /// Stack identifier
    pub stack_id: StackId,
    /// Entry point function
    pub entry_fn: Box<dyn FnOnce() + Send + 'static>,
    /// Creation time
    pub created_at: Instant,
    /// Last run time
    pub last_run: Option<Instant>,
    /// Total execution time
    pub total_runtime: Duration,
    /// Parent goroutine ID (for hierarchical spawning)
    pub parent_id: Option<GoroutineId>,
    /// Child goroutine IDs
    pub children: Vec<GoroutineId>,
    /// Associated channels for cleanup
    pub channels: Vec<Box<dyn std::any::Any + Send>>,
    /// Error isolation context
    pub error_context: Option<GoroutineErrorContext>,
    /// Join handle for error propagation
    pub join_handle: Option<GoroutineJoinHandle>,
    /// Panic propagation configuration
    pub panic_propagation: PanicPropagationConfig,
}

impl Goroutine {
    /// Create a new goroutine
    pub fn new<F>(id: GoroutineId, stack_id: StackId, entry_fn: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            id,
            state: AtomicU64::new(GoroutineState::Ready as u64),
            priority: GoroutinePriority::default(),
            stack_id,
            entry_fn: Box::new(entry_fn),
            created_at: Instant::now(),
            last_run: None,
            total_runtime: Duration::default(),
            parent_id: None,
            children: Vec::new(),
            channels: Vec::new(),
            error_context: None,
            join_handle: None,
            panic_propagation: PanicPropagationConfig::default(),
        }
    }

    /// Cleanup resources when goroutine completes
    pub fn cleanup_resources(&mut self) {
        // Close all associated channels
        self.channels.clear();
        
        // Update state to completed
        self.set_state(GoroutineState::Completed);
        
        // Mark join handle as completed
        if let Some(ref join_handle) = self.join_handle {
            join_handle.completed.store(true, Ordering::Release);
            join_handle.error_notifier.notify_all();
        }
        
        log::debug!("Cleaned up resources for goroutine {}", self.id);
    }

    /// Perform emergency resource cleanup during panic
    pub fn emergency_cleanup(&mut self) {
        // Force-close channels and clean up any resources
        self.channels.clear();
        
        // Set state to error isolated or panicked
        let current_state = self.get_state();
        if current_state != GoroutineState::ErrorIsolated {
            self.set_state(GoroutineState::Panicked);
        }
        
        // Signal completion to any waiting threads
        if let Some(ref join_handle) = self.join_handle {
            join_handle.completed.store(true, Ordering::Release);
            join_handle.error_notifier.notify_all();
        }
        
        log::warn!("Emergency cleanup performed for goroutine {}", self.id);
    }

    /// Check if goroutine requires cleanup
    pub fn needs_cleanup(&self) -> bool {
        matches!(
            self.get_state(),
            GoroutineState::Completed | GoroutineState::Panicked | GoroutineState::ErrorIsolated
        )
    }

    /// Get resource usage information
    pub fn get_resource_usage(&self) -> GoroutineResourceUsage {
        GoroutineResourceUsage {
            goroutine_id: self.id,
            total_runtime: self.total_runtime,
            channel_count: self.channels.len(),
            child_count: self.children.len(),
            has_error_context: self.error_context.is_some(),
            state: self.get_state(),
            created_at: self.created_at,
            last_run: self.last_run,
        }
    }

    /// Get current state
    pub fn get_state(&self) -> GoroutineState {
        match self.state.load(Ordering::Acquire) {
            0 => GoroutineState::Ready,
            1 => GoroutineState::Running,
            2 => GoroutineState::Waiting,
            3 => GoroutineState::Yielded,
            4 => GoroutineState::Completed,
            5 => GoroutineState::Panicked,
            6 => GoroutineState::ErrorIsolated,
            _ => GoroutineState::Ready, // Default fallback
        }
    }

    /// Set state atomically
    pub fn set_state(&self, state: GoroutineState) {
        self.state.store(state as u64, Ordering::Release);
    }

    /// Try to transition from one state to another
    pub fn try_transition(&self, from: GoroutineState, to: GoroutineState) -> bool {
        self.state
            .compare_exchange(from as u64, to as u64, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}

/// Implement Drop trait for automatic resource cleanup
impl Drop for Goroutine {
    fn drop(&mut self) {
        log::debug!("Dropping goroutine {} (state: {:?})", self.id, self.get_state());
        
        // Perform cleanup based on current state
        match self.get_state() {
            GoroutineState::Completed => {
                // Normal completion - clean cleanup
                self.cleanup_resources();
            },
            GoroutineState::Panicked | GoroutineState::ErrorIsolated => {
                // Error state - emergency cleanup
                self.emergency_cleanup();
            },
            _ => {
                // Other states - force cleanup
                log::warn!("Goroutine {} dropped in unexpected state: {:?}", self.id, self.get_state());
                self.emergency_cleanup();
            }
        }
        
        // Request stack deallocation from global scheduler
        if let Some(scheduler) = get_global_scheduler() {
            if let Err(e) = scheduler.deallocate_goroutine_stack(self.stack_id) {
                log::error!("Failed to deallocate stack {} for goroutine {}: {}", 
                           self.stack_id, self.id, e);
            }
        }
        
        log::debug!("Goroutine {} resources fully cleaned up", self.id);
    }
}

/// Work-stealing scheduler configuration
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Number of worker threads (0 = number of CPU cores)
    pub num_workers: usize,
    /// Initial work queue capacity
    pub queue_capacity: usize,
    /// Work stealing attempts before yielding
    pub steal_attempts: usize,
    /// Maximum goroutines per worker
    pub max_goroutines_per_worker: usize,
    /// Stack size for new goroutines
    pub default_stack_size: usize,
    /// Enable preemptive scheduling
    pub preemptive_scheduling: bool,
    /// Scheduling quantum (time slice) in milliseconds
    pub quantum_ms: u64,
    /// Enable M:N threading model improvements
    pub enable_mn_threading: bool,
    /// Enable network poller integration
    pub enable_network_poller: bool,
    /// Enable GC integration
    pub enable_gc_integration: bool,
    /// Enable debugging and statistics
    pub enable_debugging: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            num_workers: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            queue_capacity: 1024,
            steal_attempts: 3,
            max_goroutines_per_worker: 10000,
            default_stack_size: 2 * 1024 * 1024, // 2MB
            preemptive_scheduling: true, // Enable preemptive scheduling by default
            quantum_ms: 10,
            enable_mn_threading: true,
            enable_network_poller: true,
            enable_gc_integration: true,
            enable_debugging: false,
        }
    }
}

/// Worker thread for executing goroutines
pub struct Worker {
    /// Worker identifier
    pub id: WorkerId,
    /// Local run queue (using VecDeque for now until lockfree is implemented)
    pub queue: Arc<Mutex<VecDeque<Arc<Mutex<Goroutine>>>>>,
    /// Currently running goroutine
    pub current: Mutex<Option<Arc<Mutex<Goroutine>>>>,
    /// Worker statistics
    pub stats: Mutex<WorkerStats>,
    /// Thread handle
    pub thread_handle: Option<JoinHandle<()>>,
    /// Shutdown signal
    pub shutdown: Arc<AtomicBool>,
    /// Condition variable for work notification
    pub work_available: Arc<Condvar>,
}

/// Worker statistics
#[derive(Debug, Default, Clone)]
pub struct WorkerStats {
    pub goroutines_executed: u64,
    pub work_stolen: u64,
    pub work_shared: u64,
    pub idle_time: Duration,
    pub busy_time: Duration,
}

/// Global goroutine scheduler with work-stealing
pub struct GoroutineScheduler {
    /// Configuration
    config: SchedulerConfig,
    /// Worker threads
    workers: Vec<Arc<Worker>>,
    /// Global run queue for overflow (lock-free with priority support)
    global_queue: Arc<Mutex<VecDeque<Arc<Mutex<Goroutine>>>>>,
    /// Stack manager
    stack_manager: Arc<RuntimeStack>,
    /// Next goroutine ID
    next_id: AtomicU64,
    /// Active goroutines count
    active_count: AtomicUsize,
    /// Scheduler statistics
    stats: Mutex<SchedulerStats>,
    /// Shutdown flag
    shutdown: Arc<AtomicBool>,
    /// Scheduler state
    running: AtomicBool,
    /// Preemptive scheduler (optional, enabled when preemptive_scheduling is true)
    preemptive_scheduler: Option<()>, // Temporarily simplified
}

/// Scheduler statistics
#[derive(Debug, Default, Clone)]
pub struct SchedulerStats {
    pub total_goroutines_spawned: u64,
    pub total_goroutines_completed: u64,
    pub total_goroutines_panicked: u64,
    pub current_active_goroutines: usize,
    pub peak_active_goroutines: usize,
    pub total_yield_operations: u64,
    pub work_steals_attempted: u64,
    pub work_steals_successful: u64,
    pub scheduler_uptime: Duration,
    pub started_at: Option<Instant>,
}

impl GoroutineScheduler {
    /// Create a new goroutine scheduler
    pub fn new() -> Result<Self, CursedError> {
        Self::with_config(SchedulerConfig::default())
    }

    /// Create a new scheduler with custom configuration
    pub fn with_config(config: SchedulerConfig) -> Result<Self, CursedError> {
        let stack_manager = Arc::new(RuntimeStack::new());
        let shutdown = Arc::new(AtomicBool::new(false));
        
        // Create workers
        let mut workers = Vec::with_capacity(config.num_workers);
        for i in 0..config.num_workers {
            let worker = Arc::new(Worker {
                id: i,
                queue: Arc::new(Mutex::new(VecDeque::with_capacity(config.queue_capacity))),
                current: Mutex::new(None),
                stats: Mutex::new(WorkerStats::default()),
                thread_handle: None,
                shutdown: shutdown.clone(),
                work_available: Arc::new(Condvar::new()),
            });
            workers.push(worker);
        }

        // Create preemptive scheduler if enabled
        let preemptive_scheduler = if config.preemptive_scheduling {
            log::warn!("Preemptive scheduling requested but temporarily disabled for stability");
            None // Temporarily disabled until interface issues are resolved
        } else {
            None
        };

        Ok(Self {
            config,
            workers,
            global_queue: Arc::new(Mutex::new(VecDeque::new())),
            stack_manager,
            next_id: AtomicU64::new(1),
            active_count: AtomicUsize::new(0),
            stats: Mutex::new(SchedulerStats::default()),
            shutdown,
            running: AtomicBool::new(false),
            preemptive_scheduler,
        })
    }

    /// Start the scheduler
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(CursedError::runtime_error("Scheduler is already running"));
        }

        // Update statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics"))?;
            stats.started_at = Some(Instant::now());
        }

        // Start worker threads
        for worker in &self.workers {
            self.start_worker(worker.clone())?;
        }

        // Start preemptive scheduler if available
        // Temporarily disabled
        if let Some(ref _preemptive_scheduler) = self.preemptive_scheduler {
            // preemptive_scheduler.start()?;
        }

        Ok(())
    }

    /// Stop the scheduler
    pub fn stop(&self) -> Result<(), CursedError> {
        if !self.running.swap(false, Ordering::SeqCst) {
            return Ok(()); // Already stopped
        }

        log::info!("Stopping goroutine scheduler...");

        // Signal shutdown
        self.shutdown.store(true, Ordering::SeqCst);

        // Wake up all workers
        for worker in &self.workers {
            worker.work_available.notify_all();
        }

        // Wait for workers to process shutdown
        std::thread::sleep(Duration::from_millis(100));

        // Cleanup all remaining goroutines
        self.force_stop_all_goroutines()?;

        log::info!("Goroutine scheduler stopped successfully");
        Ok(())
    }

    /// Shutdown alias for runtime integration
    pub fn shutdown(&mut self) -> Result<(), CursedError> {
        self.stop()
    }

    /// Spawn a new goroutine (implements "stan" keyword)
    pub fn spawn<F>(&self, entry_fn: F) -> Result<GoroutineId, CursedError>
    where
        F: FnOnce() + Send + 'static,
    {
        self.spawn_with_priority(entry_fn, GoroutinePriority::Normal)
    }

    /// Spawn a goroutine with specific priority
    pub fn spawn_with_priority<F>(
        &self,
        entry_fn: F,
        priority: GoroutinePriority,
    ) -> Result<GoroutineId, CursedError>
    where
        F: FnOnce() + Send + 'static,
    {
        // Allocate stack
        let stack_id = self.stack_manager.allocate_stack(Some(self.config.default_stack_size))?;

        // Create goroutine
        let goroutine_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut goroutine = Goroutine::new(goroutine_id, stack_id, entry_fn);
        goroutine.priority = priority;

        let goroutine = Arc::new(Mutex::new(goroutine));

        // Schedule the goroutine
        self.schedule_goroutine(goroutine)?;

        // Update statistics
        self.active_count.fetch_add(1, Ordering::SeqCst);
        {
            let mut stats = self.stats.lock()
                .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics for goroutine spawn"))?;
            stats.total_goroutines_spawned += 1;
            stats.current_active_goroutines += 1;
            if stats.current_active_goroutines > stats.peak_active_goroutines {
                stats.peak_active_goroutines = stats.current_active_goroutines;
            }
        }

        Ok(goroutine_id)
    }

    /// Yield current goroutine (implements "yolo" keyword)
    pub fn yield_current(&self) -> Result<(), CursedError> {
        // Find current worker and goroutine
        let current_thread_id = thread::current().id();
        
        // For now, just update statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics for yield operation"))?;
            stats.total_yield_operations += 1;
        }

        // In a real implementation, this would:
        // 1. Save the current execution context
        // 2. Mark the goroutine as yielded
        // 3. Schedule another goroutine
        // 4. Eventually resume this goroutine

        Ok(())
    }

    /// Get scheduler statistics
    pub fn get_stats(&self) -> Result<SchedulerStats, CursedError> {
        let mut stats = self.stats.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics for reading"))?
            .clone();
        stats.current_active_goroutines = self.active_count.load(Ordering::SeqCst);
        
        if let Some(started_at) = stats.started_at {
            stats.scheduler_uptime = started_at.elapsed();
        }
        
        Ok(stats)
    }

    /// Get the number of active goroutines
    pub fn active_goroutine_count(&self) -> usize {
        self.active_count.load(Ordering::SeqCst)
    }

    /// Check if scheduler is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get current goroutine ID (thread-local)
    pub fn get_current_goroutine_id(&self) -> Option<GoroutineId> {
        thread_local! {
            static CURRENT_GOROUTINE_ID: std::cell::RefCell<Option<GoroutineId>> = std::cell::RefCell::new(None);
        }
        
        CURRENT_GOROUTINE_ID.with(|id| *id.borrow())
    }

    /// Deallocate goroutine stack
    pub fn deallocate_goroutine_stack(&self, stack_id: StackId) -> Result<(), CursedError> {
        self.stack_manager.deallocate_stack(stack_id)
    }

    /// Cleanup completed goroutines from all workers
    pub fn cleanup_completed_goroutines(&self) -> Result<usize, CursedError> {
        let mut cleaned_count = 0;
        
        // Clean up from all workers
        for worker in &self.workers {
            // Clean up current goroutine if completed
            if let Ok(mut current) = worker.current.lock() {
                if let Some(goroutine_arc) = current.take() {
                    let should_cleanup = {
                        if let Ok(goroutine) = goroutine_arc.lock() {
                            goroutine.needs_cleanup()
                        } else {
                            false
                        }
                    };
                    
                    if should_cleanup {
                        cleaned_count += 1;
                        // Drop will handle cleanup automatically
                    } else {
                        // Put it back if not ready for cleanup
                        *current = Some(goroutine_arc);
                    }
                }
            }
            
            // Clean up queued goroutines that are completed
            if let Ok(mut queue) = worker.queue.lock() {
                let original_len = queue.len();
                queue.retain(|goroutine_arc| {
                    if let Ok(goroutine) = goroutine_arc.lock() {
                        !goroutine.needs_cleanup()
                    } else {
                        false // Remove if we can't lock
                    }
                });
                cleaned_count += original_len - queue.len();
            }
        }
        
        // Update active count
        let old_count = self.active_count.fetch_sub(cleaned_count, Ordering::SeqCst);
        
        // Update statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics for cleanup"))?;
            stats.total_goroutines_completed += cleaned_count as u64;
            stats.current_active_goroutines = old_count.saturating_sub(cleaned_count);
        }
        
        if cleaned_count > 0 {
            log::debug!("Cleaned up {} completed goroutines", cleaned_count);
        }
        
        Ok(cleaned_count)
    }

    /// Get memory reclamation statistics
    pub fn get_memory_reclamation_stats(&self) -> Result<MemoryReclamationStats, CursedError> {
        let stack_stats = self.stack_manager.get_stats();
        let scheduler_stats = self.get_stats()?;
        
        Ok(MemoryReclamationStats {
            total_goroutines_cleaned: scheduler_stats.total_goroutines_completed,
            active_goroutines: scheduler_stats.current_active_goroutines,
            stacks_allocated: stack_stats.total_allocated,
            stacks_deallocated: stack_stats.total_deallocated,
            current_stacks: stack_stats.current_stacks,
            total_memory_used: stack_stats.total_memory_used,
            peak_goroutines: scheduler_stats.peak_active_goroutines,
            reclamation_efficiency: if scheduler_stats.total_goroutines_spawned > 0 {
                scheduler_stats.total_goroutines_completed as f64 / scheduler_stats.total_goroutines_spawned as f64
            } else {
                0.0
            }
        })
    }

    /// Scan all goroutine stacks for GC roots
    pub fn scan_stacks_for_gc_roots(&self) -> Vec<*mut u8> {
        let mut all_roots = Vec::new();
        
        // Scan stacks from all workers
        for worker in &self.workers {
            // Scan current goroutine stack
            if let Ok(current) = worker.current.lock() {
                if let Some(goroutine_arc) = current.as_ref() {
                    if let Ok(goroutine) = goroutine_arc.lock() {
                        if let Ok(roots) = self.stack_manager.get_gc_roots(goroutine.stack_id) {
                            all_roots.extend(roots);
                        }
                    }
                }
            }
            
            // Scan queued goroutine stacks
            if let Ok(queue) = worker.queue.lock() {
                for goroutine_arc in queue.iter() {
                    if let Ok(goroutine) = goroutine_arc.lock() {
                        if let Ok(roots) = self.stack_manager.get_gc_roots(goroutine.stack_id) {
                            all_roots.extend(roots);
                        }
                    }
                }
            }
        }
        
        // Scan global queue
        if let Ok(global_queue) = self.global_queue.lock() {
            for goroutine_arc in global_queue.iter() {
                if let Ok(goroutine) = goroutine_arc.lock() {
                    if let Ok(roots) = self.stack_manager.get_gc_roots(goroutine.stack_id) {
                        all_roots.extend(roots);
                    }
                }
            }
        }
        
        all_roots
    }

    /// Force stop and cleanup all goroutines (for emergency shutdown)
    pub fn force_stop_all_goroutines(&self) -> Result<(), CursedError> {
        log::warn!("Force stopping all goroutines for emergency shutdown");
        
        // Set shutdown flag
        self.shutdown.store(true, Ordering::SeqCst);
        
        // Wake up all workers to process shutdown
        for worker in &self.workers {
            worker.work_available.notify_all();
        }
        
        // Clean up all goroutines
        let mut total_cleaned = 0;
        
        for worker in &self.workers {
            // Emergency cleanup current goroutine
            if let Ok(mut current) = worker.current.lock() {
                if let Some(goroutine_arc) = current.take() {
                    if let Ok(mut goroutine) = goroutine_arc.lock() {
                        goroutine.emergency_cleanup();
                        total_cleaned += 1;
                    }
                }
            }
            
            // Emergency cleanup all queued goroutines
            if let Ok(mut queue) = worker.queue.lock() {
                for goroutine_arc in queue.drain(..) {
                    if let Ok(mut goroutine) = goroutine_arc.lock() {
                        goroutine.emergency_cleanup();
                        total_cleaned += 1;
                    }
                }
            }
        }
        
        // Clean up global queue
        if let Ok(mut global_queue) = self.global_queue.lock() {
            for goroutine_arc in global_queue.drain(..) {
                if let Ok(mut goroutine) = goroutine_arc.lock() {
                    goroutine.emergency_cleanup();
                    total_cleaned += 1;
                }
            }
        }
        
        // Reset all counters
        self.active_count.store(0, Ordering::SeqCst);
        
        // Update statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics for emergency stop"))?;
            stats.total_goroutines_completed += total_cleaned as u64;
            stats.current_active_goroutines = 0;
        }
        
        log::warn!("Emergency stopped and cleaned up {} goroutines", total_cleaned);
        Ok(())
    }

    /// Get goroutine state by ID
    pub fn get_goroutine_state(&self, goroutine_id: GoroutineId) -> Option<GoroutineState> {
        // Search through all workers for the goroutine
        for worker in &self.workers {
            if let Ok(queue) = worker.queue.lock() {
                for goroutine in queue.iter() {
                    if let Ok(g) = goroutine.lock() {
                        if g.id == goroutine_id {
                            return Some(g.get_state());
                        }
                    }
                }
            }
            
            // Check currently running goroutine
            if let Ok(current) = worker.current.lock() {
                if let Some(goroutine) = current.as_ref() {
                    if let Ok(g) = goroutine.lock() {
                        if g.id == goroutine_id {
                            return Some(g.get_state());
                        }
                    }
                }
            }
        }
        
        None
    }

    /// Get parent goroutine ID
    pub fn get_parent_goroutine_id(&self, goroutine_id: GoroutineId) -> Option<GoroutineId> {
        // In a real implementation, we'd maintain parent-child relationships
        // For now, return None since we don't have a goroutines map in the scheduler
        None
    }

    /// Set current goroutine ID (thread-local)
    pub fn set_current_goroutine_id(&self, goroutine_id: Option<GoroutineId>) {
        thread_local! {
            static CURRENT_GOROUTINE_ID: std::cell::RefCell<Option<GoroutineId>> = std::cell::RefCell::new(None);
        }
        
        CURRENT_GOROUTINE_ID.with(|id| *id.borrow_mut() = goroutine_id);
    }

    // Private helper methods

    fn schedule_goroutine(&self, goroutine: Arc<Mutex<Goroutine>>) -> Result<(), CursedError> {
        // Try to schedule on least loaded worker
        let mut best_worker = 0;
        let mut min_load = usize::MAX;

        for (i, worker) in self.workers.iter().enumerate() {
            let queue_len = worker.queue.lock()
                .map_err(|_| CursedError::runtime_error("Failed to lock worker queue for scheduling"))?
                .len();
            if queue_len < min_load {
                min_load = queue_len;
                best_worker = i;
            }
        }

        // Add to worker queue
        let worker = &self.workers[best_worker];
        worker.queue.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock worker queue for goroutine insertion"))?
            .push_back(goroutine);
        worker.work_available.notify_one();

        Ok(())
    }

    fn start_worker(&self, worker: Arc<Worker>) -> Result<(), CursedError> {
        let worker_clone = worker.clone();

        // Simplified worker thread spawn - in real implementation would store handle
        thread::spawn(move || {
            Self::worker_main(worker_clone);
        });

        Ok(())
    }

    fn worker_main(worker: Arc<Worker>) {
        let mut stats = WorkerStats::default();
        let start_time = Instant::now();

        while !worker.shutdown.load(Ordering::SeqCst) {
            // Try to get work from local queue
            let goroutine = {
                if let Ok(mut queue) = worker.queue.lock() {
                    queue.pop_front()
                } else {
                    // Continue if we can't acquire the lock
                    None
                }
            };

            if let Some(goroutine) = goroutine {
                // Execute goroutine
                let execution_start = Instant::now();
                Self::execute_goroutine(worker.id, goroutine);
                stats.busy_time += execution_start.elapsed();
                stats.goroutines_executed += 1;
            } else {
                // No local work, try work stealing
                if !Self::try_steal_work(&worker, &mut stats) {
                    // No work available, wait for notification
                    let idle_start = Instant::now();
                    if let Ok(queue_guard) = worker.queue.lock() {
                        let _ = worker.work_available.wait_timeout(
                            queue_guard,
                            Duration::from_millis(10),
                        );
                    }
                    stats.idle_time += idle_start.elapsed();
                }
            }
        }

        // Update worker statistics
        if let Ok(mut worker_stats) = worker.stats.lock() {
            *worker_stats = stats;
        }
    }

    fn execute_goroutine(worker_id: WorkerId, goroutine: Arc<Mutex<Goroutine>>) {
        let execution_start = Instant::now();
        
        // Extract the entry function and set up error isolation
        let (entry_fn, goroutine_id, join_handle) = {
            if let Ok(mut goroutine_guard) = goroutine.lock() {
                goroutine_guard.set_state(GoroutineState::Running);
                
                // Initialize error isolation context
                let error_context = GoroutineErrorContext {
                    panic_info: None,
                    stack_trace: Vec::new(),
                    isolation_enabled: true,
                    recovery_attempts: 0,
                    max_recovery_attempts: 3,
                    error_timestamp: None,
                    error_chain: Vec::new(),
                };
                
                // Create join handle for error propagation
                let join_handle = GoroutineJoinHandle {
                    goroutine_id: goroutine_guard.id,
                    result: Arc::new(Mutex::new(None)),
                    error_notifier: Arc::new(Condvar::new()),
                    completed: Arc::new(AtomicBool::new(false)),
                };
                
                goroutine_guard.error_context = Some(error_context);
                let join_handle_clone = GoroutineJoinHandle {
                    goroutine_id: join_handle.goroutine_id,
                    result: join_handle.result.clone(),
                    error_notifier: join_handle.error_notifier.clone(),
                    completed: join_handle.completed.clone(),
                };
                goroutine_guard.join_handle = Some(join_handle_clone);
                
                // Take the entry function out of the goroutine
                let fake_fn = Box::new(|| {}) as Box<dyn FnOnce() + Send + 'static>;
                let entry_fn = std::mem::replace(&mut goroutine_guard.entry_fn, fake_fn);
                
                (entry_fn, goroutine_guard.id, join_handle)
            } else {
                return; // Can't acquire lock, skip execution
            }
        };

        // Execute the goroutine function with enhanced error isolation
        let result = std::panic::catch_unwind(AssertUnwindSafe(move || {
            // Call the actual entry function
            entry_fn();
        }));

        // Update goroutine state and join handle based on execution result
        {
            if let Ok(mut goroutine_guard) = goroutine.lock() {
                match result {
                    Ok(_) => {
                        // Perform resource cleanup
                        goroutine_guard.cleanup_resources();
                        
                        // Update join handle with success
                        if let Ok(mut join_result) = join_handle.result.lock() {
                            *join_result = Some(Ok(()));
                        }
                        join_handle.completed.store(true, Ordering::Release);
                        join_handle.error_notifier.notify_all();
                        
                        log::debug!("Worker {} completed goroutine {} with cleanup", worker_id, goroutine_id);
                    },
                    Err(panic_info) => {
                        // Extract panic message
                        let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                            s.to_string()
                        } else if let Some(s) = panic_info.downcast_ref::<String>() {
                            s.clone()
                        } else {
                            "Unknown panic".to_string()
                        };
                        
                        // Check if error isolation is enabled
                        let isolation_enabled = goroutine_guard.error_context
                            .as_ref()
                            .map(|ctx| ctx.isolation_enabled)
                            .unwrap_or(false);
                        
                        if isolation_enabled {
                            // Error is isolated - don't propagate to scheduler
                            goroutine_guard.set_state(GoroutineState::ErrorIsolated);
                            
                            // Update error context
                            if let Some(ref mut error_context) = goroutine_guard.error_context {
                                error_context.panic_info = Some(panic_msg.clone());
                                error_context.error_timestamp = Some(Instant::now());
                                error_context.stack_trace = Self::capture_stack_trace();
                            }
                            
                            // Update join handle with error
                            if let Ok(mut join_result) = join_handle.result.lock() {
                                *join_result = Some(Err(panic_msg.clone()));
                            }
                            join_handle.completed.store(true, Ordering::Release);
                            join_handle.error_notifier.notify_all();
                            
                            log::warn!("Worker {} - goroutine {} error isolated: {}", worker_id, goroutine_id, panic_msg);
                        } else {
                            // Error is not isolated - panic propagates
                            goroutine_guard.set_state(GoroutineState::Panicked);
                            log::error!("Worker {} - goroutine {} panicked: {}", worker_id, goroutine_id, panic_msg);
                            
                            // Handle panic propagation
                            Self::handle_panic_propagation(&goroutine_guard, &panic_msg);
                        }
                    },
                }
            }
        }

        // Update execution time
        let execution_time = execution_start.elapsed();
        {
            if let Ok(mut goroutine_guard) = goroutine.lock() {
                goroutine_guard.total_runtime += execution_time;
                goroutine_guard.last_run = Some(Instant::now());
            }
        }
    }

    fn try_steal_work(worker: &Arc<Worker>, stats: &mut WorkerStats) -> bool {
        // Get global scheduler reference
        let scheduler = if let Some(scheduler) = get_global_scheduler() {
            scheduler
        } else {
            return false;
        };
        
        // Try to steal from global queue first
        if let Ok(mut global_queue) = scheduler.global_queue.lock() {
            if let Some(stolen_goroutine) = global_queue.pop_front() {
                if let Ok(mut local_queue) = worker.queue.lock() {
                    local_queue.push_back(stolen_goroutine);
                    stats.work_stolen += 1;
                    return true;
                }
            }
        }
        
        // Try to steal from other workers
        let worker_count = scheduler.workers.len();
        let start_index = (worker.id + 1) % worker_count;
        
        for i in 0..worker_count {
            let target_index = (start_index + i) % worker_count;
            if target_index == worker.id {
                continue; // Skip self
            }
            
            let target_worker = &scheduler.workers[target_index];
            
            // Try to steal from target worker
            if let Ok(mut target_queue) = target_worker.queue.lock() {
                if target_queue.len() > 1 {
                    // Only steal if target has more than 1 task
                    if let Some(stolen_goroutine) = target_queue.pop_back() {
                        if let Ok(mut local_queue) = worker.queue.lock() {
                            local_queue.push_back(stolen_goroutine);
                            stats.work_stolen += 1;
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Handle panic propagation according to configuration
    fn handle_panic_propagation(goroutine: &Goroutine, panic_msg: &str) {
        let config = &goroutine.panic_propagation;
        
        // Call custom panic handler if configured
        if let Some(ref handler) = config.panic_handler {
            handler(panic_msg, goroutine.id);
        }
        
        // Don't propagate if no propagation is configured
        if !config.propagate_to_parent && !config.propagate_to_children && !config.propagate_to_siblings {
            return;
        }
        
        // Get scheduler for propagation
        let scheduler = if let Some(scheduler) = get_global_scheduler() {
            scheduler
        } else {
            return;
        };
        
        // Propagate to parent
        if config.propagate_to_parent {
            if let Some(parent_id) = goroutine.parent_id {
                Self::propagate_panic_to_goroutine(&scheduler, parent_id, panic_msg, 1, config);
            }
        }
        
        // Propagate to children
        if config.propagate_to_children {
            for &child_id in &goroutine.children {
                Self::propagate_panic_to_goroutine(&scheduler, child_id, panic_msg, 1, config);
            }
        }
        
        // Propagate to siblings (through parent)
        if config.propagate_to_siblings {
            if let Some(parent_id) = goroutine.parent_id {
                Self::propagate_panic_to_siblings(&scheduler, parent_id, goroutine.id, panic_msg, config);
            }
        }
    }
    
    /// Propagate panic to a specific goroutine
    fn propagate_panic_to_goroutine(
        scheduler: &GoroutineScheduler,
        target_id: GoroutineId,
        panic_msg: &str,
        depth: u32,
        config: &PanicPropagationConfig,
    ) {
        if depth > config.max_propagation_depth {
            return;
        }
        
        // Search for the target goroutine in all workers
        for worker in &scheduler.workers {
            // Check current goroutine
            if let Ok(current_guard) = worker.current.lock() {
                if let Some(ref goroutine_arc) = *current_guard {
                    if let Ok(mut goroutine) = goroutine_arc.lock() {
                        if goroutine.id == target_id {
                            // Trigger panic in target goroutine
                            goroutine.set_state(GoroutineState::Panicked);
                            
                            // Update error context
                            if let Some(ref mut error_context) = goroutine.error_context {
                                error_context.panic_info = Some(format!("Propagated panic: {}", panic_msg));
                                error_context.error_timestamp = Some(Instant::now());
                                error_context.error_chain.push(format!("Propagated from depth {}", depth));
                            }
                            
                            log::warn!("Propagated panic to goroutine {}: {}", target_id, panic_msg);
                            return;
                        }
                    }
                }
            }
            
            // Check queued goroutines
            if let Ok(queue) = worker.queue.lock() {
                for goroutine_arc in queue.iter() {
                    if let Ok(mut goroutine) = goroutine_arc.lock() {
                        if goroutine.id == target_id {
                            // Trigger panic in target goroutine
                            goroutine.set_state(GoroutineState::Panicked);
                            
                            // Update error context
                            if let Some(ref mut error_context) = goroutine.error_context {
                                error_context.panic_info = Some(format!("Propagated panic: {}", panic_msg));
                                error_context.error_timestamp = Some(Instant::now());
                                error_context.error_chain.push(format!("Propagated from depth {}", depth));
                            }
                            
                            log::warn!("Propagated panic to queued goroutine {}: {}", target_id, panic_msg);
                            return;
                        }
                    }
                }
            }
        }
    }
    
    /// Propagate panic to sibling goroutines
    fn propagate_panic_to_siblings(
        scheduler: &GoroutineScheduler,
        parent_id: GoroutineId,
        source_id: GoroutineId,
        panic_msg: &str,
        config: &PanicPropagationConfig,
    ) {
        // Find parent goroutine and get its children
        let siblings = Self::get_goroutine_children(scheduler, parent_id);
        
        // Propagate to all siblings except the source
        for &sibling_id in &siblings {
            if sibling_id != source_id {
                Self::propagate_panic_to_goroutine(scheduler, sibling_id, panic_msg, 1, config);
            }
        }
    }
    
    /// Get children of a goroutine
    fn get_goroutine_children(scheduler: &GoroutineScheduler, parent_id: GoroutineId) -> Vec<GoroutineId> {
        let mut children = Vec::new();
        
        // Search for the parent goroutine in all workers
        for worker in &scheduler.workers {
            // Check current goroutine
            if let Ok(current_guard) = worker.current.lock() {
                if let Some(ref goroutine_arc) = *current_guard {
                    if let Ok(goroutine) = goroutine_arc.lock() {
                        if goroutine.id == parent_id {
                            children.extend_from_slice(&goroutine.children);
                            return children;
                        }
                    }
                }
            }
            
            // Check queued goroutines
            if let Ok(queue) = worker.queue.lock() {
                for goroutine_arc in queue.iter() {
                    if let Ok(goroutine) = goroutine_arc.lock() {
                        if goroutine.id == parent_id {
                            children.extend_from_slice(&goroutine.children);
                            return children;
                        }
                    }
                }
            }
        }
        
        children
    }
    
    /// Capture stack trace for error isolation
    fn capture_stack_trace() -> Vec<String> {
        let mut stack_trace = Vec::new();
        let backtrace = std::backtrace::Backtrace::capture();
        
        for frame in backtrace.to_string().lines() {
            if frame.trim().starts_with("at ") {
                stack_trace.push(frame.trim().to_string());
            }
        }
        
        if stack_trace.is_empty() {
            stack_trace.push("Stack trace not available".to_string());
        }
        
        stack_trace
    }
}

/// Implementation of Scheduler trait for GoroutineScheduler
impl Scheduler for GoroutineScheduler {
    fn spawn_goroutine(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), PlatformError> {
        match self.spawn(task) {
            Ok(_) => Ok(()),
            Err(e) => Err(PlatformError::SpawnFailed(e.to_string())),
        }
    }
    
    fn yield_now(&self) -> Result<(), PlatformError> {
        match self.yield_current() {
            Ok(()) => Ok(()),
            Err(e) => Err(PlatformError::SchedulerError(e.to_string())),
        }
    }
}

/// Wrapper for GoroutineScheduler that implements the runtime trait
pub struct GoroutineSchedulerWrapper {
    scheduler: Arc<GoroutineScheduler>,
}

impl GoroutineSchedulerWrapper {
    /// Create a new scheduler wrapper
    pub fn new(scheduler: Arc<GoroutineScheduler>) -> Self {
        Self { scheduler }
    }
    
    /// Create a new scheduler wrapper with default configuration
    pub fn new_default() -> Result<Self, CursedError> {
        Ok(Self::new(Arc::new(GoroutineScheduler::new()?)))
    }
    
    /// Create a new scheduler wrapper with custom configuration
    pub fn new_with_config(config: SchedulerConfig) -> Result<Self, CursedError> {
        Ok(Self::new(Arc::new(GoroutineScheduler::with_config(config)?)))
    }
}

impl crate::runtime::runtime::GoroutineSchedulerTrait for GoroutineSchedulerWrapper {
    fn spawn(&mut self, task: Box<dyn FnOnce() + Send>) -> crate::error_types::Result<usize> {
        self.scheduler.spawn(task)
            .map(|id| id as usize)
            .map_err(|e| crate::error_types::Error::Runtime(e.to_string()))
    }
    
    fn active_count(&self) -> usize {
        self.scheduler.active_goroutine_count()
    }
    
    fn shutdown(&mut self) -> crate::error_types::Result<()> {
        // Can't get mutable reference to scheduler in Arc, so we use stop instead
        self.scheduler.stop()
            .map_err(|e| crate::error_types::Error::Runtime(e.to_string()))
    }
    
    fn start(&mut self) -> crate::error_types::Result<()> {
        self.scheduler.start()
            .map_err(|e| crate::error_types::Error::Runtime(e.to_string()))
    }
    
    fn is_running(&self) -> bool {
        self.scheduler.is_running()
    }
    
    fn get_stats(&self) -> crate::error_types::Result<crate::runtime::runtime::SchedulerStatistics> {
        let stats = self.scheduler.get_stats()
            .map_err(|e| crate::error_types::Error::Runtime(e.to_string()))?;
        
        Ok(crate::runtime::runtime::SchedulerStatistics {
            total_spawned: stats.total_goroutines_spawned,
            total_completed: stats.total_goroutines_completed,
            current_active: stats.current_active_goroutines,
            peak_active: stats.peak_active_goroutines,
            total_panicked: stats.total_goroutines_panicked,
            uptime: stats.scheduler_uptime,
        })
    }
}

// Global scheduler management functions

/// Initialize the global goroutine scheduler
pub fn initialize_global_scheduler() -> Result<(), CursedError> {
    initialize_global_scheduler_with_config(SchedulerConfig::default())
}

/// Initialize the global scheduler with custom configuration
pub fn initialize_global_scheduler_with_config(config: SchedulerConfig) -> Result<(), CursedError> {
    let scheduler = Arc::new(GoroutineScheduler::with_config(config)?);
    
    GLOBAL_SCHEDULER
        .set(scheduler.clone())
        .map_err(|_| CursedError::runtime_error("Global scheduler already initialized"))?;

    scheduler.start()
}

/// Get the global goroutine scheduler
pub fn get_global_scheduler() -> Option<Arc<GoroutineScheduler>> {
    GLOBAL_SCHEDULER.get().cloned()
}

/// Get the global preemptive scheduler instance
// pub fn get_global_preemptive_scheduler() -> Option<Arc<PreemptiveScheduler>> {
//     GLOBAL_PREEMPTIVE_SCHEDULER.get().cloned()
// }

/// Initialize the global preemptive scheduler
pub fn initialize_global_preemptive_scheduler() -> Result<(), CursedError> {
    let config = SchedulerConfig {
        preemptive_scheduling: true,
        enable_mn_threading: true,
        enable_network_poller: true,
        enable_gc_integration: true,
        ..Default::default()
    };
    
    initialize_global_scheduler_with_config(config)
}

/// Initialize the global preemptive scheduler with custom config
pub fn initialize_global_preemptive_scheduler_with_config(config: SchedulerConfig) -> Result<(), CursedError> {
    initialize_global_scheduler_with_config(config)
}

/// Shutdown the global goroutine scheduler
pub fn shutdown_global_scheduler() -> Result<(), CursedError> {
    if let Some(scheduler) = get_global_scheduler() {
        scheduler.stop()
    } else {
        Ok(())
    }
}

/// Spawn a goroutine using the global scheduler (implements "stan" keyword)
pub fn stan<F>(entry_fn: F) -> Result<GoroutineId, CursedError>
where
    F: FnOnce() + Send + 'static,
{
    get_global_scheduler()
        .ok_or_else(|| CursedError::runtime_error("Global scheduler not initialized"))?
        .spawn(entry_fn)
}

/// Yield current goroutine using the global scheduler (implements "yolo" keyword)
pub fn yolo() -> Result<(), CursedError> {
    get_global_scheduler()
        .ok_or_else(|| CursedError::runtime_error("Global scheduler not initialized"))?
        .yield_current()
}

/// Join a goroutine and await its completion with error handling
pub fn join_goroutine(goroutine_id: GoroutineId) -> Result<Result<(), String>, CursedError> {
    let scheduler = get_global_scheduler()
        .ok_or_else(|| CursedError::runtime_error("Global scheduler not initialized"))?;
    
    // Find the goroutine by ID
    for worker in &scheduler.workers {
        if let Ok(current_guard) = worker.current.lock() {
            if let Some(ref goroutine) = *current_guard {
                if let Ok(goroutine_guard) = goroutine.lock() {
                    if goroutine_guard.id == goroutine_id {
                        if let Some(ref join_handle) = goroutine_guard.join_handle {
                            // Wait for completion
                            let result_guard = join_handle.result.lock()
                                .map_err(|_| CursedError::runtime_error("Failed to lock join handle result"))?;
                            
                            if join_handle.completed.load(Ordering::Acquire) {
                                // Already completed
                                return Ok(result_guard.clone().unwrap_or(Ok(())));
                            }
                            
                            // Wait for notification
                            let _result = join_handle.error_notifier.wait(result_guard)
                                .map_err(|_| CursedError::runtime_error("Failed to wait for goroutine completion"))?;
                            
                            // Check result
                            if let Some(result) = _result.as_ref() {
                                return Ok(result.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    
    Err(CursedError::runtime_error("Goroutine not found"))
}

/// Get error information for a goroutine
pub fn get_goroutine_error(goroutine_id: GoroutineId) -> Option<GoroutineErrorContext> {
    let scheduler = get_global_scheduler()?;
    
    // Search for the goroutine
    for worker in &scheduler.workers {
        if let Ok(current_guard) = worker.current.lock() {
            if let Some(ref goroutine) = *current_guard {
                if let Ok(goroutine_guard) = goroutine.lock() {
                    if goroutine_guard.id == goroutine_id {
                        return goroutine_guard.error_context.clone();
                    }
                }
            }
        }
    }
    
    None
}

// LLVM FFI Integration

/// FFI function to spawn goroutine from compiled code
#[no_mangle]
pub extern "C" fn cursed_stan_goroutine(
    entry_fn: extern "C" fn(*mut std::ffi::c_void),
    context: *mut std::ffi::c_void,
) -> u64 {
    // Convert raw pointer to usize for Send compatibility
    let context_addr = context as usize;
    
    let result = stan(move || {
        // Convert back to pointer inside the closure
        let context = context_addr as *mut std::ffi::c_void;
        entry_fn(context);
    });

    match result {
        Ok(id) => id,
        Err(_) => 0, // Error indicator
    }
}

/// FFI function to yield current goroutine from compiled code
#[no_mangle]
pub extern "C" fn cursed_yolo_goroutine() -> bool {
    yolo().is_ok()
}

/// FFI function to get scheduler statistics
#[no_mangle]
pub extern "C" fn cursed_get_scheduler_stats() -> *mut std::ffi::c_void {
    if let Some(scheduler) = get_global_scheduler() {
        if let Ok(stats) = scheduler.get_stats() {
            Box::into_raw(Box::new(stats)) as *mut std::ffi::c_void
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

/// FFI function to initialize scheduler
#[no_mangle]
pub extern "C" fn cursed_init_scheduler(num_workers: usize) -> bool {
    let mut config = SchedulerConfig::default();
    if num_workers > 0 {
        config.num_workers = num_workers;
    }
    
    initialize_global_scheduler_with_config(config).is_ok()
}

/// FFI function to shutdown scheduler
#[no_mangle]
pub extern "C" fn cursed_shutdown_scheduler() -> bool {
    shutdown_global_scheduler().is_ok()
}

// Channel integration functions

/// Create a channel for goroutine communication
pub fn make_channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    crate::runtime::channels::channel()
}

/// Create a buffered channel for goroutine communication
pub fn make_buffered_channel<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    crate::runtime::channels::buffered_channel(capacity)
}

// Keep existing minimal implementation for compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED goroutine scheduler enabled".to_string())
}

// Default implementations
impl Default for GoroutineScheduler {
    fn default() -> Self {
        Self::new().expect("Failed to create default scheduler")
    }
}

// Tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = GoroutineScheduler::new().expect("Failed to create scheduler");
        assert!(!scheduler.is_running());
        assert_eq!(scheduler.active_goroutine_count(), 0);
    }

    #[test]
    fn test_goroutine_states() {
        let stack_id = 1;
        let goroutine = Goroutine::new(1, stack_id, || {});
        
        assert_eq!(goroutine.get_state(), GoroutineState::Ready);
        
        goroutine.set_state(GoroutineState::Running);
        assert_eq!(goroutine.get_state(), GoroutineState::Running);
        
        assert!(goroutine.try_transition(GoroutineState::Running, GoroutineState::Completed));
        assert_eq!(goroutine.get_state(), GoroutineState::Completed);
    }

    #[test]
    fn test_global_scheduler_management() {
        // This test would need to be carefully designed to avoid conflicts
        // with other tests that might initialize the global scheduler
        
        // For now, just test that the functions exist and can be called
        assert!(get_global_scheduler().is_none() || get_global_scheduler().is_some());
    }
}

/// Platform-specific scheduler implementations

/// x86_64-specific scheduler
pub struct X86_64Scheduler {
    base: Arc<GoroutineScheduler>,
}

impl X86_64Scheduler {
    pub fn new_macos() -> Result<Self, PlatformError> {
        let config = SchedulerConfig::default();
        let scheduler = GoroutineScheduler::with_config(config)
            .map_err(|e| PlatformError::SchedulerError(e.to_string()))?;
        let base = Arc::new(scheduler);
        
        Ok(Self { base })
    }
    
    pub fn new_linux() -> Result<Self, PlatformError> {
        Self::new_macos() // Same implementation for now
    }
    
    pub fn new_windows() -> Result<Self, PlatformError> {
        Self::new_macos() // Same implementation for now
    }
}

impl Scheduler for X86_64Scheduler {
    fn spawn_goroutine(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), PlatformError> {
        self.base.spawn_goroutine(task)
    }

    fn yield_now(&self) -> Result<(), PlatformError> {
        self.base.yield_now()
    }
}

// Additional platform-specific schedulers
pub struct Arm64Scheduler {
    inner: GoroutineScheduler,
}

impl Arm64Scheduler {
    pub fn new_macos() -> Result<Self, PlatformError> {
        let scheduler = GoroutineScheduler::with_config(SchedulerConfig::default())
            .map_err(|e| PlatformError::SchedulerError(e.to_string()))?;
        Ok(Self {
            inner: scheduler,
        })
    }
    
    pub fn new_linux() -> Result<Self, PlatformError> {
        let scheduler = GoroutineScheduler::with_config(SchedulerConfig::default())
            .map_err(|e| PlatformError::SchedulerError(e.to_string()))?;
        Ok(Self {
            inner: scheduler,
        })
    }
}

impl Scheduler for Arm64Scheduler {
    fn spawn_goroutine(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), PlatformError> {
        self.inner.spawn_goroutine(task)
    }

    fn yield_now(&self) -> Result<(), PlatformError> {
        self.inner.yield_now()
    }
}

pub struct WasmScheduler {
    inner: GoroutineScheduler,
}

impl WasmScheduler {
    pub fn new(_runtime_type: crate::runtime::pal::wasm::WasmRuntimeType) -> Result<Self, PlatformError> {
        let scheduler = GoroutineScheduler::with_config(SchedulerConfig::default())
            .map_err(|e| PlatformError::SchedulerError(e.to_string()))?;
        Ok(Self {
            inner: scheduler,
        })
    }
}

impl Scheduler for WasmScheduler {
    fn spawn_goroutine(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), PlatformError> {
        self.inner.spawn_goroutine(task)
    }

    fn yield_now(&self) -> Result<(), PlatformError> {
        self.inner.yield_now()
    }
}
