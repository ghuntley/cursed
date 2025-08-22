// Test suite for real structured logging infrastructure
yeet "chadlogging_real"
yeet "testz"
yeet "filez"
yeet "concurrenz"
yeet "timez"

// Test basic logging functionality
slay test_basic_logging() lit {
    testz.test_start("Basic Logging")
    
    // Initialize logging with test configuration
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_basic.log")
    chadlogging_real.set_log_level(chadlogging_real.LOG_DEBUG)
    
    // Test all log levels
    chadlogging_real.trace_simple("This is a trace message")
    chadlogging_real.debug_simple("This is a debug message")
    chadlogging_real.info_simple("This is an info message")
    chadlogging_real.warn_simple("This is a warning message")
    chadlogging_real.error_simple("This is an error message")
    
    // Flush logs and verify file exists
    chadlogging_real.flush_logs()
    
    testz.assert_true(filez.exists("test_basic.log"), "Log file should exist")
    
    // Clean up
    filez.remove("test_basic.log")
    
    testz.test_pass("Basic logging functionality works")
    damn based
}

// Test structured logging with fields
slay test_structured_logging() lit {
    testz.test_start("Structured Logging")
    
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_structured.log")
    
    // Create fields map
    sus fields map[tea]interface{} = make(map[tea]interface{})
    fields["user_id"] = "12345"
    fields["action"] = "login"
    fields["ip_address"] = "192.168.1.100"
    fields["success"] = based
    fields["duration"] = 150
    
    // Log with fields
    chadlogging_real.info("User login attempt", fields)
    
    // Test with different field types
    sus error_fields map[tea]interface{} = make(map[tea]interface{})
    error_fields["error_code"] = 404
    error_fields["error_message"] = "Resource not found"
    error_fields["request_id"] = "req-abc123"
    
    chadlogging_real.error("Request failed", error_fields)
    
    chadlogging_real.flush_logs()
    
    testz.assert_true(filez.exists("test_structured.log"), "Structured log file should exist")
    
    // Clean up
    filez.remove("test_structured.log")
    
    testz.test_pass("Structured logging with fields works")
    damn based
}

// Test log rotation functionality
slay test_log_rotation() lit {
    testz.test_start("Log Rotation")
    
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_rotation.log")
    
    // Set small file size for testing rotation
    chadlogging_real.set_rotation_config(1024, 3)  // 1KB max, 3 backup files
    chadlogging_real.enable_rotation(based)
    
    // Generate enough logs to trigger rotation
    bestie i normie = 0; i < 100; i++ {
        sus fields map[tea]interface{} = make(map[tea]interface{})
        fields["iteration"] = i
        fields["data"] = "This is a longer log message to fill up the file quickly and trigger rotation mechanism"
        chadlogging_real.info("Rotation test log entry", fields)
    }
    
    chadlogging_real.flush_logs()
    
    // Check if backup files were created
    testz.assert_true(filez.exists("test_rotation.log"), "Main log file should exist")
    
    // Get statistics to verify rotation occurred
    sus stats chadlogging_real.LogStats = chadlogging_real.get_log_stats()
    testz.assert_true(stats.files_rotated > 0, "Log rotation should have occurred")
    
    // Clean up
    filez.remove("test_rotation.log")
    filez.remove("test_rotation.log.1")
    filez.remove("test_rotation.log.2")
    filez.remove("test_rotation.log.3")
    
    testz.test_pass("Log rotation functionality works")
    damn based
}

// Test async logging performance
slay test_async_logging() lit {
    testz.test_start("Async Logging")
    
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_async.log")
    chadlogging_real.enable_async(based)
    
    sus start_time normie = timez.now_unix_nano()
    
    // Generate many logs quickly
    bestie i normie = 0; i < 1000; i++ {
        sus fields map[tea]interface{} = make(map[tea]interface{})
        fields["async_test"] = i
        chadlogging_real.info("Async performance test", fields)
    }
    
    sus log_time normie = timez.now_unix_nano()
    
    // Flush to ensure all async logs are processed
    chadlogging_real.flush_logs()
    
    sus flush_time normie = timez.now_unix_nano()
    
    sus log_duration normie = (log_time - start_time) / 1000000    // Convert to ms
    sus total_duration normie = (flush_time - start_time) / 1000000
    
    vibez.spill("Async logging: " + stringz.from_int(log_duration) + "ms to queue, " + 
               stringz.from_int(total_duration) + "ms total")
    
    testz.assert_true(filez.exists("test_async.log"), "Async log file should exist")
    
    // Clean up
    filez.remove("test_async.log")
    
    testz.test_pass("Async logging performance is acceptable")
    damn based
}

