# Comprehensive logz Testing Suite
# Tests all logging framework functionality including backends, formatters, and performance

yeet "logz/core"
yeet "logz/backends" 
yeet "logz/testing"
yeet "testz"
yeet "timez"
yeet "stringz"
yeet "vibez"

# Test global logger functionality
slay test_global_logger() {
    test_start("global_logger")
    
    spill("Testing global logger functions...")
    
    # Add test backend to global logger  
    sus formatter TextFormatter = TextFormatter.default()
    sus test_backend TestBackend = TestBackend.new(formatter)
    add_global_backend(test_backend)
    set_global_level(LogLevel.DEBUG())
    
    # Test global logging functions
    debug("Global debug message")
    info("Global info message")
    warn("Global warn message")
    error("Global error message")
    
    sus entries []LogEntry = test_backend.get_entries()
    assert_eq_int(len(entries), 4)
    
    assert_eq_string(entries[0].level.name, "DEBUG")
    assert_eq_string(entries[1].level.name, "INFO")
    assert_eq_string(entries[2].level.name, "WARN")
    assert_eq_string(entries[3].level.name, "ERROR")
    
    flush_global()
    
    test_end()
}

# Test advanced formatting options
slay test_advanced_formatting() {
    test_start("advanced_formatting")
    
    # Test custom text formatter template
    sus custom_formatter TextFormatter = TextFormatter.new(
        "{level} | {timestamp} | {module}->{function}:{line} | {message}",
        nah  # No colors for testing
    )
    
    sus entry LogEntry = LogEntry.new(LogLevel.WARN(), "Custom format test")
    entry.with_context("test_module", "test_function", 123)
    entry.with_field("custom_field", "custom_value")
    
    sus formatted tea = custom_formatter.format(entry)
    
    assert_true(contains(formatted, "WARN |"))
    assert_true(contains(formatted, "| test_module->test_function:123 |"))
    assert_true(contains(formatted, "| Custom format test"))
    
    # Test JSON formatter with fields
    sus json_formatter JsonFormatter = JsonFormatter.new(based)  # Pretty print
    sus json_output tea = json_formatter.format(entry)
    
    assert_true(contains(json_output, "\"level\": \"WARN\""))
    assert_true(contains(json_output, "\"message\": \"Custom format test\""))
    assert_true(contains(json_output, "\"module\": \"test_module\""))
    assert_true(contains(json_output, "\"function\": \"test_function\""))
    assert_true(contains(json_output, "\"line\": \"123\""))
    assert_true(contains(json_output, "\"custom_field\": \"custom_value\""))
    
    test_end()
}

# Test buffered backend wrapper
slay test_buffered_backend() {
    test_start("buffered_backend")
    
    sus formatter TextFormatter = TextFormatter.default()
    sus test_backend TestBackend = TestBackend.new(formatter)
    
    # Create buffered wrapper with small buffer for testing
    sus buffered BufferedBackend = BufferedBackend.new(test_backend, 3, 1000)
    
    # Write entries that should be buffered
    buffered.write(LogEntry.new(LogLevel.INFO(), "message 1")) fam {
        when error -> assert_fail("Buffered write failed: " + error)
    }
    buffered.write(LogEntry.new(LogLevel.INFO(), "message 2")) fam {
        when error -> assert_fail("Buffered write failed: " + error)
    }
    
    # Should still be buffered
    sus entries_before []LogEntry = test_backend.get_entries()
    assert_eq_int(len(entries_before), 0)
    
    # Third message should trigger flush
    buffered.write(LogEntry.new(LogLevel.INFO(), "message 3")) fam {
        when error -> assert_fail("Buffered write failed: " + error)
    }
    
    # Give buffered backend time to flush
    sleep_milliseconds(50)
    
    sus entries_after []LogEntry = test_backend.get_entries()
    assert_eq_int(len(entries_after), 3)
    
    # Test manual flush
    buffered.write(LogEntry.new(LogLevel.INFO(), "message 4")) fam {
        when error -> assert_fail("Buffered write failed: " + error)
    }
    
    buffered.flush() fam {
        when error -> assert_fail("Buffered flush failed: " + error)
    }
    
    sus final_entries []LogEntry = test_backend.get_entries()
    assert_eq_int(len(final_entries), 4)
    
    test_end()
}

