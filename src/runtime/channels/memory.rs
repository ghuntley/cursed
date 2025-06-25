/// Memory management for CURSED channels
/// Handles efficient allocation and cleanup of channel resources

use std::sync::Arc;
use tracing::{debug, instrument, warn};

use super::channel::{Channel, ChannelSender, ChannelReceiver};

/// Channel memory pool for efficient allocation
/// Reduces allocation overhead for frequently created channels
#[derive(Debug)]
pub struct ChannelPool<T> {
    /// Pool of reusable channels
    /// Maximum pool size
    /// Default channel capacity
impl<T> ChannelPool<T> {
    /// Create a new channel pool
    #[instrument]
    pub fn new(max_size: usize, default_capacity: usize) -> Self {
        debug!(max_size, default_capacity, "Creating channel pool");
        
        Self {
        }
    }

    /// Get a channel from the pool or create a new one
    #[instrument(skip(self))]
    pub fn get(&self) -> (ChannelSender<T>, ChannelReceiver<T>) {
        let mut pool = self.pool.lock().unwrap();
        
        if let Some(channel) = pool.pop() {
            debug!("Reusing channel from pool");
            (channel.sender(), channel.receiver())
        } else {
            debug!(capacity = self.default_capacity, "Creating new channel");
            let channel = Channel::new(self.default_capacity);
            (channel.sender(), channel.receiver())
        }
    }

    /// Return a channel to the pool for reuse
    /// Note: This is conceptual - in practice, channels can't easily be reset
    #[instrument(skip(self, _channel))]
    pub fn return_channel(&self, _channel: Channel<T>) {
        let mut pool = self.pool.lock().unwrap();
        
        if pool.len() < self.max_size {
            // In a real implementation, we'd need to reset the channel state
            // For now, we just note that this is where pooling would happen
            debug!("Channel returned to pool (conceptual)");
        } else {
            debug!("Pool full, discarding channel");
        }
    }

    /// Get current pool statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> ChannelPoolStats {
        let pool = self.pool.lock().unwrap();
        ChannelPoolStats {
        }
    }
/// Statistics for channel pool
#[derive(Debug, Clone)]
pub struct ChannelPoolStats {
/// Memory tracker for channel allocations
/// Helps monitor memory usage in channel-heavy applications
#[derive(Debug)]
pub struct ChannelMemoryTracker {
    /// Total channels created
    /// Total channels dropped
    /// Peak concurrent channels
    /// Current active channels
impl ChannelMemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self {
        }
    }

    /// Record channel creation
    #[instrument(skip(self))]
    pub fn record_creation(&self) {
        use std::sync::atomic::Ordering;
        
        self.channels_created.fetch_add(1, Ordering::Relaxed);
        let active = self.active_channels.fetch_add(1, Ordering::Relaxed) + 1;
        
        // Update peak if necessary
        let mut peak = self.peak_channels.load(Ordering::Relaxed);
        while active > peak {
            match self.peak_channels.compare_exchange_weak(
                peak, active, Ordering::Relaxed, Ordering::Relaxed
            ) {
            }
        }
        
        debug!(active_channels = active, "Channel created");
    /// Record channel destruction
    #[instrument(skip(self))]
    pub fn record_destruction(&self) {
        use std::sync::atomic::Ordering;
        
        self.channels_dropped.fetch_add(1, Ordering::Relaxed);
        let active = self.active_channels.fetch_sub(1, Ordering::Relaxed) - 1;
        
        debug!(active_channels = active, "Channel destroyed");
    /// Get current memory statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> ChannelMemoryStats {
        use std::sync::atomic::Ordering;
        
        ChannelMemoryStats {
        }
    }
/// Memory statistics for channels
#[derive(Debug, Clone)]
pub struct ChannelMemoryStats {
impl Default for ChannelMemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Global channel memory tracker instance
static GLOBAL_TRACKER: std::sync::OnceLock<ChannelMemoryTracker> = std::sync::OnceLock::new();

/// Get the global channel memory tracker
pub fn global_memory_tracker() -> &'static ChannelMemoryTracker {
    GLOBAL_TRACKER.get_or_init(ChannelMemoryTracker::new)
/// Wrapper that automatically tracks channel memory usage
#[derive(Debug)]
pub struct TrackedChannel<T> {
impl<T> TrackedChannel<T> {
    /// Create a new tracked channel
    #[instrument]
    pub fn new(capacity: usize) -> Self {
        let tracker = Arc::new(ChannelMemoryTracker::new());
        tracker.record_creation();
        
        Self {
        }
    }

    /// Create sender and receiver handles
    pub fn split(self) -> (ChannelSender<T>, ChannelReceiver<T>) {
        (self.channel.sender(), self.channel.receiver())
    /// Get channel statistics
    pub fn stats(&self) -> super::channel::ChannelStats {
        self.channel.stats()
    }
}

impl<T> Drop for TrackedChannel<T> {
    #[instrument(skip(self))]
    fn drop(&mut self) {
        self._tracker.record_destruction();
    }
}

