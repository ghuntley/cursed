# timez Module Specification

## Overview
The timez module provides comprehensive time handling functionality for CURSED applications. This module implements time operations using pure CURSED code without FFI dependencies.

## Core Types

### Time
- Represents an instant in time
- Unix timestamp based (seconds since epoch)
- Nanosecond precision support

### Duration
- Represents a span of time
- Nanosecond precision
- Supports arithmetic operations

## Functions

### Time Functions
- `now() -> Time` - Get current time
- `unix(seconds normie) -> Time` - Create time from Unix timestamp
- `parse_rfc3339(timestamp tea) -> Time` - Parse RFC3339 time string
- `since_epoch(time Time) -> Duration` - Get duration since Unix epoch

### Duration Functions
- `seconds(s normie) -> Duration` - Create duration from seconds
- `milliseconds(ms normie) -> Duration` - Create duration from milliseconds
- `microseconds(us normie) -> Duration` - Create duration from microseconds
- `nanoseconds(ns normie) -> Duration` - Create duration from nanoseconds

### Time Arithmetic
- `add_duration(time Time, dur Duration) -> Time` - Add duration to time
- `sub_duration(time Time, dur Duration) -> Time` - Subtract duration from time
- `time_diff(t1 Time, t2 Time) -> Duration` - Get duration between times

### Formatting
- `format_rfc3339(time Time) -> tea` - Format time as RFC3339 string
- `format_unix(time Time) -> tea` - Format time as Unix timestamp string
- `format_human(time Time) -> tea` - Format time in human-readable format

### Utility Functions
- `sleep(dur Duration)` - Sleep for specified duration
- `is_before(t1 Time, t2 Time) -> lit` - Check if t1 is before t2
- `is_after(t1 Time, t2 Time) -> lit` - Check if t1 is after t2
- `is_zero(time Time) -> lit` - Check if time is zero value

## Implementation Notes

### Pure CURSED Implementation
- No FFI dependencies
- Uses system call simulation for time operations
- Implements UTC timezone handling
- Thread-safe operations

### Precision
- Nanosecond precision for all operations
- 64-bit integer representation for timestamps
- Overflow protection for duration arithmetic

### Compatibility
- RFC3339 standard compliance
- Unix timestamp compatibility
- ISO 8601 parsing support

## Error Handling
- Invalid time parsing returns zero time
- Duration overflow protection
- Graceful handling of system time errors

## Examples

```cursed
yeet "timez"

# Get current time
sus current_time := timez.now()

# Create durations
sus five_seconds := timez.seconds(5)
sus hundred_millis := timez.milliseconds(100)

# Time arithmetic
sus future_time := timez.add_duration(current_time, five_seconds)

# Formatting
sus time_string := timez.format_rfc3339(current_time)
vibez.spill(time_string)

# Sleep
timez.sleep(hundred_millis)

# Time comparison
sus is_past := timez.is_before(current_time, future_time)
```

## Thread Safety
All timez functions are thread-safe and can be used in concurrent goroutines without synchronization.

## Performance
- O(1) time complexity for all operations
- Minimal memory allocation
- Optimized for high-frequency usage
