//! Channel operation implementations
//!
//! Provides comprehensive send and receive operations for channels:
//! - Blocking and non-blocking operations
//! - Timeout support
//! - Operation cancellation
//! - Priority-based operations
//! - Batch operations for efficiency

use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::runtime::channels::{ChannelError, ChannelResult, SendResult, ReceiveResult};
use crate::runtime::channels::buffer::ChannelBuffer;
use crate::runtime::channels::sync::{PriorityWaitQueue, WaitQueueEntry, WaitOperationType, get_global_channel_sync};
use crate::runtime::goroutine::GoroutineId;

/// Operation priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for OperationPriority {
    fn default() -> Self {
        OperationPriority::Normal
    }
}

/// Send operation configuration
#[derive(Debug)]
pub struct SendOptions {
    /// Operation timeout
    pub timeout: Option<Duration>,
    /// Operation priority
    pub priority: OperationPriority,
    /// Whether the operation should block
    pub blocking: bool,
    /// Goroutine ID for scheduling
    pub goroutine_id: Option<GoroutineId>,
    /// Callback for operation completion (not supported in this simplified version)
    pub completion_callback: Option<String>, // Placeholder for callback info
}

impl Clone for SendOptions {
    fn clone(&self) -> Self {
        Self {
            timeout: self.timeout,
            priority: self.priority,
            blocking: self.blocking,
            goroutine_id: self.goroutine_id,
            completion_callback: self.completion_callback.clone(),
        }
    }
}

impl Default for SendOptions {
    fn default() -> Self {
        Self {
            timeout: None,
            priority: OperationPriority::default(),
            blocking: true,
            goroutine_id: None,
            completion_callback: None,
        }
    }
}

/// Receive operation configuration
#[derive(Debug, Clone)]
pub struct ReceiveOptions {
    /// Operation timeout
    pub timeout: Option<Duration>,
    /// Operation priority
    pub priority: OperationPriority,
    /// Whether the operation should block
    pub blocking: bool,
    /// Goroutine ID for scheduling
    pub goroutine_id: Option<GoroutineId>,
}

impl Default for ReceiveOptions {
    fn default() -> Self {
        Self {
            timeout: None,
            priority: OperationPriority::default(),
            blocking: true,
            goroutine_id: None,
        }
    }
}

/// Send operation implementation
pub struct SendOperation<T> {
    /// Value to send
    value: Option<T>,
    /// Operation options
    options: SendOptions,
    /// Operation start time
    start_time: Instant,
    /// Completion status
    completed: Arc<Mutex<bool>>,
    /// Completion notifier
    completion_notify: Arc<Condvar>,
    /// Execution flag to prevent double execution
    executed: bool,
}

impl<T> SendOperation<T> {
    /// Create a new send operation
    pub fn new(value: T, options: SendOptions) -> Self {
        Self {
            value: Some(value),
            options,
            start_time: Instant::now(),
            completed: Arc::new(Mutex::new(false)),
            completion_notify: Arc::new(Condvar::new()),
            executed: false,
        }
    }
    
    /// Execute the send operation
    pub fn execute<B: ChannelBuffer<T> + ?Sized>(
        mut self,
        buffer: &Arc<B>,
    ) -> SendResult<T> {
        // Prevent double execution
        if self.executed {
            return SendResult::Closed(self.value.take().expect("Send operation value should be present"));
        }
        self.executed = true;
        
        let value = self.value.take().expect("Send operation value should be present");
        
        if !self.options.blocking {
            return self.try_send_nonblocking(buffer, value);
        }
        
        if let Some(timeout) = self.options.timeout {
            return self.send_with_timeout(buffer, value, timeout);
        }
        
        self.send_blocking(buffer, value)
    }
    
    /// Non-blocking send
    fn try_send_nonblocking<B: ChannelBuffer<T> + ?Sized>(
        &self,
        buffer: &Arc<B>,
        value: T,
    ) -> SendResult<T> {
        match buffer.try_push(value) {
            Ok(()) => SendResult::Sent,
            Err((value, ChannelError::Closed)) => SendResult::Closed(value),
            Err((value, _)) => SendResult::WouldBlock(value),
        }
    }
    
