/// Enhanced process control and management operations
/// 
/// This module provides comprehensive process control functionality including advanced
/// process management, hierarchy tracking, resource monitoring, and lifecycle management.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime};
use std::thread;

use crate::stdlib::process::error::{
    ProcessResult, ProcessError, process_not_found_pid, permission_denied_pid,
    invalid_state, execution_failed, timeout_error, system_error
};
use crate::stdlib::process::core::{ProcessConfig, Process};
use crate::stdlib::process::control::{Signal, Priority};
use crate::stdlib::process::fork::{JobControlManager, fork_process, exec_program};
use crate::stdlib::process::resource_limits::{ResourceLimitManager, ResourceType, ResourceLimit};
use crate::stdlib::process::privileges::{PrivilegeManager, SecureEnvironment};

/// Options for enhanced process control operations
#[derive(Debug, Clone)]
pub struct EnhancedControlOptions {
    /// Maximum number of concurrent processes
    pub max_concurrent: usize,
    /// Default timeout for process operations
    pub default_timeout: Duration,
    /// Enable resource monitoring
    pub enable_monitoring: bool,
    /// Enable process hierarchy tracking
    pub enable_hierarchy: bool,
    /// Maximum retries for failed operations
    pub max_retries: u32,
}

impl Default for EnhancedControlOptions {
    fn default() -> Self {
        Self {
            max_concurrent: 100,
            default_timeout: Duration::from_secs(30),
            enable_monitoring: true,
            enable_hierarchy: true,
            max_retries: 3,
        }
    }
}

/// Enhanced process controller with comprehensive management capabilities
pub struct EnhancedProcessController {
    /// Map of managed processes
    processes: Arc<RwLock<HashMap<u32, EnhancedProcessInfo>>>,
    /// Process hierarchy tracking
    hierarchy: Arc<RwLock<ProcessHierarchy>>,
    /// Global process statistics
    statistics: Arc<Mutex<ProcessStatistics>>,
    /// Controller configuration
    config: ProcessControllerConfig,
    /// Running processes handles
    running_processes: Arc<Mutex<HashMap<u32, Process>>>,
    /// Process event callbacks
    event_callbacks: Arc<RwLock<Vec<Box<dyn ProcessEventCallback>>>>,
    /// Job control manager for process groups
    job_control: Arc<Mutex<JobControlManager>>,
    /// Resource limit manager
    resource_manager: Arc<Mutex<ResourceLimitManager>>,
    /// Privilege manager
    privilege_manager: Arc<Mutex<Option<PrivilegeManager>>>,
}

/// Enhanced process wrapper with advanced management capabilities
#[derive(Debug)]
pub struct EnhancedProcess {
    /// Process information
    pub info: EnhancedProcessInfo,
    /// Process handle
    pub handle: Option<std::process::Child>,
    /// Configuration
    pub config: ProcessConfig,
    /// Resource monitoring
    pub monitor: Option<Arc<Mutex<ProcessMonitor>>>,
    /// Status tracking
    pub status_tracker: Arc<Mutex<ProcessStatusTracker>>,
}

impl EnhancedProcess {
    pub fn new(pid: u32, handle: Option<std::process::Child>) -> ProcessResult<Self> {
        Ok(Self {
            info: EnhancedProcessInfo::new(pid),
            handle,
            config: ProcessConfig::default(),
            monitor: None,
            status_tracker: Arc::new(Mutex::new(ProcessStatusTracker::new(pid))),
        })
    }
    
    pub fn pid(&self) -> u32 {
        self.info.pid
    }
    
    pub fn status(&self) -> ProcessStatus {
        self.info.status
    }
}

/// Process status tracking
#[derive(Debug)]
pub struct ProcessStatusTracker {
    pub pid: u32,
    pub status: ProcessStatus,
    pub last_update: Instant,
}

impl ProcessStatusTracker {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            status: ProcessStatus::Creating,
            last_update: Instant::now(),
        }
    }
}

/// Process monitor for resource tracking
#[derive(Debug)]
pub struct ProcessMonitor {
    pub pid: u32,
    pub enabled: bool,
    pub last_check: Instant,
}

/// Enhanced process information with comprehensive metadata
#[derive(Debug, Clone)]
pub struct EnhancedProcessInfo {
    pub pid: u32,
    pub internal_id: u64,
    pub command: String,
    pub args: Vec<String>,
    pub status: ProcessStatus,
    pub start_time: Instant,
    pub parent_pid: Option<u32>,
    pub children: HashSet<u32>,
    pub working_dir: Option<String>,
    pub environment: HashMap<String, String>,
    pub resource_usage: ResourceUsage,
    pub exit_info: Option<ProcessExitInfo>,
    pub process_group: Option<u32>,
    pub session_id: Option<u32>,
    pub priority: Priority,
    pub metadata: HashMap<String, String>,
    pub io_statistics: IoStatistics,
    pub security_context: SecurityContext,
    pub resource_limits: ResourceLimits,
}

impl EnhancedProcessInfo {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            internal_id: 0,
            command: String::new(),
            args: Vec::new(),
            status: ProcessStatus::Creating,
            start_time: Instant::now(),
            parent_pid: None,
            children: HashSet::new(),
            working_dir: None,
            environment: HashMap::new(),
            resource_usage: ResourceUsage::default(),
            exit_info: None,
            process_group: None,
            session_id: None,
            priority: Priority::default(),
            metadata: HashMap::new(),
            io_statistics: IoStatistics::default(),
            security_context: SecurityContext::default(),
            resource_limits: ResourceLimits::default(),
        }
    }
}

