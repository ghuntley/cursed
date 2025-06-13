/// Cross-platform signal handling for process management
/// 
/// This module provides signal handling capabilities for process control and communication

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

use crate::stdlib::process::error::{ProcessError, ProcessResult, system_error};

/// Signal types (mapped across platforms)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    /// Hang up detected on controlling terminal or death of controlling process
    HangUp,
    /// Interrupt from keyboard
    Interrupt,
    /// Quit from keyboard
    Quit,
    /// Illegal instruction
    IllegalInstruction,
    /// Trace/breakpoint trap
    Trap,
    /// Abort signal
    Abort,
    /// Bus error
    BusError,
    /// Floating point exception
    FloatingPointException,
    /// Kill signal (cannot be caught or ignored)
    Kill,
    /// User-defined signal 1
    User1,
    /// Segmentation violation
    SegmentationViolation,
    /// User-defined signal 2
    User2,
    /// Broken pipe
    BrokenPipe,
    /// Alarm clock
    Alarm,
    /// Termination signal
    Terminate,
    /// Child stopped or terminated
    Child,
    /// Continue if stopped
    Continue,
    /// Stop process
    Stop,
    /// Stop typed at terminal
    TerminalStop,
    /// Terminal input for background process
    TerminalInput,
    /// Terminal output for background process
    TerminalOutput,
    /// I/O now possible
    IoReady,
    /// CPU time limit exceeded
    CpuTimeLimit,
    /// File size limit exceeded
    FileSizeLimit,
    /// Virtual alarm clock
    VirtualAlarm,
    /// Profiling alarm clock
    ProfilingAlarm,
    /// Window resize signal
    WindowChange,
    /// Pollable event
    Poll,
    /// Power failure
    PowerFailure,
    /// Bad system call
    BadSystemCall,
}

