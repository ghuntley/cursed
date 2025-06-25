/// Resource limits and system resource management
// use crate::stdlib::sys_core::error::{SysCoreResult, system_call_error, not_supported};
use crate::error::CursedError;

/// Resource types that can be limited
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceType {
    /// CPU time in seconds
    /// File size in bytes
    /// Core dump size in bytes
    /// Stack size in bytes
    /// Heap size in bytes
    /// Number of open files
    /// Number of processes
    /// Virtual memory size in bytes
    /// Resident set size in bytes
/// Resource limit value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResourceLimit {
    /// Soft limit (can be changed by process)
    /// Hard limit (maximum the soft limit can be set to)
impl ResourceLimit {
    /// Create a new resource limit
    pub fn new(soft: u64, hard: u64) -> Self {
        Self { soft, hard }
    }
    
    /// Create unlimited resource limit
    pub fn unlimited() -> Self {
        Self {
        }
    }
    
    /// Check if the limit is unlimited
    pub fn is_unlimited(&self) -> bool {
        self.soft == u64::MAX && self.hard == u64::MAX
    }
}

/// Process resource limits
#[derive(Debug, Clone)]
pub struct ProcessLimits {
impl Default for ProcessLimits {
    fn default() -> Self {
        Self {
        }
    }
/// Get resource limit for a specific resource type
pub fn get_resource_limit(resource: ResourceType) -> SysCoreResult<ResourceLimit> {
    #[cfg(unix)]
    {
        let resource_id = match resource {
        
        let mut rlimit = unsafe { std::mem::zeroed::<libc::rlimit>() };
        let result = unsafe { libc::getrlimit(resource_id, &mut rlimit) };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("getrlimit", errno));
        Ok(ResourceLimit {
        })
    #[cfg(not(unix))]
    {
        Err(not_supported("Resource limits not supported on this platform"))
    }
}

/// Set resource limit for a specific resource type
pub fn set_resource_limit(resource: ResourceType, limit: ResourceLimit) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let resource_id = match resource {
        
        let rlimit = libc::rlimit {
        
        let result = unsafe { libc::setrlimit(resource_id, &rlimit) };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("setrlimit", errno));
        Ok(())
    #[cfg(not(unix))]
    {
        Err(not_supported("Resource limits not supported on this platform"))
    }
}

/// Get all process limits
pub fn get_process_limits() -> SysCoreResult<ProcessLimits> {
    Ok(ProcessLimits {
    })
/// Get maximum number of open files for the current process
pub fn get_max_open_files() -> u64 {
    match get_resource_limit(ResourceType::OpenFiles) {
        Err(_) => 1024, // Default fallback
    }
}

/// Set maximum number of open files for the current process
pub fn set_max_open_files(max_files: u64) -> SysCoreResult<()> {
    let current = get_resource_limit(ResourceType::OpenFiles)?;
    let new_limit = ResourceLimit {
    set_resource_limit(ResourceType::OpenFiles, new_limit)
/// Get current resource usage
pub fn get_resource_usage(who: ResourceUsageWho) -> SysCoreResult<ResourceUsage> {
    #[cfg(unix)]
    {
        let who = match who {
        
        let mut usage = unsafe { std::mem::zeroed::<libc::rusage>() };
        let result = unsafe { libc::getrusage(who, &mut usage) };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("getrusage", errno));
        Ok(ResourceUsage {
        })
    #[cfg(not(unix))]
    {
        Err(not_supported("Resource usage not supported on this platform"))
    }
}

/// Who to get resource usage for
#[derive(Debug, Clone, Copy)]
pub enum ResourceUsageWho {
    /// Current process
    /// All terminated child processes
/// Resource usage information
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// User CPU time (seconds)
    /// User CPU time (microseconds)
    /// System CPU time (seconds)
    /// System CPU time (microseconds)
    /// Maximum resident set size (bytes)
    /// Major page faults
    /// Minor page faults
    /// Voluntary context switches
    /// Involuntary context switches
impl ResourceUsage {
    /// Get total CPU time in seconds
    pub fn total_cpu_time(&self) -> f64 {
        (self.user_time_sec + self.system_time_sec) as f64 +
        (self.user_time_usec + self.system_time_usec) as f64 / 1_000_000.0
    /// Get user CPU time in seconds
    pub fn user_cpu_time(&self) -> f64 {
        self.user_time_sec as f64 + self.user_time_usec as f64 / 1_000_000.0
    /// Get system CPU time in seconds
    pub fn system_cpu_time(&self) -> f64 {
        self.system_time_sec as f64 + self.system_time_usec as f64 / 1_000_000.0
    }
}
