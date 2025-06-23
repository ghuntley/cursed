/// Comprehensive Process Management and IPC Integration System
/// 
/// This module provides a unified integration layer that connects all process
/// and IPC functionality into a cohesive system for CURSED programs.

use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::io::{self, Read, Write, BufRead, BufReader};

use tracing::{info, warn, error, debug, instrument};

use crate::error::CursedError;
use crate::stdlib::ipc::{IpcError, IpcResult, IpcConfig, RealIpcManager, IpcConnection};
use crate::stdlib::exec_slay::{SlayCommand, SlayProcess, SlayOptions, ProcessStats};

use super::error::ProcessError;
use super::monitoring::{ProcessMonitor, MonitoringConfig};
use super::signals::{SignalHandler, SignalManager};
use super::resource_limits::{ResourceLimiter, ResourceConfig};

/// Comprehensive process manager alias for integration
pub type ComprehensiveProcessManager = ProcessIpcIntegration;

/// Comprehensive process and IPC integration system
#[derive(Debug)]
pub struct ProcessIpcIntegration {
    /// Process management components
    process_manager: Arc<RwLock<ProcessManager>>,
    /// IPC management
    ipc_manager: Arc<Mutex<RealIpcManager>>,
    /// Signal handling
    signal_manager: Arc<SignalManager>,
    /// Resource monitoring
    monitor: Arc<ProcessMonitor>,
    /// Resource limiting
    resource_limiter: Arc<ResourceLimiter>,
    /// Configuration
    config: IntegrationConfig,
    /// Process registry
    process_registry: Arc<RwLock<ProcessRegistry>>,
    /// IPC registry
    ipc_registry: Arc<RwLock<IpcRegistry>>,
    /// Event callbacks
    event_callbacks: Arc<Mutex<Vec<Box<dyn ProcessEventCallback + Send + Sync>>>>,
    /// Shutdown coordination
    shutdown_signal: Arc<(Mutex<bool>, Condvar)>,
    /// Background management thread
    management_thread: Option<thread::JoinHandle<()>>,
}

/// Process manager for tracking and controlling processes
#[derive(Debug)]
pub struct ProcessManager {
    processes: HashMap<u32, ProcessEntry>,
    next_internal_id: u32,
    command_history: Vec<CommandHistoryEntry>,
    active_pipelines: HashMap<String, PipelineEntry>,
}

/// Registry for tracking processes
#[derive(Debug)]
pub struct ProcessRegistry {
    entries: BTreeMap<u32, ProcessRegistryEntry>,
    name_to_pid: HashMap<String, u32>,
    group_memberships: HashMap<String, Vec<u32>>,
    parent_child_map: HashMap<u32, Vec<u32>>,
}

/// Registry for tracking IPC resources
#[derive(Debug)]
pub struct IpcRegistry {
    connections: HashMap<String, IpcConnectionEntry>,
    named_pipes: HashMap<String, NamedPipeEntry>,
    shared_memory: HashMap<String, SharedMemoryEntry>,
    message_queues: HashMap<String, MessageQueueEntry>,
    process_bindings: HashMap<u32, Vec<String>>, // PID -> IPC resource names
}

/// Configuration for the integration system
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    /// IPC configuration
    pub ipc_config: IpcConfig,
    /// Monitoring configuration
    pub monitoring_config: MonitoringConfig,
    /// Resource limits
    pub resource_config: ResourceConfig,
    /// Maximum number of concurrent processes
    pub max_processes: usize,
    /// Maximum number of IPC connections
    pub max_ipc_connections: usize,
    /// Process cleanup interval
    pub cleanup_interval: Duration,
    /// Default process timeout
    pub default_timeout: Duration,
    /// Enable detailed logging
    pub detailed_logging: bool,
}

