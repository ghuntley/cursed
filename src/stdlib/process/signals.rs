use crate::error::CursedError;
/// Cross-platform signal handling for process management
/// 
/// This module provides signal handling capabilities for process control and communication

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// use crate::stdlib::process::error::{ProcessError, ProcessResult, system_error};

/// Signal types (mapped across platforms)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    /// Hang up detected on controlling terminal or death of controlling process
    /// Interrupt from keyboard
    /// Quit from keyboard
    /// Illegal instruction
    /// Trace/breakpoint trap
    /// Abort signal
    /// Bus error
    /// Floating point exception
    /// Kill signal (cannot be caught or ignored)
    /// User-defined signal 1
    /// Segmentation violation
    /// User-defined signal 2
    /// Broken pipe
    /// Alarm clock
    /// Termination signal
    /// Child stopped or terminated
    /// Continue if stopped
    /// Stop process
    /// Stop typed at terminal
    /// Terminal input for background process
    /// Terminal output for background process
    /// I/O now possible
    /// CPU time limit exceeded
    /// File size limit exceeded
    /// Virtual alarm clock
    /// Profiling alarm clock
    /// Window resize signal
    /// Pollable event
    /// Power failure
    /// Bad system call
/// Type alias for Signal for compatibility
pub type SignalType = Signal;

impl Signal {
    /// Get platform-specific signal number
    pub fn as_raw(&self) -> i32 {
        #[cfg(unix)]
        {
            match self {
            }
        }
        
        #[cfg(windows)]
        {
            // Windows doesn't have POSIX signals, map to control events
            match self {
                Signal::Interrupt => 0, // CTRL_C_EVENT
                Signal::Terminate => 1, // CTRL_BREAK_EVENT
                Signal::HangUp => 2,    // CTRL_CLOSE_EVENT
                Signal::Kill => 3,      // CTRL_LOGOFF_EVENT
                _ => -1, // Not supported on Windows
            }
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            0 // Fallback
        }
    }
    
    /// Create signal from raw number
    pub fn from_raw(signum: i32) -> Option<Self> {
        #[cfg(unix)]
        {
            match signum {
            }
        }
        
        #[cfg(windows)]
        {
            match signum {
            }
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            None
        }
    }
    
    /// Get signal name
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    /// Check if signal can be caught or ignored
    pub fn can_be_caught(&self) -> bool {
        !matches!(self, Signal::Kill | Signal::Stop)
    /// Check if signal is terminating by default
    pub fn is_terminating(&self) -> bool {
            Signal::HangUp | Signal::Interrupt | Signal::Quit | 
            Signal::IllegalInstruction | Signal::Trap | Signal::Abort |
            Signal::BusError | Signal::FloatingPointException | Signal::Kill |
            Signal::SegmentationViolation | Signal::BrokenPipe | Signal::Alarm |
            Signal::Terminate | Signal::CpuTimeLimit | Signal::FileSizeLimit |
            Signal::VirtualAlarm | Signal::ProfilingAlarm | Signal::Poll |
            Signal::PowerFailure | Signal::BadSystemCall
        )
    }
}

/// Signal action
#[derive(Debug, Clone)]
pub enum SignalAction {
    /// Ignore the signal
    /// Use default action
    /// Call custom handler
/// Signal handler registration
pub struct SignalHandler {
impl SignalHandler {
    /// Create a new signal handler
    pub fn new() -> Self {
        let (signal_sender, signal_receiver) = mpsc::channel();
        let handlers = Arc::new(Mutex::new(HashMap::new()));
        let handlers_clone = Arc::clone(&handlers);
        let receiver_arc = Arc::new(Mutex::new(signal_receiver));
        let receiver_clone = Arc::clone(&receiver_arc);
        
        let handler_thread = thread::spawn(move || {
            loop {
                if let Ok(receiver) = receiver_clone.lock() {
                    match receiver.recv_timeout(Duration::from_millis(100)) {
                        Ok(signal) => {
                            if let Ok(handlers) = handlers_clone.lock() {
                                if let Some(action) = handlers.get(&signal) {
                                    match action {
                                        SignalAction::Handle(handler) => {
                                            handler(signal);
                                        }
                                        SignalAction::Ignore => {
                                            // Do nothing
                                        }
                                        SignalAction::Default => {
                                            // Let the signal perform its default action
                                            // In a real implementation, this would restore default handling
                                        }
                                    }
                                }
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            // Continue polling
                        }
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            break; // Exit thread
                        }
                    }
                }
            }
        });
        
        Self {
        }
    }
    
