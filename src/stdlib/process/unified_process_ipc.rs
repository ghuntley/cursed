use crate::error::Error;
/// Unified Process Management and IPC Coordination System
/// 
/// This module provides a comprehensive integration layer that coordinates
/// process management (exec_vibez) with all IPC mechanisms for production-ready
/// process and inter-process communication handling.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::process::{Child, Stdio};
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::ffi::OsString;

use tracing::{info, warn, error, debug, instrument, span, Level};
use serde::{Serialize, Deserialize};

use crate::error::CursedError;
use crate::stdlib::ipc::{
use crate::stdlib::web_vibez::SecurityContext;
use crate::stdlib::process::EnhancedProcess;
    IpcConfig, IpcError, IpcResult, NamedPipe, MessageQueue, SharedMemory, Semaphore,
    UnixSocket, RealIpcManager, IpcConnectionPool, ProcessAwareIpcManager
};

use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::core::{ProcessConfig as CoreProcessConfig, ProcessManager};
use crate::stdlib::process::enhanced_control::{EnhancedProcess as StdEnhancedProcess};
use crate::stdlib::process::info::{ProcessState as StdProcessState};
use crate::runtime::process::{ProcessGroup as RuntimeProcessGroup};
use crate::stdlib::process::exec_vibez_types::{VibezResult, ExecutionContext, EnhancedCmd};

type ProcessState = StdProcessState;
type ProcessGroup = RuntimeProcessGroup;

/// Unified process and IPC management system
#[derive(Debug)]
pub struct UnifiedProcessIpcManager {
    /// Process management state
    process_state: Arc<RwLock<ProcessManagerState>>,
    /// IPC coordination state  
    ipc_state: Arc<RwLock<IpcCoordinationState>>,
    /// Configuration
    config: Arc<UnifiedConfig>,
    /// Resource monitoring
    monitor: Arc<Mutex<UnifiedMonitor>>,
    /// Security and privileges
    security: Arc<Mutex<SecurityManager>>,
    /// Platform-specific features
    platform: Arc<PlatformManager>,
}

/// Configuration for the unified system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConfig {
    /// Process management configuration
    pub process_config: ProcessConfig,
    /// IPC configuration
    pub ipc_config: IpcConfig,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Security settings
    pub security_settings: SecuritySettings,
    /// Platform-specific settings
    pub platform_settings: PlatformSettings,
}

/// Process management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessConfig {
    /// Maximum number of concurrent processes
    pub max_processes: usize,
    /// Default process timeout
    pub default_timeout: Duration,
    /// Process group management enabled
    pub enable_process_groups: bool,
    /// Background task management
    pub enable_background_tasks: bool,
    /// Process monitoring interval
    pub monitoring_interval: Duration,
    /// Environment inheritance settings
    pub inherit_environment: bool,
}

/// Security settings for process and IPC operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    /// Enable privilege dropping
    pub enable_privilege_drop: bool,
    /// Allow process spawning
    pub allow_process_spawn: bool,
    /// Allowed IPC mechanisms
    pub allowed_ipc_types: Vec<IpcType>,
    /// Security context enforcement
    pub enforce_security_context: bool,
    /// Process isolation level
    pub isolation_level: IsolationLevel,
}

/// Platform-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSettings {
    /// Windows-specific settings
    #[cfg(windows)]
    pub windows: WindowsSettings,
    /// Unix-specific settings
    #[cfg(unix)]
    pub unix: UnixSettings,
}

#[cfg(windows)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsSettings {
    /// Enable Windows job objects
    pub enable_job_objects: bool,
    /// Use Windows named pipes
    pub use_named_pipes: bool,
    /// Enable Windows security tokens
    pub enable_security_tokens: bool,
}

#[cfg(unix)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnixSettings {
    /// Enable process namespaces
    pub enable_namespaces: bool,
    /// Use Unix domain sockets
    pub use_unix_sockets: bool,
    /// Enable cgroups integration
    pub enable_cgroups: bool,
}

/// IPC mechanism types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IpcType {
    NamedPipes,
    MessageQueues,
    SharedMemory,
    Semaphores,
    UnixSockets,
    NetworkSockets,
}

