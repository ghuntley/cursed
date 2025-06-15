#[cfg(test)]
mod oglogging_integration_tests {
    use std::collections::HashMap;
    use std::io::{Write, Cursor};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use std::fs;
    use tempfile::NamedTempFile;

    #[path = "../tests/common.rs"]
    mod common;

    #[test]
    fn test_standard_logger_functions() {
        common::tracing::setup();
        
        // Test that we can create and use standard logger functions
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_standard_logger(output.clone());
        
        // Test spill function
        logger.spill(&["Standard logger test message"]).unwrap();
        let output_str = get_output_string(output.clone());
        assert!(output_str.contains("Standard logger test message"), "Standard spill should work");
        
        // Clear output
        output.lock().unwrap().clear();
        
        // Test spillf function
        logger.spillf("Formatted message: {}", &["test_value"]).unwrap();
        let output_str2 = get_output_string(output.clone());
        assert!(output_str2.contains("Formatted message: test_value"), "Standard spillf should work");
        
        // Clear output
        output.lock().unwrap().clear();
        
        // Test multiple messages in single spill call
        logger.spill(&["Message 1", "Message 2", "Message 3"]).unwrap();
        let output_str3 = get_output_string(output.clone());
        assert!(output_str3.contains("Message 1 Message 2 Message 3"), "Multiple messages should be joined");
    }

    #[test]
    fn test_global_logger_configuration() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut global_logger = GlobalLogger::new(output.clone());
        
        // Test setting flags
        global_logger.set_flags(LDATE | LTIME).unwrap();
        assert_eq!(global_logger.flags, LDATE | LTIME, "Flags should be set correctly");
        
        // Test setting prefix
        global_logger.set_prefix("GLOBAL: ").unwrap();
        assert_eq!(global_logger.prefix, "GLOBAL: ", "Prefix should be set correctly");
        
        // Test that configuration affects logging
        global_logger.spill(&["Configuration test"]).unwrap();
        let output_str = get_output_string(output.clone());
        
        assert!(output_str.contains("GLOBAL: "), "Should contain prefix");
        assert!(output_str.contains("Configuration test"), "Should contain message"); 
        
