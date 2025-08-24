# CURSED Advanced Timez Module Documentation

## Overview

The CURSED Timez module provides comprehensive time and date operations with advanced features including:

- **Duration arithmetic** with nanosecond precision
- **Timer and Ticker** functionality with goroutine integration
- **Advanced parsing and formatting** with multiple layout support
- **Comprehensive timezone database** with DST transitions
- **Rate limiting and scheduling** capabilities
- **Benchmarking and performance** measurement tools

## Quick Start

```cursed
yeet "timez"

fr fr Basic time operations
sus now DateTime = time_now()
sus utc DateTime = time_utc_now()

fr fr Duration operations
sus duration Duration = duration_minutes(30)
sus future_time DateTime = time_add_duration(now, duration)

fr fr Timer operations
sus timer Timer = new_timer(duration_seconds(5))
<- timer.channel  fr fr Wait for timer

fr fr Parsing and formatting
sus rfc_time DateTime = time_parse_rfc3339("2024-01-02T15:04:05Z")
sus formatted tea = time_format_rfc3339(now)
```

## Duration System

### Creating Durations

```cursed
fr fr Basic duration creation
sus ns Duration = duration_nanoseconds(1000000)
sus us Duration = duration_microseconds(1000)
sus ms Duration = duration_milliseconds(100)
sus s Duration = duration_seconds(30)
sus m Duration = duration_minutes(15)
sus h Duration = duration_hours(2)
sus d Duration = duration_days(1)
sus w Duration = duration_weeks(1)

fr fr Special durations
sus zero Duration = duration_zero()
sus max_dur Duration = duration_max()
```

### Parsing Durations

```cursed
fr fr Parse from strings
sus d1 Duration = parse_duration("5s")           fr fr 5 seconds
sus d2 Duration = parse_duration("10m30s")       fr fr 10 minutes 30 seconds
sus d3 Duration = parse_duration("1h30m45s")     fr fr 1 hour 30 minutes 45 seconds
sus d4 Duration = parse_duration("2d12h")        fr fr 2 days 12 hours
sus d5 Duration = parse_duration("-1h")          fr fr Negative 1 hour

fr fr Must parse (panics on error)
sus required Duration = must_parse_duration("1h30m")
```

### Duration Arithmetic

```cursed
sus d1 Duration = duration_minutes(30)
sus d2 Duration = duration_minutes(15)

fr fr Arithmetic operations
sus sum Duration = duration_add(d1, d2)         fr fr 45 minutes
sus diff Duration = duration_sub(d1, d2)        fr fr 15 minutes
sus mult Duration = duration_mul(d1, 2)         fr fr 60 minutes
sus div Duration = duration_div(d1, 3)          fr fr 10 minutes

fr fr Other operations
sus abs_dur Duration = duration_abs(d1)         fr fr Absolute value
sus neg_dur Duration = duration_negate(d1)      fr fr Negate
```

### Duration Comparison

```cursed
sus d1 Duration = duration_minutes(30)
sus d2 Duration = duration_minutes(45)

assert(duration_less(d1, d2))
assert(duration_greater(d2, d1))
assert(duration_equal(d1, d1))
assert(duration_less_equal(d1, d2))
assert(duration_greater_equal(d2, d1))
```

### Duration Conversion and Formatting

```cursed
sus d Duration = parse_duration("1h30m45.123s")

fr fr Extract values
sus hours drip = duration_hours_value(d)        fr fr 1
sus minutes drip = duration_minutes_value(d)    fr fr 90 (total)
sus seconds drip = duration_seconds_value(d)    fr fr 5445 (total)
sus ms drip = duration_milliseconds_value(d)    fr fr 5445123 (total)

fr fr Formatting
sus str tea = duration_string(d)                fr fr "1h30m45.123s"

fr fr Utilities
assert(duration_is_positive(d))
assert(!duration_is_negative(d))
assert(!duration_is_zero(d))

fr fr Rounding and truncation
sus truncated Duration = duration_truncate(d, SECOND)
sus rounded Duration = duration_round(d, SECOND)
```

## Timer and Ticker System

### Basic Timer Operations

