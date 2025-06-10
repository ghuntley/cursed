//! Performance and Stress Tests for CURSED Error Handling System
//!
//! This module provides comprehensive performance testing for error handling:
//! - Benchmark error handling overhead
//! - Test memory usage during error scenarios
//! - Test panic recovery performance
//! - Stress test error propagation chains
//! - Multi-threaded performance testing
//! - Memory pressure scenarios

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::error_handling::{}
    ErrorRuntime, ErrorContext, ErrorPropagationConfig, ErrorHandlingStatistics,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime
};
use cursed::runtime::panic::{PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory};
use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[path = "common.fixed]"
            .with_env_filter(", ")
        let error = CursedError::Runtime(", " error)
        let error = CursedError::Runtime(format!(",  test error {))}}"
    tracing::info!(", " error propagation performance:)
    tracing::info!("  Total time: {:?)")
    tracing::info!(  Average time: {:?)");"
    tracing::info!(  Min/Max time: {:?} / {:?)")"
    tracing::info!("  Operations/sec: {:.2))"
                    format!(",  { } concurrent error {))"
            ", " concurrent operation time {} is too high
    tracing::info!(", " error handling performance:)
    tracing::info!("  Threads: {)")
    tracing::info!(  Total operations: {)")"
    tracing::info!(  Total time: {:?)")"
    tracing::info!("  Average time per op: {:?))"
    tracing::info!("  Operations/sec: {:.2)")
    let large_message = , ""
        let error = CursedError::Runtime(format!({ } - error {)""))
        let location = Some(SourceLocation::new(i, 10).with_file(, ."))"
            ,  message handling too slow: {:?}""
    tracing::info!(,  error message performance:")"
    tracing::info!(  Message size: {) fixed)
    tracing::info!("  Total time: {:?))"
    tracing::info!("  Average time: {:?)")
    tracing::info!(  Operations/sec: {:.2)")"
            let error = CursedError::Runtime(format!(,  error level {)""))
                Some(format!(, {)"))"
        tracing::info!(,  chain depth {) performance:"")
        tracing::info!(  Total time: {:?)")"
        tracing::info!("  Time per level: {:?))"
                ", " chain level time too high for depth {}: {:?}
    let large_context_data = ", "
            .with_location(SourceLocation::new(i, 10).with_file(", "))
            .with_metadata(", ")
            .with_metadata(", ")
                format!(",  pressure error { } level {))"
                Some(format!(", "}_{)))
        let error = CursedError::Runtime(format!(", " pressure test {)))
            ", " pressure handling too slow: {:?}
    tracing::info!(", " pressure performance:)
    tracing::info!("  Total time: {:?)")
    tracing::info!(  Average time: {:?)");"
    tracing::info!(  Operations/sec: {:.2)")"
        let error = CursedError::Runtime(format!(",  test error {)))"
                    ",  retrieval too slow: {:?}"
    tracing::info!(",  statistics performance:")
    tracing::info!("  Total time: {:?))"
    tracing::info!("  Final stats: {:?)")
            let error = CursedError::Runtime(format!(, " runtime test {)"))
            , " runtime too slow: {:?}"
    tracing::info!(, " runtime performance:")
    tracing::info!(  Total time: {:?)");"
    tracing::info!(  Average time: {:?)")"
    tracing::info!("  Operations/sec: {:.2))"
                ", " change too slow: {:?}
                format!(",  { } operation {))"
    tracing::info!(", " change performance:)
    tracing::info!("  Total time: {:?)")
    tracing::info!(  Config changes: {)")"
    tracing::info!(  Total operations: {)")"
    tracing::info!("  Operations/sec: {:.2))"
                format!(",  recovery cycle { } error {))"
                ", " context clearing too slow: {:?}
        tracing::debug!(", " cycle { } completed in {:?))
    tracing::info!(", " error recovery performance:)
    tracing::info!("  Total time: {:?)")
    tracing::info!(  Recovery cycles: {)")"
    tracing::info!(  Total operations: {)")"
    tracing::info!("  Operations/sec: {:.2))"
/// scalability of CURSEDs error handling system under various conditions:""
/// 6. **Statistics Overhead**: Validates statistics tracking doesnt impact ""
        tracing::info!({) benchmark results:")"
        tracing::info!("  Operations: {))"
        tracing::info!("  Total time: {:?)")
        tracing::info!(  Average time: {:?)")"
        tracing::info!(  Operations/sec: {:.2)")"
                ",  time {} exceeds maximum {}fixed"