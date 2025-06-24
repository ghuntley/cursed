use crate::error::Error;
/// Resource limits implementation for CURSED processes
/// 
/// Resource limits are essential for system programming and security because they:
/// - Prevent runaway processes from consuming unlimited system resources
/// - Enable fair resource sharing in multi-tenant environments
/// - Protect against denial-of-service attacks
/// - Allow building robust containerized applications
/// - Enable process sandboxing and isolation
/// - Support building system administration tools
/// 
/// This module provides comprehensive resource limit management with proper
/// error handling and cross-platform support.

use std::collections::HashMap;
use std::time::Duration;

use crate::stdlib::process::error::{
    ProcessResult, ProcessError, process_not_found_pid, permission_denied_pid,
    invalid_state, execution_failed, timeout_error, system_error, invalid_arguments
};

/// Resource types that can be limited
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// Maximum CPU time in seconds
    CpuTime,
    /// Maximum file size in bytes
    FileSize,
    /// Maximum data segment size in bytes
    DataSize,
    /// Maximum stack size in bytes
    StackSize,
    /// Maximum core file size in bytes
    CoreFileSize,
    /// Maximum resident set size in bytes
    ResidentSetSize,
    /// Maximum number of processes
    ProcessCount,
    /// Maximum number of open files
    OpenFiles,
    /// Maximum locked memory in bytes
    LockedMemory,
    /// Maximum address space in bytes
    AddressSpace,
    /// Maximum file locks
    FileLocks,
    /// Maximum pending signals
    PendingSignals,
    /// Maximum message queue bytes
    MessageQueueBytes,
    /// Maximum nice priority
    Nice,
    /// Maximum real-time priority
    RealtimePriority,
    /// Maximum real-time timeout
    RealtimeTimeout,
}

/// Resource limit values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimit {
    /// Soft limit (can be increased up to hard limit)
    pub soft: u64,
    /// Hard limit (maximum possible value)
    pub hard: u64,
}

impl ResourceLimit {
    /// Create a new resource limit
    pub fn new(soft: u64, hard: u64) -> Self {
        Self { soft, hard }
    }

    /// Create unlimited resource limit
    pub fn unlimited() -> Self {
        Self {
            soft: u64::MAX,
            hard: u64::MAX,
        }
    }

    /// Create resource limit with same soft and hard values
    pub fn fixed(value: u64) -> Self {
        Self {
            soft: value,
            hard: value,
        }
    }

    /// Check if this limit is unlimited
    pub fn is_unlimited(&self) -> bool {
        self.soft == u64::MAX && self.hard == u64::MAX
    }

    /// Check if a value exceeds the soft limit
    pub fn exceeds_soft(&self, value: u64) -> bool {
        value > self.soft
    }

    /// Check if a value exceeds the hard limit
    pub fn exceeds_hard(&self, value: u64) -> bool {
        value > self.hard
    }
}

/// Current resource usage
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    /// User CPU time used
    pub user_time: Duration,
    /// System CPU time used
    pub system_time: Duration,
    /// Maximum resident set size
    pub max_rss: u64,
    /// Integral shared memory size
    pub shared_memory: u64,
    /// Integral unshared data size
    pub unshared_data: u64,
    /// Integral unshared stack size
    pub unshared_stack: u64,
    /// Page reclaims (soft page faults)
    pub minor_faults: u64,
    /// Page faults (hard page faults)
    pub major_faults: u64,
    /// Swaps
    pub swaps: u64,
    /// Block input operations
    pub block_input: u64,
    /// Block output operations
    pub block_output: u64,
    /// IPC messages sent
    pub messages_sent: u64,
    /// IPC messages received
    pub messages_received: u64,
    /// Signals received
    pub signals_received: u64,
    /// Voluntary context switches
    pub voluntary_switches: u64,
    /// Involuntary context switches
    pub involuntary_switches: u64,
}

/// Resource limit manager
pub struct ResourceLimitManager {
    /// Current limits for different resource types
    limits: HashMap<ResourceType, ResourceLimit>,
    /// Whether limits are enforced
    enforcement_enabled: bool,
}

impl ResourceLimitManager {
    /// Create a new resource limit manager
    pub fn new() -> Self {
        Self {
            limits: HashMap::new(),
            enforcement_enabled: true,
        }
    }

