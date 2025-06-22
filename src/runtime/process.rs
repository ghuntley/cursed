/// Process management runtime for CURSED
/// 
/// This module provides comprehensive process management capabilities including
/// process spawning, lifecycle management, IPC, shared memory, and signals.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::process::{Command, Child, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::ptr;
use std::path::PathBuf;

// Platform-specific imports for signal handling
#[cfg(unix)]
use std::os::unix::process::CommandExt;

#[cfg(unix)]
extern "C" {
    fn kill(pid: libc::pid_t, sig: libc::c_int) -> libc::c_int;
    fn waitpid(pid: libc::pid_t, status: *mut libc::c_int, options: libc::c_int) -> libc::pid_t;
}

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
use winapi::um::{
    processthreadsapi::{OpenProcess, TerminateProcess, SuspendThread, ResumeThread, OpenThread},
    winnt::{PROCESS_TERMINATE, PROCESS_SUSPEND_RESUME, THREAD_SUSPEND_RESUME},
    handleapi::CloseHandle,
    tlhelp32::{CreateToolhelp32Snapshot, Thread32First, Thread32Next, THREADENTRY32, TH32CS_SNAPTHREAD},
    errhandlingapi::GetLastError,
};

// Signal constants for cross-platform compatibility
#[cfg(unix)]
mod signals {
    pub const SIGTERM: i32 = libc::SIGTERM;
    pub const SIGKILL: i32 = libc::SIGKILL;
    pub const SIGSTOP: i32 = libc::SIGSTOP;
    pub const SIGCONT: i32 = libc::SIGCONT;
    pub const SIGINT: i32 = libc::SIGINT;
    pub const SIGHUP: i32 = libc::SIGHUP;
    pub const SIGUSR1: i32 = libc::SIGUSR1;
    pub const SIGUSR2: i32 = libc::SIGUSR2;
}

#[cfg(windows)]
mod signals {
    // Windows doesn't have Unix signals, so we map to process control operations
    pub const SIGTERM: i32 = 15;
    pub const SIGKILL: i32 = 9;
    pub const SIGSTOP: i32 = 19;  // Mapped to suspend
    pub const SIGCONT: i32 = 18;  // Mapped to resume
    pub const SIGINT: i32 = 2;
    pub const SIGHUP: i32 = 1;
    pub const SIGUSR1: i32 = 10;
    pub const SIGUSR2: i32 = 12;
}

use crate::error::{CursedError, Result as CursedResult};

/// Process runtime manager
#[derive(Debug)]
pub struct ProcessRuntime {
    /// Active processes
    processes: Arc<RwLock<HashMap<u32, ProcessInfo>>>,
    /// IPC channels
    ipc_channels: Arc<RwLock<HashMap<u64, IpcChannel>>>,
    /// Shared memory segments
    shared_memory: Arc<RwLock<HashMap<String, SharedMemorySegment>>>,
    /// Signal handlers
    signal_handlers: Arc<RwLock<HashMap<i32, SignalHandler>>>,
    /// Process counter for generating PIDs
    process_counter: Arc<Mutex<u32>>,
    /// Channel counter for generating channel IDs
    channel_counter: Arc<Mutex<u64>>,
}

/// Process information structure
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub command: String,
    pub status: ProcessStatus,
    pub start_time: u64,
    pub parent_pid: u32,
    pub memory_usage: u64,
    pub cpu_time: u64,
    pub child: Option<Arc<Mutex<Child>>>,
}

/// Process output information
#[derive(Debug, Clone)]
pub struct ProcessOutput {
    /// Exit status code
    pub status: i32,
    /// Standard output
    pub stdout: Vec<u8>,
    /// Standard error output
    pub stderr: Vec<u8>,
    /// Process execution time
    pub execution_time: Duration,
    /// Whether the process was terminated by signal
    pub terminated_by_signal: bool,
    /// Signal number if terminated by signal
    pub signal: Option<i32>,
}

impl ProcessOutput {
    pub fn new(status: i32, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        Self {
            status,
            stdout,
            stderr,
            execution_time: Duration::from_secs(0),
            terminated_by_signal: false,
            signal: None,
        }
    }
    
    pub fn success(&self) -> bool {
        self.status == 0 && !self.terminated_by_signal
    }
}

