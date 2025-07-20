//! Robust timeout manager for channel operations
//!
//! This module provides a centralized, thread-safe timeout management system
//! that eliminates race conditions and memory leaks in channel timeout handling.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};
use std::sync::mpsc;

use crate::runtime::channels::{ChannelError, ChannelResult};

/// Timeout request identifier
pub type TimeoutId = u64;

/// Timeout request
#[derive(Clone)]
pub struct TimeoutRequest {
    /// Unique identifier
    pub id: TimeoutId,
    /// Deadline for the timeout
    pub deadline: Instant,
    /// Whether this timeout has been triggered
    pub triggered: Arc<AtomicBool>,
    /// Whether this timeout has been cancelled
    pub cancelled: Arc<AtomicBool>,
    /// Optional callback when timeout occurs
    pub callback: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl std::fmt::Debug for TimeoutRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeoutRequest")
            .field("id", &self.id)
            .field("deadline", &self.deadline)
            .field("triggered", &self.triggered)
            .field("cancelled", &self.cancelled)
            .field("callback", &self.callback.as_ref().map(|_| "<callback>"))
            .finish()
    }
}

/// Internal timeout manager command
#[derive(Debug)]
enum TimeoutCommand {
    /// Add a new timeout
    AddTimeout(TimeoutRequest),
    /// Cancel an existing timeout
    CancelTimeout(TimeoutId),
    /// Shutdown the manager
    Shutdown,
    /// Force cleanup of expired timeouts
    ForceCleanup,
}

/// Thread-safe timeout manager
pub struct TimeoutManager {
    /// Command sender
    command_sender: Option<mpsc::Sender<TimeoutCommand>>,
    /// Worker thread handle
    worker_thread: Option<JoinHandle<()>>,
    /// Next timeout ID
    next_id: AtomicU64,
    /// Whether the manager is running
    running: Arc<AtomicBool>,
    /// Active timeouts for cleanup tracking
    active_timeouts: Arc<Mutex<HashMap<TimeoutId, Arc<AtomicBool>>>>,
}

impl TimeoutManager {
    /// Create a new timeout manager
    pub fn new() -> Self {
        Self {
            command_sender: None,
            worker_thread: None,
            next_id: AtomicU64::new(1),
            running: Arc::new(AtomicBool::new(false)),
            active_timeouts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Start the timeout manager
    pub fn start(&mut self) -> ChannelResult<()> {
        if self.running.load(Ordering::Acquire) {
            return Ok(()); // Already running
        }

        let (sender, receiver) = mpsc::channel::<TimeoutCommand>();
        let running = Arc::clone(&self.running);
        let active_timeouts = Arc::clone(&self.active_timeouts);

        // Start worker thread
        let worker_thread = thread::spawn(move || {
            Self::worker_loop(receiver, running, active_timeouts);
        });

        self.command_sender = Some(sender);
        self.worker_thread = Some(worker_thread);
        self.running.store(true, Ordering::Release);

        Ok(())
    }

    /// Stop the timeout manager
    pub fn stop(&mut self) -> ChannelResult<()> {
        if !self.running.load(Ordering::Acquire) {
            return Ok(()); // Already stopped
        }

        // Send shutdown command
        if let Some(sender) = &self.command_sender {
            let _ = sender.send(TimeoutCommand::Shutdown);
        }

        // Wait for worker thread to finish
        if let Some(handle) = self.worker_thread.take() {
            let _ = handle.join();
        }

        self.command_sender = None;
        self.running.store(false, Ordering::Release);

        // Clean up any remaining timeouts
        self.cleanup_all_timeouts()?;

        Ok(())
    }

    /// Register a timeout
    pub fn register_timeout(&self, duration: Duration) -> ChannelResult<TimeoutHandle> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let deadline = Instant::now() + duration;
        let triggered = Arc::new(AtomicBool::new(false));
        let cancelled = Arc::new(AtomicBool::new(false));

        let request = TimeoutRequest {
            id,
            deadline,
            triggered: Arc::clone(&triggered),
            cancelled: Arc::clone(&cancelled),
            callback: None,
        };

        // Add to active timeouts tracking
        if let Ok(mut timeouts) = self.active_timeouts.lock() {
            timeouts.insert(id, Arc::clone(&cancelled));
        }

        // Send to worker thread
        if let Some(sender) = &self.command_sender {
            sender.send(TimeoutCommand::AddTimeout(request))
                .map_err(|_| ChannelError::AllocationError("Failed to register timeout".to_string()))?;
        } else {
            return Err(ChannelError::AllocationError("Timeout manager not running".to_string()));
        }

        Ok(TimeoutHandle {
            id,
            triggered,
            cancelled,
        })
    }

    /// Register a timeout with callback
    pub fn register_timeout_with_callback<F>(
        &self,
        duration: Duration,
        callback: F,
    ) -> ChannelResult<TimeoutHandle>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let deadline = Instant::now() + duration;
        let triggered = Arc::new(AtomicBool::new(false));
        let cancelled = Arc::new(AtomicBool::new(false));

        let request = TimeoutRequest {
            id,
            deadline,
            triggered: Arc::clone(&triggered),
            cancelled: Arc::clone(&cancelled),
            callback: Some(Arc::new(callback)),
        };

        // Add to active timeouts tracking
        if let Ok(mut timeouts) = self.active_timeouts.lock() {
            timeouts.insert(id, Arc::clone(&cancelled));
        }

        // Send to worker thread
        if let Some(sender) = &self.command_sender {
            sender.send(TimeoutCommand::AddTimeout(request))
                .map_err(|_| ChannelError::AllocationError("Failed to register timeout".to_string()))?;
        } else {
            return Err(ChannelError::AllocationError("Timeout manager not running".to_string()));
        }

        Ok(TimeoutHandle {
            id,
            triggered,
            cancelled,
        })
    }

