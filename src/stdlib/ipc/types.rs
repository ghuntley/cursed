/// Core types and structures for the CURSED IPC system
/// 
/// This module defines fundamental types used throughout the IPC subsystem including
/// handles, addresses, permissions, timeouts, and statistics structures.

use std::fmt;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::sync::Arc;
use crate::stdlib::ipc::error::{IpcError, IpcResult};

/// Generic handle for IPC resources
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IpcHandle {
    pub id: String,
    pub resource_type: String,
    pub created_at: SystemTime,
    pub metadata: HashMap<String, String>,
}

impl IpcHandle {
    pub fn new(id: String, resource_type: String) -> Self {
        Self {
            id,
            resource_type,
            created_at: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
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
    Path(String),
    /// Network-style address (host:port)
    Network(String, u16),
    /// Process-based address (PID)
    Process(u32),
    /// Memory-based address (offset, size)
    Memory(usize, usize),
    /// Abstract namespace address
    Abstract(String),
    /// Custom address with type identifier
    Custom(String, Vec<u8>),
}

impl IpcAddress {
    pub fn path<S: Into<String>>(path: S) -> Self {
        IpcAddress::Path(path.into())
    }
    
    pub fn network<S: Into<String>>(host: S, port: u16) -> Self {
        IpcAddress::Network(host.into(), port)
    }
    
    pub fn process(pid: u32) -> Self {
        IpcAddress::Process(pid)
    }
    
    pub fn memory(offset: usize, size: usize) -> Self {
        IpcAddress::Memory(offset, size)
    }
    
    pub fn abstract_ns<S: Into<String>>(name: S) -> Self {
        IpcAddress::Abstract(name.into())
    }
    
    pub fn custom<S: Into<String>>(type_name: S, data: Vec<u8>) -> Self {
        IpcAddress::Custom(type_name.into(), data)
    }
    
    pub fn address_type(&self) -> &'static str {
        match self {
            IpcAddress::Path(_) => "path",
            IpcAddress::Network(_, _) => "network",
            IpcAddress::Process(_) => "process",
            IpcAddress::Memory(_, _) => "memory",
            IpcAddress::Abstract(_) => "abstract",
            IpcAddress::Custom(_, _) => "custom",
        }
    }
}

impl fmt::Display for IpcAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcAddress::Path(path) => write!(f, "path:{}", path),
            IpcAddress::Network(host, port) => write!(f, "net:{}:{}", host, port),
            IpcAddress::Process(pid) => write!(f, "proc:{}", pid),
            IpcAddress::Memory(offset, size) => write!(f, "mem:{}+{}", offset, size),
            IpcAddress::Abstract(name) => write!(f, "abs:{}", name),
            IpcAddress::Custom(type_name, data) => write!(f, "{}:{} bytes", type_name, data.len()),
        }
    }
}

/// Permission structure for IPC resources
#[derive(Debug, Clone, PartialEq)]
pub struct IpcPermissions {
    pub mode: u32,
    pub owner_uid: Option<u32>,
    pub owner_gid: Option<u32>,
    pub access_control: AccessControlList,
}

impl IpcPermissions {
    pub fn new(mode: u32) -> Self {
        Self {
            mode,
            owner_uid: None,
            owner_gid: None,
            access_control: AccessControlList::default(),
        }
    }
    
    pub fn with_owner(mut self, uid: u32, gid: u32) -> Self {
        self.owner_uid = Some(uid);
        self.owner_gid = Some(gid);
        self
    }
    
    pub fn with_acl(mut self, acl: AccessControlList) -> Self {
        self.access_control = acl;
        self
    }
    
    pub fn owner_read(&self) -> bool {
        (self.mode & 0o400) != 0
    }
    
    pub fn owner_write(&self) -> bool {
        (self.mode & 0o200) != 0
    }
    
    pub fn owner_execute(&self) -> bool {
        (self.mode & 0o100) != 0
    }
    
    pub fn group_read(&self) -> bool {
        (self.mode & 0o040) != 0
    }
    
    pub fn group_write(&self) -> bool {
        (self.mode & 0o020) != 0
    }
    
    pub fn group_execute(&self) -> bool {
        (self.mode & 0o010) != 0
    }
    
    pub fn other_read(&self) -> bool {
        (self.mode & 0o004) != 0
    }
    
    pub fn other_write(&self) -> bool {
        (self.mode & 0o002) != 0
    }
    
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
    pub entries: Vec<AclEntry>,
}

impl AccessControlList {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_user_permission(mut self, uid: u32, permissions: AclPermissions) -> Self {
        self.entries.push(AclEntry::User(uid, permissions));
        self
    }
    