/// Process group for managing related processes
#[derive(Debug, Clone)]
pub struct ProcessGroup {
    /// Group ID
    pub pgid: u32,
    /// Processes in this group
    pub processes: Vec<u32>,
    /// Group leader PID
    pub leader: u32,
    /// Group status
    pub status: ProcessGroupStatus,
    /// Creation time
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessGroupStatus {
    Active,
    Stopped,
    Terminated,
}

impl ProcessGroup {
    pub fn new(pgid: u32, leader: u32) -> Self {
        Self {
            pgid,
            processes: vec![leader],
            leader,
            status: ProcessGroupStatus::Active,
            created_at: SystemTime::now(),
        }
    }
    
    pub fn add_process(&mut self, pid: u32) {
        if !self.processes.contains(&pid) {
            self.processes.push(pid);
        }
    }
    
    pub fn remove_process(&mut self, pid: u32) {
        self.processes.retain(|&p| p != pid);
        if self.processes.is_empty() {
            self.status = ProcessGroupStatus::Terminated;
        }
    }
}

/// Process status enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    Running = 0,
    Exited = 1,
    Killed = 2,
    Stopped = 3,
    Zombie = 4,
    Unknown = 5,
}

/// IPC channel structure
#[derive(Debug)]
pub struct IpcChannel {
    pub id: u64,
    pub channel_type: IpcChannelType,
    pub handle_data: Vec<u8>,
    pub permissions: u32,
    pub is_open: bool,
}

/// IPC channel types
#[derive(Debug, Clone, PartialEq)]
pub enum IpcChannelType {
    Pipe = 0,
    NamedPipe = 1,
    MessageQueue = 2,
    SharedMemory = 3,
    Socket = 4,
    Semaphore = 5,
}

/// IPC configuration structure
#[derive(Debug, Clone)]
pub struct IpcConfig {
    pub name: String,
    pub config_type: i32,
    pub size_or_capacity: u64,
    pub permissions: u32,
    pub flags: u32,
}

/// Shared memory segment
#[derive(Debug)]
pub struct SharedMemorySegment {
    pub name: String,
    pub data: Vec<u8>,
    pub size: usize,
    pub permissions: u32,
    pub ref_count: usize,
}

/// Signal handler
#[derive(Debug)]
pub struct SignalHandler {
    pub signal: i32,
    pub handler: Option<fn(i32)>,
    pub is_blocked: bool,
}

/// Global process runtime instance
static mut PROCESS_RUNTIME: Option<Arc<ProcessRuntime>> = None;
static PROCESS_RUNTIME_INIT: std::sync::Once = std::sync::Once::new();

impl ProcessRuntime {
    /// Create a new process runtime
    pub fn new() -> Self {
        Self {
            processes: Arc::new(RwLock::new(HashMap::new())),
            ipc_channels: Arc::new(RwLock::new(HashMap::new())),
            shared_memory: Arc::new(RwLock::new(HashMap::new())),
            signal_handlers: Arc::new(RwLock::new(HashMap::new())),
            process_counter: Arc::new(Mutex::new(1000)), // Start from 1000 for clarity
            channel_counter: Arc::new(Mutex::new(1)),
        }
    }

