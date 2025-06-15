/// Signal filtering and throttling utilities
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use crate::stdlib::signal_boost::core::BoostSignal;
use crate::stdlib::signal_boost::error::{SignalBoostError, SignalBoostResult};

/// Filter signals based on a predicate function
pub fn filter_signals<F>(input: Receiver<BoostSignal>, predicate: F) -> Receiver<BoostSignal>
where
    F: Fn(BoostSignal) -> bool + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        while let Ok(signal) = input.recv() {
            if predicate(signal) {
                if sender.send(signal).is_err() {
                    tracing::debug!("Filter output channel closed");
                    break;
                }
                tracing::debug!("Signal {} passed filter", signal);
            } else {
                tracing::debug!("Signal {} filtered out", signal);
            }
        }
        tracing::debug!("Signal filter thread stopped");
    });
    
    receiver
}

/// Throttle signals to prevent flooding
pub fn throttle_signals(input: Receiver<BoostSignal>, interval: Duration) -> Receiver<BoostSignal> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let mut last_sent: HashMap<BoostSignal, Instant> = HashMap::new();
        
        while let Ok(signal) = input.recv() {
            let now = Instant::now();
            let should_send = match last_sent.get(&signal) {
                Some(last_time) => now.duration_since(*last_time) >= interval,
                None => true,
            };
            
            if should_send {
                if sender.send(signal).is_err() {
                    tracing::debug!("Throttle output channel closed");
                    break;
                }
                last_sent.insert(signal, now);
                tracing::debug!("Signal {} sent (throttled)", signal);
            } else {
                tracing::debug!("Signal {} throttled", signal);
            }
        }
        tracing::debug!("Signal throttle thread stopped");
    });
    
    receiver
}

/// Debounce signals to only process the last one in a sequence
pub fn debounce_signals(input: Receiver<BoostSignal>, interval: Duration) -> Receiver<BoostSignal> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let mut pending_signals: HashMap<BoostSignal, Instant> = HashMap::new();
        
        // Process input signals
        let input_thread_sender = sender.clone();
        let pending_signals_shared = Arc::new(Mutex::new(pending_signals.clone()));
        let pending_clone = Arc::clone(&pending_signals_shared);
        
        thread::spawn(move || {
            while let Ok(signal) = input.recv() {
                let mut pending = pending_clone.lock().unwrap();
                pending.insert(signal, Instant::now());
                tracing::debug!("Signal {} received for debouncing", signal);
            }
        });
        
        // Debounce timer thread
        loop {
            thread::sleep(Duration::from_millis(10)); // Check every 10ms
            
            let now = Instant::now();
            let mut to_send = Vec::new();
            
            {
                let mut pending = pending_signals_shared.lock().unwrap();
                pending.retain(|&signal, &mut last_time| {
                    if now.duration_since(last_time) >= interval {
                        to_send.push(signal);
                        false // Remove from pending
                    } else {
                        true // Keep in pending
                    }
                });
            }
            
            for signal in to_send {
                if sender.send(signal).is_err() {
                    tracing::debug!("Debounce output channel closed");
                    return;
                }
                tracing::debug!("Signal {} sent (debounced)", signal);
            }
        }
    });
    
    receiver
}

/// Buffer signals and release them in batches
pub fn buffer_signals(input: Receiver<BoostSignal>, buffer_size: usize, flush_interval: Duration) -> Receiver<Vec<BoostSignal>> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let mut buffer = Vec::new();
        let mut last_flush = Instant::now();
        
        loop {
            match input.recv_timeout(Duration::from_millis(10)) {
                Ok(signal) => {
                    buffer.push(signal);
                    tracing::debug!("Buffered signal {}, buffer size: {}", signal, buffer.len());
                    
                    // Flush if buffer is full
                    if buffer.len() >= buffer_size {
                        if sender.send(buffer.clone()).is_err() {
                            tracing::debug!("Buffer output channel closed");
                            break;
                        }
                        tracing::debug!("Flushed buffer (size limit): {} signals", buffer.len());
                        buffer.clear();
                        last_flush = Instant::now();
                    }
                },
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Check if we should flush due to time
                    if !buffer.is_empty() && last_flush.elapsed() >= flush_interval {
                        if sender.send(buffer.clone()).is_err() {
                            tracing::debug!("Buffer output channel closed");
                            break;
                        }
                        tracing::debug!("Flushed buffer (time limit): {} signals", buffer.len());
                        buffer.clear();
                        last_flush = Instant::now();
                    }
                },
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    // Flush remaining buffer before closing
                    if !buffer.is_empty() {
                        let _ = sender.send(buffer);
                        tracing::debug!("Flushed final buffer: {} signals", buffer.len());
                    }
                    break;
                }
            }
        }
        tracing::debug!("Signal buffer thread stopped");
    });
    
    receiver
}

