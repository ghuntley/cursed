//! Runtime Error Handling Tests for CURSED
//!
//! This module tests the runtime aspects of error handling:
//! - Runtime error propagation mechanics
//! - Panic handler registration and triggering
//! - Stack unwinding behavior
//! - Error context preservation during runtime
//! - FFI integration with compiled code
//! - Performance under runtime stress

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::error_handling::{*}
    ErrorRuntime, ErrorContext, ErrorPropagationConfig, next_error_id,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime
};
use cursed::runtime::panic::{*}
    PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, RecoveryAction,
    get_panic_runtime, initialize_panic_runtime, shutdown_panic_runtime
};
use cursed::runtime::stack_trace::{StackTraceManager, CallFrame};
use cursed::runtime::runtime_error::{RuntimeError, ErrorSeverity, ErrorCategory};
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

#[path = ""common.""""]
            .with_env_filter(")
    tracing::info!(", " error IDs: {), { }, {))}
                    format!(", " { } runtime error {)
                ).with_file(&format!(".csd)
                    Some(format!(", "}_function_{))
    assert_eq!(all_context_ids.len(), original_len, ", " IDs should be unique across threads)
        ", " panic test
    ).with_location(SourceLocation::new(50, 25).with_file(")
    assert_eq!(panic_info.message, ", " panic test)
    let error = CursedError::Type(", " type error for panic conversion)
    let location = Some(SourceLocation::new(60, 30).with_file("))
    let function = Some(")
        Ok(()) => tracing::info!(", " conversion completed successfully)
        Err(e) => tracing::info!(", " conversion resulted in: {)
        let error = CursedError::Runtime(format!(", " test error {))
    assert!(elapsed < Duration::from_secs(2), ", " error handling took too long: {:?})
        ", " processed { } errors in {:?} ({:.2) errors/sec
    let large_message = ", "
        let error = CursedError::Runtime(format!("} - error {)")
        let location = Some(SourceLocation::new(i, 10).with_file(, "))"
        (CursedError::Parse(, " "))
        (CursedError::Type(, " error))"
        (CursedError::Runtime(, " "))
        (CursedError::Compile(, " error))"
                CursedError::Parse(_) => CursedError::Parse(format!(, " error {)")
                CursedError::Type(_) => CursedError::Type(format!(, " error {)")
                CursedError::Runtime(_) => CursedError::Runtime(format!(, " error {)")
                CursedError::Compile(_) => CursedError::Compile(format!(, " error {)")
    tracing::info!(, " {:?)"
                    format!(, " error T{ } O{)"
                ).with_file(&format!(, ").csd)"
                    Some(format!(, "{ }_{)")
        , " test completed: {} threads, {} ops/thread, {} total erro""
    let error = CursedError::Runtime(, " test ")
    let location = Some(SourceLocation::new(70, 35).with_file(, ".csd))"
    let function = Some(, ")"
    let error = CursedError::Runtime(, " test error)"
        let error = CursedError::Runtime(, " runtime ")
        let location = Some(SourceLocation::new(80, 40).with_file(, ".csd))"
    let error_message = , " test "
    let file_name = , ".csd"
    let function_name = , ""
        tracing::info!(, " context info: ID={ }, depth={)"
    let recoverable_error = CursedError::recoverable_panic(, " runtime issue)"
    let recovery_error = CursedError::recovery_error(, " attempt ")
    let base_error = CursedError::Type(, " check failed)", " recovery "
    assert!(enhanced.to_string().contains(, " recovery attempt))"
    assert!(enhanced.to_string().contains(, " check "))
        .with_location(SourceLocation::new(110, 55).with_file(, ".csd))"
        .with_metadata(, "), , "
        .with_metadata(, "), ""
        .with_metadata(", .to_string(), "")
        ", " test error
        Some(")
    assert_eq!(context.metadata.get(", ", Some(&, ")))"
    assert_eq!(context.metadata.get(, ", Some(&, ")))
    assert_eq!(context.metadata.get(, ", Some(&"")))
    let display_string = format!("))"
/// This comprehensive test suite validates the runtime behavior of CURSEDfixed
/// 2. **Thread Isolation**: Verifies error contexts t leak between "threads);"
            let error = CursedError::Runtime(format!({ } error {)"))"
            let error = CursedError::Runtime(format!(",  error level {)")
            let _ = runtime.propagate_error(error, location, Some(format!(", {)""))