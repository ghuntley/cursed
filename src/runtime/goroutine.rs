// Goroutine runtime system for CURSED
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicUsize, Ordering}};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// Goroutine state
#[derive(Debug, Clone, PartialEq)]
pub enum GoroutineState {
    Created,
    Running,
    Waiting,
    Blocked,
    Finished,
}

/// Goroutine configuration
#[derive(Debug, Clone)]
pub struct GoroutineConfig {
    pub stack_size: usize,
    pub priority: u8,
    pub timeout: Option<Duration>,
    pub name: Option<String>,
}

impl Default for GoroutineConfig {
    fn default() -> Self {
        Self {
            stack_size: 1024 * 1024, // 1MB default stack
            priority: 128,            // Normal priority
            timeout: None,
            name: None,
        }
    }
}

/// Goroutine handle
#[derive(Debug)]
pub struct Goroutine {
    pub id: usize,
    pub state: GoroutineState,
    pub config: GoroutineConfig,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub result: Option<Result<(), String>>,
    handle: Option<JoinHandle<Result<(), String>>>,
}

impl Goroutine {
    pub fn new(id: usize, config: GoroutineConfig) -> Self {
        Self {
            id,
            state: GoroutineState::Created,
            config,
            start_time: Instant::now(),
            end_time: None,
            result: None,
            handle: None,
        }
    }
    
    pub fn is_finished(&self) -> bool {
        matches!(self.state, GoroutineState::Finished)
    }
    
    pub fn join(&mut self) -> Result<Result<(), String>, String> {
        if let Some(handle) = self.handle.take() {
            match handle.join() {
                Ok(result) => {
                    self.result = Some(result.clone());
                    self.state = GoroutineState::Finished;
                    self.end_time = Some(Instant::now());
                    Ok(result)
                }
                Err(_) => Err("Failed to join goroutine".to_string()),
            }
        } else {
            Err("Goroutine already joined or not started".to_string())
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.end_time.unwrap_or_else(Instant::now).duration_since(self.start_time)
    }
}

/// Scheduler configuration
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub max_threads: usize,
    pub max_goroutines: usize,
    pub preemption_interval: Duration,
    pub idle_timeout: Duration,
    pub enable_work_stealing: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            max_goroutines: 10_000,
            preemption_interval: Duration::from_millis(10),
            idle_timeout: Duration::from_secs(60),
            enable_work_stealing: true,
        }
    }
}

/// Work item for the scheduler
#[derive(Debug)]
pub struct Work {
    pub goroutine_id: usize,
    pub task: Box<dyn FnOnce() -> Result<(), String> + Send + 'static>,
    pub priority: u8,
}

impl Work {
    pub fn new<F>(id: usize, task: F, priority: u8) -> Self 
    where 
        F: FnOnce() -> Result<(), String> + Send + 'static 
    {
        Self {
            goroutine_id: id,
            task: Box::new(task),
            priority,
        }
    }
}

/// Goroutine scheduler
#[derive(Debug)]
pub struct GoroutineScheduler {
    config: SchedulerConfig,
    goroutines: Arc<Mutex<HashMap<usize, Goroutine>>>,
    work_queue: Arc<Mutex<VecDeque<Work>>>,
    next_id: AtomicUsize,
    running_count: AtomicUsize,
    threads: Vec<JoinHandle<()>>,
    shutdown: Arc<Mutex<bool>>,
    condition: Arc<Condvar>,
}

impl GoroutineScheduler {
    pub fn new() -> Self {
        Self::with_config(SchedulerConfig::default())
    }
    
    pub fn with_config(config: SchedulerConfig) -> Self {
        Self {
            config,
            goroutines: Arc::new(Mutex::new(HashMap::new())),
            work_queue: Arc::new(Mutex::new(VecDeque::new())),
            next_id: AtomicUsize::new(1),
            running_count: AtomicUsize::new(0),
            threads: Vec::new(),
            shutdown: Arc::new(Mutex::new(false)),
            condition: Arc::new(Condvar::new()),
        }
    }
    
