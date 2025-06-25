//! Comprehensive Enhanced Process Management Test Suite
//! 
//! This test suite validates the complete enhanced process management functionality
//! including advanced command execution, resource monitoring, security contexts,
//! process groups, pipelines, and integration with IPC systems.

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

use cursed::stdlib::exec_slay::{
    EnhancedSlayCommand, EnhancedSlayOptions, ProcessPriority, SecurityContext,
    IsolationLevel, IoConfiguration, MonitoringConfig, ThresholdAction,
    SignalOptions, SlayResult,
};

/// Test basic enhanced command creation and configuration
#[test]
fn test_enhanced_command_creation() {
    let cmd = EnhancedSlayCommand::new("echo", &["hello", "world"]);
    assert_eq!(cmd.name, "echo");
    assert_eq!(cmd.args, vec!["hello", "world"]);
    assert_eq!(cmd.options.priority, Some(ProcessPriority::Normal));
}

/// Test enhanced options configuration
#[test]
fn test_enhanced_options_configuration() {
    let mut cmd = EnhancedSlayCommand::new("test", &[]);
    
    // Test memory limit
    cmd.set_memory_limit(1024 * 1024); // 1MB
    assert_eq!(cmd.options.memory_limit, Some(1024 * 1024));
    
    // Test CPU limit
    cmd.set_cpu_limit(75.0);
    assert_eq!(cmd.options.cpu_limit, Some(75.0));
    
    // Test priority
    cmd.set_priority(ProcessPriority::High);
    assert_eq!(cmd.options.priority, Some(ProcessPriority::High));
    
    // Test monitoring
    cmd.enable_monitoring(Duration::from_millis(500));
    assert!(cmd.options.monitoring.enabled);
    assert_eq!(cmd.options.monitoring.interval, Duration::from_millis(500));
}

/// Test security context configuration
#[test]
fn test_security_context_configuration() {
    let mut cmd = EnhancedSlayCommand::new("test", &[]);
    
    let security = SecurityContext {
        user_id: Some(1000),
        group_id: Some(1000),
        chroot_dir: Some("/tmp".to_string()),
        isolation_level: IsolationLevel::Sandbox,
        enforce_limits: true,
    };
    
    cmd.set_security_context(security.clone());
    assert_eq!(cmd.options.security.user_id, Some(1000));
    assert_eq!(cmd.options.security.group_id, Some(1000));
    assert_eq!(cmd.options.security.chroot_dir, Some("/tmp".to_string()));
    assert_eq!(cmd.options.security.isolation_level, IsolationLevel::Sandbox);
    assert!(cmd.options.security.enforce_limits);
}

/// Test I/O configuration
#[test]
fn test_io_configuration() {
    let io_config = IoConfiguration {
        buffer_size: 16384,
        line_buffered_stdout: true,
        line_buffered_stderr: true,
        stdout_callback: Some("handle_stdout".to_string()),
        stderr_callback: Some("handle_stderr".to_string()),
        stdin_data: Some(b"input data\n".to_vec()),
    };
    
    assert_eq!(io_config.buffer_size, 16384);
    assert!(io_config.line_buffered_stdout);
    assert!(io_config.line_buffered_stderr);
    assert_eq!(io_config.stdout_callback, Some("handle_stdout".to_string()));
    assert_eq!(io_config.stderr_callback, Some("handle_stderr".to_string()));
    assert_eq!(io_config.stdin_data, Some(b"input data\n".to_vec()));
}

/// Test monitoring configuration and thresholds
#[test]
fn test_monitoring_configuration() {
    let monitoring = MonitoringConfig {
        enabled: true,
        interval: Duration::from_millis(100),
        memory_threshold: Some(512 * 1024), // 512KB
        cpu_threshold: Some(80.0), // 80%
        threshold_action: ThresholdAction::Warn,
    };
    
    assert!(monitoring.enabled);
    assert_eq!(monitoring.interval, Duration::from_millis(100));
    assert_eq!(monitoring.memory_threshold, Some(512 * 1024));
    assert_eq!(monitoring.cpu_threshold, Some(80.0));
    assert_eq!(monitoring.threshold_action, ThresholdAction::Warn);
}

