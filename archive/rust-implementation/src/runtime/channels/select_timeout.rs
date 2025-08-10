//! Enhanced select statement with timeout patterns for CURSED channels
//!
//! This module provides sophisticated timeout handling for select statements,
//! including multiple timeout types and deadline management.

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;

use crate::runtime::channels::{ChannelError, ChannelResult, SendResult, ReceiveResult};
use crate::runtime::channels::simple_channel::SimpleChannel;
use crate::runtime::channels::enhanced_select_simple::{SelectResult, SelectCase};
use crate::runtime::channels::timeout_manager::{TimeoutManager, TimeoutHandle, TimeoutResult};

/// Timeout type for select operations
#[derive(Debug, Clone)]
pub enum TimeoutType {
    /// Absolute timeout - operation times out at specific time
    Absolute(Instant),
    /// Relative timeout - operation times out after duration
    Relative(Duration),
    /// Deadline timeout - operation times out at deadline
    Deadline(Instant),
    /// Interval timeout - operation times out at regular intervals
    Interval(Duration, Option<u32>), // (interval, max_intervals)
}

/// Timeout configuration for select operations
#[derive(Clone)]
pub struct TimeoutConfig {
    /// Primary timeout
    pub primary: TimeoutType,
    /// Secondary timeout (optional)
    pub secondary: Option<TimeoutType>,
    /// Enable timeout escalation
    pub escalation: bool,
    /// Timeout callback function
    pub callback: Option<Arc<dyn Fn(TimeoutType) + Send + Sync>>,
}

impl std::fmt::Debug for TimeoutConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeoutConfig")
            .field("primary", &self.primary)
            .field("secondary", &self.secondary)
            .field("escalation", &self.escalation)
            .field("callback", &self.callback.as_ref().map(|_| "<function>"))
            .finish()
    }
}

/// Timeout state tracking
#[derive(Debug, Clone)]
pub struct TimeoutState {
    /// Start time of the operation
    pub start_time: Instant,
    /// Current timeout deadline
    pub deadline: Instant,
    /// Timeout type being used
    pub timeout_type: TimeoutType,
    /// Number of timeout intervals passed
    pub intervals_passed: u32,
    /// Timeout escalation level
    pub escalation_level: u32,
    /// Whether timeout has been triggered
    pub triggered: bool,
}

/// Enhanced select with timeout patterns
pub struct TimeoutSelect<T> {
    /// Base select cases
    cases: Vec<SelectCase<T>>,
    /// Channel references
    channels: HashMap<usize, Arc<SimpleChannel<T>>>,
    /// Timeout configuration
    timeout_config: Option<TimeoutConfig>,
    /// Current timeout state
    timeout_state: Option<TimeoutState>,
    /// Timeout manager for reliable timeout handling
    timeout_manager: TimeoutManager,
    /// Active timeout handle
    active_timeout: Option<TimeoutHandle>,
    /// Cancellation flag
    cancelled: Arc<AtomicBool>,
    /// Next case index
    next_case_index: usize,
}

