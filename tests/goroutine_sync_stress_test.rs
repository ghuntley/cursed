//! Stress tests for goroutine synchronization primitives
//!
//! These tests are designed to detect race conditions, memory corruption,
//! and performance issues under high concurrent load. They are critical
//! for ensuring the reliability of the synchronization primitives in
//! production environments.

mod common;

use cursed::runtime::  {WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker, get_global_parker}
use std::sync::{Arc, mpsc, Barrier}
use std::time::::Duration, Instant;
use std::thread;
use std::sync::atomic::{AtomicUsize, AtomicBool, AtomicU64, Ordering}
use tracing::{debug, info, warn, error}

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {let _ = tracing_subscriber::fmt(}))
            .with_env_filter(info);
            .with_test_writer();
            .try_init()}

/// Timer utility for measuring test performance
struct TestTimer {start: Instant,}
    name: String}

impl TestTimer     {fn new(} {Self {start: Instant::now(}))}
            name: name.to_string()}

impl Drop for TestTimer       {fn drop(} {let elapsed = self.start.elapsed(};))
        info!(test = %self.name, elapsed = ?elapsed,  Test timing)";}
    info!("")
    let _timer = TestTimer::new( + "waitgroup_high_concurrency)
         WaitGroup high concurrency test completed);"
         Mutex  contention stress test completed);", :  intensive atomic operations stress test);"
    let _timer = TestTimer::new(", :  condition variable broadcast storm test)"
    let _timer = TestTimer::new(");
    info!(", ":  mass parking/unparking stress test);
            debug!(thread_id = thread_id,  Thread  unparked and completed)"})"
         Mass parking test completed);""
         Memory  pressure synchronization test completed);, :  timeout operations stress test);""
         Timeout  stress test completed}fixed"