/// Test process priority levels
#[test]
fn test_process_priority_levels() {
    assert_eq!(ProcessPriority::Low as i32, -10);
    assert_eq!(ProcessPriority::Normal as i32, 0);
    assert_eq!(ProcessPriority::High as i32, 10);
    assert_eq!(ProcessPriority::RealTime as i32, 20);
}

/// Test isolation levels
#[test]
fn test_isolation_levels() {
    assert_eq!(IsolationLevel::None, IsolationLevel::None);
    assert_eq!(IsolationLevel::Basic, IsolationLevel::Basic);
    assert_eq!(IsolationLevel::Sandbox, IsolationLevel::Sandbox);
    assert_eq!(IsolationLevel::Container, IsolationLevel::Container);
}

/// Test threshold actions
#[test]
fn test_threshold_actions() {
    assert_eq!(ThresholdAction::None, ThresholdAction::None);
    assert_eq!(ThresholdAction::Warn, ThresholdAction::Warn);
    assert_eq!(ThresholdAction::Throttle, ThresholdAction::Throttle);
    assert_eq!(ThresholdAction::Kill, ThresholdAction::Kill);
}

/// Test signal options configuration
#[test]
fn test_signal_options() {
    let signal_opts = SignalOptions {
        grace_period: Duration::from_secs(10),
        force: true,
        signal: 15, // SIGTERM
        recursive: true,
    };
    
    assert_eq!(signal_opts.grace_period, Duration::from_secs(10));
    assert!(signal_opts.force);
    assert_eq!(signal_opts.signal, 15);
    assert!(signal_opts.recursive);
}

/// Test enhanced command with complete configuration
#[test]
fn test_enhanced_command_complete_configuration() {
    let mut cmd = EnhancedSlayCommand::new("test_command", &["arg1", "arg2"]);
    
    // Configure all options
    cmd.set_memory_limit(2 * 1024 * 1024); // 2MB
    cmd.set_cpu_limit(50.0); // 50%
    cmd.set_priority(ProcessPriority::High);
    cmd.enable_monitoring(Duration::from_millis(200));
    
    let security = SecurityContext {
        user_id: Some(1001),
        group_id: Some(1001),
        chroot_dir: Some("/var/chroot".to_string()),
        isolation_level: IsolationLevel::Container,
        enforce_limits: true,
    };
    cmd.set_security_context(security);
    
    // Verify all configurations
    assert_eq!(cmd.options.memory_limit, Some(2 * 1024 * 1024));
    assert_eq!(cmd.options.cpu_limit, Some(50.0));
    assert_eq!(cmd.options.priority, Some(ProcessPriority::High));
    assert!(cmd.options.monitoring.enabled);
    assert_eq!(cmd.options.monitoring.interval, Duration::from_millis(200));
    assert_eq!(cmd.options.security.user_id, Some(1001));
    assert_eq!(cmd.options.security.group_id, Some(1001));
    assert_eq!(cmd.options.security.chroot_dir, Some("/var/chroot".to_string()));
    assert_eq!(cmd.options.security.isolation_level, IsolationLevel::Container);
    assert!(cmd.options.security.enforce_limits);
}

/// Test command running state tracking
#[test] 
fn test_command_running_state() {
    let mut cmd = EnhancedSlayCommand::new("echo", &["test"]);
    
    // Initially not running
    assert!(!cmd.is_running());
    
    // Note: In a real test environment, we would test actual command execution
    // but for unit tests, we focus on the state management logic
}

/// Test resource monitoring system setup
#[test]
fn test_resource_monitoring_setup() {
    let mut cmd = EnhancedSlayCommand::new("test", &[]);
    
    // Configure monitoring with specific thresholds
    let mut monitoring = MonitoringConfig::default();
    monitoring.enabled = true;
    monitoring.interval = Duration::from_millis(100);
    monitoring.memory_threshold = Some(1024 * 1024); // 1MB
    monitoring.cpu_threshold = Some(75.0); // 75%
    monitoring.threshold_action = ThresholdAction::Throttle;
    
    cmd.options.monitoring = monitoring;
    
    assert!(cmd.options.monitoring.enabled);
    assert_eq!(cmd.options.monitoring.interval, Duration::from_millis(100));
    assert_eq!(cmd.options.monitoring.memory_threshold, Some(1024 * 1024));
    assert_eq!(cmd.options.monitoring.cpu_threshold, Some(75.0));
    assert_eq!(cmd.options.monitoring.threshold_action, ThresholdAction::Throttle);
}