/// Entry for tracking a process
#[derive(Debug)]
pub struct ProcessEntry {
    pub pid: u32,
    pub internal_id: u32,
    pub command: SlayCommand,
    pub process: Option<SlayProcess>,
    pub start_time: SystemTime,
    pub status: ProcessStatus,
    pub options: SlayOptions,
    pub ipc_bindings: Vec<String>,
    pub resource_usage: Option<ProcessStats>,
    pub last_monitoring_update: SystemTime,
}

/// Registry entry for a process
#[derive(Debug, Clone)]
pub struct ProcessRegistryEntry {
    pub pid: u32,
    pub name: Option<String>,
    pub command_line: String,
    pub start_time: SystemTime,
    pub parent_pid: Option<u32>,
    pub group: Option<String>,
    pub status: ProcessStatus,
    pub resource_usage: Option<ProcessStats>,
}

/// IPC connection entry
#[derive(Debug)]
pub struct IpcConnectionEntry {
    pub name: String,
    pub connection_type: IpcConnectionType,
    pub connection: Arc<IpcConnection>,
    pub created_time: SystemTime,
    pub bound_processes: Vec<u32>,
    pub usage_stats: IpcUsageStats,
}

/// Named pipe entry
#[derive(Debug)]
pub struct NamedPipeEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_server: bool,
    pub created_time: SystemTime,
    pub bound_processes: Vec<u32>,
    pub message_count: u64,
    pub bytes_transferred: u64,
}

/// Shared memory entry
#[derive(Debug)]
pub struct SharedMemoryEntry {
    pub name: String,
    pub size: usize,
    pub created_time: SystemTime,
    pub bound_processes: Vec<u32>,
    pub access_count: u64,
    pub last_access: SystemTime,
}

/// Message queue entry
#[derive(Debug)]
pub struct MessageQueueEntry {
    pub name: String,
    pub max_messages: usize,
    pub max_message_size: usize,
    pub created_time: SystemTime,
    pub bound_processes: Vec<u32>,
    pub messages_sent: u64,
    pub messages_received: u64,
}

/// Command history entry
#[derive(Debug)]
pub struct CommandHistoryEntry {
    pub command: String,
    pub args: Vec<String>,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub exit_code: Option<i32>,
    pub pid: Option<u32>,
}

/// Pipeline execution entry
#[derive(Debug)]
pub struct PipelineEntry {
    pub id: String,
    pub commands: Vec<SlayCommand>,
    pub pids: Vec<u32>,
    pub start_time: SystemTime,
    pub status: PipelineStatus,
}

/// Process status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Created,
    Starting,
    Running,
    Paused,
    Stopping,
    Stopped,
    Failed,
    Completed,
}

/// Pipeline status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineStatus {
    Created,
    Starting,
    Running,
    Paused,
    Completed,
    Failed,
}

/// IPC connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcConnectionType {
    NamedPipe,
    UnixSocket,
    MessageQueue,
    SharedMemory,
    MemoryMappedFile,
}

/// IPC usage statistics
#[derive(Debug, Clone)]
pub struct IpcUsageStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_transferred: u64,
    pub connection_count: u64,
    pub error_count: u64,
    pub last_activity: SystemTime,
}

/// Event callback trait for process events
pub trait ProcessEventCallback {
    fn on_process_started(&self, pid: u32, command: &str);
    fn on_process_completed(&self, pid: u32, exit_code: i32);
    fn on_process_failed(&self, pid: u32, error: &str);
    fn on_ipc_connection_created(&self, name: &str, connection_type: IpcConnectionType);
    fn on_ipc_connection_closed(&self, name: &str);
    fn on_resource_limit_exceeded(&self, pid: u32, resource: &str, limit: f64, actual: f64);
}

/// Default implementation of process event callback
#[derive(Debug)]
pub struct DefaultEventCallback;

impl ProcessEventCallback for DefaultEventCallback {
    fn on_process_started(&self, pid: u32, command: &str) {
        info!("Process started: PID {} - {}", pid, command);
    }
    