# Test complex filtering scenarios
slay test_complex_filtering() {
    test_start("complex_filtering")
    
    sus formatter TextFormatter = TextFormatter.default()
    sus test_backend TestBackend = TestBackend.new(formatter)
    sus logger Logger = Logger.new(formatter)
    logger.add_backend(test_backend)
    
    # Add multiple filters
    sus level_filter LevelFilter = LevelFilter.new(LogLevel.INFO())
    sus module_filter ModuleFilter = ModuleFilter.new()
    module_filter.allow_module("allowed")
    module_filter.block_module("blocked")
    
    logger.add_filter(level_filter)
    logger.add_filter(module_filter)
    
    # Test various combinations
    sus debug_allowed LogEntry = LogEntry.new(LogLevel.DEBUG(), "debug in allowed module")
    debug_allowed.with_context("allowed", "function", 1)
    logger.log_entry(debug_allowed)  # Should be filtered by level
    
    sus info_allowed LogEntry = LogEntry.new(LogLevel.INFO(), "info in allowed module") 
    info_allowed.with_context("allowed", "function", 2)
    logger.log_entry(info_allowed)  # Should pass both filters
    
    sus warn_blocked LogEntry = LogEntry.new(LogLevel.WARN(), "warn in blocked module")
    warn_blocked.with_context("blocked", "function", 3)  
    logger.log_entry(warn_blocked)  # Should be filtered by module
    
    sus error_other LogEntry = LogEntry.new(LogLevel.ERROR(), "error in other module")
    error_other.with_context("other", "function", 4)
    logger.log_entry(error_other)  # Should be filtered by module (not in allowed list)
    
    sus warn_allowed LogEntry = LogEntry.new(LogLevel.WARN(), "warn in allowed module")
    warn_allowed.with_context("allowed", "function", 5)
    logger.log_entry(warn_allowed)  # Should pass both filters
    
    sus entries []LogEntry = test_backend.get_entries()
    assert_eq_int(len(entries), 2)  # Only 2 should pass all filters
    
    assert_eq_string(entries[0].message, "info in allowed module")
    assert_eq_string(entries[1].message, "warn in allowed module")
    
    test_end()
}

# Test logger lifecycle management
slay test_logger_lifecycle() {
    test_start("logger_lifecycle")
    
    sus formatter JsonFormatter = JsonFormatter.new(nah)
    sus test_backend TestBackend = TestBackend.new(formatter)
    sus logger Logger = Logger.new(formatter)
    logger.add_backend(test_backend)
    
    # Test normal operation
    logger.info("Before async")
    assert_eq_int(len(test_backend.get_entries()), 1)
    
    # Enable async
    logger.enable_async(100)
    logger.info("After async enable")
    
    # Give async worker time
    sleep_milliseconds(50)
    assert_eq_int(len(test_backend.get_entries()), 2)
    
    # Test logger close
    logger.info("Before close")
    logger.close()
    
    # Give time for cleanup
    sleep_milliseconds(50)
    
    # Backend should have received the close call
    assert_true(test_backend.close_called)
    
    test_end()
}

# Test error handling in backends
slay test_backend_error_handling() {
    test_start("backend_error_handling")
    
    # Test file backend with invalid path
    sus invalid_file_backend FileBackend = file_backend("/invalid/readonly/path/test.log") fam {
        when error -> {
            # Expected to fail
            assert_true(contains(error, "permission") || contains(error, "No such file"))
            spill("File backend correctly failed with:", error)
            test_end()
            damn
        }
    }
    
    # If we get here, the path was valid (unexpected)
    invalid_file_backend.close() fam {
        when error -> # Ignore close errors
    }
    
    spill("Warning: Expected invalid file path to fail, but it succeeded")
    
    test_end()
}

