//! Channel synchronization primitives
//!
//! Provides synchronization mechanisms for channel operations:
//! - Goroutine blocking and unblocking
//! - Wait queues for channel operations
//! - Priority-based scheduling
//! - Deadlock detection and prevention

use std::collections::{HashMap, VecDeque, BinaryHeap};
use std::sync::{Arc, Mutex, Condvar, RwLock};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::cmp::Ordering as CmpOrdering;

use crate::runtime::goroutine::{GoroutineId, GoroutineState};
use crate::runtime::channels::{ChannelError, ChannelResult};
use crate::runtime::channels::operations::OperationPriority;

/// Channel synchronization primitive types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncPrimitiveType {
    /// Mutex-like exclusive access
    Mutex,
    /// Condition variable for waiting
    Condition,
    /// Semaphore for counting
    Semaphore,
    /// Barrier for synchronization points
    Barrier,
}

/// Wait queue entry for blocked goroutines
#[derive(Debug, Clone)]
pub struct WaitQueueEntry {
    /// Goroutine ID
    pub goroutine_id: GoroutineId,
    /// Operation priority
    pub priority: OperationPriority,
    /// Wait start time
    pub wait_start: Instant,
    /// Timeout deadline
    pub timeout: Option<Instant>,
    /// Operation type
    pub operation_type: WaitOperationType,
}

/// Type of operation that's waiting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitOperationType {
    /// Waiting to send
    Send,
    /// Waiting to receive
    Receive,
    /// Waiting for select operation
    Select,
}

impl PartialOrd for WaitQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Ord for WaitQueueEntry {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        // Higher priority comes first
        match self.priority.cmp(&other.priority) {
            CmpOrdering::Equal => {
                // Earlier wait time comes first (FIFO within same priority)
                self.wait_start.cmp(&other.wait_start)
            }
            other => other,
        }
    }
}

impl PartialEq for WaitQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.goroutine_id == other.goroutine_id
    }
}

impl Eq for WaitQueueEntry {}

/// Priority wait queue for goroutines
pub struct PriorityWaitQueue {
    /// Priority heap of waiting goroutines
    queue: Mutex<BinaryHeap<WaitQueueEntry>>,
    /// Condition variable for notifications
    notify: Condvar,
    /// Queue statistics
    stats: Mutex<WaitQueueStats>,
}

/// Wait queue statistics
#[derive(Debug, Clone, Default)]
pub struct WaitQueueStats {
    /// Total entries added
    pub total_enqueued: usize,
    /// Total entries removed
    pub total_dequeued: usize,
    /// Current queue length
    pub current_length: usize,
    /// Peak queue length
    pub peak_length: usize,
    /// Average wait time
    pub average_wait_time: Duration,
    /// Total wait time
    pub total_wait_time: Duration,
}

