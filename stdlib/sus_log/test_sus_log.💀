yeet "testz"
yeet "sus_log"

test_start("SusLog comprehensive test suite")

fr fr Test Level functionality
slay test_level_functionality() {
    fr fr Test level string conversion
    assert_eq_string(sus_log.LevelDebug.String(), "DEBUG")
    assert_eq_string(sus_log.LevelInfo.String(), "INFO")
    assert_eq_string(sus_log.LevelWarn.String(), "WARN")
    assert_eq_string(sus_log.LevelError.String(), "ERROR")
    assert_eq_string(sus_log.LevelFatal.String(), "FATAL")
    assert_eq_string(sus_log.LevelVibe.String(), "VIBE")
    assert_eq_string(sus_log.LevelSus.String(), "SUS")
    assert_eq_string(sus_log.LevelYikes.String(), "YIKES")
    
    fr fr Test level parsing
    sus level, err := sus_log.ParseLevel("DEBUG")
    assert_eq_int(level, sus_log.LevelDebug)
    assert_eq_string(err, "")
    
    sus level2, err2 := sus_log.ParseLevel("UNKNOWN")
    assert_eq_int(level2, sus_log.LevelInfo)
    assert_eq_string(err2, "Unknown level")
    
    vibez.spill("✅ Level functionality tests passed")
}

fr fr Test Attr functionality
slay test_attr_functionality() {
    fr fr Test string attribute
    sus str_attr := sus_log.String("key", "value")
    assert_eq_string(str_attr.Key, "key")
    
    fr fr Test int attribute
    sus int_attr := sus_log.Int("count", 42)
    assert_eq_string(int_attr.Key, "count")
    
    fr fr Test bool attribute
    sus bool_attr := sus_log.Bool("enabled", based)
    assert_eq_string(bool_attr.Key, "enabled")
    
    fr fr Test any attribute
    sus any_attr := sus_log.Any("data", "test")
    assert_eq_string(any_attr.Key, "data")
    
    fr fr Test mood attribute
    sus mood_attr := sus_log.Mood("feeling", "happy")
    assert_eq_string(mood_attr.Key, "feeling")
    
    fr fr Test bussin attribute
    sus bussin_attr := sus_log.Bussin("vibe", "immaculate")
    assert_eq_string(bussin_attr.Key, "vibe")
    
    fr fr Test cap attribute
    sus cap_attr := sus_log.Cap("status", "active")
    assert_eq_string(cap_attr.Key, "status")
    
    vibez.spill("✅ Attr functionality tests passed")
}

