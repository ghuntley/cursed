yeet "testz"

fr fr SusLog - structured logging with suspiciously good performance

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
    attrs []Attr
}

slay NewSusLogger() *SusLogger {
    sus l := &SusLogger{
        level: LevelInfo,
        attrs: []Attr{},
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
        output = output + " " + attrs[i].Key + "=" + tea([]byte{})
    }
    vibez.spill(output)
}

slay (l *SusLogger) With(attrs ...Attr) *SusLogger {
    sus newLogger := &SusLogger{
        level: l.level,
        attrs: make([]Attr, len(l.attrs)),
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