    /// Set resource limit for current process
    pub fn set_limit(&mut self, resource: ResourceType, limit: ResourceLimit) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            let resource_id = match resource {
                ResourceType::CpuTime => libc::RLIMIT_CPU,
                ResourceType::FileSize => libc::RLIMIT_FSIZE,
                ResourceType::DataSize => libc::RLIMIT_DATA,
                ResourceType::StackSize => libc::RLIMIT_STACK,
                ResourceType::CoreFileSize => libc::RLIMIT_CORE,
                ResourceType::ResidentSetSize => libc::RLIMIT_RSS,
                ResourceType::ProcessCount => libc::RLIMIT_NPROC,
                ResourceType::OpenFiles => libc::RLIMIT_NOFILE,
                ResourceType::LockedMemory => libc::RLIMIT_MEMLOCK,
                ResourceType::AddressSpace => libc::RLIMIT_AS,
                #[cfg(target_os = "linux")]
                ResourceType::FileLocks => libc::RLIMIT_LOCKS,
                #[cfg(target_os = "linux")]
                ResourceType::PendingSignals => libc::RLIMIT_SIGPENDING,
                #[cfg(target_os = "linux")]
                ResourceType::MessageQueueBytes => libc::RLIMIT_MSGQUEUE,
                #[cfg(target_os = "linux")]
                ResourceType::Nice => libc::RLIMIT_NICE,
                #[cfg(target_os = "linux")]
                ResourceType::RealtimePriority => libc::RLIMIT_RTPRIO,
                #[cfg(target_os = "linux")]
                ResourceType::RealtimeTimeout => libc::RLIMIT_RTTIME,
                #[cfg(not(target_os = "linux"))]
                _ => return Err(system_error(-1, "set_limit", "Resource type not supported on this platform")),
            };

            let rlimit = libc::rlimit {
                rlim_cur: if limit.soft == u64::MAX { libc::RLIM_INFINITY } else { limit.soft },
                rlim_max: if limit.hard == u64::MAX { libc::RLIM_INFINITY } else { limit.hard },
            };