    pub fn start(&mut self) {
        for i in 0..self.config.max_threads {
            let goroutines = Arc::clone(&self.goroutines);
            let work_queue = Arc::clone(&self.work_queue);
            let shutdown = Arc::clone(&self.shutdown);
            let condition = Arc::clone(&self.condition);
            let running_count = Arc::clone(&self.running_count);
            let thread_id = i;
            
            let handle = thread::spawn(move || {
                Self::worker_thread(thread_id, goroutines, work_queue, shutdown, condition, running_count);
            });
            
            self.threads.push(handle);
        }
    }
    
    pub fn spawn<F>(&self, task: F) -> Result<usize, String>
    where
        F: FnOnce() -> Result<(), String> + Send + 'static,
    {
        self.spawn_with_config(task, GoroutineConfig::default())
    }
    
    pub fn spawn_with_config<F>(&self, task: F, config: GoroutineConfig) -> Result<usize, String>
    where
        F: FnOnce() -> Result<(), String> + Send + 'static,
    {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        
        // Check limits
        if let Ok(goroutines) = self.goroutines.lock() {
            if goroutines.len() >= self.config.max_goroutines {
                return Err("Maximum goroutines limit reached".to_string());
            }
        }
        
        let goroutine = Goroutine::new(id, config.clone());
        
        // Store goroutine
        if let Ok(mut goroutines) = self.goroutines.lock() {
            goroutines.insert(id, goroutine);
        }
        
        // Queue work
        let work = Work::new(id, task, config.priority);
        if let Ok(mut queue) = self.work_queue.lock() {
            queue.push_back(work);
            self.condition.notify_one();
        }
        
        Ok(id)
    }
    
    pub fn join(&self, id: usize) -> Result<(), String> {
        loop {
            if let Ok(mut goroutines) = self.goroutines.lock() {
                if let Some(goroutine) = goroutines.get_mut(&id) {
                    if goroutine.is_finished() {
                        return Ok(());
                    }
                } else {
                    return Err("Goroutine not found".to_string());
                }
            }
            
            // Wait a bit before checking again
            thread::sleep(Duration::from_millis(1));
        }
    }
    
    pub fn shutdown(&mut self) {
        // Signal shutdown
        if let Ok(mut shutdown) = self.shutdown.lock() {
            *shutdown = true;
            self.condition.notify_all();
        }
        
        // Wait for all threads to finish
        for handle in self.threads.drain(..) {
            let _ = handle.join();
        }
    }
    
    pub fn status(&self) -> SchedulerStatus {
        let goroutines = if let Ok(goroutines) = self.goroutines.lock() {
            goroutines.len()
        } else {
            0
        };
        
        let queue_size = if let Ok(queue) = self.work_queue.lock() {
            queue.len()
        } else {
            0
        };
        
        SchedulerStatus {
            total_goroutines: goroutines,
            running_goroutines: self.running_count.load(Ordering::SeqCst),
            queued_work: queue_size,
            active_threads: self.threads.len(),
        }
    }
    
