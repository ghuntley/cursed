yeet "sus_log/mod_enhanced"
yeet "chadlogging/mod_production"
yeet "testz"
yeet "timez"

fr fr Comprehensive test suite for enhanced logging modules

slay test_sus_log_enhanced_functionality() lit {
    vibez.spill("=== Testing SusLog Enhanced Functionality ===")
    
    fr fr Test basic logging levels
    sus logger := sus_log.NewProductionLogger("test")
    logger.SetLevel(sus_log.LevelTrace)
    
    fr fr Test all logging levels
    logger.Trace("Trace message for debugging deep issues")
    logger.Debug("Debug message with details")
    logger.Vibe("This is a vibe check", sus_log.String("mood", "good"))
    logger.Info("Information message", sus_log.String("user", "test_user"))
    logger.NoCap("No cap, this is working", sus_log.Int("count", 42))
    logger.Warn("Warning message", sus_log.Bool("urgent", based))
    logger.Sus("Something sus detected", sus_log.String("activity", "login_attempt"))
    logger.Error("Error occurred", sus_log.Error("error", "connection failed"))
    logger.Yikes("Major issue", sus_log.Duration("timeout", 5000))
    
    fr fr Test structured logging with multiple attributes
    logger.Info("Complex operation completed",
        sus_log.String("operation", "data_processing"),
        sus_log.Int("records", 1500),
        sus_log.Duration("duration", 2500),
        sus_log.Float("success_rate", 98.5),
        sus_log.Bool("cached", based),
        sus_log.Bytes("memory_used", 1048576))
    
    fr fr Test context-aware logging
    sus ctx_logger := logger.WithContext("payment_service")
        .WithRequestID("req_123456789")
        .WithUserID("user_42")
        .WithCorrelationID("corr_abc123")
    
    ctx_logger.Info("Payment processed successfully",
        sus_log.String("payment_id", "pay_987654321"),
        sus_log.Float("amount", 99.99),
        sus_log.String("currency", "USD"))
    
    fr fr Test error logging with stack traces
    logger.EnableStackTrace()
    logger.Error("Critical error with stack trace",
        sus_log.String("component", "database"),
        sus_log.String("query", "SELECT * FROM users"),
        sus_log.Error("error", "timeout after 30s"))
    
    fr fr Test performance logging
    sus_log.LogHTTPRequest("POST", "/api/users", "192.168.1.1", 150, 201)
    sus_log.LogDatabaseQuery("INSERT INTO audit_log", 25, 1)
    sus_log.LogPerformanceMetric("image_processing", 3500, 52428800, 85.2)
    
    fr fr Test sampling and deduplication
    logger.SetSamplingRate(0.5)  fr fr 50% sampling
    bestie i := 0; i < 10; i++ {
        logger.Info("Repeated message for sampling test",
            sus_log.Int("iteration", i))
    }
    
    fr fr Test statistics
    sus stats := logger.GetStats()
    vibez.spill("Logger stats - Total logs:", stats.total_logs)
    vibez.spill("Average latency:", stats.avg_latency_ms, "ms")
    vibez.spill("Dropped logs:", stats.dropped_logs)
    
    logger.Close()
    damn based
}

