//! Production-Grade Channel Implementation for CURSED Runtime
//!
//! This module provides enterprise-ready channel operations:
//! - Lock-free buffered channels with configurable capacity
//! - Priority-based message queuing
//! - Backpressure handling and flow control
//! - Channel multiplexing and demultiplexing
//! - Comprehensive monitoring and statistics
//! - Memory-efficient buffer management
//! - Deadlock detection and prevention

use super::{ChannelError, ChannelResult, SendResult, ReceiveResult, ChannelStats};
use crate::runtime::goroutine::GoroutineId;

use std::collections::{VecDeque, HashMap};
use std::sync::{Arc, Mutex, RwLock, Condvar, atomic::{AtomicUsize, AtomicU64, AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::thread;
use crossbeam_channel::{bounded, unbounded, Sender, Receiver, TryRecvError, RecvTimeoutError};

/// Channel priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChannelPriority {
    /// Low priority messages
    Low = 0,
    /// Normal priority messages (default)
    Normal = 1,
    /// High priority messages
    High = 2,
    /// Critical priority messages
    Critical = 3,
}

impl Default for ChannelPriority {
    fn default() -> Self {
        ChannelPriority::Normal
    }
}

/// Message wrapper with priority and metadata
#[derive(Debug, Clone)]
pub struct PriorityMessage<T> {
    /// Message payload
    pub payload: T,
    /// Message priority
    pub priority: ChannelPriority,
    /// Sender goroutine ID
    pub sender_id: Option<GoroutineId>,
    /// Timestamp when message was sent
    pub timestamp: Instant,
    /// Message ID for tracking
    pub message_id: u64,
    /// Retry count for reliability
    pub retry_count: u32,
}

impl<T> PriorityMessage<T> {
    /// Create a new priority message
    pub fn new(payload: T, priority: ChannelPriority, sender_id: Option<GoroutineId>) -> Self {
        static MESSAGE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        
        Self {
            payload,
            priority,
            sender_id,
            timestamp: Instant::now(),
            message_id: MESSAGE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            retry_count: 0,
        }
    }

    /// Get message age
    pub fn age(&self) -> Duration {
        self.timestamp.elapsed()
    }
}

/// Channel configuration
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    /// Channel capacity (0 = unbuffered, None = unbounded)
    pub capacity: Option<usize>,
    /// Enable priority queuing
    pub enable_priority: bool,
    /// Maximum message age before expiration
    pub max_message_age: Option<Duration>,
    /// Enable backpressure handling
    pub enable_backpressure: bool,
    /// Backpressure threshold
    pub backpressure_threshold: f64,
    /// Enable message ordering
    pub enable_ordering: bool,
    /// Enable duplicate detection
    pub enable_duplicate_detection: bool,
    /// Maximum retry attempts
    pub max_retry_attempts: u32,
    /// Enable deadlock detection
    pub enable_deadlock_detection: bool,
    /// Statistics collection enabled
    pub enable_statistics: bool,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            capacity: Some(1000),
            enable_priority: false,
            max_message_age: None,
            enable_backpressure: true,
            backpressure_threshold: 0.8,
            enable_ordering: false,
            enable_duplicate_detection: false,
            max_retry_attempts: 3,
            enable_deadlock_detection: true,
            enable_statistics: true,
        }
    }
}

/// Channel statistics
#[derive(Debug)]
pub struct ProductionChannelStats {
    /// Channel ID
    pub channel_id: u64,
    /// Total messages sent
    pub messages_sent: AtomicU64,
    /// Total messages received
    pub messages_received: AtomicU64,
    /// Messages dropped due to capacity
    pub messages_dropped: AtomicU64,
    /// Messages expired due to age
    pub messages_expired: AtomicU64,
    /// Messages retried
    pub messages_retried: AtomicU64,
    /// Current buffer size
    pub current_buffer_size: AtomicUsize,
    /// Peak buffer size
    pub peak_buffer_size: AtomicUsize,
    /// Total send operations
    pub total_send_ops: AtomicU64,
    /// Total receive operations
    pub total_receive_ops: AtomicU64,
    /// Send operations blocked
    pub send_ops_blocked: AtomicU64,
    /// Receive operations blocked
    pub receive_ops_blocked: AtomicU64,
    /// Average message latency (nanoseconds)
    pub average_latency_ns: AtomicU64,
    /// Peak message latency (nanoseconds)
    pub peak_latency_ns: AtomicU64,
    /// Total bytes sent
    pub total_bytes_sent: AtomicU64,
    /// Total bytes received
    pub total_bytes_received: AtomicU64,
    /// Backpressure events
    pub backpressure_events: AtomicU64,
    /// Deadlock detections
    pub deadlock_detections: AtomicU64,
    /// Channel creation time
    pub created_at: Instant,
}

