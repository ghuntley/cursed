/// Advanced synchronization primitives for IPC
/// 
/// This module provides comprehensive synchronization mechanisms for inter-process
/// communication including barriers, read-write locks, condition variables, and
/// coordination primitives.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Condvar};
use std::time::{Duration, Instant, SystemTime};
use std::thread;

use crate::stdlib::ipc::{
    IpcResult, IpcError, IpcHandle, ProcessId, Semaphore,
    timeout_error, communication_error, resource_error, invalid_operation
};

/// Inter-process barrier for coordinating multiple processes
#[derive(Debug)]
pub struct IpcBarrier {
    name: String,
    expected_count: usize,
    current_count: Arc<Mutex<usize>>,
    generation: Arc<Mutex<u64>>,
    condvar: Arc<Condvar>,
    timeout: Duration,
}

/// Inter-process read-write lock
#[derive(Debug)]
pub struct IpcRwLock {
    name: String,
    readers_sem: Semaphore,
    writers_sem: Semaphore,
    read_count_sem: Semaphore,
    shared_data: Arc<Mutex<RwLockData>>,
}

#[derive(Debug)]
struct RwLockData {
    readers: u32,
    writers: u32,
    writer_waiting: bool,
}

/// Inter-process condition variable
#[derive(Debug)]
pub struct IpcCondVar {
    name: String,
    waiters: Arc<Mutex<Vec<ProcessId>>>,
    notifications: Arc<Mutex<HashMap<ProcessId, bool>>>,
}

/// Process coordination manager
#[derive(Debug)]
pub struct ProcessCoordinator {
    barriers: Arc<Mutex<HashMap<String, IpcBarrier>>>,
    rwlocks: Arc<Mutex<HashMap<String, IpcRwLock>>>,
    condvars: Arc<Mutex<HashMap<String, IpcCondVar>>>,
    cleanup_thread: Option<thread::JoinHandle<()>>,
    shutdown_flag: Arc<Mutex<bool>>,
}

/// Barrier wait result
#[derive(Debug, Clone, PartialEq)]
pub enum BarrierWaitResult {
    /// This process was the last to reach the barrier
    Leader,
    /// This process was a follower
    Follower,
}

impl IpcBarrier {
    /// Create a new inter-process barrier
    pub fn new(name: &str, expected_count: usize, timeout: Duration) -> IpcResult<Self> {
        if expected_count == 0 {
            return Err(invalid_operation("Barrier count must be greater than 0"));
        }

        Ok(Self {
            name: name.to_string(),
            expected_count,
            current_count: Arc::new(Mutex::new(0)),
            generation: Arc::new(Mutex::new(0)),
            condvar: Arc::new(Condvar::new()),
            timeout,
        })
    }

    /// Wait for all processes to reach the barrier
    pub fn wait(&self) -> IpcResult<BarrierWaitResult> {
        let start_time = Instant::now();
        
        let (current_gen, is_leader) = {
            let mut count = self.current_count.lock().unwrap();
            let mut gen = self.generation.lock().unwrap();
            
            *count += 1;
            let current_gen = *gen;
            
            if *count >= self.expected_count {
                // Last process to arrive - reset for next use
                *count = 0;
                *gen += 1;
                (current_gen, true)
            } else {
                (current_gen, false)
            }
        };

        if is_leader {
            // Notify all waiting processes
            self.condvar.notify_all();
            return Ok(BarrierWaitResult::Leader);
        }

        // Wait for barrier to be reached
        let mut gen = self.generation.lock().unwrap();
        while *gen == current_gen {
            let remaining_time = self.timeout.saturating_sub(start_time.elapsed());
            if remaining_time.is_zero() {
                return Err(timeout_error("Barrier wait timeout"));
            }

            let (new_gen, timeout_result) = self.condvar
                .wait_timeout(gen, remaining_time)
                .unwrap();
            
            gen = new_gen;
            
            if timeout_result.timed_out() {
                return Err(timeout_error("Barrier wait timeout"));
            }
        }

        Ok(BarrierWaitResult::Follower)
    }

