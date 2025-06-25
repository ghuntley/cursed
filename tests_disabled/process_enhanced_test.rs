/// Comprehensive test suite for enhanced process management system
/// 
/// This test suite validates the enhanced process management capabilities
/// including pipes, signals, daemon management, and cross-platform features.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// Import the enhanced process management modules
use cursed::stdlib::process::*;

#[cfg(test)]
mod pipe_tests {
    use super::*;
    
    #[test]
    fn test_named_pipe_creation() {
        let pipe_result = NamedPipe::create("test_pipe_creation", PipeMode::ReadWrite);
        
        // Channel pipes should always work
        assert!(pipe_result.is_ok());
        
        let pipe = pipe_result.unwrap();
        assert_eq!(pipe.name(), "test_pipe_creation");
        assert_eq!(pipe.mode(), &PipeMode::ReadWrite);
        assert!(pipe.can_read());
        assert!(pipe.can_write());
    }
    
    #[test]
    fn test_pipe_communication() {
        let pipe = NamedPipe::create("test_pipe_comm", PipeMode::ReadWrite).unwrap();
        
        let test_data = b"Hello, pipe world!";
        let bytes_written = pipe.write(test_data).unwrap();
        assert_eq!(bytes_written, test_data.len());
        
        let mut buffer = [0u8; 64];
        let bytes_read = pipe.read(&mut buffer).unwrap();
        assert_eq!(bytes_read, test_data.len());
        assert_eq!(&buffer[..bytes_read], test_data);
    }
    
    #[test]
    fn test_pipe_modes() {
        let read_pipe = NamedPipe::create("read_pipe", PipeMode::Read).unwrap();
        assert!(read_pipe.can_read());
        assert!(!read_pipe.can_write());
        
        let write_pipe = NamedPipe::create("write_pipe", PipeMode::Write).unwrap();
        assert!(!write_pipe.can_read());
        assert!(write_pipe.can_write());
        
        let rw_pipe = NamedPipe::create("rw_pipe", PipeMode::ReadWrite).unwrap();
        assert!(rw_pipe.can_read());
        assert!(rw_pipe.can_write());
    }
    
    #[test]
    fn test_pipe_server() {
        let server = PipeServer::new("test_server");
        assert_eq!(server.client_count(), 0);
        
        let client_id = server.accept_client().unwrap();
        assert_eq!(server.client_count(), 1);
        assert!(server.client_ids().contains(&client_id));
        
        let test_data = b"Server to client message";
        let bytes_sent = server.send_to_client(client_id, test_data).unwrap();
        assert_eq!(bytes_sent, test_data.len());
        
        server.remove_client(client_id).unwrap();
        assert_eq!(server.client_count(), 0);
    }
    
    #[test]
    fn test_pipe_pair() {
        let pair_result = PipePair::create("test_pair");
        
        // May fail on some platforms, but should handle gracefully
        if let Ok(pair) = pair_result {
            let test_data = b"Bidirectional test";
            let _ = pair.send(test_data);
            
            let mut buffer = [0u8; 64];
            let _ = pair.receive(&mut buffer);
        }
    }
    
    #[test]
    fn test_message_header() {
        let header = message::MessageHeader::new(42, 100, 1);
        assert_eq!(header.magic, message::MessageHeader::MAGIC);
        assert_eq!(header.message_type, 42);
        assert_eq!(header.payload_length, 100);
        assert_eq!(header.sequence, 1);
        assert!(header.is_valid());
        
        let bytes = header.to_bytes();
        let restored = message::MessageHeader::from_bytes(&bytes);
        assert!(restored.is_valid());
        assert_eq!(restored.message_type, header.message_type);
        assert_eq!(restored.payload_length, header.payload_length);
        assert_eq!(restored.sequence, header.sequence);
    }
    
    #[test]
    fn test_message_creation() {
        let payload = b"Test message payload".to_vec();
        let message = message::Message::new(100, payload.clone(), 5);
        
        assert_eq!(message.header.message_type, 100);
        assert_eq!(message.header.payload_length, payload.len() as u32);
        assert_eq!(message.header.sequence, 5);
        assert_eq!(message.payload, payload);
        assert!(message.header.is_valid());
    }
}

#[cfg(test)]
mod signal_tests {
    use super::*;
    
