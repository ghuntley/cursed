/// Process information and system utilities
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use super::error::{ProcessError, ProcessResult};

/// Windows handle guard for automatic cleanup
#[cfg(target_os = "windows")]
pub struct HandleGuard(pub winapi::um::winnt::HANDLE);

#[cfg(target_os = "windows")]
impl Drop for HandleGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { winapi::um::handleapi::CloseHandle(self.0) };
        }
    }
}

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
        if fields.len() < 22 {
            return Err(ProcessError::InvalidState("Invalid stat format".to_string()));
        }
        
        let name = fields[1].trim_matches('(').trim_matches(')').to_string();
        let status_char = fields[2].chars().next().unwrap_or('?');
        let parent_pid = fields[3].parse::<u32>().ok();
        
        // Parse process status
        let status = match status_char {
            'R' => ProcessStatus::Running,
            'S' | 'D' | 'I' => ProcessStatus::Sleeping,
            'T' => ProcessStatus::Stopped,
            'Z' => ProcessStatus::Zombie,
            _ => ProcessStatus::Unknown,
        };
        
        // Parse start time (in clock ticks since boot)
        let start_time_ticks: u64 = fields[21].parse().unwrap_or(0);
        let start_time = calculate_start_time(start_time_ticks);
        
        // Parse thread count  
        let thread_count: u32 = fields[19].parse().unwrap_or(1);
        
        // Parse nice value (priority)
        let priority: i32 = fields[18].parse().unwrap_or(0);
        
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
        
        // Try to get UID/GID from status file
        let (uid, gid) = Self::get_uid_gid_linux(pid);
        
        // Get memory usage from stat
        let virtual_memory: u64 = fields[22].parse().unwrap_or(0); // vsize in bytes
        
        Ok(ProcessInfo {
            pid,
            parent_pid,
            name,
            command_line,
            executable,
            working_directory,
            status,
            start_time,
            cpu_time: None, // Will be calculated separately if needed
            memory_usage: None, // Will be read from status if needed
            virtual_memory: Some(virtual_memory),
            uid,
            gid,
            environment: None, // Can be read from environ if needed
            thread_count: Some(thread_count),
            priority: Some(priority),
        })
    }
    
    #[cfg(target_os = "linux")]
    fn get_uid_gid_linux(pid: u32) -> (Option<u32>, Option<u32>) {
        use std::fs;
        
        let status_path = format!("/proc/{}/status", pid);
        if let Ok(content) = fs::read_to_string(&status_path) {
            let mut uid = None;
            let mut gid = None;
            
            for line in content.lines() {
                if line.starts_with("Uid:") {
                    if let Some(real_uid) = line.split_whitespace().nth(1) {
                        uid = real_uid.parse().ok();
                    }
                } else if line.starts_with("Gid:") {
                    if let Some(real_gid) = line.split_whitespace().nth(1) {
                        gid = real_gid.parse().ok();
                    }
                }
                
                if uid.is_some() && gid.is_some() {
                    break;
                }
            }
            
            (uid, gid)
        } else {
            (None, None)
        }
    }
    
    #[cfg(target_os = "macos")]
    fn from_pid_macos(pid: u32) -> ProcessResult<Self> {
        use std::mem;
        use std::ptr;
        use std::ffi::CStr;
        
        // Check if process exists first
        if unsafe { libc::kill(pid as libc::pid_t, 0) } != 0 {
            return Err(ProcessError::ProcessNotFound(pid));
        }
        
        // Get basic process info using sysctl
        let mut info: libc::proc_bsdinfo = unsafe { mem::zeroed() };
        let mut size = mem::size_of::<libc::proc_bsdinfo>();
        
        let result = unsafe {
            libc::proc_pidinfo(
                pid as i32,
                libc::PROC_PIDTBSDINFO,
                0,
                &mut info as *mut _ as *mut libc::c_void,
                size as i32,
            )
        };
        
        if result <= 0 {
            return Err(ProcessError::ProcessNotFound(pid));
        }
        
        // Extract basic info
        let parent_pid = if info.pbi_ppid > 0 { Some(info.pbi_ppid as u32) } else { None };
        let name = unsafe {
            let name_ptr = info.pbi_comm.as_ptr();
            CStr::from_ptr(name_ptr).to_string_lossy().to_string()
        };
        
        // Get process status
        let status = match info.pbi_status {
            1 => ProcessStatus::Running, // SIDL
            2 => ProcessStatus::Running, // SRUN  
            3 => ProcessStatus::Sleeping, // SSLEEP
            4 => ProcessStatus::Stopped, // SSTOP
            5 => ProcessStatus::Zombie, // SZOMB
            _ => ProcessStatus::Unknown,
        };
        
        // Get start time
        let start_time = {
            let start_tv = info.pbi_start_tvsec;
            if start_tv > 0 {
                Some(std::time::UNIX_EPOCH + Duration::from_secs(start_tv))
            } else {
                None
            }
        };
        
        // Get task info for memory and thread info
        let mut task_info: libc::proc_taskinfo = unsafe { mem::zeroed() };
        let task_size = mem::size_of::<libc::proc_taskinfo>();
        
        let task_result = unsafe {
            libc::proc_pidinfo(
                pid as i32,
                libc::PROC_PIDTASKINFO,
                0,
                &mut task_info as *mut _ as *mut libc::c_void,
                task_size as i32,
            )
        };
        
        let (virtual_memory, thread_count) = if task_result > 0 {
            (Some(task_info.pti_virtual_size), Some(task_info.pti_threads_user))
        } else {
            (None, None)
        };
        
        // Try to get command line arguments
        let command_line = Self::get_command_line_macos(pid).unwrap_or_else(|_| vec![name.clone()]);
        
        // Try to get executable path
        let executable = Self::get_executable_path_macos(pid).ok();
        
        Ok(ProcessInfo {
            pid,
            parent_pid,
            name,
            command_line,
            executable,
            working_directory: None, // Would need additional system calls
            status,
            start_time,
            cpu_time: None, // Will be calculated separately if needed
            memory_usage: None, // Will be read from task info if needed
            virtual_memory,
            uid: Some(info.pbi_uid),
            gid: Some(info.pbi_gid),
            environment: None, // Can be read separately if needed
            thread_count,
            priority: Some(info.pbi_nice as i32),
        })
    }
    
    #[cfg(target_os = "macos")]
    fn get_command_line_macos(pid: u32) -> ProcessResult<Vec<String>> {
        use std::mem;
        use std::ffi::CStr;
        
        // Get process arguments using sysctl
        let mut mib = [libc::CTL_KERN, libc::KERN_PROCARGS2, pid as i32, 0];
        let mut size = 0;
        
        // Get required buffer size
        let result = unsafe {
            libc::sysctl(
                mib.as_mut_ptr(),
                3,
                ptr::null_mut(),
                &mut size,
                ptr::null_mut(),
                0,
            )
        };
        
        if result != 0 || size == 0 {
            return Err(ProcessError::ProcessNotFound(pid));
        }
        
        // Allocate buffer and get arguments
        let mut buffer = vec![0u8; size];
        let result = unsafe {
            libc::sysctl(
                mib.as_mut_ptr(),
                3,
                buffer.as_mut_ptr() as *mut libc::c_void,
                &mut size,
                ptr::null_mut(),
                0,
            )
        };
        
        if result != 0 {
            return Err(ProcessError::SystemError(
                result,
                "Failed to get process arguments".to_string()
            ));
        }
        
        // Parse arguments from buffer
        let mut args = Vec::new();
        let mut pos = mem::size_of::<i32>(); // Skip argc
        
        // Skip to first null-terminated string
        while pos < buffer.len() && buffer[pos] != 0 {
            pos += 1;
        }
        while pos < buffer.len() && buffer[pos] == 0 {
            pos += 1;
        }
        
        // Extract arguments
        while pos < buffer.len() {
            let start = pos;
            while pos < buffer.len() && buffer[pos] != 0 {
                pos += 1;
            }
            
            if pos > start {
                if let Ok(arg) = String::from_utf8(buffer[start..pos].to_vec()) {
                    args.push(arg);
                }
            }
            
            pos += 1; // Skip null terminator
        }
        
        Ok(args)
    }
    
    #[cfg(target_os = "macos")]
    fn get_executable_path_macos(pid: u32) -> ProcessResult<PathBuf> {
        use std::mem;
        use std::ffi::CStr;
        
        let mut path_info: libc::proc_vnodepathinfo = unsafe { mem::zeroed() };
        let size = mem::size_of::<libc::proc_vnodepathinfo>();
        
        let result = unsafe {
            libc::proc_pidinfo(
                pid as i32,
                libc::PROC_PIDVNODEPATHINFO,
                0,
                &mut path_info as *mut _ as *mut libc::c_void,
                size as i32,
            )
        };
        
        if result <= 0 {
            return Err(ProcessError::ProcessNotFound(pid));
        }
        
        let path_str = unsafe {
            CStr::from_ptr(path_info.pvi_cdir.vip_path.as_ptr())
                .to_string_lossy()
                .to_string()
        };
        
        Ok(PathBuf::from(path_str))
    }
    
    #[cfg(target_os = "windows")]
    fn from_pid_windows(pid: u32) -> ProcessResult<Self> {
        use std::mem;
        use std::ptr;
        use winapi::um::processthreadsapi::*;
        use winapi::um::psapi::*;
        use winapi::um::winnt::*;
        use winapi::um::handleapi::*;
        use winapi::um::tlhelp32::*;
        use winapi::shared::minwindef::*;
        
        // Open process handle
        let handle = unsafe { 
            OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                FALSE,
                pid
            )
        };
        
        if handle == ptr::null_mut() {
            return Err(ProcessError::ProcessNotFound(pid));
        }
        
        // Ensure handle is closed when function exits
        let _handle_guard = HandleGuard(handle);
        
        // Get process basic information
        let mut pbi: winapi::um::winternl::PROCESS_BASIC_INFORMATION = unsafe { mem::zeroed() };
        let mut return_length: ULONG = 0;
        
        let status = unsafe {
            winapi::um::winternl::NtQueryInformationProcess(
                handle,
                winapi::um::winternl::ProcessBasicInformation,
                &mut pbi as *mut _ as *mut winapi::ctypes::c_void,
                mem::size_of::<winapi::um::winternl::PROCESS_BASIC_INFORMATION>() as ULONG,
                &mut return_length,
            )
        };
        
        let parent_pid = if status == 0 {
            Some(pbi.InheritedFromUniqueProcessId as u32)
        } else {
            None
        };
        
        // Get process name and executable path
        let mut image_name = [0u16; 260]; // MAX_PATH
        let mut size = image_name.len() as DWORD;
        
        let name_result = unsafe {
            QueryFullProcessImageNameW(handle, 0, image_name.as_mut_ptr(), &mut size)
        };
        
        let (name, executable) = if name_result != 0 {
            let path_str = String::from_utf16_lossy(&image_name[..size as usize]);
            let path = PathBuf::from(&path_str);
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown.exe")
                .to_string();
            (name, Some(path))
        } else {
            (format!("process_{}.exe", pid), None)
        };
        
        // Get process times
        let mut creation_time: FILETIME = unsafe { mem::zeroed() };
        let mut exit_time: FILETIME = unsafe { mem::zeroed() };
        let mut kernel_time: FILETIME = unsafe { mem::zeroed() };
        let mut user_time: FILETIME = unsafe { mem::zeroed() };
        
        let times_result = unsafe {
            GetProcessTimes(
                handle,
                &mut creation_time,
                &mut exit_time,
                &mut kernel_time,
                &mut user_time,
            )
        };
        
        let start_time = if times_result != 0 {
            Some(filetime_to_system_time(&creation_time))
        } else {
            None
        };
        
        // Get memory information
        let mut mem_counters: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
        let mem_result = unsafe {
            GetProcessMemoryInfo(
                handle,
                &mut mem_counters,
                mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD,
            )
        };
        
        let (memory_usage, virtual_memory) = if mem_result != 0 {
            (
                Some(mem_counters.WorkingSetSize as u64),
                Some(mem_counters.PagefileUsage as u64),
            )
        } else {
            (None, None)
        };
        
        // Get thread count using toolhelp snapshot
        let thread_count = get_thread_count_windows(pid).unwrap_or(1);
        
        // Get process priority
        let priority_class = unsafe { GetPriorityClass(handle) };
        let priority = if priority_class != 0 {
            Some(match priority_class {
                IDLE_PRIORITY_CLASS => -15,
                BELOW_NORMAL_PRIORITY_CLASS => -10,
                NORMAL_PRIORITY_CLASS => 0,
                ABOVE_NORMAL_PRIORITY_CLASS => 10,
                HIGH_PRIORITY_CLASS => 15,
                REALTIME_PRIORITY_CLASS => 20,
                _ => 0,
            })
        } else {
            None
        };
        
        // Try to get command line (simplified)
        let command_line = get_command_line_windows(pid).unwrap_or_else(|_| vec![name.clone()]);
        
        Ok(ProcessInfo {
            pid,
            parent_pid,
            name,
            command_line,
            executable,
            working_directory: None, // Would need additional Windows API calls
            status: ProcessStatus::Running, // Simplified - would need more API calls to determine exact status
            start_time,
            cpu_time: None, // Will be calculated separately if needed
            memory_usage,
            virtual_memory,
            uid: None, // Windows doesn't use UID/GID
            gid: None,
            environment: None, // Can be read separately if needed
            thread_count: Some(thread_count),
            priority,
        })
    }
    
    #[cfg(target_os = "windows")]
    fn filetime_to_system_time(filetime: &FILETIME) -> SystemTime {
        let time_64 = ((filetime.dwHighDateTime as u64) << 32) | (filetime.dwLowDateTime as u64);
        // Convert from Windows FILETIME (100ns intervals since 1601-01-01) to Unix timestamp
        let unix_time = (time_64 / 10_000_000).saturating_sub(11_644_473_600); // Subtract seconds between 1601 and 1970
        std::time::UNIX_EPOCH + Duration::from_secs(unix_time)
    }
    
    #[cfg(target_os = "windows")]
    fn get_thread_count_windows(pid: u32) -> ProcessResult<u32> {
        use winapi::um::tlhelp32::*;
        
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
        if snapshot == INVALID_HANDLE_VALUE {
            return Ok(1); // Default to 1 thread
        }
        
        let _snapshot_guard = HandleGuard(snapshot);
        
        let mut thread_entry: THREADENTRY32 = unsafe { mem::zeroed() };
        thread_entry.dwSize = mem::size_of::<THREADENTRY32>() as DWORD;
        
        let mut count = 0u32;
        
        if unsafe { Thread32First(snapshot, &mut thread_entry) } != 0 {
            loop {
                if thread_entry.th32OwnerProcessID == pid {
                    count += 1;
                }
                
                if unsafe { Thread32Next(snapshot, &mut thread_entry) } == 0 {
                    break;
                }
            }
        }
        
        Ok(count.max(1)) // At least 1 thread
    }
    
    #[cfg(target_os = "windows")]
    fn get_command_line_windows(pid: u32) -> ProcessResult<Vec<String>> {
        // This is a simplified implementation
        // Full implementation would use WMI or ReadProcessMemory to read PEB
        let handle = unsafe { 
            OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, pid)
        };
        
        if handle == ptr::null_mut() {
            return Err(ProcessError::ProcessNotFound(pid));
        }
        
        let _handle_guard = HandleGuard(handle);
        
        // For now, just return the executable name
        let mut image_name = [0u16; 260];
        let mut size = image_name.len() as DWORD;
        
        let result = unsafe {
            QueryFullProcessImageNameW(handle, 0, image_name.as_mut_ptr(), &mut size)
        };
        
        if result != 0 {
            let path_str = String::from_utf16_lossy(&image_name[..size as usize]);
            Ok(vec![path_str])
        } else {
            Ok(vec![format!("process_{}.exe", pid)])
        }
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
        use std::mem;
        
        // Get task info for detailed memory information
        let mut task_info: libc::proc_taskinfo = unsafe { mem::zeroed() };
        let size = mem::size_of::<libc::proc_taskinfo>();
        
        let result = unsafe {
            libc::proc_pidinfo(
                self.pid as i32,
                libc::PROC_PIDTASKINFO,
                0,
                &mut task_info as *mut _ as *mut libc::c_void,
                size as i32,
            )
        };
        
        if result <= 0 {
            return Err(ProcessError::ProcessNotFound(self.pid));
        }
        
        // Get additional memory info if available
        let mut vm_info: libc::proc_regionwithpathinfo = unsafe { mem::zeroed() };
        let vm_size = mem::size_of::<libc::proc_regionwithpathinfo>();
        
        let vm_result = unsafe {
            libc::proc_pidinfo(
                self.pid as i32,
                libc::PROC_PIDREGIONPATHINFO,
                0,
                &mut vm_info as *mut _ as *mut libc::c_void,
                vm_size as i32,
            )
        };
        
        Ok(MemoryInfo {
            rss: task_info.pti_resident_size,
            vms: task_info.pti_virtual_size,
            shared: if vm_result > 0 { vm_info.prp_prinfo.pri_share_mode as u64 } else { 0 },
            text: 0, // Would need additional proc_pidinfo calls to get text segment
            data: 0, // Would need additional proc_pidinfo calls to get data segment  
            stack: 0, // Would need additional proc_pidinfo calls to get stack size
        })
    }
    
    #[cfg(target_os = "windows")]
    fn memory_info_windows(&self) -> ProcessResult<MemoryInfo> {
        use std::mem;
        use std::ptr;
        use winapi::um::processthreadsapi::*;
        use winapi::um::psapi::*;
        use winapi::um::winnt::*;
        use winapi::um::handleapi::*;
        use winapi::shared::minwindef::*;
        
        let handle = unsafe { 
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, self.pid)
        };
        
        if handle == ptr::null_mut() {
            return Err(ProcessError::ProcessNotFound(self.pid));
        }
        
        let _handle_guard = HandleGuard(handle);
        
        // Get process memory counters
        let mut mem_counters: PROCESS_MEMORY_COUNTERS_EX = unsafe { mem::zeroed() };
        mem_counters.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as DWORD;
        
        let result = unsafe {
            GetProcessMemoryInfo(
                handle,
                &mut mem_counters as *mut _ as *mut PROCESS_MEMORY_COUNTERS,
                mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as DWORD,
            )
        };
        
        if result == 0 {
            return Err(ProcessError::SystemError(
                unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
                "Failed to get process memory info".to_string()
            ));
        }
        
        // Get virtual memory information
        let mut vm_counters: winapi::um::psapi::VM_COUNTERS_EX = unsafe { mem::zeroed() };
        let mut return_length: DWORD = 0;
        
        let vm_result = unsafe {
            winapi::um::winternl::NtQueryInformationProcess(
                handle,
                winapi::um::winternl::ProcessVmCounters,
                &mut vm_counters as *mut _ as *mut winapi::ctypes::c_void,
                mem::size_of::<winapi::um::psapi::VM_COUNTERS_EX>() as ULONG,
                &mut return_length,
            )
        };
        
        let virtual_size = if vm_result == 0 {
            vm_counters.VirtualSize as u64
        } else {
            mem_counters.PrivateUsage
        };
        
        Ok(MemoryInfo {
            rss: mem_counters.WorkingSetSize as u64,
            vms: virtual_size,
            shared: 0, // Windows doesn't directly expose shared memory in the same way
            text: 0,   // Would need additional API calls to get code segment size
            data: mem_counters.PrivateUsage,
            stack: 0,  // Would need to enumerate threads and get stack sizes
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
        use std::mem;
        
        // Get task info for CPU times
        let mut task_info: libc::proc_taskinfo = unsafe { mem::zeroed() };
        let size = mem::size_of::<libc::proc_taskinfo>();
        
        let result = unsafe {
            libc::proc_pidinfo(
                self.pid as i32,
                libc::PROC_PIDTASKINFO,
                0,
                &mut task_info as *mut _ as *mut libc::c_void,
                size as i32,
            )
        };
        
        if result <= 0 {
            return Err(ProcessError::ProcessNotFound(self.pid));
        }
        
        // Convert nanoseconds to Duration
        let user_time = Duration::from_nanos(task_info.pti_total_user);
        let system_time = Duration::from_nanos(task_info.pti_total_system);
        let total_time = user_time + system_time;
        
        // Calculate CPU percentage (simplified - would need multiple samples for accuracy)
        let cpu_percent = if let Some(start_time) = self.start_time {
            if let Ok(elapsed) = SystemTime::now().duration_since(start_time) {
                let elapsed_nanos = elapsed.as_nanos() as f64;
                let cpu_nanos = total_time.as_nanos() as f64;
                if elapsed_nanos > 0.0 {
                    Some((cpu_nanos / elapsed_nanos) * 100.0)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(CpuInfo {
            total_time,
            user_time,
            system_time,
            cpu_percent,
        })
    }
    
    #[cfg(target_os = "windows")]
    fn cpu_info_windows(&self) -> ProcessResult<CpuInfo> {
        use std::mem;
        use std::ptr;
        use winapi::um::processthreadsapi::*;
        use winapi::um::winnt::*;
        use winapi::um::handleapi::*;
        use winapi::shared::minwindef::*;
        
        let handle = unsafe { 
            OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, self.pid)
        };
        
        if handle == ptr::null_mut() {
            return Err(ProcessError::ProcessNotFound(self.pid));
        }
        
        let _handle_guard = HandleGuard(handle);
        
        // Get process times
        let mut creation_time: FILETIME = unsafe { mem::zeroed() };
        let mut exit_time: FILETIME = unsafe { mem::zeroed() };
        let mut kernel_time: FILETIME = unsafe { mem::zeroed() };
        let mut user_time: FILETIME = unsafe { mem::zeroed() };
        
        let result = unsafe {
            GetProcessTimes(
                handle,
                &mut creation_time,
                &mut exit_time,
                &mut kernel_time,
                &mut user_time,
            )
        };
        
        if result == 0 {
            return Err(ProcessError::SystemError(
                unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
                "Failed to get process times".to_string()
            ));
        }
        
        // Convert FILETIME to Duration
        let user_duration = filetime_to_duration(&user_time);
        let kernel_duration = filetime_to_duration(&kernel_time);
        let total_time = user_duration + kernel_duration;
        
        // Calculate CPU percentage (simplified - would need multiple samples for accuracy)
        let cpu_percent = if let Some(start_time) = self.start_time {
            if let Ok(elapsed) = SystemTime::now().duration_since(start_time) {
                let elapsed_nanos = elapsed.as_nanos() as f64;
                let cpu_nanos = total_time.as_nanos() as f64;
                if elapsed_nanos > 0.0 {
                    Some((cpu_nanos / elapsed_nanos) * 100.0)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(CpuInfo {
            total_time,
            user_time: user_duration,
            system_time: kernel_duration,
            cpu_percent,
        })
    }
    
    #[cfg(target_os = "windows")]
    fn filetime_to_duration(filetime: &FILETIME) -> Duration {
        let time_64 = ((filetime.dwHighDateTime as u64) << 32) | (filetime.dwLowDateTime as u64);
        // FILETIME is in 100ns intervals
        Duration::from_nanos(time_64 * 100)
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
    use std::mem;
    use std::ptr;
    
    // Get list of all processes using sysctl
    let mut mib = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_ALL, 0];
    let mut size = 0;
    
    // Get required buffer size
    let result = unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            3,
            ptr::null_mut(),
            &mut size,
            ptr::null_mut(),
            0,
        )
    };
    
    if result != 0 || size == 0 {
        return Err(ProcessError::SystemError(
            result,
            "Failed to get process list size".to_string()
        ));
    }
    
    // Allocate buffer for process info
    let proc_count = size / mem::size_of::<libc::kinfo_proc>();
    let mut procs = vec![unsafe { mem::zeroed::<libc::kinfo_proc>() }; proc_count];
    
    let result = unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            3,
            procs.as_mut_ptr() as *mut libc::c_void,
            &mut size,
            ptr::null_mut(),
            0,
        )
    };
    
    if result != 0 {
        return Err(ProcessError::SystemError(
            result,
            "Failed to get process list".to_string()
        ));
    }
    
    let actual_count = size / mem::size_of::<libc::kinfo_proc>();
    procs.truncate(actual_count);
    
    let mut processes = Vec::new();
    
    for proc_info in procs {
        let pid = proc_info.kp_proc.p_pid as u32;
        let parent_pid = if proc_info.kp_eproc.e_ppid > 0 {
            Some(proc_info.kp_eproc.e_ppid as u32)
        } else {
            None
        };
        
        // Get process name
        let name = unsafe {
            let name_ptr = proc_info.kp_proc.p_comm.as_ptr();
            std::ffi::CStr::from_ptr(name_ptr)
                .to_string_lossy()
                .to_string()
        };
        
        // Determine process status
        let status = match proc_info.kp_proc.p_stat {
            1 => ProcessStatus::Running,  // SIDL
            2 => ProcessStatus::Running,  // SRUN
            3 => ProcessStatus::Sleeping, // SSLEEP
            4 => ProcessStatus::Stopped,  // SSTOP
            5 => ProcessStatus::Zombie,   // SZOMB
            _ => ProcessStatus::Unknown,
        };
        
        // Try to get memory info (this may fail for some processes)
        let memory_rss = get_process_memory_macos(pid).ok();
        
        processes.push(ProcessListEntry {
            pid,
            parent_pid,
            name,
            status,
            memory_rss,
            cpu_percent: None, // Would need additional calculations
        });
    }
    
    Ok(processes)
}

#[cfg(target_os = "macos")]
fn get_process_memory_macos(pid: u32) -> ProcessResult<u64> {
    use std::mem;
    
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
        Ok(task_info.pti_resident_size)
    } else {
        Err(ProcessError::ProcessNotFound(pid))
    }
}