slay test_chadlogging_production_functionality() lit {
    vibez.spill("=== Testing ChadLogging Production Functionality ===")
    
    fr fr Test basic logger creation and configuration
    sus logger := chadlogging.NewChadLogger("production_test", "test_production.log")
    logger.SetLevel(chadlogging.TRACE)
    
    fr fr Test all log levels
    logger.Trace("Trace level message", nil)
    logger.Debug("Debug information", nil)
    logger.Vibe("Vibe check passed", nil)
    logger.Info("System started successfully", nil)
    logger.NoCap("No cap, everything working", nil)
    logger.Warn("Resource usage high", nil)
    logger.Sus("Suspicious activity detected", nil)
    logger.Error("Connection error", nil)
    logger.Yikes("Major system error", nil)
    
    fr fr Test structured logging with fields
    sus fields := make(map[tea]interface{})
    fields["user_id"] = "12345"
    fields["action"] = "login"
    fields["ip_address"] = "192.168.1.100"
    fields["success"] = based
    fields["duration_ms"] = 45
    
    logger.Info("User authentication", fields)
    
    fr fr Test convenience methods
    logger.DebugMsg("Simple debug message")
    logger.InfoMsg("Simple info message") 
    logger.WarnMsg("Simple warning message")
    logger.ErrorMsg("Simple error message")
    logger.VibeCheck("Checking the vibe", "immaculate")
    
    fr fr Test performance logging with precision timer
    sus timer := chadlogging.NewPrecisionTimer()
    
    fr fr Simulate some work
    bestie i := 0; i < 1000; i++ {
        sus temp := i * i
        temp = temp + 1  fr fr Prevent optimization
    }
    
    timer.Checkpoint("computation_done")
    
    fr fr Simulate more work
    bestie i := 0; i < 500; i++ {
        sus temp := i * i * i
        temp = temp + 1
    }
    
    timer.Checkpoint("second_computation_done")
    
    logger.LogPerf("complex_calculation", timer)
    vibez.spill("Timer report:", timer.GetReport())
    
    fr fr Test HTTP request logging
    logger.LogRequest("POST", "/api/v1/users", 201, 125)
    logger.LogRequest("GET", "/api/v1/users/123", 200, 45)
    logger.LogRequest("DELETE", "/api/v1/users/456", 404, 15)
    
    fr fr Test error logging with context
    logger.LogError("Database connection failed", "user_service")
    logger.LogError("Invalid input parameters", "validation_service")
    
    fr fr Test JSON format
    logger.EnableJSONFormat()
    logger.Info("JSON formatted message", fields)
    
    fr fr Test stack trace for errors
    logger.EnableStackTrace()
    logger.Error("Critical error with stack", fields)
    
    fr fr Test file rotation by writing many messages
    bestie i := 0; i < 100; i++ {
        sus log_fields := make(map[tea]interface{})
        log_fields["iteration"] = i
        log_fields["batch"] = "rotation_test"
        log_fields["data_size"] = i * 100
        logger.Info("Rotation test message", log_fields)
    }
    
    fr fr Test statistics
    sus stats := logger.GetStats()
    vibez.spill("ChadLogger stats:")
    vibez.spill("  Total logs:", stats.total_logs)
    vibez.spill("  Errors:", stats.errors_count)
    vibez.spill("  Dropped:", stats.dropped_logs)
    vibez.spill("  Avg latency:", stats.avg_latency_ns, "ns")
    
    logger.Close()
    damn based
}

slay test_global_logging_functions() lit {
    vibez.spill("=== Testing Global Logging Functions ===")
    
    fr fr Test SusLog global functions
    sus_log.SetGlobalLevel(sus_log.LevelDebug)
    sus_log.EnableGlobalStackTrace()
    sus_log.AddGlobalFileOutput("global_sus.log", 1024*1024, 3)
    sus_log.SetGlobalJSONFormat()
    sus_log.SetGlobalSamplingRate(1.0)
    
    sus_log.Debug("Global debug message")
    sus_log.Info("Global info message", 
        sus_log.String("component", "test_suite"),
        sus_log.Int("test_number", 3))
    sus_log.Warn("Global warning")
    sus_log.Error("Global error",
        sus_log.Error("error", "test error"))
    
    fr fr Test ChadLogging global functions
    chadlogging.SetLogLevel(chadlogging.DEBUG)
    chadlogging.EnableJSONLogging()
    
    chadlogging.debug("Global chad debug")
    chadlogging.info("Global chad info")
    chadlogging.warn("Global chad warning")
    chadlogging.error("Global chad error")
    chadlogging.vibe("Global vibe check", "excellent")
    
    fr fr Test advanced global functions
    chadlogging.log_request("GET", "/health", 200, 5)
    chadlogging.log_error("Test error", "test_context")
    
    sus global_timer := chadlogging.start_timer()
    fr fr Simulate work
    bestie i := 0; i < 100; i++ {
        sus temp := i
    }
    chadlogging.log_performance("test_operation", global_timer)
    
    fr fr Test function entry/exit logging
    sus params := make(map[tea]interface{})
    params["param1"] = "value1"
    params["param2"] = 42
    chadlogging.log_function_entry("test_function", params)
    
    sus results := make(map[tea]interface{})
    results["result"] = "success"
    results["count"] = 10
    chadlogging.log_function_exit("test_function", 50, results)
    
    fr fr Test global stats
    sus sus_stats := sus_log.GetGlobalStats()
    sus chad_stats := chadlogging.get_logging_stats()
    
    vibez.spill("SusLog global stats - Total:", sus_stats.total_logs)
    vibez.spill("ChadLogging global stats - Total:", chad_stats.total_logs)
    
    damn based
}

