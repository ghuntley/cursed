use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use std::cmp::Ordering as CmpOrdering;

use crate::error::CursedError;
use crate::runtime::goroutine::{GoroutineScheduler, get_global_scheduler};

use super::executor::{TaskId, TaskPriority, TaskHandle, AsyncExecutor};
use super::task::{AsyncTask, TaskContext, TaskMetadata};

/// Scheduler work unit
#[derive(Debug)]
pub struct WorkItem {
    pub task_id: TaskId,
    pub priority: TaskPriority,
    pub scheduled_at: Instant,
    pub deadline: Option<Instant>,
}

impl WorkItem {
    pub fn new(task_id: TaskId, priority: TaskPriority) -> Self {
        Self {
            task_id,
            priority,
            scheduled_at: Instant::now(),
            deadline: None,
        }
    }

    pub fn with_deadline(task_id: TaskId, priority: TaskPriority, deadline: Instant) -> Self {
        Self {
            task_id,
            priority,
            scheduled_at: Instant::now(),
            deadline: Some(deadline),
        }
    }
}

impl PartialEq for WorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.scheduled_at == other.scheduled_at
    }
}

impl Eq for WorkItem {}

impl PartialOrd for WorkItem {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Ord for WorkItem {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        // First compare by priority (higher priority first)
        match other.priority.cmp(&self.priority) {
            CmpOrdering::Equal => {
                // Then by scheduled time (earlier first)
                self.scheduled_at.cmp(&other.scheduled_at)
            }
            other => other,
        }
    }
}

/// Work-stealing queue for tasks
pub struct WorkStealingQueue {
    local_queue: VecDeque<WorkItem>,
    steal_attempts: AtomicUsize,
    successful_steals: AtomicUsize,
}

impl WorkStealingQueue {
    pub fn new() -> Self {
        Self {
            local_queue: VecDeque::new(),
            steal_attempts: AtomicUsize::new(0),
            successful_steals: AtomicUsize::new(0),
        }
    }

    pub fn push_local(&mut self, item: WorkItem) {
        self.local_queue.push_back(item);
    }

    pub fn pop_local(&mut self) -> Option<WorkItem> {
        self.local_queue.pop_front()
    }

    pub fn steal(&mut self) -> Option<WorkItem> {
        self.steal_attempts.fetch_add(1, Ordering::Relaxed);
        if let Some(item) = self.local_queue.pop_back() {
            self.successful_steals.fetch_add(1, Ordering::Relaxed);
            Some(item)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.local_queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.local_queue.is_empty()
    }

    pub fn get_steal_stats(&self) -> (usize, usize) {
        (
            self.steal_attempts.load(Ordering::Relaxed),
            self.successful_steals.load(Ordering::Relaxed),
        )
    }
}

impl Default for WorkStealingQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduler worker thread
pub struct SchedulerWorker {
    pub id: usize,
    pub local_queue: Arc<Mutex<WorkStealingQueue>>,
    pub is_active: AtomicBool,
    pub tasks_executed: AtomicU64,
    pub total_runtime: Arc<Mutex<Duration>>,
    pub last_activity: Arc<Mutex<Instant>>,
    pub thread_handle: Mutex<Option<thread::JoinHandle<()>>>,
}

impl SchedulerWorker {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            local_queue: Arc::new(Mutex::new(WorkStealingQueue::new())),
            is_active: AtomicBool::new(false),
            tasks_executed: AtomicU64::new(0),
            total_runtime: Arc::new(Mutex::new(Duration::default())),
            last_activity: Arc::new(Mutex::new(Instant::now())),
            thread_handle: Mutex::new(None),
        }
    }

    pub fn queue_work(&self, work: WorkItem) {
        self.local_queue.lock().unwrap().push_local(work);
        *self.last_activity.lock().unwrap() = Instant::now();
    }

    pub fn try_steal_work(&self, other_workers: &[Arc<SchedulerWorker>]) -> Option<WorkItem> {
        for worker in other_workers {
            if worker.id != self.id {
                if let Ok(mut queue) = worker.local_queue.try_lock() {
                    if let Some(work) = queue.steal() {
                        return Some(work);
                    }
                }
            }
        }
        None
    }

    pub fn get_stats(&self) -> WorkerStats {
        let (steal_attempts, successful_steals) = {
            let queue = self.local_queue.lock().unwrap();
            queue.get_steal_stats()
        };

        WorkerStats {
            worker_id: self.id,
            is_active: self.is_active.load(Ordering::Relaxed),
            tasks_executed: self.tasks_executed.load(Ordering::Relaxed),
            queued_tasks: self.local_queue.lock().unwrap().len(),
            steal_attempts,
            successful_steals,
            total_runtime: *self.total_runtime.lock().unwrap(),
            last_activity: *self.last_activity.lock().unwrap(),
        }
    }
}

