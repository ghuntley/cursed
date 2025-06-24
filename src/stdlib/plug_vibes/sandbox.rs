use crate::error::Error;
/// Plugin sandboxing for security and resource management
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashSet;
use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
use crate::stdlib::plug_vibes::plug::{Plug, load_with_options, LoadOptions};
use crate::stdlib::value::Value;

/// Options for configuring plugin sandbox
#[derive(Debug, Clone)]
pub struct SandboxOptions {
    /// Maximum memory usage in bytes (0 = unlimited)
    pub memory_limit: u64,
    /// CPU usage limit as fraction (0.0-1.0, 0.0 = unlimited)
    pub cpu_limit: f64,
    /// Maximum execution time per operation
    pub time_limit: Duration,
    /// Allow file system access
    pub file_access: bool,
    /// Allow network access
    pub network_access: bool,
    /// Allowed file paths (whitelist)
    pub allowed_paths: Vec<String>,
    /// Allowed network hosts (whitelist)
    pub allowed_hosts: Vec<String>,
    /// Maximum number of threads the plugin can create
    pub max_threads: u32,
    /// Maximum number of file descriptors
    pub max_file_descriptors: u32,
    /// Enable syscall filtering
    pub syscall_filtering: bool,
    /// Custom syscall whitelist
    pub allowed_syscalls: Vec<String>,
}

impl Default for SandboxOptions {
    fn default() -> Self {
        Self {
            memory_limit: 64 * 1024 * 1024, // 64MB default limit
            cpu_limit: 0.5, // 50% CPU limit
            time_limit: Duration::from_secs(30),
            file_access: false,
            network_access: false,
            allowed_paths: Vec::new(),
            allowed_hosts: Vec::new(),
            max_threads: 4,
            max_file_descriptors: 100,
            syscall_filtering: true,
            allowed_syscalls: vec![
                "read".to_string(),
                "write".to_string(),
                "malloc".to_string(),
                "free".to_string(),
            ],
        }
    }
}

impl SandboxOptions {
    /// Create new sandbox options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set memory limit
    pub fn with_memory_limit(mut self, limit: u64) -> Self {
        self.memory_limit = limit;
        self
    }

    /// Set CPU limit
    pub fn with_cpu_limit(mut self, limit: f64) -> Self {
        self.cpu_limit = limit.clamp(0.0, 1.0);
        self
    }

    /// Set time limit
    pub fn with_time_limit(mut self, limit: Duration) -> Self {
        self.time_limit = limit;
        self
    }

    /// Enable file access with optional path whitelist
    pub fn with_file_access(mut self, allowed_paths: Vec<String>) -> Self {
        self.file_access = true;
        self.allowed_paths = allowed_paths;
        self
    }

    /// Enable network access with optional host whitelist
    pub fn with_network_access(mut self, allowed_hosts: Vec<String>) -> Self {
        self.network_access = true;
        self.allowed_hosts = allowed_hosts;
        self
    }

    /// Set thread limit
    pub fn with_max_threads(mut self, max: u32) -> Self {
        self.max_threads = max;
        self
    }

    /// Set file descriptor limit
    pub fn with_max_file_descriptors(mut self, max: u32) -> Self {
        self.max_file_descriptors = max;
        self
    }

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
    pub memory_used: u64,
    pub cpu_time_used: Duration,
    pub files_opened: u32,
    pub network_connections: u32,
    pub threads_created: u32,
    pub syscalls_made: u64,
}

/// Sandbox enforcement mechanisms
#[derive(Debug, Clone)]
pub enum SandboxViolation {
    MemoryLimit,
    CpuLimit,
    TimeLimit,
    FileAccess(String),
    NetworkAccess(String),
    ThreadLimit,
    FileDescriptorLimit,
    SyscallBlocked(String),
}

impl std::fmt::Display for SandboxViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SandboxViolation::MemoryLimit => write!(f, "Memory limit exceeded"),
            SandboxViolation::CpuLimit => write!(f, "CPU limit exceeded"),
            SandboxViolation::TimeLimit => write!(f, "Time limit exceeded"),
            SandboxViolation::FileAccess(path) => write!(f, "Unauthorized file access: {}", path),
            SandboxViolation::NetworkAccess(host) => write!(f, "Unauthorized network access: {}", host),
            SandboxViolation::ThreadLimit => write!(f, "Thread limit exceeded"),
            SandboxViolation::FileDescriptorLimit => write!(f, "File descriptor limit exceeded"),
            SandboxViolation::SyscallBlocked(call) => write!(f, "Blocked syscall: {}", call),
        }
    }
}

/// Plugin sandbox implementation
pub struct Sandbox {
    options: SandboxOptions,
    resource_usage: Arc<Mutex<ResourceUsage>>,
    active_plugins: Arc<Mutex<HashSet<String>>>,
    start_time: Instant,
    violation_handlers: Vec<Box<dyn Fn(SandboxViolation) + Send + Sync>>,
}