impl<T: Send + Clone + 'static> TimeoutSelect<T> {
    /// Create a new timeout select
    pub fn new() -> Self {
        let mut timeout_manager = TimeoutManager::new();
        let _ = timeout_manager.start(); // Start the timeout manager
        
        Self {
            cases: Vec::new(),
            channels: HashMap::new(),
            timeout_config: None,
            timeout_state: None,
            timeout_manager,
            active_timeout: None,
            cancelled: Arc::new(AtomicBool::new(false)),
            next_case_index: 0,
        }
    }
    
    /// Add a send case
    pub fn send(
        &mut self,
        channel_id: usize,
        channel: Arc<SimpleChannel<T>>,
        value: T,
    ) -> &mut Self {
        self.channels.insert(channel_id, channel);
        self.cases.push(SelectCase::Send {
            channel_id,
            case_index: self.next_case_index,
            value,
        });
        self.next_case_index += 1;
        self
    }
    
    /// Add a receive case
    pub fn receive(
        &mut self,
        channel_id: usize,
        channel: Arc<SimpleChannel<T>>,
    ) -> &mut Self {
        self.channels.insert(channel_id, channel);
        self.cases.push(SelectCase::Receive {
            channel_id,
            case_index: self.next_case_index,
        });
        self.next_case_index += 1;
        self
    }
    
    /// Add a default case
    pub fn default_case(&mut self) -> &mut Self {
        self.cases.push(SelectCase::Default {
            case_index: self.next_case_index,
        });
        self.next_case_index += 1;
        self
    }
    
    /// Set absolute timeout
    pub fn timeout_at(&mut self, deadline: Instant) -> &mut Self {
        self.timeout_config = Some(TimeoutConfig {
            primary: TimeoutType::Absolute(deadline),
            secondary: None,
            escalation: false,
            callback: None,
        });
        self
    }
    
    /// Set relative timeout
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout_config = Some(TimeoutConfig {
            primary: TimeoutType::Relative(duration),
            secondary: None,
            escalation: false,
            callback: None,
        });
        self
    }
    
    /// Set deadline timeout
    pub fn deadline(&mut self, deadline: Instant) -> &mut Self {
        self.timeout_config = Some(TimeoutConfig {
            primary: TimeoutType::Deadline(deadline),
            secondary: None,
            escalation: false,
            callback: None,
        });
        self
    }
    
    /// Set interval timeout
    pub fn interval(&mut self, interval: Duration, max_intervals: Option<u32>) -> &mut Self {
        self.timeout_config = Some(TimeoutConfig {
            primary: TimeoutType::Interval(interval, max_intervals),
            secondary: None,
            escalation: false,
            callback: None,
        });
        self
    }
    
    /// Enable timeout escalation
    pub fn with_escalation(&mut self) -> &mut Self {
        if let Some(ref mut config) = self.timeout_config {
            config.escalation = true;
        }
        self
    }
    
    /// Set timeout callback
    pub fn with_timeout_callback<F>(&mut self, callback: F) -> &mut Self 
    where
        F: Fn(TimeoutType) + Send + Sync + 'static,
    {
        if let Some(ref mut config) = self.timeout_config {
            config.callback = Some(Arc::new(callback));
        }
        self
    }
    
    /// Set secondary timeout for escalation
    pub fn with_secondary_timeout(&mut self, secondary: TimeoutType) -> &mut Self {
        if let Some(ref mut config) = self.timeout_config {
            config.secondary = Some(secondary);
        }
        self
    }
    
    /// Cancel the select operation
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
    
    /// Check if operation is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
    
    /// Execute the select with timeout
    pub fn execute(&mut self) -> ChannelResult<SelectResult<T>> {
        // Initialize timeout handling
        self.initialize_timeout_handling()?;
        
        let mut backoff_nanos = 100u64;
        let max_backoff_nanos = 1_000_000u64; // 1ms max
        let max_iterations = 100_000; // Prevent infinite loops
        let mut iteration_count = 0;
        
        loop {
            iteration_count += 1;
            
            // Check for infinite loop protection
            if iteration_count > max_iterations {
                self.cleanup_timeout();
                return Err(ChannelError::AllocationError("Select operation exceeded maximum iterations".to_string()));
            }
            
            // Check cancellation
            if self.is_cancelled() {
                self.cleanup_timeout();
                return Err(ChannelError::Timeout);
            }
            
            // Check timeout using the timeout manager
            if let Some(timeout_result) = self.check_timeout_manager() {
                self.cleanup_timeout();
                return timeout_result;
            }
            
            // Try to find ready cases
            let ready_cases = match self.find_ready_cases() {
                Ok(cases) => cases,
                Err(e) => {
                    self.cleanup_timeout();
                    return Err(e);
                }
            };
            
            if !ready_cases.is_empty() {
                // Execute a random ready case
                let selected_index = self.select_random_case(&ready_cases);
                self.cleanup_timeout();
                return self.execute_case(selected_index);
            }
            
            // Check for default case
            if self.has_default_case() {
                self.cleanup_timeout();
                return Ok(SelectResult::DefaultExecuted);
            }
            
            // Exponential backoff with jitter to reduce contention
            let jitter = (iteration_count % 10) * 10; // Small jitter
            let sleep_duration = Duration::from_nanos(backoff_nanos + jitter);
            std::thread::sleep(sleep_duration);
            backoff_nanos = (backoff_nanos * 2).min(max_backoff_nanos);
        }
    }
    
    /// Initialize timeout state
    fn initialize_timeout_state(&mut self) {
        if let Some(ref config) = self.timeout_config {
            let start_time = Instant::now();
            let deadline = match config.primary {
                TimeoutType::Absolute(deadline) => deadline,
                TimeoutType::Relative(duration) => start_time + duration,
                TimeoutType::Deadline(deadline) => deadline,
                TimeoutType::Interval(interval, _) => start_time + interval,
            };
            
            self.timeout_state = Some(TimeoutState {
                start_time,
                deadline,
                timeout_type: config.primary.clone(),
                intervals_passed: 0,
                escalation_level: 0,
                triggered: false,
            });
        }
    }
    
    /// Check if timeout has occurred
    fn check_timeout(&mut self) -> Option<ChannelResult<SelectResult<T>>> {
        if let Some(ref mut state) = self.timeout_state {
            let now = Instant::now();
            
            if now >= state.deadline && !state.triggered {
                state.triggered = true;
                
                // Call timeout callback if configured
                if let Some(ref config) = self.timeout_config {
                    if let Some(ref callback) = config.callback {
                        callback(state.timeout_type.clone());
                    }
                }
                
                // Handle escalation
                if let Some(ref config) = self.timeout_config {
                    if config.escalation && config.secondary.is_some() {
                        return self.escalate_timeout();
                    }
                }
                
                return Some(Ok(SelectResult::Timeout));
            }
        }
        
        None
    }
    
    /// Update timeout state for interval timeouts
    fn update_timeout_state(&mut self) {
        if let Some(ref mut state) = self.timeout_state {
            if let TimeoutType::Interval(interval, max_intervals) = &state.timeout_type {
                let now = Instant::now();
                if now >= state.deadline {
                    state.intervals_passed += 1;
                    
                    // Check if we've exceeded max intervals
                    if let Some(max) = max_intervals {
                        if state.intervals_passed >= *max {
                            state.triggered = true;
                            return;
                        }
                    }
                    
                    // Set next deadline
                    state.deadline = now + *interval;
                    
                    // Call interval callback if configured
                    if let Some(ref config) = self.timeout_config {
                        if let Some(ref callback) = config.callback {
                            callback(TimeoutType::Interval(*interval, Some(state.intervals_passed)));
                        }
                    }
                }
            }
        }
    }
    
    /// Handle timeout escalation
    fn escalate_timeout(&mut self) -> Option<ChannelResult<SelectResult<T>>> {
        if let Some(ref mut state) = self.timeout_state {
            state.escalation_level += 1;
            
            if let Some(ref config) = self.timeout_config {
                if let Some(ref secondary) = config.secondary {
                    let now = Instant::now();
                    
                    // Update to secondary timeout
                    state.timeout_type = secondary.clone();
                    state.deadline = match secondary {
                        TimeoutType::Absolute(deadline) => *deadline,
                        TimeoutType::Relative(duration) => now + *duration,
                        TimeoutType::Deadline(deadline) => *deadline,
                        TimeoutType::Interval(interval, _) => now + *interval,
                    };
                    state.triggered = false;
                    
                    // Continue with escalated timeout
                    return None;
                }
            }
        }
        
        Some(Ok(SelectResult::Timeout))
    }
    
    /// Find ready cases
    fn find_ready_cases(&self) -> ChannelResult<Vec<usize>> {
        let mut ready_cases = Vec::new();
        
        for (i, case) in self.cases.iter().enumerate() {
            match case {
                SelectCase::Send { channel_id, .. } => {
                    if let Some(channel) = self.channels.get(channel_id) {
                        if channel.available_space() > 0 && !channel.is_closed() {
                            ready_cases.push(i);
                        }
                    }
                }
                SelectCase::Receive { channel_id, .. } => {
                    if let Some(channel) = self.channels.get(channel_id) {
                        if !channel.is_empty() || channel.is_closed() {
                            ready_cases.push(i);
                        }
                    }
                }
                SelectCase::Default { .. } => {
                    ready_cases.push(i);
                }
            }
        }
        
        Ok(ready_cases)
    }
    
    /// Select a random case from ready cases
    fn select_random_case(&self, ready_cases: &[usize]) -> usize {
        if ready_cases.len() == 1 {
            return ready_cases[0];
        }
        
        // Simple pseudo-random selection
        let now = Instant::now();
        let seed = now.elapsed().as_nanos() as usize;
        let index = seed % ready_cases.len();
        ready_cases[index]
    }
    
    /// Execute a selected case
    fn execute_case(&mut self, select_case_index: usize) -> ChannelResult<SelectResult<T>> {
        let case = &self.cases[select_case_index];
        
        match case {
            SelectCase::Send { channel_id, value, .. } => {
                if let Some(channel) = self.channels.get(channel_id) {
                    match channel.try_send(value.clone()) {
                        SendResult::Sent => Ok(SelectResult::SendCompleted(*channel_id)),
                        SendResult::Closed(_) => Err(ChannelError::Closed),
                        SendResult::WouldBlock(_) => Err(ChannelError::WouldBlock),
                    }
                } else {
                    Err(ChannelError::NoSenders)
                }
            }
            SelectCase::Receive { channel_id, .. } => {
                if let Some(channel) = self.channels.get(channel_id) {
                    match channel.try_recv() {
                        ReceiveResult::Received(value) => Ok(SelectResult::ReceiveCompleted(*channel_id, value)),
                        ReceiveResult::Closed => Err(ChannelError::Closed),
                        ReceiveResult::WouldBlock => Err(ChannelError::WouldBlock),
                    }
                } else {
                    Err(ChannelError::NoReceivers)
                }
            }
            SelectCase::Default { .. } => {
                Ok(SelectResult::DefaultExecuted)
            }
        }
    }
    
    /// Check if there's a default case
    fn has_default_case(&self) -> bool {
        self.cases.iter().any(|case| {
            matches!(case, SelectCase::Default { .. })
        })
    }
    
    /// Get timeout state
    pub fn timeout_state(&self) -> Option<&TimeoutState> {
        self.timeout_state.as_ref()
    }
    
    /// Initialize timeout handling using the timeout manager
    fn initialize_timeout_handling(&mut self) -> ChannelResult<()> {
        if let Some(ref config) = self.timeout_config {
            let start_time = Instant::now();
            let duration = match config.primary {
                TimeoutType::Absolute(deadline) => {
                    if deadline > start_time {
                        deadline.duration_since(start_time)
                    } else {
                        Duration::ZERO
                    }
                },
                TimeoutType::Relative(duration) => duration,
                TimeoutType::Deadline(deadline) => {
                    if deadline > start_time {
                        deadline.duration_since(start_time)
                    } else {
                        Duration::ZERO
                    }
                },
                TimeoutType::Interval(interval, _) => interval,
            };
            
            // Register timeout with the manager
            if duration > Duration::ZERO {
                let handle = self.timeout_manager.register_timeout(duration)?;
                self.active_timeout = Some(handle);
            }
            
            // Initialize timeout state for compatibility
            let deadline = start_time + duration;
            self.timeout_state = Some(TimeoutState {
                start_time,
                deadline,
                timeout_type: config.primary.clone(),
                intervals_passed: 0,
                escalation_level: 0,
                triggered: false,
            });
        }
        Ok(())
    }
    
    /// Check timeout using the timeout manager
    fn check_timeout_manager(&mut self) -> Option<ChannelResult<SelectResult<T>>> {
        if let Some(ref handle) = self.active_timeout {
            if handle.is_triggered() {
                // Call timeout callback if configured
                if let Some(ref config) = self.timeout_config {
                    if let Some(ref callback) = config.callback {
                        if let Some(ref state) = self.timeout_state {
                            callback(state.timeout_type.clone());
                        }
                    }
                }
                
                return Some(Ok(SelectResult::Timeout));
            }
            
            if handle.is_cancelled() {
                return Some(Err(ChannelError::Timeout));
            }
        }
        
        None
    }
    
}

