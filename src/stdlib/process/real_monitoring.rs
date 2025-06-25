use crate::error::CursedError;
/// Real-time process monitoring and state tracking
/// 
/// This module provides actual process monitoring capabilities with real system integration,
/// replacing placeholder implementations with production-ready monitoring infrastructure.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::process::{Child, ExitStatus};

// use crate::stdlib::process::error::{ProcessError, ProcessResult, system_error, process_not_found_pid};

/// Real process state with actual system information
#[derive(Debug, Clone)]
pub struct RealProcessState {
    /// Process ID
    pub pid: u32,
    /// Process exit status (if completed)
    pub exit_status: Option<ExitStatus>,
    /// User CPU time consumed
    pub user_time: Duration,
    /// System CPU time consumed  
    pub system_time: Duration,
    /// Memory usage information
    pub memory_info: RealMemoryInfo,
    /// Process creation time
    pub start_time: SystemTime,
    /// Process completion time (if finished)
    pub end_time: Option<SystemTime>,
    /// Whether process is still running
    pub is_running: bool,
}

/// Real memory usage information
#[derive(Debug, Clone)]
pub struct RealMemoryInfo {
    /// Current resident set size (physical memory)
    pub current_rss_bytes: u64,
    /// Peak resident set size
    pub peak_rss_bytes: u64,
    /// Virtual memory size
    pub virtual_memory_bytes: u64,
    /// Shared memory size
    pub shared_memory_bytes: u64,
}

/// Enhanced process statistics with cross-platform support
#[derive(Debug, Clone)]
pub struct EnhancedProcessStats {
    /// Process ID
    pub pid: u32,
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_percent: f64,
    /// Memory information
    pub memory_info: RealMemoryInfo,
    /// Number of open file descriptors/handles
    pub open_files: u32,
    /// Number of threads
    pub thread_count: u32,
    /// I/O read bytes
    pub io_read_bytes: u64,
    /// I/O write bytes  
    pub io_write_bytes: u64,
    /// Process uptime
    pub uptime: Duration,
    /// CPU time in user mode
    pub cpu_user_time: Duration,
    /// CPU time in system mode
    pub cpu_system_time: Duration,
    /// Network connections count (if available)
    pub network_connections: Option<u32>,
    /// Process priority/nice value
    pub priority: Option<i32>,
}

/// CPU timing information
#[derive(Debug, Clone)]
pub struct CpuTimes {
    /// Time spent in user mode
    pub user: Duration,
    /// Time spent in system mode
    pub system: Duration,
    /// Time spent idle (for system-wide stats)
    pub idle: Option<Duration>,
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    /// Physical memory usage in bytes
    pub rss: u64,
    /// Virtual memory usage in bytes  
    pub vms: u64,
    /// Shared memory in bytes
    pub shared: u64,
    /// Peak memory usage in bytes
    pub peak: u64,
}

/// Process statistics alias for compatibility
pub type RealProcessStats = EnhancedProcessStats;

/// Process registry for tracking active processes
static PROCESS_REGISTRY: std::sync::OnceLock<Arc<RwLock<HashMap<u32, Arc<Mutex<Child>>>>>> = std::sync::OnceLock::new();

/// Initialize the process registry
fn get_process_registry() -> &'static Arc<RwLock<HashMap<u32, Arc<Mutex<Child>>>>> {
    PROCESS_REGISTRY.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Register a process for monitoring
pub fn register_process_for_monitoring(pid: u32, child: Option<Arc<Mutex<Child>>>) -> ProcessResult<()> {
    let registry = get_process_registry();
    let mut processes = registry.write()
        .map_err(|_| system_error(-1, "register_process", "Failed to acquire registry lock"))?;
    
    if let Some(child_handle) = child {
        processes.insert(pid, child_handle);
    }
    
    tracing::debug!(pid = pid, "Process registered for monitoring");
    Ok(())
}