    /// Get the number of processes currently waiting at the barrier
    pub fn waiting_count(&self) -> usize {
        *self.current_count.lock().unwrap()
    }

    /// Get the expected total number of processes
    pub fn expected_count(&self) -> usize {
        self.expected_count
    }

    /// Get the current generation (number of times barrier has been used)
    pub fn generation(&self) -> u64 {
        *self.generation.lock().unwrap()
    }
}

impl IpcRwLock {
    /// Create a new inter-process read-write lock
    pub fn new(name: &str) -> IpcResult<Self> {
        let readers_sem = Semaphore::create(&format!("{}_readers", name), 1)?;
        let writers_sem = Semaphore::create(&format!("{}_writers", name), 1)?;
        let read_count_sem = Semaphore::create(&format!("{}_read_count", name), 1)?;

        let shared_data = RwLockData {
            readers: 0,
            writers: 0,
            writer_waiting: false,
        };

        Ok(Self {
            name: name.to_string(),
            readers_sem,
            writers_sem,
            read_count_sem,
            shared_data: Arc::new(Mutex::new(shared_data)),
        })
    }

    /// Acquire a read lock
    pub fn read_lock(&self) -> IpcResult<IpcRwLockReadGuard> {
        self.read_lock_timeout(Duration::from_secs(30))
    }

