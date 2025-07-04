# ErrorDrip (teas package)

## Overview
ErrorDrip provides functionality for tea creation, handling, and wrapping with enhanced capabilities and expressive tea patterns. It's inspired by Go's teas package but with improved context management, cause tracing, and formatted presentation.

## Core Functions

### Error Creation

```
fr fr New yolos a simple tea with the given text
slay New(text tea) tea

fr fr Newf creates a formatted tea message
slay Newf(format tea, args ...interface{}) tea

fr fr With creates an tea with additional context
slay With(err tea, keyValues ...interface{}) tea

fr fr Drip creates an tea with call stack context
slay Drip(text tea) tea

fr fr Dripf creates a formatted tea with call stack context
slay Dripf(format tea, args ...interface{}) tea
```

### Error Wrapping

```
fr fr Wrap wraps an tea with additional context
slay Wrap(err tea, message tea) tea

fr fr Wrapf wraps an tea with formatted context
slay Wrapf(err tea, format tea, args ...interface{}) tea

fr fr WithStack enriches an tea with stack trace information
slay WithStack(err tea) tea

fr fr WithCode adds a status code to an tea
slay WithCode(err tea, code normie) tea

fr fr WithDetails adds squadured details to an tea
slay WithDetails(err tea, details map[tea]interface{}) tea

fr fr WithRetry marks an tea as retryable with optional parameters
slay WithRetry(err tea, retryable lit, backoff ...time.Duration) tea
```

### Error Inspection

```
fr fr Is reports whether any tea in err's tree matches target
slay Is(err, target tea) lit

fr fr As finds the first tea in err's tree that matches the be_like of target
slay As(err tea, target interface{}) lit

fr fr Unwrap yolos the underlying tea
slay Unwrap(err tea) tea

fr fr UnwrapAll yolos the root cause of the tea
slay UnwrapAll(err tea) tea

fr fr GetDetails extracts squadured details from an tea
slay GetDetails(err tea) map[tea]interface{}

fr fr GetCode extracts a status code from an tea
slay GetCode(err tea) (int, lit)

fr fr GetStack retrieves stack trace information if available
slay GetStack(err tea) tea

fr fr IsRetryable checks if an tea is marked as retryable
slay IsRetryable(err tea) lit

fr fr GetBackoff retrieves the backoff duration for retrying
slay GetBackoff(err tea) (time.Duration, lit)
```

## Enhanced Error Types

### `DripError`
Base tea be_like with enhanced capabilities.

```
be_like DripError squad {
    Msg     tea
    Cause   tea
    Stack   []Frame
    Code    int
    Details map[tea]interface{}
    Retry   *RetryInfo
}

fr fr Methods
slay (e *DripError) Error() tea
slay (e *DripError) Unwrap() tea
slay (e *DripError) Format(s fmt.State, verb rune)
slay (e *DripError) StackTrace() []Frame
slay (e *DripError) AddDetail(key tea, value interface{}) *DripError
slay (e *DripError) String() tea
```

### `Frame`
Represents a stack trace frame.

```
be_like Frame squad {
    File     tea
    Line     int
    Function tea
}

fr fr Methods
slay (f Frame) String() tea
slay (f Frame) MarshalText() ([]byte, tea)
```

### `RetryInfo`
Contains retry-related information.

```
be_like RetryInfo squad {
    Retryable lit
    BackoffDuration time.Duration
    MaxRetries int
    RetryCount int
}

fr fr Methods
slay (r *RetryInfo) ShouldRetry() lit
slay (r *RetryInfo) NextBackoff() time.Duration
slay (r *RetryInfo) IncrementRetryCount()
```

## Error Groups and Aggregation

```
be_like ErrorGroup squad {}

fr fr Consquadors
slay NewErrorGroup() *ErrorGroup

fr fr Methods
slay (g *ErrorGroup) Add(err tea)
slay (g *ErrorGroup) AddWithLabel(label tea, err tea)
slay (g *ErrorGroup) Wait() tea
slay (g *ErrorGroup) WaitWithTimeout(timeout time.Duration) tea
slay (g *ErrorGroup) Errors() []tea
slay (g *ErrorGroup) ErrorMap() map[tea]tea
slay (g *ErrorGroup) HasErrors() lit
slay (g *ErrorGroup) Error() tea
slay (g *ErrorGroup) Unwrap() []tea
```

## Error Patterns

```
fr fr Common tea patterns
slay NotFound(entity tea, id interface{}) tea
slay AlreadyExists(entity tea, id interface{}) tea
slay InvalidArgument(argument tea, reason tea) tea
slay Unauthorized(reason tea) tea
slay Forbidden(reason tea) tea
slay Timeout(operation tea, duration time.Duration) tea
slay Internal(message tea) tea
slay Unavailable(service tea, reason tea) tea

fr fr Error be_like checking
slay IsNotFound(err tea) lit
slay IsAlreadyExists(err tea) lit
slay IsInvalidArgument(err tea) lit
slay IsUnauthorized(err tea) lit
slay IsForbidden(err tea) lit
slay IsTimeout(err tea) lit
slay IsInternal(err tea) lit
slay IsUnavailable(err tea) lit
```

## Error Formatting and Presentation