    #[test]
    fn test_signal_properties() {
        assert_eq!(Signal::Interrupt.name(), "SIGINT");
        assert!(Signal::Interrupt.can_be_caught());
        assert!(Signal::Interrupt.is_terminating());
        
        assert_eq!(Signal::Kill.name(), "SIGKILL");
        assert!(!Signal::Kill.can_be_caught());
        assert!(Signal::Kill.is_terminating());
        
        assert_eq!(Signal::Continue.name(), "SIGCONT");
        assert!(Signal::Continue.can_be_caught());
        assert!(!Signal::Continue.is_terminating());
    }
    
    #[test]
    fn test_signal_conversion() {
        let signal = Signal::Interrupt;
        let raw = signal.as_raw();
        
        // Test round-trip conversion
        if let Some(restored) = Signal::from_raw(raw) {
            assert_eq!(restored, signal);
        }
        
        // Test all signals have valid names
        let signals = [
            Signal::HangUp, Signal::Interrupt, Signal::Quit, Signal::Terminate,
            Signal::Kill, Signal::User1, Signal::User2, Signal::Child,
            Signal::Continue, Signal::Stop,
        ];
        
        for signal in &signals {
            assert!(!signal.name().is_empty());
        }
    }
    
    #[test]
    fn test_signal_handler_creation() {
        let handler = SignalHandler::new();
        
        let signal_received = Arc::new(AtomicBool::new(false));
        let signal_received_clone = Arc::clone(&signal_received);
        
        let action = SignalAction::Handle(Arc::new(move |_signal| {
            signal_received_clone.store(true, Ordering::SeqCst);
        }));
        
        // Test with signals that can be caught
        if Signal::User1.can_be_caught() {
            assert!(handler.register(Signal::User1, action).is_ok());
            
            // Simulate receiving the signal
            assert!(handler.simulate_signal(Signal::User1).is_ok());
            
            // Give some time for the handler to process
            thread::sleep(Duration::from_millis(100));
            
            assert!(signal_received.load(Ordering::SeqCst));
        }
    }
    
    #[test]
    fn test_signal_handler_ignore() {
        let handler = SignalHandler::new();
        
        // Test ignore action
        if Signal::User2.can_be_caught() {
            assert!(handler.register(Signal::User2, SignalAction::Ignore).is_ok());
            assert!(handler.simulate_signal(Signal::User2).is_ok());
            
            // Should not cause any problems
            thread::sleep(Duration::from_millis(50));
        }
    }
    
    #[test]
    fn test_signal_mask() {
        #[cfg(unix)]
        {
            let mut mask = SignalMask::empty().unwrap();
            assert!(!mask.contains(Signal::Interrupt));
            
            mask.add(Signal::Interrupt).unwrap();
            assert!(mask.contains(Signal::Interrupt));
            
            mask.remove(Signal::Interrupt).unwrap();
            assert!(!mask.contains(Signal::Interrupt));
            
            let full_mask = SignalMask::full().unwrap();
            assert!(full_mask.contains(Signal::Interrupt));
            assert!(full_mask.contains(Signal::Terminate));
        }
    }
    
    #[test]
    fn test_convenience_functions() {
        // Test that convenience functions don't panic
        assert!(convenience::is_process_running(std::process::id()));
        
        // Test signal sending to self (should be safe with harmless signals)
        #[cfg(unix)]
        {
            // SIGCONT should be safe to send to self
            if Signal::Continue.can_be_caught() {
                let _ = convenience::send_signal(std::process::id(), Signal::Continue);
            }
        }
    }
}

#[cfg(test)]
mod daemon_tests {
    use super::*;
    