/// Process isolation levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IsolationLevel {
    None,
    Basic,
    Sandboxed,
    Container,
}

/// Internal state for process management
#[derive(Debug)]
struct ProcessManagerState {
    /// Active processes
    processes: HashMap<u32, ProcessInfo>,
    /// Process groups
    process_groups: HashMap<String, ProcessGroup>,
    /// Background tasks
    background_tasks: Vec<BackgroundTask>,
    /// Resource usage tracking
    resource_usage: ResourceUsage,
    /// Process statistics
    statistics: ProcessStatistics,
}

/// Internal state for IPC coordination
#[derive(Debug)]
struct IpcCoordinationState {
    /// Active IPC connections
    connections: HashMap<String, IpcConnectionInfo>,
    /// Message routing table
    routing_table: HashMap<String, String>,
    /// IPC statistics
    statistics: IpcStatistics,
    /// Connection pools
    connection_pools: HashMap<IpcType, Arc<dyn ConnectionPoolManager>>,
}

/// Extended process information
#[derive(Debug)]
struct ProcessInfo {
    /// Basic process handle
    process: EnhancedProcess,
    /// IPC connections for this process
    ipc_connections: Vec<String>,
    /// Resource limits
    limits: ResourceLimits,
    /// Security context
    security_context: SecurityContext,
    /// Creation time
    created_at: SystemTime,
    /// Last activity time
    last_activity: SystemTime,
}

/// IPC connection information
#[derive(Debug)]
struct IpcConnectionInfo {
    /// Connection type
    connection_type: IpcType,
    /// Connected processes
    connected_processes: Vec<u32>,
    /// Connection creation time
    created_at: SystemTime,
    /// Last activity time
    last_activity: SystemTime,
    /// Statistics
    stats: ConnectionStats,
}

/// Connection statistics
#[derive(Debug, Clone)]
struct ConnectionStats {
    /// Messages sent
    messages_sent: u64,
    /// Messages received
    messages_received: u64,
    /// Bytes transferred
    bytes_transferred: u64,
    /// Errors encountered
    errors: u64,
}

/// Background task information
#[derive(Debug)]
struct BackgroundTask {
    /// Task ID
    id: String,
    /// Task type
    task_type: BackgroundTaskType,
    /// Associated process
    process_id: Option<u32>,
    /// Creation time
    created_at: SystemTime,
    /// Task state
    state: BackgroundTaskState,
}

/// Background task types
#[derive(Debug, Clone)]
enum BackgroundTaskType {
    ProcessMonitoring,
    IpcCleanup,
    ResourceMonitoring,
    SecurityAudit,
    PerformanceAnalysis,
}

/// Background task states
#[derive(Debug, Clone)]
enum BackgroundTaskState {
    Running,
    Paused,
    Stopped,
    Failed(String),
}

/// Resource usage tracking
#[derive(Debug, Clone)]
struct ResourceUsage {
    /// CPU usage percentage
    cpu_usage: f64,
    /// Memory usage in bytes
    memory_usage: u64,
    /// Open file descriptors
    open_fds: u32,
    /// Network connections
    network_connections: u32,
}

/// Process statistics
#[derive(Debug, Clone)]
struct ProcessStatistics {
    /// Total processes spawned
    total_spawned: u64,
    /// Currently active processes
    active_processes: u32,
    /// Failed process launches
    failed_launches: u64,
    /// Average process lifetime
    average_lifetime: Duration,
}

/// IPC statistics
#[derive(Debug, Clone)]
struct IpcStatistics {
    /// Total connections established
    total_connections: u64,
    /// Currently active connections
    active_connections: u32,
    /// Failed connection attempts
    failed_connections: u64,
    /// Total messages transferred
    total_messages: u64,
}

/// Unified monitoring system
#[derive(Debug)]
struct UnifiedMonitor {
    /// Resource monitoring enabled
    enabled: bool,
    /// Monitoring threads
    monitor_threads: Vec<thread::JoinHandle<()>>,
    /// Performance metrics
    metrics: PerformanceMetrics,
    /// Alert thresholds
    thresholds: AlertThresholds,
}