impl Sandbox {
    /// Create a new sandbox with the given options
    pub fn new(options: SandboxOptions) -> Self {
        Self {
            options,
            resource_usage: Arc::new(Mutex::new(ResourceUsage::default())),
            active_plugins: Arc::new(Mutex::new(HashSet::new())),
            start_time: Instant::now(),
            violation_handlers: Vec::new(),
        }
    }

    /// Add a violation handler
    pub fn add_violation_handler<F>(&mut self, handler: F) 
    where
        F: Fn(SandboxViolation) + Send + Sync + 'static,
    {
        self.violation_handlers.push(Box::new(handler));
    }

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
        }

        Ok(plugin)
    }

    /// Execute a function in the sandbox with monitoring
    pub fn execute_func(
        &self,
        plugin: &Plug,
        func_name: &str,
        args: &[Value]
    ) -> PluginResult<Vec<Value>> {
        let start_time = Instant::now();

        // Pre-execution checks
        self.validate_resource_limits()?;
        self.check_execution_time(start_time)?;

        // Setup monitoring for this execution
        let resource_monitor = ResourceMonitor::new(
            Arc::clone(&self.resource_usage),
            self.options.clone(),
            start_time,
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
                }

                // Return empty result for now
                Ok(Vec::new())
            }
            Err(e) => Err(e),
        }
    }

    /// Check if file access is allowed
    pub fn check_file_access(&self, path: &str) -> PluginResult<()> {
        if !self.options.file_access {
            self.handle_violation(SandboxViolation::FileAccess(path.to_string()));
            return Err(PluginError::sandbox_violation(&format!("File access denied: {}", path)));
        }

        if !self.options.allowed_paths.is_empty() {
            let allowed = self.options.allowed_paths.iter()
                .any(|allowed_path| path.starts_with(allowed_path));
            
            if !allowed {
                self.handle_violation(SandboxViolation::FileAccess(path.to_string()));
                return Err(PluginError::sandbox_violation(&format!("File path not in whitelist: {}", path)));
            }
        }

        Ok(())
    }

    /// Check if network access is allowed
    pub fn check_network_access(&self, host: &str) -> PluginResult<()> {
        if !self.options.network_access {
            self.handle_violation(SandboxViolation::NetworkAccess(host.to_string()));
            return Err(PluginError::sandbox_violation(&format!("Network access denied: {}", host)));
        }

        if !self.options.allowed_hosts.is_empty() {
            let allowed = self.options.allowed_hosts.iter()
                .any(|allowed_host| host == allowed_host || host.ends_with(&format!(".{}", allowed_host)));
            
            if !allowed {
                self.handle_violation(SandboxViolation::NetworkAccess(host.to_string()));
                return Err(PluginError::sandbox_violation(&format!("Host not in whitelist: {}", host)));
            }
        }

        Ok(())
    }

    /// Get current resource usage
    pub fn get_resource_usage(&self) -> ResourceUsage {
        self.resource_usage.lock()
            .map(|usage| usage.clone())
            .unwrap_or_default()
    }

    /// Check if a syscall is allowed
    pub fn check_syscall(&self, syscall: &str) -> PluginResult<()> {
        if !self.options.syscall_filtering {
            return Ok(());
        }

        if !self.options.allowed_syscalls.contains(&syscall.to_string()) {
            self.handle_violation(SandboxViolation::SyscallBlocked(syscall.to_string()));
            return Err(PluginError::security_violation(&format!("Syscall blocked: {}", syscall)));
        }

        Ok(())
    }

    /// Release sandbox resources
    pub fn release(&self) -> PluginResult<()> {
        // Clear active plugins
        if let Ok(mut active) = self.active_plugins.lock() {
            active.clear();
        }

        // Reset resource usage
        if let Ok(mut usage) = self.resource_usage.lock() {
            *usage = ResourceUsage::default();
        }

        Ok(())
    }

    /// Validate current resource limits
    fn validate_resource_limits(&self) -> PluginResult<()> {
        if let Ok(usage) = self.resource_usage.lock() {
            // Check memory limit
            if self.options.memory_limit > 0 && usage.memory_used > self.options.memory_limit {
                self.handle_violation(SandboxViolation::MemoryLimit);
                return Err(PluginError::sandbox_violation("Memory limit exceeded"));
            }

            // Check thread limit
            if usage.threads_created > self.options.max_threads {
                self.handle_violation(SandboxViolation::ThreadLimit);
                return Err(PluginError::sandbox_violation("Thread limit exceeded"));
            }

            // Check file descriptor limit
            if usage.files_opened > self.options.max_file_descriptors {
                self.handle_violation(SandboxViolation::FileDescriptorLimit);
                return Err(PluginError::sandbox_violation("File descriptor limit exceeded"));
            }
        }

        Ok(())
    }

    /// Check execution time limit
    fn check_execution_time(&self, start_time: Instant) -> PluginResult<()> {
        if start_time.elapsed() > self.options.time_limit {
            self.handle_violation(SandboxViolation::TimeLimit);
            return Err(PluginError::timeout("Execution time limit exceeded"));
        }
        Ok(())
    }

    /// Handle sandbox violation
    fn handle_violation(&self, violation: SandboxViolation) {
        for handler in &self.violation_handlers {
            handler(violation.clone());
        }
    }
}

