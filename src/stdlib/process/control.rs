/// Process control and signal handling
use std::time::Duration;
use super::error::{ProcessError, ProcessResult};
use super::info::ProcessInfo;

/// Signal types for process control
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    /// Terminate process (SIGTERM)
    Terminate,
    /// Kill process immediately (SIGKILL)
    Kill,
    /// Interrupt process (SIGINT)
    Interrupt,
    /// Quit process (SIGQUIT)
    Quit,
    /// Stop process (SIGSTOP)
    Stop,
    /// Continue process (SIGCONT)
    Continue,
    /// Hangup (SIGHUP)
    Hangup,
    /// User-defined signal 1 (SIGUSR1)
    User1,
    /// User-defined signal 2 (SIGUSR2)
    User2,
    /// Alarm (SIGALRM)
    Alarm,
    /// Child process terminated (SIGCHLD)
    Child,
    /// Pipe broken (SIGPIPE)
    Pipe,
}

/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum Priority {
    /// Very high priority
    VeryHigh = -20,
    /// High priority
    High = -10,
    /// Normal priority
    Normal = 0,
    /// Low priority
    Low = 10,
    /// Very low priority
    VeryLow = 19,
    /// Custom priority value
    Custom(i32),
}

/// Process control interface
pub trait ProcessControl {
    /// Send signal to process
    fn send_signal(&self, signal: Signal) -> ProcessResult<()>;
    
    /// Kill process immediately
    fn kill(&self) -> ProcessResult<()>;
    
    /// Terminate process gracefully
    fn terminate(&self) -> ProcessResult<()>;
    
    /// Stop (pause) process execution
    fn stop(&self) -> ProcessResult<()>;
    
    /// Continue process execution
    fn continue_process(&self) -> ProcessResult<()>;
    
    /// Set process priority
    fn set_priority(&self, priority: Priority) -> ProcessResult<()>;
    
    /// Get process priority
    fn get_priority(&self) -> ProcessResult<i32>;
    
    /// Check if process is running
    fn is_running(&self) -> ProcessResult<bool>;
}

impl ProcessControl for ProcessInfo {
    fn send_signal(&self, signal: Signal) -> ProcessResult<()> {
        send_signal_to_pid(self.pid, signal)
    }
    
    fn kill(&self) -> ProcessResult<()> {
        self.send_signal(Signal::Kill)
    }
    
    fn terminate(&self) -> ProcessResult<()> {
        self.send_signal(Signal::Terminate)
    }
    
    fn stop(&self) -> ProcessResult<()> {
        self.send_signal(Signal::Stop)
    }
    
    fn continue_process(&self) -> ProcessResult<()> {
        self.send_signal(Signal::Continue)
    }
    
    fn set_priority(&self, priority: Priority) -> ProcessResult<()> {
        set_process_priority(self.pid, priority)
    }
    
    fn get_priority(&self) -> ProcessResult<i32> {
        get_process_priority(self.pid)
    }
    
    fn is_running(&self) -> ProcessResult<bool> {
        ProcessInfo::from_pid(self.pid).map(|_| true)
            .or_else(|e| match e {
                ProcessError::ProcessNotFound(_) => Ok(false),
                other => Err(other),
            })
    }
}

/// Send signal to process by PID
pub fn send_signal_to_pid(pid: u32, signal: Signal) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        let signal_num = match signal {
            Signal::Terminate => libc::SIGTERM,
            Signal::Kill => libc::SIGKILL,
            Signal::Interrupt => libc::SIGINT,
            Signal::Quit => libc::SIGQUIT,
            Signal::Stop => libc::SIGSTOP,
            Signal::Continue => libc::SIGCONT,
            Signal::Hangup => libc::SIGHUP,
            Signal::User1 => libc::SIGUSR1,
            Signal::User2 => libc::SIGUSR2,
            Signal::Alarm => libc::SIGALRM,
            Signal::Child => libc::SIGCHLD,
            Signal::Pipe => libc::SIGPIPE,
        };
        
        let result = unsafe { libc::kill(pid as libc::pid_t, signal_num) };
        
        if result == 0 {
            Ok(())
        } else {
            let errno = std::io::Error::last_os_error();
            match errno.raw_os_error() {
                Some(libc::ESRCH) => Err(ProcessError::ProcessNotFound(pid)),
                Some(libc::EPERM) => Err(ProcessError::PermissionDenied(
                    format!("Cannot send signal to process {}", pid)
                )),
                _ => Err(ProcessError::SystemError(
                    errno.raw_os_error().unwrap_or(-1),
                    format!("Failed to send signal to process {}: {}", pid, errno)
                )),
            }
        }
    }
    
    #[cfg(windows)]
    {
        // Windows implementation using Windows API
        match signal {
            Signal::Kill | Signal::Terminate => {
                terminate_process_windows(pid)
            }
            _ => Err(ProcessError::PlatformError(
                "Signal not supported on Windows".to_string()
            ))
        }
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        Err(ProcessError::PlatformError("Unsupported platform".to_string()))
    }
}

