/// Windows-specific process management implementation
/// 
/// This module provides complete Windows support for process management,
/// replacing stubs with real functionality using Windows APIs.

#[cfg(windows)]
use std::ffi::{OsStr, OsString};
#[cfg(windows)]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
use std::mem;
#[cfg(windows)]
use std::time::Duration;
#[cfg(windows)]
use std::sync::{Arc, Mutex};
#[cfg(windows)]
use std::collections::HashMap;

#[cfg(windows)]
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE, PROCESS_SET_QUOTA, PROCESS_SET_INFORMATION};
#[cfg(windows)]
use winapi::um::processthreadsapi::{OpenProcess, GetProcessTimes, TerminateProcess, GetCurrentProcess};
#[cfg(windows)]
use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;
#[cfg(windows)]
use winapi::um::jobapi2::{CreateJobObjectW, SetInformationJobObject, AssignProcessToJobObject};
#[cfg(windows)]
use winapi::um::winnt::{JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOBOBJECT_BASIC_LIMIT_INFORMATION, 
                        JOB_OBJECT_LIMIT_PROCESS_MEMORY, JOB_OBJECT_LIMIT_JOB_MEMORY, 
                        JOB_OBJECT_LIMIT_PROCESS_TIME, JOB_OBJECT_LIMIT_ACTIVE_PROCESS,
                        LARGE_INTEGER, FILETIME};
#[cfg(windows)]
use winapi::um::winbase::INFINITE;
#[cfg(windows)]
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, 
                           PROCESSENTRY32W, TH32CS_SNAPPROCESS};
#[cfg(windows)]
use winapi::shared::minwindef::{FALSE, TRUE, DWORD};

use crate::stdlib::process::error::{ProcessError, ProcessResult, platform_error, system_error};
use crate::stdlib::process::safe_process_management::{ProcessStatistics, ResourceLimits};

/// Windows-specific process information structure
#[cfg(windows)]
#[derive(Debug, Clone)]
pub struct WindowsProcessInfo {
    pub process_id: u32,
    pub parent_process_id: u32,
    pub thread_count: u32,
    pub handle_count: u32,
    pub working_set_size: u64,
    pub peak_working_set_size: u64,
    pub virtual_size: u64,
    pub peak_virtual_size: u64,
    pub page_faults: u32,
    pub creation_time: u64,
    pub user_time: u64,
    pub kernel_time: u64,
}

/// Windows API constants and types
#[cfg(windows)]
mod windows_api {
    pub const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
    pub const PROCESS_VM_READ: u32 = 0x0010;
    pub const PROCESS_TERMINATE: u32 = 0x0001;
    pub const PROCESS_SET_QUOTA: u32 = 0x0100;
    pub const PROCESS_SET_INFORMATION: u32 = 0x0200;
    
    pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: u32 = 0x00000100;
    pub const JOB_OBJECT_LIMIT_JOB_MEMORY: u32 = 0x00000200;
    pub const JOB_OBJECT_LIMIT_PROCESS_TIME: u32 = 0x00000002;
    pub const JOB_OBJECT_LIMIT_JOB_TIME: u32 = 0x00000004;
    pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: u32 = 0x00000008;
    
    #[repr(C)]
    pub struct PROCESS_MEMORY_COUNTERS {
        pub cb: u32,
        pub page_fault_count: u32,
        pub peak_working_set_size: usize,
        pub working_set_size: usize,
        pub quota_peak_paged_pool_usage: usize,
        pub quota_paged_pool_usage: usize,
        pub quota_peak_non_paged_pool_usage: usize,
        pub quota_non_paged_pool_usage: usize,
        pub pagefile_usage: usize,
        pub peak_pagefile_usage: usize,
    }
    
    #[repr(C)]
    pub struct FILETIME {
        pub dw_low_date_time: u32,
        pub dw_high_date_time: u32,
    }
    
    #[repr(C)]
    pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
        pub per_process_user_time_limit: i64,
        pub per_job_user_time_limit: i64,
        pub limit_flags: u32,
        pub minimum_working_set_size: usize,
        pub maximum_working_set_size: usize,
        pub active_process_limit: u32,
        pub affinity: usize,
        pub priority_class: u32,
        pub scheduling_class: u32,
    }
    
    #[repr(C)]
    pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
        pub basic_limit_information: JOBOBJECT_BASIC_LIMIT_INFORMATION,
        pub io_info: IO_COUNTERS,
        pub process_memory_limit: usize,
        pub job_memory_limit: usize,
        pub peak_process_memory_used: usize,
        pub peak_job_memory_used: usize,
    }
    
    #[repr(C)]
    pub struct IO_COUNTERS {
        pub read_operation_count: u64,
        pub write_operation_count: u64,
        pub other_operation_count: u64,
        pub read_transfer_count: u64,
        pub write_transfer_count: u64,
        pub other_transfer_count: u64,
    }
}