            let result = unsafe { libc::setrlimit(resource_id, &rlimit) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "set_limit",
                    &format!("Failed to set resource limit for {:?}", resource)
                ));
            }
        }

        #[cfg(windows)]
        {
            // Windows has different resource management - use job objects
            match resource {
                ResourceType::ProcessCount | ResourceType::OpenFiles => {
                    // These can be somewhat controlled on Windows
                    tracing::warn!("Resource limit {:?} partially supported on Windows", resource);
                }
                _ => {
                    return Err(system_error(-1, "set_limit", "Resource limits not fully supported on Windows"));
                }
            }
        }

        // Store the limit for tracking
        self.limits.insert(resource, limit);
        Ok(())
    }

    /// Get resource limit for current process
    pub fn get_limit(&self, resource: ResourceType) -> ProcessResult<ResourceLimit> {
        #[cfg(unix)]
        {
            let resource_id = match resource {
                ResourceType::CpuTime => libc::RLIMIT_CPU,
                ResourceType::FileSize => libc::RLIMIT_FSIZE,
                ResourceType::DataSize => libc::RLIMIT_DATA,
                ResourceType::StackSize => libc::RLIMIT_STACK,
                ResourceType::CoreFileSize => libc::RLIMIT_CORE,
                ResourceType::ResidentSetSize => libc::RLIMIT_RSS,
                ResourceType::ProcessCount => libc::RLIMIT_NPROC,
                ResourceType::OpenFiles => libc::RLIMIT_NOFILE,
                ResourceType::LockedMemory => libc::RLIMIT_MEMLOCK,
                ResourceType::AddressSpace => libc::RLIMIT_AS,
                #[cfg(target_os = "linux")]
                ResourceType::FileLocks => libc::RLIMIT_LOCKS,
                #[cfg(target_os = "linux")]
                ResourceType::PendingSignals => libc::RLIMIT_SIGPENDING,
                #[cfg(target_os = "linux")]
                ResourceType::MessageQueueBytes => libc::RLIMIT_MSGQUEUE,
                #[cfg(target_os = "linux")]
                ResourceType::Nice => libc::RLIMIT_NICE,
                #[cfg(target_os = "linux")]
                ResourceType::RealtimePriority => libc::RLIMIT_RTPRIO,
                #[cfg(target_os = "linux")]
                ResourceType::RealtimeTimeout => libc::RLIMIT_RTTIME,
                #[cfg(not(target_os = "linux"))]
                _ => return Err(system_error(-1, "get_limit", "Resource type not supported on this platform")),
            };

            let mut rlimit = libc::rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            };

            let result = unsafe { libc::getrlimit(resource_id, &mut rlimit) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "get_limit",
                    &format!("Failed to get resource limit for {:?}", resource)
                ));
            }

            let soft = if rlimit.rlim_cur == libc::RLIM_INFINITY { u64::MAX } else { rlimit.rlim_cur };
            let hard = if rlimit.rlim_max == libc::RLIM_INFINITY { u64::MAX } else { rlimit.rlim_max };

            Ok(ResourceLimit::new(soft, hard))
        }

        #[cfg(not(unix))]
        {
            // Return stored limit if available, otherwise unlimited
            Ok(self.limits.get(&resource).copied().unwrap_or_else(|| ResourceLimit::unlimited()))
        }
    }

    /// Get current resource usage
    pub fn get_usage(&self) -> ProcessResult<ResourceUsage> {
        #[cfg(unix)]
        {
            let mut rusage = unsafe { std::mem::zeroed::<libc::rusage>() };
            let result = unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut rusage) };
            
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "get_usage",
                    "Failed to get resource usage"
                ));
            }

            Ok(ResourceUsage {
                user_time: Duration::new(
                    rusage.ru_utime.tv_sec as u64,
                    (rusage.ru_utime.tv_usec * 1000) as u32
                ),
                system_time: Duration::new(
                    rusage.ru_stime.tv_sec as u64,
                    (rusage.ru_stime.tv_usec * 1000) as u32
                ),
                max_rss: rusage.ru_maxrss as u64,
                shared_memory: rusage.ru_ixrss as u64,
                unshared_data: rusage.ru_idrss as u64,
                unshared_stack: rusage.ru_isrss as u64,
                minor_faults: rusage.ru_minflt as u64,
                major_faults: rusage.ru_majflt as u64,
                swaps: rusage.ru_nswap as u64,
                block_input: rusage.ru_inblock as u64,
                block_output: rusage.ru_oublock as u64,
                messages_sent: rusage.ru_msgsnd as u64,
                messages_received: rusage.ru_msgrcv as u64,
                signals_received: rusage.ru_nsignals as u64,
                voluntary_switches: rusage.ru_nvcsw as u64,
                involuntary_switches: rusage.ru_nivcsw as u64,
            })
        }

        #[cfg(windows)]
        {
            // Windows resource usage through GetProcessTimes and other APIs
            use std::mem;
            
            let mut usage = ResourceUsage::default();
            
            // Get process handle
            let handle = unsafe { winapi::um::processthreadsapi::GetCurrentProcess() };
            
            // Get CPU times
            let mut creation_time = unsafe { mem::zeroed() };
            let mut exit_time = unsafe { mem::zeroed() };
            let mut kernel_time = unsafe { mem::zeroed() };
            let mut user_time = unsafe { mem::zeroed() };
            
            if unsafe { winapi::um::processthreadsapi::GetProcessTimes(
                handle,
                &mut creation_time,
                &mut exit_time,
                &mut kernel_time,
                &mut user_time,
            ) } != 0 {
                // Convert FILETIME to Duration (100ns intervals)
                let user_ns = ((user_time.dwHighDateTime as u64) << 32) | (user_time.dwLowDateTime as u64);
                let kernel_ns = ((kernel_time.dwHighDateTime as u64) << 32) | (kernel_time.dwLowDateTime as u64);
                
                usage.user_time = Duration::from_nanos(user_ns * 100);
                usage.system_time = Duration::from_nanos(kernel_ns * 100);
            }
            
            // Get memory usage
            let mut mem_counters = unsafe { mem::zeroed() };
            if unsafe { winapi::um::psapi::GetProcessMemoryInfo(
                handle,
                &mut mem_counters,
                mem::size_of_val(&mem_counters) as u32,
            ) } != 0 {
                usage.max_rss = mem_counters.WorkingSetSize as u64;
            }
            
            Ok(usage)
        }

        #[cfg(not(any(unix, windows)))]
        {
            Ok(ResourceUsage::default())
        }
    }

    /// Check if a resource value exceeds its limit
    pub fn check_limit(&self, resource: ResourceType, current_value: u64) -> ProcessResult<bool> {
        let limit = self.get_limit(resource)?;
        Ok(limit.exceeds_soft(current_value))
    }

    /// Set multiple limits at once
    pub fn set_limits(&mut self, limits: &[(ResourceType, ResourceLimit)]) -> ProcessResult<()> {
        for (resource, limit) in limits {
            self.set_limit(*resource, *limit)?;
        }
        Ok(())
    }

    /// Get all current limits
    pub fn get_all_limits(&self) -> ProcessResult<HashMap<ResourceType, ResourceLimit>> {
        let mut all_limits = HashMap::new();
        
        // Standard POSIX resource types
        let standard_resources = [
            ResourceType::CpuTime,
            ResourceType::FileSize,
            ResourceType::DataSize,
            ResourceType::StackSize,
            ResourceType::CoreFileSize,
            ResourceType::ResidentSetSize,
            ResourceType::ProcessCount,
            ResourceType::OpenFiles,
            ResourceType::LockedMemory,
            ResourceType::AddressSpace,
        ];

        for resource in &standard_resources {
            if let Ok(limit) = self.get_limit(*resource) {
                all_limits.insert(*resource, limit);
            }
        }

        // Linux-specific resources
        #[cfg(target_os = "linux")]
        {
            let linux_resources = [
                ResourceType::FileLocks,
                ResourceType::PendingSignals,
                ResourceType::MessageQueueBytes,
                ResourceType::Nice,
                ResourceType::RealtimePriority,
                ResourceType::RealtimeTimeout,
            ];

            for resource in &linux_resources {
                if let Ok(limit) = self.get_limit(*resource) {
                    all_limits.insert(*resource, limit);
                }
            }
        }

        Ok(all_limits)
    }

    /// Create a resource-limited environment for child processes
    pub fn create_limited_environment(&self) -> ProcessResult<LimitedEnvironment> {
        Ok(LimitedEnvironment {
            limits: self.limits.clone(),
            enforcement_enabled: self.enforcement_enabled,
        })
    }

    /// Enable or disable limit enforcement
    pub fn set_enforcement(&mut self, enabled: bool) {
        self.enforcement_enabled = enabled;
    }

    /// Check if enforcement is enabled
    pub fn is_enforcement_enabled(&self) -> bool {
        self.enforcement_enabled
    }
}