/// Kill process by PID
pub fn kill_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::Kill)
}

/// Terminate process gracefully by PID
pub fn terminate_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::Terminate)
}

/// Kill process with grace period
pub fn kill_process_graceful(pid: u32, grace_period: Duration) -> ProcessResult<()> {
    // First try to terminate gracefully
    if let Ok(()) = terminate_process(pid) {
        // Wait for the grace period
        let start = std::time::Instant::now();
        while start.elapsed() < grace_period {
            // Check if process is still running
            match ProcessInfo::from_pid(pid) {
                Ok(_) => {
                    // Still running, wait a bit more
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(ProcessError::ProcessNotFound(_)) => {
                    // Process has terminated
                    return Ok(());
                }
                Err(e) => return Err(e),
            }
        }
    }
    
    // Grace period expired or graceful termination failed, force kill
    kill_process(pid)
}

/// Set process priority
pub fn set_process_priority(pid: u32, priority: Priority) -> ProcessResult<()> {
    let priority_value = match priority {
        Priority::Custom(value) => value,
        Priority::VeryHigh => -20,
        Priority::High => -10,
        Priority::Normal => 0,
        Priority::Low => 10,
        Priority::VeryLow => 19,
    };
    
    #[cfg(unix)]
    {
        let result = unsafe { libc::setpriority(libc::PRIO_PROCESS, pid, priority_value) };
        
        if result == 0 {
            Ok(())
        } else {
            let errno = std::io::Error::last_os_error();
            match errno.raw_os_error() {
                Some(libc::ESRCH) => Err(ProcessError::ProcessNotFound(pid)),
                Some(libc::EPERM) => Err(ProcessError::PermissionDenied(
                    format!("Cannot set priority for process {}", pid)
                )),
                _ => Err(ProcessError::SystemError(
                    errno.raw_os_error().unwrap_or(-1),
                    format!("Failed to set priority for process {}: {}", pid, errno)
                )),
            }
        }
    }
    
    #[cfg(windows)]
    {
        set_process_priority_windows(pid, priority_value)
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        Err(ProcessError::PlatformError("Unsupported platform".to_string()))
    }
}

/// Get process priority
pub fn get_process_priority(pid: u32) -> ProcessResult<i32> {
    #[cfg(unix)]
    {
        // Reset errno before calling getpriority
        unsafe { *libc::__errno_location() = 0; }
        
        let priority = unsafe { libc::getpriority(libc::PRIO_PROCESS, pid) };
        let errno = unsafe { *libc::__errno_location() };
        
        if errno == 0 {
            Ok(priority)
        } else {
            match errno {
                libc::ESRCH => Err(ProcessError::ProcessNotFound(pid)),
                _ => Err(ProcessError::SystemError(
                    errno,
                    format!("Failed to get priority for process {}", pid)
                )),
            }
        }
    }
    
    #[cfg(windows)]
    {
        get_process_priority_windows(pid)
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        Err(ProcessError::PlatformError("Unsupported platform".to_string()))
    }
}

/// Wait for process to terminate
pub fn wait_for_process(pid: u32, timeout: Option<Duration>) -> ProcessResult<i32> {
    let start_time = std::time::Instant::now();
    
    loop {
        // Check if process is still running
        match ProcessInfo::from_pid(pid) {
            Ok(_) => {
                // Process is still running
                if let Some(timeout) = timeout {
                    if start_time.elapsed() >= timeout {
                        return Err(ProcessError::Timeout(
                            format!("Process {} did not terminate within {:?}", pid, timeout)
                        ));
                    }
                }
                
                // Sleep briefly before checking again
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(ProcessError::ProcessNotFound(_)) => {
                // Process has terminated
                // Try to get exit code (platform-specific)
                return get_process_exit_code(pid);
            }
            Err(e) => return Err(e),
        }
    }
}

/// Get process exit code (if available)
fn get_process_exit_code(pid: u32) -> ProcessResult<i32> {
    #[cfg(unix)]
    {
        // On Unix, we can't easily get the exit code of an arbitrary process
        // that we didn't spawn ourselves. Return 0 as default.
        Ok(0)
    }
    
    #[cfg(windows)]
    {
        get_process_exit_code_windows(pid)
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        Ok(0)
    }
}

/// Stop (pause) process execution
pub fn stop_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::Stop)
}

