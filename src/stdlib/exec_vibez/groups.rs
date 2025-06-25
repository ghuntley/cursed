use crate::error::CursedError;
/// Process group management for exec_vibez
/// 
/// Implements ProcessGroup functionality according to specs/stdlib/exec_vibez.md

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::thread;

use super::cmd::Cmd;
use super::process::{Process, ProcessState};
use super::error::{ExecResult, ExecError, execution_failed, invalid_arguments};

/// Global process group tracking
static ACTIVE_GROUPS: AtomicUsize = AtomicUsize::new(0);

/// Process group management options
#[derive(Debug, Clone)]
pub struct ProcessGroupOptions {
    /// Maximum number of concurrent processes
    /// Default timeout for all processes in the group
    /// Whether to stop all processes if one fails
    /// Whether to wait for all processes to complete
impl Default for ProcessGroupOptions {
    fn default() -> Self {
        Self {
        }
    }
/// Manages a group of related processes
#[derive(Debug)]
pub struct ProcessGroup {
    /// Group ID
    /// Commands to execute
    /// Running processes
    /// Process states for completed processes
    /// Group configuration
    /// Group start time
impl ProcessGroup {
    /// Create a new process group
    pub fn new() -> Self {
        let id = generate_group_id();
        ACTIVE_GROUPS.fetch_add(1, Ordering::Relaxed);
        
        Self {
        }
    }
    
    /// Create a new process group with options
    pub fn with_options(options: ProcessGroupOptions) -> Self {
        let mut group = Self::new();
        group.options = options;
        group
    /// Add a command to the group
    pub fn add_command(&mut self, cmd: Cmd) -> &mut Self {
        self.commands.push(cmd);
        self
    /// Set all commands for the group
    pub fn set_commands(&mut self, commands: Vec<Cmd>) -> &mut Self {
        self.commands = commands;
        self
    /// Start all processes in the group
    pub fn start_all(&mut self) -> ExecResult<()> {
        if self.commands.is_empty() {
            return Err(invalid_arguments("No commands in process group"));
        self.start_time = Some(Instant::now());
        
        // Apply concurrency limits if specified
        if let Some(max_concurrent) = self.options.max_concurrent {
            self.start_with_concurrency_limit(max_concurrent)
        } else {
            self.start_all_immediately()
        }
    }
    
    fn start_all_immediately(&mut self) -> ExecResult<()> {
        let mut processes = self.processes.lock().unwrap();
        
        for (index, mut cmd) in self.commands.iter_mut().enumerate() {
            // Apply default timeout if specified
            if let Some(timeout) = self.options.default_timeout {
                cmd.set_timeout(timeout);
            match cmd.start() {
                Ok(process) => {
                    processes.insert(index, process);
                }
                Err(e) => {
                    if self.options.fail_fast {
                        return Err(e);
                    }
                    tracing::warn!("Failed to start command {}: {}", index, e);
                }
            }
        Ok(())
    fn start_with_concurrency_limit(&mut self, max_concurrent: usize) -> ExecResult<()> {
        let mut running = 0;
        let mut command_index = 0;
        let mut processes = self.processes.lock().unwrap();
        
        while command_index < self.commands.len() || running > 0 {
            // Start new processes up to the limit
            while running < max_concurrent && command_index < self.commands.len() {
                if let Some(mut cmd) = self.commands.get_mut(command_index) {
                    // Apply default timeout if specified
                    if let Some(timeout) = self.options.default_timeout {
                        cmd.set_timeout(timeout);
                    match cmd.start() {
                        Ok(process) => {
                            processes.insert(command_index, process);
                            running += 1;
                            command_index += 1;
                        }
                        Err(e) => {
                            if self.options.fail_fast {
                                return Err(e);
                            }
                            tracing::warn!("Failed to start command {}: {}", command_index, e);
                            command_index += 1;
                        }
                    }
                } else {
                    break;
                }
            }
            
            // Check for completed processes
            let mut completed = Vec::new();
            for (&pid, process) in processes.iter() {
                if !process.is_running() {
                    completed.push(pid);
                }
            }
            
            // Remove completed processes
            for pid in completed {
                if let Some(process) = processes.remove(&pid) {
                    match process.wait() {
                        Ok(state) => {
                            self.states.lock().unwrap().insert(pid, state);
                        }
                        Err(e) => {
                            tracing::warn!("CursedError waiting for process {}: {}", pid, e);
                        }
                    }
                    running -= 1;
                }
            }
            
            // Small delay to avoid busy waiting
            if running > 0 {
                thread::sleep(Duration::from_millis(10));
            }
        }
        
        Ok(())
    /// Wait for all processes to complete
    pub fn wait_all(&mut self) -> ExecResult<Vec<ProcessState>> {
        if !self.options.wait_all {
            return Ok(Vec::new());
        let mut results = Vec::new();
        let processes = {
            let mut p = self.processes.lock().unwrap();
            std::mem::take(&mut *p)
        
        for (index, process) in processes {
            match process.wait() {
                Ok(state) => {
                    self.states.lock().unwrap().insert(index, state.clone());
                    results.push(state);
                }
                Err(e) => {
                    if self.options.fail_fast {
                        return Err(e);
                    }
                    tracing::warn!("Process {} failed: {}", index, e);
                }
            }
        Ok(results)
    /// Run all processes and wait for completion
    pub fn run(&mut self) -> ExecResult<Vec<ProcessState>> {
        self.start_all()?;
        self.wait_all()
    /// Kill all running processes
    pub fn kill_all(&self) -> ExecResult<()> {
        let processes = self.processes.lock().unwrap();
        
        for process in processes.values() {
            if let Err(e) = process.kill() {
                tracing::warn!("Failed to kill process {}: {}", process.pid(), e);
            }
        }
        
        Ok(())
    /// Get the number of running processes
    pub fn running_count(&self) -> usize {
        let processes = self.processes.lock().unwrap();
        processes.values().filter(|p| p.is_running()).count()
    /// Get the number of completed processes
    pub fn completed_count(&self) -> usize {
        self.states.lock().unwrap().len()
    /// Get all process states
    pub fn get_states(&self) -> Vec<ProcessState> {
        self.states.lock().unwrap().values().cloned().collect()
    /// Check if all processes have completed
    pub fn all_completed(&self) -> bool {
        self.running_count() == 0 && self.completed_count() == self.commands.len()
    /// Get group runtime
    pub fn runtime(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    /// Get group ID
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Drop for ProcessGroup {
    fn drop(&mut self) {
        ACTIVE_GROUPS.fetch_sub(1, Ordering::Relaxed);
        
        // Try to kill any remaining processes
        if let Err(e) = self.kill_all() {
            tracing::warn!("Failed to cleanup process group {}: {}", self.id, e);
        }
    }
/// Create a new process group
pub fn new_process_group() -> ProcessGroup {
    ProcessGroup::new()
/// Get the number of active process groups
pub fn get_active_group_count() -> usize {
    ACTIVE_GROUPS.load(Ordering::Relaxed)
/// Generate a unique group ID
fn generate_group_id() -> usize {
    use std::sync::atomic::AtomicUsize;
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
