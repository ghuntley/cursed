# logz/core.csd - Core Logging Framework Implementation
# Advanced structured logging with multiple levels, backends, and async support

# Log levels with priority ordering
squad LogLevel {
    sus priority drip
    sus name tea
    sus color tea
    
    # Standard log levels
    slay DEBUG() LogLevel { damn LogLevel{ priority: 0, name: "DEBUG", color: "\x1b[36m" } }
    slay INFO() LogLevel { damn LogLevel{ priority: 1, name: "INFO", color: "\x1b[32m" } }
    slay WARN() LogLevel { damn LogLevel{ priority: 2, name: "WARN", color: "\x1b[33m" } }
    slay ERROR() LogLevel { damn LogLevel{ priority: 3, name: "ERROR", color: "\x1b[31m" } }
    slay FATAL() LogLevel { damn LogLevel{ priority: 4, name: "FATAL", color: "\x1b[35m" } }
    
    # Compare log levels
    slay gte(self LogLevel, other LogLevel) lit {
        damn self.priority >= other.priority
    }
}

# Log entry structure
squad LogEntry {
    sus timestamp drip
    sus level LogLevel
    sus message tea
    sus module tea
    sus function tea
    sus line drip
    sus fields map<tea, tea>
    sus thread_id drip
    
    slay new(level LogLevel, message tea) LogEntry {
        yeet "timez"
        damn LogEntry{
            timestamp: current_timestamp(),
            level: level,
            message: message,
            module: "",
            function: "",
            line: 0,
            fields: map<tea, tea>{},
            thread_id: current_thread_id()
        }
    }
    
    # Add structured field
    slay with_field(mut self LogEntry, key tea, value tea) LogEntry {
        self.fields[key] = value
        damn self
    }
    
    # Add context information
    slay with_context(mut self LogEntry, module tea, function tea, line drip) LogEntry {
        self.module = module
        self.function = function  
        self.line = line
        damn self
    }
}

# Log formatting interface
collab LogFormatter {
    slay format(entry LogEntry) tea
}

# JSON formatter implementation
squad JsonFormatter {
    sus pretty lit
    
    slay new(pretty lit) JsonFormatter {
        damn JsonFormatter{ pretty: pretty }
    }
}

give JsonFormatter : LogFormatter {
    slay format(self JsonFormatter, entry LogEntry) tea {
        yeet "jsonz"
        yeet "stringz"
        
        sus obj map<tea, tea> = map<tea, tea>{}
        obj["timestamp"] = format_timestamp(entry.timestamp)
        obj["level"] = entry.level.name
        obj["message"] = entry.message
        obj["module"] = entry.module
        obj["function"] = entry.function
        obj["line"] = drip_to_string(entry.line)
        obj["thread"] = drip_to_string(entry.thread_id)
        
        # Add custom fields
        bestie (field in entry.fields) {
            obj[field.key] = field.value
        }
        
        ready (self.pretty) {
            damn json_pretty(obj)
        } otherwise {
            damn json_compact(obj)
        }
    }
}

# Text formatter with customizable layout
squad TextFormatter {
    sus template tea
    sus use_colors lit
    
    slay new(template tea, use_colors lit) TextFormatter {
        damn TextFormatter{ template: template, use_colors: use_colors }
    }
    
    slay default() TextFormatter {
        damn TextFormatter{
            template: "[{timestamp}] {level} {module}:{function}:{line} - {message}",
            use_colors: based
        }
    }
}

give TextFormatter : LogFormatter {
    slay format(self TextFormatter, entry LogEntry) tea {
        sus output tea = self.template
        
        # Replace template variables
        output = replace_all(output, "{timestamp}", format_timestamp(entry.timestamp))
        output = replace_all(output, "{level}", 
            ready (self.use_colors) {
                damn entry.level.color + entry.level.name + "\x1b[0m"
            } otherwise {
                damn entry.level.name
            })
        output = replace_all(output, "{module}", entry.module)
        output = replace_all(output, "{function}", entry.function)
        output = replace_all(output, "{line}", drip_to_string(entry.line))
        output = replace_all(output, "{message}", entry.message)
        output = replace_all(output, "{thread}", drip_to_string(entry.thread_id))
        
        # Add custom fields if present
        ready (len(entry.fields) > 0) {
            output = output + " | "
            bestie (field in entry.fields) {
                output = output + field.key + "=" + field.value + " "
            }
        }
        
        damn output
    }
}

