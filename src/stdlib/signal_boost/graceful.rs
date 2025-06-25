use crate::error::CursedError;
/// Graceful shutdown coordination for SignalBoost
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use std::time::{Duration, Instant};

use super::core::BoostSignal;
use super::error::{SignalBoostError, SignalBoostResult};

/// Options for graceful shutdown configuration
#[derive(Debug, Clone)]
pub struct ShutdownOptions {
    /// Maximum time to wait for shutdown to complete
    pub timeout: Duration,
    /// Function to call before starting shutdown
    pub pre_shutdown_fn: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Function to handle shutdown errors
    pub error_handler: Option<Arc<dyn Fn(SignalBoostError) + Send + Sync>>,
    /// Whether to keep the application alive during shutdown
    pub keep_alive: bool,
    /// Whether shutdown should be synchronous
    pub sync_shutdown: bool,
    /// Signals that trigger shutdown
    pub signals: Vec<BoostSignal>,
}

impl Default for ShutdownOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            pre_shutdown_fn: None,
            error_handler: None,
            keep_alive: false,
            sync_shutdown: true,
            signals: vec![super::core::SIGINT, super::core::SIGTERM],
        }
    }
}

/// Status of graceful shutdown process
#[derive(Debug, Clone)]
pub struct ShutdownStatus {
    /// Whether shutdown is currently in progress
    pub in_progress: bool,
    /// Time elapsed since shutdown started
    pub elapsed_time: Duration,
    /// Tasks that have completed
    pub completed_tasks: Vec<String>,
    /// Tasks that are still running
    pub remaining_tasks: Vec<String>,
    /// Errors that occurred during shutdown
    pub errors: HashMap<String, String>,
    /// Signal that triggered the shutdown
    pub shutdown_triggered_by: Option<BoostSignal>,
}

/// A shutdown task
pub type ShutdownTask = Box<dyn Fn() -> SignalBoostResult<()> + Send + Sync>;

/// A group of shutdown tasks with ordering
#[derive(Debug)]
pub struct ShutdownTaskGroup {
    /// Name of the task group
    pub name: String,
    /// Priority/order of execution (lower numbers execute first)
    pub order: i32,
    /// Tasks in this group
    pub tasks: Vec<ShutdownTask>,
}

/// Graceful shutdown coordinator
pub struct GracefulShutdown {
    /// Shutdown options
    options: ShutdownOptions,
    /// Shutdown tasks by name
    tasks: Arc<Mutex<HashMap<String, (i32, ShutdownTask)>>>,
    /// Task groups
    task_groups: Arc<Mutex<Vec<ShutdownTaskGroup>>>,
    /// Current status
    status: Arc<RwLock<ShutdownStatus>>,
    /// Whether shutdown has been started
    started: AtomicBool,
    /// Signal that triggered shutdown
    trigger_signal: Arc<Mutex<Option<BoostSignal>>>,
    /// Shutdown start time
    start_time: Arc<Mutex<Option<Instant>>>,
    /// Logger for shutdown events
    logger: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl GracefulShutdown {
    /// Create a new graceful shutdown coordinator
    pub fn new() -> Self {
        Self {
            options: ShutdownOptions::default(),
            tasks: Arc::new(Mutex::new(HashMap::new())),
            task_groups: Arc::new(Mutex::new(Vec::new())),
            status: Arc::new(RwLock::new(ShutdownStatus {
                in_progress: false,
                elapsed_time: Duration::from_secs(0),
                completed_tasks: Vec::new(),
                remaining_tasks: Vec::new(),
                errors: HashMap::new(),
                shutdown_triggered_by: None,
            })),
            started: AtomicBool::new(false),
            trigger_signal: Arc::new(Mutex::new(None)),
            start_time: Arc::new(Mutex::new(None)),
            logger: None,
        }
    }

    /// Configure shutdown options
    pub fn with_options(mut self, options: ShutdownOptions) -> Self {
        self.options = options;
        self
    }

    /// Add a shutdown task
    pub fn add<F>(&mut self, name: &str, task: F) -> &mut Self
    where
        F: Fn() -> SignalBoostResult<()> + Send + Sync + 'static,
    {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(name.to_string(), (0, Box::new(task)));
        self
    }

    /// Add a shutdown task with specific execution order
    pub fn add_with_order<F>(&mut self, name: &str, order: i32, task: F) -> &mut Self
    where
        F: Fn() -> SignalBoostResult<()> + Send + Sync + 'static,
    {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(name.to_string(), (order, Box::new(task)));
        self
    }

    /// Add a group of related shutdown tasks
    pub fn add_group<F>(&mut self, name: &str, tasks: Vec<F>) -> &mut Self
    where
        F: Fn() -> SignalBoostResult<()> + Send + Sync + 'static,
    {
        let task_group = ShutdownTaskGroup {
            name: name.to_string(),
            order: 0,
            tasks: tasks.into_iter().map(|f| Box::new(f) as ShutdownTask).collect(),
        };
        
        let mut groups = self.task_groups.lock().unwrap();
        groups.push(task_group);
        self
    }

    /// Set a logger for shutdown events
    pub fn set_logger<F>(&mut self, logger: F) -> &mut Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.logger = Some(Arc::new(logger));
        self
    }