```cursed
fr fr Create and use timers
sus timer Timer = new_timer(duration_seconds(5))
<- timer.channel  fr fr Blocks until timer expires

fr fr One-shot timer with channel
sus after_ch chan<lit> = after(duration_seconds(1))
<- after_ch

fr fr Schedule function execution
sus scheduled_timer Timer = after_func(duration_seconds(2), "my_callback")

fr fr Timer control
timer_reset(&timer, duration_seconds(10))
timer_stop(&timer)
```

### Ticker Operations

```cursed
fr fr Create ticker for regular intervals
sus ticker Ticker = new_ticker(duration_milliseconds(100))

fr fr Receive ticks
bestie (ticker.active) {
    <- ticker.channel  fr fr Receives every 100ms
    vibez.spill("Tick!", ticker.tick_count)
}

fr fr Ticker control
ticker_reset(&ticker, duration_milliseconds(200))
ticker_stop(&ticker)

fr fr Simple tick channel
sus tick_ch chan<lit> = tick(duration_seconds(1))
```

### Stopwatch and Benchmarking

```cursed
fr fr Stopwatch operations
sus sw StopWatch = new_stopwatch()

stopwatch_start(&sw)
fr fr ... do work ...
sus lap Duration = stopwatch_lap(&sw)
fr fr ... more work ...
sus final Duration = stopwatch_stop(&sw)

fr fr Check elapsed time without stopping
sus current_elapsed Duration = stopwatch_elapsed(sw)

fr fr Reset stopwatch
stopwatch_reset(&sw)

fr fr Benchmarking helpers
sus bench StopWatch = time_benchmark("my_operation")
fr fr ... operation to benchmark ...
sus result Duration = time_benchmark_end(&bench, "my_operation")
```

### Sleep and Scheduling

```cursed
fr fr Sleep operations
sleep(duration_seconds(1))                    fr fr Sleep for 1 second
sleep_until(1672531200000000000)             fr fr Sleep until timestamp
time_sleep_until(future_datetime)            fr fr Sleep until datetime

fr fr Timeout operations
sus success lit = with_timeout(duration_seconds(5), "long_operation")
sus timeout_success lit = time_timeout(duration_seconds(3), "test_op")
```

### Rate Limiting

```cursed
fr fr Create rate limiter: 10 ops/second, burst of 5
sus limiter RateLimiter = new_rate_limiter(10, 5)

fr fr Check if operation is allowed
ready (rate_limiter_allow(&limiter)) {
    fr fr Perform operation
    vibez.spill("Operation allowed")
} otherwise {
    vibez.spill("Rate limited")
}

fr fr Wait until operation is allowed
rate_limiter_wait(&limiter)
fr fr Operation will definitely be allowed now
```

## Advanced Parsing and Formatting

### Predefined Layouts

```cursed
facts LAYOUT_KITCHEN tea = "3:04PM"
facts LAYOUT_RFC822 tea = "02 Jan 06 15:04 MST"
facts LAYOUT_RFC1123 tea = "Mon, 02 Jan 2006 15:04:05 MST"
facts LAYOUT_RFC3339 tea = "2006-01-02T15:04:05Z07:00"
facts LAYOUT_RFC3339_NANO tea = "2006-01-02T15:04:05.999999999Z07:00"
facts LAYOUT_ISO8601 tea = "2006-01-02T15:04:05Z"
facts LAYOUT_DATE_ONLY tea = "2006-01-02"
facts LAYOUT_TIME_ONLY tea = "15:04:05"
facts LAYOUT_AMERICAN tea = "01/02/2006"
facts LAYOUT_EUROPEAN tea = "02/01/2006"
```

### Advanced Parsing

```cursed
fr fr Parse with specific layout
sus parsed ParsedTime = parse_time_advanced(LAYOUT_RFC3339, "2024-01-02T15:04:05Z")
ready (string_empty(parsed.parse_error)) {
    sus dt DateTime = parsed_time_to_datetime(parsed)
}

fr fr Parse in specific timezone
sus tz_parsed ParsedTime = parse_time_in_location(
    LAYOUT_RFC3339, 
    "2024-01-02T15:04:05-05:00", 
    "America/New_York"
)

fr fr Flexible parsing (tries to guess format)
sus flexible ParsedTime = parse_flexible("2024/01/02 15:04:05")

fr fr Convenience functions for common formats
sus rfc_time DateTime = time_parse_rfc3339("2024-01-02T15:04:05Z")
sus iso_time DateTime = time_parse_iso8601("2024-01-02T15:04:05Z")
```