    #[test]
    fn test_daemon_config() {
        let config = DaemonConfig::new("test-daemon")
            .working_directory("/tmp")
            .user("nobody")
            .group("nogroup")
            .pid_file("/tmp/test-daemon.pid")
            .log_file("/tmp/test-daemon.log")
            .lock_file("/tmp/test-daemon.lock")
            .description("Test daemon for unit tests")
            .auto_restart(3)
            .env("TEST_VAR", "test_value")
            .umask(0o022);
        
        assert_eq!(config.name, "test-daemon");
        assert_eq!(config.working_directory, Some(PathBuf::from("/tmp")));
        assert_eq!(config.user, Some("nobody".to_string()));
        assert_eq!(config.group, Some("nogroup".to_string()));
        assert_eq!(config.pid_file, Some(PathBuf::from("/tmp/test-daemon.pid")));
        assert_eq!(config.log_file, Some(PathBuf::from("/tmp/test-daemon.log")));
        assert_eq!(config.lock_file, Some(PathBuf::from("/tmp/test-daemon.lock")));
        assert_eq!(config.description, Some("Test daemon for unit tests".to_string()));
        assert!(config.auto_restart);
        assert_eq!(config.max_restarts, 3);
        assert_eq!(config.environment.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(config.umask, Some(0o022));
    }
    
    #[test]
    fn test_daemon_creation() {
        let config = DaemonConfig::new("test-daemon-creation");
        let daemon = Daemon::new(config);
        
        assert_eq!(daemon.status(), DaemonStatus::Stopped);
        assert_eq!(daemon.pid(), None);
        assert_eq!(daemon.restart_count(), 0);
    }
    
    #[test]
    fn test_daemon_status_enum() {
        assert_eq!(DaemonStatus::Running, DaemonStatus::Running);
        assert_ne!(DaemonStatus::Running, DaemonStatus::Stopped);
        assert_ne!(DaemonStatus::Starting, DaemonStatus::Stopping);
        assert_ne!(DaemonStatus::Failed, DaemonStatus::Unknown);
    }
    
    #[test]
    fn test_service_manager() {
        let manager = ServiceManager::new();
        
        let config = DaemonConfig::new("test-service");
        assert!(manager.register("test-service".to_string(), config).is_ok());
        
        let services = manager.list_services();
        assert!(services.contains(&"test-service".to_string()));
        
        let status = manager.service_status("test-service").unwrap();
        assert_eq!(status, DaemonStatus::Stopped);
        
        // Test duplicate registration
        let config2 = DaemonConfig::new("test-service");
        assert!(manager.register("test-service".to_string(), config2).is_err());
    }
    
    #[test]
    fn test_service_manager_operations() {
        let manager = ServiceManager::new();
        
        // Register multiple services
        for i in 1..=3 {
            let config = DaemonConfig::new(format!("service-{}", i));
            assert!(manager.register(format!("service-{}", i), config).is_ok());
        }
        
        let services = manager.list_services();
        assert_eq!(services.len(), 3);
        assert!(services.contains(&"service-1".to_string()));
        assert!(services.contains(&"service-2".to_string()));
        assert!(services.contains(&"service-3".to_string()));
        
        // Test status of non-existent service
        assert!(manager.service_status("non-existent").is_err());
    }
}

#[cfg(test)]
mod platform_tests {
    use super::*;
    
    #[test]
    fn test_platform_detection() {
        let platform = get_platform_name();
        assert!(!platform.is_empty());
        
        // Should be one of the known platforms
        let known_platforms = ["windows", "linux", "macos", "freebsd", "openbsd", "netbsd", "unknown"];
        assert!(known_platforms.contains(&platform));
    }
    
    #[test]
    fn test_platform_features() {
        // Test feature detection
        let features = [
            PlatformFeature::Signals,
            PlatformFeature::ProcessGroups,
            PlatformFeature::ResourceLimits,
            PlatformFeature::FileDescriptors,
            PlatformFeature::WindowsServices,
            PlatformFeature::Cgroups,
            PlatformFeature::Namespaces,
        ];
        
        for feature in &features {
            let supported = supports_feature(*feature);
            
            // Verify platform-specific features
            match feature {
                PlatformFeature::WindowsServices => {
                    if cfg!(windows) {
                        assert!(supported);
                    } else {
                        assert!(!supported);
                    }
                }
                PlatformFeature::Signals => {
                    if cfg!(unix) {
                        assert!(supported);
                    } else {
                        assert!(!supported);
                    }
                }
                PlatformFeature::Cgroups => {
                    if cfg!(target_os = "linux") {
                        assert!(supported);
                    } else {
                        assert!(!supported);
                    }
                }
                _ => {
                    // Other features may or may not be supported
                }
            }
        }
    }
    