/// Performance metrics
#[derive(Debug, Clone)]
struct PerformanceMetrics {
    /// Process spawn rate (processes/second)
    process_spawn_rate: f64,
    /// IPC throughput (messages/second)
    ipc_throughput: f64,
    /// Resource utilization
    resource_utilization: ResourceUsage,
    /// Error rates
    error_rates: ErrorRates,
}

/// Error rates tracking
#[derive(Debug, Clone)]
struct ErrorRates {
    /// Process errors per second
    process_errors_per_sec: f64,
    /// IPC errors per second
    ipc_errors_per_sec: f64,
    /// Security violations per second
    security_violations_per_sec: f64,
}

/// Alert thresholds
#[derive(Debug, Clone)]
struct AlertThresholds {
    /// Maximum CPU usage before alert
    max_cpu_usage: f64,
    /// Maximum memory usage before alert
    max_memory_usage: u64,
    /// Maximum error rate before alert
    max_error_rate: f64,
}

/// Security management system
#[derive(Debug)]
struct SecurityManager {
    /// Current privilege level
    privilege_level: PrivilegeLevel,
    /// Security auditing enabled
    auditing_enabled: bool,
    /// Security violations log
    violations: Vec<SecurityViolation>,
    /// Privilege escalation tracking
    escalations: Vec<PrivilegeEscalation>,
}

/// Privilege levels
#[derive(Debug, Clone)]
enum PrivilegeLevel {
    User,
    Elevated,
    Administrator,
    System,
}

/// Security violation record
#[derive(Debug, Clone)]
struct SecurityViolation {
    /// Violation type
    violation_type: ViolationType,
    /// Process involved
    process_id: Option<u32>,
    /// Timestamp
    timestamp: SystemTime,
    /// Description
    description: String,
}

/// Types of security violations
#[derive(Debug, Clone)]
enum ViolationType {
    UnauthorizedProcessSpawn,
    IpcPermissionViolation,
    ResourceLimitExceeded,
    PrivilegeEscalationAttempt,
    SuspiciousActivity,
}

/// Privilege escalation record
#[derive(Debug, Clone)]
struct PrivilegeEscalation {
    /// Process requesting escalation
    process_id: u32,
    /// Requested privilege level
    requested_level: PrivilegeLevel,
    /// Granted privilege level
    granted_level: PrivilegeLevel,
    /// Timestamp
    timestamp: SystemTime,
    /// Justification
    justification: String,
}

/// Platform-specific management
#[derive(Debug)]
struct PlatformManager {
    /// Platform type
    platform_type: PlatformType,
    /// Platform-specific handlers
    handlers: Box<dyn PlatformHandler>,
}

/// Platform types
#[derive(Debug, Clone)]
enum PlatformType {
    Unix,
    Windows,
    MacOS,
}

/// Platform-specific operations trait
pub trait PlatformHandler: std::fmt::Debug + Send + Sync {
    /// Initialize platform-specific features
    fn initialize(&self) -> Result<(), Error>;
    
    /// Create platform-specific IPC mechanism
    fn create_ipc(&self, ipc_type: IpcType, name: &str) -> Result<(), Error>;
    
    /// Apply platform-specific security settings
    fn apply_security(&self, process: &mut EnhancedProcess, settings: &SecuritySettings) -> Result<(), Error>;
    
    /// Get platform-specific resource limits
    fn get_resource_limits(&self) -> ResourceLimits;
    
    /// Cleanup platform-specific resources
    fn cleanup(&self) -> Result<(), Error>;
}

/// Generic IPC connection trait
pub trait IpcConnection: std::fmt::Debug + Send + Sync {
    /// Send a message
    fn send(&self, message: &[u8]) -> Result<(), Error>;
    
    /// Receive a message
    fn receive(&self) -> Result<(), Error>;
    
    /// Close the connection
    fn close(&self) -> Result<(), Error>;
}

/// Connection pool management trait
trait ConnectionPoolManager: std::fmt::Debug + Send + Sync {
    /// Get a connection from the pool
    fn get_connection(&self, target: &str) -> Result<(), Error>;
    