### Advanced Formatting

```cursed
fr fr Format with context
sus context FormatContext = create_format_context()
context.use_12_hour = based
context.include_timezone = based
context.precision = 3

sus formatted tea = format_time_advanced(dt, LAYOUT_RFC3339, context)

fr fr Convenience formatting functions
sus rfc_str tea = time_format_rfc3339(dt)
sus iso_str tea = time_format_iso8601(dt)
sus kitchen_str tea = time_format_kitchen(dt)     fr fr "3:04PM"

fr fr Custom pattern formatting
sus custom tea = format_with_pattern(dt, "YYYY-MM-DD HH:mm:ss", context)
```

### Format Contexts

```cursed
sus context FormatContext = create_format_context()
context.locale = "en_US"                     fr fr Locale for names
context.use_12_hour = based                  fr fr Use 12-hour format
context.include_timezone = based             fr fr Include timezone info
context.precision = 6                       fr fr Microsecond precision
context.custom_separator = " | "            fr fr Custom separators
```

## Comprehensive Timezone Database

### Timezone Operations

```cursed
fr fr Initialize database (done automatically)
initialize_timezone_database()

fr fr Find timezone information
sus ny_info *TimezoneInfo = find_timezone("America/New_York")
sus london_info *TimezoneInfo = find_timezone("Europe/London")

fr fr Validate timezones
assert(is_valid_timezone("America/New_York"))
assert(!is_valid_timezone("Invalid/Zone"))

fr fr Get timezone offsets
sus current_time drip = get_current_time_ns() / 1000000
sus ny_offset drip = get_timezone_offset_at_time("America/New_York", current_time)
sus london_offset drip = get_timezone_offset_at_time("Europe/London", current_time)
```

### DST and Transitions

```cursed
fr fr Check DST status
sus is_dst lit = is_dst_active_at_time(ny_info, current_time)

fr fr Get next DST transition
sus next_transition TimezoneTransition = get_next_dst_transition("America/New_York", current_time)

fr fr Calculate time until DST change
sus time_until_change drip = calculate_time_until_dst_change("America/New_York", current_time)
```

### Timezone Conversions

```cursed
fr fr Convert between timezones
sus utc_timestamp drip = get_current_time_ns() / 1000000
sus ny_timestamp drip = convert_timezone_timestamp(utc_timestamp, "UTC", "America/New_York")

fr fr DateTime timezone conversions (from main module)
sus utc_dt DateTime = time_to_utc(local_dt)
sus local_dt DateTime = time_from_utc(utc_dt, "America/Los_Angeles")
sus converted DateTime = time_change_timezone(dt, "Europe/Paris")
```

### Timezone Queries

```cursed
fr fr List available timezones
sus zones []tea = list_available_timezones()

fr fr Find timezones by offset
sus utc_zones []tea = get_timezone_by_offset(0)        fr fr UTC zones
sus est_zones []tea = get_timezone_by_offset(-18000)   fr fr EST zones

fr fr Get detailed information
sus detailed_info TimezoneInfo = get_timezone_info_detailed("America/New_York", current_time)

fr fr Database statistics
print_timezone_database_stats()
test_timezone_database()  fr fr Run comprehensive tests
```

## Integration with Main Timez Module

### Enhanced DateTime Operations

```cursed
fr fr Duration-based datetime arithmetic
sus future DateTime = time_add_duration(now, duration_hours(5))
sus past DateTime = time_sub_duration(now, duration_days(2))

fr fr Duration calculations
sus until_future Duration = time_duration_until(future)
sus since_past Duration = time_duration_since(past)

fr fr Create datetime in timezone
sus ny_time DateTime = time_create_with_timezone(2024, 12, 25, 15, 30, 0, "America/New_York")

fr fr Sleep until specific time
time_sleep_until(future)
```

### Timer Integration

```cursed
fr fr Wrapper functions for consistency
sus after_ch chan<lit> = time_after(duration_seconds(5))
sus tick_ch chan<lit> = time_tick(duration_milliseconds(100))
sus timeout_result lit = time_timeout(duration_seconds(10), "operation")
sus scheduled Timer = time_schedule_func(duration_minutes(5), "cleanup")
```

## Performance Considerations

### Efficient Duration Operations

- Duration arithmetic operates at nanosecond precision
- String parsing is optimized for common formats
- Memory allocation is minimized for duration operations