/// Environment with predefined resource limits
#[derive(Debug, Clone)]
pub struct LimitedEnvironment {
    limits: HashMap<ResourceType, ResourceLimit>,
    enforcement_enabled: bool,
}

impl LimitedEnvironment {
    /// Apply limits to current process
    pub fn apply(&self) -> ProcessResult<()> {
        if !self.enforcement_enabled {
            return Ok(());
        }

        let mut manager = ResourceLimitManager::new();
        for (resource, limit) in &self.limits {
            manager.set_limit(*resource, *limit)?;
        }
        Ok(())
    }

    /// Get limit for a specific resource
    pub fn get_limit(&self, resource: ResourceType) -> Option<ResourceLimit> {
        self.limits.get(&resource).copied()
    }

    /// Add or update a limit
    pub fn set_limit(&mut self, resource: ResourceType, limit: ResourceLimit) {
        self.limits.insert(resource, limit);
    }

    /// Remove a limit
    pub fn remove_limit(&mut self, resource: ResourceType) {
        self.limits.remove(&resource);
    }

    /// Create a sandboxed environment with strict limits
    pub fn sandboxed() -> Self {
        let mut env = Self {
            limits: HashMap::new(),
            enforcement_enabled: true,
        };

        // Restrictive limits suitable for sandboxing
        env.set_limit(ResourceType::CpuTime, ResourceLimit::fixed(300)); // 5 minutes
        env.set_limit(ResourceType::FileSize, ResourceLimit::fixed(100 * 1024 * 1024)); // 100MB
        env.set_limit(ResourceType::DataSize, ResourceLimit::fixed(512 * 1024 * 1024)); // 512MB
        env.set_limit(ResourceType::StackSize, ResourceLimit::fixed(8 * 1024 * 1024)); // 8MB
        env.set_limit(ResourceType::CoreFileSize, ResourceLimit::fixed(0)); // No core dumps
        env.set_limit(ResourceType::ProcessCount, ResourceLimit::fixed(10)); // Max 10 processes
        env.set_limit(ResourceType::OpenFiles, ResourceLimit::fixed(100)); // Max 100 files
        env.set_limit(ResourceType::LockedMemory, ResourceLimit::fixed(0)); // No locked memory
        env.set_limit(ResourceType::AddressSpace, ResourceLimit::fixed(1024 * 1024 * 1024)); // 1GB

        env
    }

