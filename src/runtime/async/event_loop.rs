/// Event loop implementation for async runtime
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, Condvar};
use std::task::{Context, Poll, Waker};
use std::thread::{self, ThreadId, JoinHandle};
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

use crate::runtime::r#async::{Future, TaskId, TaskHandle, TaskWaker, Timer};
use crate::error::Error;

/// Core event loop for async runtime
pub struct EventLoop {
    /// Unique ID for this event loop
    id: u64,
    /// Running flag
    running: AtomicBool,
    /// Queue of ready tasks
    ready_queue: Arc<Mutex<VecDeque<TaskId>>>,
    /// Map of pending futures
    pending_futures: Arc<Mutex<HashMap<TaskId, Box<dyn Future<Output = ()> + Send>>>>,
    /// Waker map for tasks
    wakers: Arc<Mutex<HashMap<TaskId, Waker>>>,
    /// Timer wheel for scheduled tasks
    timer_wheel: Arc<Mutex<Timer>>,
    /// Statistics
    stats: Arc<Mutex<EventLoopStats>>,
    /// Condition variable for blocking
    condvar: Arc<Condvar>,
}

/// Event loop statistics
#[derive(Debug, Clone)]
pub struct EventLoopStats {
    pub tasks_polled: u64,
    pub tasks_completed: u64,
    pub wake_ups: u64,
    pub timer_events: u64,
    pub loop_iterations: u64,
    pub total_runtime: Duration,
}

impl Default for EventLoopStats {
    fn default() -> Self {
        Self {
            tasks_polled: 0,
            tasks_completed: 0,
            wake_ups: 0,
            timer_events: 0,
            loop_iterations: 0,
            total_runtime: Duration::ZERO,
        }
    }
}

/// Event loop configuration
#[derive(Debug, Clone)]
pub struct EventLoopConfig {
    /// Maximum number of tasks to poll per iteration
    pub max_polls_per_iteration: usize,
    /// Sleep duration when no tasks are ready
    pub idle_sleep_duration: Duration,
    /// Enable detailed statistics collection
    pub collect_stats: bool,
}

impl Default for EventLoopConfig {
    fn default() -> Self {
        Self {
            max_polls_per_iteration: 100,
            idle_sleep_duration: Duration::from_millis(1),
            collect_stats: true,
        }
    }
}

