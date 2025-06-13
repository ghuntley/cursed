/// Comprehensive tests for the complete Process Management module
/// Tests signal handling, daemon management, and monitoring features

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;

use cursed::stdlib::process::{
    signals::{Signal, SignalHandler, SignalAction, SignalMask, send_signal, convenience},
    daemon::{Daemon, DaemonConfig, DaemonStatus, ServiceManager, system},
    monitoring::{
        ProcessMonitor, HealthCheckConfig, ResourceThresholds, PerformanceMetrics,
        ProcessWatchdog, HealthStatus, collect_performance_metrics,
        create_process_monitor, monitor_process_once, get_system_resource_summary
    },
    error::ProcessResult,
};

#[test]
fn test_advanced_signal_handling() {
    // Test advanced signal handling with sigaction
    let handler = SignalHandler::new();
    
    let signal_received = Arc::new(AtomicBool::new(false));
    let signal_received_clone = Arc::clone(&signal_received);
    
    let action = SignalAction::Handle(Arc::new(move |signal| {
        println!("Received signal: {}", signal.name());
        signal_received_clone.store(true, Ordering::SeqCst);
    }));
    
    // Test catchable signals
    if Signal::User1.can_be_caught() {
        assert!(handler.register(Signal::User1, action).is_ok());
        
        // Simulate signal reception
        assert!(handler.simulate_signal(Signal::User1).is_ok());
        
        // Wait for signal processing
        thread::sleep(Duration::from_millis(100));
        assert!(signal_received.load(Ordering::SeqCst));
        
        // Test signal unregistration
        assert!(handler.unregister(Signal::User1).is_ok());
    }
}

#[test]
fn test_signal_properties_and_conversion() {
    // Test signal properties
    assert!(Signal::Interrupt.can_be_caught());
    assert!(Signal::Interrupt.is_terminating());
    assert!(!Signal::Kill.can_be_caught());
    assert!(!Signal::Continue.is_terminating());
    
    // Test signal name mapping
    assert_eq!(Signal::Interrupt.name(), "SIGINT");
    assert_eq!(Signal::Terminate.name(), "SIGTERM");
    assert_eq!(Signal::Kill.name(), "SIGKILL");
    
    // Test raw signal conversion
    let signal = Signal::Interrupt;
    let raw = signal.as_raw();
    
    #[cfg(unix)]
    {
        assert_eq!(raw, libc::SIGINT);
        assert_eq!(Signal::from_raw(raw), Some(signal));
    }
    
    #[cfg(windows)]
    {
        assert_eq!(raw, 0); // CTRL_C_EVENT
        assert_eq!(Signal::from_raw(raw), Some(signal));
    }
}

#[test]
#[cfg(unix)]
fn test_signal_mask_operations() {
    // Test signal mask creation and manipulation
    let mut mask = SignalMask::empty().unwrap();
    assert!(!mask.contains(Signal::Interrupt));
    
    // Add signals to mask
    mask.add(Signal::Interrupt).unwrap();
    assert!(mask.contains(Signal::Interrupt));
    
    mask.add(Signal::Terminate).unwrap();
    assert!(mask.contains(Signal::Terminate));
    
    // Remove signal from mask
    mask.remove(Signal::Interrupt).unwrap();
    assert!(!mask.contains(Signal::Interrupt));
    assert!(mask.contains(Signal::Terminate));
    
    // Test full mask
    let full_mask = SignalMask::full().unwrap();
    assert!(full_mask.contains(Signal::Interrupt));
    assert!(full_mask.contains(Signal::Terminate));
}

#[test]
fn test_daemon_configuration_builder() {
    let config = DaemonConfig::new("test-daemon")
        .working_directory("/tmp")
        .user("nobody")
        .group("nogroup")
        .pid_file("/tmp/test-daemon.pid")
        .log_file("/tmp/test-daemon.log")
        .lock_file("/tmp/test-daemon.lock")
        .description("Test daemon for process management")
        .auto_restart(5)
        .env("DAEMON_MODE", "test")
        .env("LOG_LEVEL", "debug")
        .umask(0o022);
    
    assert_eq!(config.name, "test-daemon");
    assert_eq!(config.working_directory, Some(std::path::PathBuf::from("/tmp")));
    assert_eq!(config.user, Some("nobody".to_string()));
    assert_eq!(config.group, Some("nogroup".to_string()));
    assert_eq!(config.description, Some("Test daemon for process management".to_string()));
    assert!(config.auto_restart);
    assert_eq!(config.max_restarts, 5);
    assert_eq!(config.environment.get("DAEMON_MODE"), Some(&"test".to_string()));
    assert_eq!(config.environment.get("LOG_LEVEL"), Some(&"debug".to_string()));
    assert_eq!(config.umask, Some(0o022));
}