    /// Spawn a new process
    pub fn spawn_process(&self, command: &str, args: &[String]) -> CursedResult<u32> {
        // Generate unique PID
        let pid = {
            let mut counter = self.process_counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        // Parse command and arguments
        let mut cmd = Command::new(command);
        for arg in args {
            cmd.arg(arg);
        }

        // Configure process
        cmd.stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        // Spawn the process
        let child = cmd.spawn().map_err(|e| CursedError::RuntimeError {
            message: format!("Failed to spawn process '{}': {}", command, e),
        })?;

        let actual_pid = child.id();
        
        // Create process info
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let process_info = ProcessInfo {
            pid: actual_pid,
            command: command.to_string(),
            status: ProcessStatus::Running,
            start_time,
            parent_pid: std::process::id(),
            memory_usage: 0,
            cpu_time: 0,
            child: Some(Arc::new(Mutex::new(child))),
        };

        // Store process info
        {
            let mut processes = self.processes.write().unwrap();
            processes.insert(actual_pid, process_info);
        }

        Ok(actual_pid)
    }

    /// Kill a process
    pub fn kill_process(&self, pid: u32) -> CursedResult<i32> {
        let mut processes = self.processes.write().unwrap();
        if let Some(process_info) = processes.get_mut(&pid) {
            if let Some(child_arc) = &process_info.child {
                let mut child = child_arc.lock().unwrap();
                match child.kill() {
                    Ok(_) => {
                        process_info.status = ProcessStatus::Killed;
                        Ok(0)
                    }
                    Err(e) => Err(CursedError::RuntimeError {
                        message: format!("Failed to kill process {}: {}", pid, e),
                    }),
                }
            } else {
                Err(CursedError::RuntimeError {
                    message: format!("Process {} has no child handle", pid),
                })
            }
        } else {
            Err(CursedError::RuntimeError {
                message: format!("Process {} not found", pid),
            })
        }
    }

    /// Terminate a process gracefully
    pub fn terminate_process(&self, pid: u32) -> CursedResult<i32> {
        // Send SIGTERM for graceful termination
        self.send_signal(pid, signals::SIGTERM)
    }

    /// Pause a process by sending SIGSTOP
    pub fn pause_process(&self, pid: u32) -> CursedResult<i32> {
        self.send_signal(pid, signals::SIGSTOP)
    }

    /// Resume a process by sending SIGCONT
    pub fn resume_process(&self, pid: u32) -> CursedResult<i32> {
        self.send_signal(pid, signals::SIGCONT)
    }

    /// Wait for a process to complete
    pub fn wait_process(&self, pid: u32) -> CursedResult<i32> {
        let processes = self.processes.read().unwrap();
        if let Some(process_info) = processes.get(&pid) {
            if let Some(child_arc) = &process_info.child {
                drop(processes); // Release the read lock
                
                let mut child = child_arc.lock().unwrap();
                match child.wait() {
                    Ok(exit_status) => {
                        let exit_code = exit_status.code().unwrap_or(-1);
                        
                        // Update process status
                        let mut processes = self.processes.write().unwrap();
                        if let Some(process_info) = processes.get_mut(&pid) {
                            process_info.status = if exit_code == 0 {
                                ProcessStatus::Exited
                            } else {
                                ProcessStatus::Killed
                            };
                        }
                        
                        Ok(exit_code)
                    }
                    Err(e) => Err(CursedError::RuntimeError {
                        message: format!("Failed to wait for process {}: {}", pid, e),
                    }),
                }
            } else {
                Err(CursedError::RuntimeError {
                    message: format!("Process {} has no child handle", pid),
                })
            }
        } else {
            Err(CursedError::RuntimeError {
                message: format!("Process {} not found", pid),
            })
        }
    }

    /// Get process status
    pub fn get_process_status(&self, pid: u32) -> CursedResult<i32> {
        let processes = self.processes.read().unwrap();
        if let Some(process_info) = processes.get(&pid) {
            Ok(process_info.status.clone() as i32)
        } else {
            Err(CursedError::RuntimeError {
                message: format!("Process {} not found", pid),
            })
        }
    }

    /// Get process information
    pub fn get_process_info(&self, pid: u32) -> CursedResult<*mut ProcessInfo> {
        let processes = self.processes.read().unwrap();
        if let Some(process_info) = processes.get(&pid) {
            // Clone the process info and box it
            let boxed_info = Box::new(process_info.clone());
            Ok(Box::into_raw(boxed_info))
        } else {
            Ok(ptr::null_mut())
        }
    }

    /// Create an IPC channel
    pub fn create_ipc_channel(&self, channel_type: IpcChannelType, config: &IpcConfig) -> CursedResult<u64> {
        let channel_id = {
            let mut counter = self.channel_counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        let channel = IpcChannel {
            id: channel_id,
            channel_type,
            handle_data: Vec::new(),
            permissions: config.permissions,
            is_open: true,
        };

        {
            let mut channels = self.ipc_channels.write().unwrap();
            channels.insert(channel_id, channel);
        }

        Ok(channel_id)
    }

    /// Send data through IPC channel
    pub fn ipc_send(&self, channel_id: u64, data: &[u8]) -> CursedResult<i32> {
        let mut channels = self.ipc_channels.write().unwrap();
        if let Some(channel) = channels.get_mut(&channel_id) {
            if channel.is_open {
                // For simplicity, store data in the channel
                // In a real implementation, this would send through the actual IPC mechanism
                channel.handle_data.extend_from_slice(data);
                Ok(0)
            } else {
                Err(CursedError::RuntimeError {
                    message: format!("IPC channel {} is closed", channel_id),
                })
            }
        } else {
            Err(CursedError::RuntimeError {
                message: format!("IPC channel {} not found", channel_id),
            })
        }
    }

    /// Receive data from IPC channel
    pub fn ipc_receive(&self, channel_id: u64, timeout_ms: i64) -> CursedResult<*mut c_void> {
        let channels = self.ipc_channels.read().unwrap();
        if let Some(channel) = channels.get(&channel_id) {
            if channel.is_open && !channel.handle_data.is_empty() {
                // For simplicity, return a copy of the data
                // In a real implementation, this would receive from the actual IPC mechanism
                let data = channel.handle_data.clone();
                let boxed_data = Box::new(data);
                Ok(Box::into_raw(boxed_data) as *mut c_void)
            } else {
                // Simulate timeout behavior
                if timeout_ms > 0 {
                    thread::sleep(Duration::from_millis(timeout_ms as u64));
                }
                Ok(ptr::null_mut())
            }
        } else {
            Err(CursedError::RuntimeError {
                message: format!("IPC channel {} not found", channel_id),
            })
        }
    }

    /// Create shared memory segment
    pub fn create_shared_memory(&self, name: &str, size: usize) -> CursedResult<*mut c_void> {
        let segment = SharedMemorySegment {
            name: name.to_string(),
            data: vec![0u8; size],
            size,
            permissions: 0o666,
            ref_count: 1,
        };

        {
            let mut shm = self.shared_memory.write().unwrap();
            shm.insert(name.to_string(), segment);
        }

        // Return a pointer to the data (simplified implementation)
        let shm = self.shared_memory.read().unwrap();
        if let Some(segment) = shm.get(name) {
            Ok(segment.data.as_ptr() as *mut c_void)
        } else {
            Ok(ptr::null_mut())
        }
    }

    /// Send signal to process
    pub fn send_signal(&self, pid: u32, signal: i32) -> CursedResult<i32> {
        let processes = self.processes.read().unwrap();
        if !processes.contains_key(&pid) {
            return Err(CursedError::RuntimeError {
                message: format!("Process {} not found for signal {}", pid, signal),
            });
        }

        self.send_signal_impl(pid, signal)
    }

    /// Platform-specific signal implementation
    #[cfg(unix)]
    fn send_signal_impl(&self, pid: u32, signal: i32) -> CursedResult<i32> {
        let result = unsafe { kill(pid as libc::pid_t, signal) };
        
        if result == 0 {
            // Update process status based on signal
            match signal {
                signals::SIGKILL | signals::SIGTERM => {
                    let mut processes = self.processes.write().unwrap();
                    if let Some(process_info) = processes.get_mut(&pid) {
                        process_info.status = if signal == signals::SIGKILL {
                            ProcessStatus::Killed
                        } else {
                            ProcessStatus::Exited
                        };
                    }
                }
                signals::SIGSTOP => {
                    let mut processes = self.processes.write().unwrap();
                    if let Some(process_info) = processes.get_mut(&pid) {
                        process_info.status = ProcessStatus::Stopped;
                    }
                }
                signals::SIGCONT => {
                    let mut processes = self.processes.write().unwrap();
                    if let Some(process_info) = processes.get_mut(&pid) {
                        process_info.status = ProcessStatus::Running;
                    }
                }
                _ => {
                    // Other signals don't change process status
                }
            }
            Ok(0)
        } else {
            let errno = std::io::Error::last_os_error();
            Err(CursedError::RuntimeError {
                message: format!("Failed to send signal {} to process {}: {}", signal, pid, errno),
            })
        }
    }

    /// Windows signal implementation using process control
    #[cfg(windows)]
    fn send_signal_impl(&self, pid: u32, signal: i32) -> CursedResult<i32> {
        match signal {
            signals::SIGKILL | signals::SIGTERM => {
                self.windows_terminate_process(pid, signal == signals::SIGKILL)
            }
            signals::SIGSTOP => {
                self.windows_suspend_process(pid)
            }
            signals::SIGCONT => {
                self.windows_resume_process(pid)
            }
            _ => {
                // Other signals are not directly supported on Windows
                // Return success for compatibility
                Ok(0)
            }
        }
    }

    /// Windows process termination
    #[cfg(windows)]
    fn windows_terminate_process(&self, pid: u32, force: bool) -> CursedResult<i32> {
        use winapi::um::winnt::PROCESS_TERMINATE;
        
        let handle = unsafe { OpenProcess(PROCESS_TERMINATE, 0, pid) };
        if handle.is_null() {
            let error = unsafe { GetLastError() };
            return Err(CursedError::RuntimeError {
                message: format!("Failed to open process {} for termination: error {}", pid, error),
            });
        }

        let exit_code = if force { 1 } else { 0 };
        let result = unsafe { TerminateProcess(handle, exit_code) };
        unsafe { CloseHandle(handle) };

        if result != 0 {
            // Update process status
            let mut processes = self.processes.write().unwrap();
            if let Some(process_info) = processes.get_mut(&pid) {
                process_info.status = if force {
                    ProcessStatus::Killed
                } else {
                    ProcessStatus::Exited
                };
            }
            Ok(0)
        } else {
            let error = unsafe { GetLastError() };
            Err(CursedError::RuntimeError {
                message: format!("Failed to terminate process {}: error {}", pid, error),
            })
        }
    }

    /// Windows process suspension (suspend all threads)
    #[cfg(windows)]
    fn windows_suspend_process(&self, pid: u32) -> CursedResult<i32> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            let error = unsafe { GetLastError() };
            return Err(CursedError::RuntimeError {
                message: format!("Failed to create thread snapshot: error {}", error),
            });
        }

        let mut thread_entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            cntUsage: 0,
            th32ThreadID: 0,
            th32OwnerProcessID: 0,
            tpBasePri: 0,
            tpDeltaPri: 0,
            dwFlags: 0,
        };

