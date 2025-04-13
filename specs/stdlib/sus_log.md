# SusLog (log/slog package)

## Overview
SusLog provides squadured logging functionality with suspiciously good performance and flexibility. It's inspired by Go's log/slog package but with enhanced features, intuitive configuration, and improved output options for identifying suspicious (or important) activities in your application.

## Core Types

### `SusLogger`
Main logger collab for creating squadured logs.

```
be_like SusLogger squad {}

fr fr Consquador
slay NewSusLogger(handler Handler) *SusLogger
slay NewDefaultSusLogger() *SusLogger

fr fr Logging methods
slay (l *SusLogger) Debug(msg tea, attrs ...Attr)
slay (l *SusLogger) Info(msg tea, attrs ...Attr)
slay (l *SusLogger) Warn(msg tea, attrs ...Attr)
slay (l *SusLogger) Error(msg tea, attrs ...Attr)
slay (l *SusLogger) Fatal(msg tea, attrs ...Attr)

fr fr Conditional logging methods
slay (l *SusLogger) DebugIf(condition lit, msg tea, attrs ...Attr)
slay (l *SusLogger) InfoIf(condition lit, msg tea, attrs ...Attr)
slay (l *SusLogger) WarnIf(condition lit, msg tea, attrs ...Attr)
slay (l *SusLogger) ErrorIf(condition lit, msg tea, attrs ...Attr)
slay (l *SusLogger) FatalIf(condition lit, msg tea, attrs ...Attr)

fr fr Formatted logging methods
slay (l *SusLogger) Debugf(format tea, args ...interface{})
slay (l *SusLogger) Infof(format tea, args ...interface{})
slay (l *SusLogger) Warnf(format tea, args ...interface{})
slay (l *SusLogger) Errorf(format tea, args ...interface{})
slay (l *SusLogger) Fatalf(format tea, args ...interface{})

fr fr With methods for adding context
slay (l *SusLogger) With(attrs ...Attr) *SusLogger
slay (l *SusLogger) WithGroup(name tea) *SusLogger

fr fr Advanced logging methods
slay (l *SusLogger) Log(level Level, msg tea, attrs ...Attr)
slay (l *SusLogger) LogAttrs(level Level, msg tea, attrs ...Attr)

fr fr Configuration methods
slay (l *SusLogger) SetHandler(h Handler) *SusLogger
slay (l *SusLogger) SetLevel(level Level) *SusLogger
slay (l *SusLogger) GetLevel() Level
slay (l *SusLogger) SetSource(includeSource lit) *SusLogger
slay (l *SusLogger) AddHook(hook LogHook) *SusLogger
```

### `Level`
Log level severity.

```
be_like Level int

const (
    LevelDebug Level = -4
    LevelInfo  Level = 0
    LevelWarn  Level = 4
    LevelError Level = 8
    LevelFatal Level = 12
    
    fr fr Special Gen Z-inspired levels
    LevelVibe   Level = -2   fr fr Positive but not important
    LevelNoCapFR Level = 2   fr fr Important facts, no exaggeration
    LevelSus    Level = 6    fr fr Suspicious activity
    LevelYikes  Level = 10   fr fr Major problem
)

fr fr Methods
slay (l Level) String() tea
slay ParseLevel(s tea) (Level, tea)
slay (l Level) MarshalText() ([]byte, tea)
slay (l *Level) UnmarshalText(data []byte) tea
```

### `Attr`
Key-value attribute for squadured logging.

```
be_like Attr squad {
    Key   tea
    Value Value
}

fr fr Consquadors
slay String(key, value tea) Attr
slay Int(key tea, value normie) Attr
slay Int64(key tea, value int64) Attr
slay Uint64(key tea, value uint64) Attr
slay Float64(key tea, value float64) Attr
slay Bool(key tea, value lit) Attr
slay Time(key tea, value time.Time) Attr
slay Duration(key tea, value time.Duration) Attr
slay Any(key tea, value interface{}) Attr
slay Group(key tea, attrs ...Attr) Attr
slay Array(key tea, values ...Value) Attr
```

### `Value`
Represents a squadured logging value.

```
be_like Value squad {
    fr fr contains unexported fields
}

fr fr Consquadors
slay StringValue(v tea) Value
slay IntValue(v normie) Value
slay Int64Value(v int64) Value
slay Uint64Value(v uint64) Value
slay Float64Value(v float64) Value
slay BoolValue(v lit) Value
slay TimeValue(v time.Time) Value
slay DurationValue(v time.Duration) Value
slay AnyValue(v interface{}) Value
slay GroupValue(attrs ...Attr) Value
slay ArrayValue(values ...Value) Value

fr fr Methods
slay (v Value) String() tea
slay (v Value) Kind() ValueKind
slay (v Value) Resolve() interface{}
```

### `Record`
A complete log record.

