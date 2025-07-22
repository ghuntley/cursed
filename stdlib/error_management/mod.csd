yeet "testz"
yeet "timez"

fr fr CURSED Error Management Module
fr fr Comprehensive error handling and logging with structured types, 
fr fr error wrapping/unwrapping, stack traces, logging levels, and recovery patterns

fr fr Error severity levels
be_like error_severity smol {
    info = 0
    warning = 1  
    error = 2
    critical = 3
    fatal = 4
}

fr fr Error categories for classification
be_like error_category smol {
    memory_yikes = 0
    io_yikes = 1
    network_yikes = 2
    parse_yikes = 3
    type_yikes = 4
    runtime_yikes = 5
    security_yikes = 6
    performance_yikes = 7
}

fr fr Logging levels
be_like log_level smol {
    debug = 0
    info = 1
    warn = 2
    error = 3
    fatal = 4
}

fr fr Structured error type
be_like managed_error squad {
    message tea
    code normie
    details tea
    category error_category
    severity error_severity
    timestamp tea
    stack_trace []tea
    wrapped_error @managed_error
    context yikes.tea
}

fr fr Logger configuration
be_like logger_config squad {
    level log_level
    output_format tea
    include_timestamp lit
    include_stack_trace lit
    include_goroutine_id lit
}

fr fr Circuit breaker state for error recovery
be_like circuit_state smol {
    closed = 0
    open = 1
    half_open = 2
}

fr fr Circuit breaker for error recovery patterns
be_like circuit_breaker squad {
    failure_count normie
    failure_threshold normie
    timeout_seconds normie
    last_failure_time tea
    state circuit_state
    name tea
}

fr fr Global logger instance
sus global_logger logger_config = logger_config{
    level: log_level.info,
    output_format: "json",
    include_timestamp: based,
    include_stack_trace: based,
    include_goroutine_id: based
}

fr fr Error statistics tracking
be_like error_stats squad {
    total_errors normie
    errors_by_category yikes.normie
    errors_by_severity yikes.normie
    error_rate meal
    last_reset_time tea
}

sus global_error_stats error_stats = error_stats{
    total_errors: 0,
    errors_by_category: yikes.normie{},
    errors_by_severity: yikes.normie{},
    error_rate: 0.0,
    last_reset_time: timez.now_rfc3339()
}

fr fr Create a new managed error
slay new_error(message tea, code normie) @managed_error {
    damn @managed_error{
        message: message,
        code: code,
        details: "",
        category: error_category.runtime_yikes,
        severity: error_severity.error,
        timestamp: timez.now_rfc3339(),
        stack_trace: capture_stack_trace(),
        wrapped_error: cringe,
        context: yikes.tea{}
    }
}

fr fr Create error with full context
slay new_error_full(message tea, code normie, category error_category, severity error_severity, details tea) @managed_error {
    damn @managed_error{
        message: message,
        code: code,
        details: details,
        category: category,
        severity: severity,
        timestamp: timez.now_rfc3339(),
        stack_trace: capture_stack_trace(),
        wrapped_error: cringe,
        context: yikes.tea{}
    }
}

fr fr Wrap an existing error with additional context
slay wrap_error(err @managed_error, context tea) @managed_error {
    vibe_check err == cringe {
        damn cringe
    }
    
    damn @managed_error{
        message: context + ": " + err.message,
        code: err.code,
        details: err.details,
        category: err.category,
        severity: err.severity,
        timestamp: timez.now_rfc3339(),
        stack_trace: capture_stack_trace(),
        wrapped_error: err,
        context: err.context
    }
}

fr fr Unwrap error to get root cause
slay unwrap_error(err @managed_error) @managed_error {
    vibe_check err == cringe {
        damn cringe
    }
    
    sus current @managed_error = err
    bestie current.wrapped_error != cringe {
        current = current.wrapped_error
    }
    damn current
}

fr fr Add context to error
slay (err @managed_error) add_context(key tea, value tea) {
    vibe_check err != cringe {
        err.context[key] = value
    }
}

