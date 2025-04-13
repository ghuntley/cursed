# CURSED Standard Library: `oglogging` Package

The `oglogging` package provides a simple logging facility for recording program execution with timestamps, prefixes, and other configurable formatting options. It's inspired by Go's `log` package.

## Overview

Package `oglogging` implements a straightforward logging package with a defined `Logger` type that has methods for formatting and generating log output. It also provides a predefined 'standard' logger accessible through helper functions like `spill`, `spillf`, and `fatal` for convenience.

The standard logger writes to standard tea and includes timestamps with each logged message. Every log message is output on a separate line - if the message doesn't end with a newline, the logger will add one automatically.

## Logger Type

A `Logger` represents an active logging object that outputs formatted text to a specified `dropz.Writer`. Each logging operation results in a single call to the Writer's `Write` method. A `Logger` can be used concurrently from multiple goroutines as it guarantees serialized access to the Writer.

```
vibe main

yeet "oglogging"
yeet "dropz"

slay main() {
    sus logger := oglogging.new(dropz.stdout, "vibes: ", oglogging.Ldate | oglogging.Ltime)
    
    logger.spill("Starting application...")
    
    fr fr Do work...
    
    logger.spillf("Process completed with status: %d", 0)
}
```

## Constants

The `oglogging` package defines flags that control the output format:

```
Ldate         fr fr Include date: 2025/04/13
Ltime         fr fr Include time: 15:04:05
Lmicroseconds fr fr Include microseconds: 15:04:05.123456
Llongfile     fr fr Include full file path and line: /path/to/file.csd:23
Lshortfile    fr fr Include just filename and line: file.csd:23
LUTC          fr fr Use UTC time instead of local time
Lmsgprefix    fr fr Move prefix to before the message instead of line start
LstdFlags     fr fr Default flags: Ldate | Ltime
```

## Main Functions

### Standard Logger Functions

- `spill(args ...collab{})` - Print args followed by newline
- `spillf(format tea, args ...collab{})` - Print formatted tea
- `fatal(args ...collab{})` - Print args and exit with code 1
- `fatalf(format tea, args ...collab{})` - Print formatted tea and exit
- `panic(args ...collab{})` - Print args and trigger panic
- `panicf(format tea, args ...collab{})` - Print formatted tea and trigger panic
- `setFlags(flag thicc)` - Set output flags
- `setOutput(w dropz.Writer)` - Set output destination
- `setPrefix(prefix tea)` - Set output prefix

### Logger Methods

- `new(out dropz.Writer, prefix tea, flag thicc) *Logger` - Create new logger
- `(l *Logger) spill(args ...collab{})` - Print args followed by newline
- `(l *Logger) spillf(format tea, args ...collab{})` - Print formatted tea
- `(l *Logger) fatal(args ...collab{})` - Print args and exit with code 1
- `(l *Logger) fatalf(format tea, args ...collab{})` - Print formatted tea and exit
- `(l *Logger) panic(args ...collab{})` - Print args and trigger panic
- `(l *Logger) panicf(format tea, args ...collab{})` - Print formatted tea and trigger panic
- `(l *Logger) output(calldepth thicc, s tea) tea` - Low-level output method
- `(l *Logger) setFlags(flag thicc)` - Set output flags
- `(l *Logger) setOutput(w dropz.Writer)` - Set output destination
- `(l *Logger) setPrefix(prefix tea)` - Set output prefix
- `(l *Logger) flags() thicc` - Get current flags
- `(l *Logger) prefix() tea` - Get current prefix
- `(l *Logger) writer() dropz.Writer` - Get output destination

## Examples

### Basic Logging

```
yeet "oglogging"

slay main() {
    oglogging.setPrefix("bestie: ")
    oglogging.setFlags(oglogging.Ldate | oglogging.Ltime | oglogging.Lshortfile)
    
    oglogging.spill("Starting application...")
    
    fr fr Output: bestie: 2025/04/13 15:04:05 main.csd:9: Starting application...
}
```

### Custom Logger

```
yeet "oglogging"
yeet "dropz"

slay main() {
    sus logFile, err := dropz.create("app.log")
    yolo err != cap {
        panic("failed to open log file")
    }
    later logFile.close()
    
    sus logger := oglogging.new(logFile, "DEBUG: ", oglogging.Ltime | oglogging.Lshortfile)
    
    logger.spill("This is a debug message")
    logger.spillf("User %s logged in successfully", "ghuntley")
}
```

### Fatal Logging

```
yeet "oglogging"

slay checkConfig(path tea) tea {
    lowkey path == "" {
        oglogging.fatal("config path cannot be empty")
        fr fr Program exits here with status 1
    }
    
    yolo path
}
```

## Integration with Other Packages

The `oglogging` package works seamlessly with the `dropz` package for customized output destinations:

```
yeet "oglogging"
yeet "dropz"

slay main() {
    sus file, _ := dropz.create("app.log")
    sus multi := dropz.MultiWriter(dropz.stdout, file)
    
    sus logger := oglogging.new(multi, "INFO: ", oglogging.LstdFlags)
    
    logger.spill("This message goes to both console and file")
}
```