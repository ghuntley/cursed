/// Integration tests for CURSED panic and recovery system
///
/// Tests the complete panic/recovery infrastructure including:
/// - Panic triggering and handling
/// - Recovery scope management
/// - Error conversion utilities
/// - LLVM integration
/// - Gen Z slang panic functions

use cursed::runtime::{
    panic::{
        initialize_panic_runtime, get_panic_runtime, shutdown_panic_runtime,
        PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory,
        no_cap_panic, sus_panic, cap_panic, not_vibing_panic,
        cursed_panic_with_message, PanicConfig, RecoveryAction
    },
    recovery::{
        initialize_recovery_manager, get_recovery_manager,
        RecoveryManager, RecoveryConfig, RecoveryScope, RecoveryScopeGuard,
        catch_panic, catch_panic_with_config, panic_to_error, error_to_recovery_action,
        is_recoverable_error
    }
};
use cursed::error::{Error as CursedError, SourceLocation};
use cursed::codegen::llvm::panic::{PanicCompiler, LlvmPanicGenerator, PanicCompilerConfig};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use std::thread;

#[test]
fn test_panic_runtime_initialization() {
    // Test initialization and shutdown
    assert!(initialize_panic_runtime().is_ok());
    assert!(get_panic_runtime().is_some());
    assert!(shutdown_panic_runtime().is_ok());
}

#[test]
fn test_recovery_manager_initialization() {
    // Test recovery manager initialization
    assert!(initialize_recovery_manager().is_ok());
    assert!(get_recovery_manager().is_some());
}

#[test]
fn test_panic_info_creation() {
    let panic_info = CursedPanicInfo::new(
        "Test panic message".to_string(),
        PanicSeverity::Recoverable,
        PanicCategory::User,
    );

    assert_eq!(panic_info.message, "Test panic message");
    assert_eq!(panic_info.severity, PanicSeverity::Recoverable);
    assert_eq!(panic_info.category, PanicCategory::User);
    assert!(panic_info.panic_id > 0);
}

#[test]
fn test_panic_info_with_location() {
    let location = SourceLocation::new(10, 5);
    let panic_info = CursedPanicInfo::new(
        "Test panic".to_string(),
        PanicSeverity::Critical,
        PanicCategory::BoundsCheck,
    ).with_location(location.clone());

    assert_eq!(panic_info.source_location, Some(location));
}

#[test]
fn test_panic_info_with_goroutine() {
    let panic_info = CursedPanicInfo::new(
        "Goroutine panic".to_string(),
        PanicSeverity::Critical,
        PanicCategory::Goroutine,
    ).with_goroutine(42);

    assert_eq!(panic_info.goroutine_id, Some(42));
}

#[test]
fn test_recovery_scope_creation() {
    let config = RecoveryConfig::default();
    let scope = RecoveryScope::new("test_scope".to_string(), config);

    assert_eq!(scope.scope_id, "test_scope");
    assert!(!scope.active);
    assert_eq!(scope.depth, 0);
}

#[test]
fn test_recovery_scope_activation() {
    let config = RecoveryConfig::default();
    let mut scope = RecoveryScope::new("test_scope".to_string(), config);

    scope.activate();
    assert!(scope.active);

    scope.deactivate();
    assert!(!scope.active);
}

#[test]
fn test_recovery_manager_scope_management() {
    let manager = RecoveryManager::new();

    // Enter scope
    assert!(manager.enter_scope("test1".to_string(), None).is_ok());
    assert!(manager.in_recovery_scope());

    // Enter nested scope
    assert!(manager.enter_scope("test2".to_string(), None).is_ok());
    assert!(manager.in_recovery_scope());

    // Exit scopes
    let exited = manager.exit_scope().unwrap();
    assert_eq!(exited, Some("test2".to_string()));
    assert!(manager.in_recovery_scope());

    let exited = manager.exit_scope().unwrap();
    assert_eq!(exited, Some("test1".to_string()));
    assert!(!manager.in_recovery_scope());
}