/// Worker statistics
#[derive(Debug, Clone)]
pub struct WorkerStats {
    pub worker_id: usize,
    pub is_active: bool,
    pub tasks_executed: u64,
    pub queued_tasks: usize,
    pub steal_attempts: usize,
    pub successful_steals: usize,
    pub total_runtime: Duration,
    pub last_activity: Instant,
}

/// Scheduling policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingPolicy {
    /// Fair scheduling - round-robin
    Fair,
    /// Priority-based scheduling
    Priority,
    /// Work-stealing scheduling
    WorkStealing,
    /// Deadline-aware scheduling
    Deadline,
    /// Adaptive scheduling
    Adaptive,
}

impl Default for SchedulingPolicy {
    fn default() -> Self {
        SchedulingPolicy::WorkStealing
    }
}

/// Scheduler configuration
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub num_workers: usize,
    pub scheduling_policy: SchedulingPolicy,
    pub work_stealing_enabled: bool,
    pub preemption_enabled: bool,
    pub time_slice_duration: Duration,
    pub max_tasks_per_worker: usize,
    pub enable_load_balancing: bool,
    pub load_balance_interval: Duration,
    pub enable_metrics: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            num_workers: num_cpus::get(),
            scheduling_policy: SchedulingPolicy::default(),
            work_stealing_enabled: true,
            preemption_enabled: false,
            time_slice_duration: Duration::from_millis(10),
            max_tasks_per_worker: 1000,
            enable_load_balancing: true,
            load_balance_interval: Duration::from_millis(100),
            enable_metrics: true,
        }
    }
}

/// Scheduler statistics
#[derive(Debug, Default, Clone)]
pub struct SchedulerStats {
    pub total_tasks_scheduled: u64,
    pub total_tasks_completed: u64,
    pub total_tasks_cancelled: u64,
    pub total_work_steals: u64,
    pub active_workers: usize,
    pub average_queue_length: f64,
    pub total_scheduler_time: Duration,
    pub uptime: Duration,
    pub started_at: Option<Instant>,
}

/// Advanced async task scheduler with work-stealing and priority scheduling
pub struct AsyncScheduler {
    config: SchedulerConfig,
    workers: Vec<Arc<SchedulerWorker>>,
    global_queue: Arc<Mutex<BinaryHeap<WorkItem>>>,
    deadline_queue: Arc<Mutex<BinaryHeap<WorkItem>>>,
    running_tasks: Arc<Mutex<HashMap<TaskId, Arc<Mutex<dyn std::any::Any + Send>>>>>,
    next_worker: AtomicUsize,
    stats: Arc<Mutex<SchedulerStats>>,
    shutdown: Arc<AtomicBool>,
    load_balancer_handle: Mutex<Option<thread::JoinHandle<()>>>,
    executor: Option<Arc<AsyncExecutor>>,
    goroutine_scheduler: Option<Arc<GoroutineScheduler>>,
}

impl AsyncScheduler {
    /// Create a new async scheduler
    pub fn new() -> Result<Self, CursedError> {
        Self::with_config(SchedulerConfig::default())
    }

    /// Create a new async scheduler with custom configuration
    pub fn with_config(config: SchedulerConfig) -> Result<Self, CursedError> {
        let mut workers = Vec::with_capacity(config.num_workers);
        for i in 0..config.num_workers {
            workers.push(Arc::new(SchedulerWorker::new(i)));
        }

        let mut stats = SchedulerStats::default();
        stats.started_at = Some(Instant::now());

        Ok(Self {
            config,
            workers,
            global_queue: Arc::new(Mutex::new(BinaryHeap::new())),
            deadline_queue: Arc::new(Mutex::new(BinaryHeap::new())),
            running_tasks: Arc::new(Mutex::new(HashMap::new())),
            next_worker: AtomicUsize::new(0),
            stats: Arc::new(Mutex::new(stats)),
            shutdown: Arc::new(AtomicBool::new(false)),
            load_balancer_handle: Mutex::new(None),
            executor: None,
            goroutine_scheduler: get_global_scheduler(),
        })
    }