impl EventLoop {
    /// Create a new event loop
    pub fn new(config: EventLoopConfig) -> Self {
        static LOOP_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        let id = LOOP_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            id,
            running: AtomicBool::new(false),
            ready_queue: Arc::new(Mutex::new(VecDeque::new())),
            pending_futures: Arc::new(Mutex::new(HashMap::new())),
            wakers: Arc::new(Mutex::new(HashMap::new())),
            timer_wheel: Arc::new(Mutex::new(Timer::new())),
            stats: Arc::new(Mutex::new(EventLoopStats::default())),
            condvar: Arc::new(Condvar::new()),
        }
    }

    /// Get the event loop ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Start the event loop
    pub fn start(&self) -> Result<(), Error> {
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Run the event loop for a single iteration
    pub fn run_once(&self, config: &EventLoopConfig) -> Result<bool, Error> {
        let start_time = Instant::now();
        let mut progress_made = false;

        // Update statistics
        if config.collect_stats {
            let mut stats = self.stats.lock().unwrap();
            stats.loop_iterations += 1;
        }

        // Process timer events
        self.process_timer_events()?;

        // Poll ready tasks
        let tasks_polled = self.poll_ready_tasks(config.max_polls_per_iteration)?;
        if tasks_polled > 0 {
            progress_made = true;
        }

        // Update runtime statistics
        if config.collect_stats {
            let mut stats = self.stats.lock().unwrap();
            stats.total_runtime += start_time.elapsed();
        }

        // If no progress was made, consider sleeping
        if !progress_made {
            if let Ok(ready_queue) = self.ready_queue.try_lock() {
                if ready_queue.is_empty() {
                    std::thread::sleep(config.idle_sleep_duration);
                }
            }
        }

        Ok(progress_made)
    }

    /// Run the event loop until shutdown
    pub fn run(&self, config: EventLoopConfig) -> Result<(), Error> {
        self.start()?;
        
        while self.running.load(Ordering::SeqCst) {
            self.run_once(&config)?;
        }

        Ok(())
    }

    /// Spawn a future on the event loop
    pub fn spawn<F>(&self, future: F) -> TaskId
    where
        F: Future<Output = ()> + Send + 'static,
    {
        static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        let task_id = TaskId(TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst));

        // Add to pending futures
        {
            let mut pending = self.pending_futures.lock().unwrap();
            pending.insert(task_id, Box::new(future));
        }

        // Add to ready queue
        {
            let mut ready = self.ready_queue.lock().unwrap();
            ready.push_back(task_id);
        }

        // Notify waiting threads
        self.condvar.notify_one();

        task_id
    }

    /// Schedule a task for later execution
    pub fn schedule_task(&self, task_id: TaskId, delay: Duration) -> Result<(), Error> {
        let mut timer = self.timer_wheel.lock().unwrap();
        timer.schedule_callback(delay, Box::new(move || {
            // This will be implemented to wake the task
        }))
    }

    /// Wake a specific task
    pub fn wake_task(&self, task_id: TaskId) {
        // Add to ready queue if not already there
        {
            let mut ready = self.ready_queue.lock().unwrap();
            if !ready.contains(&task_id) {
                ready.push_back(task_id);
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.wake_ups += 1;
        }

        // Notify waiting threads
        self.condvar.notify_one();
    }

    /// Remove a completed task
    pub fn remove_task(&self, task_id: TaskId) {
        {
            let mut pending = self.pending_futures.lock().unwrap();
            pending.remove(&task_id);
        }

        {
            let mut wakers = self.wakers.lock().unwrap();
            wakers.remove(&task_id);
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.tasks_completed += 1;
        }
    }

    /// Shutdown the event loop
    pub fn shutdown(&self) {
        self.running.store(false, Ordering::SeqCst);
        self.condvar.notify_all();
    }

    /// Check if the event loop is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get event loop statistics
    pub fn statistics(&self) -> EventLoopStats {
        self.stats.lock().unwrap().clone()
    }

    /// Process timer events
    fn process_timer_events(&self) -> Result<(), Error> {
        let mut timer = self.timer_wheel.lock().unwrap();
        let expired_count = timer.process_expired_timers();

        if expired_count > 0 {
            let mut stats = self.stats.lock().unwrap();
            stats.timer_events += expired_count as u64;
        }

        Ok(())
    }

    /// Poll ready tasks up to the specified limit
    fn poll_ready_tasks(&self, max_polls: usize) -> Result<usize, Error> {
        let mut tasks_polled = 0;

        for _ in 0..max_polls {
            // Get next ready task
            let task_id = {
                let mut ready = self.ready_queue.lock().unwrap();
                ready.pop_front()
            };

            let Some(task_id) = task_id else {
                break;
            };

            // Poll the task
            if self.poll_task(task_id)? {
                tasks_polled += 1;
            }
        }

        // Update statistics
        if tasks_polled > 0 {
            let mut stats = self.stats.lock().unwrap();
            stats.tasks_polled += tasks_polled as u64;
        }

        Ok(tasks_polled)
    }

    /// Poll a specific task
    fn poll_task(&self, task_id: TaskId) -> Result<bool, Error> {
        // Get the future for this task
        let future_opt = {
            let mut pending = self.pending_futures.lock().unwrap();
            pending.remove(&task_id)
        };

        let Some(mut future) = future_opt else {
            return Ok(false);
        };

        // Create waker for this task
        let waker = self.create_task_waker(task_id);
        let mut context = Context::from_waker(&waker);

        // Poll the future
        match future.as_mut().poll(&mut context) {
            Poll::Ready(_) => {
                // Task completed
                self.remove_task(task_id);
                Ok(true)
            }
            Poll::Pending => {
                // Task not ready, put it back
                let mut pending = self.pending_futures.lock().unwrap();
                pending.insert(task_id, future);
                Ok(true)
            }
        }
    }

    /// Create a waker for the given task
    fn create_task_waker(&self, task_id: TaskId) -> Waker {
        let event_loop = Arc::new(EventLoopWakerData {
            task_id,
            ready_queue: self.ready_queue.clone(),
            condvar: self.condvar.clone(),
            stats: self.stats.clone(),
        });

        TaskWaker::new(event_loop).into_waker()
    }

    /// Block until there are ready tasks or timeout
    pub fn wait_for_tasks(&self, timeout: Duration) -> Result<(), Error> {
        let ready_queue = self.ready_queue.lock().unwrap();
        let _guard = self.condvar.wait_timeout(ready_queue, timeout).unwrap();
        Ok(())
    }
}

/// Waker data for event loop integration
pub struct EventLoopWakerData {
    task_id: TaskId,
    ready_queue: Arc<Mutex<VecDeque<TaskId>>>,
    condvar: Arc<Condvar>,
    stats: Arc<Mutex<EventLoopStats>>,
}

impl EventLoopWakerData {
    /// Wake the associated task
    pub fn wake(&self) {
        // Add to ready queue
        {
            let mut ready = self.ready_queue.lock().unwrap();
            if !ready.contains(&self.task_id) {
                ready.push_back(self.task_id);
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.wake_ups += 1;
        }

        // Notify waiting threads
        self.condvar.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::r#async::future::ready;

    #[test]
    fn test_event_loop_creation() {
        let config = EventLoopConfig::default();
        let event_loop = EventLoop::new(config);
        
        assert!(!event_loop.is_running());
    }

    #[test]
    fn test_event_loop_start_stop() {
        let config = EventLoopConfig::default();
        let event_loop = EventLoop::new(config);
        
        assert!(event_loop.start().is_ok());
        assert!(event_loop.is_running());
        
        event_loop.shutdown();
        assert!(!event_loop.is_running());
    }

    #[test]
    fn test_spawn_task() {
        let config = EventLoopConfig::default();
        let event_loop = EventLoop::new(config);
        
        let task_id = event_loop.spawn(ready(()));
        assert!(task_id.0 > 0);
    }

    #[test]
    fn test_run_once() {
        let config = EventLoopConfig::default();
        let event_loop = EventLoop::new(config);
        
        event_loop.spawn(ready(()));
        
        let progress = event_loop.run_once(&config).unwrap();
        assert!(progress);
    }
}