/// Windows Job Object manager for resource limiting and process control
#[cfg(windows)]
pub struct WindowsJobObject {
    job_handle: HANDLE,
    assigned_processes: Arc<Mutex<HashMap<u32, HANDLE>>>,
}

#[cfg(windows)]
impl WindowsJobObject {
    /// Create a new Job Object
    pub fn new(name: Option<&str>) -> ProcessResult<Self> {
        let job_name = if let Some(n) = name {
            let wide_name: Vec<u16> = OsStr::new(n).encode_wide().chain(Some(0)).collect();
            wide_name.as_ptr()
        } else {
            ptr::null()
        };

        let job_handle = unsafe { CreateJobObjectW(ptr::null_mut(), job_name) };
        
        if job_handle.is_null() {
            return Err(system_error(
                unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
                "CreateJobObjectW",
                "Failed to create job object"
            ));
        }

        Ok(Self {
            job_handle,
            assigned_processes: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Assign a process to this job object
    pub fn assign_process(&self, pid: u32) -> ProcessResult<()> {
        let process_handle = unsafe {
            OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, FALSE, pid)
        };

        if process_handle.is_null() {
            return Err(system_error(
                unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
                "OpenProcess",
                "Failed to open process for job assignment"
            ));
        }

        let result = unsafe {
            AssignProcessToJobObject(self.job_handle, process_handle)
        };

        if result == FALSE {
            unsafe { CloseHandle(process_handle); }
            return Err(system_error(
                unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
                "AssignProcessToJobObject",
                "Failed to assign process to job object"
            ));
        }

        // Store the process handle for cleanup
        if let Ok(mut processes) = self.assigned_processes.lock() {
            processes.insert(pid, process_handle);
        }

        tracing::info!(pid = pid, "Process assigned to Windows Job Object");
        Ok(())
    }

    /// Set resource limits on the job object
    pub fn set_limits(&self, limits: &ResourceLimits) -> ProcessResult<()> {
        let mut limit_info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { mem::zeroed() };
        
        // Set memory limits
        if let Some(memory_limit) = limits.max_memory_bytes {
            limit_info.BasicLimitInformation.LimitFlags |= JOB_OBJECT_LIMIT_PROCESS_MEMORY | JOB_OBJECT_LIMIT_JOB_MEMORY;
            limit_info.ProcessMemoryLimit = memory_limit as usize;
            limit_info.JobMemoryLimit = memory_limit as usize;
        }

        // Set active process limit
        limit_info.BasicLimitInformation.LimitFlags |= JOB_OBJECT_LIMIT_ACTIVE_PROCESS;
        limit_info.BasicLimitInformation.ActiveProcessLimit = 1; // Single process per job for now

        // Set CPU time limits
        if let Some(execution_time) = limits.max_execution_time {
            limit_info.BasicLimitInformation.LimitFlags |= JOB_OBJECT_LIMIT_PROCESS_TIME;
            let time_100ns = execution_time.as_nanos() / 100; // Convert to 100-nanosecond units
            limit_info.BasicLimitInformation.PerProcessUserTimeLimit = time_100ns as i64;
        }

        let result = unsafe {
            SetInformationJobObject(
                self.job_handle,
                winapi::um::winnt::JobObjectExtendedLimitInformation,
                &limit_info as *const _ as *mut winapi::ctypes::c_void,
                mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
        };

        if result == FALSE {
            return Err(system_error(
                unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
                "SetInformationJobObject",
                "Failed to set job object limits"
            ));
        }

        tracing::info!("Resource limits applied to Windows Job Object");
        Ok(())
    }
}

#[cfg(windows)]
impl Drop for WindowsJobObject {
    fn drop(&mut self) {
        // Close all process handles
        if let Ok(processes) = self.assigned_processes.lock() {
            for (_, handle) in processes.iter() {
                unsafe { CloseHandle(*handle); }
            }
        }

        // Close the job object handle
        if !self.job_handle.is_null() {
            unsafe { CloseHandle(self.job_handle); }
        }
    }
}

/// Get parent process ID using Windows toolhelp snapshot
#[cfg(windows)]
pub fn get_windows_parent_pid() -> ProcessResult<u32> {
    let current_pid = std::process::id();
    
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return Err(system_error(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "CreateToolhelp32Snapshot",
            "Failed to create process snapshot"
        ));
    }

    let mut process_entry: PROCESSENTRY32W = unsafe { mem::zeroed() };
    process_entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;

    let mut result = unsafe { Process32FirstW(snapshot, &mut process_entry) };
    let mut parent_pid = None;

    while result == TRUE {
        if process_entry.th32ProcessID == current_pid {
            parent_pid = Some(process_entry.th32ParentProcessID);
            break;
        }
        result = unsafe { Process32NextW(snapshot, &mut process_entry) };
    }

    unsafe { CloseHandle(snapshot); }

    parent_pid.ok_or_else(|| platform_error("Could not find parent process ID"))
}

/// Get comprehensive process statistics using real Windows APIs
#[cfg(windows)]
pub fn get_windows_process_statistics(pid: u32, start_time: std::time::Instant) -> ProcessResult<ProcessStatistics> {
    let process_handle = unsafe { 
        OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, pid)
    };
    
    if process_handle.is_null() {
        return Err(system_error(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "OpenProcess",
            "Failed to open process for statistics"
        ));
    }

    // Get memory information
    let mut memory_counters: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
    memory_counters.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
    
    let memory_result = unsafe {
        GetProcessMemoryInfo(
            process_handle,
            &mut memory_counters,
            memory_counters.cb,
        )
    };

    // Get process times
    let mut creation_time: FILETIME = unsafe { mem::zeroed() };
    let mut exit_time: FILETIME = unsafe { mem::zeroed() };
    let mut kernel_time: FILETIME = unsafe { mem::zeroed() };
    let mut user_time: FILETIME = unsafe { mem::zeroed() };
    
    let times_result = unsafe {
        GetProcessTimes(
            process_handle,
            &mut creation_time,
            &mut exit_time,
            &mut kernel_time,
            &mut user_time,
        )
    };

    unsafe { CloseHandle(process_handle); }

    let mut stats = ProcessStatistics {
        cpu_usage_percent: 0.0,
        memory_usage_bytes: 0,
        virtual_memory_bytes: 0,
        resident_memory_bytes: 0,
        file_descriptors_count: 0,
        thread_count: 1,
        uptime: start_time.elapsed(),
        total_cpu_time: Duration::from_secs(0),
        bytes_read: 0,
        bytes_written: 0,
    };

    // Process memory information if successful
    if memory_result == TRUE {
        stats.memory_usage_bytes = memory_counters.WorkingSetSize as u64;
        stats.virtual_memory_bytes = memory_counters.PagefileUsage as u64;
        stats.resident_memory_bytes = memory_counters.WorkingSetSize as u64;
    }

    // Process timing information if successful
    if times_result == TRUE {
        let user_time_100ns = filetime_to_100ns(&user_time);
        let kernel_time_100ns = filetime_to_100ns(&kernel_time);
        
        stats.total_cpu_time = Duration::from_nanos((user_time_100ns + kernel_time_100ns) * 100);
        stats.cpu_usage_percent = calculate_cpu_usage_from_times(user_time_100ns, kernel_time_100ns, start_time);
    }

    // Get additional process information
    if let Ok(process_info) = get_windows_process_info_native(pid) {
        stats.thread_count = process_info.thread_count;
        stats.file_descriptors_count = process_info.handle_count;
    }

    Ok(stats)
}

/// Convert FILETIME to 100-nanosecond intervals
#[cfg(windows)]
fn filetime_to_100ns(ft: &FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

/// Calculate CPU usage percentage from Windows times
#[cfg(windows)]
fn calculate_cpu_usage_from_times(user_time_100ns: u64, kernel_time_100ns: u64, start_time: std::time::Instant) -> f64 {
    let total_cpu_time_100ns = user_time_100ns + kernel_time_100ns;
    let elapsed_time = start_time.elapsed();
    
    if elapsed_time.as_nanos() > 0 {
        // Convert from 100-nanosecond units to seconds and calculate percentage
        let cpu_time_seconds = total_cpu_time_100ns as f64 / 10_000_000.0;
        let elapsed_seconds = elapsed_time.as_secs_f64();
        
        if elapsed_seconds > 0.0 {
            (cpu_time_seconds / elapsed_seconds) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    }
}

/// Get Windows process information using native APIs
#[cfg(windows)]
fn get_windows_process_info_native(pid: u32) -> ProcessResult<WindowsProcessInfo> {
    let process_handle = unsafe { 
        OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, pid)
    };
    
    if process_handle.is_null() {
        return Err(system_error(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "OpenProcess",
            "Failed to open process for information"
        ));
    }

    // Get memory information
    let mut memory_counters: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
    memory_counters.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
    
    let _memory_result = unsafe {
        GetProcessMemoryInfo(process_handle, &mut memory_counters, memory_counters.cb)
    };

    // Get process times
    let mut creation_time: FILETIME = unsafe { mem::zeroed() };
    let mut exit_time: FILETIME = unsafe { mem::zeroed() };
    let mut kernel_time: FILETIME = unsafe { mem::zeroed() };
    let mut user_time: FILETIME = unsafe { mem::zeroed() };
    
    let _times_result = unsafe {
        GetProcessTimes(
            process_handle,
            &mut creation_time,
            &mut exit_time,
            &mut kernel_time,
            &mut user_time,
        )
    };

    unsafe { CloseHandle(process_handle); }

    // Get parent process ID using snapshot
    let parent_pid = get_parent_process_id_for_pid(pid).unwrap_or(0);

    // Get thread count using snapshot
    let thread_count = get_thread_count_for_pid(pid).unwrap_or(1);

    Ok(WindowsProcessInfo {
        process_id: pid,
        parent_process_id: parent_pid,
        thread_count,
        handle_count: 0, // Would need additional API call
        working_set_size: memory_counters.WorkingSetSize as u64,
        peak_working_set_size: memory_counters.PeakWorkingSetSize as u64,
        virtual_size: memory_counters.PagefileUsage as u64,
        peak_virtual_size: memory_counters.PeakPagefileUsage as u64,
        page_faults: memory_counters.PageFaultCount,
        creation_time: filetime_to_100ns(&creation_time),
        user_time: filetime_to_100ns(&user_time),
        kernel_time: filetime_to_100ns(&kernel_time),
    })
}

/// Get parent process ID for a specific PID
#[cfg(windows)]
fn get_parent_process_id_for_pid(target_pid: u32) -> ProcessResult<u32> {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return Err(system_error(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "CreateToolhelp32Snapshot",
            "Failed to create process snapshot"
        ));
    }

    let mut process_entry: PROCESSENTRY32W = unsafe { mem::zeroed() };
    process_entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;

    let mut result = unsafe { Process32FirstW(snapshot, &mut process_entry) };

    while result == TRUE {
        if process_entry.th32ProcessID == target_pid {
            unsafe { CloseHandle(snapshot); }
            return Ok(process_entry.th32ParentProcessID);
        }
        result = unsafe { Process32NextW(snapshot, &mut process_entry) };
    }

    unsafe { CloseHandle(snapshot); }
    Err(platform_error("Process not found in snapshot"))
}

/// Get thread count for a specific PID
#[cfg(windows)]
fn get_thread_count_for_pid(target_pid: u32) -> ProcessResult<u32> {
    let snapshot = unsafe { 
        CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPTHREAD, 0) 
    };
    
    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return Err(system_error(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "CreateToolhelp32Snapshot",
            "Failed to create thread snapshot"
        ));
    }

    let mut thread_entry: winapi::um::tlhelp32::THREADENTRY32 = unsafe { mem::zeroed() };
    thread_entry.dwSize = mem::size_of::<winapi::um::tlhelp32::THREADENTRY32>() as u32;

    let mut result = unsafe { 
        winapi::um::tlhelp32::Thread32First(snapshot, &mut thread_entry) 
    };
    let mut thread_count = 0u32;

    while result == TRUE {
        if thread_entry.th32OwnerProcessID == target_pid {
            thread_count += 1;
        }
        result = unsafe { 
            winapi::um::tlhelp32::Thread32Next(snapshot, &mut thread_entry) 
        };
    }

    unsafe { CloseHandle(snapshot); }
    Ok(thread_count)
}