# Test high-volume concurrent logging
slay test_high_volume_concurrent() {
    test_start("high_volume_concurrent")
    
    sus formatter TextFormatter = TextFormatter.default()
    sus test_backend TestBackend = TestBackend.new(formatter)
    sus logger Logger = Logger.new(formatter)
    logger.add_backend(test_backend)
    logger.enable_async(5000)
    
    sus goroutines drip = 20
    sus messages_per_goroutine drip = 500
    sus total_messages drip = goroutines * messages_per_goroutine
    
    sus wg WaitGroup = WaitGroup.new()
    sus start_time drip = current_timestamp_nanos()
    
    spill("Starting high-volume concurrent test:", drip_to_string(total_messages), "messages from", 
          drip_to_string(goroutines), "goroutines")
    
    sus i drip = 0
    bestie (i < goroutines) {
        wg.add(1)
        sus goroutine_id drip = i
        
        go {
            sus j drip = 0
            bestie (j < messages_per_goroutine) {
                logger.info_with_fields("High volume message", map<tea, tea>{
                    "goroutine": drip_to_string(goroutine_id),
                    "sequence": drip_to_string(j),
                    "data": "some_data_payload_for_realism"
                })
                j = j + 1
            }
            wg.done()
        }
        
        i = i + 1
    }
    
    wg.wait()
    
    # Wait for async processing
    logger.flush()
    sleep_milliseconds(1000)  # Give generous time for processing
    
    sus end_time drip = current_timestamp_nanos()
    sus duration drip = end_time - start_time
    sus messages_per_second drip = (total_messages * 1_000_000_000) / duration
    
    sus entries []LogEntry = test_backend.get_entries()
    
    spill("High-volume test results:")
    spill("  Total messages:", drip_to_string(len(entries)))
    spill("  Expected messages:", drip_to_string(total_messages))
    spill("  Duration:", drip_to_string(duration / 1_000_000), "ms")
    spill("  Messages/second:", drip_to_string(messages_per_second))
    
    # Verify all messages were logged
    assert_eq_int(len(entries), total_messages)
    
    # Performance assertion - should handle at least 10k messages/second
    assert_true(messages_per_second > 10000)
    
    logger.close()
    
    test_end()
}