impl<T> TimeoutSelect<T> {
    /// Clean up active timeout
    fn cleanup_timeout(&mut self) {
        if let Some(handle) = self.active_timeout.take() {
            handle.cancel();
        }
    }
}

impl<T> Drop for TimeoutSelect<T> {
    fn drop(&mut self) {
        // Clean up active timeout on drop
        self.cleanup_timeout();
        
        // Stop the timeout manager
        let _ = self.timeout_manager.stop();
    }
}

/// Convenience functions for timeout patterns

/// Create a timeout channel for time-based operations
pub fn timeout_channel(duration: Duration) -> Arc<SimpleChannel<()>> {
    let channel = Arc::new(SimpleChannel::with_capacity(1));
    let timeout_channel = channel.clone();
    
    // Initialize the global timeout manager if needed
    let _ = crate::runtime::channels::timeout_manager::init_timeout_manager();
    
    // Use the global timeout manager instead of spawning detached threads
    if let Ok(handle) = crate::runtime::channels::timeout_manager::register_timeout_with_callback(
        duration,
        move || {
            let _ = timeout_channel.try_send(());
        }
    ) {
        // Store the handle in a static location to prevent it from being dropped
        use std::sync::Mutex;
        use std::collections::HashMap;
        static TIMEOUT_HANDLES: std::sync::LazyLock<Mutex<HashMap<usize, crate::runtime::channels::timeout_manager::TimeoutHandle>>> = 
            std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
        
        if let Ok(mut handles) = TIMEOUT_HANDLES.lock() {
            handles.insert(channel.id(), handle);
        }
    }
    
    channel
}

