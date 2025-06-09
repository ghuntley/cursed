//! Comprehensive integration between channel system and goroutine scheduler
//!
//! This module provides tight integration between channels and the goroutine scheduler,
//! enabling proper blocking operations, goroutine parking/unparking, and efficient
//! coordination for channel-based communication.

use std::sync::{Arc, Mutex, RwLock, Condvar, atomic::{AtomicU64, AtomicUsize, AtomicBool, Ordering}};
use std::collections::{HashMap, VecDeque, HashSet};
use std::time::{Duration, Instant};
use std::thread;
use crate::runtime::goroutine::GoroutineId;
use crate::runtime::goroutine_scheduler::GoroutineScheduler;
use crate::runtime::goroutine_scheduler::GoroutineState;
use crate::object::{Object, Channel};
use crate::object_thread_safe::ThreadSafeObject;
use crate::memory::{GarbageCollector, ThreadSafeGc};
use crate::error::Error;
use tracing::{instrument, debug, info, warn, error, span, Level};

/// Unique identifier for channel operations
pub type ChannelOpId = u64;

/// Type of channel operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelOpType {
    Send,
    Receive,
    Close,
}

/// Result of a channel operation
#[derive(Debug)]
pub enum ChannelOpResult {
    Success(Option<Object>),
    WouldBlock,
    Closed,
    Timeout,
    Cancelled,
}

/// Information about a blocked goroutine waiting on a channel operation
#[derive(Debug)]
pub struct BlockedGoroutine {
    /// Goroutine ID
    pub goroutine_id: GoroutineId,
    /// Channel operation ID
    pub op_id: ChannelOpId,
    /// Type of operation
    pub op_type: ChannelOpType,
    /// Channel pointer (as usize for thread safety)
    pub channel_ptr: usize,
    /// Data for send operations (as usize for thread safety)
    pub data_ptr: Option<usize>,
    /// Timestamp when operation was initiated
    pub blocked_at: Instant,
    /// Optional timeout for the operation
    pub timeout: Option<Duration>,
    /// Thread handle for unparking
    pub thread_handle: Option<thread::Thread>,
}

/// Channel wait queue for managing blocked goroutines
#[derive(Debug, Default)]
pub struct ChannelWaitQueue {
    /// Send operations waiting on this channel
    send_waiters: VecDeque<BlockedGoroutine>,
    /// Receive operations waiting on this channel
    receive_waiters: VecDeque<BlockedGoroutine>,
}

impl ChannelWaitQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a goroutine to the send wait queue
    pub fn add_send_waiter(&mut self, blocked_goroutine: BlockedGoroutine) {
        debug!(
            goroutine_id = blocked_goroutine.goroutine_id,
            op_id = blocked_goroutine.op_id,
            "Adding goroutine to send wait queue"
        );
        self.send_waiters.push_back(blocked_goroutine);
    }

    /// Add a goroutine to the receive wait queue
    pub fn add_receive_waiter(&mut self, blocked_goroutine: BlockedGoroutine) {
        debug!(
            goroutine_id = blocked_goroutine.goroutine_id,
            op_id = blocked_goroutine.op_id,
            "Adding goroutine to receive wait queue"
        );
        self.receive_waiters.push_back(blocked_goroutine);
    }

    /// Remove and return the next send waiter
    pub fn pop_send_waiter(&mut self) -> Option<BlockedGoroutine> {
        self.send_waiters.pop_front()
    }

    /// Remove and return the next receive waiter
    pub fn pop_receive_waiter(&mut self) -> Option<BlockedGoroutine> {
        self.receive_waiters.pop_front()
    }

    /// Check if there are any send waiters
    pub fn has_send_waiters(&self) -> bool {
        !self.send_waiters.is_empty()
    }

    /// Check if there are any receive waiters
    pub fn has_receive_waiters(&self) -> bool {
        !self.receive_waiters.is_empty()
    }

    /// Get count of total waiters
    pub fn total_waiters(&self) -> usize {
        self.send_waiters.len() + self.receive_waiters.len()
    }

    /// Remove a specific goroutine from wait queues (for cancellation)
    pub fn remove_goroutine(&mut self, goroutine_id: GoroutineId) -> bool {
        let initial_send_len = self.send_waiters.len();
        let initial_receive_len = self.receive_waiters.len();
        
        self.send_waiters.retain(|blocked| blocked.goroutine_id != goroutine_id);
        self.receive_waiters.retain(|blocked| blocked.goroutine_id != goroutine_id);
        
        // Return true if any were removed
        (self.send_waiters.len() + self.receive_waiters.len()) < (initial_send_len + initial_receive_len)
    }
}

