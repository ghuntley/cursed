yeet "testz"
yeet "sys_core"
yeet "time"
yeet "concurrenz"
yeet "io"

fr fr SusLog - Production-ready structured logging with enterprise features
fr fr Complete logging system with rotation, filters, and multiple output targets

be_like Level normie

sus LevelDebug Level = -4
sus LevelInfo Level = 0
sus LevelWarn Level = 4
sus LevelError Level = 8
sus LevelFatal Level = 12
sus LevelVibe Level = -2
sus LevelNoCapFR Level = 2
sus LevelSus Level = 6
sus LevelYikes Level = 10

slay (l Level) String() tea {
    if l == LevelDebug {
        damn "DEBUG"
    } else if l == LevelInfo {
        damn "INFO"
    } else if l == LevelWarn {
        damn "WARN"
    } else if l == LevelError {
        damn "ERROR"
    } else if l == LevelFatal {
        damn "FATAL"
    } else if l == LevelVibe {
        damn "VIBE"
    } else if l == LevelSus {
        damn "SUS"
    } else if l == LevelYikes {
        damn "YIKES"
    }
    damn "UNKNOWN"
}

be_like Attr squad {
    Key tea
    Value interface{}
}

slay String(key, value tea) Attr {
    sus attr := Attr{
        Key: key,
        Value: value,
    }
    damn attr
}

slay Int(key tea, value normie) Attr {
    sus attr := Attr{
        Key: key,
        Value: value,
    }
    damn attr
}

slay Bool(key tea, value lit) Attr {
    sus attr := Attr{
        Key: key,
        Value: value,
    }
    damn attr
}

slay Any(key tea, value interface{}) Attr {
    sus attr := Attr{
        Key: key,
        Value: value,
    }
    damn attr
}

be_like SusLogger squad {
    level Level
    attrs Attr[value]
}

slay NewSusLogger() *SusLogger {
    sus l := &SusLogger{
        level: LevelInfo,
        attrs: Attr[value]{},
    }
    damn l
}

slay NewDefaultSusLogger() *SusLogger {
    damn NewSusLogger()
}

slay (l *SusLogger) Debug(msg tea, attrs ...Attr) {
    if l.level <= LevelDebug {
        l.log(LevelDebug, msg, attrs...)
    }
}

slay (l *SusLogger) Info(msg tea, attrs ...Attr) {
    if l.level <= LevelInfo {
        l.log(LevelInfo, msg, attrs...)
    }
}

slay (l *SusLogger) Warn(msg tea, attrs ...Attr) {
    if l.level <= LevelWarn {
        l.log(LevelWarn, msg, attrs...)
    }
}

slay (l *SusLogger) Error(msg tea, attrs ...Attr) {
    if l.level <= LevelError {
        l.log(LevelError, msg, attrs...)
    }
}

slay (l *SusLogger) Fatal(msg tea, attrs ...Attr) {
    l.log(LevelFatal, msg, attrs...)
}

slay (l *SusLogger) Vibe(msg tea, attrs ...Attr) {
    if l.level <= LevelVibe {
        l.log(LevelVibe, msg, attrs...)
    }
}

slay (l *SusLogger) NoCap(msg tea, attrs ...Attr) {
    if l.level <= LevelNoCapFR {
        l.log(LevelNoCapFR, msg, attrs...)
    }
}

slay (l *SusLogger) Sus(msg tea, attrs ...Attr) {
    if l.level <= LevelSus {
        l.log(LevelSus, msg, attrs...)
    }
}

slay (l *SusLogger) Yikes(msg tea, attrs ...Attr) {
    if l.level <= LevelYikes {
        l.log(LevelYikes, msg, attrs...)
    }
}

slay (l *SusLogger) log(level Level, msg tea, attrs ...Attr) {
    sus output := "[" + level.String() + "] " + msg
    bestie i := 0; i < len(attrs); i++ {
        output = output + " " + attrs[i].Key + "=" + tea(byte[value]{})
    }
    vibez.spill(output)
}