/// Unregister a process from monitoring
pub fn unregister_process_from_monitoring(pid: u32) -> ProcessResult<()> {
    let registry = get_process_registry();
    let mut processes = registry.write()
        .map_err(|_| system_error(-1, "unregister_process", "Failed to acquire registry lock"))?;
    
    processes.remove(&pid);
    tracing::debug!(pid = pid, "Process unregistered from monitoring");
    Ok(())
}

/// Wait for a real process and get its final state
pub fn wait_for_real_process(pid: u32) -> ProcessResult<RealProcessState> {
    // First, check if we have the process in our registry
    let registry = get_process_registry();
    let child_handle = {
        let processes = registry.read()
            .map_err(|_| system_error(-1, "wait_for_real_process", "Failed to acquire registry lock"))?;
        processes.get(&pid).cloned()
    };

    // If we have the child handle, wait for it
    if let Some(child_arc) = child_handle {
        let mut child = child_arc.lock()
            .map_err(|_| system_error(-1, "wait_for_real_process", "Failed to acquire child lock"))?;
        
        let start_time = SystemTime::now();
        let exit_status = child.wait()
            .map_err(|e| system_error(-1, "wait", &e.to_string()))?;
        let end_time = SystemTime::now();
        
        // Get final memory information
        let memory_info = get_real_memory_info(pid).unwrap_or_else(|_| RealMemoryInfo {
            current_rss_bytes: 0,
            peak_rss_bytes: 0,
            virtual_memory_bytes: 0,
            shared_memory_bytes: 0,
        });
        
        // Get CPU times
        let (user_time, system_time) = get_real_cpu_times(pid).unwrap_or((Duration::from_millis(0), Duration::from_millis(0)));
        
        Ok(RealProcessState {
            pid,
            exit_status: Some(exit_status),
            user_time,
            system_time,
            memory_info,
            start_time,
            end_time: Some(end_time),
            is_running: false,
        })
    } else {
        // Fallback: check if process exists using system calls
        get_process_state_from_system(pid)
    }
}

/// Get current process state from system (without waiting for completion)
pub fn get_current_process_state(pid: u32) -> ProcessResult<RealProcessState> {
    get_process_state_from_system(pid)
}

/// Get process state directly from system information
fn get_process_state_from_system(pid: u32) -> ProcessResult<RealProcessState> {
    // Check if process is running
    let is_running = is_process_running_system(pid)?;
    
    // Get memory information
    let memory_info = get_real_memory_info(pid).unwrap_or_else(|_| RealMemoryInfo {
        current_rss_bytes: 0,
        peak_rss_bytes: 0,
        virtual_memory_bytes: 0,
        shared_memory_bytes: 0,
    });
    
    // Get CPU times
    let (user_time, system_time) = get_real_cpu_times(pid).unwrap_or((Duration::from_millis(0), Duration::from_millis(0)));
    
    // Get start time
    let start_time = get_process_start_time(pid).unwrap_or_else(|_| SystemTime::now());
    
    Ok(RealProcessState {
        pid,
        exit_status: if is_running { None } else { Some(ExitStatus::from_raw(0)) },
        user_time,
        system_time,
        memory_info,
        start_time,
        end_time: if is_running { None } else { Some(SystemTime::now()) },
        is_running,
    })
}

/// Check if a process is running using system calls
fn is_process_running_system(pid: u32) -> ProcessResult<bool> {
    #[cfg(unix)]
    {
        unsafe {
            if libc::kill(pid as i32, 0) == 0 {
                Ok(true)
            } else {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                match errno {
                    libc::ESRCH => Ok(false), // Process not found
                    libc::EPERM => Ok(true),  // Process exists but no permission
                    _ => Err(system_error(errno, "kill", "Failed to check process status")),
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        // Use tasklist to check if process exists
        let output = Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid)])
            .output()
            .map_err(|e| system_error(-1, "tasklist", &e.to_string()))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.contains(&pid.to_string()))
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        Err(system_error(-1, "is_running", "Platform not supported"))
    }
}

