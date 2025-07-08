# CURSED Error Core Module
# Provides comprehensive error handling hierarchy and utilities

# Base error interface that all errors implement
be_like error_interface collab {
    message() tea
    code() normie
    details() tea
    severity() error_severity
    stack_trace() tea
}

# Error severity levels
be_like error_severity smol {
    info = 0
    warning = 1
    error = 2
    critical = 3
    fatal = 4
}

# Base error type with full context
be_like base_error squad {
    msg tea
    error_code normie
    error_details tea
    error_severity error_severity
    trace tea
    timestamp tea
    context tea
}

# Implement error interface for base_error
slay (err @base_error) message() tea {
    damn err.msg
}

slay (err @base_error) code() normie {
    damn err.error_code
}

slay (err @base_error) details() tea {
    damn err.error_details
}

slay (err @base_error) severity() error_severity {
    damn err.error_severity
}

slay (err @base_error) stack_trace() tea {
    damn err.trace
}

# Error type hierarchy
be_like io_error squad {
    base base_error
    path tea
    operation tea
}

be_like value_error squad {
    base base_error
    value tea
    expected_type tea
}

be_like type_error squad {
    base base_error
    actual_type tea
    expected_type tea
}

be_like memory_error squad {
    base base_error
    requested_size normie
    available_size normie
}

be_like network_error squad {
    base base_error
    host tea
    port normie
    timeout_duration tea
}

be_like parse_error squad {
    base base_error
    position normie
    line normie
    column normie
}

be_like security_error squad {
    base base_error
    resource tea
    required_permission tea
}

be_like runtime_error squad {
    base base_error
    goroutine_id normie
    operation tea
}

# Error creation helpers
slay new_error(message tea, code normie) base_error {
    damn base_error{
        msg: message,
        error_code: code,
        error_details: "",
        error_severity: error,
        trace: capture_stack_trace(),
        timestamp: current_time(),
        context: ""
    }
}

slay new_io_error(message tea, path tea, operation tea) io_error {
    damn io_error{
        base: base_error{
            msg: message,
            error_code: 1001,
            error_details: "IO operation failed on: " + path,
            error_severity: error,
            trace: capture_stack_trace(),
            timestamp: current_time(),
            context: "io_operation"
        },
        path: path,
        operation: operation
    }
}

slay new_value_error(message tea, value tea, expected_type tea) value_error {
    damn value_error{
        base: base_error{
            msg: message,
            error_code: 2001,
            error_details: "Value '" + value + "' is not of expected type: " + expected_type,
            error_severity: error,
            trace: capture_stack_trace(),
            timestamp: current_time(),
            context: "value_validation"
        },
        value: value,
        expected_type: expected_type
    }
}

slay new_type_error(message tea, actual_type tea, expected_type tea) type_error {
    damn type_error{
        base: base_error{
            msg: message,
            error_code: 3001,
            error_details: "Type mismatch: got " + actual_type + ", expected " + expected_type,
            error_severity: error,
            trace: capture_stack_trace(),
            timestamp: current_time(),
            context: "type_checking"
        },
        actual_type: actual_type,
        expected_type: expected_type
    }
}

slay new_memory_error(message tea, requested normie, available normie) memory_error {
    damn memory_error{
        base: base_error{
            msg: message,
            error_code: 4001,
            error_details: "Memory allocation failed: requested " + string(requested) + " bytes, available " + string(available) + " bytes",
            error_severity: critical,
            trace: capture_stack_trace(),
            timestamp: current_time(),
            context: "memory_management"
        },
        requested_size: requested,
        available_size: available
    }
}

slay new_network_error(message tea, host tea, port normie, timeout tea) network_error {
    damn network_error{
        base: base_error{
            msg: message,
            error_code: 5001,
            error_details: "Network operation failed for " + host + ":" + string(port) + " (timeout: " + timeout + ")",
            error_severity: error,
            trace: capture_stack_trace(),
            timestamp: current_time(),
            context: "network_operation"
        },
        host: host,
        port: port,
        timeout_duration: timeout
    }
}

slay new_parse_error(message tea, position normie, line normie, column normie) parse_error {
    damn parse_error{
        base: base_error{
            msg: message,
            error_code: 6001,
            error_details: "Parse error at line " + string(line) + ", column " + string(column) + " (position " + string(position) + ")",
            error_severity: error,
            trace: capture_stack_trace(),
            timestamp: current_time(),
            context: "parsing"
        },
        position: position,
        line: line,
        column: column
    }
}