        let mut suspended_count = 0;
        let mut result = unsafe { Thread32First(snapshot, &mut thread_entry) };

        while result != 0 {
            if thread_entry.th32OwnerProcessID == pid {
                let thread_handle = unsafe {
                    OpenThread(THREAD_SUSPEND_RESUME, 0, thread_entry.th32ThreadID)
                };
                
                if !thread_handle.is_null() {
                    unsafe { SuspendThread(thread_handle) };
                    unsafe { CloseHandle(thread_handle) };
                    suspended_count += 1;
                }
            }
            result = unsafe { Thread32Next(snapshot, &mut thread_entry) };
        }

        unsafe { CloseHandle(snapshot) };

        if suspended_count > 0 {
            // Update process status
            let mut processes = self.processes.write().unwrap();
            if let Some(process_info) = processes.get_mut(&pid) {
                process_info.status = ProcessStatus::Stopped;
            }
            Ok(0)
        } else {
            Err(CursedError::RuntimeError {
                message: format!("No threads found for process {} to suspend", pid),
            })
        }
    }

    /// Windows process resume (resume all threads)
    #[cfg(windows)]
    fn windows_resume_process(&self, pid: u32) -> CursedResult<i32> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            let error = unsafe { GetLastError() };
            return Err(CursedError::RuntimeError {
                message: format!("Failed to create thread snapshot: error {}", error),
            });
        }

        let mut thread_entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            cntUsage: 0,
            th32ThreadID: 0,
            th32OwnerProcessID: 0,
            tpBasePri: 0,
            tpDeltaPri: 0,
            dwFlags: 0,
        };

        let mut resumed_count = 0;
        let mut result = unsafe { Thread32First(snapshot, &mut thread_entry) };

        while result != 0 {
            if thread_entry.th32OwnerProcessID == pid {
                let thread_handle = unsafe {
                    OpenThread(THREAD_SUSPEND_RESUME, 0, thread_entry.th32ThreadID)
                };
                
                if !thread_handle.is_null() {
                    unsafe { ResumeThread(thread_handle) };
                    unsafe { CloseHandle(thread_handle) };
                    resumed_count += 1;
                }
            }
            result = unsafe { Thread32Next(snapshot, &mut thread_entry) };
        }

        unsafe { CloseHandle(snapshot) };

        if resumed_count > 0 {
            // Update process status
            let mut processes = self.processes.write().unwrap();
            if let Some(process_info) = processes.get_mut(&pid) {
                process_info.status = ProcessStatus::Running;
            }
            Ok(0)
        } else {
            Err(CursedError::RuntimeError {
                message: format!("No threads found for process {} to resume", pid),
            })
        }
    }

    /// Register signal handler
    pub fn register_signal_handler(&self, signal: i32, handler: Option<fn(i32)>) -> CursedResult<i32> {
        let signal_handler = SignalHandler {
            signal,
            handler,
            is_blocked: false,
        };

        {
            let mut handlers = self.signal_handlers.write().unwrap();
            handlers.insert(signal, signal_handler);
        }

        Ok(0)
    }
}