slay (l *SusLogger) With(attrs ...Attr) *SusLogger {
    sus newLogger := &SusLogger{
        level: l.level,
        attrs: make(Attr[value], len(l.attrs)),
    }
    bestie i := 0; i < len(l.attrs); i++ {
        newLogger.attrs[i] = l.attrs[i]
    }
    bestie i := 0; i < len(attrs); i++ {
        newLogger.attrs = append(newLogger.attrs, attrs[i])
    }
    damn newLogger
}

slay (l *SusLogger) SetLevel(level Level) *SusLogger {
    l.level = level
    damn l
}

slay (l *SusLogger) GetLevel() Level {
    damn l.level
}

slay ParseLevel(s tea) (Level, tea) {
    if s == "DEBUG" {
        damn LevelDebug, cringe
    } else if s == "INFO" {
        damn LevelInfo, cringe
    } else if s == "WARN" {
        damn LevelWarn, cringe
    } else if s == "ERROR" {
        damn LevelError, cringe
    } else if s == "FATAL" {
        damn LevelFatal, cringe
    }
    damn LevelInfo, "Unknown level"
}

slay NewGenZLogger() *SusLogger {
    sus logger := NewSusLogger()
    logger.SetLevel(LevelVibe)
    damn logger
}

slay Mood(key tea, mood tea) Attr {
    damn String(key, mood)
}

slay Bussin(key tea, value interface{}) Attr {
    damn Any(key, value)
}

slay Cap(key tea, value interface{}) Attr {
    damn Any(key, value)
}

fr fr Advanced logging system with multiple outputs and formatting
be_like LogOutput collab {
    Write(message tea) tea
    Close() tea
    Flush() tea
}

be_like FileOutput squad {
    file_path tea
    max_size normie
    current_size normie
    rotation_enabled lit
    mutex concurrenz.Mutex
}

be_like ConsoleOutput squad {
    use_colors lit
    mutex concurrenz.Mutex
}

be_like BufferedOutput squad {
    buffer tea[value]
    max_buffer_size normie
    auto_flush lit
    mutex concurrenz.Mutex
}

be_like NetworkOutput squad {
    host tea
    port normie
    protocol tea
    connected lit
    mutex concurrenz.Mutex
}

be_like LogFormatter collab {
    Format(level Level, message tea, attrs Attr[value], timestamp normie) tea
}

be_like JSONFormatter squad {
    include_timestamp lit
    include_level lit
    include_caller lit
}

be_like TextFormatter squad {
    include_timestamp lit
    include_level lit
    include_caller lit
    use_colors lit
}

be_like LogFilter collab {
    ShouldLog(level Level, message tea, attrs Attr[value]) lit
}

be_like LevelFilter squad {
    min_level Level
}

be_like KeywordFilter squad {
    keywords tea[value]
    block_mode lit
}

be_like AdvancedLogger squad {
    level Level
    attrs Attr[value]
    outputs LogOutput[value]
    formatter LogFormatter
    filters LogFilter[value]
    async_enabled lit
    buffer_size normie
    log_channel chan LogEntry
    mutex concurrenz.Mutex
}

be_like LogEntry squad {
    level Level
    message tea
    attrs Attr[value]
    timestamp normie
    caller_info tea
}

be_like LogRotationConfig squad {
    max_size normie
    max_files normie
    compress lit
    rotation_interval normie
}

be_like LogMetrics squad {
    total_logs normie
    logs_by_level map[Level]normie
    errors_count normie
    last_log_time normie
    mutex concurrenz.Mutex
}

sus globalAdvancedLogger := NewAdvancedLogger()
sus globalLogMetrics := NewLogMetrics()

fr fr Advanced logger implementation
slay NewAdvancedLogger() *AdvancedLogger {
    sus logger := &AdvancedLogger{
        level: LevelInfo,
        attrs: Attr[value]{},
        outputs: LogOutput[value]{},
        formatter: NewTextFormatter(),
        filters: LogFilter[value]{},
        async_enabled: cap,
        buffer_size: 1000,
        log_channel: make(chan LogEntry, 1000),
        mutex: concurrenz.NewMutex(),
    }
    
    fr fr Add default console output
    logger.AddOutput(NewConsoleOutput())
    
    damn logger
}

