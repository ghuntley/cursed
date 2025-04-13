# ErrorDrip (errors package)

## Overview
ErrorDrip provides functionality for error creation, handling, and wrapping with enhanced capabilities and expressive error patterns. It's inspired by Go's errors package but with improved context management, cause tracing, and formatted presentation.

## Core Functions

### Error Creation

```go
// New returns a simple error with the given text
func New(text string) error

// Newf creates a formatted error message
func Newf(format string, args ...interface{}) error

// With creates an error with additional context
func With(err error, keyValues ...interface{}) error

// Drip creates an error with call stack context
func Drip(text string) error

// Dripf creates a formatted error with call stack context
func Dripf(format string, args ...interface{}) error
```

### Error Wrapping

```go
// Wrap wraps an error with additional context
func Wrap(err error, message string) error

// Wrapf wraps an error with formatted context
func Wrapf(err error, format string, args ...interface{}) error

// WithStack enriches an error with stack trace information
func WithStack(err error) error

// WithCode adds a status code to an error
func WithCode(err error, code int) error

// WithDetails adds structured details to an error
func WithDetails(err error, details map[string]interface{}) error

// WithRetry marks an error as retryable with optional parameters
func WithRetry(err error, retryable bool, backoff ...time.Duration) error
```

### Error Inspection

```go
// Is reports whether any error in err's tree matches target
func Is(err, target error) bool

// As finds the first error in err's tree that matches the type of target
func As(err error, target interface{}) bool

// Unwrap returns the underlying error
func Unwrap(err error) error

// UnwrapAll returns the root cause of the error
func UnwrapAll(err error) error

// GetDetails extracts structured details from an error
func GetDetails(err error) map[string]interface{}

// GetCode extracts a status code from an error
func GetCode(err error) (int, bool)

// GetStack retrieves stack trace information if available
func GetStack(err error) string

// IsRetryable checks if an error is marked as retryable
func IsRetryable(err error) bool

// GetBackoff retrieves the backoff duration for retrying
func GetBackoff(err error) (time.Duration, bool)
```

## Enhanced Error Types

### `DripError`
Base error type with enhanced capabilities.

```go
type DripError struct {
    Msg     string
    Cause   error
    Stack   []Frame
    Code    int
    Details map[string]interface{}
    Retry   *RetryInfo
}

// Methods
func (e *DripError) Error() string
func (e *DripError) Unwrap() error
func (e *DripError) Format(s fmt.State, verb rune)
func (e *DripError) StackTrace() []Frame
func (e *DripError) AddDetail(key string, value interface{}) *DripError
func (e *DripError) String() string
```

### `Frame`
Represents a stack trace frame.

```go
type Frame struct {
    File     string
    Line     int
    Function string
}

// Methods
func (f Frame) String() string
func (f Frame) MarshalText() ([]byte, error)
```

### `RetryInfo`
Contains retry-related information.

```go
type RetryInfo struct {
    Retryable bool
    BackoffDuration time.Duration
    MaxRetries int
    RetryCount int
}

// Methods
func (r *RetryInfo) ShouldRetry() bool
func (r *RetryInfo) NextBackoff() time.Duration
func (r *RetryInfo) IncrementRetryCount()
```

## Error Groups and Aggregation

```go
type ErrorGroup struct {}

// Constructors
func NewErrorGroup() *ErrorGroup

// Methods
func (g *ErrorGroup) Add(err error)
func (g *ErrorGroup) AddWithLabel(label string, err error)
func (g *ErrorGroup) Wait() error
func (g *ErrorGroup) WaitWithTimeout(timeout time.Duration) error
func (g *ErrorGroup) Errors() []error
func (g *ErrorGroup) ErrorMap() map[string]error
func (g *ErrorGroup) HasErrors() bool
func (g *ErrorGroup) Error() string
func (g *ErrorGroup) Unwrap() []error
```

## Error Patterns

```go
// Common error patterns
func NotFound(entity string, id interface{}) error
func AlreadyExists(entity string, id interface{}) error
func InvalidArgument(argument string, reason string) error
func Unauthorized(reason string) error
func Forbidden(reason string) error
func Timeout(operation string, duration time.Duration) error
func Internal(message string) error
func Unavailable(service string, reason string) error

// Error type checking
func IsNotFound(err error) bool
func IsAlreadyExists(err error) bool
func IsInvalidArgument(err error) bool
func IsUnauthorized(err error) bool
func IsForbidden(err error) bool
func IsTimeout(err error) bool
func IsInternal(err error) bool
func IsUnavailable(err error) bool
```

## Error Formatting and Presentation