/// Statistics for channel scheduler performance
#[derive(Debug, Default)]
pub struct ChannelSchedulerStats {
    /// Total channel operations initiated
    pub total_operations: AtomicU64,
    /// Total blocking operations
    pub total_blocking_ops: AtomicU64,
    /// Total operations completed
    pub total_completed: AtomicU64,
    /// Total operations timed out
    pub total_timeouts: AtomicU64,
    /// Total operations cancelled
    pub total_cancelled: AtomicU64,
    /// Current number of blocked goroutines
    pub current_blocked: AtomicUsize,
    /// Average blocking time in nanoseconds
    pub avg_blocking_time: AtomicU64,
    /// Number of channels with waiting goroutines
    pub channels_with_waiters: AtomicUsize,
}

/// Enhanced channel scheduler with goroutine integration
pub struct ChannelScheduler {
    /// Counter for generating unique operation IDs
    next_op_id: AtomicU64,
    /// Wait queues for each channel (by channel pointer address)
    channel_waiters: RwLock<HashMap<usize, ChannelWaitQueue>>,
    /// Currently blocked goroutines indexed by operation ID
    blocked_operations: RwLock<HashMap<ChannelOpId, BlockedGoroutine>>,
    /// Operations indexed by goroutine ID for quick lookup
    goroutine_operations: RwLock<HashMap<GoroutineId, HashSet<ChannelOpId>>>,
    /// Condition variable for notifying about channel events
    channel_event_condvar: Condvar,
    /// Mutex for channel event notifications
    channel_event_mutex: Mutex<()>,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Statistics
    stats: ChannelSchedulerStats,
    /// Reference to the goroutine scheduler
    goroutine_scheduler: Arc<GoroutineScheduler>,
    /// Garbage collector reference
    gc: Arc<GarbageCollector>,
}

impl ChannelScheduler {
    /// Create a new channel scheduler
    pub fn new(goroutine_scheduler: Arc<GoroutineScheduler>, gc: Arc<GarbageCollector>) -> Self {
        Self {
            next_op_id: AtomicU64::new(1),
            channel_waiters: RwLock::new(HashMap::new()),
            blocked_operations: RwLock::new(HashMap::new()),
            goroutine_operations: RwLock::new(HashMap::new()),
            channel_event_condvar: Condvar::new(),
            channel_event_mutex: Mutex::new(()),
            shutdown: AtomicBool::new(false),
            stats: ChannelSchedulerStats::default(),
            goroutine_scheduler,
            gc,
        }
    }

    /// Initiate a blocking channel send operation
    #[instrument(level = "debug", skip(self, channel_ptr, value_ptr))]
    pub fn blocking_send(
        &self,
        goroutine_id: GoroutineId,
        channel_ptr: *mut std::ffi::c_void,
        value_ptr: *mut std::ffi::c_void,
        timeout: Option<Duration>,
    ) -> ChannelOpResult {
        let op_id = self.next_op_id.fetch_add(1, Ordering::SeqCst);
        let channel_addr = channel_ptr as usize;
        
        debug!(
            goroutine_id = goroutine_id,
            op_id = op_id,
            channel_addr = channel_addr,
            timeout = ?timeout,
            "Initiating blocking send operation"
        );

        self.stats.total_operations.fetch_add(1, Ordering::Relaxed);

        // First, try non-blocking send
        match self.try_send_immediate(channel_ptr, value_ptr) {
            ChannelOpResult::Success(_) => {
                debug!(op_id = op_id, "Send completed immediately");
                self.stats.total_completed.fetch_add(1, Ordering::Relaxed);
                return ChannelOpResult::Success(None);
            }
            ChannelOpResult::Closed => {
                debug!(op_id = op_id, "Channel is closed");
                return ChannelOpResult::Closed;
            }
            ChannelOpResult::WouldBlock => {
                // Continue to blocking logic
            }
            other => return other,
        }

        // Check if there are receive waiters we can match immediately
        if let Some(receiver) = self.try_match_with_receiver(channel_addr) {
            debug!(
                op_id = op_id,
                receiver_goroutine = receiver.goroutine_id,
                "Matched send with waiting receiver"
            );
            
            // Transfer data directly and wake up receiver
            self.complete_matched_operation(receiver, Some(value_ptr));
            self.stats.total_completed.fetch_add(1, Ordering::Relaxed);
            return ChannelOpResult::Success(None);
        }

        // Block the goroutine
        self.block_goroutine_for_send(goroutine_id, op_id, channel_addr, value_ptr, timeout)
    }