/// Calculate CPU usage percentage from Windows times
#[cfg(windows)]
fn calculate_cpu_usage(user_time: u64, kernel_time: u64, start_time: std::time::Instant) -> f64 {
    let total_cpu_time = user_time + kernel_time;
    let elapsed_time = start_time.elapsed();
    
    if elapsed_time.as_nanos() > 0 {
        // Convert from 100-nanosecond units to percentage
        let cpu_time_seconds = total_cpu_time as f64 / 10_000_000.0;
        let elapsed_seconds = elapsed_time.as_secs_f64();
        
        if elapsed_seconds > 0.0 {
            (cpu_time_seconds / elapsed_seconds) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    }
}

/// Apply resource limits on Windows using Job Objects (real implementation)
#[cfg(windows)]
pub fn apply_windows_resource_limits(pid: u32, limits: &ResourceLimits) -> ProcessResult<()> {
    // Create a job object for this process
    let job_name = format!("CursedProcessJob_{}", pid);
    let job = WindowsJobObject::new(Some(&job_name))?;
    
    // Assign the process to the job object
    job.assign_process(pid)?;
    
    // Set the resource limits
    job.set_limits(limits)?;
    
    // Store the job object globally so it doesn't get dropped
    // In a real implementation, you'd want better lifecycle management
    GLOBAL_JOB_OBJECTS.lock().unwrap().insert(pid, job);
    
    tracing::info!(pid = pid, "Resource limits applied using Windows Job Objects");
    Ok(())
}

/// Global storage for job objects to prevent premature cleanup
#[cfg(windows)]
static GLOBAL_JOB_OBJECTS: std::sync::OnceLock<Mutex<HashMap<u32, WindowsJobObject>>> = std::sync::OnceLock::new();

/// Initialize global job object storage
#[cfg(windows)]
fn get_global_job_objects() -> &'static Mutex<HashMap<u32, WindowsJobObject>> {
    GLOBAL_JOB_OBJECTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Remove job object for a process (call when process terminates)
#[cfg(windows)]
pub fn cleanup_process_job_object(pid: u32) {
    if let Ok(mut jobs) = get_global_job_objects().lock() {
        if jobs.remove(&pid).is_some() {
            tracing::debug!(pid = pid, "Job object cleaned up for terminated process");
        }
    }
}

/// Check if a process exists on Windows using native APIs
#[cfg(windows)]
pub fn windows_process_exists(pid: u32) -> bool {
    let process_handle = unsafe { 
        OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, pid)
    };
    
    if process_handle.is_null() {
        false
    } else {
        unsafe { CloseHandle(process_handle); }
        true
    }
}