    fn on_process_completed(&self, pid: u32, exit_code: i32) {
        info!("Process completed: PID {} - exit code {}", pid, exit_code);
    }
    
    fn on_process_failed(&self, pid: u32, error: &str) {
        error!("Process failed: PID {} - {}", pid, error);
    }
    
    fn on_ipc_connection_created(&self, name: &str, connection_type: IpcConnectionType) {
        info!("IPC connection created: {} ({:?})", name, connection_type);
    }
    
    fn on_ipc_connection_closed(&self, name: &str) {
        info!("IPC connection closed: {}", name);
    }
    
    fn on_resource_limit_exceeded(&self, pid: u32, resource: &str, limit: f64, actual: f64) {
        warn!("Resource limit exceeded: PID {} - {} (limit: {}, actual: {})", 
              pid, resource, limit, actual);
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            ipc_config: IpcConfig::default(),
            monitoring_config: MonitoringConfig::default(),
            resource_config: ResourceConfig::default(),
            max_processes: 1000,
            max_ipc_connections: 500,
            cleanup_interval: Duration::from_secs(60),
            default_timeout: Duration::from_secs(30),
            detailed_logging: false,
        }
    }
}

impl ProcessIpcIntegration {
    /// Create new integration system
    #[instrument]
    pub fn new(config: IntegrationConfig) -> Result<(), Error> {
        info!("Initializing ProcessIpcIntegration system");
        
        // Initialize IPC manager
        let ipc_manager = RealIpcManager::new(config.ipc_config.clone())
            .map_err(|e| CursedError::RuntimeError(format!("Failed to initialize IPC manager: {}", e)))?;
        
        // Initialize signal manager
        let signal_manager = SignalManager::new()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to initialize signal manager: {}", e)))?;
        
        // Initialize process monitor
        let monitor = ProcessMonitor::new(config.monitoring_config.clone())
            .map_err(|e| CursedError::RuntimeError(format!("Failed to initialize process monitor: {}", e)))?;
        
        // Initialize resource limiter
        let resource_limiter = ResourceLimiter::new(config.resource_config.clone())
            .map_err(|e| CursedError::RuntimeError(format!("Failed to initialize resource limiter: {}", e)))?;
        
        let shutdown_signal = Arc::new((Mutex::new(false), Condvar::new()));
        
        let mut integration = Self {
            process_manager: Arc::new(RwLock::new(ProcessManager::new())),
            ipc_manager: Arc::new(Mutex::new(ipc_manager)),
            signal_manager: Arc::new(signal_manager),
            monitor: Arc::new(monitor),
            resource_limiter: Arc::new(resource_limiter),
            config,
            process_registry: Arc::new(RwLock::new(ProcessRegistry::new())),
            ipc_registry: Arc::new(RwLock::new(IpcRegistry::new())),
            event_callbacks: Arc::new(Mutex::new(vec![Box::new(DefaultEventCallback)])),
            shutdown_signal,
            management_thread: None,
        };
        
        // Start background management thread
        integration.start_management_thread()?;
        
        info!("ProcessIpcIntegration system initialized successfully");
        Ok(integration)
    }
    
    /// Start a background management thread
    #[instrument(skip(self))]
    fn start_management_thread(&mut self) -> Result<(), Error> {
        let process_manager = self.process_manager.clone();
        let ipc_registry = self.ipc_registry.clone();
        let monitor = self.monitor.clone();
        let shutdown_signal = self.shutdown_signal.clone();
        let cleanup_interval = self.config.cleanup_interval;
        
        let handle = thread::spawn(move || {
            let (lock, cvar) = &*shutdown_signal;
            
            loop {
                // Wait for shutdown signal or timeout
                let result = cvar.wait_timeout(
                    lock.lock().unwrap(),
                    cleanup_interval
                ).unwrap();
                
                if *result.0 {
                    info!("Management thread shutting down");
                    break;
                }
                
                // Perform periodic cleanup and monitoring
                Self::perform_cleanup(&process_manager, &ipc_registry);
                Self::update_monitoring(&monitor, &process_manager);
            }
        });
        
        self.management_thread = Some(handle);
        Ok(())
    }
    