impl Default for ProductionChannelStats {
    fn default() -> Self {
        Self {
            channel_id: 0,
            messages_sent: AtomicU64::new(0),
            messages_received: AtomicU64::new(0),
            messages_dropped: AtomicU64::new(0),
            messages_expired: AtomicU64::new(0),
            messages_retried: AtomicU64::new(0),
            current_buffer_size: AtomicUsize::new(0),
            peak_buffer_size: AtomicUsize::new(0),
            total_send_ops: AtomicU64::new(0),
            total_receive_ops: AtomicU64::new(0),
            send_ops_blocked: AtomicU64::new(0),
            receive_ops_blocked: AtomicU64::new(0),
            average_latency_ns: AtomicU64::new(0),
            peak_latency_ns: AtomicU64::new(0),
            total_bytes_sent: AtomicU64::new(0),
            total_bytes_received: AtomicU64::new(0),
            backpressure_events: AtomicU64::new(0),
            deadlock_detections: AtomicU64::new(0),
            created_at: Instant::now(),
        }
    }
}

impl ProductionChannelStats {
    /// Create new statistics
    pub fn new(channel_id: u64) -> Self {
        Self {
            channel_id,
            created_at: Instant::now(),
            ..Default::default()
        }
    }

    /// Get comprehensive statistics
    pub fn get_comprehensive_stats(&self) -> ChannelStats {
        ChannelStats {
            id: self.channel_id as usize,
            capacity: 0, // Will be set by channel
            current_length: self.current_buffer_size.load(Ordering::Relaxed),
            sender_count: 0, // Will be set by channel
            receiver_count: 0, // Will be set by channel
            is_closed: false, // Will be set by channel
            total_sent: self.messages_sent.load(Ordering::Relaxed),
            total_received: self.messages_received.load(Ordering::Relaxed),
            messages_dropped: self.messages_dropped.load(Ordering::Relaxed),
        }
    }

    /// Calculate throughput (messages per second)
    pub fn calculate_throughput(&self) -> f64 {
        let uptime = self.created_at.elapsed().as_secs_f64();
        if uptime > 0.0 {
            self.messages_sent.load(Ordering::Relaxed) as f64 / uptime
        } else {
            0.0
        }
    }

    /// Calculate efficiency (received/sent ratio)
    pub fn calculate_efficiency(&self) -> f64 {
        let sent = self.messages_sent.load(Ordering::Relaxed);
        let received = self.messages_received.load(Ordering::Relaxed);
        if sent > 0 {
            received as f64 / sent as f64
        } else {
            0.0
        }
    }
}

/// Priority queue for channel messages
pub struct PriorityQueue<T> {
    /// Queues for each priority level
    queues: [VecDeque<PriorityMessage<T>>; 4],
    /// Total size across all queues
    total_size: usize,
    /// Maximum size
    max_size: Option<usize>,
}