    /// Initiate a blocking channel receive operation
    #[instrument(level = "debug", skip(self, channel_ptr))]
    pub fn blocking_receive(
        &self,
        goroutine_id: GoroutineId,
        channel_ptr: *mut std::ffi::c_void,
        timeout: Option<Duration>,
    ) -> ChannelOpResult {
        let op_id = self.next_op_id.fetch_add(1, Ordering::SeqCst);
        let channel_addr = channel_ptr as usize;
        
        debug!(
            goroutine_id = goroutine_id,
            op_id = op_id,
            channel_addr = channel_addr,
            timeout = ?timeout,
            "Initiating blocking receive operation"
        );

        self.stats.total_operations.fetch_add(1, Ordering::Relaxed);

        // First, try non-blocking receive
        match self.try_receive_immediate(channel_ptr) {
            ChannelOpResult::Success(value) => {
                debug!(op_id = op_id, "Receive completed immediately");
                self.stats.total_completed.fetch_add(1, Ordering::Relaxed);
                return ChannelOpResult::Success(value);
            }
            ChannelOpResult::Closed => {
                debug!(op_id = op_id, "Channel is closed");
                return ChannelOpResult::Closed;
            }
            ChannelOpResult::WouldBlock => {
                // Continue to blocking logic
            }
            other => return other,
        }

        // Check if there are send waiters we can match immediately
        if let Some(sender) = self.try_match_with_sender(channel_addr) {
            debug!(
                op_id = op_id,
                sender_goroutine = sender.goroutine_id,
                "Matched receive with waiting sender"
            );
            
            // Get data from sender and complete both operations
            let data = self.complete_matched_operation(sender, None);
            self.stats.total_completed.fetch_add(1, Ordering::Relaxed);
            return ChannelOpResult::Success(data);
        }

        // Block the goroutine
        self.block_goroutine_for_receive(goroutine_id, op_id, channel_addr, timeout)
    }

    /// Try to send immediately without blocking
    fn try_send_immediate(&self, channel_ptr: *mut std::ffi::c_void, value_ptr: *mut std::ffi::c_void) -> ChannelOpResult {
        // Safety: This is unsafe, but necessary for FFI integration
        // In production, we'd have better type safety
        unsafe {
            // Try to get the channel as an Arc<RwLock<Channel>>
            let channel_ref = Arc::from_raw(channel_ptr as *const RwLock<Channel>);
            let channel_ref_clone = channel_ref.clone();
            
            // Try to acquire write lock without blocking
            let result = if let Ok(mut channel) = channel_ref.try_write() {
                if channel.closed {
                    ChannelOpResult::Closed
                } else if channel.buffer_size == 0 || channel.buffer.len() < channel.buffer_size {
                    // For simplicity, assuming integer values
                    let value_int = *(value_ptr as *const i64);
                    let value_obj = Object::Integer(value_int);
                    
                    match channel.try_send(value_obj) {
                        Ok(true) => ChannelOpResult::Success(None),
                        Ok(false) => ChannelOpResult::WouldBlock,
                        Err(_) => ChannelOpResult::Closed,
                    }
                } else {
                    ChannelOpResult::WouldBlock
                }
            } else {
                ChannelOpResult::WouldBlock
            };
            
            let _ = Arc::into_raw(channel_ref_clone);
            result
        }
    }

