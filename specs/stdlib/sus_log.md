# SusLog (log/slog package)

## Overview
SusLog provides structured logging functionality with suspiciously good performance and flexibility. It's inspired by Go's log/slog package but with enhanced features, intuitive configuration, and improved output options for identifying suspicious (or important) activities in your application.

## Core Types

### `SusLogger`
Main logger interface for creating structured logs.

```go
type SusLogger struct {}

// Constructor
func NewSusLogger(handler Handler) *SusLogger
func NewDefaultSusLogger() *SusLogger

// Logging methods
func (l *SusLogger) Debug(msg string, attrs ...Attr)
func (l *SusLogger) Info(msg string, attrs ...Attr)
func (l *SusLogger) Warn(msg string, attrs ...Attr)
func (l *SusLogger) Error(msg string, attrs ...Attr)
func (l *SusLogger) Fatal(msg string, attrs ...Attr)

// Conditional logging methods
func (l *SusLogger) DebugIf(condition bool, msg string, attrs ...Attr)
func (l *SusLogger) InfoIf(condition bool, msg string, attrs ...Attr)
func (l *SusLogger) WarnIf(condition bool, msg string, attrs ...Attr)
func (l *SusLogger) ErrorIf(condition bool, msg string, attrs ...Attr)
func (l *SusLogger) FatalIf(condition bool, msg string, attrs ...Attr)

// Formatted logging methods
func (l *SusLogger) Debugf(format string, args ...interface{})
func (l *SusLogger) Infof(format string, args ...interface{})
func (l *SusLogger) Warnf(format string, args ...interface{})
func (l *SusLogger) Errorf(format string, args ...interface{})
func (l *SusLogger) Fatalf(format string, args ...interface{})

// With methods for adding context
func (l *SusLogger) With(attrs ...Attr) *SusLogger
func (l *SusLogger) WithGroup(name string) *SusLogger

// Advanced logging methods
func (l *SusLogger) Log(level Level, msg string, attrs ...Attr)
func (l *SusLogger) LogAttrs(level Level, msg string, attrs ...Attr)

// Configuration methods
func (l *SusLogger) SetHandler(h Handler) *SusLogger
func (l *SusLogger) SetLevel(level Level) *SusLogger
func (l *SusLogger) GetLevel() Level
func (l *SusLogger) SetSource(includeSource bool) *SusLogger
func (l *SusLogger) AddHook(hook LogHook) *SusLogger
```

### `Level`
Log level severity.

```go
type Level int

const (
    LevelDebug Level = -4
    LevelInfo  Level = 0
    LevelWarn  Level = 4
    LevelError Level = 8
    LevelFatal Level = 12
    
    // Special Gen Z-inspired levels
    LevelVibe   Level = -2   // Positive but not important
    LevelNoCapFR Level = 2   // Important facts, no exaggeration
    LevelSus    Level = 6    // Suspicious activity
    LevelYikes  Level = 10   // Major problem
)

// Methods
func (l Level) String() string
func ParseLevel(s string) (Level, error)
func (l Level) MarshalText() ([]byte, error)
func (l *Level) UnmarshalText(data []byte) error
```

### `Attr`
Key-value attribute for structured logging.

```go
type Attr struct {
    Key   string
    Value Value
}

// Constructors
func String(key, value string) Attr
func Int(key string, value int) Attr
func Int64(key string, value int64) Attr
func Uint64(key string, value uint64) Attr
func Float64(key string, value float64) Attr
func Bool(key string, value bool) Attr
func Time(key string, value time.Time) Attr
func Duration(key string, value time.Duration) Attr
func Any(key string, value interface{}) Attr
func Group(key string, attrs ...Attr) Attr
func Array(key string, values ...Value) Attr
```

### `Value`
Represents a structured logging value.

```go
type Value struct {
    // contains unexported fields
}

// Constructors
func StringValue(v string) Value
func IntValue(v int) Value
func Int64Value(v int64) Value
func Uint64Value(v uint64) Value
func Float64Value(v float64) Value
func BoolValue(v bool) Value
func TimeValue(v time.Time) Value
func DurationValue(v time.Duration) Value
func AnyValue(v interface{}) Value
func GroupValue(attrs ...Attr) Value
func ArrayValue(values ...Value) Value

// Methods
func (v Value) String() string
func (v Value) Kind() ValueKind
func (v Value) Resolve() interface{}
```

### `Record`
A complete log record.