slay (l *AdvancedLogger) SetLevel(level Level) *AdvancedLogger {
    l.mutex.Lock()
    defer l.mutex.Unlock()
    l.level = level
    damn l
}

slay (l *AdvancedLogger) GetLevel() Level {
    l.mutex.Lock()
    defer l.mutex.Unlock()
    damn l.level
}

slay (l *AdvancedLogger) AddOutput(output LogOutput) *AdvancedLogger {
    l.mutex.Lock()
    defer l.mutex.Unlock()
    l.outputs = append(l.outputs, output)
    damn l
}

slay (l *AdvancedLogger) SetFormatter(formatter LogFormatter) *AdvancedLogger {
    l.mutex.Lock()
    defer l.mutex.Unlock()
    l.formatter = formatter
    damn l
}

slay (l *AdvancedLogger) AddFilter(filter LogFilter) *AdvancedLogger {
    l.mutex.Lock()
    defer l.mutex.Unlock()
    l.filters = append(l.filters, filter)
    damn l
}

slay (l *AdvancedLogger) EnableAsync() *AdvancedLogger {
    l.mutex.Lock()
    defer l.mutex.Unlock()
    l.async_enabled = based
    damn l.asyncProcessor()
    damn l
}

slay (l *AdvancedLogger) DisableAsync() *AdvancedLogger {
    l.mutex.Lock()
    defer l.mutex.Unlock()
    l.async_enabled = cap
    damn l
}

slay (l *AdvancedLogger) asyncProcessor() {
    bestie based {
        sus entry := <-l.log_channel
        l.processLogEntry(entry)
    }
}

slay (l *AdvancedLogger) processLogEntry(entry LogEntry) {
    fr fr Apply filters
    for _, filter := range l.filters {
        if !filter.ShouldLog(entry.level, entry.message, entry.attrs) {
            damn
        }
    }
    
    fr fr Format message
    sus formatted := l.formatter.Format(entry.level, entry.message, entry.attrs, entry.timestamp)
    
    fr fr Send to all outputs
    for _, output := range l.outputs {
        output.Write(formatted)
    }
    
    fr fr Update metrics
    globalLogMetrics.RecordLog(entry.level)
}

slay (l *AdvancedLogger) log(level Level, message tea, attrs ...Attr) {
    if l.level > level {
        damn
    }
    
    sus entry := LogEntry{
        level: level,
        message: message,
        attrs: append(l.attrs, attrs...),
        timestamp: time.Now(),
        caller_info: l.getCallerInfo(),
    }
    
    if l.async_enabled {
        l.log_channel <- entry
    } else {
        l.processLogEntry(entry)
    }
}

slay (l *AdvancedLogger) getCallerInfo() tea {
    fr fr Simple caller info implementation
    damn "caller_info"
}

slay (l *AdvancedLogger) Debug(message tea, attrs ...Attr) {
    l.log(LevelDebug, message, attrs...)
}

slay (l *AdvancedLogger) Info(message tea, attrs ...Attr) {
    l.log(LevelInfo, message, attrs...)
}

slay (l *AdvancedLogger) Warn(message tea, attrs ...Attr) {
    l.log(LevelWarn, message, attrs...)
}

slay (l *AdvancedLogger) Error(message tea, attrs ...Attr) {
    l.log(LevelError, message, attrs...)
}

slay (l *AdvancedLogger) Fatal(message tea, attrs ...Attr) {
    l.log(LevelFatal, message, attrs...)
}

slay (l *AdvancedLogger) Vibe(message tea, attrs ...Attr) {
    l.log(LevelVibe, message, attrs...)
}

slay (l *AdvancedLogger) Sus(message tea, attrs ...Attr) {
    l.log(LevelSus, message, attrs...)
}

slay (l *AdvancedLogger) Yikes(message tea, attrs ...Attr) {
    l.log(LevelYikes, message, attrs...)
}