#[cfg(target_os = "windows")]
fn get_process_list_windows() -> ProcessResult<Vec<ProcessListEntry>> {
    use std::mem;
    use std::ptr;
    use winapi::um::tlhelp32::*;
    use winapi::um::handleapi::*;
    use winapi::shared::minwindef::*;
    
    // Create snapshot of all processes
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    
    if snapshot == INVALID_HANDLE_VALUE {
        return Err(ProcessError::SystemError(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "Failed to create process snapshot".to_string()
        ));
    }
    
    let _snapshot_guard = HandleGuard(snapshot);
    
    let mut processes = Vec::new();
    let mut process_entry: PROCESSENTRY32W = unsafe { mem::zeroed() };
    process_entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as DWORD;
    
    // Iterate through processes
    if unsafe { Process32FirstW(snapshot, &mut process_entry) } != 0 {
        loop {
            let pid = process_entry.th32ProcessID;
            let parent_pid = if process_entry.th32ParentProcessID != 0 {
                Some(process_entry.th32ParentProcessID)
            } else {
                None
            };
            
            // Convert process name from UTF-16
            let name_slice = unsafe {
                let mut len = 0;
                while len < process_entry.szExeFile.len() && process_entry.szExeFile[len] != 0 {
                    len += 1;
                }
                &process_entry.szExeFile[..len]
            };
            let name = String::from_utf16_lossy(name_slice);
            
            // Get memory info if possible
            let memory_rss = get_process_memory_windows(pid).ok();
            
            processes.push(ProcessListEntry {
                pid,
                parent_pid,
                name,
                status: ProcessStatus::Running, // Simplified - would need additional API calls
                memory_rss,
                cpu_percent: None, // Would need additional calculations
            });
            
            if unsafe { Process32NextW(snapshot, &mut process_entry) } == 0 {
                break;
            }
        }
    }
    
    Ok(processes)
}

