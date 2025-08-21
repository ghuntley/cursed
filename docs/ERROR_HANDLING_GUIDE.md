# 🚨 CURSED Error Handling Guide

CURSED provides a comprehensive error handling system with structured error propagation, pattern matching, and recovery mechanisms.

## 📚 Table of Contents

- [Error Types](#error-types)
- [Error Propagation](#error-propagation)
- [Pattern Matching](#pattern-matching)
- [Error Recovery](#error-recovery)
- [Best Practices](#best-practices)
- [Advanced Patterns](#advanced-patterns)

## 🎯 Error Types

### Basic Error Handling

CURSED uses the `yikes`/`fam`/`shook` system for structured error handling:

```cursed
# Function that can fail
slay divide(a drip, b drip) yikes<drip> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

# Handle the error
sus result drip = divide(10, 2) fam {
    when "division by zero" -> {
        vibez.spill("Cannot divide by zero!")
        damn 0
    }
    when _ -> {
        vibez.spill("Unknown error occurred")
        damn -1
    }
}

vibez.spill("Result:", result)
```

### Typed Errors

```cursed
# Define custom error types
enum MathError {
    DivisionByZero,
    Overflow,
    Underflow,
    InvalidInput(tea)
}

slay safe_divide(a drip, b drip) yikes<drip, MathError> {
    ready (b == 0) {
        yikes MathError.DivisionByZero
    }
    
    sus result drip = a / b
    
    # Check for overflow (simplified example)
    ready (result > 1000000) {
        yikes MathError.Overflow
    }
    
    damn result
}

# Handle typed errors
sus result drip = safe_divide(100, 5) fam {
    when MathError.DivisionByZero -> {
        vibez.spill("Error: Division by zero")
        damn 0
    }
    when MathError.Overflow -> {
        vibez.spill("Error: Result too large")
        damn 999999
    }
    when MathError.Underflow -> {
        vibez.spill("Error: Result too small")
        damn -999999
    }
    when MathError.InvalidInput(msg) -> {
        vibez.spill("Error: Invalid input -", msg)
        damn -1
    }
}
```

### Error Context and Chaining

```cursed
# Error with context information
squad ErrorContext {
    message tea
    file tea
    line drip
    cause tea
}

impl ErrorContext {
    slay new(message tea, file tea, line drip) ErrorContext {
        damn ErrorContext{
            message: message,
            file: file,
            line: line,
            cause: ""
        }
    }
    
    slay with_cause(cause tea) ErrorContext {
        self.cause = cause
        damn self
    }
    
    slay format() tea {
        ready (stringz.len(self.cause) > 0) {
            damn stringz.format("%s at %s:%d (caused by: %s)", 
                               self.message, self.file, self.line, self.cause)
        } otherwise {
            damn stringz.format("%s at %s:%d", self.message, self.file, self.line)
        }
    }
}

# Function with contextual errors
slay read_config_file(path tea) yikes<tea, ErrorContext> {
    sus content tea = filez.read_text(path) fam {
        when "file not found" -> {
            yikes ErrorContext.new("Config file not found", "config.csd", 42)
        }
        when "permission denied" -> {
            yikes ErrorContext.new("Cannot read config file", "config.csd", 43)
                    .with_cause("insufficient permissions")
        }
        when _ -> {
            yikes ErrorContext.new("Failed to read config", "config.csd", 44)
        }
    }
    
    # Validate config content
    ready (stringz.len(content) == 0) {
        yikes ErrorContext.new("Config file is empty", "config.csd", 50)
    }
    
    damn content
}
```

## 🔄 Error Propagation

### Automatic Error Propagation

```cursed
# Error propagation with the ? operator (try syntax)
slay process_files() yikes<lit> {
    sus config tea = read_config_file("app.conf")?  # Propagates error automatically
    sus data tea = filez.read_text("data.txt")?
    
    # Process both files
    vibez.spill("Config:", config)
    vibez.spill("Data:", data)
    
    damn based
}

# Alternative explicit propagation
slay process_files_explicit() yikes<lit> {
    sus config tea = read_config_file("app.conf") fam {
        when _ -> yikes  # Re-raise the same error
    }
    
    sus data tea = filez.read_text("data.txt") fam {
        when _ -> yikes  # Re-raise the same error  
    }
    
    damn based
}
```

### Error Transformation

```cursed
# Transform errors during propagation
enum ProcessingError {
    ConfigError(tea),
    DataError(tea),
    ValidationError(tea)
}

slay process_with_transformation() yikes<lit, ProcessingError> {
    sus config tea = read_config_file("app.conf") fam {
        when ErrorContext(ctx) -> {
            yikes ProcessingError.ConfigError(ctx.format())
        }
        when _ -> {
            yikes ProcessingError.ConfigError("Unknown config error")
        }
    }
    
    sus data tea = parse_data(config) fam {
        when "invalid format" -> {
            yikes ProcessingError.DataError("Data format is invalid")
        }
        when "missing field" -> {
            yikes ProcessingError.ValidationError("Required field missing")
        }
        when _ -> {
            yikes ProcessingError.DataError("Data processing failed")
        }
    }
    
    damn based
}
```

### Error Accumulation

```cursed
# Collect multiple errors instead of failing fast
squad ValidationResult {
    success lit
    errors []tea
    warnings []tea
}

impl ValidationResult {
    slay new() ValidationResult {
        damn ValidationResult{
            success: based,
            errors: [],
            warnings: []
        }
    }
    
    slay add_error(error tea) {
        self.errors = arrayz.append(self.errors, error)
        self.success = false
    }
    
    slay add_warning(warning tea) {
        self.warnings = arrayz.append(self.warnings, warning)
    }
    
    slay is_valid() lit {
        damn self.success && len(self.errors) == 0
    }
}

slay validate_user_data(name tea, email tea, age drip) ValidationResult {
    sus result ValidationResult = ValidationResult.new()
    
    # Validate name
    ready (stringz.len(name) < 2) {
        result.add_error("Name must be at least 2 characters long")
    }
    
    ready (stringz.len(name) > 50) {
        result.add_warning("Name is unusually long")
    }
    
    # Validate email
    ready (!stringz.contains(email, "@")) {
        result.add_error("Email must contain @ symbol")
    }
    
    ready (!stringz.contains(email, ".")) {
        result.add_error("Email must contain domain extension")
    }
    
    # Validate age
    ready (age < 0) {
        result.add_error("Age cannot be negative")
    }
    
    ready (age > 150) {
        result.add_warning("Age seems unusually high")
    }
    
    damn result
}

# Usage
sus validation ValidationResult = validate_user_data("Jo", "invalid-email", 25)

ready (!validation.is_valid()) {
    vibez.spill("Validation failed with", len(validation.errors), "errors:")
    bestie (error in validation.errors) {
        vibez.spill("  ERROR:", error)
    }
}

ready (len(validation.warnings) > 0) {
    vibez.spill("Validation warnings:")
    bestie (warning in validation.warnings) {
        vibez.spill("  WARNING:", warning)
    }
}
```

## 🎯 Pattern Matching

### Advanced Error Pattern Matching

```cursed
# Comprehensive error matching
enum NetworkError {
    Timeout(drip),
    ConnectionRefused,
    DNSResolution(tea),
    SSL(tea),
    Protocol(drip, tea)
}

slay handle_network_error(error NetworkError) tea {
    sick (error) {
        when NetworkError.Timeout(ms) -> {
            ready (ms > 30000) {
                damn "Long timeout - check server status"
            } otherwise {
                damn "Network timeout - retry recommended"
            }
        }
        when NetworkError.ConnectionRefused -> {
            damn "Server is not accepting connections"
        }
        when NetworkError.DNSResolution(domain) -> {
            damn stringz.format("Cannot resolve domain: %s", domain)
        }
        when NetworkError.SSL(cert_error) -> {
            ready (stringz.contains(cert_error, "expired")) {
                damn "SSL certificate has expired"
            } otherwise {
                damn stringz.format("SSL error: %s", cert_error)
            }
        }
        when NetworkError.Protocol(code, message) -> {
            ready (code >= 400 && code < 500) {
                damn stringz.format("Client error (%d): %s", code, message)
            } ready (code >= 500) {
                damn stringz.format("Server error (%d): %s", code, message)
            } otherwise {
                damn stringz.format("Protocol error (%d): %s", code, message)
            }
        }
    }
}
```

### Guard Patterns in Error Handling

```cursed
# Pattern matching with guards
slay classify_http_error(status_code drip, message tea) tea {
    sick (status_code) {
        when code ready (code >= 200 && code < 300) -> "Success"
        when code ready (code >= 300 && code < 400) -> "Redirection"
        when 404 -> "Not Found"
        when 403 -> "Forbidden"
        when 401 -> "Unauthorized"
        when code ready (code >= 400 && code < 500) -> "Client Error"
        when code ready (code >= 500 && code < 600) -> "Server Error"
        when _ -> "Unknown Status"
    }
}

# Multiple condition guards
slay handle_file_error(error tea, file_size drip) tea {
    sick (error) {
        when "permission denied" ready (file_size > 1024*1024) -> {
            "Large file access denied - check permissions"
        }
        when "permission denied" -> {
            "File access denied - check permissions"
        }
        when "file not found" ready (stringz.contains(error, ".tmp")) -> {
            "Temporary file missing - may have been cleaned up"
        }
        when "file not found" -> {
            "File not found - check path"
        }
        when "disk full" ready (file_size > 1024*1024*1024) -> {
            "Cannot write large file - disk full"
        }
        when "disk full" -> {
            "Cannot write file - disk full"
        }
        when _ -> {
            stringz.format("Unknown file error: %s", error)
        }
    }
}
```

## 🛡️ Error Recovery

### Retry Mechanisms

```cursed
# Exponential backoff retry
squad RetryConfig {
    max_attempts drip
    initial_delay_ms drip
    backoff_factor meal
    max_delay_ms drip
}

slay with_retry<T>(config RetryConfig, operation slay() yikes<T>) yikes<T> {
    sus attempt drip = 0
    sus delay drip = config.initial_delay_ms
    
    bestie (attempt < config.max_attempts) {
        sus result T = operation() fam {
            when _ -> {
                attempt = attempt + 1
                
                ready (attempt >= config.max_attempts) {
                    yikes "maximum retry attempts exceeded"
                }
                
                vibez.spill("Attempt", attempt, "failed, retrying in", delay, "ms...")
                timez.sleep(delay)
                
                # Exponential backoff
                delay = mathz.min(
                    delay * config.backoff_factor,
                    config.max_delay_ms
                )
                
                skip  # Continue loop
            }
        }
        
        damn result  # Success case
    }
    
    yikes "retry loop ended unexpectedly"
}

# Usage
sus retry_config RetryConfig = {
    max_attempts: 5,
    initial_delay_ms: 100,
    backoff_factor: 2.0,
    max_delay_ms: 5000
}

sus result tea = with_retry(retry_config, slay() yikes<tea> {
    # This operation might fail
    sus response tea = networkz.http_get("https://api.example.com/data") fam {
        when "timeout" -> yikes "network timeout"
        when "connection refused" -> yikes "connection failed"
        when _ -> yikes "network error"
    }
    damn response
}) fam {
    when "maximum retry attempts exceeded" -> {
        vibez.spill("All retry attempts failed")
        damn "fallback data"
    }
    when _ -> {
        vibez.spill("Retry failed with unknown error")
        damn ""
    }
}
```

### Circuit Breaker Pattern

```cursed
# Circuit breaker for fault tolerance
enum CircuitState {
    Closed,    # Normal operation
    Open,      # Failing fast
    HalfOpen   # Testing recovery
}

squad CircuitBreaker {
    state CircuitState
    failure_count drip
    failure_threshold drip
    recovery_timeout_ms drip
    last_failure_time drip
    success_threshold drip
    half_open_success_count drip
}

impl CircuitBreaker {
    slay new(failure_threshold drip, recovery_timeout_ms drip) CircuitBreaker {
        damn CircuitBreaker{
            state: CircuitState.Closed,
            failure_count: 0,
            failure_threshold: failure_threshold,
            recovery_timeout_ms: recovery_timeout_ms,
            last_failure_time: 0,
            success_threshold: 3,
            half_open_success_count: 0
        }
    }
    
    slay call<T>(operation slay() yikes<T>) yikes<T> {
        sick (self.state) {
            when CircuitState.Open -> {
                sus now drip = timez.now()
                ready (now - self.last_failure_time > self.recovery_timeout_ms) {
                    self.state = CircuitState.HalfOpen
                    self.half_open_success_count = 0
                } otherwise {
                    yikes "circuit breaker is open"
                }
            }
            when CircuitState.HalfOpen -> {
                # Limited testing in half-open state
                ready (self.half_open_success_count >= self.success_threshold) {
                    self.state = CircuitState.Closed
                    self.failure_count = 0
                }
            }
            when CircuitState.Closed -> {
                # Normal operation
            }
        }
        
        # Execute operation
        sus result T = operation() fam {
            when _ -> {
                self.record_failure()
                yikes  # Re-raise error
            }
        }
        
        self.record_success()
        damn result
    }
    
    slay record_failure() {
        self.failure_count = self.failure_count + 1
        self.last_failure_time = timez.now()
        
        sick (self.state) {
            when CircuitState.Closed -> {
                ready (self.failure_count >= self.failure_threshold) {
                    self.state = CircuitState.Open
                    vibez.spill("Circuit breaker opened due to failures")
                }
            }
            when CircuitState.HalfOpen -> {
                self.state = CircuitState.Open
                vibez.spill("Circuit breaker re-opened from half-open")
            }
            when _ -> {}
        }
    }
    
    slay record_success() {
        sick (self.state) {
            when CircuitState.HalfOpen -> {
                self.half_open_success_count = self.half_open_success_count + 1
                ready (self.half_open_success_count >= self.success_threshold) {
                    self.state = CircuitState.Closed
                    self.failure_count = 0
                    vibez.spill("Circuit breaker closed after recovery")
                }
            }
            when CircuitState.Closed -> {
                # Reset failure count on success
                self.failure_count = 0
            }
            when _ -> {}
        }
    }
}

# Usage
sus breaker CircuitBreaker = CircuitBreaker.new(5, 60000)  # 5 failures, 1 minute timeout

sus result tea = breaker.call(slay() yikes<tea> {
    # Potentially failing operation
    damn networkz.http_get("https://unreliable-api.com/data")
}) fam {
    when "circuit breaker is open" -> {
        vibez.spill("Service is currently unavailable")
        damn "cached_fallback_data"
    }
    when _ -> {
        vibez.spill("Operation failed")
        damn ""
    }
}
```

### Panic Recovery

```cursed
# Panic recovery for graceful error handling
slay safe_operation<T>(operation slay() T, default_value T) T {
    recover {
        damn operation()
    } handle panic_msg -> {
        vibez.spill("Operation panicked:", panic_msg)
        damn default_value
    }
}

# Advanced panic recovery with error conversion
slay safe_operation_with_error<T>(operation slay() T) yikes<T> {
    recover {
        damn operation()
    } handle panic_msg -> {
        yikes stringz.format("operation panicked: %s", panic_msg)
    }
}

# Usage examples
sus result drip = safe_operation(slay() drip {
    sus x drip = 0
    damn 100 / x  # This will panic
}, -1)

vibez.spill("Safe result:", result)  # Will print: Safe result: -1

# With error propagation
sus safe_result drip = safe_operation_with_error(slay() drip {
    sus dangerous_array []drip = [1, 2, 3]
    damn dangerous_array[10]  # Index out of bounds
}) fam {
    when _ -> {
        vibez.spill("Handled panic as error")
        damn 0
    }
}
```

## ✅ Best Practices

### 1. Error Message Quality

```cursed
# Provide actionable error messages
slay parse_json_config(content tea) yikes<map<tea, tea>> {
    ready (stringz.len(content) == 0) {
        yikes "config file is empty - add configuration in JSON format"
    }
    
    ready (!stringz.starts_with(content, "{")) {
        yikes "config must be a valid JSON object starting with '{'"
    }
    
    sus config map<tea, tea> = jsonz.parse(content) fam {
        when "invalid json syntax" -> {
            yikes "config contains invalid JSON - check for missing commas, quotes, or brackets"
        }
        when "unexpected token" -> {
            yikes "config has JSON syntax error - validate with a JSON validator"
        }
        when _ -> {
            yikes "config is not valid JSON format"
        }
    }
    
    # Validate required fields
    ready (!("app_name" in config)) {
        yikes "config missing required field 'app_name' - add: \"app_name\": \"your-app\""
    }
    
    damn config
}
```

### 2. Fail Fast vs Graceful Degradation

```cursed
# Critical errors: fail fast
slay initialize_database() yikes<lit> {
    sus connection dbz.Connection = dbz.connect("postgresql://...") fam {
        when _ -> yikes "FATAL: Cannot connect to database - application cannot start"
    }
    
    # Migration is critical
    dbz.migrate(connection) fam {
        when _ -> yikes "FATAL: Database migration failed - application cannot start"
    }
    
    damn based
}

# Non-critical errors: graceful degradation
slay load_user_preferences(user_id drip) map<tea, tea> {
    sus preferences map<tea, tea> = load_from_cache(user_id) fam {
        when _ -> {
            vibez.spill("WARN: Cache unavailable, using defaults")
            damn default_preferences()
        }
    }
    
    # Try to refresh from database, but don't fail if unavailable
    sus db_preferences map<tea, tea> = load_from_database(user_id) fam {
        when _ -> {
            vibez.spill("WARN: Database unavailable, using cached preferences")
            damn preferences
        }
    }
    
    damn merge_preferences(preferences, db_preferences)
}
```

### 3. Error Context Preservation

```cursed
# Preserve error context through call stack
slay process_user_request(request_id tea) yikes<tea> {
    sus user_data tea = fetch_user_data(request_id) fam {
        when _ -> yikes stringz.format("failed to process request %s: user data unavailable", request_id)
    }
    
    sus processed_data tea = transform_data(user_data) fam {
        when error -> yikes stringz.format("failed to process request %s: %s", request_id, error)
    }
    
    sus result tea = save_result(processed_data) fam {
        when error -> yikes stringz.format("failed to process request %s: save failed - %s", request_id, error)
    }
    
    damn result
}
```

### 4. Error Logging and Monitoring

```cursed
# Structured error logging
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal
}

squad ErrorLogger {
    level LogLevel
    output chan<tea>
}

impl ErrorLogger {
    slay new() ErrorLogger {
        sus logger ErrorLogger = {
            level: LogLevel.Info,
            output: make_channel<tea>()
        }
        
        # Start logging goroutine
        go {
            bestie (message := <-logger.output) {
                vibez.spill(timez.format_time(timez.now(), "2006-01-02 15:04:05"), message)
            }
        }
        
        damn logger
    }
    
    slay log_error(context tea, error tea, request_id tea) {
        sus message tea = jsonz.marshal({
            "level": "ERROR",
            "context": context,
            "error": error,
            "request_id": request_id,
            "timestamp": timez.now()
        }) fam {
            when _ -> damn stringz.format("ERROR: %s - %s (request: %s)", context, error, request_id)
        }
        
        self.output <- message
    }
}

# Global error logger
sus error_logger ErrorLogger = ErrorLogger.new()

slay with_error_logging<T>(context tea, request_id tea, operation slay() yikes<T>) yikes<T> {
    sus result T = operation() fam {
        when error -> {
            error_logger.log_error(context, error, request_id)
            yikes  # Re-raise
        }
    }
    
    damn result
}
```

## 🏗️ Advanced Patterns

### Result Type Pattern

```cursed
# Result type for explicit error handling
enum Result<T> {
    Ok(T),
    Error(tea)
}

impl<T> Result<T> {
    slay is_ok() lit {
        sick (self) {
            when Result.Ok(_) -> based
            when _ -> false
        }
    }
    
    slay is_error() lit {
        damn !self.is_ok()
    }
    
    slay unwrap() yikes<T> {
        sick (self) {
            when Result.Ok(value) -> damn value
            when Result.Error(error) -> yikes error
        }
    }
    
    slay unwrap_or(default T) T {
        sick (self) {
            when Result.Ok(value) -> damn value
            when Result.Error(_) -> damn default
        }
    }
    
    slay map<U>(func slay(T) U) Result<U> {
        sick (self) {
            when Result.Ok(value) -> damn Result.Ok(func(value))
            when Result.Error(error) -> damn Result.Error(error)
        }
    }
    
    slay and_then<U>(func slay(T) Result<U>) Result<U> {
        sick (self) {
            when Result.Ok(value) -> damn func(value)
            when Result.Error(error) -> damn Result.Error(error)
        }
    }
}

# Usage
slay safe_divide(a drip, b drip) Result<drip> {
    ready (b == 0) {
        damn Result.Error("division by zero")
    }
    damn Result.Ok(a / b)
}

sus result Result<drip> = safe_divide(10, 2)
    .map(slay(x drip) drip { damn x * 2 })
    .and_then(slay(x drip) Result<drip> {
        ready (x > 100) {
            damn Result.Error("result too large")
        }
        damn Result.Ok(x)
    })

ready (result.is_ok()) {
    vibez.spill("Success:", result.unwrap())
} otherwise {
    vibez.spill("Error:", result.unwrap_or(0))
}
```

### Error Aggregation Pattern

```cursed
# Collect and combine multiple errors
squad ErrorCollector {
    errors []tea
    context tea
}

impl ErrorCollector {
    slay new(context tea) ErrorCollector {
        damn ErrorCollector{errors: [], context: context}
    }
    
    slay add_error(error tea) {
        self.errors = arrayz.append(self.errors, error)
    }
    
    slay try<T>(operation slay() yikes<T>) yikes<T> {
        sus result T = operation() fam {
            when error -> {
                self.add_error(error)
                yikes error  # Early return on error
            }
        }
        damn result
    }
    
    slay try_continue<T>(operation slay() yikes<T>, default T) T {
        sus result T = operation() fam {
            when error -> {
                self.add_error(error)
                damn default
            }
        }
        damn result
    }
    
    slay has_errors() lit {
        damn len(self.errors) > 0
    }
    
    slay get_summary() tea {
        ready (!self.has_errors()) {
            damn stringz.format("%s: completed successfully", self.context)
        }
        
        sus error_count drip = len(self.errors)
        sus summary tea = stringz.format("%s: %d errors occurred:", self.context, error_count)
        
        bestie (error in self.errors) {
            summary = summary + "\n  - " + error
        }
        
        damn summary
    }
}

# Usage
slay validate_system() tea {
    sus collector ErrorCollector = ErrorCollector.new("System Validation")
    
    # Collect errors without stopping
    collector.try_continue(slay() yikes<lit> {
        check_database_connection()
    }, false)
    
    collector.try_continue(slay() yikes<lit> {
        verify_api_keys()
    }, false)
    
    collector.try_continue(slay() yikes<lit> {
        validate_configuration()
    }, false)
    
    damn collector.get_summary()
}
```

---

**CURSED's error handling system provides comprehensive, type-safe error management with pattern matching, recovery mechanisms, and advanced error composition patterns! 🚨**
