// Process spawning utilities for CURSED
use super::core::{Process, ProcessId, ProcessManager};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::error::CursedError;

/// Process spawner configuration
#[derive(Debug, Clone)]
pub struct SpawnConfig {
    /// Working directory for the process
    /// Environment variables
    /// Whether to inherit parent environment
    /// Process timeout
    /// Whether to capture stdout
    /// Whether to capture stderr
    /// Process priority
/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessPriority {
impl Default for SpawnConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Process spawner for creating and managing processes
#[derive(Debug)]
pub struct ProcessSpawner {
impl ProcessSpawner {
    /// Create a new process spawner
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: SpawnConfig) -> Self {
        Self {
        }
    }
    
    /// Spawn a new process
    pub fn spawn<S: Into<String>>(
        args: Vec<String>
    ) -> crate::error::Result<ProcessId> {
        let mut process = Process::new(name.into(), command.into())
            .with_args(args);
        
        // Apply configuration
        if let Some(dir) = &self.config.working_dir {
            process = process.with_working_dir(dir.clone());
        if self.config.inherit_env {
            let mut env = std::env::vars().collect::<HashMap<_, _>>();
            env.extend(self.config.env.clone());
            process = process.with_env(env);
        } else {
            process = process.with_env(self.config.env.clone());
        self.manager.spawn_process(process)
    /// Spawn a simple command
    pub fn spawn_command<S: Into<String>>(
        command: S
    ) -> crate::error::Result<ProcessId> {
        let command_str = command.into();
        self.spawn(command_str.clone(), command_str, Vec::new())
    /// Spawn with timeout
    pub fn spawn_with_timeout<S: Into<String>>(
        timeout: std::time::Duration
    ) -> crate::error::Result<ProcessId> {
        let mut config = self.config.clone();
        config.timeout = Some(timeout);
        
        let old_config = std::mem::replace(&mut self.config, config);
        let result = self.spawn(name, command, args);
        self.config = old_config;
        
        result
    /// Set working directory
    pub fn set_working_dir(&mut self, dir: PathBuf) {
        self.config.working_dir = Some(dir);
    /// Add environment variable
    pub fn add_env<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.config.env.insert(key.into(), value.into());
    /// Set environment variables
    pub fn set_env(&mut self, env: HashMap<String, String>) {
        self.config.env = env;
    /// Enable/disable environment inheritance
    pub fn inherit_env(&mut self, inherit: bool) {
        self.config.inherit_env = inherit;
    /// Set process priority
    pub fn set_priority(&mut self, priority: ProcessPriority) {
        self.config.priority = priority;
    /// Get process by ID
    pub fn get_process(&self, id: ProcessId) -> Option<&Process> {
        self.manager.get_process(id)
    /// Kill process by ID
    pub fn kill_process(&mut self, id: ProcessId) -> crate::error::Result<()> {
        self.manager.kill_process(id)
    /// Wait for process completion
    pub fn wait_for_process(&mut self, id: ProcessId) -> crate::error::Result<i32> {
        self.manager.wait_for_process(id)
    /// List all processes
    pub fn list_processes(&self) -> Vec<ProcessId> {
        self.manager.list_processes()
    /// List running processes
    pub fn running_processes(&self) -> Vec<ProcessId> {
        self.manager.running_processes()
    /// Clean up finished processes
    pub fn cleanup(&mut self) {
        self.manager.cleanup_finished();
    /// Kill all processes
    pub fn kill_all(&mut self) -> crate::error::Result<()> {
        self.manager.kill_all()
    }
}

impl Default for ProcessSpawner {
    fn default() -> Self {
        Self::new()
    }
}

impl SpawnConfig {
    /// Create new spawn configuration
    pub fn new() -> Self {
        Self::default()
    /// Set working directory
    pub fn with_working_dir(mut self, dir: PathBuf) -> Self {
        self.working_dir = Some(dir);
        self
    /// Add environment variable
    pub fn with_env_var<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.env.insert(key.into(), value.into());
        self
    /// Set environment variables
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    /// Set environment inheritance
    pub fn with_inherit_env(mut self, inherit: bool) -> Self {
        self.inherit_env = inherit;
        self
    /// Set timeout
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    /// Enable stdout capture
    pub fn with_stdout_capture(mut self, capture: bool) -> Self {
        self.capture_stdout = capture;
        self
    /// Enable stderr capture
    pub fn with_stderr_capture(mut self, capture: bool) -> Self {
        self.capture_stderr = capture;
        self
    /// Set process priority
    pub fn with_priority(mut self, priority: ProcessPriority) -> Self {
        self.priority = priority;
        self
    }
}
