use crate::error::Error;
/// Enhanced Background Task Management for CURSED
/// 
/// This module provides comprehensive background task execution and management,
/// allowing processes to run asynchronously with full monitoring, control,
/// and status tracking capabilities.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, mpsc, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::process::{Child, ExitStatus};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, execution_failed, timeout_error, invalid_arguments
};

use crate::stdlib::process::enhanced_exec_slay::{SlayCommand, SlayProcess, SlayProcessState};
use crate::stdlib::process::real_monitoring::{ProcessStats, get_real_process_stats};

/// Background task manager for managing multiple tasks
pub type BackgroundTaskManager = SlayTaskManager;

/// Task handle for managing individual tasks
pub type TaskHandle = SlayTask;

/// SlayTask represents a background task with comprehensive status tracking
#[derive(Debug)]
pub struct SlayTask {
    /// Task ID
    pub id: u64,
    /// Original command
    pub command: SlayCommand,
    /// Task start time
    pub start_time: Instant,
    /// Task completion time
    pub completion_time: Option<Instant>,
    /// Exit code when finished
    pub exit_code: Option<i32>,
    /// Whether task has finished
    pub finished: bool,
    /// Task error if any
    pub error: Option<String>,
    /// Captured stdout output
    pub output: Vec<u8>,
    /// Captured stderr output
    pub stderr_output: Vec<u8>,
    /// Combined output (stdout + stderr)
    pub combined_output: Vec<u8>,
    /// Process handle
    process: Option<Arc<Mutex<Child>>>,
    /// Task state
    state: TaskState,
    /// Resource usage statistics
    stats: Arc<Mutex<Option<ProcessStats>>>,
    /// Output capture channels
    output_rx: Option<mpsc::Receiver<OutputChunk>>,
    /// Task configuration
    config: TaskConfig,
}

/// Task execution state
#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    Created,
    Starting,
    Running,
    Completed,
    Failed,
    Killed,
    Timeout,
}

/// Task configuration options
#[derive(Debug, Clone)]
pub struct TaskConfig {
    /// Capture output in real-time
    pub capture_output: bool,
    /// Maximum output buffer size
    pub max_output_size: usize,
    /// Task timeout
    pub timeout: Option<Duration>,
    /// Enable resource monitoring
    pub monitor_resources: bool,
    /// Monitoring interval
    pub monitor_interval: Duration,
    /// Auto-cleanup on completion
    pub auto_cleanup: bool,
    /// Priority level
    pub priority: TaskPriority,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            capture_output: true,
            max_output_size: 1024 * 1024, // 1MB
            timeout: None,
            monitor_resources: false,
            monitor_interval: Duration::from_secs(1),
            auto_cleanup: true,
            priority: TaskPriority::Normal,
        }
    }
}

/// Task priority levels
#[derive(Debug, Clone, PartialEq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Output chunk for real-time streaming
#[derive(Debug, Clone)]
pub struct OutputChunk {
    pub data: Vec<u8>,
    pub is_stderr: bool,
    pub timestamp: Instant,
}

impl SlayTask {
    /// Create a new task from a command
    pub fn new(id: u64, command: SlayCommand, config: TaskConfig) -> Self {
        Self {
            id,
            command,
            start_time: Instant::now(),
            completion_time: None,
            exit_code: None,
            finished: false,
            error: None,
            output: Vec::new(),
            stderr_output: Vec::new(),
            combined_output: Vec::new(),
            process: None,
            state: TaskState::Created,
            stats: Arc::new(Mutex::new(None)),
            output_rx: None,
            config,
        }
    }

    /// Start the background task
    pub fn start(&mut self) -> ProcessResult<()> {
        self.state = TaskState::Starting;
        self.start_time = Instant::now();
        
        // Configure the command for background execution
        let mut cmd = std::process::Command::new(&self.command.path);
        cmd.args(&self.command.args);
        
        // Set working directory
        if let Some(dir) = &self.command.dir {
            cmd.current_dir(dir);
        }
        
        // Set environment variables
        for env_pair in &self.command.env {
            if let Some((key, value)) = env_pair.split_once('=') {
                cmd.env(key, value);
            }
        }
        
        // Configure I/O based on capture settings
        if self.config.capture_output {
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
        } else {
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());
        }
        cmd.stdin(std::process::Stdio::null());
        
