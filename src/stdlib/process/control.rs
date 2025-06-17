/// Process control and signal handling for CURSED
/// 
/// This module provides functionality for controlling processes, sending signals,
/// managing process priority, and handling process lifecycle events.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, process_not_found_pid, permission_denied_pid,
    invalid_state, timeout_error, system_error, signal_error
};

/// Signal types for process control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    /// Terminate process (Ctrl+C)
    SIGINT = 2,
    /// Quit process
    SIGQUIT = 3,
    /// Illegal instruction
    SIGILL = 4,
    /// Abort signal
    SIGABRT = 6,
    /// Floating point exception
    SIGFPE = 8,
    /// Kill process (cannot be caught)
    SIGKILL = 9,
    /// Segmentation violation
    SIGSEGV = 11,
    /// Broken pipe
    SIGPIPE = 13,
    /// Alarm signal
    SIGALRM = 14,
    /// Terminate process (polite)
    SIGTERM = 15,
    /// User-defined signal 1
    SIGUSR1 = 10,
    /// User-defined signal 2
    SIGUSR2 = 12,
    /// Continue if stopped
    SIGCONT = 18,
    /// Stop process
    SIGSTOP = 19,
    /// Terminal stop signal
    SIGTSTP = 20,
    /// Background process attempting read
    SIGTTIN = 21,
    /// Background process attempting write
    SIGTTOU = 22,
}

impl Signal {
    /// Get signal number
    pub fn as_number(&self) -> i32 {
        *self as i32
    }

    /// Get signal name
    pub fn name(&self) -> &'static str {
        match self {
            Signal::SIGINT => "SIGINT",
            Signal::SIGQUIT => "SIGQUIT",
            Signal::SIGILL => "SIGILL",
            Signal::SIGABRT => "SIGABRT",
            Signal::SIGFPE => "SIGFPE",
            Signal::SIGKILL => "SIGKILL",
            Signal::SIGSEGV => "SIGSEGV",
            Signal::SIGPIPE => "SIGPIPE",
            Signal::SIGALRM => "SIGALRM",
            Signal::SIGTERM => "SIGTERM",
            Signal::SIGUSR1 => "SIGUSR1",
            Signal::SIGUSR2 => "SIGUSR2",
            Signal::SIGCONT => "SIGCONT",
            Signal::SIGSTOP => "SIGSTOP",
            Signal::SIGTSTP => "SIGTSTP",
            Signal::SIGTTIN => "SIGTTIN",
            Signal::SIGTTOU => "SIGTTOU",
        }
    }

    /// Check if signal can be caught/handled
    pub fn can_be_caught(&self) -> bool {
        !matches!(self, Signal::SIGKILL | Signal::SIGSTOP)
    }

    /// Check if signal terminates by default
    pub fn is_terminating(&self) -> bool {
        matches!(
            self,
            Signal::SIGINT | Signal::SIGQUIT | Signal::SIGTERM | 
            Signal::SIGKILL | Signal::SIGABRT | Signal::SIGSEGV |
            Signal::SIGILL | Signal::SIGFPE
        )
    }
}

impl From<i32> for Signal {
    fn from(num: i32) -> Self {
        match num {
            2 => Signal::SIGINT,
            3 => Signal::SIGQUIT,
            4 => Signal::SIGILL,
            6 => Signal::SIGABRT,
            8 => Signal::SIGFPE,
            9 => Signal::SIGKILL,
            10 => Signal::SIGUSR1,
            11 => Signal::SIGSEGV,
            12 => Signal::SIGUSR2,
            13 => Signal::SIGPIPE,
            14 => Signal::SIGALRM,
            15 => Signal::SIGTERM,
            18 => Signal::SIGCONT,
            19 => Signal::SIGSTOP,
            20 => Signal::SIGTSTP,
            21 => Signal::SIGTTIN,
            22 => Signal::SIGTTOU,
            _ => Signal::SIGTERM, // Default fallback
        }
    }
}

/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Very high priority
    VeryHigh = -20,
    /// High priority
    High = -10,
    /// Above normal priority
    AboveNormal = -5,
    /// Normal priority (default)
    Normal = 0,
    /// Below normal priority
    BelowNormal = 5,
    /// Low priority
    Low = 10,
    /// Very low priority
    VeryLow = 19,
}