### Timer System Performance

- Timers use lightweight goroutines
- Channel operations are non-blocking where possible
- Rate limiting uses efficient token bucket algorithm

### Timezone Database Efficiency

- Database is loaded once and cached
- DST calculations are optimized for common years
- Timezone lookups use efficient data structures

## Common Patterns

### Timeout with Select

```cursed
sus operation_ch chan<lit> = make_channel_buffered(1)
sus timeout_ch chan<lit> = time_after(duration_seconds(30))

fr fr Start operation in goroutine
go {
    fr fr ... perform operation ...
    operation_ch <- based
}

select {
    case <- operation_ch:
        vibez.spill("Operation completed")
    case <- timeout_ch:
        vibez.spill("Operation timed out")
}
```

### Rate-Limited Processing

```cursed
sus limiter RateLimiter = new_rate_limiter(10, 5)  fr fr 10/sec, burst 5
sus items []tea = get_items_to_process()

bestie (item in items) {
    rate_limiter_wait(&limiter)
    process_item(item)
}
```

### Benchmarking Code

```cursed
slay benchmark_my_function() {
    sus sw StopWatch = time_benchmark("my_function")
    
    sus i drip = 0
    bestie (i < 1000) {
        my_function()
        i = i + 1
    }
    
    sus elapsed Duration = time_benchmark_end(&sw, "my_function")
    vibez.spill("Average per operation:", duration_string(duration_div(elapsed, 1000)))
}
```

### Scheduling Tasks

```cursed
fr fr Schedule daily task at 3 AM
sus now DateTime = time_now()
sus tomorrow_3am DateTime = time_create(now.year, now.month, now.day + 1, 3, 0, 0)
sus until_3am Duration = time_duration_until(tomorrow_3am)

sus daily_timer Timer = time_schedule_func(until_3am, "daily_backup")
```

### Working with Different Time Formats

```cursed
fr fr Parse various formats
sus formats []tea = [
    LAYOUT_RFC3339,
    LAYOUT_RFC1123,
    LAYOUT_ISO8601,
    LAYOUT_AMERICAN,
    LAYOUT_EUROPEAN
]

sus input tea = get_time_string_from_user()
sus i drip = 0
bestie (i < len(formats)) {
    sus parsed ParsedTime = parse_time_advanced(formats[i], input)
    ready (string_empty(parsed.parse_error)) {
        sus dt DateTime = parsed_time_to_datetime(parsed)
        vibez.spill("Successfully parsed as:", formats[i])
        break
    }
    i = i + 1
}
```

## Error Handling

### Duration Parsing Errors

```cursed
sus duration Duration = parse_duration("invalid")
ready (duration_is_zero(duration)) {
    vibez.spill("Failed to parse duration")
    fr fr Use must_parse_duration if you want to panic on error
}
```

### Time Parsing Errors

```cursed
sus parsed ParsedTime = parse_time_advanced(LAYOUT_RFC3339, "invalid time")
ready (!string_empty(parsed.parse_error)) {
    vibez.spill("Parse error:", parsed.parse_error)
    fr fr Handle error appropriately
}
```

### Timezone Errors

```cursed
sus tz_info *TimezoneInfo = find_timezone("Invalid/Zone")
ready (tz_info == null) {
    vibez.spill("Invalid timezone")
    fr fr Fall back to UTC or handle error
}
```

## Testing

The comprehensive test suite is available in `comprehensive_timez_test.csd`:

```bash
./zig-out/bin/cursed-zig comprehensive_timez_test.csd
```

This tests all advanced time features including:
- Duration creation, parsing, and arithmetic
- Timer and ticker operations
- Stopwatch functionality
- Timezone database operations
- Advanced parsing and formatting
- Performance benchmarks
- Integration between modules

## Migration from Basic Timez

If you're upgrading from basic timez usage:

1. **Duration arithmetic**: Replace manual millisecond calculations with Duration operations
2. **Timer operations**: Replace `timer_start()` with `new_timer(duration)`  
3. **Parsing**: Replace `parse_iso8601()` with `parse_time_advanced()`
4. **Formatting**: Replace `time_format()` with `format_time_advanced()`
5. **Timezone operations**: Use the comprehensive timezone database

The enhanced module maintains backward compatibility with existing timez functions while providing significant new capabilities.