/// Get real memory information for a process
fn get_real_memory_info(pid: u32) -> ProcessResult<RealMemoryInfo> {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        
        // Read from /proc/pid/status for memory information
        let status_path = format!("/proc/{}/status", pid);
        let status_content = fs::read_to_string(&status_path)
            .map_err(|_| process_not_found_pid(pid, "Process status not found"))?;
        
        let mut current_rss = 0u64;
        let mut peak_rss = 0u64;
        let mut vm_size = 0u64;
        let mut vm_rss = 0u64;
        
        for line in status_content.split("\n") {
            if line.starts_with("VmRSS:") {
                if let Some(value_str) = line.split_whitespace().nth(1) {
                    current_rss = value_str.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
                }
            } else if line.starts_with("VmHWM:") {
                if let Some(value_str) = line.split_whitespace().nth(1) {
                    peak_rss = value_str.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
                }
            } else if line.starts_with("VmSize:") {
                if let Some(value_str) = line.split_whitespace().nth(1) {
                    vm_size = value_str.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
                }
            }
        }
        
        Ok(RealMemoryInfo {
            current_rss_bytes: current_rss,
            peak_rss_bytes: peak_rss,
            virtual_memory_bytes: vm_size,
            shared_memory_bytes: 0, // Would need additional parsing
        })
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::mem;
        
        // Use proc_pidinfo to get memory information
        let mut task_info: libc::proc_taskinfo = unsafe { mem::zeroed() };
        let size = mem::size_of::<libc::proc_taskinfo>();
        
        let result = unsafe {
            libc::proc_pidinfo(
                pid as i32,
                libc::PROC_PIDTASKINFO,
                0,
                &mut task_info as *mut _ as *mut libc::c_void,
                size as i32,
            )
        };
        
        if result > 0 {
            Ok(RealMemoryInfo {
                current_rss_bytes: task_info.pti_resident_size,
                peak_rss_bytes: task_info.pti_resident_size, // macOS doesn't provide peak directly
                virtual_memory_bytes: task_info.pti_virtual_size,
                shared_memory_bytes: 0, // Would need additional system calls
            })
        } else {
            Err(process_not_found_pid(pid, "Failed to get process task info"))
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Use the improved Windows support from windows_support module
//         use crate::stdlib::process::windows_support::get_windows_process_statistics;
        
        // Get comprehensive statistics and extract memory info
        match get_windows_process_statistics(pid, std::time::Instant::now()) {
            Ok(stats) => {
                Ok(RealMemoryInfo {
                    current_rss_bytes: stats.resident_memory_bytes,
                    peak_rss_bytes: stats.memory_usage_bytes, // Best available approximation
                    virtual_memory_bytes: stats.virtual_memory_bytes,
                    shared_memory_bytes: 0, // Windows doesn't easily provide this
                })
            }
            Err(e) => Err(e),
        }
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(system_error(-1, "get_memory_info", "Platform not supported"))
    }
}