    /// Set the executor for this scheduler
    pub fn set_executor(&mut self, executor: Arc<AsyncExecutor>) {
        self.executor = Some(executor);
    }

    /// Start the scheduler
    pub fn start(&self) -> Result<(), CursedError> {
        // Start worker threads
        for worker in &self.workers {
            self.start_worker(worker.clone())?;
        }

        // Start load balancer if enabled
        if self.config.enable_load_balancing {
            self.start_load_balancer()?;
        }

        Ok(())
    }

    /// Stop the scheduler
    pub fn stop(&self) -> Result<(), CursedError> {
        self.shutdown.store(true, Ordering::SeqCst);

        // Stop load balancer
        if let Some(handle) = self.load_balancer_handle.lock().unwrap().take() {
            let _ = handle.join();
        }

        // Wait for workers to finish
        for worker in &self.workers {
            if let Some(handle) = worker.thread_handle.lock().unwrap().take() {
                let _ = handle.join();
            }
        }

        Ok(())
    }

    /// Schedule a task
    pub fn schedule_task(&self, task_id: TaskId, priority: TaskPriority) -> Result<(), CursedError> {
        let work_item = WorkItem::new(task_id, priority);
        
        match self.config.scheduling_policy {
            SchedulingPolicy::Fair => self.schedule_fair(work_item),
            SchedulingPolicy::Priority => self.schedule_priority(work_item),
            SchedulingPolicy::WorkStealing => self.schedule_work_stealing(work_item),
            SchedulingPolicy::Deadline => self.schedule_deadline(work_item),
            SchedulingPolicy::Adaptive => self.schedule_adaptive(work_item),
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_tasks_scheduled += 1;
        }

        Ok(())
    }

    /// Schedule a task with deadline
    pub fn schedule_task_with_deadline(
        &self,
        task_id: TaskId,
        priority: TaskPriority,
        deadline: Instant,
    ) -> Result<(), CursedError> {
        let work_item = WorkItem::with_deadline(task_id, priority, deadline);
        
        // Always use deadline queue for tasks with deadlines
        self.deadline_queue.lock().unwrap().push(work_item);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_tasks_scheduled += 1;
        }

