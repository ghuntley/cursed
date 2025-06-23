/// Enhanced exec_vibez types that are missing from the main module
/// 
/// This module provides the missing types that are being imported but not defined.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::path::PathBuf;

use crate::error::CursedError;

/// Result type for exec_vibez operations
pub type VibezResult<(), Error>;

/// Execution context for enhanced command execution
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Working directory
    pub working_dir: Option<PathBuf>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Timeout duration
    pub timeout: Option<Duration>,
    /// Resource limits
    pub resource_limits: Option<ResourceLimits>,
    /// Security context
    pub security_context: Option<SecurityContext>,
    /// Execution mode
    pub execution_mode: ExecutionMode,
    /// Priority level
    pub priority: Priority,
}

/// Enhanced command structure for process execution
#[derive(Debug)]
pub struct EnhancedCmd {
    /// Command program
    pub program: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Working directory
    pub dir: Option<PathBuf>,
    /// Execution context
    pub context: Option<Arc<ExecutionContext>>,
    /// Resource limits
    pub resource_limits: Option<ResourceLimits>,
    /// Security settings
    pub security: Option<SecurityContext>,
    /// Process priority
    pub priority: Priority,
    /// Timeout configuration
    pub timeout: Option<Duration>,
}

/// Resource limits for process execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    pub max_memory: Option<u64>,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: Option<f64>,
    /// Maximum execution time
    pub max_execution_time: Option<Duration>,
    /// Maximum file descriptors
    pub max_file_descriptors: Option<u32>,
    /// Maximum processes
    pub max_processes: Option<u32>,
}

/// Security context for process execution
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// User ID to run as
    pub user_id: Option<u32>,
    /// Group ID to run as
    pub group_id: Option<u32>,
    /// Allowed capabilities
    pub capabilities: Vec<String>,
    /// Chroot directory
    pub chroot_dir: Option<PathBuf>,
    /// Network access allowed
    pub network_access: bool,
    /// File system access allowed
    pub filesystem_access: bool,
}

/// Execution mode for commands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Execute synchronously
    Sync,
    /// Execute asynchronously
    Async,
    /// Execute in background
    Background,
    /// Execute with detached process
    Detached,
}

/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            working_dir: None,
            env_vars: HashMap::new(),
            timeout: None,
            resource_limits: None,
            security_context: None,
            execution_mode: ExecutionMode::Sync,
            priority: Priority::Normal,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: None,
            max_cpu_percent: None,
            max_execution_time: None,
            max_file_descriptors: None,
            max_processes: None,
        }
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user_id: None,
            group_id: None,
            capabilities: Vec::new(),
            chroot_dir: None,
            network_access: true,
            filesystem_access: true,
        }
    }
}

impl EnhancedCmd {
    /// Create a new enhanced command
    pub fn new<S: AsRef<str>>(program: S) -> Self {
        Self {
            program: program.as_ref().to_string(),
            args: Vec::new(),
            env: HashMap::new(),
            dir: None,
            context: None,
            resource_limits: None,
            security: None,
            priority: Priority::Normal,
            timeout: None,
        }
    }

    /// Add an argument to the command
    pub fn arg<S: AsRef<str>>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    /// Add multiple arguments to the command
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    }

    /// Set environment variable
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.env.insert(key.as_ref().to_string(), val.as_ref().to_string());
        self
    }

    /// Set working directory
    pub fn current_dir<P: Into<PathBuf>>(&mut self, dir: P) -> &mut Self {
        self.dir = Some(dir.into());
        self
    }

    /// Set execution context
    pub fn context(&mut self, context: ExecutionContext) -> &mut Self {
        self.context = Some(Arc::new(context));
        self
    }

    /// Set resource limits
    pub fn resource_limits(&mut self, limits: ResourceLimits) -> &mut Self {
        self.resource_limits = Some(limits);
        self
    }

    /// Set security context
    pub fn security(&mut self, security: SecurityContext) -> &mut Self {
        self.security = Some(security);
        self
    }

    /// Set process priority
    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Set timeout
    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }
}