#[cfg(target_os = "windows")]
fn get_process_memory_windows(pid: u32) -> ProcessResult<u64> {
    use std::mem;
    use std::ptr;
    use winapi::um::processthreadsapi::*;
    use winapi::um::psapi::*;
    use winapi::um::winnt::*;
    use winapi::um::handleapi::*;
    use winapi::shared::minwindef::*;
    
    let handle = unsafe { 
        OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid)
    };
    
    if handle == ptr::null_mut() {
        return Err(ProcessError::ProcessNotFound(pid));
    }
    
    let _handle_guard = HandleGuard(handle);
    
    let mut mem_counters: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
    
    let result = unsafe {
        GetProcessMemoryInfo(
            handle,
            &mut mem_counters,
            mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD,
        )
    };
    
    if result != 0 {
        Ok(mem_counters.WorkingSetSize as u64)
    } else {
        Err(ProcessError::SystemError(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "Failed to get process memory info".to_string()
        ))
    }
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

/// Calculate process start time from clock ticks since boot
#[cfg(target_os = "linux")]
fn calculate_start_time(start_time_ticks: u64) -> Option<SystemTime> {
    use std::fs;
    
    // Get system boot time from /proc/stat
    let boot_time = if let Ok(stat_content) = fs::read_to_string("/proc/stat") {
        let mut result = None;
        for line in stat_content.lines() {
            if line.starts_with("btime ") {
                if let Some(boot_time_str) = line.split_whitespace().nth(1) {
                    if let Ok(boot_time_secs) = boot_time_str.parse::<u64>() {
                        result = Some(std::time::UNIX_EPOCH + Duration::from_secs(boot_time_secs));
                        break;
                    }
                }
            }
        }
        result
    } else {
        None
    };
    
    if let Some(boot) = boot_time {
        // Convert ticks to seconds (assuming 100 ticks per second on most systems)
        let ticks_per_second = 100;
        let start_offset = Duration::from_nanos((start_time_ticks * 1_000_000_000) / ticks_per_second);
        Some(boot + start_offset)
    } else {
        None
    }
}

// Add num_cpus dependency (this would need to be added to Cargo.toml)
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
}