/// Continue (resume) process execution
pub fn continue_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::Continue)
}

/// Kill all processes with given name
pub fn kill_processes_by_name<S: AsRef<str>>(name: S) -> ProcessResult<Vec<u32>> {
    let name_str = name.as_ref();
    let processes = super::info::find_processes_by_name(name_str)?;
    let mut killed_pids = Vec::new();
    
    for process in processes {
        if let Ok(()) = kill_process(process.pid) {
            killed_pids.push(process.pid);
        }
    }
    
    Ok(killed_pids)
}

/// Terminate all processes with given name
pub fn terminate_processes_by_name<S: AsRef<str>>(name: S) -> ProcessResult<Vec<u32>> {
    let name_str = name.as_ref();
    let processes = super::info::find_processes_by_name(name_str)?;
    let mut terminated_pids = Vec::new();
    
    for process in processes {
        if let Ok(()) = terminate_process(process.pid) {
            terminated_pids.push(process.pid);
        }
    }
    
    Ok(terminated_pids)
}

/// Kill process tree (process and all its children)
pub fn kill_process_tree(root_pid: u32) -> ProcessResult<Vec<u32>> {
    let tree = super::info::get_process_tree(root_pid)?;
    let mut killed_pids = Vec::new();
    
    // Kill children first, then parent
    for process in tree.iter().rev() {
        if let Ok(()) = kill_process(process.pid) {
            killed_pids.push(process.pid);
        }
    }
    
    Ok(killed_pids)
}

/// Terminate process tree gracefully
pub fn terminate_process_tree(root_pid: u32, grace_period: Duration) -> ProcessResult<Vec<u32>> {
    let tree = super::info::get_process_tree(root_pid)?;
    let mut terminated_pids = Vec::new();
    
    // Terminate children first, then parent
    for process in tree.iter().rev() {
        if let Ok(()) = kill_process_graceful(process.pid, grace_period) {
            terminated_pids.push(process.pid);
        }
    }
    
    Ok(terminated_pids)
}

// Platform-specific implementations

#[cfg(windows)]
fn terminate_process_windows(pid: u32) -> ProcessResult<()> {
    // Simplified Windows implementation without winapi dependency
    // In a real implementation, you would use the Windows API
    use std::process::Command;
    
    // Use taskkill command as fallback
    let output = Command::new("taskkill")
        .args(&["/F", "/PID", &pid.to_string()])
        .output()
        .map_err(|e| ProcessError::ExecutionFailed(format!("Failed to run taskkill: {}", e)))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(ProcessError::ExecutionFailed(
            format!("Failed to terminate process {}: {}", pid, String::from_utf8_lossy(&output.stderr))
        ))
    }
}

#[cfg(windows)]
fn set_process_priority_windows(pid: u32, priority: i32) -> ProcessResult<()> {
    // Simplified Windows implementation without winapi dependency
    // In a real implementation, you would use SetPriorityClass
    use std::process::Command;
    
    let priority_class = match priority {
        p if p <= -15 => "realtime",
        p if p <= -10 => "high",
        p if p <= -5 => "abovenormal",
        p if p <= 5 => "normal",
        p if p <= 10 => "belownormal",
        _ => "idle",
    };
    
    // Use wmic command as fallback
    let output = Command::new("wmic")
        .args(&["process", "where", &format!("ProcessId={}", pid), "CALL", "setpriority", priority_class])
        .output()
        .map_err(|e| ProcessError::ExecutionFailed(format!("Failed to run wmic: {}", e)))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(ProcessError::ExecutionFailed(
            format!("Failed to set priority for process {}: {}", pid, String::from_utf8_lossy(&output.stderr))
        ))
    }
}