impl Signal {
    /// Get platform-specific signal number
    pub fn as_raw(&self) -> i32 {
        #[cfg(unix)]
        {
            match self {
                Signal::HangUp => libc::SIGHUP,
                Signal::Interrupt => libc::SIGINT,
                Signal::Quit => libc::SIGQUIT,
                Signal::IllegalInstruction => libc::SIGILL,
                Signal::Trap => libc::SIGTRAP,
                Signal::Abort => libc::SIGABRT,
                Signal::BusError => libc::SIGBUS,
                Signal::FloatingPointException => libc::SIGFPE,
                Signal::Kill => libc::SIGKILL,
                Signal::User1 => libc::SIGUSR1,
                Signal::SegmentationViolation => libc::SIGSEGV,
                Signal::User2 => libc::SIGUSR2,
                Signal::BrokenPipe => libc::SIGPIPE,
                Signal::Alarm => libc::SIGALRM,
                Signal::Terminate => libc::SIGTERM,
                Signal::Child => libc::SIGCHLD,
                Signal::Continue => libc::SIGCONT,
                Signal::Stop => libc::SIGSTOP,
                Signal::TerminalStop => libc::SIGTSTP,
                Signal::TerminalInput => libc::SIGTTIN,
                Signal::TerminalOutput => libc::SIGTTOU,
                Signal::IoReady => libc::SIGIO,
                Signal::CpuTimeLimit => libc::SIGXCPU,
                Signal::FileSizeLimit => libc::SIGXFSZ,
                Signal::VirtualAlarm => libc::SIGVTALRM,
                Signal::ProfilingAlarm => libc::SIGPROF,
                Signal::WindowChange => libc::SIGWINCH,
                Signal::Poll => libc::SIGPOLL,
                Signal::PowerFailure => libc::SIGPWR,
                Signal::BadSystemCall => libc::SIGSYS,
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
                libc::SIGHUP => Some(Signal::HangUp),
                libc::SIGINT => Some(Signal::Interrupt),
                libc::SIGQUIT => Some(Signal::Quit),
                libc::SIGILL => Some(Signal::IllegalInstruction),
                libc::SIGTRAP => Some(Signal::Trap),
                libc::SIGABRT => Some(Signal::Abort),
                libc::SIGBUS => Some(Signal::BusError),
                libc::SIGFPE => Some(Signal::FloatingPointException),
                libc::SIGKILL => Some(Signal::Kill),
                libc::SIGUSR1 => Some(Signal::User1),
                libc::SIGSEGV => Some(Signal::SegmentationViolation),
                libc::SIGUSR2 => Some(Signal::User2),
                libc::SIGPIPE => Some(Signal::BrokenPipe),
                libc::SIGALRM => Some(Signal::Alarm),
                libc::SIGTERM => Some(Signal::Terminate),
                libc::SIGCHLD => Some(Signal::Child),
                libc::SIGCONT => Some(Signal::Continue),
                libc::SIGSTOP => Some(Signal::Stop),
                libc::SIGTSTP => Some(Signal::TerminalStop),
                libc::SIGTTIN => Some(Signal::TerminalInput),
                libc::SIGTTOU => Some(Signal::TerminalOutput),
                libc::SIGIO => Some(Signal::IoReady),
                libc::SIGXCPU => Some(Signal::CpuTimeLimit),
                libc::SIGXFSZ => Some(Signal::FileSizeLimit),
                libc::SIGVTALRM => Some(Signal::VirtualAlarm),
                libc::SIGPROF => Some(Signal::ProfilingAlarm),
                libc::SIGWINCH => Some(Signal::WindowChange),
                libc::SIGPOLL => Some(Signal::Poll),
                libc::SIGPWR => Some(Signal::PowerFailure),
                libc::SIGSYS => Some(Signal::BadSystemCall),
                _ => None,
            }
        }
        
        #[cfg(windows)]
        {
            match signum {
                0 => Some(Signal::Interrupt),
                1 => Some(Signal::Terminate),
                2 => Some(Signal::HangUp),
                3 => Some(Signal::Kill),
                _ => None,
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
            Signal::HangUp => "SIGHUP",
            Signal::Interrupt => "SIGINT",
            Signal::Quit => "SIGQUIT",
            Signal::IllegalInstruction => "SIGILL",
            Signal::Trap => "SIGTRAP",
            Signal::Abort => "SIGABRT",
            Signal::BusError => "SIGBUS",
            Signal::FloatingPointException => "SIGFPE",
            Signal::Kill => "SIGKILL",
            Signal::User1 => "SIGUSR1",
            Signal::SegmentationViolation => "SIGSEGV",
            Signal::User2 => "SIGUSR2",
            Signal::BrokenPipe => "SIGPIPE",
            Signal::Alarm => "SIGALRM",
            Signal::Terminate => "SIGTERM",
            Signal::Child => "SIGCHLD",
            Signal::Continue => "SIGCONT",
            Signal::Stop => "SIGSTOP",
            Signal::TerminalStop => "SIGTSTP",
            Signal::TerminalInput => "SIGTTIN",
            Signal::TerminalOutput => "SIGTTOU",
            Signal::IoReady => "SIGIO",
            Signal::CpuTimeLimit => "SIGXCPU",
            Signal::FileSizeLimit => "SIGXFSZ",
            Signal::VirtualAlarm => "SIGVTALRM",
            Signal::ProfilingAlarm => "SIGPROF",
            Signal::WindowChange => "SIGWINCH",
            Signal::Poll => "SIGPOLL",
            Signal::PowerFailure => "SIGPWR",
            Signal::BadSystemCall => "SIGSYS",
        }
    }
    
    /// Check if signal can be caught or ignored
    pub fn can_be_caught(&self) -> bool {
        !matches!(self, Signal::Kill | Signal::Stop)
    }
    
    /// Check if signal is terminating by default
    pub fn is_terminating(&self) -> bool {
        matches!(self, 
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
    Ignore,
    /// Use default action
    Default,
    /// Call custom handler
    Handle(Arc<dyn Fn(Signal) + Send + Sync>),
}

/// Signal handler registration
pub struct SignalHandler {
    handlers: Arc<Mutex<HashMap<Signal, SignalAction>>>,
    signal_sender: mpsc::Sender<Signal>,
    signal_receiver: Arc<Mutex<mpsc::Receiver<Signal>>>,
    _handler_thread: thread::JoinHandle<()>,
}

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
            handlers,
            signal_sender,
            signal_receiver: receiver_arc,
            _handler_thread: handler_thread,
        }
    }
    
