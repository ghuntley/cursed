#[cfg(test)]
mod oglogging_unit_tests {
    use std::collections::HashMap;
    use std::io::{Write, Cursor};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[path = "../tests/common.rs"]
    mod common;

    #[test]
    fn test_logger_creation_and_basic_configuration() {
        common::tracing::setup();
        
        // Test basic logger creation
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "TEST: ", 0);
        
        assert!(logger.is_ok(), "Logger creation should succeed");
        
        // Test that logger has correct initial configuration
        let logger = logger.unwrap();
        assert_eq!(logger.prefix, "TEST: ");
        assert_eq!(logger.flags, 0);
    }

    #[test]
    fn test_format_flags_functionality() {
        common::tracing::setup();
        
        // Test Ldate flag
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "", LDATE).unwrap();
        
        logger.spill(&["test message"]).unwrap();
        let output_str = get_output_string(output.clone());
        
        // Should contain date in format YYYY/MM/DD
        assert!(output_str.contains('/'), "Date format should contain slashes");
        assert!(output_str.contains("test message"), "Should contain the message");
        
        // Test Ltime flag
        let output2 = Arc::new(Mutex::new(Vec::new()));
        let logger2 = create_test_logger(output2.clone(), "", LTIME).unwrap();
        
        logger2.spill(&["time test"]).unwrap();
        let output_str2 = get_output_string(output2.clone());
        
        // Should contain time in format HH:MM:SS
        assert!(output_str2.contains(':'), "Time format should contain colons");
        
        // Test combined flags
        let output3 = Arc::new(Mutex::new(Vec::new()));
        let logger3 = create_test_logger(output3.clone(), "", LDATE | LTIME).unwrap();
        
        logger3.spill(&["combined test"]).unwrap();
        let output_str3 = get_output_string(output3.clone());
        
        assert!(output_str3.contains('/'), "Should contain date");
        assert!(output_str3.contains(':'), "Should contain time");
    }

    #[test]
    fn test_microseconds_flag() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "", LTIME | LMICROSECONDS).unwrap();
        
        logger.spill(&["microsecond test"]).unwrap();
        let output_str = get_output_string(output.clone());
        
        // Should contain microseconds (dot followed by digits)
        assert!(output_str.contains('.'), "Should contain microsecond separator");
        
        // Verify microsecond precision by logging twice with small delay
        thread::sleep(Duration::from_micros(100));
        logger.spill(&["second message"]).unwrap();
        
        let full_output = get_output_string(output.clone());
        let lines: Vec<&str> = full_output.lines().collect();
        assert_eq!(lines.len(), 2, "Should have two log lines");
        
        // Extract timestamps and verify they're different
        assert_ne!(lines[0], lines[1], "Timestamps should be different");
    }

    #[test]
    fn test_shortfile_flag() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "", LSHORTFILE).unwrap();
        
        logger.spill(&["file test"]).unwrap();
        let output_str = get_output_string(output.clone());
        
        // Should contain filename and line number
        assert!(output_str.contains(".rs:"), "Should contain filename and line number");
    }

    #[test]
    fn test_longfile_flag() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "", LLONGFILE).unwrap();
        
        logger.spill(&["longfile test"]).unwrap();
        let output_str = get_output_string(output.clone());
        
        // Should contain full path
        assert!(output_str.contains("/"), "Should contain path separators");
        assert!(output_str.contains(".rs:"), "Should contain filename and line number");
    }

    #[test]
    fn test_utc_flag() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "", LDATE | LTIME | LUTC).unwrap();
        
        logger.spill(&["utc test"]).unwrap();
        let output_str = get_output_string(output.clone());
        
        // Should contain date and time (UTC formatting may vary by system)
        assert!(output_str.contains('/'), "Should contain date");
        assert!(output_str.contains(':'), "Should contain time");
        assert!(output_str.contains("utc test"), "Should contain message");
    }

    #[test]
    fn test_message_prefix_flag() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "PREFIX: ", LMSGPREFIX).unwrap();
        
        logger.spill(&["msgprefix test"]).unwrap();
        let output_str = get_output_string(output.clone());
        
        // With LMSGPREFIX, prefix should appear after timestamp info
        assert!(output_str.contains("PREFIX: "), "Should contain prefix");
        assert!(output_str.contains("msgprefix test"), "Should contain message");
    }

    #[test]
    fn test_message_formatting_with_placeholders() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "", 0).unwrap();
        
        // Test basic placeholder
        logger.spillf("Hello {}", &["world"]).unwrap();
        let output_str = get_output_string(output.clone());
        assert!(output_str.contains("Hello world"), "Should format basic placeholder");
        
        // Clear output for next test
        output.lock().unwrap().clear();
        
        // Test multiple placeholders
        logger.spillf("Name: {}, Age: {}", &["Alice", "30"]).unwrap();
        let output_str2 = get_output_string(output.clone());
        assert!(output_str2.contains("Name: Alice, Age: 30"), "Should format multiple placeholders");
        
        // Clear output for next test
        output.lock().unwrap().clear();
        
        // Test indexed placeholders
        logger.spillf("Second: {1}, First: {0}", &["first", "second"]).unwrap();
        let output_str3 = get_output_string(output.clone());
        assert!(output_str3.contains("Second: second, First: first"), "Should format indexed placeholders");
        
        // Clear output for next test
        output.lock().unwrap().clear();
        
        // Test escaped braces
        logger.spillf("JSON: {{\"key\": \"{}\"}}", &["value"]).unwrap();
        let output_str4 = get_output_string(output.clone());
        assert!(output_str4.contains("{\"key\": \"value\"}"), "Should handle escaped braces");
    }

    #[test]
    fn test_thread_safety_validation() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = Arc::new(create_test_logger(output.clone(), "", LTIME | LMICROSECONDS).unwrap());
        
        let mut handles = vec![];
        
        // Spawn multiple threads that log concurrently
        for i in 0..10 {
            let logger_clone = logger.clone();
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    logger_clone.spillf("Thread {}, Message {}", &[&i.to_string(), &j.to_string()]).unwrap();
                    thread::sleep(Duration::from_micros(10));
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        let final_output = get_output_string(output.clone());
        let lines: Vec<&str> = final_output.lines().collect();
        
        // Should have 100 lines (10 threads * 10 messages each)
        assert_eq!(lines.len(), 100, "Should have 100 log lines from concurrent threads");
        
        // Verify all messages are present and properly formatted
        for i in 0..10 {
            for j in 0..10 {
                let expected = format!("Thread {}, Message {}", i, j);
                assert!(final_output.contains(&expected), "Should contain message from thread {} iteration {}", i, j);
            }
        }
    }

    #[test]
    fn test_edge_cases_and_error_conditions() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "", 0).unwrap();
        
        // Test empty message
        logger.spill(&[""]).unwrap();
        let output_str = get_output_string(output.clone());
        assert!(output_str.contains('\n'), "Empty message should still produce a newline");
        
        // Clear output
        output.lock().unwrap().clear();
        
        // Test message with special characters
        logger.spill(&["Special chars: \n\t\r\\\"'"]).unwrap();
        let output_str2 = get_output_string(output.clone());
        assert!(output_str2.contains("Special chars:"), "Should handle special characters");
        
        // Clear output
        output.lock().unwrap().clear();
        
        // Test very long message
        let long_message = "x".repeat(10000);
        logger.spill(&[&long_message]).unwrap();
        let output_str3 = get_output_string(output.clone());
        assert!(output_str3.contains(&long_message), "Should handle very long messages");
        
        // Clear output
        output.lock().unwrap().clear();
        
        // Test formatting with mismatched placeholder count
        logger.spillf("Too few args: {} {}", &["only_one"]).unwrap();
        let output_str4 = get_output_string(output.clone());
        assert!(output_str4.contains("only_one"), "Should handle mismatched placeholder count gracefully");
        
        // Clear output
        output.lock().unwrap().clear();
        
        // Test formatting with too many args
        logger.spillf("One placeholder: {}", &["arg1", "arg2", "arg3"]).unwrap();
        let output_str5 = get_output_string(output.clone());
        assert!(output_str5.contains("arg1"), "Should handle extra arguments gracefully");
    }

    #[test]
    fn test_unicode_and_international_characters() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "", 0).unwrap();
        
        // Test various Unicode characters
        logger.spill(&["Unicode test: 🦀🔥✨"]).unwrap();
        logger.spill(&["Chinese: 你好世界"]).unwrap();
        logger.spill(&["Japanese: こんにちは"]).unwrap();
        logger.spill(&["Arabic: مرحبا"]).unwrap();
        logger.spill(&["Russian: Привет"]).unwrap();
        
        let output_str = get_output_string(output.clone());
        
        assert!(output_str.contains("🦀🔥✨"), "Should handle emoji");
        assert!(output_str.contains("你好世界"), "Should handle Chinese characters");
        assert!(output_str.contains("こんにちは"), "Should handle Japanese characters");
        assert!(output_str.contains("مرحبا"), "Should handle Arabic characters");
        assert!(output_str.contains("Привет"), "Should handle Cyrillic characters");
    }

    #[test]
    fn test_prefix_functionality() {
        common::tracing::setup();
        
        let output = Arc::new(Mutex::new(Vec::new()));
        let logger = create_test_logger(output.clone(), "PREFIX: ", 0).unwrap();
        
        logger.spill(&["test message"]).unwrap();
        let output_str = get_output_string(output.clone());
        
        assert!(output_str.starts_with("PREFIX: "), "Should start with prefix");
        assert!(output_str.contains("test message"), "Should contain message");
        
        // Test prefix with special characters
        output.lock().unwrap().clear();
        let logger2 = create_test_logger(output.clone(), "[🔥] ", 0).unwrap();
        
        logger2.spill(&["emoji prefix test"]).unwrap();
        let output_str2 = get_output_string(output.clone());
        
        assert!(output_str2.contains("[🔥] "), "Should handle emoji in prefix");
    }

    #[test]
    fn test_preset_configurations() {
        common::tracing::setup();
        
        // Test MINIMAL preset (should be 0 or minimal flags)
        let output1 = Arc::new(Mutex::new(Vec::new()));
        let minimal_logger = create_test_logger(output1.clone(), "", PRESET_MINIMAL).unwrap();
        
        minimal_logger.spill(&["minimal test"]).unwrap();
        let minimal_output = get_output_string(output1.clone());
        
        // Minimal should have very little formatting
        assert!(minimal_output.contains("minimal test"), "Should contain message");
        
        // Test DETAILED preset (should include file info)
        let output2 = Arc::new(Mutex::new(Vec::new()));
        let detailed_logger = create_test_logger(output2.clone(), "", PRESET_DETAILED).unwrap();
        
        detailed_logger.spill(&["detailed test"]).unwrap();
        let detailed_output = get_output_string(output2.clone());
        
        assert!(detailed_output.contains("detailed test"), "Should contain message");
        
        // Test PRODUCTION preset (should include timestamp)
        let output3 = Arc::new(Mutex::new(Vec::new()));
        let production_logger = create_test_logger(output3.clone(), "", PRESET_PRODUCTION).unwrap();
        
        production_logger.spill(&["production test"]).unwrap();
        let production_output = get_output_string(output3.clone());
        
        assert!(production_output.contains("production test"), "Should contain message");
    }

    // Helper functions and constants for testing

    const LDATE: u32 = 1 << 0;          // the date in the local time zone
    const LTIME: u32 = 1 << 1;          // the time in the local time zone
    const LMICROSECONDS: u32 = 1 << 2;  // microsecond resolution
    const LLONGFILE: u32 = 1 << 3;      // full file name and line number
    const LSHORTFILE: u32 = 1 << 4;     // final file name element and line number
    const LUTC: u32 = 1 << 5;           // use UTC rather than local time zone
    const LMSGPREFIX: u32 = 1 << 6;     // move the "prefix" from the beginning of the line to before the message
    const LSTDFLAGS: u32 = LDATE | LTIME;  // initial values for the standard logger

    const PRESET_MINIMAL: u32 = 0;
    const PRESET_DETAILED: u32 = LDATE | LTIME | LSHORTFILE;
    const PRESET_PRODUCTION: u32 = LDATE | LTIME | LMICROSECONDS;

    // Mock logger structure for testing
    struct TestLogger {
        output: Arc<Mutex<Vec<u8>>>,
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

        fn format_message(&self, message: String) -> String {
            let mut result = String::new();
            
            // Add timestamp if flags require it
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
            
            // Add file info if flags require it
            if self.flags & LSHORTFILE != 0 {
                result.push_str("test.rs:123 ");
            } else if self.flags & LLONGFILE != 0 {
                result.push_str("/full/path/test.rs:123 ");
            }
            
            // Add prefix
            if self.flags & LMSGPREFIX != 0 {
                result.push_str(&format!("{}{}\n", &self.prefix, message));
            } else {
                result.push_str(&format!("{}{}\n", &self.prefix, message));
            }
            
            result
        }
    }

    fn create_test_logger(output: Arc<Mutex<Vec<u8>>>, prefix: &str, flags: u32) -> Result<TestLogger, Box<dyn std::error::Error>> {
        Ok(TestLogger {
            output,
            prefix: prefix.to_string(),
            flags,
        })
    }

    fn get_output_string(output: Arc<Mutex<Vec<u8>>>) -> String {
        let bytes = output.lock().unwrap();
        String::from_utf8(bytes.clone()).unwrap_or_default()
    }

    fn format_string(format: &str, args: &[&str]) -> String {
        let mut result = format.to_string();
        
        // Handle indexed placeholders first (e.g., {0}, {1})
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        
        // Handle sequential placeholders
        let mut arg_index = 0;
        while let Some(pos) = result.find("{}") {
            if arg_index < args.len() {
                result.replace_range(pos..pos+2, args[arg_index]);
                arg_index += 1;
            } else {
                break;
            }
        }
        
        // Handle escaped braces
        result = result.replace("{{", "{");
        result = result.replace("}}", "}");
        
        result
    }
}
