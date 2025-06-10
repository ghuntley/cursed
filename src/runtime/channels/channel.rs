/// Core channel implementation for CURSED programming language
/// Provides Go-like channel semantics with dm<T> syntax

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, Weak};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};

use super::{ChannelError, ChannelResult, ReceiveResult, SendResult};

/// Channel state for tracking lifecycle
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelState {
    Open,
    Closed,
    Drained,
}

/// Waiting sender information
#[derive(Debug)]
pub struct WaitingSender<T> {
    pub value: T,
    pub notify: Arc<Condvar>,
    pub completed: Arc<AtomicBool>,
}

/// Waiting receiver information
#[derive(Debug)]
pub struct WaitingReceiver<T> {
    pub result: Arc<Mutex<Option<T>>>,
    pub notify: Arc<Condvar>,
    pub completed: Arc<AtomicBool>,
}

/// Internal channel data protected by mutex
#[derive(Debug)]
pub struct ChannelData<T> {
    /// Buffered messages
    pub buffer: VecDeque<T>,
    /// Maximum buffer capacity (0 for unbuffered)
    pub capacity: usize,
    /// Current channel state
    pub state: ChannelState,
    /// Senders waiting to send (for unbuffered channels)
    pub waiting_senders: VecDeque<WaitingSender<T>>,
    /// Receivers waiting to receive
    pub waiting_receivers: VecDeque<WaitingReceiver<T>>,
    /// Number of active sender handles
    pub sender_count: usize,
    /// Number of active receiver handles
    pub receiver_count: usize,
}

/// Core channel implementation
/// Supports both buffered (capacity > 0) and unbuffered (capacity = 0) channels
#[derive(Debug)]
pub struct Channel<T> {
    /// Protected channel data
    data: Arc<Mutex<ChannelData<T>>>,
    /// Condition variable for blocking operations
    condvar: Arc<Condvar>,
    /// Total operations counter for debugging
    operation_count: AtomicUsize,
}

impl<T> Channel<T> {
    /// Create a new channel with specified capacity
    /// - capacity = 0: unbuffered channel (synchronous)
    /// - capacity > 0: buffered channel (asynchronous up to capacity)
    #[instrument]
    pub fn new(capacity: usize) -> Self {
        debug!(capacity, "Creating new channel");
        
        let channel = Self {
            data: Arc::new(Mutex::new(ChannelData {
                buffer: VecDeque::with_capacity(capacity),
                capacity,
                state: ChannelState::Open,
                waiting_senders: VecDeque::new(),
                waiting_receivers: VecDeque::new(),
                sender_count: 0,
                receiver_count: 0,
            })),
            condvar: Arc::new(Condvar::new()),
            operation_count: AtomicUsize::new(0),
        };
        
        info!(capacity, "Channel created successfully");
        channel
    }

    /// Create a sender handle for this channel
    #[instrument(skip(self))]
    pub fn sender(&self) -> ChannelSender<T> {
        let mut data = self.data.lock().unwrap();
        data.sender_count += 1;
        
        debug!(sender_count = data.sender_count, "Created sender handle");
        
        ChannelSender {
            channel: Arc::downgrade(&self.data),
            condvar: self.condvar.clone(),
            operation_count: &self.operation_count,
        }
    }

    /// Create a receiver handle for this channel
    #[instrument(skip(self))]
    pub fn receiver(&self) -> ChannelReceiver<T> {
        let mut data = self.data.lock().unwrap();
        data.receiver_count += 1;
        
        debug!(receiver_count = data.receiver_count, "Created receiver handle");
        
        ChannelReceiver {
            channel: Arc::downgrade(&self.data),
            condvar: self.condvar.clone(),
            operation_count: &self.operation_count,
        }
    }

    /// Get channel statistics for debugging and monitoring
    #[instrument(skip(self))]
    pub fn stats(&self) -> ChannelStats {
        let data = self.data.lock().unwrap();
        ChannelStats {
            capacity: data.capacity,
            buffer_size: data.buffer.len(),
            sender_count: data.sender_count,
            receiver_count: data.receiver_count,
            waiting_senders: data.waiting_senders.len(),
            waiting_receivers: data.waiting_receivers.len(),
            is_closed: data.state != ChannelState::Open,
            operation_count: self.operation_count.load(Ordering::Relaxed),
        }
    }

    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.state != ChannelState::Open
    }

    /// Check if channel buffer is empty
    pub fn is_empty(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.buffer.is_empty() && data.waiting_senders.is_empty()
    }

    /// Check if channel buffer is full
    pub fn is_full(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.capacity > 0 && data.buffer.len() >= data.capacity
    }

    /// Get current buffer length
    pub fn len(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.buffer.len()
    }

    /// Get channel capacity
    pub fn capacity(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.capacity
    }
}

/// Channel sender handle
/// Automatically decrements sender count when dropped
#[derive(Debug)]
pub struct ChannelSender<T> {
    channel: Weak<Mutex<ChannelData<T>>>,
    condvar: Arc<Condvar>,
    operation_count: *const AtomicUsize,
}

