/// Process information and system utilities
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use super::error::{ProcessError, ProcessResult};

/// Process status enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    /// Process is running
    Running,
    /// Process is sleeping/waiting
    Sleeping,
    /// Process is stopped
    Stopped,
    /// Process is a zombie (finished but not cleaned up)
    Zombie,
    /// Process status is unknown
    Unknown,
}

/// Process information structure
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Parent process ID
    pub parent_pid: Option<u32>,
    /// Process name
    pub name: String,
    /// Command line arguments
    pub command_line: Vec<String>,
    /// Executable path
    pub executable: Option<PathBuf>,
    /// Current working directory
    pub working_directory: Option<PathBuf>,
    /// Process status
    pub status: ProcessStatus,
    /// Start time
    pub start_time: Option<SystemTime>,
    /// CPU time used
    pub cpu_time: Option<Duration>,
    /// Memory usage in bytes
    pub memory_usage: Option<u64>,
    /// Virtual memory size in bytes
    pub virtual_memory: Option<u64>,
    /// User ID (Unix only)
    pub uid: Option<u32>,
    /// Group ID (Unix only)
    pub gid: Option<u32>,
    /// Environment variables
    pub environment: Option<HashMap<String, String>>,
    /// Number of threads
    pub thread_count: Option<u32>,
    /// Process priority
    pub priority: Option<i32>,
}

/// Memory usage information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    /// Resident set size (physical memory)
    pub rss: u64,
    /// Virtual memory size
    pub vms: u64,
    /// Shared memory
    pub shared: u64,
    /// Text (code) memory
    pub text: u64,
    /// Data memory
    pub data: u64,
    /// Stack memory
    pub stack: u64,
}

/// CPU usage information
#[derive(Debug, Clone)]
pub struct CpuInfo {
    /// Total CPU time
    pub total_time: Duration,
    /// User CPU time
    pub user_time: Duration,
    /// System CPU time
    pub system_time: Duration,
    /// CPU usage percentage
    pub cpu_percent: Option<f64>,
}

/// System process list entry
#[derive(Debug, Clone)]
pub struct ProcessListEntry {
    /// Process ID
    pub pid: u32,
    /// Parent process ID
    pub parent_pid: Option<u32>,
    /// Process name
    pub name: String,
    /// Process status
    pub status: ProcessStatus,
    /// Memory usage (RSS) in bytes
    pub memory_rss: Option<u64>,
    /// CPU usage percentage
    pub cpu_percent: Option<f64>,
}

impl ProcessInfo {
    /// Create ProcessInfo from process ID
    pub fn from_pid(pid: u32) -> ProcessResult<Self> {
        // Platform-specific implementation
        #[cfg(target_os = "linux")]
        return Self::from_pid_linux(pid);
        
        #[cfg(target_os = "macos")]
        return Self::from_pid_macos(pid);
        
        #[cfg(target_os = "windows")]
        return Self::from_pid_windows(pid);
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        Err(ProcessError::PlatformError("Unsupported platform".to_string()))
    }

    /// Get current process information
    pub fn current() -> ProcessResult<Self> {
        Self::from_pid(std::process::id())
    }

    /// Get parent process information
    pub fn parent(&self) -> ProcessResult<Option<Self>> {
        match self.parent_pid {
            Some(ppid) => Ok(Some(Self::from_pid(ppid)?)),
            None => Ok(None),
        }
    }

    /// Get child processes
    pub fn children(&self) -> ProcessResult<Vec<Self>> {
        let all_processes = get_process_list()?;
        let children: Result<Vec<_>, _> = all_processes
            .into_iter()
            .filter(|p| p.parent_pid == Some(self.pid))
            .map(|p| Self::from_pid(p.pid))
            .collect();
        children
    }

    /// Check if process is still running
    pub fn is_running(&self) -> bool {
        match Self::from_pid(self.pid) {
            Ok(info) => info.status == ProcessStatus::Running,
            Err(_) => false,
        }
    }

    /// Get detailed memory information
    pub fn memory_info(&self) -> ProcessResult<MemoryInfo> {
        #[cfg(target_os = "linux")]
        return self.memory_info_linux();
        
        #[cfg(target_os = "macos")]
        return self.memory_info_macos();
        
        #[cfg(target_os = "windows")]
        return self.memory_info_windows();
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        Err(ProcessError::PlatformError("Unsupported platform".to_string()))
    }

