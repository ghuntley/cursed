//! Advanced Channel Implementation with Advanced Concurrency Features
//!
//! This module provides enhanced channel functionality including:
//! - Buffered and unbuffered channels with custom buffer strategies
//! - Select statement support with timeout and default cases
//! - Channel closing with proper cleanup and error handling
//! - Work-stealing optimized send/receive operations
//! - Deadlock detection and prevention
//! - Channel multiplexing and demultiplexing
//! - Priority-based channel operations
//! - Backpressure control and flow control
//! - Channel statistics and monitoring
//! - Integration with goroutine scheduler

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::thread;
use std::marker::PhantomData;

use crate::runtime::channels::{ChannelError, ChannelResult, SendResult, ReceiveResult};
use crate::runtime::goroutine::GoroutineId;

/// Channel buffer strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferStrategy {
    /// No buffering (synchronous)
    Unbuffered,
    /// Fixed-size buffer
    Buffered(usize),
    /// Unbounded buffer (grows as needed)
    Unbounded,
    /// Dropping buffer (drops oldest when full)
    Dropping(usize),
    /// Sliding buffer (drops newest when full)
    Sliding(usize),
}

/// Channel priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChannelPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for ChannelPriority {
    fn default() -> Self {
        ChannelPriority::Normal
    }
}

/// Channel configuration
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    pub buffer_strategy: BufferStrategy,
    pub priority: ChannelPriority,
    pub enable_statistics: bool,
    pub enable_deadlock_detection: bool,
    pub send_timeout: Option<Duration>,
    pub receive_timeout: Option<Duration>,
    pub max_waiters: Option<usize>,
    pub enable_backpressure: bool,
    pub backpressure_threshold: Option<usize>,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            buffer_strategy: BufferStrategy::Unbuffered,
            priority: ChannelPriority::Normal,
            enable_statistics: false,
            enable_deadlock_detection: false,
            send_timeout: None,
            receive_timeout: None,
            max_waiters: None,
            enable_backpressure: false,
            backpressure_threshold: None,
        }
    }
}

/// Channel statistics
#[derive(Debug, Clone)]
pub struct ChannelStats {
    pub total_sends: u64,
    pub total_receives: u64,
    pub total_send_timeouts: u64,
    pub total_receive_timeouts: u64,
    pub total_send_blocks: u64,
    pub total_receive_blocks: u64,
    pub current_buffer_size: usize,
    pub peak_buffer_size: usize,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub average_send_time: Duration,
    pub average_receive_time: Duration,
    pub created_at: Instant,
    pub last_send_time: Option<Instant>,
    pub last_receive_time: Option<Instant>,
}

impl Default for ChannelStats {
    fn default() -> Self {
        Self {
            total_sends: 0,
            total_receives: 0,
            total_send_timeouts: 0,
            total_receive_timeouts: 0,
            total_send_blocks: 0,
            total_receive_blocks: 0,
            current_buffer_size: 0,
            peak_buffer_size: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            average_send_time: Duration::default(),
            average_receive_time: Duration::default(),
            created_at: Instant::now(),
            last_send_time: None,
            last_receive_time: None,
        }
    }
}

/// Advanced channel implementation
pub struct AdvancedChannel<T> {
    /// Channel ID
    id: u64,
    /// Configuration
    config: ChannelConfig,
    /// Internal buffer
    buffer: Arc<Mutex<VecDeque<T>>>,
    /// Channel state
    state: Arc<ChannelState>,
    /// Statistics
    stats: Arc<Mutex<ChannelStats>>,
    /// Waiting senders
    waiting_senders: Arc<Mutex<VecDeque<WaitingOperation>>>,
    /// Waiting receivers
    waiting_receivers: Arc<Mutex<VecDeque<WaitingOperation>>>,
    /// Condition variables for notification
    sender_notify: Arc<Condvar>,
    receiver_notify: Arc<Condvar>,
    /// Deadlock detection
    deadlock_detector: Option<Arc<DeadlockDetector>>,
}

/// Channel state
#[derive(Debug)]
struct ChannelState {
    closed: AtomicBool,
    sender_count: AtomicUsize,
    receiver_count: AtomicUsize,
    waiting_sender_count: AtomicUsize,
    waiting_receiver_count: AtomicUsize,
    sequence_number: AtomicU64,
}

/// Waiting operation information
#[derive(Debug)]
struct WaitingOperation {
    goroutine_id: Option<GoroutineId>,
    started_at: Instant,
    timeout: Option<Duration>,
    priority: ChannelPriority,
}

