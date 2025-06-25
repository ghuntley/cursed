// Core process management types and functionality
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use crate::error::CursedError;

/// Process identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u32);

impl ProcessId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Process handle for managing running processes
#[derive(Debug)]
pub struct ProcessHandle {
    pub id: ProcessId,
    pub child: Arc<Mutex<Child>>,
    pub command: String,
    pub started_at: std::time::Instant,
}

impl ProcessHandle {
    pub fn new(id: ProcessId, child: Child, command: String) -> Self {
        Self {
            id,
            child: Arc::new(Mutex::new(child)),
            command,
            started_at: std::time::Instant::now(),
        }
    }
    
    pub fn is_running(&self) -> bool {
        if let Ok(mut child) = self.child.lock() {
            child.try_wait().unwrap_or(None).is_none()
        } else {
            false
        }
    }
    
    pub fn kill(&self) -> Result<(), std::io::Error> {
        if let Ok(mut child) = self.child.lock() {
            child.kill()
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to acquire process lock"
            ))
        }
    }
    
    pub fn wait(&self) -> Result<std::process::ExitStatus, std::io::Error> {
        if let Ok(mut child) = self.child.lock() {
            child.wait()
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to acquire process lock"
            ))
        }
    }
    
    pub fn uptime(&self) -> std::time::Duration {
        self.started_at.elapsed()
    }
}

/// Process representation
#[derive(Debug)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub working_dir: Option<std::path::PathBuf>,
    pub handle: Option<ProcessHandle>,
    pub status: ProcessStatus,
}

/// Process status
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    NotStarted,
    Running,
    Finished(i32),
    Killed,
    Failed(String),
}

impl Process {
    pub fn new(name: String, command: String) -> Self {
        Self {
            id: ProcessId::new(0), // Will be set when started
            name,
            command,
            args: Vec::new(),
            env: HashMap::new(),
            working_dir: None,
            handle: None,
            status: ProcessStatus::NotStarted,
        }
    }
    
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
    
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }
    
    pub fn with_working_dir(mut self, dir: std::path::PathBuf) -> Self {
        self.working_dir = Some(dir);
        self
    }
    
    pub fn start(&mut self) -> crate::error::Result<()> {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);
        
        for (key, value) in &self.env {
            cmd.env(key, value);
        }
        
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        
        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .stdin(Stdio::piped());
        
        match cmd.spawn() {
            Ok(child) => {
                let id = ProcessId::new(child.id());
                self.id = id;
                self.handle = Some(ProcessHandle::new(id, child, self.command.clone()));
                self.status = ProcessStatus::Running;
                Ok(())
            },
            Err(e) => {
                self.status = ProcessStatus::Failed(e.to_string());
                Err(crate::error::CursedError::Runtime(format!("Failed to start process: {}", e)))
            }
        }
    }
    
    pub fn is_running(&self) -> bool {
        matches!(self.status, ProcessStatus::Running) &&
        self.handle.as_ref().map_or(false, |h| h.is_running())
    }
    
    pub fn kill(&mut self) -> crate::error::crate::error::Result<()> {
        if let Some(handle) = &self.handle {
            handle.kill()?;
            self.status = ProcessStatus::Killed;
        }
        Ok(())
    }
    
    pub fn wait(&mut self) -> Result<i32> {
        if let Some(handle) = &self.handle {
            let exit_status = handle.wait()?;
            let code = exit_status.code().unwrap_or(-1);
            self.status = ProcessStatus::Finished(code);
            Ok(code)
        } else {
            Err(crate::error::CursedError::Runtime("No process handle available".to_string()))
        }
    }
}

/// Process manager for handling multiple processes
#[derive(Debug)]
pub struct ProcessManager {
    processes: HashMap<ProcessId, Process>,
    next_id: u32,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
            next_id: 1,
        }
    }
    
    pub fn spawn_process(&mut self, mut process: Process) -> crate::error::Result<ProcessId> {
        process.id = ProcessId::new(self.next_id);
        self.next_id += 1;
        
        process.start()?;
        let id = process.id;
        self.processes.insert(id, process);
        Ok(id)
    }
    
    pub fn get_process(&self, id: ProcessId) -> Option<&Process> {
        self.processes.get(&id)
    }
    
    pub fn get_process_mut(&mut self, id: ProcessId) -> Option<&mut Process> {
        self.processes.get_mut(&id)
    }
    
    pub fn kill_process(&mut self, id: ProcessId) -> crate::error::Result<()> {
        if let Some(process) = self.processes.get_mut(&id) {
            process.kill()
        } else {
            Err(crate::error::CursedError::Runtime(format!("Process {} not found", id.0)))
        }
    }
    
    pub fn wait_for_process(&mut self, id: ProcessId) -> crate::error::Result<i32> {
        if let Some(process) = self.processes.get_mut(&id) {
            process.wait()
        } else {
            Err(crate::error::CursedError::Runtime(format!("Process {} not found", id.0)))
        }
    }
    
    pub fn list_processes(&self) -> Vec<ProcessId> {
        self.processes.keys().copied().collect()
    }
    
    pub fn running_processes(&self) -> Vec<ProcessId> {
        self.processes.iter()
            .filter(|(_, process)| process.is_running())
            .map(|(id, _)| *id)
            .collect()
    }
    
    pub fn cleanup_finished(&mut self) {
        let finished_ids: Vec<ProcessId> = self.processes.iter()
            .filter(|(_, process)| !process.is_running())
            .map(|(id, _)| *id)
            .collect();
        
        for id in finished_ids {
            self.processes.remove(&id);
        }
    }
    
    pub fn kill_all(&mut self) -> crate::error::Result<()> {
        for process in self.processes.values_mut() {
            if process.is_running() {
                let _ = process.kill(); // Ignore individual errors
            }
        }
        Ok(())
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