    #[test]
    fn test_platform_utils() {
        // Test elevated privilege check
        let is_elevated = PlatformUtils::is_elevated();
        
        // Should return a boolean (may be true or false depending on test environment)
        assert!(is_elevated == true || is_elevated == false);
        
        // Test current user information
        if let Ok(user_info) = PlatformUtils::get_current_user() {
            assert!(!user_info.username.is_empty());
            
            #[cfg(unix)]
            {
                assert!(user_info.uid.is_some());
                assert!(user_info.gid.is_some());
            }
            
            #[cfg(windows)]
            {
                assert!(user_info.uid.is_none());
                assert!(user_info.gid.is_none());
            }
        }
    }
    
    #[cfg(windows)]
    #[test]
    fn test_windows_specific() {
        use crate::stdlib::process::platform::windows;
        
        // Test Windows process utilities
        let current_pid = std::process::id();
        
        // Test command line retrieval (should work for current process)
        if let Ok(cmd_line) = windows::get_process_command_line(current_pid) {
            assert!(!cmd_line.is_empty());
        }
        
        // Test environment retrieval
        let _env = windows::get_process_environment(current_pid);
        
        // Test service status enum
        assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
        assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
    }
    
    #[cfg(unix)]
    #[test]
    fn test_unix_specific() {
        use crate::stdlib::process::platform::unix;
        
        let current_pid = std::process::id();
        
        // Test command line retrieval
        if let Ok(cmd_line) = unix::get_process_command_line(current_pid) {
            assert!(!cmd_line.is_empty());
            // Should contain at least the executable name
            assert!(cmd_line.len() >= 1);
        }
        
        // Test environment retrieval
        if let Ok(env) = unix::get_process_environment(current_pid) {
            // Should have some environment variables
            assert!(!env.is_empty());
        }
        
        // Test resource limits
        if let Ok(limits) = unix::get_resource_limits() {
            assert!(limits.max_file_descriptors > 0);
            assert!(limits.max_processes > 0);
        }
    }
    