    fn worker_thread(
        _thread_id: usize,
        goroutines: Arc<Mutex<HashMap<usize, Goroutine>>>,
        work_queue: Arc<Mutex<VecDeque<Work>>>,
        shutdown: Arc<Mutex<bool>>,
        condition: Arc<Condvar>,
        running_count: Arc<AtomicUsize>,
    ) {
        loop {
            // Check for shutdown
            if let Ok(shutdown_flag) = shutdown.lock() {
                if *shutdown_flag {
                    break;
                }
            }
            
            // Get work from queue
            let work = {
                let mut queue = work_queue.lock().unwrap();
                if queue.is_empty() {
                    // Wait for work
                    let _guard = condition.wait_timeout(queue, Duration::from_millis(100)).unwrap();
                    if _guard.0.is_empty() {
                        continue;
                    }
                }
                queue.pop_front()
            };
            
            if let Some(work) = work {
                let goroutine_id = work.goroutine_id;
                running_count.fetch_add(1, Ordering::SeqCst);
                
                // Update goroutine state
                if let Ok(mut goroutines) = goroutines.lock() {
                    if let Some(goroutine) = goroutines.get_mut(&goroutine_id) {
                        goroutine.state = GoroutineState::Running;
                    }
                }
                
                // Execute task
                let result = (work.task)();
                
                // Update goroutine state
                if let Ok(mut goroutines) = goroutines.lock() {
                    if let Some(goroutine) = goroutines.get_mut(&goroutine_id) {
                        goroutine.state = GoroutineState::Finished;
                        goroutine.result = Some(result);
                        goroutine.end_time = Some(Instant::now());
                    }
                }
                
                running_count.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }
}

impl Default for GoroutineScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GoroutineScheduler {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Scheduler status information
#[derive(Debug, Clone)]
pub struct SchedulerStatus {
    pub total_goroutines: usize,
    pub running_goroutines: usize,
    pub queued_work: usize,
    pub active_threads: usize,
}

/// Safe point for goroutine switching
#[derive(Debug)]
pub struct SafePoint {
    pub location: String,
    pub can_preempt: bool,
}

impl SafePoint {
    pub fn new(location: String) -> Self {
        Self {
            location,
            can_preempt: true,
        }
    }
    
    pub fn no_preempt(location: String) -> Self {
        Self {
            location,
            can_preempt: false,
        }
    }
}

/// Global scheduler instance
static mut GLOBAL_SCHEDULER: Option<GoroutineScheduler> = None;
static SCHEDULER_INIT: std::sync::Once = std::sync::Once::new();

pub fn get_global_scheduler() -> Option<&'static mut GoroutineScheduler> {
    SCHEDULER_INIT.call_once(|| {
        unsafe {
            GLOBAL_SCHEDULER = Some(GoroutineScheduler::new());
            if let Some(ref mut scheduler) = GLOBAL_SCHEDULER {
                scheduler.start();
            }
        }
    });
    
    unsafe { GLOBAL_SCHEDULER.as_mut() }
}

pub fn init_scheduler() -> Result<(), String> {
    get_global_scheduler();
    Ok(())
}

pub fn spawn_goroutine<F>(task: F) -> Result<usize, String>
where
    F: FnOnce() -> Result<(), String> + Send + 'static,
{
    if let Some(scheduler) = get_global_scheduler() {
        scheduler.spawn(task)
    } else {
        Err("Scheduler not initialized".to_string())
    }
}

pub fn join_goroutine(id: usize) -> Result<(), String> {
    if let Some(scheduler) = get_global_scheduler() {
        scheduler.join(id)
    } else {
        Err("Scheduler not initialized".to_string())
    }
}

pub fn scheduler_status() -> Option<SchedulerStatus> {
    get_global_scheduler().map(|s| s.status())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_goroutine_creation() {
        let config = GoroutineConfig::default();
        let goroutine = Goroutine::new(1, config);
        
        assert_eq!(goroutine.id, 1);
        assert_eq!(goroutine.state, GoroutineState::Created);
    }
    
    #[test]
    fn test_scheduler_creation() {
        let scheduler = GoroutineScheduler::new();
        let status = scheduler.status();
        
        assert_eq!(status.total_goroutines, 0);
        assert_eq!(status.running_goroutines, 0);
        assert_eq!(status.queued_work, 0);
    }
    
    #[test]
    fn test_scheduler_config() {
        let config = SchedulerConfig {
            max_threads: 2,
            max_goroutines: 100,
            ..Default::default()
        };
        
        let scheduler = GoroutineScheduler::with_config(config);
        assert_eq!(scheduler.config.max_threads, 2);
        assert_eq!(scheduler.config.max_goroutines, 100);
    }
}