/// Resource monitoring for sandbox enforcement
struct ResourceMonitor {
    resource_usage: Arc<Mutex<ResourceUsage>>,
    options: SandboxOptions,
    start_time: Instant,
}

impl ResourceMonitor {
    fn new(
        resource_usage: Arc<Mutex<ResourceUsage>>,
        options: SandboxOptions,
        start_time: Instant,
    ) -> Self {
        Self {
            resource_usage,
            options,
            start_time,
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
                }

                thread::sleep(monitor_interval);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_options_default() {
        let options = SandboxOptions::default();
        
        assert_eq!(options.memory_limit, 64 * 1024 * 1024);
        assert_eq!(options.cpu_limit, 0.5);
        assert_eq!(options.time_limit, Duration::from_secs(30));
        assert!(!options.file_access);
        assert!(!options.network_access);
        assert_eq!(options.max_threads, 4);
        assert!(options.syscall_filtering);
    }

    #[test]
    fn test_sandbox_options_builder() {
        let options = SandboxOptions::new()
            .with_memory_limit(128 * 1024 * 1024)
            .with_cpu_limit(0.8)
            .with_time_limit(Duration::from_secs(60))
            .with_file_access(vec!["/tmp".to_string()])
            .with_network_access(vec!["localhost".to_string()])
            .with_max_threads(8);

        assert_eq!(options.memory_limit, 128 * 1024 * 1024);
        assert_eq!(options.cpu_limit, 0.8);
        assert_eq!(options.time_limit, Duration::from_secs(60));
        assert!(options.file_access);
        assert!(options.network_access);
        assert_eq!(options.allowed_paths, vec!["/tmp".to_string()]);
        assert_eq!(options.allowed_hosts, vec!["localhost".to_string()]);
        assert_eq!(options.max_threads, 8);
    }

    #[test]
    fn test_sandbox_creation() {
        let options = SandboxOptions::default();
        let sandbox = Sandbox::new(options);
        
        let usage = sandbox.get_resource_usage();
        assert_eq!(usage.memory_used, 0);
        assert_eq!(usage.threads_created, 0);
    }

    #[test]
    fn test_file_access_check() {
        let options = SandboxOptions::new()
            .with_file_access(vec!["/tmp".to_string(), "/var/log".to_string()]);
        let sandbox = Sandbox::new(options);
        
        // Allowed paths
        assert!(sandbox.check_file_access("/tmp/test.txt").is_ok());
        assert!(sandbox.check_file_access("/var/log/app.log").is_ok());
        
        // Disallowed path
        assert!(sandbox.check_file_access("/etc/passwd").is_err());
    }

    #[test]
    fn test_network_access_check() {
        let options = SandboxOptions::new()
            .with_network_access(vec!["example.com".to_string(), "localhost".to_string()]);
        let sandbox = Sandbox::new(options);
        
        // Allowed hosts
        assert!(sandbox.check_network_access("example.com").is_ok());
        assert!(sandbox.check_network_access("api.example.com").is_ok()); // subdomain allowed
        assert!(sandbox.check_network_access("localhost").is_ok());
        
        // Disallowed host
        assert!(sandbox.check_network_access("malicious.com").is_err());
    }

    #[test]
    fn test_syscall_check() {
        let options = SandboxOptions::new()
            .with_syscall_filtering(true, vec!["read".to_string(), "write".to_string()]);
        let sandbox = Sandbox::new(options);
        
        // Allowed syscalls
        assert!(sandbox.check_syscall("read").is_ok());
        assert!(sandbox.check_syscall("write").is_ok());
        
        // Blocked syscall
        assert!(sandbox.check_syscall("execve").is_err());
    }

    #[test]
    fn test_resource_usage_tracking() {
        let options = SandboxOptions::default();
        let sandbox = Sandbox::new(options);
        
        let usage = sandbox.get_resource_usage();
        assert_eq!(usage.memory_used, 0);
        assert_eq!(usage.cpu_time_used, Duration::from_secs(0));
    }

    #[test]
    fn test_sandbox_release() {
        let options = SandboxOptions::default();
        let sandbox = Sandbox::new(options);
        
        let result = sandbox.release();
        assert!(result.is_ok());
        
        let usage = sandbox.get_resource_usage();
        assert_eq!(usage.memory_used, 0);
    }
}
