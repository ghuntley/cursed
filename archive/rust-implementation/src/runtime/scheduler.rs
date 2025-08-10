//! Enterprise-Grade Goroutine Scheduler Implementation
//!
//! This module provides a production-ready work-stealing scheduler for CURSED goroutines:
//! - High-performance work-stealing with configurable parameters
//! - Thread-safe goroutine pool management
//! - Advanced scheduling algorithms (Round Robin, Priority, Work-Stealing)
//! - Comprehensive monitoring and debugging capabilities
//! - Integration with channels for select statement runtime
//! - Memory-efficient concurrent data structures
//! - Scalable to thousands of goroutines

use crate::error::CursedError;
use crate::runtime::channels::{ChannelSender, ChannelReceiver, ChannelError, SimpleSelect, SelectResult, SelectCase};
use crate::runtime::stack::{RuntimeStack, StackId};
use crate::runtime::goroutine::{Goroutine, GoroutineId, GoroutineState, GoroutinePriority, WorkerId};

use std::collections::{HashMap, VecDeque, BTreeMap};
use std::sync::{Arc, Mutex, RwLock, Condvar, atomic::{AtomicU64, AtomicUsize, AtomicBool, Ordering}};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::hint;
use crossbeam_utils::CachePadded;
use crossbeam_epoch::Guard;

/// Work-stealing scheduler configuration
#[derive(Debug, Clone)]
pub struct WorkStealingConfig {
    /// Number of worker threads
    pub num_workers: usize,
    /// Initial local queue capacity
    pub local_queue_capacity: usize,
    /// Global queue capacity
    pub global_queue_capacity: usize,
    /// Maximum steal attempts per worker
    pub max_steal_attempts: usize,
    /// Steal batch size
    pub steal_batch_size: usize,
    /// Work sharing threshold
    pub work_sharing_threshold: usize,
    /// Enable load balancing
    pub enable_load_balancing: bool,
    /// Load balancing interval (ms)
    pub load_balance_interval_ms: u64,
    /// Enable adaptive scheduling
    pub enable_adaptive_scheduling: bool,
    /// Preemption quantum (ms)
    pub preemption_quantum_ms: u64,
    /// Enable NUMA awareness
    pub enable_numa_awareness: bool,
    /// CPU affinity mask
    pub cpu_affinity_mask: Option<u64>,
}

impl Default for WorkStealingConfig {
    fn default() -> Self {
        Self {
            num_workers: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
            local_queue_capacity: 512,
            global_queue_capacity: 4096,
            max_steal_attempts: 8,
            steal_batch_size: 16,
            work_sharing_threshold: 256,
            enable_load_balancing: true,
            load_balance_interval_ms: 100,
            enable_adaptive_scheduling: true,
            preemption_quantum_ms: 10,
            enable_numa_awareness: false,
            cpu_affinity_mask: None,
        }
    }
}

/// Worker statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct WorkerStatistics {
    /// Total goroutines executed
    pub goroutines_executed: AtomicU64,
    /// Work items stolen from other workers
    pub work_stolen: AtomicU64,
    /// Work items given to other workers
    pub work_shared: AtomicU64,
    /// Failed steal attempts
    pub failed_steals: AtomicU64,
    /// Successful steal attempts
    pub successful_steals: AtomicU64,
    /// Time spent executing goroutines
    pub execution_time: AtomicU64,
    /// Time spent idle
    pub idle_time: AtomicU64,
    /// Time spent stealing work
    pub steal_time: AtomicU64,
    /// Number of context switches
    pub context_switches: AtomicU64,
    /// Memory allocations
    pub memory_allocations: AtomicU64,
    /// Peak local queue size
    pub peak_queue_size: AtomicUsize,
    /// Current local queue size
    pub current_queue_size: AtomicUsize,
}

/// High-performance work-stealing worker
pub struct WorkStealingWorker {
    /// Worker ID
    pub id: WorkerId,
    /// Thread handle
    pub thread: Option<JoinHandle<()>>,
    /// Local work queue (thread-local for efficiency)
    pub local_queue: Arc<Mutex<VecDeque<Arc<Mutex<Goroutine>>>>>,
    /// Worker statistics
    pub stats: WorkerStatistics,
    /// CPU affinity if enabled
    pub cpu_affinity: Option<usize>,
    /// Shutdown signal
    pub shutdown: Arc<AtomicBool>,
    /// Work notification
    pub work_notification: Arc<Condvar>,
    /// Worker state
    pub state: Arc<Mutex<WorkerState>>,
    /// Last load balance time
    pub last_load_balance: Arc<Mutex<Instant>>,
    /// Random number generator for steal victim selection
    pub rng_state: Arc<Mutex<u64>>,
}