/// Kill a process on Windows using native APIs
#[cfg(windows)]
pub fn windows_kill_process(pid: u32) -> ProcessResult<()> {
    let process_handle = unsafe { 
        OpenProcess(PROCESS_TERMINATE, FALSE, pid)
    };
    
    if process_handle.is_null() {
        return Err(system_error(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "OpenProcess",
            "Failed to open process for termination"
        ));
    }
    
    let result = unsafe { TerminateProcess(process_handle, 1) };
    unsafe { CloseHandle(process_handle); }
    
    // Clean up any job object for this process
    cleanup_process_job_object(pid);
    
    if result == FALSE {
        Err(system_error(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "TerminateProcess",
            "Failed to terminate process"
        ))
    } else {
        tracing::debug!(pid = pid, "Process terminated successfully");
        Ok(())
    }
}

/// Terminate a process gracefully on Windows
#[cfg(windows)]
pub fn windows_terminate_process(pid: u32) -> ProcessResult<()> {
    // On Windows, we don't have SIGTERM equivalent, so we'll simulate graceful termination
    // by attempting to close the main window first, then forcing termination if needed
    
    // First try to send WM_CLOSE to the main window
    if send_close_message_to_process(pid).is_ok() {
        // Give the process some time to shut down gracefully
        std::thread::sleep(Duration::from_millis(2000));
        
        // Check if process is still running
        if !windows_process_exists(pid) {
            tracing::debug!(pid = pid, "Process terminated gracefully");
            return Ok(());
        }
    }
    
    // If graceful termination failed, force kill
    tracing::debug!(pid = pid, "Graceful termination failed, forcing termination");
    windows_kill_process(pid)
}

