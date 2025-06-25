/// Comprehensive integration tests for SignalBoost module
use cursed::stdlib::signal_boost::*;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::sync::mpsc;
use std::time::Duration;
use std::thread;

#[test]
fn test_signal_boost_module_initialization() {
    let result = initialize_signal_boost();
    assert!(result.is_ok());
    
    let stats = get_signal_boost_statistics();
    // Module should be initialized but not necessarily have active components
    assert!(stats.active_handlers <= 100); // Reasonable upper bound
}

#[test]
fn test_boost_signal_constants() {
    // Test that all signal constants are properly defined
    assert_eq!(SIGINT.0, 2);
    assert_eq!(SIGTERM.0, 15);
    assert_eq!(SIGHUP.0, 1);
    assert_eq!(SIGQUIT.0, 3);
    assert_eq!(SIGUSR1.0, 10);
    assert_eq!(SIGUSR2.0, 12);
    
    // Test signal names
    assert_eq!(SIGINT.name(), "SIGINT");
    assert_eq!(SIGTERM.name(), "SIGTERM");
    assert_eq!(SIGHUP.name(), "SIGHUP");
}

#[test]
fn test_signal_notification_basic() {
    let signals = vec![SIGUSR1, SIGUSR2];
    let result = notify(&signals);
    assert!(result.is_ok());
    
    let (receiver, mut handle) = result.unwrap();
    assert!(handle.is_active());
    assert_eq!(handle.signals(), &signals);
    
    // Test handle operations
    let new_signals = vec![SIGTERM];
    assert!(handle.reset(new_signals.clone()).is_ok());
    assert_eq!(handle.signals(), &new_signals);
    
    handle.stop();
    assert!(!handle.is_active());
}

#[test]
fn test_signal_handler_registration() {
    let mut handler = SignalHandler::new();
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = Arc::clone(&called);
    
    // Test registering signal handlers
    let result = handler.register(SIGUSR1, move |signal| {
        assert_eq!(signal, SIGUSR1);
        called_clone.store(true, Ordering::SeqCst);
    });
    assert!(result.is_ok());
    
    // Test handler statistics
    let stats = handler.get_statistics();
    assert_eq!(stats.total_signals, 1);
    assert_eq!(stats.total_handlers, 1);
    
    // Test function handler registration
    let called_func = Arc::new(AtomicBool::new(false));
    let called_func_clone = Arc::clone(&called_func);
    
    let result = handler.register_func(SIGUSR2, move || {
        called_func_clone.store(true, Ordering::SeqCst);
    });
    assert!(result.is_ok());
    
    assert_eq!(handler.signal_count(), 2);
    assert_eq!(handler.handler_count(), 2);
    
    // Test unregistering
    assert!(handler.unregister(SIGUSR1).is_ok());
    assert_eq!(handler.signal_count(), 1);
}

#[test]
fn test_signal_handler_priority() {
    let mut handler = SignalHandler::new();
    let execution_order = Arc::new(Mutex::new(Vec::new()));
    
    // Register handlers with different priorities
    let order1 = Arc::clone(&execution_order);
    handler.register_with_priority(SIGUSR1, 10, "high_priority", move |_| {
        order1.lock().unwrap().push("high");
    }).unwrap();
    
    let order2 = Arc::clone(&execution_order);
    handler.register_with_priority(SIGUSR1, 5, "low_priority", move |_| {
        order2.lock().unwrap().push("low");
    }).unwrap();
    
    // Test priority setting
    assert!(handler.set_priority(SIGUSR1, "low_priority", 15).is_ok());
    
    // Test named handler unregistration
    assert!(handler.unregister_named(SIGUSR1, "high_priority").is_ok());
    assert_eq!(handler.handler_count(), 1);
}

#[test]
fn test_graceful_shutdown_basic() {
    let mut shutdown = GracefulShutdown::new();
    let task_executed = Arc::new(AtomicBool::new(false));
    let task_clone = Arc::clone(&task_executed);
    
    // Add shutdown task
    let result = shutdown.add("test_task", move || {
        task_clone.store(true, Ordering::SeqCst);
        Ok(())
    });
    assert!(result.is_ok());
    
    // Test status
    let status = shutdown.status();
    assert!(!status.in_progress);
    assert!(status.completed_tasks.is_empty());
    assert!(status.errors.is_empty());
    
    // Test timeout setting
    shutdown.set_timeout(Duration::from_secs(60));
    assert_eq!(shutdown.options.timeout, Duration::from_secs(60));
}