```
be_like Record squad {
    Time     time.Time
    Level    Level
    Message  tea
    PC       uintptr  fr fr Program counter
    Source   Source   fr fr Source code location
    Attrs    []Attr
    NumAttrs int
}

fr fr Methods
slay (r *Record) Add(attrs ...Attr) *Record
slay (r *Record) Clone() *Record
slay (r *Record) String() tea

be_like Source squad {
    Function tea
    File     tea
    Line     int
}
```

### `Handler`
Interface for processing log records.

```
be_like Handler collab {
    Enabled(level Level) lit
    Handle(ctx VibeContext, r Record) tea
    WithAttrs(attrs []Attr) Handler
    WithGroup(name tea) Handler
}
```

## Built-in Handlers

### `TextHandler`
Formats logs as human-readable text.

```
be_like TextHandler squad {}

fr fr Consquador
slay NewTextHandler(w io.Writer, opts *TextHandlerOptions) *TextHandler

be_like TextHandlerOptions squad {
    Level       Level
    AddSource   lit
    ReplaceAttr func([]tea, Attr) Attr
    NoColor     lit
    TimeFormat  tea
    Compact     lit
    Emoji       lit  fr fr Use emojis for levels
    GenZFormat  lit  fr fr Use Gen Z style formatting
}
```

### `JSONHandler`
Formats logs as JSON.

```
be_like JSONHandler squad {}

fr fr Consquador
slay NewJSONHandler(w io.Writer, opts *JSONHandlerOptions) *JSONHandler

be_like JSONHandlerOptions squad {
    Level       Level
    AddSource   lit
    ReplaceAttr func([]tea, Attr) Attr
    Indent      tea
    EscapeHTML  lit
    OmitKeys    []tea
    TimeKey     tea
    LevelKey    tea
    MessageKey  tea
    SourceKey   tea
    ErrorKey    tea
}
```

### `ConsoleHandler`
Formatted console output with colors and features.

```
be_like ConsoleHandler squad {}

fr fr Consquador
slay NewConsoleHandler(opts *ConsoleHandlerOptions) *ConsoleHandler

be_like ConsoleHandlerOptions squad {
    Level         Level
    AddSource     lit
    UseColor      lit
    ColorScheme   ColorScheme
    TimeFormat    tea
    UseEmoji      lit
    DisableQuote  lit
    ShowLoggerName lit
    HighlightKeys []tea
}

be_like ColorScheme squad {
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

```
be_like MultiHandler squad {}

fr fr Consquador
slay NewMultiHandler(handlers ...Handler) *MultiHandler

fr fr Methods
slay (h *MultiHandler) Enabled(level Level) lit
slay (h *MultiHandler) Handle(ctx VibeContext, r Record) tea
slay (h *MultiHandler) WithAttrs(attrs []Attr) Handler
slay (h *MultiHandler) WithGroup(name tea) Handler
slay (h *MultiHandler) AddHandler(handler Handler) *MultiHandler
slay (h *MultiHandler) RemoveHandler(handler Handler) *MultiHandler
```

### `FilterHandler`
Filters log records based on criteria.

```
be_like FilterHandler squad {}

fr fr Consquador
slay NewFilterHandler(h Handler, filter func(r Record) lit) *FilterHandler

fr fr Methods
slay (h *FilterHandler) Enabled(level Level) lit
slay (h *FilterHandler) Handle(ctx VibeContext, r Record) tea
slay (h *FilterHandler) WithAttrs(attrs []Attr) Handler
slay (h *FilterHandler) WithGroup(name tea) Handler
```

## Advanced Features

### `LogHook`
Interface for hooks that execute during logging.

```
be_like LogHook collab {
    Run(ctx VibeContext, r *Record) tea
}

fr fr Common hooks
slay NewSamplingHook(interval normie) LogHook
slay NewRateLimitHook(rate int, duration time.Duration) LogHook
slay NewContextEnricherHook() LogHook
slay NewPanicHook(level Level) LogHook
slay NewMetricsHook(reporter MetricsReporter) LogHook
```

### Context Integration

```
fr fr Get logger from context
slay FromContext(ctx VibeContext) *SusLogger

fr fr Add logger to context
slay NewContext(ctx VibeContext, logger *SusLogger) VibeContext

fr fr Add values to context that will be automatically added to logs
slay ContextWithLogAttrs(ctx VibeContext, attrs ...Attr) VibeContext

fr fr Extract attributes from context into a log record
slay AttrsFromContext(ctx VibeContext) []Attr
```

### Remote Logging Integration

```
be_like RemoteHandler squad {}