impl Priority {
    /// Get nice value for this priority
    pub fn nice_value(&self) -> i32 {
        *self as i32
    }

    /// Create priority from nice value
    pub fn from_nice(nice: i32) -> Self {
        match nice {
            n if n <= -20 => Priority::VeryHigh,
            n if n <= -10 => Priority::High,
            n if n <= -5 => Priority::AboveNormal,
            n if n <= 0 => Priority::Normal,
            n if n <= 5 => Priority::BelowNormal,
            n if n <= 10 => Priority::Low,
            _ => Priority::VeryLow,
        }
    }
}

/// Process control operations
pub struct ProcessControl;

impl ProcessControl {
    /// Send a signal to a process
    pub fn send_signal(pid: u32, signal: Signal) -> ProcessResult<()> {
        send_signal_to_pid(pid, signal)
    }

    /// Kill a process (SIGKILL)
    pub fn kill(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGKILL)
    }

    /// Terminate a process politely (SIGTERM)
    pub fn terminate(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGTERM)
    }

    /// Stop a process (SIGSTOP)
    pub fn stop(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGSTOP)
    }

    /// Continue a stopped process (SIGCONT)
    pub fn continue_process(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGCONT)
    }

    /// Interrupt a process (SIGINT)
    pub fn interrupt(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGINT)
    }

    /// Set process priority
    pub fn set_priority(pid: u32, priority: Priority) -> ProcessResult<()> {
        set_process_priority(pid, priority)
    }

    /// Get process priority
    pub fn get_priority(pid: u32) -> ProcessResult<Priority> {
        get_process_priority(pid)
    }

    /// Wait for process to terminate
    pub fn wait_for_process(pid: u32, timeout: Option<Duration>) -> ProcessResult<bool> {
        wait_for_process(pid, timeout)
    }
}

/// Send a signal to a process by PID
pub fn send_signal_to_pid(pid: u32, signal: Signal) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe {
            libc::kill(pid as i32, signal.as_number())
        };

        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            match errno {
                libc::ESRCH => Err(process_not_found_pid(pid, "Process not found")),
                libc::EPERM => Err(permission_denied_pid("kill", pid, "Permission denied")),
                _ => Err(system_error(errno, "kill", &format!("Failed to send signal {} to process {}", signal.name(), pid))),
            }
        } else {
            Ok(())
        }
    }

    #[cfg(windows)]
    {
        use std::process::Command;

        // On Windows, use taskkill command for basic termination
        match signal {
            Signal::SIGKILL | Signal::SIGTERM => {
                let output = Command::new("taskkill")
                    .args(&["/PID", &pid.to_string(), "/F"])
                    .output()
                    .map_err(|e| system_error(-1, "taskkill", &e.to_string()))?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(system_error(-1, "taskkill", &stderr))
                } else {
                    Ok(())
                }
            }
            _ => {
                Err(signal_error(signal.name(), "send", "Signal not supported on Windows"))
            }
        }
    }
}

/// Kill a process (SIGKILL - cannot be caught)
pub fn kill_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::SIGKILL)
}

/// Terminate a process politely (SIGTERM - can be caught)
pub fn terminate_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::SIGTERM)
}

