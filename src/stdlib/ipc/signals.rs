/// Signal handling for IPC in CURSED
/// 
/// This module provides signal-based inter-process communication and event handling,
/// including process signals, custom signal handlers, and signal coordination.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime};
use std::thread;

use crate::stdlib::ipc::{
    IpcResult, IpcError, IpcHandle, IpcPermissions, IpcConfig,
    invalid_operation, timeout_error, permission_denied
};
use crate::stdlib::ipc::types::IpcHandleType;
use crate::stdlib::ipc::error::{
    communication_error_detailed, system_error, resource_error_detailed
};

/// Signal handler for IPC events
pub struct SignalHandler {
    handle: IpcHandle,
    config: SignalConfig,
    handlers: Arc<RwLock<HashMap<String, Box<dyn SignalCallback + Send + Sync>>>>,
    statistics: Arc<Mutex<SignalStatistics>>,
    pending_signals: Arc<Mutex<Vec<SignalEvent>>>,
}

/// Signal configuration
#[derive(Debug, Clone)]
pub struct SignalConfig {
    pub name: String,
    pub permissions: IpcPermissions,
    pub enable_queuing: bool,
    pub max_queue_size: usize,
    pub enable_priority: bool,
    pub timeout: Duration,
    pub auto_cleanup: bool,
}

impl SignalConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            permissions: IpcPermissions::read_write(),
            enable_queuing: true,
            max_queue_size: 1000,
            enable_priority: false,
            timeout: Duration::from_secs(30),
            auto_cleanup: true,
        }
    }

    pub fn with_queuing(mut self, enabled: bool, max_size: usize) -> Self {
        self.enable_queuing = enabled;
        self.max_queue_size = max_size;
        self
    }

    pub fn with_priority(mut self) -> Self {
        self.enable_priority = true;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// IPC signal types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Signal {
    /// Process lifecycle signals
    ProcessStarted,
    ProcessStopped,
    ProcessCrashed,
    
    /// Resource signals
    ResourceAvailable,
    ResourceExhausted,
    
    /// Communication signals
    DataAvailable,
    ConnectionEstablished,
    ConnectionLost,
    
    /// Synchronization signals
    BarrierReached,
    LockAcquired,
    LockReleased,
    
    /// Custom user-defined signal
    Custom(String),
}

impl Signal {
    pub fn name(&self) -> String {
        match self {
            Signal::ProcessStarted => "ProcessStarted".to_string(),
            Signal::ProcessStopped => "ProcessStopped".to_string(),
            Signal::ProcessCrashed => "ProcessCrashed".to_string(),
            Signal::ResourceAvailable => "ResourceAvailable".to_string(),
            Signal::ResourceExhausted => "ResourceExhausted".to_string(),
            Signal::DataAvailable => "DataAvailable".to_string(),
            Signal::ConnectionEstablished => "ConnectionEstablished".to_string(),
            Signal::ConnectionLost => "ConnectionLost".to_string(),
            Signal::BarrierReached => "BarrierReached".to_string(),
            Signal::LockAcquired => "LockAcquired".to_string(),
            Signal::LockReleased => "LockReleased".to_string(),
            Signal::Custom(name) => name.clone(),
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "ProcessStarted" => Some(Signal::ProcessStarted),
            "ProcessStopped" => Some(Signal::ProcessStopped),
            "ProcessCrashed" => Some(Signal::ProcessCrashed),
            "ResourceAvailable" => Some(Signal::ResourceAvailable),
            "ResourceExhausted" => Some(Signal::ResourceExhausted),
            "DataAvailable" => Some(Signal::DataAvailable),
            "ConnectionEstablished" => Some(Signal::ConnectionEstablished),
            "ConnectionLost" => Some(Signal::ConnectionLost),
            "BarrierReached" => Some(Signal::BarrierReached),
            "LockAcquired" => Some(Signal::LockAcquired),
            "LockReleased" => Some(Signal::LockReleased),
            _ => Some(Signal::Custom(name.to_string())),
        }
    }
}

/// Signal action configuration
#[derive(Debug, Clone)]
pub struct SignalAction {
    pub signal: Signal,
    pub action_type: ActionType,
    pub priority: SignalPriority,
    pub timeout: Option<Duration>,
    pub retry_count: u32,
}

impl SignalAction {
    pub fn new(signal: Signal, action_type: ActionType) -> Self {
        Self {
            signal,
            action_type,
            priority: SignalPriority::Normal,
            timeout: None,
            retry_count: 0,
        }
    }

