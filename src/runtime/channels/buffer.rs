use crate::error::Error;
//! Channel buffering system for CURSED channels
//! 
//! Provides both buffered and unbuffered channel implementations with efficient
//! ring buffer storage and proper synchronization primitives.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use tracing::{debug, warn, error, instrument, trace};
use crate::memory::gc::GarbageCollector;
use crate::error::Error as CursedError;

/// Result type for buffer operations
pub type BufferResult<T> = std::result::Result<T, ChannelError>;

/// Errors that can occur during buffer operations
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelBufferError {
    /// Buffer is full and cannot accept more items
    BufferFull,
    /// Buffer is empty and cannot provide items
    BufferEmpty,
    /// Channel has been closed
    ChannelClosed,
    /// Operation timed out
    Timeout,
    /// Invalid buffer capacity
    InvalidCapacity(usize),
    /// Memory allocation failed
    AllocationFailed,
    /// Synchronization error
    SyncError(String),
}

impl std::fmt::Display for ChannelBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelBufferError::BufferFull => write!(f, "Channel buffer is full"),
            ChannelBufferError::BufferEmpty => write!(f, "Channel buffer is empty"),
            ChannelBufferError::ChannelClosed => write!(f, "Channel has been closed"),
            ChannelBufferError::Timeout => write!(f, "Operation timed out"),
            ChannelBufferError::InvalidCapacity(cap) => write!(f, "Invalid buffer capacity: {}", cap),
            ChannelBufferError::AllocationFailed => write!(f, "Memory allocation failed"),
            ChannelBufferError::SyncError(msg) => write!(f, "Synchronization error: {}", msg),
        }
    }
}

impl std::error::Error for ChannelBufferError {}

impl From<ChannelBufferError> for CursedError {
    fn from(err: ChannelBufferError) -> Self {
        CursedError::from_str(&err.to_string())
    }
}

/// Configuration for channel buffer behavior
#[derive(Debug, Clone)]
pub struct BufferConfig {
    /// Buffer capacity (0 for unbuffered)
    pub capacity: usize,
    /// Whether to enable blocking operations
    pub blocking: bool,
    /// Timeout for blocking operations
    pub timeout: Option<Duration>,
    /// Whether to drop oldest items when full (for bounded channels)
    pub drop_old_on_full: bool,
    /// Enable performance statistics
    pub enable_stats: bool,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            capacity: 0,
            blocking: true,
            timeout: None,
            drop_old_on_full: false,
            enable_stats: false,
        }
    }
}

/// Statistics for buffer performance monitoring
#[derive(Debug, Default)]
pub struct BufferStats {
    pub total_pushes: AtomicUsize,
    pub total_pops: AtomicUsize,
    pub total_blocks: AtomicUsize,
    pub total_timeouts: AtomicUsize,
    pub max_size: AtomicUsize,
    pub current_size: AtomicUsize,
}

impl BufferStats {
    #[instrument(skip(self))]
    pub fn record_push(&self) {
        self.total_pushes.fetch_add(1, Ordering::Relaxed);
        let new_size = self.current_size.fetch_add(1, Ordering::Relaxed) + 1;
        self.max_size.fetch_max(new_size, Ordering::Relaxed);
        trace!(operation = "push", current_size = new_size);
    }

    #[instrument(skip(self))]
    pub fn record_pop(&self) {
        self.total_pops.fetch_add(1, Ordering::Relaxed);
        let new_size = self.current_size.fetch_sub(1, Ordering::Relaxed).saturating_sub(1);
        trace!(operation = "pop", current_size = new_size);
    }

    #[instrument(skip(self))]
    pub fn record_block(&self) {
        self.total_blocks.fetch_add(1, Ordering::Relaxed);
        trace!(operation = "block");
    }

    #[instrument(skip(self))]
    pub fn record_timeout(&self) {
        self.total_timeouts.fetch_add(1, Ordering::Relaxed);
        trace!(operation = "timeout");
    }