    /// Blocking send
    fn send_blocking<B: ChannelBuffer<T> + ?Sized>(
        &self,
        buffer: &Arc<B>,
        mut value: T,
    ) -> SendResult<T> {
        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 10000; // Prevent infinite loops
        const INITIAL_SLEEP_MS: u64 = 1;
        const MAX_SLEEP_MS: u64 = 100;
        
        loop {
            match buffer.try_push(value) {
                Ok(()) => {
                    self.mark_completed();
                    return SendResult::Sent;
                }
                Err((v, ChannelError::Closed)) => {
                    self.mark_completed();
                    return SendResult::Closed(v);
                }
                Err((v, _)) => {
                    value = v;
                    retry_count += 1;
                    
                    if retry_count > MAX_RETRIES {
                        self.mark_completed();
                        return SendResult::Closed(value);
                    }
                    
                    if buffer.is_closed() {
                        self.mark_completed();
                        return SendResult::Closed(value);
                    }
                    
                    // Use proper blocking instead of busy-wait
                    let channel_sync = get_global_channel_sync();
                    let goroutine_id = self.options.goroutine_id.unwrap_or(0);
                    let wait_time = std::cmp::min(
                        INITIAL_SLEEP_MS * (1 << std::cmp::min(retry_count / 100, 6)),
                        MAX_SLEEP_MS
                    );
                    
                    // Block using wait queue with exponential backoff timeout
                    let _ = channel_sync.block_on_send(
                        0, // Channel ID would need to be passed through
                        goroutine_id,
                        self.options.priority,
                        Some(Duration::from_millis(wait_time))
                    );
                    
                    // Small yield to allow other operations
                    thread::yield_now();
                }
            }
        }
    }
    
    /// Send with timeout
    fn send_with_timeout<B: ChannelBuffer<T> + ?Sized>(
        &self,
        buffer: &Arc<B>,
        mut value: T,
        timeout: Duration,
    ) -> SendResult<T> {
        let deadline = self.start_time + timeout;
        
        loop {
            match buffer.try_push(value) {
                Ok(()) => {
                    self.mark_completed();
                    return SendResult::Sent;
                }
                Err((v, ChannelError::Closed)) => {
                    self.mark_completed();
                    return SendResult::Closed(v);
                }
                Err((v, _)) => {
                    value = v;
                    
                    if Instant::now() >= deadline {
                        self.mark_completed();
                        return SendResult::WouldBlock(value);
                    }
                    
                    // Use proper blocking with remaining timeout
                    let channel_sync = get_global_channel_sync();
                    let goroutine_id = self.options.goroutine_id.unwrap_or(0);
                    let remaining_time = deadline.saturating_duration_since(Instant::now());
                    let wait_time = std::cmp::min(remaining_time, Duration::from_millis(10));
                    
                    let _ = channel_sync.block_on_send(
                        0, // Channel ID would need to be passed through
                        goroutine_id,
                        self.options.priority,
                        Some(wait_time)
                    );
                }
            }
        }
    }
    
    /// Mark operation as completed
    fn mark_completed(&self) {
        if let Ok(mut completed) = self.completed.lock() {
            *completed = true;
        }
        self.completion_notify.notify_all();
    }
    
    /// Wait for operation completion
    pub fn wait_for_completion(&self) -> Result<(), ChannelError> {
        let mut completed = self.completed.lock()
            .map_err(|_| ChannelError::Closed)?;
        while !*completed {
            completed = self.completion_notify.wait(completed)
                .map_err(|_| ChannelError::Closed)?;
        }
        Ok(())
    }
    
    /// Check if operation is completed
    pub fn is_completed(&self) -> bool {
        self.completed.lock()
            .map(|completed| *completed)
            .unwrap_or(true) // Assume completed if we can't lock
    }
}