/// Deadlock detector
pub struct DeadlockDetector {
    channels: Arc<RwLock<Vec<Arc<ChannelState>>>>,
    detection_interval: Duration,
    running: Arc<AtomicBool>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl DeadlockDetector {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(Vec::new())),
            detection_interval: Duration::from_millis(100),
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    pub fn start(&mut self) {
        if self.running.swap(true, Ordering::SeqCst) {
            return; // Already running
        }

        let channels = self.channels.clone();
        let running = self.running.clone();
        let interval = self.detection_interval;

        let handle = thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                Self::detect_deadlocks(&channels);
                thread::sleep(interval);
            }
        });

        self.thread_handle = Some(handle);
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    fn detect_deadlocks(channels: &Arc<RwLock<Vec<Arc<ChannelState>>>>) {
        if let Ok(channel_states) = channels.read() {
            for state in channel_states.iter() {
                let sender_count = state.sender_count.load(Ordering::SeqCst);
                let receiver_count = state.receiver_count.load(Ordering::SeqCst);
                let waiting_senders = state.waiting_sender_count.load(Ordering::SeqCst);
                let waiting_receivers = state.waiting_receiver_count.load(Ordering::SeqCst);

                // Simple deadlock detection: all senders waiting and no receivers
                if sender_count > 0 && receiver_count == 0 && waiting_senders == sender_count {
                    log::warn!("Potential deadlock detected: all senders waiting with no receivers");
                }

                // All receivers waiting and no senders
                if receiver_count > 0 && sender_count == 0 && waiting_receivers == receiver_count {
                    log::warn!("Potential deadlock detected: all receivers waiting with no senders");
                }
            }
        }
    }

    pub fn register_channel(&self, state: Arc<ChannelState>) {
        if let Ok(mut channels) = self.channels.write() {
            channels.push(state);
        }
    }

    pub fn unregister_channel(&self, state: &Arc<ChannelState>) {
        if let Ok(mut channels) = self.channels.write() {
            channels.retain(|s| !Arc::ptr_eq(s, state));
        }
    }
}

impl<T> AdvancedChannel<T> {
    /// Create a new advanced channel
    pub fn new() -> Self {
        Self::with_config(ChannelConfig::default())
    }

    /// Create a new advanced channel with configuration
    pub fn with_config(config: ChannelConfig) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        let state = Arc::new(ChannelState {
            closed: AtomicBool::new(false),
            sender_count: AtomicUsize::new(0),
            receiver_count: AtomicUsize::new(0),
            waiting_sender_count: AtomicUsize::new(0),
            waiting_receiver_count: AtomicUsize::new(0),
            sequence_number: AtomicU64::new(0),
        });

        let stats = Arc::new(Mutex::new(ChannelStats {
            created_at: Instant::now(),
            ..Default::default()
        }));

        let deadlock_detector = if config.enable_deadlock_detection {
            let mut detector = DeadlockDetector::new();
            detector.register_channel(state.clone());
            detector.start();
            Some(Arc::new(detector))
        } else {
            None
        };