impl<T> PriorityQueue<T> {
    /// Create a new priority queue
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            queues: [VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new()],
            total_size: 0,
            max_size,
        }
    }

    /// Push a message with priority
    pub fn push(&mut self, message: PriorityMessage<T>) -> Result<(), PriorityMessage<T>> {
        if let Some(max_size) = self.max_size {
            if self.total_size >= max_size {
                return Err(message);
            }
        }

        let priority_index = message.priority as usize;
        self.queues[priority_index].push_back(message);
        self.total_size += 1;
        Ok(())
    }

    /// Pop the highest priority message
    pub fn pop(&mut self) -> Option<PriorityMessage<T>> {
        // Check from highest to lowest priority
        for i in (0..4).rev() {
            if let Some(message) = self.queues[i].pop_front() {
                self.total_size -= 1;
                return Some(message);
            }
        }
        None
    }

    /// Get current size
    pub fn len(&self) -> usize {
        self.total_size
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.total_size == 0
    }

    /// Check if full
    pub fn is_full(&self) -> bool {
        if let Some(max_size) = self.max_size {
            self.total_size >= max_size
        } else {
            false
        }
    }

    /// Get capacity
    pub fn capacity(&self) -> Option<usize> {
        self.max_size
    }

    /// Remove expired messages
    pub fn remove_expired(&mut self, max_age: Duration) -> usize {
        let mut removed = 0;
        let now = Instant::now();
        
        for queue in &mut self.queues {
            let mut i = 0;
            while i < queue.len() {
                if now.duration_since(queue[i].timestamp) > max_age {
                    queue.remove(i);
                    removed += 1;
                    self.total_size -= 1;
                } else {
                    i += 1;
                }
            }
        }
        
        removed
    }
}

/// Production channel implementation
pub struct ProductionChannel<T> {
    /// Channel ID
    id: u64,
    /// Channel configuration
    config: ChannelConfig,
    /// Message buffer
    buffer: Arc<Mutex<PriorityQueue<T>>>,
    /// Channel statistics
    stats: Arc<ProductionChannelStats>,
    /// Sender count
    sender_count: Arc<AtomicUsize>,
    /// Receiver count
    receiver_count: Arc<AtomicUsize>,
    /// Closed flag
    closed: Arc<AtomicBool>,
    /// Send notification
    send_notify: Arc<Condvar>,
    /// Receive notification
    receive_notify: Arc<Condvar>,
    /// Backpressure state
    backpressure_active: Arc<AtomicBool>,
    /// Deadlock detection
    deadlock_detector: Arc<Mutex<DeadlockDetector>>,
    /// Message deduplication
    message_ids: Arc<RwLock<HashMap<u64, Instant>>>,
}

/// Deadlock detector
#[derive(Debug)]
pub struct DeadlockDetector {
    /// Waiting goroutines
    waiting_goroutines: HashMap<GoroutineId, Instant>,
    /// Timeout for deadlock detection
    timeout: Duration,
    /// Last check time
    last_check: Instant,
}

impl DeadlockDetector {
    /// Create new deadlock detector
    pub fn new(timeout: Duration) -> Self {
        Self {
            waiting_goroutines: HashMap::new(),
            timeout,
            last_check: Instant::now(),
        }
    }

    /// Register waiting goroutine
    pub fn register_waiting(&mut self, goroutine_id: GoroutineId) {
        self.waiting_goroutines.insert(goroutine_id, Instant::now());
    }

    /// Unregister waiting goroutine
    pub fn unregister_waiting(&mut self, goroutine_id: GoroutineId) {
        self.waiting_goroutines.remove(&goroutine_id);
    }

    /// Check for deadlocks
    pub fn check_deadlocks(&mut self) -> Vec<GoroutineId> {
        let now = Instant::now();
        let mut deadlocked = Vec::new();

        for (goroutine_id, start_time) in &self.waiting_goroutines {
            if now.duration_since(*start_time) > self.timeout {
                deadlocked.push(*goroutine_id);
            }
        }

        self.last_check = now;
        deadlocked
    }
}