        Ok(())
    }

    /// Cancel a task
    pub fn cancel_task(&self, task_id: TaskId) -> Result<(), CursedError> {
        // Remove from running tasks
        self.running_tasks.lock().unwrap().remove(&task_id);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_tasks_cancelled += 1;
        }

        Ok(())
    }

    /// Get scheduler statistics
    pub fn get_stats(&self) -> SchedulerStats {
        let mut stats = self.stats.lock().unwrap().clone();
        
        // Update runtime stats
        if let Some(started_at) = stats.started_at {
            stats.uptime = started_at.elapsed();
        }

        // Count active workers
        stats.active_workers = self.workers
            .iter()
            .filter(|w| w.is_active.load(Ordering::Relaxed))
            .count();

        // Calculate average queue length
        let total_queued: usize = self.workers
            .iter()
            .map(|w| w.local_queue.lock().unwrap().len())
            .sum();
        
        stats.average_queue_length = if self.workers.is_empty() {
            0.0
        } else {
            total_queued as f64 / self.workers.len() as f64
        };

        stats
    }

    /// Get worker statistics
    pub fn get_worker_stats(&self) -> Vec<WorkerStats> {
        self.workers.iter().map(|w| w.get_stats()).collect()
    }

    /// Check if scheduler is running
    pub fn is_running(&self) -> bool {
        !self.shutdown.load(Ordering::SeqCst)
    }

    // Private scheduling methods

    fn schedule_fair(&self, work_item: WorkItem) {
        let next_worker = self.next_worker.fetch_add(1, Ordering::SeqCst) % self.workers.len();
        self.workers[next_worker].queue_work(work_item);
    }

    fn schedule_priority(&self, work_item: WorkItem) {
        // Find the worker with the highest priority capacity
        let mut best_worker = 0;
        let mut best_score = i32::MIN;

        for (i, worker) in self.workers.iter().enumerate() {
            let queue_len = worker.local_queue.lock().unwrap().len() as i32;
            let score = work_item.priority as i32 * 100 - queue_len;
            
            if score > best_score {
                best_score = score;
                best_worker = i;
            }
        }

        self.workers[best_worker].queue_work(work_item);
    }

    fn schedule_work_stealing(&self, work_item: WorkItem) {
        // Find the least loaded worker
        let mut best_worker = 0;
        let mut min_load = usize::MAX;

        for (i, worker) in self.workers.iter().enumerate() {
            let load = worker.local_queue.lock().unwrap().len();
            if load < min_load {
                min_load = load;
                best_worker = i;
            }
        }

        self.workers[best_worker].queue_work(work_item);
    }

    fn schedule_deadline(&self, work_item: WorkItem) {
        if work_item.deadline.is_some() {
            self.deadline_queue.lock().unwrap().push(work_item);
        } else {
            self.schedule_priority(work_item);
        }
    }

    fn schedule_adaptive(&self, work_item: WorkItem) {
        // Adaptive scheduling based on current system load
        let total_queued: usize = self.workers
            .iter()
            .map(|w| w.local_queue.lock().unwrap().len())
            .sum();

        let average_load = total_queued as f64 / self.workers.len() as f64;

        if average_load > 10.0 {
            // High load - use work stealing
            self.schedule_work_stealing(work_item);
        } else if work_item.priority as u8 >= TaskPriority::High as u8 {
            // Low load but high priority - use priority scheduling
            self.schedule_priority(work_item);
        } else {
            // Low load, normal priority - use fair scheduling
            self.schedule_fair(work_item);
        }
    }

    fn start_worker(&self, worker: Arc<SchedulerWorker>) -> Result<(), CursedError> {
        let worker_clone = worker.clone();
        let workers_clone = self.workers.clone();
        let global_queue = self.global_queue.clone();
        let deadline_queue = self.deadline_queue.clone();
        let shutdown = self.shutdown.clone();
        let config = self.config.clone();
        let stats = self.stats.clone();

        let handle = thread::spawn(move || {
            Self::worker_main(
                worker_clone,
                workers_clone,
                global_queue,
                deadline_queue,
                shutdown,
                config,
                stats,
            );
        });

        *worker.thread_handle.lock().unwrap() = Some(handle);
        Ok(())
    }

    fn worker_main(
        worker: Arc<SchedulerWorker>,
        workers: Vec<Arc<SchedulerWorker>>,
        global_queue: Arc<Mutex<BinaryHeap<WorkItem>>>,
        deadline_queue: Arc<Mutex<BinaryHeap<WorkItem>>>,
        shutdown: Arc<AtomicBool>,
        config: SchedulerConfig,
        stats: Arc<Mutex<SchedulerStats>>,
    ) {
        worker.is_active.store(true, Ordering::SeqCst);
        let mut idle_time = Duration::default();
        let mut last_activity = Instant::now();

        while !shutdown.load(Ordering::SeqCst) {
            let work_start = Instant::now();
            let mut work_found = false;

            // Check deadline queue first
            if let Ok(mut deadline_q) = deadline_queue.try_lock() {
                if let Some(work_item) = deadline_q.pop() {
                    Self::execute_work_item(work_item, &worker, &stats);
                    work_found = true;
                }
            }

            // Check local queue
            if !work_found {
                if let Some(work_item) = worker.local_queue.lock().unwrap().pop_local() {
                    Self::execute_work_item(work_item, &worker, &stats);
                    work_found = true;
                }
            }

            // Try work stealing if enabled and no local work
            if !work_found && config.work_stealing_enabled {
                if let Some(work_item) = worker.try_steal_work(&workers) {
                    Self::execute_work_item(work_item, &worker, &stats);
                    work_found = true;
                    
                    let mut stats_guard = stats.lock().unwrap();
                    stats_guard.total_work_steals += 1;
                }
            }

            // Check global queue as fallback
            if !work_found {
                if let Ok(mut global_q) = global_queue.try_lock() {
                    if let Some(work_item) = global_q.pop() {
                        Self::execute_work_item(work_item, &worker, &stats);
                        work_found = true;
                    }
                }
            }

            if work_found {
                last_activity = Instant::now();
                *worker.total_runtime.lock().unwrap() += work_start.elapsed();
            } else {
                // No work found - sleep briefly to avoid busy waiting
                thread::sleep(Duration::from_micros(100));
                idle_time += work_start.elapsed();
            }

            // Update last activity
            *worker.last_activity.lock().unwrap() = last_activity;
        }

        worker.is_active.store(false, Ordering::SeqCst);
    }

    fn execute_work_item(
        work_item: WorkItem,
        worker: &Arc<SchedulerWorker>,
        stats: &Arc<Mutex<SchedulerStats>>,
    ) {
        // In a real implementation, this would execute the actual task
        // For now, we just simulate work and update statistics
        
        worker.tasks_executed.fetch_add(1, Ordering::SeqCst);
        
        let mut stats_guard = stats.lock().unwrap();
        stats_guard.total_tasks_completed += 1;
        
        // Simulate some work
        thread::sleep(Duration::from_micros(10));
    }

    fn start_load_balancer(&self) -> Result<(), CursedError> {
        let workers = self.workers.clone();
        let shutdown = self.shutdown.clone();
        let interval = self.config.load_balance_interval;

        let handle = thread::spawn(move || {
            while !shutdown.load(Ordering::SeqCst) {
                Self::balance_load(&workers);
                thread::sleep(interval);
            }
        });

        *self.load_balancer_handle.lock().unwrap() = Some(handle);
        Ok(())
    }

    fn balance_load(workers: &[Arc<SchedulerWorker>]) {
        // Simple load balancing - move work from heavily loaded to lightly loaded workers
        let mut worker_loads: Vec<(usize, usize)> = workers
            .iter()
            .enumerate()
            .map(|(i, w)| (i, w.local_queue.lock().unwrap().len()))
            .collect();

        worker_loads.sort_by_key(|(_, load)| *load);

        if worker_loads.len() >= 2 {
            let (lightest_idx, lightest_load) = worker_loads[0];
            let (heaviest_idx, heaviest_load) = worker_loads[worker_loads.len() - 1];

            // If the difference is significant, steal some work
            if heaviest_load > lightest_load + 5 {
                let work_to_steal = (heaviest_load - lightest_load) / 4;
                
                for _ in 0..work_to_steal {
                    if let Some(work_item) = workers[heaviest_idx]
                        .local_queue
                        .lock()
                        .unwrap()
                        .steal()
                    {
                        workers[lightest_idx].queue_work(work_item);
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

impl Default for AsyncScheduler {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// Global scheduler instance
static GLOBAL_ASYNC_SCHEDULER: once_cell::sync::OnceCell<Arc<AsyncScheduler>> = once_cell::sync::OnceCell::new();

/// Initialize the global async scheduler
pub fn initialize_global_async_scheduler() -> Result<(), CursedError> {
    initialize_global_async_scheduler_with_config(SchedulerConfig::default())
}

/// Initialize the global async scheduler with custom configuration
pub fn initialize_global_async_scheduler_with_config(config: SchedulerConfig) -> Result<(), CursedError> {
    let scheduler = Arc::new(AsyncScheduler::with_config(config)?);
    
    GLOBAL_ASYNC_SCHEDULER
        .set(scheduler.clone())
        .map_err(|_| CursedError::runtime_error("Global async scheduler already initialized"))?;

    scheduler.start()?;
    Ok(())
}

/// Get the global async scheduler
pub fn get_global_async_scheduler() -> Option<Arc<AsyncScheduler>> {
    GLOBAL_ASYNC_SCHEDULER.get().cloned()
}

/// Shutdown the global async scheduler
pub fn shutdown_global_async_scheduler() -> Result<(), CursedError> {
    if let Some(scheduler) = get_global_async_scheduler() {
        scheduler.stop()
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_item_ordering() {
        let item1 = WorkItem::new(1, TaskPriority::Normal);
        let item2 = WorkItem::new(2, TaskPriority::High);
        
        assert!(item2 > item1); // Higher priority should be greater
    }

    #[test]
    fn test_work_stealing_queue() {
        let mut queue = WorkStealingQueue::new();
        let item = WorkItem::new(1, TaskPriority::Normal);
        
        queue.push_local(item);
        assert_eq!(queue.len(), 1);
        
        let stolen = queue.steal();
        assert!(stolen.is_some());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_scheduler_creation() {
        let scheduler = AsyncScheduler::new().unwrap();
        assert!(scheduler.is_running());
        
        let stats = scheduler.get_stats();
        assert_eq!(stats.total_tasks_scheduled, 0);
    }
}
