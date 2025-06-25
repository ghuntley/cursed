use crate::error::CursedError;
/// Process control and signal handling for CURSED
/// 
/// This module provides functionality for controlling processes, sending signals,
/// managing process priority, and handling process lifecycle events.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Placeholder imports disabled
    invalid_state, timeout_error, system_error, signal_error
// };

/// Signal types for process control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    /// Terminate process (Ctrl+C)
    /// Quit process
    /// Illegal instruction
    /// Abort signal
    /// Floating point exception
    /// Kill process (cannot be caught)
    /// Segmentation violation
    /// Broken pipe
    /// Alarm signal
    /// Terminate process (polite)
    /// User-defined signal 1
    /// User-defined signal 2
    /// Continue if stopped
    /// Stop process
    /// Terminal stop signal
    /// Background process attempting read
    /// Background process attempting write
impl Signal {
    /// Get signal number
    pub fn as_number(&self) -> i32 {
        *self as i32
    /// Get signal name
    pub fn name(&self) -> &'static str {
        match self {
        }
    }

    /// Check if signal can be caught/handled
    pub fn can_be_caught(&self) -> bool {
        !matches!(self, Signal::SIGKILL | Signal::SIGSTOP)
    /// Check if signal terminates by default
    pub fn is_terminating(&self) -> bool {
        matches!(
            Signal::SIGINT | Signal::SIGQUIT | Signal::SIGTERM | 
            Signal::SIGKILL | Signal::SIGABRT | Signal::SIGSEGV |
            Signal::SIGILL | Signal::SIGFPE
        )
    }
}

impl From<i32> for Signal {
    fn from(num: i32) -> Self {
        match num {
            _ => Signal::SIGTERM, // Default fallback
        }
    }
/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Very high priority
    /// High priority
    /// Above normal priority
    /// Normal priority (default)
    /// Below normal priority
    /// Low priority
    /// Very low priority
impl Priority {
    /// Get nice value for this priority
    pub fn nice_value(&self) -> i32 {
        *self as i32
    /// Create priority from nice value
    pub fn from_nice(nice: i32) -> Self {
        match nice {
        }
    }
/// Process control operations
pub struct ProcessControl;

impl ProcessControl {
    /// Send a signal to a process
    pub fn send_signal(pid: u32, signal: Signal) -> ProcessResult<()> {
        send_signal_to_pid(pid, signal)
    /// Kill a process (SIGKILL)
    pub fn kill(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGKILL)
    /// Terminate a process politely (SIGTERM)
    pub fn terminate(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGTERM)
    /// Stop a process (SIGSTOP)
    pub fn stop(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGSTOP)
    /// Continue a stopped process (SIGCONT)
    pub fn continue_process(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGCONT)
    /// Interrupt a process (SIGINT)
    pub fn interrupt(pid: u32) -> ProcessResult<()> {
        Self::send_signal(pid, Signal::SIGINT)
    /// Set process priority
    pub fn set_priority(pid: u32, priority: Priority) -> ProcessResult<()> {
        set_process_priority(pid, priority)
    /// Get process priority
    pub fn get_priority(pid: u32) -> ProcessResult<Priority> {
        get_process_priority(pid)
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

        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            match errno {
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
/// Terminate a process politely (SIGTERM - can be caught)
pub fn terminate_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::SIGTERM)
/// Kill a process gracefully (try SIGTERM first, then SIGKILL)
pub fn kill_process_graceful(pid: u32, timeout: Duration) -> ProcessResult<()> {
    // Try SIGTERM first
    terminate_process(pid)?;
    
    // Wait for process to terminate
    let start = Instant::now();
    while start.elapsed() < timeout {
//         if !crate::stdlib::process::is_process_running(pid) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    // If still running, use SIGKILL
    kill_process(pid)
/// Set process priority (nice value)
pub fn set_process_priority(pid: u32, priority: Priority) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe {
            libc::setpriority(libc::PRIO_PROCESS, pid, priority.nice_value())

        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            match errno {
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
        
        // Use WMIC to set process priority
        let output = Command::new("wmic")
            .args(&[
                priority_class
            ])
            .output()
            .map_err(|e| system_error(-1, "set_priority", &e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            // Try PowerShell approach as fallback
            let ps_script = format!(
                match priority {
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
        // errno cleared - not needed with std::io::Error

        let result = unsafe {
            libc::getpriority(libc::PRIO_PROCESS, pid)

        let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
        if errno != 0 {
            match errno {
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
                _ => Priority::Normal, // Default fallback
            
            Ok(priority)
        } else {
            // Fallback: Use WMIC
            let wmic_output = Command::new("wmic")
                .args(&[
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
                            return Ok(priority);
                        }
                    }
                }
            }
            
            // Final fallback
            Ok(Priority::Normal)
        }
    }
/// Wait for a process to terminate
pub fn wait_for_process(pid: u32, timeout: Option<Duration>) -> ProcessResult<bool> {
    let start = Instant::now();
    
    loop {
//         if !crate::stdlib::process::is_process_running(pid) {
            return Ok(true);
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
/// Continue a stopped process (SIGCONT)
pub fn continue_process(pid: u32) -> ProcessResult<()> {
    send_signal_to_pid(pid, Signal::SIGCONT)
/// Kill processes by name
pub fn kill_processes_by_name(name: &str) -> ProcessResult<Vec<u32>> {
//     let processes = crate::stdlib::process::find_processes_by_name(name)?;
    let mut killed_pids = Vec::new();
    
    for process in processes {
        if kill_process(process.pid).is_ok() {
            killed_pids.push(process.pid);
        }
    }
    
    Ok(killed_pids)
/// Terminate processes by name
pub fn terminate_processes_by_name(name: &str) -> ProcessResult<Vec<u32>> {
//     let processes = crate::stdlib::process::find_processes_by_name(name)?;
    let mut terminated_pids = Vec::new();
    
    for process in processes {
        if terminate_process(process.pid).is_ok() {
            terminated_pids.push(process.pid);
        }
    }
    
    Ok(terminated_pids)
/// Kill entire process tree
pub fn kill_process_tree(root_pid: u32) -> ProcessResult<Vec<u32>> {
//     let tree = crate::stdlib::process::get_process_tree(root_pid)?;
    let mut killed_pids = Vec::new();
    
    // Kill children first, then parent
    for process in tree.iter().rev() {
        if kill_process(process.pid).is_ok() {
            killed_pids.push(process.pid);
        }
    }
    
    Ok(killed_pids)
/// Terminate entire process tree
pub fn terminate_process_tree(root_pid: u32) -> ProcessResult<Vec<u32>> {
//     let tree = crate::stdlib::process::get_process_tree(root_pid)?;
    let mut terminated_pids = Vec::new();
    
    // Terminate children first, then parent
    for process in tree.iter().rev() {
        if terminate_process(process.pid).is_ok() {
            terminated_pids.push(process.pid);
        }
    }
    
    Ok(terminated_pids)
/// Signal handler trait
pub trait SignalHandler: Send + Sync {
    /// Handle received signal
    fn handle_signal(&self, signal: Signal) -> ProcessResult<()>;
/// Simple signal handler registry
static mut SIGNAL_HANDLERS: Option<Arc<Mutex<HashMap<Signal, Box<dyn SignalHandler>>>>> = None;

/// Setup a signal handler
pub fn setup_signal_handler<H: SignalHandler + 'static>(signal: Signal, handler: H) -> ProcessResult<()> {
    if !signal.can_be_caught() {
        return Err(signal_error(signal.name(), "setup_handler", "Signal cannot be caught"));
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
                    &format!("Failed to install handler for {}", signal.name())
                ));
            }
        }
    Ok(())
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
/// Ignore a signal
pub fn ignore_signal(signal: Signal) -> ProcessResult<()> {
    if !signal.can_be_caught() {
        return Err(signal_error(signal.name(), "ignore", "Signal cannot be ignored"));
    #[cfg(unix)]
    {
        unsafe {
            let result = libc::signal(signal.as_number(), libc::SIG_IGN);
            if result == libc::SIG_ERR {
                return Err(system_error(
                    &format!("Failed to ignore {}", signal.name())
                ));
            }
        }
    Ok(())
/// Reset signal to default behavior
pub fn reset_signal_handler(signal: Signal) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        unsafe {
            let result = libc::signal(signal.as_number(), libc::SIG_DFL);
            if result == libc::SIG_ERR {
                return Err(system_error(
                    &format!("Failed to reset handler for {}", signal.name())
                ));
            }
        }
    // Remove from our handlers map
    unsafe {
        if let Some(handlers) = &SIGNAL_HANDLERS {
            handlers.lock().unwrap().remove(&signal);
        }
    }

    Ok(())
/// Example signal handler implementation
pub struct DefaultSignalHandler;

impl SignalHandler for DefaultSignalHandler {
    fn handle_signal(&self, signal: Signal) -> ProcessResult<()> {
        eprintln!("Received signal: {} ({})", signal.name(), signal.as_number());
        
        if signal.is_terminating() {
            eprintln!("Terminating due to signal {}", signal.name());
            std::process::exit(signal.as_number());
        Ok(())
    }
}


/// Control options for process management
#[derive(Debug, Clone)]
pub struct ControlOptions {
    /// Default timeout for operations
    /// Enable automatic cleanup
    /// Enable process monitoring
    /// Signal timeout
    /// Maximum processes to control
    /// Enable logging
/// Process control information
#[derive(Debug, Clone)]
pub struct ProcessControlInfo {
    /// Process ID
    /// Process name
    /// Control state
    /// Last signal sent
    /// Last operation time
    /// Control flags
/// Control state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlState {
    /// Process is running normally
    /// Process is paused/stopped
    /// Process is being terminated
    /// Process has terminated
    /// Process state is unknown
/// Control flags for process management
#[derive(Debug, Clone)]
pub struct ControlFlags {
    /// Can send signals
    /// Can terminate
    /// Can modify priority
    /// Is monitored
impl Default for ControlOptions {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ControlFlags {
    fn default() -> Self {
        Self {
        }
    }
impl ProcessController {
    /// Create a new process controller
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create with custom options
    pub fn with_options(options: ControlOptions) -> Self {
        Self {
        }
    }

    /// Add process to control
    pub fn add_process(&self, pid: u32, name: String) -> ProcessResult<()> {
        let mut processes = self.processes.lock().unwrap();
        
        if let Some(max) = self.options.max_processes {
            if processes.len() >= max as usize {
                return Err(invalid_state("Maximum number of controlled processes reached"));
            }
        }
        
        let control_info = ProcessControlInfo {
        
        processes.insert(pid, control_info);
        Ok(())
    /// Remove process from control
    pub fn remove_process(&self, pid: u32) -> ProcessResult<()> {
        let mut processes = self.processes.lock().unwrap();
        processes.remove(&pid);
        Ok(())
    /// Get process control info
    pub fn get_process_info(&self, pid: u32) -> Option<ProcessControlInfo> {
        let processes = self.processes.lock().unwrap();
        processes.get(&pid).cloned()
    /// List all controlled processes
    pub fn list_processes(&self) -> Vec<ProcessControlInfo> {
        let processes = self.processes.lock().unwrap();
        processes.values().cloned().collect()
    /// Send signal to process
    pub fn signal_process(&self, pid: u32, signal: Signal) -> ProcessResult<()> {
        // Update control info
        {
            let mut processes = self.processes.lock().unwrap();
            if let Some(info) = processes.get_mut(&pid) {
                info.last_signal = Some(signal);
                info.last_operation = Instant::now();
            }
        }
        
        // Send the signal
        send_signal(pid, signal)
    /// Terminate process
    pub fn terminate_process(&self, pid: u32) -> ProcessResult<()> {
        // Update control info
        {
            let mut processes = self.processes.lock().unwrap();
            if let Some(info) = processes.get_mut(&pid) {
                info.state = ControlState::Terminating;
                info.last_operation = Instant::now();
            }
        }
        
        // Terminate the process
        let result = terminate_process(pid);
        
        // Update state based on result
        {
            let mut processes = self.processes.lock().unwrap();
            if let Some(info) = processes.get_mut(&pid) {
                info.state = if result.is_ok() {
                    ControlState::Terminated
                } else {
                    ControlState::Unknown
            }
        }
        
        result
    /// Pause process (send SIGSTOP)
    pub fn pause_process(&self, pid: u32) -> ProcessResult<()> {
        let result = self.signal_process(pid, Signal::SIGSTOP);
        
        if result.is_ok() {
            let mut processes = self.processes.lock().unwrap();
            if let Some(info) = processes.get_mut(&pid) {
                info.state = ControlState::Paused;
            }
        }
        
        result
    /// Resume process (send SIGCONT)
    pub fn resume_process(&self, pid: u32) -> ProcessResult<()> {
        let result = self.signal_process(pid, Signal::SIGCONT);
        
        if result.is_ok() {
            let mut processes = self.processes.lock().unwrap();
            if let Some(info) = processes.get_mut(&pid) {
                info.state = ControlState::Running;
            }
        }
        
        result
    /// Get control options
    pub fn get_options(&self) -> &ControlOptions {
        &self.options
    /// Update control options
    pub fn set_options(&mut self, options: ControlOptions) {
        self.monitoring_enabled = options.enable_monitoring;
        self.options = options;
    /// Clean up terminated processes
    pub fn cleanup(&self) -> ProcessResult<usize> {
        let mut processes = self.processes.lock().unwrap();
        let mut removed = 0;
        
        processes.retain(|&pid, info| {
            if info.state == ControlState::Terminated || !process_exists(pid) {
                removed += 1;
                false
            } else {
                true
            }
        });
        
        Ok(removed)
    }
}
