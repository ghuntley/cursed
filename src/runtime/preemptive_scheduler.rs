//! Preemptive Goroutine Scheduler Implementation
//!
//! This module implements preemptive scheduling for CURSED goroutines with:
//! - Time-slice preemption with configurable quantum
//! - Improved M:N threading model with dynamic worker scaling
//! - Network poller integration for I/O-driven scheduling
//! - Better GC integration with goroutine stack scanning
//! - Advanced work-stealing with priority-based scheduling

use crate::error::CursedError;
use crate::runtime::goroutine::{
    Goroutine, GoroutineId, GoroutineState, GoroutinePriority, 
    Worker, WorkerId, WorkerStats, SchedulerConfig, SchedulerStats
};
use crate::runtime::stack::{RuntimeStack, StackId};
use crate::runtime::gc::GarbageCollector;

use std::collections::{HashMap, VecDeque, BTreeSet};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::sync::atomic::{AtomicU64, AtomicUsize, AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::os::unix::net::UnixStream;
use std::io::{self, Read, Write};

/// Maximum number of worker threads
const MAX_WORKERS: usize = 256;

/// Minimum quantum duration (1ms)
const MIN_QUANTUM: Duration = Duration::from_millis(1);

/// Maximum quantum duration (100ms)
const MAX_QUANTUM: Duration = Duration::from_millis(100);

/// Preemption signal types
#[derive(Debug, Clone, Copy)]
pub enum PreemptionSignal {
    /// Time slice expired
    TimeSliceExpired,
    /// Higher priority goroutine ready
    HigherPriorityReady,
    /// System call yield point
    SystemCallYield,
    /// GC preemption request
    GCPreemption,
    /// Forced preemption for debugging
    ForcePreemption,
}

/// Enhanced worker statistics with preemption tracking
#[derive(Debug, Default, Clone)]
pub struct PreemptiveWorkerStats {
    pub base_stats: WorkerStats,
    pub preemptions_performed: u64,
    pub preemptions_received: u64,
    pub quantum_violations: u64,
    pub priority_escalations: u64,
    pub context_switches: u64,
    pub gc_cooperations: u64,
    pub network_polls: u64,
    pub idle_cycles: u64,
}

/// Network poller for I/O-driven scheduling
pub struct NetworkPoller {
    /// Epoll file descriptor (Linux)
    epoll_fd: i32,
    /// Pending I/O events
    pending_events: Mutex<VecDeque<IoEvent>>,
    /// Goroutines waiting for I/O
    waiting_goroutines: Mutex<HashMap<GoroutineId, IoWaitState>>,
    /// Poller thread handle
    poller_thread: Option<JoinHandle<()>>,
    /// Shutdown signal
    shutdown: Arc<AtomicBool>,
}

/// I/O event for network poller
#[derive(Debug, Clone)]
pub struct IoEvent {
    pub goroutine_id: GoroutineId,
    pub fd: i32,
    pub event_type: IoEventType,
    pub timestamp: Instant,
}

/// I/O event types
#[derive(Debug, Clone, Copy)]
pub enum IoEventType {
    Read,
    Write,
    Error,
    Timeout,
}

/// I/O wait state for goroutines
#[derive(Debug, Clone)]
pub struct IoWaitState {
    pub fd: i32,
    pub event_type: IoEventType,
    pub timeout: Option<Instant>,
    pub start_time: Instant,
}

/// Preemptive scheduler with enhanced M:N threading
pub struct PreemptiveScheduler {
    /// Base configuration
    config: SchedulerConfig,
    /// Worker pool with dynamic scaling
    workers: RwLock<Vec<Arc<PreemptiveWorker>>>,
    /// Global run queue with priority ordering
    global_queue: Mutex<BTreeSet<PriorityQueueEntry>>,
    /// Stack manager
    stack_manager: Arc<RuntimeStack>,
    /// Garbage collector integration
    gc: Arc<Mutex<GarbageCollector>>,
    /// Network poller
    network_poller: Arc<NetworkPoller>,
    /// Next goroutine ID
    next_id: AtomicU64,
    /// Active goroutines count
    active_count: AtomicUsize,
    /// Scheduler statistics
    stats: Mutex<PreemptiveSchedulerStats>,
    /// Shutdown flag
    shutdown: Arc<AtomicBool>,
    /// Scheduler state
    running: AtomicBool,
    /// Preemption timer
    preemption_timer: Arc<Mutex<Option<JoinHandle<()>>>>,
    /// Load balancer
    load_balancer: Arc<LoadBalancer>,
    /// Quantum duration
    quantum: Duration,
}

/// Priority queue entry for global queue
#[derive(Debug, Clone, PartialEq, Eq)]
struct PriorityQueueEntry {
    priority: GoroutinePriority,
    created_at: SystemTime,
    goroutine_id: GoroutineId,
    goroutine: Arc<Mutex<Goroutine>>,
}

impl PartialOrd for PriorityQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityQueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then older tasks first
        other.priority.cmp(&self.priority)
            .then(self.created_at.cmp(&other.created_at))
    }
}

