yeet "testz"
yeet "sus_log"

# Comprehensive test suite for sus_log module
# Structured logging with suspiciously good performance

test_start("test_log_levels")
# Test log level constants and string conversion
assert_eq_int(LevelDebug, -4)
assert_eq_int(LevelInfo, 0)
assert_eq_int(LevelWarn, 4)
assert_eq_int(LevelError, 8)
assert_eq_int(LevelFatal, 12)
assert_eq_int(LevelVibe, -2)
assert_eq_int(LevelSus, 6)
assert_eq_int(LevelYikes, 10)

assert_eq_string(LevelDebug.String(), "DEBUG")
assert_eq_string(LevelInfo.String(), "INFO")
assert_eq_string(LevelWarn.String(), "WARN")
assert_eq_string(LevelError.String(), "ERROR")
assert_eq_string(LevelFatal.String(), "FATAL")
assert_eq_string(LevelVibe.String(), "VIBE")
assert_eq_string(LevelSus.String(), "SUS")
assert_eq_string(LevelYikes.String(), "YIKES")
print_test_summary()

test_start("test_attr_creation")
# Test attribute creation functions
sus stringAttr := String("key1", "value1")
assert_eq_string(stringAttr.Key, "key1")
assert_eq_string(stringAttr.Value.(tea), "value1")

sus intAttr := Int("key2", 42)
assert_eq_string(intAttr.Key, "key2")
assert_eq_int(intAttr.Value.(normie), 42)

sus boolAttr := Bool("key3", based)
assert_eq_string(boolAttr.Key, "key3")
assert_eq_string(boolAttr.Value.(lit), based)

sus anyAttr := Any("key4", "any_value")
assert_eq_string(anyAttr.Key, "key4")
assert_eq_string(anyAttr.Value.(tea), "any_value")
print_test_summary()

test_start("test_logger_creation")
# Test logger creation and configuration
sus logger := NewSusLogger()
assert_eq_int(logger.GetLevel(), LevelInfo)
assert_eq_int(len(logger.attrs), 0)

sus defaultLogger := NewDefaultSusLogger()
assert_eq_int(defaultLogger.GetLevel(), LevelInfo)

sus genZLogger := NewGenZLogger()
assert_eq_int(genZLogger.GetLevel(), LevelVibe)
print_test_summary()

test_start("test_logger_level_setting")
# Test logger level setting and getting
sus logger := NewSusLogger()
logger.SetLevel(LevelDebug)
assert_eq_int(logger.GetLevel(), LevelDebug)

logger.SetLevel(LevelError)
assert_eq_int(logger.GetLevel(), LevelError)

logger.SetLevel(LevelVibe)
assert_eq_int(logger.GetLevel(), LevelVibe)
print_test_summary()

test_start("test_logger_with_attrs")
# Test logger with attributes
sus logger := NewSusLogger()
sus attr1 := String("service", "web")
sus attr2 := Int("port", 8080)

sus newLogger := logger.With(attr1, attr2)
assert_eq_int(len(newLogger.attrs), 2)
assert_eq_string(newLogger.attrs[0].Key, "service")
assert_eq_string(newLogger.attrs[0].Value.(tea), "web")
assert_eq_string(newLogger.attrs[1].Key, "port")
assert_eq_int(newLogger.attrs[1].Value.(normie), 8080)

# Original logger should be unchanged
assert_eq_int(len(logger.attrs), 0)
print_test_summary()

test_start("test_parse_level")
# Test level parsing
sus level, err := ParseLevel("DEBUG")
assert_eq_int(level, LevelDebug)
assert_eq_string(err, cringe)

level, err = ParseLevel("INFO")
assert_eq_int(level, LevelInfo)
assert_eq_string(err, cringe)

level, err = ParseLevel("WARN")
assert_eq_int(level, LevelWarn)
assert_eq_string(err, cringe)

level, err = ParseLevel("ERROR")
assert_eq_int(level, LevelError)
assert_eq_string(err, cringe)

level, err = ParseLevel("FATAL")
assert_eq_int(level, LevelFatal)
assert_eq_string(err, cringe)

level, err = ParseLevel("UNKNOWN")
assert_eq_int(level, LevelInfo)
assert_eq_string(err, "Unknown level")
print_test_summary()

test_start("test_logging_output")
# Test logging output (basic functionality)
sus logger := NewSusLogger()
logger.SetLevel(LevelDebug)

# Test all log levels
logger.Debug("Debug message")
logger.Info("Info message")
logger.Warn("Warning message")
logger.Error("Error message")
logger.Fatal("Fatal message")
logger.Vibe("Vibe message")
logger.NoCap("NoCap message")
logger.Sus("Sus message")
logger.Yikes("Yikes message")
print_test_summary()

test_start("test_logging_with_attributes")
# Test logging with attributes
sus logger := NewSusLogger()
sus attr1 := String("user", "alice")
sus attr2 := Int("request_id", 12345)

logger.Info("User logged in", attr1, attr2)
logger.Error("Login failed", attr1, String("reason", "invalid_password"))
print_test_summary()

test_start("test_log_level_filtering")
# Test log level filtering
sus logger := NewSusLogger()
logger.SetLevel(LevelWarn)

# These should not log (below threshold)
logger.Debug("Debug - should not appear")
logger.Info("Info - should not appear")
logger.Vibe("Vibe - should not appear")

# These should log (at or above threshold)
logger.Warn("Warning - should appear")
logger.Error("Error - should appear")
logger.Fatal("Fatal - should appear")
logger.Sus("Sus - should appear")
logger.Yikes("Yikes - should appear")
print_test_summary()