/// Process status enumeration with detailed states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    /// Process is currently running
    Running,
    /// Process has been stopped via signal
    Stopped,
    /// Process is paused/suspended
    Paused,
    /// Process has finished but hasn't been reaped
    Zombie,
    /// Process has terminated normally
    Terminated,
    /// Process was killed by signal
    Killed,
    /// Process status is unknown
    Unknown,
    /// Process is being created
    Creating,
    /// Process is being destroyed
    Destroying,
    /// Process is waiting for I/O
    Waiting,
    /// Process is in uninterruptible sleep
    Uninterruptible,
}

/// Resource usage tracking
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    pub cpu_time: Duration,
    pub memory_bytes: u64,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
}

/// I/O statistics tracking
#[derive(Debug, Clone, Default)]
pub struct IoStatistics {
    pub read_ops: u64,
    pub write_ops: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
}

/// Security context information
#[derive(Debug, Clone, Default)]
pub struct SecurityContext {
    pub user_id: Option<u32>,
    pub group_id: Option<u32>,
    pub capabilities: Vec<String>,
}

/// Resource limits configuration
#[derive(Debug, Clone, Default)]
pub struct ResourceLimits {
    pub max_memory: Option<u64>,
    pub max_cpu_time: Option<Duration>,
    pub max_file_descriptors: Option<u32>,
}

/// Process exit information
#[derive(Debug, Clone)]
pub struct ProcessExitInfo {
    pub exit_code: i32,
    pub signal: Option<i32>,
    pub exit_time: Instant,
    pub reason: ExitReason,
}

#[derive(Debug, Clone)]
pub enum ExitReason {
    Normal,
    Signal(i32),
    Error(String),
}

// Priority enum is imported from control module to avoid duplication

/// Process hierarchy tracking
#[derive(Debug, Clone)]
pub struct ProcessHierarchy {
    /// Parent-child relationships
    parent_child: HashMap<u32, HashSet<u32>>,
    /// Child-parent relationships  
    child_parent: HashMap<u32, u32>,
    /// Process groups
    groups: HashMap<u32, HashSet<u32>>,
    /// Session leaders
    sessions: HashMap<u32, HashSet<u32>>,
    /// Process creation order
    creation_order: Vec<u32>,
}

/// Controller configuration
#[derive(Debug, Clone)]
pub struct ProcessControllerConfig {
    /// Maximum number of processes to manage
    pub max_processes: usize,
    /// Process monitoring interval
    pub monitoring_interval: Duration,
    /// Enable resource monitoring
    pub enable_resource_monitoring: bool,
    /// Process cleanup timeout
    pub cleanup_timeout: Duration,
    /// Enable signal handling
    pub enable_signal_handling: bool,
    /// Enable process hierarchy tracking
    pub enable_hierarchy_tracking: bool,
    /// Enable I/O monitoring
    pub enable_io_monitoring: bool,
    /// Enable security context tracking
    pub enable_security_tracking: bool,
    /// Resource usage history size
    pub resource_history_size: usize,
    /// Enable automatic cleanup
    pub enable_auto_cleanup: bool,
}

/// Global process statistics
#[derive(Debug, Clone, Default)]
pub struct ProcessStatistics {
    pub total_processes_created: u64,
    pub total_processes_destroyed: u64,
    pub active_processes: u64,
    pub peak_processes: u64,
    pub total_cpu_time: Duration,
    pub total_memory_used: u64,
    pub signal_count: u64,
    pub failed_operations: u64,
    pub total_io_bytes: u64,
    pub average_process_lifetime: Duration,
    pub memory_pressure_events: u64,
    pub resource_limit_violations: u64,
}

/// Process event callback trait
pub trait ProcessEventCallback: Send + Sync {
    /// Called when a process is created
    fn on_process_created(&self, info: &EnhancedProcessInfo) -> ProcessResult<()>;
    
    /// Called when a process exits
    fn on_process_exited(&self, info: &EnhancedProcessInfo, exit_info: &ProcessExitInfo) -> ProcessResult<()>;
    
    /// Called when a process status changes
    fn on_status_changed(&self, info: &EnhancedProcessInfo, old_status: ProcessStatus, new_status: ProcessStatus) -> ProcessResult<()>;
    
    /// Called when resource limits are exceeded
    fn on_resource_limit_exceeded(&self, info: &EnhancedProcessInfo, resource: &str, limit: u64, current: u64) -> ProcessResult<()>;
    
    /// Called on process errors
    fn on_process_error(&self, pid: u32, error: &ProcessError) -> ProcessResult<()>;
}

impl EnhancedProcessController {
    /// Create a new enhanced process controller with default configuration
    pub fn new() -> Self {
        Self::with_config(ProcessControllerConfig::default())
    }

    /// Create a new enhanced process controller with custom configuration
    pub fn with_config(config: ProcessControllerConfig) -> Self {
        let privilege_manager = PrivilegeManager::new().ok();
        
        let controller = Self {
            processes: Arc::new(RwLock::new(HashMap::new())),
            hierarchy: Arc::new(RwLock::new(ProcessHierarchy::new())),
            statistics: Arc::new(Mutex::new(ProcessStatistics::default())),
            running_processes: Arc::new(Mutex::new(HashMap::new())),
            event_callbacks: Arc::new(RwLock::new(Vec::new())),
            job_control: Arc::new(Mutex::new(JobControlManager::new())),
            resource_manager: Arc::new(Mutex::new(ResourceLimitManager::new())),
            privilege_manager: Arc::new(Mutex::new(privilege_manager)),
            config,
        };

        // Start monitoring thread if enabled
        if controller.config.enable_resource_monitoring {
            controller.start_monitoring_thread();
        }

        // Start cleanup thread if enabled
        if controller.config.enable_auto_cleanup {
            controller.start_cleanup_thread();
        }

        controller
    }