    /// Cancel a timeout
    pub fn cancel_timeout(&self, id: TimeoutId) -> ChannelResult<()> {
        // Mark as cancelled in tracking
        if let Ok(mut timeouts) = self.active_timeouts.lock() {
            if let Some(cancelled) = timeouts.get(&id) {
                cancelled.store(true, Ordering::Release);
            }
        }

        // Send cancel command to worker
        if let Some(sender) = &self.command_sender {
            sender.send(TimeoutCommand::CancelTimeout(id))
                .map_err(|_| ChannelError::AllocationError("Failed to cancel timeout".to_string()))?;
        }

        Ok(())
    }

    /// Force cleanup of expired timeouts
    pub fn force_cleanup(&self) -> ChannelResult<()> {
        if let Some(sender) = &self.command_sender {
            sender.send(TimeoutCommand::ForceCleanup)
                .map_err(|_| ChannelError::AllocationError("Failed to force cleanup".to_string()))?;
        }
        Ok(())
    }

    /// Clean up all timeouts (called during shutdown)
    fn cleanup_all_timeouts(&self) -> ChannelResult<()> {
        if let Ok(mut timeouts) = self.active_timeouts.lock() {
            for (_, cancelled) in timeouts.iter() {
                cancelled.store(true, Ordering::Release);
            }
            timeouts.clear();
        }
        Ok(())
    }

    /// Worker thread main loop
    fn worker_loop(
        receiver: mpsc::Receiver<TimeoutCommand>,
        running: Arc<AtomicBool>,
        active_timeouts: Arc<Mutex<HashMap<TimeoutId, Arc<AtomicBool>>>>,
    ) {
        let mut pending_timeouts: HashMap<TimeoutId, TimeoutRequest> = HashMap::new();
        let mut last_cleanup = Instant::now();

        while running.load(Ordering::Acquire) {
            // Process commands with timeout to allow periodic cleanup
            match receiver.recv_timeout(Duration::from_millis(10)) {
                Ok(TimeoutCommand::AddTimeout(request)) => {
                    pending_timeouts.insert(request.id, request);
                }
                Ok(TimeoutCommand::CancelTimeout(id)) => {
                    if let Some(request) = pending_timeouts.remove(&id) {
                        request.cancelled.store(true, Ordering::Release);
                    }
                    // Also remove from active tracking
                    if let Ok(mut timeouts) = active_timeouts.lock() {
                        timeouts.remove(&id);
                    }
                }
                Ok(TimeoutCommand::Shutdown) => {
                    running.store(false, Ordering::Release);
                    break;
                }
                Ok(TimeoutCommand::ForceCleanup) => {
                    Self::process_timeouts(&mut pending_timeouts, &active_timeouts);
                    last_cleanup = Instant::now();
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Normal timeout, continue processing
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    running.store(false, Ordering::Release);
                    break;
                }
            }

            // Process pending timeouts
            Self::process_timeouts(&mut pending_timeouts, &active_timeouts);

            // Periodic cleanup every 5 seconds
            if last_cleanup.elapsed() > Duration::from_secs(5) {
                Self::cleanup_expired_tracking(&active_timeouts);
                last_cleanup = Instant::now();
            }
        }

        // Final cleanup
        for (_, request) in pending_timeouts.iter() {
            request.cancelled.store(true, Ordering::Release);
        }
    }

