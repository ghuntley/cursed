/// Resource limits and system resource management
// use crate::stdlib::sys_core::error::{SysCoreResult, system_call_error, not_supported};
use crate::error::CursedError;

/// Resource types that can be limited
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceType {
    /// CPU time in seconds
    CpuTime,
    /// File size in bytes
    FileSize,
    /// Core dump size in bytes
    CoreSize,
    /// Stack size in bytes
    StackSize,
    /// Heap size in bytes
    HeapSize,
    /// Number of open files
    OpenFiles,
    /// Number of processes
    Processes,
    /// Virtual memory size in bytes
    VirtualMemory,
    /// Resident set size in bytes
    ResidentMemory,
}

/// Resource limit value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResourceLimit {
    /// Soft limit (can be changed by process)
    pub soft: u64,
    /// Hard limit (maximum the soft limit can be set to)
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
    
    /// Check if the limit is unlimited
    pub fn is_unlimited(&self) -> bool {
        self.soft == u64::MAX && self.hard == u64::MAX
    }
}

/// Process resource limits
#[derive(Debug, Clone)]
pub struct ProcessLimits {
    pub cpu_time: Option<ResourceLimit>,
    pub file_size: Option<ResourceLimit>,
    pub core_size: Option<ResourceLimit>,
    pub stack_size: Option<ResourceLimit>,
    pub heap_size: Option<ResourceLimit>,
    pub open_files: Option<ResourceLimit>,
    pub processes: Option<ResourceLimit>,
    pub virtual_memory: Option<ResourceLimit>,
    pub resident_memory: Option<ResourceLimit>,
}

impl Default for ProcessLimits {
    fn default() -> Self {
        Self {
            cpu_time: None,
            file_size: None,
            core_size: None,
            stack_size: None,
            heap_size: None,
            open_files: None,
            processes: None,
            virtual_memory: None,
            resident_memory: None,
        }
    }
}

/// Get resource limit for a specific resource type
pub fn get_resource_limit(resource: ResourceType) -> SysCoreResult<ResourceLimit> {
    #[cfg(unix)]
    {
        let resource_id = match resource {
            ResourceType::CpuTime => libc::RLIMIT_CPU,
            ResourceType::FileSize => libc::RLIMIT_FSIZE,
            ResourceType::CoreSize => libc::RLIMIT_CORE,
            ResourceType::StackSize => libc::RLIMIT_STACK,
            ResourceType::HeapSize => libc::RLIMIT_DATA,
            ResourceType::OpenFiles => libc::RLIMIT_NOFILE,
            ResourceType::Processes => libc::RLIMIT_NPROC,
            ResourceType::VirtualMemory => libc::RLIMIT_AS,
            ResourceType::ResidentMemory => libc::RLIMIT_RSS,
        };
        
        let mut rlimit = unsafe { std::mem::zeroed::<libc::rlimit>() };
        let result = unsafe { libc::getrlimit(resource_id, &mut rlimit) };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("getrlimit", errno));
        }
        
        Ok(ResourceLimit {
            soft: rlimit.rlim_cur,
            hard: rlimit.rlim_max,
        })
    }
    
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
            ResourceType::CpuTime => libc::RLIMIT_CPU,
            ResourceType::FileSize => libc::RLIMIT_FSIZE,
            ResourceType::CoreSize => libc::RLIMIT_CORE,
            ResourceType::StackSize => libc::RLIMIT_STACK,
            ResourceType::HeapSize => libc::RLIMIT_DATA,
            ResourceType::OpenFiles => libc::RLIMIT_NOFILE,
            ResourceType::Processes => libc::RLIMIT_NPROC,
            ResourceType::VirtualMemory => libc::RLIMIT_AS,
            ResourceType::ResidentMemory => libc::RLIMIT_RSS,
        };
        
        let rlimit = libc::rlimit {
            rlim_cur: limit.soft,
            rlim_max: limit.hard,
        };
        
        let result = unsafe { libc::setrlimit(resource_id, &rlimit) };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("setrlimit", errno));
        }
        
        Ok(())
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("Resource limits not supported on this platform"))
    }
}