    /// Spawn a new process with IPC integration
    #[instrument(skip(self))]
    pub fn spawn_process(&self, mut command: SlayCommand, ipc_bindings: Vec<String>) -> Result<(), Error> {
        // Check process limits
        {
            let manager = self.process_manager.read()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire process manager lock".to_string()))?;
            
            if manager.processes.len() >= self.config.max_processes {
                return Err(CursedError::RuntimeError("Maximum process limit reached".to_string()));
            }
        }
        
        // Start the process
        command.start()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to start process: {}", e)))?;
        
        let process = command.process()
            .ok_or_else(|| CursedError::RuntimeError("Failed to get process handle".to_string()))?;
        
        // Get PID from the underlying system process
        let pid = self.extract_pid_from_process(&process)?;
        
        // Create process entry
        let mut manager = self.process_manager.write()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire process manager lock".to_string()))?;
        
        let internal_id = manager.next_internal_id;
        manager.next_internal_id += 1;
        
        let entry = ProcessEntry {
            pid,
            internal_id,
            command: command.clone(),
            process: Some(process),
            start_time: SystemTime::now(),
            status: ProcessStatus::Running,
            options: command.options.clone(),
            ipc_bindings: ipc_bindings.clone(),
            resource_usage: None,
            last_monitoring_update: SystemTime::now(),
        };
        
        manager.processes.insert(pid, entry);
        
        // Add to process registry
        {
            let mut registry = self.process_registry.write()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire process registry lock".to_string()))?;
            
            let registry_entry = ProcessRegistryEntry {
                pid,
                name: None,
                command_line: command.to_string(),
                start_time: SystemTime::now(),
                parent_pid: None, // Could be enhanced to track parent PIDs
                group: None,
                status: ProcessStatus::Running,
                resource_usage: None,
            };
            
            registry.entries.insert(pid, registry_entry);
        }
        
        // Bind IPC resources
        for ipc_name in &ipc_bindings {
            if let Err(e) = self.bind_ipc_to_process(pid, ipc_name) {
                warn!("Failed to bind IPC resource {} to process {}: {}", ipc_name, pid, e);
            }
        }
        
        // Notify callbacks
        self.notify_process_started(pid, &command.to_string());
        
        info!("Process spawned successfully: PID {} - {}", pid, command.to_string());
        Ok(pid)
    }
    
    /// Create named pipe with process binding
    #[instrument(skip(self))]
    pub fn create_named_pipe(&self, name: &str, is_server: bool, bind_to_process: Option<u32>) -> Result<(), Error> {
        // Create the IPC connection
        let connection = {
            let mut ipc_manager = self.ipc_manager.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC manager lock".to_string()))?;
            
            ipc_manager.create_named_pipe(name, is_server)
                .map_err(|e| CursedError::RuntimeError(format!("Failed to create named pipe: {}", e)))?
        };
        
        // Register in IPC registry
        {
            let mut registry = self.ipc_registry.write()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC registry lock".to_string()))?;
            
            let entry = IpcConnectionEntry {
                name: name.to_string(),
                connection_type: IpcConnectionType::NamedPipe,
                connection,
                created_time: SystemTime::now(),
                bound_processes: bind_to_process.map(|pid| vec![pid]).unwrap_or_default(),
                usage_stats: IpcUsageStats {
                    messages_sent: 0,
                    messages_received: 0,
                    bytes_transferred: 0,
                    connection_count: 0,
                    error_count: 0,
                    last_activity: SystemTime::now(),
                },
            };
            
            registry.connections.insert(name.to_string(), entry);
            
            // Add to process bindings if specified
            if let Some(pid) = bind_to_process {
                registry.process_bindings.entry(pid).or_insert_with(Vec::new).push(name.to_string());
            }
        }
        
        // Notify callbacks
        self.notify_ipc_connection_created(name, IpcConnectionType::NamedPipe);
        
        info!("Named pipe created: {} (server: {})", name, is_server);
        Ok(())
    }
    
