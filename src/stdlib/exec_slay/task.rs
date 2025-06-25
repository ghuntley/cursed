use crate::error::CursedError;
// SlayTask implementation for background task management

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use super::{SlayCommand, SlayResult, SharedProcessState};

/// Represents a background task running a command
#[derive(Debug)]
pub struct SlayTask {
    /// The command being executed
    pub command: SlayCommand,
    /// Task start time
    pub start_time: Instant,
    /// Shared state with the running process
    state: Arc<Mutex<TaskState>>,
    /// Background thread handle
    thread_handle: Option<thread::JoinHandle<SlayResult<()>>>,
}

/// Internal state for background tasks
#[derive(Debug)]
struct TaskState {
    /// Whether the task is finished
    finished: bool,
    /// Exit code when finished
    exit_code: Option<i32>,
    /// CursedError message if any
    error: Option<String>,
    /// Captured output
    output: Vec<u8>,
    /// Combined output (stdout + stderr)
    combined_output: Vec<u8>,
}

impl TaskState {
    fn new() -> Self {
        Self {
            finished: false,
            exit_code: None,
            error: None,
            output: Vec::new(),
            combined_output: Vec::new(),
        }
    }
}

impl SlayTask {
    /// Create and start a new background task
    pub fn run_background(mut command: SlayCommand) -> Self {
        let state = Arc::new(Mutex::new(TaskState::new()));
        let state_clone = state.clone();
        let start_time = Instant::now();

        // Spawn background thread
        let thread_handle = thread::spawn(move || {
            let result = command.run();
            
            let mut task_state = state_clone.lock().unwrap();
            task_state.finished = true;
            
            match result {
                Ok(()) => {
                    task_state.exit_code = command.exit_code();
                    
                    // Collect output
                    if let Ok(output) = command.output() {
                        task_state.output = output;
                    }
                    
                    if let Ok(combined) = command.combined_output() {
                        task_state.combined_output = combined;
                    }
                }
                Err(e) => {
                    task_state.error = Some(e.to_string());
                    task_state.exit_code = Some(-1);
                }
            }
            
            result
        });

        Self {
            command,
            start_time,
            state,
            thread_handle: Some(thread_handle),
        }
    }

    /// Wait for the background task to complete
    pub fn wait(&mut self) -> SlayResult<()> {
        if let Some(handle) = self.thread_handle.take() {
            match handle.join() {
                Ok(result) => result,
                Err(_) => Err(CursedError::RuntimeError(
                    "Background task thread panicked".to_string()
                )),
            }
        } else {
            // Already waited or not started
            let state = self.state.lock().unwrap();
            if let Some(ref error) = state.error {
                Err(CursedError::RuntimeError(error.clone()))
            } else {
                Ok(())
            }
        }
    }

    /// Kill the background task
    pub fn kill(&mut self) -> SlayResult<()> {
        // Kill the underlying process
        if let Some(process) = self.command.process() {
            process.kill()?;
        }

        // Mark as finished with error
        let mut state = self.state.lock().unwrap();
        state.finished = true;
        state.exit_code = Some(-1);
        state.error = Some("Task was killed".to_string());

        Ok(())
    }

    /// Check if the task is still running
    pub fn is_running(&self) -> bool {
        let state = self.state.lock().unwrap();
        !state.finished && self.command.is_running()
    }

    /// Get the elapsed time since task start
    pub fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get the task's exit code if finished
    pub fn exit_code(&self) -> Option<i32> {
        let state = self.state.lock().unwrap();
        state.exit_code
    }

    /// Check if the task finished successfully
    pub fn is_finished(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.finished
    }

    /// Get the error message if any
    pub fn error(&self) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.error.clone()
    }

    /// Get the task's output (stdout only)
    pub fn get_output(&self) -> SlayResult<Vec<u8>> {
        let state = self.state.lock().unwrap();
        
        if !state.finished {
            return Err(CursedError::RuntimeError(
                "Task not finished yet".to_string()
            ));
        }
        
        Ok(state.output.clone())
    }

    /// Get the task's combined output (stdout + stderr)
    pub fn get_combined_output(&self) -> SlayResult<Vec<u8>> {
        let state = self.state.lock().unwrap();
        
        if !state.finished {
            return Err(CursedError::RuntimeError(
                "Task not finished yet".to_string()
            ));
        }
        
        Ok(state.combined_output.clone())
    }

    /// Wait for the task with a timeout
    pub fn wait_timeout(&mut self, timeout: Duration) -> SlayResult<bool> {
        let start = Instant::now();
        
        while start.elapsed() < timeout {
            if self.is_finished() {
                return self.wait().map(|_| true);
            }
            thread::sleep(Duration::from_millis(10));
        }
        
        // Timeout reached
        Ok(false)
    }

    /// Get a status summary of the task
    pub fn status(&self) -> TaskStatus {
        let state = self.state.lock().unwrap();
        
        if !state.finished {
            TaskStatus::Running
        } else if let Some(ref error) = state.error {
            TaskStatus::Failed(error.clone())
        } else if let Some(code) = state.exit_code {
            if code == 0 {
                TaskStatus::Completed
            } else {
                TaskStatus::Failed(format!("Exit code: {}", code))
            }
        } else {
            TaskStatus::Unknown
        }
    }

    /// Get the command string representation
    pub fn command_string(&self) -> String {
        self.command.to_string()
    }
}

/// Status of a background task
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    /// Task is currently running
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed with error message
    Failed(String),
    /// Task status is unknown
    Unknown,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Running => write!(f, "Running"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed(msg) => write!(f, "Failed: {}", msg),
            TaskStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Convenience function to run a command in the background
pub fn run_background(command: SlayCommand) -> SlayTask {
    SlayTask::run_background(command)
}

/// Task manager for handling multiple background tasks
#[derive(Debug)]
pub struct SlayTaskManager {
    /// Active tasks
    tasks: Vec<SlayTask>,
}

impl SlayTaskManager {
    /// Create a new task manager
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    /// Add a task to the manager
    pub fn add_task(&mut self, task: SlayTask) -> usize {
        self.tasks.push(task);
        self.tasks.len() - 1
    }

    /// Run a command as a background task and add it to the manager
    pub fn run_background(&mut self, command: SlayCommand) -> usize {
        let task = SlayTask::run_background(command);
        self.add_task(task)
    }

    /// Get a task by index
    pub fn get_task(&mut self, index: usize) -> Option<&mut SlayTask> {
        self.tasks.get_mut(index)
    }

    /// Wait for all tasks to complete
    pub fn wait_all(&mut self) -> SlayResult<()> {
        for task in &mut self.tasks {
            task.wait()?;
        }
        Ok(())
    }

    /// Kill all running tasks
    pub fn kill_all(&mut self) -> SlayResult<()> {
        for task in &mut self.tasks {
            if task.is_running() {
                task.kill()?;
            }
        }
        Ok(())
    }

    /// Get the number of running tasks
    pub fn running_count(&self) -> usize {
        self.tasks.iter().filter(|task| task.is_running()).count()
    }

    /// Get the total number of tasks
    pub fn total_count(&self) -> usize {
        self.tasks.len()
    }

    /// Remove finished tasks from the manager
    pub fn cleanup_finished(&mut self) {
        self.tasks.retain(|task| task.is_running());
    }

    /// Get status of all tasks
    pub fn status_summary(&self) -> Vec<(usize, TaskStatus, String)> {
        self.tasks
            .iter()
            .enumerate()
            .map(|(i, task)| (i, task.status(), task.command_string()))
            .collect()
    }
}

impl Default for SlayTaskManager {
    fn default() -> Self {
        Self::new()
    }
}

