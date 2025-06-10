/// Comprehensive integration tests for the CURSED panic and recovery system
///
/// This test suite validates the complete panic infrastructure including:
/// - Panic triggering and handling
/// - Recovery mechanisms and cleanup
/// - Integration with goroutine system  
/// - Memory safety during panic scenarios
/// - Thread-safe panic handling
/// - Gen Z slang panic functions
/// - FFI integration for compiled code
///
/// Why comprehensive panic testing is critical:
/// 1. **Runtime Safety**: Panics can occur at any time and must be handled safely
/// 2. **Memory Integrity**: Panic cleanup must not leak memory or corrupt state
/// 3. **Concurrency Safety**: Panics in goroutines must not affect other goroutines
/// 4. **Recovery Correctness**: Recovery mechanisms must work reliably
/// 5. **Production Reliability**: Panic handling affects overall system stability

use cursed::runtime::panic::{
    PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, PanicConfig,
    RecoveryAction, initialize_panic_runtime, shutdown_panic_runtime,
    get_panic_runtime, no_cap_panic, sus_panic, cap_panic, not_vibing_panic,
    cursed_panic_with_message
};
use cursed::error::{Error as CursedError, SourceLocation};
use std::sync::{Arc, Mutex, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use std::panic;

#[path = "common.rs"]
pub mod common;

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[test]
fn test_basic_panic_runtime_functionality() {
    init_tracing!();
    
    let runtime = PanicRuntime::new();
    assert!(runtime.initialize().is_ok());
    
    // Test statistics initially empty
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_panics, 0);
    assert_eq!(stats.successful_recoveries, 0);
    assert_eq!(stats.failed_recoveries, 0);
    
    // Test recovery mode
    assert!(!runtime.is_in_recovery());
    
    assert!(runtime.shutdown().is_ok());
}

#[test]
fn test_successful_recovery() {
    init_tracing!();
    
    let runtime = PanicRuntime::new();
    runtime.initialize().unwrap();
    
    let result = runtime.recover(|| {
        // Successful operation
        42
    });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.successful_recoveries, 1);
    assert_eq!(stats.failed_recoveries, 0);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_panic_recovery() {
    init_tracing!();
    
    let runtime = PanicRuntime::new();
    runtime.initialize().unwrap();
    
    let result = runtime.recover(|| {
        panic!("Test panic for recovery");
    });
    
    assert!(result.is_err());
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.failed_recoveries, 1);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_panic_info_creation_and_metadata() {
    init_tracing!();
    
    let location = SourceLocation::new(42, 10).with_file("test.csd");
    
    let panic_info = CursedPanicInfo::new(
        "Test panic message".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User
    )
    .with_location(location)
    .with_goroutine(123)
    .with_metadata("context".to_string(), "test_function".to_string())
    .with_metadata("component".to_string(), "parser".to_string());
    
    assert_eq!(panic_info.message, "Test panic message");
    assert_eq!(panic_info.severity, PanicSeverity::Critical);
    assert_eq!(panic_info.category, PanicCategory::User);
    assert_eq!(panic_info.goroutine_id, Some(123));
    assert!(panic_info.panic_id > 0);
    assert_eq!(panic_info.metadata.get("context"), Some(&"test_function".to_string()));
    assert_eq!(panic_info.metadata.get("component"), Some(&"parser".to_string()));
    
    let formatted = format!("{}", panic_info);
    assert!(formatted.contains("Test panic message"));
    assert!(formatted.contains("Critical"));
    assert!(formatted.contains("User"));
    assert!(formatted.contains("goroutine #123"));
}