    pub fn with_priority(mut self, priority: SignalPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_retry(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }
}

/// Signal action types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    /// Default system action
    Default,
    /// Ignore the signal
    Ignore,
    /// Call custom handler
    Custom,
    /// Block the signal
    Block,
}

/// Signal priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignalPriority {
    Low = 1,
    Normal = 5,
    High = 8,
    Critical = 10,
}

/// Signal mask for blocking/unblocking signals
pub struct SignalMask {
    blocked_signals: HashMap<Signal, bool>,
}

impl SignalMask {
    pub fn new() -> Self {
        Self {
            blocked_signals: HashMap::new(),
        }
    }

    pub fn block(&mut self, signal: Signal) {
        self.blocked_signals.insert(signal, true);
    }

    pub fn unblock(&mut self, signal: Signal) {
        self.blocked_signals.insert(signal, false);
    }

    pub fn is_blocked(&self, signal: &Signal) -> bool {
        self.blocked_signals.get(signal).copied().unwrap_or(false)
    }

    pub fn clear(&mut self) {
        self.blocked_signals.clear();
    }
}

/// Signal event
#[derive(Debug, Clone)]
pub struct SignalEvent {
    pub signal: Signal,
    pub data: Vec<u8>,
    pub sender_pid: u32,
    pub timestamp: SystemTime,
    pub priority: SignalPriority,
    pub sequence_number: u64,
}

