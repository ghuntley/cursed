/// Channel send and receive operations for CURSED programming language
/// Implements Go-like channel semantics: ch <- value and value := <-ch

use std::sync::atomic::Ordering;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, trace, warn};

use super::channel::{ChannelData, ChannelReceiver, ChannelSender, ChannelState, WaitingReceiver, WaitingSender};
use super::{ChannelError, ChannelResult, ReceiveResult, SendResult};

/// Send operations for channels
impl<T> ChannelSender<T> {
    /// Blocking send operation: ch <- value
    /// Blocks until value is sent or channel is closed
    #[instrument(skip(self, value))]
    pub fn send(&self, value: T) -> ChannelResult<()> {
        self.increment_operation_count();
        
        let data_arc = self.channel.upgrade()
            .ok_or(ChannelError::Closed)?;
        
        let mut data = data_arc.lock().unwrap();
        
        // Check if channel is closed
        if data.state != ChannelState::Open {
            error!("Attempted to send on closed channel");
            return Err(ChannelError::Closed);
        }
        
        // Fast path: try direct send to waiting receiver
        if let Some(waiting_receiver) = data.waiting_receivers.pop_front() {
            trace!("Direct send to waiting receiver");
            self.deliver_to_receiver(waiting_receiver, value);
            return Ok(());
        }
        
        // Buffered channel: try to add to buffer
        if data.capacity > 0 {
            if data.buffer.len() < data.capacity {
                debug!(buffer_len = data.buffer.len(), capacity = data.capacity, 
                       "Adding value to buffer");
                data.buffer.push_back(value);
                self.condvar.notify_one();
                return Ok(());
            } else {
                debug!("Channel buffer is full, must block");
            }
        }
        
        // Must block: add to waiting senders
        let notify = Arc::new(Condvar::new());
        let completed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        
        let waiting_sender = WaitingSender {
            value,
            notify: notify.clone(),
            completed: completed.clone(),
        };
        
        data.waiting_senders.push_back(waiting_sender);
        
        // Release the main lock and wait
        drop(data);
        
        let mut guard = data_arc.lock().unwrap();
        while !completed.load(Ordering::Acquire) {
            // Check if channel was closed while waiting
            if guard.state != ChannelState::Open {
                // Remove ourselves from waiting list if still there
                guard.waiting_senders.retain(|ws| !Arc::ptr_eq(&ws.completed, &completed));
                return Err(ChannelError::Closed);
            }
            
            guard = notify.wait(guard).unwrap();
        }
        
        info!("Blocking send completed successfully");
        Ok(())
    }

    /// Non-blocking send operation: try_send
    /// Returns immediately with result
    #[instrument(skip(self, value))]
    pub fn try_send(&self, value: T) -> SendResult<T> {
        self.increment_operation_count();
        
        let data_arc = match self.channel.upgrade() {
            Some(arc) => arc,
            None => return SendResult::Closed(value),
        };
        
        let mut data = data_arc.lock().unwrap();
        
        // Check if channel is closed
        if data.state != ChannelState::Open {
            debug!("try_send on closed channel");
            return SendResult::Closed(value);
        }
        
        // Try direct send to waiting receiver
        if let Some(waiting_receiver) = data.waiting_receivers.pop_front() {
            trace!("try_send: direct send to waiting receiver");
            self.deliver_to_receiver(waiting_receiver, value);
            return SendResult::Sent;
        }
        
        // For buffered channels, try to add to buffer
        if data.capacity > 0 && data.buffer.len() < data.capacity {
            debug!(buffer_len = data.buffer.len(), "try_send: added to buffer");
            data.buffer.push_back(value);
            self.condvar.notify_one();
            return SendResult::Sent;
        }
        
        // Would block
        debug!("try_send would block");
        SendResult::WouldBlock(value)
    }