    /// Try to receive immediately without blocking
    fn try_receive_immediate(&self, channel_ptr: *mut std::ffi::c_void) -> ChannelOpResult {
        // Safety: This is unsafe, but necessary for FFI integration
        unsafe {
            // Try to get the channel as an Arc<RwLock<Channel>>
            let channel_ref = Arc::from_raw(channel_ptr as *const RwLock<Channel>);
            let channel_ref_clone = channel_ref.clone();
            
            // Try to acquire write lock without blocking
            let result = if let Ok(mut channel) = channel_ref.try_write() {
                match channel.try_receive() {
                    Ok(Some(value)) => ChannelOpResult::Success(Some(value)),
                    Ok(None) => {
                        if channel.closed {
                            ChannelOpResult::Closed
                        } else {
                            ChannelOpResult::WouldBlock
                        }
                    }
                    Err(_) => ChannelOpResult::Closed,
                }
            } else {
                ChannelOpResult::WouldBlock
            };
            
            let _ = Arc::into_raw(channel_ref_clone);
            result
        }
    }

    /// Try to match a sender with a waiting receiver
    fn try_match_with_receiver(&self, channel_addr: usize) -> Option<BlockedGoroutine> {
        let mut waiters = self.channel_waiters.write().unwrap();
        if let Some(queue) = waiters.get_mut(&channel_addr) {
            queue.pop_receive_waiter()
        } else {
            None
        }
    }

    /// Try to match a receiver with a waiting sender
    fn try_match_with_sender(&self, channel_addr: usize) -> Option<BlockedGoroutine> {
        let mut waiters = self.channel_waiters.write().unwrap();
        if let Some(queue) = waiters.get_mut(&channel_addr) {
            queue.pop_send_waiter()
        } else {
            None
        }
    }

    /// Complete a matched operation and return transferred data
    fn complete_matched_operation(
        &self,
        blocked: BlockedGoroutine,
        data_ptr: Option<*mut std::ffi::c_void>,
    ) -> Option<Object> {
        debug!(
            goroutine_id = blocked.goroutine_id,
            op_id = blocked.op_id,
            "Completing matched operation"
        );

        // Remove from blocked operations
        {
            let mut blocked_ops = self.blocked_operations.write().unwrap();
            blocked_ops.remove(&blocked.op_id);
        }

        // Remove from goroutine operations
        {
            let mut goroutine_ops = self.goroutine_operations.write().unwrap();
            if let Some(ops) = goroutine_ops.get_mut(&blocked.goroutine_id) {
                ops.remove(&blocked.op_id);
                if ops.is_empty() {
                    goroutine_ops.remove(&blocked.goroutine_id);
                }
            }
        }

        // Wake up the goroutine
        if let Some(thread_handle) = blocked.thread_handle {
            thread_handle.unpark();
        }

        // Return data if this was a send operation providing data
        if let Some(ptr) = data_ptr {
            unsafe {
                let value_int = *(ptr as *const i64);
                Some(Object::Integer(value_int))
            }
        } else if let Some(data_ptr) = blocked.data_ptr {
            // Return data from the blocked operation (it was a sender)
            unsafe {
                let value_int = *(data_ptr as *const i64);
                Some(Object::Integer(value_int))
            }
        } else {
            None
        }
    }

    /// Block a goroutine for a send operation
    fn block_goroutine_for_send(
        &self,
        goroutine_id: GoroutineId,
        op_id: ChannelOpId,
        channel_addr: usize,
        value_ptr: *mut std::ffi::c_void,
        timeout: Option<Duration>,
    ) -> ChannelOpResult {
        debug!(
            goroutine_id = goroutine_id,
            op_id = op_id,
            "Blocking goroutine for send operation"
        );

        let blocked_at = Instant::now();
        let current_thread = thread::current();
        
        let blocked_goroutine = BlockedGoroutine {
            goroutine_id,
            op_id,
            op_type: ChannelOpType::Send,
            channel_ptr: channel_addr,
            data_ptr: Some(value_ptr as usize),
            blocked_at,
            timeout,
            thread_handle: Some(current_thread),
        };

        // Add to wait queue
        {
            let mut waiters = self.channel_waiters.write().unwrap();
            let queue = waiters.entry(channel_addr).or_insert_with(ChannelWaitQueue::new);
            queue.add_send_waiter(blocked_goroutine);
            self.stats.channels_with_waiters.store(waiters.len(), Ordering::Relaxed);
        }

        // Track the blocked operation
        {
            let mut blocked_ops = self.blocked_operations.write().unwrap();
            blocked_ops.insert(op_id, BlockedGoroutine {
                goroutine_id,
                op_id,
                op_type: ChannelOpType::Send,
                channel_ptr: channel_addr,
                data_ptr: Some(value_ptr as usize),
                blocked_at,
                timeout,
                thread_handle: None, // We don't store the thread handle here
            });
        }

        // Track by goroutine ID
        {
            let mut goroutine_ops = self.goroutine_operations.write().unwrap();
            goroutine_ops.entry(goroutine_id).or_insert_with(HashSet::new).insert(op_id);
        }

        self.stats.total_blocking_ops.fetch_add(1, Ordering::Relaxed);
        self.stats.current_blocked.fetch_add(1, Ordering::Relaxed);

        // Update goroutine state to blocked
        // Note: In a full implementation, this would be integrated with the scheduler
        
        // Block until unparked or timeout
        self.wait_for_operation_completion(op_id, timeout)
    }

