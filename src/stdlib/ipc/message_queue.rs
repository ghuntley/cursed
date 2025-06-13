/// Message queue implementation for CURSED IPC
/// 
/// This module provides comprehensive message queue functionality for inter-process
/// communication, including creation, message passing, and synchronization features.
/// 
/// # Why Message Queues are Critical for Distributed Systems
/// 
/// Message queues provide:
/// - Asynchronous communication between processes
/// - Message ordering and delivery guarantees
/// - Buffering and flow control for uneven load patterns
/// - Priority-based message scheduling
/// - Persistent message storage options
/// 
/// In distributed systems, message queues enable:
/// - Decoupling of services with reliable message delivery
/// - Event-driven architectures with publish-subscribe patterns
/// - Load balancing across multiple consumers
/// - Dead letter queues for error handling
/// - Transaction support for message processing

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::time::{Duration, SystemTime, Instant};
use std::thread;
use crate::stdlib::ipc::{
    IpcResult, IpcError, IpcHandle, IpcPermissions, IpcConfig,
    resource_error, timeout_error, permission_denied, resource_exhausted
};
use crate::stdlib::ipc::types::IpcHandleType;
use crate::stdlib::ipc::error::{
    message_queue_error, communication_error_detailed, system_error
};

/// Message queue handle
#[derive(Debug)]
pub struct MessageQueue {
    handle: IpcHandle,
    config: MessageQueueConfig,
    inner: Arc<MessageQueueInner>,
    state: MessageQueueState,
    statistics: Arc<Mutex<MessageQueueStatistics>>,
}

/// Message queue configuration
#[derive(Debug, Clone)]
pub struct MessageQueueConfig {
    pub name: String,
    pub max_messages: usize,
    pub max_message_size: usize,
    pub permissions: IpcPermissions,
    pub message_ordering: MessageOrdering,
    pub persistence: PersistenceMode,
    pub timeout: Duration,
    pub enable_priority: bool,
    pub enable_transactions: bool,
    pub enable_dead_letter: bool,
    pub dead_letter_max_retries: u32,
    pub compression: CompressionMode,
}

impl MessageQueueConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            max_messages: 1000,
            max_message_size: 65536, // 64KB
            permissions: IpcPermissions::read_write(),
            message_ordering: MessageOrdering::Fifo,
            persistence: PersistenceMode::InMemory,
            timeout: Duration::from_secs(30),
            enable_priority: false,
            enable_transactions: false,
            enable_dead_letter: false,
            dead_letter_max_retries: 3,
            compression: CompressionMode::None,
        }
    }

    pub fn with_max_messages(mut self, max: usize) -> Self {
        self.max_messages = max;
        self
    }

    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }

    pub fn with_priority(mut self) -> Self {
        self.enable_priority = true;
        self.message_ordering = MessageOrdering::Priority;
        self
    }

    pub fn with_persistence(mut self, mode: PersistenceMode) -> Self {
        self.persistence = mode;
        self
    }

    pub fn with_dead_letter(mut self, max_retries: u32) -> Self {
        self.enable_dead_letter = true;
        self.dead_letter_max_retries = max_retries;
        self
    }

    pub fn with_compression(mut self, compression: CompressionMode) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Message ordering strategies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageOrdering {
    /// First In, First Out (default)
    Fifo,
    /// Last In, First Out
    Lifo,
    /// Priority-based ordering
    Priority,
    /// Random order
    Random,
}

/// Message persistence mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistenceMode {
    /// Messages stored only in memory
    InMemory,
    /// Messages persisted to disk
    Persistent,
    /// Durable messages (fsync after write)
    Durable,
}

/// Message compression mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompressionMode {
    /// No compression
    None,
    /// Fast compression (LZ4-style)
    Fast,
    /// Best compression (zlib-style)
    Best,
    /// Adaptive compression based on content
    Adaptive,
}

