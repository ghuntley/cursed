//! Process management and execution for CURSED runtime
//!
//! Handles process creation, execution, monitoring, and communication
//! for CURSED programs including goroutine management and IPC.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::stack_trace::StackTrace;
use std::collections::HashMap;
use std::process::{Command, Child, Stdio};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::fmt;

/// Process handle for CURSED runtime
pub struct ProcessHandle {
    /// Process ID
    pub pid: u32,
    /// Process name/command
    pub name: String,
    /// Process state
    pub state: ProcessState,
    /// Start time
    pub start_time: Instant,
    /// Resource usage
    pub resources: ProcessResources,
    /// Communication channels
    pub channels: ProcessChannels,
    /// Process metadata
    pub metadata: ProcessMetadata,
}

/// Process execution state
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    /// Process is starting up
    Starting,
    /// Process is running normally
    Running,
    /// Process is suspended/paused
    Suspended,
    /// Process is terminating
    Terminating,
    /// Process has completed successfully
    Completed(i32),
    /// Process failed with error
    Failed(String),
    /// Process was killed
    Killed,
}

/// Process resource usage tracking
#[derive(Debug, Clone, Default)]
pub struct ProcessResources {
    /// CPU time used (milliseconds)
    pub cpu_time_ms: u64,
    /// Memory usage (bytes)
    pub memory_bytes: usize,
    /// Number of file descriptors
    pub file_descriptors: usize,
    /// Network connections
    pub network_connections: usize,
    /// Goroutines spawned
    pub goroutines_spawned: usize,
    /// Channels created
    pub channels_created: usize,
}

/// Communication channels for process
pub struct ProcessChannels {
    /// Standard input
    pub stdin: Option<Box<dyn std::io::Write + Send>>,
    /// Standard output
    pub stdout: Option<Box<dyn std::io::Read + Send>>,
    /// Standard error
    pub stderr: Option<Box<dyn std::io::Read + Send>>,
    /// IPC channels
    pub ipc_channels: HashMap<String, IpcChannel>,
}

impl std::fmt::Debug for ProcessChannels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessChannels")
            .field("stdin", &self.stdin.is_some())
            .field("stdout", &self.stdout.is_some()) 
            .field("stderr", &self.stderr.is_some())
            .field("ipc_channels", &self.ipc_channels)
            .finish()
    }
}

/// Inter-process communication channel
#[derive(Debug, Clone)]
pub struct IpcChannel {
    /// Channel ID
    pub id: String,
    /// Channel type
    pub channel_type: IpcChannelType,
    /// Message queue
    pub messages: Arc<Mutex<Vec<IpcMessage>>>,
    /// Channel state
    pub state: IpcChannelState,
}

/// Types of IPC channels
#[derive(Debug, Clone, PartialEq)]
pub enum IpcChannelType {
    /// Named pipe
    NamedPipe,
    /// Unix domain socket
    UnixSocket,
    /// Shared memory
    SharedMemory,
    /// Message queue
    MessageQueue,
    /// CURSED channel (goroutine-style)
    CursedChannel,
}

/// IPC channel state
#[derive(Debug, Clone, PartialEq)]
pub enum IpcChannelState {
    /// Channel is being created
    Creating,
    /// Channel is open and ready
    Open,
    /// Channel is closed
    Closed,
    /// Channel has an error
    Error(String),
}

/// IPC message
#[derive(Debug, Clone)]
pub struct IpcMessage {
    /// Message ID
    pub id: u64,
    /// Sender process ID
    pub sender_pid: u32,
    /// Message data
    pub data: Vec<u8>,
    /// Message type
    pub message_type: String,
    /// Timestamp
    pub timestamp: Instant,
    /// Priority
    pub priority: MessagePriority,
}

/// Message priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Process metadata
#[derive(Debug, Clone, Default)]
pub struct ProcessMetadata {
    /// Process description
    pub description: String,
    /// Process tags
    pub tags: Vec<String>,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Working directory
    pub working_directory: Option<String>,
    /// Command line arguments
    pub arguments: Vec<String>,
    /// Parent process ID
    pub parent_pid: Option<u32>,
    /// Child process IDs
    pub child_pids: Vec<u32>,
}

impl ProcessHandle {
    /// Create a new process handle
    pub fn new(pid: u32, name: String) -> Self {
        Self {
            pid,
            name,
            state: ProcessState::Starting,
            start_time: Instant::now(),
            resources: ProcessResources::default(),
            channels: ProcessChannels {
                stdin: None,
                stdout: None,
                stderr: None,
                ipc_channels: HashMap::new(),
            },
            metadata: ProcessMetadata::default(),
        }
    }