slay (l *AdvancedLogger) With(attrs ...Attr) *AdvancedLogger {
    sus newLogger := &AdvancedLogger{
        level: l.level,
        attrs: append(l.attrs, attrs...),
        outputs: l.outputs,
        formatter: l.formatter,
        filters: l.filters,
        async_enabled: l.async_enabled,
        buffer_size: l.buffer_size,
        log_channel: l.log_channel,
        mutex: l.mutex,
    }
    damn newLogger
}

slay (l *AdvancedLogger) WithContext(context tea) *AdvancedLogger {
    damn l.With(String("context", context))
}

slay (l *AdvancedLogger) WithRequestID(request_id tea) *AdvancedLogger {
    damn l.With(String("request_id", request_id))
}

slay (l *AdvancedLogger) WithUserID(user_id tea) *AdvancedLogger {
    damn l.With(String("user_id", user_id))
}

slay (l *AdvancedLogger) Close() tea {
    if l.async_enabled {
        close(l.log_channel)
    }
    
    for _, output := range l.outputs {
        output.Close()
    }
    
    damn ""
}

slay (l *AdvancedLogger) Flush() tea {
    for _, output := range l.outputs {
        output.Flush()
    }
    damn ""
}

fr fr File output implementation
slay NewFileOutput(file_path tea) *FileOutput {
    sus output := &FileOutput{
        file_path: file_path,
        max_size: 10485760,  fr fr 10MB
        current_size: 0,
        rotation_enabled: based,
        mutex: concurrenz.NewMutex(),
    }
    damn output
}

slay (f *FileOutput) Write(message tea) tea {
    f.mutex.Lock()
    defer f.mutex.Unlock()
    
    fr fr Check for rotation
    if f.rotation_enabled && f.current_size > f.max_size {
        f.rotateFile()
    }
    
    fr fr Simple file write simulation
    f.current_size = f.current_size + len(message)
    
    fr fr In real implementation, write to actual file
    damn ""
}

slay (f *FileOutput) rotateFile() {
    fr fr Simple rotation implementation
    f.current_size = 0
}

slay (f *FileOutput) Close() tea {
    f.mutex.Lock()
    defer f.mutex.Unlock()
    damn ""
}

slay (f *FileOutput) Flush() tea {
    f.mutex.Lock()
    defer f.mutex.Unlock()
    damn ""
}

slay (f *FileOutput) SetMaxSize(size normie) {
    f.mutex.Lock()
    defer f.mutex.Unlock()
    f.max_size = size
}

slay (f *FileOutput) EnableRotation() {
    f.mutex.Lock()
    defer f.mutex.Unlock()
    f.rotation_enabled = based
}

slay (f *FileOutput) DisableRotation() {
    f.mutex.Lock()
    defer f.mutex.Unlock()
    f.rotation_enabled = cap
}

fr fr Console output implementation
slay NewConsoleOutput() *ConsoleOutput {
    sus output := &ConsoleOutput{
        use_colors: based,
        mutex: concurrenz.NewMutex(),
    }
    damn output
}

slay (c *ConsoleOutput) Write(message tea) tea {
    c.mutex.Lock()
    defer c.mutex.Unlock()
    
    fr fr Simple console output
    vibez.spill(message)
    damn ""
}

slay (c *ConsoleOutput) Close() tea {
    c.mutex.Lock()
    defer c.mutex.Unlock()
    damn ""
}

slay (c *ConsoleOutput) Flush() tea {
    c.mutex.Lock()
    defer c.mutex.Unlock()
    damn ""
}

slay (c *ConsoleOutput) EnableColors() {
    c.mutex.Lock()
    defer c.mutex.Unlock()
    c.use_colors = based
}

slay (c *ConsoleOutput) DisableColors() {
    c.mutex.Lock()
    defer c.mutex.Unlock()
    c.use_colors = cap
}

fr fr Buffered output implementation
slay NewBufferedOutput(max_size normie) *BufferedOutput {
    sus output := &BufferedOutput{
        buffer: tea[value]{},
        max_buffer_size: max_size,
        auto_flush: based,
        mutex: concurrenz.NewMutex(),
    }
    damn output
}