fr fr Test basic SusLogger functionality
slay test_basic_logger() {
    sus logger := sus_log.NewSusLogger()
    
    fr fr Test initial level
    assert_eq_int(logger.GetLevel(), sus_log.LevelInfo)
    
    fr fr Test set level
    logger.SetLevel(sus_log.LevelDebug)
    assert_eq_int(logger.GetLevel(), sus_log.LevelDebug)
    
    fr fr Test logging methods (just ensure they don't crash)
    logger.Debug("Debug message")
    logger.Info("Info message")
    logger.Warn("Warning message")
    logger.Error("Error message")
    logger.Fatal("Fatal message")
    logger.Vibe("Vibe message")
    logger.Sus("Sus message")
    logger.Yikes("Yikes message")
    
    fr fr Test with attributes
    logger.Info("Test with attributes", sus_log.String("key", "value"), sus_log.Int("count", 1))
    
    fr fr Test with chaining
    sus contextLogger := logger.With(sus_log.String("context", "test"))
    contextLogger.Info("Context message")
    
    vibez.spill("✅ Basic logger tests passed")
}

fr fr Test GenZ logger
slay test_genz_logger() {
    sus logger := sus_log.NewGenZLogger()
    
    fr fr Test initial level should be LevelVibe
    assert_eq_int(logger.GetLevel(), sus_log.LevelVibe)
    
    fr fr Test logging methods
    logger.Vibe("That's a vibe")
    logger.NoCap("No cap fr fr")
    logger.Sus("That's sus")
    logger.Yikes("Big yikes")
    
    vibez.spill("✅ GenZ logger tests passed")
}

fr fr Test advanced logger functionality
slay test_advanced_logger() {
    sus logger := sus_log.NewAdvancedLogger()
    
    fr fr Test initial state
    assert_eq_int(logger.GetLevel(), sus_log.LevelInfo)
    
    fr fr Test level changes
    logger.SetLevel(sus_log.LevelDebug)
    assert_eq_int(logger.GetLevel(), sus_log.LevelDebug)
    
    fr fr Test logging methods
    logger.Debug("Debug message")
    logger.Info("Info message")
    logger.Warn("Warning message")
    logger.Error("Error message")
    logger.Fatal("Fatal message")
    
    fr fr Test with attributes
    logger.Info("Test with attributes", sus_log.String("key", "value"))
    
    fr fr Test with context methods
    sus contextLogger := logger.WithContext("test_context")
    contextLogger.Info("Context message")
    
    sus requestLogger := logger.WithRequestID("req-123")
    requestLogger.Info("Request message")
    
    sus userLogger := logger.WithUserID("user-456")
    userLogger.Info("User message")
    
    fr fr Test close and flush
    logger.Flush()
    logger.Close()
    
    vibez.spill("✅ Advanced logger tests passed")
}

fr fr Test file output
slay test_file_output() {
    sus output := sus_log.NewFileOutput("test.log")
    
    fr fr Test write
    output.Write("Test message")
    
    fr fr Test max size settings
    output.SetMaxSize(1024)
    
    fr fr Test rotation
    output.EnableRotation()
    output.DisableRotation()
    
    fr fr Test close and flush
    output.Flush()
    output.Close()
    
    vibez.spill("✅ File output tests passed")
}

fr fr Test console output
slay test_console_output() {
    sus output := sus_log.NewConsoleOutput()
    
    fr fr Test write
    output.Write("Console test message")
    
    fr fr Test color settings
    output.EnableColors()
    output.DisableColors()
    
    fr fr Test close and flush
    output.Flush()
    output.Close()
    
    vibez.spill("✅ Console output tests passed")
}

fr fr Test buffered output
slay test_buffered_output() {
    sus output := sus_log.NewBufferedOutput(10)
    
    fr fr Test initial buffer size
    assert_eq_int(output.GetBufferSize(), 0)
    
    fr fr Test write
    output.Write("Test message 1")
    output.Write("Test message 2")
    
    fr fr Test buffer size
    assert_eq_int(output.GetBufferSize(), 2)
    
    fr fr Test flush
    output.Flush()
    assert_eq_int(output.GetBufferSize(), 0)
    
    fr fr Test close
    output.Close()
    
    vibez.spill("✅ Buffered output tests passed")
}

fr fr Test network output
slay test_network_output() {
    sus output := sus_log.NewNetworkOutput("localhost", 8080, "tcp")
    
    fr fr Test write
    output.Write("Network test message")
    
    fr fr Test close and flush
    output.Flush()
    output.Close()
    
    vibez.spill("✅ Network output tests passed")
}

fr fr Test JSON formatter
slay test_json_formatter() {
    sus formatter := sus_log.NewJSONFormatter()
    
    fr fr Test format
    sus attrs := sus_log[value].Attr{sus_log.String("key", "value")}
    sus result := formatter.Format(sus_log.LevelInfo, "Test message", attrs, 1234567890)
    
    fr fr Should contain JSON structure
    assert_true(result != "")
    assert_true(len(result) > 0)
    
    vibez.spill("✅ JSON formatter tests passed")
}

fr fr Test text formatter
slay test_text_formatter() {
    sus formatter := sus_log.NewTextFormatter()
    
    fr fr Test format
    sus attrs := sus_log[value].Attr{sus_log.String("key", "value")}
    sus result := formatter.Format(sus_log.LevelInfo, "Test message", attrs, 1234567890)
    
    fr fr Should contain formatted text
    assert_true(result != "")
    assert_true(len(result) > 0)
    
    vibez.spill("✅ Text formatter tests passed")
}

fr fr Test level filter
slay test_level_filter() {
    sus filter := sus_log.NewLevelFilter(sus_log.LevelWarn)
    
    fr fr Test should log higher levels
    assert_true(filter.ShouldLog(sus_log.LevelError, "Error message", sus_log[value].Attr{}))
    assert_true(filter.ShouldLog(sus_log.LevelWarn, "Warning message", sus_log[value].Attr{}))
    
    fr fr Test should not log lower levels
    assert_true(!filter.ShouldLog(sus_log.LevelInfo, "Info message", sus_log[value].Attr{}))
    assert_true(!filter.ShouldLog(sus_log.LevelDebug, "Debug message", sus_log[value].Attr{}))
    
    vibez.spill("✅ Level filter tests passed")
}

fr fr Test keyword filter
slay test_keyword_filter() {
    sus keywords := tea[value]{"sensitive", "password"}
    sus filter := sus_log.NewKeywordFilter(keywords, based)
    
    fr fr Test should block sensitive keywords
    assert_true(!filter.ShouldLog(sus_log.LevelInfo, "This contains sensitive data", sus_log[value].Attr{}))
    assert_true(filter.ShouldLog(sus_log.LevelInfo, "This is safe", sus_log[value].Attr{}))
    
    vibez.spill("✅ Keyword filter tests passed")
}

fr fr Test log metrics
slay test_log_metrics() {
    sus metrics := sus_log.NewLogMetrics()
    
    fr fr Test initial state
    assert_eq_int(metrics.GetTotalLogs(), 0)
    assert_eq_int(metrics.GetErrorsCount(), 0)
    assert_eq_int(metrics.GetLogsByLevel(sus_log.LevelInfo), 0)
    
    fr fr Test recording logs
    metrics.RecordLog(sus_log.LevelInfo)
    metrics.RecordLog(sus_log.LevelError)
    metrics.RecordLog(sus_log.LevelInfo)
    
    fr fr Test metrics
    assert_eq_int(metrics.GetTotalLogs(), 3)
    assert_eq_int(metrics.GetErrorsCount(), 1)
    assert_eq_int(metrics.GetLogsByLevel(sus_log.LevelInfo), 2)
    assert_eq_int(metrics.GetLogsByLevel(sus_log.LevelError), 1)
    
    fr fr Test reset
    metrics.Reset()
    assert_eq_int(metrics.GetTotalLogs(), 0)
    assert_eq_int(metrics.GetErrorsCount(), 0)
    
    vibez.spill("✅ Log metrics tests passed")
}

fr fr Test global logging functions
slay test_global_logging() {
    fr fr Test level settings
    sus_log.SetGlobalLevel(sus_log.LevelDebug)
    
    fr fr Test global logging methods
    sus_log.LogDebug("Global debug message")
    sus_log.LogInfo("Global info message")
    sus_log.LogWarn("Global warning message")
    sus_log.LogError("Global error message")
    sus_log.LogFatal("Global fatal message")
    sus_log.LogVibe("Global vibe message")
    sus_log.LogSus("Global sus message")
    sus_log.LogYikes("Global yikes message")
    
    fr fr Test with attributes
    sus_log.LogInfo("Global message with attributes", sus_log.String("key", "value"))
    
    fr fr Test get global logger
    sus globalLogger := sus_log.GetGlobalLogger()
    assert_true(globalLogger != cringe)
    
    fr fr Test get log metrics
    sus metrics := sus_log.GetLogMetrics()
    assert_true(metrics != cringe)
    
    fr fr Test flush and close
    sus_log.FlushGlobalLogger()
    sus_log.CloseGlobalLogger()
    
    vibez.spill("✅ Global logging tests passed")
}

fr fr Test structured logging helpers
slay test_structured_logging() {
    fr fr Test request logging
    sus_log.LogRequest("GET", "/api/users", "user123", 150)
    
    fr fr Test database query logging
    sus_log.LogDatabaseQuery("SELECT * FROM users", 50, 10)
    
    fr fr Test error logging
    sus_log.LogError("Database connection failed", "startup")
    
    fr fr Test performance logging
    sus_log.LogPerformance("user_search", 200, 1024)
    
    fr fr Test security logging
    sus_log.LogSecurity("failed_login", "user123", "192.168.1.100", "medium")
    
    vibez.spill("✅ Structured logging tests passed")
}

fr fr Test log configuration
slay test_log_configuration() {
    sus config := sus_log.NewDefaultLogConfig()
    
    fr fr Test default values
    assert_eq_int(config.level, sus_log.LevelInfo)
    assert_eq_string(config.output_file, "app.log")
    assert_true(config.rotation_enabled)
    assert_true(config.async_enabled)
    assert_true(!config.json_format)
    assert_true(!config.include_caller)
    
    fr fr Test configure global logger
    sus_log.ConfigureGlobalLogger(config)
    
    vibez.spill("✅ Log configuration tests passed")
}

fr fr Test async logging
slay test_async_logging() {
    sus logger := sus_log.NewAdvancedLogger()
    
    fr fr Test async enable/disable
    logger.EnableAsync()
    logger.DisableAsync()
    
    fr fr Test global async
    sus_log.EnableGlobalAsync()
    sus_log.DisableGlobalAsync()
    
    vibez.spill("✅ Async logging tests passed")
}

fr fr Test formatter and output integration
slay test_formatter_output_integration() {
    sus logger := sus_log.NewAdvancedLogger()
    
    fr fr Test adding multiple outputs
    logger.AddOutput(sus_log.NewConsoleOutput())
    logger.AddOutput(sus_log.NewFileOutput("test.log"))
    logger.AddOutput(sus_log.NewBufferedOutput(10))
    
    fr fr Test setting formatters
    logger.SetFormatter(sus_log.NewJSONFormatter())
    logger.SetFormatter(sus_log.NewTextFormatter())
    
    fr fr Test adding filters
    logger.AddFilter(sus_log.NewLevelFilter(sus_log.LevelWarn))
    logger.AddFilter(sus_log.NewKeywordFilter(tea[value]{"test"}, cap))
    
    fr fr Test logging with all components
    logger.Info("Integration test message", sus_log.String("component", "integration"))
    
    vibez.spill("✅ Formatter and output integration tests passed")
}

fr fr Run all tests
test_level_functionality()
test_attr_functionality()
test_basic_logger()
test_genz_logger()
test_advanced_logger()
test_file_output()
test_console_output()
test_buffered_output()
test_network_output()
test_json_formatter()
test_text_formatter()
test_level_filter()
test_keyword_filter()
test_log_metrics()
test_global_logging()
test_structured_logging()
test_log_configuration()
test_async_logging()
test_formatter_output_integration()

print_test_summary()
vibez.spill("🎉 All SusLog tests completed successfully!")
