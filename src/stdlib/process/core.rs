// Core process management functionality for CURSED
use std::collections::HashMap;
use std::process::{Command, Child, Stdio};
use std::io::{Result, Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Process configuration
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub program: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub working_dir: Option<String>,
    pub stdin: Option<Stdio>,
    pub stdout: Option<Stdio>,
    pub stderr: Option<Stdio>,
    pub timeout: Option<Duration>,
}

impl ProcessConfig {
    pub fn new(program: String) -> Self {
        Self {
            program,
            args: Vec::new(),
            env: HashMap::new(),
            working_dir: None,
            stdin: Some(Stdio::piped()),
            stdout: Some(Stdio::piped()),
            stderr: Some(Stdio::piped()),
            timeout: None,
        }
    }
    
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
    
    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.env.insert(key, value);
        self
    }
    
    pub fn with_working_dir(mut self, dir: String) -> Self {
        self.working_dir = Some(dir);
        self
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

/// Process state
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Running,
    Finished(i32),
    Killed,
    Failed(String),
}

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub command: String,
    pub state: ProcessState,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

/// Process handle
#[derive(Debug)]
pub struct ProcessHandle {
    pub info: ProcessInfo,
    child: Option<Child>,
}

impl ProcessHandle {
    pub fn new(child: Child, command: String) -> Self {
        let info = ProcessInfo {
            pid: child.id(),
            command,
            state: ProcessState::Running,
            start_time: Instant::now(),
            end_time: None,
        };
        
        Self {
            info,
            child: Some(child),
        }
    }
    
    pub fn wait(&mut self) -> Result<i32> {
        if let Some(mut child) = self.child.take() {
            let result = child.wait()?;
            let exit_code = result.code().unwrap_or(-1);
            self.info.state = ProcessState::Finished(exit_code);
            self.info.end_time = Some(Instant::now());
            Ok(exit_code)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Process already finished"))
        }
    }
    
    pub fn kill(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill()?;
            self.info.state = ProcessState::Killed;
            self.info.end_time = Some(Instant::now());
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Process already finished"))
        }
    }
    
    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    let exit_code = status.code().unwrap_or(-1);
                    self.info.state = ProcessState::Finished(exit_code);
                    self.info.end_time = Some(Instant::now());
                    false
                }
                Ok(None) => true,
                Err(_) => {
                    self.info.state = ProcessState::Failed("Failed to check process status".to_string());
                    false
                }
            }
        } else {
            false
        }
    }
}

/// Process group for managing multiple processes
#[derive(Debug)]
pub struct ProcessGroup {
    processes: HashMap<u32, ProcessHandle>,
    name: String,
}

impl ProcessGroup {
    pub fn new(name: String) -> Self {
        Self {
            processes: HashMap::new(),
            name,
        }
    }
    
    pub fn add_process(&mut self, handle: ProcessHandle) {
        let pid = handle.info.pid;
        self.processes.insert(pid, handle);
    }
    
    pub fn get_process(&mut self, pid: u32) -> Option<&mut ProcessHandle> {
        self.processes.get_mut(&pid)
    }
    
    pub fn kill_all(&mut self) -> Result<()> {
        for (_, handle) in self.processes.iter_mut() {
            if handle.is_running() {
                handle.kill()?;
            }
        }
        Ok(())
    }
    
    pub fn wait_all(&mut self) -> Result<Vec<i32>> {
        let mut exit_codes = Vec::new();
        for (_, handle) in self.processes.iter_mut() {
            if handle.is_running() {
                exit_codes.push(handle.wait()?);
            }
        }
        Ok(exit_codes)
    }
    
    pub fn running_count(&mut self) -> usize {
        self.processes.values_mut().filter(|h| h.is_running()).count()
    }
}

/// Process manager
#[derive(Debug)]
pub struct ProcessManager {
    processes: Arc<Mutex<HashMap<u32, ProcessHandle>>>,
    groups: Arc<Mutex<HashMap<String, ProcessGroup>>>,
    next_id: Arc<Mutex<u32>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
            groups: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
    