    /// Get CPU usage information
    pub fn cpu_info(&self) -> ProcessResult<CpuInfo> {
        #[cfg(target_os = "linux")]
        return self.cpu_info_linux();
        
        #[cfg(target_os = "macos")]
        return self.cpu_info_macos();
        
        #[cfg(target_os = "windows")]
        return self.cpu_info_windows();
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        Err(ProcessError::PlatformError("Unsupported platform".to_string()))
    }

    // Platform-specific implementations (simplified for now)
    
    #[cfg(target_os = "linux")]
    fn from_pid_linux(pid: u32) -> ProcessResult<Self> {
        use std::fs;
        
        // Read from /proc filesystem
        let proc_path = format!("/proc/{}", pid);
        if !std::path::Path::new(&proc_path).exists() {
            return Err(ProcessError::ProcessNotFound(pid));
        }
        
        // Read basic info from /proc/[pid]/stat
        let stat_path = format!("/proc/{}/stat", pid);
        let stat_content = fs::read_to_string(&stat_path)
            .map_err(|_| ProcessError::ProcessNotFound(pid))?;
        
        let fields: Vec<&str> = stat_content.split_whitespace().collect();
        if fields.len() < 4 {
            return Err(ProcessError::InvalidState("Invalid stat format".to_string()));
        }
        
        let name = fields[1].trim_matches('(').trim_matches(')').to_string();
        let parent_pid = fields[3].parse::<u32>().ok();
        
        // Read command line
        let cmdline_path = format!("/proc/{}/cmdline", pid);
        let cmdline = fs::read_to_string(&cmdline_path).unwrap_or_default();
        let command_line: Vec<String> = cmdline
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        // Read executable path
        let exe_path = format!("/proc/{}/exe", pid);
        let executable = fs::read_link(&exe_path).ok();
        
        // Read working directory
        let cwd_path = format!("/proc/{}/cwd", pid);
        let working_directory = fs::read_link(&cwd_path).ok();
        
        Ok(ProcessInfo {
            pid,
            parent_pid,
            name,
            command_line,
            executable,
            working_directory,
            status: ProcessStatus::Running, // Simplified
            start_time: None,
            cpu_time: None,
            memory_usage: None,
            virtual_memory: None,
            uid: None,
            gid: None,
            environment: None,
            thread_count: None,
            priority: None,
        })
    }
    
    #[cfg(target_os = "macos")]
    fn from_pid_macos(pid: u32) -> ProcessResult<Self> {
        // macOS implementation would use sysctl and libproc
        // This is a simplified stub
        Ok(ProcessInfo {
            pid,
            parent_pid: None,
            name: format!("process_{}", pid),
            command_line: vec![],
            executable: None,
            working_directory: None,
            status: ProcessStatus::Unknown,
            start_time: None,
            cpu_time: None,
            memory_usage: None,
            virtual_memory: None,
            uid: None,
            gid: None,
            environment: None,
            thread_count: None,
            priority: None,
        })
    }
    
    #[cfg(target_os = "windows")]
    fn from_pid_windows(pid: u32) -> ProcessResult<Self> {
        // Windows implementation would use Windows API
        // This is a simplified stub
        Ok(ProcessInfo {
            pid,
            parent_pid: None,
            name: format!("process_{}.exe", pid),
            command_line: vec![],
            executable: None,
            working_directory: None,
            status: ProcessStatus::Unknown,
            start_time: None,
            cpu_time: None,
            memory_usage: None,
            virtual_memory: None,
            uid: None,
            gid: None,
            environment: None,
            thread_count: None,
            priority: None,
        })
    }
    