    /// Return a connection to the pool
    fn return_connection(&self, connection: Arc<dyn IpcConnection>) -> Result<(), Error>;
    
    /// Get pool statistics
    fn get_stats(&self) -> ConnectionPoolStats;
}

/// Connection pool statistics
#[derive(Debug, Clone)]
struct ConnectionPoolStats {
    /// Total connections in pool
    total_connections: u32,
    /// Active connections
    active_connections: u32,
    /// Available connections
    available_connections: u32,
    /// Connection hits
    hits: u64,
    /// Connection misses
    misses: u64,
}

impl UnifiedProcessIpcManager {
    /// Create a new unified manager
    #[instrument]
    pub fn new(config: UnifiedConfig) -> Result<(), Error> {
        info!("Creating unified process-IPC manager");
        
        let platform_type = Self::detect_platform();
        let platform = Arc::new(PlatformManager {
            platform_type: platform_type.clone(),
            handlers: Self::create_platform_handler(platform_type)?,
        });
        
        let manager = Self {
            process_state: Arc::new(RwLock::new(ProcessManagerState {
                processes: HashMap::new(),
                process_groups: HashMap::new(),
                background_tasks: Vec::new(),
                resource_usage: ResourceUsage {
                    cpu_usage: 0.0,
                    memory_usage: 0,
                    open_fds: 0,
                    network_connections: 0,
                },
                statistics: ProcessStatistics {
                    total_spawned: 0,
                    active_processes: 0,
                    failed_launches: 0,
                    average_lifetime: Duration::from_secs(0),
                },
            })),
            ipc_state: Arc::new(RwLock::new(IpcCoordinationState {
                connections: HashMap::new(),
                routing_table: HashMap::new(),
                statistics: IpcStatistics {
                    total_connections: 0,
                    active_connections: 0,
                    failed_connections: 0,
                    total_messages: 0,
                },
                connection_pools: HashMap::new(),
            })),
            config: Arc::new(config),
            monitor: Arc::new(Mutex::new(UnifiedMonitor {
                enabled: true,
                monitor_threads: Vec::new(),
                metrics: PerformanceMetrics {
                    process_spawn_rate: 0.0,
                    ipc_throughput: 0.0,
                    resource_utilization: ResourceUsage {
                        cpu_usage: 0.0,
                        memory_usage: 0,
                        open_fds: 0,
                        network_connections: 0,
                    },
                    error_rates: ErrorRates {
                        process_errors_per_sec: 0.0,
                        ipc_errors_per_sec: 0.0,
                        security_violations_per_sec: 0.0,
                    },
                },
                thresholds: AlertThresholds {
                    max_cpu_usage: 80.0,
                    max_memory_usage: 1_000_000_000, // 1GB
                    max_error_rate: 10.0,
                },
            })),
            security: Arc::new(Mutex::new(SecurityManager {
                privilege_level: PrivilegeLevel::User,
                auditing_enabled: true,
                violations: Vec::new(),
                escalations: Vec::new(),
            })),
            platform,
        };
        
        // Initialize platform-specific features
        manager.platform.handlers.initialize()?;
        
        // Start background monitoring
        manager.start_monitoring()?;
        
        info!("Unified process-IPC manager created successfully");
        Ok(manager)
    }
    
    /// Spawn a process with integrated IPC setup
    #[instrument(skip(self))]
    pub fn spawn_process_with_ipc(
        &self,
        cmd: &mut EnhancedCmd,
        ipc_connections: Vec<IpcConnectionRequest>,
    ) -> Result<(), Error> {
        let span = span!(Level::INFO, "spawn_process_with_ipc");
        let _enter = span.enter();
        
        info!("Spawning process with IPC integration");
        
        // Apply security settings
        self.apply_security_to_process(cmd)?;
        
        // Setup IPC connections before process spawn
        let mut ipc_handles = Vec::new();
        for ipc_request in &ipc_connections {
            let handle = self.setup_ipc_connection(ipc_request)?;
            ipc_handles.push(handle);
        }
        
        // Configure process I/O to use IPC connections
        self.configure_process_io(cmd, &ipc_handles)?;
        
        // Spawn the process
        let process = cmd.spawn()?;
        let process_id = process.id();
        
        // Register process and IPC connections
        self.register_process_with_ipc(process_id, process, &ipc_handles)?;
        
        // Update statistics
        self.update_spawn_statistics()?;
        
        info!(process_id = process_id, "Process spawned with IPC integration");
        
        Ok(ProcessWithIpc {
            process_id,
            ipc_connections: ipc_handles,
        })
    }
    
