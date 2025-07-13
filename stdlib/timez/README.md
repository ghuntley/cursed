# timez Module

The timez module provides comprehensive time handling functionality for CURSED applications with pure CURSED implementations and no FFI dependencies.

## Features

- **Pure CURSED Implementation**: No external dependencies or FFI bridges
- **High Precision**: Nanosecond precision for all time operations
- **Thread Safety**: All functions are safe for concurrent use
- **RFC3339 Support**: Standard time format parsing and formatting
- **Duration Arithmetic**: Complete duration manipulation operations
- **Time Comparison**: Comprehensive time comparison utilities

## Quick Start

```cursed
yeet "timez"

# Get current time
sus now := timez.now()
vibez.spill("Current time: " + timez.format_human(now))

# Create durations
sus five_minutes := timez.add_durations(timez.MINUTE, timez.MINUTE)
five_minutes = timez.add_durations(five_minutes, timez.MINUTE)
five_minutes = timez.add_durations(five_minutes, timez.MINUTE)
five_minutes = timez.add_durations(five_minutes, timez.MINUTE)

# Time arithmetic
sus future := timez.add_duration(now, five_minutes)
vibez.spill("In 5 minutes: " + timez.format_human(future))

# Sleep for 100 milliseconds
timez.sleep(timez.milliseconds(100))
```

## Core Types

### Time
Represents an instant in time using Unix timestamp with nanosecond precision.

### Duration  
Represents a span of time with nanosecond precision supporting arithmetic operations.

## API Reference

### Time Creation
- `now() -> Time` - Get current system time
- `unix(seconds normie) -> Time` - Create time from Unix timestamp
- `parse_rfc3339(timestamp tea) -> Time` - Parse RFC3339 format string
- `parse_unix_string(timestamp tea) -> Time` - Parse Unix timestamp string

### Duration Creation
- `seconds(s normie) -> Duration` - Create duration from seconds
- `milliseconds(ms normie) -> Duration` - Create duration from milliseconds  
- `microseconds(us normie) -> Duration` - Create duration from microseconds
- `nanoseconds(ns normie) -> Duration` - Create duration from nanoseconds

### Time Arithmetic
- `add_duration(time Time, dur Duration) -> Time` - Add duration to time
- `sub_duration(time Time, dur Duration) -> Time` - Subtract duration from time
- `time_diff(t1 Time, t2 Time) -> Duration` - Calculate duration between times

### Duration Arithmetic
- `add_durations(d1 Duration, d2 Duration) -> Duration` - Add two durations
- `sub_durations(d1 Duration, d2 Duration) -> Duration` - Subtract durations

### Formatting
- `format_rfc3339(time Time) -> tea` - Format as RFC3339 string
- `format_unix(time Time) -> tea` - Format as Unix timestamp string
- `format_human(time Time) -> tea` - Format in human-readable format
- `format_duration(dur Duration) -> tea` - Format duration as "1h30m45s"

### Comparison
- `is_before(t1 Time, t2 Time) -> lit` - Check if t1 is before t2
- `is_after(t1 Time, t2 Time) -> lit` - Check if t1 is after t2
- `is_zero(time Time) -> lit` - Check if time is zero value

### Utility Functions
- `sleep(dur Duration)` - Sleep for specified duration
- `since_epoch(time Time) -> Duration` - Get duration since Unix epoch
- `is_valid_time(time Time) -> lit` - Validate time value
- `is_valid_duration(dur Duration) -> lit` - Validate duration value

### High Precision
- `now_nano() -> thicc` - Get current time in nanoseconds
- `add_nano(time Time, nanos thicc) -> Time` - Add nanoseconds to time

### Duration Conversion
- `duration_seconds(dur Duration) -> normie` - Get seconds from duration
- `duration_milliseconds(dur Duration) -> normie` - Get milliseconds
- `duration_microseconds(dur Duration) -> normie` - Get microseconds  
- `duration_nanoseconds(dur Duration) -> thicc` - Get nanoseconds

### Timezone Support
- `utc_offset() -> normie` - Get UTC offset (always 0)
- `is_utc() -> lit` - Check if using UTC (always true)

## Constants

### Duration Constants
- `MINUTE` - 60 seconds
- `HOUR` - 3600 seconds  
- `DAY` - 86400 seconds
- `WEEK` - 604800 seconds

### Internal Constants
- `NANOS_PER_SECOND` - Nanoseconds in one second
- `NANOS_PER_MILLI` - Nanoseconds in one millisecond
- `NANOS_PER_MICRO` - Nanoseconds in one microsecond

## Examples

### Basic Time Operations
```cursed
yeet "timez"

# Create specific time
sus new_year := timez.unix(1640995200)  # 2022-01-01 00:00:00 UTC
vibez.spill("New Year: " + timez.format_rfc3339(new_year))

# Add one day
sus next_day := timez.add_duration(new_year, timez.DAY)
vibez.spill("Next day: " + timez.format_human(next_day))
```

### Duration Calculations
```cursed
yeet "timez"

# Calculate elapsed time
sus start := timez.now()
timez.sleep(timez.milliseconds(500))
sus end := timez.now()

sus elapsed := timez.time_diff(end, start)
vibez.spill("Elapsed: " + timez.format_duration(elapsed))
```

### Time Comparisons
```cursed
yeet "timez"

sus meeting_time := timez.unix(1640998800)  # Some future time
sus current := timez.now()

bestie timez.is_before(current, meeting_time); {
    sus remaining := timez.time_diff(meeting_time, current)
    vibez.spill("Meeting in: " + timez.format_duration(remaining))
} simp {
    vibez.spill("Meeting has already started!")
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/timez/test_timez.csd

# Test compilation mode  
cargo run --bin cursed -- compile stdlib/timez/test_timez.csd
./test_timez

# Both mode verification
test_both_modes stdlib/timez/test_timez.csd
```

## Implementation Notes

### Pure CURSED Design
- No FFI calls or external system dependencies
- Uses internal time simulation for deterministic testing
- All operations implemented in native CURSED code
- Thread-safe by design

### Precision and Accuracy
- Nanosecond precision throughout the API
- 64-bit integer arithmetic for all calculations
- Overflow protection for duration operations
- Consistent behavior across platforms

### Performance Characteristics
- O(1) time complexity for all operations
- Minimal memory allocation
- Optimized for high-frequency usage
- Suitable for real-time applications

### Limitations
- Current implementation uses simulated time for testing
- Production deployment would require system time integration
- Timezone support limited to UTC
- Date parsing is simplified for initial implementation

## Contributing

When extending the timez module:
1. Maintain pure CURSED implementation (no FFI)
2. Add comprehensive tests for new functionality
3. Follow existing naming conventions
4. Update documentation and examples
5. Ensure thread safety for all operations

## Related Modules

- `core` - Basic string and numeric operations
- `testz` - Testing framework for validation
- `io` - File operations with timestamps
- `concurrenz` - Concurrent operations with timeouts