    #[cfg(target_os = "linux")]
    fn memory_info_linux(&self) -> ProcessResult<MemoryInfo> {
        use std::fs;
        
        let status_path = format!("/proc/{}/status", self.pid);
        let status_content = fs::read_to_string(&status_path)
            .map_err(|_| ProcessError::ProcessNotFound(self.pid))?;
        
        let mut rss = 0;
        let mut vms = 0;
        
        for line in status_content.lines() {
            if line.starts_with("VmRSS:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    rss = value.parse::<u64>().unwrap_or(0) * 1024; // Convert kB to bytes
                }
            } else if line.starts_with("VmSize:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    vms = value.parse::<u64>().unwrap_or(0) * 1024; // Convert kB to bytes
                }
            }
        }
        
        Ok(MemoryInfo {
            rss,
            vms,
            shared: 0,
            text: 0,
            data: 0,
            stack: 0,
        })
    }
    
    #[cfg(target_os = "macos")]
    fn memory_info_macos(&self) -> ProcessResult<MemoryInfo> {
        // macOS implementation stub
        Ok(MemoryInfo {
            rss: 0,
            vms: 0,
            shared: 0,
            text: 0,
            data: 0,
            stack: 0,
        })
    }
    
    #[cfg(target_os = "windows")]
    fn memory_info_windows(&self) -> ProcessResult<MemoryInfo> {
        // Windows implementation stub
        Ok(MemoryInfo {
            rss: 0,
            vms: 0,
            shared: 0,
            text: 0,
            data: 0,
            stack: 0,
        })
    }
    
    #[cfg(target_os = "linux")]
    fn cpu_info_linux(&self) -> ProcessResult<CpuInfo> {
        use std::fs;
        
        let stat_path = format!("/proc/{}/stat", self.pid);
        let stat_content = fs::read_to_string(&stat_path)
            .map_err(|_| ProcessError::ProcessNotFound(self.pid))?;
        
        let fields: Vec<&str> = stat_content.split_whitespace().collect();
        if fields.len() < 17 {
            return Err(ProcessError::InvalidState("Invalid stat format".to_string()));
        }
        
        // CPU times are in clock ticks
        let user_time_ticks = fields[13].parse::<u64>().unwrap_or(0);
        let system_time_ticks = fields[14].parse::<u64>().unwrap_or(0);
        
        // Convert ticks to duration (assuming 100 ticks per second)
        let ticks_per_second = 100;
        let user_time = Duration::from_nanos((user_time_ticks * 1_000_000_000) / ticks_per_second);
        let system_time = Duration::from_nanos((system_time_ticks * 1_000_000_000) / ticks_per_second);
        let total_time = user_time + system_time;
        
        Ok(CpuInfo {
            total_time,
            user_time,
            system_time,
            cpu_percent: None,
        })
    }
    
    #[cfg(target_os = "macos")]
    fn cpu_info_macos(&self) -> ProcessResult<CpuInfo> {
        // macOS implementation stub
        Ok(CpuInfo {
            total_time: Duration::from_secs(0),
            user_time: Duration::from_secs(0),
            system_time: Duration::from_secs(0),
            cpu_percent: None,
        })
    }
    
    #[cfg(target_os = "windows")]
    fn cpu_info_windows(&self) -> ProcessResult<CpuInfo> {
        // Windows implementation stub
        Ok(CpuInfo {
            total_time: Duration::from_secs(0),
            user_time: Duration::from_secs(0),
            system_time: Duration::from_secs(0),
            cpu_percent: None,
        })
    }
}

/// Get current process ID
pub fn get_current_pid() -> u32 {
    std::process::id()
}

/// Get parent process ID
pub fn get_parent_pid() -> ProcessResult<Option<u32>> {
    let current = ProcessInfo::current()?;
    Ok(current.parent_pid)
}

/// Check if a process with given PID exists
pub fn is_process_running(pid: u32) -> bool {
    ProcessInfo::from_pid(pid).is_ok()
}

/// Get process information by PID
pub fn get_process_info(pid: u32) -> ProcessResult<ProcessInfo> {
    ProcessInfo::from_pid(pid)
}

/// Get memory usage of a process
pub fn get_process_memory(pid: u32) -> ProcessResult<MemoryInfo> {
    let info = ProcessInfo::from_pid(pid)?;
    info.memory_info()
}

/// Get CPU usage of a process
pub fn get_process_cpu(pid: u32) -> ProcessResult<CpuInfo> {
    let info = ProcessInfo::from_pid(pid)?;
    info.cpu_info()
}