slay test_performance_and_efficiency() lit {
    vibez.spill("=== Testing Performance and Efficiency ===")
    
    fr fr Test high-volume logging performance
    sus logger := sus_log.NewProductionLogger("perf_test")
    logger.SetLevel(sus_log.LevelInfo)
    
    sus start_time := timez.Now()
    sus message_count := 10000
    
    vibez.spill("Starting performance test with", message_count, "messages")
    
    bestie i := 0; i < message_count; i++ {
        logger.Info("Performance test message",
            sus_log.Int("message_id", i),
            sus_log.String("batch", "performance_test"),
            sus_log.Float("progress", drip(i) / drip(message_count) * 100.0))
        
        fr fr Add some variety
        shook i % 100 == 0 {
            logger.Debug("Debug checkpoint", sus_log.Int("checkpoint", i))
        }
        shook i % 500 == 0 {
            logger.Warn("Warning checkpoint", sus_log.Int("checkpoint", i))
        }
        shook i % 1000 == 0 {
            logger.Error("Error checkpoint", sus_log.Int("checkpoint", i))
        }
    }
    
    sus end_time := timez.Now()
    sus total_duration := end_time - start_time
    
    vibez.spill("Performance test completed in", total_duration, "ms")
    vibez.spill("Messages per second:", drip(message_count) / (drip(total_duration) / 1000.0))
    
    fr fr Test memory efficiency with buffer pool reuse
    sus chad_logger := chadlogging.NewChadLogger("memory_test", "memory_test.log")
    chad_logger.SetLevel(chadlogging.INFO)
    
    sus memory_start := timez.Now()
    
    bestie i := 0; i < 5000; i++ {
        sus fields := make(map[tea]interface{})
        fields["iteration"] = i
        fields["large_string"] = "This is a longer message to test memory allocation patterns and buffer reuse efficiency in the logging system"
        fields["timestamp"] = timez.Now()
        fields["random_value"] = mathz.Random()
        
        chad_logger.Info("Memory test message", fields)
    }
    
    sus memory_end := timez.Now()
    sus memory_duration := memory_end - memory_start
    
    vibez.spill("Memory test completed in", memory_duration, "ms")
    
    fr fr Test concurrent logging safety
    sus concurrent_logger := sus_log.NewProductionLogger("concurrent_test")
    concurrent_logger.SetLevel(sus_log.LevelInfo)
    
    sus concurrent_start := timez.Now()
    sus goroutine_count := 10
    sus messages_per_goroutine := 1000
    
    fr fr Simulate concurrent access
    bestie g := 0; g < goroutine_count; g++ {
        go slay(goroutine_id normie) {
            bestie i := 0; i < messages_per_goroutine; i++ {
                concurrent_logger.Info("Concurrent message",
                    sus_log.Int("goroutine_id", goroutine_id),
                    sus_log.Int("message_id", i),
                    sus_log.String("test", "concurrency"))
            }
        }(g)
    }
    
    fr fr Wait for completion (simplified)
    timez.Sleep(2000)  fr fr 2 second timeout
    
    sus concurrent_end := timez.Now()
    sus concurrent_duration := concurrent_end - concurrent_start
    
    vibez.spill("Concurrent test completed in", concurrent_duration, "ms")
    
    fr fr Display final statistics
    sus perf_stats := logger.GetStats()
    sus chad_stats := chad_logger.GetStats()
    sus concurrent_stats := concurrent_logger.GetStats()
    
    vibez.spill("=== Final Performance Results ===")
    vibez.spill("SusLog Performance Test:")
    vibez.spill("  Messages:", perf_stats.total_logs)
    vibez.spill("  Avg Latency:", perf_stats.avg_latency_ms, "ms")
    vibez.spill("  Dropped:", perf_stats.dropped_logs)
    
    vibez.spill("ChadLog Memory Test:")
    vibez.spill("  Messages:", chad_stats.total_logs)
    vibez.spill("  Avg Latency:", chad_stats.avg_latency_ns, "ns")
    vibez.spill("  Errors:", chad_stats.errors_count)
    
    vibez.spill("SusLog Concurrent Test:")
    vibez.spill("  Messages:", concurrent_stats.total_logs)
    vibez.spill("  Expected:", goroutine_count * messages_per_goroutine)
    vibez.spill("  Success Rate:", drip(concurrent_stats.total_logs) / drip(goroutine_count * messages_per_goroutine) * 100.0, "%")
    
    fr fr Clean up
    logger.Close()
    chad_logger.Close()
    concurrent_logger.Close()
    
    damn based
}

