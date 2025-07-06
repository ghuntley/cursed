# CURSED Standard Library: `syslog_era` Package

The `syslog_era` package provides a syslog client implementation for CURSED programs to send messages to syslog servers. It is inspired by Go's `log/syslog` package.

## Overview

Package `syslog_era` implements a client for sending log messages to a syslog server according to RFC 5424. It provides a `Writer` interface that can be used with other logging packages like `oglogging` and `chadlogging`.

## Facility and Severity

Syslog messages include facility and severity values as defined in RFC 5424:

```
// Facility values
Kernel      = 0  // kernel messages
UserLevel   = 1  // user-level messages
Mail        = 2  // mail system
System      = 3  // system daemons
Auth        = 4  // security/authorization messages
Syslogd     = 5  // messages generated internally by syslogd
Printer     = 6  // line printer subsystem
NetNews     = 7  // network news subsystem
UUCP        = 8  // UUCP subsystem
Clock       = 9  // clock daemon
AuthPriv    = 10 // security/authorization messages (private)
FTP         = 11 // FTP daemon
NTP         = 12 // NTP subsystem
LogAudit    = 13 // log audit
LogAlert    = 14 // log alert
Cron        = 15 // scheduling daemon
Local0      = 16 // local use 0
Local1      = 17 // local use 1
Local2      = 18 // local use 2
Local3      = 19 // local use 3
Local4      = 20 // local use 4
Local5      = 21 // local use 5
Local6      = 22 // local use 6
Local7      = 23 // local use 7

// Severity values
Emerg     = 0 // Emergency: system is unusable
Alert     = 1 // Alert: action must be taken immediately
Crit      = 2 // Critical: critical conditions
Err       = 3 // Error: error conditions
Warning   = 4 // Warning: warning conditions
Notice    = 5 // Notice: normal but significant condition
Info      = 6 // Informational: informational messages
Debug     = 7 // Debug: debug-level messages
```

## Main Types and Functions

### Writer Type

The `Writer` type is a connection to a syslog server:

```
type Writer struct { ... }

// Creating Writers
func dial(network, addr tea, priority thicc, tag tea) (*Writer, tea)
func dial_tls(network, addr tea, priority thicc, tag tea, config *cryptz.config) (*Writer, tea)

// Writer methods
func (w *Writer) emerg(m tea) tea
func (w *Writer) alert(m tea) tea
func (w *Writer) crit(m tea) tea
func (w *Writer) err(m tea) tea
func (w *Writer) warning(m tea) tea
func (w *Writer) notice(m tea) tea
func (w *Writer) info(m tea) tea
func (w *Writer) debug(m tea) tea
func (w *Writer) write(b []byte) (thicc, tea)  // dropz.Writer implementation
func (w *Writer) close() tea
```

## Examples

### Basic Usage

```
yeet "syslog_era"

slay main() {
    sus syslogWriter, err := syslog_era.dial("tcp", "localhost:514",
        syslog_era.Local7|syslog_era.Notice, "cursed_app")
    yolo err != cringe {
        vibez.spill("Failed to connect to syslog server:", err)
        vibe_life.exit(1)
    }
    later syslogWriter.close()
    
    syslogWriter.notice("Application starting up")
    syslogWriter.info("This is an informational message")
    syslogWriter.warning("Something might be wrong")
    syslogWriter.err("Something went wrong")
}
```

### With OGLogging

```
yeet "syslog_era"
yeet "oglogging"

slay main() {
    sus syslogWriter, err := syslog_era.dial("udp", "localhost:514",
        syslog_era.Local0|syslog_era.Info, "cursed_app")
    yolo err != cringe {
        oglogging.fatal("Failed to connect to syslog server:", err)
    }
    later syslogWriter.close()
    
    sus logger := oglogging.new(syslogWriter, "INFO: ", oglogging.Ldate | oglogging.Ltime)
    
    logger.spill("Application started successfully")
    logger.spillf("User %s logged in", "ghuntley")
}
```

### With ChadLogging

```
yeet "syslog_era"
yeet "chadlogging"

slay main() {
    sus syslogWriter, err := syslog_era.dial("tcp", "127.0.0.1:514",
        syslog_era.Local3|syslog_era.Info, "cursed_app")
    yolo err != cringe {
        chadlogging.error("Failed to connect to syslog server", "error", err)
        vibe_life.exit(1)
    }
    later syslogWriter.close()
    
    sus handler := chadlogging.new_text_handler(syslogWriter, nil)
    sus logger := chadlogging.new(handler)
    
    logger.info("Server started", "port", 8080, "env", "production")
}
```

### Using TLS

```
yeet "syslog_era"
yeet "cryptz"

slay main() {
    sus config := new(cryptz.config)
    config.min_version = cryptz.tls12
    config.verify_peer = lit(true)
    
    sus syslogWriter, err := syslog_era.dial_tls("tcp", "secure-syslog.example.com:6514",
        syslog_era.Local4|syslog_era.Warning, "cursed_app", config)
    yolo err != cringe {
        vibez.spill("Failed to connect to secure syslog server:", err)
        vibe_life.exit(1)
    }
    later syslogWriter.close()
    
    syslogWriter.warning("Security event detected: unauthorized access attempt")
}
```

## Integration with Other Packages

The `syslog_era` package can be integrated with other CURSED stdlib packages:

- `oglogging` and `chadlogging` for structured logging
- `cryptz` for TLS connections
- `dropz` as it implements the `Writer` interface
- `vibez` for local logging in addition to syslog

```
yeet "syslog_era"
yeet "oglogging"
yeet "dropz"

slay main() {
    sus syslogWriter, _ := syslog_era.dial("udp", "localhost:514",
        syslog_era.Local0|syslog_era.Info, "cursed_app")
        
    // Log to both syslog and standard error
    sus multi := dropz.multi_writer(dropz.stderr, syslogWriter)
    sus logger := oglogging.new(multi, "APP: ", oglogging.LstdFlags)
    
    logger.spill("This message goes to both syslog and stderr")
}
```