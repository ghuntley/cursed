//! Deadlock detection and prevention utilities
//!
//! This module provides utilities to prevent deadlocks when waiting for locks
//! particularly important in the garbage collector to avoid blocking operations.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Mutex, MutexGuard};
use std::time::{Duration, Instant};

/// Default timeout for lock operations (500ms)
const DEFAULT_TIMEOUT_MS: u64 = 500;

/// Try to acquire a read lock with timeout to prevent deadlocks
pub fn try_read_with_timeout<'a, T: ?Sized>(
    lock: &'a RwLock<T>,
    timeout_ms: Option<u64>,
    context: Option<&str>
) -> Option<RwLockReadGuard<'a, T>> {
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS));
    let start = Instant::now();
    
    // Try acquiring the lock repeatedly until timeout, with increasing backoff
    let mut attempt_count = 0;
    while start.elapsed() < timeout {
        match lock.try_read() {
            Ok(guard) => {
                if let Some(ctx) = context {
                    println!("Lock acquired (read) for context: {} after {} attempts", ctx, attempt_count);
                }
                return Some(guard);
            },
            Err(_) => {
                attempt_count += 1;
                
                // Exponential backoff with a max delay of 50ms
                let backoff_ms = std::cmp::min(1 << std::cmp::min(attempt_count / 10, 5), 50);
                std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                
                // Log on every tenth attempt to avoid log spam
                if attempt_count % 10 == 0 && context.is_some() {
                    println!("Still waiting for read lock: {} (attempt: {}, elapsed: {}ms)", 
                             context.unwrap(), attempt_count, start.elapsed().as_millis());
                }
            },
        }
    }
    
    if let Some(ctx) = context {
        println!("Failed to acquire read lock for context: {}", ctx);
    }
    None
}

/// Try to acquire a write lock with timeout to prevent deadlocks
pub fn try_write_with_timeout<'a, T: ?Sized>(
    lock: &'a RwLock<T>,
    timeout_ms: Option<u64>,
    context: Option<&str>
) -> Option<RwLockWriteGuard<'a, T>> {
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS));
    let start = Instant::now();
    
    // Try acquiring the lock repeatedly until timeout, with increasing backoff
    let mut attempt_count = 0;
    while start.elapsed() < timeout {
        match lock.try_write() {
            Ok(guard) => {
                if let Some(ctx) = context {
                    println!("Lock acquired (write) for context: {} after {} attempts", ctx, attempt_count);
                }
                return Some(guard);
            },
            Err(_) => {
                attempt_count += 1;
                
                // Exponential backoff with a max delay of 50ms
                let backoff_ms = std::cmp::min(1 << std::cmp::min(attempt_count / 10, 5), 50);
                std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                
                // Log on every tenth attempt to avoid log spam
                if attempt_count % 10 == 0 && context.is_some() {
                    println!("Still waiting for write lock: {} (attempt: {}, elapsed: {}ms)", 
                             context.unwrap(), attempt_count, start.elapsed().as_millis());
                }
            },
        }
    }
    
    if let Some(ctx) = context {
        println!("Failed to acquire write lock for context: {}", ctx);
    }
    None
}

/// Try to acquire a mutex lock with timeout to prevent deadlocks
pub fn try_lock_with_timeout<'a, T: ?Sized>(
    mutex: &'a Mutex<T>,
    timeout_ms: Option<u64>,
    context: Option<&str>
) -> Option<MutexGuard<'a, T>> {
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS));
    let start = Instant::now();
    
    // Try acquiring the lock repeatedly until timeout, with increasing backoff
    let mut attempt_count = 0;
    while start.elapsed() < timeout {
        match mutex.try_lock() {
            Ok(guard) => {
                if let Some(ctx) = context {
                    println!("Lock acquired (mutex) for context: {} after {} attempts", ctx, attempt_count);
                }
                return Some(guard);
            },
            Err(_) => {
                attempt_count += 1;
                
                // Exponential backoff with a max delay of 50ms
                let backoff_ms = std::cmp::min(1 << std::cmp::min(attempt_count / 10, 5), 50);
                std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                
                // Log on every tenth attempt to avoid log spam
                if attempt_count % 10 == 0 && context.is_some() {
                    println!("Still waiting for mutex lock: {} (attempt: {}, elapsed: {}ms)", 
                             context.unwrap(), attempt_count, start.elapsed().as_millis());
                }
            },
        }
    }
    
    if let Some(ctx) = context {
        println!("Failed to acquire mutex lock for context: {}", ctx);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};
    use std::thread;
    
    #[test]
    fn test_lock_timeout() {
        let lock = RwLock::new(42);
        
        // Acquire write lock in another thread and hold it
        let write_lock = lock.write().unwrap();
        
        // Try to acquire read lock with timeout in this thread
        let result = try_read_with_timeout(&lock, Some(100), Some("test_lock_timeout"));
        
        // Should timeout
        assert!(result.is_none());
        
        // Drop the write lock
        drop(write_lock);
        
        // Now we should be able to acquire read lock
        let result = try_read_with_timeout(&lock, Some(100), Some("test_lock_timeout_after_drop"));
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 42);
    }
    
    #[test]
    fn test_mutex_timeout() {
        let mutex = Mutex::new(42);
        
        // Acquire lock in another thread and hold it
        let lock = mutex.lock().unwrap();
        
        // Try to acquire lock with timeout in this thread
        let result = try_lock_with_timeout(&mutex, Some(100), Some("test_mutex_timeout"));
        
        // Should timeout
        assert!(result.is_none());
        
        // Drop the lock
        drop(lock);
        
        // Now we should be able to acquire lock
        let result = try_lock_with_timeout(&mutex, Some(100), Some("test_mutex_timeout_after_drop"));
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 42);
    }
}