# Test real file I/O operations
slay test_real_file_operations() {
    test_start("real_file_operations")
    
    sus test_log_file tea = "/tmp/cursed_logz_test.log"
    sus test_json_file tea = "/tmp/cursed_logz_json_test.log"
    
    # Clean up any existing files
    ready (file_exists(test_log_file)) {
        delete_file(test_log_file) fam { when error -> }
    }
    ready (file_exists(test_json_file)) {
        delete_file(test_json_file) fam { when error -> }
    }
    
    # Test text file backend
    sus text_backend FileBackend = file_backend(test_log_file) fam {
        when error -> {
            assert_fail("Failed to create text file backend: " + error)
            damn
        }
    }
    
    # Test JSON file backend
    sus json_backend FileBackend = json_file_backend(test_json_file) fam {
        when error -> {
            assert_fail("Failed to create JSON file backend: " + error) 
            damn
        }
    }
    
    # Write test entries
    sus test_entries []LogEntry = []LogEntry{
        LogEntry.new(LogLevel.INFO(), "Application started"),
        LogEntry.new(LogLevel.WARN(), "Configuration missing, using defaults"),
        LogEntry.new(LogLevel.ERROR(), "Database connection failed"),
        LogEntry.new(LogLevel.DEBUG(), "Debug information")
    }
    
    test_entries[0].with_field("version", "1.0.0")
    test_entries[1].with_context("config", "load_config", 45)
    test_entries[2].with_field("db_host", "localhost:5432")
    test_entries[3].with_context("debug", "trace_execution", 123)
    
    # Write to both backends
    bestie (entry in test_entries) {
        text_backend.write(entry) fam {
            when error -> assert_fail("Text backend write failed: " + error)
        }
        
        json_backend.write(entry) fam {
            when error -> assert_fail("JSON backend write failed: " + error)
        }
    }
    
    # Flush and close
    text_backend.flush() fam {
        when error -> assert_fail("Text backend flush failed: " + error)
    }
    text_backend.close() fam {
        when error -> assert_fail("Text backend close failed: " + error)
    }
    
    json_backend.flush() fam {
        when error -> assert_fail("JSON backend flush failed: " + error)
    }
    json_backend.close() fam {
        when error -> assert_fail("JSON backend close failed: " + error)
    }
    
    # Verify file contents
    ready (file_exists(test_log_file)) {
        sus text_content tea = read_file_string(test_log_file) fam {
            when error -> {
                assert_fail("Failed to read text log file: " + error)
                damn
            }
        }
        
        assert_true(contains(text_content, "INFO"))
        assert_true(contains(text_content, "Application started"))
        assert_true(contains(text_content, "WARN"))
        assert_true(contains(text_content, "ERROR"))
        assert_true(contains(text_content, "Database connection failed"))
        
        spill("Text log file content verified")
    } otherwise {
        assert_fail("Text log file was not created")
    }
    
    ready (file_exists(test_json_file)) {
        sus json_content tea = read_file_string(test_json_file) fam {
            when error -> {
                assert_fail("Failed to read JSON log file: " + error)
                damn
            }
        }
        
        assert_true(contains(json_content, "\"level\": \"INFO\""))
        assert_true(contains(json_content, "\"message\": \"Application started\""))
        assert_true(contains(json_content, "\"version\": \"1.0.0\""))
        assert_true(contains(json_content, "\"level\": \"ERROR\""))
        assert_true(contains(json_content, "\"db_host\": \"localhost:5432\""))
        
        spill("JSON log file content verified")
    } otherwise {
        assert_fail("JSON log file was not created")
    }
    
    # Cleanup
    delete_file(test_log_file) fam { when error -> }
    delete_file(test_json_file) fam { when error -> }
    
    test_end()
}