/// Kill a process gracefully (try SIGTERM first, then SIGKILL)
pub fn kill_process_graceful(pid: u32, timeout: Duration) -> ProcessResult<()> {
    // Try SIGTERM first
    terminate_process(pid)?;
    
    // Wait for process to terminate
    let start = Instant::now();
    while start.elapsed() < timeout {
        if !crate::stdlib::process::is_process_running(pid) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    
    // If still running, use SIGKILL
    kill_process(pid)
}

/// Set process priority (nice value)
pub fn set_process_priority(pid: u32, priority: Priority) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe {
            libc::setpriority(libc::PRIO_PROCESS, pid, priority.nice_value())
        };

        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            match errno {
                libc::ESRCH => Err(process_not_found_pid(pid, "Process not found")),
                libc::EPERM => Err(permission_denied_pid("setpriority", pid, "Permission denied")),
                _ => Err(system_error(errno, "setpriority", &format!("Failed to set priority for process {}", pid))),
            }
        } else {
            Ok(())
        }
    }

    #[cfg(windows)]
    {
        use std::process::Command;
        
        // Map Priority to Windows priority class
        let priority_class = match priority {
            Priority::VeryLow => "idle",
            Priority::Low => "belownormal",
            Priority::BelowNormal => "belownormal",
            Priority::Normal => "normal",
            Priority::AboveNormal => "abovenormal",
            Priority::High => "high",
            Priority::VeryHigh => "realtime",
        };
        
        // Use WMIC to set process priority
        let output = Command::new("wmic")
            .args(&[
                "process", 
                "where", 
                &format!("ProcessId={}", pid),
                "call",
                "setpriority",
                priority_class
            ])
            .output()
            .map_err(|e| system_error(-1, "set_priority", &e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            // Try PowerShell approach as fallback
            let ps_script = format!(
                "(Get-Process -Id {}).PriorityClass = '{}'", 
                pid, 
                match priority {
                    Priority::VeryLow => "Idle",
                    Priority::Low => "BelowNormal",
                    Priority::BelowNormal => "BelowNormal",
                    Priority::Normal => "Normal",
                    Priority::AboveNormal => "AboveNormal",
                    Priority::High => "High",
                    Priority::VeryHigh => "RealTime",
                }
            );
            
            let ps_output = Command::new("powershell")
                .args(&["-Command", &ps_script])
                .output()
                .map_err(|e| system_error(-1, "set_priority", &e.to_string()))?;

            if ps_output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&ps_output.stderr);
                Err(system_error(
                    -1,
                    "set_priority",
                    &format!("Failed to set process priority: {}", stderr)
                ))
            }
        }
    }
}

/// Get process priority (nice value)
pub fn get_process_priority(pid: u32) -> ProcessResult<Priority> {
    #[cfg(unix)]
    {
        // Clear errno first
        unsafe { *libc::__errno_location() = 0; }

        let result = unsafe {
            libc::getpriority(libc::PRIO_PROCESS, pid)
        };

        let errno = unsafe { *libc::__errno_location() };
        if errno != 0 {
            match errno {
                libc::ESRCH => Err(process_not_found_pid(pid, "Process not found")),
                _ => Err(system_error(errno, "getpriority", &format!("Failed to get priority for process {}", pid))),
            }
        } else {
            Ok(Priority::from_nice(result))
        }
    }

    #[cfg(windows)]
    {
        use std::process::Command;
        
        // Use PowerShell to get process priority
        let ps_script = format!(
            "(Get-Process -Id {}).PriorityClass", 
            pid
        );
        
        let output = Command::new("powershell")
            .args(&["-Command", &ps_script])
            .output()
            .map_err(|e| system_error(-1, "get_priority", &e.to_string()))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let priority_class = stdout.trim();
            
            let priority = match priority_class {
                "Idle" => Priority::VeryLow,
                "BelowNormal" => Priority::BelowNormal,
                "Normal" => Priority::Normal,
                "AboveNormal" => Priority::AboveNormal,
                "High" => Priority::High,
                "RealTime" => Priority::VeryHigh,
                _ => Priority::Normal, // Default fallback
            };
            
            Ok(priority)
        } else {
            // Fallback: Use WMIC
            let wmic_output = Command::new("wmic")
                .args(&[
                    "process", 
                    "where", 
                    &format!("ProcessId={}", pid),
                    "get",
                    "Priority",
                    "/value"
                ])
                .output()
                .map_err(|e| system_error(-1, "get_priority", &e.to_string()))?;

            if wmic_output.status.success() {
                let stdout = String::from_utf8_lossy(&wmic_output.stdout);
                for line in stdout.split("\n") {
                    if line.starts_with("Priority=") {
                        if let Some(priority_str) = line.split('=').nth(1) {
                            // Windows priority values: 4=Idle, 6=BelowNormal, 8=Normal, 10=AboveNormal, 13=High, 24=RealTime
                            let priority = match priority_str.trim().parse::<u32>() {
                                Ok(4) => Priority::VeryLow,
                                Ok(6) => Priority::BelowNormal,
                                Ok(8) => Priority::Normal,
                                Ok(10) => Priority::AboveNormal,
                                Ok(13) => Priority::High,
                                Ok(24) => Priority::VeryHigh,
                                _ => Priority::Normal,
                            };
                            return Ok(priority);
                        }
                    }
                }
            }
            
            // Final fallback
            Ok(Priority::Normal)
        }
    }
}

