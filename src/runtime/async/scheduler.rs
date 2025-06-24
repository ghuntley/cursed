use crate::error::Error;
/// Async scheduler that integrates with the existing goroutine scheduler
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, Condvar};
use std::time::{Duration, Instant};
use std::thread::{self, ThreadId};

use crate::runtime::r#async::{Task, TaskId, TaskPriority, TaskState};
use crate::runtime::goroutine::{GoroutineScheduler, SafePoint};

/// Configuration for the async scheduler
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Maximum number of tasks in ready queue
    pub max_ready_queue_size: usize,
    /// Task scheduling quantum (time slice)
    pub scheduling_quantum: Duration,
    /// Enable priority-based scheduling
    pub enable_priority_scheduling: bool,
    /// Work stealing threshold
    pub work_stealing_threshold: usize,
    /// Integration with goroutine scheduler
    pub goroutine_integration: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_ready_queue_size: 10000,
            scheduling_quantum: Duration::from_millis(10),
            enable_priority_scheduling: true,
            work_stealing_threshold: 100,
            goroutine_integration: true,
        }
    }
}

/// Work-stealing queue for load balancing between async workers
pub struct WorkStealingQueue<T> {
    local_queue: VecDeque<T>,
    steal_queue: VecDeque<T>,
    total_size: usize,
}

impl<T> WorkStealingQueue<T> {
    pub fn new() -> Self {
        Self {
            local_queue: VecDeque::new(),
            steal_queue: VecDeque::new(),
            total_size: 0,
        }
    }

    /// Push a task to the local queue
    pub fn push_local(&mut self, item: T) {
        self.local_queue.push_back(item);
        self.total_size += 1;
    }

    /// Pop a task from the local queue (LIFO for cache efficiency)
    pub fn pop_local(&mut self) -> Option<T> {
        if let Some(item) = self.local_queue.pop_back() {
            self.total_size -= 1;
            Some(item)
        } else {
            None
        }
    }

    /// Steal a task from this queue (called by other workers)
    pub fn steal(&mut self) -> Option<T> {
        // Try to steal from the front of the local queue
        if let Some(item) = self.local_queue.pop_front() {
            self.total_size -= 1;
            return Some(item);
        }

        // Try the steal queue
        if let Some(item) = self.steal_queue.pop_front() {
            self.total_size -= 1;
            return Some(item);
        }

        None
    }