        // Spawn the process
        let mut child = cmd.spawn()
            .map_err(|e| execution_failed(&self.command.path, &e.to_string()))?;
        
        // Set up output capture if enabled
        if self.config.capture_output {
            self.setup_output_capture(&mut child)?;
        }
        
        self.process = Some(Arc::new(Mutex::new(child)));
        self.state = TaskState::Running;
        
        // Start resource monitoring if enabled
        if self.config.monitor_resources {
            self.start_resource_monitoring();
        }
        
        // Start timeout handling if configured
        if let Some(timeout) = self.config.timeout {
            self.start_timeout_handling(timeout);
        }
        
        Ok(())
    }

    /// Wait for task completion
    pub fn wait(&mut self) -> ProcessResult<()> {
        if let Some(process) = &self.process {
            if let Ok(mut child) = process.lock() {
                let status = child.wait()
                    .map_err(|e| ProcessError::IoError {
                        operation: "wait".to_string(),
                        error_type: format!("{:?}", e.kind()),
                        message: e.to_string(),
                    })?;
                
                self.completion_time = Some(Instant::now());
                self.exit_code = status.code();
                self.finished = true;
                
                if status.success() {
                    self.state = TaskState::Completed;
                } else {
                    self.state = TaskState::Failed;
                    self.error = Some(format!("Process exited with code {}", 
                        status.code().unwrap_or(-1)));
                }
                
                // Collect any remaining output
                self.collect_remaining_output();
            }
        }
        Ok(())
    }

    /// Kill the background task
    pub fn kill(&mut self) -> ProcessResult<()> {
        if let Some(process) = &self.process {
            if let Ok(mut child) = process.lock() {
                child.kill()
                    .map_err(|e| ProcessError::IoError {
                        operation: "kill".to_string(),
                        error_type: format!("{:?}", e.kind()),
                        message: e.to_string(),
                    })?;
                
                self.state = TaskState::Killed;
                self.finished = true;
                self.completion_time = Some(Instant::now());
            }
        }
        Ok(())
    }

    /// Check if task is still running
    pub fn is_running(&mut self) -> bool {
        if self.finished {
            return false;
        }
        
        if let Some(process) = &self.process {
            if let Ok(mut child) = process.lock() {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        // Process finished
                        self.completion_time = Some(Instant::now());
                        self.exit_code = status.code();
                        self.finished = true;
                        
                        if status.success() {
                            self.state = TaskState::Completed;
                        } else {
                            self.state = TaskState::Failed;
                        }
                        
                        false
                    }
                    Ok(None) => true, // Still running
                    Err(_) => {
                        // Error checking status, assume dead
                        self.finished = true;
                        self.state = TaskState::Failed;
                        false
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get elapsed time since task started
    pub fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get total execution time (if finished)
    pub fn execution_time(&self) -> Option<Duration> {
        self.completion_time.map(|end| end.duration_since(self.start_time))
    }

    /// Get captured output
    pub fn get_output(&self) -> ProcessResult<Vec<u8>> {
        Ok(self.output.clone())
    }

    /// Get captured stderr
    pub fn get_stderr(&self) -> ProcessResult<Vec<u8>> {
        Ok(self.stderr_output.clone())
    }

    /// Get combined output (stdout + stderr)
    pub fn get_combined_output(&self) -> ProcessResult<Vec<u8>> {
        Ok(self.combined_output.clone())
    }

    /// Get current resource usage statistics
    pub fn get_stats(&self) -> Option<ProcessStats> {
        if let Ok(stats_guard) = self.stats.lock() {
            stats_guard.clone()
        } else {
            None
        }
    }

    /// Get task state
    pub fn state(&self) -> TaskState {
        self.state.clone()
    }

    /// Get process ID if running
    pub fn pid(&self) -> Option<u32> {
        if let Some(process) = &self.process {
            if let Ok(child) = process.lock() {
                Some(child.id())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Set up output capture with real-time streaming
    fn setup_output_capture(&mut self, child: &mut Child) -> ProcessResult<()> {
        let (tx, rx) = mpsc::channel();
        self.output_rx = Some(rx);
        
        // Capture stdout
        if let Some(stdout) = child.stdout.take() {
            let tx_stdout = tx.clone();
            thread::spawn(move || {
                use std::io::Read;
                let mut reader = stdout;
                let mut buffer = [0; 1024];
                
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break, // EOF
                        Ok(n) => {
                            let chunk = OutputChunk {
                                data: buffer[..n].to_vec(),
                                is_stderr: false,
                                timestamp: Instant::now(),
                            };
                            if tx_stdout.send(chunk).is_err() {
                                break; // Receiver dropped
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }
        
        // Capture stderr
        if let Some(stderr) = child.stderr.take() {
            thread::spawn(move || {
                use std::io::Read;
                let mut reader = stderr;
                let mut buffer = [0; 1024];
                
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break, // EOF
                        Ok(n) => {
                            let chunk = OutputChunk {
                                data: buffer[..n].to_vec(),
                                is_stderr: true,
                                timestamp: Instant::now(),
                            };
                            if tx.send(chunk).is_err() {
                                break; // Receiver dropped
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }
        
        Ok(())
    }

    /// Start resource monitoring in background
    fn start_resource_monitoring(&self) {
        if let Some(pid) = self.pid() {
            let pid = pid;
            let interval = self.config.monitor_interval;
            let stats_ref = self.stats.clone();
            
            thread::spawn(move || {
                loop {
                    if let Ok(stats) = get_real_process_stats(pid) {
                        // Store stats in task
                        if let Ok(mut stats_guard) = stats_ref.lock() {
                            *stats_guard = Some(stats);
                        }
                        thread::sleep(interval);
                    } else {
                        break; // Process likely dead
                    }
                }
            });
        }
    }

    /// Start timeout handling
    fn start_timeout_handling(&self, timeout: Duration) {
        if let Some(process) = self.process.clone() {
            thread::spawn(move || {
                thread::sleep(timeout);
                
                // Check if process is still running and kill it
                if let Ok(mut child) = process.lock() {
                    if child.try_wait().map(|s| s.is_none()).unwrap_or(false) {
                        let _ = child.kill();
                    }
                }
            });
        }
    }

    /// Collect any remaining output from channels
    fn collect_remaining_output(&mut self) {
        if let Some(ref rx) = self.output_rx {
            while let Ok(chunk) = rx.try_recv() {
                if chunk.is_stderr {
                    self.stderr_output.extend(chunk.data.clone());
                } else {
                    self.output.extend(chunk.data.clone());
                }
                self.combined_output.extend(chunk.data);
            }
        }
    }
}

/// Background task manager for coordinating multiple tasks
pub struct TaskManager {
    /// Active tasks
    tasks: Arc<RwLock<HashMap<u64, Arc<Mutex<SlayTask>>>>>,
    /// Next task ID
    next_id: Arc<Mutex<u64>>,
    /// Task completion notification
    completion_notifier: Arc<(Mutex<Vec<u64>>, Condvar)>,
    /// Manager configuration
    config: ManagerConfig,
    /// Cleanup thread handle
    cleanup_thread: Option<thread::JoinHandle<()>>,
    /// Active flag
    active: Arc<Mutex<bool>>,
}

/// Task manager configuration
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: usize,
    /// Cleanup interval for finished tasks
    pub cleanup_interval: Duration,
    /// Default task timeout
    pub default_timeout: Option<Duration>,
    /// Enable automatic resource monitoring
    pub auto_monitor: bool,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 100,
            cleanup_interval: Duration::from_secs(60),
            default_timeout: None,
            auto_monitor: false,
        }
    }
}

impl TaskManager {
    /// Create a new task manager
    pub fn new(config: ManagerConfig) -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            completion_notifier: Arc::new((Mutex::new(Vec::new()), Condvar::new())),
            config,
            cleanup_thread: None,
            active: Arc::new(Mutex::new(true)),
        }
    }

    /// Start the task manager
    pub fn start(&mut self) -> ProcessResult<()> {
        let tasks = self.tasks.clone();
        let active = self.active.clone();
        let cleanup_interval = self.config.cleanup_interval;
        
        // Start cleanup thread
        let cleanup_handle = thread::spawn(move || {
            loop {
                thread::sleep(cleanup_interval);
                
                if let Ok(active_flag) = active.lock() {
                    if !*active_flag {
                        break;
                    }
                }
                
                // Clean up finished tasks
                if let Ok(mut task_map) = tasks.write() {
                    let mut to_remove = Vec::new();
                    
                    for (&id, task) in task_map.iter() {
                        if let Ok(task_guard) = task.lock() {
                            if task_guard.finished && task_guard.config.auto_cleanup {
                                to_remove.push(id);
                            }
                        }
                    }
                    
                    for id in to_remove {
                        task_map.remove(&id);
                    }
                }
            }
        });
        
        self.cleanup_thread = Some(cleanup_handle);
        Ok(())
    }

    /// Stop the task manager
    pub fn stop(&mut self) -> ProcessResult<()> {
        // Signal shutdown
        if let Ok(mut active) = self.active.lock() {
            *active = false;
        }
        
        // Wait for cleanup thread
        if let Some(handle) = self.cleanup_thread.take() {
            handle.join().map_err(|_| 
                ProcessError::System {
                    operation: "stop_manager".to_string(),
                    message: "Failed to join cleanup thread".to_string(),
                }
            )?;
        }
        
        // Kill all remaining tasks
        self.kill_all_tasks()?;
        
        Ok(())
    }

    /// Submit a task for background execution
    pub fn submit_task(&self, command: SlayCommand, config: Option<TaskConfig>) -> ProcessResult<u64> {
        // Check concurrent task limit
        if let Ok(tasks) = self.tasks.read() {
            let running_count = tasks.values()
                .filter_map(|task| task.lock().ok())
                .filter(|task| task.is_running())
                .count();
            
            if running_count >= self.config.max_concurrent_tasks {
                return Err(ProcessError::System {
                    operation: "submit_task".to_string(),
                    message: "Maximum concurrent tasks reached".to_string(),
                });
            }
        }
        
        // Get next task ID
        let id = if let Ok(mut next_id) = self.next_id.lock() {
            let id = *next_id;
            *next_id += 1;
            id
        } else {
            return Err(ProcessError::System {
                operation: "submit_task".to_string(),
                message: "Failed to generate task ID".to_string(),
            });
        };
        
        // Create task with merged configuration
        let mut task_config = config.unwrap_or_default();
        if task_config.timeout.is_none() {
            task_config.timeout = self.config.default_timeout;
        }
        if !task_config.monitor_resources && self.config.auto_monitor {
            task_config.monitor_resources = true;
        }
        
        let mut task = SlayTask::new(id, command, task_config);
        
        // Start the task
        task.start()?;
        
        // Store in manager
        if let Ok(mut tasks) = self.tasks.write() {
            tasks.insert(id, Arc::new(Mutex::new(task)));
        }
        
        Ok(id)
    }

    /// Get task by ID
    pub fn get_task(&self, id: u64) -> Option<Arc<Mutex<SlayTask>>> {
        if let Ok(tasks) = self.tasks.read() {
            tasks.get(&id).cloned()
        } else {
            None
        }
    }

    /// Wait for a specific task to complete
    pub fn wait_for_task(&self, id: u64) -> ProcessResult<()> {
        if let Some(task) = self.get_task(id) {
            if let Ok(mut task_guard) = task.lock() {
                task_guard.wait()?;
            }
        }
        Ok(())
    }

    /// Wait for all tasks to complete
    pub fn wait_for_all(&self) -> ProcessResult<()> {
        loop {
            let task_ids: Vec<u64> = if let Ok(tasks) = self.tasks.read() {
                tasks.keys().cloned().collect()
            } else {
                break;
            };
            
            if task_ids.is_empty() {
                break;
            }
            
            let mut all_finished = true;
            for id in task_ids {
                if let Some(task) = self.get_task(id) {
                    if let Ok(mut task_guard) = task.lock() {
                        if task_guard.is_running() {
                            all_finished = false;
                        }
                    }
                }
            }
            
            if all_finished {
                break;
            }
            
            thread::sleep(Duration::from_millis(100));
        }
        
        Ok(())
    }

    /// Kill a specific task
    pub fn kill_task(&self, id: u64) -> ProcessResult<()> {
        if let Some(task) = self.get_task(id) {
            if let Ok(mut task_guard) = task.lock() {
                task_guard.kill()?;
            }
        }
        Ok(())
    }

    /// Kill all tasks
    pub fn kill_all_tasks(&self) -> ProcessResult<()> {
        if let Ok(tasks) = self.tasks.read() {
            for task in tasks.values() {
                if let Ok(mut task_guard) = task.lock() {
                    let _ = task_guard.kill(); // Ignore errors
                }
            }
        }
        Ok(())
    }

    /// Get list of all task IDs
    pub fn list_tasks(&self) -> Vec<u64> {
        if let Ok(tasks) = self.tasks.read() {
            tasks.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get task statistics
    pub fn get_task_stats(&self) -> TaskManagerStats {
        if let Ok(tasks) = self.tasks.read() {
            let mut stats = TaskManagerStats::default();
            
            for task in tasks.values() {
                if let Ok(task_guard) = task.lock() {
                    match task_guard.state() {
                        TaskState::Running => stats.running_tasks += 1,
                        TaskState::Completed => stats.completed_tasks += 1,
                        TaskState::Failed => stats.failed_tasks += 1,
                        TaskState::Killed => stats.killed_tasks += 1,
                        _ => {}
                    }
                }
            }
            
            stats.total_tasks = tasks.len();
            stats
        } else {
            TaskManagerStats::default()
        }
    }
}

/// Task manager statistics
#[derive(Debug, Default)]
pub struct TaskManagerStats {
    pub total_tasks: usize,
    pub running_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub killed_tasks: usize,
}

/// Global task manager instance
static mut GLOBAL_TASK_MANAGER: Option<TaskManager> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Get or create the global task manager
pub fn get_global_task_manager() -> &'static mut TaskManager {
    unsafe {
        INIT.call_once(|| {
            let config = ManagerConfig::default();
            let mut manager = TaskManager::new(config);
            let _ = manager.start();
            GLOBAL_TASK_MANAGER = Some(manager);
        });
        
        GLOBAL_TASK_MANAGER.as_mut().unwrap()
    }
}

/// Convenience function to run a command in the background
pub fn run_background(command: SlayCommand) -> ProcessResult<u64> {
    let manager = get_global_task_manager();
    manager.submit_task(command, None)
}

/// Convenience function to run a command with specific configuration
pub fn run_background_with_config(command: SlayCommand, config: TaskConfig) -> ProcessResult<u64> {
    let manager = get_global_task_manager();
    manager.submit_task(command, Some(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::process::enhanced_exec_slay::SlayCommand;
use crate::stdlib::process::info::ProcessState;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_task_creation() {
        let command = SlayCommand::new("echo", &["test"]);
        let config = TaskConfig::default();
        let task = SlayTask::new(1, command, config);
        
        assert_eq!(task.id, 1);
        assert_eq!(task.state(), TaskState::Created);
        assert!(!task.finished);
    }

    #[test]
    fn test_task_manager_creation() {
        let config = ManagerConfig::default();
        let manager = TaskManager::new(config);
        
        assert_eq!(manager.list_tasks().len(), 0);
    }

    #[test]
    #[cfg(unix)]
    fn test_background_task_execution() {
        let command = SlayCommand::new("echo", &["background_test"]);
        let task_id = run_background(command).unwrap();
        
        let manager = get_global_task_manager();
        thread::sleep(Duration::from_millis(100)); // Give task time to complete
        
        if let Some(task) = manager.get_task(task_id) {
            if let Ok(task_guard) = task.lock() {
                // Task should have completed quickly
                assert!(task_guard.finished || !task_guard.is_running());
            }
        }
    }
}