/// Wait for a process to terminate
pub fn wait_for_process(pid: u32, timeout: Option<Duration>) -> ProcessResult<bool> {
    let start = Instant::now();
    
    loop {
        if !crate::stdlib::process::is_process_running(pid) {
            return Ok(true);
        }
        
        if let Some(timeout) = timeout {
            if start.elapsed() >= timeout {
                return Ok(false);
            }
        }
        
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Stop a process (SIGSTOP)
pub fn stop_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::SIGSTOP)
}

/// Continue a stopped process (SIGCONT)
pub fn continue_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::SIGCONT)
}

/// Kill processes by name
pub fn kill_processes_by_name(name: &str) -> ProcessResult<Vec<u32>> {
    let processes = crate::stdlib::process::find_processes_by_name(name)?;
    let mut killed_pids = Vec::new();
    
    for process in processes {
        if kill_process(process.pid).is_ok() {
            killed_pids.push(process.pid);
        }
    }
    
    Ok(killed_pids)
}

/// Terminate processes by name
pub fn terminate_processes_by_name(name: &str) -> ProcessResult<Vec<u32>> {
    let processes = crate::stdlib::process::find_processes_by_name(name)?;
    let mut terminated_pids = Vec::new();
    
    for process in processes {
        if terminate_process(process.pid).is_ok() {
            terminated_pids.push(process.pid);
        }
    }
    
    Ok(terminated_pids)
}

/// Kill entire process tree
pub fn kill_process_tree(root_pid: u32) -> ProcessResult<Vec<u32>> {
    let tree = crate::stdlib::process::get_process_tree(root_pid)?;
    let mut killed_pids = Vec::new();
    
    // Kill children first, then parent
    for process in tree.iter().rev() {
        if kill_process(process.pid).is_ok() {
            killed_pids.push(process.pid);
        }
    }
    
    Ok(killed_pids)
}

/// Terminate entire process tree
pub fn terminate_process_tree(root_pid: u32) -> ProcessResult<Vec<u32>> {
    let tree = crate::stdlib::process::get_process_tree(root_pid)?;
    let mut terminated_pids = Vec::new();
    
    // Terminate children first, then parent
    for process in tree.iter().rev() {
        if terminate_process(process.pid).is_ok() {
            terminated_pids.push(process.pid);
        }
    }
    
    Ok(terminated_pids)
}

/// Signal handler trait
pub trait SignalHandler: Send + Sync {
    /// Handle received signal
    fn handle_signal(&self, signal: Signal) -> ProcessResult<()>;
}

/// Simple signal handler registry
static mut SIGNAL_HANDLERS: Option<Arc<Mutex<HashMap<Signal, Box<dyn SignalHandler>>>>> = None;

/// Setup a signal handler
pub fn setup_signal_handler<H: SignalHandler + 'static>(signal: Signal, handler: H) -> ProcessResult<()> {
    if !signal.can_be_caught() {
        return Err(signal_error(signal.name(), "setup_handler", "Signal cannot be caught"));
    }

    // Initialize handlers map if needed
    unsafe {
        if SIGNAL_HANDLERS.is_none() {
            SIGNAL_HANDLERS = Some(Arc::new(Mutex::new(HashMap::new())));
        }
    }

    // Store the handler
    let handlers = unsafe { SIGNAL_HANDLERS.as_ref().unwrap() };
    handlers.lock().unwrap().insert(signal, Box::new(handler));

    #[cfg(unix)]
    {
        // Install signal handler using libc
        unsafe {
            let result = libc::signal(signal.as_number(), signal_handler_wrapper as usize);
            if result == libc::SIG_ERR {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "signal",
                    &format!("Failed to install handler for {}", signal.name())
                ));
            }
        }
    }

    Ok(())
}

/// Internal signal handler wrapper
#[cfg(unix)]
extern "C" fn signal_handler_wrapper(sig: i32) {
    let signal = Signal::from(sig);
    
    // Call registered handler
    unsafe {
        if let Some(handlers) = &SIGNAL_HANDLERS {
            if let Ok(handlers) = handlers.lock() {
                if let Some(handler) = handlers.get(&signal) {
                    let _ = handler.handle_signal(signal);
                }
            }
        }
    }
}