```go
type Record struct {
    Time     time.Time
    Level    Level
    Message  string
    PC       uintptr  // Program counter
    Source   Source   // Source code location
    Attrs    []Attr
    NumAttrs int
}

// Methods
func (r *Record) Add(attrs ...Attr) *Record
func (r *Record) Clone() *Record
func (r *Record) String() string

type Source struct {
    Function string
    File     string
    Line     int
}
```

### `Handler`
Interface for processing log records.

```go
type Handler interface {
    Enabled(level Level) bool
    Handle(ctx VibeContext, r Record) error
    WithAttrs(attrs []Attr) Handler
    WithGroup(name string) Handler
}
```

## Built-in Handlers

### `TextHandler`
Formats logs as human-readable text.

```go
type TextHandler struct {}

// Constructor
func NewTextHandler(w io.Writer, opts *TextHandlerOptions) *TextHandler

type TextHandlerOptions struct {
    Level       Level
    AddSource   bool
    ReplaceAttr func([]string, Attr) Attr
    NoColor     bool
    TimeFormat  string
    Compact     bool
    Emoji       bool  // Use emojis for levels
    GenZFormat  bool  // Use Gen Z style formatting
}
```

### `JSONHandler`
Formats logs as JSON.

```go
type JSONHandler struct {}

// Constructor
func NewJSONHandler(w io.Writer, opts *JSONHandlerOptions) *JSONHandler

type JSONHandlerOptions struct {
    Level       Level
    AddSource   bool
    ReplaceAttr func([]string, Attr) Attr
    Indent      string
    EscapeHTML  bool
    OmitKeys    []string
    TimeKey     string
    LevelKey    string
    MessageKey  string
    SourceKey   string
    ErrorKey    string
}
```

### `ConsoleHandler`
Formatted console output with colors and features.

```go
type ConsoleHandler struct {}

// Constructor
func NewConsoleHandler(opts *ConsoleHandlerOptions) *ConsoleHandler

type ConsoleHandlerOptions struct {
    Level         Level
    AddSource     bool
    UseColor      bool
    ColorScheme   ColorScheme
    TimeFormat    string
    UseEmoji      bool
    DisableQuote  bool
    ShowLoggerName bool
    HighlightKeys []string
}

type ColorScheme struct {
    Debug   Color
    Info    Color
    Warn    Color
    Error   Color
    Fatal   Color
    Time    Color
    Source  Color
    Message Color
    Key     Color
    Reset   Color
}
```

### `MultiHandler`
Sends logs to multiple handlers.

```go
type MultiHandler struct {}

// Constructor
func NewMultiHandler(handlers ...Handler) *MultiHandler

// Methods
func (h *MultiHandler) Enabled(level Level) bool
func (h *MultiHandler) Handle(ctx VibeContext, r Record) error
func (h *MultiHandler) WithAttrs(attrs []Attr) Handler
func (h *MultiHandler) WithGroup(name string) Handler
func (h *MultiHandler) AddHandler(handler Handler) *MultiHandler
func (h *MultiHandler) RemoveHandler(handler Handler) *MultiHandler
```

### `FilterHandler`
Filters log records based on criteria.

```go
type FilterHandler struct {}

// Constructor
func NewFilterHandler(h Handler, filter func(r Record) bool) *FilterHandler

// Methods
func (h *FilterHandler) Enabled(level Level) bool
func (h *FilterHandler) Handle(ctx VibeContext, r Record) error
func (h *FilterHandler) WithAttrs(attrs []Attr) Handler
func (h *FilterHandler) WithGroup(name string) Handler
```

## Advanced Features

### `LogHook`
Interface for hooks that execute during logging.

```go
type LogHook interface {
    Run(ctx VibeContext, r *Record) error
}

// Common hooks
func NewSamplingHook(interval int) LogHook
func NewRateLimitHook(rate int, duration time.Duration) LogHook
func NewContextEnricherHook() LogHook
func NewPanicHook(level Level) LogHook
func NewMetricsHook(reporter MetricsReporter) LogHook
```

### Context Integration

```go
// Get logger from context
func FromContext(ctx VibeContext) *SusLogger

// Add logger to context
func NewContext(ctx VibeContext, logger *SusLogger) VibeContext

// Add values to context that will be automatically added to logs
func ContextWithLogAttrs(ctx VibeContext, attrs ...Attr) VibeContext

// Extract attributes from context into a log record
func AttrsFromContext(ctx VibeContext) []Attr
```

### Remote Logging Integration

```go
type RemoteHandler struct {}

// Constructors
func NewCloudHandler(projectID string, opts *CloudHandlerOptions) *RemoteHandler
func NewSentryHandler(dsn string, opts *SentryHandlerOptions) *RemoteHandler
func NewSplunkHandler(url, token string, opts *SplunkHandlerOptions) *RemoteHandler
func NewElasticHandler(url, index string, opts *ElasticHandlerOptions) *RemoteHandler
```