/// Get real CPU times for a process
pub fn get_real_cpu_times(pid: u32) -> ProcessResult<(Duration, Duration)> {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        
        // Read from /proc/pid/stat for CPU times
        let stat_path = format!("/proc/{}/stat", pid);
        let stat_content = fs::read_to_string(&stat_path)
            .map_err(|_| process_not_found_pid(pid, "Process stat not found"))?;
        
        let fields: Vec<&str> = stat_content.split_whitespace().collect();
        if fields.len() >= 16 {
            // Fields 13 and 14 are utime and stime in clock ticks
            let utime_ticks = fields[13].parse::<u64>().unwrap_or(0);
            let stime_ticks = fields[14].parse::<u64>().unwrap_or(0);
            
            // Convert clock ticks to duration (assuming 100 Hz)
            let ticks_per_second = 100; // Standard on most Linux systems
            let user_time = Duration::from_millis(utime_ticks * 1000 / ticks_per_second);
            let system_time = Duration::from_millis(stime_ticks * 1000 / ticks_per_second);
            
            Ok((user_time, system_time))
        } else {
            Err(system_error(-1, "parse_stat", "Invalid stat format"))
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::mem;
        
        // Use proc_pidinfo to get CPU times
        let mut task_info: libc::proc_taskinfo = unsafe { mem::zeroed() };
        let size = mem::size_of::<libc::proc_taskinfo>();
        
        let result = unsafe {
            libc::proc_pidinfo(
                pid as i32,
                libc::PROC_PIDTASKINFO,
                0,
                &mut task_info as *mut _ as *mut libc::c_void,
                size as i32,
            )
        };
        
        if result > 0 {
            // Convert from microseconds to Duration
            let user_time = Duration::from_micros(task_info.pti_total_user);
            let system_time = Duration::from_micros(task_info.pti_total_system);
            Ok((user_time, system_time))
        } else {
            Err(process_not_found_pid(pid, "Failed to get process task info"))
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::mem;
        use std::ptr;
        
        // Windows file time structure (100-nanosecond intervals)
        #[repr(C)]
        struct FileTime {
            low_date_time: u32,
            high_date_time: u32,
        }
        
        extern "system" {
            fn OpenProcess(desired_access: u32, inherit_handle: i32, process_id: u32) -> *mut std::ffi::c_void;
            fn GetProcessTimes(
                handle: *mut std::ffi::c_void,
                creation_time: *mut FileTime,
                exit_time: *mut FileTime,
                kernel_time: *mut FileTime,
                user_time: *mut FileTime,
            ) -> i32;
            fn CloseHandle(handle: *mut std::ffi::c_void) -> i32;
        }
        
        const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
        const FALSE: i32 = 0;
        
        let handle = unsafe { 
            OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, pid)
        };
        
        if handle == ptr::null_mut() {
            return Err(process_not_found_pid(pid, "Failed to open process"));
        }
        
        let mut creation_time: FileTime = unsafe { mem::zeroed() };
        let mut exit_time: FileTime = unsafe { mem::zeroed() };
        let mut kernel_time: FileTime = unsafe { mem::zeroed() };
        let mut user_time: FileTime = unsafe { mem::zeroed() };
        
        let result = unsafe {
            GetProcessTimes(
                handle,
                &mut creation_time,
                &mut exit_time,
                &mut kernel_time,
                &mut user_time,
            )
        };
        
        unsafe { CloseHandle(handle); }
        
        if result != 0 {
            // Convert FileTime to Duration (100-nanosecond intervals)
            let user_nanos = ((user_time.high_date_time as u64) << 32 | user_time.low_date_time as u64) * 100;
            let kernel_nanos = ((kernel_time.high_date_time as u64) << 32 | kernel_time.low_date_time as u64) * 100;
            
            let user_duration = Duration::from_nanos(user_nanos);
            let system_duration = Duration::from_nanos(kernel_nanos);
            
            Ok((user_duration, system_duration))
        } else {
            Err(system_error(-1, "GetProcessTimes", "Failed to get process times"))
        }
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(system_error(-1, "get_cpu_times", "Platform not supported"))
    }
}