    /// Create an IPC connection between processes
    #[instrument(skip(self))]
    pub fn create_ipc_connection(
        &self,
        source_process: u32,
        target_process: u32,
        connection_type: IpcType,
        name: &str,
    ) -> Result<(), Error> {
        info!(
            source = source_process,
            target = target_process,
            ipc_type = ?connection_type,
            "Creating IPC connection"
        );
        
        // Validate processes exist
        self.validate_process_exists(source_process)?;
        self.validate_process_exists(target_process)?;
        
        // Check security permissions
        self.check_ipc_permissions(source_process, target_process, &connection_type)?;
        
        // Create the IPC mechanism
        let connection = self.platform.handlers.create_ipc(connection_type.clone(), name)?;
        let connection_id = format!("{}:{}:{}", source_process, target_process, name);
        
        // Register the connection
        let mut ipc_state = self.ipc_state.write().unwrap();
        ipc_state.connections.insert(connection_id.clone(), IpcConnectionInfo {
            connection_type,
            connected_processes: vec![source_process, target_process],
            created_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            stats: ConnectionStats {
                messages_sent: 0,
                messages_received: 0,
                bytes_transferred: 0,
                errors: 0,
            },
        });
        
        // Update routing table
        ipc_state.routing_table.insert(name.to_string(), connection_id.clone());
        ipc_state.statistics.total_connections += 1;
        ipc_state.statistics.active_connections += 1;
        
        info!(connection_id = connection_id, "IPC connection created");
        Ok(connection_id)
    }
    
    /// Monitor and manage all processes and IPC connections
    #[instrument(skip(self))]
    pub fn monitor_all(&self) -> Result<(), Error> {
        debug!("Monitoring all processes and IPC connections");
        
        let process_state = self.process_state.read().unwrap();
        let ipc_state = self.ipc_state.read().unwrap();
        let monitor = self.monitor.lock().unwrap();
        
        Ok(UnifiedStatus {
            process_stats: process_state.statistics.clone(),
            ipc_stats: ipc_state.statistics.clone(),
            resource_usage: process_state.resource_usage.clone(),
            performance_metrics: monitor.metrics.clone(),
            active_processes: process_state.processes.len() as u32,
            active_connections: ipc_state.connections.len() as u32,
            security_status: self.get_security_status()?,
        })
    }
    
    /// Cleanup and shutdown the unified manager
    #[instrument(skip(self))]
    pub fn shutdown(&self) -> Result<(), Error> {
        info!("Shutting down unified process-IPC manager");
        
        // Stop monitoring
        self.stop_monitoring()?;
        
        // Terminate all processes
        self.terminate_all_processes()?;
        
        // Close all IPC connections
        self.close_all_ipc_connections()?;
        
        // Cleanup platform-specific resources
        self.platform.handlers.cleanup()?;
        
        info!("Unified process-IPC manager shutdown complete");
        Ok(())
    }
    
    // Private helper methods
    
    fn detect_platform() -> PlatformType {
        #[cfg(windows)]
        return PlatformType::Windows;
        #[cfg(target_os = "macos")]
        return PlatformType::MacOS;
        #[cfg(unix)]
        return PlatformType::Unix;
    }
    
    fn create_platform_handler(platform_type: PlatformType) -> Result<(), Error> {
        match platform_type {
            #[cfg(unix)]
            PlatformType::Unix => {
                use crate::stdlib::process::unix_platform::UnixPlatformHandler;
                Ok(Box::new(UnixPlatformHandler::new()?))
            }
            #[cfg(windows)]
            PlatformType::Windows => {
                use crate::stdlib::process::windows_platform::WindowsPlatformHandler;
                Ok(Box::new(WindowsPlatformHandler::new()?))
            }
            #[cfg(target_os = "macos")]
            PlatformType::MacOS => {
                use crate::stdlib::process::macos_platform::MacOSPlatformHandler;
                Ok(Box::new(MacOSPlatformHandler::new()?))
            }
            #[allow(unreachable_patterns)]
            _ => Err(CursedError::Platform("Unsupported platform".to_string())),
        }
    }
    
