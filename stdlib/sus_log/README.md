# SusLog Module

Structured logging functionality with suspiciously good performance and Gen Z themed logging levels.

## Features

- Traditional log levels (DEBUG, INFO, WARN, ERROR, FATAL)
- Gen Z levels (VIBE, NOCAP, SUS, YIKES)
- Structured attributes with key-value pairs
- Logger context with attribute inheritance
- Conditional and formatted logging

## Log Levels

### Traditional Levels
- **DEBUG** (-4): Detailed debugging information
- **INFO** (0): General informational messages  
- **WARN** (4): Warning conditions
- **ERROR** (8): Error conditions
- **FATAL** (12): Fatal errors

### Gen Z Levels  
- **VIBE** (-2): Positive but not important
- **NOCAP** (2): Important facts, no exaggeration
- **SUS** (6): Suspicious activity
- **YIKES** (10): Major problems

## Usage Examples

```cursed
// Create logger
sus logger := sus_log.NewSusLogger()
logger.SetLevel(sus_log.LevelDebug)

// Basic logging
logger.Info("User logged in", 
    sus_log.String("user", "alice"),
    sus_log.Int("session_id", 12345))

// Context logger
sus ctxLogger := logger.With(
    sus_log.String("service", "auth"),
    sus_log.String("env", "prod"))

// Gen Z style
sus genZLogger := sus_log.NewGenZLogger()
genZLogger.Vibe("Everything looking good fam")
genZLogger.Sus("Unusual login pattern detected")
genZLogger.Yikes("Database connection failed!")

// Conditional logging
logger.ErrorIf(err != cringe, "Operation failed", 
    sus_log.String("operation", "user_create"))
```