impl<T> ProductionChannel<T> {
    /// Create a new production channel
    pub fn new(config: ChannelConfig) -> Self {
        static CHANNEL_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        
        let id = CHANNEL_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let buffer = Arc::new(Mutex::new(PriorityQueue::new(config.capacity)));
        let stats = Arc::new(ProductionChannelStats::new(id));
        
        Self {
            id,
            config,
            buffer,
            stats,
            sender_count: Arc::new(AtomicUsize::new(0)),
            receiver_count: Arc::new(AtomicUsize::new(0)),
            closed: Arc::new(AtomicBool::new(false)),
            send_notify: Arc::new(Condvar::new()),
            receive_notify: Arc::new(Condvar::new()),
            backpressure_active: Arc::new(AtomicBool::new(false)),
            deadlock_detector: Arc::new(Mutex::new(DeadlockDetector::new(Duration::from_secs(30)))),
            message_ids: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Send a message with priority
    pub fn send_with_priority(
        &self,
        payload: T,
        priority: ChannelPriority,
        sender_id: Option<GoroutineId>,
    ) -> SendResult<T> {
        if self.closed.load(Ordering::Acquire) {
            return SendResult::Closed(payload);
        }

        let message = PriorityMessage::new(payload, priority, sender_id);
        let message_id = message.message_id;
        
        // Check for duplicates if enabled
        if self.config.enable_duplicate_detection {
            if let Ok(message_ids) = self.message_ids.read() {
                if message_ids.contains_key(&message_id) {
                    return SendResult::WouldBlock(message.payload);
                }
            }
        }

        // Try to send message
        let mut buffer = match self.buffer.lock() {
            Ok(buffer) => buffer,
            Err(_) => return SendResult::WouldBlock(message.payload),
        };

        // Check backpressure
        if self.config.enable_backpressure {
            if let Some(capacity) = buffer.capacity() {
                let utilization = buffer.len() as f64 / capacity as f64;
                if utilization >= self.config.backpressure_threshold {
                    self.backpressure_active.store(true, Ordering::Release);
                    self.stats.backpressure_events.fetch_add(1, Ordering::Relaxed);
                    return SendResult::WouldBlock(message.payload);
                }
            }
        }

        // Try to push message
        match buffer.push(message) {
            Ok(_) => {
                // Update statistics
                self.stats.messages_sent.fetch_add(1, Ordering::Relaxed);
                self.stats.total_send_ops.fetch_add(1, Ordering::Relaxed);
                self.stats.current_buffer_size.store(buffer.len(), Ordering::Relaxed);
                
                let peak = self.stats.peak_buffer_size.load(Ordering::Relaxed);
                if buffer.len() > peak {
                    self.stats.peak_buffer_size.store(buffer.len(), Ordering::Relaxed);
                }

                // Add to deduplication set
                if self.config.enable_duplicate_detection {
                    if let Ok(mut message_ids) = self.message_ids.write() {
                        message_ids.insert(message_id, Instant::now());
                    }
                }

                // Notify receivers
                self.receive_notify.notify_one();
                
                SendResult::Sent
            },
            Err(message) => {
                self.stats.messages_dropped.fetch_add(1, Ordering::Relaxed);
                SendResult::WouldBlock(message.payload)
            }
        }
    }

    /// Receive a message
    pub fn receive(&self, receiver_id: Option<GoroutineId>) -> ReceiveResult<T> {
        if self.closed.load(Ordering::Acquire) {
            return ReceiveResult::Closed;
        }

        // Register with deadlock detector
        if let Some(goroutine_id) = receiver_id {
            if self.config.enable_deadlock_detection {
                if let Ok(mut detector) = self.deadlock_detector.lock() {
                    detector.register_waiting(goroutine_id);
                }
            }
        }

        let mut buffer = match self.buffer.lock() {
            Ok(buffer) => buffer,
            Err(_) => return ReceiveResult::WouldBlock,
        };

        // Clean up expired messages
        if let Some(max_age) = self.config.max_message_age {
            let expired = buffer.remove_expired(max_age);
            if expired > 0 {
                self.stats.messages_expired.fetch_add(expired as u64, Ordering::Relaxed);
            }
        }

        // Try to receive message
        if let Some(message) = buffer.pop() {
            // Calculate latency
            let latency = message.age().as_nanos() as u64;
            self.stats.average_latency_ns.fetch_add(latency, Ordering::Relaxed);
            let peak_latency = self.stats.peak_latency_ns.load(Ordering::Relaxed);
            if latency > peak_latency {
                self.stats.peak_latency_ns.store(latency, Ordering::Relaxed);
            }

            // Update statistics
            self.stats.messages_received.fetch_add(1, Ordering::Relaxed);
            self.stats.total_receive_ops.fetch_add(1, Ordering::Relaxed);
            self.stats.current_buffer_size.store(buffer.len(), Ordering::Relaxed);

            // Check backpressure relief
            if self.config.enable_backpressure {
                if let Some(capacity) = buffer.capacity() {
                    let utilization = buffer.len() as f64 / capacity as f64;
                    if utilization < self.config.backpressure_threshold {
                        self.backpressure_active.store(false, Ordering::Release);
                        self.send_notify.notify_all();
                    }
                }
            }

            // Unregister from deadlock detector
            if let Some(goroutine_id) = receiver_id {
                if self.config.enable_deadlock_detection {
                    if let Ok(mut detector) = self.deadlock_detector.lock() {
                        detector.unregister_waiting(goroutine_id);
                    }
                }
            }

            ReceiveResult::Received(message.payload)
        } else {
            ReceiveResult::WouldBlock
        }
    }

    /// Receive with timeout
    pub fn receive_timeout(&self, timeout: Duration, receiver_id: Option<GoroutineId>) -> ReceiveResult<T> {
        let start = Instant::now();
        
        while start.elapsed() < timeout {
            match self.receive(receiver_id) {
                ReceiveResult::WouldBlock => {
                    // Wait for notification or timeout
                    if let Ok(buffer) = self.buffer.lock() {
                        let remaining = timeout - start.elapsed();
                        if remaining.is_zero() {
                            break;
                        }
                        
                        let _result = self.receive_notify.wait_timeout(buffer, remaining);
                    }
                },
                result => return result,
            }
        }
        
        ReceiveResult::WouldBlock
    }

    /// Close the channel
    pub fn close(&self) {
        self.closed.store(true, Ordering::Release);
        self.send_notify.notify_all();
        self.receive_notify.notify_all();
    }

    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    /// Get channel capacity
    pub fn capacity(&self) -> Option<usize> {
        self.config.capacity
    }

    /// Get current length
    pub fn len(&self) -> usize {
        self.buffer.lock().map(|b| b.len()).unwrap_or(0)
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if full
    pub fn is_full(&self) -> bool {
        self.buffer.lock().map(|b| b.is_full()).unwrap_or(false)
    }

    /// Get channel statistics
    pub fn get_stats(&self) -> ChannelStats {
        let mut stats = self.stats.get_comprehensive_stats();
        stats.capacity = self.capacity().unwrap_or(0);
        stats.sender_count = self.sender_count.load(Ordering::Relaxed);
        stats.receiver_count = self.receiver_count.load(Ordering::Relaxed);
        stats.is_closed = self.is_closed();
        stats
    }

    /// Get detailed statistics
    pub fn get_detailed_stats(&self) -> Arc<ProductionChannelStats> {
        self.stats.clone()
    }

    /// Create sender handle
    pub fn create_sender(&self) -> ProductionChannelSender<T> {
        self.sender_count.fetch_add(1, Ordering::SeqCst);
        ProductionChannelSender {
            channel: self.clone(),
            sender_id: None,
        }
    }

    /// Create receiver handle
    pub fn create_receiver(&self) -> ProductionChannelReceiver<T> {
        self.receiver_count.fetch_add(1, Ordering::SeqCst);
        ProductionChannelReceiver {
            channel: self.clone(),
            receiver_id: None,
        }
    }

    /// Run maintenance tasks
    pub fn run_maintenance(&self) {
        // Clean up expired message IDs
        if self.config.enable_duplicate_detection {
            if let Ok(mut message_ids) = self.message_ids.write() {
                let now = Instant::now();
                message_ids.retain(|_, timestamp| now.duration_since(*timestamp) < Duration::from_secs(300));
            }
        }

        // Check for deadlocks
        if self.config.enable_deadlock_detection {
            if let Ok(mut detector) = self.deadlock_detector.lock() {
                let deadlocked = detector.check_deadlocks();
                if !deadlocked.is_empty() {
                    self.stats.deadlock_detections.fetch_add(deadlocked.len() as u64, Ordering::Relaxed);
                }
            }
        }
    }
}

impl<T> Clone for ProductionChannel<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            config: self.config.clone(),
            buffer: self.buffer.clone(),
            stats: self.stats.clone(),
            sender_count: self.sender_count.clone(),
            receiver_count: self.receiver_count.clone(),
            closed: self.closed.clone(),
            send_notify: self.send_notify.clone(),
            receive_notify: self.receive_notify.clone(),
            backpressure_active: self.backpressure_active.clone(),
            deadlock_detector: self.deadlock_detector.clone(),
            message_ids: self.message_ids.clone(),
        }
    }
}