    /// Get process uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Check if process is alive
    pub fn is_alive(&self) -> bool {
        matches!(self.state, 
            ProcessState::Starting | 
            ProcessState::Running | 
            ProcessState::Suspended)
    }

    /// Update resource usage
    pub fn update_resources(&mut self, resources: ProcessResources) {
        self.resources = resources;
    }

    /// Add IPC channel
    pub fn add_ipc_channel(&mut self, channel: IpcChannel) {
        self.channels.ipc_channels.insert(channel.id.clone(), channel);
    }

    /// Send message via IPC channel
    pub fn send_ipc_message(&self, channel_id: &str, message: IpcMessage) -> CursedResult<()> {
        if let Some(channel) = self.channels.ipc_channels.get(channel_id) {
            if channel.state == IpcChannelState::Open {
                let mut messages = channel.messages.lock().map_err(|_| {
                    Error::Runtime("Failed to acquire message queue lock".to_string())
                })?;
                messages.push(message);
                Ok(())
            } else {
                Err(Error::Runtime(format!("IPC channel {} is not open", channel_id)))
            }
        } else {
            Err(Error::Runtime(format!("IPC channel {} not found", channel_id)))
        }
    }

    /// Receive message from IPC channel
    pub fn receive_ipc_message(&self, channel_id: &str) -> CursedResult<Option<IpcMessage>> {
        if let Some(channel) = self.channels.ipc_channels.get(channel_id) {
            let mut messages = channel.messages.lock().map_err(|_| {
                Error::Runtime("Failed to acquire message queue lock".to_string())
            })?;
            let message = if !messages.is_empty() {
                Some(messages.remove(0))
            } else {
                None
            };
            Ok(message)
        } else {
            Err(Error::Runtime(format!("IPC channel {} not found", channel_id)))
        }
    }
}

impl Clone for ProcessHandle {
    fn clone(&self) -> Self {
        Self {
            pid: self.pid,
            name: self.name.clone(),
            state: self.state.clone(),
            start_time: self.start_time,
            resources: self.resources.clone(),
            channels: ProcessChannels {
                stdin: None, // Cannot clone trait objects
                stdout: None,
                stderr: None,
                ipc_channels: self.channels.ipc_channels.clone(),
            },
            metadata: self.metadata.clone(),
        }
    }
}

impl std::fmt::Debug for ProcessHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessHandle")
            .field("pid", &self.pid)
            .field("name", &self.name)
            .field("state", &self.state)
            .field("start_time", &self.start_time)
            .field("resources", &self.resources)
            .field("channels", &self.channels)
            .field("metadata", &self.metadata)
            .finish()
    }
}

/// Process manager for CURSED runtime
pub struct ProcessManager {
    /// Active processes
    processes: RwLock<HashMap<u32, ProcessHandle>>,
    /// Process counter for generating PIDs
    pid_counter: Mutex<u32>,
    /// Manager configuration
    config: ProcessManagerConfig,
    /// Manager statistics
    stats: Mutex<ProcessManagerStats>,
}

/// Configuration for process manager
#[derive(Debug, Clone)]
pub struct ProcessManagerConfig {
    /// Maximum number of processes
    pub max_processes: Option<usize>,
    /// Default timeout for process operations
    pub default_timeout: Duration,
    /// Whether to capture stdout/stderr
    pub capture_output: bool,
    /// Maximum memory per process
    pub max_memory_per_process: Option<usize>,
    /// Enable resource monitoring
    pub enable_monitoring: bool,
}

impl Default for ProcessManagerConfig {
    fn default() -> Self {
        Self {
            max_processes: Some(100),
            default_timeout: Duration::from_secs(30),
            capture_output: true,
            max_memory_per_process: Some(1024 * 1024 * 1024), // 1GB
            enable_monitoring: true,
        }
    }
}