/// Channel receiver handle  
/// Automatically decrements receiver count when dropped
#[derive(Debug)]
pub struct ChannelReceiver<T> {
    channel: Weak<Mutex<ChannelData<T>>>,
    condvar: Arc<Condvar>,
    operation_count: *const AtomicUsize,
}

/// Channel statistics for monitoring and debugging
#[derive(Debug, Clone)]
pub struct ChannelStats {
    pub capacity: usize,
    pub buffer_size: usize,
    pub sender_count: usize,
    pub receiver_count: usize,
    pub waiting_senders: usize,
    pub waiting_receivers: usize,
    pub is_closed: bool,
    pub operation_count: usize,
}

// Safe to send between threads since we use proper synchronization
unsafe impl<T: Send> Send for Channel<T> {}
unsafe impl<T: Send> Sync for Channel<T> {}
unsafe impl<T: Send> Send for ChannelSender<T> {}
unsafe impl<T: Send> Sync for ChannelSender<T> {}
unsafe impl<T: Send> Send for ChannelReceiver<T> {}
unsafe impl<T: Send> Sync for ChannelReceiver<T> {}

impl<T> Drop for ChannelSender<T> {
    #[instrument(skip(self))]
    fn drop(&mut self) {
        if let Some(data_arc) = self.channel.upgrade() {
            let mut data = data_arc.lock().unwrap();
            data.sender_count -= 1;
            
            debug!(remaining_senders = data.sender_count, "Sender handle dropped");
            
            // If no more senders, close the channel
            if data.sender_count == 0 {
                data.state = ChannelState::Closed;
                info!("Channel closed - no more senders");
                
                // Wake up any waiting receivers
                self.condvar.notify_all();
            }
        }
    }
}

impl<T> Drop for ChannelReceiver<T> {
    #[instrument(skip(self))]
    fn drop(&mut self) {
        if let Some(data_arc) = self.channel.upgrade() {
            let mut data = data_arc.lock().unwrap();
            data.receiver_count -= 1;
            
            debug!(remaining_receivers = data.receiver_count, "Receiver handle dropped");
            
            // Wake up any waiting senders in case they need to know about receiver count
            self.condvar.notify_all();
        }
    }
}

impl<T> Clone for ChannelSender<T> {
    #[instrument(skip(self))]
    fn clone(&self) -> Self {
        if let Some(data_arc) = self.channel.upgrade() {
            let mut data = data_arc.lock().unwrap();
            data.sender_count += 1;
            
            debug!(sender_count = data.sender_count, "Cloned sender handle");
        }
        
        Self {
            channel: self.channel.clone(),
            condvar: self.condvar.clone(),
            operation_count: self.operation_count,
        }
    }
}

impl<T> Clone for ChannelReceiver<T> {
    #[instrument(skip(self))]
    fn clone(&self) -> Self {
        if let Some(data_arc) = self.channel.upgrade() {
            let mut data = data_arc.lock().unwrap();
            data.receiver_count += 1;
            
            debug!(receiver_count = data.receiver_count, "Cloned receiver handle");
        }
        
        Self {
            channel: self.channel.clone(),
            condvar: self.condvar.clone(),
            operation_count: self.operation_count,
        }
    }
}

/// Create a new unbuffered channel (synchronous)
pub fn channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    let ch = Channel::new(0);
    (ch.sender(), ch.receiver())
}

/// Create a new buffered channel with specified capacity
pub fn buffered_channel<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    let ch = Channel::new(capacity);
    (ch.sender(), ch.receiver())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_channel_creation() {
        let ch = Channel::<i32>::new(5);
        assert_eq!(ch.capacity(), 5);
        assert!(ch.is_empty());
        assert!(!ch.is_full());
        assert!(!ch.is_closed());
    }

    #[test]
    fn test_unbuffered_channel_creation() {
        let ch = Channel::<i32>::new(0);
        assert_eq!(ch.capacity(), 0);
        assert!(ch.is_empty());
        assert!(!ch.is_closed());
    }

    #[test]
    fn test_sender_receiver_handles() {
        let ch = Channel::<i32>::new(1);
        let _sender = ch.sender();
        let _receiver = ch.receiver();
        
        let stats = ch.stats();
        assert_eq!(stats.sender_count, 1);
        assert_eq!(stats.receiver_count, 1);
    }

    #[test]
    fn test_handle_cloning() {
        let ch = Channel::<i32>::new(1);
        let sender1 = ch.sender();
        let sender2 = sender1.clone();
        
        let stats = ch.stats();
        assert_eq!(stats.sender_count, 2);
    }

    #[test]
    fn test_convenience_constructors() {
        let (_tx, _rx) = channel::<i32>();
        let (_tx, _rx) = buffered_channel::<i32>(10);
        // Should not panic
    }
}
