/// Simple message queue implementation for CURSED IPC
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, BufWriter, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};
use crate::stdlib::ipc::error::{IpcError, IpcResult};
use serde::{Serialize, Deserialize};

/// Message queue configuration
#[derive(Debug, Clone)]
pub struct QueueConfig {
    pub name: String,
    pub max_size: usize,
    pub max_message_size: usize,
    pub persistent: bool,
    pub permissions: u32,
}

impl QueueConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            max_size: 100,
            max_message_size: 8192,
            persistent: true,
            permissions: 0o600,
        }
    }

    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_size = size;
        self
    }

    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }

    pub fn in_memory(mut self) -> Self {
        self.persistent = false;
        self
    }

    pub fn with_permissions(mut self, permissions: u32) -> Self {
        self.permissions = permissions;
        self
    }
}

/// Message type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Binary,
    Json,
    Command,
    Event,
    Response,
}

/// Message in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub msg_type: MessageType,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub priority: u8,
    pub sender: Option<String>,
    pub reply_to: Option<u64>,
}

impl Message {
    /// Create a new text message
    pub fn new_text(data: &str) -> Self {
        Self {
            id: generate_message_id(),
            msg_type: MessageType::Text,
            data: data.as_bytes().to_vec(),
            timestamp: timestamp_now(),
            priority: 50, // Default priority
            sender: None,
            reply_to: None,
        }
    }

    /// Create a new binary message
    pub fn new_binary(data: &[u8]) -> Self {
        Self {
            id: generate_message_id(),
            msg_type: MessageType::Binary,
            data: data.to_vec(),
            timestamp: timestamp_now(),
            priority: 50,
            sender: None,
            reply_to: None,
        }
    }

    /// Create a new JSON message
    pub fn new_json(data: &str) -> IpcResult<Self> {
        // Validate JSON
        serde_json::from_str::<serde_json::Value>(data)
            .map_err(|e| IpcError::InvalidInput(format!("Invalid JSON: {}", e)))?;
        
        Ok(Self {
            id: generate_message_id(),
            msg_type: MessageType::Json,
            data: data.as_bytes().to_vec(),
            timestamp: timestamp_now(),
            priority: 50,
            sender: None,
            reply_to: None,
        })
    }

    /// Set message priority (0-255, higher = more important)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Set sender identifier
    pub fn with_sender(mut self, sender: &str) -> Self {
        self.sender = Some(sender.to_string());
        self
    }

    /// Set reply-to message ID
    pub fn in_reply_to(mut self, message_id: u64) -> Self {
        self.reply_to = Some(message_id);
        self
    }

    /// Get message data as string
    pub fn as_string(&self) -> IpcResult<String> {
        String::from_utf8(self.data.clone())
            .map_err(|e| IpcError::InvalidInput(format!("Invalid UTF-8: {}", e)))
    }

    /// Get message data as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Get message size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Check if message is expired (older than duration)
    pub fn is_expired(&self, max_age: Duration) -> bool {
        let now = timestamp_now();
        let age = Duration::from_secs(now.saturating_sub(self.timestamp));
        age > max_age
    }
}

/// In-memory message queue with optional persistence
pub struct MessageQueue {
    config: QueueConfig,
    messages: Arc<Mutex<VecDeque<Message>>>,
    file: Option<BufWriter<File>>,
    next_id: Arc<Mutex<u64>>,
    total_sent: Arc<Mutex<u64>>,
    total_received: Arc<Mutex<u64>>,
}

impl MessageQueue {
    /// Create a new message queue
    pub fn create(name: &str) -> IpcResult<Self> {
        let config = QueueConfig::new(name);
        Self::create_with_config(config)
    }