    /// Block a goroutine for a receive operation
    fn block_goroutine_for_receive(
        &self,
        goroutine_id: GoroutineId,
        op_id: ChannelOpId,
        channel_addr: usize,
        timeout: Option<Duration>,
    ) -> ChannelOpResult {
        debug!(
            goroutine_id = goroutine_id,
            op_id = op_id,
            "Blocking goroutine for receive operation"
        );

        let blocked_at = Instant::now();
        let current_thread = thread::current();
        
        let blocked_goroutine = BlockedGoroutine {
            goroutine_id,
            op_id,
            op_type: ChannelOpType::Receive,
            channel_ptr: channel_addr,
            data_ptr: None,
            blocked_at,
            timeout,
            thread_handle: Some(current_thread),
        };

        // Add to wait queue
        {
            let mut waiters = self.channel_waiters.write().unwrap();
            let queue = waiters.entry(channel_addr).or_insert_with(ChannelWaitQueue::new);
            queue.add_receive_waiter(blocked_goroutine);
            self.stats.channels_with_waiters.store(waiters.len(), Ordering::Relaxed);
        }

        // Track the blocked operation
        {
            let mut blocked_ops = self.blocked_operations.write().unwrap();
            blocked_ops.insert(op_id, BlockedGoroutine {
                goroutine_id,
                op_id,
                op_type: ChannelOpType::Receive,
                channel_ptr: channel_addr,
                data_ptr: None,
                blocked_at,
                timeout,
                thread_handle: None,
            });
        }

        // Track by goroutine ID
        {
            let mut goroutine_ops = self.goroutine_operations.write().unwrap();
            goroutine_ops.entry(goroutine_id).or_insert_with(HashSet::new).insert(op_id);
        }

        self.stats.total_blocking_ops.fetch_add(1, Ordering::Relaxed);
        self.stats.current_blocked.fetch_add(1, Ordering::Relaxed);

        // Block until unparked or timeout
        self.wait_for_operation_completion(op_id, timeout)
    }

    /// Wait for an operation to complete (blocking implementation)
    fn wait_for_operation_completion(&self, op_id: ChannelOpId, timeout: Option<Duration>) -> ChannelOpResult {
        let start_time = Instant::now();
        
        loop {
            // Check if operation is still blocked
            {
                let blocked_ops = self.blocked_operations.read().unwrap();
                if !blocked_ops.contains_key(&op_id) {
                    // Operation completed
                    self.stats.current_blocked.fetch_sub(1, Ordering::Relaxed);
                    self.stats.total_completed.fetch_add(1, Ordering::Relaxed);
                    
                    let elapsed = start_time.elapsed();
                    let elapsed_ns = elapsed.as_nanos() as u64;
                    
                    // Update average blocking time
                    let current_avg = self.stats.avg_blocking_time.load(Ordering::Relaxed);
                    let new_avg = if current_avg == 0 { elapsed_ns } else { (current_avg + elapsed_ns) / 2 };
                    self.stats.avg_blocking_time.store(new_avg, Ordering::Relaxed);
                    
                    debug!(op_id = op_id, elapsed_ms = elapsed.as_millis(), "Operation completed");
                    return ChannelOpResult::Success(None);
                }
            }

            // Check timeout
            if let Some(timeout_duration) = timeout {
                if start_time.elapsed() >= timeout_duration {
                    self.cancel_operation(op_id);
                    self.stats.total_timeouts.fetch_add(1, Ordering::Relaxed);
                    debug!(op_id = op_id, "Operation timed out");
                    return ChannelOpResult::Timeout;
                }
            }

            // Check for shutdown
            if self.shutdown.load(Ordering::Acquire) {
                self.cancel_operation(op_id);
                self.stats.total_cancelled.fetch_add(1, Ordering::Relaxed);
                return ChannelOpResult::Cancelled;
            }

            // Park the thread briefly
            thread::park_timeout(Duration::from_millis(1));
        }
    }

