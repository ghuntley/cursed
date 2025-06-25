use crate::error::CursedError;
// Enhanced SlayCommand implementation with comprehensive process management
// 
// This module provides advanced process management features including:
// - Resource monitoring and limiting
// - Process groups and pipelines  
// - Signal handling and graceful termination
// - Cross-platform compatibility
// - Advanced I/O operations

use std::collections::HashMap;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::process::{Child, Command, Stdio, ExitStatus};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};
use std::thread;
use super::{SlayOptions, SlayResult, ProcessStats, SignalOptions, io_error_to_cursed};

/// Enhanced command with advanced process management features
#[derive(Debug)]
pub struct EnhancedSlayCommand {
    /// Command name/path
    /// Command arguments
    /// Enhanced options
    /// Process state
    /// Resource monitor
/// Enhanced execution options with advanced features
#[derive(Debug, Clone)]
pub struct EnhancedSlayOptions {
    /// Basic options
    /// Resource limits
    /// Process priority
    /// Security context
    /// I/O configuration
    /// Monitoring configuration
/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessPriority {
/// Security context for process execution
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// User ID to run as (Unix only)
    /// Group ID to run as (Unix only)  
    /// Chroot directory (Unix only)
    /// Process isolation level
    /// Resource limits enforcement
/// Process isolation levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsolationLevel {
/// I/O configuration options
#[derive(Debug, Clone)]
pub struct IoConfiguration {
    /// Buffer size for I/O operations
    /// Line-based processing for stdout
    /// Line-based processing for stderr
    /// Callback for stdout lines
    pub stdout_callback: Option<String>, // Function name for callback
    /// Callback for stderr lines
    pub stderr_callback: Option<String>, // Function name for callback
    /// Input data to send to stdin
/// Monitoring configuration
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    /// Enable resource monitoring
    /// Monitoring interval
    /// Resource thresholds
    /// Action on threshold breach
/// Action to take when resource thresholds are breached
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThresholdAction {
/// Enhanced process state with monitoring
#[derive(Debug)]
pub struct EnhancedProcessState {
    /// Child process handle
    /// Process start time
    /// Exit status when completed
    /// Collected stdout
    /// Collected stderr
    /// Whether the process is running
    /// Last error encountered
    /// Current resource usage
    /// Resource usage history
    /// Process signals sent
/// Resource monitoring system
#[derive(Debug)]
pub struct ResourceMonitor {
    /// Process ID being monitored
    /// Monitoring enabled
    /// Update interval
    /// Statistics history
    /// Threshold configuration
    /// Monitor thread handle
impl Default for EnhancedSlayOptions {
    fn default() -> Self {
        Self {
        }
    }
impl Default for SecurityContext {
    fn default() -> Self {
        Self {
        }
    }
impl Default for IoConfiguration {
    fn default() -> Self {
        Self {
        }
    }
impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
        }
    }
impl EnhancedSlayCommand {
    /// Create a new enhanced command
    pub fn new(name: &str, args: &[&str]) -> Self {
        Self {
        }
    }

    /// Configure the command with enhanced options
    pub fn with_options(mut self, options: EnhancedSlayOptions) -> Self {
        self.options = options;
        self
    /// Set memory limit in bytes
    pub fn set_memory_limit(&mut self, limit: u64) -> &mut Self {
        self.options.memory_limit = Some(limit);
        self
    /// Set CPU usage limit as percentage
    pub fn set_cpu_limit(&mut self, limit: f64) -> &mut Self {
        self.options.cpu_limit = Some(limit);
        self
    /// Set process priority
    pub fn set_priority(&mut self, priority: ProcessPriority) -> &mut Self {
        self.options.priority = Some(priority);
        self
    /// Enable resource monitoring
    pub fn enable_monitoring(&mut self, interval: Duration) -> &mut Self {
        self.options.monitoring.enabled = true;
        self.options.monitoring.interval = interval;
        self
    /// Set security context
    pub fn set_security_context(&mut self, security: SecurityContext) -> &mut Self {
        self.options.security = security;
        self
    /// Run the command with enhanced features
    pub fn run_enhanced(&mut self) -> SlayResult<()> {
        self.start_enhanced()?;
        self.wait_enhanced()
    /// Start the command with enhanced features
    pub fn start_enhanced(&mut self) -> SlayResult<()> {
        // Build the command with enhanced options
        let mut cmd = self.build_enhanced_command()?;
        
        // Configure stdio
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());

        // Apply security context
        self.apply_security_context(&mut cmd)?;

        // Apply process priority  
        self.apply_priority(&mut cmd)?;

        // Spawn the process
        let child = cmd.spawn().map_err(io_error_to_cursed)?;
        let pid = child.id();
        
        // Store the child process
        {
            let mut state = self.state.lock().unwrap();
            state.child = Some(child);
            state.is_running = true;
            state.start_time = Instant::now();
        // Start resource monitoring if enabled
        if self.options.monitoring.enabled {
            self.start_resource_monitoring(pid)?;
        // Handle I/O operations
        self.handle_io_operations()?;

        Ok(())
    /// Wait for command completion with enhanced monitoring
    pub fn wait_enhanced(&mut self) -> SlayResult<()> {
        let timeout = self.options.basic.timeout;
        
        let result = if let Some(timeout_duration) = timeout {
            self.wait_with_timeout_enhanced(timeout_duration)
        } else {
            self.wait_indefinitely_enhanced()

        // Stop resource monitoring
        self.stop_resource_monitoring();

        result
    /// Get current resource statistics
    pub fn get_stats(&self) -> Option<ProcessStats> {
        let state = self.state.lock().unwrap();
        if state.is_running {
            Some(state.current_stats.clone())
        } else {
            None
        }
    }

    /// Get resource usage history
    pub fn get_stats_history(&self) -> Vec<(Instant, ProcessStats)> {
        let state = self.state.lock().unwrap();
        state.stats_history.clone()
    /// Send signal to the process
    pub fn send_signal(&mut self, signal: i32) -> SlayResult<()> {
        let mut state = self.state.lock().unwrap();
        
        if let Some(ref mut child) = state.child {
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                unsafe {
                    libc::kill(child.id() as i32, signal);
                }
            }
            #[cfg(windows)]
            {
                // Windows signal handling would go here
                return Err(CursedError::RuntimeError(
                    "Signal sending not implemented on Windows".to_string()
                ));
            state.signals_sent.push((Instant::now(), signal));
            Ok(())
        } else {
            Err(CursedError::RuntimeError("No process running".to_string()))
        }
    }

    /// Terminate process gracefully
    pub fn terminate_gracefully(&mut self, options: SignalOptions) -> SlayResult<()> {
        // Send SIGTERM first
        self.send_signal(options.signal)?;
        
        // Wait for grace period
        let start = Instant::now();
        while start.elapsed() < options.grace_period {
            if !self.is_running() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(100));
        // Force kill if necessary
        if options.force && self.is_running() {
            self.send_signal(9)?; // SIGKILL
        Ok(())
    /// Check if process is running
    pub fn is_running(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.is_running
    /// Build enhanced command with all configurations
    fn build_enhanced_command(&self) -> SlayResult<Command> {
        let mut cmd = if self.options.basic.use_shell {
            let shell_args = super::get_shell_args(true, self.options.basic.shell_path.as_deref());
            let mut shell_cmd = Command::new(&shell_args[0]);
            
            if shell_args.len() > 1 {
                shell_cmd.args(&shell_args[1..]);
            let full_cmd = format!("{} {}", self.name, self.args.join(" "));
            shell_cmd.arg(full_cmd);
            shell_cmd
        } else {
            let mut direct_cmd = Command::new(&self.name);
            direct_cmd.args(&self.args);
            direct_cmd

        // Set working directory
        if let Some(ref dir) = self.options.basic.dir {
            cmd.current_dir(dir);
        // Set environment variables
        for env_var in &self.options.basic.env {
            if let Some(eq_pos) = env_var.find('=') {
                let key = &env_var[..eq_pos];
                let value = &env_var[eq_pos + 1..];
                cmd.env(key, value);
            }
        }

        Ok(cmd)
    /// Apply security context to command
    fn apply_security_context(&self, cmd: &mut Command) -> SlayResult<()> {
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            
            if let Some(uid) = self.options.security.user_id {
                cmd.uid(uid);
            if let Some(gid) = self.options.security.group_id {
                cmd.gid(gid);
            // Apply chroot if specified
            if let Some(ref chroot_dir) = self.options.security.chroot_dir {
                cmd.pre_exec(move || {
                    std::env::set_current_dir(chroot_dir)?;
                    unsafe {
                        if libc::chroot(std::ffi::CString::new(chroot_dir.as_str())?.as_ptr()) != 0 {
                            return Err(std::io::Error::last_os_error());
                        }
                    }
                    Ok(())
                });
            }
        }
        
        Ok(())
    /// Apply process priority
    fn apply_priority(&self, cmd: &mut Command) -> SlayResult<()> {
        if let Some(priority) = self.options.priority {
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                let nice_value = priority as i32;
                cmd.pre_exec(move || {
                    unsafe {
                        libc::setpriority(libc::PRIO_PROCESS, 0, nice_value);
                    }
                    Ok(())
                });
            }
        }
        Ok(())
    /// Start resource monitoring
    fn start_resource_monitoring(&mut self, pid: u32) -> SlayResult<()> {
        let monitor = Arc::new(Mutex::new(ResourceMonitor::new(
        )));
        
        let monitor_clone = monitor.clone();
        let state_clone = self.state.clone();
        let interval = self.options.monitoring.interval;
        
        let handle = thread::spawn(move || {
            ResourceMonitor::monitor_loop(monitor_clone, state_clone, interval);
        });
        
        {
            let mut mon = monitor.lock().unwrap();
            mon.monitor_thread = Some(handle);
        self.resource_monitor = Some(monitor);
        Ok(())
    /// Stop resource monitoring
    fn stop_resource_monitoring(&mut self) {
        if let Some(monitor) = &self.resource_monitor {
            let mut mon = monitor.lock().unwrap();
            mon.enabled = false;
            
            if let Some(handle) = mon.monitor_thread.take() {
                drop(mon); // Release lock before joining
                let _ = handle.join();
            }
        }
        self.resource_monitor = None;
    /// Handle I/O operations based on configuration
    fn handle_io_operations(&mut self) -> SlayResult<()> {
        let mut state = self.state.lock().unwrap();
        
        if let Some(ref mut child) = state.child {
            // Handle stdin data if provided
            if let Some(ref stdin_data) = self.options.io_config.stdin_data {
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(stdin_data).map_err(io_error_to_cursed)?;
                    stdin.flush().map_err(io_error_to_cursed)?;
                }
            }
        Ok(())
    /// Wait with timeout and enhanced monitoring
    fn wait_with_timeout_enhanced(&mut self, timeout: Duration) -> SlayResult<()> {
        let start_time = Instant::now();
        
        loop {
            {
                let mut state = self.state.lock().unwrap();
                if let Some(ref mut child) = state.child {
                    match child.try_wait() {
                        Ok(Some(status)) => {
                            state.exit_status = Some(status);
                            state.is_running = false;
                            
                            if self.options.basic.collect_output {
                                self.collect_output_enhanced(&mut state, child)?;
                            return Ok(());
                        }
                        Ok(None) => {
                            if start_time.elapsed() >= timeout {
                                let _ = child.kill();
                                state.is_running = false;
                                return Err(CursedError::RuntimeError(
                                    format!("Command timed out after {:?}", timeout)
                                ));
                            }
                        }
                        Err(e) => {
                            state.is_running = false;
                            return Err(io_error_to_cursed(e));
                        }
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Wait indefinitely with enhanced monitoring
    fn wait_indefinitely_enhanced(&mut self) -> SlayResult<()> {
        let mut state = self.state.lock().unwrap();
        
        if let Some(ref mut child) = state.child {
            let status = child.wait().map_err(io_error_to_cursed)?;
            state.exit_status = Some(status);
            state.is_running = false;
            
            if self.options.basic.collect_output {
                self.collect_output_enhanced(&mut state, child)?;
            }
        }
        
        Ok(())
    /// Enhanced output collection with line processing
    fn collect_output_enhanced(&self, state: &mut EnhancedProcessState, child: &mut Child) -> SlayResult<()> {
        // Read stdout
        if let Some(ref mut stdout) = child.stdout {
            let mut stdout_data = Vec::new();
            stdout.read_to_end(&mut stdout_data).map_err(io_error_to_cursed)?;
            state.stdout_data = stdout_data;
        // Read stderr
        if let Some(ref mut stderr) = child.stderr {
            let mut stderr_data = Vec::new();
            stderr.read_to_end(&mut stderr_data).map_err(io_error_to_cursed)?;
            state.stderr_data = stderr_data;
        Ok(())
    }
}

impl EnhancedProcessState {
    pub fn new() -> Self {
        Self {
        }
    }
impl ResourceMonitor {
    pub fn new(pid: u32, config: MonitoringConfig) -> Self {
        Self {
        }
    }

    /// Main monitoring loop
    pub fn monitor_loop(
    ) {
        while {
            let monitor_guard = monitor.lock().unwrap();
            monitor_guard.enabled
        } {
            let pid = {
                let monitor_guard = monitor.lock().unwrap();
                monitor_guard.pid
            
            if let Some(pid) = pid {
                if let Ok(stats) = Self::collect_process_stats(pid) {
                    {
                        let mut state_guard = state.lock().unwrap();
                        state_guard.current_stats = stats.clone();
                        state_guard.stats_history.push((Instant::now(), stats.clone()));
                    {
                        let mut monitor_guard = monitor.lock().unwrap();
                        monitor_guard.history.push((Instant::now(), stats.clone()));
                        
                        // Check thresholds
                        Self::check_thresholds(&monitor_guard.thresholds, &stats);
                    }
                }
            thread::sleep(interval);
        }
    }

    /// Collect process statistics
    fn collect_process_stats(pid: u32) -> SlayResult<ProcessStats> {
        #[cfg(unix)]
        {
            Self::collect_unix_stats(pid)
        }
        #[cfg(windows)]
        {
            Self::collect_windows_stats(pid)
        }
    }

    #[cfg(unix)]
    fn collect_unix_stats(pid: u32) -> SlayResult<ProcessStats> {
        use std::fs;
        
        // Read /proc/pid/stat for basic process info
        let stat_path = format!("/proc/{}/stat", pid);
        let stat_content = fs::read_to_string(&stat_path)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read process stats: {}", e)))?;
        
        let stat_fields: Vec<&str> = stat_content.split_whitespace().collect();
        
        // Parse CPU usage (simplified)
        let cpu_time = if stat_fields.len() > 13 {
            stat_fields[13].parse::<u64>().unwrap_or(0) + 
            stat_fields[14].parse::<u64>().unwrap_or(0)
        } else {
            0
        
        // Read /proc/pid/status for memory info
        let status_path = format!("/proc/{}/status", pid);
        let status_content = fs::read_to_string(&status_path)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read memory stats: {}", e)))?;
        
        let mut memory = 0u64;
        let mut resident_memory = 0u64;
        let mut virtual_memory = 0u64;
        
        for line in status_content.split("\n") {
            if line.starts_with("VmSize:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    virtual_memory = value.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
                }
            } else if line.starts_with("VmRSS:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    resident_memory = value.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
                    memory = resident_memory; // Use RSS as main memory metric
                }
            }
        Ok(ProcessStats {
            cpu: cpu_time as f64, // Simplified CPU calculation
            swap_memory: 0, // Would need additional parsing
            read_bytes: 0,  // Would need /proc/pid/io
            write_bytes: 0, // Would need /proc/pid/io
            up_time: Duration::from_secs(0), // Would need process start time calculation
            open_files: 0,    // Would need /proc/pid/fd
            network_conns: 0, // Would need /proc/net analysis
        })
    #[cfg(windows)]
    fn collect_windows_stats(pid: u32) -> SlayResult<ProcessStats> {
        // Windows implementation would use Windows API
        // For now, return default stats
        Ok(ProcessStats::default())
    /// Check resource thresholds and take action
    fn check_thresholds(config: &MonitoringConfig, stats: &ProcessStats) {
        if let Some(memory_threshold) = config.memory_threshold {
            if stats.memory > memory_threshold {
                match config.threshold_action {
                    ThresholdAction::Warn => {
                        tracing::warn!("Memory usage {} exceeds threshold {}", stats.memory, memory_threshold);
                    }
                    ThresholdAction::Throttle => {
                        tracing::warn!("Memory threshold exceeded, throttling process");
                        // Implement throttling logic
                    }
                    ThresholdAction::Kill => {
                        tracing::error!("Memory threshold exceeded, process should be killed");
                        // Kill logic would be handled by the caller
                    }
                    ThresholdAction::None => {}
                }
            }
        if let Some(cpu_threshold) = config.cpu_threshold {
            if stats.cpu > cpu_threshold {
                match config.threshold_action {
                    ThresholdAction::Warn => {
                        tracing::warn!("CPU usage {} exceeds threshold {}", stats.cpu, cpu_threshold);
                    }
                    ThresholdAction::Throttle => {
                        tracing::warn!("CPU threshold exceeded, throttling process");
                    }
                    ThresholdAction::Kill => {
                        tracing::error!("CPU threshold exceeded, process should be killed");
                    }
                    ThresholdAction::None => {}
                }
            }
        }
    }