    /// Create message queue with custom configuration
    pub fn create_with_config(config: QueueConfig) -> IpcResult<Self> {
        let messages = Arc::new(Mutex::new(VecDeque::with_capacity(config.max_size)));
        let next_id = Arc::new(Mutex::new(1));
        let total_sent = Arc::new(Mutex::new(0));
        let total_received = Arc::new(Mutex::new(0));

        let file = if config.persistent {
            let path = get_queue_file_path(&config.name);
            
            #[cfg(unix)]
            let mut options = OpenOptions::new()
                .create(true)
                .append(true)
                .mode(config.permissions);
            
            #[cfg(not(unix))]
            let mut options = OpenOptions::new()
                .create(true)
                .append(true);

            let file = options.open(&path).map_err(IpcError::from)?;
            Some(BufWriter::new(file))
        } else {
            None
        };

        let queue = Self {
            config: config.clone(),
            messages,
            file,
            next_id,
            total_sent,
            total_received,
        };

        // Load existing messages if persistent
        if config.persistent {
            queue.load_messages_from_file()?;
        }

        // Register with IPC registry
        let path = if config.persistent {
            get_queue_file_path(&config.name).to_string_lossy().to_string()
        } else {
            format!("memory:{}", config.name)
        };
        crate::stdlib::ipc::register_message_queue(config.name, path)?;

        Ok(queue)
    }

    /// Send a message to the queue
    pub fn send(&mut self, message: Message) -> IpcResult<u64> {
        if message.size() > self.config.max_message_size {
            return Err(IpcError::InvalidInput(format!(
                "Message size {} exceeds maximum {}",
                message.size(),
                self.config.max_message_size
            )));
        }

        crate::stdlib::ipc::increment_operations();

        let mut messages = self.messages.lock()
            .map_err(|_| IpcError::Internal("Failed to acquire messages lock".to_string()))?;

        // Check queue capacity
        if messages.len() >= self.config.max_size {
            // Remove oldest message if at capacity
            messages.pop_front();
        }

        let message_id = message.id;

        // Insert message in priority order (higher priority first)
        let insert_pos = messages.iter()
            .position(|m| m.priority < message.priority)
            .unwrap_or(messages.len());
        
        messages.insert(insert_pos, message.clone());

        // Persist to file if configured
        if let Some(file) = &mut self.file {
            let serialized = serde_json::to_string(&message)
                .map_err(|e| IpcError::Internal(format!("Failed to serialize message: {}", e)))?;
            writeln!(file, "{}", serialized)
                .map_err(|e| IpcError::IoError(format!("Failed to write message: {}", e)))?;
            file.flush().map_err(IpcError::from)?;
        }

        // Update statistics
        if let Ok(mut count) = self.total_sent.lock() {
            *count += 1;
        }

        Ok(message_id)
    }

    /// Receive a message from the queue
    pub fn receive(&mut self) -> IpcResult<Option<Message>> {
        crate::stdlib::ipc::increment_operations();

        let mut messages = self.messages.lock()
            .map_err(|_| IpcError::Internal("Failed to acquire messages lock".to_string()))?;

        match messages.pop_front() {
            Some(message) => {
                // Update statistics
                if let Ok(mut count) = self.total_received.lock() {
                    *count += 1;
                }
                Ok(Some(message))
            }
            None => Ok(None),
        }
    }