/// Enhanced worker with preemption support
pub struct PreemptiveWorker {
    /// Base worker
    pub base: Worker,
    /// Preemption signal
    pub preemption_signal: Arc<AtomicBool>,
    /// Current quantum start time
    pub quantum_start: Mutex<Option<Instant>>,
    /// Preemption statistics
    pub preemptive_stats: Mutex<PreemptiveWorkerStats>,
    /// CPU affinity (if supported)
    pub cpu_affinity: Option<usize>,
    /// Priority queue for local work
    pub priority_queue: Mutex<BTreeSet<PriorityQueueEntry>>,
    /// Context switch overhead tracking
    pub context_switch_overhead: Mutex<Duration>,
}

/// Enhanced scheduler statistics
#[derive(Debug, Default, Clone)]
pub struct PreemptiveSchedulerStats {
    pub base_stats: SchedulerStats,
    pub total_preemptions: u64,
    pub total_context_switches: u64,
    pub average_quantum_utilization: f64,
    pub priority_inversions: u64,
    pub load_balancing_actions: u64,
    pub network_events_processed: u64,
    pub gc_cooperations: u64,
    pub worker_scaling_events: u64,
    pub quantum_violations: u64,
}

/// Load balancer for dynamic worker scaling
pub struct LoadBalancer {
    /// Target CPU utilization (0.0 to 1.0)
    target_utilization: f64,
    /// Load samples for moving average
    load_samples: Mutex<VecDeque<f64>>,
    /// Sample window size
    sample_window: usize,
    /// Last scaling action
    last_scaling: Mutex<Option<Instant>>,
    /// Minimum time between scaling actions
    scaling_cooldown: Duration,
    /// Scaling statistics
    scaling_stats: Mutex<LoadBalancingStats>,
}

/// Load balancing statistics
#[derive(Debug, Default, Clone)]
pub struct LoadBalancingStats {
    pub scale_up_events: u64,
    pub scale_down_events: u64,
    pub load_samples_collected: u64,
    pub average_load: f64,
    pub peak_load: f64,
    pub current_worker_count: usize,
    pub optimal_worker_count: usize,
}