    /// Acquire a read lock with timeout
    pub fn read_lock_timeout(&self, timeout: Duration) -> IpcResult<IpcRwLockReadGuard> {
        let start_time = Instant::now();

        // Check if writers are waiting/active
        {
            let data = self.shared_data.lock().unwrap();
            if data.writers > 0 || data.writer_waiting {
                if start_time.elapsed() >= timeout {
                    return Err(timeout_error("Read lock timeout"));
                }
                // Wait briefly and try again
                drop(data);
                thread::sleep(Duration::from_millis(1));
            }
        }

        // Acquire read count semaphore
        self.read_count_sem.acquire_timeout(timeout)?;

        // Increment reader count
        {
            let mut data = self.shared_data.lock().unwrap();
            data.readers += 1;
            
            // If this is the first reader, acquire the writers semaphore
            if data.readers == 1 {
                self.writers_sem.acquire_timeout(timeout)?;
            }
        }

        // Release read count semaphore
        self.read_count_sem.release()?;

        Ok(IpcRwLockReadGuard {
            lock: self,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Acquire a write lock
    pub fn write_lock(&self) -> IpcResult<IpcRwLockWriteGuard> {
        self.write_lock_timeout(Duration::from_secs(30))
    }

    /// Acquire a write lock with timeout
    pub fn write_lock_timeout(&self, timeout: Duration) -> IpcResult<IpcRwLockWriteGuard> {
        // Mark that a writer is waiting
        {
            let mut data = self.shared_data.lock().unwrap();
            data.writer_waiting = true;
        }

        // Acquire readers semaphore (blocks new readers)
        self.readers_sem.acquire_timeout(timeout)?;

        // Acquire writers semaphore (ensures exclusive access)
        self.writers_sem.acquire_timeout(timeout)?;

        // Update state
        {
            let mut data = self.shared_data.lock().unwrap();
            data.writers += 1;
            data.writer_waiting = false;
        }

        Ok(IpcRwLockWriteGuard {
            lock: self,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Try to acquire a read lock without blocking
    pub fn try_read_lock(&self) -> IpcResult<Option<IpcRwLockReadGuard>> {
        match self.read_lock_timeout(Duration::from_nanos(1)) {
            Ok(guard) => Ok(Some(guard)),
            Err(IpcError::TimeoutError { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Try to acquire a write lock without blocking
    pub fn try_write_lock(&self) -> IpcResult<Option<IpcRwLockWriteGuard>> {
        match self.write_lock_timeout(Duration::from_nanos(1)) {
            Ok(guard) => Ok(Some(guard)),
            Err(IpcError::TimeoutError { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn release_read_lock(&self) -> IpcResult<()> {
        self.read_count_sem.acquire()?;
        
        let should_release_writers = {
            let mut data = self.shared_data.lock().unwrap();
            data.readers -= 1;
            data.readers == 0
        };

        if should_release_writers {
            self.writers_sem.release()?;
        }

        self.read_count_sem.release()?;
        Ok(())
    }

    fn release_write_lock(&self) -> IpcResult<()> {
        {
            let mut data = self.shared_data.lock().unwrap();
            data.writers -= 1;
        }

        self.writers_sem.release()?;
        self.readers_sem.release()?;
        Ok(())
    }
}

/// Read lock guard for IpcRwLock
pub struct IpcRwLockReadGuard<'a> {
    lock: &'a IpcRwLock,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for IpcRwLockReadGuard<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.lock.release_read_lock() {
            eprintln!("Error releasing read lock: {}", e);
        }
    }
}

/// Write lock guard for IpcRwLock
pub struct IpcRwLockWriteGuard<'a> {
    lock: &'a IpcRwLock,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for IpcRwLockWriteGuard<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.lock.release_write_lock() {
            eprintln!("Error releasing write lock: {}", e);
        }
    }
}

impl IpcCondVar {
    /// Create a new inter-process condition variable
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            waiters: Arc::new(Mutex::new(Vec::new())),
            notifications: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Wait for a notification
    pub fn wait(&self, process_id: ProcessId) -> IpcResult<()> {
        self.wait_timeout(process_id, Duration::from_secs(u64::MAX))
    }

    /// Wait for a notification with timeout
    pub fn wait_timeout(&self, process_id: ProcessId, timeout: Duration) -> IpcResult<()> {
        let start_time = Instant::now();

        // Add to waiters list
        {
            let mut waiters = self.waiters.lock().unwrap();
            if !waiters.contains(&process_id) {
                waiters.push(process_id);
            }
        }

        // Wait for notification
        while start_time.elapsed() < timeout {
            {
                let mut notifications = self.notifications.lock().unwrap();
                if notifications.remove(&process_id).unwrap_or(false) {
                    // Remove from waiters list
                    let mut waiters = self.waiters.lock().unwrap();
                    waiters.retain(|&pid| pid != process_id);
                    return Ok(());
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }

        // Timeout - remove from waiters list
        {
            let mut waiters = self.waiters.lock().unwrap();
            waiters.retain(|&pid| pid != process_id);
        }

        Err(timeout_error("Condition variable wait timeout"))
    }

    /// Notify one waiting process
    pub fn notify_one(&self) -> IpcResult<bool> {
        let waiters = self.waiters.lock().unwrap();
        if let Some(&process_id) = waiters.first() {
            let mut notifications = self.notifications.lock().unwrap();
            notifications.insert(process_id, true);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Notify all waiting processes
    pub fn notify_all(&self) -> IpcResult<usize> {
        let waiters = self.waiters.lock().unwrap();
        let count = waiters.len();
        
        {
            let mut notifications = self.notifications.lock().unwrap();
            for &process_id in waiters.iter() {
                notifications.insert(process_id, true);
            }
        }

        Ok(count)
    }

    /// Get the number of waiting processes
    pub fn waiting_count(&self) -> usize {
        self.waiters.lock().unwrap().len()
    }
}

impl ProcessCoordinator {
    /// Create a new process coordinator
    pub fn new() -> Self {
        let barriers = Arc::new(Mutex::new(HashMap::new()));
        let rwlocks = Arc::new(Mutex::new(HashMap::new()));
        let condvars = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_flag = Arc::new(Mutex::new(false));

        // Start cleanup thread
        let cleanup_barriers = barriers.clone();
        let cleanup_rwlocks = rwlocks.clone();
        let cleanup_condvars = condvars.clone();
        let cleanup_shutdown = shutdown_flag.clone();

        let cleanup_handle = thread::spawn(move || {
            Self::cleanup_loop(cleanup_barriers, cleanup_rwlocks, cleanup_condvars, cleanup_shutdown);
        });

        Self {
            barriers,
            rwlocks,
            condvars,
            cleanup_thread: Some(cleanup_handle),
            shutdown_flag,
        }
    }

    /// Get or create a barrier
    pub fn get_barrier(&self, name: &str, expected_count: usize, timeout: Duration) -> IpcResult<Arc<IpcBarrier>> {
        let mut barriers = self.barriers.lock().unwrap();
        
        if let Some(barrier) = barriers.get(name) {
            Ok(Arc::new(barrier.clone()))
        } else {
            let barrier = IpcBarrier::new(name, expected_count, timeout)?;
            let barrier_arc = Arc::new(barrier.clone());
            barriers.insert(name.to_string(), barrier);
            Ok(barrier_arc)
        }
    }

    /// Get or create a read-write lock
    pub fn get_rwlock(&self, name: &str) -> IpcResult<Arc<IpcRwLock>> {
        let mut rwlocks = self.rwlocks.lock().unwrap();
        
        if let Some(rwlock) = rwlocks.get(name) {
            Ok(Arc::new(rwlock.clone()))
        } else {
            let rwlock = IpcRwLock::new(name)?;
            let rwlock_arc = Arc::new(rwlock.clone());
            rwlocks.insert(name.to_string(), rwlock);
            Ok(rwlock_arc)
        }
    }

    /// Get or create a condition variable
    pub fn get_condvar(&self, name: &str) -> Arc<IpcCondVar> {
        let mut condvars = self.condvars.lock().unwrap();
        
        if let Some(condvar) = condvars.get(name) {
            Arc::new(condvar.clone())
        } else {
            let condvar = IpcCondVar::new(name);
            let condvar_arc = Arc::new(condvar.clone());
            condvars.insert(name.to_string(), condvar);
            condvar_arc
        }
    }

    /// Remove a barrier
    pub fn remove_barrier(&self, name: &str) -> bool {
        let mut barriers = self.barriers.lock().unwrap();
        barriers.remove(name).is_some()
    }

    /// Remove a read-write lock
    pub fn remove_rwlock(&self, name: &str) -> bool {
        let mut rwlocks = self.rwlocks.lock().unwrap();
        rwlocks.remove(name).is_some()
    }

    /// Remove a condition variable
    pub fn remove_condvar(&self, name: &str) -> bool {
        let mut condvars = self.condvars.lock().unwrap();
        condvars.remove(name).is_some()
    }

    /// Get statistics about managed synchronization primitives
    pub fn get_statistics(&self) -> CoordinatorStatistics {
        let barriers = self.barriers.lock().unwrap();
        let rwlocks = self.rwlocks.lock().unwrap();
        let condvars = self.condvars.lock().unwrap();

        CoordinatorStatistics {
            active_barriers: barriers.len(),
            active_rwlocks: rwlocks.len(),
            active_condvars: condvars.len(),
            total_waiting_processes: condvars.values().map(|cv| cv.waiting_count()).sum(),
        }
    }

    /// Shutdown the coordinator
    pub fn shutdown(&mut self) -> IpcResult<()> {
        {
            let mut shutdown = self.shutdown_flag.lock().unwrap();
            *shutdown = true;
        }

        if let Some(handle) = self.cleanup_thread.take() {
            handle.join().map_err(|_| {
                resource_error("Failed to join cleanup thread")
            })?;
        }

        Ok(())
    }

    fn cleanup_loop(
        barriers: Arc<Mutex<HashMap<String, IpcBarrier>>>,
        _rwlocks: Arc<Mutex<HashMap<String, IpcRwLock>>>,
        condvars: Arc<Mutex<HashMap<String, IpcCondVar>>>,
        shutdown_flag: Arc<Mutex<bool>>,
    ) {
        while !*shutdown_flag.lock().unwrap() {
            // Clean up condition variables with no waiters
            {
                let mut condvars_map = condvars.lock().unwrap();
                condvars_map.retain(|_, condvar| condvar.waiting_count() > 0);
            }

            // Clean up old notifications
            {
                let condvars_map = condvars.lock().unwrap();
                for condvar in condvars_map.values() {
                    let mut notifications = condvar.notifications.lock().unwrap();
                    // In a real implementation, you might want to clean up old notifications
                    // based on timestamp or other criteria
                    if notifications.len() > 1000 {
                        notifications.clear();
                    }
                }
            }

            thread::sleep(Duration::from_secs(10));
        }
    }
}

impl Default for ProcessCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ProcessCoordinator {
    fn drop(&mut self) {
        if let Err(e) = self.shutdown() {
            eprintln!("Error during ProcessCoordinator shutdown: {}", e);
        }
    }
}

/// Statistics for the process coordinator
#[derive(Debug, Clone)]
pub struct CoordinatorStatistics {
    pub active_barriers: usize,
    pub active_rwlocks: usize,
    pub active_condvars: usize,
    pub total_waiting_processes: usize,
}

// Re-implement Clone for IpcBarrier (simple fields only)
impl Clone for IpcBarrier {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            expected_count: self.expected_count,
            current_count: Arc::new(Mutex::new(*self.current_count.lock().unwrap())),
            generation: Arc::new(Mutex::new(*self.generation.lock().unwrap())),
            condvar: Arc::new(Condvar::new()),
            timeout: self.timeout,
        }
    }
}

// Re-implement Clone for IpcRwLock (this is a simplified version)
impl Clone for IpcRwLock {
    fn clone(&self) -> Self {
        // In a real implementation, you'd want to share the underlying semaphores
        // This is a simplified version for demonstration
        Self::new(&self.name).unwrap()
    }
}

// Re-implement Clone for IpcCondVar
impl Clone for IpcCondVar {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            waiters: Arc::new(Mutex::new(self.waiters.lock().unwrap().clone())),
            notifications: Arc::new(Mutex::new(self.notifications.lock().unwrap().clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_barrier_creation() {
        let barrier = IpcBarrier::new("test_barrier", 3, Duration::from_secs(10)).unwrap();
        assert_eq!(barrier.expected_count(), 3);
        assert_eq!(barrier.waiting_count(), 0);
        assert_eq!(barrier.generation(), 0);
    }

    #[test]
    fn test_barrier_single_process() {
        let barrier = IpcBarrier::new("test_single", 1, Duration::from_secs(1)).unwrap();
        let result = barrier.wait().unwrap();
        assert_eq!(result, BarrierWaitResult::Leader);
        assert_eq!(barrier.generation(), 1);
    }

    #[test]
    fn test_rwlock_creation() {
        let _rwlock = IpcRwLock::new("test_rwlock").unwrap();
    }

    #[test]
    fn test_condvar_creation() {
        let condvar = IpcCondVar::new("test_condvar");
        assert_eq!(condvar.waiting_count(), 0);
    }

    #[test]
    fn test_condvar_notify() {
        let condvar = IpcCondVar::new("test_notify");
        
        // No waiters, should return false
        assert_eq!(condvar.notify_one().unwrap(), false);
        assert_eq!(condvar.notify_all().unwrap(), 0);
    }

    #[test]
    fn test_process_coordinator() {
        let coordinator = ProcessCoordinator::new();
        
        let barrier = coordinator.get_barrier("test", 2, Duration::from_secs(10)).unwrap();
        assert_eq!(barrier.expected_count(), 2);
        
        let condvar = coordinator.get_condvar("test_cv");
        assert_eq!(condvar.waiting_count(), 0);
        
        let stats = coordinator.get_statistics();
        assert_eq!(stats.active_barriers, 1);
        assert_eq!(stats.active_condvars, 1);
    }

    #[test]
    fn test_coordinator_cleanup() {
        let coordinator = ProcessCoordinator::new();
        
        let _barrier = coordinator.get_barrier("cleanup_test", 2, Duration::from_secs(10)).unwrap();
        assert!(coordinator.remove_barrier("cleanup_test"));
        assert!(!coordinator.remove_barrier("nonexistent"));
    }
}
