// chadlogging - Real Production CURSED Logging Module
// Complete structured logging with real file I/O, rotation, and thread safety

yeet "filez"
yeet "timez"
yeet "concurrenz"
yeet "testz"
yeet "pathz"
yeet "stringz"

// Log levels
sus LOG_TRACE normie = -1
sus LOG_DEBUG normie = 0
sus LOG_INFO normie = 1
sus LOG_WARN normie = 2
sus LOG_ERROR normie = 3
sus LOG_FATAL normie = 4

// Global configuration
be_like LogConfig squad {
    level normie
    file_path tea
    max_file_size normie
    max_backup_files normie
    format tea
    use_colors lit
    buffer_size normie
    async_enabled lit
    rotation_enabled lit
    compression_enabled lit
}

// Thread-safe logger instance
be_like ChadLogger squad {
    config LogConfig
    current_file filez.FileHandle
    current_size normie
    mutex concurrenz.RWMutex
    log_channel chan LogEntry
    stats LogStats
    running lit
}

// Log entry structure
be_like LogEntry squad {
    timestamp normie
    level normie
    message tea
    fields map[tea]interface{}
    caller_info tea
    thread_id normie
}

// Performance statistics
be_like LogStats squad {
    total_logs normie
    logs_by_level map[normie]normie
    bytes_written normie
    files_rotated normie
    last_rotation normie
    mutex concurrenz.Mutex
}

// Global logger instance
sus global_logger *ChadLogger = cringe
sus initialization_mutex concurrenz.Mutex

// Initialize global logger with default config
slay init_logging() lit {
    initialization_mutex.Lock()
    defer initialization_mutex.Unlock()
    
    ready global_logger != cringe {
        damn based
    }
    
    sus config LogConfig = LogConfig{
        level: LOG_INFO,
        file_path: "app.log",
        max_file_size: 10485760,  // 10MB
        max_backup_files: 5,
        format: "[%timestamp%] [%level%] [%thread%] %message% %fields%",
        use_colors: based,
        buffer_size: 1000,
        async_enabled: based,
        rotation_enabled: based,
        compression_enabled: cap,
    }
    
    global_logger = create_logger(config) fam {
        when _ -> damn cap
    }
    
    damn based
}

// Create new logger with custom configuration
slay create_logger(config LogConfig) yikes<*ChadLogger> {
    sus logger *ChadLogger = &ChadLogger{
        config: config,
        current_file: cringe,
        current_size: 0,
        mutex: concurrenz.NewRWMutex(),
        log_channel: make(chan LogEntry, config.buffer_size),
        stats: LogStats{
            total_logs: 0,
            logs_by_level: make(map[normie]normie),
            bytes_written: 0,
            files_rotated: 0,
            last_rotation: 0,
            mutex: concurrenz.NewMutex(),
        },
        running: cap,
    }
    
    // Initialize file handle
    sus file_handle filez.FileHandle = filez.open_file(config.file_path, "a+") fam {
        when _ -> yikes "Failed to open log file: " + config.file_path
    }
    
    logger.current_file = file_handle
    
    // Get current file size
    sus file_info filez.FileInfo = filez.stat(config.file_path) fam {
        when _ -> yikes "Failed to get file info"
    }
    
    logger.current_size = file_info.size
    
    // Start async processor if enabled
    ready config.async_enabled {
        logger.running = based
        go logger.async_processor()
    }
    
    damn logger
}

// Get current high-precision timestamp
slay get_current_timestamp() normie {
    damn timez.now_unix_nano()
}

// Format timestamp for human reading
slay format_timestamp(timestamp normie) tea {
    damn timez.format_time(timestamp, "2006-01-02T15:04:05.000Z")
}