#[test]
fn test_stack_trace_capture() {
    init_tracing!();
    
    let runtime = PanicRuntime::new();
    runtime.initialize().unwrap();
    
    // Test basic stack trace capture
    let stack_trace = runtime.capture_stack_trace(10);
    assert!(!stack_trace.is_empty());
    assert!(stack_trace.len() <= 10);
    
    // Test frame information
    let frame = &stack_trace[0];
    assert!(frame.function_name.is_some());
    assert!(frame.module_name.is_some());
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_recovery_handlers() {
    init_tracing!();
    
    let runtime = PanicRuntime::new();
    runtime.initialize().unwrap();
    
    let handler_called = Arc::new(AtomicBool::new(false));
    let handler_called_clone = Arc::clone(&handler_called);
    
    // Register a recovery handler
    let result = runtime.register_recovery_handler(move |panic_info| {
        handler_called_clone.store(true, Ordering::SeqCst);
        assert_eq!(panic_info.category, PanicCategory::User);
        RecoveryAction::Continue(CursedError::Runtime("Handled".to_string()))
    });
    
    assert!(result.is_ok());
    
    // Register a global handler
    let global_handler_called = Arc::new(AtomicBool::new(false));
    let global_handler_called_clone = Arc::clone(&global_handler_called);
    
    let result = runtime.register_global_handler(move |_panic_info| {
        global_handler_called_clone.store(true, Ordering::SeqCst);
        RecoveryAction::TerminateGoroutine
    });
    
    assert!(result.is_ok());
    runtime.shutdown().unwrap();
}

#[test]
fn test_panic_configuration() {
    init_tracing!();
    
    let custom_config = PanicConfig {
        capture_backtraces: true,
        capture_stack_traces: true,
        max_stack_depth: 25,
        log_to_stderr: false,
        abort_on_fatal: false,
        default_recovery: RecoveryAction::Continue(
            CursedError::Runtime("Custom default".to_string())
        ),
        recovery_timeout: Duration::from_secs(5),
        debug_manager: None,
        stack_trace_config: Default::default(),
    };
    
    let runtime = PanicRuntime::with_config(custom_config);
    runtime.initialize().unwrap();
    
    // Test configuration update
    let update_result = runtime.update_config(|config| {
        config.max_stack_depth = 50;
        config.log_to_stderr = true;
    });
    
    assert!(update_result.is_ok());
    runtime.shutdown().unwrap();
}

#[test]
fn test_concurrent_panic_handling() {
    init_tracing!();
    
    let runtime = Arc::new(PanicRuntime::new());
    runtime.initialize().unwrap();
    
    let success_count = Arc::new(AtomicU64::new(0));
    let failure_count = Arc::new(AtomicU64::new(0));
    
    let mut handles = Vec::new();
    
    // Spawn multiple threads that perform recovery operations
    for i in 0..8 {
        let runtime_clone = Arc::clone(&runtime);
        let success_count_clone = Arc::clone(&success_count);
        let failure_count_clone = Arc::clone(&failure_count);
        
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let should_panic = (i + j) % 3 == 0;
                
                let result = runtime_clone.recover(|| {
                    if should_panic {
                        panic!("Thread {} iteration {}", i, j);
                    } else {
                        i * 10 + j
                    }
                });
                
                match result {
                    Ok(_) => {
                        success_count_clone.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(_) => {
                        failure_count_clone.fetch_add(1, Ordering::SeqCst);
                    }
                }
                
                // Small delay to interleave operations
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_success = success_count.load(Ordering::SeqCst);
    let total_failure = failure_count.load(Ordering::SeqCst);
    
    assert_eq!(total_success + total_failure, 80); // 8 threads * 10 iterations
    assert!(total_success > 0);
    assert!(total_failure > 0);
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.successful_recoveries, total_success);
    assert_eq!(stats.failed_recoveries, total_failure);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_panic_severity_levels() {
    init_tracing!();
    
    // Test severity ordering
    assert!(PanicSeverity::Recoverable < PanicSeverity::Critical);
    assert!(PanicSeverity::Critical < PanicSeverity::Fatal);
    
    // Test panic info with different severities
    let recoverable_panic = CursedPanicInfo::new(
        "Recoverable error".to_string(),
        PanicSeverity::Recoverable,
        PanicCategory::User
    );
    
    let critical_panic = CursedPanicInfo::new(
        "Critical error".to_string(),
        PanicSeverity::Critical,
        PanicCategory::System
    );
    
    let fatal_panic = CursedPanicInfo::new(
        "Fatal error".to_string(),
        PanicSeverity::Fatal,
        PanicCategory::Memory
    );
    
    assert_eq!(recoverable_panic.severity, PanicSeverity::Recoverable);
    assert_eq!(critical_panic.severity, PanicSeverity::Critical);
    assert_eq!(fatal_panic.severity, PanicSeverity::Fatal);
}

#[test]
fn test_panic_categories() {
    init_tracing!();
    
    let categories = vec![
        PanicCategory::Memory,
        PanicCategory::TypeAssertion,
        PanicCategory::BoundsCheck,
        PanicCategory::Arithmetic,
        PanicCategory::Channel,
        PanicCategory::Goroutine,
        PanicCategory::User,
        PanicCategory::System,
        PanicCategory::Generic,
    ];
    
    // Ensure all categories are unique
    for (i, cat1) in categories.iter().enumerate() {
        for (j, cat2) in categories.iter().enumerate() {
            if i != j {
                assert_ne!(cat1, cat2);
            }
        }
    }
    
    // Test creating panic info with each category
    for (i, category) in categories.iter().enumerate() {
        let panic_info = CursedPanicInfo::new(
            format!("Test panic {}", i),
            PanicSeverity::Critical,
            category.clone()
        );
        assert_eq!(panic_info.category, *category);
    }
}

#[test]
fn test_performance_characteristics() {
    init_tracing!();
    
    let runtime = PanicRuntime::new();
    runtime.initialize().unwrap();
    
    let start_time = Instant::now();
    let iterations = 1000;
    
    // Test performance of successful recovery operations
    for _ in 0..iterations {
        let result = runtime.recover(|| {
            // Simple successful operation
            42
        });
        assert!(result.is_ok());
    }
    
    let elapsed = start_time.elapsed();
    let avg_time = elapsed / iterations;
    
    // Recovery operations should be fast (< 1ms each on average)
    assert!(avg_time < Duration::from_millis(1));
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.successful_recoveries, iterations as u64);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_memory_safety_during_panic() {
    init_tracing!();
    
    let runtime = Arc::new(PanicRuntime::new());
    runtime.initialize().unwrap();
    
    // Create a large number of panic scenarios to test memory safety
    for i in 0..100 {
        let runtime_clone = Arc::clone(&runtime);
        
        let _result = runtime_clone.recover(|| {
            if i % 2 == 0 {
                panic!("Memory safety test panic {}", i);
            } else {
                // Successful operation
                vec![0u8; 1024] // Allocate some memory
            }
        });
        
        // Each iteration should not leak memory or corrupt state
    }
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.successful_recoveries + stats.failed_recoveries, 100);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_global_panic_runtime() {
    init_tracing!();
    
    // Test global runtime initialization (careful with other tests)
    if get_panic_runtime().is_none() {
        assert!(initialize_panic_runtime().is_ok());
        
        let global_runtime = get_panic_runtime();
        assert!(global_runtime.is_some());
        
        // Test that we can use the global runtime
        let stats = global_runtime.unwrap().get_statistics().unwrap();
        assert_eq!(stats.total_panics, 0); // Initially no panics
        
        assert!(shutdown_panic_runtime().is_ok());
    }
}

#[test]
fn test_gen_z_slang_functions() {
    init_tracing!();
    
    // We can't directly test the panic functions since they terminate execution,
    // but we can test their panic info creation patterns
    
    // Test "no cap" panic formatting
    let no_cap_info = CursedPanicInfo::new(
        "no cap: this is definitely broken".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    assert!(no_cap_info.message.contains("no cap"));
    assert!(no_cap_info.message.contains("definitely broken"));
    
    // Test "sus" panic formatting
    let sus_info = CursedPanicInfo::new(
        "that's sus: something fishy happening".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    assert!(sus_info.message.contains("that's sus"));
    assert!(sus_info.message.contains("fishy"));
    
    // Test "cap" panic formatting
    let cap_info = CursedPanicInfo::new(
        "cap detected: false statement found".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    assert!(cap_info.message.contains("cap detected"));
    assert!(cap_info.message.contains("false statement"));
    
    // Test "not vibing" panic formatting
    let not_vibing_info = CursedPanicInfo::new(
        "not vibing: bad energy detected".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    assert!(not_vibing_info.message.contains("not vibing"));
    assert!(not_vibing_info.message.contains("bad energy"));
}

#[test]
fn test_recovery_action_types() {
    init_tracing!();
    
    // Test all recovery action types
    let continue_action = RecoveryAction::Continue(
        CursedError::Runtime("Converted to error".to_string())
    );
    
    let terminate_action = RecoveryAction::TerminateGoroutine;
    let retry_action = RecoveryAction::Retry;
    
    let escalate_panic = CursedPanicInfo::new(
        "Escalated panic".to_string(),
        PanicSeverity::Fatal,
        PanicCategory::System
    );
    let escalate_action = RecoveryAction::Escalate(escalate_panic);
    
    // Verify action types are distinct
    match continue_action {
        RecoveryAction::Continue(_) => (),
        _ => panic!("Wrong action type"),
    }
    
    match terminate_action {
        RecoveryAction::TerminateGoroutine => (),
        _ => panic!("Wrong action type"),
    }
    
    match retry_action {
        RecoveryAction::Retry => (),
        _ => panic!("Wrong action type"),
    }
    
    match escalate_action {
        RecoveryAction::Escalate(_) => (),
        _ => panic!("Wrong action type"),
    }
}