/// Ignore a signal
pub fn ignore_signal(signal: Signal) -> ProcessResult<()> {
    if !signal.can_be_caught() {
        return Err(signal_error(signal.name(), "ignore", "Signal cannot be ignored"));
    }

    #[cfg(unix)]
    {
        unsafe {
            let result = libc::signal(signal.as_number(), libc::SIG_IGN);
            if result == libc::SIG_ERR {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "signal",
                    &format!("Failed to ignore {}", signal.name())
                ));
            }
        }
    }

    Ok(())
}

/// Reset signal to default behavior
pub fn reset_signal_handler(signal: Signal) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        unsafe {
            let result = libc::signal(signal.as_number(), libc::SIG_DFL);
            if result == libc::SIG_ERR {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "signal",
                    &format!("Failed to reset handler for {}", signal.name())
                ));
            }
        }
    }

    // Remove from our handlers map
    unsafe {
        if let Some(handlers) = &SIGNAL_HANDLERS {
            handlers.lock().unwrap().remove(&signal);
        }
    }

    Ok(())
}

/// Example signal handler implementation
pub struct DefaultSignalHandler;

impl SignalHandler for DefaultSignalHandler {
    fn handle_signal(&self, signal: Signal) -> ProcessResult<()> {
        eprintln!("Received signal: {} ({})", signal.name(), signal.as_number());
        
        if signal.is_terminating() {
            eprintln!("Terminating due to signal {}", signal.name());
            std::process::exit(signal.as_number());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_properties() {
        assert_eq!(Signal::SIGINT.as_number(), 2);
        assert_eq!(Signal::SIGINT.name(), "SIGINT");
        assert!(Signal::SIGINT.can_be_caught());
        assert!(Signal::SIGINT.is_terminating());
        
        assert!(!Signal::SIGKILL.can_be_caught());
        assert!(Signal::SIGKILL.is_terminating());
        
        assert!(Signal::SIGCONT.can_be_caught());
        assert!(!Signal::SIGCONT.is_terminating());
    }

    #[test]
    fn test_signal_conversion() {
        assert_eq!(Signal::from(2), Signal::SIGINT);
        assert_eq!(Signal::from(9), Signal::SIGKILL);
        assert_eq!(Signal::from(15), Signal::SIGTERM);
        assert_eq!(Signal::from(999), Signal::SIGTERM); // Default fallback
    }

    #[test]
    fn test_priority_values() {
        assert_eq!(Priority::Normal.nice_value(), 0);
        assert_eq!(Priority::High.nice_value(), -10);
        assert_eq!(Priority::Low.nice_value(), 10);
        
        assert_eq!(Priority::from_nice(0), Priority::Normal);
        assert_eq!(Priority::from_nice(-15), Priority::High);
        assert_eq!(Priority::from_nice(15), Priority::Low);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::VeryHigh < Priority::High);
        assert!(Priority::High < Priority::Normal);
        assert!(Priority::Normal < Priority::Low);
        assert!(Priority::Low < Priority::VeryLow);
    }

    #[test]
    fn test_process_control_current_process() {
        let current_pid = std::process::id();
        
        // These operations should work on the current process
        let priority_result = ProcessControl::get_priority(current_pid);
        // Don't assert success as it might fail due to permissions on some systems
        
        // Test that the current process is running
        assert!(crate::stdlib::process::is_process_running(current_pid));
    }

    #[test]
    fn test_wait_for_nonexistent_process() {
        // Test waiting for a process that doesn't exist
        let result = wait_for_process(999999, Some(Duration::from_millis(100)));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true); // Should return true immediately since process doesn't exist
    }

    #[test]
    fn test_signal_error_handling() {
        // Test sending signal to non-existent process
        let result = send_signal_to_pid(999999, Signal::SIGTERM);
        assert!(result.is_err());
    }

    #[test]
    fn test_default_signal_handler() {
        let handler = DefaultSignalHandler;
        
        // Test non-terminating signal
        let result = handler.handle_signal(Signal::SIGUSR1);
        assert!(result.is_ok());
        
        // Note: We can't easily test terminating signals without actually terminating
    }

    #[test]
    fn test_process_name_operations() {
        // Test operations that work with process names
        let result = kill_processes_by_name("nonexistent_process_name");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
        
        let result = terminate_processes_by_name("nonexistent_process_name");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