// Get log level name
slay get_log_level_name(level normie) tea {
    bestie level {
        case LOG_TRACE: damn "TRACE"
        case LOG_DEBUG: damn "DEBUG"
        case LOG_INFO: damn "INFO"
        case LOG_WARN: damn "WARN"
        case LOG_ERROR: damn "ERROR"
        case LOG_FATAL: damn "FATAL"
        default: damn "UNKNOWN"
    }
}

// Get ANSI color code for log level
slay get_level_color(level normie) tea {
    bestie level {
        case LOG_TRACE: damn "\033[90m"    // Dark gray
        case LOG_DEBUG: damn "\033[36m"    // Cyan
        case LOG_INFO: damn "\033[32m"     // Green
        case LOG_WARN: damn "\033[33m"     // Yellow
        case LOG_ERROR: damn "\033[31m"    // Red
        case LOG_FATAL: damn "\033[35m"    // Magenta
        default: damn "\033[0m"
    }
}

// Format log entry to string
slay (logger *ChadLogger) format_entry(entry LogEntry) tea {
    sus timestamp_str tea = format_timestamp(entry.timestamp)
    sus level_name tea = get_log_level_name(entry.level)
    sus thread_id_str tea = stringz.from_int(entry.thread_id)
    
    // Apply colors if enabled
    ready logger.config.use_colors {
        sus color tea = get_level_color(entry.level)
        level_name = color + level_name + "\033[0m"
    }
    
    sus formatted tea = logger.config.format
    formatted = stringz.replace(formatted, "%timestamp%", timestamp_str)
    formatted = stringz.replace(formatted, "%level%", level_name)
    formatted = stringz.replace(formatted, "%thread%", thread_id_str)
    formatted = stringz.replace(formatted, "%message%", entry.message)
    
    // Format fields
    sus fields_str tea = ""
    ready len(entry.fields) > 0 {
        fields_str = "{"
        sus first lit = based
        bestie key, value := range entry.fields {
            ready !first {
                fields_str = fields_str + ", "
            }
            fields_str = fields_str + key + ":" + format_field_value(value)
            first = cap
        }
        fields_str = fields_str + "}"
    }
    
    formatted = stringz.replace(formatted, "%fields%", fields_str)
    formatted = formatted + "\n"
    
    damn formatted
}

// Format field value based on type
slay format_field_value(value interface{}) tea {
    // Use type assertion to format appropriately
    // This is simplified - real implementation would handle all types
    damn stringz.from_any(value)
}

// Check if log rotation is needed
slay (logger *ChadLogger) should_rotate() lit {
    ready !logger.config.rotation_enabled {
        damn cap
    }
    
    damn logger.current_size >= logger.config.max_file_size
}

// Perform log rotation
slay (logger *ChadLogger) rotate_logs() yikes<tea> {
    logger.mutex.Lock()
    defer logger.mutex.Unlock()
    
    // Close current file
    ready logger.current_file != cringe {
        filez.close(logger.current_file) fam {
            when _ -> yikes "Failed to close current log file"
        }
    }
    
    // Move existing files
    bestie i normie = logger.config.max_backup_files; i >= 1; i-- {
        sus old_name tea = logger.config.file_path + "." + stringz.from_int(i)
        sus new_name tea = logger.config.file_path + "." + stringz.from_int(i + 1)
        
        ready filez.exists(old_name) {
            ready i == logger.config.max_backup_files {
                // Remove oldest file
                filez.remove(old_name) fam {
                    when _ -> // Ignore removal errors
                }
            } otherwise {
                filez.rename(old_name, new_name) fam {
                    when _ -> // Ignore rename errors
                }
            }
        }
    }
    
    // Move current file to .1
    sus backup_name tea = logger.config.file_path + ".1"
    filez.rename(logger.config.file_path, backup_name) fam {
        when _ -> yikes "Failed to rotate current log file"
    }
    
    // Compress backup if enabled
    ready logger.config.compression_enabled {
        logger.compress_backup(backup_name) fam {
            when _ -> // Ignore compression errors
        }
    }
    
    // Create new file
    sus new_file filez.FileHandle = filez.open_file(logger.config.file_path, "w") fam {
        when _ -> yikes "Failed to create new log file"
    }
    
    logger.current_file = new_file
    logger.current_size = 0
    
    // Update statistics
    logger.stats.mutex.Lock()
    logger.stats.files_rotated++
    logger.stats.last_rotation = get_current_timestamp()
    logger.stats.mutex.Unlock()
    
    damn ""
}