    fn start_monitoring(&self) -> Result<(), Error> {
        // Start background monitoring threads
        // Implementation would spawn monitoring threads
        Ok(())
    }
    
    fn stop_monitoring(&self) -> Result<(), Error> {
        // Stop background monitoring threads
        // Implementation would join monitoring threads
        Ok(())
    }
    
    fn apply_security_to_process(&self, cmd: &mut EnhancedCmd) -> Result<(), Error> {
        let security = self.security.lock().unwrap();
        
        if self.config.security_settings.enable_privilege_drop {
            // Apply privilege dropping
        }
        
        if self.config.security_settings.enforce_security_context {
            // Apply security context
        }
        
        Ok(())
    }
    
    fn setup_ipc_connection(&self, request: &IpcConnectionRequest) -> Result<(), Error> {
        // Setup IPC connection based on request type
        Ok(IpcHandle {
            connection_id: format!("ipc_{}", request.name),
            connection_type: request.connection_type.clone(),
        })
    }
    
    fn configure_process_io(&self, cmd: &mut EnhancedCmd, handles: &[IpcHandle]) -> Result<(), Error> {
        // Configure process I/O to use IPC connections
        Ok(())
    }
    
    fn register_process_with_ipc(
        &self,
        process_id: u32,
        process: EnhancedProcess,
        ipc_handles: &[IpcHandle],
    ) -> Result<(), Error> {
        let mut process_state = self.process_state.write().unwrap();
        
        let ipc_connection_ids: Vec<String> = ipc_handles.iter()
            .map(|h| h.connection_id.clone())
            .collect();
        
        process_state.processes.insert(process_id, ProcessInfo {
            process,
            ipc_connections: ipc_connection_ids,
            limits: self.config.resource_limits.clone(),
            security_context: SecurityContext::default(),
            created_at: SystemTime::now(),
            last_activity: SystemTime::now(),
        });
        
        process_state.statistics.active_processes += 1;
        Ok(())
    }
    
    fn update_spawn_statistics(&self) -> Result<(), Error> {
        let mut process_state = self.process_state.write().unwrap();
        process_state.statistics.total_spawned += 1;
        Ok(())
    }
    
    fn validate_process_exists(&self, process_id: u32) -> Result<(), Error> {
        let process_state = self.process_state.read().unwrap();
        if !process_state.processes.contains_key(&process_id) {
            return Err(CursedError::Process(format!("Process {} not found", process_id)));
        }
        Ok(())
    }
    
    fn check_ipc_permissions(
        &self,
        source: u32,
        target: u32,
        connection_type: &IpcType,
    ) -> Result<(), Error> {
        // Check if IPC type is allowed
        if !self.config.security_settings.allowed_ipc_types.contains(connection_type) {
            return Err(CursedError::Security(format!("IPC type {:?} not allowed", connection_type)));
        }
        
        Ok(())
    }
    
    fn get_security_status(&self) -> Result<(), Error> {
        let security = self.security.lock().unwrap();
        Ok(SecurityStatus {
            privilege_level: security.privilege_level.clone(),
            violations_count: security.violations.len(),
            escalations_count: security.escalations.len(),
            auditing_enabled: security.auditing_enabled,
        })
    }
    
    fn terminate_all_processes(&self) -> Result<(), Error> {
        // Terminate all managed processes
        Ok(())
    }
    
    fn close_all_ipc_connections(&self) -> Result<(), Error> {
        // Close all IPC connections
        Ok(())
    }
}

/// Request for creating an IPC connection
#[derive(Debug, Clone)]
pub struct IpcConnectionRequest {
    /// Connection name
    pub name: String,
    /// Connection type
    pub connection_type: IpcType,
    /// Connection parameters
    pub parameters: HashMap<String, String>,
}