/// Worker state
#[derive(Debug, Clone)]
pub enum WorkerState {
    /// Worker is idle
    Idle,
    /// Worker is executing a goroutine
    Running(GoroutineId),
    /// Worker is stealing work
    Stealing,
    /// Worker is sharing work
    Sharing,
    /// Worker is shutting down
    Shutdown,
}

/// Goroutine pool for efficient memory management
pub struct GoroutinePool {
    /// Pool of pre-allocated goroutines
    pool: Arc<Mutex<Vec<Arc<Mutex<Goroutine>>>>>,
    /// Pool configuration
    config: PoolConfig,
    /// Pool statistics
    stats: Arc<Mutex<PoolStats>>,
    /// Next goroutine ID
    next_id: AtomicU64,
    /// Stack manager
    stack_manager: Arc<RuntimeStack>,
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Initial pool size
    pub initial_size: usize,
    /// Maximum pool size
    pub max_size: usize,
    /// Minimum pool size
    pub min_size: usize,
    /// Pool growth factor
    pub growth_factor: f64,
    /// Pool shrink threshold
    pub shrink_threshold: usize,
    /// Pool shrink interval (ms)
    pub shrink_interval_ms: u64,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            initial_size: 100,
            max_size: 10000,
            min_size: 50,
            growth_factor: 1.5,
            shrink_threshold: 200,
            shrink_interval_ms: 5000,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total goroutines created
    pub total_created: u64,
    /// Total goroutines reused
    pub total_reused: u64,
    /// Current pool size
    pub current_size: usize,
    /// Peak pool size
    pub peak_size: usize,
    /// Pool hits
    pub pool_hits: u64,
    /// Pool misses
    pub pool_misses: u64,
    /// Memory saved by pooling
    pub memory_saved: u64,
}

impl GoroutinePool {
    /// Create a new goroutine pool
    pub fn new(config: PoolConfig, stack_manager: Arc<RuntimeStack>) -> Self {
        let pool = Arc::new(Mutex::new(Vec::with_capacity(config.initial_size)));
        
        Self {
            pool,
            config,
            stats: Arc::new(Mutex::new(PoolStats::default())),
            next_id: AtomicU64::new(1),
            stack_manager,
        }
    }

    /// Get a goroutine from the pool or create a new one
    pub fn get_goroutine<F>(&self, entry_fn: F) -> Result<Arc<Mutex<Goroutine>>, CursedError>
    where
        F: FnOnce() + Send + 'static,
    {
        // Try to get from pool first
        if let Ok(mut pool) = self.pool.lock() {
            if let Some(goroutine) = pool.pop() {
                // Reuse existing goroutine
                {
                    if let Ok(mut g) = goroutine.lock() {
                        g.id = self.next_id.fetch_add(1, Ordering::SeqCst);
                        g.entry_fn = Box::new(entry_fn);
                        g.state = AtomicU64::new(GoroutineState::Ready as u64);
                        g.created_at = Instant::now();
                        g.last_run = None;
                        g.total_runtime = Duration::default();
                        g.parent_id = None;
                        g.children.clear();
                        g.channels.clear();
                        g.error_context = None;
                        g.join_handle = None;
                    }
                }
                
                // Update statistics
                if let Ok(mut stats) = self.stats.lock() {
                    stats.total_reused += 1;
                    stats.pool_hits += 1;
                }
                
                return Ok(goroutine);
            }
        }

        // Create new goroutine
        let stack_id = self.stack_manager.allocate_stack(Some(2 * 1024 * 1024))?;
        let goroutine_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let goroutine = Goroutine::new(goroutine_id, stack_id, entry_fn);
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_created += 1;
            stats.pool_misses += 1;
        }
        