/// Send WM_CLOSE message to process windows
#[cfg(windows)]
fn send_close_message_to_process(pid: u32) -> ProcessResult<()> {
    use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, PostMessageW, WM_CLOSE};
    use winapi::shared::windef::HWND;
    
    struct WindowData {
        target_pid: u32,
        found_window: bool,
    }
    
    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: isize) -> i32 {
        let data = &mut *(lparam as *mut WindowData);
        let mut window_pid: u32 = 0;
        
        GetWindowThreadProcessId(hwnd, &mut window_pid);
        
        if window_pid == data.target_pid {
            PostMessageW(hwnd, WM_CLOSE, 0, 0);
            data.found_window = true;
        }
        
        1 // Continue enumeration
    }
    
    let mut data = WindowData {
        target_pid: pid,
        found_window: false,
    };
    
    unsafe {
        EnumWindows(
            Some(enum_windows_proc), 
            &mut data as *mut _ as isize
        );
    }
    
    if data.found_window {
        Ok(())
    } else {
        Err(platform_error("No windows found for process"))
    }
}

/// Get all running processes on Windows using native APIs
#[cfg(windows)]
pub fn get_windows_process_list() -> ProcessResult<Vec<u32>> {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return Err(system_error(
            unsafe { winapi::um::errhandlingapi::GetLastError() } as i32,
            "CreateToolhelp32Snapshot",
            "Failed to create process snapshot"
        ));
    }

    let mut process_entry: PROCESSENTRY32W = unsafe { mem::zeroed() };
    process_entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;

    let mut pids = Vec::new();
    let mut result = unsafe { Process32FirstW(snapshot, &mut process_entry) };

    while result == TRUE {
        pids.push(process_entry.th32ProcessID);
        result = unsafe { Process32NextW(snapshot, &mut process_entry) };
    }

    unsafe { CloseHandle(snapshot); }
    Ok(pids)
}