    /// Move half of local queue to steal queue for better stealing
    pub fn prepare_for_stealing(&mut self) {
        let half = self.local_queue.len() / 2;
        for _ in 0..half {
            if let Some(item) = self.local_queue.pop_front() {
                self.steal_queue.push_back(item);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.total_size
    }

    pub fn is_empty(&self) -> bool {
        self.total_size == 0
    }
}

impl<T> Default for WorkStealingQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Task scheduler entry
struct SchedulerTask {
    task_id: TaskId,
    priority: TaskPriority,
    created_at: Instant,
    last_poll: Option<Instant>,
    poll_count: u64,
}

/// Priority-based task queue
struct PriorityTaskQueue {
    critical: VecDeque<SchedulerTask>,
    high: VecDeque<SchedulerTask>,
    normal: VecDeque<SchedulerTask>,
    low: VecDeque<SchedulerTask>,
    total_size: usize,
}

impl PriorityTaskQueue {
    pub fn new() -> Self {
        Self {
            critical: VecDeque::new(),
            high: VecDeque::new(),
            normal: VecDeque::new(),
            low: VecDeque::new(),
            total_size: 0,
        }
    }

    pub fn push(&mut self, task: SchedulerTask) {
        match task.priority {
            TaskPriority::Critical => self.critical.push_back(task),
            TaskPriority::High => self.high.push_back(task),
            TaskPriority::Normal => self.normal.push_back(task),
            TaskPriority::Low => self.low.push_back(task),
        }
        self.total_size += 1;
    }

    pub fn pop(&mut self) -> Option<SchedulerTask> {
        // Check queues in priority order
        if let Some(task) = self.critical.pop_front() {
            self.total_size -= 1;
            return Some(task);
        }

        if let Some(task) = self.high.pop_front() {
            self.total_size -= 1;
            return Some(task);
        }

        if let Some(task) = self.normal.pop_front() {
            self.total_size -= 1;
            return Some(task);
        }

        if let Some(task) = self.low.pop_front() {
            self.total_size -= 1;
            return Some(task);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.total_size
    }

    pub fn is_empty(&self) -> bool {
        self.total_size == 0
    }
}

/// Async scheduler that coordinates with goroutine scheduler
pub struct AsyncScheduler {
    config: SchedulerConfig,
    ready_queue: Arc<Mutex<PriorityTaskQueue>>,
    work_stealing_queues: Arc<Mutex<HashMap<ThreadId, WorkStealingQueue<SchedulerTask>>>>,
    waiting_tasks: Arc<Mutex<HashMap<TaskId, SchedulerTask>>>,
    goroutine_scheduler: Option<Arc<GoroutineScheduler>>,
    scheduler_condvar: Arc<Condvar>,
    worker_threads: Vec<thread::JoinHandle<()>>,
    shutdown_signal: Arc<Mutex<bool>>,
    statistics: Arc<Mutex<SchedulerStatistics>>,
}

/// Scheduler statistics
#[derive(Debug, Clone, Default)]
pub struct SchedulerStatistics {
    pub total_tasks_scheduled: u64,
    pub tasks_completed: u64,
    pub tasks_preempted: u64,
    pub work_steal_attempts: u64,
    pub work_steal_successes: u64,
    pub average_task_time: Duration,
    pub scheduler_overhead: Duration,
}

impl AsyncScheduler {
    /// Create a new async scheduler
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            config,
            ready_queue: Arc::new(Mutex::new(PriorityTaskQueue::new())),
            work_stealing_queues: Arc::new(Mutex::new(HashMap::new())),
            waiting_tasks: Arc::new(Mutex::new(HashMap::new())),
            goroutine_scheduler: None,
            scheduler_condvar: Arc::new(Condvar::new()),
            worker_threads: Vec::new(),
            shutdown_signal: Arc::new(Mutex::new(false)),
            statistics: Arc::new(Mutex::new(SchedulerStatistics::default())),
        }
    }

    /// Set the goroutine scheduler for integration
    pub fn set_goroutine_scheduler(&mut self, scheduler: Arc<GoroutineScheduler>) {
        self.goroutine_scheduler = Some(scheduler);
    }

    /// Start the scheduler with worker threads
    pub fn start(&mut self, num_workers: usize) {
        for worker_id in 0..num_workers {
            let ready_queue = self.ready_queue.clone();
            let work_stealing_queues = self.work_stealing_queues.clone();
            let waiting_tasks = self.waiting_tasks.clone();
            let scheduler_condvar = self.scheduler_condvar.clone();
            let shutdown_signal = self.shutdown_signal.clone();
            let statistics = self.statistics.clone();
            let config = self.config.clone();
            let goroutine_scheduler = self.goroutine_scheduler.clone();

            let handle = thread::Builder::new()
                .name(format!("async-scheduler-{}", worker_id))
                .spawn(move || {
                    Self::worker_loop(
                        worker_id,
                        ready_queue,
                        work_stealing_queues,
                        waiting_tasks,
                        scheduler_condvar,
                        shutdown_signal,
                        statistics,
                        config,
                        goroutine_scheduler,
                    );
                })
                .expect("Failed to spawn scheduler worker");

            self.worker_threads.push(handle);
        }
    }

    /// Schedule a task
    pub fn schedule_task(&self, task_id: TaskId, priority: TaskPriority) {
        let scheduler_task = SchedulerTask {
            task_id,
            priority,
            created_at: Instant::now(),
            last_poll: None,
            poll_count: 0,
        };

        let mut ready_queue = self.ready_queue.lock().unwrap();
        ready_queue.push(scheduler_task);

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.total_tasks_scheduled += 1;
        }

        // Notify workers
        self.scheduler_condvar.notify_one();
    }

