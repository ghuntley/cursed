// Async process management for CURSED
use super::core::{ProcessId, ProcessStatus};
use std::process::Stdio;
use tokio::process::{Child, Command};
use crate::error::CursedError;

/// Async process handle
#[derive(Debug)]
pub struct AsyncProcessHandle {
impl AsyncProcessHandle {
    /// Create new async process handle
    pub fn new(id: ProcessId, child: Child, command: String) -> Self {
        Self {
        }
    }
    
    /// Wait for process completion
    pub async fn wait(&mut self) -> crate::error::Result<std::process::ExitStatus> {
        if let Some(mut child) = self.child.take() {
            let exit_status = child.wait().await?;
            let code = exit_status.code().unwrap_or(-1);
            self.status = ProcessStatus::Finished(code);
            Ok(exit_status)
        } else {
            Err(crate::error::CursedError::Runtime("Process already completed".to_string()))
        }
    }
    
    /// Kill the process
    pub fn kill(&mut self) -> crate::error::crate::error::Result<()> {
        if let Some(mut child) = self.child.take() {
            child.start_kill()?;
            self.status = ProcessStatus::Killed;
        }
        Ok(())
    /// Try to wait without blocking
    pub fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>> {
        if let Some(child) = &mut self.child {
            let result = child.try_wait()?;
            if let Some(exit_status) = result {
                let code = exit_status.code().unwrap_or(-1);
                self.status = ProcessStatus::Finished(code);
                self.child = None;
            }
            Ok(result)
        } else {
            Ok(None)
        }
    }
    
    /// Check if process is still running
    pub fn is_running(&mut self) -> bool {
        match self.try_wait() {
        }
    }
    
    /// Get process uptime
    pub fn uptime(&self) -> std::time::Duration {
        self.started_at.elapsed()
    /// Get process ID
    pub fn process_id(&self) -> Option<u32> {
        self.child.as_ref().and_then(|c| c.id())
    }
}

/// Async process for spawning and managing async processes
#[derive(Debug)]
pub struct AsyncProcess {
impl AsyncProcess {
    /// Create new async process
    pub fn new<S: Into<String>>(name: S, command: S) -> Self {
        Self {
        }
    }
    
    /// Add arguments
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    /// Add environment variables
    pub fn with_env(mut self, env: std::collections::HashMap<String, String>) -> Self {
        self.env = env;
        self
    /// Set working directory
    pub fn with_working_dir(mut self, dir: std::path::PathBuf) -> Self {
        self.working_dir = Some(dir);
        self
    /// Spawn the process
    pub async fn spawn(&mut self) -> crate::error::Result<ProcessId> {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);
        
        for (key, value) in &self.env {
            cmd.env(key, value);
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .stdin(Stdio::piped());
        
        let child = cmd.spawn()?;
        let id = ProcessId::new(child.id().unwrap_or(0));
        
        self.handle = Some(AsyncProcessHandle::new(id, child, self.command.clone()));
        Ok(id)
    /// Wait for process completion
    pub async fn wait(&mut self) -> crate::error::Result<i32> {
        if let Some(handle) = &mut self.handle {
            let exit_status = handle.wait().await?;
            Ok(exit_status.code().unwrap_or(-1))
        } else {
            Err(crate::error::CursedError::Runtime("Process not started".to_string()))
        }
    }
    
    /// Kill the process
    pub fn kill(&mut self) -> crate::error::Result<()> {
        if let Some(handle) = &mut self.handle {
            handle.kill()
        } else {
            Err(crate::error::CursedError::Runtime("Process not started".to_string()))
        }
    }
    
    /// Check if process is running
    pub fn is_running(&mut self) -> bool {
        self.handle.as_mut().map_or(false, |h| h.is_running())
    /// Get process handle
    pub fn handle(&self) -> Option<&AsyncProcessHandle> {
        self.handle.as_ref()
    /// Get mutable process handle
    pub fn handle_mut(&mut self) -> Option<&mut AsyncProcessHandle> {
        self.handle.as_mut()
    }
}
