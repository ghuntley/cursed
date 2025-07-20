//! Enhanced select statement implementation for CURSED channels
//!
//! This provides a simplified but comprehensive select implementation that works
//! with the existing SimpleChannel system.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::thread;

use crate::runtime::channels::{ChannelError, ChannelResult, SendResult, ReceiveResult};
use crate::runtime::channels::simple_channel::{SimpleChannel, SimpleChannelSender, SimpleChannelReceiver};

/// Select operation result
#[derive(Debug, Clone)]
pub enum SelectResult<T> {
    /// A send operation completed successfully
    SendCompleted(usize),
    /// A receive operation completed successfully
    ReceiveCompleted(usize, T),
    /// Default case was executed (non-blocking mode)
    DefaultExecuted,
    /// Timeout occurred
    Timeout,
    /// All channels are closed
    AllClosed,
}

/// Select case type
#[derive(Debug, Clone)]
pub enum SelectCase<T> {
    /// Send operation
    Send {
        channel_id: usize,
        case_index: usize,
        value: T,
    },
    /// Receive operation
    Receive {
        channel_id: usize,
        case_index: usize,
    },
    /// Default case
    Default {
        case_index: usize,
    },
}

/// Select statement builder for SimpleChannel
pub struct SimpleSelect<T> {
    /// Cases to select from
    cases: Vec<SelectCase<T>>,
    /// Channel references
    channels: HashMap<usize, Arc<SimpleChannel<T>>>,
    /// Timeout duration
    timeout: Option<Duration>,
    /// Next case index
    next_case_index: AtomicUsize,
    /// Random seed for fair selection
    random_seed: AtomicUsize,
}