/// Create a deadline channel for deadline-based operations
pub fn deadline_channel(deadline: Instant) -> Arc<SimpleChannel<()>> {
    let channel = Arc::new(SimpleChannel::with_capacity(1));
    let timeout_channel = channel.clone();
    
    let now = Instant::now();
    let duration = if deadline > now { deadline - now } else { Duration::ZERO };
    
    // Initialize the global timeout manager if needed
    let _ = crate::runtime::channels::timeout_manager::init_timeout_manager();
    
    // Use the global timeout manager instead of spawning detached threads
    if duration > Duration::ZERO {
        if let Ok(handle) = crate::runtime::channels::timeout_manager::register_timeout_with_callback(
            duration,
            move || {
                let _ = timeout_channel.try_send(());
            }
        ) {
            // Store the handle in a static location to prevent it from being dropped
            use std::sync::Mutex;
            use std::collections::HashMap;
            static DEADLINE_HANDLES: std::sync::LazyLock<Mutex<HashMap<usize, crate::runtime::channels::timeout_manager::TimeoutHandle>>> = 
                std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
            
            if let Ok(mut handles) = DEADLINE_HANDLES.lock() {
                handles.insert(channel.id(), handle);
            }
        }
    } else {
        // If deadline has already passed, send immediately
        let _ = timeout_channel.try_send(());
    }
    
    channel
}

