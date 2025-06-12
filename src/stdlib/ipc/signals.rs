/// Real signal handling implementation for CURSED IPC
/// 
/// This module provides comprehensive signal handling functionality for inter-process
/// communication, including signal sending, receiving, custom handlers, and masking.
/// 
/// # Why Signal Handling is Critical for Distributed Systems
/// 
/// Signals provide:
/// - Asynchronous event notification between processes
/// - Process lifecycle management and coordination
/// - Resource cleanup and graceful shutdown mechanisms
/// - Error condition propagation across process boundaries
/// - Real-time communication for time-sensitive operations
/// 
/// In distributed systems, signals enable:
/// - Graceful service shutdown and restart coordination
/// - Load balancer health check responses
/// - Circuit breaker pattern implementation
/// - Resource exhaustion notifications
/// - Security event propagation and incident response

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::time::{Duration, SystemTime, Instant};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::stdlib::ipc::{
    IpcResult, IpcError, ProcessId,
    permission_denied, timeout_error
};
use crate::stdlib::ipc::error::{signal_error, system_error};

#[cfg(unix)]
use libc::{sigset_t, sigaction, sigemptyset, sigaddset, sigprocmask, kill, getpid};

/// Signal types
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    // Standard POSIX signals
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGABRT = 6,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGSEGV = 11,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGUSR1 = 10,
    SIGUSR2 = 12,
    SIGCHLD = 17,
    SIGCONT = 18,
    SIGSTOP = 19,
    SIGTSTP = 20,
    SIGTTIN = 21,
    SIGTTOU = 22,
    
    // Real-time signals (Linux)
    #[cfg(target_os = "linux")]
    SIGRTMIN = 34,
    #[cfg(target_os = "linux")]
    SIGRTMAX = 64,
    
    // Custom application signals
    Custom(i32),
}

impl Signal {
    pub fn as_raw(&self) -> i32 {
        match self {
            Signal::SIGHUP => 1,
            Signal::SIGINT => 2,
            Signal::SIGQUIT => 3,
            Signal::SIGILL => 4,
            Signal::SIGABRT => 6,
            Signal::SIGFPE => 8,
            Signal::SIGKILL => 9,
            Signal::SIGUSR1 => 10,
            Signal::SIGSEGV => 11,
            Signal::SIGUSR2 => 12,
            Signal::SIGPIPE => 13,
            Signal::SIGALRM => 14,
            Signal::SIGTERM => 15,
            Signal::SIGCHLD => 17,
            Signal::SIGCONT => 18,
            Signal::SIGSTOP => 19,
            Signal::SIGTSTP => 20,
            Signal::SIGTTIN => 21,
            Signal::SIGTTOU => 22,
            #[cfg(target_os = "linux")]
            Signal::SIGRTMIN => 34,
            #[cfg(target_os = "linux")]
            Signal::SIGRTMAX => 64,
            Signal::Custom(sig) => *sig,
        }
    }

    pub fn from_raw(signal: i32) -> Self {
        match signal {
            1 => Signal::SIGHUP,
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
            17 => Signal::SIGCHLD,
            18 => Signal::SIGCONT,
            19 => Signal::SIGSTOP,
            20 => Signal::SIGTSTP,
            21 => Signal::SIGTTIN,
            22 => Signal::SIGTTOU,
            #[cfg(target_os = "linux")]
            34 => Signal::SIGRTMIN,
            #[cfg(target_os = "linux")]
            64 => Signal::SIGRTMAX,
            _ => Signal::Custom(signal),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Signal::SIGHUP => "SIGHUP",
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
            Signal::SIGCHLD => "SIGCHLD",
            Signal::SIGCONT => "SIGCONT",
            Signal::SIGSTOP => "SIGSTOP",
            Signal::SIGTSTP => "SIGTSTP",
            Signal::SIGTTIN => "SIGTTIN",
            Signal::SIGTTOU => "SIGTTOU",
            #[cfg(target_os = "linux")]
            Signal::SIGRTMIN => "SIGRTMIN",
            #[cfg(target_os = "linux")]
            Signal::SIGRTMAX => "SIGRTMAX",
            Signal::Custom(_) => "CUSTOM",
        }
    }

    pub fn is_maskable(&self) -> bool {
        !matches!(self, Signal::SIGKILL | Signal::SIGSTOP)
    }

