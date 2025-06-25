use crate::error::CursedError;
/// Enhanced exec_vibez types that are missing from the main module
/// 
/// This module provides the missing types that are being imported but not defined.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::path::PathBuf;


/// Result type for exec_vibez operations
pub type VibezResult<T> = std::result::Result<T, ProcessError>;

/// Execution context for enhanced command execution
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Working directory
    /// Environment variables
    /// Timeout duration
    /// Resource limits
    /// Security context
    /// Execution mode
    /// Priority level
/// Enhanced command structure for process execution
#[derive(Debug)]
pub struct EnhancedCmd {
    /// Command program
    /// Command arguments
    /// Environment variables
    /// Working directory
    /// Execution context
    /// Resource limits
    /// Security settings
    /// Process priority
    /// Timeout configuration
/// Resource limits for process execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    /// Maximum CPU usage percentage
    /// Maximum execution time
    /// Maximum file descriptors
    /// Maximum processes
/// Security context for process execution
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// User ID to run as
    /// Group ID to run as
    /// Allowed capabilities
    /// Chroot directory
    /// Network access allowed
    /// File system access allowed
/// Execution mode for commands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Execute synchronously
    /// Execute asynchronously
    /// Execute in background
    /// Execute with detached process
/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    /// Low priority
    /// Normal priority
    /// High priority
    /// Critical priority
impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
        }
    }
impl Default for SecurityContext {
    fn default() -> Self {
        Self {
        }
    }
impl EnhancedCmd {
    /// Create a new enhanced command
    pub fn new<S: AsRef<str>>(program: S) -> Self {
        Self {
        }
    }

    /// Add an argument to the command
    pub fn arg<S: AsRef<str>>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    /// Add multiple arguments to the command
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
    {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    /// Set environment variable
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
    {
        self.env.insert(key.as_ref().to_string(), val.as_ref().to_string());
        self
    /// Set working directory
    pub fn current_dir<P: Into<PathBuf>>(&mut self, dir: P) -> &mut Self {
        self.dir = Some(dir.into());
        self
    /// Set execution context
    pub fn context(&mut self, context: ExecutionContext) -> &mut Self {
        self.context = Some(Arc::new(context));
        self
    /// Set resource limits
    pub fn resource_limits(&mut self, limits: ResourceLimits) -> &mut Self {
        self.resource_limits = Some(limits);
        self
    /// Set security context
    pub fn security(&mut self, security: SecurityContext) -> &mut Self {
        self.security = Some(security);
        self
    /// Set process priority
    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = priority;
        self
    /// Set timeout
    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }
}
