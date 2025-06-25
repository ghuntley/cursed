use crate::error::CursedError;
/// Core types and structures for the CURSED IPC system
/// 
/// This module defines fundamental types used throughout the IPC subsystem including
/// handles, addresses, permissions, timeouts, and statistics structures.

use std::fmt;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::sync::Arc;
// use crate::stdlib::ipc::error::{IpcError, IpcResult};

/// Generic handle for IPC resources
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IpcHandle {
impl IpcHandle {
    pub fn new(id: String, resource_type: String) -> Self {
        Self {
        }
    }
    
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    pub fn age(&self) -> Duration {
        SystemTime::now().duration_since(self.created_at).unwrap_or_default()
    }
}

impl fmt::Display for IpcHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.resource_type, self.id)
    }
}

/// Generic address for IPC endpoints
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IpcAddress {
    /// Named path-based address (pipes, sockets)
    /// Network-style address (host:port)
    /// Process-based address (PID)
    /// Memory-based address (offset, size)
    /// Abstract namespace address
    /// Custom address with type identifier
impl IpcAddress {
    pub fn path<S: Into<String>>(path: S) -> Self {
        IpcAddress::Path(path.into())
    pub fn network<S: Into<String>>(host: S, port: u16) -> Self {
        IpcAddress::Network(host.into(), port)
    pub fn process(pid: u32) -> Self {
        IpcAddress::Process(pid)
    pub fn memory(offset: usize, size: usize) -> Self {
        IpcAddress::Memory(offset, size)
    pub fn abstract_ns<S: Into<String>>(name: S) -> Self {
        IpcAddress::Abstract(name.into())
    pub fn custom<S: Into<String>>(type_name: S, data: Vec<u8>) -> Self {
        IpcAddress::Custom(type_name.into(), data)
    pub fn address_type(&self) -> &'static str {
        match self {
        }
    }
impl fmt::Display for IpcAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Permission structure for IPC resources
#[derive(Debug, Clone, PartialEq)]
pub struct IpcPermissions {
impl IpcPermissions {
    pub fn new(mode: u32) -> Self {
        Self {
        }
    }
    
    pub fn with_owner(mut self, uid: u32, gid: u32) -> Self {
        self.owner_uid = Some(uid);
        self.owner_gid = Some(gid);
        self
    pub fn with_acl(mut self, acl: AccessControlList) -> Self {
        self.access_control = acl;
        self
    pub fn owner_read(&self) -> bool {
        (self.mode & 0o400) != 0
    pub fn owner_write(&self) -> bool {
        (self.mode & 0o200) != 0
    pub fn owner_execute(&self) -> bool {
        (self.mode & 0o100) != 0
    pub fn group_read(&self) -> bool {
        (self.mode & 0o040) != 0
    pub fn group_write(&self) -> bool {
        (self.mode & 0o020) != 0
    pub fn group_execute(&self) -> bool {
        (self.mode & 0o010) != 0
    pub fn other_read(&self) -> bool {
        (self.mode & 0o004) != 0
    pub fn other_write(&self) -> bool {
        (self.mode & 0o002) != 0
    pub fn other_execute(&self) -> bool {
        (self.mode & 0o001) != 0
    }
}

impl Default for IpcPermissions {
    fn default() -> Self {
        Self::new(0o600) // Owner read/write only
    }
}

/// Access Control List for fine-grained permissions
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccessControlList {
impl AccessControlList {
    pub fn new() -> Self {
        Self::default()
    pub fn add_user_permission(mut self, uid: u32, permissions: AclPermissions) -> Self {
        self.entries.push(AclEntry::User(uid, permissions));
        self
    pub fn add_group_permission(mut self, gid: u32, permissions: AclPermissions) -> Self {
        self.entries.push(AclEntry::Group(gid, permissions));
        self
    pub fn check_access(&self, uid: u32, gid: u32, requested: AclPermissions) -> bool {
        for entry in &self.entries {
            match entry {
                AclEntry::User(entry_uid, perms) if *entry_uid == uid => {
                    return perms.contains(requested);
                }
                AclEntry::Group(entry_gid, perms) if *entry_gid == gid => {
                    return perms.contains(requested);
                }
            }
        }
        false
    }
}

/// Access Control List entry
#[derive(Debug, Clone, PartialEq)]
pub enum AclEntry {
/// ACL permissions bit flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AclPermissions {
impl AclPermissions {
    pub const READ: Self = Self { bits: 0b001 };
    pub const WRITE: Self = Self { bits: 0b010 };
    pub const EXECUTE: Self = Self { bits: 0b100 };
    
    pub const fn new() -> Self {
        Self { bits: 0 }
    }
    
    pub const fn with_read(mut self) -> Self {
        self.bits |= Self::READ.bits;
        self
    pub const fn with_write(mut self) -> Self {
        self.bits |= Self::WRITE.bits;
        self
    pub const fn with_execute(mut self) -> Self {
        self.bits |= Self::EXECUTE.bits;
        self
    pub fn contains(self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    pub fn read(self) -> bool {
        (self.bits & Self::READ.bits) != 0
    pub fn write(self) -> bool {
        (self.bits & Self::WRITE.bits) != 0
    pub fn execute(self) -> bool {
        (self.bits & Self::EXECUTE.bits) != 0
    }
}

impl Default for AclPermissions {
    fn default() -> Self {
        Self::new()
    }
}

/// Timeout configuration for IPC operations
#[derive(Debug, Clone, PartialEq)]
pub enum IpcTimeout {
    /// No timeout (blocking)
    /// Immediate return (non-blocking)
    /// Timeout after specified duration
    /// Timeout at absolute time
impl IpcTimeout {
    pub fn none() -> Self {
        IpcTimeout::None
    pub fn immediate() -> Self {
        IpcTimeout::Immediate
    pub fn after(duration: Duration) -> Self {
        IpcTimeout::Duration(duration)
    pub fn at(time: SystemTime) -> Self {
        IpcTimeout::Absolute(time)
    pub fn is_blocking(&self) -> bool {
        matches!(self, IpcTimeout::None)
    pub fn is_immediate(&self) -> bool {
        matches!(self, IpcTimeout::Immediate)
    pub fn remaining_time(&self) -> Option<Duration> {
        match self {
            IpcTimeout::Absolute(time) => {
                SystemTime::now().duration_since(*time).ok()
            }
        }
    }
}

impl Default for IpcTimeout {
    fn default() -> Self {
        IpcTimeout::None
    }
}

impl From<Duration> for IpcTimeout {
    fn from(duration: Duration) -> Self {
        IpcTimeout::Duration(duration)
    }
}

impl From<SystemTime> for IpcTimeout {
    fn from(time: SystemTime) -> Self {
        IpcTimeout::Absolute(time)
    }
}

/// Generic statistics structure for IPC resources
#[derive(Debug, Clone, Default)]
pub struct IpcStatistics {
    // Operation counts
    
    // Data transfer
    
    // Timing
    
    // Resource usage
    
    // CursedError breakdown
    
    // Resource-specific metrics
    
    // Timestamps
impl IpcStatistics {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    
    pub fn record_operation(&mut self, success: bool, duration: Duration) {
        self.total_operations += 1;
        self.last_operation = Some(SystemTime::now());
        
        if success {
            self.successful_operations += 1;
        } else {
            self.failed_operations += 1;
            self.last_error = Some(SystemTime::now());
        // Update timing statistics
        self.total_time += duration;
        if duration < self.min_operation_time {
            self.min_operation_time = duration;
        }
        if duration > self.max_operation_time {
            self.max_operation_time = duration;
        // Calculate running average
        if self.total_operations > 0 {
            self.average_operation_time = self.total_time / self.total_operations as u32;
        }
    }
    
    pub fn record_data_transfer(&mut self, sent: u64, received: u64) {
        self.bytes_sent += sent;
        self.bytes_received += received;
    pub fn record_message(&mut self, sent: bool) {
        if sent {
            self.messages_sent += 1;
        } else {
            self.messages_received += 1;
        }
    }
    
    pub fn record_error(&mut self, error_type: &str) {
        self.last_error = Some(SystemTime::now());
        match error_type {
            _ => {
                let count = self.custom_metrics.entry(format!("{}_errors", error_type)).or_insert(0);
                *count += 1;
            }
        }
    pub fn set_custom_metric(&mut self, name: &str, value: u64) {
        self.custom_metrics.insert(name.to_string(), value);
    pub fn increment_custom_metric(&mut self, name: &str, delta: u64) {
        let count = self.custom_metrics.entry(name.to_string()).or_insert(0);
        *count += delta;
    pub fn success_rate(&self) -> f64 {
        if self.total_operations == 0 {
            0.0
        } else {
            self.successful_operations as f64 / self.total_operations as f64
        }
    }
    
    pub fn error_rate(&self) -> f64 {
        if self.total_operations == 0 {
            0.0
        } else {
            self.failed_operations as f64 / self.total_operations as f64
        }
    }
    
    pub fn uptime(&self) -> Duration {
        SystemTime::now().duration_since(self.created_at).unwrap_or_default()
    }
}

/// Capabilities supported by the IPC subsystem
#[derive(Debug, Clone, PartialEq)]
pub struct IpcCapabilities {
impl IpcCapabilities {
    pub fn supported_mechanisms(&self) -> Vec<&'static str> {
        let mut mechanisms = Vec::new();
        
        if self.message_queues { mechanisms.push("message_queues"); }
        if self.named_pipes { mechanisms.push("named_pipes"); }
        if self.anonymous_pipes { mechanisms.push("anonymous_pipes"); }
        if self.shared_memory { mechanisms.push("shared_memory"); }
        if self.semaphores { mechanisms.push("semaphores"); }
        if self.signals { mechanisms.push("signals"); }
        if self.unix_sockets { mechanisms.push("unix_sockets"); }
        if self.file_locking { mechanisms.push("file_locking"); }
        if self.rpc { mechanisms.push("rpc"); }
        if self.security { mechanisms.push("security"); }
        mechanisms
    pub fn count_supported(&self) -> usize {
        self.supported_mechanisms().len()
    }
}

impl Default for IpcCapabilities {
    fn default() -> Self {
        Self {
        }
    }
/// Resource limit configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceLimits {
impl ResourceLimits {
    pub fn unlimited() -> Self {
        Self {
        }
    }
    
    pub fn conservative() -> Self {
        Self {
            max_memory: Some(1024 * 1024 * 100), // 100MB
            max_message_size: Some(1024 * 1024),  // 1MB
        }
    }
    
    pub fn with_max_handles(mut self, limit: usize) -> Self {
        self.max_handles = Some(limit);
        self
    pub fn with_max_memory(mut self, limit: usize) -> Self {
        self.max_memory = Some(limit);
        self
    pub fn with_max_message_size(mut self, limit: usize) -> Self {
        self.max_message_size = Some(limit);
        self
    pub fn with_max_queue_depth(mut self, limit: usize) -> Self {
        self.max_queue_depth = Some(limit);
        self
    pub fn with_max_connections(mut self, limit: usize) -> Self {
        self.max_connections = Some(limit);
        self
    pub fn check_handles(&self, current: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_handles {
            if current >= limit {
                return Err(IpcError::ResourceExhausted(
                    format!("Handle limit exceeded: {} >= {}", current, limit)
                ));
            }
        }
        Ok(())
    pub fn check_memory(&self, current: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_memory {
            if current >= limit {
                return Err(IpcError::ResourceExhausted(
                    format!("Memory limit exceeded: {} >= {}", current, limit)
                ));
            }
        }
        Ok(())
    pub fn check_message_size(&self, size: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_message_size {
            if size >= limit {
                return Err(IpcError::InvalidInput(
                    format!("Message size too large: {} >= {}", size, limit)
                ));
            }
        }
        Ok(())
    pub fn check_queue_depth(&self, depth: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_queue_depth {
            if depth >= limit {
                return Err(IpcError::ResourceExhausted(
                    format!("Queue depth limit exceeded: {} >= {}", depth, limit)
                ));
            }
        }
        Ok(())
    pub fn check_connections(&self, current: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_connections {
            if current >= limit {
                return Err(IpcError::ResourceExhausted(
                    format!("Connection limit exceeded: {} >= {}", current, limit)
                ));
            }
        }
        Ok(())
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::conservative()
    }
}