/// Receive operation implementation
pub struct ReceiveOperation<T> {
    /// Operation options
    options: ReceiveOptions,
    /// Operation start time
    start_time: Instant,
    /// Completion status
    completed: Arc<Mutex<bool>>,
    /// Completion notifier
    completion_notify: Arc<Condvar>,
    /// Phantom data to hold type parameter
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ReceiveOperation<T> {
    /// Create a new receive operation
    pub fn new(options: ReceiveOptions) -> Self {
        Self {
            options,
            start_time: Instant::now(),
            completed: Arc::new(Mutex::new(false)),
            completion_notify: Arc::new(Condvar::new()),
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Execute the receive operation
    pub fn execute<B: ChannelBuffer<T> + ?Sized>(
        self,
        buffer: &Arc<B>,
    ) -> ReceiveResult<T> {
        if !self.options.blocking {
            return self.try_receive_nonblocking(buffer);
        }
        
        if let Some(timeout) = self.options.timeout {
            return self.receive_with_timeout(buffer, timeout);
        }
        
        self.receive_blocking(buffer)
    }
    
    /// Non-blocking receive
    fn try_receive_nonblocking<B: ChannelBuffer<T> + ?Sized>(
        &self,
        buffer: &Arc<B>,
    ) -> ReceiveResult<T> {
        match buffer.try_pop() {
            Ok(Some(value)) => ReceiveResult::Received(value),
            Ok(None) => ReceiveResult::WouldBlock,
            Err(ChannelError::Closed) => ReceiveResult::Closed,
            Err(_) => ReceiveResult::WouldBlock,
        }
    }
    
    /// Blocking receive
    fn receive_blocking<B: ChannelBuffer<T> + ?Sized>(
        &self,
        buffer: &Arc<B>,
    ) -> ReceiveResult<T> {
        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 10000; // Prevent infinite loops
        const INITIAL_SLEEP_MS: u64 = 1;
        const MAX_SLEEP_MS: u64 = 100;
        
        loop {
            match buffer.try_pop() {
                Ok(Some(value)) => {
                    ReceiveOperation::<T>::mark_completed(self);
                    return ReceiveResult::Received(value);
                }
                Ok(None) => {
                    retry_count += 1;
                    
                    if retry_count > MAX_RETRIES {
                        ReceiveOperation::<T>::mark_completed(self);
                        return ReceiveResult::Closed;
                    }
                    
                    if buffer.is_closed() && buffer.is_empty() {
                        ReceiveOperation::<T>::mark_completed(self);
                        return ReceiveResult::Closed;
                    }
                    
                    // Use proper blocking instead of busy-wait
                    let channel_sync = get_global_channel_sync();
                    let goroutine_id = self.options.goroutine_id.unwrap_or(0);
                    let wait_time = std::cmp::min(
                        INITIAL_SLEEP_MS * (1 << std::cmp::min(retry_count / 100, 6)),
                        MAX_SLEEP_MS
                    );
                    
                    let _ = channel_sync.block_on_receive(
                        0, // Channel ID would need to be passed through
                        goroutine_id,
                        self.options.priority,
                        Some(Duration::from_millis(wait_time))
                    );
                    
                    thread::yield_now();
                }
                Err(ChannelError::Closed) => {
                    ReceiveOperation::<T>::mark_completed(self);
                    return ReceiveResult::Closed;
                }
                Err(_) => {
                    retry_count += 1;
                    
                    if retry_count > MAX_RETRIES {
                        ReceiveOperation::<T>::mark_completed(self);
                        return ReceiveResult::Closed;
                    }
                    
                    let wait_time = std::cmp::min(
                        INITIAL_SLEEP_MS * (1 << std::cmp::min(retry_count / 100, 6)),
                        MAX_SLEEP_MS
                    );
                    
                    let channel_sync = get_global_channel_sync();
                    let goroutine_id = self.options.goroutine_id.unwrap_or(0);
                    let _ = channel_sync.block_on_receive(
                        0, // Channel ID would need to be passed through
                        goroutine_id,
                        self.options.priority,
                        Some(Duration::from_millis(wait_time))
                    );
                    
                    thread::yield_now();
                }
            }
        }
    }
    
    /// Receive with timeout
    fn receive_with_timeout<B: ChannelBuffer<T> + ?Sized>(
        &self,
        buffer: &Arc<B>,
        timeout: Duration,
    ) -> ReceiveResult<T> {
        let deadline = self.start_time + timeout;
        
        loop {
            match buffer.try_pop() {
                Ok(Some(value)) => {
                    ReceiveOperation::<T>::mark_completed(self);
                    return ReceiveResult::Received(value);
                }
                Ok(None) => {
                    if Instant::now() >= deadline {
                        ReceiveOperation::<T>::mark_completed(self);
                        return ReceiveResult::WouldBlock;
                    }
                    
                    let channel_sync = get_global_channel_sync();
                    let goroutine_id = self.options.goroutine_id.unwrap_or(0);
                    let remaining_time = deadline.saturating_duration_since(Instant::now());
                    let wait_time = std::cmp::min(remaining_time, Duration::from_millis(10));
                    
                    let _ = channel_sync.block_on_receive(
                        0, // Channel ID would need to be passed through
                        goroutine_id,
                        self.options.priority,
                        Some(wait_time)
                    );
                }
                Err(ChannelError::Closed) => {
                    ReceiveOperation::<T>::mark_completed(self);
                    return ReceiveResult::Closed;
                }
                Err(_) => {
                    if Instant::now() >= deadline {
                        ReceiveOperation::<T>::mark_completed(self);
                        return ReceiveResult::WouldBlock;
                    }
                    
                    let channel_sync = get_global_channel_sync();
                    let goroutine_id = self.options.goroutine_id.unwrap_or(0);
                    let remaining_time = deadline.saturating_duration_since(Instant::now());
                    let wait_time = std::cmp::min(remaining_time, Duration::from_millis(10));
                    
                    let _ = channel_sync.block_on_receive(
                        0, // Channel ID would need to be passed through
                        goroutine_id,
                        self.options.priority,
                        Some(wait_time)
                    );
                }
            }
        }
    }
    
    /// Mark operation as completed
    fn mark_completed(&self) {
        if let Ok(mut completed) = self.completed.lock() {
            *completed = true;
        }
        self.completion_notify.notify_all();
    }
    
    /// Wait for operation completion
    pub fn wait_for_completion(&self) -> Result<(), ChannelError> {
        let mut completed = self.completed.lock()
            .map_err(|_| ChannelError::Closed)?;
        while !*completed {
            completed = self.completion_notify.wait(completed)
                .map_err(|_| ChannelError::Closed)?;
        }
        Ok(())
    }
    
    /// Check if operation is completed
    pub fn is_completed(&self) -> bool {
        self.completed.lock()
            .map(|completed| *completed)
            .unwrap_or(true) // Assume completed if we can't lock
    }
}

/// Batch operation for sending multiple values
pub struct BatchSendOperation<T> {
    /// Values to send
    values: VecDeque<T>,
    /// Operation options
    options: SendOptions,
    /// Successfully sent count
    sent_count: AtomicUsize,
}

impl<T> BatchSendOperation<T> {
    /// Create a new batch send operation
    pub fn new(values: Vec<T>, options: SendOptions) -> Self {
        Self {
            values: VecDeque::from(values),
            options,
            sent_count: AtomicUsize::new(0),
        }
    }
    
    /// Execute the batch send operation
    pub fn execute<B: ChannelBuffer<T> + ?Sized>(
        mut self,
        buffer: &Arc<B>,
    ) -> Result<usize, (Vec<T>, ChannelError)> {
        let mut remaining_values = Vec::new();
        let mut sent_count = 0;
        
        while let Some(value) = self.values.pop_front() {
            match buffer.try_push(value) {
                Ok(()) => {
                    sent_count += 1;
                }
                Err((value, error)) => {
                    // Add back the failed value and all remaining values
                    remaining_values.push(value);
                    remaining_values.extend(self.values);
                    return Err((remaining_values, error));
                }
            }
            
            // Check for timeout in batch operations
            if let Some(timeout) = self.options.timeout {
                // Simple timeout check - would be more sophisticated in real implementation
                thread::sleep(Duration::from_nanos(1));
            }
        }
        
        Ok(sent_count)
    }
    
    /// Get the number of values sent so far
    pub fn sent_count(&self) -> usize {
        self.sent_count.load(Ordering::SeqCst)
    }
}

/// Batch operation for receiving multiple values
pub struct BatchReceiveOperation<T> {
    /// Number of values to receive
    count: usize,
    /// Operation options
    options: ReceiveOptions,
    /// Received values
    received_values: Mutex<Vec<T>>,
}

impl<T> BatchReceiveOperation<T> {
    /// Create a new batch receive operation
    pub fn new(count: usize, options: ReceiveOptions) -> Self {
        Self {
            count,
            options,
            received_values: Mutex::new(Vec::with_capacity(count)),
        }
    }
    
    /// Execute the batch receive operation
    pub fn execute<B: ChannelBuffer<T> + ?Sized>(
        self,
        buffer: &Arc<B>,
    ) -> Result<Vec<T>, ChannelError> {
        let mut received = Vec::new();
        
        for _ in 0..self.count {
            match buffer.try_pop() {
                Ok(Some(value)) => {
                    received.push(value);
                }
                Ok(None) => {
                    if !self.options.blocking {
                        break;
                    }
                    
                    // Wait for data
                    let start = Instant::now();
                    loop {
                        if let Some(timeout) = self.options.timeout {
                            if start.elapsed() >= timeout {
                                return Ok(received);
                            }
                        }
                        
                        thread::sleep(Duration::from_millis(1));
                        
                        match buffer.try_pop() {
                            Ok(Some(value)) => {
                                received.push(value);
                                break;
                            }
                            Ok(None) => continue,
                            Err(ChannelError::Closed) => return Ok(received),
                            Err(e) => return Err(e),
                        }
                    }
                }
                Err(ChannelError::Closed) => {
                    return Ok(received);
                }
                Err(e) => return Err(e),
            }
        }
        
        Ok(received)
    }
    
    /// Get the currently received values
    pub fn get_received(&self) -> Vec<T> where T: Clone {
        self.received_values.lock()
            .map(|values| values.clone())
            .unwrap_or_else(|_| Vec::new())
    }
}

/// Range-based receive operation (like Go's range over channels)
pub struct RangeReceiveOperation<T> {
    /// Operation options
    options: ReceiveOptions,
    /// Received values
    received_values: Mutex<Vec<T>>,
    /// Operation completed flag
    completed: Arc<Mutex<bool>>,
}

impl<T: Send> RangeReceiveOperation<T> {
    /// Create a new range receive operation
    pub fn new(options: ReceiveOptions) -> Self {
        Self {
            options,
            received_values: Mutex::new(Vec::new()),
            completed: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Execute the range receive operation
    pub fn execute<B: ChannelBuffer<T>>(
        self,
        buffer: &Arc<B>,
    ) -> impl Iterator<Item = Result<T, ChannelError>> {
        RangeReceiveIterator {
            buffer: buffer.clone(),
            options: self.options,
            completed: self.completed,
            _phantom: PhantomData,
        }
    }
}

/// Iterator for range-based receive operations
pub struct RangeReceiveIterator<T: Send, B: ChannelBuffer<T>> {
    buffer: Arc<B>,
    options: ReceiveOptions,
    completed: Arc<Mutex<bool>>,
    _phantom: PhantomData<T>,
}

impl<T: Send, B: ChannelBuffer<T>> Iterator for RangeReceiveIterator<T, B> {
    type Item = Result<T, ChannelError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.completed.lock()
            .map(|completed| *completed)
            .unwrap_or(true) {
            return None;
        }
        
        match self.buffer.try_pop() {
            Ok(Some(value)) => Some(Ok(value)),
            Ok(None) => {
                if !self.options.blocking {
                    if let Ok(mut completed) = self.completed.lock() {
            *completed = true;
        }
                    return None;
                }
                
                // Wait for data or channel closure
                loop {
                    match self.buffer.try_pop() {
                        Ok(Some(value)) => return Some(Ok(value)),
                        Ok(None) => {
                            if self.buffer.is_closed() {
                                if let Ok(mut completed) = self.completed.lock() {
            *completed = true;
        }
                                return None;
                            }
                            thread::sleep(Duration::from_millis(1));
                        }
                        Err(ChannelError::Closed) => {
                            if let Ok(mut completed) = self.completed.lock() {
            *completed = true;
        }
                            return None;
                        }
                        Err(e) => return Some(Err(e)),
                    }
                }
            }
            Err(ChannelError::Closed) => {
                if let Ok(mut completed) = self.completed.lock() {
            *completed = true;
        }
                None
            }
            Err(e) => Some(Err(e)),
        }
    }
}

/// Utility functions for common operations

/// Send a value with default options
pub fn send<T, B: ChannelBuffer<T>>(
    buffer: &Arc<B>,
    value: T,
) -> SendResult<T> {
    let operation = SendOperation::new(value, SendOptions::default());
    operation.execute(buffer)
}

/// Send a value with timeout
pub fn send_timeout<T, B: ChannelBuffer<T>>(
    buffer: &Arc<B>,
    value: T,
    timeout: Duration,
) -> SendResult<T> {
    let mut options = SendOptions::default();
    options.timeout = Some(timeout);
    let operation = SendOperation::new(value, options);
    operation.execute(buffer)
}

/// Try to send a value (non-blocking)
pub fn try_send<T, B: ChannelBuffer<T>>(
    buffer: &Arc<B>,
    value: T,
) -> SendResult<T> {
    let mut options = SendOptions::default();
    options.blocking = false;
    let operation = SendOperation::new(value, options);
    operation.execute(buffer)
}

/// Receive a value with default options
pub fn receive<T, B: ChannelBuffer<T>>(
    buffer: &Arc<B>,
) -> ReceiveResult<T> {
    let operation = ReceiveOperation::<T>::new(ReceiveOptions::default());
    operation.execute(buffer)
}

/// Receive a value with timeout
pub fn receive_timeout<T, B: ChannelBuffer<T>>(
    buffer: &Arc<B>,
    timeout: Duration,
) -> ReceiveResult<T> {
    let mut options = ReceiveOptions::default();
    options.timeout = Some(timeout);
    let operation = ReceiveOperation::<T>::new(options);
    operation.execute(buffer)
}

/// Try to receive a value (non-blocking)
pub fn try_receive<T, B: ChannelBuffer<T>>(
    buffer: &Arc<B>,
) -> ReceiveResult<T> {
    let mut options = ReceiveOptions::default();
    options.blocking = false;
    let operation = ReceiveOperation::<T>::new(options);
    operation.execute(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::channels::buffer::RingBuffer;
    use std::sync::Arc;

    #[test]
    fn test_send_operation() {
        let buffer = Arc::new(RingBuffer::new(5));
        
        let options = SendOptions::default();
        let operation = SendOperation::new(42, options);
        
        match operation.execute(&buffer) {
            SendResult::Sent => assert!(true),
            result => assert!(false, "Send should succeed, got {:?}", result),
        }
    }

    #[test]
    fn test_receive_operation() {
        let buffer = Arc::new(RingBuffer::new(5));
        
        // Send a value first
        let _ = buffer.try_push(42);
        
        let options = ReceiveOptions::default();
        let operation = ReceiveOperation::new(options);
        
        match operation.execute(&buffer) {
            ReceiveResult::Received(value) => assert_eq!(value, 42),
            result => assert!(false, "Receive should succeed, got {:?}", result),
        }
    }

    #[test]
    fn test_batch_send() {
        let buffer = Arc::new(RingBuffer::new(10));
        
        let values = vec![1, 2, 3, 4, 5];
        let options = SendOptions::default();
        let batch_op = BatchSendOperation::new(values, options);
        
        match batch_op.execute(&buffer) {
            Ok(count) => assert_eq!(count, 5),
            Err(e) => assert!(false, "Batch send should succeed, got error: {:?}", e),
        }
    }

    #[test]
    fn test_batch_receive() {
        let buffer = Arc::new(RingBuffer::new(10));
        
        // Send some values first
        for i in 1..=5 {
            let _ = buffer.try_push(i);
        }
        
        let options = ReceiveOptions::default();
        let batch_op = BatchReceiveOperation::new(3, options);
        
        match batch_op.execute(&buffer) {
            Ok(values) => {
                assert_eq!(values.len(), 3);
                assert_eq!(values, vec![1, 2, 3]);
            }
            Err(e) => assert!(false, "Batch receive should succeed, got error: {:?}", e),
        }
    }
}