    pub fn add_group_permission(mut self, gid: u32, permissions: AclPermissions) -> Self {
        self.entries.push(AclEntry::Group(gid, permissions));
        self
    }
    
    pub fn check_access(&self, uid: u32, gid: u32, requested: AclPermissions) -> bool {
        for entry in &self.entries {
            match entry {
                AclEntry::User(entry_uid, perms) if *entry_uid == uid => {
                    return perms.contains(requested);
                }
                AclEntry::Group(entry_gid, perms) if *entry_gid == gid => {
                    return perms.contains(requested);
                }
                _ => continue,
            }
        }
        false
    }
}

/// Access Control List entry
#[derive(Debug, Clone, PartialEq)]
pub enum AclEntry {
    User(u32, AclPermissions),
    Group(u32, AclPermissions),
}

/// ACL permissions bit flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AclPermissions {
    bits: u8,
}

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
    }
    
    pub const fn with_write(mut self) -> Self {
        self.bits |= Self::WRITE.bits;
        self
    }
    
    pub const fn with_execute(mut self) -> Self {
        self.bits |= Self::EXECUTE.bits;
        self
    }
    
    pub fn contains(self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }
    
    pub fn read(self) -> bool {
        (self.bits & Self::READ.bits) != 0
    }
    
    pub fn write(self) -> bool {
        (self.bits & Self::WRITE.bits) != 0
    }
    
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
    None,
    /// Immediate return (non-blocking)
    Immediate,
    /// Timeout after specified duration
    Duration(Duration),
    /// Timeout at absolute time
    Absolute(SystemTime),
}

impl IpcTimeout {
    pub fn none() -> Self {
        IpcTimeout::None
    }
    
    pub fn immediate() -> Self {
        IpcTimeout::Immediate
    }
    
    pub fn after(duration: Duration) -> Self {
        IpcTimeout::Duration(duration)
    }
    
    pub fn at(time: SystemTime) -> Self {
        IpcTimeout::Absolute(time)
    }
    
    pub fn is_blocking(&self) -> bool {
        matches!(self, IpcTimeout::None)
    }
    
    pub fn is_immediate(&self) -> bool {
        matches!(self, IpcTimeout::Immediate)
    }
    
    pub fn remaining_time(&self) -> Option<Duration> {
        match self {
            IpcTimeout::None => None,
            IpcTimeout::Immediate => Some(Duration::from_secs(0)),
            IpcTimeout::Duration(d) => Some(*d),
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
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub blocked_operations: u64,
    
    // Data transfer
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    
    // Timing
    pub total_time: Duration,
    pub average_operation_time: Duration,
    pub min_operation_time: Duration,
    pub max_operation_time: Duration,
    
    // Resource usage
    pub memory_usage: usize,
    pub handles_created: u64,
    pub handles_destroyed: u64,
    pub peak_concurrent_operations: usize,
    
    // Error breakdown
    pub timeout_errors: u64,
    pub permission_errors: u64,
    pub resource_errors: u64,
    pub io_errors: u64,
    
    // Resource-specific metrics
    pub custom_metrics: HashMap<String, u64>,
    
    // Timestamps
    pub created_at: SystemTime,
    pub last_operation: Option<SystemTime>,
    pub last_error: Option<SystemTime>,
}

impl IpcStatistics {
    pub fn new() -> Self {
        Self {
            created_at: SystemTime::now(),
            min_operation_time: Duration::from_secs(u64::MAX),
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
        }
        
        // Update timing statistics
        self.total_time += duration;
        if duration < self.min_operation_time {
            self.min_operation_time = duration;
        }
        if duration > self.max_operation_time {
            self.max_operation_time = duration;
        }
        
        // Calculate running average
        if self.total_operations > 0 {
            self.average_operation_time = self.total_time / self.total_operations as u32;
        }
    }
    
    pub fn record_data_transfer(&mut self, sent: u64, received: u64) {
        self.bytes_sent += sent;
        self.bytes_received += received;
    }
    
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
            "timeout" => self.timeout_errors += 1,
            "permission" => self.permission_errors += 1,
            "resource" => self.resource_errors += 1,
            "io" => self.io_errors += 1,
            _ => {
                let count = self.custom_metrics.entry(format!("{}_errors", error_type)).or_insert(0);
                *count += 1;
            }
        }
    }
    
    pub fn set_custom_metric(&mut self, name: &str, value: u64) {
        self.custom_metrics.insert(name.to_string(), value);
    }
    