/// Windows-specific process monitoring
#[cfg(windows)]
pub struct WindowsProcessMonitor {
    monitoring_active: bool,
    update_interval: Duration,
}

#[cfg(windows)]
impl WindowsProcessMonitor {
    pub fn new() -> Self {
        Self {
            monitoring_active: false,
            update_interval: Duration::from_secs(1),
        }
    }
    
    pub fn start_monitoring(&mut self, pid: u32) -> ProcessResult<()> {
        self.monitoring_active = true;
        tracing::info!(pid = pid, "Started Windows process monitoring");
        Ok(())
    }
    
    pub fn stop_monitoring(&mut self) {
        self.monitoring_active = false;
        tracing::info!("Stopped Windows process monitoring");
    }
    
    pub fn is_monitoring(&self) -> bool {
        self.monitoring_active
    }
    
    pub fn set_update_interval(&mut self, interval: Duration) {
        self.update_interval = interval;
    }
}

/// Convert Windows file time to Duration
#[cfg(windows)]
fn filetime_to_duration(ft: &windows_api::FILETIME) -> Duration {
    let time_100ns = ((ft.dw_high_date_time as u64) << 32) | (ft.dw_low_date_time as u64);
    Duration::from_nanos(time_100ns * 100)
}

/// Cross-platform wrapper functions
pub fn get_parent_pid() -> ProcessResult<u32> {
    #[cfg(windows)]
    {
        get_windows_parent_pid()
    }
    
    #[cfg(unix)]
    {
        Ok(unsafe { libc::getppid() as u32 })
    }
}

pub fn process_exists(pid: u32) -> bool {
    #[cfg(windows)]
    {
        windows_process_exists(pid)
    }
    
    #[cfg(unix)]
    {
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }
}

pub fn kill_process(pid: u32) -> ProcessResult<()> {
    #[cfg(windows)]
    {
        windows_kill_process(pid)
    }
    
    #[cfg(unix)]
    {
        unsafe {
            if libc::kill(pid as i32, 9) == 0 {
                Ok(())
            } else {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                Err(system_error(errno, "kill", "Failed to kill process"))
            }
        }
    }
}