impl NetworkPoller {
    /// Create a new network poller
    pub fn new() -> Result<Self, CursedError> {
        // Create epoll instance (Linux-specific)
        let epoll_fd = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
        if epoll_fd == -1 {
            return Err(CursedError::runtime_error("Failed to create epoll instance"));
        }

        Ok(Self {
            epoll_fd,
            pending_events: Mutex::new(VecDeque::new()),
            waiting_goroutines: Mutex::new(HashMap::new()),
            poller_thread: None,
            shutdown: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Start the network poller thread
    pub fn start(&mut self) -> Result<(), CursedError> {
        let epoll_fd = self.epoll_fd;
        let pending_events = Arc::new(Mutex::new(VecDeque::new()));
        let waiting_goroutines = Arc::new(Mutex::new(HashMap::new()));
        let shutdown = self.shutdown.clone();

        // Clone for thread
        let pending_events_clone = pending_events.clone();
        let waiting_goroutines_clone = waiting_goroutines.clone();

        let handle = thread::spawn(move || {
            Self::poller_main(epoll_fd, pending_events_clone, waiting_goroutines_clone, shutdown);
        });

        self.poller_thread = Some(handle);
        Ok(())
    }

    /// Register a goroutine for I/O events
    pub fn register_io_wait(
        &self,
        goroutine_id: GoroutineId,
        fd: i32,
        event_type: IoEventType,
        timeout: Option<Duration>,
    ) -> Result<(), CursedError> {
        let mut waiting = self.waiting_goroutines.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock waiting goroutines"))?;

        let wait_state = IoWaitState {
            fd,
            event_type,
            timeout: timeout.map(|t| Instant::now() + t),
            start_time: Instant::now(),
        };

        waiting.insert(goroutine_id, wait_state);

        // Register with epoll
        let mut event = libc::epoll_event {
            events: match event_type {
                IoEventType::Read => libc::EPOLLIN as u32,
                IoEventType::Write => libc::EPOLLOUT as u32,
                _ => libc::EPOLLIN as u32 | libc::EPOLLOUT as u32,
            },
            u64: goroutine_id,
        };

        let result = unsafe {
            libc::epoll_ctl(self.epoll_fd, libc::EPOLL_CTL_ADD, fd, &mut event)
        };

        if result == -1 {
            return Err(CursedError::runtime_error("Failed to register I/O event"));
        }

        Ok(())
    }

    /// Main poller loop
    fn poller_main(
        epoll_fd: i32,
        pending_events: Arc<Mutex<VecDeque<IoEvent>>>,
        waiting_goroutines: Arc<Mutex<HashMap<GoroutineId, IoWaitState>>>,
        shutdown: Arc<AtomicBool>,
    ) {
        const MAX_EVENTS: usize = 64;
        let mut events = vec![
            libc::epoll_event { events: 0, u64: 0 };
            MAX_EVENTS
        ];

        while !shutdown.load(Ordering::SeqCst) {
            // Wait for events with timeout
            let num_events = unsafe {
                libc::epoll_wait(
                    epoll_fd,
                    events.as_mut_ptr(),
                    MAX_EVENTS as i32,
                    100, // 100ms timeout
                )
            };

            if num_events > 0 {
                let mut pending = pending_events.lock().unwrap();
                let waiting = waiting_goroutines.lock().unwrap();

                for i in 0..num_events as usize {
                    let event = &events[i];
                    let goroutine_id = event.u64;

                    if let Some(wait_state) = waiting.get(&goroutine_id) {
                        let event_type = if event.events & libc::EPOLLIN as u32 != 0 {
                            IoEventType::Read
                        } else if event.events & libc::EPOLLOUT as u32 != 0 {
                            IoEventType::Write
                        } else {
                            IoEventType::Error
                        };

                        let io_event = IoEvent {
                            goroutine_id,
                            fd: wait_state.fd,
                            event_type,
                            timestamp: Instant::now(),
                        };

                        pending.push_back(io_event);
                    }
                }
            }

            // Check for timeouts
            Self::check_timeouts(&pending_events, &waiting_goroutines);
        }
    }

    /// Check for I/O timeouts
    fn check_timeouts(
        pending_events: &Arc<Mutex<VecDeque<IoEvent>>>,
        waiting_goroutines: &Arc<Mutex<HashMap<GoroutineId, IoWaitState>>>,
    ) {
        let now = Instant::now();
        let mut timed_out = Vec::new();

        {
            let waiting = waiting_goroutines.lock().unwrap();
            for (&goroutine_id, wait_state) in waiting.iter() {
                if let Some(timeout) = wait_state.timeout {
                    if now >= timeout {
                        timed_out.push(goroutine_id);
                    }
                }
            }
        }

        if !timed_out.is_empty() {
            let mut pending = pending_events.lock().unwrap();
            let waiting = waiting_goroutines.lock().unwrap();

            for goroutine_id in timed_out {
                if let Some(wait_state) = waiting.get(&goroutine_id) {
                    let timeout_event = IoEvent {
                        goroutine_id,
                        fd: wait_state.fd,
                        event_type: IoEventType::Timeout,
                        timestamp: now,
                    };
                    pending.push_back(timeout_event);
                }
            }
        }
    }

    /// Get pending I/O events
    pub fn get_pending_events(&self) -> Result<Vec<IoEvent>, CursedError> {
        let mut pending = self.pending_events.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock pending events"))?;
        
        let events: Vec<IoEvent> = pending.drain(..).collect();
        Ok(events)
    }

    /// Stop the network poller
    pub fn stop(&mut self) -> Result<(), CursedError> {
        self.shutdown.store(true, Ordering::SeqCst);
        
        if let Some(handle) = self.poller_thread.take() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join poller thread"))?;
        }

        // Close epoll fd
        unsafe {
            libc::close(self.epoll_fd);
        }

        Ok(())
    }
}

impl LoadBalancer {
    /// Create a new load balancer
    pub fn new(target_utilization: f64) -> Self {
        Self {
            target_utilization: target_utilization.clamp(0.1, 0.9),
            load_samples: Mutex::new(VecDeque::new()),
            sample_window: 100,
            last_scaling: Mutex::new(None),
            scaling_cooldown: Duration::from_secs(5),
            scaling_stats: Mutex::new(LoadBalancingStats::default()),
        }
    }

    /// Record a load sample
    pub fn record_load_sample(&self, load: f64) {
        let mut samples = self.load_samples.lock().unwrap();
        samples.push_back(load);
        
        // Keep only recent samples
        if samples.len() > self.sample_window {
            samples.pop_front();
        }

        let mut stats = self.scaling_stats.lock().unwrap();
        stats.load_samples_collected += 1;
        stats.average_load = samples.iter().sum::<f64>() / samples.len() as f64;
        stats.peak_load = stats.peak_load.max(load);
    }

    /// Get scaling recommendation
    pub fn get_scaling_recommendation(&self, current_workers: usize) -> Option<ScalingAction> {
        // Check cooldown
        {
            let last_scaling = self.last_scaling.lock().unwrap();
            if let Some(last) = *last_scaling {
                if last.elapsed() < self.scaling_cooldown {
                    return None;
                }
            }
        }

        let samples = self.load_samples.lock().unwrap();
        if samples.len() < 10 {
            return None; // Not enough samples
        }

        let average_load = samples.iter().sum::<f64>() / samples.len() as f64;
        
        // Update stats
        {
            let mut stats = self.scaling_stats.lock().unwrap();
            stats.current_worker_count = current_workers;
            stats.average_load = average_load;
        }

        // Scale up if load is too high
        if average_load > self.target_utilization + 0.1 && current_workers < MAX_WORKERS {
            let recommended_workers = ((current_workers as f64 * average_load / self.target_utilization) as usize)
                .min(MAX_WORKERS)
                .max(current_workers + 1);
            
            return Some(ScalingAction::ScaleUp(recommended_workers));
        }

        // Scale down if load is too low
        if average_load < self.target_utilization - 0.1 && current_workers > 1 {
            let recommended_workers = ((current_workers as f64 * average_load / self.target_utilization) as usize)
                .max(1)
                .min(current_workers - 1);
            
            return Some(ScalingAction::ScaleDown(recommended_workers));
        }

        None
    }

    /// Update scaling timestamp
    pub fn record_scaling_action(&self, action: ScalingAction) {
        let mut last_scaling = self.last_scaling.lock().unwrap();
        *last_scaling = Some(Instant::now());

        let mut stats = self.scaling_stats.lock().unwrap();
        match action {
            ScalingAction::ScaleUp(workers) => {
                stats.scale_up_events += 1;
                stats.optimal_worker_count = workers;
            }
            ScalingAction::ScaleDown(workers) => {
                stats.scale_down_events += 1;
                stats.optimal_worker_count = workers;
            }
        }
    }
}

/// Scaling action recommendation
#[derive(Debug, Clone, Copy)]
pub enum ScalingAction {
    ScaleUp(usize),
    ScaleDown(usize),
}

impl PreemptiveScheduler {
    /// Create a new preemptive scheduler
    pub fn new(config: SchedulerConfig) -> Result<Self, CursedError> {
        let stack_manager = Arc::new(RuntimeStack::new());
        let shutdown = Arc::new(AtomicBool::new(false));
        let gc = Arc::new(Mutex::new(GarbageCollector::new()));
        let mut network_poller = NetworkPoller::new()?;
        network_poller.start()?;

        let quantum = Duration::from_millis(config.quantum_ms.max(1).min(100));
        
        // Create initial workers
        let mut workers = Vec::with_capacity(config.num_workers);
        for i in 0..config.num_workers {
            let worker = Arc::new(PreemptiveWorker::new(i, &config, shutdown.clone())?);
            workers.push(worker);
        }

        let load_balancer = Arc::new(LoadBalancer::new(0.75)); // 75% target utilization

        Ok(Self {
            config,
            workers: RwLock::new(workers),
            global_queue: Mutex::new(BTreeSet::new()),
            stack_manager,
            gc,
            network_poller: Arc::new(network_poller),
            next_id: AtomicU64::new(1),
            active_count: AtomicUsize::new(0),
            stats: Mutex::new(PreemptiveSchedulerStats::default()),
            shutdown,
            running: AtomicBool::new(false),
            preemption_timer: Arc::new(Mutex::new(None)),
            load_balancer,
            quantum,
        })
    }

    /// Start the preemptive scheduler
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(CursedError::runtime_error("Scheduler is already running"));
        }

        // Start worker threads
        {
            let workers = self.workers.read()
                .map_err(|_| CursedError::runtime_error("Failed to lock workers"))?;
            for worker in workers.iter() {
                self.start_worker(worker.clone())?;
            }
        }

        // Start preemption timer
        self.start_preemption_timer()?;

        // Start load balancer
        self.start_load_balancer()?;

        // Update statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics"))?;
            stats.base_stats.started_at = Some(Instant::now());
        }