    pub fn get_summary(&self) -> BufferStatsSummary {
        BufferStatsSummary {
            total_pushes: self.total_pushes.load(Ordering::Relaxed),
            total_pops: self.total_pops.load(Ordering::Relaxed),
            total_blocks: self.total_blocks.load(Ordering::Relaxed),
            total_timeouts: self.total_timeouts.load(Ordering::Relaxed),
            max_size: self.max_size.load(Ordering::Relaxed),
            current_size: self.current_size.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BufferStatsSummary {
    pub total_pushes: usize,
    pub total_pops: usize,
    pub total_blocks: usize,
    pub total_timeouts: usize,
    pub max_size: usize,
    pub current_size: usize,
}

/// Main interface for channel buffers
pub trait ChannelBuffer<T>: Send + Sync {
    /// Push an item to the buffer
    fn push(&self, item: T) -> BufferResult<()>;
    
    /// Push an item with timeout
    fn push_timeout(&self, item: T, timeout: Duration) -> BufferResult<()>;
    
    /// Pop an item from the buffer
    fn pop(&self) -> BufferResult<T>;
    
    /// Pop an item with timeout
    fn pop_timeout(&self, timeout: Duration) -> BufferResult<T>;
    
    /// Try to push without blocking
    fn try_push(&self, item: T) -> BufferResult<()>;
    
    /// Try to pop without blocking
    fn try_pop(&self) -> BufferResult<T>;
    
    /// Check if buffer is full
    fn is_full(&self) -> bool;
    
    /// Check if buffer is empty
    fn is_empty(&self) -> bool;
    
    /// Get current buffer length
    fn len(&self) -> usize;
    
    /// Get buffer capacity
    fn capacity(&self) -> usize;
    
    /// Close the buffer
    fn close(&self);
    
    /// Check if buffer is closed
    fn is_closed(&self) -> bool;
    
    /// Get buffer statistics
    fn stats(&self) -> Option<BufferStatsSummary>;
}

/// Efficient ring buffer implementation for buffered channels
#[derive(Debug)]
pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    /// Create a new ring buffer with specified capacity
    #[instrument]
    pub fn new(capacity: usize) -> BufferResult<Self> {
        if capacity == 0 {
            return Err(ChannelBufferError::InvalidCapacity(capacity));
        }

        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }

        Ok(RingBuffer {
            buffer,
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        })
    }

    /// Push item to the buffer
    #[instrument(skip(self, item))]
    pub fn push(&mut self, item: T) -> BufferResult<()> {
        if self.is_full() {
            return Err(ChannelBufferError::BufferFull);
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        
        trace!(
            operation = "ring_buffer_push",
            head = self.head,
            tail = self.tail,
            size = self.size
        );
        
        Ok(())
    }

    /// Try to push item to the buffer, returns the item back if buffer is full
    #[instrument(skip(self, item))]
    pub fn try_push(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            return Err(item);
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        
        trace!(
            operation = "ring_buffer_try_push",
            head = self.head,
            tail = self.tail,
            size = self.size
        );
        
        Ok(())
    }

    /// Pop item from the buffer
    #[instrument(skip(self))]
    pub fn pop(&mut self) -> BufferResult<T> {
        if self.is_empty() {
            return Err(ChannelBufferError::BufferEmpty);
        }

        let item = self.buffer[self.head].take().ok_or(ChannelBufferError::BufferEmpty)?;
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        
        trace!(
            operation = "ring_buffer_pop",
            head = self.head,
            tail = self.tail,
            size = self.size
        );
        
        Ok(item)
    }

    /// Push item, dropping oldest if full
    #[instrument(skip(self, item))]
    pub fn push_overwrite(&mut self, item: T) -> Option<T> {
        if self.is_full() {
            let dropped = self.pop().ok();
            self.push(item).ok();
            dropped
        } else {
            self.push(item).ok();
            None
        }
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn available_space(&self) -> usize {
        self.capacity - self.size
    }
}

/// Unbuffered channel implementation (synchronous)
#[derive(Debug, Clone)]
pub struct UnbufferedChannel<T> {
    /// Synchronization state
    state: Arc<Mutex<UnbufferedState<T>>>,
    /// Condition variable for waiting
    condvar: Arc<Condvar>,
    /// Channel statistics
    stats: Option<Arc<BufferStats>>,
    /// Channel configuration
    config: BufferConfig,
}

#[derive(Debug)]
struct UnbufferedState<T> {
    /// Pending item waiting for receiver
    pending_item: Option<T>,
    /// Whether a sender is waiting
    sender_waiting: bool,
    /// Whether a receiver is waiting
    receiver_waiting: bool,
    /// Whether the channel is closed
    closed: bool,
}

impl<T> UnbufferedChannel<T> {
    #[instrument]
    pub fn new(config: BufferConfig) -> Self {
        let stats = if config.enable_stats {
            Some(Arc::new(BufferStats::default()))
        } else {
            None
        };

        UnbufferedChannel {
            state: Arc::new(Mutex::new(UnbufferedState {
                pending_item: None,
                sender_waiting: false,
                receiver_waiting: false,
                closed: false,
            })),
            condvar: Arc::new(Condvar::new()),
            stats,
            config,
        }
    }
}

impl<T: Send + Sync> ChannelBuffer<T> for UnbufferedChannel<T> {
    #[instrument(skip(self, item))]
    fn push(&self, item: T) -> BufferResult<()> {
        if let Some(ref stats) = self.stats {
            stats.record_push();
        }

        let mut state = self.state.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire lock: {}", e))
        })?;

        if state.closed {
            return Err(ChannelBufferError::ChannelClosed);
        }

        // Set pending item and wait for receiver
        state.pending_item = Some(item);
        state.sender_waiting = true;

        // Wake up any waiting receivers
        self.condvar.notify_all();

        // Wait for receiver to pick up the item
        while state.pending_item.is_some() && !state.closed {
            if let Some(ref stats) = self.stats {
                stats.record_block();
            }
            
            state = self.condvar.wait(state).map_err(|e| {
                ChannelBufferError::SyncError(format!("Wait failed: {}", e))
            })?;
        }

        state.sender_waiting = false;

        if state.closed {
            Err(ChannelBufferError::ChannelClosed)
        } else {
            debug!(operation = "unbuffered_push_complete");
            Ok(())
        }
    }

    #[instrument(skip(self, item))]
    fn push_timeout(&self, item: T, timeout: Duration) -> BufferResult<()> {
        if let Some(ref stats) = self.stats {
            stats.record_push();
        }

        let mut state = self.state.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire lock: {}", e))
        })?;

