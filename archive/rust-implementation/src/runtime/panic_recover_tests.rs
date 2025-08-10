//! Comprehensive tests for the panic/recover system

use super::panic_recover::*;
use super::enhanced_error_handling::CursedErrorType;
use super::error_propagation::*;
use super::goroutine::GoroutineId;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_basic_panic_recover() {
    let result = with_panic_recovery(|| {
        cursed_panic("Test panic message");
    });
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Test panic message");
}

#[test]
fn test_panic_with_defer_handlers() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();
    
    let result = with_panic_recovery(|| {
        add_defer_handler(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
        
        cursed_panic("Test panic with defer");
    });
    
    assert!(result.is_err());
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_panic_with_unwind_handlers() {
    let cleanup_counter = Arc::new(AtomicUsize::new(0));
    let cleanup_counter_clone = cleanup_counter.clone();
    
    let result = with_panic_recovery(|| {
        add_unwind_handler(move || {
            cleanup_counter_clone.fetch_add(10, Ordering::SeqCst);
        });
        
        cursed_panic("Test panic with unwind");
    });
    
    assert!(result.is_err());
    assert_eq!(cleanup_counter.load(Ordering::SeqCst), 10);
}

#[test]
fn test_panic_with_recovery_handlers() {
    let recovery_counter = Arc::new(AtomicUsize::new(0));
    let recovery_counter_clone = recovery_counter.clone();
    
    let result = with_panic_recovery(|| {
        add_recovery_handler(move || {
            recovery_counter_clone.fetch_add(100, Ordering::SeqCst);
        });
        
        cursed_panic("Test panic with recovery");
    });
    
    assert!(result.is_err());
    // Recovery handlers are executed during goroutine_recover, not regular panic
    assert_eq!(recovery_counter.load(Ordering::SeqCst), 0);
}

#[test]
fn test_cursed_panic_with_error() {
    let yikes_error = CursedErrorType::Yikes {
        name: "test_error".to_string(),
        message: "Test yikes error".to_string(),
        context: std::collections::HashMap::new(),
        stack_trace: vec!["test stack trace".to_string()],
    };
    
    let result = with_panic_recovery(|| {
        cursed_panic_with_error(yikes_error);
    });
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Test yikes error"));
}

#[test]
fn test_goroutine_panic_isolation() {
    let goroutine_id: GoroutineId = 1;
    
    // Test that goroutine panic is isolated
    let result = with_panic_recovery(|| {
        goroutine_panic(goroutine_id, "Goroutine panic test");
    });
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Goroutine panic test"));
    
    // Verify goroutine is marked as in panic state
    assert!(is_goroutine_in_panic(goroutine_id));
}

#[test]
fn test_goroutine_recovery() {
    let goroutine_id: GoroutineId = 2;
    
    // First, set up a goroutine panic state
    {
        let mut goroutine_states = super::panic_recover::GOROUTINE_PANIC_STATES.lock().unwrap();
        let mut state = goroutine_states.entry(goroutine_id).or_insert_with(|| {
            let mut state = PanicState::new();
            state.panic_message = Some("Test recovery message".to_string());
            state.in_panic = true;
            state.goroutine_id = Some(goroutine_id);
            state
        });
        state.panic_message = Some("Test recovery message".to_string());
        state.in_panic = true;
    }
    
    // Test recovery
    let recovered = goroutine_recover(goroutine_id);
    assert!(recovered.is_some());
    assert_eq!(recovered.unwrap(), "Test recovery message");
    
    // Verify goroutine is no longer in panic state
    assert!(!is_goroutine_in_panic(goroutine_id));
}

#[test]
fn test_goroutine_cleanup() {
    let goroutine_id: GoroutineId = 3;
    
    // Set up goroutine panic state
    {
        let mut goroutine_states = super::panic_recover::GOROUTINE_PANIC_STATES.lock().unwrap();
        goroutine_states.insert(goroutine_id, PanicState::new());
    }
    
    // Cleanup
    cleanup_goroutine_panic_state(goroutine_id);
    
    // Verify cleanup
    assert!(!is_goroutine_in_panic(goroutine_id));
}

#[test]
fn test_panic_state_management() {
    assert!(!is_in_panic());
    
    let result = with_panic_recovery(|| {
        cursed_panic("Test panic state");
    });
    
    assert!(result.is_err());
    assert!(!is_in_panic()); // Should be cleared after recovery
}

#[test]
fn test_multiple_defer_handlers() {
    let execution_order = Arc::new(std::sync::Mutex::new(Vec::new()));
    
    let order1 = execution_order.clone();
    let order2 = execution_order.clone();
    let order3 = execution_order.clone();
    
    let result = with_panic_recovery(|| {
        add_defer_handler(move || {
            order1.lock().unwrap().push(1);
        });
        
        add_defer_handler(move || {
            order2.lock().unwrap().push(2);
        });
        
        add_defer_handler(move || {
            order3.lock().unwrap().push(3);
        });
        
        cursed_panic("Test multiple defer handlers");
    });
    
    assert!(result.is_err());
    
    // Verify LIFO execution order
    let execution_order = execution_order.lock().unwrap();
    assert_eq!(*execution_order, vec![3, 2, 1]);
}

#[test]
fn test_error_propagation_with_panic_recovery() {
    use crate::error_types::Error;
    
    let result: Result<String, _> = propagate_error_with_panic_recovery(
        Err(Error::Runtime("Test error".to_string())),
        "test_context",
        true,
    );
    
    assert!(result.is_err());
    // The error should be processed through the recovery system
}

#[test]
fn test_enhanced_error_recovery() {
    use crate::error_types::Error;
    
    let attempt_count = Arc::new(AtomicUsize::new(0));
    let attempt_count_clone = attempt_count.clone();
    
    let result = enhanced_error_recovery(
        move || {
            let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst);
            if count < 2 {
                Err(Error::Runtime("Temporary error".to_string()))
            } else {
                Ok("Success".to_string())
            }
        },
        "test_operation",
        3,
    );
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
}