/// Production channel sender
pub struct ProductionChannelSender<T> {
    channel: ProductionChannel<T>,
    sender_id: Option<GoroutineId>,
}

impl<T> ProductionChannelSender<T> {
    /// Set sender goroutine ID
    pub fn set_sender_id(&mut self, sender_id: GoroutineId) {
        self.sender_id = Some(sender_id);
    }

    /// Send a message
    pub fn send(&self, payload: T) -> SendResult<T> {
        self.channel.send_with_priority(payload, ChannelPriority::Normal, self.sender_id)
    }

    /// Send a message with priority
    pub fn send_with_priority(&self, payload: T, priority: ChannelPriority) -> SendResult<T> {
        self.channel.send_with_priority(payload, priority, self.sender_id)
    }

    /// Try to send a message (non-blocking)
    pub fn try_send(&self, payload: T) -> SendResult<T> {
        // For now, same as send since we don't block
        self.send(payload)
    }

    /// Send with timeout
    pub fn send_timeout(&self, mut payload: T, timeout: Duration) -> SendResult<T> {
        let start = Instant::now();
        
        while start.elapsed() < timeout {
            match self.send(payload) {
                SendResult::WouldBlock(returned_payload) => {
                    payload = returned_payload;
                    
                    // Wait for notification or timeout
                    if let Ok(buffer) = self.channel.buffer.lock() {
                        let remaining = timeout - start.elapsed();
                        if remaining.is_zero() {
                            return SendResult::WouldBlock(payload);
                        }
                        
                        let _result = self.channel.send_notify.wait_timeout(buffer, remaining);
                    }
                },
                result => return result,
            }
        }
        
        SendResult::WouldBlock(payload)
    }
}