    /// Set shutdown timeout
    pub fn set_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.options.timeout = timeout;
        self
    }

    /// Start listening for shutdown signals
    pub fn start(&mut self) -> SignalBoostResult<()> {
        if self.started.load(Ordering::SeqCst) {
            return Err(SignalBoostError::InvalidState("Already started".to_string()));
        }

        self.started.store(true, Ordering::SeqCst);

        // Set up signal handling
        let (receiver, _handle) = super::core::notify(&self.options.signals)?;
        let shutdown_ref = Arc::new(Mutex::new(self as *mut GracefulShutdown));
        let trigger_signal = self.trigger_signal.clone();
        let logger = self.logger.clone();

        // Spawn signal handling thread
        thread::spawn(move || {
            if let Some(ref log) = logger {
                log("Graceful shutdown signal handler started");
            }

            // Wait for shutdown signal
            if let Ok(signal) = receiver.recv() {
                if let Some(ref log) = logger {
                    log(&format!("Received shutdown signal: {}", signal));
                }

                // Record trigger signal
                *trigger_signal.lock().unwrap() = Some(signal);

                // Trigger shutdown
                unsafe {
                    if let Ok(shutdown_ptr) = shutdown_ref.lock() {
                        if !shutdown_ptr.is_null() {
                            let shutdown = &mut **shutdown_ptr;
                            let _ = shutdown.shutdown();
                        }
                    }
                }
            }
        });

        if let Some(ref log) = self.logger {
            log("Graceful shutdown system started");
        }

        Ok(())
    }

    /// Manually trigger shutdown
    pub fn shutdown(&mut self) -> SignalBoostResult<()> {
        if !self.started.load(Ordering::SeqCst) {
            return Err(SignalBoostError::InvalidState("Not started".to_string()));
        }

        // Update status
        {
            let mut status = self.status.write().unwrap();
            if status.in_progress {
                return Ok(()); // Already shutting down
            }
            status.in_progress = true;
            status.shutdown_triggered_by = *self.trigger_signal.lock().unwrap();
        }

        *self.start_time.lock().unwrap() = Some(Instant::now());

        if let Some(ref log) = self.logger {
            log("Starting graceful shutdown");
        }

        // Call pre-shutdown function
        if let Some(ref pre_shutdown) = self.options.pre_shutdown_fn {
            pre_shutdown();
        }

        // Execute shutdown in background if async
        if !self.options.sync_shutdown {
            let tasks = self.tasks.clone();
            let task_groups = self.task_groups.clone();
            let status = self.status.clone();
            let start_time = self.start_time.clone();
            let options = self.options.clone();
            let logger = self.logger.clone();

            thread::spawn(move || {
                Self::execute_shutdown_tasks(tasks, task_groups, status, start_time, options, logger);
            });
        } else {
            Self::execute_shutdown_tasks(
                self.tasks.clone(),
                self.task_groups.clone(),
                self.status.clone(),
                self.start_time.clone(),
                self.options.clone(),
                self.logger.clone(),
            );
        }

        Ok(())
    }

    /// Wait for shutdown to complete
    pub fn wait(&self) -> SignalBoostResult<()> {
        let timeout = self.options.timeout;
        let start = Instant::now();

        loop {
            {
                let status = self.status.read().unwrap();
                if !status.in_progress {
                    break;
                }
            }

            if start.elapsed() >= timeout {
                return Err(SignalBoostError::Timeout(format!(
                    "Shutdown did not complete within {:?}",
                    timeout
                )));
            }

            thread::sleep(Duration::from_millis(10));
        }

        Ok(())
    }

    /// Get current shutdown status
    pub fn status(&self) -> ShutdownStatus {
        let mut status = self.status.read().unwrap().clone();
        
        // Update elapsed time
        if let Some(start_time) = *self.start_time.lock().unwrap() {
            status.elapsed_time = start_time.elapsed();
        }

        status
    }

    /// Execute shutdown tasks
    fn execute_shutdown_tasks(
        tasks: Arc<Mutex<HashMap<String, (i32, ShutdownTask)>>>,
        task_groups: Arc<Mutex<Vec<ShutdownTaskGroup>>>,
        status: Arc<RwLock<ShutdownStatus>>,
        start_time: Arc<Mutex<Option<Instant>>>,
        options: ShutdownOptions,
        logger: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    ) {
        let shutdown_start = start_time.lock().unwrap().unwrap_or_else(Instant::now);

        // Collect all tasks with their orders
        let mut all_tasks = Vec::new();

        // Add individual tasks
        {
            let tasks_map = tasks.lock().unwrap();
            for (name, (order, _)) in tasks_map.iter() {
                all_tasks.push((name.clone(), *order));
            }
        }

        // Add group tasks
        {
            let groups = task_groups.lock().unwrap();
            for group in groups.iter() {
                for (i, _) in group.tasks.iter().enumerate() {
                    let task_name = format!("{}[{}]", group.name, i);
                    all_tasks.push((task_name, group.order));
                }
            }
        }

        // Sort tasks by order
        all_tasks.sort_by_key(|(_, order)| *order);

        // Update status with remaining tasks
        {
            let mut status_guard = status.write().unwrap();
            status_guard.remaining_tasks = all_tasks.iter().map(|(name, _)| name.clone()).collect();
        }

        // Execute tasks in order
        for (task_name, _order) in all_tasks {
            // Check timeout
            if shutdown_start.elapsed() >= options.timeout {
                if let Some(ref log) = logger {
                    log(&format!("Shutdown timeout reached, stopping execution"));
                }
                break;
            }

            if let Some(ref log) = logger {
                log(&format!("Executing shutdown task: {}", task_name));
            }

            // Execute the task
            let result = if task_name.contains('[') {
                // Group task
                let parts: Vec<&str> = task_name.split('[').collect();
                let group_name = parts[0];
                let index: usize = parts[1].trim_end_matches(']').parse().unwrap_or(0);

                let groups = task_groups.lock().unwrap();
                if let Some(group) = groups.iter().find(|g| g.name == group_name) {
                    if let Some(task) = group.tasks.get(index) {
                        task()
                    } else {
                        Err(SignalBoostError::InvalidState(format!("Task index {} not found in group {}", index, group_name)))
                    }
                } else {
                    Err(SignalBoostError::InvalidState(format!("Task group {} not found", group_name)))
                }
            } else {
                // Individual task
                let tasks_map = tasks.lock().unwrap();
                if let Some((_, task)) = tasks_map.get(&task_name) {
                    task()
                } else {
                    Err(SignalBoostError::InvalidState(format!("Task {} not found", task_name)))
                }
            };

            // Update status
            {
                let mut status_guard = status.write().unwrap();
                status_guard.remaining_tasks.retain(|name| name != &task_name);
                
                match result {
                    Ok(()) => {
                        status_guard.completed_tasks.push(task_name.clone());
                        if let Some(ref log) = logger {
                            log(&format!("Completed shutdown task: {}", task_name));
                        }
                    }
                    Err(e) => {
                        status_guard.errors.insert(task_name.clone(), e.to_string());
                        if let Some(ref log) = logger {
                            log(&format!("Failed shutdown task {}: {}", task_name, e));
                        }
                        if let Some(ref error_handler) = options.error_handler {
                            error_handler(e);
                        }
                    }
                }
            }
        }

        // Mark shutdown complete
        {
            let mut status_guard = status.write().unwrap();
            status_guard.in_progress = false;
            status_guard.elapsed_time = shutdown_start.elapsed();
        }

        if let Some(ref log) = logger {
            log("Graceful shutdown completed");
        }
    }
}

// Global state for tracking active graceful shutdowns
static ACTIVE_GRACEFUL_SHUTDOWNS: AtomicUsize = AtomicUsize::new(0);

/// Get the number of active graceful shutdown coordinators
pub fn get_active_count() -> usize {
    ACTIVE_GRACEFUL_SHUTDOWNS.load(Ordering::SeqCst)
}

impl Drop for GracefulShutdown {
    fn drop(&mut self) {
        ACTIVE_GRACEFUL_SHUTDOWNS.fetch_sub(1, Ordering::SeqCst);
    }
}

impl Default for GracefulShutdown {
    fn default() -> Self {
        ACTIVE_GRACEFUL_SHUTDOWNS.fetch_add(1, Ordering::SeqCst);
        Self::new()
    }
}