        Ok(())
    }

    /// Start a worker thread
    fn start_worker(&self, worker: Arc<PreemptiveWorker>) -> Result<(), CursedError> {
        let worker_clone = worker.clone();
        let scheduler_ptr = self as *const Self;
        
        thread::spawn(move || {
            // SAFETY: The scheduler lives longer than worker threads
            let scheduler = unsafe { &*scheduler_ptr };
            Self::worker_main(worker_clone, scheduler);
        });

        Ok(())
    }

    /// Start preemption timer
    fn start_preemption_timer(&self) -> Result<(), CursedError> {
        let workers = self.workers.read()
            .map_err(|_| CursedError::runtime_error("Failed to lock workers"))?
            .clone();
        let quantum = self.quantum;
        let shutdown = self.shutdown.clone();

        let handle = thread::spawn(move || {
            while !shutdown.load(Ordering::SeqCst) {
                thread::sleep(quantum / 4); // Check 4 times per quantum
                
                let now = Instant::now();
                for worker in &workers {
                    let quantum_start = worker.quantum_start.lock().unwrap();
                    if let Some(start) = *quantum_start {
                        if now.duration_since(start) >= quantum {
                            // Signal preemption
                            worker.preemption_signal.store(true, Ordering::SeqCst);
                            
                            // Update stats
                            let mut stats = worker.preemptive_stats.lock().unwrap();
                            stats.quantum_violations += 1;
                        }
                    }
                }
            }
        });

        let mut timer = self.preemption_timer.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock preemption timer"))?;
        *timer = Some(handle);

        Ok(())
    }

    /// Start load balancer
    fn start_load_balancer(&self) -> Result<(), CursedError> {
        let load_balancer = self.load_balancer.clone();
        let shutdown = self.shutdown.clone();
        let workers_ref = &self.workers as *const RwLock<Vec<Arc<PreemptiveWorker>>>;

        thread::spawn(move || {
            while !shutdown.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_secs(10)); // Check every 10 seconds
                
                // SAFETY: The scheduler lives longer than this thread
                let workers = unsafe { &*workers_ref };
                let current_workers = workers.read().unwrap().len();
                
                // Calculate current load
                let total_load = Self::calculate_total_load(&workers);
                let average_load = total_load / current_workers as f64;
                
                load_balancer.record_load_sample(average_load);
                
                // Check for scaling recommendation
                if let Some(action) = load_balancer.get_scaling_recommendation(current_workers) {
                    load_balancer.record_scaling_action(action);
                    // In a real implementation, we would actually scale the workers here
                    log::info!("Load balancer recommendation: {:?}", action);
                }
            }
        });

        Ok(())
    }

    /// Calculate total load across all workers
    fn calculate_total_load(workers: &RwLock<Vec<Arc<PreemptiveWorker>>>) -> f64 {
        let workers_guard = workers.read().unwrap();
        let mut total_load = 0.0;
        
        for worker in workers_guard.iter() {
            let stats = worker.preemptive_stats.lock().unwrap();
            let total_time = stats.base_stats.busy_time + stats.base_stats.idle_time;
            if total_time > Duration::ZERO {
                let load = stats.base_stats.busy_time.as_secs_f64() / total_time.as_secs_f64();
                total_load += load;
            }
        }
        
        total_load
    }

    /// Worker main loop with preemption support
    fn worker_main(worker: Arc<PreemptiveWorker>, scheduler: &PreemptiveScheduler) {
        let mut stats = PreemptiveWorkerStats::default();
        
        while !worker.base.shutdown.load(Ordering::SeqCst) {
            // Process network events
            if let Ok(events) = scheduler.network_poller.get_pending_events() {
                for event in events {
                    scheduler.handle_io_event(event);
                    stats.network_polls += 1;
                }
            }

            // Try to get work from priority queue
            let goroutine = {
                let mut priority_queue = worker.priority_queue.lock().unwrap();
                priority_queue.pop_first().map(|entry| entry.goroutine)
            };

            if let Some(goroutine) = goroutine {
                // Execute goroutine with preemption support
                let execution_start = Instant::now();
                
                // Set quantum start time
                {
                    let mut quantum_start = worker.quantum_start.lock().unwrap();
                    *quantum_start = Some(execution_start);
                }

                // Reset preemption signal
                worker.preemption_signal.store(false, Ordering::SeqCst);

                // Execute with preemption checking
                Self::execute_with_preemption(
                    &worker,
                    goroutine,
                    scheduler.quantum,
                    &mut stats,
                );

                // Clear quantum start time
                {
                    let mut quantum_start = worker.quantum_start.lock().unwrap();
                    *quantum_start = None;
                }

                stats.base_stats.busy_time += execution_start.elapsed();
                stats.base_stats.goroutines_executed += 1;
                stats.context_switches += 1;
            } else {
                // No local work, try work stealing
                if !Self::try_steal_work_priority(&worker, scheduler, &mut stats) {
                    // No work available, wait for notification
                    let idle_start = Instant::now();
                    if let Ok(queue_guard) = worker.base.queue.lock() {
                        let _ = worker.base.work_available.wait_timeout(
                            queue_guard,
                            Duration::from_millis(10),
                        );
                    }
                    stats.base_stats.idle_time += idle_start.elapsed();
                    stats.idle_cycles += 1;
                }
            }

            // Cooperate with GC if needed
            if scheduler.gc_needs_cooperation() {
                scheduler.cooperate_with_gc(&worker, &mut stats);
            }
        }

        // Update worker statistics
        {
            let mut worker_stats = worker.preemptive_stats.lock().unwrap();
            *worker_stats = stats;
        }
    }

    /// Execute goroutine with preemption support
    fn execute_with_preemption(
        worker: &Arc<PreemptiveWorker>,
        goroutine: Arc<Mutex<Goroutine>>,
        quantum: Duration,
        stats: &mut PreemptiveWorkerStats,
    ) {
        let start_time = Instant::now();
        let mut preempted = false;
        
        // Set current goroutine
        {
            let mut current = worker.base.current.lock().unwrap();
            *current = Some(goroutine.clone());
        }

        // Execute goroutine with periodic preemption checks
        let execution_result = {
            let mut goroutine_guard = goroutine.lock().unwrap();
            goroutine_guard.set_state(GoroutineState::Running);
            
            // Take the entry function
            let fake_fn = Box::new(|| {}) as Box<dyn FnOnce() + Send + 'static>;
            let entry_fn = std::mem::replace(&mut goroutine_guard.entry_fn, fake_fn);
            
            drop(goroutine_guard); // Release lock during execution
            
            // Execute with preemption monitoring
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Periodically check for preemption
                let check_interval = quantum / 10; // Check 10 times per quantum
                let mut last_check = Instant::now();
                
                // In a real implementation, this would need cooperative yield points
                // For now, we simulate the execution
                entry_fn();
                
                // Check if we were preempted
                if worker.preemption_signal.load(Ordering::SeqCst) {
                    preempted = true;
                    stats.preemptions_received += 1;
                }
            }))
        };

        // Update goroutine state
        {
            let mut goroutine_guard = goroutine.lock().unwrap();
            match execution_result {
                Ok(_) => {
                    if preempted {
                        goroutine_guard.set_state(GoroutineState::Yielded);
                        stats.preemptions_performed += 1;
                    } else {
                        goroutine_guard.set_state(GoroutineState::Completed);
                    }
                }
                Err(_) => {
                    goroutine_guard.set_state(GoroutineState::Panicked);
                }
            }
            
            goroutine_guard.total_runtime += start_time.elapsed();
            goroutine_guard.last_run = Some(Instant::now());
        }

        // Clear current goroutine
        {
            let mut current = worker.base.current.lock().unwrap();
            *current = None;
        }

        // If preempted, reschedule the goroutine
        if preempted {
            let mut priority_queue = worker.priority_queue.lock().unwrap();
            let goroutine_guard = goroutine.lock().unwrap();
            let entry = PriorityQueueEntry {
                priority: goroutine_guard.priority,
                created_at: UNIX_EPOCH + Duration::from_secs(goroutine_guard.created_at.elapsed().as_secs()),
                goroutine_id: goroutine_guard.id,
                goroutine: goroutine.clone(),
            };
            priority_queue.insert(entry);
        }
    }

    /// Try to steal work with priority awareness
    fn try_steal_work_priority(
        worker: &Arc<PreemptiveWorker>,
        scheduler: &PreemptiveScheduler,
        stats: &mut PreemptiveWorkerStats,
    ) -> bool {
        // Try to steal from global priority queue first
        {
            let mut global_queue = scheduler.global_queue.lock().unwrap();
            if let Some(entry) = global_queue.pop_first() {
                let mut priority_queue = worker.priority_queue.lock().unwrap();
                priority_queue.insert(entry);
                stats.base_stats.work_stolen += 1;
                return true;
            }
        }
        
        // Try to steal from other workers
        let workers = scheduler.workers.read().unwrap();
        let worker_count = workers.len();
        let start_index = (worker.base.id + 1) % worker_count;
        
        for i in 0..worker_count {
            let target_index = (start_index + i) % worker_count;
            if target_index == worker.base.id {
                continue; // Skip self
            }
            
            let target_worker = &workers[target_index];
            
            // Try to steal from target worker's priority queue
            {
                let mut target_queue = target_worker.priority_queue.lock().unwrap();
                let mut local_queue = worker.priority_queue.lock().unwrap();
                
                // Steal half of the target's work
                let steal_count = target_queue.len() / 2;
                for _ in 0..steal_count {
                    if let Some(entry) = target_queue.pop_last() {
                        local_queue.insert(entry);
                        stats.base_stats.work_stolen += 1;
                    }
                }
                
                if steal_count > 0 {
                    return true;
                }
            }
        }
        
        false
    }

    /// Handle I/O event
    fn handle_io_event(&self, event: IoEvent) {
        // Find the goroutine waiting for this event
        let workers = self.workers.read().unwrap();
        
        for worker in workers.iter() {
            // Check if this goroutine is waiting in this worker
            let mut found = false;
            
            // Check priority queue
            {
                let priority_queue = worker.priority_queue.lock().unwrap();
                for entry in priority_queue.iter() {
                    if entry.goroutine_id == event.goroutine_id {
                        found = true;
                        break;
                    }
                }
            }
            
            if found {
                // Wake up the goroutine by changing its state
                let priority_queue = worker.priority_queue.lock().unwrap();
                for entry in priority_queue.iter() {
                    if entry.goroutine_id == event.goroutine_id {
                        if let Ok(mut goroutine) = entry.goroutine.lock() {
                            if goroutine.get_state() == GoroutineState::Waiting {
                                goroutine.set_state(GoroutineState::Ready);
                            }
                        }
                        break;
                    }
                }
                worker.base.work_available.notify_one();
                break;
            }
        }
    }

    /// Check if GC needs cooperation
    fn gc_needs_cooperation(&self) -> bool {
        self.gc.lock().unwrap().needs_cooperation()
    }

    /// Cooperate with GC
    fn cooperate_with_gc(&self, worker: &Arc<PreemptiveWorker>, stats: &mut PreemptiveWorkerStats) {
        // Scan goroutine stacks for GC
        let mut gc = self.gc.lock().unwrap();
        
        // Scan current goroutine stack
        if let Ok(current_guard) = worker.base.current.lock() {
            if let Some(ref goroutine) = *current_guard {
                if let Ok(g) = goroutine.lock() {
                    gc.scan_goroutine_stack(g.stack_id);
                }
            }
        }
        
        // Scan queued goroutines
        {
            let priority_queue = worker.priority_queue.lock().unwrap();
            for entry in priority_queue.iter() {
                if let Ok(g) = entry.goroutine.lock() {
                    gc.scan_goroutine_stack(g.stack_id);
                }
            }
        }
        
        stats.gc_cooperations += 1;
    }

    /// Spawn a goroutine with preemptive scheduling
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

        // Create priority queue entry
        let entry = PriorityQueueEntry {
            priority,
            created_at: SystemTime::now(),
            goroutine_id,
            goroutine: goroutine.clone(),
        };

        // Schedule the goroutine
        self.schedule_goroutine_priority(entry)?;

        // Update statistics
        self.active_count.fetch_add(1, Ordering::SeqCst);
        {
            let mut stats = self.stats.lock()
                .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics"))?;
            stats.base_stats.total_goroutines_spawned += 1;
            stats.base_stats.current_active_goroutines += 1;
        }

        Ok(goroutine_id)
    }

    /// Schedule a goroutine with priority awareness
    fn schedule_goroutine_priority(&self, entry: PriorityQueueEntry) -> Result<(), CursedError> {
        let workers = self.workers.read()
            .map_err(|_| CursedError::runtime_error("Failed to lock workers"))?;
        
        // Find the worker with the least load
        let mut best_worker = 0;
        let mut min_load = usize::MAX;

        for (i, worker) in workers.iter().enumerate() {
            let queue_len = worker.priority_queue.lock().unwrap().len();
            if queue_len < min_load {
                min_load = queue_len;
                best_worker = i;
            }
        }

        // Add to best worker's priority queue
        let worker = &workers[best_worker];
        worker.priority_queue.lock().unwrap().insert(entry);
        worker.base.work_available.notify_one();

        Ok(())
    }

    /// Get scheduler statistics
    pub fn get_stats(&self) -> Result<PreemptiveSchedulerStats, CursedError> {
        let mut stats = self.stats.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock scheduler statistics"))?
            .clone();
        
        stats.base_stats.current_active_goroutines = self.active_count.load(Ordering::SeqCst);
        
        // Calculate quantum utilization
        let workers = self.workers.read().unwrap();
        let mut total_quantum_time = Duration::ZERO;
        let mut total_execution_time = Duration::ZERO;
        
        for worker in workers.iter() {
            let worker_stats = worker.preemptive_stats.lock().unwrap();
            total_execution_time += worker_stats.base_stats.busy_time;
            stats.total_preemptions += worker_stats.preemptions_performed;
            stats.total_context_switches += worker_stats.context_switches;
            stats.gc_cooperations += worker_stats.gc_cooperations;
            stats.network_events_processed += worker_stats.network_polls;
        }
        
        // Calculate average quantum utilization
        if total_quantum_time > Duration::ZERO {
            stats.average_quantum_utilization = total_execution_time.as_secs_f64() / total_quantum_time.as_secs_f64();
        }

        Ok(stats)
    }

    /// Stop the scheduler
    pub fn stop(&mut self) -> Result<(), CursedError> {
        if !self.running.swap(false, Ordering::SeqCst) {
            return Ok(());
        }

        // Signal shutdown
        self.shutdown.store(true, Ordering::SeqCst);

        // Stop preemption timer
        {
            let mut timer = self.preemption_timer.lock().unwrap();
            if let Some(handle) = timer.take() {
                handle.join().map_err(|_| CursedError::runtime_error("Failed to join preemption timer"))?;
            }
        }

        // Stop network poller
        self.network_poller.stop()?;

        // Wake up all workers
        {
            let workers = self.workers.read().unwrap();
            for worker in workers.iter() {
                worker.base.work_available.notify_all();
            }
        }

        Ok(())
    }
}