impl SignalEvent {
    pub fn new(signal: Signal, data: Vec<u8>, sender_pid: u32) -> Self {
        static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        
        Self {
            signal,
            data,
            sender_pid,
            timestamp: SystemTime::now(),
            priority: SignalPriority::Normal,
            sequence_number: SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }

    pub fn with_priority(mut self, priority: SignalPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn age(&self) -> Duration {
        self.timestamp.elapsed().unwrap_or(Duration::from_secs(0))
    }
}

/// Signal callback trait
pub trait SignalCallback {
    /// Handle received signal
    fn handle_signal(&self, event: &SignalEvent) -> IpcResult<()>;
    
    /// Get callback ID
    fn callback_id(&self) -> &str;
    
    /// Check if callback can handle signal
    fn can_handle(&self, signal: &Signal) -> bool;
}

/// Signal statistics
#[derive(Debug, Clone)]
pub struct SignalStatistics {
    pub signals_sent: u64,
    pub signals_received: u64,
    pub signals_blocked: u64,
    pub signals_queued: u64,
    pub handler_invocations: u64,
    pub handler_errors: u64,
    pub average_handling_time: Duration,
    pub last_activity: Option<SystemTime>,
    pub creation_time: SystemTime,
}

impl SignalStatistics {
    pub fn new() -> Self {
        Self {
            signals_sent: 0,
            signals_received: 0,
            signals_blocked: 0,
            signals_queued: 0,
            handler_invocations: 0,
            handler_errors: 0,
            average_handling_time: Duration::from_nanos(0),
            last_activity: None,
            creation_time: SystemTime::now(),
        }
    }

    pub fn record_signal_sent(&mut self) {
        self.signals_sent += 1;
        self.last_activity = Some(SystemTime::now());
    }

    pub fn record_signal_received(&mut self) {
        self.signals_received += 1;
        self.last_activity = Some(SystemTime::now());
    }

    pub fn record_handler_execution(&mut self, duration: Duration) {
        self.handler_invocations += 1;
        self.last_activity = Some(SystemTime::now());
        
        // Update average handling time
        if self.handler_invocations > 1 {
            let total_nanos = self.average_handling_time.as_nanos() as u64 * (self.handler_invocations - 1) 
                            + duration.as_nanos() as u64;
            self.average_handling_time = Duration::from_nanos(total_nanos / self.handler_invocations);
        } else {
            self.average_handling_time = duration;
        }
    }

    pub fn record_handler_error(&mut self) {
        self.handler_errors += 1;
        self.last_activity = Some(SystemTime::now());
    }
}

impl SignalHandler {
    /// Create a new signal handler
    pub fn new(config: SignalConfig) -> IpcResult<Self> {
        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::DomainSocket // Using domain socket as closest match
        );

        Ok(Self {
            handle,
            config,
            handlers: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(Mutex::new(SignalStatistics::new())),
            pending_signals: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Register a signal handler
    pub fn register<H>(&mut self, signal: Signal, handler: H) -> IpcResult<()>
    where
        H: SignalCallback + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(signal.name(), Box::new(handler));
        Ok(())
    }

    /// Unregister a signal handler
    pub fn unregister(&mut self, signal: Signal) -> IpcResult<()> {
        let mut handlers = self.handlers.write().unwrap();
        handlers.remove(&signal.name());
        Ok(())
    }

    /// Send a signal to a process
    pub fn send_signal(&self, target_pid: u32, signal: Signal) -> IpcResult<()> {
        self.send_signal_with_data(target_pid, signal, Vec::new())
    }

    /// Send a signal with data to a process
    pub fn send_signal_with_data(&self, target_pid: u32, signal: Signal, data: Vec<u8>) -> IpcResult<()> {
        let event = SignalEvent::new(signal, data, std::process::id());
        
        // In a real implementation, this would send to the target process
        // For now, we'll simulate by adding to our own pending signals
        if target_pid == std::process::id() {
            self.queue_signal(event)?;
        } else {
            // Simulate sending to external process
            eprintln!("Sending signal {} to process {}", event.signal.name(), target_pid);
        }

        if let Ok(mut stats) = self.statistics.lock() {
            stats.record_signal_sent();
        }

        Ok(())
    }

    /// Process pending signals
    pub fn process_signals(&self) -> IpcResult<usize> {
        let mut pending = self.pending_signals.lock().unwrap();
        let signals_to_process: Vec<SignalEvent> = pending.drain(..).collect();
        drop(pending);

        let mut processed_count = 0;

        for event in signals_to_process {
            self.handle_signal_event(&event)?;
            processed_count += 1;
        }

        Ok(processed_count)
    }

    /// Wait for a specific signal
    pub fn wait_for_signal(&self, signal: Signal, timeout: Duration) -> IpcResult<Option<SignalEvent>> {
        let start_time = std::time::Instant::now();

        loop {
            // Check pending signals
            {
                let mut pending = self.pending_signals.lock().unwrap();
                if let Some(pos) = pending.iter().position(|e| e.signal == signal) {
                    let event = pending.remove(pos);
                    return Ok(Some(event));
                }
            }

            // Check timeout
            if start_time.elapsed() >= timeout {
                return Ok(None);
            }

            // Small delay before checking again
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Check if signals are pending
    pub fn signal_pending(&self, signal: Signal) -> bool {
        let pending = self.pending_signals.lock().unwrap();
        pending.iter().any(|e| e.signal == signal)
    }

    /// Get signal statistics
    pub fn get_statistics(&self) -> SignalStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| SignalStatistics::new())
    }

    fn queue_signal(&self, event: SignalEvent) -> IpcResult<()> {
        if !self.config.enable_queuing {
            return self.handle_signal_event(&event);
        }

        let mut pending = self.pending_signals.lock().unwrap();
        
        if pending.len() >= self.config.max_queue_size {
            return Err(resource_error_detailed(
                "signal_queue",
                "queue_signal",
                "Signal queue is full"
            ));
        }

        if self.config.enable_priority {
            // Insert in priority order
            let pos = pending.iter().position(|e| e.priority < event.priority)
                .unwrap_or(pending.len());
            pending.insert(pos, event);
        } else {
            // FIFO order
            pending.push(event);
        }

        if let Ok(mut stats) = self.statistics.lock() {
            stats.signals_queued += 1;
        }

        Ok(())
    }

    fn handle_signal_event(&self, event: &SignalEvent) -> IpcResult<()> {
        let handlers = self.handlers.read().unwrap();
        
        if let Some(handler) = handlers.get(&event.signal.name()) {
            let start_time = std::time::Instant::now();
            let result = handler.handle_signal(event);
            let duration = start_time.elapsed();

            if let Ok(mut stats) = self.statistics.lock() {
                if result.is_ok() {
                    stats.record_handler_execution(duration);
                } else {
                    stats.record_handler_error();
                }
                stats.record_signal_received();
            }

            result
        } else {
            // No handler registered, just record the signal
            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_signal_received();
            }
            Ok(())
        }
    }
}

/// Signal set for managing multiple signals
pub struct SignalSet {
    signals: HashMap<Signal, bool>,
}

impl SignalSet {
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }

    pub fn add(&mut self, signal: Signal) {
        self.signals.insert(signal, true);
    }

    pub fn remove(&mut self, signal: Signal) {
        self.signals.remove(&signal);
    }

    pub fn contains(&self, signal: &Signal) -> bool {
        self.signals.contains_key(signal)
    }

    pub fn clear(&mut self) {
        self.signals.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }

    pub fn len(&self) -> usize {
        self.signals.len()
    }
}

// Global signal handler registry
lazy_static::lazy_static! {
    static ref SIGNAL_REGISTRY: Arc<RwLock<HashMap<String, Arc<Mutex<SignalHandler>>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
}

/// Module-level functions for signal management

/// Send a signal to a process
pub fn send_signal(target_pid: u32, signal: Signal) -> IpcResult<()> {
    // Create a temporary handler for sending
    let config = SignalConfig::new("temp_sender");
    let handler = SignalHandler::new(config)?;
    handler.send_signal(target_pid, signal)
}

/// Block a signal
pub fn block_signal(signal: Signal) -> IpcResult<()> {
    // In a real implementation, this would block the signal system-wide
    eprintln!("Blocking signal: {}", signal.name());
    Ok(())
}

/// Unblock a signal
pub fn unblock_signal(signal: Signal) -> IpcResult<()> {
    // In a real implementation, this would unblock the signal system-wide
    eprintln!("Unblocking signal: {}", signal.name());
    Ok(())
}

/// Ignore a signal
pub fn ignore_signal(signal: Signal) -> IpcResult<()> {
    // In a real implementation, this would set the signal to be ignored
    eprintln!("Ignoring signal: {}", signal.name());
    Ok(())
}

/// Register a signal handler globally
pub fn register_signal_handler(name: &str, handler: SignalHandler) -> IpcResult<()> {
    SIGNAL_REGISTRY.write().unwrap()
        .insert(name.to_string(), Arc::new(Mutex::new(handler)));
    Ok(())
}

/// Unregister a signal handler
pub fn unregister_signal_handler(name: &str) -> IpcResult<()> {
    SIGNAL_REGISTRY.write().unwrap().remove(name);
    Ok(())
}

/// Wait for a signal with timeout
pub fn wait_for_signal(signal: Signal, timeout: Duration) -> IpcResult<Option<SignalEvent>> {
    // Try to find a handler that can wait for this signal
    let registry = SIGNAL_REGISTRY.read().unwrap();
    if let Some(handler_arc) = registry.values().next() {
        let handler = handler_arc.lock().unwrap();
        handler.wait_for_signal(signal, timeout)
    } else {
        Ok(None)
    }
}

/// Check if a signal is pending
pub fn signal_pending(signal: Signal) -> bool {
    let registry = SIGNAL_REGISTRY.read().unwrap();
    for handler_arc in registry.values() {
        if let Ok(handler) = handler_arc.lock() {
            if handler.signal_pending(signal.clone()) {
                return true;
            }
        }
    }
    false
}

/// Setup default signal handlers for common IPC events
pub fn setup_default_signal_handlers() -> IpcResult<()> {
    let config = SignalConfig::new("default_handler");
    let mut handler = SignalHandler::new(config)?;
    
    // Register handlers for common signals
    handler.register(Signal::ProcessStarted, DefaultSignalCallback::new("process_started"))?;
    handler.register(Signal::ProcessStopped, DefaultSignalCallback::new("process_stopped"))?;
    handler.register(Signal::DataAvailable, DefaultSignalCallback::new("data_available"))?;
    
    register_signal_handler("default", handler)?;
    Ok(())
}

/// Clean up signal handlers
pub fn cleanup_signal_handlers() -> IpcResult<()> {
    SIGNAL_REGISTRY.write().unwrap().clear();
    Ok(())
}

/// Get average signal handling time
pub fn get_average_handling_time() -> u64 {
    let registry = SIGNAL_REGISTRY.read().unwrap();
    let mut total_time = Duration::from_nanos(0);
    let mut count = 0;

    for handler_arc in registry.values() {
        if let Ok(handler) = handler_arc.lock() {
            let stats = handler.get_statistics();
            total_time += stats.average_handling_time;
            count += 1;
        }
    }

    if count > 0 {
        (total_time / count).as_nanos() as u64
    } else {
        0
    }
}

/// Default signal callback implementation
pub struct DefaultSignalCallback {
    id: String,
}

impl DefaultSignalCallback {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
        }
    }
}