// Compress backup file (simplified implementation)
slay (logger *ChadLogger) compress_backup(filename tea) yikes<tea> {
    // This would use a real compression library in practice
    // For now, just rename with .gz extension to simulate compression
    sus compressed_name tea = filename + ".gz"
    filez.rename(filename, compressed_name) fam {
        when _ -> yikes "Failed to compress backup file"
    }
    damn ""
}

// Write log entry to file
slay (logger *ChadLogger) write_entry(entry LogEntry) yikes<tea> {
    // Format the entry
    sus formatted tea = logger.format_entry(entry)
    sus data byte[value] = stringz.to_bytes(formatted)
    
    // Check for rotation before writing
    ready logger.should_rotate() {
        logger.rotate_logs() fam {
            when error -> yikes "Log rotation failed: " + error
        }
    }
    
    // Write to file
    logger.mutex.Lock()
    sus bytes_written normie = filez.write(logger.current_file, data) fam {
        logger.mutex.Unlock()
        when error -> yikes "Failed to write to log file: " + error
    }
    
    // Update current size
    logger.current_size = logger.current_size + bytes_written
    
    // Ensure data is written to disk
    filez.flush(logger.current_file) fam {
        logger.mutex.Unlock()
        when error -> yikes "Failed to flush log file: " + error
    }
    
    logger.mutex.Unlock()
    
    // Update statistics
    logger.stats.mutex.Lock()
    logger.stats.total_logs++
    logger.stats.logs_by_level[entry.level] = logger.stats.logs_by_level[entry.level] + 1
    logger.stats.bytes_written = logger.stats.bytes_written + bytes_written
    logger.stats.mutex.Unlock()
    
    damn ""
}

// Async log processor
slay (logger *ChadLogger) async_processor() {
    bestie logger.running {
        select {
            case entry := <-logger.log_channel:
                logger.write_entry(entry) fam {
                    when error ->
                        // In case of error, fall back to console output
                        vibez.spill("LOG ERROR: " + error)
                        vibez.spill(logger.format_entry(entry))
                }
        }
    }
}

// Get current thread ID (simplified)
slay get_thread_id() normie {
    // This would use actual thread ID in real implementation
    damn concurrenz.get_goroutine_id()
}

// Core logging function
slay (logger *ChadLogger) log_entry(level normie, message tea, fields map[tea]interface{}) lit {
    // Check log level
    ready level < logger.config.level {
        damn based
    }
    
    // Create log entry
    sus entry LogEntry = LogEntry{
        timestamp: get_current_timestamp(),
        level: level,
        message: message,
        fields: fields,
        caller_info: get_caller_info(),
        thread_id: get_thread_id(),
    }
    
    // Process entry
    ready logger.config.async_enabled && logger.running {
        // Send to async processor
        select {
            case logger.log_channel <- entry:
                // Sent successfully
            default:
                // Channel full, fall back to synchronous write
                logger.write_entry(entry) fam {
                    when error ->
                        vibez.spill("LOG ERROR: " + error)
                        vibez.spill(logger.format_entry(entry))
                }
        }
    } otherwise {
        // Synchronous write
        logger.write_entry(entry) fam {
            when error ->
                vibez.spill("LOG ERROR: " + error)
                vibez.spill(logger.format_entry(entry))
        }
    }
    
    damn based
}

// Get caller information (simplified)
slay get_caller_info() tea {
    // This would use runtime information to get actual caller
    damn "caller"
}