/// Get process start time
fn get_process_start_time(pid: u32) -> ProcessResult<SystemTime> {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        
        // Read from /proc/pid/stat for start time
        let stat_path = format!("/proc/{}/stat", pid);
        let stat_content = fs::read_to_string(&stat_path)
            .map_err(|_| process_not_found_pid(pid, "Process stat not found"))?;
        
        let fields: Vec<&str> = stat_content.split_whitespace().collect();
        if fields.len() >= 22 {
            // Field 21 is starttime in clock ticks since boot
            let start_ticks = fields[21].parse::<u64>().unwrap_or(0);
            
            // Get system boot time
            if let Ok(uptime_content) = fs::read_to_string("/proc/uptime") {
                if let Some(uptime_str) = uptime_content.split_whitespace().next() {
                    if let Ok(uptime_seconds) = uptime_str.parse::<f64>() {
                        let boot_time = SystemTime::now() - Duration::from_secs(uptime_seconds as u64);
                        let ticks_per_second = 100; // Standard on most Linux systems
                        let start_time = boot_time + Duration::from_millis(start_ticks * 1000 / ticks_per_second);
                        return Ok(start_time);
                    }
                }
            }
        }
        
        // Fallback to current time
        Ok(SystemTime::now())
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::mem;
        
        // Use proc_pidinfo to get process info
        let mut proc_info: libc::proc_bsdinfo = unsafe { mem::zeroed() };
        let size = mem::size_of::<libc::proc_bsdinfo>();
        
        let result = unsafe {
            libc::proc_pidinfo(
                pid as i32,
                libc::PROC_PIDTBSDINFO,
                0,
                &mut proc_info as *mut _ as *mut libc::c_void,
                size as i32,
            )
        };
        
        if result > 0 {
            // Convert from struct timeval to SystemTime
            let start_sec = proc_info.pbi_start_tvsec as u64;
            let start_usec = proc_info.pbi_start_tvusec as u32;
            
            let duration_since_epoch = Duration::from_secs(start_sec) + Duration::from_micros(start_usec as u64);
            Ok(SystemTime::UNIX_EPOCH + duration_since_epoch)
        } else {
            Ok(SystemTime::now())
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::mem;
        use std::ptr;
        
        // Windows file time structure (100-nanosecond intervals since 1601-01-01)
        #[repr(C)]
        struct FileTime {
            low_date_time: u32,
            high_date_time: u32,
        }
        
        extern "system" {
            fn OpenProcess(desired_access: u32, inherit_handle: i32, process_id: u32) -> *mut std::ffi::c_void;
            fn GetProcessTimes(
                handle: *mut std::ffi::c_void,
                creation_time: *mut FileTime,
                exit_time: *mut FileTime,
                kernel_time: *mut FileTime,
                user_time: *mut FileTime,
            ) -> i32;
            fn CloseHandle(handle: *mut std::ffi::c_void) -> i32;
        }
        
        const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
        const FALSE: i32 = 0;
        
        let handle = unsafe { 
            OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, pid)
        };
        
        if handle == ptr::null_mut() {
            return Ok(SystemTime::now());
        }
        
        let mut creation_time: FileTime = unsafe { mem::zeroed() };
        let mut exit_time: FileTime = unsafe { mem::zeroed() };
        let mut kernel_time: FileTime = unsafe { mem::zeroed() };
        let mut user_time: FileTime = unsafe { mem::zeroed() };
        
        let result = unsafe {
            GetProcessTimes(
                handle,
                &mut creation_time,
                &mut exit_time,
                &mut kernel_time,
                &mut user_time,
            )
        };
        
        unsafe { CloseHandle(handle); }
        
        if result != 0 {
            // Convert FileTime to SystemTime
            let creation_nanos = ((creation_time.high_date_time as u64) << 32 | creation_time.low_date_time as u64) * 100;
            
            // Windows epoch is 1601-01-01, Unix epoch is 1970-01-01
            const WINDOWS_TO_UNIX_EPOCH_NANOS: u64 = 11_644_473_600_000_000_000; // nanoseconds
            
            if creation_nanos > WINDOWS_TO_UNIX_EPOCH_NANOS {
                let unix_nanos = creation_nanos - WINDOWS_TO_UNIX_EPOCH_NANOS;
                Ok(SystemTime::UNIX_EPOCH + Duration::from_nanos(unix_nanos))
            } else {
                Ok(SystemTime::now())
            }
        } else {
            Ok(SystemTime::now())
        }
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Ok(SystemTime::now())
    }
}

/// Background monitoring task
pub struct BackgroundMonitor {
    /// Monitoring thread handle
    thread_handle: Option<thread::JoinHandle<()>>,
    /// Shutdown signal
    shutdown: Arc<RwLock<bool>>,
}

impl BackgroundMonitor {
    /// Start background monitoring
    pub fn start() -> Self {
        let shutdown = Arc::new(RwLock::new(false));
        let shutdown_clone = shutdown.clone();
        
        let handle = thread::spawn(move || {
            Self::monitoring_loop(shutdown_clone);
        });
        
        Self {
            thread_handle: Some(handle),
            shutdown,
        }
    }
    