/// Rate limit signals globally across all signal types
pub fn rate_limit_signals(input: Receiver<BoostSignal>, max_per_second: usize) -> Receiver<BoostSignal> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let mut signal_times = Vec::new();
        let window = Duration::from_secs(1);
        
        while let Ok(signal) = input.recv() {
            let now = Instant::now();
            
            // Remove old timestamps outside the window
            signal_times.retain(|&time| now.duration_since(time) < window);
            
            // Check if we're under the rate limit
            if signal_times.len() < max_per_second {
                signal_times.push(now);
                if sender.send(signal).is_err() {
                    tracing::debug!("Rate limit output channel closed");
                    break;
                }
                tracing::debug!("Signal {} sent (rate limited: {}/{})", signal, signal_times.len(), max_per_second);
            } else {
                tracing::debug!("Signal {} dropped (rate limit exceeded: {}/{})", signal, signal_times.len(), max_per_second);
            }
        }
        tracing::debug!("Signal rate limit thread stopped");
    });
    
    receiver
}

/// Sample signals - only pass through every Nth signal
pub fn sample_signals(input: Receiver<BoostSignal>, sample_rate: usize) -> Receiver<BoostSignal> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let mut counter = 0;
        
        while let Ok(signal) = input.recv() {
            counter += 1;
            
            if counter % sample_rate == 0 {
                if sender.send(signal).is_err() {
                    tracing::debug!("Sample output channel closed");
                    break;
                }
                tracing::debug!("Signal {} sampled (every {} signals)", signal, sample_rate);
            } else {
                tracing::debug!("Signal {} skipped (sample rate {})", signal, sample_rate);
            }
        }
        tracing::debug!("Signal sample thread stopped");
    });
    
    receiver
}

/// Deduplicate consecutive identical signals
pub fn deduplicate_signals(input: Receiver<BoostSignal>) -> Receiver<BoostSignal> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let mut last_signal: Option<BoostSignal> = None;
        
        while let Ok(signal) = input.recv() {
            if Some(signal) != last_signal {
                if sender.send(signal).is_err() {
                    tracing::debug!("Deduplicate output channel closed");
                    break;
                }
                tracing::debug!("Signal {} sent (deduplicated)", signal);
                last_signal = Some(signal);
            } else {
                tracing::debug!("Signal {} deduplicated (consecutive)", signal);
            }
        }
        tracing::debug!("Signal deduplicate thread stopped");
    });
    
    receiver
}

/// Transform signals using a mapping function
pub fn transform_signals<F>(input: Receiver<BoostSignal>, transform: F) -> Receiver<BoostSignal>
where
    F: Fn(BoostSignal) -> Option<BoostSignal> + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        while let Ok(signal) = input.recv() {
            if let Some(transformed) = transform(signal) {
                if sender.send(transformed).is_err() {
                    tracing::debug!("Transform output channel closed");
                    break;
                }
                tracing::debug!("Signal {} transformed to {}", signal, transformed);
            } else {
                tracing::debug!("Signal {} transformed to None (filtered)", signal);
            }
        }
        tracing::debug!("Signal transform thread stopped");
    });
    
    receiver
}

/// Priority queue for signals - higher priority signals are sent first
pub fn prioritize_signals(input: Receiver<BoostSignal>, get_priority: fn(BoostSignal) -> i32) -> Receiver<BoostSignal> {
    use std::collections::BinaryHeap;
    use std::cmp::Ordering;
    
    #[derive(Eq, PartialEq)]
    struct PrioritySignal {
        signal: BoostSignal,
        priority: i32,
        timestamp: Instant,
    }
    
    impl Ord for PrioritySignal {
        fn cmp(&self, other: &Self) -> Ordering {
            // Higher priority first, then older timestamp first
            match self.priority.cmp(&other.priority) {
                Ordering::Equal => other.timestamp.cmp(&self.timestamp),
                other => other,
            }
        }
    }
    
    impl PartialOrd for PrioritySignal {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let mut priority_queue = BinaryHeap::new();
        let mut last_send = Instant::now();
        let send_interval = Duration::from_millis(10);
        
        loop {
            // Try to receive new signals
            match input.recv_timeout(Duration::from_millis(1)) {
                Ok(signal) => {
                    let priority = get_priority(signal);
                    priority_queue.push(PrioritySignal {
                        signal,
                        priority,
                        timestamp: Instant::now(),
                    });
                    tracing::debug!("Signal {} queued with priority {}", signal, priority);
                },
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Normal timeout, continue to send queued signals
                },
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    // Input closed, send remaining signals and exit
                    while let Some(priority_signal) = priority_queue.pop() {
                        let _ = sender.send(priority_signal.signal);
                    }
                    break;
                }
            }
            
            // Send highest priority signal if enough time has passed
            if last_send.elapsed() >= send_interval {
                if let Some(priority_signal) = priority_queue.pop() {
                    if sender.send(priority_signal.signal).is_err() {
                        tracing::debug!("Priority output channel closed");
                        break;
                    }
                    tracing::debug!("Signal {} sent (priority {})", priority_signal.signal, priority_signal.priority);
                    last_send = Instant::now();
                }
            }
        }
        tracing::debug!("Signal priority thread stopped");
    });
    
    receiver
}