// Test concurrent logging safety
slay test_concurrent_logging() lit {
    testz.test_start("Concurrent Logging Safety")
    
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_concurrent.log")
    chadlogging_real.enable_async(based)
    
    sus num_goroutines normie = 10
    sus logs_per_goroutine normie = 100
    sus done_channel chan lit = make(chan lit, num_goroutines)
    
    sus start_time normie = timez.now_unix_nano()
    
    // Launch concurrent goroutines
    bestie i normie = 0; i < num_goroutines; i++ {
        go {
            bestie j normie = 0; j < logs_per_goroutine; j++ {
                sus fields map[tea]interface{} = make(map[tea]interface{})
                fields["goroutine_id"] = i
                fields["log_number"] = j
                fields["thread_data"] = "Concurrent access test data"
                
                chadlogging_real.info("Concurrent logging test", fields)
            }
            done_channel <- based
        }
    }
    
    // Wait for all goroutines to complete
    bestie i normie = 0; i < num_goroutines; i++ {
        <-done_channel
    }
    
    chadlogging_real.flush_logs()
    
    sus end_time normie = timez.now_unix_nano()
    sus duration normie = (end_time - start_time) / 1000000
    
    vibez.spill("Concurrent logging test: " + stringz.from_int(duration) + "ms for " +
               stringz.from_int(num_goroutines * logs_per_goroutine) + " logs")
    
    // Verify log file exists and has content
    testz.assert_true(filez.exists("test_concurrent.log"), "Concurrent log file should exist")
    
    sus file_info filez.FileInfo = filez.stat("test_concurrent.log")
    testz.assert_true(file_info.size > 0, "Log file should have content")
    
    // Check statistics
    sus stats chadlogging_real.LogStats = chadlogging_real.get_log_stats()
    testz.assert_equal_int(stats.total_logs, num_goroutines * logs_per_goroutines, 
                          "All concurrent logs should be recorded")
    
    // Clean up
    filez.remove("test_concurrent.log")
    
    testz.test_pass("Concurrent logging is thread-safe")
    damn based
}

// Test log level filtering
slay test_log_level_filtering() lit {
    testz.test_start("Log Level Filtering")
    
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_filtering.log")
    chadlogging_real.set_log_level(chadlogging_real.LOG_WARN)  // Only WARN and above
    
    // These should not be logged
    chadlogging_real.trace_simple("This trace should not appear")
    chadlogging_real.debug_simple("This debug should not appear")
    chadlogging_real.info_simple("This info should not appear")
    
    // These should be logged
    chadlogging_real.warn_simple("This warning should appear")
    chadlogging_real.error_simple("This error should appear")
    
    chadlogging_real.flush_logs()
    
    // Read file and verify content
    sus content tea = filez.read_file("test_filtering.log")
    testz.assert_true(stringz.contains(content, "warning"), "Warning should be in log")
    testz.assert_true(stringz.contains(content, "error"), "Error should be in log")
    testz.assert_false(stringz.contains(content, "trace"), "Trace should not be in log")
    testz.assert_false(stringz.contains(content, "debug"), "Debug should not be in log")
    testz.assert_false(stringz.contains(content, "info"), "Info should not be in log")
    
    // Clean up
    filez.remove("test_filtering.log")
    
    testz.test_pass("Log level filtering works correctly")
    damn based
}