    /// Spawn a new process with the given configuration
    pub fn spawn_process(&self, config: ProcessConfig) -> ProcessResult<u32> {
        // Check process limit
        {
            let processes = self.processes.read().unwrap();
            if processes.len() >= self.config.max_processes {
                return Err(system_error(
                    -1,
                    "spawn_process",
                    "Maximum process limit reached"
                ));
            }
        }

        // Create the process
        let mut process = crate::stdlib::process::core::spawn_process(config.clone())?;
        let pid = process.id();

        // Create enhanced process info
        let process_info = EnhancedProcessInfo {
            pid,
            internal_id: self.generate_internal_id(),
            command: config.command.clone(),
            args: config.args.clone(),
            status: ProcessStatus::Running,
            start_time: Instant::now(),
            parent_pid: self.get_current_pid(),
            children: HashSet::new(),
            working_dir: config.working_dir.as_ref().map(|p| p.to_string_lossy().to_string()),
            environment: config.env_vars.clone(),
            resource_usage: ResourceUsage::default(),
            exit_info: None,
            process_group: config.process_group,
            session_id: None,
            priority: Priority::Normal,
            metadata: HashMap::new(),
            io_statistics: IoStatistics::default(),
            security_context: SecurityContext::default(),
            resource_limits: ResourceLimits::default(),
        };

        // Store process info
        {
            let mut processes = self.processes.write().unwrap();
            processes.insert(pid, process_info.clone());
        }

        // Store running process handle
        {
            let mut running = self.running_processes.lock().unwrap();
            running.insert(pid, process);
        }

        // Update hierarchy if enabled
        if self.config.enable_hierarchy_tracking {
            let mut hierarchy = self.hierarchy.write().unwrap();
            hierarchy.add_process(pid, process_info.parent_pid);
            if let Some(pgid) = config.process_group {
                hierarchy.add_to_group(pid, pgid);
            }
        }

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.total_processes_created += 1;
            stats.active_processes += 1;
            if stats.active_processes > stats.peak_processes {
                stats.peak_processes = stats.active_processes;
            }
        }

        // Notify callbacks
        self.notify_process_created(&process_info)?;

