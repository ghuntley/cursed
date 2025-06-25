use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;
use crate::error::Result as CursedResult;
use super::{MemoryOrder, atomic_error};

/// Atomic wait group for goroutine synchronization
/// Allows one or more goroutines to wait for a collection of other goroutines to finish
#[derive(Debug)]
pub struct WaitGroup {
impl WaitGroup {
    /// Create a new wait group with counter initialized to 0
    pub fn new() -> Self {
        Self {
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
        if new_count == 0 {
            // Counter reached zero, notify all waiters
            let (lock, cvar) = &*self.waiters;
            if let Ok(mut finished) = lock.lock() {
                *finished = true;
                cvar.notify_all();
            }
        }
        
        Ok(())
    /// Decrement the counter by 1
    /// Equivalent to add(-1)
    pub fn done(&self) -> CursedResult<()> {
        self.add(-1)
    /// Wait for the counter to reach 0
    /// Blocks the current goroutine until the counter is 0
    pub fn wait(&self) -> CursedResult<()> {
        // Quick check first
        if self.counter.load(Ordering::SeqCst) == 0 {
            return Ok(());
        let (lock, cvar) = &*self.waiters;
        let mut finished = lock.lock().map_err(|_| atomic_error("Failed to acquire wait lock"))?;
        
        while self.counter.load(Ordering::SeqCst) != 0 && !*finished {
            finished = cvar.wait(finished).map_err(|_| atomic_error("Wait condition failed"))?;
        Ok(())
    /// Wait for the counter to reach 0 with a timeout
    /// Returns true if the wait completed before timeout, false if it timed out
    pub fn wait_timeout(&self, timeout: Duration) -> CursedResult<bool> {
        // Quick check first
        if self.counter.load(Ordering::SeqCst) == 0 {
            return Ok(true);
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
    /// Get the current counter value
    /// This is primarily for debugging and testing
    pub fn count(&self) -> i32 {
        self.counter.load(Ordering::SeqCst)
    /// Reset the wait group to initial state
    /// This is generally not safe to use in concurrent scenarios
    pub fn reset(&self) {
        self.counter.store(0, Ordering::SeqCst);
        let (lock, cvar) = &*self.waiters;
        if let Ok(mut finished) = lock.lock() {
            *finished = false;
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
        }
    }
/// Create a new wait group
/// Convenience function for creating wait groups
pub fn new_wait_group() -> WaitGroup {
    WaitGroup::new()