fr fr Get error context
slay (err @managed_error) get_context(key tea) tea {
    vibe_check err == cringe {
        damn ""
    }
    
    vibe_check value, exists := err.context[key]; exists {
        damn value
    }
    damn ""
}

fr fr Capture stack trace (simplified implementation)
slay capture_stack_trace() []tea { fr fr In a real implementation, this would capture actual stack frames
    sus trace []tea = []tea{
        "error_management.capture_stack_trace()",
        "error_management.new_error()",
        "caller_function()"
    }
    damn trace
}

fr fr Format error for display
slay format_error(err @managed_error) tea {
    vibe_check err == cringe {
        damn "no error"
    }
    
    sus formatted tea = "[" + severity_to_string(err.severity) + "] "
    formatted = formatted + "Error " + string(err.code) + ": " + err.message
    
    vibe_check err.details != "" {
        formatted = formatted + " (" + err.details + ")"
    }
    
    vibe_check err.wrapped_error != cringe {
        formatted = formatted + " | Caused by: " + format_error(err.wrapped_error)
    }
    
    damn formatted
}

fr fr Convert severity to string
slay severity_to_string(severity error_severity) tea {
    vibe_check severity {
        mood error_severity.info:
            damn "INFO"
        mood error_severity.warning:
            damn "WARN"
        mood error_severity.error:
            damn "ERROR"
        mood error_severity.critical:
            damn "CRITICAL"
        mood error_severity.fatal:
            damn "FATAL"
        basic:
            damn "UNKNOWN"
    }
}

fr fr Convert category to string
slay category_to_string(category error_category) tea {
    vibe_check category {
        mood error_category.memory_yikes:
            damn "MEMORY"
        mood error_category.io_yikes:
            damn "IO"
        mood error_category.network_yikes:
            damn "NETWORK"
        mood error_category.parse_yikes:
            damn "PARSE"
        mood error_category.type_yikes:
            damn "TYPE"
        mood error_category.runtime_yikes:
            damn "RUNTIME"
        mood error_category.security_yikes:
            damn "SECURITY"
        mood error_category.performance_yikes:
            damn "PERFORMANCE"
        basic:
            damn "UNKNOWN"
    }
}

fr fr Update error statistics
slay update_error_stats(err @managed_error) {
    vibe_check err == cringe {
        damn
    }
    
    global_error_stats.total_errors++ fr fr Update category counts
    sus category_key tea = string(err.category)
    vibe_check count, exists := global_error_stats.errors_by_category[category_key]; exists {
        global_error_stats.errors_by_category[category_key] = count + 1
    } basic {
        global_error_stats.errors_by_category[category_key] = 1
    } fr fr Update severity counts
    sus severity_key tea = string(err.severity)
    vibe_check count, exists := global_error_stats.errors_by_severity[severity_key]; exists {
        global_error_stats.errors_by_severity[severity_key] = count + 1
    } basic {
        global_error_stats.errors_by_severity[severity_key] = 1
    }
}

fr fr Configure logger
slay configure_logger(config logger_config) {
    global_logger = config
}

fr fr Log message at specified level
slay log_at_level(level log_level, message tea, context yikes.tea) {
    vibe_check level < global_logger.level {
        damn fr fr Don't log below configured level
    }
    
    sus log_entry yikes.tea = yikes.tea{
        "level": level_to_string(level),
        "message": message,
        "timestamp": timez.now_rfc3339()
    }
    
    vibe_check global_logger.include_goroutine_id {
        log_entry["goroutine_id"] = "1" fr fr Simplified
    } fr fr Add context
    bestie key, value := mood context {
        log_entry[key] = value
    } fr fr Format and output
    sus formatted tea = format_log_entry(log_entry)
    vibez.spill(formatted)
}