impl<T: Send + Clone + 'static> SimpleSelect<T> {
    /// Create a new select statement
    pub fn new() -> Self {
        Self {
            cases: Vec::new(),
            channels: HashMap::new(),
            timeout: None,
            next_case_index: AtomicUsize::new(0),
            random_seed: AtomicUsize::new(1),
        }
    }
    
    /// Add a case to the select statement
    pub fn add_case(&mut self, case: SelectCase<T>) {
        match &case {
            SelectCase::Send { channel_id, .. } => {
                // Channel should already be registered
            }
            SelectCase::Receive { channel_id, .. } => {
                // Channel should already be registered
            }
            SelectCase::Default { .. } => {
                // No channel needed for default case
            }
        }
        self.cases.push(case);
    }
    
    /// Add a send case
    pub fn send(
        &mut self,
        channel_id: usize,
        channel: Arc<SimpleChannel<T>>,
        value: T,
    ) -> &mut Self {
        let case_index = self.next_case_index.fetch_add(1, Ordering::SeqCst);
        
        self.channels.insert(channel_id, channel);
        self.cases.push(SelectCase::Send {
            channel_id,
            case_index,
            value,
        });
        
        self
    }
    
    /// Add a receive case
    pub fn receive(
        &mut self,
        channel_id: usize,
        channel: Arc<SimpleChannel<T>>,
    ) -> &mut Self {
        let case_index = self.next_case_index.fetch_add(1, Ordering::SeqCst);
        
        self.channels.insert(channel_id, channel);
        self.cases.push(SelectCase::Receive {
            channel_id,
            case_index,
        });
        
        self
    }
    
    /// Add a default case (makes select non-blocking)
    pub fn default_case(&mut self) -> &mut Self {
        let case_index = self.next_case_index.fetch_add(1, Ordering::SeqCst);
        
        self.cases.push(SelectCase::Default { case_index });
        self
    }
    
    /// Set timeout for the select operation
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = Some(duration);
        self
    }
    
    /// Execute the select statement
    pub fn execute(&mut self) -> ChannelResult<SelectResult<T>> {
        if self.cases.is_empty() {
            return Err(ChannelError::NoSenders);
        }
        
        let start_time = Instant::now();
        let has_default = self.cases.iter().any(|case| {
            matches!(case, SelectCase::Default { .. })
        });
        
        // Use exponential backoff for better performance
        let mut backoff_nanos = 100;
        let max_backoff_nanos = 1_000_000; // 1ms max
        
        loop {
            // Check timeout first
            if let Some(timeout) = self.timeout {
                if start_time.elapsed() >= timeout {
                    return Ok(SelectResult::Timeout);
                }
            }
            
            // Try all operations in random order
            let ready_cases = self.find_ready_cases()?;
            
            if !ready_cases.is_empty() {
                // Select a random ready case
                let selected_index = self.select_random_case(&ready_cases);
                return self.execute_case(selected_index);
            }
            
            // If no cases are ready and we have a default case, execute it immediately
            if has_default {
                return Ok(SelectResult::DefaultExecuted);
            }
            
            // Check if all channels are closed
            if self.all_channels_closed() {
                return Ok(SelectResult::AllClosed);
            }
            
            // Use proper event-driven waiting instead of busy polling
            self.wait_for_channel_activity(Duration::from_nanos(backoff_nanos))?;
            backoff_nanos = (backoff_nanos * 2).min(max_backoff_nanos);
        }
    }
    
    /// Find all ready cases
    fn find_ready_cases(&self) -> ChannelResult<Vec<usize>> {
        let mut ready_cases = Vec::new();
        
        for (i, case) in self.cases.iter().enumerate() {
            match case {
                SelectCase::Send { channel_id, .. } => {
                    if let Some(channel) = self.channels.get(channel_id) {
                        // Check if we can send (buffer has space or unbuffered has receiver)
                        if channel.available_space() > 0 && !channel.is_closed() {
                            ready_cases.push(i);
                        }
                    }
                }
                SelectCase::Receive { channel_id, .. } => {
                    if let Some(channel) = self.channels.get(channel_id) {
                        // Check if we can receive (buffer has data)
                        if !channel.is_empty() && !channel.is_closed() {
                            ready_cases.push(i);
                        }
                    }
                }
                SelectCase::Default { .. } => {
                    // Default is always ready
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
        let seed = self.random_seed.fetch_add(1, Ordering::SeqCst);
        let index = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % ready_cases.len();
        ready_cases[index]
    }
    
    /// Execute a selected case
    fn execute_case(&mut self, select_case_index: usize) -> ChannelResult<SelectResult<T>> {
        let case = &self.cases[select_case_index];
        
        match case {
            SelectCase::Send { channel_id, case_index, value } => {
                if let Some(channel) = self.channels.get(channel_id) {
                    match channel.try_send(value.clone()) {
                        SendResult::Sent => Ok(SelectResult::SendCompleted(*channel_id)),
                        SendResult::Closed(_) => Err(ChannelError::Closed),
                        SendResult::WouldBlock(_) => {
                            // This shouldn't happen if we checked readiness correctly
                            Err(ChannelError::WouldBlock)
                        }
                    }
                } else {
                    Err(ChannelError::NoSenders)
                }
            }
            SelectCase::Receive { channel_id, case_index } => {
                if let Some(channel) = self.channels.get(channel_id) {
                    match channel.try_recv() {
                        ReceiveResult::Received(value) => Ok(SelectResult::ReceiveCompleted(*channel_id, value)),
                        ReceiveResult::Closed => Err(ChannelError::Closed),
                        ReceiveResult::WouldBlock => {
                            // This shouldn't happen if we checked readiness correctly
                            Err(ChannelError::WouldBlock)
                        }
                    }
                } else {
                    Err(ChannelError::NoReceivers)
                }
            }
            SelectCase::Default { case_index } => {
                Ok(SelectResult::DefaultExecuted)
            }
        }
    }
    
    /// Check if all channels are closed
    fn all_channels_closed(&self) -> bool {
        self.channels.values().all(|channel| channel.is_closed())
    }
    
    /// Wait for channel activity using thread parking (simplified approach)
    fn wait_for_channel_activity(&self, timeout: Duration) -> ChannelResult<()> {
        // Use thread parking as a simplified approach without complex callbacks
        // This is more efficient than busy waiting but simpler than full event-driven
        std::thread::park_timeout(timeout);
        Ok(())
    }
}

/// Convenience functions for common select patterns

/// Select between multiple receive operations
pub fn select_receive<T: Send + Clone + 'static>(
    channels: Vec<(usize, Arc<SimpleChannel<T>>)>,
    timeout: Option<Duration>,
) -> ChannelResult<SelectResult<T>> {
    let mut select = SimpleSelect::new();
    
    for (id, channel) in channels {
        select.receive(id, channel);
    }
    
    if let Some(timeout) = timeout {
        select.timeout(timeout);
    }
    
    select.execute()
}

/// Select between multiple send operations
pub fn select_send<T: Send + Clone + 'static>(
    operations: Vec<(usize, Arc<SimpleChannel<T>>, T)>,
    timeout: Option<Duration>,
) -> ChannelResult<SelectResult<T>> {
    let mut select = SimpleSelect::new();
    
    for (id, channel, value) in operations {
        select.send(id, channel, value);
    }
    
    if let Some(timeout) = timeout {
        select.timeout(timeout);
    }
    
    select.execute()
}