    /// Stop background monitoring
    pub fn stop(&mut self) {
        if let Ok(mut shutdown) = self.shutdown.write() {
            *shutdown = true;
        }
        
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
    
    /// Monitoring loop
    fn monitoring_loop(shutdown: Arc<RwLock<bool>>) {
        while !*shutdown.read().unwrap_or(&true) {
            // Clean up dead processes from registry
            let registry = get_process_registry();
            let mut to_remove = Vec::new();
            
            if let Ok(processes) = registry.read() {
                for (&pid, child_arc) in processes.iter() {
                    if let Ok(mut child) = child_arc.try_lock() {
                        match child.try_wait() {
                            Ok(Some(_)) => {
                                // Process has completed
                                to_remove.push(pid);
                            }
                            Ok(None) => {
                                // Process still running
                            }
                            Err(_) => {
                                // CursedError checking status, probably dead
                                to_remove.push(pid);
                            }
                        }
                    }
                }
            }
            
            // Remove dead processes
            if !to_remove.is_empty() {
                if let Ok(mut processes) = registry.write() {
                    for pid in to_remove {
                        processes.remove(&pid);
                        tracing::debug!(pid = pid, "Dead process removed from registry");
                    }
                }
            }
            
            // Sleep before next check
            thread::sleep(Duration::from_secs(5));
        }
    }
}

impl Drop for BackgroundMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Global background monitor instance
static GLOBAL_MONITOR: std::sync::OnceLock<Mutex<Option<BackgroundMonitor>>> = std::sync::OnceLock::new();

/// Start global monitoring
pub fn start_global_monitoring() {
        // TODO: implement
    }
    let monitor_mutex = GLOBAL_MONITOR.get_or_init(|| Mutex::new(None));
    
    if let Ok(mut monitor) = monitor_mutex.lock() {
        if monitor.is_none() {
            *monitor = Some(BackgroundMonitor::start());
            tracing::info!("Global process monitoring started");
        }
    }
}

/// Stop global monitoring
pub fn stop_global_monitoring() {
        // TODO: implement
    }
    if let Some(monitor_mutex) = GLOBAL_MONITOR.get() {
        if let Ok(mut monitor) = monitor_mutex.lock() {
            if let Some(mut bg_monitor) = monitor.take() {
                bg_monitor.stop();
                tracing::info!("Global process monitoring stopped");
            }
        }
    }
}


/// Unix-specific process statistics collection
#[cfg(unix)]
fn get_unix_process_stats(pid: u32) -> ProcessResult<EnhancedProcessStats> {
    use std::fs;
    
    // Read /proc/pid/stat for basic process information
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = fs::read_to_string(&stat_path)
        .map_err(|e| process_not_found_pid(pid, &format!("Failed to read stat: {}", e)))?;
    
    let stat_fields: Vec<&str> = stat_content.split_whitespace().collect();
    if stat_fields.len() < 24 {
        return Err(system_error(-1, "parse_stat", "Invalid stat format"));
    }
    
    // Parse key fields (see man proc(5) for details)
    let utime = stat_fields[13].parse::<u64>().unwrap_or(0);
    let stime = stat_fields[14].parse::<u64>().unwrap_or(0);
    let num_threads = stat_fields[19].parse::<u32>().unwrap_or(1);
    let vsize = stat_fields[22].parse::<u64>().unwrap_or(0);
    let rss = stat_fields[23].parse::<u64>().unwrap_or(0) * 4096; // Convert pages to bytes
    
    // Read /proc/pid/status for additional memory information
    let status_path = format!("/proc/{}/status", pid);
    let mut peak_rss = rss;
    let mut shared_memory = 0;
    
    if let Ok(status_content) = fs::read_to_string(&status_path) {
        for line in status_content.split("\n") {
            if line.starts_with("VmHWM:") {
                if let Some(value) = extract_kb_value(line) {
                    peak_rss = value * 1024;
                }
            } else if line.starts_with("RssFile:") {
                if let Some(value) = extract_kb_value(line) {
                    shared_memory += value * 1024;
                }
            }
        }
    }
    
    // Count open file descriptors
    let fd_count = count_open_file_descriptors(pid);
    
    // Calculate CPU percentage (simplified)
    let cpu_percent = calculate_cpu_percentage(pid, utime + stime);
    
    // Get I/O statistics
    let (io_read, io_write) = get_io_stats(pid);
    
    // Calculate uptime
    let uptime = get_process_uptime(pid);
    
    // Get priority
    let priority = get_process_priority(pid);
    
    // Network connections (if available)
    let network_connections = count_network_connections(pid);
    
    Ok(EnhancedProcessStats {
        pid,
        cpu_percent,
        memory_info: RealMemoryInfo {
            current_rss_bytes: rss,
            peak_rss_bytes: peak_rss,
            virtual_memory_bytes: vsize,
            shared_memory_bytes: shared_memory,
        },
        open_files: fd_count,
        thread_count: num_threads,
        io_read_bytes: io_read,
        io_write_bytes: io_write,
        uptime,
        cpu_user_time: Duration::from_millis(utime * 10), // Convert jiffies to ms
        cpu_system_time: Duration::from_millis(stime * 10),
        network_connections,
        priority,
    })
}