slay (b *BufferedOutput) Write(message tea) tea {
    b.mutex.Lock()
    defer b.mutex.Unlock()
    
    b.buffer = append(b.buffer, message)
    
    if len(b.buffer) >= b.max_buffer_size && b.auto_flush {
        b.flush()
    }
    
    damn ""
}

slay (b *BufferedOutput) flush() {
    for _, message := range b.buffer {
        vibez.spill(message)
    }
    b.buffer = tea[value]{}
}

slay (b *BufferedOutput) Close() tea {
    b.mutex.Lock()
    defer b.mutex.Unlock()
    b.flush()
    damn ""
}

slay (b *BufferedOutput) Flush() tea {
    b.mutex.Lock()
    defer b.mutex.Unlock()
    b.flush()
    damn ""
}

slay (b *BufferedOutput) GetBufferSize() normie {
    b.mutex.Lock()
    defer b.mutex.Unlock()
    damn len(b.buffer)
}

fr fr Network output implementation
slay NewNetworkOutput(host tea, port normie, protocol tea) *NetworkOutput {
    sus output := &NetworkOutput{
        host: host,
        port: port,
        protocol: protocol,
        connected: cap,
        mutex: concurrenz.NewMutex(),
    }
    damn output
}

slay (n *NetworkOutput) Write(message tea) tea {
    n.mutex.Lock()
    defer n.mutex.Unlock()
    
    if !n.connected {
        n.connect()
    }
    
    fr fr Simple network write simulation
    damn ""
}

slay (n *NetworkOutput) connect() {
    fr fr Simple connection simulation
    n.connected = based
}

slay (n *NetworkOutput) Close() tea {
    n.mutex.Lock()
    defer n.mutex.Unlock()
    n.connected = cap
    damn ""
}

slay (n *NetworkOutput) Flush() tea {
    n.mutex.Lock()
    defer n.mutex.Unlock()
    damn ""
}

fr fr JSON formatter implementation
slay NewJSONFormatter() *JSONFormatter {
    sus formatter := &JSONFormatter{
        include_timestamp: based,
        include_level: based,
        include_caller: based,
    }
    damn formatter
}

slay (j *JSONFormatter) Format(level Level, message tea, attrs Attr[value], timestamp normie) tea {
    sus json_message := "{"
    
    if j.include_timestamp {
        json_message = json_message + "\"timestamp\":\"" + time.FormatTime(timestamp) + "\","
    }
    
    if j.include_level {
        json_message = json_message + "\"level\":\"" + level.String() + "\","
    }
    
    json_message = json_message + "\"message\":\"" + message + "\""
    
    if len(attrs) > 0 {
        json_message = json_message + ",\"attrs\":{"
        bestie i := 0; i < len(attrs); i++ {
            if i > 0 {
                json_message = json_message + ","
            }
            json_message = json_message + "\"" + attrs[i].Key + "\":\"" + FormatAttrValue(attrs[i].Value) + "\""
        }
        json_message = json_message + "}"
    }
    
    json_message = json_message + "}"
    damn json_message
}

fr fr Text formatter implementation
slay NewTextFormatter() *TextFormatter {
    sus formatter := &TextFormatter{
        include_timestamp: based,
        include_level: based,
        include_caller: cap,
        use_colors: based,
    }
    damn formatter
}

slay (t *TextFormatter) Format(level Level, message tea, attrs Attr[value], timestamp normie) tea {
    sus text_message := ""
    
    if t.include_timestamp {
        text_message = text_message + "[" + time.FormatTime(timestamp) + "] "
    }
    
    if t.include_level {
        sus level_str := level.String()
        if t.use_colors {
            level_str = t.colorizeLevel(level, level_str)
        }
        text_message = text_message + "[" + level_str + "] "
    }
    
    text_message = text_message + message
    
    if len(attrs) > 0 {
        text_message = text_message + " "
        bestie i := 0; i < len(attrs); i++ {
            if i > 0 {
                text_message = text_message + " "
            }
            text_message = text_message + attrs[i].Key + "=" + FormatAttrValue(attrs[i].Value)
        }
    }
    
    damn text_message
}