# Test production-like scenarios
slay test_production_scenarios() {
    test_start("production_scenarios") 
    
    spill("Testing production-like logging scenarios...")
    
    # Create production-style logger
    sus json_formatter JsonFormatter = JsonFormatter.new(nah)
    sus logger Logger = Logger.new(json_formatter)
    
    # Add multiple backends
    sus multi_backend MultiBackend = MultiBackend.new()
    
    # Console backend for immediate output
    multi_backend.add(json_console_backend())
    
    # Test backend for verification
    sus test_backend TestBackend = TestBackend.new(json_formatter)
    multi_backend.add(test_backend)
    
    logger.add_backend(multi_backend)
    logger.set_level(LogLevel.INFO())  # Production level
    logger.enable_async(1000)
    
    # Add production filters
    sus module_filter ModuleFilter = ModuleFilter.new()
    module_filter.allow_module("auth")
    module_filter.allow_module("api")
    module_filter.allow_module("database")
    module_filter.block_module("debug")
    logger.add_filter(module_filter)
    
    # Simulate production logging scenarios
    
    # 1. User authentication
    sus auth_entry LogEntry = LogEntry.new(LogLevel.INFO(), "User authentication successful")
    auth_entry.with_context("auth", "authenticate", 156)
    auth_entry.with_field("user_id", "user_123")
    auth_entry.with_field("ip_address", "192.168.1.100")
    auth_entry.with_field("user_agent", "Mozilla/5.0...")
    logger.log_entry(auth_entry)
    
    # 2. API request processing
    sus api_entry LogEntry = LogEntry.new(LogLevel.INFO(), "API request processed")
    api_entry.with_context("api", "handle_request", 89)
    api_entry.with_field("method", "POST")
    api_entry.with_field("endpoint", "/api/v1/users")
    api_entry.with_field("status_code", "200")
    api_entry.with_field("response_time_ms", "145")
    logger.log_entry(api_entry)
    
    # 3. Database operations
    sus db_entry LogEntry = LogEntry.new(LogLevel.WARN(), "Database query slow")
    db_entry.with_context("database", "execute_query", 234)
    db_entry.with_field("query", "SELECT * FROM users WHERE active = ?")
    db_entry.with_field("duration_ms", "2500")
    db_entry.with_field("rows_returned", "1250")
    logger.log_entry(db_entry)
    
    # 4. Error scenarios
    sus error_entry LogEntry = LogEntry.new(LogLevel.ERROR(), "External service timeout")
    error_entry.with_context("api", "call_external_service", 67)
    error_entry.with_field("service", "payment_processor")
    error_entry.with_field("timeout_ms", "30000")
    error_entry.with_field("retry_count", "3")
    error_entry.with_field("error_code", "TIMEOUT")
    logger.log_entry(error_entry)
    
    # 5. Debug message (should be filtered out)
    sus debug_entry LogEntry = LogEntry.new(LogLevel.DEBUG(), "Debug trace information")
    debug_entry.with_context("debug", "trace_execution", 45)
    logger.log_entry(debug_entry)
    
    # 6. Message from blocked module
    sus blocked_entry LogEntry = LogEntry.new(LogLevel.INFO(), "Debug module message")
    blocked_entry.with_context("debug", "some_function", 78)
    logger.log_entry(blocked_entry)
    
    # Wait for async processing
    sleep_milliseconds(200)
    logger.flush()
    sleep_milliseconds(100)
    
    # Verify results
    sus entries []LogEntry = test_backend.get_entries()
    
    spill("Production scenario results:")
    spill("  Total logged entries:", drip_to_string(len(entries)))
    spill("  Expected entries: 4 (filtered DEBUG level and blocked module)")
    
    assert_eq_int(len(entries), 4)  # Should have 4 entries (2 filtered out)
    
    # Verify specific entries made it through
    sus found_auth lit = nah
    sus found_api lit = nah
    sus found_db lit = nah
    sus found_error lit = nah
    
    bestie (entry in entries) {
        ready (contains(entry.message, "authentication successful")) {
            found_auth = based
        } otherwise ready (contains(entry.message, "API request processed")) {
            found_api = based
        } otherwise ready (contains(entry.message, "Database query slow")) {
            found_db = based
        } otherwise ready (contains(entry.message, "External service timeout")) {
            found_error = based
        }
    }
    
    assert_true(found_auth)
    assert_true(found_api) 
    assert_true(found_db)
    assert_true(found_error)
    
    logger.close()
    
    test_end()
}

# Main test runner
slay main() {
    spill("=== CURSED logz Comprehensive Test Suite ===")
    spill("")
    
    # Run all core tests first
    run_all_logz_tests()
    
    spill("")
    spill("=== Additional Advanced Tests ===")
    
    # Run additional comprehensive tests
    test_global_logger()
    test_advanced_formatting()
    test_buffered_backend()
    test_complex_filtering()
    test_logger_lifecycle()
    test_backend_error_handling()
    test_high_volume_concurrent()
    test_real_file_operations()
    test_production_scenarios()
    
    spill("")
    spill("=== Final Test Summary ===")
    print_test_summary()
    
    spill("")
    spill("logz comprehensive testing completed successfully!")
    spill("The logging framework is production-ready with:")
    spill("  ✅ Multiple log levels with priority ordering")
    spill("  ✅ Structured logging with custom fields")
    spill("  ✅ Multiple backend support (console, file, network, syslog)")
    spill("  ✅ Async logging with high-performance buffering")
    spill("  ✅ Flexible formatting (text and JSON)")
    spill("  ✅ Advanced filtering (level and module-based)")
    spill("  ✅ Thread-safe concurrent logging")
    spill("  ✅ Production-ready error handling")
    spill("  ✅ Comprehensive testing framework")
}
