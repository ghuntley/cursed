use crate::error::CursedError;
/// Plugin sandboxing for security and resource management
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashSet;
// use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
// use crate::stdlib::plug_vibes::plug::{Plug, load_with_options, LoadOptions};
// use crate::stdlib::value::Value;

/// Options for configuring plugin sandbox
#[derive(Debug, Clone)]
pub struct SandboxOptions {
    /// Maximum memory usage in bytes (0 = unlimited)
    /// CPU usage limit as fraction (0.0-1.0, 0.0 = unlimited)
    /// Maximum execution time per operation
    /// Allow file system access
    /// Allow network access
    /// Allowed file paths (whitelist)
    /// Allowed network hosts (whitelist)
    /// Maximum number of threads the plugin can create
    /// Maximum number of file descriptors
    /// Enable syscall filtering
    /// Custom syscall whitelist
impl Default for SandboxOptions {
    fn default() -> Self {
        Self {
            memory_limit: 64 * 1024 * 1024, // 64MB default limit
            cpu_limit: 0.5, // 50% CPU limit
            allowed_syscalls: vec![
        }
    }
impl SandboxOptions {
    /// Create new sandbox options with defaults
    pub fn new() -> Self {
        Self::default()
    /// Set memory limit
    pub fn with_memory_limit(mut self, limit: u64) -> Self {
        self.memory_limit = limit;
        self
    /// Set CPU limit
    pub fn with_cpu_limit(mut self, limit: f64) -> Self {
        self.cpu_limit = limit.clamp(0.0, 1.0);
        self
    /// Set time limit
    pub fn with_time_limit(mut self, limit: Duration) -> Self {
        self.time_limit = limit;
        self
    /// Enable file access with optional path whitelist
    pub fn with_file_access(mut self, allowed_paths: Vec<String>) -> Self {
        self.file_access = true;
        self.allowed_paths = allowed_paths;
        self
    /// Enable network access with optional host whitelist
    pub fn with_network_access(mut self, allowed_hosts: Vec<String>) -> Self {
        self.network_access = true;
        self.allowed_hosts = allowed_hosts;
        self
    /// Set thread limit
    pub fn with_max_threads(mut self, max: u32) -> Self {
        self.max_threads = max;
        self
    /// Set file descriptor limit
    pub fn with_max_file_descriptors(mut self, max: u32) -> Self {
        self.max_file_descriptors = max;
        self
    /// Configure syscall filtering
    pub fn with_syscall_filtering(mut self, enabled: bool, allowed: Vec<String>) -> Self {
        self.syscall_filtering = enabled;
        self.allowed_syscalls = allowed;
        self
    }
}

/// Resource usage tracking
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
/// Sandbox enforcement mechanisms
#[derive(Debug, Clone)]
pub enum SandboxViolation {
impl std::fmt::Display for SandboxViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// Plugin sandbox implementation
pub struct Sandbox {
impl Sandbox {
    /// Create a new sandbox with the given options
    pub fn new(options: SandboxOptions) -> Self {
        Self {
        }
    }

    /// Add a violation handler
    pub fn add_violation_handler<F>(&mut self, handler: F) 
    where
    {
        self.violation_handlers.push(Box::new(handler));
    /// Load a plugin in the sandbox
    pub fn load_plugin(&self, path: &str) -> PluginResult<Plug> {
        // Validate sandbox constraints before loading
        self.validate_resource_limits()?;

        // Create restricted load options
        let mut load_options = LoadOptions::default();
        load_options.isolation = true;
        load_options.sandbox = true;
        load_options.timeout = self.options.time_limit;

        // Load the plugin with restrictions
        let plugin = load_with_options(path, load_options)?;

        // Track the loaded plugin
        if let Ok(mut active) = self.active_plugins.lock() {
            let plugin_name = plugin.info().name.clone();
            active.insert(plugin_name);
        Ok(plugin)
    /// Execute a function in the sandbox with monitoring
    pub fn execute_func(
        args: &[Value]
    ) -> PluginResult<Vec<Value>> {
        let start_time = Instant::now();

        // Pre-execution checks
        self.validate_resource_limits()?;
        self.check_execution_time(start_time)?;

        // Setup monitoring for this execution
        let resource_monitor = ResourceMonitor::new(
        );

        // Start monitoring in background
        let _monitor_handle = resource_monitor.start_monitoring();

        // Execute the function
        match plugin.lookup_func(func_name) {
            Ok(_func) => {
                // In a real implementation, we'd call the function here
                // and monitor its execution
                
                // Simulate execution time check
                if start_time.elapsed() > self.options.time_limit {
                    self.handle_violation(SandboxViolation::TimeLimit);
                    return Err(PluginError::timeout("Function execution timeout"));
                // Return empty result for now
                Ok(Vec::new())
            }
        }
    }

    /// Check if file access is allowed
    pub fn check_file_access(&self, path: &str) -> PluginResult<()> {
        if !self.options.file_access {
            self.handle_violation(SandboxViolation::FileAccess(path.to_string()));
            return Err(PluginError::sandbox_violation(&format!("File access denied: {}", path)));
        if !self.options.allowed_paths.is_empty() {
            let allowed = self.options.allowed_paths.iter()
                .any(|allowed_path| path.starts_with(allowed_path));
            
            if !allowed {
                self.handle_violation(SandboxViolation::FileAccess(path.to_string()));
                return Err(PluginError::sandbox_violation(&format!("File path not in whitelist: {}", path)));
            }
        }

        Ok(())
    /// Check if network access is allowed
    pub fn check_network_access(&self, host: &str) -> PluginResult<()> {
        if !self.options.network_access {
            self.handle_violation(SandboxViolation::NetworkAccess(host.to_string()));
            return Err(PluginError::sandbox_violation(&format!("Network access denied: {}", host)));
        if !self.options.allowed_hosts.is_empty() {
            let allowed = self.options.allowed_hosts.iter()
                .any(|allowed_host| host == allowed_host || host.ends_with(&format!(".{}", allowed_host)));
            
            if !allowed {
                self.handle_violation(SandboxViolation::NetworkAccess(host.to_string()));
                return Err(PluginError::sandbox_violation(&format!("Host not in whitelist: {}", host)));
            }
        }

        Ok(())
    /// Get current resource usage
    pub fn get_resource_usage(&self) -> ResourceUsage {
        self.resource_usage.lock()
            .map(|usage| usage.clone())
            .unwrap_or_default()
    /// Check if a syscall is allowed
    pub fn check_syscall(&self, syscall: &str) -> PluginResult<()> {
        if !self.options.syscall_filtering {
            return Ok(());
        if !self.options.allowed_syscalls.contains(&syscall.to_string()) {
            self.handle_violation(SandboxViolation::SyscallBlocked(syscall.to_string()));
            return Err(PluginError::security_violation(&format!("Syscall blocked: {}", syscall)));
        Ok(())
    /// Release sandbox resources
    pub fn release(&self) -> PluginResult<()> {
        // Clear active plugins
        if let Ok(mut active) = self.active_plugins.lock() {
            active.clear();
        // Reset resource usage
        if let Ok(mut usage) = self.resource_usage.lock() {
            *usage = ResourceUsage::default();
        Ok(())
    /// Validate current resource limits
    fn validate_resource_limits(&self) -> PluginResult<()> {
        if let Ok(usage) = self.resource_usage.lock() {
            // Check memory limit
            if self.options.memory_limit > 0 && usage.memory_used > self.options.memory_limit {
                self.handle_violation(SandboxViolation::MemoryLimit);
                return Err(PluginError::sandbox_violation("Memory limit exceeded"));
            // Check thread limit
            if usage.threads_created > self.options.max_threads {
                self.handle_violation(SandboxViolation::ThreadLimit);
                return Err(PluginError::sandbox_violation("Thread limit exceeded"));
            // Check file descriptor limit
            if usage.files_opened > self.options.max_file_descriptors {
                self.handle_violation(SandboxViolation::FileDescriptorLimit);
                return Err(PluginError::sandbox_violation("File descriptor limit exceeded"));
            }
        }

        Ok(())
    /// Check execution time limit
    fn check_execution_time(&self, start_time: Instant) -> PluginResult<()> {
        if start_time.elapsed() > self.options.time_limit {
            self.handle_violation(SandboxViolation::TimeLimit);
            return Err(PluginError::timeout("Execution time limit exceeded"));
        }
        Ok(())
    /// Handle sandbox violation
    fn handle_violation(&self, violation: SandboxViolation) {
        for handler in &self.violation_handlers {
            handler(violation.clone());
        }
    }
/// Resource monitoring for sandbox enforcement
struct ResourceMonitor {
impl ResourceMonitor {
    fn new(
    ) -> Self {
        Self {
        }
    }

    fn start_monitoring(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let monitor_interval = Duration::from_millis(100);
            
            while self.start_time.elapsed() < self.options.time_limit {
                // Monitor resource usage
                if let Ok(mut usage) = self.resource_usage.lock() {
                    // Update CPU time (simplified)
                    usage.cpu_time_used = self.start_time.elapsed();
                    
                    // In a real implementation, we'd gather actual system metrics here
                    // using tools like procfs on Linux, or system APIs on other platforms
                thread::sleep(monitor_interval);
            }
        })
    }
}