impl PriorityWaitQueue {
    /// Create a new priority wait queue
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(BinaryHeap::new()),
            notify: Condvar::new(),
            stats: Mutex::new(WaitQueueStats::default()),
        }
    }
    
    /// Add a goroutine to the wait queue
    pub fn enqueue(&self, entry: WaitQueueEntry) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(entry);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_enqueued += 1;
            stats.current_length = queue.len();
            if stats.current_length > stats.peak_length {
                stats.peak_length = stats.current_length;
            }
        }
        
        // Notify waiting threads
        self.notify.notify_one();
    }
    
    /// Remove the highest priority goroutine from the queue
    pub fn dequeue(&self) -> Option<WaitQueueEntry> {
        let mut queue = self.queue.lock().unwrap();
        
        if let Some(entry) = queue.pop() {
            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.total_dequeued += 1;
                stats.current_length = queue.len();
                
                let wait_time = entry.wait_start.elapsed();
                stats.total_wait_time += wait_time;
                let total_processed = stats.total_dequeued;
                stats.average_wait_time = stats.total_wait_time / total_processed as u32;
            }
            
            Some(entry)
        } else {
            None
        }
    }
    
    /// Wait for an entry to become available
    pub fn wait_for_entry(&self) -> Option<WaitQueueEntry> {
        let mut queue = self.queue.lock().unwrap();
        
        while queue.is_empty() {
            queue = self.notify.wait(queue).unwrap();
        }
        
        self.dequeue()
    }
    
    /// Wait for an entry with timeout
    pub fn wait_for_entry_timeout(&self, timeout: Duration) -> Option<WaitQueueEntry> {
        let mut queue = self.queue.lock().unwrap();
        
        if queue.is_empty() {
            let (_guard, result) = self.notify.wait_timeout(queue, timeout).unwrap();
            if result.timed_out() {
                return None;
            }
        }
        
        self.dequeue()
    }
    
    /// Remove a specific goroutine from the queue
    pub fn remove_goroutine(&self, goroutine_id: GoroutineId) -> bool {
        let mut queue = self.queue.lock().unwrap();
        
        // Convert to vec, remove entry, and rebuild heap
        let mut entries: Vec<_> = queue.drain().collect();
        let initial_len = entries.len();
        
        entries.retain(|entry| entry.goroutine_id != goroutine_id);
        
        // Rebuild the heap
        *queue = entries.into_iter().collect();
        
        // Update statistics if something was removed
        let removed = initial_len != queue.len();
        if removed {
            let mut stats = self.stats.lock().unwrap();
            stats.current_length = queue.len();
        }
        
        removed
    }
    
    /// Get current queue length
    pub fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }
    
    /// Get queue statistics
    pub fn stats(&self) -> WaitQueueStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Clear all entries and return them
    pub fn clear(&self) -> Vec<WaitQueueEntry> {
        let mut queue = self.queue.lock().unwrap();
        let entries: Vec<_> = queue.drain().collect();
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.current_length = 0;
        }
        
        entries
    }
}

/// Channel synchronization manager
pub struct ChannelSync {
    /// Send wait queues by channel ID
    send_queues: RwLock<HashMap<usize, Arc<PriorityWaitQueue>>>,
    /// Receive wait queues by channel ID
    receive_queues: RwLock<HashMap<usize, Arc<PriorityWaitQueue>>>,
    /// Select wait queues by select ID
    select_queues: RwLock<HashMap<usize, Arc<PriorityWaitQueue>>>,
    /// Goroutine state tracking
    goroutine_states: RwLock<HashMap<GoroutineId, GoroutineWaitState>>,
    /// Deadlock detector
    deadlock_detector: Arc<DeadlockDetector>,
    /// Synchronization statistics
    stats: Mutex<ChannelSyncStats>,
}

/// Goroutine wait state
#[derive(Debug, Clone)]
pub struct GoroutineWaitState {
    /// Current wait operation
    pub operation: WaitOperationType,
    /// Channel being waited on
    pub channel_id: Option<usize>,
    /// Select operation ID
    pub select_id: Option<usize>,
    /// Wait start time
    pub wait_start: Instant,
    /// Timeout deadline
    pub timeout: Option<Instant>,
}

/// Channel synchronization statistics
#[derive(Debug, Clone, Default)]
pub struct ChannelSyncStats {
    /// Total send operations blocked
    pub sends_blocked: usize,
    /// Total receive operations blocked
    pub receives_blocked: usize,
    /// Total select operations blocked
    pub selects_blocked: usize,
    /// Total goroutines unblocked
    pub goroutines_unblocked: usize,
    /// Total deadlocks detected
    pub deadlocks_detected: usize,
    /// Average block time
    pub average_block_time: Duration,
}

impl ChannelSync {
    /// Create a new channel synchronization manager
    pub fn new() -> Self {
        Self {
            send_queues: RwLock::new(HashMap::new()),
            receive_queues: RwLock::new(HashMap::new()),
            select_queues: RwLock::new(HashMap::new()),
            goroutine_states: RwLock::new(HashMap::new()),
            deadlock_detector: Arc::new(DeadlockDetector::new()),
            stats: Mutex::new(ChannelSyncStats::default()),
        }
    }
    