        Ok(pid)
    }

    /// Kill a process by PID
    pub fn kill_process(&self, pid: u32) -> ProcessResult<()> {
        self.send_signal(pid, Signal::SIGKILL)
    }

    /// Terminate a process gracefully
    pub fn terminate_process(&self, pid: u32) -> ProcessResult<()> {
        self.send_signal(pid, Signal::SIGTERM)
    }

    /// Pause a process (send SIGSTOP)
    pub fn pause_process(&self, pid: u32) -> ProcessResult<()> {
        self.send_signal(pid, Signal::SIGSTOP)?;
        self.update_process_status(pid, ProcessStatus::Paused)?;
        Ok(())
    }

    /// Resume a paused process (send SIGCONT)
    pub fn resume_process(&self, pid: u32) -> ProcessResult<()> {
        self.send_signal(pid, Signal::SIGCONT)?;
        self.update_process_status(pid, ProcessStatus::Running)?;
        Ok(())
    }

    /// Send a signal to a process
    pub fn send_signal(&self, pid: u32, signal: Signal) -> ProcessResult<()> {
        // Verify process exists in our management
        if !self.process_exists(pid) {
            return Err(process_not_found_pid(pid, "Process not found in controller"));
        }

        // Use existing signal sending implementation
        crate::stdlib::process::control::send_signal_to_pid(pid, signal)?;

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.signal_count += 1;
        }

        Ok(())
    }

    /// Wait for a process to complete
    pub fn wait_for_process(&self, pid: u32) -> ProcessResult<ProcessExitInfo> {
        self.wait_for_process_timeout(pid, None)
    }

    /// Wait for a process with timeout
    pub fn wait_for_process_timeout(&self, pid: u32, timeout: Option<Duration>) -> ProcessResult<ProcessExitInfo> {
        let start_time = Instant::now();
        
        loop {
            // Check if process has already exited
            if let Some(exit_info) = self.get_exit_info(pid)? {
                return Ok(exit_info);
            }

            // Check timeout
            if let Some(timeout) = timeout {
                if start_time.elapsed() >= timeout {
                    return Err(timeout_error(
                        "wait_for_process",
                        timeout,
                        &format!("Process {} did not exit within timeout", pid)
                    ));
                }
            }

            // Check if process still exists
            if !self.process_exists(pid) {
                // Process disappeared, create exit info
                let exit_info = ProcessExitInfo {
                    exit_code: None,
                    signal: None,
                    exit_time: SystemTime::now(),
                    core_dumped: false,
                    total_runtime: start_time.elapsed(),
                    total_cpu_time: Duration::default(),
                    peak_memory_usage: 0,
                };
                
                // Update process info with exit info
                self.mark_process_exited(pid, exit_info.clone())?;
                
                return Ok(exit_info);
            }

            // Sleep briefly before checking again
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Get process information
    pub fn get_process_info(&self, pid: u32) -> ProcessResult<EnhancedProcessInfo> {
        let processes = self.processes.read().unwrap();
        processes.get(&pid).cloned()
            .ok_or_else(|| process_not_found_pid(pid, "Process not found"))
    }

    /// Update process information
    pub fn update_process_info(&self, pid: u32, info: EnhancedProcessInfo) -> ProcessResult<()> {
        let mut processes = self.processes.write().unwrap();
        if processes.contains_key(&pid) {
            processes.insert(pid, info);
            Ok(())
        } else {
            Err(process_not_found_pid(pid, "Process not found"))
        }
    }

    /// List all managed processes
    pub fn list_processes(&self) -> Vec<EnhancedProcessInfo> {
        let processes = self.processes.read().unwrap();
        processes.values().cloned().collect()
    }

    /// List processes by status
    pub fn list_processes_by_status(&self, status: ProcessStatus) -> Vec<EnhancedProcessInfo> {
        let processes = self.processes.read().unwrap();
        processes.values()
            .filter(|info| info.status == status)
            .cloned()
            .collect()
    }

    /// Get process children
    pub fn get_process_children(&self, pid: u32) -> ProcessResult<Vec<u32>> {
        if !self.config.enable_hierarchy_tracking {
            return Ok(Vec::new());
        }
        
        let hierarchy = self.hierarchy.read().unwrap();
        Ok(hierarchy.get_children(pid))
    }

    /// Get process parent
    pub fn get_process_parent(&self, pid: u32) -> ProcessResult<Option<u32>> {
        if !self.config.enable_hierarchy_tracking {
            return Ok(None);
        }
        
        let hierarchy = self.hierarchy.read().unwrap();
        Ok(hierarchy.get_parent(pid))
    }

    /// Kill process and all children
    pub fn kill_process_tree(&self, pid: u32) -> ProcessResult<Vec<u32>> {
        let mut killed_processes = Vec::new();
        
        if self.config.enable_hierarchy_tracking {
            let children = self.get_process_children(pid)?;
            
            // Kill children first (depth-first)
            for child_pid in children {
                let mut child_killed = self.kill_process_tree(child_pid)?;
                killed_processes.append(&mut child_killed);
            }
        }
        
        // Kill the process itself
        self.kill_process(pid)?;
        killed_processes.push(pid);
        
        Ok(killed_processes)
    }

    /// Create process group
    pub fn create_process_group(&self, leader_pid: u32, pgid: Option<u32>) -> ProcessResult<u32> {
        let job_control = self.job_control.lock()
            .map_err(|_| system_error(-1, "create_process_group", "Failed to lock job control"))?;
        job_control.create_process_group(leader_pid, pgid)
    }

    /// Add process to existing group
    pub fn add_to_process_group(&self, pid: u32, pgid: u32) -> ProcessResult<()> {
        let job_control = self.job_control.lock()
            .map_err(|_| system_error(-1, "add_to_process_group", "Failed to lock job control"))?;
        job_control.add_to_group(pid, pgid)
    }

    /// Set resource limit for a resource type
    pub fn set_resource_limit(&self, resource: ResourceType, limit: ResourceLimit) -> ProcessResult<()> {
        let mut resource_manager = self.resource_manager.lock()
            .map_err(|_| system_error(-1, "set_resource_limit", "Failed to lock resource manager"))?;
        resource_manager.set_limit(resource, limit)
    }

    /// Apply secure environment to current process
    pub fn apply_secure_environment(&self, env: &SecureEnvironment) -> ProcessResult<()> {
        env.apply()
    }

    /// Drop privileges using privilege manager
    pub fn drop_privileges(&self, target_uid: u32, target_gid: u32) -> ProcessResult<()> {
        let mut privilege_manager_opt = self.privilege_manager.lock()
            .map_err(|_| system_error(-1, "drop_privileges", "Failed to lock privilege manager"))?;
        
        if let Some(ref mut privilege_manager) = *privilege_manager_opt {
            privilege_manager.drop_privileges(target_uid, target_gid)
        } else {
            Err(system_error(-1, "drop_privileges", "Privilege manager not available"))
        }
    }

    /// Fork and exec a new process with enhanced control
    pub fn fork_exec_process<S: AsRef<str>>(&self, program: S, args: &[S], env: Option<&[(S, S)]>) -> ProcessResult<u32> {
        let fork_result = fork_process()?;
        
        if fork_result.is_parent {
            // Parent process - return child PID
            let child_pid = fork_result.child_pid.unwrap();
            
            // Register the child process for monitoring
            let process_info = EnhancedProcessInfo {
                pid: child_pid,
                internal_id: self.generate_internal_id(),
                command: program.as_ref().to_string(),
                args: args.iter().map(|s| s.as_ref().to_string()).collect(),
                status: ProcessStatus::Running,
                start_time: fork_result.fork_time,
                parent_pid: Some(std::process::id()),
                children: HashSet::new(),
                working_dir: None,
                environment: HashMap::new(),
                resource_usage: ResourceUsage::default(),
                exit_info: None,
                process_group: None,
                session_id: None,
                priority: Priority::Normal,
                metadata: HashMap::new(),
                io_statistics: IoStatistics::default(),
                security_context: SecurityContext::default(),
                resource_limits: crate::stdlib::process::enhanced_control::ResourceLimits::default(),
            };

            {
                let mut processes = self.processes.write().unwrap();
                processes.insert(child_pid, process_info);
            }

            Ok(child_pid)
        } else {
            // Child process - exec the program
            if let Err(e) = exec_program(program, args, env) {
                eprintln!("Child exec failed: {}", e);
                std::process::exit(1);
            }
            
            // This line should never be reached
            std::process::exit(0);
        }
    }

    /// Set process priority  
    pub fn set_process_priority(&self, pid: u32, priority: Priority) -> ProcessResult<()> {
        // Use existing priority setting implementation
        crate::stdlib::process::control::set_process_priority(pid, priority)?;

        // Update our internal info
        {
            let mut processes = self.processes.write().unwrap();
            if let Some(info) = processes.get_mut(&pid) {
                info.priority = priority;
            } else {
                return Err(process_not_found_pid(pid, "Process not found"));
            }
        }

        Ok(())
    }

    /// Set resource limits for a process
    pub fn set_resource_limits(&self, pid: u32, limits: ResourceLimits) -> ProcessResult<()> {
        let mut processes = self.processes.write().unwrap();
        if let Some(info) = processes.get_mut(&pid) {
            info.resource_limits = limits;
            Ok(())
        } else {
            Err(process_not_found_pid(pid, "Process not found"))
        }
    }

    /// Check if resource limits are exceeded
    pub fn check_resource_limits(&self, pid: u32) -> ProcessResult<Vec<String>> {
        let processes = self.processes.read().unwrap();
        if let Some(info) = processes.get(&pid) {
            let mut violations = Vec::new();
            
            if let Some(max_memory) = info.resource_limits.max_memory {
                if info.resource_usage.memory_rss > max_memory {
                    violations.push(format!("Memory limit exceeded: {} > {}", info.resource_usage.memory_rss, max_memory));
                }
            }
            
            if let Some(max_files) = info.resource_limits.max_open_files {
                if info.resource_usage.open_files > max_files {
                    violations.push(format!("Open files limit exceeded: {} > {}", info.resource_usage.open_files, max_files));
                }
            }
            
            if let Some(max_threads) = info.resource_limits.max_threads {
                if info.resource_usage.threads > max_threads {
                    violations.push(format!("Threads limit exceeded: {} > {}", info.resource_usage.threads, max_threads));
                }
            }
            
            Ok(violations)
        } else {
            Err(process_not_found_pid(pid, "Process not found"))
        }
    }

    /// Add process event callback
    pub fn add_event_callback(&self, callback: Box<dyn ProcessEventCallback>) {
        let mut callbacks = self.event_callbacks.write().unwrap();
        callbacks.push(callback);
    }

    /// Remove all event callbacks
    pub fn clear_event_callbacks(&self) {
        let mut callbacks = self.event_callbacks.write().unwrap();
        callbacks.clear();
    }

    /// Get controller statistics
    pub fn get_statistics(&self) -> ProcessStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// Check if a process exists and is managed by this controller
    pub fn process_exists(&self, pid: u32) -> bool {
        let processes = self.processes.read().unwrap();
        processes.contains_key(&pid)
    }

    /// Clean up completed processes
    pub fn cleanup_completed_processes(&self) -> ProcessResult<usize> {
        let mut cleanup_count = 0;
        let completed_pids: Vec<u32>;
        
        // Find completed processes
        {
            let processes = self.processes.read().unwrap();
            completed_pids = processes.values()
                .filter(|info| matches!(info.status, ProcessStatus::Terminated | ProcessStatus::Zombie | ProcessStatus::Killed))
                .map(|info| info.pid)
                .collect();
        }

        // Remove completed processes
        for pid in completed_pids {
            if self.remove_process(pid).is_ok() {
                cleanup_count += 1;
            }
        }

        Ok(cleanup_count)
    }

    /// Get detailed process tree
    pub fn get_process_tree(&self, root_pid: u32) -> ProcessResult<Vec<EnhancedProcessInfo>> {
        if !self.config.enable_hierarchy_tracking {
            return Ok(vec![self.get_process_info(root_pid)?]);
        }
        
        let mut tree = Vec::new();
        let mut to_visit = vec![root_pid];
        
        while let Some(pid) = to_visit.pop() {
            if let Ok(info) = self.get_process_info(pid) {
                tree.push(info);
                
                // Add children to visit list
                if let Ok(children) = self.get_process_children(pid) {
                    to_visit.extend(children);
                }
            }
        }
        
        Ok(tree)
    }

    // Helper methods

    fn update_process_status(&self, pid: u32, new_status: ProcessStatus) -> ProcessResult<()> {
        let old_status;
        {
            let mut processes = self.processes.write().unwrap();
            if let Some(info) = processes.get_mut(&pid) {
                old_status = info.status;
                info.status = new_status;
            } else {
                return Err(process_not_found_pid(pid, "Process not found"));
            }
        }

        // Notify callbacks of status change
        if let Ok(info) = self.get_process_info(pid) {
            self.notify_status_changed(&info, old_status, new_status)?;
        }

        Ok(())
    }

    fn get_exit_info(&self, pid: u32) -> ProcessResult<Option<ProcessExitInfo>> {
        let processes = self.processes.read().unwrap();
        if let Some(info) = processes.get(&pid) {
            Ok(info.exit_info.clone())
        } else {
            Err(process_not_found_pid(pid, "Process not found"))
        }
    }

    fn mark_process_exited(&self, pid: u32, exit_info: ProcessExitInfo) -> ProcessResult<()> {
        {
            let mut processes = self.processes.write().unwrap();
            if let Some(info) = processes.get_mut(&pid) {
                info.exit_info = Some(exit_info.clone());
                info.status = if exit_info.signal.is_some() {
                    ProcessStatus::Killed
                } else {
                    ProcessStatus::Terminated
                };
            } else {
                return Err(process_not_found_pid(pid, "Process not found"));
            }
        }

        // Remove from running processes
        {
            let mut running = self.running_processes.lock().unwrap();
            running.remove(&pid);
        }

        // Notify callbacks
        if let Ok(info) = self.get_process_info(pid) {
            self.notify_process_exited(&info, &exit_info)?;
        }

        Ok(())
    }

    fn generate_internal_id(&self) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        NEXT_ID.fetch_add(1, Ordering::SeqCst)
    }

    fn get_current_pid(&self) -> Option<u32> {
        Some(std::process::id())
    }

    fn remove_process(&self, pid: u32) -> ProcessResult<()> {
        // Remove from processes map
        {
            let mut processes = self.processes.write().unwrap();
            if processes.remove(&pid).is_none() {
                return Err(process_not_found_pid(pid, "Process not found"));
            }
        }

        // Remove from running processes
        {
            let mut running = self.running_processes.lock().unwrap();
            running.remove(&pid);
        }

        // Remove from hierarchy if enabled
        if self.config.enable_hierarchy_tracking {
            let mut hierarchy = self.hierarchy.write().unwrap();
            hierarchy.remove_process(pid);
        }

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.active_processes = stats.active_processes.saturating_sub(1);
            stats.total_processes_destroyed += 1;
        }

        Ok(())
    }

    fn start_monitoring_thread(&self) {
        let processes = Arc::clone(&self.processes);
        let interval = self.config.monitoring_interval;
        
        thread::spawn(move || {
            loop {
                thread::sleep(interval);
                
                // Update resource usage for all processes
                let pids: Vec<u32> = {
                    let procs = processes.read().unwrap();
                    procs.keys().cloned().collect()
                };
                
                for pid in pids {
                    if let Ok(usage) = get_process_resource_usage(pid) {
                        let mut procs = processes.write().unwrap();
                        if let Some(info) = procs.get_mut(&pid) {
                            info.resource_usage = usage;
                        }
                    }
                }
            }
        });
    }

    fn start_cleanup_thread(&self) {
        let controller_weak = Arc::downgrade(&Arc::new(self.clone()));
        let interval = self.config.cleanup_timeout;
        
        thread::spawn(move || {
            loop {
                thread::sleep(interval);
                
                if let Some(controller) = controller_weak.upgrade() {
                    let _ = controller.cleanup_completed_processes();
                } else {
                    break; // Controller dropped
                }
            }
        });
    }

    // Callback notification methods

    fn notify_process_created(&self, info: &EnhancedProcessInfo) -> ProcessResult<()> {
        let callbacks = self.event_callbacks.read().unwrap();
        for callback in callbacks.iter() {
            if let Err(e) = callback.on_process_created(info) {
                eprintln!("Process created callback error: {:?}", e);
            }
        }
        Ok(())
    }

    fn notify_process_exited(&self, info: &EnhancedProcessInfo, exit_info: &ProcessExitInfo) -> ProcessResult<()> {
        let callbacks = self.event_callbacks.read().unwrap();
        for callback in callbacks.iter() {
            if let Err(e) = callback.on_process_exited(info, exit_info) {
                eprintln!("Process exited callback error: {:?}", e);
            }
        }
        Ok(())
    }

    fn notify_status_changed(&self, info: &EnhancedProcessInfo, old_status: ProcessStatus, new_status: ProcessStatus) -> ProcessResult<()> {
        let callbacks = self.event_callbacks.read().unwrap();
        for callback in callbacks.iter() {
            if let Err(e) = callback.on_status_changed(info, old_status, new_status) {
                eprintln!("Status changed callback error: {:?}", e);
            }
        }
        Ok(())
    }
}