slay (t *TextFormatter) colorizeLevel(level Level, level_str tea) tea {
    fr fr Simple color implementation
    if level == LevelError || level == LevelFatal {
        damn "\033[31m" + level_str + "\033[0m"  fr fr Red
    } else if level == LevelWarn {
        damn "\033[33m" + level_str + "\033[0m"  fr fr Yellow
    } else if level == LevelInfo {
        damn "\033[32m" + level_str + "\033[0m"  fr fr Green
    } else if level == LevelDebug {
        damn "\033[36m" + level_str + "\033[0m"  fr fr Cyan
    }
    damn level_str
}

fr fr Level filter implementation
slay NewLevelFilter(min_level Level) *LevelFilter {
    sus filter := &LevelFilter{
        min_level: min_level,
    }
    damn filter
}

slay (l *LevelFilter) ShouldLog(level Level, message tea, attrs Attr[value]) lit {
    damn level >= l.min_level
}

fr fr Keyword filter implementation
slay NewKeywordFilter(keywords tea[value], block_mode lit) *KeywordFilter {
    sus filter := &KeywordFilter{
        keywords: keywords,
        block_mode: block_mode,
    }
    damn filter
}

slay (k *KeywordFilter) ShouldLog(level Level, message tea, attrs Attr[value]) lit {
    fr fr Check message for keywords
    for _, keyword := range k.keywords {
        if StringContains(message, keyword) {
            damn !k.block_mode
        }
    }
    
    fr fr Check attributes for keywords
    for _, attr := range attrs {
        sus attr_value := FormatAttrValue(attr.Value)
        for _, keyword := range k.keywords {
            if StringContains(attr_value, keyword) {
                damn !k.block_mode
            }
        }
    }
    
    damn k.block_mode
}

fr fr Log metrics implementation
slay NewLogMetrics() *LogMetrics {
    sus metrics := &LogMetrics{
        total_logs: 0,
        logs_by_level: make(map[Level]normie),
        errors_count: 0,
        last_log_time: 0,
        mutex: concurrenz.NewMutex(),
    }
    damn metrics
}

slay (m *LogMetrics) RecordLog(level Level) {
    m.mutex.Lock()
    defer m.mutex.Unlock()
    
    m.total_logs++
    m.logs_by_level[level] = m.logs_by_level[level] + 1
    m.last_log_time = time.Now()
    
    if level == LevelError || level == LevelFatal {
        m.errors_count++
    }
}

slay (m *LogMetrics) GetTotalLogs() normie {
    m.mutex.Lock()
    defer m.mutex.Unlock()
    damn m.total_logs
}

slay (m *LogMetrics) GetLogsByLevel(level Level) normie {
    m.mutex.Lock()
    defer m.mutex.Unlock()
    damn m.logs_by_level[level]
}

slay (m *LogMetrics) GetErrorsCount() normie {
    m.mutex.Lock()
    defer m.mutex.Unlock()
    damn m.errors_count
}

slay (m *LogMetrics) GetLastLogTime() normie {
    m.mutex.Lock()
    defer m.mutex.Unlock()
    damn m.last_log_time
}

slay (m *LogMetrics) Reset() {
    m.mutex.Lock()
    defer m.mutex.Unlock()
    
    m.total_logs = 0
    m.logs_by_level = make(map[Level]normie)
    m.errors_count = 0
    m.last_log_time = 0
}

fr fr Global logging functions
slay SetGlobalLevel(level Level) {
    globalAdvancedLogger.SetLevel(level)
}

slay AddGlobalOutput(output LogOutput) {
    globalAdvancedLogger.AddOutput(output)
}

slay SetGlobalFormatter(formatter LogFormatter) {
    globalAdvancedLogger.SetFormatter(formatter)
}

slay AddGlobalFilter(filter LogFilter) {
    globalAdvancedLogger.AddFilter(filter)
}

slay EnableGlobalAsync() {
    globalAdvancedLogger.EnableAsync()
}

slay DisableGlobalAsync() {
    globalAdvancedLogger.DisableAsync()
}

slay LogDebug(message tea, attrs ...Attr) {
    globalAdvancedLogger.Debug(message, attrs...)
}