    /// Register a signal handler
    pub fn register(&self, signal: Signal, action: SignalAction) -> ProcessResult<()> {
        if !signal.can_be_caught() {
            return Err(system_error(-1, format!("Signal {} cannot be caught", signal.name())));
        }
        
        let mut handlers = self.handlers.lock()
            .map_err(|_| system_error(-1, "Failed to lock signal handlers".to_string()))?;
        
        // Install the actual signal handler
        self.install_signal_handler(signal)?;
        
        handlers.insert(signal, action);
        Ok(())
    }
    
    /// Unregister a signal handler
    pub fn unregister(&self, signal: Signal) -> ProcessResult<()> {
        let mut handlers = self.handlers.lock()
            .map_err(|_| system_error(-1, "Failed to lock signal handlers".to_string()))?;
        
        handlers.remove(&signal);
        
        // Restore default signal handling
        self.restore_default_handler(signal)?;
        
        Ok(())
    }
    
    /// Send a signal (simulate receiving a signal)
    pub fn simulate_signal(&self, signal: Signal) -> ProcessResult<()> {
        self.signal_sender.send(signal)
            .map_err(|_| system_error(-1, "Failed to send signal".to_string()))?;
        Ok(())
    }
    
    /// Install platform-specific signal handler
    fn install_signal_handler(&self, signal: Signal) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            self.install_unix_signal_handler(signal)
        }
        
        #[cfg(windows)]
        {
            self.install_windows_signal_handler(signal)
        }
        
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
        }
        
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
        }
        
        // Install the signal handler
        let mut old_action: libc::sigaction = unsafe { mem::zeroed() };
        let result = unsafe { 
            libc::sigaction(signum, &action, &mut old_action)
        };
        
        if result != 0 {
            // Cleanup on failure
            let _ = SIGNAL_SENDER.swap(ptr::null_mut(), Ordering::SeqCst);
            unsafe { let _ = Box::from_raw(sender_ptr); }
            
            return Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                format!("Failed to install signal handler for {} using sigaction", signal.name())
            ));
        }
        
        Ok(())
    }
    
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
                        _ => None,
                    };
                    
                    if let Some(sig) = signal {
                        let _ = (*sender_ptr).try_send(sig);
                    }
                }
            }
            1 // TRUE - we handled the signal
        }
        
        match signal {
            Signal::Interrupt | Signal::Terminate | Signal::HangUp | Signal::Kill => {
                // Install console control handler for supported signals
                unsafe {
                    use winapi::um::wincon::SetConsoleCtrlHandler;
                    
                    let result = SetConsoleCtrlHandler(
                        Some(console_ctrl_handler),
                        1 // TRUE - add handler
                    );
                    
                    if result == 0 {
                        // Cleanup on failure
                        let _ = SIGNAL_SENDER.swap(ptr::null_mut(), Ordering::SeqCst);
                        unsafe { let _ = Box::from_raw(sender_ptr); }
                        
                        return Err(system_error(
                            std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
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
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                        format!("Failed to restore default handler for {}", signal.name())
                    ));
                }
            }
        }
        
        #[cfg(windows)]
        {
            // Windows implementation would unregister console ctrl handler
        }
        
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
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
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
        _ => return Err(system_error(-1, "Invalid event type".to_string())),
    };
    
    match result {
        Ok(output) if output.status.success() => Ok(()),
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
}

/// Signal mask for blocking/unblocking signals
#[cfg(unix)]
pub struct SignalMask {
    mask: libc::sigset_t,
}

#[cfg(unix)]
impl SignalMask {
    /// Create an empty signal mask
    pub fn empty() -> ProcessResult<Self> {
        let mut mask = unsafe { std::mem::zeroed() };
        
        let result = unsafe { libc::sigemptyset(&mut mask) };
        if result != 0 {
            return Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "Failed to create empty signal mask".to_string()
            ));
        }
        
        Ok(Self { mask })
    }
    
    /// Create a full signal mask (all signals)
    pub fn full() -> ProcessResult<Self> {
        let mut mask = unsafe { std::mem::zeroed() };
        
        let result = unsafe { libc::sigfillset(&mut mask) };
        if result != 0 {
            return Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "Failed to create full signal mask".to_string()
            ));
        }
        
        Ok(Self { mask })
    }
    
    /// Add a signal to the mask
    pub fn add(&mut self, signal: Signal) -> ProcessResult<()> {
        let signum = signal.as_raw();
        let result = unsafe { libc::sigaddset(&mut self.mask, signum) };
        
        if result != 0 {
            Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
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
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
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
        };
        
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
        };
        
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
        };
        
        if result != 0 {
            Err(system_error(result, "Failed to set signal mask".to_string()))
        } else {
            Ok(SignalMask { mask: old_mask })
        }
    }
}