    /// Create shared memory with process binding
    #[instrument(skip(self))]
    pub fn create_shared_memory(&self, name: &str, size: usize, bind_to_process: Option<u32>) -> Result<(), Error> {
        // Create the IPC connection
        let connection = {
            let mut ipc_manager = self.ipc_manager.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC manager lock".to_string()))?;
            
            ipc_manager.create_shared_memory(name, size)
                .map_err(|e| CursedError::RuntimeError(format!("Failed to create shared memory: {}", e)))?
        };
        
        // Register in IPC registry
        {
            let mut registry = self.ipc_registry.write()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC registry lock".to_string()))?;
            
            let entry = IpcConnectionEntry {
                name: name.to_string(),
                connection_type: IpcConnectionType::SharedMemory,
                connection,
                created_time: SystemTime::now(),
                bound_processes: bind_to_process.map(|pid| vec![pid]).unwrap_or_default(),
                usage_stats: IpcUsageStats {
                    messages_sent: 0,
                    messages_received: 0,
                    bytes_transferred: 0,
                    connection_count: 0,
                    error_count: 0,
                    last_activity: SystemTime::now(),
                },
            };
            
            registry.connections.insert(name.to_string(), entry);
            
            // Add to process bindings if specified
            if let Some(pid) = bind_to_process {
                registry.process_bindings.entry(pid).or_insert_with(Vec::new).push(name.to_string());
            }
        }
        
        // Notify callbacks
        self.notify_ipc_connection_created(name, IpcConnectionType::SharedMemory);
        
        info!("Shared memory created: {} (size: {} bytes)", name, size);
        Ok(())
    }
    
    /// Create message queue with process binding
    #[instrument(skip(self))]
    pub fn create_message_queue(&self, name: &str, bind_to_process: Option<u32>) -> Result<(), Error> {
        // Create the IPC connection
        let connection = {
            let mut ipc_manager = self.ipc_manager.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC manager lock".to_string()))?;
            
            ipc_manager.create_message_queue(name)
                .map_err(|e| CursedError::RuntimeError(format!("Failed to create message queue: {}", e)))?
        };
        
        // Register in IPC registry
        {
            let mut registry = self.ipc_registry.write()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC registry lock".to_string()))?;
            
            let entry = IpcConnectionEntry {
                name: name.to_string(),
                connection_type: IpcConnectionType::MessageQueue,
                connection,
                created_time: SystemTime::now(),
                bound_processes: bind_to_process.map(|pid| vec![pid]).unwrap_or_default(),
                usage_stats: IpcUsageStats {
                    messages_sent: 0,
                    messages_received: 0,
                    bytes_transferred: 0,
                    connection_count: 0,
                    error_count: 0,
                    last_activity: SystemTime::now(),
                },
            };
            
            registry.connections.insert(name.to_string(), entry);
            
            // Add to process bindings if specified
            if let Some(pid) = bind_to_process {
                registry.process_bindings.entry(pid).or_insert_with(Vec::new).push(name.to_string());
            }
        }
        
        // Notify callbacks
        self.notify_ipc_connection_created(name, IpcConnectionType::MessageQueue);
        
        info!("Message queue created: {}", name);
        Ok(())
    }
    