# Log backend interface
collab LogBackend {
    slay write(entry LogEntry) yikes<tea>
    slay flush() yikes<tea>
    slay close() yikes<tea>
}

# Logger filter interface
collab LogFilter {
    slay should_log(entry LogEntry) lit
}

# Level-based filter
squad LevelFilter {
    sus min_level LogLevel
    
    slay new(min_level LogLevel) LevelFilter {
        damn LevelFilter{ min_level: min_level }
    }
}

give LevelFilter : LogFilter {
    slay should_log(self LevelFilter, entry LogEntry) lit {
        damn entry.level.gte(self.min_level)
    }
}

# Module-based filter
squad ModuleFilter {
    sus allowed_modules []tea
    sus blocked_modules []tea
    
    slay new() ModuleFilter {
        damn ModuleFilter{
            allowed_modules: []tea{},
            blocked_modules: []tea{}
        }
    }
    
    slay allow_module(mut self ModuleFilter, module tea) ModuleFilter {
        append(&self.allowed_modules, module)
        damn self
    }
    
    slay block_module(mut self ModuleFilter, module tea) ModuleFilter {
        append(&self.blocked_modules, module)
        damn self
    }
}

give ModuleFilter : LogFilter {
    slay should_log(self ModuleFilter, entry LogEntry) lit {
        # Check blocked modules first
        bestie (blocked in self.blocked_modules) {
            ready (contains(entry.module, blocked)) {
                damn nah
            }
        }
        
        # If allowed list is empty, allow all (except blocked)
        ready (len(self.allowed_modules) == 0) {
            damn based
        }
        
        # Check allowed modules
        bestie (allowed in self.allowed_modules) {
            ready (contains(entry.module, allowed)) {
                damn based
            }
        }
        
        damn nah
    }
}