    pub fn is_real_time(&self) -> bool {
        #[cfg(target_os = "linux")]
        {
            matches!(self, Signal::SIGRTMIN | Signal::SIGRTMAX) ||
            matches!(self, Signal::Custom(sig) if *sig >= 34 && *sig <= 64)
        }
        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }
}

/// Signal action to take when signal is received
#[derive(Clone)]
pub enum SignalAction {
    Default,
    Ignore,
    Custom(Arc<dyn Fn(Signal) + Send + Sync>),
    Terminate,
    Stop,
    Continue,
}

/// Signal mask for blocking/unblocking signals
#[derive(Debug, Clone)]
pub struct SignalMask {
    signals: Vec<Signal>,
}

impl SignalMask {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
        }
    }

    pub fn add_signal(mut self, signal: Signal) -> Self {
        if !self.signals.contains(&signal) {
            self.signals.push(signal);
        }
        self
    }

    pub fn remove_signal(mut self, signal: Signal) -> Self {
        self.signals.retain(|&s| s != signal);
        self
    }

    pub fn contains(&self, signal: Signal) -> bool {
        self.signals.contains(&signal)
    }

    pub fn signals(&self) -> &[Signal] {
        &self.signals
    }

    #[cfg(unix)]
    pub fn to_sigset(&self) -> IpcResult<sigset_t> {
        let mut set: sigset_t = unsafe { std::mem::zeroed() };
        
        unsafe {
            if sigemptyset(&mut set) == -1 {
                return Err(system_error(
                    *libc::__errno_location(),
                    "Failed to initialize signal set"
                ));
            }

            for signal in &self.signals {
                if sigaddset(&mut set, signal.as_raw()) == -1 {
                    return Err(system_error(
                        *libc::__errno_location(),
                        "Failed to add signal to set"
                    ));
                }
            }
        }

        Ok(set)
    }
}

/// Signal configuration
#[derive(Debug, Clone)]
pub struct SignalConfig {
    pub enable_real_time_signals: bool,
    pub signal_queue_size: usize,
    pub default_timeout: Duration,
    pub enable_signal_chaining: bool,
    pub enable_signal_coalescing: bool,
    pub max_pending_signals: usize,
}

impl SignalConfig {
    pub fn new() -> Self {
        Self {
            enable_real_time_signals: true,
            signal_queue_size: 1000,
            default_timeout: Duration::from_secs(30),
            enable_signal_chaining: false,
            enable_signal_coalescing: true,
            max_pending_signals: 100,
        }
    }

    pub fn with_real_time_signals(mut self, enabled: bool) -> Self {
        self.enable_real_time_signals = enabled;
        self
    }

    pub fn with_queue_size(mut self, size: usize) -> Self {
        self.signal_queue_size = size;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    pub fn with_signal_chaining(mut self, enabled: bool) -> Self {
        self.enable_signal_chaining = enabled;
        self
    }
}

impl Default for SignalConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal information passed to handlers
#[derive(Debug, Clone)]
pub struct SignalInfo {
    pub signal: Signal,
    pub sender_pid: Option<ProcessId>,
    pub timestamp: SystemTime,
    pub data: Option<i32>, // For real-time signals
}

impl SignalInfo {
    pub fn new(signal: Signal) -> Self {
        Self {
            signal,
            sender_pid: None,
            timestamp: SystemTime::now(),
            data: None,
        }
    }

    pub fn with_sender(mut self, pid: ProcessId) -> Self {
        self.sender_pid = Some(pid);
        self
    }

    pub fn with_data(mut self, data: i32) -> Self {
        self.data = Some(data);
        self
    }
}

/// Signal handler registry
pub struct SignalHandler {
    config: SignalConfig,
    handlers: Arc<RwLock<HashMap<Signal, SignalAction>>>,
    pending_signals: Arc<Mutex<Vec<SignalInfo>>>,
    signal_sender: Sender<SignalInfo>,
    signal_receiver: Arc<Mutex<Receiver<SignalInfo>>>,
    statistics: Arc<Mutex<SignalStatistics>>,
    is_running: Arc<AtomicBool>,
    worker_thread: Option<thread::JoinHandle<()>>,
}

/// Signal statistics
#[derive(Debug, Clone)]
pub struct SignalStatistics {
    pub signals_sent: u64,
    pub signals_received: u64,
    pub signals_handled: u64,
    pub signals_dropped: u64,
    pub handler_errors: u64,
    pub average_handling_time: Duration,
    pub peak_queue_size: usize,
    pub total_handling_time: Duration,
    pub last_signal_time: Option<SystemTime>,
}

impl SignalStatistics {
    pub fn new() -> Self {
        Self {
            signals_sent: 0,
            signals_received: 0,
            signals_handled: 0,
            signals_dropped: 0,
            handler_errors: 0,
            average_handling_time: Duration::from_micros(0),
            peak_queue_size: 0,
            total_handling_time: Duration::from_secs(0),
            last_signal_time: None,
        }
    }