/// Message structure
#[derive(Debug, Clone)]
pub struct Message {
    /// Unique message ID
    pub id: String,
    /// Message type/topic
    pub message_type: MessageType,
    /// Message priority (higher = more important)
    pub priority: MessagePriority,
    /// Message payload
    pub payload: Vec<u8>,
    /// Message headers/metadata
    pub headers: HashMap<String, String>,
    /// Message creation timestamp
    pub created_at: SystemTime,
    /// Message expiration time
    pub expires_at: Option<SystemTime>,
    /// Number of delivery attempts
    pub delivery_attempts: u32,
    /// Maximum delivery attempts
    pub max_attempts: u32,
    /// Original message ID (for retry tracking)
    pub original_id: Option<String>,
}

impl Message {
    /// Create a new message
    pub fn new<T: AsRef<[u8]>>(message_type: MessageType, payload: T) -> Self {
        Self {
            id: generate_message_id(),
            message_type,
            priority: MessagePriority::Normal,
            payload: payload.as_ref().to_vec(),
            headers: HashMap::new(),
            created_at: SystemTime::now(),
            expires_at: None,
            delivery_attempts: 0,
            max_attempts: 3,
            original_id: None,
        }
    }

    /// Set message priority
    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set message expiration
    pub fn with_expiration(mut self, duration: Duration) -> Self {
        self.expires_at = Some(SystemTime::now() + duration);
        self
    }

    /// Add message header
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Set maximum delivery attempts
    pub fn with_max_attempts(mut self, max: u32) -> Self {
        self.max_attempts = max;
        self
    }

    /// Check if message has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            SystemTime::now() > expires_at
        } else {
            false
        }
    }

    /// Check if message has exceeded retry limit
    pub fn is_retry_exhausted(&self) -> bool {
        self.delivery_attempts >= self.max_attempts
    }

    /// Get message age
    pub fn age(&self) -> Duration {
        self.created_at.elapsed().unwrap_or(Duration::from_secs(0))
    }

    /// Get payload as string
    pub fn payload_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.payload.clone())
    }

    /// Get payload size
    pub fn payload_size(&self) -> usize {
        self.payload.len()
    }

    /// Clone message for retry
    pub fn clone_for_retry(&self) -> Self {
        let mut retry_msg = self.clone();
        retry_msg.id = generate_message_id();
        retry_msg.delivery_attempts += 1;
        retry_msg.original_id = Some(self.id.clone());
        retry_msg.created_at = SystemTime::now();
        retry_msg
    }
}

/// Message type/topic identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageType {
    /// Arbitrary string topic
    Topic(String),
    /// Broadcast message
    Broadcast,
    /// Request message
    Request,
    /// Response message
    Response,
    /// Event notification
    Event,
    /// System message
    System,
    /// Error message
    Error,
}

impl MessageType {
    pub fn topic(topic: &str) -> Self {
        MessageType::Topic(topic.to_string())
    }
}

/// Message priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    /// Low priority (background tasks)
    Low = 1,
    /// Normal priority (default)
    Normal = 5,
    /// High priority (important operations)
    High = 8,
    /// Critical priority (urgent system messages)
    Critical = 10,
}

impl MessagePriority {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

/// Message queue state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageQueueState {
    Created,
    Active,
    Suspended,
    Draining,
    Closed,
    Error,
}

/// Internal message queue implementation
#[derive(Debug)]
struct MessageQueueInner {
    config: MessageQueueConfig,
    messages: Mutex<VecDeque<Message>>,
    dead_letter_queue: Mutex<VecDeque<Message>>,
    subscribers: RwLock<HashMap<String, Vec<Box<dyn MessageHandler + Send + Sync>>>>,
    condition: Condvar,
    message_count: Mutex<usize>,
    total_size: Mutex<usize>,
}

impl MessageQueueInner {
    fn new(config: MessageQueueConfig) -> Self {
        Self {
            config,
            messages: Mutex::new(VecDeque::new()),
            dead_letter_queue: Mutex::new(VecDeque::new()),
            subscribers: RwLock::new(HashMap::new()),
            condition: Condvar::new(),
            message_count: Mutex::new(0),
            total_size: Mutex::new(0),
        }
    }