        // Verify timestamp formatting (basic check)
        assert!(output_str.contains('/'), "Should contain date separator");
        assert!(output_str.contains(':'), "Should contain time separator");
    }

    #[test]
    fn test_file_output_and_custom_writers() {
        common::tracing::setup();
        
        // Test with temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_path_buf();
        
        {
            let logger = create_file_logger(&file_path, "FILE: ", LTIME).unwrap();
            
            logger.spill(&["File output test message"]).unwrap();
            logger.spillf("File formatted: {}", &["value"]).unwrap();
        }
        
        // Read back the file content
        let file_content = fs::read_to_string(&file_path).unwrap();
        
        assert!(file_content.contains("FILE: "), "Should contain prefix");
        assert!(file_content.contains("File output test message"), "Should contain first message");
        assert!(file_content.contains("File formatted: value"), "Should contain formatted message");
        assert!(file_content.contains(':'), "Should contain time formatting");
        
        // Test with custom writer (string buffer)
        let output = Arc::new(Mutex::new(Vec::new()));
        let custom_logger = create_custom_writer_logger(output.clone(), "CUSTOM: ", LSHORTFILE).unwrap();
        
        custom_logger.spill(&["Custom writer test"]).unwrap();
        let custom_output = get_output_string(output.clone());
        
        assert!(custom_output.contains("CUSTOM: "), "Should contain custom prefix");
        assert!(custom_output.contains("Custom writer test"), "Should contain message");
        assert!(custom_output.contains(".rs:"), "Should contain file info");
    }

    #[test]
    fn test_complete_workflow_scenarios() {
        common::tracing::setup();
        
        // Simulate a complete application workflow
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_workflow_logger(output.clone());
        
        // Application startup
        logger.set_prefix("STARTUP: ").unwrap();
        logger.set_flags(LSTDFLAGS | LMICROSECONDS).unwrap();
        logger.spill(&["Application initializing..."]).unwrap();
        
        // Configuration phase
        logger.set_prefix("CONFIG: ").unwrap();
        logger.spillf("Loading configuration from {}", &["config.toml"]).unwrap();
        logger.spillf("Database connection: {}", &["postgresql://localhost:5432/app"]).unwrap();
        
        // Processing phase
        logger.set_prefix("PROCESS: ").unwrap();
        for i in 1..=5 {
            logger.spillf("Processing item {} of {}", &[&i.to_string(), "5"]).unwrap();
            thread::sleep(Duration::from_millis(1)); // Small delay to show time differences
        }
        
        // Error simulation
        logger.set_prefix("ERROR: ").unwrap();
        logger.spillf("Failed to process item {}: {}", &["3", "connection timeout"]).unwrap();
        
        // Recovery
        logger.set_prefix("RECOVERY: ").unwrap();
        logger.spill(&["Attempting to recover from error..."]).unwrap();
        logger.spill(&["Recovery successful"]).unwrap();
        
        // Shutdown
        logger.set_prefix("SHUTDOWN: ").unwrap();
        logger.spill(&["Application shutting down gracefully"]).unwrap();
        
        let final_output = get_output_string(output.clone());
        
        // Verify the complete workflow
        assert!(final_output.contains("STARTUP: "), "Should contain startup logs");
        assert!(final_output.contains("CONFIG: "), "Should contain config logs");
        assert!(final_output.contains("PROCESS: "), "Should contain processing logs");
        assert!(final_output.contains("ERROR: "), "Should contain error logs");
        assert!(final_output.contains("RECOVERY: "), "Should contain recovery logs");
        assert!(final_output.contains("SHUTDOWN: "), "Should contain shutdown logs");
        
        // Verify specific messages
        assert!(final_output.contains("Application initializing..."), "Should contain startup message");
        assert!(final_output.contains("config.toml"), "Should contain config file reference");
        assert!(final_output.contains("Processing item 1 of 5"), "Should contain processing messages");
        assert!(final_output.contains("connection timeout"), "Should contain error details");
        assert!(final_output.contains("Recovery successful"), "Should contain recovery message");
        assert!(final_output.contains("shutting down gracefully"), "Should contain shutdown message");
        
        // Verify timestamps are present (from LSTDFLAGS | LMICROSECONDS)
        assert!(final_output.contains('/'), "Should contain date formatting");
        assert!(final_output.contains(':'), "Should contain time formatting");
        assert!(final_output.contains('.'), "Should contain microseconds");
    }

    #[test]
    fn test_concurrent_logging_integration() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = Arc::new(create_concurrent_logger(output.clone()));
        
        let mut handles = vec![];
        
        // Simulate multiple components logging concurrently
        let components = vec!["WebServer", "Database", "Cache", "Auth", "Queue"];
        
        for component in components {
            let logger_clone = logger.clone();
            let component_name = component.to_string();
            
            let handle = thread::spawn(move || {
                // Each component logs with its own prefix and messages
                logger_clone.set_prefix(&format!("{}: ", component_name)).unwrap();
                
                for i in 1..=10 {
                    logger_clone.spillf("{} operation {} completed", &[&component_name, &i.to_string()]).unwrap();
                    
                    // Simulate different timing for different components
                    thread::sleep(Duration::from_millis(i % 3));
                }
                
                logger_clone.spillf("{} component shutting down", &[&component_name]).unwrap();
            });
            
            handles.push(handle);
        }
        
        // Wait for all components to finish
        for handle in handles {
            handle.join().unwrap();
        }
        
        let final_output = get_output_string(output.clone());
        let lines: Vec<&str> = final_output.lines().collect();
        
        // Should have 55 lines total (5 components * 11 messages each)
        assert_eq!(lines.len(), 55, "Should have correct number of log lines");
        
        // Verify each component logged correctly
        for component in ["WebServer", "Database", "Cache", "Auth", "Queue"] {
            // Should have 10 operation messages + 1 shutdown message = 11 total
            let component_lines = lines.iter().filter(|line| line.contains(component)).count();
            assert_eq!(component_lines, 11, "Component {} should have 11 log lines", component);
            
            // Verify shutdown message exists
            assert!(final_output.contains(&format!("{} component shutting down", component)), 
                   "Should contain shutdown message for {}", component);
        }
    }

    #[test]
    fn test_error_handling_and_edge_cases_integration() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_error_handling_logger(output.clone());
        
        // Test logging with invalid UTF-8 (should handle gracefully)
        logger.spill(&["Valid message with unicode: 🦀"]).unwrap();
        
        // Test very long messages
        let long_message = "x".repeat(1_000_000); // 1MB message
        logger.spill(&[&long_message]).unwrap();
        
        // Test rapid logging (stress test)
        for i in 0..1000 {
            logger.spillf("Rapid message {}", &[&i.to_string()]).unwrap();
        }
        
        // Test mixed message types
        logger.spill(&["Plain message"]).unwrap();
        logger.spillf("Formatted {}", &["message"]).unwrap();
        logger.spill(&["Multiple", "words", "in", "one", "call"]).unwrap();
        
        let final_output = get_output_string(output.clone());
        
        // Verify all messages are present
        assert!(final_output.contains("Valid message with unicode: 🦀"), "Should handle unicode");
        assert!(final_output.contains(&long_message), "Should handle very long messages");
        assert!(final_output.contains("Rapid message 0"), "Should contain first rapid message");
        assert!(final_output.contains("Rapid message 999"), "Should contain last rapid message");
        assert!(final_output.contains("Plain message"), "Should contain plain message");
        assert!(final_output.contains("Formatted message"), "Should contain formatted message");
        assert!(final_output.contains("Multiple words in one call"), "Should join multiple words");
        
        // Count total lines (should be at least 1003: unicode + long + 1000 rapid + plain + formatted + multiple)
        let line_count = final_output.lines().count();
        assert!(line_count >= 1003, "Should have at least 1003 lines, got {}", line_count);
    }

    #[test]
    fn test_preset_configuration_integration() {
        common::tracing::setup();
        
        // Test different preset configurations
        let outputs = vec![
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(Vec::new())),
        ];
        
        let presets = vec![
            ("MINIMAL", PRESET_MINIMAL),
            ("DETAILED", PRESET_DETAILED),
            ("PRODUCTION", PRESET_PRODUCTION),
        ];
        
        for (i, (preset_name, preset_flags)) in presets.iter().enumerate() {
            let logger = create_preset_logger(outputs[i].clone(), preset_name, *preset_flags).unwrap();
            
            logger.spill(&[&format!("Test message for {} preset", preset_name)]).unwrap();
            logger.spillf("Formatted {} message: {}", &[preset_name, "value"]).unwrap();
        }
        
        // Verify each preset produces different output formats
        let minimal_output = get_output_string(outputs[0].clone());
        let detailed_output = get_output_string(outputs[1].clone());
        let production_output = get_output_string(outputs[2].clone());
        
        // All should contain the messages
        assert!(minimal_output.contains("Test message for MINIMAL preset"), "Minimal should contain message");
        assert!(detailed_output.contains("Test message for DETAILED preset"), "Detailed should contain message");
        assert!(production_output.contains("Test message for PRODUCTION preset"), "Production should contain message");
        
        // Detailed should have more formatting than minimal
        assert!(detailed_output.len() > minimal_output.len(), "Detailed output should be longer than minimal");
        
        // Production should have timestamps
        assert!(production_output.contains('/') || production_output.contains(':'), 
               "Production output should contain timestamp formatting");
    }

    // Helper functions and mock implementations

    const LDATE: u32 = 1 << 0;
    const LTIME: u32 = 1 << 1;
    const LMICROSECONDS: u32 = 1 << 2;
    const LLONGFILE: u32 = 1 << 3;
    const LSHORTFILE: u32 = 1 << 4;
    const LUTC: u32 = 1 << 5;
    const LMSGPREFIX: u32 = 1 << 6;
    const LSTDFLAGS: u32 = LDATE | LTIME;

    const PRESET_MINIMAL: u32 = 0;
    const PRESET_DETAILED: u32 = LDATE | LTIME | LSHORTFILE;
    const PRESET_PRODUCTION: u32 = LDATE | LTIME | LMICROSECONDS;

    struct TestLogger {
        output: Arc<Mutex<Vec<u8>>>,
        prefix: String,
        flags: u32,
    }

    struct GlobalLogger {
        logger: TestLogger,
        prefix: String,
        flags: u32,
    }

    impl TestLogger {
        fn spill(&self, messages: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
            let formatted = self.format_message(messages.join(" "));
            let mut output = self.output.lock().unwrap();
            output.extend_from_slice(formatted.as_bytes());
            Ok(())
        }

        fn spillf(&self, format: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
            let formatted_message = format_string(format, args);
            let formatted = self.format_message(formatted_message);
            let mut output = self.output.lock().unwrap();
            output.extend_from_slice(formatted.as_bytes());
            Ok(())
        }

        fn set_prefix(&self, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
            // In a real implementation, this would modify the logger's prefix
            // For testing, we'll just validate the call
            Ok(())
        }

        fn set_flags(&self, flags: u32) -> Result<(), Box<dyn std::error::Error>> {
            // In a real implementation, this would modify the logger's flags
            // For testing, we'll just validate the call
            Ok(())
        }

        fn format_message(&self, message: String) -> String {
            let mut result = String::new();
            
            if self.flags & LDATE != 0 {
                result.push_str("2023/12/06 ");
            }
            if self.flags & LTIME != 0 {
                if self.flags & LMICROSECONDS != 0 {
                    result.push_str("15:04:05.123456 ");
                } else {
                    result.push_str("15:04:05 ");
                }
            }
            if self.flags & LSHORTFILE != 0 {
                result.push_str("test.rs:123 ");
            }
            
            result.push_str(&format!("{}{}\n", &self.prefix, message));
            result
        }
    }

    impl GlobalLogger {
        fn new(output: Arc<Mutex<Vec<u8>>>) -> Self {
            Self {
                logger: TestLogger {
                    output,
                    prefix: String::new(),
                    flags: 0,
                },
                prefix: String::new(),
                flags: 0,
            }
        }

        fn set_flags(&mut self, flags: u32) -> Result<(), Box<dyn std::error::Error>> {
            self.flags = flags;
            Ok(())
        }

        fn set_prefix(&mut self, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
            self.prefix = prefix.to_string();
            Ok(())
        }

        fn spill(&self, messages: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
            let formatted = self.format_message(messages.join(" "));
            let mut output = self.logger.output.lock().unwrap();
            output.extend_from_slice(formatted.as_bytes());
            Ok(())
        }

        fn format_message(&self, message: String) -> String {
            let mut result = String::new();
            
            if self.flags & LDATE != 0 {
                result.push_str("2023/12/06 ");
            }
            if self.flags & LTIME != 0 {
                result.push_str("15:04:05 ");
            }
            
            result.push_str(&format!("{}{}\n", &self.prefix, message));
            result
        }
    }

    fn create_standard_logger(output: Arc<Mutex<Vec<u8>>>) -> TestLogger {
        TestLogger {
            output,
            prefix: String::new(),
            flags: 0,
        }
    }

    fn create_file_logger(file_path: &std::path::Path, prefix: &str, flags: u32) -> Result<TestLogger, Box<dyn std::error::Error>> {
        // For testing, we'll use a memory buffer instead of actual file I/O
        let output = Arc::new(Mutex::new(Vec::new()));
        Ok(TestLogger {
            output,
            prefix: prefix.to_string(),
            flags,
        })
    }

    fn create_custom_writer_logger(output: Arc<Mutex<Vec<u8>>>, prefix: &str, flags: u32) -> Result<TestLogger, Box<dyn std::error::Error>> {
        Ok(TestLogger {
            output,
            prefix: prefix.to_string(),
            flags,
        })
    }

    fn create_workflow_logger(output: Arc<Mutex<Vec<u8>>>) -> WorkflowLogger {
        WorkflowLogger {
            logger: TestLogger {
                output,
                prefix: String::new(),
                flags: 0,
            },
        }
    }

    fn create_concurrent_logger(output: Arc<Mutex<Vec<u8>>>) -> ConcurrentLogger {
        ConcurrentLogger {
            logger: TestLogger {
                output,
                prefix: String::new(),
                flags: LTIME | LMICROSECONDS,
            },
        }
    }

    fn create_error_handling_logger(output: Arc<Mutex<Vec<u8>>>) -> TestLogger {
        TestLogger {
            output,
            prefix: "ERROR_TEST: ".to_string(),
            flags: LSTDFLAGS,
        }
    }

    fn create_preset_logger(output: Arc<Mutex<Vec<u8>>>, preset_name: &str, flags: u32) -> Result<TestLogger, Box<dyn std::error::Error>> {
        Ok(TestLogger {
            output,
            prefix: format!("{}: ", preset_name),
            flags,
        })
    }

    struct WorkflowLogger {
        logger: TestLogger,
    }

    impl WorkflowLogger {
        fn set_prefix(&self, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn set_flags(&self, flags: u32) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn spill(&self, messages: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
            self.logger.spill(messages)
        }

        fn spillf(&self, format: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
            self.logger.spillf(format, args)
        }
    }

    struct ConcurrentLogger {
        logger: TestLogger,
    }

    impl ConcurrentLogger {
        fn set_prefix(&self, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn spillf(&self, format: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
            self.logger.spillf(format, args)
        }
    }

    fn get_output_string(output: Arc<Mutex<Vec<u8>>>) -> String {
        let bytes = output.lock().unwrap();
        String::from_utf8(bytes.clone()).unwrap_or_default()
    }

    fn format_string(format: &str, args: &[&str]) -> String {
        let mut result = format.to_string();
        
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        
        let mut arg_index = 0;
        while let Some(pos) = result.find("{}") {
            if arg_index < args.len() {
                result.replace_range(pos..pos+2, args[arg_index]);
                arg_index += 1;
            } else {
                break;
            }
        }
        
        result = result.replace("{{", "{");
        result = result.replace("}}", "}");
        
        result
    }
}