    /// Create a development environment with reasonable limits
    pub fn development() -> Self {
        let mut env = Self {
            limits: HashMap::new(),
            enforcement_enabled: true,
        };

        // More permissive limits for development
        env.set_limit(ResourceType::CpuTime, ResourceLimit::unlimited());
        env.set_limit(ResourceType::FileSize, ResourceLimit::fixed(10 * 1024 * 1024 * 1024)); // 10GB
        env.set_limit(ResourceType::DataSize, ResourceLimit::fixed(8 * 1024 * 1024 * 1024)); // 8GB
        env.set_limit(ResourceType::StackSize, ResourceLimit::fixed(64 * 1024 * 1024)); // 64MB
        env.set_limit(ResourceType::CoreFileSize, ResourceLimit::fixed(1024 * 1024 * 1024)); // 1GB
        env.set_limit(ResourceType::ProcessCount, ResourceLimit::fixed(1000)); // Max 1000 processes
        env.set_limit(ResourceType::OpenFiles, ResourceLimit::fixed(4096)); // Max 4096 files

        env
    }
}

/// Resource monitoring
pub struct ResourceMonitor {
    manager: ResourceLimitManager,
    thresholds: HashMap<ResourceType, f64>, // Percentage thresholds (0.0 - 1.0)
}

impl ResourceMonitor {
    /// Create a new resource monitor
    pub fn new() -> Self {
        Self {
            manager: ResourceLimitManager::new(),
            thresholds: HashMap::new(),
        }
    }

    /// Set warning threshold for a resource (as percentage of limit)
    pub fn set_threshold(&mut self, resource: ResourceType, threshold: f64) -> ProcessResult<()> {
        if threshold < 0.0 || threshold > 1.0 {
            return Err(invalid_arguments("set_threshold", "threshold", "Threshold must be between 0.0 and 1.0"));
        }
        self.thresholds.insert(resource, threshold);
        Ok(())
    }

    /// Check current usage against thresholds
    pub fn check_thresholds(&self) -> ProcessResult<Vec<ResourceWarning>> {
        let mut warnings = Vec::new();
        let usage = self.manager.get_usage()?;

        for (resource, threshold) in &self.thresholds {
            let limit = self.manager.get_limit(*resource)?;
            let current_value = match resource {
                ResourceType::CpuTime => usage.user_time.as_secs() + usage.system_time.as_secs(),
                ResourceType::ResidentSetSize => usage.max_rss,
                _ => continue, // Would need to map other resource types to usage fields
            };

            if !limit.is_unlimited() && current_value as f64 / limit.soft as f64 > *threshold {
                warnings.push(ResourceWarning {
                    resource: *resource,
                    current_value,
                    limit_value: limit.soft,
                    threshold: *threshold,
                    percentage: current_value as f64 / limit.soft as f64,
                });
            }
        }

        Ok(warnings)
    }
}

/// Resource usage warning
#[derive(Debug, Clone)]
pub struct ResourceWarning {
    pub resource: ResourceType,
    pub current_value: u64,
    pub limit_value: u64,
    pub threshold: f64,
    pub percentage: f64,
}

impl std::fmt::Display for ResourceWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Resource {:?} at {:.1}% of limit ({}/{}, threshold {:.1}%)",
            self.resource,
            self.percentage * 100.0,
            self.current_value,
            self.limit_value,
            self.threshold * 100.0
        )
    }
}

/// Convenience functions for common resource limit operations

/// Set memory limit for current process
pub fn set_memory_limit(limit_mb: u64) -> ProcessResult<()> {
    let mut manager = ResourceLimitManager::new();
    let limit_bytes = limit_mb * 1024 * 1024;
    manager.set_limit(ResourceType::AddressSpace, ResourceLimit::fixed(limit_bytes))
}

/// Set CPU time limit for current process
pub fn set_cpu_time_limit(seconds: u64) -> ProcessResult<()> {
    let mut manager = ResourceLimitManager::new();
    manager.set_limit(ResourceType::CpuTime, ResourceLimit::fixed(seconds))
}

/// Set file descriptor limit for current process
pub fn set_file_descriptor_limit(count: u64) -> ProcessResult<()> {
    let mut manager = ResourceLimitManager::new();
    manager.set_limit(ResourceType::OpenFiles, ResourceLimit::fixed(count))
}

/// Get current memory usage in MB
pub fn get_memory_usage_mb() -> ProcessResult<u64> {
    let manager = ResourceLimitManager::new();
    let usage = manager.get_usage()?;
    Ok(usage.max_rss / 1024 / 1024)
}