    pub fn spawn_process(&self, config: ProcessConfig) -> Result<ProcessHandle> {
        let mut cmd = Command::new(&config.program);
        
        if !config.args.is_empty() {
            cmd.args(&config.args);
        }
        
        for (key, value) in &config.env {
            cmd.env(key, value);
        }
        
        if let Some(ref dir) = config.working_dir {
            cmd.current_dir(dir);
        }
        
        if let Some(stdin) = config.stdin {
            cmd.stdin(stdin);
        }
        
        if let Some(stdout) = config.stdout {
            cmd.stdout(stdout);
        }
        
        if let Some(stderr) = config.stderr {
            cmd.stderr(stderr);
        }
        
        let child = cmd.spawn()?;
        let command_str = format!("{} {}", config.program, config.args.join(" "));
        let handle = ProcessHandle::new(child, command_str);
        
        let pid = handle.info.pid;
        if let Ok(mut processes) = self.processes.lock() {
            processes.insert(pid, handle);
        }
        
        self.get_process(pid).ok_or_else(|| Error::new(ErrorKind::Other, "Failed to store process"))
    }
    
    pub fn get_process(&self, pid: u32) -> Option<ProcessHandle> {
        if let Ok(mut processes) = self.processes.lock() {
            processes.remove(&pid)
        } else {
            None
        }
    }
    
    pub fn kill_process(&self, pid: u32) -> Result<()> {
        if let Ok(mut processes) = self.processes.lock() {
            if let Some(handle) = processes.get_mut(&pid) {
                handle.kill()
            } else {
                Err(Error::new(ErrorKind::NotFound, "Process not found"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Failed to access processes"))
        }
    }
    
    pub fn create_group(&self, name: String) -> Result<()> {
        if let Ok(mut groups) = self.groups.lock() {
            groups.insert(name.clone(), ProcessGroup::new(name));
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Failed to create process group"))
        }
    }
    
    pub fn list_processes(&self) -> Vec<ProcessInfo> {
        if let Ok(processes) = self.processes.lock() {
            processes.values().map(|h| h.info.clone()).collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// IO redirection configuration
#[derive(Debug, Clone)]
pub struct IoRedirection {
    pub stdin_file: Option<String>,
    pub stdout_file: Option<String>,
    pub stderr_file: Option<String>,
    pub append_stdout: bool,
    pub append_stderr: bool,
}

impl IoRedirection {
    pub fn new() -> Self {
        Self {
            stdin_file: None,
            stdout_file: None,
            stderr_file: None,
            append_stdout: false,
            append_stderr: false,
        }
    }
    
    pub fn with_stdin_file(mut self, file: String) -> Self {
        self.stdin_file = Some(file);
        self
    }
    
    pub fn with_stdout_file(mut self, file: String, append: bool) -> Self {
        self.stdout_file = Some(file);
        self.append_stdout = append;
        self
    }
    
    pub fn with_stderr_file(mut self, file: String, append: bool) -> Self {
        self.stderr_file = Some(file);
        self.append_stderr = append;
        self
    }
}

impl Default for IoRedirection {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions
pub fn spawn_process(config: ProcessConfig) -> Result<ProcessHandle> {
    let manager = ProcessManager::new();
    manager.spawn_process(config)
}

pub fn run_command(config: ProcessConfig) -> Result<i32> {
    let manager = ProcessManager::new();
    let mut handle = manager.spawn_process(config)?;
    handle.wait()
}

pub fn run_command_timeout(config: ProcessConfig, timeout: Duration) -> Result<i32> {
    let manager = ProcessManager::new();
    let mut handle = manager.spawn_process(config)?;
    
    let start = Instant::now();
    while start.elapsed() < timeout {
        if !handle.is_running() {
            return handle.wait();
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    
    handle.kill()?;
    Err(Error::new(ErrorKind::TimedOut, "Process timed out"))
}

pub fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_config_creation() {
        let config = ProcessConfig::new("echo".to_string())
            .with_args(vec!["hello".to_string()])
            .with_timeout(Duration::from_secs(5));
        
        assert_eq!(config.program, "echo");
        assert_eq!(config.args, vec!["hello"]);
        assert_eq!(config.timeout, Some(Duration::from_secs(5)));
    }
    
    #[test]
    fn test_process_manager_creation() {
        let manager = ProcessManager::new();
        assert_eq!(manager.list_processes().len(), 0);
    }
    
    #[test]
    fn test_io_redirection_config() {
        let io = IoRedirection::new()
            .with_stdout_file("output.txt".to_string(), false)
            .with_stderr_file("error.txt".to_string(), true);
        
        assert_eq!(io.stdout_file, Some("output.txt".to_string()));
        assert_eq!(io.stderr_file, Some("error.txt".to_string()));
        assert_eq!(io.append_stderr, true);
    }
}