/// Composite filter that applies multiple filtering stages
pub struct SignalFilterChain {
    stages: Vec<Box<dyn Fn(Receiver<BoostSignal>) -> Receiver<BoostSignal> + Send>>,
}

impl SignalFilterChain {
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
        }
    }
    
    pub fn add_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(Receiver<BoostSignal>) -> Receiver<BoostSignal> + Send + 'static,
    {
        self.stages.push(Box::new(filter));
        self
    }
    
    pub fn add_predicate_filter<P>(self, predicate: P) -> Self
    where
        P: Fn(BoostSignal) -> bool + Send + 'static,
    {
        self.add_filter(move |input| filter_signals(input, predicate))
    }
    
    pub fn add_throttle(self, interval: Duration) -> Self {
        self.add_filter(move |input| throttle_signals(input, interval))
    }
    
    pub fn add_debounce(self, interval: Duration) -> Self {
        self.add_filter(move |input| debounce_signals(input, interval))
    }
    
    pub fn add_rate_limit(self, max_per_second: usize) -> Self {
        self.add_filter(move |input| rate_limit_signals(input, max_per_second))
    }
    
    pub fn add_sample(self, sample_rate: usize) -> Self {
        self.add_filter(move |input| sample_signals(input, sample_rate))
    }
    
    pub fn add_deduplicate(self) -> Self {
        self.add_filter(|input| deduplicate_signals(input))
    }
    
    pub fn apply(self, input: Receiver<BoostSignal>) -> Receiver<BoostSignal> {
        let mut current = input;
        
        for stage in self.stages {
            current = stage(current);
        }
        
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::signal_boost::core::{SIGINT, SIGTERM, SIGHUP};
    use std::sync::mpsc;
    use std::time::Duration;
    
    #[test]
    fn test_filter_signals() {
        let (sender, input) = mpsc::channel();
        let output = filter_signals(input, |signal| signal == SIGINT);
        
        sender.send(SIGINT).unwrap();
        sender.send(SIGTERM).unwrap();
        sender.send(SIGINT).unwrap();
        drop(sender);
        
        let mut received = Vec::new();
        while let Ok(signal) = output.recv() {
            received.push(signal);
        }
        
        assert_eq!(received.len(), 2);
        assert!(received.iter().all(|&s| s == SIGINT));
    }
    
    #[test]
    fn test_throttle_signals() {
        let (sender, input) = mpsc::channel();
        let output = throttle_signals(input, Duration::from_millis(100));
        
        // Send signals rapidly
        sender.send(SIGINT).unwrap();
        sender.send(SIGINT).unwrap();
        sender.send(SIGINT).unwrap();
        
        // Should only receive one initially
        let signal = output.recv_timeout(Duration::from_millis(50)).unwrap();
        assert_eq!(signal, SIGINT);
        
        // Should timeout on second signal (throttled)
        assert!(output.recv_timeout(Duration::from_millis(50)).is_err());
        
        drop(sender);
    }
    
    #[test]
    fn test_debounce_signals() {
        let (sender, input) = mpsc::channel();
        let output = debounce_signals(input, Duration::from_millis(50));
        
        // Send signals rapidly
        sender.send(SIGINT).unwrap();
        thread::sleep(Duration::from_millis(10));
        sender.send(SIGINT).unwrap();
        thread::sleep(Duration::from_millis(10));
        sender.send(SIGINT).unwrap();
        
        // Wait for debounce interval
        thread::sleep(Duration::from_millis(100));
        
        // Should only receive one signal (debounced)
        let signal = output.recv_timeout(Duration::from_millis(50)).unwrap();
        assert_eq!(signal, SIGINT);
        
        // Should not receive another
        assert!(output.recv_timeout(Duration::from_millis(50)).is_err());
        
        drop(sender);
    }
    
    #[test]
    fn test_buffer_signals() {
        let (sender, input) = mpsc::channel();
        let output = buffer_signals(input, 3, Duration::from_millis(100));
        
        // Send signals to fill buffer
        sender.send(SIGINT).unwrap();
        sender.send(SIGTERM).unwrap();
        sender.send(SIGHUP).unwrap();
        
        // Should receive a batch
        let batch = output.recv_timeout(Duration::from_millis(50)).unwrap();
        assert_eq!(batch.len(), 3);
        assert_eq!(batch[0], SIGINT);
        assert_eq!(batch[1], SIGTERM);
        assert_eq!(batch[2], SIGHUP);
        
        drop(sender);
    }
    
    #[test]
    fn test_rate_limit_signals() {
        let (sender, input) = mpsc::channel();
        let output = rate_limit_signals(input, 2); // 2 per second
        
        // Send 3 signals rapidly
        sender.send(SIGINT).unwrap();
        sender.send(SIGTERM).unwrap();
        sender.send(SIGHUP).unwrap();
        
        // Should receive first 2
        assert!(output.recv_timeout(Duration::from_millis(50)).is_ok());
        assert!(output.recv_timeout(Duration::from_millis(50)).is_ok());
        
        // Third should be rate limited
        assert!(output.recv_timeout(Duration::from_millis(50)).is_err());
        
        drop(sender);
    }
    
    #[test]
    fn test_sample_signals() {
        let (sender, input) = mpsc::channel();
        let output = sample_signals(input, 2); // Every 2nd signal
        
        sender.send(SIGINT).unwrap();  // Skipped
        sender.send(SIGTERM).unwrap(); // Sent
        sender.send(SIGHUP).unwrap();  // Skipped
        sender.send(SIGINT).unwrap();  // Sent
        drop(sender);
        
        let mut received = Vec::new();
        while let Ok(signal) = output.recv() {
            received.push(signal);
        }
        
        assert_eq!(received.len(), 2);
        assert_eq!(received[0], SIGTERM);
        assert_eq!(received[1], SIGINT);
    }
    
    #[test]
    fn test_deduplicate_signals() {
        let (sender, input) = mpsc::channel();
        let output = deduplicate_signals(input);
        
        sender.send(SIGINT).unwrap();
        sender.send(SIGINT).unwrap();  // Duplicate
        sender.send(SIGTERM).unwrap();
        sender.send(SIGTERM).unwrap(); // Duplicate
        sender.send(SIGINT).unwrap();  // Different from last
        drop(sender);
        
        let mut received = Vec::new();
        while let Ok(signal) = output.recv() {
            received.push(signal);
        }
        
        assert_eq!(received.len(), 3);
        assert_eq!(received[0], SIGINT);
        assert_eq!(received[1], SIGTERM);
        assert_eq!(received[2], SIGINT);
    }
    
    #[test]
    fn test_transform_signals() {
        let (sender, input) = mpsc::channel();
        let output = transform_signals(input, |signal| {
            if signal == SIGINT {
                Some(SIGTERM) // Transform SIGINT to SIGTERM
            } else if signal == SIGTERM {
                None // Filter out SIGTERM
            } else {
                Some(signal) // Pass through others
            }
        });
        
        sender.send(SIGINT).unwrap();
        sender.send(SIGTERM).unwrap();
        sender.send(SIGHUP).unwrap();
        drop(sender);
        
        let mut received = Vec::new();
        while let Ok(signal) = output.recv() {
            received.push(signal);
        }
        
        assert_eq!(received.len(), 2);
        assert_eq!(received[0], SIGTERM); // Transformed from SIGINT
        assert_eq!(received[1], SIGHUP);  // Passed through
    }
    
    #[test]
    fn test_signal_filter_chain() {
        let (sender, input) = mpsc::channel();
        
        let chain = SignalFilterChain::new()
            .add_predicate_filter(|signal| signal == SIGINT || signal == SIGTERM)
            .add_deduplicate()
            .add_throttle(Duration::from_millis(10));
        
        let output = chain.apply(input);
        
        sender.send(SIGINT).unwrap();
        sender.send(SIGINT).unwrap();  // Should be deduplicated
        sender.send(SIGHUP).unwrap();  // Should be filtered out
        sender.send(SIGTERM).unwrap();
        drop(sender);
        
        thread::sleep(Duration::from_millis(50));
        
        let mut received = Vec::new();
        while let Ok(signal) = output.recv_timeout(Duration::from_millis(10)) {
            received.push(signal);
        }
        
        // Should receive SIGINT and SIGTERM (SIGHUP filtered, duplicate SIGINT removed)
        assert!(received.len() >= 1);
        assert!(received.contains(&SIGINT));
    }
}
