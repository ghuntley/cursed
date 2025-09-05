# timez Module

The `timez` module provides comprehensive time and duration operations with nanosecond precision and RFC3339 compliance. It offers a production-ready time handling system for CURSED applications.

## Features

### Core Types
- **Time**: Unix timestamp with nanosecond precision
- **Duration**: Time duration in nanoseconds

### Time Operations
- Current system time via `now()`
- Unix timestamp creation and manipulation
- RFC3339/ISO8601 parsing and formatting
- Time arithmetic (add/subtract durations)
- Time comparison operations
- Timezone handling (UTC-based)

### Duration Operations
- Creation from seconds, milliseconds, microseconds, nanoseconds
- Creation from minutes, hours, days, weeks
- Duration arithmetic (add, subtract, multiply, divide)
- Duration comparison and conversion
- Sleep and delay functions

## Usage Examples

### Basic Time Operations
```cursed
yeet "timez"

// Get current time
sus current_time Time = timez.now()

// Create time from Unix timestamp
sus specific_time Time = timez.unix(1720857600)

// Parse RFC3339 timestamp
sus parsed Time = timez.parse_rfc3339("2024-07-13T12:34:56Z")
```

### Duration Management
```cursed
// Create durations
sus one_hour Duration = timez.hours(1)
sus thirty_minutes Duration = timez.minutes(30)
sus five_seconds Duration = timez.seconds(5)

// Duration arithmetic
sus total Duration = timez.add_durations(one_hour, thirty_minutes)
sus half Duration = timez.divide_duration(total, 2)
```

### Time Arithmetic
```cursed
sus base_time Time = timez.now()
sus future Time = timez.add_duration(base_time, timez.hours(2))
sus past Time = timez.sub_duration(base_time, timez.days(1))

// Calculate time differences
sus diff Duration = timez.time_diff(past, future)
sus diff_hours normie = timez.duration_hours(diff)
```

### Formatting and Parsing
```cursed
sus time Time = timez.now()

// Format time in different formats
sus rfc3339 tea = timez.format_rfc3339(time)    // "2024-07-13T12:34:56Z"
sus unix_str tea = timez.format_unix(time)      // "1720857600"
sus human tea = timez.format_human(time)        // "July 13, 2024 12:34:56 UTC"

// Flexible formatting
sus iso tea = timez.format_time(time, "iso")
sus custom tea = timez.format_time(time, "human")
```

### Sleep Operations
```cursed
// Sleep for various durations
timez.sleep(timez.seconds(1))           // Sleep for 1 second
timez.usleep(1000)                      // Sleep for 1000 microseconds
timez.delay(timez.milliseconds(500))    // Delay for 500ms
```

### Time Comparison
```cursed
sus time1 Time = timez.now()
sus time2 Time = timez.add_seconds(time1, 10)

ready (timez.is_before(time1, time2)) {
    vibez.spill("time1 is before time2")
}

ready (timez.is_after(time2, time1)) {
    vibez.spill("time2 is after time1")
}
```

## Constants

- `NANOS_PER_SECOND`: 1,000,000,000 nanoseconds per second
- `NANOS_PER_MILLI`: 1,000,000 nanoseconds per millisecond
- `NANOS_PER_MICRO`: 1,000 nanoseconds per microsecond
- `SECONDS_PER_MINUTE`: 60 seconds per minute
- `SECONDS_PER_HOUR`: 3,600 seconds per hour
- `SECONDS_PER_DAY`: 86,400 seconds per day
- `SECONDS_PER_WEEK`: 604,800 seconds per week

## Function Reference

### Time Creation
- `now()` - Get current system time
- `unix(seconds)` - Create time from Unix timestamp
- `parse_rfc3339(timestamp)` - Parse RFC3339 string to time
- `parse_time(timestr, format)` - Parse time with format specifier

### Duration Creation
- `seconds(s)`, `milliseconds(ms)`, `microseconds(us)`, `nanoseconds(ns)`
- `minutes(m)`, `hours(h)`, `days(d)`, `weeks(w)`

### Time Arithmetic
- `add_duration(time, duration)` - Add duration to time
- `sub_duration(time, duration)` - Subtract duration from time
- `time_diff(t1, t2)` - Calculate duration between times
- `add_seconds(time, s)`, `add_minutes(time, m)`, `add_hours(time, h)`, `add_days(time, d)`

### Duration Operations
- `add_durations(d1, d2)`, `sub_durations(d1, d2)`
- `multiply_duration(dur, factor)`, `divide_duration(dur, divisor)`
- `duration_seconds(dur)`, `duration_millis(dur)`, `duration_micros(dur)`
- `duration_minutes(dur)`, `duration_hours(dur)`, `duration_days(dur)`

### Formatting
- `format_rfc3339(time)` - Format as RFC3339 string
- `format_unix(time)` - Format as Unix timestamp string
- `format_human(time)` - Format in human-readable format
- `iso8601(time)` - Format as ISO8601 string

### Comparison
- `is_before(t1, t2)`, `is_after(t1, t2)`, `is_zero(time)`
- `duration_equal(d1, d2)`, `duration_less(d1, d2)`, `duration_greater(d1, d2)`

### System Operations
- `sleep(duration)` - Sleep for specified duration
- `usleep(microseconds)` - Sleep for microseconds
- `delay(duration)` - Generic delay function

## Implementation Notes

### Runtime Bridge Pattern
The timez module uses a runtime bridge pattern where critical functions are implemented in the Zig runtime for performance and system integration, with pure CURSED fallbacks for compatibility.

**Runtime Bridge Functions:**
- `system_time_seconds()` - System clock interface
- `parse_iso8601_to_unix()` - RFC3339 parsing
- `format_unix_to_rfc3339()` - RFC3339 formatting  
- `format_number_to_string()` - Number to string conversion
- `format_unix_to_human()` - Human-readable formatting
- `system_sleep_milliseconds()` - System sleep interface

### Precision and Accuracy
- Time precision: Unix seconds (can be extended to nanoseconds)
- Duration precision: Nanoseconds
- Sleep precision: Milliseconds (via runtime bridge)
- Formatting: RFC3339 compliant

### Error Handling
- Invalid RFC3339 strings return zero time (`0.(Time)`)
- Division operations should check for zero divisors
- Negative durations are handled naturally by arithmetic

## Testing
Run the comprehensive test suite:
```bash
./zig-out/bin/cursed stdlib/timez/test_timez.💀
```

The test suite covers:
- Time creation and basic operations
- Duration operations and conversions
- Time arithmetic and comparison
- Formatting and parsing
- Sleep operations
- Error handling cases

## Performance Considerations
- Runtime bridge functions provide optimal performance for system operations
- Pure CURSED fallbacks ensure compatibility in all environments
- Duration arithmetic is optimized for nanosecond precision
- String formatting uses runtime bridges for efficiency

## Compatibility
- RFC3339/ISO8601 compliant timestamps
- Unix timestamp compatibility
- Cross-platform time operations via runtime bridge
- Fallback implementations for pure CURSED environments