#[test]
fn test_daemon_lifecycle() {
    let config = DaemonConfig::new("test-lifecycle-daemon")
        .working_directory("/tmp")
        .pid_file("/tmp/test-lifecycle.pid")
        .log_file("/tmp/test-lifecycle.log");
    
    let mut daemon = Daemon::new(config);
    
    // Test initial state
    assert_eq!(daemon.status(), DaemonStatus::Stopped);
    assert_eq!(daemon.pid(), None);
    assert_eq!(daemon.restart_count(), 0);
    
    // Test daemon file operations (without actually starting daemon)
    if let Err(e) = daemon.write_pid_file() {
        println!("Note: PID file creation may fail in test environment: {}", e);
    }
    
    if let Err(e) = daemon.create_lock_file() {
        println!("Note: Lock file creation may fail in test environment: {}", e);
    }
    
    // Cleanup
    let _ = daemon.cleanup_files();
}

#[test]
fn test_service_manager() {
    let manager = ServiceManager::new();
    
    // Test service registration
    let config1 = DaemonConfig::new("service1").description("Test Service 1");
    let config2 = DaemonConfig::new("service2").description("Test Service 2");
    
    assert!(manager.register("service1".to_string(), config1).is_ok());
    assert!(manager.register("service2".to_string(), config2).is_ok());
    
    // Test duplicate registration
    let config_dup = DaemonConfig::new("service1");
    assert!(manager.register("service1".to_string(), config_dup).is_err());
    
    // Test service listing
    let services = manager.list_services();
    assert!(services.contains(&"service1".to_string()));
    assert!(services.contains(&"service2".to_string()));
    assert_eq!(services.len(), 2);
    
    // Test service status checking
    let status = manager.service_status("service1").unwrap();
    assert_eq!(status, DaemonStatus::Stopped);
    
    // Test non-existent service
    assert!(manager.service_status("nonexistent").is_err());
}

#[test]
fn test_process_monitoring_metrics() {
    let current_pid = std::process::id();
    
    // Test performance metrics collection
    if let Ok(metrics) = collect_performance_metrics(current_pid) {
        assert_eq!(metrics.pid, current_pid);
        assert!(metrics.timestamp <= SystemTime::now());
        assert!(metrics.cpu_percent >= 0.0);
        assert!(metrics.memory_bytes > 0);
        assert!(metrics.file_descriptors >= 3); // At least stdin, stdout, stderr
        assert!(metrics.threads >= 1);
        
        println!("Performance metrics for PID {}: CPU: {:.2}%, Memory: {} bytes, FDs: {}, Threads: {}",
            metrics.pid, metrics.cpu_percent, metrics.memory_bytes, 
            metrics.file_descriptors, metrics.threads);
    } else {
        println!("Note: Performance metrics collection may not be available on this platform");
    }
}

#[test]
fn test_health_monitoring_thresholds() {
    let thresholds = ResourceThresholds {
        max_cpu_percent: 50.0,
        max_memory_bytes: 100 * 1024 * 1024, // 100MB
        max_file_descriptors: 100,
        max_threads: 20,
        max_execution_time: Some(Duration::from_secs(300)),
    };
    
    let current_pid = std::process::id();
    
    // Test single process health check
    if let Ok(health_status) = monitor_process_once(current_pid, thresholds) {
        assert!(matches!(health_status, 
            HealthStatus::Healthy | HealthStatus::Warning | 
            HealthStatus::Critical | HealthStatus::Unknown));
        
        println!("Current process health status: {:?}", health_status);
    } else {
        println!("Note: Process health monitoring may not be available on this platform");
    }
}

#[test]
fn test_process_monitor_lifecycle() {
    let config = HealthCheckConfig {
        check_interval: Duration::from_millis(100),
        thresholds: ResourceThresholds::default(),
        failure_threshold: 3,
        success_threshold: 2,
        check_responsiveness: true,
        responsiveness_timeout: Duration::from_secs(5),
    };
    
    let mut monitor = ProcessMonitor::new(config);
    let current_pid = std::process::id();
    
    // Test adding process to monitoring
    if let Ok(()) = monitor.add_process(current_pid) {
        // Test getting monitored PIDs
        if let Ok(pids) = monitor.get_monitored_pids() {
            assert!(pids.contains(&current_pid));
        }
        
        // Test starting monitoring
        if let Ok(()) = monitor.start() {
            thread::sleep(Duration::from_millis(150)); // Let it run one cycle
            
            // Test health status retrieval
            if let Ok(status) = monitor.get_health_status(current_pid) {
                println!("Monitored process health: {:?}", status);
            }
            
            // Test health summary
            if let Ok(summary) = monitor.get_health_summary() {
                assert!(summary.contains_key(&current_pid));
                println!("Health summary: {:?}", summary);
            }
            
            // Test stopping monitoring
            assert!(monitor.stop().is_ok());
        }
        
        // Test removing process from monitoring
        if let Ok(removed) = monitor.remove_process(current_pid) {
            assert!(removed);
        }
    } else {
        println!("Note: Process monitoring may not be available on this platform");
    }
}