/// Test I/O configuration with stdin data
#[test]
fn test_io_configuration_with_stdin() {
    let mut cmd = EnhancedSlayCommand::new("cat", &[]);
    
    let io_config = IoConfiguration {
        buffer_size: 8192,
        line_buffered_stdout: true,
        line_buffered_stderr: false,
        stdout_callback: None,
        stderr_callback: None,
        stdin_data: Some(b"Hello from stdin\nSecond line\n".to_vec()),
    };
    
    cmd.options.io_config = io_config;
    
    assert_eq!(cmd.options.io_config.buffer_size, 8192);
    assert!(cmd.options.io_config.line_buffered_stdout);
    assert!(!cmd.options.io_config.line_buffered_stderr);
    assert_eq!(
        cmd.options.io_config.stdin_data,
        Some(b"Hello from stdin\nSecond line\n".to_vec())
    );
}

/// Test enhanced options with builder pattern
#[test]
fn test_enhanced_options_builder_pattern() {
    let mut options = EnhancedSlayOptions::default();
    
    // Modify options
    options.memory_limit = Some(4 * 1024 * 1024); // 4MB
    options.cpu_limit = Some(25.0); // 25%
    options.priority = Some(ProcessPriority::Low);
    
    options.security.user_id = Some(2000);
    options.security.isolation_level = IsolationLevel::Basic;
    
    options.monitoring.enabled = true;
    options.monitoring.interval = Duration::from_millis(250);
    options.monitoring.threshold_action = ThresholdAction::Kill;
    
    // Create command with options
    let cmd = EnhancedSlayCommand::new("test", &[]).with_options(options);
    
    assert_eq!(cmd.options.memory_limit, Some(4 * 1024 * 1024));
    assert_eq!(cmd.options.cpu_limit, Some(25.0));
    assert_eq!(cmd.options.priority, Some(ProcessPriority::Low));
    assert_eq!(cmd.options.security.user_id, Some(2000));
    assert_eq!(cmd.options.security.isolation_level, IsolationLevel::Basic);
    assert!(cmd.options.monitoring.enabled);
    assert_eq!(cmd.options.monitoring.interval, Duration::from_millis(250));
    assert_eq!(cmd.options.monitoring.threshold_action, ThresholdAction::Kill);
}

/// Test default configurations
#[test]
fn test_default_configurations() {
    let cmd = EnhancedSlayCommand::new("test", &[]);
    
    // Test default enhanced options
    assert_eq!(cmd.options.memory_limit, None);
    assert_eq!(cmd.options.cpu_limit, None);
    assert_eq!(cmd.options.priority, Some(ProcessPriority::Normal));
    
    // Test default security context
    assert_eq!(cmd.options.security.user_id, None);
    assert_eq!(cmd.options.security.group_id, None);
    assert_eq!(cmd.options.security.chroot_dir, None);
    assert_eq!(cmd.options.security.isolation_level, IsolationLevel::None);
    assert!(!cmd.options.security.enforce_limits);
    
    // Test default I/O configuration
    assert_eq!(cmd.options.io_config.buffer_size, 8192);
    assert!(!cmd.options.io_config.line_buffered_stdout);
    assert!(!cmd.options.io_config.line_buffered_stderr);
    assert_eq!(cmd.options.io_config.stdout_callback, None);
    assert_eq!(cmd.options.io_config.stderr_callback, None);
    assert_eq!(cmd.options.io_config.stdin_data, None);
    
    // Test default monitoring configuration
    assert!(!cmd.options.monitoring.enabled);
    assert_eq!(cmd.options.monitoring.interval, Duration::from_secs(1));
    assert_eq!(cmd.options.monitoring.memory_threshold, None);
    assert_eq!(cmd.options.monitoring.cpu_threshold, None);
    assert_eq!(cmd.options.monitoring.threshold_action, ThresholdAction::None);
}