test_start("test_chained_logger_attributes")
# Test chained logger with attributes
sus logger := NewSusLogger()
sus baseLogger := logger.With(String("service", "auth"))
sus requestLogger := baseLogger.With(Int("request_id", 123))

requestLogger.Info("Processing request")
requestLogger.Error("Request failed")

# Verify attribute inheritance
assert_eq_int(len(requestLogger.attrs), 2)
assert_eq_string(requestLogger.attrs[0].Key, "service")
assert_eq_string(requestLogger.attrs[1].Key, "request_id")
print_test_summary()

test_start("test_custom_attr_helpers")
# Test custom attribute helper functions
sus moodAttr := Mood("user_mood", "happy")
assert_eq_string(moodAttr.Key, "user_mood")
assert_eq_string(moodAttr.Value.(tea), "happy")

sus bussinAttr := Bussin("performance", "excellent")
assert_eq_string(bussinAttr.Key, "performance")
assert_eq_string(bussinAttr.Value.(tea), "excellent")

sus capAttr := Cap("status", "active")
assert_eq_string(capAttr.Key, "status")
assert_eq_string(capAttr.Value.(tea), "active")
print_test_summary()

test_start("test_logger_level_hierarchy")
# Test logger level hierarchy
sus logger := NewSusLogger()

# Test level comparisons
logger.SetLevel(LevelError)
assert_true(logger.GetLevel() > LevelWarn)
assert_true(logger.GetLevel() > LevelInfo)
assert_true(logger.GetLevel() > LevelDebug)
assert_true(logger.GetLevel() < LevelFatal)

logger.SetLevel(LevelDebug)
assert_true(logger.GetLevel() < LevelInfo)
assert_true(logger.GetLevel() < LevelWarn)
assert_true(logger.GetLevel() < LevelError)
print_test_summary()

test_start("test_multiple_loggers")
# Test multiple independent loggers
sus logger1 := NewSusLogger()
sus logger2 := NewSusLogger()

logger1.SetLevel(LevelDebug)
logger2.SetLevel(LevelError)

# Verify independence
assert_eq_int(logger1.GetLevel(), LevelDebug)
assert_eq_int(logger2.GetLevel(), LevelError)

logger1 = logger1.With(String("logger", "first"))
logger2 = logger2.With(String("logger", "second"))

assert_eq_int(len(logger1.attrs), 1)
assert_eq_int(len(logger2.attrs), 1)
assert_eq_string(logger1.attrs[0].Value.(tea), "first")
assert_eq_string(logger2.attrs[0].Value.(tea), "second")
print_test_summary()

# Integration tests
test_start("integration_tests")
# Test integration with application logging patterns
sus appLogger := NewSusLogger()
appLogger.SetLevel(LevelInfo)

# Simulate HTTP request logging
sus requestLogger := appLogger.With(
    String("method", "GET"),
    String("path", "/api/users"),
    Int("status", 200)
)

requestLogger.Info("Request processed")

# Simulate error logging
sus errorLogger := appLogger.With(
    String("component", "database"),
    String("operation", "connect")
)

errorLogger.Error("Database connection failed")

# Simulate debug logging
sus debugLogger := appLogger.With(String("module", "auth"))
debugLogger.SetLevel(LevelDebug)
debugLogger.Debug("Auth token validated")
print_test_summary()

# Performance benchmarks
test_start("performance_benchmarks")
# Test logging performance
sus logger := NewSusLogger()
logger.SetLevel(LevelInfo)

# Benchmark basic logging
bestie i := 0; i < 1000; i++ {
    logger.Info("Performance test message")
}

# Benchmark logging with attributes
sus attr1 := String("test", "performance")
sus attr2 := Int("iteration", 999)

bestie i := 0; i < 1000; i++ {
    logger.Info("Performance test with attributes", attr1, attr2)
}

# Benchmark logger creation
bestie i := 0; i < 100; i++ {
    sus tempLogger := NewSusLogger()
    tempLogger.SetLevel(LevelWarn)
    tempLogger.Info("This should not log")
}
print_test_summary()

# Edge case testing
test_start("edge_cases")
# Test edge cases and error conditions
sus logger := NewSusLogger()

# Test empty messages
logger.Info("")
logger.Error("")

# Test nil/empty attributes
logger.Info("Test with empty attrs")

# Test level edge cases
logger.SetLevel(LevelFatal)
logger.Info("This should not log")
logger.Fatal("This should log")

# Test unknown level string conversion
sus unknownLevel := Level(999)
assert_eq_string(unknownLevel.String(), "UNKNOWN")

# Test attribute edge cases
sus emptyAttr := String("", "")
assert_eq_string(emptyAttr.Key, "")
assert_eq_string(emptyAttr.Value.(tea), "")

sus zeroAttr := Int("zero", 0)
assert_eq_string(zeroAttr.Key, "zero")
assert_eq_int(zeroAttr.Value.(normie), 0)

sus falseAttr := Bool("false", cap)
assert_eq_string(falseAttr.Key, "false")
assert_eq_string(falseAttr.Value.(lit), cap)

# Test logger with many attributes
sus manyAttrs := logger.With(
    String("attr1", "value1"),
    String("attr2", "value2"),
    String("attr3", "value3"),
    Int("attr4", 4),
    Bool("attr5", based)
)
assert_eq_int(len(manyAttrs.attrs), 5)
print_test_summary()