    /// Block a goroutine on a send operation
    pub fn block_on_send(
        &self,
        channel_id: usize,
        goroutine_id: GoroutineId,
        priority: OperationPriority,
        timeout: Option<Duration>,
    ) -> ChannelResult<()> {
        let wait_entry = WaitQueueEntry {
            goroutine_id,
            priority,
            wait_start: Instant::now(),
            timeout: timeout.map(|t| Instant::now() + t),
            operation_type: WaitOperationType::Send,
        };
        
        // Get or create send queue for channel
        let queue = {
            let mut queues = self.send_queues.write().unwrap();
            queues.entry(channel_id)
                .or_insert_with(|| Arc::new(PriorityWaitQueue::new()))
                .clone()
        };
        
        // Record goroutine state
        {
            let mut states = self.goroutine_states.write().unwrap();
            states.insert(goroutine_id, GoroutineWaitState {
                operation: WaitOperationType::Send,
                channel_id: Some(channel_id),
                select_id: None,
                wait_start: wait_entry.wait_start,
                timeout: wait_entry.timeout,
            });
        }
        
        // Check for potential deadlock
        self.deadlock_detector.check_for_deadlock(goroutine_id, channel_id)?;
        
        // Add to wait queue
        queue.enqueue(wait_entry);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.sends_blocked += 1;
        }
        