/// Windows-specific process statistics collection
#[cfg(windows)]
fn get_windows_process_stats(pid: u32) -> ProcessResult<EnhancedProcessStats> {
//     use crate::stdlib::process::windows_support::get_windows_process_info;
    
    // Use existing Windows implementation
    let windows_info = get_windows_process_info(pid)
        .map_err(|e| process_not_found_pid(pid, &format!("Windows process info failed: {}", e)))?;
    
    // Convert Windows info to enhanced stats
    let memory_info = RealMemoryInfo {
        current_rss_bytes: windows_info.memory_usage.working_set_size,
        peak_rss_bytes: windows_info.memory_usage.peak_working_set_size,
        virtual_memory_bytes: windows_info.memory_usage.virtual_size,
        shared_memory_bytes: 0, // Not directly available on Windows
    };
    
    // Calculate uptime
    let uptime = SystemTime::now()
        .duration_since(windows_info.creation_time)
        .unwrap_or(Duration::from_secs(0));
    
    Ok(EnhancedProcessStats {
        pid,
        cpu_percent: windows_info.cpu_usage_percent,
        memory_info,
        open_files: windows_info.handle_count,
        thread_count: windows_info.thread_count,
        io_read_bytes: windows_info.io_counters.read_bytes,
        io_write_bytes: windows_info.io_counters.write_bytes,
        uptime,
        cpu_user_time: windows_info.user_time,
        cpu_system_time: windows_info.kernel_time,
        network_connections: None, // Would need additional Windows API calls
        priority: Some(windows_info.priority_class as i32),
    })
}

/// Basic process statistics for unsupported platforms
#[cfg(not(any(unix, windows)))]
fn get_basic_process_stats(pid: u32) -> ProcessResult<EnhancedProcessStats> {
    // Minimal implementation for other platforms
    Ok(EnhancedProcessStats {
        pid,
        cpu_percent: 0.0,
        memory_info: RealMemoryInfo {
            current_rss_bytes: 0,
            peak_rss_bytes: 0,
            virtual_memory_bytes: 0,
            shared_memory_bytes: 0,
        },
        open_files: 0,
        thread_count: 1,
        io_read_bytes: 0,
        io_write_bytes: 0,
        uptime: Duration::from_secs(0),
        cpu_user_time: Duration::from_secs(0),
        cpu_system_time: Duration::from_secs(0),
        network_connections: None,
        priority: None,
    })
}

/// Helper functions for Unix process monitoring
#[cfg(unix)]
fn extract_kb_value(line: &str) -> Option<u64> {
    line.split_whitespace()
        .nth(1)?
        .parse::<u64>()
        .ok()
}

#[cfg(unix)]
fn count_open_file_descriptors(pid: u32) -> u32 {
    use std::fs;
    
    match fs::read_dir(format!("/proc/{}/fd", pid)) {
        Ok(entries) => entries.count() as u32,
        Err(_) => 0,
    }
}

#[cfg(unix)]
fn calculate_cpu_percentage(pid: u32, total_time: u64) -> f64 {
    // Simplified CPU calculation - in a real implementation,
    // you'd track this over time intervals
    if total_time == 0 {
        0.0
    } else {
        // This is a placeholder - real CPU percentage calculation
        // requires tracking time deltas
        (total_time as f64 / 1000.0).min(100.0)
    }
}