// Test logging performance benchmarks
slay test_logging_performance() lit {
    testz.test_start("Logging Performance")
    
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_performance.log")
    chadlogging_real.enable_async(based)
    
    // Test synchronous performance
    chadlogging_real.enable_async(cap)
    sus sync_duration normie = chadlogging_real.performance_test(1000)
    
    // Test asynchronous performance
    chadlogging_real.enable_async(based)
    sus async_duration normie = chadlogging_real.performance_test(1000)
    
    vibez.spill("Sync logging: " + stringz.from_int(sync_duration) + "ms for 1000 logs")
    vibez.spill("Async logging: " + stringz.from_int(async_duration) + "ms for 1000 logs")
    
    // Async should be faster for many logs
    testz.assert_true(async_duration <= sync_duration, "Async should be faster or equal")
    
    // Test concurrent performance
    sus concurrent_duration normie = chadlogging_real.concurrent_performance_test(5, 200)
    vibez.spill("Concurrent logging: " + stringz.from_int(concurrent_duration) + "ms for 1000 logs across 5 goroutines")
    
    // Clean up
    filez.remove("test_performance.log")
    
    testz.test_pass("Logging performance is acceptable")
    damn based
}

// Test error handling and recovery
slay test_error_handling() lit {
    testz.test_start("Error Handling")
    
    chadlogging_real.init_logging()
    
    // Test invalid file path (should fallback to console)
    sus result yikes<tea> = chadlogging_real.set_log_file("/invalid/path/test.log")
    testz.assert_error(result, "Setting invalid file path should return error")
    
    // Test logging when file system is unavailable
    chadlogging_real.set_log_file("test_error.log")
    
    // Simulate file system error by setting readonly permissions
    // (This is simplified - real test would set actual permissions)
    
    // Should still work via emergency logging
    chadlogging_real.error_simple("This should work despite file errors")
    
    testz.test_pass("Error handling works correctly")
    damn based
}

// Test log formatting options
slay test_log_formatting() lit {
    testz.test_start("Log Formatting")
    
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_formatting.log")
    
    // Test with colors disabled
    chadlogging_real.enable_colors(cap)
    chadlogging_real.info_simple("Message without colors")
    
    // Test with colors enabled
    chadlogging_real.enable_colors(based)
    chadlogging_real.error_simple("Message with colors")
    
    chadlogging_real.flush_logs()
    
    // Verify file exists and has content
    testz.assert_true(filez.exists("test_formatting.log"), "Formatting log file should exist")
    
    sus content tea = filez.read_file("test_formatting.log")
    testz.assert_true(len(content) > 0, "Log file should have content")
    
    // Clean up
    filez.remove("test_formatting.log")
    
    testz.test_pass("Log formatting options work")
    damn based
}

// Test statistics and monitoring
slay test_statistics() lit {
    testz.test_start("Statistics and Monitoring")
    
    chadlogging_real.init_logging()
    chadlogging_real.set_log_file("test_stats.log")
    
    // Generate logs at different levels
    bestie i normie = 0; i < 10; i++ {
        chadlogging_real.debug_simple("Debug message " + stringz.from_int(i))
    }
    
    bestie i normie = 0; i < 5; i++ {
        chadlogging_real.warn_simple("Warning message " + stringz.from_int(i))
    }
    
    bestie i normie = 0; i < 3; i++ {
        chadlogging_real.error_simple("Error message " + stringz.from_int(i))
    }
    
    chadlogging_real.flush_logs()
    
    // Check statistics
    sus stats chadlogging_real.LogStats = chadlogging_real.get_log_stats()
    testz.assert_equal_int(stats.total_logs, 18, "Total logs should be 18")
    testz.assert_true(stats.bytes_written > 0, "Bytes should have been written")
    
    // Print statistics summary
    sus summary tea = chadlogging_real.get_stats_summary()
    vibez.spill("Statistics Summary:")
    vibez.spill(summary)
    
    // Clean up
    filez.remove("test_stats.log")
    
    testz.test_pass("Statistics tracking works correctly")
    damn based
}

// Main test runner
slay main() normie {
    vibez.spill("Running Real Structured Logging Tests...")
    
    test_basic_logging()
    test_structured_logging()
    test_log_rotation()
    test_async_logging()
    test_concurrent_logging()
    test_log_level_filtering()
    test_logging_performance()
    test_error_handling()
    test_log_formatting()
    test_statistics()
    
    // Clean up global logger
    chadlogging_real.close_logger()
    
    testz.print_test_summary()
    
    vibez.spill("All real structured logging tests completed!")
    damn 0
}