    /// Execute a pipeline of commands
    #[instrument(skip(self))]
    pub fn execute_pipeline(&self, pipeline_id: &str, commands: Vec<SlayCommand>) -> Result<(), Error> {
        let mut pids = Vec::new();
        let start_time = SystemTime::now();
        
        // Record pipeline start
        {
            let mut manager = self.process_manager.write()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire process manager lock".to_string()))?;
            
            let entry = PipelineEntry {
                id: pipeline_id.to_string(),
                commands: commands.clone(),
                pids: Vec::new(),
                start_time,
                status: PipelineStatus::Starting,
            };
            
            manager.active_pipelines.insert(pipeline_id.to_string(), entry);
        }
        
        // Execute commands in sequence
        for (i, mut command) in commands.into_iter().enumerate() {
            // For pipeline commands, redirect stdout of previous to stdin of current
            if i > 0 {
                // This would need more sophisticated piping implementation
                // For now, we just execute them sequentially
            }
            
            let pid = self.spawn_process(command, Vec::new())?;
            pids.push(pid);
        }
        
        // Update pipeline with PIDs
        {
            let mut manager = self.process_manager.write()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire process manager lock".to_string()))?;
            
            if let Some(entry) = manager.active_pipelines.get_mut(pipeline_id) {
                entry.pids = pids.clone();
                entry.status = PipelineStatus::Running;
            }
        }
        
        info!("Pipeline executed: {} with {} commands, PIDs: {:?}", pipeline_id, pids.len(), pids);
        Ok(pids)
    }
    
    /// Get process information
    #[instrument(skip(self))]
    pub fn get_process_info(&self, pid: u32) -> Result<(), Error> {
        let registry = self.process_registry.read()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire process registry lock".to_string()))?;
        
        registry.entries.get(&pid)
            .cloned()
            .ok_or_else(|| CursedError::RuntimeError(format!("Process {} not found", pid)))
    }
    
    /// List all active processes
    #[instrument(skip(self))]
    pub fn list_processes(&self) -> Result<(), Error> {
        let registry = self.process_registry.read()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire process registry lock".to_string()))?;
        
        Ok(registry.entries.values().cloned().collect())
    }
    
    /// List all IPC connections
    #[instrument(skip(self))]
    pub fn list_ipc_connections(&self) -> Result<(), Error> {
        let registry = self.ipc_registry.read()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC registry lock".to_string()))?;
        
        Ok(registry.connections.keys().cloned().collect())
    }
    
    /// Get IPC connection information
    #[instrument(skip(self))]
    pub fn get_ipc_info(&self, name: &str) -> Result<(), Error> {
        let registry = self.ipc_registry.read()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC registry lock".to_string()))?;
        
        registry.connections.get(name)
            .cloned()
            .ok_or_else(|| CursedError::RuntimeError(format!("IPC connection {} not found", name)))
    }
    
    /// Kill a process
    #[instrument(skip(self))]
    pub fn kill_process(&self, pid: u32) -> Result<(), Error> {
        let mut manager = self.process_manager.write()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire process manager lock".to_string()))?;
        
        if let Some(entry) = manager.processes.get_mut(&pid) {
            if let Some(ref process) = entry.process {
                process.kill()
                    .map_err(|e| CursedError::RuntimeError(format!("Failed to kill process {}: {}", pid, e)))?;
                
                entry.status = ProcessStatus::Stopped;
            }
        }
        
        info!("Process killed: {}", pid);
        Ok(())
    }
    
    /// Wait for process completion
    #[instrument(skip(self))]
    pub fn wait_for_process(&self, pid: u32, timeout: Option<Duration>) -> Result<(), Error> {
        let start = Instant::now();
        
        loop {
            // Check if process is still running
            {
                let manager = self.process_manager.read()
                    .map_err(|_| CursedError::RuntimeError("Failed to acquire process manager lock".to_string()))?;
                
                if let Some(entry) = manager.processes.get(&pid) {
                    match entry.status {
                        ProcessStatus::Completed => {
                            // Get exit code from the command
                            if let Some(exit_code) = entry.command.exit_code() {
                                return Ok(exit_code);
                            }
                        }
                        ProcessStatus::Failed => {
                            return Err(CursedError::RuntimeError(format!("Process {} failed", pid)));
                        }
                        ProcessStatus::Stopped => {
                            return Ok(-1); // Killed
                        }
                        _ => {
                            // Still running, continue waiting
                        }
                    }
                }
            }
            
            // Check timeout
            if let Some(timeout) = timeout {
                if start.elapsed() >= timeout {
                    return Err(CursedError::RuntimeError(format!("Timeout waiting for process {}", pid)));
                }
            }
            
            thread::sleep(Duration::from_millis(100));
        }
    }
    