    /// Register a signal handler
    pub fn register(&self, signal: Signal, action: SignalAction) -> ProcessResult<()> {
        if !signal.can_be_caught() {
            return Err(system_error(-1, format!("Signal {} cannot be caught", signal.name())));
        let mut handlers = self.handlers.lock()
            .map_err(|_| system_error(-1, "Failed to lock signal handlers".to_string()))?;
        
        // Install the actual signal handler
        self.install_signal_handler(signal)?;
        
        handlers.insert(signal, action);
        Ok(())
    /// Unregister a signal handler
    pub fn unregister(&self, signal: Signal) -> ProcessResult<()> {
        let mut handlers = self.handlers.lock()
            .map_err(|_| system_error(-1, "Failed to lock signal handlers".to_string()))?;
        
        handlers.remove(&signal);
        
        // Restore default signal handling
        self.restore_default_handler(signal)?;
        
        Ok(())
    /// Send a signal (simulate receiving a signal)
    pub fn simulate_signal(&self, signal: Signal) -> ProcessResult<()> {
        self.signal_sender.send(signal)
            .map_err(|_| system_error(-1, "Failed to send signal".to_string()))?;
        Ok(())
    /// Install platform-specific signal handler
    fn install_signal_handler(&self, signal: Signal) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            self.install_unix_signal_handler(signal)
        #[cfg(windows)]
        {
            self.install_windows_signal_handler(signal)
        #[cfg(not(any(unix, windows)))]
        {
            Ok(()) // No-op for unsupported platforms
        }
    }
    
    #[cfg(unix)]
    fn install_unix_signal_handler(&self, signal: Signal) -> ProcessResult<()> {
        use std::sync::atomic::{AtomicPtr, Ordering};
        use std::ptr;
        use std::mem;
        
        let signum = signal.as_raw();
        let sender = self.signal_sender.clone();
        
        // Thread-safe signal handler registration using sigaction
        static SIGNAL_SENDER: AtomicPtr<mpsc::Sender<Signal>> = AtomicPtr::new(ptr::null_mut());
        
        // Store sender in static location for signal handler access
        let sender_box = Box::new(sender);
        let sender_ptr = Box::into_raw(sender_box);
        
        // Safely replace any existing sender
        let old_ptr = SIGNAL_SENDER.swap(sender_ptr, Ordering::SeqCst);
        if !old_ptr.is_null() {
            unsafe { let _ = Box::from_raw(old_ptr); } // Cleanup old sender
        // Advanced signal handler using sigaction for better control
        extern "C" fn signal_handler(signum: i32, _info: *mut libc::siginfo_t, _context: *mut libc::c_void) {
            let sender_ptr = SIGNAL_SENDER.load(Ordering::SeqCst);
            if !sender_ptr.is_null() {
                unsafe {
                    if let Some(signal) = Signal::from_raw(signum) {
                        // Signal-safe send operation (fire and forget)
                        let _ = (*sender_ptr).try_send(signal);
                    }
                }
            }
        }
        
        // Setup sigaction structure for advanced signal handling
        let mut action: libc::sigaction = unsafe { mem::zeroed() };
        action.sa_sigaction = signal_handler as usize;
        action.sa_flags = libc::SA_SIGINFO | libc::SA_RESTART | libc::SA_NOCLDSTOP;
        
        // Block other signals during handler execution
        unsafe {
            libc::sigemptyset(&mut action.sa_mask);
            libc::sigaddset(&mut action.sa_mask, libc::SIGINT);
            libc::sigaddset(&mut action.sa_mask, libc::SIGTERM);
            libc::sigaddset(&mut action.sa_mask, libc::SIGQUIT);
        // Install the signal handler
        let mut old_action: libc::sigaction = unsafe { mem::zeroed() };
        let result = unsafe { 
            libc::sigaction(signum, &action, &mut old_action)
        
        if result != 0 {
            // Cleanup on failure
            let _ = SIGNAL_SENDER.swap(ptr::null_mut(), Ordering::SeqCst);
            return Err(system_error(
                format!("Failed to install signal handler for {} using sigaction", signal.name())
            ));
        Ok(())
    #[cfg(windows)]
    fn install_windows_signal_handler(&self, signal: Signal) -> ProcessResult<()> {
        use std::sync::atomic::{AtomicPtr, Ordering};
        use std::ptr;
        
        // Windows console control handler for signal-like events
        static SIGNAL_SENDER: AtomicPtr<mpsc::Sender<Signal>> = AtomicPtr::new(ptr::null_mut());
        
        let sender = self.signal_sender.clone();
        let sender_box = Box::new(sender);
        let sender_ptr = Box::into_raw(sender_box);
        
        // Safely replace any existing sender
        let old_ptr = SIGNAL_SENDER.swap(sender_ptr, Ordering::SeqCst);
        if !old_ptr.is_null() {
            unsafe { let _ = Box::from_raw(old_ptr); }
        }
        
        // Console control handler function
        extern "system" fn console_ctrl_handler(ctrl_type: u32) -> i32 {
            let sender_ptr = SIGNAL_SENDER.load(Ordering::SeqCst);
            if !sender_ptr.is_null() {
                unsafe {
                    let signal = match ctrl_type {
                        0 => Some(Signal::Interrupt),    // CTRL_C_EVENT
                        1 => Some(Signal::Terminate),    // CTRL_BREAK_EVENT  
                        2 => Some(Signal::HangUp),       // CTRL_CLOSE_EVENT
                        5 => Some(Signal::Kill),         // CTRL_LOGOFF_EVENT
                        6 => Some(Signal::Kill),         // CTRL_SHUTDOWN_EVENT
                    
                    if let Some(sig) = signal {
                        let _ = (*sender_ptr).try_send(sig);
                    }
                }
            }
            1 // TRUE - we handled the signal
        match signal {
            Signal::Interrupt | Signal::Terminate | Signal::HangUp | Signal::Kill => {
                // Install console control handler for supported signals
                unsafe {
                    use winapi::um::wincon::SetConsoleCtrlHandler;
                    
                    let result = SetConsoleCtrlHandler(
                        1 // TRUE - add handler
                    );
                    
                    if result == 0 {
                        // Cleanup on failure
                        let _ = SIGNAL_SENDER.swap(ptr::null_mut(), Ordering::SeqCst);
                        return Err(system_error(
                            format!("Failed to install console control handler for {}", signal.name())
                        ));
                    }
                }
                Ok(())
            }
            _ => Err(system_error(-1, format!("Signal {} not supported on Windows", signal.name())))
        }
    }
    
    /// Restore default signal handling
    fn restore_default_handler(&self, signal: Signal) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            let signum = signal.as_raw();
            unsafe {
                if libc::signal(signum, libc::SIG_DFL) == libc::SIG_ERR {
                    return Err(system_error(
                        format!("Failed to restore default handler for {}", signal.name())
                    ));
                }
            }
        #[cfg(windows)]
        {
            // Windows implementation would unregister console ctrl handler
        Ok(())
    }
}

/// Send a signal to a process
pub fn send_signal(pid: u32, signal: Signal) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        let signum = signal.as_raw();
        let result = unsafe { libc::kill(pid as libc::pid_t, signum) };
        
        if result == 0 {
            Ok(())
        } else {
            Err(system_error(
                format!("Failed to send signal {} to process {}", signal.name(), pid)
            ))
        }
    }
    
    #[cfg(windows)]
    {
        // Windows signal sending using GenerateConsoleCtrlEvent or TerminateProcess
        match signal {
            Signal::Interrupt => {
                // GenerateConsoleCtrlEvent(CTRL_C_EVENT, pid)
                windows_send_ctrl_event(pid, 0)
            }
            Signal::Terminate => {
                // GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid)
                windows_send_ctrl_event(pid, 1)
            }
            Signal::Kill => {
                // TerminateProcess
                windows_terminate_process(pid)
            }
            _ => Err(system_error(-1, format!("Signal {} not supported on Windows", signal.name())))
        }
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        Err(system_error(-1, "Signal sending not supported on this platform".to_string()))
    }
}

#[cfg(windows)]
fn windows_send_ctrl_event(pid: u32, event_type: u32) -> ProcessResult<()> {
    use std::process::Command;
    
    // Use taskkill as a fallback (not perfect but works)
    let result = match event_type {
        0 => Command::new("taskkill").arg("/PID").arg(pid.to_string()).output(),
        1 => Command::new("taskkill").arg("/F").arg("/PID").arg(pid.to_string()).output(),
    
    match result {
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(system_error(-1, format!("Failed to send signal: {}", error_msg)))
        }
        Err(e) => Err(system_error(-1, format!("Failed to execute taskkill: {}", e)))
    }
}

#[cfg(windows)]
fn windows_terminate_process(pid: u32) -> ProcessResult<()> {
    windows_send_ctrl_event(pid, 1) // Force terminate
/// Signal mask for blocking/unblocking signals
#[cfg(unix)]
pub struct SignalMask {
#[cfg(unix)]
impl SignalMask {
    /// Create an empty signal mask
    pub fn empty() -> ProcessResult<Self> {
        let mut mask = unsafe { std::mem::zeroed() };
        
        let result = unsafe { libc::sigemptyset(&mut mask) };
        if result != 0 {
            return Err(system_error(
                "Failed to create empty signal mask".to_string()
            ));
        Ok(Self { mask })
    /// Create a full signal mask (all signals)
    pub fn full() -> ProcessResult<Self> {
        let mut mask = unsafe { std::mem::zeroed() };
        
        let result = unsafe { libc::sigfillset(&mut mask) };
        if result != 0 {
            return Err(system_error(
                "Failed to create full signal mask".to_string()
            ));
        Ok(Self { mask })
    /// Add a signal to the mask
    pub fn add(&mut self, signal: Signal) -> ProcessResult<()> {
        let signum = signal.as_raw();
        let result = unsafe { libc::sigaddset(&mut self.mask, signum) };
        
        if result != 0 {
            Err(system_error(
                format!("Failed to add signal {} to mask", signal.name())
            ))
        } else {
            Ok(())
        }
    }
    
    /// Remove a signal from the mask
    pub fn remove(&mut self, signal: Signal) -> ProcessResult<()> {
        let signum = signal.as_raw();
        let result = unsafe { libc::sigdelset(&mut self.mask, signum) };
        
        if result != 0 {
            Err(system_error(
                format!("Failed to remove signal {} from mask", signal.name())
            ))
        } else {
            Ok(())
        }
    }
    
    /// Check if signal is in the mask
    pub fn contains(&self, signal: Signal) -> bool {
        let signum = signal.as_raw();
        unsafe { libc::sigismember(&self.mask, signum) == 1 }
    }
    
    /// Block signals in this mask
    pub fn block(&self) -> ProcessResult<SignalMask> {
        let mut old_mask = unsafe { std::mem::zeroed() };
        
        let result = unsafe { 
            libc::pthread_sigmask(libc::SIG_BLOCK, &self.mask, &mut old_mask) 
        
        if result != 0 {
            Err(system_error(result, "Failed to block signals".to_string()))
        } else {
            Ok(SignalMask { mask: old_mask })
        }
    }
    
    /// Unblock signals in this mask
    pub fn unblock(&self) -> ProcessResult<SignalMask> {
        let mut old_mask = unsafe { std::mem::zeroed() };
        
        let result = unsafe { 
            libc::pthread_sigmask(libc::SIG_UNBLOCK, &self.mask, &mut old_mask) 
        
        if result != 0 {
            Err(system_error(result, "Failed to unblock signals".to_string()))
        } else {
            Ok(SignalMask { mask: old_mask })
        }
    }
    
    /// Set signal mask (replace current mask)
    pub fn set(&self) -> ProcessResult<SignalMask> {
        let mut old_mask = unsafe { std::mem::zeroed() };
        
        let result = unsafe { 
            libc::pthread_sigmask(libc::SIG_SETMASK, &self.mask, &mut old_mask) 
        
        if result != 0 {
            Err(system_error(result, "Failed to set signal mask".to_string()))
        } else {
            Ok(SignalMask { mask: old_mask })
        }
    }
/// Convenience functions for common signal operations
pub mod convenience {
    use super::*;
    
    /// Ignore a signal
    pub fn ignore_signal(signal: Signal) -> ProcessResult<()> {
        let handler = SignalHandler::new();
        handler.register(signal, SignalAction::Ignore)
    /// Restore default signal handling
    pub fn default_signal(signal: Signal) -> ProcessResult<()> {
        let handler = SignalHandler::new();
        handler.register(signal, SignalAction::Default)
    /// Kill a process
    pub fn kill_process(pid: u32) -> ProcessResult<()> {
        send_signal(pid, Signal::Kill)
    /// Terminate a process gracefully
    pub fn terminate_process(pid: u32) -> ProcessResult<()> {
        send_signal(pid, Signal::Terminate)
    /// Interrupt a process
    pub fn interrupt_process(pid: u32) -> ProcessResult<()> {
        send_signal(pid, Signal::Interrupt)
    /// Send hangup signal
    pub fn hangup_process(pid: u32) -> ProcessResult<()> {
        send_signal(pid, Signal::HangUp)
    /// Check if a process is running by sending signal 0
    pub fn is_process_running(pid: u32) -> bool {
        send_signal(pid, Signal::from_raw(0).unwrap_or(Signal::Terminate)).is_ok()
    }
}

/// Signal manager for coordinating signal handling across processes
#[derive(Debug)]
pub struct SignalManager {
    /// Signal handlers per process
    /// Global signal handler
    /// Manager configuration
/// Configuration for signal manager
#[derive(Debug, Clone)]
pub struct SignalManagerConfig {
    /// Enable global signal handling
    /// Timeout for signal operations
    /// Maximum number of pending signals
impl Default for SignalManagerConfig {
    fn default() -> Self {
        Self {
        }
    }
impl SignalManager {
    /// Create a new signal manager
    pub fn new() -> ProcessResult<Self> {
        Self::with_config(SignalManagerConfig::default())
    /// Create a new signal manager with configuration
    pub fn with_config(config: SignalManagerConfig) -> ProcessResult<Self> {
        let global_handler = Arc::new(SignalHandler::new());
        Ok(Self {
        })
    /// Register a signal handler for a specific process
    pub fn register_handler(&self, pid: u32, handler: Arc<SignalHandler>) -> ProcessResult<()> {
        if let Ok(mut handlers) = self.handlers.lock() {
            handlers.insert(pid, handler);
            Ok(())
        } else {
            Err(system_error(-1, "Failed to acquire handlers lock".to_string()))
        }
    }

    /// Unregister a signal handler for a specific process
    pub fn unregister_handler(&self, pid: u32) -> ProcessResult<()> {
        if let Ok(mut handlers) = self.handlers.lock() {
            handlers.remove(&pid);
            Ok(())
        } else {
            Err(system_error(-1, "Failed to acquire handlers lock".to_string()))
        }
    }

    /// Send a signal to a specific process
    pub fn send_signal(&self, pid: u32, signal: Signal) -> ProcessResult<()> {
        if let Ok(handlers) = self.handlers.lock() {
            if let Some(handler) = handlers.get(&pid) {
                handler.handle_signal(signal)
            } else {
                // Use global handler if no specific handler
                self.global_handler.handle_signal(signal)
            }
        } else {
            Err(system_error(-1, "Failed to acquire handlers lock".to_string()))
        }
    }

    /// Get the global signal handler
    pub fn global_handler(&self) -> Arc<SignalHandler> {
        Arc::clone(&self.global_handler)
    }
}

impl Default for SignalManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default SignalManager")
    }
}