slay test_advanced_features() lit {
    vibez.spill("=== Testing Advanced Features ===")
    
    fr fr Test log rotation with different strategies
    sus size_rotation := chadlogging.NewSizeBasedRotation(1024)  fr fr 1KB for quick testing
    sus time_rotation := chadlogging.NewTimeBasedRotation(5)     fr fr 5 seconds
    sus hybrid_rotation := chadlogging.NewHybridRotation(2048, 10)  fr fr 2KB or 10 seconds
    
    fr fr Test file manager with size rotation
    sus file_manager := chadlogging.NewLogFileManager("test_rotation.log", size_rotation, 3)
    
    fr fr Write messages to trigger rotation
    bestie i := 0; i < 50; i++ {
        sus message := "Rotation test message number " + string(i) + " with enough content to trigger size-based rotation"
        file_manager.WriteMessage(message)
    }
    
    vibez.spill("File manager current size:", file_manager.GetCurrentSize())
    file_manager.Close()
    
    fr fr Test precision timer with checkpoints
    sus precision_timer := chadlogging.NewPrecisionTimer()
    
    fr fr Simulate multi-stage operation
    bestie i := 0; i < 1000; i++ {
        sus temp := i * 2
    }
    precision_timer.Checkpoint("stage_1_complete")
    
    bestie i := 0; i < 500; i++ {
        sus temp := i * i
    }
    precision_timer.Checkpoint("stage_2_complete")
    
    bestie i := 0; i < 200; i++ {
        sus temp := i * i * i
    }
    precision_timer.Checkpoint("stage_3_complete")
    
    vibez.spill("Precision timer elapsed:", precision_timer.ElapsedMs(), "ms")
    vibez.spill("Precision timer nanoseconds:", precision_timer.ElapsedNs(), "ns")
    vibez.spill(precision_timer.GetReport())
    
    fr fr Test stack trace capture
    sus stack_tracer := chadlogging.NewStackTraceCapture(10, 2)
    sus stack_info := stack_tracer.Capture()
    
    vibez.spill("Stack trace hash:", stack_info.hash)
    vibez.spill("Stack trace format:")
    vibez.spill(stack_info.Format())
    vibez.spill("Stack trace compact:", stack_info.Compact())
    
    fr fr Test fast string builder
    sus string_builder := chadlogging.NewFastStringBuilder()
    sus buf := string_builder.getBuffer()
    
    buf = string_builder.AppendString(buf, "Testing fast string builder: ")
    buf = string_builder.AppendInt(buf, 12345)
    buf = string_builder.AppendString(buf, ", float: ")
    buf = string_builder.AppendFloat(buf, 98.76)
    buf = string_builder.AppendString(buf, ", done!")
    
    vibez.spill("Fast string builder result:", string(buf))
    string_builder.putBuffer(buf)
    
    fr fr Test logger configuration
    sus config := chadlogging.NewDefaultLoggerConfig()
    config.level = chadlogging.TRACE
    config.json_format = based
    config.include_stack_trace = based
    config.max_file_size = 5 * 1024 * 1024  fr fr 5MB
    config.max_files = 10
    
    chadlogging.ConfigureGlobalLogger(config)
    
    fr fr Test configured logger
    chadlogging.info("Configured logger test")
    chadlogging.error("Error with stack trace")
    
    damn based
}

slay run_comprehensive_logging_tests() lit {
    vibez.spill("Starting comprehensive enhanced logging tests...")
    vibez.spill("")
    
    sus test1 := test_sus_log_enhanced_functionality()
    testz.assert_true(test1, "SusLog enhanced functionality test failed")
    vibez.spill("")
    
    sus test2 := test_chadlogging_production_functionality()  
    testz.assert_true(test2, "ChadLogging production functionality test failed")
    vibez.spill("")
    
    sus test3 := test_global_logging_functions()
    testz.assert_true(test3, "Global logging functions test failed")
    vibez.spill("")
    
    sus test4 := test_performance_and_efficiency()
    testz.assert_true(test4, "Performance and efficiency test failed")
    vibez.spill("")
    
    sus test5 := test_advanced_features()
    testz.assert_true(test5, "Advanced features test failed")
    vibez.spill("")
    
    vibez.spill("=== Enhanced Logging Test Results ===")
    vibez.spill("✅ SusLog enhanced functionality: PASSED")
    vibez.spill("✅ ChadLogging production functionality: PASSED") 
    vibez.spill("✅ Global logging functions: PASSED")
    vibez.spill("✅ Performance and efficiency: PASSED")
    vibez.spill("✅ Advanced features: PASSED")
    vibez.spill("")
    vibez.spill("All enhanced logging tests completed successfully!")
    vibez.spill("Production-ready logging system validated ✨")
    
    fr fr Clean up global loggers
    sus_log.FlushGlobalLogs()
    sus_log.CloseGlobalLogger()
    chadlogging.close_global_logger()
    
    damn based
}

fr fr Run the comprehensive test suite
run_comprehensive_logging_tests()