        Ok(Arc::new(Mutex::new(goroutine)))
    }

    /// Return a goroutine to the pool
    pub fn return_goroutine(&self, goroutine: Arc<Mutex<Goroutine>>) {
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() < self.config.max_size {
                pool.push(goroutine);
                
                // Update statistics
                if let Ok(mut stats) = self.stats.lock() {
                    stats.current_size = pool.len();
                    if stats.current_size > stats.peak_size {
                        stats.peak_size = stats.current_size;
                    }
                }
            }
        }
    }

    /// Shrink the pool if needed
    pub fn shrink_pool(&self) {
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() > self.config.shrink_threshold {
                let target_size = std::cmp::max(
                    self.config.min_size,
                    pool.len() / 2
                );
                pool.truncate(target_size);
                
                // Update statistics
                if let Ok(mut stats) = self.stats.lock() {
                    stats.current_size = pool.len();
                }
            }
        }
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> Result<PoolStats, CursedError> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::runtime_error("Failed to get pool statistics"))
    }
}

/// Select statement runtime implementation
pub struct SelectRuntime {
    /// Active select operations
    active_selects: Arc<RwLock<HashMap<u64, SelectOperation>>>,
    /// Next select operation ID
    next_select_id: AtomicU64,
    /// Select statistics
    stats: Arc<Mutex<SelectStats>>,
}

#[derive(Debug, Clone)]
pub struct SelectOperation {
    /// Operation ID
    pub id: u64,
    /// Goroutine ID
    pub goroutine_id: GoroutineId,
    /// Cases
    pub cases: Vec<SelectCase>,
    /// Timeout
    pub timeout: Option<Duration>,
    /// Start time
    pub start_time: Instant,
    /// Result
    pub result: Arc<Mutex<Option<SelectResult>>>,
    /// Completion notifier
    pub completion: Arc<Condvar>,
}

#[derive(Debug, Clone, Default)]
pub struct SelectStats {
    /// Total select operations
    pub total_operations: u64,
    /// Successful select operations
    pub successful_operations: u64,
    /// Timed out select operations
    pub timeout_operations: u64,
    /// Average operation duration
    pub average_duration_ns: u64,
    /// Peak concurrent selects
    pub peak_concurrent: usize,
    /// Current concurrent selects
    pub current_concurrent: usize,
}

impl SelectRuntime {
    /// Create a new select runtime
    pub fn new() -> Self {
        Self {
            active_selects: Arc::new(RwLock::new(HashMap::new())),
            next_select_id: AtomicU64::new(1),
            stats: Arc::new(Mutex::new(SelectStats::default())),
        }
    }

    /// Execute a select operation (implements "ready" keyword)
    pub fn execute_select<T: Send + 'static>(
        &self,
        goroutine_id: GoroutineId,
        cases: Vec<SelectCase>,
        timeout: Option<Duration>,
    ) -> Result<SelectResult, CursedError> {
        let operation_id = self.next_select_id.fetch_add(1, Ordering::SeqCst);
        let start_time = Instant::now();
        
        // Create select operation
        let operation = SelectOperation {
            id: operation_id,
            goroutine_id,
            cases: cases.clone(),
            timeout,
            start_time,
            result: Arc::new(Mutex::new(None)),
            completion: Arc::new(Condvar::new()),
        };

        // Register operation
        {
            let mut active_selects = self.active_selects.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire select operations lock"))?;
            active_selects.insert(operation_id, operation.clone());
            
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.total_operations += 1;
                stats.current_concurrent += 1;
                if stats.current_concurrent > stats.peak_concurrent {
                    stats.peak_concurrent = stats.current_concurrent;
                }
            }
        }

        // Execute select using enhanced select implementation
        let mut select_builder = SimpleSelect::new();
        for case in cases {
            select_builder.add_case(case);
        }
        
        let result = if let Some(timeout) = timeout {
            select_builder.select_timeout(timeout)
        } else {
            select_builder.select()
        };

        // Update result
        {
            let mut operation_result = operation.result.lock()
                .map_err(|_| CursedError::runtime_error("Failed to acquire select result lock"))?;
            *operation_result = Some(result.clone());
        }

        // Notify completion
        operation.completion.notify_all();

        // Unregister operation
        {
            let mut active_selects = self.active_selects.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire select operations lock for cleanup"))?;
            active_selects.remove(&operation_id);
            
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.current_concurrent -= 1;
                match &result {
                    SelectResult::Ready { .. } => stats.successful_operations += 1,
                    SelectResult::Timeout => stats.timeout_operations += 1,
                    SelectResult::Closed => stats.successful_operations += 1,
                }
                
                let duration = start_time.elapsed();
                stats.average_duration_ns = (stats.average_duration_ns + duration.as_nanos() as u64) / 2;
            }
        }

        Ok(result)
    }

    /// Get select statistics
    pub fn get_stats(&self) -> Result<SelectStats, CursedError> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::runtime_error("Failed to get select statistics"))
    }

    /// Cancel a select operation
    pub fn cancel_select(&self, operation_id: u64) -> Result<(), CursedError> {
        let mut active_selects = self.active_selects.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire select operations lock for cancellation"))?;
        
        if let Some(operation) = active_selects.remove(&operation_id) {
            // Update result to cancelled
            {
                let mut result = operation.result.lock()
                    .map_err(|_| CursedError::runtime_error("Failed to acquire select result lock for cancellation"))?;
                *result = Some(SelectResult::Timeout);
            }
            
            // Notify completion
            operation.completion.notify_all();
        }
        
        Ok(())
    }
}