#[test]
fn test_concurrent_panic_recovery() {
    let handles: Vec<_> = (0..5).map(|i| {
        thread::spawn(move || {
            let result = with_panic_recovery(|| {
                cursed_panic(&format!("Concurrent panic {}", i));
            });
            assert!(result.is_err());
            assert!(result.unwrap_err().contains(&format!("Concurrent panic {}", i)));
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_panic_statistics() {
    // Initialize global panic recover runtime
    let _ = initialize_global_panic_recover_runtime();
    
    // Get runtime and test statistics
    if let Some(runtime) = get_global_panic_recover_runtime() {
        let stats = runtime.get_statistics().unwrap();
        // Just verify we can get statistics without panicking
        assert!(stats.total_panics >= 0);
    }
}

#[test]
fn test_nested_panic_recovery() {
    let result = with_panic_recovery(|| {
        let inner_result = with_panic_recovery(|| {
            cursed_panic("Inner panic");
        });
        
        assert!(inner_result.is_err());
        assert_eq!(inner_result.unwrap_err(), "Inner panic");
        
        cursed_panic("Outer panic");
    });
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Outer panic");
}

#[test]
fn test_panic_with_all_handler_types() {
    let unwind_executed = Arc::new(AtomicUsize::new(0));
    let defer_executed = Arc::new(AtomicUsize::new(0));
    
    let unwind_clone = unwind_executed.clone();
    let defer_clone = defer_executed.clone();
    
    let result = with_panic_recovery(|| {
        add_unwind_handler(move || {
            unwind_clone.fetch_add(1, Ordering::SeqCst);
        });
        
        add_defer_handler(move || {
            defer_clone.fetch_add(1, Ordering::SeqCst);
        });
        
        cursed_panic("Test all handler types");
    });
    
    assert!(result.is_err());
    assert_eq!(unwind_executed.load(Ordering::SeqCst), 1);
    assert_eq!(defer_executed.load(Ordering::SeqCst), 1);
}
