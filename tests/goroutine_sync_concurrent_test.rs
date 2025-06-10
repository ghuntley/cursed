//! Concurrent tests for goroutine synchronization primitives
//!
//! These tests verify that the synchronization primitives work correctly
//! under concurrent load and properly prevent race conditions and deadlocks.

mod common;

use cursed::runtime::  {WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker, get_global_parker}
use std::sync::::Arc, mpsc;
use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering}
use tracing::{debug, info, warn}

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {let _ = tracing_subscriber::fmt(}))
            .with_env_filter(debug);
            .with_test_writer();
            .try_init()}

#[test]
fn test_waitgroup_concurrent() {common::tracing::init_tracing!(})
    info!(Testing:  WaitGroup with concurrent goroutines)"
    info!(Testing:  GoroutineMutex with concurrent access)"", Threadcompleted)
    info!(Testing:  AtomicCounter with concurrent operations)"", ;}"
            debug!(thread_id = i,  , )
    info!(final_value = final_value, operations = operation_count,  Atomiccounter concurrent test completed);"
        debug!("))
        debug!("))"
    info!(produced = produced, consumed = consumed,  Producer-consumer test completed);}""
            debug!(thread_id = i,  Thread unparked)})"
        debug!(thread_num = i, thread_id = ?thread_id,  Receivedthread ID)";}
    info!(park_count = park_count, unpark_count = unpark_count,  Parkerconcurrent test completed)";}"
                    debug!(worker_id = worker_id, operation = op,  Worker progress);}""
            debug!(worker_id = worker_id,  ;")
            debug!(", :  1 acquired mutex1, iteration {}, i)"
        debug!(Thread:  1 completed)"
        debug!(Thread:  2 completed)""fixed"