fr fr Log error with full context
slay log_error(err @managed_error, additional_context yikes.tea) {
    vibe_check err == cringe {
        damn
    }
    
    sus context yikes.tea = yikes.tea{
        "error_code": string(err.code),
        "error_category": category_to_string(err.category),
        "error_severity": severity_to_string(err.severity),
        "error_details": err.details
    }
    
    vibe_check global_logger.include_stack_trace && len(err.stack_trace) > 0 {
        context["stack_trace"] = err.stack_trace[0] fr fr First frame
    } fr fr Add error context
    bestie key, value := mood err.context {
        context["ctx_" + key] = value
    } fr fr Add additional context
    bestie key, value := mood additional_context {
        context[key] = value
    }
    
    log_at_level(log_level.error, err.message, context)
    update_error_stats(err)
}

fr fr Convenience logging functions
slay log_debug(message tea, context yikes.tea) {
    log_at_level(log_level.debug, message, context)
}

slay log_info(message tea, context yikes.tea) {
    log_at_level(log_level.info, message, context)
}

slay log_warn(message tea, context yikes.tea) {
    log_at_level(log_level.warn, message, context)
}

slay log_fatal(message tea, context yikes.tea) {
    log_at_level(log_level.fatal, message, context)
}

fr fr Convert log level to string
slay level_to_string(level log_level) tea {
    vibe_check level {
        mood log_level.debug:
            damn "DEBUG"
        mood log_level.info:
            damn "INFO"
        mood log_level.warn:
            damn "WARN"
        mood log_level.error:
            damn "ERROR"
        mood log_level.fatal:
            damn "FATAL"
        basic:
            damn "UNKNOWN"
    }
}

fr fr Format log entry based on configuration
slay format_log_entry(entry yikes.tea) tea {
    vibe_check global_logger.output_format == "json" {
        damn format_log_json(entry)
    } basic {
        damn format_log_text(entry)
    }
}

fr fr Format log entry as JSON (simplified)
slay format_log_json(entry yikes.tea) tea {
    sus result tea = "{"
    sus first lit = based
    
    bestie key, value := mood entry {
        vibe_check !first {
            result = result + ","
        }
        result = result + "\"" + key + "\":\"" + value + "\""
        first = cap
    }
    
    result = result + "}"
    damn result
}

fr fr Format log entry as text
slay format_log_text(entry yikes.tea) tea {
    sus timestamp tea = entry["timestamp"]
    sus level tea = entry["level"]
    sus message tea = entry["message"]
    
    sus result tea = timestamp + " [" + level + "] " + message
    
    bestie key, value := mood entry {
        vibe_check key != "timestamp" && key != "level" && key != "message" {
            result = result + " " + key + "=" + value
        }
    }
    
    damn result
}

fr fr Create new circuit breaker
slay new_circuit_breaker(name tea, failure_threshold normie, timeout_seconds normie) @circuit_breaker {
    damn @circuit_breaker{
        name: name,
        failure_count: 0,
        failure_threshold: failure_threshold,
        timeout_seconds: timeout_seconds,
        last_failure_time: "",
        state: circuit_state.closed
    }
}

fr fr Execute operation with circuit breaker protection
slay (cb @circuit_breaker) execute(operation slay() @managed_error) @managed_error {
    vibe_check cb.state {
        mood circuit_state.open: fr fr Check if we should transition to half-open
            vibe_check cb.should_attempt_reset() {
                cb.state = circuit_state.half_open
            } basic {
                damn new_error("Circuit breaker is open", 503)
            }
        mood circuit_state.half_open: fr fr Allow one test call
        basic: fr fr closed state - normal operation
    }
    
    sus err @managed_error = operation()
    vibe_check err != cringe {
        cb.on_failure()
        damn err
    }
    
    cb.on_success()
    damn cringe
}

fr fr Handle circuit breaker failure
slay (cb @circuit_breaker) on_failure() {
    cb.failure_count++
    cb.last_failure_time = timez.now_rfc3339()
    
    vibe_check cb.failure_count >= cb.failure_threshold {
        cb.state = circuit_state.open
        log_warn("Circuit breaker opened", yikes.tea{
            "circuit_breaker": cb.name,
            "failure_count": string(cb.failure_count)
        })
    }
}