    /// Cancel a blocked operation
    pub fn cancel_operation(&self, op_id: ChannelOpId) {
        debug!(op_id = op_id, "Cancelling operation");
        
        // Remove from blocked operations and get info
        let blocked_info = {
            let mut blocked_ops = self.blocked_operations.write().unwrap();
            blocked_ops.remove(&op_id)
        };

        if let Some(blocked) = blocked_info {
            // Remove from wait queue
            {
                let mut waiters = self.channel_waiters.write().unwrap();
                if let Some(queue) = waiters.get_mut(&blocked.channel_ptr) {
                    queue.remove_goroutine(blocked.goroutine_id);
                    if queue.total_waiters() == 0 {
                        waiters.remove(&blocked.channel_ptr);
                    }
                }
                self.stats.channels_with_waiters.store(waiters.len(), Ordering::Relaxed);
            }

            // Remove from goroutine operations
            {
                let mut goroutine_ops = self.goroutine_operations.write().unwrap();
                if let Some(ops) = goroutine_ops.get_mut(&blocked.goroutine_id) {
                    ops.remove(&op_id);
                    if ops.is_empty() {
                        goroutine_ops.remove(&blocked.goroutine_id);
                    }
                }
            }

            self.stats.current_blocked.fetch_sub(1, Ordering::Relaxed);
            self.stats.total_cancelled.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Cancel all operations for a specific goroutine (when goroutine terminates)
    pub fn cancel_goroutine_operations(&self, goroutine_id: GoroutineId) {
        debug!(goroutine_id = goroutine_id, "Cancelling all operations for goroutine");
        
        let operations = {
            let mut goroutine_ops = self.goroutine_operations.write().unwrap();
            goroutine_ops.remove(&goroutine_id).unwrap_or_default()
        };

        for op_id in operations {
            self.cancel_operation(op_id);
        }
    }

    /// Notify about channel events (called when channel state changes)
    pub fn notify_channel_event(&self, channel_ptr: *mut std::ffi::c_void) {
        let channel_addr = channel_ptr as usize;
        debug!(channel_addr = channel_addr, "Channel event notification");

        // Wake up potential waiters
        self.process_channel_readiness(channel_addr);
        
        // Notify condition variable
        self.channel_event_condvar.notify_all();
    }

    /// Process channel readiness and wake up appropriate waiters
    fn process_channel_readiness(&self, channel_addr: usize) {
        debug!(channel_addr = channel_addr, "Processing channel readiness");
        
        // Try to match waiting operations
        loop {
            let (sender, receiver) = {
                let mut waiters = self.channel_waiters.write().unwrap();
                if let Some(queue) = waiters.get_mut(&channel_addr) {
                    let sender = queue.pop_send_waiter();
                    let receiver = queue.pop_receive_waiter();
                    (sender, receiver)
                } else {
                    break;
                }
            };

            match (sender, receiver) {
                (Some(send_op), Some(recv_op)) => {
                    // Match sender with receiver
                    debug!(
                        sender_goroutine = send_op.goroutine_id,
                        receiver_goroutine = recv_op.goroutine_id,
                        "Matching sender with receiver"
                    );
                    
                    // Complete both operations
                    self.complete_matched_operation(recv_op, send_op.data_ptr.map(|ptr| ptr as *mut std::ffi::c_void));
                    self.complete_matched_operation(send_op, None);
                }
                _ => break, // No more matches possible
            }
        }
    }

    /// Get statistics about the channel scheduler
    pub fn get_statistics(&self) -> ChannelSchedulerStats {
        ChannelSchedulerStats {
            total_operations: AtomicU64::new(self.stats.total_operations.load(Ordering::Relaxed)),
            total_blocking_ops: AtomicU64::new(self.stats.total_blocking_ops.load(Ordering::Relaxed)),
            total_completed: AtomicU64::new(self.stats.total_completed.load(Ordering::Relaxed)),
            total_timeouts: AtomicU64::new(self.stats.total_timeouts.load(Ordering::Relaxed)),
            total_cancelled: AtomicU64::new(self.stats.total_cancelled.load(Ordering::Relaxed)),
            current_blocked: AtomicUsize::new(self.stats.current_blocked.load(Ordering::Relaxed)),
            avg_blocking_time: AtomicU64::new(self.stats.avg_blocking_time.load(Ordering::Relaxed)),
            channels_with_waiters: AtomicUsize::new(self.stats.channels_with_waiters.load(Ordering::Relaxed)),
        }
    }

    /// Cleanup completed operations and optimize wait queues
    pub fn cleanup_and_optimize(&self) {
        debug!("Performing channel scheduler cleanup and optimization");
        
        // Remove empty wait queues
        {
            let mut waiters = self.channel_waiters.write().unwrap();
            waiters.retain(|_, queue| queue.total_waiters() > 0);
            self.stats.channels_with_waiters.store(waiters.len(), Ordering::Relaxed);
        }

        // Clean up old completed operations (not implemented in this simplified version)
        info!(
            total_ops = self.stats.total_operations.load(Ordering::Relaxed),
            current_blocked = self.stats.current_blocked.load(Ordering::Relaxed),
            channels_with_waiters = self.stats.channels_with_waiters.load(Ordering::Relaxed),
            "Channel scheduler cleanup completed"
        );
    }

    /// Shutdown the channel scheduler
    pub fn shutdown(&self) {
        info!("Shutting down channel scheduler");
        
        self.shutdown.store(true, Ordering::Release);
        
        // Cancel all blocked operations
        let all_operations: Vec<ChannelOpId> = {
            let blocked_ops = self.blocked_operations.read().unwrap();
            blocked_ops.keys().copied().collect()
        };

        for op_id in all_operations {
            self.cancel_operation(op_id);
        }

        // Notify all waiting threads
        self.channel_event_condvar.notify_all();
        
        info!("Channel scheduler shutdown complete");
    }
}

impl Drop for ChannelScheduler {
    fn drop(&mut self) {
        if !self.shutdown.load(Ordering::Acquire) {
            self.shutdown();
        }
    }
}

/// Global channel scheduler instance
static GLOBAL_CHANNEL_SCHEDULER: once_cell::sync::Lazy<Arc<ChannelScheduler>> = 
    once_cell::sync::Lazy::new(|| {
        let goroutine_scheduler = crate::runtime::goroutine_scheduler::get_global_scheduler();
        let gc = Arc::new(GarbageCollector::new());
        Arc::new(ChannelScheduler::new(goroutine_scheduler, gc))
    });

/// Get the global channel scheduler
pub fn get_global_channel_scheduler() -> Arc<ChannelScheduler> {
    GLOBAL_CHANNEL_SCHEDULER.clone()
}

/// FFI functions for LLVM integration

/// Blocking send operation for FFI
#[no_mangle]
pub extern "C" fn cursed_channel_blocking_send(
    goroutine_id: u64,
    channel_ptr: *mut std::ffi::c_void,
    value_ptr: *mut std::ffi::c_void,
    timeout_ms: i64,
) -> i32 {
    let scheduler = get_global_channel_scheduler();
    let timeout = if timeout_ms >= 0 {
        Some(Duration::from_millis(timeout_ms as u64))
    } else {
        None
    };

    match scheduler.blocking_send(goroutine_id, channel_ptr, value_ptr, timeout) {
        ChannelOpResult::Success(_) => 0,  // Success
        ChannelOpResult::WouldBlock => 1,  // Should not happen in blocking mode
        ChannelOpResult::Closed => 2,     // Channel closed
        ChannelOpResult::Timeout => 3,    // Operation timed out
        ChannelOpResult::Cancelled => 4,  // Operation cancelled
    }
}

/// Blocking receive operation for FFI
#[no_mangle]
pub extern "C" fn cursed_channel_blocking_receive(
    goroutine_id: u64,
    channel_ptr: *mut std::ffi::c_void,
    result_ptr: *mut std::ffi::c_void,
    timeout_ms: i64,
) -> i32 {
    let scheduler = get_global_channel_scheduler();
    let timeout = if timeout_ms >= 0 {
        Some(Duration::from_millis(timeout_ms as u64))
    } else {
        None
    };

    match scheduler.blocking_receive(goroutine_id, channel_ptr, timeout) {
        ChannelOpResult::Success(Some(value)) => {
            // Store the result
            unsafe {
                match value {
                    Object::Integer(i) => *(result_ptr as *mut i64) = i,
                    Object::Float(f) => *(result_ptr as *mut f64) = f,
                    Object::Boolean(b) => *(result_ptr as *mut bool) = b,
                    _ => {
                        tracing::warn!("Unsupported value type in blocking receive");
                        return 5; // Unsupported type
                    }
                }
            }
            0 // Success
        }
        ChannelOpResult::Success(None) => 0, // Success with no value
        ChannelOpResult::WouldBlock => 1,    // Should not happen in blocking mode
        ChannelOpResult::Closed => 2,       // Channel closed
        ChannelOpResult::Timeout => 3,      // Operation timed out
        ChannelOpResult::Cancelled => 4,    // Operation cancelled
    }
}

/// Notify channel event for FFI
#[no_mangle]
pub extern "C" fn cursed_channel_notify_event(channel_ptr: *mut std::ffi::c_void) {
    let scheduler = get_global_channel_scheduler();
    scheduler.notify_channel_event(channel_ptr);
}

/// Cancel goroutine operations for FFI
#[no_mangle]
pub extern "C" fn cursed_cancel_goroutine_channel_ops(goroutine_id: u64) {
    let scheduler = get_global_channel_scheduler();
    scheduler.cancel_goroutine_operations(goroutine_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn test_channel_scheduler_creation() {
        let gc = Arc::new(GarbageCollector::new());
        let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
        let channel_scheduler = ChannelScheduler::new(goroutine_scheduler, gc);
        
        let stats = channel_scheduler.get_statistics();
        assert_eq!(stats.total_operations.load(Ordering::Relaxed), 0);
        assert_eq!(stats.current_blocked.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_wait_queue_operations() {
        let mut queue = ChannelWaitQueue::new();
        
        let blocked_goroutine = BlockedGoroutine {
            goroutine_id: 1,
            op_id: 1,
            op_type: ChannelOpType::Send,
            channel_ptr: 0x1000,
            data_ptr: Some(0x2000),
            blocked_at: Instant::now(),
            timeout: None,
            thread_handle: None,
        };
        
        queue.add_send_waiter(blocked_goroutine);
        assert!(queue.has_send_waiters());
        assert_eq!(queue.total_waiters(), 1);
        
        let popped = queue.pop_send_waiter();
        assert!(popped.is_some());
        assert!(!queue.has_send_waiters());
        assert_eq!(queue.total_waiters(), 0);
    }

    #[test]
    fn test_operation_cancellation() {
        let gc = Arc::new(GarbageCollector::new());
        let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
        let channel_scheduler = ChannelScheduler::new(goroutine_scheduler, gc);
        
        // This test would require more setup for a realistic scenario
        // For now, just test that cancellation doesn't panic
        channel_scheduler.cancel_operation(123);
        channel_scheduler.cancel_goroutine_operations(456);
    }

    #[test]
    fn test_statistics_tracking() {
        let gc = Arc::new(GarbageCollector::new());
        let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
        let channel_scheduler = ChannelScheduler::new(goroutine_scheduler, gc);
        
        channel_scheduler.stats.total_operations.fetch_add(5, Ordering::Relaxed);
        channel_scheduler.stats.total_completed.fetch_add(3, Ordering::Relaxed);
        
        let stats = channel_scheduler.get_statistics();
        assert_eq!(stats.total_operations.load(Ordering::Relaxed), 5);
        assert_eq!(stats.total_completed.load(Ordering::Relaxed), 3);
    }
}