impl Clone for EnhancedProcessController {
    fn clone(&self) -> Self {
        Self {
            processes: Arc::clone(&self.processes),
            hierarchy: Arc::clone(&self.hierarchy),
            statistics: Arc::clone(&self.statistics),
            running_processes: Arc::clone(&self.running_processes),
            event_callbacks: Arc::clone(&self.event_callbacks),
            job_control: Arc::clone(&self.job_control),
            resource_manager: Arc::clone(&self.resource_manager),
            privilege_manager: Arc::clone(&self.privilege_manager),
            config: self.config.clone(),
        }
    }
}

impl ProcessHierarchy {
    fn new() -> Self {
        Self {
            parent_child: HashMap::new(),
            child_parent: HashMap::new(),
            groups: HashMap::new(),
            sessions: HashMap::new(),
            creation_order: Vec::new(),
        }
    }

    fn add_process(&mut self, pid: u32, parent_pid: Option<u32>) {
        self.creation_order.push(pid);
        
        if let Some(parent) = parent_pid {
            self.parent_child.entry(parent).or_default().insert(pid);
            self.child_parent.insert(pid, parent);
        }
    }

    fn remove_process(&mut self, pid: u32) {
        // Remove from creation order
        self.creation_order.retain(|&p| p != pid);
        
        // Remove from parent-child relationships
        if let Some(parent) = self.child_parent.remove(&pid) {
            if let Some(children) = self.parent_child.get_mut(&parent) {
                children.remove(&pid);
            }
        }
        
        // Remove as parent and orphan children
        if let Some(children) = self.parent_child.remove(&pid) {
            for child in children {
                self.child_parent.remove(&child);
                // Could implement re-parenting to init process here
            }
        }
        
        // Remove from groups
        for (_, group_members) in self.groups.iter_mut() {
            group_members.remove(&pid);
        }
        
        // Remove from sessions
        for (_, session_members) in self.sessions.iter_mut() {
            session_members.remove(&pid);
        }
    }