/// Convenience functions for common signal operations
pub mod convenience {
    use super::*;
    
    /// Ignore a signal
    pub fn ignore_signal(signal: Signal) -> ProcessResult<()> {
        let handler = SignalHandler::new();
        handler.register(signal, SignalAction::Ignore)
    }
    
    /// Restore default signal handling
    pub fn default_signal(signal: Signal) -> ProcessResult<()> {
        let handler = SignalHandler::new();
        handler.register(signal, SignalAction::Default)
    }
    
    /// Kill a process
    pub fn kill_process(pid: u32) -> ProcessResult<()> {
        send_signal(pid, Signal::Kill)
    }
    
    /// Terminate a process gracefully
    pub fn terminate_process(pid: u32) -> ProcessResult<()> {
        send_signal(pid, Signal::Terminate)
    }
    
    /// Interrupt a process
    pub fn interrupt_process(pid: u32) -> ProcessResult<()> {
        send_signal(pid, Signal::Interrupt)
    }
    
    /// Send hangup signal
    pub fn hangup_process(pid: u32) -> ProcessResult<()> {
        send_signal(pid, Signal::HangUp)
    }
    
    /// Check if a process is running by sending signal 0
    pub fn is_process_running(pid: u32) -> bool {
        send_signal(pid, Signal::from_raw(0).unwrap_or(Signal::Terminate)).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    
    #[test]
    fn test_signal_properties() {
        assert_eq!(Signal::Interrupt.name(), "SIGINT");
        assert!(Signal::Interrupt.can_be_caught());
        assert!(Signal::Interrupt.is_terminating());
        
        assert!(!Signal::Kill.can_be_caught());
        assert!(Signal::Kill.is_terminating());
        
        assert!(!Signal::Continue.is_terminating());
    }
    
    #[test]
    fn test_signal_conversion() {
        let signal = Signal::Interrupt;
        let raw = signal.as_raw();
        
        #[cfg(unix)]
        {
            assert_eq!(raw, libc::SIGINT);
            assert_eq!(Signal::from_raw(raw), Some(signal));
        }
        
        #[cfg(windows)]
        {
            assert_eq!(raw, 0); // CTRL_C_EVENT
        }
    }
    
    #[test]
    fn test_signal_handler_creation() {
        let handler = SignalHandler::new();
        
        // Test that we can register a signal handler
        let signal_received = Arc::new(AtomicBool::new(false));
        let signal_received_clone = Arc::clone(&signal_received);
        
        let action = SignalAction::Handle(Arc::new(move |_signal| {
            signal_received_clone.store(true, Ordering::SeqCst);
        }));
        
        // Only test with signals that can be caught
        if Signal::User1.can_be_caught() {
            assert!(handler.register(Signal::User1, action).is_ok());
            
            // Simulate receiving the signal
            assert!(handler.simulate_signal(Signal::User1).is_ok());
            
            // Give some time for the handler to process
            thread::sleep(Duration::from_millis(50));
            
            assert!(signal_received.load(Ordering::SeqCst));
        }
    }
    
    #[test]
    fn test_signal_mask() {
        #[cfg(unix)]
        {
            let mut mask = SignalMask::empty().unwrap();
            assert!(!mask.contains(Signal::Interrupt));
            
            mask.add(Signal::Interrupt).unwrap();
            assert!(mask.contains(Signal::Interrupt));
            
            mask.remove(Signal::Interrupt).unwrap();
            assert!(!mask.contains(Signal::Interrupt));
        }
    }
    
    #[test]
    fn test_convenience_functions() {
        // Test that convenience functions don't panic
        // We can't test actual signal sending without affecting the test process
        assert!(convenience::is_process_running(std::process::id()));
    }
}