    pub fn increment_custom_metric(&mut self, name: &str, delta: u64) {
        let count = self.custom_metrics.entry(name.to_string()).or_insert(0);
        *count += delta;
    }
    
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
    pub message_queues: bool,
    pub named_pipes: bool,
    pub anonymous_pipes: bool,
    pub shared_memory: bool,
    pub semaphores: bool,
    pub signals: bool,
    pub unix_sockets: bool,
    pub file_locking: bool,
    pub rpc: bool,
    pub security: bool,
    pub transport_abstraction: bool,
}

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
        if self.transport_abstraction { mechanisms.push("transport_abstraction"); }
        
        mechanisms
    }
    
    pub fn count_supported(&self) -> usize {
        self.supported_mechanisms().len()
    }
}

impl Default for IpcCapabilities {
    fn default() -> Self {
        Self {
            message_queues: true,
            named_pipes: cfg!(unix),
            anonymous_pipes: true,
            shared_memory: true,
            semaphores: true,
            signals: cfg!(unix),
            unix_sockets: cfg!(unix),
            file_locking: true,
            rpc: true,
            security: true,
            transport_abstraction: true,
        }
    }
}

/// Resource limit configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceLimits {
    pub max_handles: Option<usize>,
    pub max_memory: Option<usize>,
    pub max_message_size: Option<usize>,
    pub max_queue_depth: Option<usize>,
    pub max_connections: Option<usize>,
}

impl ResourceLimits {
    pub fn unlimited() -> Self {
        Self {
            max_handles: None,
            max_memory: None,
            max_message_size: None,
            max_queue_depth: None,
            max_connections: None,
        }
    }
    
    pub fn conservative() -> Self {
        Self {
            max_handles: Some(1000),
            max_memory: Some(1024 * 1024 * 100), // 100MB
            max_message_size: Some(1024 * 1024),  // 1MB
            max_queue_depth: Some(1000),
            max_connections: Some(100),
        }
    }
    
    pub fn with_max_handles(mut self, limit: usize) -> Self {
        self.max_handles = Some(limit);
        self
    }
    
    pub fn with_max_memory(mut self, limit: usize) -> Self {
        self.max_memory = Some(limit);
        self
    }
    
    pub fn with_max_message_size(mut self, limit: usize) -> Self {
        self.max_message_size = Some(limit);
        self
    }
    
    pub fn with_max_queue_depth(mut self, limit: usize) -> Self {
        self.max_queue_depth = Some(limit);
        self
    }
    
    pub fn with_max_connections(mut self, limit: usize) -> Self {
        self.max_connections = Some(limit);
        self
    }
    
    pub fn check_handles(&self, current: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_handles {
            if current >= limit {
                return Err(IpcError::ResourceExhausted(
                    format!("Handle limit exceeded: {} >= {}", current, limit)
                ));
            }
        }
        Ok(())
    }
    
    pub fn check_memory(&self, current: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_memory {
            if current >= limit {
                return Err(IpcError::ResourceExhausted(
                    format!("Memory limit exceeded: {} >= {}", current, limit)
                ));
            }
        }
        Ok(())
    }
    
    pub fn check_message_size(&self, size: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_message_size {
            if size >= limit {
                return Err(IpcError::InvalidInput(
                    format!("Message size too large: {} >= {}", size, limit)
                ));
            }
        }
        Ok(())
    }
    
    pub fn check_queue_depth(&self, depth: usize) -> IpcResult<()> {
        if let Some(limit) = self.max_queue_depth {
            if depth >= limit {
                return Err(IpcError::ResourceExhausted(
                    format!("Queue depth limit exceeded: {} >= {}", depth, limit)
                ));
            }
        }
        Ok(())
    }
    
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_handle_creation() {
        let handle = IpcHandle::new("test_handle".to_string(), "pipe".to_string())
            .with_metadata("owner".to_string(), "test_process".to_string());
        
        assert_eq!(handle.id, "test_handle");
        assert_eq!(handle.resource_type, "pipe");
        assert_eq!(handle.get_metadata("owner"), Some(&"test_process".to_string()));
        assert!(handle.age() < Duration::from_secs(1));
    }

    #[test]
    fn test_ipc_address_variants() {
        let path_addr = IpcAddress::path("/tmp/test");
        let net_addr = IpcAddress::network("localhost", 8080);
        let proc_addr = IpcAddress::process(1234);
        let mem_addr = IpcAddress::memory(0x1000, 4096);
        
        assert_eq!(path_addr.address_type(), "path");
        assert_eq!(net_addr.address_type(), "network");
        assert_eq!(proc_addr.address_type(), "process");
        assert_eq!(mem_addr.address_type(), "memory");
        
        assert_eq!(format!("{}", path_addr), "path:/tmp/test");
        assert_eq!(format!("{}", net_addr), "net:localhost:8080");
        assert_eq!(format!("{}", proc_addr), "proc:1234");
        assert_eq!(format!("{}", mem_addr), "mem:4096+4096");
    }

