/// Comprehensive test suite for IPC module functionality

#[cfg(test)]
mod tests {
    use cursed::stdlib::ipc::*;
    use cursed::stdlib::ipc::signals::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use std::sync::mpsc;

    #[test]
    fn test_signal_boost_initialization() {
        assert!(initialize_signal_boost().is_ok());
        assert!(cleanup_signal_boost().is_ok());
    }

    #[test]
    fn test_boost_signal_properties() {
        let sig = BoostSignal::SIGINT;
        assert_eq!(sig.signal_number(), 2);
        assert_eq!(sig.name(), "SIGINT");
        assert_eq!(format!("{}", sig), "SIGINT (2)");
    }

    #[test]
    fn test_signal_handler_registration() {
        let mut handler = SignalHandler::new();
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        
        handler.register(BoostSignal::SIGINT, move |_| {
            *called_clone.lock().unwrap() = true;
        });
        
        // Start and stop the handler
        assert!(handler.handle().is_ok());
        assert!(handler.stop().is_ok());
    }

    #[test]
    fn test_graceful_shutdown() {
        let mut shutdown = GracefulShutdown::new();
        let executed = Arc::new(Mutex::new(false));
        let executed_clone = executed.clone();
        
        shutdown.add("test_task", move || {
            *executed_clone.lock().unwrap() = true;
            Ok(())
        });
        
        // Configure with options
        shutdown.with_options(ShutdownOptions {
            timeout: Duration::from_secs(5),
            ..Default::default()
        });
        
        // Manual shutdown test
        assert!(shutdown.shutdown().is_ok());
        
        // Verify task was executed
        assert!(*executed.lock().unwrap());
    }

    #[test]
    fn test_signal_multiplexer() {
        let mut mux = SignalMultiplexer::new();
        let (tx1, _rx1) = mpsc::channel();
        let (tx2, _rx2) = mpsc::channel();
        
        let id1 = mux.add(tx1, &[BoostSignal::SIGINT]);
        let id2 = mux.add(tx2, &[BoostSignal::SIGTERM, BoostSignal::SIGHUP]);
        
        assert_eq!(mux.count(), 2);
        
        mux.remove(id1);
        assert_eq!(mux.count(), 1);
        
        mux.remove(id2);
        assert_eq!(mux.count(), 0);
    }

    #[test]
    fn test_signal_filtering() {
        let (tx, rx) = mpsc::channel();
        
        // Send test signals
        tx.send(BoostSignal::SIGINT).unwrap();
        tx.send(BoostSignal::SIGTERM).unwrap();
        tx.send(BoostSignal::SIGUSR1).unwrap();
        drop(tx);
        
        // Filter to only allow SIGINT and SIGTERM
        let filtered = filter_signals(rx, |sig| {
            sig == BoostSignal::SIGINT || sig == BoostSignal::SIGTERM
        });
        
        let signals: Vec<_> = filtered.iter().collect();
        assert_eq!(signals.len(), 2);
        assert!(signals.contains(&BoostSignal::SIGINT));
        assert!(signals.contains(&BoostSignal::SIGTERM));
        assert!(!signals.contains(&BoostSignal::SIGUSR1));
    }

    #[test]
    fn test_signal_throttling() {
        let (tx, rx) = mpsc::channel();
        
        // Send rapid signals
        for _ in 0..5 {
            tx.send(BoostSignal::SIGINT).unwrap();
        }
        drop(tx);
        
        // Throttle with 100ms interval
        let throttled = throttle_signals(rx, Duration::from_millis(100));
        
        // Should receive fewer signals due to throttling
        let start = std::time::Instant::now();
        let signals: Vec<_> = throttled.iter().collect();
        let duration = start.elapsed();
        
        assert!(!signals.is_empty());
        assert!(duration >= Duration::from_millis(50)); // Some time should pass
    }

    #[test]
    fn test_vibe_checker() {
        let mut checker = vibe_check(BoostSignal::SIGUSR1, || true);
        
        assert!(checker.get_status()); // Should be healthy
        assert!(checker.start().is_ok());
        assert!(checker.stop().is_ok());
    }

    #[test]
    fn test_signal_actions() {
        // Test ignore action
        assert!(ignore_action(BoostSignal::SIGINT));
        
        // Test exit with code action
        let exit_action = exit_with_code_action(42);
        // We can't actually test the exit action without terminating the test
        assert!(exit_action.is_some());
    }

    #[test]
    fn test_ipc_config_default() {
        let config = IpcConfig::default();
        assert_eq!(config.max_message_size, 65536);
        assert_eq!(config.max_queue_size, 1000);
        assert_eq!(config.default_permissions, 0o666);
        assert_eq!(config.pipe_buffer_size, 8192);
    }

    #[test]
    fn test_ipc_initialization() {
        assert!(initialize_ipc().is_ok());
        assert!(cleanup_ipc().is_ok());
    }