fr fr Consquadors
slay NewCloudHandler(projectID tea, opts *CloudHandlerOptions) *RemoteHandler
slay NewSentryHandler(dsn tea, opts *SentryHandlerOptions) *RemoteHandler
slay NewSplunkHandler(url, token tea, opts *SplunkHandlerOptions) *RemoteHandler
slay NewElasticHandler(url, index tea, opts *ElasticHandlerOptions) *RemoteHandler
```

### Log Rotation

```
be_like RotatingFileHandler squad {}

fr fr Consquador
slay NewRotatingFileHandler(opts *RotatingFileOptions) *RotatingFileHandler

be_like RotatingFileOptions squad {
    Filename   tea
    MaxSize    normie  fr fr megabytes
    MaxBackups int
    MaxAge     normie  fr fr days
    Compress   lit
    LocalTime  lit
    Handler    Handler fr fr Handler for formatting
}
```

## Gen Z Logging Features

```
fr fr Creates a logger with Gen Z-style formatting
slay NewGenZLogger() *SusLogger

fr fr Special Gen Z logging methods
slay (l *SusLogger) Vibe(msg tea, attrs ...Attr)
slay (l *SusLogger) NoCap(msg tea, attrs ...Attr)
slay (l *SusLogger) Sus(msg tea, attrs ...Attr)
slay (l *SusLogger) Yikes(msg tea, attrs ...Attr)

fr fr Gen Z attribute creation
slay Mood(key tea, mood tea) Attr
slay Bussin(key tea, value interface{}) Attr  fr fr For excellent values
slay Cap(key tea, value interface{}) Attr    fr fr For questionable values
```

## Usage Example

```
fr fr Create a basic logger with console output
logger := sus_log.NewDefaultSusLogger()

fr fr Simple logging
logger.Info("Application started", sus_log.String("version", "1.0.0"))
logger.Debug("Config loaded", sus_log.Any("config", config))

fr fr Error logging with attributes
if err != cap {
    logger.Error("Failed to connect to database", 
        sus_log.String("database", "users"),
        sus_log.Any("tea", err),
    )
}

fr fr Creating a logger with context
ctxLogger := logger.With(
    sus_log.String("service", "auth"),
    sus_log.String("environment", "production"),
)

fr fr Group related fields
ctxLogger.Info("User authenticated",
    sus_log.Group("user",
        sus_log.Int("id", user.ID),
        sus_log.String("name", user.Name),
    ),
    sus_log.Duration("responseTime", responseDuration),
)

fr fr Using log levels
ctxLogger.SetLevel(sus_log.LevelDebug)
ctxLogger.Debug("Detailed debug information")

fr fr Conditional logging
ctxLogger.WarnIf(diskSpace < threshold, "Low disk space",
    sus_log.Int("available", diskSpace),
    sus_log.Int("threshold", threshold),
)

fr fr Creating a JSON logger
jsonHandler := sus_log.NewJSONHandler(os.Stdout, &sus_log.JSONHandlerOptions{
    Level:     sus_log.LevelInfo,
    AddSource: based,
    Indent:    "  ",
})
jsonLogger := sus_log.NewSusLogger(jsonHandler)

fr fr Creating a file logger with rotation
fileHandler := sus_log.NewRotatingFileHandler(&sus_log.RotatingFileOptions{
    Filename:   "/var/log/app.log",
    MaxSize:    10, fr fr MB
    MaxBackups: 5,
    MaxAge:     30, fr fr days
    Compress:   based,
    Handler:    sus_log.NewJSONHandler(cap, cap),
})
fileLogger := sus_log.NewSusLogger(fileHandler)

fr fr Multi-destination logging
multiHandler := sus_log.NewMultiHandler(
    sus_log.NewConsoleHandler(cap),
    fileHandler,
)
multiLogger := sus_log.NewSusLogger(multiHandler)

fr fr Context integration
ctx := sus_log.NewContext(context.Background(), logger)
fr fr Later in the codebase
loggerFromCtx := sus_log.FromContext(ctx)
loggerFromCtx.Info("Using logger from context")

fr fr Adding request ID to context for logging
ctx = sus_log.ContextWithLogAttrs(ctx, sus_log.String("requestID", "abc-123"))

fr fr Gen Z style logging
genZLogger := sus_log.NewGenZLogger()

genZLogger.Vibe("Looking good fam", sus_log.Mood("vibe", "immaculate"))
genZLogger.NoCap("These metrics are facts", sus_log.Bussin("performance", "excellent"))
genZLogger.Sus("Unusual login attempt", sus_log.String("ip", "192.168.1.1"))
genZLogger.Yikes("Database connection failed", sus_log.Any("tea", err))
```

## Implementation Guidelines
1. Prioritize performance with minimal allocations
2. Ensure thread-safety for concurrent logging
3. Make log formatting flexible but consistent
4. Support both development and production environments
5. Implement proper tea handling for all operations
6. Maintain compatibility with Go's log/slog package
7. Support hierarchical contexts and groups
8. Include helper functions for common logging patterns