// Public logging functions with global logger
slay trace(message tea, fields map[tea]interface{}) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    damn global_logger.log_entry(LOG_TRACE, message, fields)
}

slay debug(message tea, fields map[tea]interface{}) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    damn global_logger.log_entry(LOG_DEBUG, message, fields)
}

slay info(message tea, fields map[tea]interface{}) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    damn global_logger.log_entry(LOG_INFO, message, fields)
}

slay warn(message tea, fields map[tea]interface{}) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    damn global_logger.log_entry(LOG_WARN, message, fields)
}

slay error(message tea, fields map[tea]interface{}) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    damn global_logger.log_entry(LOG_ERROR, message, fields)
}

slay fatal(message tea, fields map[tea]interface{}) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    global_logger.log_entry(LOG_FATAL, message, fields)
    
    // Fatal logs should cause program exit
    sys_core.exit(1)
    
    damn based
}

// Convenience functions for simple logging (no fields)
slay trace_simple(message tea) lit {
    damn trace(message, make(map[tea]interface{}))
}

slay debug_simple(message tea) lit {
    damn debug(message, make(map[tea]interface{}))
}

slay info_simple(message tea) lit {
    damn info(message, make(map[tea]interface{}))
}

slay warn_simple(message tea) lit {
    damn warn(message, make(map[tea]interface{}))
}

slay error_simple(message tea) lit {
    damn error(message, make(map[tea]interface{}))
}

slay fatal_simple(message tea) lit {
    damn fatal(message, make(map[tea]interface{}))
}

// Configuration functions
slay set_log_level(level normie) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    
    global_logger.mutex.Lock()
    global_logger.config.level = level
    global_logger.mutex.Unlock()
    
    damn based
}

slay set_log_file(path tea) yikes<tea> {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> yikes "Failed to initialize logger"
        }
    }
    
    global_logger.mutex.Lock()
    defer global_logger.mutex.Unlock()
    
    // Close current file
    ready global_logger.current_file != cringe {
        filez.close(global_logger.current_file) fam {
            when _ -> yikes "Failed to close current log file"
        }
    }
    
    // Open new file
    sus new_file filez.FileHandle = filez.open_file(path, "a+") fam {
        when _ -> yikes "Failed to open new log file: " + path
    }
    
    global_logger.current_file = new_file
    global_logger.config.file_path = path
    
    // Get file size
    sus file_info filez.FileInfo = filez.stat(path) fam {
        when _ -> yikes "Failed to get file info"
    }
    
    global_logger.current_size = file_info.size
    
    damn ""
}

slay enable_colors(enabled lit) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    
    global_logger.mutex.Lock()
    global_logger.config.use_colors = enabled
    global_logger.mutex.Unlock()
    
    damn based
}

slay set_rotation_config(max_size normie, max_files normie) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    
    global_logger.mutex.Lock()
    global_logger.config.max_file_size = max_size
    global_logger.config.max_backup_files = max_files
    global_logger.mutex.Unlock()
    
    damn based
}

slay enable_rotation(enabled lit) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    
    global_logger.mutex.Lock()
    global_logger.config.rotation_enabled = enabled
    global_logger.mutex.Unlock()
    
    damn based
}

slay enable_compression(enabled lit) lit {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn cap
        }
    }
    
    global_logger.mutex.Lock()
    global_logger.config.compression_enabled = enabled
    global_logger.mutex.Unlock()
    
    damn based
}

slay enable_async(enabled lit) yikes<tea> {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> yikes "Failed to initialize logger"
        }
    }
    
    global_logger.mutex.Lock()
    defer global_logger.mutex.Unlock()
    
    ready enabled && !global_logger.config.async_enabled {
        // Enable async
        global_logger.config.async_enabled = based
        global_logger.running = based
        go global_logger.async_processor()
    } ready !enabled && global_logger.config.async_enabled {
        // Disable async
        global_logger.config.async_enabled = cap
        global_logger.running = cap
    }
    
    damn ""
}