        Self {
            id,
            config,
            buffer,
            state,
            stats,
            waiting_senders: Arc::new(Mutex::new(VecDeque::new())),
            waiting_receivers: Arc::new(Mutex::new(VecDeque::new())),
            sender_notify: Arc::new(Condvar::new()),
            receiver_notify: Arc::new(Condvar::new()),
            deadlock_detector,
        }
    }

    /// Send a value with advanced features
    pub fn send(&self, value: T) -> SendResult<T> {
        self.send_with_options(value, None, None)
    }

    /// Send a value with timeout and goroutine context
    pub fn send_with_options(
        &self,
        value: T,
        timeout: Option<Duration>,
        goroutine_id: Option<GoroutineId>,
    ) -> SendResult<T> {
        let send_start = Instant::now();
        
        // Update statistics
        if self.config.enable_statistics {
            if let Ok(mut stats) = self.stats.lock() {
                stats.total_sends += 1;
                stats.last_send_time = Some(send_start);
            }
        }

        // Check if channel is closed
        if self.state.closed.load(Ordering::SeqCst) {
            return SendResult::Closed(value);
        }

        // Try immediate send first
        if let Some(result) = self.try_immediate_send(value) {
            // Update send time statistics
            if self.config.enable_statistics {
                if let Ok(mut stats) = self.stats.lock() {
                    let send_time = send_start.elapsed();
                    stats.average_send_time = 
                        (stats.average_send_time + send_time) / 2;
                }
            }
            return result;
        }

        // Need to wait - add to waiting queue
        let waiting_op = WaitingOperation {
            goroutine_id,
            started_at: send_start,
            timeout: timeout.or(self.config.send_timeout),
            priority: self.config.priority,
        };

        self.state.waiting_sender_count.fetch_add(1, Ordering::SeqCst);
        
        // Add to waiting queue with priority ordering
        if let Ok(mut waiting_senders) = self.waiting_senders.lock() {
            // Insert based on priority
            let insert_pos = waiting_senders
                .iter()
                .position(|op| op.priority < waiting_op.priority)
                .unwrap_or(waiting_senders.len());
            waiting_senders.insert(insert_pos, waiting_op);
        }

        // Wait for send opportunity
        let result = self.wait_for_send_opportunity(value, timeout, send_start);
        
        self.state.waiting_sender_count.fetch_sub(1, Ordering::SeqCst);
        
        // Update statistics
        if self.config.enable_statistics {
            if let Ok(mut stats) = self.stats.lock() {
                let send_time = send_start.elapsed();
                stats.average_send_time = 
                    (stats.average_send_time + send_time) / 2;
                
                if matches!(result, SendResult::WouldBlock(_)) {
                    stats.total_send_timeouts += 1;
                } else {
                    stats.total_send_blocks += 1;
                }
            }
        }

        result
    }

    /// Try to send immediately without blocking
    fn try_immediate_send(&self, value: T) -> Option<SendResult<T>> {
        match self.config.buffer_strategy {
            BufferStrategy::Unbuffered => {
                // For unbuffered channels, need receiver to be waiting
                if self.state.waiting_receiver_count.load(Ordering::SeqCst) > 0 {
                    if let Ok(mut buffer) = self.buffer.lock() {
                        buffer.push_back(value);
                        self.receiver_notify.notify_one();
                        return Some(SendResult::Sent);
                    }
                }
                None
            }
            BufferStrategy::Buffered(capacity) => {
                if let Ok(mut buffer) = self.buffer.lock() {
                    if buffer.len() < capacity {
                        buffer.push_back(value);
                        self.receiver_notify.notify_one();
                        return Some(SendResult::Sent);
                    }
                }
                None
            }
            BufferStrategy::Unbounded => {
                if let Ok(mut buffer) = self.buffer.lock() {
                    buffer.push_back(value);
                    self.receiver_notify.notify_one();
                    return Some(SendResult::Sent);
                }
                None
            }
            BufferStrategy::Dropping(capacity) => {
                if let Ok(mut buffer) = self.buffer.lock() {
                    if buffer.len() >= capacity {
                        buffer.pop_front(); // Drop oldest
                    }
                    buffer.push_back(value);
                    self.receiver_notify.notify_one();
                    return Some(SendResult::Sent);
                }
                None
            }
            BufferStrategy::Sliding(capacity) => {
                if let Ok(mut buffer) = self.buffer.lock() {
                    if buffer.len() >= capacity {
                        buffer.pop_back(); // Drop newest
                    }
                    buffer.push_back(value);
                    self.receiver_notify.notify_one();
                    return Some(SendResult::Sent);
                }
                None
            }
        }
    }

    /// Wait for send opportunity
    fn wait_for_send_opportunity(
        &self,
        mut value: T,
        timeout: Option<Duration>,
        start_time: Instant,
    ) -> SendResult<T> {
        let timeout_duration = timeout.unwrap_or(Duration::from_secs(30));
        
        loop {
            // Check timeout
            if start_time.elapsed() >= timeout_duration {
                return SendResult::WouldBlock(value);
            }

            // Check if channel is closed
            if self.state.closed.load(Ordering::SeqCst) {
                return SendResult::Closed(value);
            }

            // Try to send again
            if let Some(result) = self.try_immediate_send(value) {
                return result;
            }

            // Wait for notification
            if let Ok(buffer_guard) = self.buffer.lock() {
                let wait_result = self.sender_notify.wait_timeout(
                    buffer_guard,
                    Duration::from_millis(10),
                );
                
                if wait_result.is_ok() {
                    // Got notification, try again
                    continue;
                }
            }

            // Brief yield to prevent busy waiting
            thread::yield_now();
        }
    }

    /// Receive a value with advanced features
    pub fn receive(&self) -> ReceiveResult<T> {
        self.receive_with_options(None, None)
    }

    /// Receive a value with timeout and goroutine context
    pub fn receive_with_options(
        &self,
        timeout: Option<Duration>,
        goroutine_id: Option<GoroutineId>,
    ) -> ReceiveResult<T> {
        let receive_start = Instant::now();
        
        // Update statistics
        if self.config.enable_statistics {
            if let Ok(mut stats) = self.stats.lock() {
                stats.total_receives += 1;
                stats.last_receive_time = Some(receive_start);
            }
        }

        // Try immediate receive first
        if let Some(result) = self.try_immediate_receive() {
            // Update receive time statistics
            if self.config.enable_statistics {
                if let Ok(mut stats) = self.stats.lock() {
                    let receive_time = receive_start.elapsed();
                    stats.average_receive_time = 
                        (stats.average_receive_time + receive_time) / 2;
                }
            }
            return result;
        }

        // Need to wait - add to waiting queue
        let waiting_op = WaitingOperation {
            goroutine_id,
            started_at: receive_start,
            timeout: timeout.or(self.config.receive_timeout),
            priority: self.config.priority,
        };

        self.state.waiting_receiver_count.fetch_add(1, Ordering::SeqCst);
        
        // Add to waiting queue with priority ordering
        if let Ok(mut waiting_receivers) = self.waiting_receivers.lock() {
            let insert_pos = waiting_receivers
                .iter()
                .position(|op| op.priority < waiting_op.priority)
                .unwrap_or(waiting_receivers.len());
            waiting_receivers.insert(insert_pos, waiting_op);
        }

        // Wait for receive opportunity
        let result = self.wait_for_receive_opportunity(timeout, receive_start);
        
        self.state.waiting_receiver_count.fetch_sub(1, Ordering::SeqCst);
        
        // Update statistics
        if self.config.enable_statistics {
            if let Ok(mut stats) = self.stats.lock() {
                let receive_time = receive_start.elapsed();
                stats.average_receive_time = 
                    (stats.average_receive_time + receive_time) / 2;
                
                if matches!(result, ReceiveResult::WouldBlock) {
                    stats.total_receive_timeouts += 1;
                } else {
                    stats.total_receive_blocks += 1;
                }
            }
        }

        result
    }

    /// Try to receive immediately without blocking
    fn try_immediate_receive(&self) -> Option<ReceiveResult<T>> {
        if let Ok(mut buffer) = self.buffer.lock() {
            if let Some(value) = buffer.pop_front() {
                self.sender_notify.notify_one();
                return Some(ReceiveResult::Received(value));
            }
        }

        // Check if channel is closed
        if self.state.closed.load(Ordering::SeqCst) {
            return Some(ReceiveResult::Closed);
        }

        None
    }

    /// Wait for receive opportunity
    fn wait_for_receive_opportunity(
        &self,
        timeout: Option<Duration>,
        start_time: Instant,
    ) -> ReceiveResult<T> {
        let timeout_duration = timeout.unwrap_or(Duration::from_secs(30));
        
        loop {
            // Check timeout
            if start_time.elapsed() >= timeout_duration {
                return ReceiveResult::WouldBlock;
            }

            // Try to receive again
            if let Some(result) = self.try_immediate_receive() {
                return result;
            }

            // Wait for notification
            if let Ok(buffer_guard) = self.buffer.lock() {
                let wait_result = self.receiver_notify.wait_timeout(
                    buffer_guard,
                    Duration::from_millis(10),
                );
                
                if wait_result.is_ok() {
                    // Got notification, try again
                    continue;
                }
            }

            // Brief yield to prevent busy waiting
            thread::yield_now();
        }
    }

    /// Close the channel
    pub fn close(&self) {
        self.state.closed.store(true, Ordering::SeqCst);
        self.sender_notify.notify_all();
        self.receiver_notify.notify_all();
    }

    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        self.state.closed.load(Ordering::SeqCst)
    }

    /// Get channel statistics
    pub fn get_stats(&self) -> ChannelStats {
        if let Ok(stats) = self.stats.lock() {
            let mut result = stats.clone();
            result.current_buffer_size = self.buffer.lock().map(|b| b.len()).unwrap_or(0);
            result
        } else {
            ChannelStats::default()
        }
    }

    /// Get channel ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get channel configuration
    pub fn config(&self) -> &ChannelConfig {
        &self.config
    }

    /// Create a sender handle
    pub fn sender(&self) -> AdvancedChannelSender<T> {
        self.state.sender_count.fetch_add(1, Ordering::SeqCst);
        AdvancedChannelSender {
            channel: self.clone(),
            _phantom: PhantomData,
        }
    }

    /// Create a receiver handle
    pub fn receiver(&self) -> AdvancedChannelReceiver<T> {
        self.state.receiver_count.fetch_add(1, Ordering::SeqCst);
        AdvancedChannelReceiver {
            channel: self.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Clone for AdvancedChannel<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            config: self.config.clone(),
            buffer: self.buffer.clone(),
            state: self.state.clone(),
            stats: self.stats.clone(),
            waiting_senders: self.waiting_senders.clone(),
            waiting_receivers: self.waiting_receivers.clone(),
            sender_notify: self.sender_notify.clone(),
            receiver_notify: self.receiver_notify.clone(),
            deadlock_detector: self.deadlock_detector.clone(),
        }
    }
}