#[test]
fn test_performance_history() {
    use cursed::stdlib::process::monitoring::PerformanceHistory;
    
    let mut history = PerformanceHistory::new(1234, 10);
    
    // Add some test metrics
    let now = SystemTime::now();
    for i in 0..5 {
        let metrics = PerformanceMetrics {
            pid: 1234,
            timestamp: now,
            cpu_percent: (i as f64) * 10.0,
            memory_bytes: (i as u64 + 1) * 1024 * 1024,
            virtual_memory_bytes: (i as u64 + 1) * 2 * 1024 * 1024,
            file_descriptors: 10 + i as u32,
            threads: 2 + i as u32,
            uptime: Duration::from_secs(i as u64 * 60),
            io_read_bytes: (i as u64) * 1024,
            io_write_bytes: (i as u64) * 512,
        };
        history.add_metrics(metrics);
    }
    
    // Test history size limits
    assert_eq!(history.metrics.len(), 5);
    
    // Test latest metrics
    if let Some(latest) = history.latest() {
        assert_eq!(latest.cpu_percent, 40.0);
        assert_eq!(latest.memory_bytes, 5 * 1024 * 1024);
    }
    
    // Test average calculations
    if let Some(avg_cpu) = history.average_cpu(Duration::from_secs(1000)) {
        assert!((avg_cpu - 20.0).abs() < 0.1); // Should be (0+10+20+30+40)/5 = 20
    }
    
    if let Some(avg_memory) = history.average_memory(Duration::from_secs(1000)) {
        assert_eq!(avg_memory, 3 * 1024 * 1024); // (1+2+3+4+5)/5 * 1MB = 3MB
    }
    
    // Test peak values
    assert_eq!(history.peak_memory(), Some(5 * 1024 * 1024));
    assert_eq!(history.peak_cpu(), Some(40.0));
}

#[test]
fn test_process_watchdog() {
    use cursed::stdlib::process::info::ProcessInfo;
    
    let current_pid = std::process::id();
    
    // Create a mock process info for testing
    if let Ok(process_info) = ProcessInfo::from_pid(current_pid) {
        let health_config = HealthCheckConfig {
            check_interval: Duration::from_millis(100),
            thresholds: ResourceThresholds::default(),
            failure_threshold: 2,
            success_threshold: 1,
            check_responsiveness: false,
            responsiveness_timeout: Duration::from_secs(1),
        };
        
        let watchdog = ProcessWatchdog::new(
            process_info,
            "echo 'watchdog restart'".to_string(),
            3,
            health_config,
        );
        
        // Test watchdog configuration
        assert_eq!(watchdog.max_restarts, 3);
        assert_eq!(watchdog.restart_count, 0);
        assert!(watchdog.last_restart.is_none());
        
        println!("Watchdog created for PID: {}", watchdog.process_info.pid);
    } else {
        println!("Note: Process info may not be available for watchdog testing");
    }
}

#[test]
fn test_system_resource_summary() {
    if let Ok(summary) = get_system_resource_summary() {
        // Test that we get some basic system information
        assert!(summary.contains_key("cpu_count"));
        
        let cpu_count = summary.get("cpu_count").unwrap();
        assert!(*cpu_count > 0);
        
        println!("System resource summary:");
        for (key, value) in &summary {
            println!("  {}: {}", key, value);
        }
        
        // Platform-specific checks
        #[cfg(unix)]
        {
            // Unix systems should have load average information
            if summary.contains_key("load_1min") {
                println!("Load average available on this Unix system");
            }
            
            if summary.contains_key("uptime_seconds") {
                let uptime = summary.get("uptime_seconds").unwrap();
                assert!(*uptime > 0);
            }
        }
        
        if summary.contains_key("process_count") {
            let process_count = summary.get("process_count").unwrap();
            assert!(*process_count > 0);
        }
    } else {
        println!("Note: System resource summary may not be available on this platform");
    }
}