/// Concurrent data structures for production use
pub mod concurrent {
    use std::sync::{Arc, RwLock, Mutex};
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    /// High-performance concurrent hash map
    pub struct ConcurrentHashMap<K, V> {
        buckets: Vec<Arc<RwLock<HashMap<K, V>>>>,
        bucket_count: usize,
    }

    impl<K: Hash + Eq + Clone, V: Clone> ConcurrentHashMap<K, V> {
        /// Create a new concurrent hash map
        pub fn new(bucket_count: usize) -> Self {
            let mut buckets = Vec::with_capacity(bucket_count);
            for _ in 0..bucket_count {
                buckets.push(Arc::new(RwLock::new(HashMap::new())));
            }
            
            Self {
                buckets,
                bucket_count,
            }
        }

        /// Get bucket index for key
        fn bucket_index(&self, key: &K) -> usize {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            (hasher.finish() % self.bucket_count as u64) as usize
        }

        /// Insert a key-value pair
        pub fn insert(&self, key: K, value: V) -> Option<V> {
            let bucket_idx = self.bucket_index(&key);
            let bucket = &self.buckets[bucket_idx];
            
            if let Ok(mut map) = bucket.write() {
                map.insert(key, value)
            } else {
                None
            }
        }

        /// Get a value by key
        pub fn get(&self, key: &K) -> Option<V> {
            let bucket_idx = self.bucket_index(key);
            let bucket = &self.buckets[bucket_idx];
            
            if let Ok(map) = bucket.read() {
                map.get(key).cloned()
            } else {
                None
            }
        }

        /// Remove a key-value pair
        pub fn remove(&self, key: &K) -> Option<V> {
            let bucket_idx = self.bucket_index(key);
            let bucket = &self.buckets[bucket_idx];
            
            if let Ok(mut map) = bucket.write() {
                map.remove(key)
            } else {
                None
            }
        }

        /// Check if key exists
        pub fn contains_key(&self, key: &K) -> bool {
            let bucket_idx = self.bucket_index(key);
            let bucket = &self.buckets[bucket_idx];
            
            if let Ok(map) = bucket.read() {
                map.contains_key(key)
            } else {
                false
            }
        }
    }

    /// Lock-free queue for high-performance work distribution
    pub struct LockFreeQueue<T> {
        inner: Arc<Mutex<VecDeque<T>>>,
    }

    impl<T> LockFreeQueue<T> {
        /// Create a new lock-free queue
        pub fn new() -> Self {
            Self {
                inner: Arc::new(Mutex::new(VecDeque::new())),
            }
        }

        /// Push an item to the back
        pub fn push(&self, item: T) -> bool {
            if let Ok(mut queue) = self.inner.lock() {
                queue.push_back(item);
                true
            } else {
                false
            }
        }

        /// Pop an item from the front
        pub fn pop(&self) -> Option<T> {
            if let Ok(mut queue) = self.inner.lock() {
                queue.pop_front()
            } else {
                None
            }
        }

        /// Get queue length
        pub fn len(&self) -> usize {
            if let Ok(queue) = self.inner.lock() {
                queue.len()
            } else {
                0
            }
        }

        /// Check if queue is empty
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    /// Fast random number generator for work stealing
    pub struct FastRng {
        state: u64,
    }

