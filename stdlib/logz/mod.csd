# logz/mod.csd - Module entry point for logz logging framework
# Comprehensive logging system with multiple backends and structured logging

# Re-export core logging functionality
yeet "logz/core"
yeet "logz/backends"

# Export log levels
export LogLevel
export LogEntry

# Export formatters
export LogFormatter
export TextFormatter
export JsonFormatter

# Export filters
export LogFilter
export LevelFilter
export ModuleFilter

# Export core logger
export Logger

# Export backends
export LogBackend
export ConsoleBackend
export FileBackend
export NetworkBackend
export SyslogBackend
export MultiBackend
export BufferedBackend

# Export factory functions
export console_backend
export json_console_backend
export file_backend
export json_file_backend
export network_backend
export syslog_backend

# Export global logger functions
export debug
export info
export warn  
export error
export fatal
export set_global_level
export add_global_backend
export flush_global

# Export utility functions
export format_timestamp
export current_timestamp
export current_thread_id

# Module metadata
sus MODULE_NAME tea = "logz"
sus MODULE_VERSION tea = "1.0.0"
sus MODULE_DESCRIPTION tea = "Comprehensive logging framework with structured logging, multiple backends, async support, and enterprise-grade features"

export MODULE_NAME
export MODULE_VERSION
export MODULE_DESCRIPTION