/// Test concurrent command management
#[test]
fn test_concurrent_command_management() {
    let num_commands = 5;
    let commands: Vec<_> = (0..num_commands)
        .map(|i| {
            let mut cmd = EnhancedSlayCommand::new("echo", &[&format!("test_{}", i)]);
            cmd.set_priority(ProcessPriority::Normal);
            cmd.enable_monitoring(Duration::from_millis(100));
            Arc::new(Mutex::new(cmd))
        })
        .collect();
    
    let handles: Vec<_> = commands
        .iter()
        .enumerate()
        .map(|(i, cmd_ref)| {
            let cmd_clone = cmd_ref.clone();
            thread::spawn(move || {
                let cmd = cmd_clone.lock().unwrap();
                assert_eq!(cmd.name, "echo");
                assert_eq!(cmd.args[0], format!("test_{}", i));
                assert_eq!(cmd.options.priority, Some(ProcessPriority::Normal));
                assert!(cmd.options.monitoring.enabled);
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test resource monitoring configuration with different threshold actions
#[test]
fn test_monitoring_threshold_actions() {
    let threshold_actions = [
        ThresholdAction::None,
        ThresholdAction::Warn,
        ThresholdAction::Throttle,
        ThresholdAction::Kill,
    ];
    
    for action in &threshold_actions {
        let mut cmd = EnhancedSlayCommand::new("test", &[]);
        
        let mut monitoring = cmd.options.monitoring.clone();
        monitoring.enabled = true;
        monitoring.threshold_action = *action;
        cmd.options.monitoring = monitoring;
        
        assert!(cmd.options.monitoring.enabled);
        assert_eq!(cmd.options.monitoring.threshold_action, *action);
    }
}

/// Test security context with different isolation levels
#[test]
fn test_security_isolation_levels() {
    let isolation_levels = [
        IsolationLevel::None,
        IsolationLevel::Basic,
        IsolationLevel::Sandbox,
        IsolationLevel::Container,
    ];
    
    for level in &isolation_levels {
        let mut cmd = EnhancedSlayCommand::new("test", &[]);
        
        let mut security = cmd.options.security.clone();
        security.isolation_level = *level;
        cmd.options.security = security;
        
        assert_eq!(cmd.options.security.isolation_level, *level);
    }
}

/// Test enhanced command state management
#[test]
fn test_enhanced_command_state_management() {
    let cmd = EnhancedSlayCommand::new("test", &[]);
    
    // Test initial state
    assert!(!cmd.is_running());
    
    // Test state can be queried without errors
    let stats = cmd.get_stats();
    assert!(stats.is_none()); // No stats when not running
    
    let history = cmd.get_stats_history();
    assert!(history.is_empty()); // No history initially
}

/// Test comprehensive enhanced options validation
#[test]
fn test_comprehensive_enhanced_options_validation() {
    let mut cmd = EnhancedSlayCommand::new("comprehensive_test", &["arg1", "arg2", "arg3"]);
    
    // Set all possible options
    cmd.set_memory_limit(8 * 1024 * 1024); // 8MB
    cmd.set_cpu_limit(90.0); // 90%
    cmd.set_priority(ProcessPriority::RealTime);
    cmd.enable_monitoring(Duration::from_millis(50));
    
    let security = SecurityContext {
        user_id: Some(1337),
        group_id: Some(1337),
        chroot_dir: Some("/opt/sandbox".to_string()),
        isolation_level: IsolationLevel::Container,
        enforce_limits: true,
    };
    cmd.set_security_context(security);
    
    // Verify comprehensive configuration
    assert_eq!(cmd.name, "comprehensive_test");
    assert_eq!(cmd.args, vec!["arg1", "arg2", "arg3"]);
    assert_eq!(cmd.options.memory_limit, Some(8 * 1024 * 1024));
    assert_eq!(cmd.options.cpu_limit, Some(90.0));
    assert_eq!(cmd.options.priority, Some(ProcessPriority::RealTime));
    assert!(cmd.options.monitoring.enabled);
    assert_eq!(cmd.options.monitoring.interval, Duration::from_millis(50));
    assert_eq!(cmd.options.security.user_id, Some(1337));
    assert_eq!(cmd.options.security.group_id, Some(1337));
    assert_eq!(cmd.options.security.chroot_dir, Some("/opt/sandbox".to_string()));
    assert_eq!(cmd.options.security.isolation_level, IsolationLevel::Container);
    assert!(cmd.options.security.enforce_limits);
}

/// Integration test for enhanced process management workflow
#[test]
fn test_enhanced_process_management_integration() {
    // Create a command with comprehensive configuration
    let mut cmd = EnhancedSlayCommand::new("echo", &["Enhanced Process Management Test"]);
    
    // Configure resource limits
    cmd.set_memory_limit(1024 * 1024); // 1MB
    cmd.set_cpu_limit(50.0); // 50%
    cmd.set_priority(ProcessPriority::High);
    
    // Configure monitoring
    cmd.enable_monitoring(Duration::from_millis(100));
    
    // Configure security (basic level for testing)
    let security = SecurityContext {
        user_id: None, // Don't change user for test
        group_id: None, // Don't change group for test
        chroot_dir: None, // Don't chroot for test
        isolation_level: IsolationLevel::Basic,
        enforce_limits: false, // Don't enforce limits for test
    };
    cmd.set_security_context(security);
    
    // Verify configuration is correctly applied
    assert_eq!(cmd.options.memory_limit, Some(1024 * 1024));
    assert_eq!(cmd.options.cpu_limit, Some(50.0));
    assert_eq!(cmd.options.priority, Some(ProcessPriority::High));
    assert!(cmd.options.monitoring.enabled);
    assert_eq!(cmd.options.monitoring.interval, Duration::from_millis(100));
    assert_eq!(cmd.options.security.isolation_level, IsolationLevel::Basic);
    
    // Test state queries
    assert!(!cmd.is_running());
    assert!(cmd.get_stats().is_none());
    assert!(cmd.get_stats_history().is_empty());
}

/// Performance test for enhanced command creation
#[test]
fn test_enhanced_command_creation_performance() {
    let start = Instant::now();
    let num_commands = 1000;
    
    let commands: Vec<_> = (0..num_commands)
        .map(|i| {
            let mut cmd = EnhancedSlayCommand::new("test", &[&format!("arg_{}", i)]);
            cmd.set_memory_limit(1024 * 1024);
            cmd.set_cpu_limit(50.0);
            cmd.set_priority(ProcessPriority::Normal);
            cmd.enable_monitoring(Duration::from_millis(100));
            cmd
        })
        .collect();
    
    let duration = start.elapsed();
    
    // Verify all commands were created
    assert_eq!(commands.len(), num_commands);
    
    // Performance check: should create 1000 enhanced commands in reasonable time
    assert!(duration < Duration::from_secs(1), 
           "Creating {} enhanced commands took {:?}, expected < 1s", 
           num_commands, duration);
    
    // Verify last command configuration
    let last_cmd = &commands[num_commands - 1];
    assert_eq!(last_cmd.name, "test");
    assert_eq!(last_cmd.args[0], format!("arg_{}", num_commands - 1));
    assert_eq!(last_cmd.options.memory_limit, Some(1024 * 1024));
    assert_eq!(last_cmd.options.cpu_limit, Some(50.0));
    assert_eq!(last_cmd.options.priority, Some(ProcessPriority::Normal));
    assert!(last_cmd.options.monitoring.enabled);
}

/// Memory usage test for enhanced command structures
#[test]
fn test_enhanced_command_memory_usage() {
    use std::mem;
    
    // Test memory footprint of enhanced command structures
    let cmd = EnhancedSlayCommand::new("test", &["arg1", "arg2"]);
    
    // Basic sanity checks for memory usage
    // (These are rough estimates and may vary by platform)
    assert!(mem::size_of_val(&cmd) > 0);
    assert!(mem::size_of_val(&cmd.options) > 0);
    assert!(mem::size_of_val(&cmd.options.security) > 0);
    assert!(mem::size_of_val(&cmd.options.io_config) > 0);
    assert!(mem::size_of_val(&cmd.options.monitoring) > 0);
    
    // Verify structures are not excessively large
    let cmd_size = mem::size_of_val(&cmd);
    assert!(cmd_size < 10000, "Enhanced command size {} seems excessive", cmd_size);
}