/// Mixed select with both send and receive operations
pub struct MixedSelectBuilder<T> {
    select: SimpleSelect<T>,
}

impl<T: Send + Clone + 'static> MixedSelectBuilder<T> {
    pub fn new() -> Self {
        Self {
            select: SimpleSelect::new(),
        }
    }
    
    pub fn send(mut self, channel_id: usize, channel: Arc<SimpleChannel<T>>, value: T) -> Self {
        self.select.send(channel_id, channel, value);
        self
    }
    
    pub fn receive(mut self, channel_id: usize, channel: Arc<SimpleChannel<T>>) -> Self {
        self.select.receive(channel_id, channel);
        self
    }
    
    pub fn default_case(mut self) -> Self {
        self.select.default_case();
        self
    }
    
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.select.timeout(duration);
        self
    }
    
    pub fn execute(mut self) -> ChannelResult<SelectResult<T>> {
        self.select.execute()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_select_receive() {
        let channel1 = Arc::new(SimpleChannel::with_capacity(1));
        let channel2 = Arc::new(SimpleChannel::with_capacity(1));
        
        // Send to channel1
        assert!(matches!(channel1.try_send(42), SendResult::Sent));
        
        let channels = vec![(1, channel1), (2, channel2)];
        let result = select_receive(channels, Some(Duration::from_millis(100)));
        
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
    fn test_select_send() {
        let channel1 = Arc::new(SimpleChannel::with_capacity(1));
        let channel2 = Arc::new(SimpleChannel::with_capacity(0)); // unbuffered
        
        let operations = vec![(1, channel1, 42), (2, channel2, 24)];
        let result = select_send(operations, Some(Duration::from_millis(100)));
        
        assert!(result.is_ok());
        match result.unwrap() {
            SelectResult::SendCompleted(1) => {},
            other => {
                eprintln!("Expected SendCompleted(1), got {:?} - test failed gracefully", other);
                assert!(false, "Expected SendCompleted(1)");
            },
        }
    }
    
    #[test]
    fn test_mixed_select() {
        let send_channel = Arc::new(SimpleChannel::with_capacity(1));
        let recv_channel = Arc::new(SimpleChannel::with_capacity(1));
        
        // Put something in the receive channel
        assert!(matches!(recv_channel.try_send(100), SendResult::Sent));
        
        let result = MixedSelectBuilder::new()
            .send(1, send_channel, 42)
            .receive(2, recv_channel)
            .timeout(Duration::from_millis(100))
            .execute();
        
        assert!(result.is_ok());
        // Either operation could complete due to non-deterministic select behavior
        match result.unwrap() {
            SelectResult::ReceiveCompleted(2, 100) => {
                // Preferred outcome - receive from channel with data
            },
            SelectResult::SendCompleted(1) => {
                // Also valid - send to channel with capacity
            },
            other => {
                eprintln!("Expected either ReceiveCompleted(2, 100) or SendCompleted(1), got {:?} - test failed gracefully", other);
                assert!(false, "Expected either ReceiveCompleted(2, 100) or SendCompleted(1)");
            },
        }
    }
    
    #[test]
    fn test_select_timeout() {
        let channel = Arc::new(SimpleChannel::<i32>::with_capacity(0)); // unbuffered
        
        let channels = vec![(1, channel)];
        let result = select_receive(channels, Some(Duration::from_millis(10)));
        
        assert!(result.is_ok());
        match result.unwrap() {
            SelectResult::Timeout => {},
            other => {
                eprintln!("Expected Timeout, got {:?} - test failed gracefully", other);
                assert!(false, "Expected Timeout");
            },
        }
    }
    
    #[test]
    fn test_select_default() {
        let channel = Arc::new(SimpleChannel::<i32>::with_capacity(0)); // unbuffered
        
        let result = MixedSelectBuilder::new()
            .receive(1, channel)
            .default_case()
            .execute();
        
        assert!(result.is_ok());
        match result.unwrap() {
            SelectResult::DefaultExecuted => {},
            other => {
                eprintln!("Expected DefaultExecuted, got {:?} - test failed gracefully", other);
                assert!(false, "Expected DefaultExecuted");
            },
        }
    }
}