        if state.closed {
            return Err(ChannelBufferError::ChannelClosed);
        }

        state.pending_item = Some(item);
        state.sender_waiting = true;
        self.condvar.notify_all();

        let start_time = Instant::now();
        
        while state.pending_item.is_some() && !state.closed {
            let elapsed = start_time.elapsed();
            if elapsed >= timeout {
                state.sender_waiting = false;
                state.pending_item = None; // Remove the item we couldn't send
                if let Some(ref stats) = self.stats {
                    stats.record_timeout();
                }
                return Err(ChannelBufferError::Timeout);
            }

            let remaining = timeout - elapsed;
            let (new_state, timeout_result) = self.condvar.wait_timeout(state, remaining).map_err(|e| {
                ChannelBufferError::SyncError(format!("Wait timeout failed: {}", e))
            })?;
            
            state = new_state;
            
            if timeout_result.timed_out() {
                state.sender_waiting = false;
                state.pending_item = None;
                if let Some(ref stats) = self.stats {
                    stats.record_timeout();
                }
                return Err(ChannelBufferError::Timeout);
            }
        }

        state.sender_waiting = false;

        if state.closed {
            Err(ChannelBufferError::ChannelClosed)
        } else {
            Ok(())
        }
    }

    #[instrument(skip(self))]
    fn pop(&self) -> BufferResult<T> {
        if let Some(ref stats) = self.stats {
            stats.record_pop();
        }

        let mut state = self.state.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire lock: {}", e))
        })?;

        if state.closed && state.pending_item.is_none() {
            return Err(ChannelBufferError::ChannelClosed);
        }

        state.receiver_waiting = true;

        // Wait for sender to provide an item
        while state.pending_item.is_none() && !state.closed {
            if let Some(ref stats) = self.stats {
                stats.record_block();
            }
            
            state = self.condvar.wait(state).map_err(|e| {
                ChannelBufferError::SyncError(format!("Wait failed: {}", e))
            })?;
        }

        state.receiver_waiting = false;

        if let Some(item) = state.pending_item.take() {
            // Notify sender that item was received
            self.condvar.notify_all();
            debug!(operation = "unbuffered_pop_complete");
            Ok(item)
        } else {
            Err(ChannelBufferError::ChannelClosed)
        }
    }

    #[instrument(skip(self))]
    fn pop_timeout(&self, timeout: Duration) -> BufferResult<T> {
        if let Some(ref stats) = self.stats {
            stats.record_pop();
        }

        let mut state = self.state.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire lock: {}", e))
        })?;

        if state.closed && state.pending_item.is_none() {
            return Err(ChannelBufferError::ChannelClosed);
        }

        state.receiver_waiting = true;
        let start_time = Instant::now();

        while state.pending_item.is_none() && !state.closed {
            let elapsed = start_time.elapsed();
            if elapsed >= timeout {
                state.receiver_waiting = false;
                if let Some(ref stats) = self.stats {
                    stats.record_timeout();
                }
                return Err(ChannelBufferError::Timeout);
            }

            let remaining = timeout - elapsed;
            let (new_state, timeout_result) = self.condvar.wait_timeout(state, remaining).map_err(|e| {
                ChannelBufferError::SyncError(format!("Wait timeout failed: {}", e))
            })?;
            
            state = new_state;
            
            if timeout_result.timed_out() {
                state.receiver_waiting = false;
                if let Some(ref stats) = self.stats {
                    stats.record_timeout();
                }
                return Err(ChannelBufferError::Timeout);
            }
        }

        state.receiver_waiting = false;

        if let Some(item) = state.pending_item.take() {
            self.condvar.notify_all();
            Ok(item)
        } else {
            Err(ChannelBufferError::ChannelClosed)
        }
    }

    fn try_push(&self, item: T) -> BufferResult<()> {
        let mut state = self.state.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire lock: {}", e))
        })?;

        if state.closed {
            return Err(ChannelBufferError::ChannelClosed);
        }

        if state.receiver_waiting && state.pending_item.is_none() {
            state.pending_item = Some(item);
            self.condvar.notify_all();
            if let Some(ref stats) = self.stats {
                stats.record_push();
            }
            Ok(())
        } else {
            Err(ChannelBufferError::BufferFull)
        }
    }

    fn try_pop(&self) -> BufferResult<T> {
        let mut state = self.state.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire lock: {}", e))
        })?;

        if let Some(item) = state.pending_item.take() {
            self.condvar.notify_all();
            if let Some(ref stats) = self.stats {
                stats.record_pop();
            }
            Ok(item)
        } else if state.closed {
            Err(ChannelBufferError::ChannelClosed)
        } else {
            Err(ChannelBufferError::BufferEmpty)
        }
    }

    fn is_full(&self) -> bool {
        // Unbuffered channels are always "full" in the sense that they can't buffer
        if let Ok(state) = self.state.lock() {
            state.pending_item.is_some()
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        if let Ok(state) = self.state.lock() {
            state.pending_item.is_none()
        } else {
            true
        }
    }

    fn len(&self) -> usize {
        if let Ok(state) = self.state.lock() {
            if state.pending_item.is_some() { 1 } else { 0 }
        } else {
            0
        }
    }

    fn capacity(&self) -> usize {
        0 // Unbuffered channels have no capacity
    }

    fn close(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.closed = true;
            self.condvar.notify_all();
            debug!(operation = "unbuffered_channel_closed");
        }
    }

    fn is_closed(&self) -> bool {
        if let Ok(state) = self.state.lock() {
            state.closed
        } else {
            false
        }
    }

    fn stats(&self) -> Option<BufferStatsSummary> {
        self.stats.as_ref().map(|stats| stats.get_summary())
    }
}