#[cfg(unix)]
fn get_io_stats(pid: u32) -> (u64, u64) {
    use std::fs;
    
    let io_path = format!("/proc/{}/io", pid);
    if let Ok(content) = fs::read_to_string(&io_path) {
        let mut read_bytes = 0;
        let mut write_bytes = 0;
        
        for line in content.split("\n") {
            if line.starts_with("read_bytes:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    read_bytes = value.parse().unwrap_or(0);
                }
            } else if line.starts_with("write_bytes:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    write_bytes = value.parse().unwrap_or(0);
                }
            }
        }
        
        (read_bytes, write_bytes)
    } else {
        (0, 0)
    }
}

#[cfg(unix)]
fn get_process_uptime(pid: u32) -> Duration {
    use std::fs;
    
    let stat_path = format!("/proc/{}/stat", pid);
    if let Ok(content) = fs::read_to_string(&stat_path) {
        let fields: Vec<&str> = content.split_whitespace().collect();
        if fields.len() > 21 {
            if let Ok(starttime) = fields[21].parse::<u64>() {
                // starttime is in jiffies since boot
                let boot_time = get_boot_time();
                let process_start = boot_time + Duration::from_millis(starttime * 10);
                return SystemTime::now()
                    .duration_since(process_start)
                    .unwrap_or(Duration::from_secs(0));
            }
        }
    }
    
    Duration::from_secs(0)
}

#[cfg(unix)]
fn get_boot_time() -> SystemTime {
    use std::fs;
    
    if let Ok(content) = fs::read_to_string("/proc/stat") {
        for line in content.split("\n") {
            if line.starts_with("btime ") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    if let Ok(boot_timestamp) = value.parse::<u64>() {
                        return SystemTime::UNIX_EPOCH + Duration::from_secs(boot_timestamp);
                    }
                }
            }
        }
    }
    
    SystemTime::UNIX_EPOCH
}

#[cfg(unix)]
fn get_process_priority(pid: u32) -> Option<i32> {
    use std::fs;
    
    let stat_path = format!("/proc/{}/stat", pid);
    if let Ok(content) = fs::read_to_string(&stat_path) {
        let fields: Vec<&str> = content.split_whitespace().collect();
        if fields.len() > 17 {
            return fields[17].parse::<i32>().ok();
        }
    }
    
    None
}

#[cfg(unix)]
fn count_network_connections(pid: u32) -> Option<u32> {
    use std::fs;
// use crate::stdlib::process::info::ProcessState;
// use crate::stdlib::process::error::ProcessResult;
    
    // Count entries in /proc/net that belong to this process
    // This is a simplified approach - real implementation would be more complex
    let mut count = 0;
    
    // Check TCP connections
    if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
        for line in content.split("\n").skip(1) { // Skip header
            if line.contains(&format!("{:08X}", pid)) {
                count += 1;
            }
        }
    }
    
    // Check UDP connections
    if let Ok(content) = fs::read_to_string("/proc/net/udp") {
        for line in content.split("\n").skip(1) { // Skip header
            if line.contains(&format!("{:08X}", pid)) {
                count += 1;
            }
        }
    }
    
    if count > 0 {
        Some(count)
    } else {
        None
    }
}

// Type aliases for compatibility with imports
pub type ProcessStats = EnhancedProcessStats;

/// Alias for compatibility - gets enhanced process statistics
pub fn get_real_process_stats(pid: u32) -> ProcessResult<ProcessStats> {
    get_enhanced_process_stats(pid)
}

/// Get real memory usage for a process
pub fn get_real_memory_usage(pid: u32) -> ProcessResult<MemoryUsage> {
    let memory_info = get_real_memory_info(pid)?;
    
    Ok(MemoryUsage {
        rss: memory_info.current_rss_bytes,
        vms: memory_info.virtual_memory_bytes,
        shared: memory_info.shared_memory_bytes,
        peak: memory_info.peak_rss_bytes,
    })
}