slay new_security_error(message tea, resource tea, permission tea) security_error {
    damn security_error{
        base: base_error{
            msg: message,
            error_code: 7001,
            error_details: "Security violation: insufficient permissions for " + resource + " (required: " + permission + ")",
            error_severity: critical,
            trace: capture_stack_trace(),
            timestamp: current_time(),
            context: "security"
        },
        resource: resource,
        required_permission: permission
    }
}

slay new_runtime_error(message tea, goroutine_id normie, operation tea) runtime_error {
    damn runtime_error{
        base: base_error{
            msg: message,
            error_code: 8001,
            error_details: "Runtime error in goroutine " + string(goroutine_id) + " during " + operation,
            error_severity: error,
            trace: capture_stack_trace(),
            timestamp: current_time(),
            context: "runtime"
        },
        goroutine_id: goroutine_id,
        operation: operation
    }
}

# Error propagation helpers
slay wrap_error(err yikes, context tea) yikes {
    vibe_check err == cringe {
        damn cringe
    }
    
    damn yikes{
        message: context + ": " + err.message(),
        code: err.code(),
        details: err.details()
    }
}

slay chain_error(base_err yikes, new_err yikes) yikes {
    vibe_check base_err == cringe {
        damn new_err
    }
    
    vibe_check new_err == cringe {
        damn base_err
    }
    
    damn yikes{
        message: new_err.message() + " (caused by: " + base_err.message() + ")",
        code: new_err.code(),
        details: new_err.details() + " | Previous: " + base_err.details()
    }
}

# Error aggregation for multiple errors
slay combine_errors(errors []yikes) yikes {
    vibe_check len(errors) == 0 {
        damn cringe
    }
    
    vibe_check len(errors) == 1 {
        damn errors[0]
    }
    
    sus combined_message tea = "Multiple errors occurred: "
    sus combined_details tea = ""
    
    bestie i := 0; i < len(errors); i++ {
        vibe_check i > 0 {
            combined_message = combined_message + "; "
            combined_details = combined_details + " | "
        }
        combined_message = combined_message + errors[i].message()
        combined_details = combined_details + errors[i].details()
    }
    
    damn yikes{
        message: combined_message,
        code: 9001,
        details: combined_details
    }
}

# Backtrace capture functionality
slay capture_stack_trace() tea {
    # In a real implementation, this would capture the actual stack trace
    # For now, we'll return a placeholder
    damn "Stack trace captured at " + current_time()
}

slay current_time() tea {
    # In a real implementation, this would return the current timestamp
    # For now, we'll return a placeholder
    damn "2025-01-07 00:00:00"
}

# Error comparison utilities
slay is_error_type(err yikes, error_type tea) lit {
    vibe_check err == cringe {
        damn cap
    }
    
    vibe_check error_type == "io_error" {
        damn err.code() >= 1000 && err.code() < 2000
    }
    
    vibe_check error_type == "value_error" {
        damn err.code() >= 2000 && err.code() < 3000
    }
    
    vibe_check error_type == "type_error" {
        damn err.code() >= 3000 && err.code() < 4000
    }
    
    vibe_check error_type == "memory_error" {
        damn err.code() >= 4000 && err.code() < 5000
    }
    
    vibe_check error_type == "network_error" {
        damn err.code() >= 5000 && err.code() < 6000
    }
    
    vibe_check error_type == "parse_error" {
        damn err.code() >= 6000 && err.code() < 7000
    }
    
    vibe_check error_type == "security_error" {
        damn err.code() >= 7000 && err.code() < 8000
    }
    
    vibe_check error_type == "runtime_error" {
        damn err.code() >= 8000 && err.code() < 9000
    }
    
    damn cap
}

slay is_temporary_error(err yikes) lit {
    vibe_check err == cringe {
        damn cap
    }
    
    # Network errors and IO errors are often temporary
    damn is_error_type(err, "network_error") || is_error_type(err, "io_error")
}

slay is_critical_error(err yikes) lit {
    vibe_check err == cringe {
        damn cap
    }
    
    # Memory and security errors are critical
    damn is_error_type(err, "memory_error") || is_error_type(err, "security_error")
}

# Error formatting utilities
slay format_error(err yikes) tea {
    vibe_check err == cringe {
        damn "no error"
    }
    
    damn "[Error " + string(err.code()) + "] " + err.message() + " | Details: " + err.details()
}

slay format_error_json(err yikes) tea {
    vibe_check err == cringe {
        damn "{\"error\": null}"
    }
    
    damn "{\"error\": {\"code\": " + string(err.code()) + ", \"message\": \"" + err.message() + "\", \"details\": \"" + err.details() + "\"}}"
}