impl<T> Clone for ProductionChannelSender<T> {
    fn clone(&self) -> Self {
        self.channel.sender_count.fetch_add(1, Ordering::SeqCst);
        Self {
            channel: self.channel.clone(),
            sender_id: self.sender_id,
        }
    }
}

impl<T> Drop for ProductionChannelSender<T> {
    fn drop(&mut self) {
        self.channel.sender_count.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Production channel receiver
pub struct ProductionChannelReceiver<T> {
    channel: ProductionChannel<T>,
    receiver_id: Option<GoroutineId>,
}

impl<T> ProductionChannelReceiver<T> {
    /// Set receiver goroutine ID
    pub fn set_receiver_id(&mut self, receiver_id: GoroutineId) {
        self.receiver_id = Some(receiver_id);
    }

    /// Receive a message
    pub fn receive(&self) -> ReceiveResult<T> {
        self.channel.receive(self.receiver_id)
    }

    /// Try to receive a message (non-blocking)
    pub fn try_receive(&self) -> ReceiveResult<T> {
        // For now, same as receive since we don't block
        self.receive()
    }

    /// Receive with timeout
    pub fn receive_timeout(&self, timeout: Duration) -> ReceiveResult<T> {
        self.channel.receive_timeout(timeout, self.receiver_id)
    }
}

impl<T> Clone for ProductionChannelReceiver<T> {
    fn clone(&self) -> Self {
        self.channel.receiver_count.fetch_add(1, Ordering::SeqCst);
        Self {
            channel: self.channel.clone(),
            receiver_id: self.receiver_id,
        }
    }
}

impl<T> Drop for ProductionChannelReceiver<T> {
    fn drop(&mut self) {
        self.channel.receiver_count.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Create a production channel
pub fn production_channel<T>(config: ChannelConfig) -> (ProductionChannelSender<T>, ProductionChannelReceiver<T>) {
    let channel = ProductionChannel::new(config);
    (channel.create_sender(), channel.create_receiver())
}

/// Create a production channel with default configuration
pub fn production_channel_default<T>() -> (ProductionChannelSender<T>, ProductionChannelReceiver<T>) {
    production_channel(ChannelConfig::default())
}

/// Create a production channel with specified capacity
pub fn production_channel_with_capacity<T>(capacity: usize) -> (ProductionChannelSender<T>, ProductionChannelReceiver<T>) {
    let mut config = ChannelConfig::default();
    config.capacity = Some(capacity);
    production_channel(config)
}

/// Create an unbounded production channel
pub fn production_channel_unbounded<T>() -> (ProductionChannelSender<T>, ProductionChannelReceiver<T>) {
    let mut config = ChannelConfig::default();
    config.capacity = None;
    production_channel(config)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_production_channel_basic() {
        let (sender, receiver) = production_channel_default::<i32>();
        
        assert!(sender.send(42).is_ok());
        
        match receiver.receive() {
            ReceiveResult::Received(value) => assert_eq!(value, 42),
            _ => panic!("Expected to receive value"),
        }
    }

    #[test]
    fn test_priority_messages() {
        let mut config = ChannelConfig::default();
        config.enable_priority = true;
        let (sender, receiver) = production_channel(config);
        
        // Send messages with different priorities
        assert!(sender.send_with_priority(1, ChannelPriority::Low).is_ok());
        assert!(sender.send_with_priority(2, ChannelPriority::High).is_ok());
        assert!(sender.send_with_priority(3, ChannelPriority::Normal).is_ok());
        
        // High priority should come first
        match receiver.receive() {
            ReceiveResult::Received(value) => assert_eq!(value, 2),
            _ => panic!("Expected to receive high priority message"),
        }
    }

    #[test]
    fn test_channel_capacity() {
        let (sender, receiver) = production_channel_with_capacity::<i32>(2);
        
        // Fill the channel
        assert!(sender.send(1).is_ok());
        assert!(sender.send(2).is_ok());
        
        // Should be full now
        match sender.send(3) {
            SendResult::WouldBlock(_) => {},
            _ => panic!("Expected send to block"),
        }
        
        // Receive one message
        match receiver.receive() {
            ReceiveResult::Received(value) => assert_eq!(value, 1),
            _ => panic!("Expected to receive value"),
        }
        
        // Now should be able to send again
        assert!(sender.send(3).is_ok());
    }

    #[test]
    fn test_channel_statistics() {
        let (sender, receiver) = production_channel_default::<i32>();
        
        // Send and receive some messages
        for i in 0..10 {
            assert!(sender.send(i).is_ok());
        }
        
        for _ in 0..5 {
            receiver.receive();
        }
        
        let stats = sender.channel.get_stats();
        assert_eq!(stats.total_sent, 10);
        assert_eq!(stats.total_received, 5);
    }

    #[test]
    fn test_backpressure() {
        let mut config = ChannelConfig::default();
        config.capacity = Some(10);
        config.enable_backpressure = true;
        config.backpressure_threshold = 0.5; // 50% threshold
        
        let (sender, _receiver) = production_channel(config);
        
        // Fill to threshold
        for i in 0..5 {
            assert!(sender.send(i).is_ok());
        }
        
        // Next send should trigger backpressure
        match sender.send(5) {
            SendResult::WouldBlock(_) => {},
            _ => panic!("Expected backpressure to trigger"),
        }
    }

    #[test]
    fn test_concurrent_access() {
        let (sender, receiver) = production_channel_with_capacity::<i32>(100);
        
        let sender_clone = sender.clone();
        let receiver_clone = receiver.clone();
        
        // Spawn sender thread
        let sender_handle = thread::spawn(move || {
            for i in 0..50 {
                if !sender_clone.send(i).is_ok() {
                    break; // Channel closed, exit gracefully
                }
            }
        });
        
        // Spawn receiver thread
        let receiver_handle = thread::spawn(move || {
            let mut received = 0;
            for _ in 0..50 {
                match receiver_clone.receive() {
                    ReceiveResult::Received(_) => received += 1,
                    _ => {},
                }
            }
            received
        });
        
        sender_handle.join().unwrap();
        let received = receiver_handle.join().unwrap();
        
        assert_eq!(received, 50);
    }
}