```go
// Format an error with specified verbosity
func FormatError(err error, verbose bool) string

// Get a detailed error report with related information
func ErrorReport(err error) string

// Convert an error to JSON format
func ErrorToJSON(err error) ([]byte, error)

// Create a structured error response for APIs
func ErrorResponse(err error) map[string]interface{}

// Create a pretty-formatted error for console display
func PrettyError(err error) string

// Format options
type FormatOptions struct {
    IncludeStack  bool
    IncludeDetails bool
    ColorOutput   bool
    Verbosity     int
    MaxDepth      int
}

// Format with options
func FormatErrorWithOptions(err error, opts FormatOptions) string
```

## Error Handling Utilities

```go
// Try executes a function and returns an error if it panics
func Try(fn func() error) (err error)

// Must panics if err is not nil
func Must(err error)

// Recover turns a panic into an error
func Recover(fn func()) (err error)

// Check checks if err is not nil, and if so, adds source context and returns it
func Check(err error) error

// CheckWrap checks if err is not nil, wraps it, and returns it
func CheckWrap(err error, message string) error

// Ignore silently ignores specific error types
func Ignore(err error, ignoredErrors ...error) error

// Handle processes errors with registered handlers
func Handle(err error) bool

// RegisterHandler registers an error handler for a specific error type
func RegisterHandler(matcher func(error) bool, handler func(error))
```

## GenZ-Themed Error Handling

```go
// Create GenZ-themed error messages
func BigYikes(message string) error
func LowKey(message string) error
func NoCap(message string, facts ...interface{}) error
func SusError(message string) error
func VibeCheck(condition bool, message string) error

// Error level constants
const (
    LevelYikes   = 5 // Critical
    LevelSus     = 4 // Error
    LevelLowKey  = 3 // Warning
    LevelNoCapFr = 2 // Info
    LevelVibe    = 1 // Debug
)

// Add mood/severity level to errors
func WithLevel(err error, level int) error
func GetLevel(err error) (int, bool)
```

## Usage Example

```go
// Basic error creation
err := error_drip.New("connection failed")

// Formatted error creation
err = error_drip.Newf("connection to %s failed", "database")

// Error with stack trace
err = error_drip.Drip("something went wrong")

// Wrapping errors with context
if err != nil {
    return error_drip.Wrap(err, "while connecting to database")
}

// Adding structured details
err = error_drip.WithDetails(err, map[string]interface{}{
    "host":      "db.example.com",
    "port":      5432,
    "reconnect": true,
})

// Creating typed errors
if userID == "" {
    return error_drip.InvalidArgument("userID", "cannot be empty")
}

if !userExists {
    return error_drip.NotFound("user", userID)
}

// Checking error types
if error_drip.IsNotFound(err) {
    // Handle not found case
    return nil, "resource not found"
}

// Working with error groups
group := error_drip.NewErrorGroup()

for i, task := range tasks {
    i, task := i, task // Capture loop variables
    go func() {
        if err := processTask(task); err != nil {
            group.AddWithLabel(fmt.Sprintf("task-%d", i), err)
        }
    }()
}

if err := group.Wait(); err != nil {
    // Handle aggregated errors
    for label, taskErr := range group.ErrorMap() {
        vibez.spill("Error in", label, ":", taskErr)
    }
    return err
}

// Retry handling
err = someOperation()
if err != nil {
    return error_drip.WithRetry(err, true, 5*time.Second)
}

// Error recovery
err = error_drip.Recover(func() {
    // Code that might panic
    if x == 0 {
        panic("division by zero")
    }
    result = 10 / x
})

if err != nil {
    vibez.spill("Recovered from panic:", err)
}

// GenZ-styled error handling
if value < 0 {
    return error_drip.BigYikes("value cannot be negative, bruh")
}

if error_drip.VibeCheck(len(users) == 0, "no users found, check database connection") {
    // Handle empty users list
    return alternativeUsers
}

// Formatted error output
err = fetchData()
if err != nil {
    prettyErr := error_drip.PrettyError(err)
    vibez.spill(prettyErr)
    
    response := error_drip.ErrorResponse(err)
    jsonResponse, _ := json.Marshal(response)
    sendErrorResponse(jsonResponse)
}
```

## Implementation Guidelines
1. Maintain compatibility with the standard errors package
2. Ensure efficient stack trace capture and storage
3. Minimize memory allocations in the error creation path
4. Provide clear, actionable error messages
5. Support structured error details for better diagnosis
6. Implement thread-safe error group aggregation
7. Enable customizable error formatting and presentation
8. Include performance optimizations for high-frequency error paths