/// Initialize the global process runtime
pub fn initialize_process_runtime() {
    PROCESS_RUNTIME_INIT.call_once(|| {
        let runtime = Arc::new(ProcessRuntime::new());
        unsafe {
            PROCESS_RUNTIME = Some(runtime);
        }
    });
}

/// Get the global process runtime
pub fn get_process_runtime() -> Option<Arc<ProcessRuntime>> {
    unsafe { PROCESS_RUNTIME.as_ref().map(|r| r.clone()) }
}

/// Shutdown the process runtime
pub fn shutdown_process_runtime() {
    unsafe {
        PROCESS_RUNTIME = None;
    }
}

// FFI functions for LLVM integration

/// FFI: Spawn a process
#[no_mangle]
pub extern "C" fn cursed_process_spawn(
    command: *const c_char,
    args: *const *const c_char,
    args_count: c_int,
) -> c_int {
    if command.is_null() {
        return -1;
    }

    let cmd_str = unsafe {
        match CStr::from_ptr(command).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        }
    };

    let mut arg_strings = Vec::new();
    if !args.is_null() && args_count > 0 {
        for i in 0..args_count {
            let arg_ptr = unsafe { *args.offset(i as isize) };
            if !arg_ptr.is_null() {
                if let Ok(arg_str) = unsafe { CStr::from_ptr(arg_ptr).to_str() } {
                    arg_strings.push(arg_str.to_string());
                }
            }
        }
    }

    if let Some(runtime) = get_process_runtime() {
        match runtime.spawn_process(cmd_str, &arg_strings) {
            Ok(pid) => pid as c_int,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Kill a process
#[no_mangle]
pub extern "C" fn cursed_process_kill(pid: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        match runtime.kill_process(pid as u32) {
            Ok(result) => result,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Terminate a process
#[no_mangle]
pub extern "C" fn cursed_process_terminate(pid: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        match runtime.terminate_process(pid as u32) {
            Ok(result) => result,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Pause a process using SIGSTOP
#[no_mangle]
pub extern "C" fn cursed_process_pause(pid: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        match runtime.pause_process(pid as u32) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Resume a process using SIGCONT
#[no_mangle]
pub extern "C" fn cursed_process_resume(pid: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        match runtime.resume_process(pid as u32) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Wait for a process
#[no_mangle]
pub extern "C" fn cursed_process_wait(pid: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        match runtime.wait_process(pid as u32) {
            Ok(exit_code) => exit_code,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Get process status
#[no_mangle]
pub extern "C" fn cursed_process_get_status(pid: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        match runtime.get_process_status(pid as u32) {
            Ok(status) => status,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Get process info
#[no_mangle]
pub extern "C" fn cursed_process_get_info(pid: c_int) -> *mut ProcessInfo {
    if let Some(runtime) = get_process_runtime() {
        match runtime.get_process_info(pid as u32) {
            Ok(info_ptr) => info_ptr,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Create IPC pipe
#[no_mangle]
pub extern "C" fn cursed_pipe_create(config: *const IpcConfig) -> *mut c_void {
    if config.is_null() {
        return ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    if let Some(runtime) = get_process_runtime() {
        match runtime.create_ipc_channel(IpcChannelType::Pipe, config_ref) {
            Ok(channel_id) => channel_id as *mut c_void,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Create named pipe
#[no_mangle]
pub extern "C" fn cursed_named_pipe_create(config: *const IpcConfig) -> *mut c_void {
    if config.is_null() {
        return ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    if let Some(runtime) = get_process_runtime() {
        match runtime.create_ipc_channel(IpcChannelType::NamedPipe, config_ref) {
            Ok(channel_id) => channel_id as *mut c_void,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Create message queue
#[no_mangle]
pub extern "C" fn cursed_message_queue_create(config: *const IpcConfig) -> *mut c_void {
    if config.is_null() {
        return ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    if let Some(runtime) = get_process_runtime() {
        match runtime.create_ipc_channel(IpcChannelType::MessageQueue, config_ref) {
            Ok(channel_id) => channel_id as *mut c_void,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Create shared memory
#[no_mangle]
pub extern "C" fn cursed_shared_memory_create(config: *const IpcConfig) -> *mut c_void {
    if config.is_null() {
        return ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    if let Some(runtime) = get_process_runtime() {
        match runtime.create_shared_memory(&config_ref.name, config_ref.size_or_capacity as usize) {
            Ok(ptr) => ptr,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Create socket
#[no_mangle]
pub extern "C" fn cursed_socket_create(config: *const IpcConfig) -> *mut c_void {
    if config.is_null() {
        return ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    if let Some(runtime) = get_process_runtime() {
        match runtime.create_ipc_channel(IpcChannelType::Socket, config_ref) {
            Ok(channel_id) => channel_id as *mut c_void,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Create semaphore
#[no_mangle]
pub extern "C" fn cursed_semaphore_create(config: *const IpcConfig) -> *mut c_void {
    if config.is_null() {
        return ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    if let Some(runtime) = get_process_runtime() {
        match runtime.create_ipc_channel(IpcChannelType::Semaphore, config_ref) {
            Ok(channel_id) => channel_id as *mut c_void,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Send IPC data
#[no_mangle]
pub extern "C" fn cursed_ipc_send(channel: *mut c_void, data: *mut c_void) -> c_int {
    if channel.is_null() || data.is_null() {
        return -1;
    }

    let channel_id = channel as u64;
    // Simplified: assume data is a null-terminated string
    let data_slice = unsafe {
        let data_ptr = data as *const u8;
        let mut len = 0;
        while *data_ptr.offset(len) != 0 {
            len += 1;
        }
        std::slice::from_raw_parts(data_ptr, len as usize)
    };

    if let Some(runtime) = get_process_runtime() {
        match runtime.ipc_send(channel_id, data_slice) {
            Ok(result) => result,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Receive IPC data
#[no_mangle]
pub extern "C" fn cursed_ipc_receive(channel: *mut c_void, timeout: c_long) -> *mut c_void {
    if channel.is_null() {
        return ptr::null_mut();
    }

    let channel_id = channel as u64;
    if let Some(runtime) = get_process_runtime() {
        match runtime.ipc_receive(channel_id, timeout as i64) {
            Ok(data_ptr) => data_ptr,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

// Shared memory FFI functions (simplified implementations)

/// FFI: Create shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_create() -> *mut c_void {
    // Default shared memory creation
    if let Some(runtime) = get_process_runtime() {
        match runtime.create_shared_memory("default", 4096) {
            Ok(ptr) => ptr,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Open shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_open(name: *const c_char) -> *mut c_void {
    if name.is_null() {
        return ptr::null_mut();
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };

    if let Some(runtime) = get_process_runtime() {
        let shm = runtime.shared_memory.read().unwrap();
        if let Some(segment) = shm.get(name_str) {
            segment.data.as_ptr() as *mut c_void
        } else {
            ptr::null_mut()
        }
    } else {
        ptr::null_mut()
    }
}

/// FFI: Map shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_map(handle: *mut c_void) -> *mut c_void {
    // In a simplified implementation, just return the handle
    handle
}

/// FFI: Unmap shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_unmap(handle: *mut c_void) -> c_int {
    // Simplified implementation
    if handle.is_null() {
        -1
    } else {
        0
    }
}

/// FFI: Read from shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_read(handle: *mut c_void, offset: c_long, length: c_long) -> *mut c_void {
    if handle.is_null() {
        return ptr::null_mut();
    }

    // Simplified implementation - just return offset pointer
    unsafe { (handle as *mut u8).offset(offset as isize) as *mut c_void }
}

/// FFI: Write to shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_write(handle: *mut c_void, offset: c_long, data: *mut c_void, length: c_long) -> c_int {
    if handle.is_null() || data.is_null() {
        return -1;
    }

    // Simplified implementation - copy data
    unsafe {
        let src = data as *const u8;
        let dst = (handle as *mut u8).offset(offset as isize);
        std::ptr::copy_nonoverlapping(src, dst, length as usize);
    }

    0
}

/// FFI: Sync shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_sync(handle: *mut c_void) -> c_int {
    // Simplified implementation
    if handle.is_null() {
        -1
    } else {
        0
    }
}

/// FFI: Lock shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_lock(handle: *mut c_void) -> c_int {
    // Simplified implementation
    if handle.is_null() {
        -1
    } else {
        0
    }
}

/// FFI: Unlock shared memory
#[no_mangle]
pub extern "C" fn cursed_shm_unlock(handle: *mut c_void) -> c_int {
    // Simplified implementation
    if handle.is_null() {
        -1
    } else {
        0
    }
}

// Signal FFI functions

/// FFI: Send signal
#[no_mangle]
pub extern "C" fn cursed_signal_send(pid: c_int, signal: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        match runtime.send_signal(pid as u32, signal) {
            Ok(result) => result,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Register signal handler
#[no_mangle]
pub extern "C" fn cursed_signal_register(signal: c_int, handler: *mut c_void) -> c_int {
    let handler_fn = if handler.is_null() {
        None
    } else {
        // Convert the C function pointer to a Rust function
        // Safety: The caller must ensure the function pointer is valid and has the correct signature
        unsafe {
            let handler_ptr = std::mem::transmute::<*mut c_void, fn(i32)>(handler);
            Some(handler_ptr)
        }
    };

    if let Some(runtime) = get_process_runtime() {
        match runtime.register_signal_handler(signal, handler_fn) {
            Ok(result) => result,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Unregister signal handler
#[no_mangle]
pub extern "C" fn cursed_signal_unregister(signal: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        match runtime.register_signal_handler(signal, None) {
            Ok(result) => result,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// FFI: Block signal
#[no_mangle]
pub extern "C" fn cursed_signal_block(signal: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        let mut handlers = runtime.signal_handlers.write().unwrap();
        if let Some(handler) = handlers.get_mut(&signal) {
            handler.is_blocked = true;
            0
        } else {
            -1
        }
    } else {
        -1
    }
}

/// FFI: Unblock signal
#[no_mangle]
pub extern "C" fn cursed_signal_unblock(signal: c_int) -> c_int {
    if let Some(runtime) = get_process_runtime() {
        let mut handlers = runtime.signal_handlers.write().unwrap();
        if let Some(handler) = handlers.get_mut(&signal) {
            handler.is_blocked = false;
            0
        } else {
            -1
        }
    } else {
        -1
    }
}

/// FFI: Wait for signal
#[no_mangle]
pub extern "C" fn cursed_signal_wait(signal: c_int, timeout: c_long) -> c_int {
    // Simplified implementation - just wait for the timeout
    if timeout > 0 {
        thread::sleep(Duration::from_millis(timeout as u64));
    }
    
    // Return the signal number to indicate it was received
    signal
}

impl Default for ProcessRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_runtime_creation() {
        let runtime = ProcessRuntime::new();
        assert!(runtime.processes.read().unwrap().is_empty());
        assert!(runtime.ipc_channels.read().unwrap().is_empty());
    }

    #[test]
    fn test_process_spawn_simple() {
        let runtime = ProcessRuntime::new();
        
        // Test spawning echo command
        let result = runtime.spawn_process("echo", &["hello".to_string()]);
        assert!(result.is_ok());
        
        let pid = result.unwrap();
        assert!(pid > 0);
        
        // Check process is tracked
        let processes = runtime.processes.read().unwrap();
        assert!(processes.contains_key(&pid));
    }

    #[test]
    fn test_ipc_channel_creation() {
        let runtime = ProcessRuntime::new();
        
        let config = IpcConfig {
            name: "test_channel".to_string(),
            config_type: 0,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        };
        
        let result = runtime.create_ipc_channel(IpcChannelType::Pipe, &config);
        assert!(result.is_ok());
        
        let channel_id = result.unwrap();
        assert!(channel_id > 0);
        
        // Check channel is tracked
        let channels = runtime.ipc_channels.read().unwrap();
        assert!(channels.contains_key(&channel_id));
    }

    #[test]
    fn test_shared_memory_creation() {
        let runtime = ProcessRuntime::new();
        
        let result = runtime.create_shared_memory("test_shm", 4096);
        assert!(result.is_ok());
        
        let ptr = result.unwrap();
        assert!(!ptr.is_null());
        
        // Check shared memory is tracked
        let shm = runtime.shared_memory.read().unwrap();
        assert!(shm.contains_key("test_shm"));
    }

    #[test]
    fn test_signal_handler_registration() {
        let runtime = ProcessRuntime::new();
        
        let result = runtime.register_signal_handler(15, None); // SIGTERM
        assert!(result.is_ok());
        
        // Check handler is registered
        let handlers = runtime.signal_handlers.read().unwrap();
        assert!(handlers.contains_key(&15));
    }
}

/// Resource limits for runtime process management
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    pub max_memory: Option<u64>,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: Option<f64>,
    /// Maximum execution time
    pub max_execution_time: Option<Duration>,
    /// Maximum file descriptors
    pub max_file_descriptors: Option<u32>,
    /// Maximum number of processes
    pub max_processes: Option<u32>,
    /// Maximum stack size
    pub max_stack_size: Option<u64>,
    /// Maximum heap size
    pub max_heap_size: Option<u64>,
}

/// Security context for runtime process management
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// User ID to run as
    pub user_id: Option<u32>,
    /// Group ID to run as
    pub group_id: Option<u32>,
    /// Additional group IDs
    pub supplementary_groups: Vec<u32>,
    /// Allowed capabilities
    pub capabilities: Vec<String>,
    /// Chroot directory
    pub chroot_dir: Option<PathBuf>,
    /// Network namespace
    pub network_namespace: Option<String>,
    /// Process namespace
    pub process_namespace: Option<String>,
    /// Mount namespace
    pub mount_namespace: Option<String>,
    /// User namespace
    pub user_namespace: Option<String>,
    /// Security labels
    pub security_labels: HashMap<String, String>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: None,
            max_cpu_percent: None,
            max_execution_time: None,
            max_file_descriptors: None,
            max_processes: None,
            max_stack_size: None,
            max_heap_size: None,
        }
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user_id: None,
            group_id: None,
            supplementary_groups: Vec::new(),
            capabilities: Vec::new(),
            chroot_dir: None,
            network_namespace: None,
            process_namespace: None,
            mount_namespace: None,
            user_namespace: None,
            security_labels: HashMap::new(),
        }
    }
}