        Ok(())
    }
    
    /// Block a goroutine on a receive operation
    pub fn block_on_receive(
        &self,
        channel_id: usize,
        goroutine_id: GoroutineId,
        priority: OperationPriority,
        timeout: Option<Duration>,
    ) -> ChannelResult<()> {
        let wait_entry = WaitQueueEntry {
            goroutine_id,
            priority,
            wait_start: Instant::now(),
            timeout: timeout.map(|t| Instant::now() + t),
            operation_type: WaitOperationType::Receive,
        };
        
        // Get or create receive queue for channel
        let queue = {
            let mut queues = self.receive_queues.write().unwrap();
            queues.entry(channel_id)
                .or_insert_with(|| Arc::new(PriorityWaitQueue::new()))
                .clone()
        };
        
        // Record goroutine state
        {
            let mut states = self.goroutine_states.write().unwrap();
            states.insert(goroutine_id, GoroutineWaitState {
                operation: WaitOperationType::Receive,
                channel_id: Some(channel_id),
                select_id: None,
                wait_start: wait_entry.wait_start,
                timeout: wait_entry.timeout,
            });
        }
        
        // Check for potential deadlock
        self.deadlock_detector.check_for_deadlock(goroutine_id, channel_id)?;
        
        // Add to wait queue
        queue.enqueue(wait_entry);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.receives_blocked += 1;
        }
        
        Ok(())
    }
    
    /// Unblock the next waiting sender for a channel
    pub fn unblock_sender(&self, channel_id: usize) -> Option<GoroutineId> {
        let queue = {
            let queues = self.send_queues.read().unwrap();
            queues.get(&channel_id).cloned()
        };
        
        if let Some(queue) = queue {
            if let Some(entry) = queue.dequeue() {
                // Remove from goroutine states
                {
                    let mut states = self.goroutine_states.write().unwrap();
                    if let Some(state) = states.remove(&entry.goroutine_id) {
                        // Update statistics
                        let mut stats = self.stats.lock().unwrap();
                        stats.goroutines_unblocked += 1;
                        
                        let block_time = state.wait_start.elapsed();
                        stats.average_block_time = 
                            (stats.average_block_time + block_time) / 2;
                    }
                }
                
                return Some(entry.goroutine_id);
            }
        }
        
        None
    }
    
    /// Unblock the next waiting receiver for a channel
    pub fn unblock_receiver(&self, channel_id: usize) -> Option<GoroutineId> {
        let queue = {
            let queues = self.receive_queues.read().unwrap();
            queues.get(&channel_id).cloned()
        };
        
        if let Some(queue) = queue {
            if let Some(entry) = queue.dequeue() {
                // Remove from goroutine states
                {
                    let mut states = self.goroutine_states.write().unwrap();
                    if let Some(state) = states.remove(&entry.goroutine_id) {
                        // Update statistics
                        let mut stats = self.stats.lock().unwrap();
                        stats.goroutines_unblocked += 1;
                        
                        let block_time = state.wait_start.elapsed();
                        stats.average_block_time = 
                            (stats.average_block_time + block_time) / 2;
                    }
                }
                
                return Some(entry.goroutine_id);
            }
        }
        
        None
    }
    
    /// Unblock a specific goroutine
    pub fn unblock_goroutine(&self, goroutine_id: GoroutineId) -> bool {
        let state = {
            let mut states = self.goroutine_states.write().unwrap();
            states.remove(&goroutine_id)
        };
        
        if let Some(state) = state {
            match state.operation {
                WaitOperationType::Send => {
                    if let Some(channel_id) = state.channel_id {
                        let queues = self.send_queues.read().unwrap();
                        if let Some(queue) = queues.get(&channel_id) {
                            queue.remove_goroutine(goroutine_id);
                        }
                    }
                }
                WaitOperationType::Receive => {
                    if let Some(channel_id) = state.channel_id {
                        let queues = self.receive_queues.read().unwrap();
                        if let Some(queue) = queues.get(&channel_id) {
                            queue.remove_goroutine(goroutine_id);
                        }
                    }
                }
                WaitOperationType::Select => {
                    if let Some(select_id) = state.select_id {
                        let queues = self.select_queues.read().unwrap();
                        if let Some(queue) = queues.get(&select_id) {
                            queue.remove_goroutine(goroutine_id);
                        }
                    }
                }
            }
            
            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.goroutines_unblocked += 1;
            }
            
            return true;
        }
        
        false
    }
    
    /// Get waiting goroutines for a channel
    pub fn get_waiting_senders(&self, channel_id: usize) -> Vec<WaitQueueEntry> {
        let queues = self.send_queues.read().unwrap();
        if let Some(queue) = queues.get(&channel_id) {
            // This is a simple implementation - would need better access to queue contents
            Vec::new()
        } else {
            Vec::new()
        }
    }
    
    /// Get waiting receivers for a channel
    pub fn get_waiting_receivers(&self, channel_id: usize) -> Vec<WaitQueueEntry> {
        let queues = self.receive_queues.read().unwrap();
        if let Some(queue) = queues.get(&channel_id) {
            // This is a simple implementation - would need better access to queue contents
            Vec::new()
        } else {
            Vec::new()
        }
    }
    
    /// Get synchronization statistics
    pub fn stats(&self) -> ChannelSyncStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Clean up closed channels
    pub fn cleanup_channel(&self, channel_id: usize) {
        // Unblock all waiting goroutines
        let send_entries = {
            let mut queues = self.send_queues.write().unwrap();
            if let Some(queue) = queues.remove(&channel_id) {
                queue.clear()
            } else {
                Vec::new()
            }
        };
        
        let receive_entries = {
            let mut queues = self.receive_queues.write().unwrap();
            if let Some(queue) = queues.remove(&channel_id) {
                queue.clear()
            } else {
                Vec::new()
            }
        };
        
        // Remove goroutine states
        {
            let mut states = self.goroutine_states.write().unwrap();
            for entry in send_entries.iter().chain(receive_entries.iter()) {
                states.remove(&entry.goroutine_id);
            }
        }
    }
}

/// Deadlock detection system
pub struct DeadlockDetector {
    /// Dependency graph: goroutine -> channel dependencies
    dependencies: RwLock<HashMap<GoroutineId, Vec<usize>>>,
    /// Channel ownership: channel -> goroutine
    channel_owners: RwLock<HashMap<usize, GoroutineId>>,
    /// Detection enabled flag
    enabled: AtomicBool,
}

impl DeadlockDetector {
    /// Create a new deadlock detector
    pub fn new() -> Self {
        Self {
            dependencies: RwLock::new(HashMap::new()),
            channel_owners: RwLock::new(HashMap::new()),
            enabled: AtomicBool::new(true),
        }
    }
    