    /// Cleanup completed processes and unused IPC resources
    #[instrument(skip(process_manager, ipc_registry))]
    fn perform_cleanup(
        process_manager: &Arc<RwLock<ProcessManager>>,
        ipc_registry: &Arc<RwLock<IpcRegistry>>
    ) {
        // Clean up completed processes
        {
            let mut manager = match process_manager.write() {
                Ok(manager) => manager,
                Err(_) => return,
            };
            
            let mut to_remove = Vec::new();
            
            for (&pid, entry) in &mut manager.processes {
                if !entry.command.is_running() {
                    entry.status = if entry.command.exit_code().map(|c| c == 0).unwrap_or(false) {
                        ProcessStatus::Completed
                    } else {
                        ProcessStatus::Failed
                    };
                    
                    // Mark for removal after some delay to allow for status queries
                    if entry.last_monitoring_update.elapsed().unwrap_or(Duration::ZERO) > Duration::from_secs(300) {
                        to_remove.push(pid);
                    }
                }
            }
            
            for pid in to_remove {
                manager.processes.remove(&pid);
                debug!("Cleaned up process entry for PID {}", pid);
            }
        }
        
        // Clean up unused IPC resources would go here
        // This is a placeholder for more sophisticated cleanup logic
    }
    
    /// Update monitoring information
    #[instrument(skip(monitor, process_manager))]
    fn update_monitoring(
        monitor: &Arc<ProcessMonitor>,
        process_manager: &Arc<RwLock<ProcessManager>>
    ) {
        let mut manager = match process_manager.write() {
            Ok(manager) => manager,
            Err(_) => return,
        };
        
        for (&pid, entry) in &mut manager.processes {
            if let Some(ref process) = entry.process {
                // This would get real process statistics in a full implementation
                // For now, we just update the timestamp
                entry.last_monitoring_update = SystemTime::now();
            }
        }
    }
    
    /// Bind IPC resource to process
    #[instrument(skip(self))]
    fn bind_ipc_to_process(&self, pid: u32, ipc_name: &str) -> Result<(), Error> {
        let mut registry = self.ipc_registry.write()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC registry lock".to_string()))?;
        
        // Add to IPC connection's bound processes
        if let Some(entry) = registry.connections.get_mut(ipc_name) {
            if !entry.bound_processes.contains(&pid) {
                entry.bound_processes.push(pid);
            }
        }
        
        // Add to process bindings
        registry.process_bindings.entry(pid).or_insert_with(Vec::new).push(ipc_name.to_string());
        
        Ok(())
    }
    
    /// Extract PID from SlayProcess
    #[instrument(skip(self, process))]
    fn extract_pid_from_process(&self, process: &SlayProcess) -> Result<(), Error> {
        // Extract the actual PID from the underlying system process handle
        process.pid()
            .ok_or_else(|| CursedError::RuntimeError("Process not running or PID unavailable".to_string()))
    }
    
    /// Notify event callbacks of process started
    #[instrument(skip(self))]
    fn notify_process_started(&self, pid: u32, command: &str) {
        if let Ok(callbacks) = self.event_callbacks.lock() {
            for callback in callbacks.iter() {
                callback.on_process_started(pid, command);
            }
        }
    }
    
