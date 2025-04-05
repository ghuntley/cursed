//! Channel implementation for CURSED language
//! Provides basic send and receive operations for concurrent programming

use std::sync::{Mutex, Condvar};
use std::collections::VecDeque;
use std::sync::Arc;

/// A Channel represents a typed communication channel
/// using a thread-safe queue with blocking operations
#[derive(Clone)]
pub struct Channel<T> {
    internal: Arc<ChannelInternal<T>>,
}

struct ChannelInternal<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> Channel<T> 
where 
    T: Clone + Send + 'static
{
    /// Create a new channel
    pub fn new() -> Self {
        Channel {
            internal: Arc::new(ChannelInternal {
                queue: Mutex::new(VecDeque::new()),
                condvar: Condvar::new(),
            }),
        }
    }

    /// Send a value to the channel
    pub fn send(&self, value: T) {
        let mut queue = self.internal.queue.lock().unwrap();
        queue.push_back(value);
        self.internal.condvar.notify_one();
    }

    /// Receive a value from the channel (blocking)
    pub fn receive(&self) -> T {
        let mut queue = self.internal.queue.lock().unwrap();
        
        // Wait until a value is available
        while queue.is_empty() {
            queue = self.internal.condvar.wait(queue).unwrap();
        }
        
        queue.pop_front().unwrap()
    }
    
    /// Try to receive a value without blocking
    /// Returns None if no value is available
    pub fn try_receive(&self) -> Option<T> {
        let mut queue = self.internal.queue.lock().unwrap();
        queue.pop_front()
    }
}