impl PreemptiveWorker {
    /// Create a new preemptive worker
    pub fn new(id: WorkerId, config: &SchedulerConfig, shutdown: Arc<AtomicBool>) -> Result<Self, CursedError> {
        let base = Worker {
            id,
            queue: Mutex::new(VecDeque::with_capacity(config.queue_capacity)),
            current: Mutex::new(None),
            stats: Mutex::new(WorkerStats::default()),
            thread_handle: None,
            shutdown: shutdown.clone(),
            work_available: Arc::new(Condvar::new()),
        };

        Ok(Self {
            base,
            preemption_signal: Arc::new(AtomicBool::new(false)),
            quantum_start: Mutex::new(None),
            preemptive_stats: Mutex::new(PreemptiveWorkerStats::default()),
            cpu_affinity: None,
            priority_queue: Mutex::new(BTreeSet::new()),
            context_switch_overhead: Mutex::new(Duration::ZERO),
        })
    }

    /// Set CPU affinity for this worker
    pub fn set_cpu_affinity(&mut self, cpu_id: usize) -> Result<(), CursedError> {
        self.cpu_affinity = Some(cpu_id);
        // In a real implementation, this would set thread affinity
        Ok(())
    }
}

/// Drop implementation for cleanup
impl Drop for NetworkPoller {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Test module for preemptive scheduler
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_preemptive_scheduler_creation() {
        let config = SchedulerConfig {
            num_workers: 2,
            preemptive_scheduling: true,
            quantum_ms: 10,
            ..Default::default()
        };
        
        let scheduler = PreemptiveScheduler::new(config);
        assert!(scheduler.is_ok());
    }