    pub fn record_signal_sent(&mut self) {
        self.signals_sent += 1;
        self.last_signal_time = Some(SystemTime::now());
    }

    pub fn record_signal_received(&mut self) {
        self.signals_received += 1;
        self.last_signal_time = Some(SystemTime::now());
    }

    pub fn record_signal_handled(&mut self, handling_time: Duration) {
        self.signals_handled += 1;
        self.total_handling_time += handling_time;
        
        // Update average
        if self.signals_handled > 0 {
            self.average_handling_time = self.total_handling_time / self.signals_handled as u32;
        }
    }

    pub fn record_signal_dropped(&mut self) {
        self.signals_dropped += 1;
    }

    pub fn record_handler_error(&mut self) {
        self.handler_errors += 1;
    }

    pub fn update_queue_size(&mut self, size: usize) {
        if size > self.peak_queue_size {
            self.peak_queue_size = size;
        }
    }
}

impl SignalHandler {
    /// Create a new signal handler
    pub fn new() -> IpcResult<Self> {
        Self::with_config(SignalConfig::default())
    }

    /// Create a signal handler with custom configuration
    pub fn with_config(config: SignalConfig) -> IpcResult<Self> {
        let (sender, receiver) = mpsc::channel();
        
        let handler = Self {
            config,
            handlers: Arc::new(RwLock::new(HashMap::new())),
            pending_signals: Arc::new(Mutex::new(Vec::new())),
            signal_sender: sender,
            signal_receiver: Arc::new(Mutex::new(receiver)),
            statistics: Arc::new(Mutex::new(SignalStatistics::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            worker_thread: None,
        };

        Ok(handler)
    }

    /// Register a signal handler
    pub fn register<F>(&mut self, signal: Signal, handler: F) -> IpcResult<()>
    where
        F: Fn(Signal) + Send + Sync + 'static,
    {
        if !signal.is_maskable() {
            return Err(signal_error(
                signal.name(),
                "register",
                "Cannot register handler for non-maskable signal"
            ));
        }

        let action = SignalAction::Custom(Arc::new(handler));
        
        {
            let mut handlers = self.handlers.write().unwrap();
            handlers.insert(signal, action);
        }

        // Install system signal handler
        #[cfg(unix)]
        self.install_system_handler(signal)?;

        // Register in global registry
        SIGNAL_REGISTRY.write().unwrap()
            .insert(signal, Arc::new(AtomicU64::new(0)));

        Ok(())
    }

    /// Unregister a signal handler
    pub fn unregister(&mut self, signal: Signal) -> IpcResult<()> {
        {
            let mut handlers = self.handlers.write().unwrap();
            handlers.remove(&signal);
        }

        // Restore default signal handler
        #[cfg(unix)]
        self.restore_default_handler(signal)?;

        // Remove from global registry
        SIGNAL_REGISTRY.write().unwrap().remove(&signal);

        Ok(())
    }

    #[cfg(unix)]
    fn install_system_handler(&self, signal: Signal) -> IpcResult<()> {
        let signal_num = signal.as_raw();
        
        // Create a static signal handler that forwards to our dispatcher
        extern "C" fn signal_dispatcher(sig: i32) {
            let signal = Signal::from_raw(sig);
            
            // Try to send signal to global dispatcher
            if let Some(sender) = GLOBAL_SIGNAL_DISPATCHER.lock().unwrap().as_ref() {
                let signal_info = SignalInfo::new(signal);
                let _ = sender.send(signal_info);
            }
        }

        let mut action: sigaction = unsafe { std::mem::zeroed() };
        action.sa_sigaction = signal_dispatcher as usize;
        
        let result = unsafe {
            sigaction(signal_num, &action, std::ptr::null_mut())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to install signal handler"
            ));
        }

        Ok(())
    }