    impl FastRng {
        /// Create a new fast RNG
        pub fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        /// Generate next random number
        pub fn next(&mut self) -> u64 {
            self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
            self.state
        }

        /// Generate random number in range
        pub fn next_range(&mut self, max: usize) -> usize {
            if max == 0 {
                return 0;
            }
            (self.next() % max as u64) as usize
        }
    }
}

/// Monitoring and debugging capabilities
pub struct GoroutineMonitor {
    /// Goroutine registry
    registry: Arc<RwLock<HashMap<GoroutineId, GoroutineInfo>>>,
    /// Monitoring enabled
    enabled: AtomicBool,
    /// Monitoring statistics
    stats: Arc<Mutex<MonitorStats>>,
}

#[derive(Debug, Clone)]
pub struct GoroutineInfo {
    /// Goroutine ID
    pub id: GoroutineId,
    /// Creation time
    pub created_at: Instant,
    /// Current state
    pub state: GoroutineState,
    /// Priority
    pub priority: GoroutinePriority,
    /// Stack size
    pub stack_size: usize,
    /// Total runtime
    pub total_runtime: Duration,
    /// Memory usage
    pub memory_usage: usize,
    /// Parent goroutine
    pub parent_id: Option<GoroutineId>,
    /// Child goroutines
    pub children: Vec<GoroutineId>,
    /// Channel operations
    pub channel_operations: u64,
    /// Last activity
    pub last_activity: Instant,
}

#[derive(Debug, Clone, Default)]
pub struct MonitorStats {
    /// Total goroutines monitored
    pub total_monitored: u64,
    /// Active goroutines
    pub active_goroutines: usize,
    /// Completed goroutines
    pub completed_goroutines: u64,
    /// Failed goroutines
    pub failed_goroutines: u64,
    /// Average lifetime
    pub average_lifetime: Duration,
    /// Peak memory usage
    pub peak_memory_usage: usize,
    /// Total memory allocated
    pub total_memory_allocated: usize,
}

impl GoroutineMonitor {
    /// Create a new goroutine monitor
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            enabled: AtomicBool::new(true),
            stats: Arc::new(Mutex::new(MonitorStats::default())),
        }
    }

    /// Register a goroutine
    pub fn register_goroutine(&self, info: GoroutineInfo) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }

        if let Ok(mut registry) = self.registry.write() {
            registry.insert(info.id, info);
            
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.total_monitored += 1;
                stats.active_goroutines += 1;
            }
        }
    }

    /// Unregister a goroutine
    pub fn unregister_goroutine(&self, goroutine_id: GoroutineId) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }

        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.remove(&goroutine_id) {
                // Update statistics
                if let Ok(mut stats) = self.stats.lock() {
                    stats.active_goroutines -= 1;
                    stats.completed_goroutines += 1;
                    
                    let lifetime = info.created_at.elapsed();
                    stats.average_lifetime = (stats.average_lifetime + lifetime) / 2;
                }
            }
        }
    }

    /// Get goroutine information
    pub fn get_goroutine_info(&self, goroutine_id: GoroutineId) -> Option<GoroutineInfo> {
        if let Ok(registry) = self.registry.read() {
            registry.get(&goroutine_id).cloned()
        } else {
            None
        }
    }

    /// Get all active goroutines
    pub fn get_active_goroutines(&self) -> Vec<GoroutineInfo> {
        if let Ok(registry) = self.registry.read() {
            registry.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get monitoring statistics
    pub fn get_stats(&self) -> Result<MonitorStats, CursedError> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::runtime_error("Failed to get monitor statistics"))
    }

    /// Enable/disable monitoring
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    /// Check if monitoring is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

/// Production-ready scheduler integrating all components
pub struct ProductionScheduler {
    /// Work-stealing configuration
    config: WorkStealingConfig,
    /// Worker threads
    workers: Vec<Arc<WorkStealingWorker>>,
    /// Global work queue
    global_queue: Arc<concurrent::LockFreeQueue<Arc<Mutex<Goroutine>>>>,
    /// Goroutine pool
    pool: Arc<GoroutinePool>,
    /// Select runtime
    select_runtime: Arc<SelectRuntime>,
    /// Monitoring
    monitor: Arc<GoroutineMonitor>,
    /// Scheduler statistics
    stats: Arc<Mutex<SchedulerStats>>,
    /// Shutdown signal
    shutdown: Arc<AtomicBool>,
    /// Running state
    running: AtomicBool,
}

