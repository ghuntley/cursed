//! Deadlock detection utilities for lock acquisition
//!
//! This module provides utilities to detect and report potential deadlocks
//! when acquiring locks. It helps diagnose issues with lock contention and
//! recursive locking that might lead to program hangs.

use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

/// Try to acquire a read lock with deadlock detection
pub fn try_read_with_timeout<'a, T>(
    lock: &'a RwLock<T>,
    timeout: Duration,
    context: &str
) -> Option<RwLockReadGuard<'a, T>> {
    let start = Instant::now();
    
    // Special fast path for thread-safe code to avoid deadlocks
    if context.contains("thread_safe") || context.contains("ThreadSafe") {
        match lock.try_read() {
            Ok(guard) => {
                println!("[LOCK] Acquired read lock immediately for thread-safe operation in {}", context);
                return Some(guard);
            },
            Err(_) => {
                println!("[LOCK] ⚠️ Failed to acquire read lock immediately for thread-safe operation in {}", context);
                return None; // Fail immediately instead of retrying and potentially deadlocking
            }
        }
    }
    
    // Try a bunch of times with small sleeps in between
    let max_attempts = 100;
    let sleep_duration = timeout / max_attempts;
    
    for attempt in 1..=max_attempts {
        match lock.try_read() {
            Ok(guard) => {
                println!("[LOCK] Acquired read lock after {} attempts ({:?}) in {}", 
                         attempt, start.elapsed(), context);
                return Some(guard);
            },
            Err(_) => {
                // Sleep a bit before trying again
                std::thread::sleep(sleep_duration);
                
                // If we're approaching timeout, report potential deadlock
                if start.elapsed() > timeout * 9 / 10 {
                    println!("[LOCK] ⚠️ Potential deadlock detected acquiring read lock in {}", context);
                    println!("[LOCK] Stack trace: {:#?}", std::backtrace::Backtrace::capture());
                }
            }
        }
    }
    
    println!("[LOCK] ❌ Failed to acquire read lock after {:?} in {}", timeout, context);
    None
}

/// Try to acquire a write lock with deadlock detection
pub fn try_write_with_timeout<'a, T>(
    lock: &'a RwLock<T>,
    timeout: Duration,
    context: &str
) -> Option<RwLockWriteGuard<'a, T>> {
    let start = Instant::now();
    
    // Special fast path for thread-safe code to avoid deadlocks
    if context.contains("thread_safe") || context.contains("ThreadSafe") {
        match lock.try_write() {
            Ok(guard) => {
                println!("[LOCK] Acquired write lock immediately for thread-safe operation in {}", context);
                return Some(guard);
            },
            Err(_) => {
                println!("[LOCK] ⚠️ Failed to acquire write lock immediately for thread-safe operation in {}", context);
                return None; // Fail immediately instead of retrying and potentially deadlocking
            }
        }
    }
    
    // Try a bunch of times with small sleeps in between
    let max_attempts = 100;
    let sleep_duration = timeout / max_attempts;
    
    for attempt in 1..=max_attempts {
        match lock.try_write() {
            Ok(guard) => {
                println!("[LOCK] Acquired write lock after {} attempts ({:?}) in {}", 
                         attempt, start.elapsed(), context);
                return Some(guard);
            },
            Err(_) => {
                // Sleep a bit before trying again
                std::thread::sleep(sleep_duration);
                
                // If we're approaching timeout, report potential deadlock
                if start.elapsed() > timeout * 9 / 10 {
                    println!("[LOCK] ⚠️ Potential deadlock detected acquiring write lock in {}", context);
                    println!("[LOCK] Stack trace: {:#?}", std::backtrace::Backtrace::capture());
                }
            }
        }
    }
    
    println!("[LOCK] ❌ Failed to acquire write lock after {:?} in {}", timeout, context);
    None
}

/// Try to acquire a mutex with deadlock detection
pub fn try_mutex_with_timeout<'a, T>(
    lock: &'a Mutex<T>,
    timeout: Duration,
    context: &str
) -> Option<MutexGuard<'a, T>> {
    let start = Instant::now();
    
    // Try a bunch of times with small sleeps in between
    let max_attempts = 100;
    let sleep_duration = timeout / max_attempts;
    
    for attempt in 1..=max_attempts {
        match lock.try_lock() {
            Ok(guard) => {
                println!("[LOCK] Acquired mutex after {} attempts ({:?}) in {}", 
                         attempt, start.elapsed(), context);
                return Some(guard);
            },
            Err(_) => {
                // Sleep a bit before trying again
                std::thread::sleep(sleep_duration);
                
                // If we're approaching timeout, report potential deadlock
                if start.elapsed() > timeout * 9 / 10 {
                    println!("[LOCK] ⚠️ Potential deadlock detected acquiring mutex in {}", context);
                    println!("[LOCK] Stack trace: {:#?}", std::backtrace::Backtrace::capture());
                }
            }
        }
    }
    
    println!("[LOCK] ❌ Failed to acquire mutex after {:?} in {}", timeout, context);
    None
}