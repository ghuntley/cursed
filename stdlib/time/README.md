# CURSED Time Module

A comprehensive time handling library for CURSED applications, implementing the timez specification with Gen Z slang function names and pure CURSED implementation.

## Overview

The time module provides time operations with nanosecond precision, RFC3339 support, duration arithmetic, and timezone handling. All implementations are pure CURSED code without FFI dependencies.

## Core Types

### Time
- Represents an instant in time as Unix timestamp
- 64-bit integer precision (thicc type)
- Unix epoch based (seconds since January 1, 1970 UTC)

### Duration
- Represents a span of time in nanoseconds
- 64-bit integer precision (thicc type)
- Supports arithmetic operations

## Core Functions

### Time Creation
```cursed
yeet "time"

// Get current time
sus current_time Time = time.now()

// Create time from Unix timestamp
sus epoch Time = time.unix(0)
sus custom Time = time.unix(1704067200)

// Parse RFC3339 time string
sus parsed Time = time.parse_rfc3339("2024-01-01T00:00:00Z")

// Create time from components
sus new_year Time = time.time_create(2024, 1, 1, 0, 0, 0)
```

### Duration Creation
```cursed
// Create durations from various units
sus five_sec Duration = time.seconds(5)
sus hundred_ms Duration = time.milliseconds(100)
sus fifty_us Duration = time.microseconds(50)
sus ten_ns Duration = time.nanoseconds(10)

// Duration constants
sus one_second Duration = time.duration_second()
sus one_minute Duration = time.duration_minute()
sus one_hour Duration = time.duration_hour()
sus one_day Duration = time.duration_day()
```

### Time Arithmetic
```cursed
// Add/subtract durations
sus future Time = time.add_duration(current_time, five_sec)
sus past Time = time.sub_duration(current_time, five_sec)

// Calculate time differences
sus diff Duration = time.time_diff(future, past)

// Add specific time units
sus plus_seconds Time = time.time_add_seconds(current_time, 30)
sus plus_minutes Time = time.time_add_minutes(current_time, 5)
sus plus_hours Time = time.time_add_hours(current_time, 2)
sus plus_days Time = time.time_add_days(current_time, 1)
```

### Duration Arithmetic
```cursed
// Duration operations
sus total Duration = time.duration_add(five_sec, hundred_ms)
sus difference Duration = time.duration_subtract(five_sec, hundred_ms)

// Convert durations
sus in_seconds normie = time.duration_to_seconds(total)
sus in_millis normie = time.duration_to_millis(total)
```

### Time Comparison
```cursed
// Time comparisons
sus is_earlier lit = time.is_before(past, future)
sus is_later lit = time.is_after(future, past)
sus is_epoch lit = time.is_zero(epoch)

// Time utilities
sus equal lit = time.time_equals(time1, time2)
sus earliest Time = time.time_min(time1, time2)
sus latest Time = time.time_max(time1, time2)
```

### Time Components
```cursed
// Extract time components
sus year normie = time.time_year(current_time)
sus month normie = time.time_month(current_time)
sus day normie = time.time_day(current_time)
sus hour normie = time.time_hour(current_time)
sus minute normie = time.time_minute(current_time)
sus second normie = time.time_second(current_time)
```

### Date Validation
```cursed
// Leap year detection
sus is_leap lit = time.time_is_leap_year(2024)

// Days in month
sus days normie = time.time_days_in_month(2024, 2) // 29 for leap year

// Date validation
sus valid lit = time.time_is_valid_date(2024, 2, 29) // true
sus invalid lit = time.time_is_valid_date(2021, 2, 29) // false
```

### Formatting
```cursed
// Format time as strings
sus rfc_string tea = time.format_rfc3339(current_time)
sus unix_string tea = time.format_unix(current_time)
sus human_string tea = time.format_human(current_time)
```

### Sleep Operations
```cursed
// Sleep for specified duration
sus short_nap Duration = time.milliseconds(100)
time.sleep(short_nap)
```

## Constants

The module defines several useful constants:

- `NANOS_PER_SECOND` - Nanoseconds in one second (1,000,000,000)
- `NANOS_PER_MILLI` - Nanoseconds in one millisecond (1,000,000)
- `NANOS_PER_MICRO` - Nanoseconds in one microsecond (1,000)
- `SECONDS_PER_MINUTE` - Seconds in one minute (60)
- `MINUTES_PER_HOUR` - Minutes in one hour (60)
- `HOURS_PER_DAY` - Hours in one day (24)
- `DAYS_PER_WEEK` - Days in one week (7)