impl SignalCallback for DefaultSignalCallback {
    fn handle_signal(&self, event: &SignalEvent) -> IpcResult<()> {
        eprintln!("Default handler [{}]: Received signal {} from PID {} with {} bytes of data",
                 self.id, event.signal.name(), event.sender_pid, event.data.len());
        Ok(())
    }

    fn callback_id(&self) -> &str {
        &self.id
    }

    fn can_handle(&self, _signal: &Signal) -> bool {
        true // Default handler can handle any signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_names() {
        assert_eq!(Signal::ProcessStarted.name(), "ProcessStarted");
        assert_eq!(Signal::DataAvailable.name(), "DataAvailable");
        assert_eq!(Signal::Custom("test".to_string()).name(), "test");
    }

    #[test]
    fn test_signal_from_name() {
        assert_eq!(Signal::from_name("ProcessStarted"), Some(Signal::ProcessStarted));
        assert_eq!(Signal::from_name("DataAvailable"), Some(Signal::DataAvailable));
        
        if let Some(Signal::Custom(name)) = Signal::from_name("custom_signal") {
            assert_eq!(name, "custom_signal");
        } else {
            panic!("Expected custom signal");
        }
    }

    #[test]
    fn test_signal_config() {
        let config = SignalConfig::new("test_signal")
            .with_queuing(true, 500)
            .with_priority()
            .with_timeout(Duration::from_secs(10));

        assert_eq!(config.name, "test_signal");
        assert!(config.enable_queuing);
        assert_eq!(config.max_queue_size, 500);
        assert!(config.enable_priority);
        assert_eq!(config.timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_signal_event() {
        let event = SignalEvent::new(
            Signal::ProcessStarted,
            b"test data".to_vec(),
            1234
        ).with_priority(SignalPriority::High);

        assert_eq!(event.signal, Signal::ProcessStarted);
        assert_eq!(event.data, b"test data");
        assert_eq!(event.sender_pid, 1234);
        assert_eq!(event.priority, SignalPriority::High);
    }

    #[test]
    fn test_signal_action() {
        let action = SignalAction::new(Signal::DataAvailable, ActionType::Custom)
            .with_priority(SignalPriority::High)
            .with_timeout(Duration::from_secs(5))
            .with_retry(3);

        assert_eq!(action.signal, Signal::DataAvailable);
        assert_eq!(action.action_type, ActionType::Custom);
        assert_eq!(action.priority, SignalPriority::High);
        assert_eq!(action.timeout, Some(Duration::from_secs(5)));
        assert_eq!(action.retry_count, 3);
    }

    #[test]
    fn test_signal_mask() {
        let mut mask = SignalMask::new();
        assert!(!mask.is_blocked(&Signal::ProcessStarted));

        mask.block(Signal::ProcessStarted);
        assert!(mask.is_blocked(&Signal::ProcessStarted));

        mask.unblock(Signal::ProcessStarted);
        assert!(!mask.is_blocked(&Signal::ProcessStarted));

        mask.clear();
        assert!(!mask.is_blocked(&Signal::ProcessStarted));
    }

    #[test]
    fn test_signal_set() {
        let mut set = SignalSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);

        set.add(Signal::ProcessStarted);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 1);
        assert!(set.contains(&Signal::ProcessStarted));

        set.remove(Signal::ProcessStarted);
        assert!(set.is_empty());
        assert!(!set.contains(&Signal::ProcessStarted));
    }