# Error recovery patterns
slay retry_with_backoff(operation slay() yikes, max_attempts normie, base_delay normie) yikes {
    sus attempt normie = 0
    sus delay normie = base_delay
    
    bestie attempt < max_attempts {
        sus err = operation()
        vibe_check err == cringe {
            damn cringe  # Success
        }
        
        vibe_check !is_temporary_error(err) {
            damn err  # Non-temporary error, don't retry
        }
        
        attempt++
        
        # Exponential backoff
        # In a real implementation, this would actually delay
        delay = delay * 2
        
        vibe_check attempt >= max_attempts {
            damn wrap_error(err, "Operation failed after " + string(max_attempts) + " attempts")
        }
    }
    
    damn yikes{
        message: "Maximum retry attempts exceeded",
        code: 9999,
        details: "Failed after " + string(max_attempts) + " attempts"
    }
}

# Circuit breaker error handling
be_like circuit_breaker squad {
    failure_count normie
    failure_threshold normie
    success_threshold normie
    state circuit_state
    last_failure_time tea
}

be_like circuit_state smol {
    closed = 0
    open = 1
    half_open = 2
}

slay new_circuit_breaker(failure_threshold normie, success_threshold normie) circuit_breaker {
    damn circuit_breaker{
        failure_count: 0,
        failure_threshold: failure_threshold,
        success_threshold: success_threshold,
        state: closed,
        last_failure_time: ""
    }
}

slay (cb @circuit_breaker) call(operation slay() yikes) yikes {
    vibe_check cb.state == open {
        damn yikes{
            message: "Circuit breaker is open",
            code: 9100,
            details: "Circuit breaker preventing calls due to previous failures"
        }
    }
    
    sus err = operation()
    vibe_check err == cringe {
        cb.on_success()
        damn cringe
    }
    
    cb.on_failure()
    damn err
}

slay (cb @circuit_breaker) on_success() {
    vibe_check cb.state == half_open {
        cb.state = closed
        cb.failure_count = 0
    }
}

slay (cb @circuit_breaker) on_failure() {
    cb.failure_count++
    cb.last_failure_time = current_time()
    
    vibe_check cb.failure_count >= cb.failure_threshold {
        cb.state = open
    }
}

# Error statistics and monitoring
be_like error_stats squad {
    total_errors normie
    error_rate meal
    errors_by_type map[tea]normie
    recent_errors []yikes
}

sus global_error_stats error_stats = error_stats{
    total_errors: 0,
    error_rate: 0.0,
    errors_by_type: make(map[tea]normie),
    recent_errors: make([]yikes, 0, 100)
}

slay record_error(err yikes) {
    vibe_check err == cringe {
        damn
    }
    
    global_error_stats.total_errors++
    
    sus error_type tea = "unknown"
    vibe_check is_error_type(err, "io_error") {
        error_type = "io_error"
    } basic vibe_check is_error_type(err, "value_error") {
        error_type = "value_error"
    } basic vibe_check is_error_type(err, "type_error") {
        error_type = "type_error"
    } basic vibe_check is_error_type(err, "memory_error") {
        error_type = "memory_error"
    } basic vibe_check is_error_type(err, "network_error") {
        error_type = "network_error"
    } basic vibe_check is_error_type(err, "parse_error") {
        error_type = "parse_error"
    } basic vibe_check is_error_type(err, "security_error") {
        error_type = "security_error"
    } basic vibe_check is_error_type(err, "runtime_error") {
        error_type = "runtime_error"
    }
    
    global_error_stats.errors_by_type[error_type]++
    
    # Keep only recent errors (last 100)
    global_error_stats.recent_errors = append(global_error_stats.recent_errors, err)
    vibe_check len(global_error_stats.recent_errors) > 100 {
        global_error_stats.recent_errors = global_error_stats.recent_errors[1:]
    }
}

slay get_error_stats() error_stats {
    damn global_error_stats
}

# Error context preservation
be_like error_context squad {
    goroutine_id normie
    function_name tea
    file_name tea
    line_number normie
    user_context tea
}

slay capture_error_context(user_context tea) error_context {
    damn error_context{
        goroutine_id: get_current_goroutine_id(),
        function_name: get_current_function_name(),
        file_name: get_current_file_name(),
        line_number: get_current_line_number(),
        user_context: user_context
    }
}

# Placeholder implementations for runtime functions
slay get_current_goroutine_id() normie {
    damn 1  # In real implementation, this would return actual goroutine ID
}

slay get_current_function_name() tea {
    damn "current_function"  # In real implementation, this would return actual function name
}

slay get_current_file_name() tea {
    damn "current_file.csd"  # In real implementation, this would return actual file name
}

slay get_current_line_number() normie {
    damn 1  # In real implementation, this would return actual line number
}