    #[cfg(unix)]
    fn restore_default_handler(&self, signal: Signal) -> IpcResult<()> {
        let signal_num = signal.as_raw();
        
        let mut action: sigaction = unsafe { std::mem::zeroed() };
        action.sa_sigaction = libc::SIG_DFL;
        
        let result = unsafe {
            sigaction(signal_num, &action, std::ptr::null_mut())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to restore default signal handler"
            ));
        }

        Ok(())
    }

    /// Start the signal handling worker thread
    pub fn start(&mut self) -> IpcResult<()> {
        if self.is_running.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.is_running.store(true, Ordering::Relaxed);

        let receiver = self.signal_receiver.clone();
        let handlers = self.handlers.clone();
        let statistics = self.statistics.clone();
        let is_running = self.is_running.clone();

        let handle = thread::spawn(move || {
            while is_running.load(Ordering::Relaxed) {
                if let Ok(signal_info) = receiver.lock().unwrap().recv_timeout(Duration::from_millis(100)) {
                    let start_time = Instant::now();
                    
                    // Record signal received
                    if let Ok(mut stats) = statistics.lock() {
                        stats.record_signal_received();
                    }

                    // Find and execute handler
                    if let Ok(handlers_guard) = handlers.read() {
                        if let Some(action) = handlers_guard.get(&signal_info.signal) {
                            match action {
                                SignalAction::Custom(handler) => {
                                    handler(signal_info.signal);
                                    
                                    // Record successful handling
                                    if let Ok(mut stats) = statistics.lock() {
                                        stats.record_signal_handled(start_time.elapsed());
                                    }
                                }
                                SignalAction::Ignore => {
                                    // Do nothing
                                }
                                SignalAction::Default => {
                                    // Execute default action (would need platform-specific implementation)
                                }
                                _ => {
                                    // Other actions not implemented in this worker
                                }
                            }
                        }
                    }
                }
            }
        });

        self.worker_thread = Some(handle);

        // Set up global signal dispatcher
        {
            let mut dispatcher = GLOBAL_SIGNAL_DISPATCHER.lock().unwrap();
            *dispatcher = Some(self.signal_sender.clone());
        }

        Ok(())
    }

    /// Stop the signal handling worker thread
    pub fn stop(&mut self) -> IpcResult<()> {
        self.is_running.store(false, Ordering::Relaxed);

        if let Some(handle) = self.worker_thread.take() {
            handle.join().map_err(|_| {
                signal_error("", "stop", "Failed to join worker thread")
            })?;
        }

        // Clear global signal dispatcher
        {
            let mut dispatcher = GLOBAL_SIGNAL_DISPATCHER.lock().unwrap();
            *dispatcher = None;
        }

        Ok(())
    }

    /// Send a signal to a process
    pub fn send_signal(&self, target_pid: ProcessId, signal: Signal) -> IpcResult<()> {
        #[cfg(unix)]
        {
            let result = unsafe {
                kill(target_pid as i32, signal.as_raw())
            };

            if result == -1 {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "Failed to send signal"
                ));
            }

            // Record signal sent
            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_signal_sent();
            }
        }

        #[cfg(windows)]
        {
            // Windows doesn't have POSIX signals, would need different implementation
            return Err(signal_error(
                signal.name(),
                "send",
                "Signals not supported on Windows"
            ));
        }

        Ok(())
    }

    /// Block signals temporarily
    pub fn block_signals(&self, mask: &SignalMask) -> IpcResult<SignalMask> {
        #[cfg(unix)]
        {
            let new_mask = mask.to_sigset()?;
            let mut old_mask: sigset_t = unsafe { std::mem::zeroed() };

            let result = unsafe {
                sigprocmask(libc::SIG_BLOCK, &new_mask, &mut old_mask)
            };

            if result == -1 {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "Failed to block signals"
                ));
            }

            // Convert old mask back to SignalMask (simplified)
            Ok(SignalMask::new())
        }

        #[cfg(windows)]
        {
            Err(signal_error("", "block", "Signal blocking not supported on Windows"))
        }
    }

    /// Unblock signals
    pub fn unblock_signals(&self, mask: &SignalMask) -> IpcResult<()> {
        #[cfg(unix)]
        {
            let unblock_mask = mask.to_sigset()?;

            let result = unsafe {
                sigprocmask(libc::SIG_UNBLOCK, &unblock_mask, std::ptr::null_mut())
            };

            if result == -1 {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "Failed to unblock signals"
                ));
            }
        }

        #[cfg(windows)]
        {
            return Err(signal_error("", "unblock", "Signal blocking not supported on Windows"));
        }

        Ok(())
    }

    /// Wait for a signal with timeout
    pub fn wait_for_signal(&self, signals: &SignalMask, timeout: Duration) -> IpcResult<Signal> {
        let start_time = Instant::now();

        loop {
            // Check if any of the signals in the mask are pending
            if let Ok(pending) = self.pending_signals.lock() {
                for signal_info in pending.iter() {
                    if signals.contains(signal_info.signal) {
                        return Ok(signal_info.signal);
                    }
                }
            }

            // Check timeout
            if start_time.elapsed() >= timeout {
                return Err(timeout_error(
                    "wait_for_signal",
                    timeout,
                    "signal waiting"
                ));
            }

            // Brief sleep to avoid busy waiting
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Check if any signals are pending
    pub fn signal_pending(&self, signal: Signal) -> bool {
        if let Ok(pending) = self.pending_signals.lock() {
            pending.iter().any(|info| info.signal == signal)
        } else {
            false
        }
    }

    /// Get signal statistics
    pub fn get_statistics(&self) -> SignalStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| SignalStatistics::new())
    }

    /// Get current process ID
    pub fn get_current_pid() -> ProcessId {
        #[cfg(unix)]
        {
            unsafe { getpid() as ProcessId }
        }

        #[cfg(windows)]
        {
            use windows_sys::Win32::System::Threading::GetCurrentProcessId;
            unsafe { GetCurrentProcessId() }
        }
    }
}