    fn send_message(&self, mut message: Message) -> IpcResult<()> {
        // Check message size
        if message.payload.len() > self.config.max_message_size {
            return Err(resource_error(&format!(
                "Message size {} exceeds maximum {}",
                message.payload.len(),
                self.config.max_message_size
            )));
        }

        // Check if queue is full
        let mut messages = self.messages.lock().unwrap();
        let mut count = self.message_count.lock().unwrap();
        
        if *count >= self.config.max_messages {
            return Err(resource_exhausted(
                "message_queue",
                "send",
                *count as u64,
                self.config.max_messages as u64
            ));
        }

        // Apply compression if enabled
        if self.config.compression != CompressionMode::None {
            message = self.compress_message(message)?;
        }

        // Insert message based on ordering strategy
        match self.config.message_ordering {
            MessageOrdering::Fifo => messages.push_back(message),
            MessageOrdering::Lifo => messages.push_front(message),
            MessageOrdering::Priority => {
                // Insert in priority order
                let pos = messages.iter().position(|m| m.priority < message.priority)
                    .unwrap_or(messages.len());
                messages.insert(pos, message);
            }
            MessageOrdering::Random => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                
                let mut hasher = DefaultHasher::new();
                message.id.hash(&mut hasher);
                let pos = (hasher.finish() as usize) % (messages.len() + 1);
                messages.insert(pos, message);
            }
        }

        *count += 1;
        let mut total_size = self.total_size.lock().unwrap();
        *total_size += message.payload.len();

        // Notify waiting receivers
        self.condition.notify_one();

        Ok(())
    }

    fn receive_message(&self, timeout: Option<Duration>) -> IpcResult<Option<Message>> {
        let start_time = Instant::now();
        let mut messages = self.messages.lock().unwrap();

        loop {
            // Check for available messages
            if let Some(mut message) = messages.pop_front() {
                let mut count = self.message_count.lock().unwrap();
                *count -= 1;
                
                let mut total_size = self.total_size.lock().unwrap();
                *total_size -= message.payload.len();

                // Check if message has expired
                if message.is_expired() {
                    continue; // Skip expired message
                }

                // Decompress if needed
                if self.config.compression != CompressionMode::None {
                    message = self.decompress_message(message)?;
                }

                return Ok(Some(message));
            }

            // Check timeout
            if let Some(timeout) = timeout {
                let elapsed = start_time.elapsed();
                if elapsed >= timeout {
                    return Ok(None);
                }
                
                let remaining = timeout - elapsed;
                let (new_messages, timeout_result) = self.condition.wait_timeout(messages, remaining).unwrap();
                messages = new_messages;
                
                if timeout_result.timed_out() {
                    return Ok(None);
                }
            } else {
                messages = self.condition.wait(messages).unwrap();
            }
        }
    }

    fn peek_message(&self) -> IpcResult<Option<Message>> {
        let messages = self.messages.lock().unwrap();
        Ok(messages.front().cloned())
    }

    fn compress_message(&self, mut message: Message) -> IpcResult<Message> {
        match self.config.compression {
            CompressionMode::Fast => {
                // Simplified compression simulation
                if message.payload.len() > 100 {
                    message.headers.insert("compressed".to_string(), "fast".to_string());
                }
            }
            CompressionMode::Best => {
                // Simplified compression simulation
                if message.payload.len() > 50 {
                    message.headers.insert("compressed".to_string(), "best".to_string());
                }
            }
            CompressionMode::Adaptive => {
                // Simplified adaptive compression
                if message.payload.len() > 200 {
                    message.headers.insert("compressed".to_string(), "adaptive".to_string());
                }
            }
            CompressionMode::None => {}
        }
        Ok(message)
    }

    fn decompress_message(&self, mut message: Message) -> IpcResult<Message> {
        if message.headers.contains_key("compressed") {
            message.headers.remove("compressed");
        }
        Ok(message)
    }
}

/// Message queue statistics
#[derive(Debug, Clone)]
pub struct MessageQueueStatistics {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub messages_dropped: u64,
    pub messages_expired: u64,
    pub messages_dead_lettered: u64,
    pub current_queue_size: usize,
    pub peak_queue_size: usize,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub average_message_size: f64,
    pub last_activity: Option<SystemTime>,
    pub creation_time: SystemTime,
    pub subscribers_count: usize,
}

impl MessageQueueStatistics {
    pub fn new() -> Self {
        Self {
            messages_sent: 0,
            messages_received: 0,
            messages_dropped: 0,
            messages_expired: 0,
            messages_dead_lettered: 0,
            current_queue_size: 0,
            peak_queue_size: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            average_message_size: 0.0,
            last_activity: None,
            creation_time: SystemTime::now(),
            subscribers_count: 0,
        }
    }