    /// Process pending timeouts
    fn process_timeouts(
        pending_timeouts: &mut HashMap<TimeoutId, TimeoutRequest>,
        active_timeouts: &Arc<Mutex<HashMap<TimeoutId, Arc<AtomicBool>>>>,
    ) {
        let now = Instant::now();
        let mut to_remove = Vec::new();

        for (id, request) in pending_timeouts.iter() {
            // Check if cancelled
            if request.cancelled.load(Ordering::Acquire) {
                to_remove.push(*id);
                continue;
            }

            // Check if expired
            if now >= request.deadline {
                // Mark as triggered
                request.triggered.store(true, Ordering::Release);

                // Execute callback if present
                if let Some(callback) = &request.callback {
                    callback();
                }

                to_remove.push(*id);
            }
        }

        // Remove processed timeouts
        for id in to_remove {
            pending_timeouts.remove(&id);
            
            // Remove from active tracking
            if let Ok(mut timeouts) = active_timeouts.lock() {
                timeouts.remove(&id);
            }
        }
    }

    /// Clean up expired tracking entries
    fn cleanup_expired_tracking(active_timeouts: &Arc<Mutex<HashMap<TimeoutId, Arc<AtomicBool>>>>) {
        if let Ok(mut timeouts) = active_timeouts.lock() {
            timeouts.retain(|_, cancelled| !cancelled.load(Ordering::Acquire));
        }
    }

    /// Check if timeout manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    /// Get active timeout count (for monitoring)
    pub fn active_timeout_count(&self) -> usize {
        if let Ok(timeouts) = self.active_timeouts.lock() {
            timeouts.len()
        } else {
            0
        }
    }
}

impl Drop for TimeoutManager {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Handle for a registered timeout
pub struct TimeoutHandle {
    /// Timeout ID
    pub id: TimeoutId,
    /// Whether the timeout has been triggered
    triggered: Arc<AtomicBool>,
    /// Whether the timeout has been cancelled
    cancelled: Arc<AtomicBool>,
}

impl TimeoutHandle {
    /// Check if the timeout has been triggered
    pub fn is_triggered(&self) -> bool {
        self.triggered.load(Ordering::Acquire)
    }

    /// Check if the timeout has been cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Cancel this timeout
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Wait for the timeout to trigger or be cancelled
    pub fn wait(&self) -> TimeoutResult {
        while !self.is_triggered() && !self.is_cancelled() {
            thread::sleep(Duration::from_millis(1));
        }

        if self.is_cancelled() {
            TimeoutResult::Cancelled
        } else {
            TimeoutResult::Triggered
        }
    }