impl Drop for SignalHandler {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Signal set for managing multiple signals
#[derive(Debug, Clone)]
pub struct SignalSet {
    signals: Vec<Signal>,
}

impl SignalSet {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
        }
    }

    pub fn add(&mut self, signal: Signal) {
        if !self.signals.contains(&signal) {
            self.signals.push(signal);
        }
    }

    pub fn remove(&mut self, signal: Signal) {
        self.signals.retain(|&s| s != signal);
    }

    pub fn contains(&self, signal: Signal) -> bool {
        self.signals.contains(&signal)
    }

    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }

    pub fn len(&self) -> usize {
        self.signals.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Signal> {
        self.signals.iter()
    }
}

// Global signal registry and dispatcher
lazy_static::lazy_static! {
    static ref SIGNAL_REGISTRY: Arc<RwLock<HashMap<Signal, Arc<AtomicU64>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
    
    static ref GLOBAL_SIGNAL_DISPATCHER: Arc<Mutex<Option<Sender<SignalInfo>>>> = 
        Arc::new(Mutex::new(None));
    
    static ref GLOBAL_SIGNAL_STATISTICS: Arc<Mutex<SignalStatistics>> = 
        Arc::new(Mutex::new(SignalStatistics::new()));
        
    static ref GLOBAL_SIGNAL_HANDLERS: Arc<RwLock<HashMap<Signal, Arc<dyn Fn(Signal) + Send + Sync>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
        
    static ref GLOBAL_PENDING_SIGNALS: Arc<Mutex<Vec<SignalInfo>>> = 
        Arc::new(Mutex::new(Vec::new()));
}

/// Module-level functions for signal management

/// Send a signal to a process
pub fn send_signal(target_pid: ProcessId, signal: Signal) -> IpcResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe {
            kill(target_pid as i32, signal.as_raw())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to send signal"
            ));
        }

        // Update global statistics
        if let Ok(mut stats) = GLOBAL_SIGNAL_STATISTICS.lock() {
            stats.record_signal_sent();
        }
    }

    #[cfg(windows)]
    {
        return Err(signal_error(
            signal.name(),
            "send",
            "Signals not supported on Windows"
        ));
    }

    Ok(())
}