    fn get_children(&self, pid: u32) -> Vec<u32> {
        self.parent_child.get(&pid)
            .map(|children| children.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn get_parent(&self, pid: u32) -> Option<u32> {
        self.child_parent.get(&pid).cloned()
    }

    fn add_to_group(&mut self, pid: u32, group_id: u32) {
        self.groups.entry(group_id).or_default().insert(pid);
    }

    fn get_group_members(&self, group_id: u32) -> Vec<u32> {
        self.groups.get(&group_id)
            .map(|members| members.iter().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for ProcessControllerConfig {
    fn default() -> Self {
        Self {
            max_processes: 1000,
            monitoring_interval: Duration::from_secs(5),
            enable_resource_monitoring: true,
            cleanup_timeout: Duration::from_secs(30),
            enable_signal_handling: true,
            enable_hierarchy_tracking: true,
            enable_io_monitoring: true,
            enable_security_tracking: true,
            resource_history_size: 100,
            enable_auto_cleanup: true,
        }
    }
}

/// Get enhanced resource usage for a specific process
fn get_process_resource_usage(pid: u32) -> ProcessResult<ResourceUsage> {
    #[cfg(unix)]
    {
        use std::fs;
        
        let mut usage = ResourceUsage::default();
        
        // Read from /proc/{pid}/stat for basic info
        let stat_path = format!("/proc/{}/stat", pid);
        if let Ok(stat_content) = fs::read_to_string(&stat_path) {
            let parts: Vec<&str> = stat_content.split_whitespace().collect();
            if parts.len() >= 24 {
                // Parse CPU times (in clock ticks)
                let user_time = parts[13].parse::<u64>().unwrap_or(0);
                let system_time = parts[14].parse::<u64>().unwrap_or(0);
                let vsize = parts[22].parse::<u64>().unwrap_or(0);
                let rss = parts[23].parse::<u64>().unwrap_or(0);
                
                // Convert clock ticks to duration (assuming 100 Hz)
                let clock_ticks_per_sec = 100;
                usage.cpu_time_user = Duration::from_millis(user_time * 1000 / clock_ticks_per_sec);
                usage.cpu_time_system = Duration::from_millis(system_time * 1000 / clock_ticks_per_sec);
                
                // RSS is in pages, convert to bytes (assuming 4KB pages)
                usage.memory_rss = rss * 4096;
                usage.memory_vms = vsize;
            }
        }
        
        // Read I/O statistics from /proc/{pid}/io
        let io_path = format!("/proc/{}/io", pid);
        if let Ok(io_content) = fs::read_to_string(&io_path) {
            for line in io_content.split("\n") {
                if let Some((key, value)) = line.split_once(':') {
                    let value = value.trim().parse::<u64>().unwrap_or(0);
                    match key {
                        "read_bytes" => usage.total_read_bytes = value,
                        "write_bytes" => usage.total_write_bytes = value,
                        "syscr" => usage.syscalls_read = value,
                        "syscw" => usage.syscalls_write = value,
                        _ => {}
                    }
                }
            }
        }
        
        // Count open files
        usage.open_files = count_open_files(pid);
        usage.network_connections = count_network_connections(pid);
        usage.threads = count_threads(pid);
        usage.last_updated = Some(SystemTime::now());
        
        // Calculate CPU percentage from stat file
        if let Ok(stat_content) = std::fs::read_to_string(&format!("/proc/{}/stat", pid)) {
            let fields: Vec<&str> = stat_content.split_whitespace().collect();
            if fields.len() > 21 {
                // utime (14th field) + stime (15th field) = total CPU time
                if let (Ok(utime), Ok(stime)) = (fields[13].parse::<u64>(), fields[14].parse::<u64>()) {
                    let total_time = utime + stime;
                    let clock_ticks = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as u64;
                    let seconds = total_time / clock_ticks;
                    let process_uptime = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    usage.cpu_percent = if process_uptime > 0 {
                        (seconds as f64 / process_uptime as f64) * 100.0
                    } else {
                        0.0
                    };
                }
            }
        }
        
        // Calculate memory percentage from meminfo
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            let mut total_memory_kb = 0u64;
            for line in meminfo.split("\n") {
                if line.starts_with("MemTotal:") {
                    if let Some(value) = line.split_whitespace().nth(1) {
                        if let Ok(mem) = value.parse::<u64>() {
                            total_memory_kb = mem;
                            break;
                        }
                    }
                }
            }
            if total_memory_kb > 0 && usage.memory_rss > 0 {
                usage.memory_percent = (usage.memory_rss as f64 / (total_memory_kb * 1024) as f64) * 100.0;
            }
        }
        
        return Ok(usage);
    }
    
    #[cfg(windows)]
    {
        // Windows implementation using basic process information
        use std::process::Command;
        
        let mut usage = ResourceUsage::default();
        usage.last_updated = Some(SystemTime::now());
        
        // Use tasklist command to get process information
        if let Ok(output) = Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid), "/FO", "CSV"])
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.split("\n").skip(1) { // Skip header
                    let fields: Vec<&str> = line.split(',').collect();
                    if fields.len() >= 5 {
                        // Parse memory usage (typically in KB)
                        let memory_str = fields[4].trim_matches('"').replace(",", "");
                        if let Some(memory_part) = memory_str.split(' ').next() {
                            if let Ok(memory_kb) = memory_part.parse::<u64>() {
                                usage.memory_rss = memory_kb * 1024; // Convert to bytes
                                usage.memory_vms = usage.memory_rss; // Approximation
                            }
                        }
                    }
                }
            }
        }
        