/// Statistics for process manager
#[derive(Debug, Default, Clone)]
pub struct ProcessManagerStats {
    pub processes_created: usize,
    pub processes_completed: usize,
    pub processes_failed: usize,
    pub processes_killed: usize,
    pub total_cpu_time_ms: u64,
    pub total_memory_bytes: usize,
    pub ipc_messages_sent: usize,
    pub ipc_messages_received: usize,
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            processes: RwLock::new(HashMap::new()),
            pid_counter: Mutex::new(1000), // Start from 1000 to avoid system PIDs
            config: ProcessManagerConfig::default(),
            stats: Mutex::new(ProcessManagerStats::default()),
        }
    }

    /// Create process manager with configuration
    pub fn with_config(config: ProcessManagerConfig) -> Self {
        Self {
            processes: RwLock::new(HashMap::new()),
            pid_counter: Mutex::new(1000),
            config,
            stats: Mutex::new(ProcessManagerStats::default()),
        }
    }

    /// Spawn a new process
    pub fn spawn_process(&self, command: &str, args: &[String]) -> CursedResult<u32> {
        // Check process limit
        if let Some(max_processes) = self.config.max_processes {
            let process_count = self.processes.read().unwrap().len();
            if process_count >= max_processes {
                return Err(Error::Runtime("Maximum process limit reached".to_string()));
            }
        }

        // Generate new PID
        let pid = {
            let mut counter = self.pid_counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        // Create process handle
        let mut process_handle = ProcessHandle::new(pid, command.to_string());
        process_handle.metadata.arguments = args.to_vec();

        // In a real implementation, this would actually spawn the process
        // For now, we'll simulate it
        process_handle.state = ProcessState::Running;

        // Register process
        {
            let mut processes = self.processes.write().unwrap();
            processes.insert(pid, process_handle);
        }

        // Update stats
        {
            let mut stats = self.stats.lock().unwrap();
            stats.processes_created += 1;
        }

        Ok(pid)
    }

    /// Spawn a CURSED program
    pub fn spawn_cursed_program(&self, program_path: &str, args: &[String]) -> CursedResult<u32> {
        let mut command_args = vec!["cursed".to_string(), "run".to_string(), program_path.to_string()];
        command_args.extend_from_slice(args);
        
        self.spawn_process("cursed", &command_args)
    }

    /// Kill a process
    pub fn kill_process(&self, pid: u32) -> CursedResult<()> {
        let mut processes = self.processes.write().unwrap();
        
        if let Some(process) = processes.get_mut(&pid) {
            if process.is_alive() {
                process.state = ProcessState::Killed;
                
                let mut stats = self.stats.lock().unwrap();
                stats.processes_killed += 1;
                
                Ok(())
            } else {
                Err(Error::Runtime(format!("Process {} is not alive", pid)))
            }
        } else {
            Err(Error::Runtime(format!("Process {} not found", pid)))
        }
    }

    /// Get process handle
    pub fn get_process(&self, pid: u32) -> Option<ProcessHandle> {
        let processes = self.processes.read().unwrap();
        processes.get(&pid).cloned()
    }

    /// List all processes
    pub fn list_processes(&self) -> Vec<u32> {
        let processes = self.processes.read().unwrap();
        processes.keys().cloned().collect()
    }

    /// Wait for process to complete
    pub fn wait_for_process(&self, pid: u32, timeout: Option<Duration>) -> CursedResult<ProcessState> {
        let timeout = timeout.unwrap_or(self.config.default_timeout);
        let start_time = Instant::now();

        loop {
            {
                let processes = self.processes.read().unwrap();
                if let Some(process) = processes.get(&pid) {
                    if !process.is_alive() {
                        return Ok(process.state.clone());
                    }
                } else {
                    return Err(Error::Runtime(format!("Process {} not found", pid)));
                }
            }

            if start_time.elapsed() > timeout {
                return Err(Error::Runtime(format!("Timeout waiting for process {}", pid)));
            }

            // Sleep for a short time before checking again
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// Create IPC channel between processes
    pub fn create_ipc_channel(&self, from_pid: u32, to_pid: u32, channel_type: IpcChannelType) -> CursedResult<String> {
        let channel_id = format!("ipc_{}_{}", from_pid, to_pid);
        
        let ipc_channel = IpcChannel {
            id: channel_id.clone(),
            channel_type,
            messages: Arc::new(Mutex::new(Vec::new())),
            state: IpcChannelState::Open,
        };

        // Add channel to both processes
        {
            let mut processes = self.processes.write().unwrap();
            
            if let Some(from_process) = processes.get_mut(&from_pid) {
                from_process.add_ipc_channel(ipc_channel.clone());
            } else {
                return Err(Error::Runtime(format!("Source process {} not found", from_pid)));
            }
            
            if from_pid != to_pid {
                if let Some(to_process) = processes.get_mut(&to_pid) {
                    to_process.add_ipc_channel(ipc_channel);
                } else {
                    return Err(Error::Runtime(format!("Target process {} not found", to_pid)));
                }
            }
        }

        Ok(channel_id)
    }

    /// Send IPC message
    pub fn send_ipc_message(&self, from_pid: u32, channel_id: &str, data: Vec<u8>, message_type: String) -> CursedResult<()> {
        let message = IpcMessage {
            id: self.generate_message_id(),
            sender_pid: from_pid,
            data,
            message_type,
            timestamp: Instant::now(),
            priority: MessagePriority::Normal,
        };

        let processes = self.processes.read().unwrap();
        if let Some(process) = processes.get(&from_pid) {
            process.send_ipc_message(channel_id, message)?;
            
            let mut stats = self.stats.lock().unwrap();
            stats.ipc_messages_sent += 1;
            
            Ok(())
        } else {
            Err(Error::Runtime(format!("Process {} not found", from_pid)))
        }
    }

    /// Receive IPC message
    pub fn receive_ipc_message(&self, pid: u32, channel_id: &str) -> CursedResult<Option<IpcMessage>> {
        let processes = self.processes.read().unwrap();
        if let Some(process) = processes.get(&pid) {
            let message = process.receive_ipc_message(channel_id)?;
            
            if message.is_some() {
                let mut stats = self.stats.lock().unwrap();
                stats.ipc_messages_received += 1;
            }
            
            Ok(message)
        } else {
            Err(Error::Runtime(format!("Process {} not found", pid)))
        }
    }

    /// Generate unique message ID
    fn generate_message_id(&self) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static MESSAGE_COUNTER: AtomicU64 = AtomicU64::new(1);
        MESSAGE_COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    /// Update process resources
    pub fn update_process_resources(&self, pid: u32, resources: ProcessResources) -> CursedResult<()> {
        let mut processes = self.processes.write().unwrap();
        if let Some(process) = processes.get_mut(&pid) {
            process.update_resources(resources);
            Ok(())
        } else {
            Err(Error::Runtime(format!("Process {} not found", pid)))
        }
    }

    /// Get manager statistics
    pub fn get_stats(&self) -> ProcessManagerStats {
        self.stats.lock().unwrap().clone()
    }

    /// Get active process count
    pub fn active_process_count(&self) -> usize {
        let processes = self.processes.read().unwrap();
        processes.values().filter(|p| p.is_alive()).count()
    }

    /// Cleanup completed processes
    pub fn cleanup_completed_processes(&self) -> usize {
        let mut processes = self.processes.write().unwrap();
        let initial_count = processes.len();
        
        processes.retain(|_, process| process.is_alive());
        
        initial_count - processes.len()
    }

    /// Set process manager configuration
    pub fn set_config(&mut self, config: ProcessManagerConfig) {
        self.config = config;
    }

    /// Get process manager configuration
    pub fn get_config(&self) -> ProcessManagerConfig {
        self.config.clone()
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global process manager instance
static GLOBAL_PROCESS_MANAGER: std::sync::LazyLock<Mutex<ProcessManager>> = 
    std::sync::LazyLock::new(|| Mutex::new(ProcessManager::new()));

/// Get the global process manager
pub fn get_global_process_manager() -> &'static Mutex<ProcessManager> {
    &GLOBAL_PROCESS_MANAGER
}

/// Utility functions for process management
pub mod utils {
    use super::*;

    /// Spawn a simple command
    pub fn spawn_command(command: &str, args: &[&str]) -> CursedResult<u32> {
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let manager = get_global_process_manager().lock().map_err(|_| {
            Error::Runtime("Failed to acquire process manager lock".to_string())
        })?;
        
        manager.spawn_process(command, &args)
    }

    /// Run a CURSED program and wait for completion
    pub fn run_cursed_program(program_path: &str, args: &[&str], timeout: Option<Duration>) -> CursedResult<ProcessState> {
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let manager = get_global_process_manager().lock().map_err(|_| {
            Error::Runtime("Failed to acquire process manager lock".to_string())
        })?;
        
        let pid = manager.spawn_cursed_program(program_path, &args)?;
        manager.wait_for_process(pid, timeout)
    }

    /// Get current process count
    pub fn get_process_count() -> CursedResult<usize> {
        let manager = get_global_process_manager().lock().map_err(|_| {
            Error::Runtime("Failed to acquire process manager lock".to_string())
        })?;
        
        Ok(manager.active_process_count())
    }

    /// Create a simple IPC channel
    pub fn create_simple_ipc(from_pid: u32, to_pid: u32) -> CursedResult<String> {
        let manager = get_global_process_manager().lock().map_err(|_| {
            Error::Runtime("Failed to acquire process manager lock".to_string())
        })?;
        
        manager.create_ipc_channel(from_pid, to_pid, IpcChannelType::CursedChannel)
    }

    /// Send a simple text message via IPC
    pub fn send_text_message(from_pid: u32, channel_id: &str, text: &str) -> CursedResult<()> {
        let manager = get_global_process_manager().lock().map_err(|_| {
            Error::Runtime("Failed to acquire process manager lock".to_string())
        })?;
        
        manager.send_ipc_message(
            from_pid, 
            channel_id, 
            text.as_bytes().to_vec(), 
            "text".to_string()
        )
    }

    /// Receive a text message via IPC
    pub fn receive_text_message(pid: u32, channel_id: &str) -> CursedResult<Option<String>> {
        let manager = get_global_process_manager().lock().map_err(|_| {
            Error::Runtime("Failed to acquire process manager lock".to_string())
        })?;
        
        if let Some(message) = manager.receive_ipc_message(pid, channel_id)? {
            if message.message_type == "text" {
                if let Ok(text) = String::from_utf8(message.data) {
                    return Ok(Some(text));
                }
            }
        }
        
        Ok(None)
    }

    /// Kill all processes
    pub fn kill_all_processes() -> CursedResult<usize> {
        let manager = get_global_process_manager().lock().map_err(|_| {
            Error::Runtime("Failed to acquire process manager lock".to_string())
        })?;
        
        let pids = manager.list_processes();
        let mut killed_count = 0;
        
        for pid in pids {
            if manager.kill_process(pid).is_ok() {
                killed_count += 1;
            }
        }
        
        Ok(killed_count)
    }

    /// Get process statistics summary
    pub fn get_process_stats_summary() -> CursedResult<String> {
        let manager = get_global_process_manager().lock().map_err(|_| {
            Error::Runtime("Failed to acquire process manager lock".to_string())
        })?;
        
        let stats = manager.get_stats();
        let active_count = manager.active_process_count();
        
        Ok(format!(
            "Active: {} | Created: {} | Completed: {} | Failed: {} | Killed: {} | IPC Messages: {}/{}",
            active_count,
            stats.processes_created,
            stats.processes_completed,
            stats.processes_failed,
            stats.processes_killed,
            stats.ipc_messages_sent,
            stats.ipc_messages_received
        ))
    }
}

/// Process execution environment
pub struct ProcessEnvironment {
    /// Environment variables
    pub variables: HashMap<String, String>,
    /// Working directory
    pub working_directory: String,
    /// PATH directories
    pub path_dirs: Vec<String>,
    /// Process capabilities
    pub capabilities: ProcessCapabilities,
}

/// Process capabilities and permissions
#[derive(Debug, Clone, Default)]
pub struct ProcessCapabilities {
    /// Can spawn child processes
    pub can_spawn_children: bool,
    /// Can access file system
    pub can_access_filesystem: bool,
    /// Can access network
    pub can_access_network: bool,
    /// Can create IPC channels
    pub can_create_ipc: bool,
    /// Maximum memory usage
    pub max_memory_bytes: Option<usize>,
    /// Maximum CPU time
    pub max_cpu_time_ms: Option<u64>,
}

impl ProcessEnvironment {
    /// Create a new process environment
    pub fn new() -> Self {
        Self {
            variables: std::env::vars().collect(),
            working_directory: std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path_dirs: std::env::var("PATH")
                .unwrap_or_default()
                .split(':')
                .map(|s| s.to_string())
                .collect(),
            capabilities: ProcessCapabilities::default(),
        }
    }

    /// Create a restricted environment
    pub fn restricted() -> Self {
        Self {
            variables: HashMap::new(),
            working_directory: "/tmp".to_string(),
            path_dirs: vec!["/usr/bin".to_string(), "/bin".to_string()],
            capabilities: ProcessCapabilities {
                can_spawn_children: false,
                can_access_filesystem: false,
                can_access_network: false,
                can_create_ipc: false,
                max_memory_bytes: Some(64 * 1024 * 1024), // 64MB
                max_cpu_time_ms: Some(10000), // 10 seconds
            },
        }
    }

    /// Set environment variable
    pub fn set_var(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    /// Get environment variable
    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
}

impl Default for ProcessEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

/// Public function that was likely being used by external code
pub fn get_minimal_result() -> CursedResult<String> {
    Ok("CURSED process management system initialized".to_string())
}