    /// Send with timeout
    /// Blocks for up to the specified duration
    #[instrument(skip(self, value))]
    pub fn send_timeout(&self, value: T, timeout: Duration) -> SendResult<T> {
        self.increment_operation_count();
        
        let data_arc = match self.channel.upgrade() {
            Some(arc) => arc,
            None => return SendResult::Closed(value),
        };
        
        let mut data = data_arc.lock().unwrap();
        
        // Check if channel is closed
        if data.state != ChannelState::Open {
            debug!("send_timeout on closed channel");
            return SendResult::Closed(value);
        }
        
        // Try fast path first
        if let Some(waiting_receiver) = data.waiting_receivers.pop_front() {
            trace!("send_timeout: direct send to waiting receiver");
            self.deliver_to_receiver(waiting_receiver, value);
            return SendResult::Sent;
        }
        
        // For buffered channels, try buffer
        if data.capacity > 0 && data.buffer.len() < data.capacity {
            debug!("send_timeout: added to buffer");
            data.buffer.push_back(value);
            self.condvar.notify_one();
            return SendResult::Sent;
        }
        
        // Must wait with timeout
        let notify = Arc::new(Condvar::new());
        let completed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        
        let waiting_sender = WaitingSender {
            value,
            notify: notify.clone(),
            completed: completed.clone(),
        };
        
        data.waiting_senders.push_back(waiting_sender);
        
        // Release lock and wait with timeout
        drop(data);
        
        let start_time = Instant::now();
        let mut guard = data_arc.lock().unwrap();
        
        while !completed.load(Ordering::Acquire) && start_time.elapsed() < timeout {
            // Check if channel was closed
            if guard.state != ChannelState::Open {
                guard.waiting_senders.retain(|ws| !Arc::ptr_eq(&ws.completed, &completed));
                // We need to extract the value from the waiting sender that was removed
                return SendResult::Closed(self.extract_value_from_waiting_senders(&mut guard, &completed));
            }
            
            let remaining = timeout.saturating_sub(start_time.elapsed());
            if remaining.is_zero() {
                break;
            }
            
            let (new_guard, timeout_result) = notify.wait_timeout(guard, remaining).unwrap();
            guard = new_guard;
            
            if timeout_result.timed_out() {
                break;
            }
        }
        
        if completed.load(Ordering::Acquire) {
            info!("send_timeout completed successfully");
            SendResult::Sent
        } else {
            // Timeout occurred, remove from waiting list and return value
            let value = self.extract_value_from_waiting_senders(&mut guard, &completed);
            debug!("send_timeout timed out");
            SendResult::WouldBlock(value)
        }
    }

    /// Close the channel for sending
    /// Existing buffered messages can still be received
    #[instrument(skip(self))]
    pub fn close(&self) -> ChannelResult<()> {
        let data_arc = self.channel.upgrade()
            .ok_or(ChannelError::Closed)?;
        
        let mut data = data_arc.lock().unwrap();
        
        if data.state == ChannelState::Open {
            data.state = super::channel::ChannelState::Closed;
            info!("Channel explicitly closed");
            
            // Wake up all waiting operations
            self.condvar.notify_all();
        }
        
        Ok(())
    }

    /// Helper to deliver value to a waiting receiver
    fn deliver_to_receiver(&self, waiting_receiver: WaitingReceiver<T>, value: T) {
        if let Ok(mut result) = waiting_receiver.result.lock() {
            *result = Some(value);
            waiting_receiver.completed.store(true, Ordering::Release);
            waiting_receiver.notify.notify_one();
        }
    }

    /// Helper to extract value from waiting senders list
    fn extract_value_from_waiting_senders(&self, data: &mut std::sync::MutexGuard<ChannelData<T>>, completed: &Arc<std::sync::atomic::AtomicBool>) -> T {
        // Find and remove the waiting sender, extracting its value
        if let Some(pos) = data.waiting_senders.iter().position(|ws| Arc::ptr_eq(&ws.completed, completed)) {
            data.waiting_senders.remove(pos).unwrap().value
        } else {
            // This should not happen in normal operation
            panic!("Could not find waiting sender to extract value from");
        }
    }