    /// Notify event callbacks of IPC connection created
    #[instrument(skip(self))]
    fn notify_ipc_connection_created(&self, name: &str, connection_type: IpcConnectionType) {
        if let Ok(callbacks) = self.event_callbacks.lock() {
            for callback in callbacks.iter() {
                callback.on_ipc_connection_created(name, connection_type);
            }
        }
    }
    
    /// Add event callback
    #[instrument(skip(self, callback))]
    pub fn add_event_callback(&self, callback: Box<dyn ProcessEventCallback + Send + Sync>) -> Result<(), Error> {
        let mut callbacks = self.event_callbacks.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire callbacks lock".to_string()))?;
        
        callbacks.push(callback);
        Ok(())
    }
    
    /// Shutdown the integration system
    #[instrument(skip(self))]
    pub fn shutdown(&mut self) -> Result<(), Error> {
        info!("Shutting down ProcessIpcIntegration system");
        
        // Signal shutdown
        {
            let (lock, cvar) = &*self.shutdown_signal;
            let mut shutdown = lock.lock().unwrap();
            *shutdown = true;
            cvar.notify_all();
        }
        
        // Wait for management thread
        if let Some(handle) = self.management_thread.take() {
            handle.join().map_err(|_| CursedError::RuntimeError("Failed to join management thread".to_string()))?;
        }
        
        // Shutdown IPC manager
        {
            let mut ipc_manager = self.ipc_manager.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire IPC manager lock".to_string()))?;
            
            ipc_manager.shutdown()
                .map_err(|e| CursedError::RuntimeError(format!("Failed to shutdown IPC manager: {}", e)))?;
        }
        
        info!("ProcessIpcIntegration system shutdown complete");
        Ok(())
    }
}

impl ProcessManager {
    fn new() -> Self {
        Self {
            processes: HashMap::new(),
            next_internal_id: 1,
            command_history: Vec::new(),
            active_pipelines: HashMap::new(),
        }
    }
}

impl ProcessRegistry {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            name_to_pid: HashMap::new(),
            group_memberships: HashMap::new(),
            parent_child_map: HashMap::new(),
        }
    }
}

impl IpcRegistry {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
            named_pipes: HashMap::new(),
            shared_memory: HashMap::new(),
            message_queues: HashMap::new(),
            process_bindings: HashMap::new(),
        }
    }
}

impl Drop for ProcessIpcIntegration {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

/// Convenience function to create a new integration system with default config
pub fn create_integration_system() -> Result<(), Error> {
    ProcessIpcIntegration::new(IntegrationConfig::default())
}

/// Convenience function to create a named pipe through the integration system
pub fn create_named_pipe(name: &str) -> Result<(), Error> {
    let integration = create_integration_system()?;
    integration.create_named_pipe(name, true, None)?;
    Ok(integration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
use crate::stdlib::process::error::ProcessError;
    
    #[test]
    fn test_integration_system_creation() {
        let integration = ProcessIpcIntegration::new(IntegrationConfig::default());
        assert!(integration.is_ok());
    }
    
    #[test]
    fn test_process_registry() {
        let registry = ProcessRegistry::new();
        assert!(registry.entries.is_empty());
        assert!(registry.name_to_pid.is_empty());
    }
    
    #[test]
    fn test_ipc_registry() {
        let registry = IpcRegistry::new();
        assert!(registry.connections.is_empty());
        assert!(registry.process_bindings.is_empty());
    }
    
    #[test]
    fn test_default_config() {
        let config = IntegrationConfig::default();
        assert_eq!(config.max_processes, 1000);
        assert_eq!(config.max_ipc_connections, 500);
        assert_eq!(config.cleanup_interval, Duration::from_secs(60));
    }
    
    #[test]
    fn test_event_callback() {
        let callback = DefaultEventCallback;
        callback.on_process_started(1234, "test command");
        callback.on_process_completed(1234, 0);
        callback.on_ipc_connection_created("test", IpcConnectionType::NamedPipe);
    }
}
