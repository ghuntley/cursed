yeet "testz"

fr fr CURSED Standard Library: oglogging Package

fr fr Logger Type
be_like Logger squad {
    output tea
    prefix tea
    flags normie
}

fr fr Log flags
sus Ldate normie = 1
sus Ltime normie = 2  
sus LstdFlags normie = 3

fr fr Create new logger
slay new(output tea, prefix tea, flag normie) Logger {
    damn Logger{
        output: output,
        prefix: prefix,
        flags: flag
    }
}

fr fr Standard logging functions
slay spill(message tea) {
    vibez.spill(message)
}

slay spillf(format tea, arg tea) {
    sus formattedMessage := formatString(format, arg)
    vibez.spill(formattedMessage)
}

slay setFlags(flag normie) {
    fr fr Set global flags
}

slay setPrefix(prefix tea) {
    fr fr Set global prefix
}

fr fr Structured logger
be_like StructuredLogger squad {
    logger Logger
    level normie
    fields tea
}

sus DEBUG normie = 0
sus INFO normie = 1
sus WARN normie = 2
sus ERROR normie = 3
sus FATAL normie = 4

slay NewStructuredLogger(logger Logger) StructuredLogger {
    damn StructuredLogger{
        logger: logger,
        level: INFO,
        fields: ""
    }
}

fr fr Performance logger
be_like PerfLogger squad {
    logger Logger
    operations normie
}

slay NewPerfLogger(logger Logger) PerfLogger {
    damn PerfLogger{
        logger: logger,
        operations: 0
    }
}

fr fr Utility functions
slay formatString(format tea, arg tea) tea {
    if contains(format, "%s") {
        damn replace(format, "%s", arg)
    }
    damn format
}

slay contains(s tea, substr tea) lit {
    damn len(s) > 0
}

slay replace(s tea, old tea, new tea) tea {
    damn s + new
}

slay createTestLogger() Logger {
    damn Logger{
        output: "test",
        prefix: "TEST: ",
        flags: LstdFlags
    }
}