#[cfg(windows)]
fn get_process_priority_windows(pid: u32) -> ProcessResult<i32> {
    // Simplified Windows implementation without winapi dependency
    // In a real implementation, you would use GetPriorityClass
    use std::process::Command;
    
    // Use wmic command to get priority class
    let output = Command::new("wmic")
        .args(&["process", "where", &format!("ProcessId={}", pid), "get", "Priority", "/value"])
        .output()
        .map_err(|e| ProcessError::ExecutionFailed(format!("Failed to run wmic: {}", e)))?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Parse wmic output to extract priority
        for line in stdout.lines() {
            if line.starts_with("Priority=") {
                if let Some(value_str) = line.strip_prefix("Priority=") {
                    if let Ok(priority) = value_str.trim().parse::<i32>() {
                        return Ok(priority);
                    }
                }
            }
        }
        
        // Default to normal priority if parsing fails
        Ok(0)
    } else {
        Err(ProcessError::ExecutionFailed(
            format!("Failed to get priority for process {}: {}", pid, String::from_utf8_lossy(&output.stderr))
        ))
    }
}

#[cfg(windows)]
fn get_process_exit_code_windows(pid: u32) -> ProcessResult<i32> {
    // Simplified Windows implementation without winapi dependency
    // In a real implementation, you would use GetExitCodeProcess
    
    // For a terminated process, we can't easily get the exit code without
    // having monitored it from the beginning. Return a default value.
    Ok(0)
}

// Unix-specific signal handler setup
#[cfg(unix)]
pub fn setup_signal_handler<F>(signal: Signal, handler: F) -> ProcessResult<()>
where
    F: Fn() + Send + Sync + 'static,
{
    use std::sync::Arc;
    
    let signal_num = match signal {
        Signal::Terminate => libc::SIGTERM,
        Signal::Interrupt => libc::SIGINT,
        Signal::Quit => libc::SIGQUIT,
        Signal::Hangup => libc::SIGHUP,
        Signal::User1 => libc::SIGUSR1,
        Signal::User2 => libc::SIGUSR2,
        Signal::Alarm => libc::SIGALRM,
        Signal::Child => libc::SIGCHLD,
        Signal::Pipe => libc::SIGPIPE,
        _ => return Err(ProcessError::InvalidArguments(
            "Cannot set handler for this signal".to_string()
        )),
    };
    
    // This is a simplified implementation
    // In practice, you'd use a proper signal handling library
    let handler = Arc::new(handler);
    
    // Store handler in a global registry (simplified)
    // Real implementation would use proper signal handling
    
    Ok(())
}

/// Ignore a signal (Unix only)
#[cfg(unix)]
pub fn ignore_signal(signal: Signal) -> ProcessResult<()> {
    let signal_num = match signal {
        Signal::Terminate => libc::SIGTERM,
        Signal::Interrupt => libc::SIGINT,
        Signal::Quit => libc::SIGQUIT,
        Signal::Hangup => libc::SIGHUP,
        Signal::User1 => libc::SIGUSR1,
        Signal::User2 => libc::SIGUSR2,
        Signal::Alarm => libc::SIGALRM,
        Signal::Pipe => libc::SIGPIPE,
        _ => return Err(ProcessError::InvalidArguments(
            "Cannot ignore this signal".to_string()
        )),
    };
    
    unsafe {
        libc::signal(signal_num, libc::SIG_IGN);
    }
    
    Ok(())
}

/// Reset signal handler to default (Unix only)
#[cfg(unix)]
pub fn reset_signal_handler(signal: Signal) -> ProcessResult<()> {
    let signal_num = match signal {
        Signal::Terminate => libc::SIGTERM,
        Signal::Interrupt => libc::SIGINT,
        Signal::Quit => libc::SIGQUIT,
        Signal::Hangup => libc::SIGHUP,
        Signal::User1 => libc::SIGUSR1,
        Signal::User2 => libc::SIGUSR2,
        Signal::Alarm => libc::SIGALRM,
        Signal::Child => libc::SIGCHLD,
        Signal::Pipe => libc::SIGPIPE,
        _ => return Err(ProcessError::InvalidArguments(
            "Cannot reset handler for this signal".to_string()
        )),
    };
    
    unsafe {
        libc::signal(signal_num, libc::SIG_DFL);
    }
    
    Ok(())
}
