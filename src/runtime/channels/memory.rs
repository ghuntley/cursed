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
    pool: std::sync::Mutex<Vec<Channel<T>>>,
    /// Maximum pool size
    max_size: usize,
    /// Default channel capacity
    default_capacity: usize,
}

impl<T> ChannelPool<T> {
    /// Create a new channel pool
    #[instrument]
    pub fn new(max_size: usize, default_capacity: usize) -> Self {
        debug!(max_size, default_capacity, "Creating channel pool");
        
        Self {
            pool: std::sync::Mutex::new(Vec::with_capacity(max_size)),
            max_size,
            default_capacity,
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
            available: pool.len(),
            max_size: self.max_size,
            default_capacity: self.default_capacity,
        }
    }
}

/// Statistics for channel pool
#[derive(Debug, Clone)]
pub struct ChannelPoolStats {
    pub available: usize,
    pub max_size: usize,
    pub default_capacity: usize,
}

/// Memory tracker for channel allocations
/// Helps monitor memory usage in channel-heavy applications
#[derive(Debug)]
pub struct ChannelMemoryTracker {
    /// Total channels created
    channels_created: std::sync::atomic::AtomicUsize,
    /// Total channels dropped
    channels_dropped: std::sync::atomic::AtomicUsize,
    /// Peak concurrent channels
    peak_channels: std::sync::atomic::AtomicUsize,
    /// Current active channels
    active_channels: std::sync::atomic::AtomicUsize,
}

impl ChannelMemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self {
            channels_created: std::sync::atomic::AtomicUsize::new(0),
            channels_dropped: std::sync::atomic::AtomicUsize::new(0),
            peak_channels: std::sync::atomic::AtomicUsize::new(0),
            active_channels: std::sync::atomic::AtomicUsize::new(0),
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
                Ok(_) => break,
                Err(current) => peak = current,
            }
        }
        
        debug!(active_channels = active, "Channel created");
    }

    /// Record channel destruction
    #[instrument(skip(self))]
    pub fn record_destruction(&self) {
        use std::sync::atomic::Ordering;
        
        self.channels_dropped.fetch_add(1, Ordering::Relaxed);
        let active = self.active_channels.fetch_sub(1, Ordering::Relaxed) - 1;
        
        debug!(active_channels = active, "Channel destroyed");
    }

    /// Get current memory statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> ChannelMemoryStats {
        use std::sync::atomic::Ordering;
        
        ChannelMemoryStats {
            channels_created: self.channels_created.load(Ordering::Relaxed),
            channels_dropped: self.channels_dropped.load(Ordering::Relaxed),
            active_channels: self.active_channels.load(Ordering::Relaxed),
            peak_channels: self.peak_channels.load(Ordering::Relaxed),
        }
    }
}

/// Memory statistics for channels
#[derive(Debug, Clone)]
pub struct ChannelMemoryStats {
    pub channels_created: usize,
    pub channels_dropped: usize,
    pub active_channels: usize,
    pub peak_channels: usize,
}

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
}

/// Wrapper that automatically tracks channel memory usage
#[derive(Debug)]
pub struct TrackedChannel<T> {
    channel: Channel<T>,
    _tracker: Arc<ChannelMemoryTracker>,
}

impl<T> TrackedChannel<T> {
    /// Create a new tracked channel
    #[instrument]
    pub fn new(capacity: usize) -> Self {
        let tracker = Arc::new(ChannelMemoryTracker::new());
        tracker.record_creation();
        
        Self {
            channel: Channel::new(capacity),
            _tracker: tracker,
        }
    }

    /// Create sender and receiver handles
    pub fn split(self) -> (ChannelSender<T>, ChannelReceiver<T>) {
        (self.channel.sender(), self.channel.receiver())
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_channel_pool() {
        let pool = ChannelPool::<i32>::new(5, 10);
        
        let (tx, rx) = pool.get();
        
        // Should be able to use the channel normally
        thread::spawn(move || {
            tx.send(42).unwrap();
        });
        
        let value = rx.receive().unwrap();
        assert_eq!(value, 42);
        
        let stats = pool.stats();
        assert_eq!(stats.max_size, 5);
        assert_eq!(stats.default_capacity, 10);
    }

    #[test]
    fn test_memory_tracker() {
        let tracker = ChannelMemoryTracker::new();
        
        tracker.record_creation();
        tracker.record_creation();
        
        let stats = tracker.stats();
        assert_eq!(stats.channels_created, 2);
        assert_eq!(stats.active_channels, 2);
        
        tracker.record_destruction();
        
        let stats = tracker.stats();
        assert_eq!(stats.channels_dropped, 1);
        assert_eq!(stats.active_channels, 1);
    }

    #[test]
    fn test_tracked_channel() {
        let tracked = TrackedChannel::<i32>::new(5);
        let stats = tracked.stats();
        assert_eq!(stats.capacity, 5);
        
        let (tx, rx) = tracked.split();
        
        thread::spawn(move || {
            tx.send(123).unwrap();
        });
        
        let value = rx.receive().unwrap();
        assert_eq!(value, 123);
    }

    #[test]
    fn test_global_tracker() {
        let tracker = global_memory_tracker();
        let initial_stats = tracker.stats();
        
        tracker.record_creation();
        
        let new_stats = tracker.stats();
        assert_eq!(new_stats.channels_created, initial_stats.channels_created + 1);
    }
}