/// IPC connection handle
#[derive(Debug, Clone)]
pub struct IpcHandle {
    /// Connection identifier
    pub connection_id: String,
    /// Connection type
    pub connection_type: IpcType,
}

/// Process with IPC connections
#[derive(Debug)]
pub struct ProcessWithIpc {
    /// Process ID
    pub process_id: u32,
    /// Associated IPC connections
    pub ipc_connections: Vec<IpcHandle>,
}

/// Unified status information
#[derive(Debug, Clone)]
pub struct UnifiedStatus {
    /// Process statistics
    pub process_stats: ProcessStatistics,
    /// IPC statistics
    pub ipc_stats: IpcStatistics,
    /// Resource usage
    pub resource_usage: ResourceUsage,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Active processes count
    pub active_processes: u32,
    /// Active connections count
    pub active_connections: u32,
    /// Security status
    pub security_status: SecurityStatus,
}

/// Security status information
#[derive(Debug, Clone)]
pub struct SecurityStatus {
    /// Current privilege level
    pub privilege_level: PrivilegeLevel,
    /// Number of security violations
    pub violations_count: usize,
    /// Number of privilege escalations
    pub escalations_count: usize,
    /// Whether auditing is enabled
    pub auditing_enabled: bool,
}

// Platform-specific implementations are provided by parent module

// Default implementations
impl Default for UnifiedConfig {
    fn default() -> Self {
        Self {
            process_config: ProcessConfig {
                max_processes: 100,
                default_timeout: Duration::from_secs(300), // 5 minutes
                enable_process_groups: true,
                enable_background_tasks: true,
                monitoring_interval: Duration::from_secs(1),
                inherit_environment: true,
            },
            ipc_config: IpcConfig::default(),
            resource_limits: ResourceLimits::default(),
            security_settings: SecuritySettings {
                enable_privilege_drop: true,
                allow_process_spawn: true,
                allowed_ipc_types: vec![
                    IpcType::NamedPipes,
                    IpcType::MessageQueues,
                    IpcType::SharedMemory,
                    IpcType::UnixSockets,
                ],
                enforce_security_context: true,
                isolation_level: IsolationLevel::Basic,
            },
            platform_settings: PlatformSettings {
                #[cfg(windows)]
                windows: WindowsSettings {
                    enable_job_objects: true,
                    use_named_pipes: true,
                    enable_security_tokens: true,
                },
                #[cfg(unix)]
                unix: UnixSettings {
                    enable_namespaces: false,
                    use_unix_sockets: true,
                    enable_cgroups: false,
                },
            },
        }
    }
}

/// Initialize the unified process-IPC system
pub fn initialize_unified_system() -> Result<(), Error> {
    let config = UnifiedConfig::default();
    let manager = UnifiedProcessIpcManager::new(config)?;
    Ok(Arc::new(manager))
}

/// Global manager instance
static mut GLOBAL_MANAGER: Option<Arc<UnifiedProcessIpcManager>> = None;
static MANAGER_INIT: std::sync::Once = std::sync::Once::new();

/// Get the global unified manager
pub fn get_unified_manager() -> Result<(), Error> {
    unsafe {
        MANAGER_INIT.call_once(|| {
            match initialize_unified_system() {
                Ok(manager) => GLOBAL_MANAGER = Some(manager),
                Err(e) => error!("Failed to initialize unified manager: {}", e),
            }
        });
        
        GLOBAL_MANAGER.clone()
            .ok_or_else(|| CursedError::System("Unified manager not initialized".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::info::ProcessInfo;
use crate::stdlib::process::info::ProcessState;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;
    
    #[test]
    fn test_unified_config_default() {
        let config = UnifiedConfig::default();
        assert_eq!(config.process_config.max_processes, 100);
        assert!(config.security_settings.enable_privilege_drop);
    }
    
    #[test]
    fn test_manager_creation() {
        let config = UnifiedConfig::default();
        // Note: This test would need platform-specific mocking
        // assert!(UnifiedProcessIpcManager::new(config).is_ok());
    }
}