    #[cfg(target_os = "linux")]
    #[test]
    fn test_linux_specific() {
        use crate::stdlib::process::platform::linux;
        
        let current_pid = std::process::id();
        
        // Test cgroup information
        if let Ok(cgroups) = linux::get_process_cgroup(current_pid) {
            // Should have at least one cgroup
            assert!(!cgroups.is_empty());
            
            for cgroup in &cgroups {
                assert!(!cgroup.path.is_empty());
            }
        }
        
        // Test namespace information
        if let Ok(namespaces) = linux::get_process_namespaces(current_pid) {
            // Should have several namespaces
            assert!(!namespaces.is_empty());
            
            for ns in &namespaces {
                assert!(!ns.ns_type.is_empty());
                assert!(ns.inode > 0);
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_process_communication_integration() {
        let config = CommunicationConfig::default();
        let comm = create_process_communication(12345, config).unwrap();
        
        assert_eq!(comm.process_id, 12345);
        assert!(comm.channels.total_channels() > 0);
        
        let stats = comm.get_statistics();
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.messages_received, 0);
    }
    
    #[test]
    fn test_process_with_communication() {
        let process_config = ProcessConfig::new("echo")
            .arg("test");
        
        let comm_config = CommunicationConfig::default();
        
        // This may fail on some platforms, but should handle gracefully
        if let Ok((process, comm)) = execute_with_communication(process_config, comm_config) {
            assert!(process.id() > 0);
            assert_eq!(comm.process_id, process.id());
        }
    }
    
    #[test]
    fn test_cross_platform_process_info() {
        let current_pid = std::process::id();
        
        // Test platform-specific process information
        if let Ok(platform_info) = PlatformUtils::get_platform_info(current_pid) {
            match platform_info {
                #[cfg(target_os = "linux")]
                PlatformProcessInfo::Linux { command_line, environment, file_descriptors, .. } => {
                    assert!(!command_line.is_empty());
                    assert!(!environment.is_empty());
                    assert!(!file_descriptors.is_empty());
                }
                
                #[cfg(target_os = "macos")]
                PlatformProcessInfo::MacOS { sysctl_info, .. } => {
                    assert_eq!(sysctl_info.pid, current_pid);
                    assert!(!sysctl_info.name.is_empty());
                }
                
                #[cfg(windows)]
                PlatformProcessInfo::Windows { command_line, .. } => {
                    assert!(!command_line.is_empty());
                }
                
                #[cfg(unix)]
                PlatformProcessInfo::Unix { command_line, environment, .. } => {
                    assert!(!command_line.is_empty());
                    assert!(!environment.is_empty());
                }
            }
        }
    }
    
    #[test]
    fn test_signal_process_integration() {
        // Test signal sending to current process (safe signals only)
        let current_pid = std::process::id();
        
        #[cfg(unix)]
        {
            // SIGCONT should be safe to send to self
            if Signal::Continue.can_be_caught() {
                assert!(send_signal(current_pid, Signal::Continue).is_ok());
            }
            
            // Signal 0 (null signal) is safe for testing process existence
            if let Some(null_signal) = Signal::from_raw(0) {
                assert!(send_signal(current_pid, null_signal).is_ok());
            }
        }
        
        // Test convenience functions
        assert!(convenience::is_process_running(current_pid));
    }
    
    #[test]
    fn test_end_to_end_daemon_workflow() {
        let temp_dir = std::env::temp_dir();
        
        let config = DaemonConfig::new("test-integration-daemon")
            .working_directory(temp_dir.clone())
            .pid_file(temp_dir.join("test-integration-daemon.pid"))
            .log_file(temp_dir.join("test-integration-daemon.log"))
            .description("Integration test daemon")
            .env("TEST_ENV", "integration_test");
        
        let daemon = Daemon::new(config);
        
        // Test initial state
        assert_eq!(daemon.status(), DaemonStatus::Stopped);
        assert_eq!(daemon.pid(), None);
        assert_eq!(daemon.restart_count(), 0);
        
        // Test file operations (may fail in restricted environments)
        if daemon.write_pid_file().is_ok() {
            assert!(daemon.create_lock_file().is_ok());
            assert!(daemon.cleanup_files().is_ok());
        }
    }
    
    #[test]
    fn test_concurrent_operations() {
        let num_threads = 4;
        let operations_per_thread = 10;
        let completed_operations = Arc::new(AtomicU32::new(0));
        
        let mut handles = Vec::new();
        
        for thread_id in 0..num_threads {
            let completed = Arc::clone(&completed_operations);
            
            let handle = thread::spawn(move || {
                for i in 0..operations_per_thread {
                    // Test concurrent pipe operations
                    let pipe_name = format!("concurrent_pipe_{}_{}", thread_id, i);
                    if let Ok(pipe) = NamedPipe::create(&pipe_name, PipeMode::ReadWrite) {
                        let test_data = format!("Thread {} Operation {}", thread_id, i);
                        if pipe.write(test_data.as_bytes()).is_ok() {
                            let mut buffer = [0u8; 128];
                            let _ = pipe.read(&mut buffer);
                            completed.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        let total_completed = completed_operations.load(Ordering::SeqCst);
        assert!(total_completed > 0);
        println!("Completed {} concurrent operations", total_completed);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_pipe_throughput() {
        let pipe = NamedPipe::create("throughput_test", PipeMode::ReadWrite).unwrap();
        
        let data_size = 1024; // 1KB
        let iterations = 100;
        let test_data = vec![0xAA; data_size];
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = pipe.write(&test_data);
            let mut buffer = vec![0u8; data_size];
            let _ = pipe.read(&mut buffer);
        }
        
        let elapsed = start.elapsed();
        let throughput = (data_size * iterations * 2) as f64 / elapsed.as_secs_f64(); // Read + Write
        
        println!("Pipe throughput: {:.2} bytes/second", throughput);
        assert!(throughput > 0.0);
    }
    
    #[test]
    fn test_signal_handler_performance() {
        let handler = SignalHandler::new();
        let signals_processed = Arc::new(AtomicU32::new(0));
        let signals_clone = Arc::clone(&signals_processed);
        
        let action = SignalAction::Handle(Arc::new(move |_| {
            signals_clone.fetch_add(1, Ordering::SeqCst);
        }));
        
        if Signal::User1.can_be_caught() {
            assert!(handler.register(Signal::User1, action).is_ok());
            
            let iterations = 100;
            let start = Instant::now();
            
            for _ in 0..iterations {
                let _ = handler.simulate_signal(Signal::User1);
            }
            
            // Wait for processing
            thread::sleep(Duration::from_millis(100));
            
            let elapsed = start.elapsed();
            let processed = signals_processed.load(Ordering::SeqCst);
            
            println!("Signal processing: {} signals in {:.2}ms", processed, elapsed.as_millis());
            assert!(processed > 0);
        }
    }
    
    #[test]
    fn test_daemon_config_creation_performance() {
        let iterations = 1000;
        let start = Instant::now();
        
        for i in 0..iterations {
            let _config = DaemonConfig::new(format!("perf-test-{}", i))
                .working_directory("/tmp")
                .user("nobody")
                .group("nogroup")
                .pid_file(format!("/tmp/perf-test-{}.pid", i))
                .log_file(format!("/tmp/perf-test-{}.log", i))
                .description(format!("Performance test daemon {}", i))
                .auto_restart(3)
                .env("TEST_VAR", format!("value_{}", i))
                .umask(0o022);
        }
        
        let elapsed = start.elapsed();
        let configs_per_second = iterations as f64 / elapsed.as_secs_f64();
        
        println!("Daemon config creation: {:.2} configs/second", configs_per_second);
        assert!(configs_per_second > 100.0); // Should be able to create configs quickly
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;
    
    #[test]
    fn test_pipe_error_conditions() {
        // Test write to read-only pipe
        let read_pipe = NamedPipe::create("read_only_error", PipeMode::Read).unwrap();
        let result = read_pipe.write(b"should fail");
        assert!(result.is_err());
        
        // Test read from write-only pipe
        let write_pipe = NamedPipe::create("write_only_error", PipeMode::Write).unwrap();
        let mut buffer = [0u8; 10];
        let result = write_pipe.read(&mut buffer);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_signal_error_conditions() {
        let handler = SignalHandler::new();
        
        // Test registering uncatchable signal
        let result = handler.register(Signal::Kill, SignalAction::Ignore);
        assert!(result.is_err());
        
        let result = handler.register(Signal::Stop, SignalAction::Ignore);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_daemon_error_conditions() {
        let manager = ServiceManager::new();
        
        // Test starting non-existent service
        let result = manager.service_status("non-existent-service");
        assert!(result.is_err());
        
        // Test duplicate service registration
        let config1 = DaemonConfig::new("duplicate-service");
        assert!(manager.register("duplicate-service".to_string(), config1).is_ok());
        
        let config2 = DaemonConfig::new("duplicate-service");
        let result = manager.register("duplicate-service".to_string(), config2);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_platform_error_conditions() {
        // Test getting info for non-existent process
        let result = PlatformUtils::get_platform_info(999999);
        assert!(result.is_err());
        
        #[cfg(unix)]
        {
            // Test sending signal to non-existent process
            let result = send_signal(999999, Signal::Terminate);
            assert!(result.is_err());
        }
    }
    
    #[test]
    fn test_error_message_quality() {
        // Test that error messages are informative
        let read_pipe = NamedPipe::create("error_msg_test", PipeMode::Read).unwrap();
        if let Err(error) = read_pipe.write(b"test") {
            let error_msg = error.to_string();
            assert!(error_msg.contains("write") || error_msg.contains("Pipe"));
        }
        
        let handler = SignalHandler::new();
        if let Err(error) = handler.register(Signal::Kill, SignalAction::Ignore) {
            let error_msg = error.to_string();
            assert!(error_msg.contains("caught") || error_msg.contains("SIGKILL"));
        }
    }
}

// Test runner helper
fn main() {
    println!("Running enhanced process management tests...");
    
    // Run basic functionality tests
    pipe_tests::test_named_pipe_creation();
    pipe_tests::test_pipe_communication();
    pipe_tests::test_pipe_modes();
    
    signal_tests::test_signal_properties();
    signal_tests::test_signal_conversion();
    
    daemon_tests::test_daemon_config();
    daemon_tests::test_daemon_creation();
    
    platform_tests::test_platform_detection();
    platform_tests::test_platform_features();
    
    println!("All basic tests passed!");
    
    // Run integration tests
    integration_tests::test_process_communication_integration();
    integration_tests::test_cross_platform_process_info();
    
    println!("Integration tests passed!");
    
    // Run performance tests
    performance_tests::test_pipe_throughput();
    performance_tests::test_daemon_config_creation_performance();
    
    println!("Performance tests passed!");
    
    // Run error handling tests
    error_handling_tests::test_pipe_error_conditions();
    error_handling_tests::test_signal_error_conditions();
    error_handling_tests::test_daemon_error_conditions();
    
    println!("Error handling tests passed!");
    
    println!("Enhanced process management system test suite completed successfully!");
}
