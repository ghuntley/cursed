yeet "testz"
yeet "oglogging"

test_start("oglogging basic tests")

fr fr Test basic logging
slay test_basic_logging() lit {
    oglogging.spill("Basic test message")
    oglogging.spillf("Formatted: %s", "test")
    damn based
}

fr fr Test logger creation
slay test_logger_creation() lit {
    sus logger := oglogging.new("output", "PREFIX: ", oglogging.LstdFlags)
    damn based
}

fr fr Test structured logger
slay test_structured_logging() lit {
    sus logger := oglogging.new("output", "", oglogging.LstdFlags)
    sus structured := oglogging.NewStructuredLogger(logger)
    damn based
}

fr fr Test performance logger
slay test_performance_logger() lit {
    sus logger := oglogging.new("output", "", oglogging.LstdFlags)
    sus perfLogger := oglogging.NewPerfLogger(logger)
    damn based
}

fr fr Test utility functions
slay test_utilities() lit {
    sus testLogger := oglogging.createTestLogger()
    damn based
}

fr fr Run all tests
assert_true(test_basic_logging())
assert_true(test_logger_creation())
assert_true(test_structured_logging())
assert_true(test_performance_logger())
assert_true(test_utilities())

print_test_summary()
