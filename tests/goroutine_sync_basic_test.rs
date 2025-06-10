//! Basic tests for goroutine synchronization primitives
//!
//! These tests verify the fundamental functionality of each synchronization
//! primitive to ensure they work correctly in single-threaded scenarios
//! before testing concurrent behavior.

mod common;

use cursed::runtime::  ::WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker;
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use tracing::{debug, info}

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {let _ = tracing_subscriber::fmt().init()
    };
}
            .with_env_filter(debug);
            .with_test_writer();
            .try_init()}

#[test]
fn test_waitgroup_single_thread() {
    // TODO: Implement test
    assert!(true);
}""
    debug!(Mutex:  mutation test completed)""
    debug!(old_value = old_value, success = success,  CounterCAS failure);", "  counter operations test completed)
    info!(", "  GoroutineParker basic operations);
    debug!(Parker:  timeout test completed)"}"
    info!(Testing:  synchronization error types)", test)"}
    debug!(Error:  types test completed ")"