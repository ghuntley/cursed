use crate::error::CursedError;
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
/// Waiting sender information
#[derive(Debug)]
pub struct WaitingSender<T> {
/// Waiting receiver information
#[derive(Debug)]
pub struct WaitingReceiver<T> {
/// Internal channel data protected by mutex
#[derive(Debug)]
pub struct ChannelData<T> {
    /// Buffered messages
    /// Maximum buffer capacity (0 for unbuffered)
    /// Current channel state
    /// Senders waiting to send (for unbuffered channels)
    /// Receivers waiting to receive
    /// Number of active sender handles
    /// Number of active receiver handles
/// Core channel implementation
/// Supports both buffered (capacity > 0) and unbuffered (capacity = 0) channels
#[derive(Debug)]
pub struct Channel<T> {
    /// Protected channel data
    /// Condition variable for blocking operations
    /// Total operations counter for debugging
impl<T> Channel<T> {
    /// Create a new channel with specified capacity
    /// - capacity = 0: unbuffered channel (synchronous)
    /// - capacity > 0: buffered channel (asynchronous up to capacity)
    #[instrument]
    pub fn new(capacity: usize) -> Self {
        debug!(capacity, "Creating new channel");
        
        let channel = Self {
            data: Arc::new(Mutex::new(ChannelData {
        
        info!(capacity, "Channel created successfully");
        channel
    /// Create a sender handle for this channel
    #[instrument(skip(self))]
    pub fn sender(&self) -> ChannelSender<T> {
        let mut data = self.data.lock().unwrap();
        data.sender_count += 1;
        
        debug!(sender_count = data.sender_count, "Created sender handle");
        
        ChannelSender {
        }
    }

    /// Create a receiver handle for this channel
    #[instrument(skip(self))]
    pub fn receiver(&self) -> ChannelReceiver<T> {
        let mut data = self.data.lock().unwrap();
        data.receiver_count += 1;
        
        debug!(receiver_count = data.receiver_count, "Created receiver handle");
        
        ChannelReceiver {
        }
    }

    /// Get channel statistics for debugging and monitoring
    #[instrument(skip(self))]
    pub fn stats(&self) -> ChannelStats {
        let data = self.data.lock().unwrap();
        ChannelStats {
        }
    }

    /// Check if channel is closed
    pub fn is_closed(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.state != ChannelState::Open
    /// Check if channel buffer is empty
    pub fn is_empty(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.buffer.is_empty() && data.waiting_senders.is_empty()
    /// Check if channel buffer is full
    pub fn is_full(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.capacity > 0 && data.buffer.len() >= data.capacity
    /// Get current buffer length
    pub fn len(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.buffer.len()
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
/// Channel receiver handle  
/// Automatically decrements receiver count when dropped
#[derive(Debug)]
pub struct ChannelReceiver<T> {
/// Channel statistics for monitoring and debugging
#[derive(Debug, Clone)]
pub struct ChannelStats {
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
impl<T> Clone for ChannelSender<T> {
    #[instrument(skip(self))]
    fn clone(&self) -> Self {
        if let Some(data_arc) = self.channel.upgrade() {
            let mut data = data_arc.lock().unwrap();
            data.sender_count += 1;
            
            debug!(sender_count = data.sender_count, "Cloned sender handle");
        Self {
        }
    }
impl<T> Clone for ChannelReceiver<T> {
    #[instrument(skip(self))]
    fn clone(&self) -> Self {
        if let Some(data_arc) = self.channel.upgrade() {
            let mut data = data_arc.lock().unwrap();
            data.receiver_count += 1;
            
            debug!(receiver_count = data.receiver_count, "Cloned receiver handle");
        Self {
        }
    }
impl<T> ChannelSender<T> {
    /// Get a reference to the channel data (weak reference)
    pub fn channel(&self) -> &Weak<Mutex<ChannelData<T>>> {
        &self.channel
    /// Get a reference to the condition variable
    pub fn condvar(&self) -> &Arc<Condvar> {
        &self.condvar
    /// Get a reference to the operation count
    pub fn operation_count(&self) -> &AtomicUsize {
        unsafe { &*self.operation_count }
    }
impl<T> ChannelReceiver<T> {
    /// Get a reference to the channel data (weak reference)
    pub fn channel(&self) -> &Weak<Mutex<ChannelData<T>>> {
        &self.channel
    /// Get a reference to the condition variable
    pub fn condvar(&self) -> &Arc<Condvar> {
        &self.condvar
    /// Get a reference to the operation count
    pub fn operation_count(&self) -> &AtomicUsize {
        unsafe { &*self.operation_count }
    }
/// Create a new unbuffered channel (synchronous)
pub fn channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    let ch = Channel::new(0);
    (ch.sender(), ch.receiver())
/// Create a new buffered channel with specified capacity
pub fn buffered_channel<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    let ch = Channel::new(capacity);
    (ch.sender(), ch.receiver())