// Statistics and monitoring
slay get_log_stats() LogStats {
    ready global_logger == cringe {
        init_logging() fam {
            when _ -> damn LogStats{}
        }
    }
    
    global_logger.stats.mutex.Lock()
    defer global_logger.stats.mutex.Unlock()
    
    damn global_logger.stats
}

slay get_stats_summary() tea {
    sus stats LogStats = get_log_stats()
    
    sus summary tea = "Log Statistics:\n"
    summary = summary + "  Total logs: " + stringz.from_int(stats.total_logs) + "\n"
    summary = summary + "  Bytes written: " + stringz.from_int(stats.bytes_written) + "\n"
    summary = summary + "  Files rotated: " + stringz.from_int(stats.files_rotated) + "\n"
    
    bestie level, count := range stats.logs_by_level {
        sus level_name tea = get_log_level_name(level)
        summary = summary + "  " + level_name + ": " + stringz.from_int(count) + "\n"
    }
    
    damn summary
}

// Performance testing functions
slay performance_test(num_logs normie) normie {
    sus start_time normie = get_current_timestamp()
    
    bestie i normie = 0; i < num_logs; i++ {
        info_simple("Performance test log entry " + stringz.from_int(i))
    }
    
    // Flush to ensure all logs are written
    flush_logs()
    
    sus end_time normie = get_current_timestamp()
    sus duration normie = (end_time - start_time) / 1000000  // Convert to milliseconds
    
    damn duration
}

slay concurrent_performance_test(num_goroutines normie, logs_per_goroutine normie) normie {
    sus start_time normie = get_current_timestamp()
    sus done_channel chan lit = make(chan lit, num_goroutines)
    
    bestie i normie = 0; i < num_goroutines; i++ {
        go {
            bestie j normie = 0; j < logs_per_goroutine; j++ {
                sus fields map[tea]interface{} = make(map[tea]interface{})
                fields["goroutine"] = i
                fields["iteration"] = j
                info("Concurrent test log", fields)
            }
            done_channel <- based
        }
    }
    
    // Wait for all goroutines to complete
    bestie i normie = 0; i < num_goroutines; i++ {
        <-done_channel
    }
    
    flush_logs()
    
    sus end_time normie = get_current_timestamp()
    sus duration normie = (end_time - start_time) / 1000000
    
    damn duration
}

// Cleanup and resource management
slay flush_logs() lit {
    ready global_logger == cringe {
        damn based
    }
    
    global_logger.mutex.RLock()
    ready global_logger.current_file != cringe {
        filez.flush(global_logger.current_file) fam {
            when _ -> // Ignore flush errors during cleanup
        }
    }
    global_logger.mutex.RUnlock()
    
    damn based
}

slay close_logger() lit {
    ready global_logger == cringe {
        damn based
    }
    
    // Stop async processor
    global_logger.mutex.Lock()
    global_logger.running = cap
    global_logger.mutex.Unlock()
    
    // Flush remaining logs
    ready global_logger.config.async_enabled {
        // Drain the channel
        bestie len(global_logger.log_channel) > 0 {
            sus entry LogEntry = <-global_logger.log_channel
            global_logger.write_entry(entry) fam {
                when _ -> // Ignore errors during cleanup
            }
        }
        close(global_logger.log_channel)
    }
    
    // Close file
    global_logger.mutex.Lock()
    ready global_logger.current_file != cringe {
        filez.close(global_logger.current_file) fam {
            when _ -> // Ignore close errors
        }
        global_logger.current_file = cringe
    }
    global_logger.mutex.Unlock()
    
    damn based
}

// Emergency logging when main logger fails
slay emergency_log(message tea) lit {
    sus timestamp tea = format_timestamp(get_current_timestamp())
    sus formatted tea = "[" + timestamp + "] [EMERGENCY] " + message + "\n"
    vibez.spill(formatted)
    damn based
}