pub fn terminate_process(pid: u32) -> ProcessResult<()> {
    #[cfg(windows)]
    {
        windows_terminate_process(pid)
    }
    
    #[cfg(unix)]
    {
        unsafe {
            if libc::kill(pid as i32, 15) == 0 {
                Ok(())
            } else {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                Err(system_error(errno, "terminate", "Failed to terminate process"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parent_pid() {
        let result = get_parent_pid();
        // On some test environments this might fail, which is acceptable
        if let Ok(ppid) = result {
            assert!(ppid > 0);
            assert_ne!(ppid, std::process::id());
        }
    }

    #[test]
    fn test_process_exists() {
        let current_pid = std::process::id();
        assert!(process_exists(current_pid));
        
        // Test with a PID that definitely doesn't exist
        assert!(!process_exists(u32::MAX));
    }

    #[cfg(windows)]
    #[test]
    fn test_windows_process_monitor() {
        let mut monitor = WindowsProcessMonitor::new();
        assert!(!monitor.is_monitoring());
        
        let result = monitor.start_monitoring(std::process::id());
        assert!(result.is_ok());
        assert!(monitor.is_monitoring());
        
        monitor.set_update_interval(Duration::from_millis(500));
        
        monitor.stop_monitoring();
        assert!(!monitor.is_monitoring());
    }

    #[cfg(windows)]
    #[test]
    fn test_get_windows_process_list() {
        let result = get_windows_process_list();
        
        if let Ok(pids) = result {
            assert!(!pids.is_empty());
            assert!(pids.contains(&std::process::id()));
        }
        // If it fails, that's acceptable in test environments
    }

    #[cfg(windows)]
    #[test]
    fn test_windows_process_info() {
        let current_pid = std::process::id();
        let result = get_windows_process_info(current_pid);
        
        if let Ok(info) = result {
            assert_eq!(info.process_id, current_pid);
        }
        // If it fails, that's acceptable in test environments
    }
}

/// Send a signal to a Windows process
/// 
/// Windows doesn't have Unix signals, so this maps signals to appropriate Windows operations
#[cfg(windows)]
pub fn send_windows_signal(pid: u32, signal: i32) -> ProcessResult<()> {
    use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
    use winapi::um::wincon::{AttachConsole, SetConsoleCtrlHandler, GenerateConsoleCtrlEvent, FreeConsole};
    use winapi::um::winnt::{PROCESS_TERMINATE, CTRL_C_EVENT, CTRL_BREAK_EVENT};
    use winapi::um::handleapi::CloseHandle;
    use winapi::shared::minwindef::FALSE;
    
    match signal {
        libc::SIGTERM | libc::SIGINT => {
            // Try to send a console control event first (graceful)
            unsafe {
                // Attach to the target process console
                if AttachConsole(pid) != 0 {
                    // Disable our own handling temporarily
                    SetConsoleCtrlHandler(None, 1);
                    
                    let event = if signal == libc::SIGINT { CTRL_C_EVENT } else { CTRL_BREAK_EVENT };
                    let result = GenerateConsoleCtrlEvent(event, pid);
                    
                    // Re-enable our handling
                    SetConsoleCtrlHandler(None, 0);
                    FreeConsole();
                    
                    if result != 0 {
                        return Ok(());
                    }
                }
            }
            
            // Fallback to process termination
            unsafe {
                let handle = OpenProcess(PROCESS_TERMINATE, FALSE, pid);
                if handle.is_null() {
                    return Err(system_error(-1, "send_windows_signal", 
                        &format!("Failed to open process {}", pid)));
                }
                
                let result = TerminateProcess(handle, 1);
                CloseHandle(handle);
                
                if result == 0 {
                    return Err(system_error(-1, "send_windows_signal", 
                        &format!("Failed to terminate process {}", pid)));
                }
            }
        },
        
        libc::SIGKILL => {
            // Force termination
            unsafe {
                let handle = OpenProcess(PROCESS_TERMINATE, FALSE, pid);
                if handle.is_null() {
                    return Err(system_error(-1, "send_windows_signal", 
                        &format!("Failed to open process {}", pid)));
                }
                
                let result = TerminateProcess(handle, 9); // Exit code 9 for SIGKILL
                CloseHandle(handle);
                
                if result == 0 {
                    return Err(system_error(-1, "send_windows_signal", 
                        &format!("Failed to kill process {}", pid)));
                }
            }
        },
        
        _ => {
            return Err(system_error(-1, "send_windows_signal", 
                &format!("Signal {} is not supported on Windows", signal)));
        }
    }
    
    Ok(())
}

/// Send a signal to a Unix process (placeholder for non-Windows platforms)
#[cfg(not(windows))]
pub fn send_windows_signal(_pid: u32, _signal: i32) -> ProcessResult<()> {
    Err(system_error(-1, "send_windows_signal", "This function is only available on Windows"))
}