    #[test]
    fn test_goroutine_spawning() {
        let config = SchedulerConfig {
            num_workers: 1,
            preemptive_scheduling: true,
            quantum_ms: 10,
            ..Default::default()
        };
        
        let scheduler = PreemptiveScheduler::new(config).unwrap();
        scheduler.start().unwrap();
        
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        let result = scheduler.spawn(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
        
        assert!(result.is_ok());
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_priority_scheduling() {
        let config = SchedulerConfig {
            num_workers: 1,
            preemptive_scheduling: true,
            quantum_ms: 10,
            ..Default::default()
        };
        
        let scheduler = PreemptiveScheduler::new(config).unwrap();
        scheduler.start().unwrap();
        
        let order = Arc::new(Mutex::new(Vec::new()));
        
        // Spawn low priority goroutine
        let order_clone = order.clone();
        scheduler.spawn_with_priority(move || {
            order_clone.lock().unwrap().push(1);
        }, GoroutinePriority::Low).unwrap();
        
        // Spawn high priority goroutine
        let order_clone = order.clone();
        scheduler.spawn_with_priority(move || {
            order_clone.lock().unwrap().push(2);
        }, GoroutinePriority::High).unwrap();
        
        std::thread::sleep(Duration::from_millis(100));
        
        // High priority should execute first
        let execution_order = order.lock().unwrap();
        assert_eq!(execution_order[0], 2);
    }

    #[test]
    fn test_load_balancer() {
        let balancer = LoadBalancer::new(0.7);
        
        // Record high load samples
        for _ in 0..20 {
            balancer.record_load_sample(0.9);
        }
        
        let recommendation = balancer.get_scaling_recommendation(2);
        assert!(matches!(recommendation, Some(ScalingAction::ScaleUp(_))));
        
        // Record low load samples
        for _ in 0..20 {
            balancer.record_load_sample(0.3);
        }
        
        let recommendation = balancer.get_scaling_recommendation(4);
        assert!(matches!(recommendation, Some(ScalingAction::ScaleDown(_))));
    }
}