    #[test]
    fn test_error_helper_functions() {
        let err = connection_error("localhost:8080", "Connection refused");
        assert_eq!(err.category(), "ConnectionError");
        
        let err = invalid_operation("test_op", "Invalid parameters");
        assert_eq!(err.category(), "General");
        
        let err = ipc_error("General error");
        assert_eq!(err.category(), "General");
        
        let err = out_of_resources("memory", "Not enough space");
        assert_eq!(err.category(), "General");
    }

    #[test]
    fn test_notify_handle() {
        let (tx, _rx) = mpsc::channel();
        let mut handle = notify(tx, &[BoostSignal::SIGINT, BoostSignal::SIGTERM]);
        
        // Test reset functionality
        assert!(handle.reset(&[BoostSignal::SIGHUP]).is_ok());
        
        // Stop the handle
        handle.stop();
    }

    #[test]
    fn test_shutdown_status() {
        let shutdown = GracefulShutdown::new();
        let status = shutdown.status();
        
        assert!(!status.in_progress);
        assert_eq!(status.completed_tasks.len(), 0);
        assert_eq!(status.remaining_tasks.len(), 0);
        assert_eq!(status.errors.len(), 0);
        assert!(status.shutdown_triggered_by.is_none());
    }

    #[test]
    fn test_complex_shutdown_workflow() {
        let mut shutdown = GracefulShutdown::new();
        let counter = Arc::new(Mutex::new(0));
        
        // Add multiple tasks
        for i in 0..3 {
            let counter_clone = counter.clone();
            shutdown.add_with_order(&format!("task_{}", i), i, move || {
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
                Ok(())
            });
        }
        
        // Execute shutdown
        assert!(shutdown.shutdown().is_ok());
        
        // Verify all tasks executed
        assert_eq!(*counter.lock().unwrap(), 3);
        
        let status = shutdown.status();
        assert_eq!(status.completed_tasks.len(), 3);
        assert_eq!(status.errors.len(), 0);
    }

    #[test]
    fn test_signal_boost_genZ_features() {
        // Test yeet_on_signal (creates handle but doesn't actually exit)
        let handle = yeet_on_signal(BoostSignal::SIGQUIT, "Test yeet message");
        // Handle creation should succeed
        drop(handle);
        
        // Test nocap_reload_config
        let reloaded = Arc::new(Mutex::new(false));
        let reloaded_clone = reloaded.clone();
        
        let handle = nocap_reload_config("/tmp/test_config.json", move || {
            *reloaded_clone.lock().unwrap() = true;
            Ok(())
        });
        
        // Handle creation should succeed
        drop(handle);
    }

    #[test]
    fn test_signal_debouncing() {
        let (tx, rx) = mpsc::channel();
        
        // Send rapid signals
        for _ in 0..3 {
            tx.send(BoostSignal::SIGINT).unwrap();
        }
        drop(tx);
        
        // Debounce with 50ms interval
        let debounced = debounce_signals(rx, Duration::from_millis(50));
        
        // Should receive only the last signal after debounce period
        std::thread::sleep(Duration::from_millis(100));
        
        let signals: Vec<_> = debounced.try_iter().collect();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0], BoostSignal::SIGINT);
    }

    #[test]
    fn test_ipc_traits_and_stats() {
        let stats = IpcResourceStats::default();
        assert_eq!(stats.bytes_read, 0);
        assert_eq!(stats.bytes_written, 0);
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
        assert_eq!(stats.connections, 0);
    }

    #[test]
    fn test_comprehensive_signal_setup() {
        // Test a complete signal handling workflow
        assert!(initialize_signal_boost().is_ok());
        
        let mut handler = SignalHandler::new();
        let signal_received = Arc::new(Mutex::new(None));
        let signal_received_clone = signal_received.clone();
        
        handler
            .register(BoostSignal::SIGUSR1, move |sig| {
                *signal_received_clone.lock().unwrap() = Some(sig);
            })
            .enable_debug(true)
            .set_priority(BoostSignal::SIGUSR1, 10);
        
        assert!(handler.handle().is_ok());
        
        // Set up graceful shutdown
        let mut shutdown = GracefulShutdown::new().with_options(ShutdownOptions {
            timeout: Duration::from_secs(1),
            signals: vec![BoostSignal::SIGTERM],
            ..Default::default()
        });
        
        let shutdown_executed = Arc::new(Mutex::new(false));
        let shutdown_executed_clone = shutdown_executed.clone();
        
        shutdown.add("cleanup", move || {
            *shutdown_executed_clone.lock().unwrap() = true;
            Ok(())
        });
        
        assert!(shutdown.start().is_ok());
        
        // Cleanup
        assert!(handler.stop().is_ok());
        assert!(cleanup_signal_boost().is_ok());
    }
}