    /// Check for potential deadlock
    pub fn check_for_deadlock(&self, goroutine_id: GoroutineId, channel_id: usize) -> ChannelResult<()> {
        if !self.enabled.load(Ordering::Relaxed) {
            return Ok(());
        }
        
        // Add dependency
        {
            let mut deps = self.dependencies.write().unwrap();
            deps.entry(goroutine_id)
                .or_insert_with(Vec::new)
                .push(channel_id);
        }
        
        // Check for cycles using DFS
        if self.has_cycle(goroutine_id)? {
            return Err(ChannelError::AllocationError("Potential deadlock detected".to_string()));
        }
        
        Ok(())
    }
    
    /// Check if there's a cycle in the dependency graph
    fn has_cycle(&self, start_goroutine: GoroutineId) -> ChannelResult<bool> {
        let mut visited = std::collections::HashSet::new();
        let mut stack = std::collections::HashSet::new();
        
        self.dfs_visit(start_goroutine, &mut visited, &mut stack)
    }
    
    /// DFS visit for cycle detection
    fn dfs_visit(
        &self,
        goroutine_id: GoroutineId,
        visited: &mut std::collections::HashSet<GoroutineId>,
        stack: &mut std::collections::HashSet<GoroutineId>,
    ) -> ChannelResult<bool> {
        if stack.contains(&goroutine_id) {
            return Ok(true); // Cycle detected
        }
        
        if visited.contains(&goroutine_id) {
            return Ok(false);
        }
        
        visited.insert(goroutine_id);
        stack.insert(goroutine_id);
        
        // Check dependencies
        let deps = self.dependencies.read().unwrap();
        if let Some(channels) = deps.get(&goroutine_id) {
            let owners = self.channel_owners.read().unwrap();
            
            for &channel_id in channels {
                if let Some(&owner_id) = owners.get(&channel_id) {
                    if owner_id != goroutine_id {
                        if self.dfs_visit(owner_id, visited, stack)? {
                            return Ok(true);
                        }
                    }
                }
            }
        }
        
        stack.remove(&goroutine_id);
        Ok(false)
    }
    
    /// Remove goroutine from dependency tracking
    pub fn remove_goroutine(&self, goroutine_id: GoroutineId) {
        let mut deps = self.dependencies.write().unwrap();
        deps.remove(&goroutine_id);
    }
    
    /// Enable or disable deadlock detection
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }
}

/// Global channel synchronization manager
static GLOBAL_CHANNEL_SYNC: once_cell::sync::Lazy<ChannelSync> = 
    once_cell::sync::Lazy::new(|| ChannelSync::new());

/// Get the global channel synchronization manager
pub fn get_global_channel_sync() -> &'static ChannelSync {
    &GLOBAL_CHANNEL_SYNC
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_wait_queue() {
        let queue = PriorityWaitQueue::new();
        
        // Add entries with different priorities
        let low_entry = WaitQueueEntry {
            goroutine_id: 1,
            priority: OperationPriority::Low,
            wait_start: Instant::now(),
            timeout: None,
            operation_type: WaitOperationType::Send,
        };
        
        let high_entry = WaitQueueEntry {
            goroutine_id: 2,
            priority: OperationPriority::High,
            wait_start: Instant::now(),
            timeout: None,
            operation_type: WaitOperationType::Send,
        };
        
        queue.enqueue(low_entry);
        queue.enqueue(high_entry);
        
        // High priority should come out first
        let first = queue.dequeue().unwrap();
        assert_eq!(first.goroutine_id, 2);
        assert_eq!(first.priority, OperationPriority::High);
        
        let second = queue.dequeue().unwrap();
        assert_eq!(second.goroutine_id, 1);
        assert_eq!(second.priority, OperationPriority::Low);
    }

    #[test]
    fn test_channel_sync() {
        let sync = ChannelSync::new();
        
        // Block a goroutine on send
        sync.block_on_send(1, 42, OperationPriority::Normal, None).unwrap();
        
        // Unblock it
        let unblocked = sync.unblock_sender(1);
        assert_eq!(unblocked, Some(42));
    }

    #[test]
    fn test_deadlock_detector() {
        let detector = DeadlockDetector::new();
        
        // Simple case - should not detect deadlock
        assert!(detector.check_for_deadlock(1, 100).is_ok());
        
        detector.remove_goroutine(1);
    }
}