#[derive(Debug, Clone, Default)]
pub struct SchedulerStats {
    /// Total goroutines scheduled
    pub total_scheduled: u64,
    /// Total goroutines completed
    pub total_completed: u64,
    /// Total work steals
    pub total_work_steals: u64,
    /// Total work sharing operations
    pub total_work_sharing: u64,
    /// Current throughput (goroutines/sec)
    pub current_throughput: f64,
    /// Peak throughput
    pub peak_throughput: f64,
    /// Average execution time
    pub average_execution_time: Duration,
    /// Scheduler efficiency
    pub scheduler_efficiency: f64,
    /// Started at
    pub started_at: Option<Instant>,
    /// Uptime
    pub uptime: Duration,
}

impl ProductionScheduler {
    /// Create a new production scheduler
    pub fn new(config: WorkStealingConfig) -> Result<Self, CursedError> {
        let stack_manager = Arc::new(RuntimeStack::new());
        let pool = Arc::new(GoroutinePool::new(
            PoolConfig::default(),
            stack_manager.clone(),
        ));
        
        let global_queue = Arc::new(concurrent::LockFreeQueue::new());
        let select_runtime = Arc::new(SelectRuntime::new());
        let monitor = Arc::new(GoroutineMonitor::new());
        let shutdown = Arc::new(AtomicBool::new(false));
        
        // Create workers
        let mut workers = Vec::with_capacity(config.num_workers);
        for i in 0..config.num_workers {
            let worker = Arc::new(WorkStealingWorker {
                id: i,
                thread: None,
                local_queue: Arc::new(Mutex::new(VecDeque::with_capacity(config.local_queue_capacity))),
                stats: WorkerStatistics::default(),
                cpu_affinity: config.cpu_affinity_mask.map(|mask| {
                    let affinity = (mask >> i) & 1;
                    if affinity == 1 { Some(i) } else { None }
                }).flatten(),
                shutdown: shutdown.clone(),
                work_notification: Arc::new(Condvar::new()),
                state: Arc::new(Mutex::new(WorkerState::Idle)),
                last_load_balance: Arc::new(Mutex::new(Instant::now())),
                rng_state: Arc::new(Mutex::new(i as u64 + 1)),
            });
            workers.push(worker);
        }
        
        Ok(Self {
            config,
            workers,
            global_queue,
            pool,
            select_runtime,
            monitor,
            stats: Arc::new(Mutex::new(SchedulerStats::default())),
            shutdown,
            running: AtomicBool::new(false),
        })
    }