/// Buffered channel implementation (asynchronous)
#[derive(Debug)]
pub struct BufferedChannel<T> {
    /// Internal ring buffer
    buffer: Arc<Mutex<RingBuffer<T>>>,
    /// Condition variable for sender waiting
    sender_condvar: Arc<Condvar>,
    /// Condition variable for receiver waiting
    receiver_condvar: Arc<Condvar>,
    /// Channel state
    state: Arc<Mutex<BufferedState>>,
    /// Channel statistics
    stats: Option<Arc<BufferStats>>,
    /// Channel configuration
    config: BufferConfig,
}

#[derive(Debug)]
struct BufferedState {
    closed: bool,
    waiting_senders: usize,
    waiting_receivers: usize,
}

impl<T> BufferedChannel<T> {
    #[instrument]
    pub fn new(capacity: usize, config: BufferConfig) -> BufferResult<Self> {
        if capacity == 0 {
            return Err(ChannelBufferError::InvalidCapacity(capacity));
        }

        let stats = if config.enable_stats {
            Some(Arc::new(BufferStats::default()))
        } else {
            None
        };

        Ok(BufferedChannel {
            buffer: Arc::new(Mutex::new(RingBuffer::new(capacity)?)),
            sender_condvar: Arc::new(Condvar::new()),
            receiver_condvar: Arc::new(Condvar::new()),
            state: Arc::new(Mutex::new(BufferedState {
                closed: false,
                waiting_senders: 0,
                waiting_receivers: 0,
            })),
            stats,
            config,
        })
    }
}