    /// Wait for the timeout with a maximum wait duration
    pub fn wait_timeout(&self, max_wait: Duration) -> TimeoutResult {
        let start = Instant::now();
        
        while !self.is_triggered() && !self.is_cancelled() && start.elapsed() < max_wait {
            thread::sleep(Duration::from_millis(1));
        }

        if self.is_cancelled() {
            TimeoutResult::Cancelled
        } else if self.is_triggered() {
            TimeoutResult::Triggered
        } else {
            TimeoutResult::WaitTimeout
        }
    }
}

/// Result of waiting for a timeout
#[derive(Debug, Clone, PartialEq)]
pub enum TimeoutResult {
    /// Timeout was triggered
    Triggered,
    /// Timeout was cancelled
    Cancelled,
    /// Wait operation itself timed out
    WaitTimeout,
}

/// Global timeout manager instance
static GLOBAL_TIMEOUT_MANAGER: std::sync::LazyLock<Mutex<TimeoutManager>> = 
    std::sync::LazyLock::new(|| Mutex::new(TimeoutManager::new()));

/// Initialize the global timeout manager
pub fn init_timeout_manager() -> ChannelResult<()> {
    if let Ok(mut manager) = GLOBAL_TIMEOUT_MANAGER.lock() {
        manager.start()
    } else {
        Err(ChannelError::AllocationError("Failed to lock timeout manager".to_string()))
    }
}

/// Shutdown the global timeout manager
pub fn shutdown_timeout_manager() -> ChannelResult<()> {
    if let Ok(mut manager) = GLOBAL_TIMEOUT_MANAGER.lock() {
        manager.stop()
    } else {
        Err(ChannelError::AllocationError("Failed to lock timeout manager".to_string()))
    }
}

/// Register a timeout using the global manager
pub fn register_timeout(duration: Duration) -> ChannelResult<TimeoutHandle> {
    if let Ok(manager) = GLOBAL_TIMEOUT_MANAGER.lock() {
        manager.register_timeout(duration)
    } else {
        Err(ChannelError::AllocationError("Failed to lock timeout manager".to_string()))
    }
}

/// Register a timeout with callback using the global manager
pub fn register_timeout_with_callback<F>(
    duration: Duration,
    callback: F,
) -> ChannelResult<TimeoutHandle>
where
    F: Fn() + Send + Sync + 'static,
{
    if let Ok(manager) = GLOBAL_TIMEOUT_MANAGER.lock() {
        manager.register_timeout_with_callback(duration, callback)
    } else {
        Err(ChannelError::AllocationError("Failed to lock timeout manager".to_string()))
    }
}

/// Cancel a timeout using the global manager
pub fn cancel_timeout(id: TimeoutId) -> ChannelResult<()> {
    if let Ok(manager) = GLOBAL_TIMEOUT_MANAGER.lock() {
        manager.cancel_timeout(id)
    } else {
        Err(ChannelError::AllocationError("Failed to lock timeout manager".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_timeout_manager_basic() {
        let mut manager = TimeoutManager::new();
        assert!(manager.start().is_ok());

        let handle = manager.register_timeout(Duration::from_millis(50)).unwrap();
        assert!(!handle.is_triggered());
        
        // Wait for timeout
        std::thread::sleep(Duration::from_millis(100));
        assert!(handle.is_triggered());

        assert!(manager.stop().is_ok());
    }

    #[test]
    fn test_timeout_cancellation() {
        let mut manager = TimeoutManager::new();
        assert!(manager.start().is_ok());

        let handle = manager.register_timeout(Duration::from_millis(100)).unwrap();
        
        // Cancel immediately
        handle.cancel();
        assert!(handle.is_cancelled());

        // Should not trigger after timeout period
        std::thread::sleep(Duration::from_millis(150));
        assert!(!handle.is_triggered());

        assert!(manager.stop().is_ok());
    }

    #[test]
    fn test_timeout_callback() {
        let mut manager = TimeoutManager::new();
        assert!(manager.start().is_ok());

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let handle = manager.register_timeout_with_callback(
            Duration::from_millis(50),
            move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        ).unwrap();

        // Wait for callback
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert!(handle.is_triggered());

        assert!(manager.stop().is_ok());
    }

    #[test]
    fn test_multiple_timeouts() {
        let mut manager = TimeoutManager::new();
        assert!(manager.start().is_ok());

        let handles: Vec<_> = (0..10)
            .map(|i| manager.register_timeout(Duration::from_millis(50 + i * 10)).unwrap())
            .collect();

        // Wait for all timeouts
        std::thread::sleep(Duration::from_millis(200));

        for handle in handles {
            assert!(handle.is_triggered());
        }

        assert!(manager.stop().is_ok());
    }

    #[test]
    fn test_global_timeout_manager() {
        assert!(init_timeout_manager().is_ok());

        let handle = register_timeout(Duration::from_millis(50)).unwrap();
        
        std::thread::sleep(Duration::from_millis(100));
        assert!(handle.is_triggered());

        assert!(shutdown_timeout_manager().is_ok());
    }
}