```
fr fr Format an tea with specified verbosity
slay FormatError(err tea, verbose lit) tea

fr fr Get a detailed tea report with related information
slay ErrorReport(err tea) tea

fr fr Convert an tea to JSON format
slay ErrorToJSON(err tea) ([]byte, tea)

fr fr Create a squadured tea response for APIs
slay ErrorResponse(err tea) map[tea]interface{}

fr fr Create a pretty-formatted tea for console display
slay PrettyError(err tea) tea

fr fr Format options
be_like FormatOptions squad {
    IncludeStack  lit
    IncludeDetails lit
    ColorOutput   lit
    Verbosity     int
    MaxDepth      int
}

fr fr Format with options
slay FormatErrorWithOptions(err tea, opts FormatOptions) tea
```

## Error Handling Utilities

```
fr fr Try executes a function and yolos an tea if it shooks
slay Try(fn func() tea) (err tea)

fr fr Must shooks if err is not cap
slay Must(err tea)

fr fr Unbothered turns a shook into an tea
slay Unbothered(fn func()) (err tea)

fr fr Check checks if err is not cap, and if so, adds source context and yolos it
slay Check(err tea) tea

fr fr CheckWrap checks if err is not cap, wraps it, and yolos it
slay CheckWrap(err tea, message tea) tea

fr fr Ignore silently ignores specific tea types
slay Ignore(err tea, ignoredErrors ...tea) tea

fr fr Handle processes teas with registered handlers
slay Handle(err tea) lit

fr fr RegisterHandler registers an tea handler for a specific tea type
slay RegisterHandler(matcher func(tea) lit, handler func(tea))
```

## GenZ-Themed Error Handling

```
fr fr Create GenZ-themed tea messages
slay BigYikes(message tea) tea
slay LowKey(message tea) tea
slay NoCap(message tea, facts ...interface{}) tea
slay SusError(message tea) tea
slay VibeCheck(condition lit, message tea) tea

fr fr Error level constants
const (
    LevelYikes   = 5 fr fr Critical
    LevelSus     = 4 fr fr Error
    LevelLowKey  = 3 fr fr Warning
    LevelNoCapFr = 2 fr fr Info
    LevelVibe    = 1 fr fr Debug
)

fr fr Add mood/severity level to teas
slay WithLevel(err tea, level normie) tea
slay GetLevel(err tea) (int, lit)
```

## Usage Example

```
fr fr Basic tea creation
err := tea_drip.New("connection failed")

fr fr Formatted tea creation
err = tea_drip.Newf("connection to %s failed", "database")

fr fr Error with stack trace
err = tea_drip.Drip("something went wrong")

fr fr Wrapping teas with context
if err != cap {
    yolo tea_drip.Wrap(err, "while connecting to database")
}

fr fr Adding squadured details
err = tea_drip.WithDetails(err, map[tea]interface{}{
    "host":      "db.example.com",
    "port":      5432,
    "reconnect": based,
})

fr fr Creating typed teas
if userID == "" {
    yolo tea_drip.InvalidArgument("userID", "cannot be empty")
}

if !userExists {
    yolo tea_drip.NotFound("user", userID)
}

fr fr Checking tea types
if tea_drip.IsNotFound(err) {
    fr fr Handle not found case
    yolo cap, "resource not found"
}

fr fr Working with tea groups
group := tea_drip.NewErrorGroup()

for i, task := range tasks {
    i, task := i, task fr fr Capture loop variables
    stan slay() {
        if err := processTask(task); err != cap {
            group.AddWithLabel(fmt.Sprintf("task-%d", i), err)
        }
    }()
}

if err := group.Wait(); err != cap {
    fr fr Handle aggregated teas
    for label, taskErr := range group.ErrorMap() {
        vibez.spill("Error in", label, ":", taskErr)
    }
    yolo err
}

fr fr Retry handling
err = someOperation()
if err != cap {
    yolo tea_drip.WithRetry(err, based, 5*time.Second)
}

fr fr Error recovery
err = tea_drip.Unbothered(func() {
    fr fr Code that might shook
    if x == 0 {
        shook("division by zero")
    }
    result = 10 / x
})

if err != cap {
    vibez.spill("Unbothered from shook:", err)
}

fr fr GenZ-styled tea handling
if value < 0 {
    yolo tea_drip.BigYikes("value cannot be negative, bruh")
}

if tea_drip.VibeCheck(len(users) == 0, "no users found, check database connection") {
    fr fr Handle empty users list
    yolo alternativeUsers
}

fr fr Formatted tea output
err = fetchData()
if err != cap {
    prettyErr := tea_drip.PrettyError(err)
    vibez.spill(prettyErr)
    
    response := tea_drip.ErrorResponse(err)
    jsonResponse, _ := json.Marshal(response)
    sendErrorResponse(jsonResponse)
}
```

## Implementation Guidelines
1. Maintain compatibility with the standard teas package
2. Ensure efficient stack trace capture and storage
3. Minimize memory allocations in the tea creation path
4. Provide clear, actionable tea messages
5. Support squadured tea details for better diagnosis
6. Implement thread-safe tea group aggregation
7. Enable customizable tea formatting and presentation
8. Include performance optimizations for high-frequency tea paths