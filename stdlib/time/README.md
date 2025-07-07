# CURSED Time Library Tests

This directory contains comprehensive tests for the CURSED time and date standard library.

## Test Coverage

The `test_time.csd` file provides complete test coverage for all time functions:

### Current Time Functions
- `time_now()` - Current Unix timestamp (seconds)
- `time_now_millis()` - Current time in milliseconds
- `time_now_micros()` - Current time in microseconds
- `time_now_nanos()` - Current time in nanoseconds

### Time Creation
- `time_from_timestamp()` - Create from Unix timestamp
- `time_from_millis()` - Create from milliseconds
- `time_create()` - Create from components (year, month, day, etc.)
- `time_parse()` - Parse from string with format

### Time Formatting
- `time_format()` - Format with custom format string
- `time_to_string()` - Default string representation
- `time_to_iso8601()` - ISO 8601 format
- `time_to_rfc3339()` - RFC 3339 format

### Time Components
- `time_year()` / `time_month()` / `time_day()` - Date components
- `time_hour()` / `time_minute()` / `time_second()` - Time components
- `time_weekday()` - Day of week (0=Sunday, 6=Saturday)
- `time_day_of_year()` - Day number in year (1-366)

### Time Arithmetic
- `time_add_years()` / `time_add_months()` / `time_add_days()` - Date arithmetic
- `time_add_hours()` / `time_add_minutes()` / `time_add_seconds()` - Time arithmetic
- `time_subtract()` - Time difference as duration
- `time_diff_days()` / `time_diff_hours()` / etc. - Specific difference units

### Duration Operations
- `duration_from_seconds()` / `duration_from_millis()` - Create duration
- `duration_to_seconds()` / `duration_to_millis()` - Convert duration
- `duration_add()` / `duration_subtract()` - Duration arithmetic

### Timezone Operations
- `time_utc()` / `time_local()` - Current time in UTC/local
- `time_to_utc()` / `time_to_local()` - Convert between timezones
- `time_timezone_offset()` - Get timezone offset in seconds

### Time Validation
- `time_is_leap_year()` - Check if year is leap year
- `time_days_in_month()` - Days in specific month/year
- `time_is_valid_date()` - Validate date components
- `time_is_weekend()` - Check if date is weekend

### Sleep and Timing
- `time_sleep()` - Sleep for seconds
- `time_sleep_millis()` - Sleep for milliseconds
- `time_sleep_micros()` - Sleep for microseconds

### Benchmarking
- `time_benchmark()` - Measure function execution time
- `time_measure()` - Execute function and return result + duration

### Time Constants
- `time_seconds_per_minute()` - 60
- `time_minutes_per_hour()` - 60
- `time_hours_per_day()` - 24
- `time_days_per_week()` - 7
- `time_months_per_year()` - 12
- `time_millis_per_second()` - 1000
- `time_micros_per_second()` - 1,000,000
- `time_nanos_per_second()` - 1,000,000,000

### Edge Cases Tested
- Epoch time (1970-01-01)
- Far future dates (2100+)
- Leap year handling (February 29)
- Year boundaries (December 31 → January 1)
- Different date formats
- Timezone edge cases

## Date/Time Concepts

The time library handles common temporal operations:

- **Unix Timestamps**: Seconds since January 1, 1970 UTC
- **Date Components**: Year, month, day with proper validation
- **Time Components**: Hours, minutes, seconds (24-hour format)
- **Durations**: Time spans that can be added/subtracted
- **Timezones**: UTC and local time conversion
- **Leap Years**: Proper handling of February 29

## Format Strings

Common format specifiers for parsing/formatting:

- `%Y` - 4-digit year (2021)
- `%m` - Month number (01-12)
- `%d` - Day of month (01-31)
- `%H` - Hour (00-23)
- `%M` - Minute (00-59)
- `%S` - Second (00-59)

## Running Tests

```bash
# Run time tests specifically
cargo run --bin cursed stdlib/time/test_time.csd

# Run all stdlib tests
cargo run --bin cursed test
```

## Test Results

All tests verify:
- Correct time calculations
- Proper date arithmetic
- Timezone handling
- Format parsing/generation
- Leap year calculations
- Duration operations
- Sleep timing accuracy
- Benchmarking functionality

The tests account for system timing variations and provide reasonable tolerances for timing-sensitive operations.

## Usage Examples

### Basic Time Operations
```cursed
sus now datetime = time_local()
sus year normie = time_year(now)
sus formatted tea = time_format(now, "%Y-%m-%d %H:%M:%S")
```

### Date Arithmetic
```cursed
sus birthday datetime = time_create(1990, 6, 15, 0, 0, 0)
sus next_year datetime = time_add_years(birthday, 1)
sus age_days normie = time_diff_days(time_local(), birthday)
```

### Duration Handling
```cursed
sus start datetime = time_local()
time_sleep(2)
sus end datetime = time_local()
sus elapsed duration = time_subtract(end, start)
sus seconds normie = duration_to_seconds(elapsed)
```

### Benchmarking
```cursed
slay expensive_operation() {
    // Some computation
    damn result
}

sus benchmark_time duration = time_benchmark(expensive_operation)
sus execution_ms normie = duration_to_millis(benchmark_time)
```

## Platform Considerations

- **System Time**: Relies on system clock accuracy
- **Timezone Data**: Uses system timezone information
- **Sleep Precision**: Sleep functions have platform-dependent precision
- **Leap Seconds**: Not handled (follows POSIX time)
- **Year Range**: Typically 1970-2038 on 32-bit systems, extended on 64-bit

## Important Notes

- **Always validate dates** before creating datetime objects
- **Handle timezone conversions** carefully for international applications
- **Use appropriate precision** (seconds vs milliseconds) for your use case
- **Account for leap years** in date calculations
- **Consider daylight saving time** when working with local times