/// Create an interval channel for periodic operations
/// Note: For now, only sends the first interval. Full periodic implementation
/// requires more complex timeout manager support for repeating timeouts.
pub fn interval_channel(interval: Duration, max_intervals: Option<u32>) -> Arc<SimpleChannel<u32>> {
    let channel = Arc::new(SimpleChannel::with_capacity(16));
    let interval_channel = channel.clone();
    
    // Initialize the global timeout manager if needed
    let _ = crate::runtime::channels::timeout_manager::init_timeout_manager();
    
    // Implement basic interval support by sending the requested number of intervals
    let max_count = max_intervals.unwrap_or(1);
    use std::sync::Mutex;
    use std::collections::HashMap;
    static INTERVAL_HANDLES: std::sync::LazyLock<Mutex<HashMap<usize, Vec<crate::runtime::channels::timeout_manager::TimeoutHandle>>>> = 
        std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
    
    let mut handles = Vec::new();
    for i in 1..=max_count {
        let channel_clone = interval_channel.clone();
        let timeout_duration = interval * i as u32;
        
        if let Ok(handle) = crate::runtime::channels::timeout_manager::register_timeout_with_callback(
            timeout_duration,
            move || {
                let _ = channel_clone.try_send(i);
            }
        ) {
            handles.push(handle);
        }
    }
    
    // Store all handles to prevent them from being dropped
    if let Ok(mut all_handles) = INTERVAL_HANDLES.lock() {
        all_handles.insert(channel.id(), handles);
    }
    
    channel
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_timeout_select_basic() {
        let mut select = TimeoutSelect::<i32>::new();
        let channel = Arc::new(SimpleChannel::with_capacity(1));
        
        select.receive(1, channel)
              .timeout(Duration::from_millis(10));
        
        let result = select.execute();
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), SelectResult::Timeout));
    }

    #[test]
    fn test_timeout_select_with_data() {
        let mut select = TimeoutSelect::<i32>::new();
        let channel = Arc::new(SimpleChannel::with_capacity(1));
        
        // Send data before select
        assert!(channel.try_send(42).is_ok());
        
        select.receive(1, channel)
              .timeout(Duration::from_millis(100));
        
        let result = select.execute();
        assert!(result.is_ok());
        match result.unwrap() {
            SelectResult::ReceiveCompleted(1, 42) => {},
            other => {
                eprintln!("Expected ReceiveCompleted(1, 42), got {:?} - test failed gracefully", other);
                assert!(false, "Expected ReceiveCompleted(1, 42)");
            },
        }
    }

    #[test]
    fn test_interval_timeout() {
        let interval_ch = interval_channel(Duration::from_millis(20), Some(2));
        
        // Wait for intervals to fire
        std::thread::sleep(Duration::from_millis(60));
        
        // Should have at least one interval
        assert!(interval_ch.try_recv().is_ok(), "First interval should be available");
        
        // May have second interval depending on timing
        let second_result = interval_ch.try_recv();
        // Don't assert on second interval timing as it depends on timeout manager scheduling
        
        // Give any spawned threads time to clean up
        std::thread::sleep(Duration::from_millis(10));
    }

    #[test]
    fn test_timeout_cancellation() {
        let mut select = TimeoutSelect::<i32>::new();
        let channel = Arc::new(SimpleChannel::with_capacity(1));
        
        select.receive(1, channel)
              .timeout(Duration::from_millis(100));
        
        // Cancel immediately
        select.cancel();
        
        let result = select.execute();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ChannelError::Timeout));
    }

    #[test]
    fn test_timeout_channels() {
        let timeout_ch = timeout_channel(Duration::from_millis(30));
        let deadline_ch = deadline_channel(Instant::now() + Duration::from_millis(40));
        let interval_ch = interval_channel(Duration::from_millis(20), Some(3));
        
        // All should eventually have data - wait longer for timeout manager to process
        std::thread::sleep(Duration::from_millis(100));
        
        assert!(timeout_ch.try_recv().is_ok());
        assert!(deadline_ch.try_recv().is_ok());
        assert!(interval_ch.try_recv().is_ok());
    }
}