fr fr Handle circuit breaker success
slay (cb @circuit_breaker) on_success() {
    cb.failure_count = 0
    cb.state = circuit_state.closed
}

fr fr Check if circuit breaker should attempt reset
slay (cb @circuit_breaker) should_attempt_reset() lit {
    vibe_check cb.last_failure_time == "" {
        damn based
    } fr fr Simplified time comparison - in real implementation would parse timestamps
    damn based fr fr For demo purposes, always allow reset attempts
}

fr fr Retry operation with exponential backoff
slay retry_with_backoff(operation slay() @managed_error, max_attempts normie) @managed_error {
    sus attempt normie = 0
    
    bestie attempt < max_attempts {
        sus err @managed_error = operation()
        vibe_check err == cringe {
            damn cringe fr fr Success
        }
        
        attempt++
        log_warn("Operation failed, retrying", yikes.tea{
            "attempt": string(attempt),
            "max_attempts": string(max_attempts),
            "error": err.message
        })
        
        vibe_check attempt < max_attempts { fr fr Exponential backoff delay (simplified)
            sus delay_ms normie = attempt * attempt * 100
            vibez.spill("Waiting " + string(delay_ms) + "ms before retry...")
        }
    }
    
    damn new_error("Operation failed after " + string(max_attempts) + " attempts", 500)
}

fr fr Get current error statistics
slay get_error_stats() error_stats {
    damn global_error_stats
}

fr fr Reset error statistics
slay reset_error_stats() {
    global_error_stats = error_stats{
        total_errors: 0,
        errors_by_category: yikes.tea{},
        errors_by_severity: yikes.tea{},
        error_rate: 0.0,
        last_reset_time: timez.now_rfc3339()
    }
}

fr fr Check if error is temporary/retryable
slay is_temporary_error(err @managed_error) lit {
    vibe_check err == cringe {
        damn cap
    } fr fr Consider network and IO errors as potentially temporary
    damn err.category == error_category.network_yikes || 
         err.category == error_category.io_yikes ||
         (err.code >= 500 && err.code < 600)
}

fr fr Check if error is critical
slay is_critical_error(err @managed_error) lit {
    vibe_check err == cringe {
        damn cap
    }
    
    damn err.severity == error_severity.critical || 
         err.severity == error_severity.fatal
}

fr fr Safe operation execution with panic recovery
slay safe_execute(operation slay() @managed_error) @managed_error {
    sus result @managed_error = cringe
    
    fam {
        result = operation()
    } sus panic_value {
        result = new_error_full(
            "Operation panicked: " + string(panic_value),
            500,
            error_category.runtime_yikes,
            error_severity.critical,
            "Panic recovered in safe_execute"
        )
        
        log_error(result, yikes.tea{
            "panic_value": string(panic_value),
            "recovery_point": "safe_execute"
        })
    }
    
    damn result
}

fr fr Error aggregation for multiple operations
slay aggregate_errors(errors []@managed_error) @managed_error {
    vibe_check len(errors) == 0 {
        damn cringe
    }
    
    vibe_check len(errors) == 1 {
        damn errors[0]
    }
    
    sus message tea = "Multiple errors occurred:"
    sus severity error_severity = error_severity.info
    sus total_count normie = 0
    
    bestie i := 0; i < len(errors); i++ {
        vibe_check errors[i] != cringe {
            total_count++
            message = message + " [" + string(i+1) + "] " + errors[i].message
            
            vibe_check errors[i].severity > severity {
                severity = errors[i].severity
            }
        }
    }
    
    sus aggregated @managed_error = new_error_full(
        message,
        400,
        error_category.runtime_yikes,
        severity,
        "Aggregated " + string(total_count) + " errors"
    )
    
    aggregated.add_context("error_count", string(total_count))
    damn aggregated
}