        // Get total system memory for percentage calculation
        if let Ok(output) = Command::new("wmic")
            .args(&["OS", "get", "TotalVisibleMemorySize", "/value"])
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.split("\n") {
                    if line.starts_with("TotalVisibleMemorySize=") {
                        if let Some(value_str) = line.split('=').nth(1) {
                            if let Ok(total_kb) = value_str.trim().parse::<u64>() {
                                if total_kb > 0 && usage.memory_rss > 0 {
                                    usage.memory_percent = (usage.memory_rss as f64 / (total_kb * 1024) as f64) * 100.0;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        return Ok(usage);
    }
    
    Ok(ResourceUsage::default())
}

#[cfg(unix)]
fn count_open_files(pid: u32) -> u32 {
    use std::fs;
    
    let fd_path = format!("/proc/{}/fd", pid);
    if let Ok(entries) = fs::read_dir(&fd_path) {
        entries.count() as u32
    } else {
        0
    }
}

#[cfg(unix)]
fn count_network_connections(pid: u32) -> u32 {
    let mut count = 0;
    
    // Count TCP connections
    count += count_connections_for_protocol(pid, "tcp");
    count += count_connections_for_protocol(pid, "tcp6");
    
    // Count UDP connections
    count += count_connections_for_protocol(pid, "udp");
    count += count_connections_for_protocol(pid, "udp6");
    
    count
}

fn count_connections_for_protocol(pid: u32, protocol: &str) -> u32 {
    use std::fs;
    use std::collections::HashSet;
    
    let mut process_inodes = HashSet::new();
    
    // Get all inodes for the process file descriptors
    let fd_path = format!("/proc/{}/fd", pid);
    if let Ok(entries) = fs::read_dir(&fd_path) {
        for entry in entries.flatten() {
            if let Ok(link) = fs::read_link(entry.path()) {
                if let Some(link_str) = link.to_str() {
                    if link_str.starts_with("socket:[") && link_str.ends_with(']') {
                        if let Some(inode_str) = link_str.strip_prefix("socket:[").and_then(|s| s.strip_suffix(']')) {
                            if let Ok(inode) = inode_str.parse::<u64>() {
                                process_inodes.insert(inode);
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Count connections in /proc/net/{protocol}
    let net_path = format!("/proc/net/{}", protocol);
    if let Ok(content) = fs::read_to_string(&net_path) {
        let mut count = 0;
        for line in content.split("\n").skip(1) { // Skip header
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() > 9 {
                // The inode is typically in the 10th field (index 9)
                if let Ok(inode) = fields[9].parse::<u64>() {
                    if process_inodes.contains(&inode) {
                        count += 1;
                    }
                }
            }
        }
        count
    } else {
        0
    }
}

#[cfg(unix)]
fn count_threads(pid: u32) -> u32 {
    use std::fs;
use crate::stdlib::process::info::ProcessInfo;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;
use crate::stdlib::process::enhanced_control::EnhancedProcessInfo;
    
    let task_path = format!("/proc/{}/task", pid);
    if let Ok(entries) = fs::read_dir(&task_path) {
        entries.count() as u32
    } else {
        1 // At least the main thread
    }
}

#[cfg(windows)]
fn count_open_files(_pid: u32) -> u32 { 0 }
#[cfg(windows)]
fn count_network_connections(_pid: u32) -> u32 { 0 }
#[cfg(windows)]
fn count_threads(_pid: u32) -> u32 { 1 }

/// Example process event callback implementation
pub struct DefaultProcessEventCallback;

impl ProcessEventCallback for DefaultProcessEventCallback {
    fn on_process_created(&self, info: &EnhancedProcessInfo) -> ProcessResult<()> {
        println!("Process created: PID {} ({})", info.pid, info.command);
        Ok(())
    }
    
    fn on_process_exited(&self, info: &EnhancedProcessInfo, exit_info: &ProcessExitInfo) -> ProcessResult<()> {
        println!("Process exited: PID {} ({}) - exit code: {:?}, runtime: {:?}", 
                 info.pid, info.command, exit_info.exit_code, exit_info.total_runtime);
        Ok(())
    }
    
    fn on_status_changed(&self, info: &EnhancedProcessInfo, old_status: ProcessStatus, new_status: ProcessStatus) -> ProcessResult<()> {
        println!("Process status changed: PID {} ({}) - {:?} -> {:?}", 
                 info.pid, info.command, old_status, new_status);
        Ok(())
    }
    
    fn on_resource_limit_exceeded(&self, info: &EnhancedProcessInfo, resource: &str, limit: u64, current: u64) -> ProcessResult<()> {
        println!("Resource limit exceeded: PID {} ({}) - {} limit {} exceeded with {}", 
                 info.pid, info.command, resource, limit, current);
        Ok(())
    }
    
    fn on_process_error(&self, pid: u32, error: &ProcessError) -> ProcessResult<()> {
        println!("Process error: PID {} - {:?}", pid, error);
        Ok(())
    }
}