    /// Receive a message with timeout
    pub fn receive_timeout(&mut self, timeout: Duration) -> IpcResult<Option<Message>> {
        let start = SystemTime::now();
        
        loop {
            if let Some(message) = self.receive()? {
                return Ok(Some(message));
            }

            if start.elapsed().unwrap_or(Duration::ZERO) >= timeout {
                return Ok(None);
            }

            // Small delay to avoid busy waiting
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Peek at the next message without removing it
    pub fn peek(&self) -> IpcResult<Option<Message>> {
        let messages = self.messages.lock()
            .map_err(|_| IpcError::Internal("Failed to acquire messages lock".to_string()))?;
        
        Ok(messages.front().cloned())
    }

    /// Get the number of messages in the queue
    pub fn len(&self) -> usize {
        self.messages.lock()
            .map(|messages| messages.len())
            .unwrap_or(0)
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear all messages from the queue
    pub fn clear(&mut self) -> IpcResult<()> {
        let mut messages = self.messages.lock()
            .map_err(|_| IpcError::Internal("Failed to acquire messages lock".to_string()))?;
        
        messages.clear();
        Ok(())
    }

    /// Get queue statistics
    pub fn statistics(&self) -> QueueStatistics {
        QueueStatistics {
            name: self.config.name.clone(),
            current_size: self.len(),
            max_size: self.config.max_size,
            total_sent: self.total_sent.lock().map(|c| *c).unwrap_or(0),
            total_received: self.total_received.lock().map(|c| *c).unwrap_or(0),
            max_message_size: self.config.max_message_size,
            persistent: self.config.persistent,
        }
    }

    /// Load messages from persistent file
    fn load_messages_from_file(&self) -> IpcResult<()> {
        let path = get_queue_file_path(&self.config.name);
        
        if !path.exists() {
            return Ok(());
        }

        let file = File::open(&path).map_err(IpcError::from)?;
        let reader = BufReader::new(file);

        let mut messages = self.messages.lock()
            .map_err(|_| IpcError::Internal("Failed to acquire messages lock".to_string()))?;

        for line in reader.split("\n") {
            let line = line.map_err(IpcError::from)?;
            if let Ok(message) = serde_json::from_str::<Message>(&line) {
                if messages.len() < self.config.max_size {
                    messages.push_back(message);
                }
            }
        }

        Ok(())
    }
}

impl Drop for MessageQueue {
    fn drop(&mut self) {
        if let Some(file) = &mut self.file {
            let _ = file.flush();
        }
        let _ = crate::stdlib::ipc::unregister_message_queue(&self.config.name);
    }
}

/// Queue statistics
#[derive(Debug, Clone)]
pub struct QueueStatistics {
    pub name: String,
    pub current_size: usize,
    pub max_size: usize,
    pub total_sent: u64,
    pub total_received: u64,
    pub max_message_size: usize,
    pub persistent: bool,
}

/// Create a message queue
pub fn create_message_queue(name: &str) -> IpcResult<MessageQueue> {
    MessageQueue::create(name)
}

/// Open an existing message queue
pub fn open_message_queue(name: &str) -> IpcResult<MessageQueue> {
    MessageQueue::create(name) // Same as create for file-based queues
}

/// Send a message to a queue
pub fn send_message(queue: &mut MessageQueue, message: Message) -> IpcResult<u64> {
    queue.send(message)
}

/// Receive a message from a queue
pub fn receive_message(queue: &mut MessageQueue) -> IpcResult<Option<Message>> {
    queue.receive()
}

/// Remove a message queue
pub fn remove_message_queue(name: &str) -> IpcResult<()> {
    let path = get_queue_file_path(name);
    if path.exists() {
        std::fs::remove_file(path).map_err(IpcError::from)?;
    }
    Ok(())
}

// Helper functions

fn get_queue_file_path(name: &str) -> PathBuf {
    let base_path = std::env::temp_dir();
    base_path.join(format!("cursed_mq_{}.json", name))
}

static mut MESSAGE_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn generate_message_id() -> u64 {
    unsafe {
        MESSAGE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

fn timestamp_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_config() {
        let config = QueueConfig::new("test")
            .with_max_size(50)
            .with_max_message_size(4096)
            .in_memory()
            .with_permissions(0o644);

        assert_eq!(config.name, "test");
        assert_eq!(config.max_size, 50);
        assert_eq!(config.max_message_size, 4096);
        assert!(!config.persistent);
        assert_eq!(config.permissions, 0o644);
    }

    #[test]
    fn test_message_creation() {
        let msg = Message::new_text("Hello, world!")
            .with_priority(100)
            .with_sender("test_sender");

        assert_eq!(msg.msg_type, MessageType::Text);
        assert_eq!(msg.as_string().unwrap(), "Hello, world!");
        assert_eq!(msg.priority, 100);
        assert_eq!(msg.sender.as_deref(), Some("test_sender"));
    }

    #[test]
    fn test_message_queue_operations() {
        let mut queue = MessageQueue::create("test_queue").unwrap();

        // Send messages
        let msg1 = Message::new_text("First message").with_priority(50);
        let msg2 = Message::new_text("Second message").with_priority(100);
        let msg3 = Message::new_text("Third message").with_priority(25);

        let id1 = queue.send(msg1).unwrap();
        let id2 = queue.send(msg2).unwrap();
        let id3 = queue.send(msg3).unwrap();

        assert_eq!(queue.len(), 3);

        // Messages should be ordered by priority (highest first)
        let received1 = queue.receive().unwrap().unwrap();
        assert_eq!(received1.priority, 100);
        assert_eq!(received1.as_string().unwrap(), "Second message");

        let received2 = queue.receive().unwrap().unwrap();
        assert_eq!(received2.priority, 50);

        let received3 = queue.receive().unwrap().unwrap();
        assert_eq!(received3.priority, 25);

        assert!(queue.is_empty());
        assert_eq!(queue.receive().unwrap(), None);

        // Cleanup
        let _ = remove_message_queue("test_queue");
    }

    #[test]
    fn test_queue_capacity() {
        let config = QueueConfig::new("test_capacity").with_max_size(2);
        let mut queue = MessageQueue::create_with_config(config).unwrap();

        // Fill queue to capacity
        let msg1 = Message::new_text("Message 1");
        let msg2 = Message::new_text("Message 2");
        let msg3 = Message::new_text("Message 3");

        queue.send(msg1).unwrap();
        queue.send(msg2).unwrap();
        assert_eq!(queue.len(), 2);

        // Adding third message should remove oldest
        queue.send(msg3).unwrap();
        assert_eq!(queue.len(), 2);

        // First message should be gone
        let received = queue.receive().unwrap().unwrap();
        assert_eq!(received.as_string().unwrap(), "Message 2");

        // Cleanup
        let _ = remove_message_queue("test_capacity");
    }

    #[test]
    fn test_queue_peek() {
        let mut queue = MessageQueue::create("test_peek").unwrap();
        let msg = Message::new_text("Peek test");
        queue.send(msg).unwrap();

        // Peek should not remove message
        let peeked = queue.peek().unwrap().unwrap();
        assert_eq!(peeked.as_string().unwrap(), "Peek test");
        assert_eq!(queue.len(), 1);

        // Receive should still work
        let received = queue.receive().unwrap().unwrap();
        assert_eq!(received.as_string().unwrap(), "Peek test");
        assert_eq!(queue.len(), 0);

        // Cleanup
        let _ = remove_message_queue("test_peek");
    }

    #[test]
    fn test_json_message() {
        let json_data = r#"{"name": "test", "value": 42}"#;
        let msg = Message::new_json(json_data).unwrap();
        
        assert_eq!(msg.msg_type, MessageType::Json);
        assert_eq!(msg.as_string().unwrap(), json_data);

        // Test invalid JSON
        let invalid_json = r#"{"invalid": json}"#;
        assert!(Message::new_json(invalid_json).is_err());
    }

    #[test]
    fn test_queue_statistics() {
        let mut queue = MessageQueue::create("test_stats").unwrap();
        
        let stats = queue.statistics();
        assert_eq!(stats.name, "test_stats");
        assert_eq!(stats.current_size, 0);
        assert_eq!(stats.total_sent, 0);
        assert_eq!(stats.total_received, 0);

        // Send and receive messages
        let msg = Message::new_text("Test message");
        queue.send(msg).unwrap();
        
        let stats = queue.statistics();
        assert_eq!(stats.total_sent, 1);
        assert_eq!(stats.current_size, 1);

        queue.receive().unwrap();
        
        let stats = queue.statistics();
        assert_eq!(stats.total_received, 1);
        assert_eq!(stats.current_size, 0);

        // Cleanup
        let _ = remove_message_queue("test_stats");
    }
}