/// Get current CPU time used
pub fn get_cpu_time() -> ProcessResult<Duration> {
    let manager = ResourceLimitManager::new();
    let usage = manager.get_usage()?;
    Ok(usage.user_time + usage.system_time)
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_resource_limit() {
        let limit = ResourceLimit::new(1024, 2048);
        assert_eq!(limit.soft, 1024);
        assert_eq!(limit.hard, 2048);
        assert!(!limit.is_unlimited());
        assert!(limit.exceeds_soft(1025));
        assert!(!limit.exceeds_soft(1000));
    }

    #[test]
    fn test_unlimited_limit() {
        let limit = ResourceLimit::unlimited();
        assert!(limit.is_unlimited());
        assert!(!limit.exceeds_soft(u64::MAX - 1));
    }

    #[test]
    fn test_resource_limit_manager() {
        let mut manager = ResourceLimitManager::new();
        assert!(manager.is_enforcement_enabled());
        
        manager.set_enforcement(false);
        assert!(!manager.is_enforcement_enabled());
    }

    #[test]
    fn test_limited_environment() {
        let mut env = LimitedEnvironment::sandboxed();
        
        let cpu_limit = env.get_limit(ResourceType::CpuTime).unwrap();
        assert_eq!(cpu_limit.soft, 300);
        
        env.set_limit(ResourceType::CpuTime, ResourceLimit::fixed(600));
        let updated_limit = env.get_limit(ResourceType::CpuTime).unwrap();
        assert_eq!(updated_limit.soft, 600);
    }

    #[test]
    fn test_resource_monitor() {
        let mut monitor = ResourceMonitor::new();
        assert!(monitor.set_threshold(ResourceType::CpuTime, 0.8).is_ok());
        assert!(monitor.set_threshold(ResourceType::CpuTime, 1.5).is_err());
    }

    #[test]
    fn test_resource_warning_display() {
        let warning = ResourceWarning {
            resource: ResourceType::CpuTime,
            current_value: 800,
            limit_value: 1000,
            threshold: 0.8,
            percentage: 0.8,
        };
        
        let display = format!("{}", warning);
        assert!(display.contains("CpuTime"));
        assert!(display.contains("80.0%"));
    }

    #[cfg(unix)]
    #[test]
    fn test_get_resource_usage() {
        let manager = ResourceLimitManager::new();
        let usage = manager.get_usage();
        assert!(usage.is_ok());
        
        let usage = usage.unwrap();
        // Process should have used some CPU time and memory
        assert!(usage.user_time.as_nanos() > 0 || usage.system_time.as_nanos() > 0);
    }

    #[cfg(unix)]
    #[test]
    fn test_resource_limit_operations() {
        let mut manager = ResourceLimitManager::new();
        
        // Test getting current limits (should work on any Unix system)
        let cpu_limit = manager.get_limit(ResourceType::CpuTime);
        assert!(cpu_limit.is_ok());
        
        let file_limit = manager.get_limit(ResourceType::OpenFiles);
        assert!(file_limit.is_ok());
    }
}

// Type aliases for compatibility with imports
pub type ResourceLimiter = ResourceLimitManager;

/// Configuration for resource limiting operations
#[derive(Debug, Clone)]
pub struct ResourceConfig {
    /// Default limits to apply
    pub default_limits: HashMap<ResourceType, ResourceLimit>,
    /// Enable automatic monitoring
    pub enable_monitoring: bool,
    /// Monitoring interval
    pub monitoring_interval: Duration,
    /// Warning thresholds (percentage of limit)
    pub warning_thresholds: HashMap<ResourceType, f64>,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        let mut default_limits = HashMap::new();
        let mut warning_thresholds = HashMap::new();
        
        // Set reasonable defaults
        default_limits.insert(ResourceType::CpuTime, ResourceLimit::unlimited());
        default_limits.insert(ResourceType::OpenFiles, ResourceLimit::hard(1024));
        default_limits.insert(ResourceType::ProcessCount, ResourceLimit::hard(256));
        
        // Set warning thresholds at 80%
        warning_thresholds.insert(ResourceType::CpuTime, 0.8);
        warning_thresholds.insert(ResourceType::OpenFiles, 0.8);
        warning_thresholds.insert(ResourceType::ProcessCount, 0.8);
        
        Self {
            default_limits,
            enable_monitoring: true,
            monitoring_interval: Duration::from_secs(10),
            warning_thresholds,
        }
    }
}
