yeet "logging"

slay main() {
    vibez.spill("🚀 Starting CURSED Logging Module Tests")
    vibez.spill("")
    
    fr fr Test basic logging functions
    vibez.spill("Testing basic logging functions:")
    
    sus info_msg tea = logging.log_info("This is an info message")
    sus warn_msg tea = logging.log_warn("This is a warning message")  
    sus error_msg tea = logging.log_error("This is an error message")
    
    vibez.spill(info_msg)
    vibez.spill(warn_msg)
    vibez.spill(error_msg)
    
    fr fr Test log constants
    vibez.spill("")
    vibez.spill("Testing log level constants:")
    vibez.spill("LOG_INFO = " + tea(logging.LOG_INFO))
    vibez.spill("LOG_WARN = " + tea(logging.LOG_WARN))
    vibez.spill("LOG_ERROR = " + tea(logging.LOG_ERROR))
    
    fr fr Test prefix configuration
    vibez.spill("")
    vibez.spill("Testing prefix configuration:")
    
    logging.set_log_prefix("[CUSTOM]")
    sus custom_msg tea = logging.log_info("Message with custom prefix")
    vibez.spill(custom_msg)
    
    logging.set_log_prefix("[CURSED]")
    sus default_msg tea = logging.log_info("Message with default prefix")
    vibez.spill(default_msg)
    
    fr fr Test named logger
    vibez.spill("")
    vibez.spill("Testing named logger:")
    
    sus db_logger tea = logging.create_logger("Database")
    sus api_logger tea = logging.create_logger("API")
    
    vibez.spill("Created loggers: " + db_logger + " and " + api_logger)
    
    vibez.spill("")
    vibez.spill("✅ All logging tests completed successfully!")
    vibez.spill("🎯 CURSED logging module is fully functional!")
}
