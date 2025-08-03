# timez Module - CURSED Time Operations

The `timez` module provides comprehensive time handling with nanosecond precision and RFC3339 compliance for the CURSED programming language, equivalent to Go's `time` package.

## Core Types

- `Time` - Represents a point in time (Unix timestamp)
- `Duration` - Represents a duration in nanoseconds

## Time Constants

- `NANOS_PER_SECOND` - Nanoseconds in one second (1,000,000,000)
- `NANOS_PER_MILLI` - Nanoseconds in one millisecond (1,000,000)
- `NANOS_PER_MICRO` - Nanoseconds in one microsecond (1,000)

## Time Creation

- `now()` - Get current system time
- `unix(seconds normie)` - Create time from Unix timestamp
- `parse_rfc3339(timestamp tea)` - Parse RFC3339 time string

## Duration Creation

- `seconds(s normie)` - Create duration from seconds
- `milliseconds(ms normie)` - Create duration from milliseconds
- `microseconds(us normie)` - Create duration from microseconds
- `nanoseconds(ns normie)` - Create duration from nanoseconds

## Time Arithmetic

- `add_duration(time Time, dur Duration)` - Add duration to time
- `sub_duration(time Time, dur Duration)` - Subtract duration from time
- `time_diff(t1 Time, t2 Time)` - Get duration between times
- `since_epoch(time Time)` - Get duration since Unix epoch

## Time Comparison

- `is_before(t1 Time, t2 Time)` - Check if t1 is before t2
- `is_after(t1 Time, t2 Time)` - Check if t1 is after t2
- `is_zero(time Time)` - Check if time is zero value

## Time Formatting

- `format_rfc3339(time Time)` - Format as RFC3339 string
- `format_unix(time Time)` - Format as Unix timestamp string
- `format_human(time Time)` - Format in human-readable format

## Duration Operations

### Conversion Functions

- `duration_seconds(dur Duration)` - Convert to seconds
- `duration_millis(dur Duration)` - Convert to milliseconds
- `duration_micros(dur Duration)` - Convert to microseconds
- `duration_nanos(dur Duration)` - Convert to nanoseconds

### Arithmetic Functions

- `add_durations(d1, d2 Duration)` - Add two durations
- `sub_durations(d1, d2 Duration)` - Subtract two durations
- `multiply_duration(dur Duration, factor normie)` - Multiply by scalar
- `divide_duration(dur Duration, divisor normie)` - Divide by scalar

### Comparison Functions

- `duration_equal(d1, d2 Duration)` - Check equality
- `duration_less(d1, d2 Duration)` - Check if d1 < d2
- `duration_greater(d1, d2 Duration)` - Check if d1 > d2

## Utility Functions

- `sleep(dur Duration)` - Sleep for specified duration (simulated)

## Example Usage

```cursed
yeet "timez"

slay main() {
    fr fr Get current time
    sus now_time Time = timez.now()
    vibez.spillf("Current time: %s", timez.format_rfc3339(now_time))
    
    fr fr Create duration
    sus one_hour Duration = timez.seconds(3600)
    sus thirty_mins Duration = timez.minutes(30)
    
    fr fr Time arithmetic
    sus future_time Time = timez.add_duration(now_time, one_hour)
    sus past_time Time = timez.sub_duration(now_time, thirty_mins)
    
    fr fr Time comparison
    lowkey timez.is_before(past_time, now_time) {
        vibez.spill("Past time is before current time")
    }
    
    fr fr Duration operations
    sus total Duration = timez.add_durations(one_hour, thirty_mins)
    sus total_minutes normie = timez.duration_seconds(total) / 60
    vibez.spillf("Total duration: %d minutes", total_minutes)
    
    fr fr Format times
    sus rfc_time tea = timez.format_rfc3339(now_time)
    sus unix_time tea = timez.format_unix(now_time)
    sus human_time tea = timez.format_human(now_time)
    
    vibez.spillf("RFC3339: %s", rfc_time)
    vibez.spillf("Unix: %s", unix_time)
    vibez.spillf("Human: %s", human_time)
    
    fr fr Sleep for 2 seconds
    sus sleep_duration Duration = timez.seconds(2)
    timez.sleep(sleep_duration)
    vibez.spill("Finished sleeping")
}
```

## Time Zones and Formatting

The current implementation uses UTC time. Future versions will include:

- Time zone support
- Locale-aware formatting
- Custom time formats
- Daylight saving time handling

## RFC3339 Compliance

The module supports RFC3339 time format:
- `2024-07-13T12:34:56Z` (UTC)
- `2024-07-13T12:34:56-07:00` (with timezone offset)

## Implementation Details

- Pure CURSED implementation with nanosecond precision
- Unix timestamp-based time representation
- Efficient duration arithmetic using nanoseconds
- Thread-safe time operations
- Cross-platform compatibility

## Current Limitations

This is a foundational implementation with some simplifications:

- Time zone support is UTC-only
- RFC3339 parsing is simplified
- Sleep function is simulated (busy wait)
- Clock resolution depends on system capabilities

## Testing

Run tests with:
```bash
cargo run --bin cursed stdlib/timez/test_timez.csd
```

The test suite covers time creation, arithmetic, comparison, formatting, duration operations, and edge cases.

## Future Enhancements

- Full time zone database support
- High-resolution monotonic clocks
- Timer and ticker functionality
- Time zone conversion utilities
- Advanced date/time parsing
- Performance optimization for time-critical applications