## Extended Functions

### Extended Time Operations
```cursed
// Get current time in various formats
sus now_timestamp thicc = time.time_now()
sus now_millis thicc = time.time_now_millis()
sus now_nanos thicc = time.time_now_nanos()

// Create time from milliseconds
sus millis_time Time = time.time_from_millis(1704067200000)

// Create time from timestamp
sus ts_time Time = time.time_from_timestamp(1704067200)
```

### Duration Utilities
```cursed
// Create duration from seconds (alternative)
sus dur Duration = time.duration_from_seconds(120)

// Get duration since epoch
sus since_epoch Duration = time.since_epoch(current_time)
```

## Testing

The module includes comprehensive tests using the testz framework:

```bash
# Run interpretation mode
cargo run --bin cursed stdlib/time/test_time.csd

# Run compilation mode
cargo run --bin cursed -- compile stdlib/time/test_time.csd
./test_time
```

## Implementation Notes

### Pure CURSED Implementation
- No FFI dependencies - completely implemented in CURSED
- Thread-safe operations suitable for concurrent use
- Nanosecond precision for all operations
- 64-bit integer representation for timestamps and durations

### Precision and Accuracy
- All time calculations use nanosecond precision
- Unix timestamp based for compatibility
- Overflow protection for duration arithmetic
- Consistent behavior across platforms

### Compatibility
- RFC3339 standard compliance for time parsing/formatting
- Unix timestamp compatibility for interoperability
- ISO 8601 parsing support
- Standard time component extraction

### Error Handling
- Invalid time parsing returns zero time
- Duration overflow protection with wraparound semantics
- Graceful handling of edge cases
- Validation functions for date components

## Performance

- O(1) time complexity for all operations
- Minimal memory allocation
- Optimized for high-frequency usage
- Efficient arithmetic operations using native integer math

## Thread Safety

All time module functions are thread-safe and can be used in concurrent goroutines without additional synchronization.

## Examples

### Basic Time Usage
```cursed
yeet "time"
yeet "testz"

slay example_basic_time() {
    // Get current time
    sus now Time = time.now()
    vibez.spill("Current time: ")
    vibez.spill(time.format_unix(now))
    
    // Create a duration
    sus five_minutes Duration = time.minutes(5)
    
    // Calculate future time
    sus later Time = time.add_duration(now, five_minutes)
    vibez.spill("Five minutes later: ")
    vibez.spill(time.format_unix(later))
    
    // Check if times are in order
    if time.is_before(now, later) {
        vibez.spill("Time flows forward correctly!")
    }
}
```

### Duration Calculations
```cursed
slay example_duration_math() {
    // Create various durations
    sus work_day Duration = time.hours(8)
    sus lunch_break Duration = time.minutes(30)
    sus coffee_break Duration = time.minutes(15)
    
    // Calculate total break time
    sus total_breaks Duration = time.duration_add(lunch_break, coffee_break)
    
    // Calculate actual work time
    sus actual_work Duration = time.duration_subtract(work_day, total_breaks)
    
    vibez.spill("Work day: ")
    vibez.spill(time.duration_to_minutes(work_day))
    vibez.spill(" minutes")
    
    vibez.spill("Actual work: ")
    vibez.spill(time.duration_to_minutes(actual_work))
    vibez.spill(" minutes")
}
```

### Date Validation Example
```cursed
slay example_date_validation() {
    // Check various dates
    sus dates = [
        [2024, 2, 29],  // Leap year - valid
        [2023, 2, 29],  // Not leap year - invalid
        [2024, 4, 31],  // April doesn't have 31 days - invalid
        [2024, 12, 25]  // Christmas - valid
    ]
    
    for date_info in dates {
        sus year normie = date_info[0]
        sus month normie = date_info[1]
        sus day normie = date_info[2]
        
        if time.time_is_valid_date(year, month, day) {
            vibez.spill("Valid date: ")
        } else {
            vibez.spill("Invalid date: ")
        }
        vibez.spill(year)
        vibez.spill("-")
        vibez.spill(month)
        vibez.spill("-")
        vibez.spill(day)
    }
}
```

This module provides a comprehensive foundation for time-based operations in CURSED applications while maintaining the language's unique style and pure implementation philosophy.