impl<T: Send + Sync> ChannelBuffer<T> for BufferedChannel<T> {
    #[instrument(skip(self, item))]
    fn push(&self, item: T) -> BufferResult<()> {
        if let Some(ref stats) = self.stats {
            stats.record_push();
        }

        let mut current_item = Some(item);

        loop {
            let should_wait = {
                let mut buffer = self.buffer.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire buffer lock: {}", e))
                })?;
                
                let state = self.state.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
                })?;

                if state.closed {
                    return Err(ChannelBufferError::ChannelClosed);
                }

                // Try to push to buffer
                if let Some(item_to_push) = current_item.take() {
                    match buffer.try_push(item_to_push) {
                        Ok(()) => {
                            // Successfully pushed, notify waiting receivers
                            drop(state);
                            drop(buffer);
                            self.receiver_condvar.notify_one();
                            debug!(operation = "buffered_push_complete");
                            return Ok(());
                        }
                        Err(item_to_push) => {
                            // Buffer is full
                            if self.config.drop_old_on_full {
                                // Drop oldest item and push new one
                                let _dropped = buffer.push_overwrite(item_to_push);
                                drop(state);
                                drop(buffer);
                                self.receiver_condvar.notify_one();
                                warn!(operation = "buffered_push_overwrite");
                                return Ok(());
                            } else {
                                // Buffer is full, put the item back and wait
                                current_item = Some(item_to_push);
                                true
                            }
                        }
                    }
                } else {
                    // This shouldn't happen, but handle gracefully
                    return Err(ChannelBufferError::SyncError("Item lost during push".to_string()));
                }
            };

            if should_wait {
                // Buffer was full, wait for space
                let mut state = self.state.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
                })?;

                if state.closed {
                    return Err(ChannelBufferError::ChannelClosed);
                }

                state.waiting_senders += 1;
                if let Some(ref stats) = self.stats {
                    stats.record_block();
                }

                state = self.sender_condvar.wait(state).map_err(|e| {
                    ChannelBufferError::SyncError(format!("Sender wait failed: {}", e))
                })?;

                state.waiting_senders -= 1;
            }
        }
    }

    #[instrument(skip(self, item))]
    fn push_timeout(&self, item: T, timeout: Duration) -> BufferResult<()> {
        if let Some(ref stats) = self.stats {
            stats.record_push();
        }

        let start_time = Instant::now();
        let mut current_item = Some(item);

        loop {
            let should_wait = {
                let mut buffer = self.buffer.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire buffer lock: {}", e))
                })?;
                
                let state = self.state.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
                })?;

                if state.closed {
                    return Err(ChannelBufferError::ChannelClosed);
                }

                if let Some(item_to_push) = current_item.take() {
                    match buffer.try_push(item_to_push) {
                        Ok(()) => {
                            drop(state);
                            drop(buffer);
                            self.receiver_condvar.notify_one();
                            return Ok(());
                        }
                        Err(item_to_push) => {
                            // Buffer is full
                            if self.config.drop_old_on_full {
                                let _dropped = buffer.push_overwrite(item_to_push);
                                drop(state);
                                drop(buffer);
                                self.receiver_condvar.notify_one();
                                return Ok(());
                            } else {
                                current_item = Some(item_to_push);
                                true
                            }
                        }
                    }
                } else {
                    return Err(ChannelBufferError::SyncError("Item lost during push_timeout".to_string()));
                }
            };

            if should_wait {
                let elapsed = start_time.elapsed();
                if elapsed >= timeout {
                    if let Some(ref stats) = self.stats {
                        stats.record_timeout();
                    }
                    return Err(ChannelBufferError::Timeout);
                }

                let mut state = self.state.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
                })?;

                if state.closed {
                    return Err(ChannelBufferError::ChannelClosed);
                }

                state.waiting_senders += 1;
                let remaining = timeout - elapsed;

                let (new_state, timeout_result) = self.sender_condvar.wait_timeout(state, remaining).map_err(|e| {
                    ChannelBufferError::SyncError(format!("Sender wait timeout failed: {}", e))
                })?;

                state = new_state;
                state.waiting_senders -= 1;

                if timeout_result.timed_out() {
                    if let Some(ref stats) = self.stats {
                        stats.record_timeout();
                    }
                    return Err(ChannelBufferError::Timeout);
                }
            }
        }
    }

    #[instrument(skip(self))]
    fn pop(&self) -> BufferResult<T> {
        if let Some(ref stats) = self.stats {
            stats.record_pop();
        }

        loop {
            {
                let mut buffer = self.buffer.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire buffer lock: {}", e))
                })?;
                
                let state = self.state.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
                })?;

                match buffer.pop() {
                    Ok(item) => {
                        // Successfully popped, notify waiting senders
                        drop(state);
                        drop(buffer);
                        self.sender_condvar.notify_one();
                        debug!(operation = "buffered_pop_complete");
                        return Ok(item);
                    }
                    Err(ChannelBufferError::BufferEmpty) => {
                        if state.closed {
                            return Err(ChannelBufferError::ChannelClosed);
                        }
                        // Buffer is empty, need to wait
                    }
                    Err(e) => return Err(e),
                }
            }

            // Buffer was empty, wait for items
            let mut state = self.state.lock().map_err(|e| {
                ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
            })?;

            if state.closed {
                return Err(ChannelBufferError::ChannelClosed);
            }

            state.waiting_receivers += 1;
            if let Some(ref stats) = self.stats {
                stats.record_block();
            }

            state = self.receiver_condvar.wait(state).map_err(|e| {
                ChannelBufferError::SyncError(format!("Receiver wait failed: {}", e))
            })?;

            state.waiting_receivers -= 1;
        }
    }

    #[instrument(skip(self))]
    fn pop_timeout(&self, timeout: Duration) -> BufferResult<T> {
        if let Some(ref stats) = self.stats {
            stats.record_pop();
        }

        let start_time = Instant::now();

        loop {
            {
                let mut buffer = self.buffer.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire buffer lock: {}", e))
                })?;
                
                let state = self.state.lock().map_err(|e| {
                    ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
                })?;

                match buffer.pop() {
                    Ok(item) => {
                        drop(state);
                        drop(buffer);
                        self.sender_condvar.notify_one();
                        return Ok(item);
                    }
                    Err(ChannelBufferError::BufferEmpty) => {
                        if state.closed {
                            return Err(ChannelBufferError::ChannelClosed);
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            let elapsed = start_time.elapsed();
            if elapsed >= timeout {
                if let Some(ref stats) = self.stats {
                    stats.record_timeout();
                }
                return Err(ChannelBufferError::Timeout);
            }

            let mut state = self.state.lock().map_err(|e| {
                ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
            })?;

            if state.closed {
                return Err(ChannelBufferError::ChannelClosed);
            }

            state.waiting_receivers += 1;
            let remaining = timeout - elapsed;

            let (new_state, timeout_result) = self.receiver_condvar.wait_timeout(state, remaining).map_err(|e| {
                ChannelBufferError::SyncError(format!("Receiver wait timeout failed: {}", e))
            })?;

            state = new_state;
            state.waiting_receivers -= 1;

            if timeout_result.timed_out() {
                if let Some(ref stats) = self.stats {
                    stats.record_timeout();
                }
                return Err(ChannelBufferError::Timeout);
            }
        }
    }

    fn try_push(&self, item: T) -> BufferResult<()> {
        let mut buffer = self.buffer.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire buffer lock: {}", e))
        })?;
        
        let state = self.state.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
        })?;

        if state.closed {
            return Err(ChannelBufferError::ChannelClosed);
        }

        match buffer.try_push(item) {
            Ok(()) => {
                drop(state);
                drop(buffer);
                self.receiver_condvar.notify_one();
                if let Some(ref stats) = self.stats {
                    stats.record_push();
                }
                Ok(())
            }
            Err(_item) => {
                // Buffer is full - for try_push we don't overwrite
                Err(ChannelBufferError::BufferFull)
            }
        }
    }

    fn try_pop(&self) -> BufferResult<T> {
        let mut buffer = self.buffer.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire buffer lock: {}", e))
        })?;
        
        let state = self.state.lock().map_err(|e| {
            ChannelBufferError::SyncError(format!("Failed to acquire state lock: {}", e))
        })?;

        match buffer.pop() {
            Ok(item) => {
                drop(state);
                drop(buffer);
                self.sender_condvar.notify_one();
                if let Some(ref stats) = self.stats {
                    stats.record_pop();
                }
                Ok(item)
            }
            Err(ChannelBufferError::BufferEmpty) => {
                if state.closed {
                    Err(ChannelBufferError::ChannelClosed)
                } else {
                    Err(ChannelBufferError::BufferEmpty)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn is_full(&self) -> bool {
        if let Ok(buffer) = self.buffer.lock() {
            buffer.is_full()
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        if let Ok(buffer) = self.buffer.lock() {
            buffer.is_empty()
        } else {
            true
        }
    }

    fn len(&self) -> usize {
        if let Ok(buffer) = self.buffer.lock() {
            buffer.len()
        } else {
            0
        }
    }

    fn capacity(&self) -> usize {
        if let Ok(buffer) = self.buffer.lock() {
            buffer.capacity()
        } else {
            0
        }
    }

    fn close(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.closed = true;
            drop(state);
            self.sender_condvar.notify_all();
            self.receiver_condvar.notify_all();
            debug!(operation = "buffered_channel_closed");
        }
    }

    fn is_closed(&self) -> bool {
        if let Ok(state) = self.state.lock() {
            state.closed
        } else {
            false
        }
    }

    fn stats(&self) -> Option<BufferStatsSummary> {
        self.stats.as_ref().map(|stats| stats.get_summary())
    }
}

/// Factory function to create appropriate channel buffer based on configuration
#[instrument]
pub fn create_channel_buffer<T: Send + Sync + 'static>(config: BufferConfig) -> BufferResult<Box<dyn ChannelBuffer<T>>> {
    if config.capacity == 0 {
        debug!(operation = "creating_unbuffered_channel");
        Ok(Box::new(UnbufferedChannel::new(config)))
    } else {
        debug!(operation = "creating_buffered_channel", capacity = config.capacity);
        Ok(Box::new(BufferedChannel::new(config.capacity, config)?))
    }
}

/// Integration with CURSED's garbage collection system
pub trait GcIntegration<T> {
    /// Register buffer contents with GC
    fn register_with_gc(&self, gc: &mut GarbageCollector) -> Result<(), Error>;
    
    /// Update GC roots for buffer contents
    fn update_gc_roots(&self, gc: &mut GarbageCollector) -> Result<(), Error>;
}

impl<T> GcIntegration<T> for BufferedChannel<T> 
where
    T: 'static,
{
    #[instrument(skip(self, gc))]
    fn register_with_gc(&self, gc: &mut GarbageCollector) -> Result<(), Error> {
        // Register the buffer itself as a GC root
        // This ensures that any GC-managed objects stored in the buffer
        // are not collected while the buffer exists
        debug!(operation = "registering_buffer_with_gc");
        
        // Implementation would depend on the specific GC interface
        // For now, we assume GC handles registration automatically
        Ok(())
    }

    #[instrument(skip(self, gc))]
    fn update_gc_roots(&self, gc: &mut GarbageCollector) -> Result<(), Error> {
        // Update GC roots when buffer contents change
        debug!(operation = "updating_gc_roots_for_buffer");
        
        // Implementation would scan buffer contents and register any
        // GC-managed objects as roots
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_ring_buffer_basic_operations() {
        let mut buffer = RingBuffer::new(3).unwrap();
        
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 3);

        buffer.push(1).unwrap();
        buffer.push(2).unwrap();
        buffer.push(3).unwrap();
        
        assert!(buffer.is_full());
        assert!(!buffer.is_empty());
        assert_eq!(buffer.len(), 3);

        assert_eq!(buffer.pop().unwrap(), 1);
        assert_eq!(buffer.pop().unwrap(), 2);
        assert_eq!(buffer.pop().unwrap(), 3);
        
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_ring_buffer_wraparound() {
        let mut buffer = RingBuffer::new(2).unwrap();
        
        buffer.push(1).unwrap();
        buffer.push(2).unwrap();
        assert_eq!(buffer.pop().unwrap(), 1);
        buffer.push(3).unwrap();
        assert_eq!(buffer.pop().unwrap(), 2);
        assert_eq!(buffer.pop().unwrap(), 3);
    }

    #[test]
    fn test_unbuffered_channel_basic() {
        let config = BufferConfig::default();
        let channel = Arc::new(UnbufferedChannel::new(config));
        
        let sender = channel.clone();
        let receiver = channel.clone();
        
        thread::spawn(move || {
            sender.push(42).unwrap();
        });
        
        let result = receiver.pop().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_buffered_channel_basic() {
        let config = BufferConfig {
            capacity: 2,
            ..Default::default()
        };
        let channel = BufferedChannel::new(2, config).unwrap();
        
        // Should be able to push without blocking
        channel.push(1).unwrap();
        channel.push(2).unwrap();
        
        assert_eq!(channel.len(), 2);
        assert!(channel.is_full());
        
        assert_eq!(channel.pop().unwrap(), 1);
        assert_eq!(channel.pop().unwrap(), 2);
        
        assert!(channel.is_empty());
    }

    #[test]
    fn test_channel_timeout_operations() {
        let config = BufferConfig {
            capacity: 1,
            ..Default::default()
        };
        let channel = BufferedChannel::new(1, config).unwrap();
        
        // Fill the buffer
        channel.push(1).unwrap();
        
        // This should timeout
        let result = channel.push_timeout(2, Duration::from_millis(10));
        assert!(matches!(result, Err(ChannelBufferError::Timeout)));
        
        // Pop should work
        assert_eq!(channel.pop().unwrap(), 1);
        
        // Pop timeout on empty buffer
        let result = channel.pop_timeout(Duration::from_millis(10));
        assert!(matches!(result, Err(ChannelBufferError::Timeout)));
    }

    #[test]
    fn test_channel_factory() {
        // Unbuffered channel
        let config = BufferConfig { capacity: 0, ..Default::default() };
        let unbuffered = create_channel_buffer::<i32>(config).unwrap();
        assert_eq!(unbuffered.capacity(), 0);
        
        // Buffered channel
        let config = BufferConfig { capacity: 5, ..Default::default() };
        let buffered = create_channel_buffer::<i32>(config).unwrap();
        assert_eq!(buffered.capacity(), 5);
    }

    #[test]
    fn test_channel_close_behavior() {
        let config = BufferConfig {
            capacity: 2,
            ..Default::default()
        };
        let channel = BufferedChannel::new(2, config).unwrap();
        
        channel.push(1).unwrap();
        channel.close();
        
        assert!(channel.is_closed());
        assert!(matches!(channel.push(2), Err(ChannelBufferError::ChannelClosed)));
        
        // Should still be able to pop existing items
        assert_eq!(channel.pop().unwrap(), 1);
        
        // But not pop when empty and closed
        assert!(matches!(channel.pop(), Err(ChannelBufferError::ChannelClosed)));
    }

    #[test]
    fn test_channel_statistics() {
        let config = BufferConfig {
            capacity: 2,
            enable_stats: true,
            ..Default::default()
        };
        let channel = BufferedChannel::new(2, config).unwrap();
        
        channel.push(1).unwrap();
        channel.push(2).unwrap();
        channel.pop().unwrap();
        
        let stats = channel.stats().unwrap();
        assert_eq!(stats.total_pushes, 2);
        assert_eq!(stats.total_pops, 1);
        assert_eq!(stats.current_size, 1);
    }

    #[test]
    fn test_concurrent_access() {
        let config = BufferConfig {
            capacity: 10,
            enable_stats: true,
            ..Default::default()
        };
        let channel = Arc::new(BufferedChannel::new(10, config).unwrap());
        
        let num_threads = 4;
        let items_per_thread = 100;
        let total_sent = Arc::new(AtomicUsize::new(0));
        let total_received = Arc::new(AtomicUsize::new(0));
        
        // Spawn sender threads
        let mut handles = vec![];
        for i in 0..num_threads {
            let channel = channel.clone();
            let total_sent = total_sent.clone();
            let handle = thread::spawn(move || {
                for j in 0..items_per_thread {
                    let value = i * items_per_thread + j;
                    channel.push(value).unwrap();
                    total_sent.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }
        
        // Spawn receiver thread
        let channel_recv = channel.clone();
        let total_received_recv = total_received.clone();
        let recv_handle = thread::spawn(move || {
            for _ in 0..(num_threads * items_per_thread) {
                channel_recv.pop().unwrap();
                total_received_recv.fetch_add(1, Ordering::Relaxed);
            }
        });
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        recv_handle.join().unwrap();
        
        // Verify counts
        assert_eq!(total_sent.load(Ordering::Relaxed), num_threads * items_per_thread);
        assert_eq!(total_received.load(Ordering::Relaxed), num_threads * items_per_thread);
        assert!(channel.is_empty());
        
        let stats = channel.stats().unwrap();
        assert_eq!(stats.total_pushes, num_threads * items_per_thread);
        assert_eq!(stats.total_pops, num_threads * items_per_thread);
    }
}