    /// Increment operation counter for statistics
    fn increment_operation_count(&self) {
        unsafe {
            (*self.operation_count).fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// Receive operations for channels
impl<T> ChannelReceiver<T> {
    /// Blocking receive operation: value := <-ch
    /// Blocks until value is received or channel is closed and drained
    #[instrument(skip(self))]
    pub fn receive(&self) -> ChannelResult<T> {
        self.increment_operation_count();
        
        let data_arc = self.channel.upgrade()
            .ok_or(ChannelError::Closed)?;
        
        let mut data = data_arc.lock().unwrap();
        
        // Fast path: try to get from buffer
        if let Some(value) = data.buffer.pop_front() {
            trace!("Received value from buffer");
            
            // Check if there's a waiting sender we can now accommodate
            if let Some(waiting_sender) = data.waiting_senders.pop_front() {
                data.buffer.push_back(waiting_sender.value);
                waiting_sender.completed.store(true, Ordering::Release);
                waiting_sender.notify.notify_one();
            }
            
            self.condvar.notify_one();
            return Ok(value);
        }
        
        // Try direct receive from waiting sender (unbuffered case)
        if let Some(waiting_sender) = data.waiting_senders.pop_front() {
            trace!("Direct receive from waiting sender");
            let value = waiting_sender.value;
            waiting_sender.completed.store(true, Ordering::Release);
            waiting_sender.notify.notify_one();
            return Ok(value);
        }
        
        // Check if channel is closed and drained
        if data.state != ChannelState::Open && data.buffer.is_empty() && data.waiting_senders.is_empty() {
            debug!("Receive on closed and drained channel");
            return Err(ChannelError::Closed);
        }
        
        // Must block: add to waiting receivers
        let result_mutex = Arc::new(Mutex::new(None));
        let notify = Arc::new(Condvar::new());
        let completed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        
        let waiting_receiver = WaitingReceiver {
            result: result_mutex.clone(),
            notify: notify.clone(),
            completed: completed.clone(),
        };
        
        data.waiting_receivers.push_back(waiting_receiver);
        
        // Release the main lock and wait
        drop(data);
        
        let mut guard = data_arc.lock().unwrap();
        while !completed.load(Ordering::Acquire) {
            // Check if channel is closed and drained
            if guard.state != ChannelState::Open 
                && guard.buffer.is_empty() 
                && guard.waiting_senders.is_empty() {
                // Remove ourselves from waiting list
                guard.waiting_receivers.retain(|wr| !Arc::ptr_eq(&wr.completed, &completed));
                return Err(ChannelError::Closed);
            }
            
            guard = notify.wait(guard).unwrap();
        }
        
        // Get the result
        let result = result_mutex.lock().unwrap().take()
            .ok_or(ChannelError::InvalidState)?;
        
        info!("Blocking receive completed successfully");
        Ok(result)
    }

    /// Non-blocking receive operation: try_receive
    /// Returns immediately with result
    #[instrument(skip(self))]
    pub fn try_receive(&self) -> ReceiveResult<T> {
        self.increment_operation_count();
        
        let data_arc = match self.channel.upgrade() {
            Some(arc) => arc,
            None => return ReceiveResult::Closed,
        };
        
        let mut data = data_arc.lock().unwrap();
        
        // Try to get from buffer
        if let Some(value) = data.buffer.pop_front() {
            trace!("try_receive: got value from buffer");
            
            // Check if there's a waiting sender we can now accommodate
            if let Some(waiting_sender) = data.waiting_senders.pop_front() {
                data.buffer.push_back(waiting_sender.value);
                waiting_sender.completed.store(true, Ordering::Release);
                waiting_sender.notify.notify_one();
            }
            
            self.condvar.notify_one();
            return ReceiveResult::Received(value);
        }
        
        // Try direct receive from waiting sender
        if let Some(waiting_sender) = data.waiting_senders.pop_front() {
            trace!("try_receive: direct receive from waiting sender");
            let value = waiting_sender.value;
            waiting_sender.completed.store(true, Ordering::Release);
            waiting_sender.notify.notify_one();
            return ReceiveResult::Received(value);
        }
        
        // Check if channel is closed and drained
        if data.state != ChannelState::Open {
            debug!("try_receive on closed channel");
            return ReceiveResult::Closed;
        }
        
        // Would block
        debug!("try_receive would block");
        ReceiveResult::WouldBlock
    }

    /// Receive with timeout
    /// Blocks for up to the specified duration
    #[instrument(skip(self))]
    pub fn receive_timeout(&self, timeout: Duration) -> ReceiveResult<T> {
        self.increment_operation_count();
        
        let data_arc = match self.channel.upgrade() {
            Some(arc) => arc,
            None => return ReceiveResult::Closed,
        };
        
        let mut data = data_arc.lock().unwrap();
        
        // Try fast paths first
        if let Some(value) = data.buffer.pop_front() {
            trace!("receive_timeout: got value from buffer");
            
            if let Some(waiting_sender) = data.waiting_senders.pop_front() {
                data.buffer.push_back(waiting_sender.value);
                waiting_sender.completed.store(true, Ordering::Release);
                waiting_sender.notify.notify_one();
            }
            
            self.condvar.notify_one();
            return ReceiveResult::Received(value);
        }
        
        if let Some(waiting_sender) = data.waiting_senders.pop_front() {
            trace!("receive_timeout: direct receive from waiting sender");
            let value = waiting_sender.value;
            waiting_sender.completed.store(true, Ordering::Release);
            waiting_sender.notify.notify_one();
            return ReceiveResult::Received(value);
        }
        
        // Check if channel is closed and drained
        if data.state != ChannelState::Open {
            debug!("receive_timeout on closed channel");
            return ReceiveResult::Closed;
        }
        
        // Must wait with timeout
        let result_mutex = Arc::new(Mutex::new(None));
        let notify = Arc::new(Condvar::new());
        let completed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        
        let waiting_receiver = WaitingReceiver {
            result: result_mutex.clone(),
            notify: notify.clone(),
            completed: completed.clone(),
        };
        
        data.waiting_receivers.push_back(waiting_receiver);
        
        // Release lock and wait with timeout
        drop(data);
        
        let start_time = Instant::now();
        let mut guard = data_arc.lock().unwrap();
        
        while !completed.load(Ordering::Acquire) && start_time.elapsed() < timeout {
            // Check if channel is closed and drained
            if guard.state != ChannelState::Open 
                && guard.buffer.is_empty() 
                && guard.waiting_senders.is_empty() {
                guard.waiting_receivers.retain(|wr| !Arc::ptr_eq(&wr.completed, &completed));
                return ReceiveResult::Closed;
            }
            
            let remaining = timeout.saturating_sub(start_time.elapsed());
            if remaining.is_zero() {
                break;
            }
            
            let (new_guard, timeout_result) = notify.wait_timeout(guard, remaining).unwrap();
            guard = new_guard;
            
            if timeout_result.timed_out() {
                break;
            }
        }
        
        if completed.load(Ordering::Acquire) {
            let result = result_mutex.lock().unwrap().take()
                .expect("Completed receive should have result");
            info!("receive_timeout completed successfully");
            ReceiveResult::Received(result)
        } else {
            // Timeout occurred, remove from waiting list
            guard.waiting_receivers.retain(|wr| !Arc::ptr_eq(&wr.completed, &completed));
            debug!("receive_timeout timed out");
            ReceiveResult::WouldBlock
        }
    }

    /// Check if channel has pending messages or senders
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        let data_arc = match self.channel.upgrade() {
            Some(arc) => arc,
            None => return true,
        };
        
        let data = data_arc.lock().unwrap();
        data.buffer.is_empty() && data.waiting_senders.is_empty()
    }

    /// Check if channel is closed
    #[instrument(skip(self))]
    pub fn is_closed(&self) -> bool {
        let data_arc = match self.channel.upgrade() {
            Some(arc) => arc,
            None => return true,
        };
        
        let data = data_arc.lock().unwrap();
        data.state != ChannelState::Open
    }

    /// Get number of pending messages in buffer
    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        let data_arc = match self.channel.upgrade() {
            Some(arc) => arc,
            None => return 0,
        };
        
        let data = data_arc.lock().unwrap();
        data.buffer.len()
    }

    /// Increment operation counter for statistics
    fn increment_operation_count(&self) {
        unsafe {
            (*self.operation_count).fetch_add(1, Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::channels::channel::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_unbuffered_send_receive() {
        let (tx, rx) = channel::<i32>();
        
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            tx.send(42).unwrap();
        });
        
        let value = rx.receive().unwrap();
        assert_eq!(value, 42);
        
        handle.join().unwrap();
    }

    #[test]
    fn test_buffered_send_receive() {
        let (tx, rx) = buffered_channel::<i32>(3);
        
        // Send multiple values without blocking
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        tx.send(3).unwrap();
        
        // Receive them back
        assert_eq!(rx.receive().unwrap(), 1);
        assert_eq!(rx.receive().unwrap(), 2);
        assert_eq!(rx.receive().unwrap(), 3);
    }

    #[test]
    fn test_try_send_receive() {
        let (tx, rx) = buffered_channel::<i32>(1);
        
        // Should succeed
        match tx.try_send(42) {
            SendResult::Sent => {},
            _ => panic!("Expected send to succeed"),
        }
        
        // Should block (buffer full)
        match tx.try_send(43) {
            SendResult::WouldBlock(value) => assert_eq!(value, 43),
            _ => panic!("Expected send to block"),
        }
        
        // Should receive
        match rx.try_receive() {
            ReceiveResult::Received(value) => assert_eq!(value, 42),
            _ => panic!("Expected receive to succeed"),
        }
        
        // Should block (buffer empty)
        match rx.try_receive() {
            ReceiveResult::WouldBlock => {},
            _ => panic!("Expected receive to block"),
        }
    }

    #[test]
    fn test_channel_close() {
        let (tx, rx) = channel::<i32>();
        
        tx.close().unwrap();
        
        // Send should fail
        assert!(tx.send(42).is_err());
        
        // Receive should fail (closed and empty)
        assert!(rx.receive().is_err());
    }

    #[test]
    fn test_timeout_operations() {
        let (tx, rx) = channel::<i32>();
        
        // Receive timeout should timeout
        let start = Instant::now();
        match rx.receive_timeout(Duration::from_millis(50)) {
            ReceiveResult::WouldBlock => {},
            _ => panic!("Expected timeout"),
        }
        assert!(start.elapsed() >= Duration::from_millis(40));
        
        // Send timeout should timeout  
        let start = Instant::now();
        match tx.send_timeout(42, Duration::from_millis(50)) {
            SendResult::WouldBlock(value) => assert_eq!(value, 42),
            _ => panic!("Expected timeout"),
        }
        assert!(start.elapsed() >= Duration::from_millis(40));
    }

    #[test]
    fn test_multiple_senders_receivers() {
        let (tx, rx) = buffered_channel::<i32>(10);
        
        let tx1 = tx.clone();
        let tx2 = tx.clone();
        let rx1 = rx.clone();
        let rx2 = rx.clone();
        
        let sender1 = thread::spawn(move || {
            for i in 0..5 {
                tx1.send(i).unwrap();
            }
        });
        
        let sender2 = thread::spawn(move || {
            for i in 5..10 {
                tx2.send(i).unwrap();
            }
        });
        
        let receiver1 = thread::spawn(move || {
            let mut values = Vec::new();
            for _ in 0..5 {
                values.push(rx1.receive().unwrap());
            }
            values
        });
        
        let receiver2 = thread::spawn(move || {
            let mut values = Vec::new();
            for _ in 0..5 {
                values.push(rx2.receive().unwrap());
            }
            values
        });
        
        sender1.join().unwrap();
        sender2.join().unwrap();
        
        let mut all_values = receiver1.join().unwrap();
        all_values.extend(receiver2.join().unwrap());
        all_values.sort();
        
        assert_eq!(all_values, (0..10).collect::<Vec<_>>());
    }
}