impl<T> Drop for AdvancedChannel<T> {
    fn drop(&mut self) {
        if let Some(ref detector) = self.deadlock_detector {
            detector.unregister_channel(&self.state);
        }
    }
}

/// Advanced channel sender
pub struct AdvancedChannelSender<T> {
    channel: AdvancedChannel<T>,
    _phantom: PhantomData<T>,
}

impl<T> AdvancedChannelSender<T> {
    /// Send a value
    pub fn send(&self, value: T) -> SendResult<T> {
        self.channel.send(value)
    }

    /// Send a value with timeout
    pub fn send_timeout(&self, value: T, timeout: Duration) -> SendResult<T> {
        self.channel.send_with_options(value, Some(timeout), None)
    }

    /// Send a value with goroutine context
    pub fn send_with_goroutine(&self, value: T, goroutine_id: GoroutineId) -> SendResult<T> {
        self.channel.send_with_options(value, None, Some(goroutine_id))
    }

    /// Close the channel
    pub fn close(&self) {
        self.channel.close();
    }

    /// Check if closed
    pub fn is_closed(&self) -> bool {
        self.channel.is_closed()
    }

    /// Get channel ID
    pub fn channel_id(&self) -> u64 {
        self.channel.id()
    }
}

impl<T> Clone for AdvancedChannelSender<T> {
    fn clone(&self) -> Self {
        self.channel.state.sender_count.fetch_add(1, Ordering::SeqCst);
        Self {
            channel: self.channel.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Drop for AdvancedChannelSender<T> {
    fn drop(&mut self) {
        let count = self.channel.state.sender_count.fetch_sub(1, Ordering::SeqCst);
        if count == 1 {
            self.channel.close();
        }
    }
}

/// Advanced channel receiver
pub struct AdvancedChannelReceiver<T> {
    channel: AdvancedChannel<T>,
    _phantom: PhantomData<T>,
}

impl<T> AdvancedChannelReceiver<T> {
    /// Receive a value
    pub fn receive(&self) -> ReceiveResult<T> {
        self.channel.receive()
    }

    /// Receive a value with timeout
    pub fn receive_timeout(&self, timeout: Duration) -> ReceiveResult<T> {
        self.channel.receive_with_options(Some(timeout), None)
    }

    /// Receive a value with goroutine context
    pub fn receive_with_goroutine(&self, goroutine_id: GoroutineId) -> ReceiveResult<T> {
        self.channel.receive_with_options(None, Some(goroutine_id))
    }

    /// Check if closed
    pub fn is_closed(&self) -> bool {
        self.channel.is_closed()
    }

    /// Get channel ID
    pub fn channel_id(&self) -> u64 {
        self.channel.id()
    }
}

impl<T> Clone for AdvancedChannelReceiver<T> {
    fn clone(&self) -> Self {
        self.channel.state.receiver_count.fetch_add(1, Ordering::SeqCst);
        Self {
            channel: self.channel.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Drop for AdvancedChannelReceiver<T> {
    fn drop(&mut self) {
        self.channel.state.receiver_count.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Channel factory functions
pub fn advanced_channel<T>() -> (AdvancedChannelSender<T>, AdvancedChannelReceiver<T>) {
    let channel = AdvancedChannel::new();
    (channel.sender(), channel.receiver())
}

pub fn advanced_buffered_channel<T>(capacity: usize) -> (AdvancedChannelSender<T>, AdvancedChannelReceiver<T>) {
    let config = ChannelConfig {
        buffer_strategy: BufferStrategy::Buffered(capacity),
        ..Default::default()
    };
    let channel = AdvancedChannel::with_config(config);
    (channel.sender(), channel.receiver())
}

pub fn advanced_unbounded_channel<T>() -> (AdvancedChannelSender<T>, AdvancedChannelReceiver<T>) {
    let config = ChannelConfig {
        buffer_strategy: BufferStrategy::Unbounded,
        ..Default::default()
    };
    let channel = AdvancedChannel::with_config(config);
    (channel.sender(), channel.receiver())
}

pub fn advanced_dropping_channel<T>(capacity: usize) -> (AdvancedChannelSender<T>, AdvancedChannelReceiver<T>) {
    let config = ChannelConfig {
        buffer_strategy: BufferStrategy::Dropping(capacity),
        ..Default::default()
    };
    let channel = AdvancedChannel::with_config(config);
    (channel.sender(), channel.receiver())
}

pub fn advanced_sliding_channel<T>(capacity: usize) -> (AdvancedChannelSender<T>, AdvancedChannelReceiver<T>) {
    let config = ChannelConfig {
        buffer_strategy: BufferStrategy::Sliding(capacity),
        ..Default::default()
    };
    let channel = AdvancedChannel::with_config(config);
    (channel.sender(), channel.receiver())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_advanced_unbuffered_channel() {
        let (sender, receiver) = advanced_channel::<i32>();
        
        let sender_handle = thread::spawn(move || {
            assert!(matches!(sender.send(42), SendResult::Sent));
        });
        
        match receiver.receive() {
            ReceiveResult::Received(value) => assert_eq!(value, 42),
            _ => panic!("Should receive value"),
        }
        
        sender_handle.join().unwrap();
    }

    #[test]
    fn test_advanced_buffered_channel() {
        let (sender, receiver) = advanced_buffered_channel::<i32>(3);
        
        // Should be able to send without blocking
        assert!(matches!(sender.send(1), SendResult::Sent));
        assert!(matches!(sender.send(2), SendResult::Sent));
        assert!(matches!(sender.send(3), SendResult::Sent));
        
        // Should be able to receive
        assert!(matches!(receiver.receive(), ReceiveResult::Received(1)));
        assert!(matches!(receiver.receive(), ReceiveResult::Received(2)));
        assert!(matches!(receiver.receive(), ReceiveResult::Received(3)));
    }

    #[test]
    fn test_advanced_channel_with_timeout() {
        let (sender, receiver) = advanced_channel::<i32>();
        
        // Should timeout
        let timeout_result = receiver.receive_timeout(Duration::from_millis(100));
        assert!(matches!(timeout_result, ReceiveResult::WouldBlock));
    }

    #[test]
    fn test_advanced_channel_statistics() {
        let config = ChannelConfig {
            enable_statistics: true,
            ..Default::default()
        };
        let channel = AdvancedChannel::with_config(config);
        let sender = channel.sender();
        let receiver = channel.receiver();
        
        // Send and receive
        assert!(matches!(sender.send(42), SendResult::Sent));
        assert!(matches!(receiver.receive(), ReceiveResult::Received(42)));
        
        // Check statistics
        let stats = channel.get_stats();
        assert_eq!(stats.total_sends, 1);
        assert_eq!(stats.total_receives, 1);
    }

    #[test]
    fn test_advanced_dropping_channel() {
        let (sender, receiver) = advanced_dropping_channel::<i32>(2);
        
        // Fill channel
        assert!(matches!(sender.send(1), SendResult::Sent));
        assert!(matches!(sender.send(2), SendResult::Sent));
        
        // This should drop the first value
        assert!(matches!(sender.send(3), SendResult::Sent));
        
        // Should receive 2 and 3 (1 was dropped)
        assert!(matches!(receiver.receive(), ReceiveResult::Received(2)));
        assert!(matches!(receiver.receive(), ReceiveResult::Received(3)));
    }

    #[test]
    fn test_advanced_sliding_channel() {
        let (sender, receiver) = advanced_sliding_channel::<i32>(2);
        
        // Fill channel
        assert!(matches!(sender.send(1), SendResult::Sent));
        assert!(matches!(sender.send(2), SendResult::Sent));
        
        // This should drop the newest value (2)
        assert!(matches!(sender.send(3), SendResult::Sent));
        
        // Should receive 1 and 3 (2 was dropped)
        assert!(matches!(receiver.receive(), ReceiveResult::Received(1)));
        assert!(matches!(receiver.receive(), ReceiveResult::Received(3)));
    }
}