    #[test]
    fn test_ipc_permissions() {
        let perms = IpcPermissions::new(0o755)
            .with_owner(1000, 1000);
        
        assert!(perms.owner_read());
        assert!(perms.owner_write());
        assert!(perms.owner_execute());
        assert!(perms.group_read());
        assert!(!perms.group_write());
        assert!(perms.group_execute());
        assert!(perms.other_read());
        assert!(!perms.other_write());
        assert!(perms.other_execute());
        
        assert_eq!(perms.owner_uid, Some(1000));
        assert_eq!(perms.owner_gid, Some(1000));
    }

    #[test]
    fn test_acl_permissions() {
        let read_write = AclPermissions::new().with_read().with_write();
        let read_only = AclPermissions::READ;
        
        assert!(read_write.contains(read_only));
        assert!(read_write.read());
        assert!(read_write.write());
        assert!(!read_write.execute());
        
        assert!(read_only.read());
        assert!(!read_only.write());
        assert!(!read_only.execute());
    }

    #[test]
    fn test_access_control_list() {
        let acl = AccessControlList::new()
            .add_user_permission(1000, AclPermissions::READ.with_write())
            .add_group_permission(100, AclPermissions::READ);
        
        assert!(acl.check_access(1000, 50, AclPermissions::READ));
        assert!(acl.check_access(1000, 50, AclPermissions::WRITE));
        assert!(!acl.check_access(1000, 50, AclPermissions::EXECUTE));
        
        assert!(acl.check_access(2000, 100, AclPermissions::READ));
        assert!(!acl.check_access(2000, 100, AclPermissions::WRITE));
    }

    #[test]
    fn test_ipc_timeout() {
        let none_timeout = IpcTimeout::none();
        let immediate_timeout = IpcTimeout::immediate();
        let duration_timeout = IpcTimeout::after(Duration::from_secs(30));
        
        assert!(none_timeout.is_blocking());
        assert!(!none_timeout.is_immediate());
        
        assert!(!immediate_timeout.is_blocking());
        assert!(immediate_timeout.is_immediate());
        
        assert!(!duration_timeout.is_blocking());
        assert!(!duration_timeout.is_immediate());
        assert_eq!(duration_timeout.remaining_time(), Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_ipc_statistics() {
        let mut stats = IpcStatistics::new();
        
        assert_eq!(stats.total_operations, 0);
        assert_eq!(stats.success_rate(), 0.0);
        
        stats.record_operation(true, Duration::from_millis(10));
        stats.record_operation(false, Duration::from_millis(20));
        stats.record_operation(true, Duration::from_millis(15));
        
        assert_eq!(stats.total_operations, 3);
        assert_eq!(stats.successful_operations, 2);
        assert_eq!(stats.failed_operations, 1);
        assert!((stats.success_rate() - 2.0/3.0).abs() < f64::EPSILON);
        
        stats.record_data_transfer(100, 200);
        assert_eq!(stats.bytes_sent, 100);
        assert_eq!(stats.bytes_received, 200);
        
        stats.record_error("timeout");
        assert_eq!(stats.timeout_errors, 1);
        
        stats.set_custom_metric("custom_count", 42);
        assert_eq!(stats.custom_metrics.get("custom_count"), Some(&42));
    }

    #[test]
    fn test_ipc_capabilities() {
        let caps = IpcCapabilities::default();
        let mechanisms = caps.supported_mechanisms();
        
        assert!(mechanisms.contains(&"message_queues"));
        assert!(mechanisms.contains(&"anonymous_pipes"));
        assert!(mechanisms.contains(&"shared_memory"));
        assert!(mechanisms.contains(&"semaphores"));
        assert!(mechanisms.contains(&"file_locking"));
        assert!(mechanisms.contains(&"rpc"));
        assert!(mechanisms.contains(&"security"));
        
        assert!(caps.count_supported() >= 7); // At least these basic ones
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits::conservative()
            .with_max_handles(500)
            .with_max_memory(1024 * 1024);
        
        assert!(limits.check_handles(100).is_ok());
        assert!(limits.check_handles(600).is_err());
        
        assert!(limits.check_memory(1024).is_ok());
        assert!(limits.check_memory(2 * 1024 * 1024).is_err());
        
        assert!(limits.check_message_size(1024).is_ok());
        assert!(limits.check_message_size(2 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_unlimited_resource_limits() {
        let limits = ResourceLimits::unlimited();
        
        assert!(limits.check_handles(1_000_000).is_ok());
        assert!(limits.check_memory(1024 * 1024 * 1024).is_ok());
        assert!(limits.check_message_size(100 * 1024 * 1024).is_ok());
        assert!(limits.check_queue_depth(10_000).is_ok());
        assert!(limits.check_connections(1_000).is_ok());
    }
}