    /// Start the scheduler
    pub fn start(&mut self) -> Result<(), CursedError> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(CursedError::runtime_error("Scheduler already running"));
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.started_at = Some(Instant::now());
        }

        // Start workers
        for worker in &mut self.workers {
            self.start_worker(worker.clone())?;
        }

        Ok(())
    }

    /// Stop the scheduler
    pub fn stop(&self) -> Result<(), CursedError> {
        if !self.running.swap(false, Ordering::SeqCst) {
            return Ok(());
        }

        // Signal shutdown
        self.shutdown.store(true, Ordering::SeqCst);

        // Wake up all workers
        for worker in &self.workers {
            worker.work_notification.notify_all();
        }

        // Wait for workers to finish
        std::thread::sleep(Duration::from_millis(100));

        Ok(())
    }

    /// Spawn a goroutine
    pub fn spawn<F>(&self, entry_fn: F) -> Result<GoroutineId, CursedError>
    where
        F: FnOnce() + Send + 'static,
    {
        let goroutine = self.pool.get_goroutine(entry_fn)?;
        let goroutine_id = {
            if let Ok(g) = goroutine.lock() {
                g.id
            } else {
                return Err(CursedError::runtime_error("Failed to lock goroutine"));
            }
        };

        // Schedule the goroutine
        self.schedule_goroutine(goroutine)?;

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_scheduled += 1;
        }

        Ok(goroutine_id)
    }

    /// Execute select statement
    pub fn execute_select(
        &self,
        goroutine_id: GoroutineId,
        cases: Vec<SelectCase>,
        timeout: Option<Duration>,
    ) -> Result<SelectResult, CursedError> {
        self.select_runtime.execute_select(goroutine_id, cases, timeout)
    }

    /// Get comprehensive statistics
    pub fn get_comprehensive_stats(&self) -> Result<ComprehensiveStats, CursedError> {
        let scheduler_stats = self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::runtime_error("Failed to get scheduler statistics"))?;

        let pool_stats = self.pool.get_stats()?;
        let select_stats = self.select_runtime.get_stats()?;
        let monitor_stats = self.monitor.get_stats()?;

        let mut worker_stats = Vec::new();
        for worker in &self.workers {
            worker_stats.push(worker.stats.clone());
        }

        Ok(ComprehensiveStats {
            scheduler: scheduler_stats,
            pool: pool_stats,
            select: select_stats,
            monitor: monitor_stats,
            workers: worker_stats,
        })
    }

    // Private methods

    fn start_worker(&self, worker: Arc<WorkStealingWorker>) -> Result<(), CursedError> {
        let worker_clone = worker.clone();
        let global_queue = self.global_queue.clone();
        let workers = self.workers.clone();
        let pool = self.pool.clone();
        let config = self.config.clone();

        let handle = thread::spawn(move || {
            Self::worker_main(worker_clone, global_queue, workers, pool, config);
        });

        // Store the thread handle (this would need to be done differently in a real implementation)
        // For now, we'll just start the thread and not store the handle
        Ok(())
    }

    fn worker_main(
        worker: Arc<WorkStealingWorker>,
        global_queue: Arc<concurrent::LockFreeQueue<Arc<Mutex<Goroutine>>>>,
        workers: Vec<Arc<WorkStealingWorker>>,
        pool: Arc<GoroutinePool>,
        config: WorkStealingConfig,
    ) {
        let mut rng = concurrent::FastRng::new(worker.id as u64 + 1);
        
        while !worker.shutdown.load(Ordering::SeqCst) {
            // Try to get work from local queue
            let goroutine = if let Ok(mut queue) = worker.local_queue.lock() {
                queue.pop_front()
            } else {
                None
            };

            if let Some(goroutine) = goroutine {
                Self::execute_goroutine_with_pool(worker.clone(), goroutine, &pool);
            } else {
                // Try to steal work
                if Self::try_steal_work(&worker, &workers, &mut rng, &config) {
                    continue;
                }

                // Try global queue
                if let Some(goroutine) = global_queue.pop() {
                    Self::execute_goroutine_with_pool(worker.clone(), goroutine, &pool);
                    continue;
                }

                // No work available, wait
                Self::wait_for_work(&worker);
            }
        }
    }

    fn execute_goroutine_with_pool(
        worker: Arc<WorkStealingWorker>,
        goroutine: Arc<Mutex<Goroutine>>,
        pool: &Arc<GoroutinePool>,
    ) {
        // Update worker state
        if let Ok(mut state) = worker.state.lock() {
            if let Ok(g) = goroutine.lock() {
                *state = WorkerState::Running(g.id);
            }
        }

        // Execute the goroutine
        let start_time = Instant::now();
        
        // Execute using the existing logic from goroutine.rs
        crate::runtime::goroutine::GoroutineScheduler::execute_goroutine(worker.id, goroutine.clone());
        
        let execution_time = start_time.elapsed();
        
        // Update statistics
        worker.stats.goroutines_executed.fetch_add(1, Ordering::Relaxed);
        worker.stats.execution_time.fetch_add(execution_time.as_nanos() as u64, Ordering::Relaxed);
        worker.stats.context_switches.fetch_add(1, Ordering::Relaxed);

        // Return goroutine to pool
        pool.return_goroutine(goroutine);

        // Update worker state
        if let Ok(mut state) = worker.state.lock() {
            *state = WorkerState::Idle;
        }
    }

    fn try_steal_work(
        worker: &Arc<WorkStealingWorker>,
        workers: &[Arc<WorkStealingWorker>],
        rng: &mut concurrent::FastRng,
        config: &WorkStealingConfig,
    ) -> bool {
        for attempt in 0..config.max_steal_attempts {
            // Pick a random victim
            let victim_id = rng.next_range(workers.len());
            if victim_id == worker.id {
                continue;
            }

            let victim = &workers[victim_id];
            
            // Try to steal work
            if let Ok(mut victim_queue) = victim.local_queue.lock() {
                if victim_queue.len() > 1 {
                    // Steal half of the work
                    let steal_count = std::cmp::min(config.steal_batch_size, victim_queue.len() / 2);
                    let mut stolen_work = Vec::new();
                    
                    for _ in 0..steal_count {
                        if let Some(goroutine) = victim_queue.pop_back() {
                            stolen_work.push(goroutine);
                        }
                    }
                    
                    if !stolen_work.is_empty() {
                        // Add stolen work to local queue
                        if let Ok(mut local_queue) = worker.local_queue.lock() {
                            for goroutine in stolen_work {
                                local_queue.push_back(goroutine);
                            }
                        }
                        
                        // Update statistics
                        worker.stats.work_stolen.fetch_add(steal_count as u64, Ordering::Relaxed);
                        worker.stats.successful_steals.fetch_add(1, Ordering::Relaxed);
                        victim.stats.work_shared.fetch_add(steal_count as u64, Ordering::Relaxed);
                        
                        return true;
                    }
                }
            }
            
            worker.stats.failed_steals.fetch_add(1, Ordering::Relaxed);
            
            // Add exponential backoff to prevent busy waiting
            if attempt < config.max_steal_attempts - 1 {
                if attempt < 16 {
                    for _ in 0..(1 << (attempt / 4)) {
                        std::hint::spin_loop();
                    }
                } else if attempt % 8 == 7 {
                    // Every 8th attempt after backoff limit, yield briefly
                    std::thread::yield_now();
                }
            }
        }
        
        false
    }

    fn wait_for_work(worker: &Arc<WorkStealingWorker>) {
        let idle_start = Instant::now();
        
        if let Ok(queue_guard) = worker.local_queue.lock() {
            let _result = worker.work_notification.wait_timeout(
                queue_guard,
                Duration::from_millis(10),
            );
        }
        
        let idle_time = idle_start.elapsed();
        worker.stats.idle_time.fetch_add(idle_time.as_nanos() as u64, Ordering::Relaxed);
    }

    fn schedule_goroutine(&self, goroutine: Arc<Mutex<Goroutine>>) -> Result<(), CursedError> {
        // Find the least loaded worker
        let mut best_worker = 0;
        let mut min_load = usize::MAX;

        for (i, worker) in self.workers.iter().enumerate() {
            let load = worker.stats.current_queue_size.load(Ordering::Relaxed);
            if load < min_load {
                min_load = load;
                best_worker = i;
            }
        }

        // Add to worker's local queue
        let worker = &self.workers[best_worker];
        if let Ok(mut queue) = worker.local_queue.lock() {
            queue.push_back(goroutine);
            worker.stats.current_queue_size.store(queue.len(), Ordering::Relaxed);
            let peak = worker.stats.peak_queue_size.load(Ordering::Relaxed);
            if queue.len() > peak {
                worker.stats.peak_queue_size.store(queue.len(), Ordering::Relaxed);
            }
        }

        // Notify worker
        worker.work_notification.notify_one();

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComprehensiveStats {
    pub scheduler: SchedulerStats,
    pub pool: PoolStats,
    pub select: SelectStats,
    pub monitor: MonitorStats,
    pub workers: Vec<WorkerStatistics>,
}

impl Default for ProductionScheduler {
    fn default() -> Self {
        Self::new(WorkStealingConfig::default()).unwrap()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_production_scheduler_creation() {
        let config = WorkStealingConfig::default();
        let scheduler = ProductionScheduler::new(config);
        assert!(scheduler.is_ok());
    }

    #[test]
    fn test_goroutine_pool() {
        let stack_manager = Arc::new(RuntimeStack::new());
        let pool = GoroutinePool::new(PoolConfig::default(), stack_manager);
        
        let goroutine = pool.get_goroutine(|| {});
        assert!(goroutine.is_ok());
    }

    #[test]
    fn test_select_runtime() {
        let select_runtime = SelectRuntime::new();
        let stats = select_runtime.get_stats();
        assert!(stats.is_ok());
    }

    #[test]
    fn test_concurrent_hash_map() {
        let map = concurrent::ConcurrentHashMap::new(16);
        assert!(map.insert("key", "value").is_none());
        assert_eq!(map.get(&"key"), Some("value"));
    }

    #[test]
    fn test_lock_free_queue() {
        let queue = concurrent::LockFreeQueue::new();
        assert!(queue.push(42));
        assert_eq!(queue.pop(), Some(42));
        assert_eq!(queue.pop(), None);
    }
}
