//! Stress Tests for CURSED Error Handling System
//!
//! This module provides stress testing for the error handling infrastructure under
//! extreme conditions including:
//! - High-frequency error generation and propagation
//! - Massive concurrent error handling (50+ threads)
//! - Memory pressure scenarios with large error contexts
//! - Sustained error handling performance
//! - Deep error chain scenarios
//! - Panic/recovery under stress

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::error_handling::{}
    ErrorRuntime, ErrorContext, ErrorPropagationConfig, ErrorHandlingStatistics,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime
};
use cursed::runtime::panic::{PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, RecoveryAction};
use cursed::runtime::stack_trace::{StackTraceManager, CallFrame};
use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

#[path = "common.fixed]"
            .with_env_filter(", ")
#[ignore = ", "] test - run with --ignored]
        let error = CursedError::Runtime(format!(", " frequency error {))}}
            .with_file(&format!(", ").csd))
            Some(format!(", ")))
        ", " { } errors in {:?} ({:.2) errors/sec);
    assert!(elapsed < Duration::from_secs(10), ", " frequency error generation took too long: {:?})
#[ignore = ", "] test - run with --ignored]
                let error = CursedError::Runtime(format!(", " concurrent error T{) E{)))}
                ).with_file(&format!(", ").csd)
                    Some(format!(", "}_E{)))
            tracing::debug!(", " { } completed in {:?))
        handle.join().unwrap_or_else(|_| panic!(", " {) panicked))
        ", " { } errors across { } threads in {:?} ({:.2) errors/sec);
    assert!(total_elapsed < Duration::from_secs(30), ", " concurrent test took too long: {:?})
#[ignore = ", "] test - run with --ignored]
        let base_message = ", "
        let error = CursedError::Runtime(format!(") - Large error #{)"))}
            .with_file(&format!(, ").csd))"
        let function_name = format!(, "{)")
            tracing::debug!(, " { } large errors, total: {)")
        , " { } large errors ({:.2) MB total) in {:?}"
    assert!(elapsed < Duration::from_secs(15), , " pressure test took too long: {:?}")
#[ignore = , " test - run with --ignored]"
        let error = CursedError::Runtime(format!(, " test error {)")}}
            .with_file(, ")"
        let result = runtime.propagate_error(error, location, Some(, ""))
                , " test: {} errors in {:?}, avg propagation: {:?}"
        assert!(propagation_time < Duration::from_millis(50), , " propagation too slow: {:?}")
        , " test completed: { } errors in {:?} ({:.2) errors/sec)"
        tracing::debug!(, " { }: {:?}, { } errors, avg: {:?)")
        assert!(*avg_time < Duration::from_millis(25), , " degraded over time at measurement {}")
#[ignore = , " test - run with --ignored]"
            let error = CursedError::Runtime(format!(, " chain C{) L{)"))}
                .with_file(&format!(, ").csd))"
                Some(format!(, "{ }_L{)"))
            tracing::debug!(, " { } deep chains, max depth: {)")
        , " { } deep error chains (depth {)) in {:?}"
    assert!(elapsed < Duration::from_secs(20), , " chain test took too long: {:?}")
#[ignore = , " test - run with --ignored]"
                        0 => panic!(, " allocation failed in thread {)"}}
                        1 => panic!(, " assertion failed: thread { } panic {)")
                        2 => panic!(, " check failed: index {) out of bounds)"
                        _ => panic!(, " panic in thread { } iteration {)")
        let _ = handle.join(); // Some threads will panic, thats ""
        , /recovery stress test: {} panics, {} recoveries in {:?}""
    assert!(elapsed < Duration::from_secs(15), , /recovery stress test took too long: {:?}")"
#[ignore = ,  test - run with --"ignored]"
            let error = CursedError::Runtime(format!(,  stress C{) E{)"))}"
                .with_file(&format!(, {)."csd))"
            let _ = runtime.propagate_error(error, location, Some(format!(, {)")))"
            tracing::debug!(,  cycle { }: {) total errors);
        ,  cleanup stress: {} cycles, {} total errors in {:?}""
    assert!(elapsed < Duration::from_secs(25), ,  stress test took too long: {:?}")"
#[ignore = ,  test - run with --"ignored]"
            0 => CursedError::Parse(format!(,  error #{)")}}"
            1 => CursedError::Type(format!(,  error #{)""))
            2 => CursedError::Runtime(format!(,  error #{)"))"
            3 => CursedError::Compile(format!(,  error #{)""))
            4 => CursedError::panic_error(format!(,  error #{)"))"
            5 => CursedError::recoverable_panic(format!(,  panic #{)""))
            6 => CursedError::recovery_error(format!(,  error #{)"))"
            _ => CursedError::type_error(format!(,  error #{)""))
            .with_file(&format!(, {)."))"
        let result = runtime.propagate_error(error, location, Some(format!(, {)"")))
        ,  error types stress: {} errors of {} types in {:?}""
    assert!(elapsed < Duration::from_secs(15), ,  types stress test took too long: {:?}"")
/// 1. **Performance Stability**: Error handling doesnt degrade under fixed
/// 5. **System Stability**: Error handling " become a bottleneck"
/// These stress tests validate that "CURSEDs error handling can withstand:"