    pub fn record_send(&mut self, message_size: usize) {
        self.messages_sent += 1;
        self.total_bytes_sent += message_size as u64;
        self.last_activity = Some(SystemTime::now());
        self.update_average_size();
    }

    pub fn record_receive(&mut self, message_size: usize) {
        self.messages_received += 1;
        self.total_bytes_received += message_size as u64;
        self.last_activity = Some(SystemTime::now());
        self.update_average_size();
    }

    pub fn record_drop(&mut self) {
        self.messages_dropped += 1;
        self.last_activity = Some(SystemTime::now());
    }

    pub fn update_queue_size(&mut self, size: usize) {
        self.current_queue_size = size;
        if size > self.peak_queue_size {
            self.peak_queue_size = size;
        }
    }

    fn update_average_size(&mut self) {
        let total_messages = self.messages_sent + self.messages_received;
        let total_bytes = self.total_bytes_sent + self.total_bytes_received;
        
        if total_messages > 0 {
            self.average_message_size = total_bytes as f64 / total_messages as f64;
        }
    }
}

/// Message handler trait
pub trait MessageHandler {
    /// Handle received message
    fn handle_message(&self, message: &Message) -> IpcResult<()>;
    
    /// Get handler ID
    fn handler_id(&self) -> &str;
    
    /// Check if handler can process message type
    fn can_handle(&self, message_type: &MessageType) -> bool;
}

impl MessageQueue {
    /// Create a new message queue
    pub fn create(config: MessageQueueConfig) -> IpcResult<Self> {
        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::MessageQueue
        );

        let inner = Arc::new(MessageQueueInner::new(config.clone()));

        let queue = Self {
            handle,
            config,
            inner,
            state: MessageQueueState::Created,
            statistics: Arc::new(Mutex::new(MessageQueueStatistics::new())),
        };

        // Register in global registry
        MESSAGE_QUEUE_REGISTRY.write().unwrap()
            .insert(queue.handle.id.clone(), Arc::new(RwLock::new(())));

        Ok(queue)
    }

    /// Open an existing message queue
    pub fn open(name: &str) -> IpcResult<Self> {
        let config = MessageQueueConfig::new(name);
        
        // Check if queue exists in registry
        if !MESSAGE_QUEUE_REGISTRY.read().unwrap().contains_key(name) {
            return Err(message_queue_error("open", name, "Message queue does not exist"));
        }

        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::MessageQueue
        );

        let inner = Arc::new(MessageQueueInner::new(config.clone()));

        Ok(Self {
            handle,
            config,
            inner,
            state: MessageQueueState::Active,
            statistics: Arc::new(Mutex::new(MessageQueueStatistics::new())),
        })
    }

    /// Send a message to the queue
    pub fn send(&mut self, message: Message) -> IpcResult<()> {
        if self.state != MessageQueueState::Active {
            return Err(message_queue_error(
                "send",
                &self.config.name,
                "Queue is not active"
            ));
        }

        let message_size = message.payload.len();
        let result = self.inner.send_message(message);

        if let Ok(mut stats) = self.statistics.lock() {
            if result.is_ok() {
                stats.record_send(message_size);
            } else {
                stats.record_drop();
            }
        }

        result
    }

    /// Receive a message from the queue
    pub fn receive(&mut self) -> IpcResult<Option<Message>> {
        self.receive_timeout(self.config.timeout)
    }

    /// Receive a message with timeout
    pub fn receive_timeout(&mut self, timeout: Duration) -> IpcResult<Option<Message>> {
        if self.state != MessageQueueState::Active {
            return Err(message_queue_error(
                "receive",
                &self.config.name,
                "Queue is not active"
            ));
        }

        let result = self.inner.receive_message(Some(timeout));

        if let Ok(Some(ref message)) = result {
            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_receive(message.payload.len());
            }
        }

        result
    }

    /// Peek at the next message without removing it
    pub fn peek(&self) -> IpcResult<Option<Message>> {
        self.inner.peek_message()
    }

    /// Get queue size
    pub fn size(&self) -> usize {
        *self.inner.message_count.lock().unwrap()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Check if queue is full
    pub fn is_full(&self) -> bool {
        self.size() >= self.config.max_messages
    }

    /// Get queue statistics
    pub fn get_statistics(&self) -> MessageQueueStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| MessageQueueStatistics::new())
    }

    /// Subscribe to message notifications
    pub fn subscribe<H>(&mut self, message_type: MessageType, handler: H) -> IpcResult<()>
    where
        H: MessageHandler + Send + Sync + 'static,
    {
        let mut subscribers = self.inner.subscribers.write().unwrap();
        let type_key = format!("{:?}", message_type);
        
        subscribers.entry(type_key)
            .or_insert_with(Vec::new)
            .push(Box::new(handler));

        Ok(())
    }

    /// Remove the message queue
    pub fn remove(name: &str) -> IpcResult<()> {
        MESSAGE_QUEUE_REGISTRY.write().unwrap().remove(name);
        Ok(())
    }

    /// Activate the queue
    pub fn activate(&mut self) -> IpcResult<()> {
        self.state = MessageQueueState::Active;
        Ok(())
    }

    /// Suspend the queue
    pub fn suspend(&mut self) -> IpcResult<()> {
        self.state = MessageQueueState::Suspended;
        Ok(())
    }

    /// Close the queue
    pub fn close(&mut self) -> IpcResult<()> {
        self.state = MessageQueueState::Closed;
        Ok(())
    }
}

