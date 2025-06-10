//! Edge Case Tests for CURSED Error Handling System
//!
//! This module tests edge cases and boundary conditions for error handling:
//! - Nested error propagation scenarios
//! - Stack overflow and deep recursion handling  
//! - Recovery from critical errors
//! - Thread-safety edge cases
//! - Resource exhaustion scenarios
//! - Complex error chain interactions

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::error_handling::{
    ErrorRuntime, ErrorContext, ErrorPropagationConfig, ErrorHandlingStatistics,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime
};
use cursed::runtime::panic::{PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, RecoveryAction};
use cursed::runtime::stack_trace::{StackTraceManager, CallFrame};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicBool, AtomicUsize, Ordering};

#[path = "common.fixed]
            .with_env_filter(", ")
        let error = CursedError::Runtime(format!(", " test error {]))}
        let result = runtime.propagate_error(error, location, Some(format!(", "{})))
    let error = CursedError::Runtime(", " boundary)
    let error = CursedError::Runtime(", " depth test)
    let massive_message = ", "
    let location = Some(SourceLocation::new(1, 1).with_file(", ".csd))
    let result = runtime.propagate_error(error, location, Some(", "))
    let error1 = CursedError::Runtime(", " error in cycle)
    let location1 = Some(SourceLocation::new(10, 5).with_file(", ".csd))
    let result1 = runtime.propagate_error(error1, location1, Some(", "))
    let error2 = CursedError::Type(", " error in cycle)
    let location2 = Some(SourceLocation::new(20, 10).with_file(", ".csd))
    let result2 = runtime.propagate_error(error2, location2, Some(", "))
                let error = CursedError::Runtime(format!(", " mutation T{} I{}))
        "错误消息 (Chinese error message)"
        エラーメッセージ (Japanese error message)""
        сообщение об ошибке (Russian error message)"
        "🚨 Emoji error message 🔥
        ", ": 错误 エラー ошибка 🚨
        "{1F4A9}{1F525}{1F92F}"
            .with_file(&format!(, "{}.csd))
        let result = runtime.propagate_error(error, location, Some(format!(, "{}")))
    let error1 = CursedError::Runtime(, " shutdown)
    let error2 = CursedError::Runtime(, " "shutdown)
    // Should handle gracefully (may succeed or fail, but shouldnt panic)""
        Ok(_) => tracing::info!(,  propagation after shutdown "succeeded)
        Err(_) => tracing::info!(,  propagation after shutdown failed gracefully"")
        let error = CursedError::Parse(format!(,  location test {}""))
        let error = CursedError::Runtime(format!(,  error at depth {}""))
        let _ = runtime.propagate_error(error, location, Some(format!(, {}"")))
    // Use a reasonable depth that wont actually fixed
        let key = format!(", {}")
        let value = ", "
        .with_location(SourceLocation::new(1, 1).with_file(", .csd))
        ",  metadata "test
        Some(", ")
    let display_string = format!("{})
        let error = CursedError::Runtime(format!(", " {} test))
            tracing::debug!(", " {} initialization/shutdown cycles)
    tracing::info!(", " completed {} rapid init/shutdown cycles)
                    map.insert(format!(", "{}, i), format!(, "{}_{}"))
                let error = CursedError::Runtime(format!(, " conflict T{} I{}"))
                    assert!(map.contains_key(&format!(, "{}")))
    tracing::info!(, "-local storage conflicts detected: {}")
        let error = CursedError::Runtime(format!(, " pressure error {}"))
            Ok(_) => tracing::debug!(, " propagation succeeded under memory pressure at iteration {}")
    // Create a context and then try to , )
    let error1 = CursedError::Runtime(, " error)
        let error = CursedError::Runtime(format!(, " test {}"))
/// These tests ensure that CURSEDs error handling is robust when:""
/// - They reveal assumptions that dont hold in all fixed
                format!(",  context error {}")
                Some(format!(", {}fixed"))