#[test]
fn test_catch_panic_success() {
    let result = catch_panic(|| {
        42
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_catch_panic_failure() {
    let result = catch_panic(|| -> i32 {
        panic!("Test panic message");
    });

    assert!(result.is_err());
    match result.unwrap_err() {
        CursedError::Recovery { message, .. } => {
            assert!(message.contains("Test panic message"));
        }
        _ => panic!("Expected Recovery error"),
    }
}

#[test]
fn test_catch_panic_with_config() {
    let config = RecoveryConfig {
        timeout: Duration::from_secs(10),
        convert_to_error: true,
        log_recovery: false,
        max_attempts: 1,
        propagate_unrecoverable: false,
        recoverable_categories: vec![PanicCategory::User],
    };

    let result = catch_panic_with_config(|| -> i32 {
        panic!("Configured panic test");
    }, Some(config));

    assert!(result.is_err());
}

#[test]
fn test_panic_to_error_conversion() {
    let panic_info = CursedPanicInfo::new(
        "Test conversion".to_string(),
        PanicSeverity::Recoverable,
        PanicCategory::User,
    );

    let error = panic_to_error(&panic_info);

    match error {
        CursedError::Panic { message, recoverable, .. } => {
            assert_eq!(message, "Test conversion");
            assert!(recoverable);
        }
        _ => panic!("Expected Panic error"),
    }
}

#[test]
fn test_error_to_recovery_action() {
    // Test recoverable panic
    let recoverable_error = CursedError::Panic {
        message: "Recoverable".to_string(),
        panic_id: Some(1),
        recoverable: true,
        source_location: None,
    };

    let action = error_to_recovery_action(recoverable_error);
    match action {
        RecoveryAction::Continue(_) => (), // Expected
        _ => panic!("Expected Continue action"),
    }

    // Test unrecoverable panic
    let unrecoverable_error = CursedError::Panic {
        message: "Fatal".to_string(),
        panic_id: Some(2),
        recoverable: false,
        source_location: None,
    };

    let action = error_to_recovery_action(unrecoverable_error);
    match action {
        RecoveryAction::TerminateGoroutine => (), // Expected
        _ => panic!("Expected TerminateGoroutine action"),
    }
}

#[test]
fn test_error_recoverability_checks() {
    // Test recoverable errors
    let recoverable_panic = CursedError::Panic {
        message: "Recoverable".to_string(),
        panic_id: Some(1),
        recoverable: true,
        source_location: None,
    };
    assert!(is_recoverable_error(&recoverable_panic));

    let runtime_error = CursedError::Runtime("Runtime error".to_string());
    assert!(is_recoverable_error(&runtime_error));

    let io_error = CursedError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found"
    ));
    assert!(is_recoverable_error(&io_error));

    // Test unrecoverable errors
    let unrecoverable_panic = CursedError::Panic {
        message: "Fatal".to_string(),
        panic_id: Some(2),
        recoverable: false,
        source_location: None,
    };
    assert!(!is_recoverable_error(&unrecoverable_panic));

    let compile_error = CursedError::Compile("Compile error".to_string());
    assert!(!is_recoverable_error(&compile_error));

    let parse_error = CursedError::Parse("Parse error".to_string());
    assert!(!is_recoverable_error(&parse_error));
}

#[test]
fn test_recovery_scope_guard() {
    // Test RAII-style scope management
    {
        let _guard = RecoveryScopeGuard::new("test_guard".to_string(), None);
        // Scope is automatically exited when guard is dropped
    }
    
    // Test with custom config
    let config = RecoveryConfig {
        timeout: Duration::from_secs(5),
        ..RecoveryConfig::default()
    };
    
    {
        let _guard = RecoveryScopeGuard::new("test_guard_config".to_string(), Some(config));
        // Scope is automatically exited when guard is dropped
    }
}

#[test]
fn test_panic_runtime_with_recovery() {
    // Initialize both systems
    assert!(initialize_panic_runtime().is_ok());
    assert!(initialize_recovery_manager().is_ok());

    let runtime = get_panic_runtime().unwrap();

    // Test recovery functionality
    let result = runtime.recover(|| {
        "successful operation"
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "successful operation");

    // Test recovery with panic
    let result = runtime.recover(|| -> &str {
        panic!("Test recovery panic");
    });

    assert!(result.is_err());

    // Cleanup
    assert!(shutdown_panic_runtime().is_ok());
}

#[test]
fn test_panic_severity_ordering() {
    assert!(PanicSeverity::Recoverable < PanicSeverity::Critical);
    assert!(PanicSeverity::Critical < PanicSeverity::Fatal);
}

#[test]
fn test_panic_categories() {
    // Test that all categories are distinct
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

    for (i, cat1) in categories.iter().enumerate() {
        for (j, cat2) in categories.iter().enumerate() {
            if i != j {
                assert_ne!(cat1, cat2);
            }
        }
    }
}

#[test]
fn test_recovery_config_defaults() {
    let config = RecoveryConfig::default();

    assert_eq!(config.timeout, Duration::from_secs(30));
    assert!(config.convert_to_error);
    assert!(!config.log_recovery);
    assert_eq!(config.max_attempts, 3);
    assert!(config.propagate_unrecoverable);
    assert!(!config.recoverable_categories.is_empty());
}

#[test]
fn test_panic_runtime_statistics() {
    assert!(initialize_panic_runtime().is_ok());
    
    let runtime = get_panic_runtime().unwrap();
    let stats = runtime.get_statistics().unwrap();

    // Initial stats should be zero
    assert_eq!(stats.total_panics, 0);
    assert_eq!(stats.successful_recoveries, 0);
    assert_eq!(stats.failed_recoveries, 0);

    assert!(shutdown_panic_runtime().is_ok());
}

#[test]
fn test_recovery_manager_statistics() {
    let manager = RecoveryManager::new();
    let stats = manager.get_statistics().unwrap();

    assert_eq!(stats.total_attempts, 0);
    assert_eq!(stats.successful_recoveries, 0);
    assert_eq!(stats.failed_recoveries, 0);
    assert_eq!(stats.recovery_timeouts, 0);
}

#[test]
fn test_nested_recovery_scopes() {
    let manager = RecoveryManager::new();

    // Enter multiple nested scopes
    assert!(manager.enter_scope("scope1".to_string(), None).is_ok());
    assert!(manager.enter_scope("scope2".to_string(), None).is_ok());
    assert!(manager.enter_scope("scope3".to_string(), None).is_ok());

    // Check current scope
    let current = manager.current_scope().unwrap();
    assert!(current.is_some());
    let scope = current.unwrap();
    assert_eq!(scope.scope_id, "scope3");
    assert_eq!(scope.depth, 2); // 0-based depth

    // Exit all scopes
    assert_eq!(manager.exit_scope().unwrap(), Some("scope3".to_string()));
    assert_eq!(manager.exit_scope().unwrap(), Some("scope2".to_string()));
    assert_eq!(manager.exit_scope().unwrap(), Some("scope1".to_string()));
    assert_eq!(manager.exit_scope().unwrap(), None);
}

#[test]
fn test_recovery_scope_recoverability_check() {
    let mut config = RecoveryConfig::default();
    config.recoverable_categories = vec![PanicCategory::User, PanicCategory::Arithmetic];
    
    let scope = RecoveryScope::new("test".to_string(), config);

    assert!(scope.is_recoverable(&PanicCategory::User));
    assert!(scope.is_recoverable(&PanicCategory::Arithmetic));
    assert!(!scope.is_recoverable(&PanicCategory::Memory));
    assert!(!scope.is_recoverable(&PanicCategory::System));
}

#[test]
fn test_panic_runtime_configuration() {
    let custom_config = PanicConfig {
        capture_backtraces: false,
        capture_stack_traces: true,
        max_stack_depth: 50,
        log_to_stderr: false,
        abort_on_fatal: false,
        default_recovery: RecoveryAction::Continue(
            CursedError::Runtime("Custom recovery".to_string())
        ),
        recovery_timeout: Duration::from_secs(10),
        debug_manager: None,
        stack_trace_config: Default::default(),
    };

    let runtime = PanicRuntime::with_config(custom_config);
    assert!(runtime.initialize().is_ok());
    
    // Test config update
    let result = runtime.update_config(|config| {
        config.max_stack_depth = 25;
        config.log_to_stderr = true;
    });
    
    assert!(result.is_ok());
    assert!(runtime.shutdown().is_ok());
}

#[test]
fn test_concurrent_panic_recovery() {
    assert!(initialize_panic_runtime().is_ok());
    assert!(initialize_recovery_manager().is_ok());
    
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                let result = catch_panic(|| {
                    if i % 2 == 0 {
                        42 + i
                    } else {
                        panic!("Thread {} panic", i);
                    }
                });
                
                // Even threads should succeed, odd should fail
                if i % 2 == 0 {
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap(), 42 + i);
                } else {
                    assert!(result.is_err());
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert!(shutdown_panic_runtime().is_ok());
}

#[test] 
fn test_panic_with_metadata() {
    let panic_info = CursedPanicInfo::new(
        "Test with metadata".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User,
    )
    .with_metadata("context".to_string(), "test_context".to_string())
    .with_metadata("thread".to_string(), "main".to_string());

    assert_eq!(panic_info.metadata.get("context"), Some(&"test_context".to_string()));
    assert_eq!(panic_info.metadata.get("thread"), Some(&"main".to_string()));
}

#[test]
fn test_recovery_timeout_behavior() {
    let config = RecoveryConfig {
        timeout: Duration::from_millis(10), // Very short timeout
        ..RecoveryConfig::default()
    };
    
    let scope = RecoveryScope::new("timeout_test".to_string(), config);
    
    // Initially should not be timed out
    assert!(!scope.has_timed_out());
    
    // After a brief sleep, it should be timed out
    thread::sleep(Duration::from_millis(20));
    assert!(scope.has_timed_out());
}

#[test]
fn test_error_display_formatting() {
    let panic_info = CursedPanicInfo::new(
        "Display test".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User,
    )
    .with_location(SourceLocation::new(10, 5))
    .with_goroutine(42);

    let display = format!("{}", panic_info);
    assert!(display.contains("Display test"));
    assert!(display.contains("Critical"));
    assert!(display.contains("User"));
    assert!(display.contains("goroutine #42"));
}

// Gen Z slang panic function testing (can't actually call them since they abort)
#[test]
fn test_gen_z_panic_info_creation() {
    // Test that Gen Z slang panic info is created correctly
    // We can't test the actual panic functions since they abort execution,
    // but we can test panic info creation and formatting
    
    let no_cap_info = CursedPanicInfo::new(
        "no cap: test".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User,
    );
    assert!(no_cap_info.message.contains("no cap"));

    let sus_info = CursedPanicInfo::new(
        "that's sus: test".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User,
    );
    assert!(sus_info.message.contains("that's sus"));

    let cap_info = CursedPanicInfo::new(
        "cap detected: test".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User,
    );
    assert!(cap_info.message.contains("cap detected"));

    let not_vibing_info = CursedPanicInfo::new(
        "not vibing: test".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User,
    );
    assert!(not_vibing_info.message.contains("not vibing"));
}

#[test] 
fn test_llvm_panic_compiler_configuration() {
    let config = PanicCompilerConfig {
        generate_debug_info: true,
        inline_panic_checks: false,
        optimize_recovery: true,
        max_recovery_depth: 16,
    };

    // Test that config can be created and has expected values
    assert!(config.generate_debug_info);
    assert!(!config.inline_panic_checks);
    assert!(config.optimize_recovery);
    assert_eq!(config.max_recovery_depth, 16);
}

#[test]
fn test_llvm_panic_compiler_defaults() {
    let config = PanicCompilerConfig::default();
    
    assert!(config.generate_debug_info);
    assert!(!config.inline_panic_checks);
    assert!(config.optimize_recovery);
    assert_eq!(config.max_recovery_depth, 32);
}