    /// Mark a task as waiting
    pub fn mark_task_waiting(&self, task_id: TaskId) {
        let mut ready_queue = self.ready_queue.lock().unwrap();
        let mut waiting_tasks = self.waiting_tasks.lock().unwrap();

        // Move task from ready to waiting if it exists
        // For now, we'll just add it to waiting
        let scheduler_task = SchedulerTask {
            task_id,
            priority: TaskPriority::Normal,
            created_at: Instant::now(),
            last_poll: Some(Instant::now()),
            poll_count: 1,
        };

        waiting_tasks.insert(task_id, scheduler_task);
    }

    /// Wake up a waiting task
    pub fn wake_task(&self, task_id: TaskId) {
        let mut waiting_tasks = self.waiting_tasks.lock().unwrap();
        let mut ready_queue = self.ready_queue.lock().unwrap();

        if let Some(mut task) = waiting_tasks.remove(&task_id) {
            task.poll_count += 1;
            ready_queue.push(task);
            
            // Notify workers
            drop(ready_queue);
            drop(waiting_tasks);
            self.scheduler_condvar.notify_one();
        }
    }

    /// Get scheduler statistics
    pub fn statistics(&self) -> SchedulerStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// Shutdown the scheduler
    pub fn shutdown(&mut self) {
        // Signal shutdown
        {
            let mut shutdown = self.shutdown_signal.lock().unwrap();
            *shutdown = true;
        }

        // Notify all workers
        self.scheduler_condvar.notify_all();

        // Wait for workers to finish
        for handle in self.worker_threads.drain(..) {
            let _ = handle.join();
        }
    }

    /// Worker thread main loop
    fn worker_loop(
        worker_id: usize,
        ready_queue: Arc<Mutex<PriorityTaskQueue>>,
        work_stealing_queues: Arc<Mutex<HashMap<ThreadId, WorkStealingQueue<SchedulerTask>>>>,
        waiting_tasks: Arc<Mutex<HashMap<TaskId, SchedulerTask>>>,
        scheduler_condvar: Arc<Condvar>,
        shutdown_signal: Arc<Mutex<bool>>,
        statistics: Arc<Mutex<SchedulerStatistics>>,
        config: SchedulerConfig,
        goroutine_scheduler: Option<Arc<GoroutineScheduler>>,
    ) {
        let thread_id = thread::current().id();

        // Initialize work stealing queue for this worker
        {
            let mut queues = work_stealing_queues.lock().unwrap();
            queues.insert(thread_id, WorkStealingQueue::new());
        }

        loop {
            // Check for shutdown
            {
                let shutdown = shutdown_signal.lock().unwrap();
                if *shutdown {
                    break;
                }
            }

            // Try to get a task from the ready queue
            let task = {
                let mut queue = ready_queue.lock().unwrap();
                if queue.is_empty() {
                    // Wait for tasks
                    let _queue = scheduler_condvar
                        .wait_timeout(queue, config.scheduling_quantum)
                        .unwrap().0;
                    queue.pop()
                } else {
                    queue.pop()
                }
            };

            if let Some(mut scheduler_task) = task {
                let start_time = Instant::now();

                // Execute the task (this would involve polling the actual Task)
                Self::execute_scheduler_task(&mut scheduler_task, &config, &goroutine_scheduler);

                let execution_time = start_time.elapsed();

                // Update statistics
                {
                    let mut stats = statistics.lock().unwrap();
                    stats.tasks_completed += 1;
                    stats.scheduler_overhead += execution_time;
                    
                    if stats.tasks_completed > 0 {
                        stats.average_task_time = stats.scheduler_overhead / stats.tasks_completed as u32;
                    }
                }
            } else {
                // Try work stealing
                Self::try_work_stealing(&work_stealing_queues, thread_id, &statistics);
            }

            // Coordinate with goroutine scheduler for GC safe points
            if let Some(ref goroutine_sched) = goroutine_scheduler {
                if config.goroutine_integration {
                    let _ = goroutine_sched.coordinate_gc(Duration::from_millis(1));
                }
            }
        }
    }