#[test]
fn test_graceful_shutdown_with_options() {
    let pre_shutdown_called = Arc::new(AtomicBool::new(false));
    let pre_clone = Arc::clone(&pre_shutdown_called);
    let error_called = Arc::new(AtomicBool::new(false));
    let error_clone = Arc::clone(&error_called);
    
    let options = ShutdownOptions {
        timeout: Duration::from_secs(10),
        pre_shutdown_fn: Some(Arc::new(move || {
            pre_clone.store(true, Ordering::SeqCst);
        })),
        error_handler: Some(Arc::new(move |_| {
            error_clone.store(true, Ordering::SeqCst);
        })),
        keep_alive: true,
        sync_shutdown: false,
        signals: vec![SIGINT, SIGTERM],
    };
    
    let shutdown = GracefulShutdown::with_options(options);
    assert_eq!(shutdown.options.timeout, Duration::from_secs(10));
    assert!(shutdown.options.keep_alive);
    assert!(!shutdown.options.sync_shutdown);
}

#[test]
fn test_shutdown_task_group() {
    let call_count = Arc::new(AtomicUsize::new(0));
    
    let count1 = Arc::clone(&call_count);
    let count2 = Arc::clone(&call_count);
    
    let group = ShutdownTaskGroup::new("test_group".to_string(), false)
        .add_task("task1", move || {
            count1.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .add_task("task2", move || {
            count2.fetch_add(1, Ordering::SeqCst);
            Ok(())
        });
    
    assert_eq!(group.tasks.len(), 2);
    assert_eq!(group.name, "test_group");
    assert!(!group.parallel);
    
    let mut shutdown = GracefulShutdown::new();
    assert!(shutdown.add_group(group).is_ok());
}

#[test]
fn test_signal_multiplexer() {
    let mut multiplexer = SignalMultiplexer::new();
    assert_eq!(multiplexer.count(), 0);
    assert!(!multiplexer.is_running());
    
    // Add subscriptions
    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::channel();
    
    let handle1 = multiplexer.add(sender1, &[SIGUSR1]).unwrap();
    let handle2 = multiplexer.add(sender2, &[SIGUSR1, SIGUSR2]).unwrap();
    
    assert_eq!(multiplexer.count(), 2);
    assert!(handle1.is_active());
    assert!(handle2.is_active());
    assert_ne!(handle1.id(), handle2.id());
    
    // Test monitored signals
    let monitored = multiplexer.monitored_signals();
    assert!(monitored.contains(&SIGUSR1));
    assert!(monitored.contains(&SIGUSR2));
    
    // Test statistics
    let stats = multiplexer.get_statistics();
    assert_eq!(stats.active_subscriptions, 2);
    assert_eq!(stats.total_subscriptions, 2);
    
    // Remove subscription
    assert!(multiplexer.remove(handle1.id()).is_ok());
    assert_eq!(multiplexer.count(), 1);
    
    // Remove by handle
    assert!(multiplexer.remove_handle(&handle2).is_ok());
    assert_eq!(multiplexer.count(), 0);
    assert!(!handle2.is_active());
}

#[test]
fn test_signal_actions() {
    // Test ignore action
    let ignore = ignore_action();
    assert!(ignore(SIGUSR1));
    
    // Test custom log action
    let logged = Arc::new(AtomicBool::new(false));
    let logged_clone = Arc::clone(&logged);
    let log_action = custom_log_action(move |_| {
        logged_clone.store(true, Ordering::SeqCst);
    });
    assert!(log_action(SIGUSR1));
    assert!(logged.load(Ordering::SeqCst));
    
    // Test shook action
    let shook = shook_action();
    assert!(shook(SIGUSR1));
    
    // Test chained actions
    let called1 = Arc::new(AtomicBool::new(false));
    let called2 = Arc::new(AtomicBool::new(false));
    
    let called1_clone = Arc::clone(&called1);
    let called2_clone = Arc::clone(&called2);
    
    let action1 = create_action(move |_| {
        called1_clone.store(true, Ordering::SeqCst);
        true
    });
    let action2 = create_action(move |_| {
        called2_clone.store(true, Ordering::SeqCst);
        true
    });
    
    let chained = chain_actions(vec![action1, action2]);
    assert!(chained(SIGUSR1));
    assert!(called1.load(Ordering::SeqCst));
    assert!(called2.load(Ordering::SeqCst));
}

#[test]
fn test_signal_filtering() {
    let (sender, input) = mpsc::channel();
    
    // Test basic filtering
    let output = filter_signals(input, |signal| signal == SIGUSR1);
    
    sender.send(SIGUSR1).unwrap();
    sender.send(SIGUSR2).unwrap();
    sender.send(SIGUSR1).unwrap();
    drop(sender);
    
    let mut received = Vec::new();
    while let Ok(signal) = output.recv() {
        received.push(signal);
    }
    
    assert_eq!(received.len(), 2);
    assert!(received.iter().all(|&s| s == SIGUSR1));
}

#[test]
fn test_signal_throttling() {
    let (sender, input) = mpsc::channel();
    let output = throttle_signals(input, Duration::from_millis(100));
    
    // Send signals rapidly
    sender.send(SIGUSR1).unwrap();
    sender.send(SIGUSR1).unwrap();
    sender.send(SIGUSR1).unwrap();
    
    // Should only receive one initially
    let signal = output.recv_timeout(Duration::from_millis(50)).unwrap();
    assert_eq!(signal, SIGUSR1);
    
    // Should timeout on second signal (throttled)
    assert!(output.recv_timeout(Duration::from_millis(50)).is_err());
    
    drop(sender);
}

#[test]
fn test_signal_debouncing() {
    let (sender, input) = mpsc::channel();
    let output = debounce_signals(input, Duration::from_millis(50));
    
    // Send signals rapidly
    sender.send(SIGUSR1).unwrap();
    thread::sleep(Duration::from_millis(10));
    sender.send(SIGUSR1).unwrap();
    thread::sleep(Duration::from_millis(10));
    sender.send(SIGUSR1).unwrap();
    
    // Wait for debounce interval
    thread::sleep(Duration::from_millis(100));
    
    // Should only receive one signal (debounced)
    let signal = output.recv_timeout(Duration::from_millis(50)).unwrap();
    assert_eq!(signal, SIGUSR1);
    
    // Should not receive another
    assert!(output.recv_timeout(Duration::from_millis(50)).is_err());
    
    drop(sender);
}

#[test]
fn test_vibe_checker() {
    let health_status = Arc::new(AtomicBool::new(true));
    let health_clone = Arc::clone(&health_status);
    
    let mut checker = vibe_check(SIGUSR1, move || {
        health_clone.load(Ordering::SeqCst)
    });
    
    assert!(!checker.is_running());
    assert!(checker.get_status()); // Default true
    
    // Test start/stop
    assert!(checker.start().is_ok());
    assert!(checker.is_running());
    
    assert!(checker.stop().is_ok());
    assert!(!checker.is_running());
    
    // Test double start error
    assert!(checker.start().is_ok());
    assert!(checker.start().is_err());
    checker.stop().unwrap();
}

#[test]
fn test_yeet_on_signal() {
    // Note: We can't test the actual exit behavior in a unit test
    let result = yeet_on_signal(SIGUSR1, "Test yeet message");
    assert!(result.is_ok());
    
    let mut handle = result.unwrap();
    handle.stop(); // Clean up immediately
}

#[test]
fn test_no_cap_reload_config() {
    let reload_called = Arc::new(AtomicBool::new(false));
    let reload_clone = Arc::clone(&reload_called);
    
    let result = no_cap_reload_config("test_config.toml", move || {
        reload_clone.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    assert!(result.is_ok());
    
    let mut handle = result.unwrap();
    handle.stop(); // Clean up
}

#[test]
fn test_bussin_logger() {
    let signals = vec![SIGUSR1, SIGUSR2];
    let mut logger = BussinLogger::new(signals.clone());
    
    assert!(!logger.is_running());
    assert_eq!(logger.signals, signals);
    
    assert!(logger.start().is_ok());
    assert!(logger.is_running());
    
    assert!(logger.stop().is_ok());
    assert!(!logger.is_running());
}

#[test]
fn test_fr_fr_reporter() {
    let signals = vec![SIGUSR1, SIGUSR2];
    let mut reporter = FrFrReporter::new(signals.clone(), Duration::from_secs(1));
    
    let counts = reporter.get_counts();
    assert!(counts.is_empty());
    
    // Test start/stop (without actually triggering signals)
    assert!(reporter.start(signals).is_ok());
    assert!(reporter.stop().is_ok());
}

#[test]
fn test_signal_filter_chain() {
    let (sender, input) = mpsc::channel();
    
    let chain = SignalFilterChain::new()
        .add_predicate_filter(|signal| signal == SIGUSR1 || signal == SIGUSR2)
        .add_deduplicate()
        .add_throttle(Duration::from_millis(10));
    
    let output = chain.apply(input);
    
    sender.send(SIGUSR1).unwrap();
    sender.send(SIGUSR1).unwrap();  // Should be deduplicated
    sender.send(SIGTERM).unwrap();  // Should be filtered out
    sender.send(SIGUSR2).unwrap();
    drop(sender);
    
    thread::sleep(Duration::from_millis(50));
    
    let mut received = Vec::new();
    while let Ok(signal) = output.recv_timeout(Duration::from_millis(10)) {
        received.push(signal);
    }
    
    // Should receive SIGUSR1 and SIGUSR2 (SIGTERM filtered, duplicate SIGUSR1 removed)
    assert!(received.len() >= 1);
    assert!(received.contains(&SIGUSR1) || received.contains(&SIGUSR2));
}

#[test]
fn test_error_types() {
    use crate::stdlib::signal_boost::error::*;
    
    // Test error creation functions
    let invalid_signal_err = invalid_signal("Test invalid signal");
    assert!(matches!(invalid_signal_err, SignalBoostError::InvalidSignal(_)));
    
    let system_err = system_error("Test system error");
    assert!(matches!(system_err, SignalBoostError::SystemError(_)));
    
    let permission_err = permission_denied("Test permission error");
    assert!(matches!(permission_err, SignalBoostError::PermissionDenied(_)));
    
    let timeout_err = timeout_error("Test timeout");
    assert!(matches!(timeout_err, SignalBoostError::Timeout(_)));
    
    // Test error display
    assert!(invalid_signal_err.to_string().contains("Invalid signal"));
    assert!(system_err.to_string().contains("System error"));
}

#[test]
fn test_concurrent_signal_handling() {
    let mut handler = SignalHandler::new();
    let call_count = Arc::new(AtomicUsize::new(0));
    
    // Register multiple handlers for the same signal
    for i in 0..5 {
        let count_clone = Arc::clone(&call_count);
        handler.register_with_priority(SIGUSR1, i, &format!("handler_{}", i), move |_| {
            count_clone.fetch_add(1, Ordering::SeqCst);
        }).unwrap();
    }
    
    assert_eq!(handler.handler_count(), 5);
    
    let stats = handler.get_statistics();
    assert_eq!(stats.total_handlers, 5);
    assert_eq!(stats.total_signals, 1);
}

#[test]
fn test_module_statistics() {
    // Initialize module
    initialize_signal_boost().unwrap();
    
    let stats = get_signal_boost_statistics();
    
    // Should have reasonable values
    assert!(stats.active_handlers < 1000);
    assert!(stats.active_multiplexers < 1000);
    assert!(stats.graceful_shutdowns < 1000);
    // signals_processed starts at 0 and may increment
}

#[test]
fn test_comprehensive_signal_workflow() {
    // This test demonstrates a complete signal handling workflow
    
    // 1. Set up signal handler
    let mut handler = SignalHandler::new();
    let handler_called = Arc::new(AtomicBool::new(false));
    let handler_clone = Arc::clone(&handler_called);
    
    handler.register(SIGUSR1, move |_| {
        handler_clone.store(true, Ordering::SeqCst);
    }).unwrap();
    
    // 2. Set up graceful shutdown
    let mut shutdown = GracefulShutdown::new();
    let shutdown_called = Arc::new(AtomicBool::new(false));
    let shutdown_clone = Arc::clone(&shutdown_called);
    
    shutdown.add("test_cleanup", move || {
        shutdown_clone.store(true, Ordering::SeqCst);
        Ok(())
    }).unwrap();
    
    // 3. Set up multiplexer
    let mut multiplexer = SignalMultiplexer::new();
    let (sender, _receiver) = mpsc::channel();
    let _handle = multiplexer.add(sender, &[SIGUSR2]).unwrap();
    
    // 4. Set up filtering
    let (filter_sender, filter_input) = mpsc::channel();
    let _filtered_output = filter_signals(filter_input, |signal| signal == SIGUSR1);
    
    // 5. Verify everything is set up correctly
    assert_eq!(handler.signal_count(), 1);
    assert!(!shutdown.is_shutdown_in_progress());
    assert_eq!(multiplexer.count(), 1);
    
    // Clean up
    drop(filter_sender);
    multiplexer.clear().unwrap();
}