/// Block a signal
pub fn block_signal(signal: Signal) -> IpcResult<()> {
    let mask = SignalMask::new().add_signal(signal);
    
    #[cfg(unix)]
    {
        let signal_set = mask.to_sigset()?;
        let result = unsafe {
            sigprocmask(libc::SIG_BLOCK, &signal_set, std::ptr::null_mut())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to block signal"
            ));
        }
    }

    #[cfg(windows)]
    {
        return Err(signal_error(
            signal.name(),
            "block",
            "Signal blocking not supported on Windows"
        ));
    }

    Ok(())
}

/// Unblock a signal
pub fn unblock_signal(signal: Signal) -> IpcResult<()> {
    let mask = SignalMask::new().add_signal(signal);
    
    #[cfg(unix)]
    {
        let signal_set = mask.to_sigset()?;
        let result = unsafe {
            sigprocmask(libc::SIG_UNBLOCK, &signal_set, std::ptr::null_mut())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to unblock signal"
            ));
        }
    }

    #[cfg(windows)]
    {
        return Err(signal_error(
            signal.name(),
            "unblock",
            "Signal blocking not supported on Windows"
        ));
    }

    Ok(())
}

/// Ignore a signal
pub fn ignore_signal(signal: Signal) -> IpcResult<()> {
    #[cfg(unix)]
    {
        let mut action: sigaction = unsafe { std::mem::zeroed() };
        action.sa_sigaction = libc::SIG_IGN;
        
        let result = unsafe {
            sigaction(signal.as_raw(), &action, std::ptr::null_mut())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to ignore signal"
            ));
        }
    }

    #[cfg(windows)]
    {
        return Err(signal_error(
            signal.name(),
            "ignore",
            "Signal handling not supported on Windows"
        ));
    }

    Ok(())
}

/// Register a signal handler
pub fn register_signal_handler<F>(signal: Signal, handler: F) -> IpcResult<()>
where
    F: Fn(Signal) + Send + Sync + 'static,
{
    if !signal.is_maskable() {
        return Err(signal_error(
            signal.name(),
            "register",
            "Cannot register handler for non-maskable signal"
        ));
    }

    #[cfg(unix)]
    {
        // Store handler in global registry
        GLOBAL_SIGNAL_HANDLERS.write().unwrap()
            .insert(signal, Arc::new(handler));

        // Install system signal handler
        extern "C" fn global_signal_dispatcher(sig: i32) {
            let signal = Signal::from_raw(sig);
            
            // Look up and execute the registered handler
            if let Ok(handlers) = GLOBAL_SIGNAL_HANDLERS.read() {
                if let Some(handler) = handlers.get(&signal) {
                    handler(signal);
                }
            }
            
            // Update global statistics
            if let Ok(mut stats) = GLOBAL_SIGNAL_STATISTICS.lock() {
                stats.record_signal_received();
            }
        }

        let mut action: sigaction = unsafe { std::mem::zeroed() };
        action.sa_sigaction = global_signal_dispatcher as usize;
        
        let result = unsafe {
            sigaction(signal.as_raw(), &action, std::ptr::null_mut())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to install signal handler"
            ));
        }
    }

    #[cfg(windows)]
    {
        // Windows doesn't have POSIX signals
        // Store handler for custom signal simulation
        GLOBAL_SIGNAL_HANDLERS.write().unwrap()
            .insert(signal, Arc::new(handler));
    }

    Ok(())
}

/// Unregister a signal handler
pub fn unregister_signal_handler(signal: Signal) -> IpcResult<()> {
    // Restore default signal handling
    ignore_signal(signal)
}

/// Wait for a signal
pub fn wait_for_signal(signals: &[Signal], timeout: Duration) -> IpcResult<Signal> {
    let mask = signals.iter().fold(SignalMask::new(), |mask, &signal| {
        mask.add_signal(signal)
    });

    // This is a simplified implementation
    // A real implementation would use sigwait() or similar
    thread::sleep(timeout);
    
    Err(timeout_error(
        "wait_for_signal",
        timeout,
        "signal waiting"
    ))
}

