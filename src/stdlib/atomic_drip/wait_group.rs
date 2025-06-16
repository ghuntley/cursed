use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;
use crate::error::Result as CursedResult;
use super::{MemoryOrder, atomic_error};

/// Atomic wait group for goroutine synchronization
/// Allows one or more goroutines to wait for a collection of other goroutines to finish
#[derive(Debug)]
pub struct WaitGroup {
    counter: AtomicI32,
    waiters: Arc<(Mutex<bool>, Condvar)>,
}

impl WaitGroup {
    /// Create a new wait group with counter initialized to 0
    pub fn new() -> Self {
        Self {
            counter: AtomicI32::new(0),
            waiters: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    /// Add delta to the wait group counter
    /// If delta is positive, it increases the counter
    /// If delta is negative, it decreases the counter
    /// If the counter becomes 0, all waiting goroutines are released
    pub fn add(&self, delta: i32) -> CursedResult<()> {
        let old_count = self.counter.fetch_add(delta, Ordering::SeqCst);
        let new_count = old_count + delta;
        
        if new_count < 0 {
            // Counter went negative, this is an error
            self.counter.store(0, Ordering::SeqCst);
            return Err(atomic_error("WaitGroup counter went negative"));
        }
        
        if new_count == 0 {
            // Counter reached zero, notify all waiters
            let (lock, cvar) = &*self.waiters;
            if let Ok(mut finished) = lock.lock() {
                *finished = true;
                cvar.notify_all();
            }
        }
        
        Ok(())
    }

    /// Decrement the counter by 1
    /// Equivalent to add(-1)
    pub fn done(&self) -> CursedResult<()> {
        self.add(-1)
    }

    /// Wait for the counter to reach 0
    /// Blocks the current goroutine until the counter is 0
    pub fn wait(&self) -> CursedResult<()> {
        // Quick check first
        if self.counter.load(Ordering::SeqCst) == 0 {
            return Ok(());
        }
        
        let (lock, cvar) = &*self.waiters;
        let mut finished = lock.lock().map_err(|_| atomic_error("Failed to acquire wait lock"))?;
        
        while self.counter.load(Ordering::SeqCst) != 0 && !*finished {
            finished = cvar.wait(finished).map_err(|_| atomic_error("Wait condition failed"))?;
        }
        
        Ok(())
    }

    /// Wait for the counter to reach 0 with a timeout
    /// Returns true if the wait completed before timeout, false if it timed out
    pub fn wait_timeout(&self, timeout: Duration) -> CursedResult<bool> {
        // Quick check first
        if self.counter.load(Ordering::SeqCst) == 0 {
            return Ok(true);
        }
        
        let (lock, cvar) = &*self.waiters;
        let mut finished = lock.lock().map_err(|_| atomic_error("Failed to acquire wait lock"))?;
        
        while self.counter.load(Ordering::SeqCst) != 0 && !*finished {
            let (guard, timeout_result) = cvar.wait_timeout(finished, timeout)
                .map_err(|_| atomic_error("Wait timeout condition failed"))?;
            finished = guard;
            
            if timeout_result.timed_out() {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    /// Get the current counter value
    /// This is primarily for debugging and testing
    pub fn count(&self) -> i32 {
        self.counter.load(Ordering::SeqCst)
    }

    /// Reset the wait group to initial state
    /// This is generally not safe to use in concurrent scenarios
    pub fn reset(&self) {
        self.counter.store(0, Ordering::SeqCst);
        let (lock, cvar) = &*self.waiters;
        if let Ok(mut finished) = lock.lock() {
            *finished = false;
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for WaitGroup {}
unsafe impl Sync for WaitGroup {}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        Self {
            counter: AtomicI32::new(self.counter.load(Ordering::SeqCst)),
            waiters: Arc::clone(&self.waiters),
        }
    }
}

/// Create a new wait group
/// Convenience function for creating wait groups
pub fn new_wait_group() -> WaitGroup {
    WaitGroup::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_wait_group_basic() {
        let wg = WaitGroup::new();
        assert_eq!(wg.count(), 0);
        
        wg.add(1).unwrap();
        assert_eq!(wg.count(), 1);
        
        wg.done().unwrap();
        assert_eq!(wg.count(), 0);
    }

    #[test]
    fn test_wait_group_negative_counter() {
        let wg = WaitGroup::new();
        
        // Trying to call done() when counter is 0 should fail
        let result = wg.done();
        assert!(result.is_err());
        assert_eq!(wg.count(), 0);
    }

    #[test]
    fn test_wait_group_concurrent() {
        let wg = Arc::new(WaitGroup::new());
        let wg_clone = Arc::clone(&wg);
        
        // Add work for 3 threads
        wg.add(3).unwrap();
        
        let mut handles = vec![];
        
        for i in 0..3 {
            let wg_ref = Arc::clone(&wg);
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(10 + i * 10));
                wg_ref.done().unwrap();
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        wg_clone.wait().unwrap();
        assert_eq!(wg.count(), 0);
        
        // Clean up threads
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_wait_group_timeout() {
        let wg = WaitGroup::new();
        wg.add(1).unwrap();
        
        // Should timeout since we never call done()
        let result = wg.wait_timeout(Duration::from_millis(50));
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should be false (timed out)
        
        // Now complete the wait
        wg.done().unwrap();
        
        // Should complete immediately
        let result = wg.wait_timeout(Duration::from_millis(50));
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should be true (completed)
    }

    #[test]
    fn test_wait_group_multiple_waiters() {
        let wg = Arc::new(WaitGroup::new());
        wg.add(2).unwrap();
        
        let wg1 = Arc::clone(&wg);
        let wg2 = Arc::clone(&wg);
        let wg3 = Arc::clone(&wg);
        
        let mut completed = vec![false; 2];
        let completed_ptr = Arc::new(Mutex::new(completed));
        
        // Two waiters
        let c1 = Arc::clone(&completed_ptr);
        let waiter1 = thread::spawn(move || {
            wg1.wait().unwrap();
            if let Ok(mut c) = c1.lock() {
                c[0] = true;
            }
        });
        
        let c2 = Arc::clone(&completed_ptr);
        let waiter2 = thread::spawn(move || {
            wg2.wait().unwrap();
            if let Ok(mut c) = c2.lock() {
                c[1] = true;
            }
        });
        
        // Brief delay to ensure waiters are waiting
        thread::sleep(Duration::from_millis(20));
        
        // Complete the work
        wg3.done().unwrap();
        wg3.done().unwrap();
        
        waiter1.join().unwrap();
        waiter2.join().unwrap();
        
        let final_completed = completed_ptr.lock().unwrap();
        assert!(final_completed[0]);
        assert!(final_completed[1]);
        assert_eq!(wg.count(), 0);
    }

    #[test]
    fn test_wait_group_immediate_completion() {
        let wg = WaitGroup::new();
        
        // Should complete immediately since counter is already 0
        let result = wg.wait();
        assert!(result.is_ok());
        
        let result = wg.wait_timeout(Duration::from_millis(10));
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_wait_group_reset() {
        let wg = WaitGroup::new();
        wg.add(5).unwrap();
        assert_eq!(wg.count(), 5);
        
        wg.reset();
        assert_eq!(wg.count(), 0);
        
        // Should be able to use after reset
        wg.add(2).unwrap();
        assert_eq!(wg.count(), 2);
        wg.done().unwrap();
        wg.done().unwrap();
        assert_eq!(wg.count(), 0);
    }

    #[test]
    fn test_wait_group_clone() {
        let wg1 = WaitGroup::new();
        wg1.add(3).unwrap();
        
        let wg2 = wg1.clone();
        assert_eq!(wg2.count(), 3);
        
        wg2.done().unwrap();
        assert_eq!(wg1.count(), 2);
        assert_eq!(wg2.count(), 2);
    }
}