/// Get all process limits
pub fn get_process_limits() -> SysCoreResult<ProcessLimits> {
    Ok(ProcessLimits {
        cpu_time: get_resource_limit(ResourceType::CpuTime).ok(),
        file_size: get_resource_limit(ResourceType::FileSize).ok(),
        core_size: get_resource_limit(ResourceType::CoreSize).ok(),
        stack_size: get_resource_limit(ResourceType::StackSize).ok(),
        heap_size: get_resource_limit(ResourceType::HeapSize).ok(),
        open_files: get_resource_limit(ResourceType::OpenFiles).ok(),
        processes: get_resource_limit(ResourceType::Processes).ok(),
        virtual_memory: get_resource_limit(ResourceType::VirtualMemory).ok(),
        resident_memory: get_resource_limit(ResourceType::ResidentMemory).ok(),
    })
}

/// Get maximum number of open files for the current process
pub fn get_max_open_files() -> u64 {
    match get_resource_limit(ResourceType::OpenFiles) {
        Ok(limit) => limit.soft,
        Err(_) => 1024, // Default fallback
    }
}

/// Set maximum number of open files for the current process
pub fn set_max_open_files(max_files: u64) -> SysCoreResult<()> {
    let current = get_resource_limit(ResourceType::OpenFiles)?;
    let new_limit = ResourceLimit {
        soft: max_files.min(current.hard),
        hard: current.hard,
    };
    set_resource_limit(ResourceType::OpenFiles, new_limit)
}

/// Get current resource usage
pub fn get_resource_usage(who: ResourceUsageWho) -> SysCoreResult<ResourceUsage> {
    #[cfg(unix)]
    {
        let who = match who {
            ResourceUsageWho::Self_ => libc::RUSAGE_SELF,
            ResourceUsageWho::Children => libc::RUSAGE_CHILDREN,
        };
        
        let mut usage = unsafe { std::mem::zeroed::<libc::rusage>() };
        let result = unsafe { libc::getrusage(who, &mut usage) };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("getrusage", errno));
        }
        
        Ok(ResourceUsage {
            user_time_sec: usage.ru_utime.tv_sec as u64,
            user_time_usec: usage.ru_utime.tv_usec as u64,
            system_time_sec: usage.ru_stime.tv_sec as u64,
            system_time_usec: usage.ru_stime.tv_usec as u64,
            max_resident_set_size: usage.ru_maxrss as u64,
            page_faults: usage.ru_majflt as u64,
            minor_page_faults: usage.ru_minflt as u64,
            voluntary_context_switches: usage.ru_nvcsw as u64,
            involuntary_context_switches: usage.ru_nivcsw as u64,
        })
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("Resource usage not supported on this platform"))
    }
}

/// Who to get resource usage for
#[derive(Debug, Clone, Copy)]
pub enum ResourceUsageWho {
    /// Current process
    Self_,
    /// All terminated child processes
    Children,
}

/// Resource usage information
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// User CPU time (seconds)
    pub user_time_sec: u64,
    /// User CPU time (microseconds)
    pub user_time_usec: u64,
    /// System CPU time (seconds)
    pub system_time_sec: u64,
    /// System CPU time (microseconds)
    pub system_time_usec: u64,
    /// Maximum resident set size (bytes)
    pub max_resident_set_size: u64,
    /// Major page faults
    pub page_faults: u64,
    /// Minor page faults
    pub minor_page_faults: u64,
    /// Voluntary context switches
    pub voluntary_context_switches: u64,
    /// Involuntary context switches
    pub involuntary_context_switches: u64,
}

impl ResourceUsage {
    /// Get total CPU time in seconds
    pub fn total_cpu_time(&self) -> f64 {
        (self.user_time_sec + self.system_time_sec) as f64 +
        (self.user_time_usec + self.system_time_usec) as f64 / 1_000_000.0
    }
    
    /// Get user CPU time in seconds
    pub fn user_cpu_time(&self) -> f64 {
        self.user_time_sec as f64 + self.user_time_usec as f64 / 1_000_000.0
    }
    
    /// Get system CPU time in seconds
    pub fn system_cpu_time(&self) -> f64 {
        self.system_time_sec as f64 + self.system_time_usec as f64 / 1_000_000.0
    }
}