/// Get list of all running processes
pub fn get_process_list() -> ProcessResult<Vec<ProcessListEntry>> {
    #[cfg(target_os = "linux")]
    return get_process_list_linux();
    
    #[cfg(target_os = "macos")]
    return get_process_list_macos();
    
    #[cfg(target_os = "windows")]
    return get_process_list_windows();
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    Err(ProcessError::PlatformError("Unsupported platform".to_string()))
}

#[cfg(target_os = "linux")]
fn get_process_list_linux() -> ProcessResult<Vec<ProcessListEntry>> {
    use std::fs;
    
    let proc_dir = std::path::Path::new("/proc");
    let mut processes = Vec::new();
    
    if let Ok(entries) = fs::read_dir(proc_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();
            
            // Check if it's a numeric directory (PID)
            if let Ok(pid) = name_str.parse::<u32>() {
                if let Ok(info) = ProcessInfo::from_pid(pid) {
                    processes.push(ProcessListEntry {
                        pid,
                        parent_pid: info.parent_pid,
                        name: info.name,
                        status: info.status,
                        memory_rss: info.memory_usage,
                        cpu_percent: None,
                    });
                }
            }
        }
    }
    
    Ok(processes)
}

#[cfg(target_os = "macos")]
fn get_process_list_macos() -> ProcessResult<Vec<ProcessListEntry>> {
    // macOS implementation stub
    Ok(Vec::new())
}

#[cfg(target_os = "windows")]
fn get_process_list_windows() -> ProcessResult<Vec<ProcessListEntry>> {
    // Windows implementation stub
    Ok(Vec::new())
}

/// Find processes by name
pub fn find_processes_by_name<S: AsRef<str>>(name: S) -> ProcessResult<Vec<ProcessListEntry>> {
    let name_str = name.as_ref();
    let all_processes = get_process_list()?;
    
    Ok(all_processes
        .into_iter()
        .filter(|p| p.name.contains(name_str))
        .collect())
}

/// Get process tree starting from a root PID
pub fn get_process_tree(root_pid: u32) -> ProcessResult<Vec<ProcessInfo>> {
    let mut tree = Vec::new();
    let mut to_visit = vec![root_pid];
    
    while let Some(pid) = to_visit.pop() {
        if let Ok(info) = ProcessInfo::from_pid(pid) {
            // Get children of this process
            if let Ok(children) = info.children() {
                for child in &children {
                    to_visit.push(child.pid);
                }
            }
            tree.push(info);
        }
    }
    
    Ok(tree)
}

/// Get system load average (Unix only)
#[cfg(unix)]
pub fn get_load_average() -> ProcessResult<(f64, f64, f64)> {
    use std::fs;
    
    let loadavg_content = fs::read_to_string("/proc/loadavg")
        .map_err(|e| ProcessError::SystemError(
            e.raw_os_error().unwrap_or(-1),
            "Cannot read load average".to_string()
        ))?;
    
    let parts: Vec<&str> = loadavg_content.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(ProcessError::InvalidState("Invalid loadavg format".to_string()));
    }
    
    let load1 = parts[0].parse::<f64>()
        .map_err(|_| ProcessError::InvalidState("Invalid load1 value".to_string()))?;
    let load5 = parts[1].parse::<f64>()
        .map_err(|_| ProcessError::InvalidState("Invalid load5 value".to_string()))?;
    let load15 = parts[2].parse::<f64>()
        .map_err(|_| ProcessError::InvalidState("Invalid load15 value".to_string()))?;
    
    Ok((load1, load5, load15))
}

/// Get number of CPU cores
pub fn get_cpu_count() -> usize {
    num_cpus::get()
}

/// Get system uptime
#[cfg(unix)]
pub fn get_system_uptime() -> ProcessResult<Duration> {
    use std::fs;
    
    let uptime_content = fs::read_to_string("/proc/uptime")
        .map_err(|e| ProcessError::SystemError(
            e.raw_os_error().unwrap_or(-1),
            "Cannot read uptime".to_string()
        ))?;
    
    let uptime_seconds = uptime_content
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .ok_or_else(|| ProcessError::InvalidState("Invalid uptime format".to_string()))?;
    
    Ok(Duration::from_secs_f64(uptime_seconds))
}

// Add num_cpus dependency (this would need to be added to Cargo.toml)
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
}
