# timez Module

Pure CURSED time operations module providing comprehensive time handling with nanosecond precision and RFC3339 compliance.

## Features

- **Nanosecond Precision**: All time operations support nanosecond precision
- **RFC3339 Compliance**: Standard-compliant time formatting and parsing
- **Pure CURSED**: No FFI dependencies, implemented entirely in CURSED
- **Thread Safe**: All functions are safe for concurrent use
- **Duration Arithmetic**: Full support for time and duration calculations

## Types

- `Time` - Represents an instant in time (Unix timestamp based)
- `Duration` - Represents a span of time in nanoseconds

## Core Functions

### Time Creation
```cursed
sus current := timez.now()                    # Current time
sus epoch := timez.unix(1720857600)           # From Unix timestamp
sus parsed := timez.parse_rfc3339("2024-07-13T12:34:56Z")
```

### Duration Creation
```cursed
sus five_sec := timez.seconds(5)
sus hundred_ms := timez.milliseconds(100)
sus thousand_us := timez.microseconds(1000)
sus billion_ns := timez.nanoseconds(1000000000)
```

### Time Arithmetic
```cursed
sus future := timez.add_duration(current, five_sec)
sus past := timez.sub_duration(current, hundred_ms)
sus diff := timez.time_diff(future, past)
```

### Formatting
```cursed
sus rfc_string := timez.format_rfc3339(current)     # "2024-07-13T12:34:56Z"
sus unix_string := timez.format_unix(current)       # "1720857600"
sus human_string := timez.format_human(current)     # "July 13, 2024 12:34:56 UTC"
```

### Comparisons
```cursed
sus is_earlier := timez.is_before(past, future)
sus is_later := timez.is_after(future, past)
sus is_epoch := timez.is_zero(epoch)
```

### Duration Operations
```cursed
sus total := timez.add_durations(five_sec, hundred_ms)
sus remaining := timez.sub_durations(total, hundred_ms)
sus doubled := timez.multiply_duration(five_sec, 2)
sus halved := timez.divide_duration(doubled, 2)
```

### Utility Functions
```cursed
timez.sleep(hundred_ms)                           # Sleep for duration
sus seconds_val := timez.duration_seconds(five_sec)
sus millis_val := timez.duration_millis(hundred_ms)
```

## Examples

### Basic Time Operations
```cursed
yeet "timez"

# Get current time and create durations
sus now := timez.now()
sus delay := timez.seconds(30)
sus future := timez.add_duration(now, delay)

# Format and display
sus time_str := timez.format_rfc3339(future)
vibez.spill("Future time: " + time_str)
```

### Duration Calculations
```cursed
yeet "timez"

# Create and combine durations
sus minutes := timez.seconds(120)  # 2 minutes
sus extra := timez.milliseconds(500)
sus total := timez.add_durations(minutes, extra)

# Convert to different units
sus total_ms := timez.duration_millis(total)
vibez.spill("Total milliseconds: " + total_ms)
```

### Time Comparisons
```cursed
yeet "timez"

sus start := timez.now()
timez.sleep(timez.milliseconds(100))
sus end := timez.now()

sus elapsed := timez.time_diff(start, end)
sus is_positive := timez.duration_greater(elapsed, timez.nanoseconds(0))

if is_positive {
    vibez.spill("Time elapsed successfully")
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
```

## Implementation Notes

- Uses 64-bit integers for timestamp representation
- Nanosecond precision maintained throughout all operations
- RFC3339 formatting follows ISO 8601 standards
- Pure CURSED implementation ensures maximum portability
- Thread-safe design suitable for concurrent applications

## Constants

- `NANOS_PER_SECOND = 1,000,000,000`
- `NANOS_PER_MILLI = 1,000,000` 
- `NANOS_PER_MICRO = 1,000`

## Performance

- O(1) time complexity for all operations
- Minimal memory allocation
- Optimized for high-frequency time operations
- Suitable for real-time applications