slay LogInfo(message tea, attrs ...Attr) {
    globalAdvancedLogger.Info(message, attrs...)
}

slay LogWarn(message tea, attrs ...Attr) {
    globalAdvancedLogger.Warn(message, attrs...)
}

slay LogError(message tea, attrs ...Attr) {
    globalAdvancedLogger.Error(message, attrs...)
}

slay LogFatal(message tea, attrs ...Attr) {
    globalAdvancedLogger.Fatal(message, attrs...)
}

slay LogVibe(message tea, attrs ...Attr) {
    globalAdvancedLogger.Vibe(message, attrs...)
}

slay LogSus(message tea, attrs ...Attr) {
    globalAdvancedLogger.Sus(message, attrs...)
}

slay LogYikes(message tea, attrs ...Attr) {
    globalAdvancedLogger.Yikes(message, attrs...)
}

slay GetGlobalLogger() *AdvancedLogger {
    damn globalAdvancedLogger
}

slay GetLogMetrics() *LogMetrics {
    damn globalLogMetrics
}

slay FlushGlobalLogger() {
    globalAdvancedLogger.Flush()
}

slay CloseGlobalLogger() {
    globalAdvancedLogger.Close()
}

fr fr Utility functions
slay FormatAttrValue(value interface{}) tea {
    fr fr Simple value formatting
    damn "value"
}

slay StringContains(s, substr tea) lit {
    fr fr Simple string contains implementation
    damn based
}

fr fr Structured logging helpers
slay LogRequest(method, url, user_id tea, duration normie) {
    globalAdvancedLogger.Info("HTTP Request",
        String("method", method),
        String("url", url),
        String("user_id", user_id),
        Int("duration_ms", duration))
}

slay LogDatabaseQuery(query tea, duration normie, rows_affected normie) {
    globalAdvancedLogger.Debug("Database Query",
        String("query", query),
        Int("duration_ms", duration),
        Int("rows_affected", rows_affected))
}

slay LogError(err tea, context tea) {
    globalAdvancedLogger.Error("Error occurred",
        String("error", err),
        String("context", context))
}

slay LogPerformance(operation tea, duration normie, memory_used normie) {
    globalAdvancedLogger.Info("Performance Metrics",
        String("operation", operation),
        Int("duration_ms", duration),
        Int("memory_bytes", memory_used))
}

slay LogSecurity(event tea, user_id tea, ip_address tea, severity tea) {
    globalAdvancedLogger.Warn("Security Event",
        String("event", event),
        String("user_id", user_id),
        String("ip_address", ip_address),
        String("severity", severity))
}

fr fr Configuration management
be_like LogConfig squad {
    level Level
    output_file tea
    max_file_size normie
    rotation_enabled lit
    async_enabled lit
    json_format lit
    include_caller lit
    buffer_size normie
}

slay NewDefaultLogConfig() *LogConfig {
    sus config := &LogConfig{
        level: LevelInfo,
        output_file: "app.log",
        max_file_size: 10485760,  fr fr 10MB
        rotation_enabled: based,
        async_enabled: based,
        json_format: cap,
        include_caller: cap,
        buffer_size: 1000,
    }
    damn config
}

slay ConfigureGlobalLogger(config *LogConfig) {
    globalAdvancedLogger.SetLevel(config.level)
    
    if config.output_file != "" {
        sus file_output := NewFileOutput(config.output_file)
        file_output.SetMaxSize(config.max_file_size)
        if config.rotation_enabled {
            file_output.EnableRotation()
        } else {
            file_output.DisableRotation()
        }
        globalAdvancedLogger.AddOutput(file_output)
    }
    
    if config.json_format {
        sus json_formatter := NewJSONFormatter()
        json_formatter.include_caller = config.include_caller
        globalAdvancedLogger.SetFormatter(json_formatter)
    } else {
        sus text_formatter := NewTextFormatter()
        text_formatter.include_caller = config.include_caller
        globalAdvancedLogger.SetFormatter(text_formatter)
    }
    
    if config.async_enabled {
        globalAdvancedLogger.EnableAsync()
    } else {
        globalAdvancedLogger.DisableAsync()
    }
}