    /// Execute a scheduler task
    fn execute_scheduler_task(
        task: &mut SchedulerTask,
        _config: &SchedulerConfig,
        _goroutine_scheduler: &Option<Arc<GoroutineScheduler>>,
    ) {
        task.last_poll = Some(Instant::now());
        task.poll_count += 1;

        // Here we would actually poll the real Task
        // For now, we'll just simulate some work
        thread::sleep(Duration::from_micros(100));
    }

    /// Try to steal work from other workers
    fn try_work_stealing(
        work_stealing_queues: &Arc<Mutex<HashMap<ThreadId, WorkStealingQueue<SchedulerTask>>>>,
        current_thread: ThreadId,
        statistics: &Arc<Mutex<SchedulerStatistics>>,
    ) {
        let mut queues = work_stealing_queues.lock().unwrap();
        let mut stats = statistics.lock().unwrap();
        
        stats.work_steal_attempts += 1;
        
        // Try to steal from other threads
        for (thread_id, queue) in queues.iter_mut() {
            if *thread_id != current_thread && !queue.is_empty() {
                if let Some(_stolen_task) = queue.steal() {
                    stats.work_steal_successes += 1;
                    // Execute the stolen task
                    break;
                }
            }
        }
    }

    /// Integrate with goroutine safe points
    pub fn coordinate_with_goroutines(&self) -> Result<(), Error> {
        if let Some(ref scheduler) = self.goroutine_scheduler {
            scheduler.coordinate_gc(Duration::from_millis(10))
        } else {
            Ok(())
        }
    }
}

impl Drop for AsyncScheduler {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Create a scheduler with goroutine integration
pub fn create_integrated_scheduler(
    config: SchedulerConfig,
    goroutine_scheduler: Option<Arc<GoroutineScheduler>>,
) -> AsyncScheduler {
    let mut scheduler = AsyncScheduler::new(config);
    
    if let Some(goroutine_sched) = goroutine_scheduler {
        scheduler.set_goroutine_scheduler(goroutine_sched);
    }
    
    scheduler
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let config = SchedulerConfig::default();
        let scheduler = AsyncScheduler::new(config);
        
        let stats = scheduler.statistics();
        assert_eq!(stats.total_tasks_scheduled, 0);
    }

    #[test]
    fn test_priority_queue() {
        let mut queue = PriorityTaskQueue::new();
        
        let high_task = SchedulerTask {
            task_id: TaskId(1),
            priority: TaskPriority::High,
            created_at: Instant::now(),
            last_poll: None,
            poll_count: 0,
        };
        
        let low_task = SchedulerTask {
            task_id: TaskId(2),
            priority: TaskPriority::Low,
            created_at: Instant::now(),
            last_poll: None,
            poll_count: 0,
        };
        
        queue.push(low_task);
        queue.push(high_task);
        
        // High priority should come out first
        let first = queue.pop().unwrap();
        assert_eq!(first.task_id.as_u64(), 1);
    }

    #[test]
    fn test_work_stealing_queue() {
        let mut queue = WorkStealingQueue::new();
        
        queue.push_local(1);
        queue.push_local(2);
        queue.push_local(3);
        
        assert_eq!(queue.len(), 3);
        
        // Local pop should be LIFO
        assert_eq!(queue.pop_local(), Some(3));
        
        // Steal should be FIFO
        assert_eq!(queue.steal(), Some(1));
    }
}