    #[test]
    fn test_signal_statistics() {
        let mut stats = SignalStatistics::new();
        assert_eq!(stats.signals_sent, 0);
        assert_eq!(stats.handler_invocations, 0);

        stats.record_signal_sent();
        assert_eq!(stats.signals_sent, 1);

        stats.record_handler_execution(Duration::from_millis(10));
        assert_eq!(stats.handler_invocations, 1);
        assert_eq!(stats.average_handling_time, Duration::from_millis(10));

        stats.record_handler_execution(Duration::from_millis(20));
        assert_eq!(stats.handler_invocations, 2);
        assert_eq!(stats.average_handling_time, Duration::from_millis(15));
    }

    #[test]
    fn test_signal_priority_ordering() {
        assert!(SignalPriority::Critical > SignalPriority::High);
        assert!(SignalPriority::High > SignalPriority::Normal);
        assert!(SignalPriority::Normal > SignalPriority::Low);
    }

    #[test]
    fn test_default_signal_callback() {
        let callback = DefaultSignalCallback::new("test");
        assert_eq!(callback.callback_id(), "test");
        assert!(callback.can_handle(&Signal::ProcessStarted));

        let event = SignalEvent::new(Signal::DataAvailable, Vec::new(), 1234);
        let result = callback.handle_signal(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_global_functions() {
        assert!(!signal_pending(Signal::ProcessStarted));
        assert_eq!(get_average_handling_time(), 0);
    }
}