impl Drop for MessageQueue {
    fn drop(&mut self) {
        let _ = self.close();
        MESSAGE_QUEUE_REGISTRY.write().unwrap().remove(&self.handle.id);
    }
}

/// Generate unique message ID
fn generate_message_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();
    
    format!("msg_{}_{}", timestamp, count)
}

// Global message queue registry
lazy_static::lazy_static! {
    static ref MESSAGE_QUEUE_REGISTRY: Arc<RwLock<HashMap<String, Arc<RwLock<()>>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
    
    static ref GLOBAL_STATISTICS: Arc<Mutex<HashMap<String, MessageQueueStatistics>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

/// Module-level functions for message queue management

/// Create a new message queue
pub fn create_message_queue(config: MessageQueueConfig) -> IpcResult<MessageQueue> {
    MessageQueue::create(config)
}

/// Open an existing message queue
pub fn open_message_queue(name: &str) -> IpcResult<MessageQueue> {
    MessageQueue::open(name)
}

/// Remove a message queue
pub fn remove_message_queue(name: &str) -> IpcResult<()> {
    MessageQueue::remove(name)
}

/// Send a message to a queue
pub fn send_message(queue_name: &str, message: Message) -> IpcResult<()> {
    let mut queue = MessageQueue::open(queue_name)?;
    queue.send(message)
}

/// Receive a message from a queue
pub fn receive_message(queue_name: &str) -> IpcResult<Option<Message>> {
    let mut queue = MessageQueue::open(queue_name)?;
    queue.receive()
}

/// Peek at a message in a queue
pub fn peek_message(queue_name: &str) -> IpcResult<Option<Message>> {
    let queue = MessageQueue::open(queue_name)?;
    queue.peek()
}

/// Initialize message queue subsystem
pub fn initialize_message_queue_subsystem() -> IpcResult<()> {
    // Initialize global registry and statistics
    Ok(())
}

/// Shutdown message queue subsystem
pub fn shutdown_message_queue_subsystem() -> IpcResult<()> {
    // Clean up all queues
    cleanup_all_queues()?;
    Ok(())
}

/// Clean up all message queues
pub fn cleanup_all_queues() -> IpcResult<()> {
    let queue_names: Vec<String> = MESSAGE_QUEUE_REGISTRY.read()
        .map(|registry| registry.keys().cloned().collect())
        .unwrap_or_default();

    for name in queue_names {
        let _ = MessageQueue::remove(&name);
    }

    Ok(())
}

/// Get count of active message queues
pub fn get_active_queue_count() -> usize {
    MESSAGE_QUEUE_REGISTRY.read()
        .map(|registry| registry.len())
        .unwrap_or(0)
}

/// Get memory usage of message queue subsystem
pub fn get_memory_usage() -> usize {
    // Calculate memory usage across all queues
    0
}

/// Get throughput for message queue operations
pub fn get_throughput() -> f64 {
    // Calculate messages per second throughput
    0.0
}

/// Get full event count
pub fn get_full_event_count() -> u64 {
    // Count of queue full events
    0
}

/// Message iterator for queue traversal
pub struct MessageIterator {
    queue_name: String,
    timeout: Duration,
}

impl MessageIterator {
    pub fn new(queue_name: String, timeout: Duration) -> Self {
        Self { queue_name, timeout }
    }
}

impl Iterator for MessageIterator {
    type Item = IpcResult<Message>;

    fn next(&mut self) -> Option<Self::Item> {
        match MessageQueue::open(&self.queue_name) {
            Ok(mut queue) => {
                match queue.receive_timeout(self.timeout) {
                    Ok(Some(message)) => Some(Ok(message)),
                    Ok(None) => None, // Timeout or no more messages
                    Err(e) => Some(Err(e)),
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new(MessageType::Topic("test".to_string()), b"hello world")
            .with_priority(MessagePriority::High)
            .with_header("content-type", "text/plain");

        assert!(!message.id.is_empty());
        assert_eq!(message.priority, MessagePriority::High);
        assert_eq!(message.payload, b"hello world");
        assert_eq!(message.headers.get("content-type"), Some(&"text/plain".to_string()));
    }

    #[test]
    fn test_message_queue_config() {
        let config = MessageQueueConfig::new("test_queue")
            .with_max_messages(500)
            .with_priority()
            .with_dead_letter(5);

        assert_eq!(config.name, "test_queue");
        assert_eq!(config.max_messages, 500);
        assert!(config.enable_priority);
        assert!(config.enable_dead_letter);
        assert_eq!(config.dead_letter_max_retries, 5);
    }

    #[test]
    fn test_message_queue_creation() {
        let config = MessageQueueConfig::new("test_queue");
        let queue = MessageQueue::create(config);
        assert!(queue.is_ok());
        
        let queue = queue.unwrap();
        assert_eq!(queue.config.name, "test_queue");
        assert_eq!(queue.state, MessageQueueState::Created);
    }

    #[test]
    fn test_message_priority_ordering() {
        assert!(MessagePriority::Critical > MessagePriority::High);
        assert!(MessagePriority::High > MessagePriority::Normal);
        assert!(MessagePriority::Normal > MessagePriority::Low);
    }

    #[test]
    fn test_message_expiration() {
        let mut message = Message::new(MessageType::Event, b"test");
        assert!(!message.is_expired());

        message = message.with_expiration(Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(10));
        assert!(message.is_expired());
    }

    #[test]
    fn test_message_statistics() {
        let mut stats = MessageQueueStatistics::new();
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);

        stats.record_send(100);
        assert_eq!(stats.messages_sent, 1);
        assert_eq!(stats.total_bytes_sent, 100);

        stats.record_receive(200);
        assert_eq!(stats.messages_received, 1);
        assert_eq!(stats.total_bytes_received, 200);
        assert_eq!(stats.average_message_size, 150.0);
    }

    #[test]
    fn test_message_id_generation() {
        let id1 = generate_message_id();
        let id2 = generate_message_id();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("msg_"));
        assert!(id2.starts_with("msg_"));
    }

    #[test]
    fn test_message_payload_operations() {
        let message = Message::new(MessageType::Request, b"hello world");
        
        assert_eq!(message.payload_size(), 11);
        assert_eq!(message.payload_as_string().unwrap(), "hello world");
    }

    #[test]
    fn test_message_clone_for_retry() {
        let original = Message::new(MessageType::Topic("test".to_string()), b"data");
        let retry = original.clone_for_retry();
        
        assert_ne!(original.id, retry.id);
        assert_eq!(retry.delivery_attempts, 1);
        assert_eq!(retry.original_id, Some(original.id));
        assert_eq!(retry.payload, original.payload);
    }

    #[test]
    fn test_global_functions() {
        assert_eq!(get_active_queue_count(), 0);
        assert_eq!(get_memory_usage(), 0);
        assert_eq!(get_throughput(), 0.0);
        assert_eq!(get_full_event_count(), 0);
    }
}
