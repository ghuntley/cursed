// Stub implementations for process-related types to resolve compilation errors
// This provides minimal implementations while the full process system is being completed

use crate::error::CursedError;
use std::collections::HashMap;

/// Stub ProcessManager for process execution FFI
#[derive(Debug, Default)]
pub struct ProcessManager {
    pub processes: HashMap<String, ProcessHandle>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn spawn_process(&mut self, _cmd: &str) -> crate::error::Result<ProcessHandle> {
        Ok(ProcessHandle { id: 0 })
    }
}

/// Stub ProcessHandle
#[derive(Debug, Clone)]
pub struct ProcessHandle {
    pub id: u32,
}

/// Stub SlayCommand
#[derive(Debug, Default)]
pub struct SlayCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl SlayCommand {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            args: Vec::new(),
        }
    }
}

/// Stub SlayOptions
#[derive(Debug, Default)]
pub struct SlayOptions {
    pub timeout: Option<std::time::Duration>,
}

/// Stub Cmd type for exec_vibez
#[derive(Debug, Default)]
pub struct Cmd {
    pub command: String,
}

impl Cmd {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
        }
    }
}

/// Stub ProcessContext
#[derive(Debug, Default)]
pub struct ProcessContext {
    pub environment: HashMap<String, String>,
}

/// Stub SlayTask
#[derive(Debug, Default)]
pub struct SlayTask {
    pub name: String,
}

impl SlayTask {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Stub SlayPipeline
#[derive(Debug, Default)]
pub struct SlayPipeline {
    pub commands: Vec<SlayCommand>,
}

impl SlayPipeline {
    pub fn new() -> Self {
        Self::default()
    }
}
