// Test logging framework (logz package)
yeet "logz"
yeet "vibez"

vibez.spill("=== Testing Logz Logging Framework ===")

// Test basic logging levels
log_info("This is an info message")
log_warn("This is a warning message")
log_error("This is an error message")
log_debug("This is a debug message")

// Test structured logging
log_info_structured("user_action", map("user_id", 12345, "action", "login", "ip_address", "192.168.1.1"))

// Test logger configuration
sus logger Logger = create_logger("test_app")
logger.set_level(LOG_LEVEL_DEBUG)
logger.set_format(LOG_FORMAT_JSON)

logger.info("Logger configured successfully")
logger.error("Test error with structured data", map("error_code", 500, "module", "auth"))

// Test file logging
sus file_logger Logger = create_file_logger("test.log")
file_logger.info("This message should go to file")

vibez.spill("✅ Basic logging functionality: PASSED")

// Test log rotation
sus rotating_logger Logger = create_rotating_logger("app.log", 1024, 5)  // 1KB max, 5 files
rotating_logger.info("Testing log rotation")

// Test async logging
sus async_logger Logger = create_async_logger("async.log")
async_logger.info("Async log message 1")
async_logger.info("Async log message 2")
flush_async_logger(async_logger)

vibez.spill("✅ Advanced logging features: PASSED")
vibez.spill("=== Logz Testing Complete ===")