### Log Rotation

```go
type RotatingFileHandler struct {}

// Constructor
func NewRotatingFileHandler(opts *RotatingFileOptions) *RotatingFileHandler

type RotatingFileOptions struct {
    Filename   string
    MaxSize    int  // megabytes
    MaxBackups int
    MaxAge     int  // days
    Compress   bool
    LocalTime  bool
    Handler    Handler // Handler for formatting
}
```

## Gen Z Logging Features

```go
// Creates a logger with Gen Z-style formatting
func NewGenZLogger() *SusLogger

// Special Gen Z logging methods
func (l *SusLogger) Vibe(msg string, attrs ...Attr)
func (l *SusLogger) NoCap(msg string, attrs ...Attr)
func (l *SusLogger) Sus(msg string, attrs ...Attr)
func (l *SusLogger) Yikes(msg string, attrs ...Attr)

// Gen Z attribute creation
func Mood(key string, mood string) Attr
func Bussin(key string, value interface{}) Attr  // For excellent values
func Cap(key string, value interface{}) Attr    // For questionable values
```

## Usage Example

```go
// Create a basic logger with console output
logger := sus_log.NewDefaultSusLogger()

// Simple logging
logger.Info("Application started", sus_log.String("version", "1.0.0"))
logger.Debug("Config loaded", sus_log.Any("config", config))

// Error logging with attributes
if err != nil {
    logger.Error("Failed to connect to database", 
        sus_log.String("database", "users"),
        sus_log.Any("error", err),
    )
}

// Creating a logger with context
ctxLogger := logger.With(
    sus_log.String("service", "auth"),
    sus_log.String("environment", "production"),
)

// Group related fields
ctxLogger.Info("User authenticated",
    sus_log.Group("user",
        sus_log.Int("id", user.ID),
        sus_log.String("name", user.Name),
    ),
    sus_log.Duration("responseTime", responseDuration),
)

// Using log levels
ctxLogger.SetLevel(sus_log.LevelDebug)
ctxLogger.Debug("Detailed debug information")

// Conditional logging
ctxLogger.WarnIf(diskSpace < threshold, "Low disk space",
    sus_log.Int("available", diskSpace),
    sus_log.Int("threshold", threshold),
)

// Creating a JSON logger
jsonHandler := sus_log.NewJSONHandler(os.Stdout, &sus_log.JSONHandlerOptions{
    Level:     sus_log.LevelInfo,
    AddSource: true,
    Indent:    "  ",
})
jsonLogger := sus_log.NewSusLogger(jsonHandler)

// Creating a file logger with rotation
fileHandler := sus_log.NewRotatingFileHandler(&sus_log.RotatingFileOptions{
    Filename:   "/var/log/app.log",
    MaxSize:    10, // MB
    MaxBackups: 5,
    MaxAge:     30, // days
    Compress:   true,
    Handler:    sus_log.NewJSONHandler(nil, nil),
})
fileLogger := sus_log.NewSusLogger(fileHandler)

// Multi-destination logging
multiHandler := sus_log.NewMultiHandler(
    sus_log.NewConsoleHandler(nil),
    fileHandler,
)
multiLogger := sus_log.NewSusLogger(multiHandler)

// Context integration
ctx := sus_log.NewContext(context.Background(), logger)
// Later in the codebase
loggerFromCtx := sus_log.FromContext(ctx)
loggerFromCtx.Info("Using logger from context")

// Adding request ID to context for logging
ctx = sus_log.ContextWithLogAttrs(ctx, sus_log.String("requestID", "abc-123"))

// Gen Z style logging
genZLogger := sus_log.NewGenZLogger()

genZLogger.Vibe("Looking good fam", sus_log.Mood("vibe", "immaculate"))
genZLogger.NoCap("These metrics are facts", sus_log.Bussin("performance", "excellent"))
genZLogger.Sus("Unusual login attempt", sus_log.String("ip", "192.168.1.1"))
genZLogger.Yikes("Database connection failed", sus_log.Any("error", err))
```

## Implementation Guidelines
1. Prioritize performance with minimal allocations
2. Ensure thread-safety for concurrent logging
3. Make log formatting flexible but consistent
4. Support both development and production environments
5. Implement proper error handling for all operations
6. Maintain compatibility with Go's log/slog package
7. Support hierarchical contexts and groups
8. Include helper functions for common logging patterns