# Core logger implementation
squad Logger {
    sus backends []LogBackend
    sus formatter LogFormatter
    sus filters []LogFilter
    sus min_level LogLevel
    sus async_enabled lit
    sus buffer_size drip
    sus log_buffer chan<LogEntry>
    sus worker_active lit
    
    slay new(formatter LogFormatter) Logger {
        sus logger Logger = Logger{
            backends: []LogBackend{},
            formatter: formatter,
            filters: []LogFilter{},
            min_level: LogLevel.DEBUG(),
            async_enabled: nah,
            buffer_size: 1000,
            log_buffer: make_channel<LogEntry>(1000),
            worker_active: nah
        }
        damn logger
    }
    
    # Add backend to logger
    slay add_backend(mut self Logger, backend LogBackend) Logger {
        append(&self.backends, backend)
        damn self
    }
    
    # Add filter to logger
    slay add_filter(mut self Logger, filter LogFilter) Logger {
        append(&self.filters, filter)
        damn self
    }
    
    # Set minimum log level
    slay set_level(mut self Logger, level LogLevel) Logger {
        self.min_level = level
        damn self
    }
    
    # Enable async logging
    slay enable_async(mut self Logger, buffer_size drip) Logger {
        self.async_enabled = based
        self.buffer_size = buffer_size
        self.log_buffer = make_channel<LogEntry>(buffer_size)
        self.start_async_worker()
        damn self
    }
    
    # Start async log processing worker
    slay start_async_worker(mut self Logger) {
        ready (!self.worker_active) {
            self.worker_active = based
            go {
                bestie (self.worker_active) {
                    sus entry LogEntry = <-self.log_buffer fam {
                        when _ -> break
                    }
                    self.write_entry(entry)
                }
            }
        }
    }
    
    # Core logging method
    slay log_entry(mut self Logger, entry LogEntry) {
        # Apply filters
        bestie (filter in self.filters) {
            ready (!filter.should_log(entry)) {
                damn
            }
        }
        
        # Check minimum level
        ready (!entry.level.gte(self.min_level)) {
            damn
        }
        
        ready (self.async_enabled) {
            # Send to async worker
            self.log_buffer <- entry fam {
                when _ -> {
                    # Buffer full, write directly
                    self.write_entry(entry)
                }
            }
        } otherwise {
            # Write directly
            self.write_entry(entry)
        }
    }
    
    # Write entry to all backends
    slay write_entry(self Logger, entry LogEntry) {
        bestie (backend in self.backends) {
            backend.write(entry) fam {
                when error -> {
                    # TODO: Handle backend errors
                }
            }
        }
    }
    
    # Convenience logging methods
    slay debug(mut self Logger, message tea) {
        self.log_entry(LogEntry.new(LogLevel.DEBUG(), message))
    }
    
    slay debug_with_fields(mut self Logger, message tea, fields map<tea, tea>) {
        sus entry LogEntry = LogEntry.new(LogLevel.DEBUG(), message)
        bestie (field in fields) {
            entry.with_field(field.key, field.value)
        }
        self.log_entry(entry)
    }
    
    slay info(mut self Logger, message tea) {
        self.log_entry(LogEntry.new(LogLevel.INFO(), message))
    }
    
    slay info_with_fields(mut self Logger, message tea, fields map<tea, tea>) {
        sus entry LogEntry = LogEntry.new(LogLevel.INFO(), message)
        bestie (field in fields) {
            entry.with_field(field.key, field.value)
        }
        self.log_entry(entry)
    }
    
    slay warn(mut self Logger, message tea) {
        self.log_entry(LogEntry.new(LogLevel.WARN(), message))
    }
    
    slay warn_with_fields(mut self Logger, message tea, fields map<tea, tea>) {
        sus entry LogEntry = LogEntry.new(LogLevel.WARN(), message)
        bestie (field in fields) {
            entry.with_field(field.key, field.value)
        }
        self.log_entry(entry)
    }
    
    slay error(mut self Logger, message tea) {
        self.log_entry(LogEntry.new(LogLevel.ERROR(), message))
    }
    
    slay error_with_fields(mut self Logger, message tea, fields map<tea, tea>) {
        sus entry LogEntry = LogEntry.new(LogLevel.ERROR(), message)
        bestie (field in fields) {
            entry.with_field(field.key, field.value)
        }
        self.log_entry(entry)
    }
    
    slay fatal(mut self Logger, message tea) {
        self.log_entry(LogEntry.new(LogLevel.FATAL(), message))
        self.flush()
        yeet "procesz"
        exit(1)
    }
    
    # Flush all backends
    slay flush(self Logger) {
        bestie (backend in self.backends) {
            backend.flush() fam {
                when error -> {
                    # TODO: Handle flush errors
                }
            }
        }
    }
    
    # Close logger and all backends
    slay close(mut self Logger) {
        ready (self.async_enabled) {
            self.worker_active = nah
            close_channel(self.log_buffer)
        }
        
        bestie (backend in self.backends) {
            backend.close() fam {
                when error -> {
                    # TODO: Handle close errors
                }
            }
        }
    }
}

# Utility functions
slay format_timestamp(timestamp drip) tea {
    yeet "timez"
    damn format_unix_timestamp(timestamp, "2006-01-02T15:04:05.000Z")
}

slay current_timestamp() drip {
    yeet "timez"
    damn unix_timestamp()
}

slay current_thread_id() drip {
    yeet "concurrenz"
    damn goroutine_id()
}

# Global logger instance
sus global_logger Logger = Logger.new(TextFormatter.default())

# Global convenience functions
slay debug(message tea) {
    global_logger.debug(message)
}

slay info(message tea) {
    global_logger.info(message)
}

slay warn(message tea) {
    global_logger.warn(message)
}

slay error(message tea) {
    global_logger.error(message)
}

slay fatal(message tea) {
    global_logger.fatal(message)
}

slay set_global_level(level LogLevel) {
    global_logger.set_level(level)
}

slay add_global_backend(backend LogBackend) {
    global_logger.add_backend(backend)
}

slay flush_global() {
    global_logger.flush()
}