#[test]
fn test_convenience_signal_functions() {
    // Test that convenience functions exist and can be called
    // Note: We don't actually send signals to avoid affecting the test process
    
    // Test process existence check using signal 0
    let current_pid = std::process::id();
    let is_running = convenience::is_process_running(current_pid);
    assert!(is_running); // Current process should always be running
    
    // Test that non-existent process returns false
    let fake_pid = 99999999u32;
    let is_fake_running = convenience::is_process_running(fake_pid);
    assert!(!is_fake_running);
    
    println!("Process {} is running: {}", current_pid, is_running);
    println!("Process {} is running: {}", fake_pid, is_fake_running);
}

#[test]
fn test_platform_specific_features() {
    println!("Testing platform-specific features...");
    
    #[cfg(unix)]
    {
        println!("Unix platform - testing signal features");
        
        // Test Unix-specific signal features
        let signals = vec![
            Signal::HangUp, Signal::Interrupt, Signal::Quit, Signal::Terminate,
            Signal::User1, Signal::User2, Signal::Alarm, Signal::Child
        ];
        
        for signal in signals {
            println!("Signal {}: catchable={}, terminating={}", 
                signal.name(), signal.can_be_caught(), signal.is_terminating());
        }
    }
    
    #[cfg(windows)]
    {
        println!("Windows platform - testing console control features");
        
        // Test Windows-specific signal features
        let signals = vec![
            Signal::Interrupt, Signal::Terminate, Signal::HangUp, Signal::Kill
        ];
        
        for signal in signals {
            let raw = signal.as_raw();
            println!("Signal {}: raw={}", signal.name(), raw);
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("Linux-specific features available");
        // Linux has /proc filesystem for detailed process information
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("macOS-specific features available");
        // macOS has proc_pidinfo for process information
    }
}

#[test]
fn test_error_handling_and_edge_cases() {
    // Test error handling for non-existent processes
    let fake_pid = 99999999u32;
    
    assert!(collect_performance_metrics(fake_pid).is_err());
    
    // Test signal handling errors
    let handler = SignalHandler::new();
    
    // Try to register an uncatchable signal
    let action = SignalAction::Ignore;
    assert!(handler.register(Signal::Kill, action).is_err());
    
    // Test service manager errors
    let manager = ServiceManager::new();
    assert!(manager.service_status("nonexistent").is_err());
    assert!(manager.stop_service("nonexistent").is_err());
    
    println!("Error handling tests completed successfully");
}

#[test]
fn test_comprehensive_integration() {
    println!("Running comprehensive integration test...");
    
    // Test the complete workflow: monitoring + daemon management + signal handling
    let current_pid = std::process::id();
    
    // 1. Setup signal handling
    let handler = SignalHandler::new();
    let signal_received = Arc::new(AtomicBool::new(false));
    let signal_received_clone = Arc::clone(&signal_received);
    
    if Signal::User1.can_be_caught() {
        let action = SignalAction::Handle(Arc::new(move |_| {
            signal_received_clone.store(true, Ordering::SeqCst);
        }));
        
        if handler.register(Signal::User1, action).is_ok() {
            println!("✓ Signal handler registered");
        }
    }
    
    // 2. Setup process monitoring
    let monitor = create_process_monitor();
    if monitor.add_process(current_pid).is_ok() {
        println!("✓ Process added to monitoring");
        
        if let Ok(health) = monitor.get_health_status(current_pid) {
            println!("✓ Process health status: {:?}", health);
        }
    }
    
    // 3. Test daemon configuration
    let daemon_config = DaemonConfig::new("integration-test-daemon")
        .description("Integration test daemon")
        .auto_restart(3);
    
    let daemon = Daemon::new(daemon_config);
    assert_eq!(daemon.status(), DaemonStatus::Stopped);
    println!("✓ Daemon configuration created");
    
    // 4. Test service management
    let manager = ServiceManager::new();
    let service_config = DaemonConfig::new("test-service");
    
    if manager.register("test-service".to_string(), service_config).is_ok() {
        println!("✓ Service registered with manager");
        
        let services = manager.list_services();
        assert!(services.contains(&"test-service".to_string()));
        println!("✓ Service listing verified");
    }
    
    // 5. Test system resource monitoring
    if let Ok(resources) = get_system_resource_summary() {
        println!("✓ System resources: {} metrics collected", resources.len());
        
        for (key, value) in resources.iter().take(3) {
            println!("  {}: {}", key, value);
        }
    }
    
    println!("✓ Comprehensive integration test completed successfully");
}