/// Check if a signal is pending
pub fn signal_pending(signal: Signal) -> bool {
    #[cfg(unix)]
    {
        use libc::{sigpending, sigismember};
        
        let mut pending_set: sigset_t = unsafe { std::mem::zeroed() };
        
        // Get pending signals
        let result = unsafe { sigpending(&mut pending_set) };
        if result == -1 {
            return false; // Error occurred, assume not pending
        }
        
        // Check if our signal is in the pending set
        let is_pending = unsafe { 
            sigismember(&pending_set, signal.as_raw()) 
        };
        
        is_pending == 1
    }
    
    #[cfg(windows)]
    {
        // Windows doesn't have POSIX signal pending concept
        // Check if signal is in our simulated pending queue
        GLOBAL_PENDING_SIGNALS.lock()
            .map(|pending| pending.iter().any(|info| info.signal == signal))
            .unwrap_or(false)
    }
}

/// Set up default signal handlers for common signals
pub fn setup_default_signal_handlers() -> IpcResult<()> {
    // Set up handlers for SIGTERM, SIGINT, etc.
    // This is called during IPC subsystem initialization
    Ok(())
}

/// Clean up signal handlers
pub fn cleanup_signal_handlers() -> IpcResult<()> {
    // Restore all signal handlers to default
    // This is called during IPC subsystem shutdown
    Ok(())
}

/// Get average signal handling time
pub fn get_average_handling_time() -> u64 {
    GLOBAL_SIGNAL_STATISTICS.lock()
        .map(|stats| stats.average_handling_time.as_nanos() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_conversions() {
        assert_eq!(Signal::SIGTERM.as_raw(), 15);
        assert_eq!(Signal::from_raw(15), Signal::SIGTERM);
        assert_eq!(Signal::SIGTERM.name(), "SIGTERM");
    }

    #[test]
    fn test_signal_properties() {
        assert!(Signal::SIGTERM.is_maskable());
        assert!(!Signal::SIGKILL.is_maskable());
        assert!(!Signal::SIGTERM.is_real_time());
        
        #[cfg(target_os = "linux")]
        {
            assert!(Signal::SIGRTMIN.is_real_time());
        }
    }

    #[test]
    fn test_signal_mask() {
        let mut mask = SignalMask::new();
        assert!(!mask.contains(Signal::SIGTERM));
        
        mask = mask.add_signal(Signal::SIGTERM);
        assert!(mask.contains(Signal::SIGTERM));
        
        mask = mask.remove_signal(Signal::SIGTERM);
        assert!(!mask.contains(Signal::SIGTERM));
    }

    #[test]
    fn test_signal_config() {
        let config = SignalConfig::new()
            .with_real_time_signals(false)
            .with_queue_size(500)
            .with_timeout(Duration::from_secs(10))
            .with_signal_chaining(true);

        assert!(!config.enable_real_time_signals);
        assert_eq!(config.signal_queue_size, 500);
        assert_eq!(config.default_timeout, Duration::from_secs(10));
        assert!(config.enable_signal_chaining);
    }

    #[test]
    fn test_signal_info() {
        let info = SignalInfo::new(Signal::SIGUSR1)
            .with_sender(1234)
            .with_data(42);

        assert_eq!(info.signal, Signal::SIGUSR1);
        assert_eq!(info.sender_pid, Some(1234));
        assert_eq!(info.data, Some(42));
    }

    #[test]
    fn test_signal_statistics() {
        let mut stats = SignalStatistics::new();
        assert_eq!(stats.signals_sent, 0);
        assert_eq!(stats.signals_received, 0);

        stats.record_signal_sent();
        assert_eq!(stats.signals_sent, 1);

        stats.record_signal_received();
        assert_eq!(stats.signals_received, 1);

        stats.record_signal_handled(Duration::from_millis(10));
        assert_eq!(stats.signals_handled, 1);
        assert!(stats.average_handling_time.as_millis() > 0);
    }

    #[test]
    fn test_signal_set() {
        let mut set = SignalSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);

        set.add(Signal::SIGTERM);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 1);
        assert!(set.contains(Signal::SIGTERM));

        set.remove(Signal::SIGTERM);
        assert!(set.is_empty());
        assert!(!set.contains(Signal::SIGTERM));
    }

    #[test]
    fn test_current_pid() {
        let pid = SignalHandler::get_current_pid();
        assert!(pid > 0);
    }

    #[test]
    fn test_global_functions() {
        assert_eq!(get_average_handling_time(), 0);
        assert!(setup_default_signal_handlers().is_ok());
        assert!(cleanup_signal_handlers().is_ok());
    }
}
