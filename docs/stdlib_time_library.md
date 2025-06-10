# CURSED Time & Date Library

The CURSED time library provides comprehensive time, date, and duration functionality for the CURSED programming language. This library offers everything from basic time operations to advanced benchmarking and performance measurement tools.

## Overview

The time library is organized into several modules:

- **Core Types**: `DateTime`, `Date`, `Time`, `Duration`, `Instant`
- **Formatting & Parsing**: String conversion and parsing for various formats
- **Timezone Support**: UTC, local time, and international timezone handling
- **Relative Time**: Human-readable time descriptions and calculations
- **Sleep & Timing**: Sleep functions, timers, and timeouts
- **Benchmarking**: Performance measurement and profiling tools

## Quick Start

```cursed
import "stdlib::time";

slay main() {
    // Get current time
    facts now_time = now()?;
    println(&format("Current time: {}", format_iso8601(&now_time)?))?;
    
    // Create a duration and add it
    facts in_2_hours = now_time.add_duration(hours(2))?;
    println(&format("In 2 hours: {}", format_iso8601(&in_2_hours)?))?;
    
    // Parse and format dates
    facts birthday = parse_iso8601("1990-05-15T10:30:00")?;
    println(&format("Birthday: {}", format_us(&birthday)?))?;
    
    // Measure performance
    facts (result, elapsed) = time_it(|| {
        // Some computation
        42 * 1337
    })?;
    println(&format("Computed {} in {}", result, elapsed.humanize()))?;
}
```

## Core Types

### DateTime

Represents a complete date and time.

```cursed
// Create from components
facts dt = DateTime::from_components(2023, 12, 25, 15, 30, 45, 0)?;

// Create from date and time
facts date = Date::new(2023, 12, 25)?;
facts time = Time::new(15, 30, 45, 0)?;
facts dt = DateTime::new(date, time);

// Get current time
facts now_dt = now()?;
facts utc_dt = utc_now()?;

// Convert to/from timestamps
facts timestamp = dt.to_timestamp();
facts from_ts = from_timestamp(timestamp)?;
```

### Date

Represents a calendar date (year, month, day).

```cursed
// Create a date
facts date = Date::new(2023, 12, 25)?;

// Date arithmetic
facts tomorrow = date.add_days(1)?;
facts last_week = date.add_days(-7)?;

// Get weekday
facts weekday = date.weekday();
println(&format("Today is {}", weekday.name()))?;

// Calendar functions
facts day_of_year = date.day_of_year();
facts week_of_year = date.week_of_year();
```

### Time

Represents a time of day (hour, minute, second, nanosecond).

```cursed
// Create a time
facts time = Time::new(15, 30, 45, 0)?;

// Create from seconds since midnight
facts time = Time::from_seconds(54645)?; // 15:10:45

// Convert to seconds
facts seconds = time.to_seconds();
facts nanoseconds = time.to_nanoseconds();
```

### Duration

Represents a length of time.

```cursed
// Create durations
facts d1 = seconds(30);
facts d2 = minutes(5);
facts d3 = hours(2);
facts d4 = days(7);
facts d5 = weeks(2);

// Duration arithmetic
facts total = d1.add(&d2)?;
facts difference = d3.subtract(&d2)?;
facts doubled = d1.multiply(2)?;
facts halved = d1.divide(2)?;

// Parse from text
facts parsed = parse_duration("2h 30m 15s")?;

// Human readable
println(&format("Duration: {}", d3.humanize()))?; // "2 hours"
```

### Instant

High-precision timestamp for measuring elapsed time.

```cursed
// Create an instant
facts start = Instant::now();

// Do some work...
sleep(milliseconds(100))?;

// Measure elapsed time
facts elapsed = start.elapsed();
println(&format("Elapsed: {}", elapsed.humanize()))?;
```

## Formatting and Parsing

### Supported Formats

- **ISO 8601**: `2023-12-25T15:30:45`
- **RFC 3339**: `2023-12-25T15:30:45Z`
- **RFC 2822**: `Mon, 25 Dec 2023 15:30:45 +0000`
- **US Format**: `12/25/2023 3:30:45 PM`
- **European**: `25/12/2023 15:30:45`
- **Custom formats**: Using format strings

### DateTime Formatting

```cursed
facts dt = DateTime::from_components(2023, 12, 25, 15, 30, 45, 0)?;

// Standard formats
facts iso = format_iso8601(&dt)?;           // "2023-12-25T15:30:45"
facts rfc = format_rfc3339(&dt)?;           // "2023-12-25T15:30:45Z"
facts us = format_us(&dt)?;                 // "12/25/2023 3:30:45 PM"
facts eu = format_european(&dt)?;           // "25/12/2023 15:30:45"

// Custom format
facts custom = format_custom(&dt, "%A, %B %d, %Y at %I:%M %p")?;
// "Monday, December 25, 2023 at 03:30 PM"
```

### DateTime Parsing

```cursed
// Parse standard formats
facts dt1 = parse_iso8601("2023-12-25T15:30:45")?;
facts dt2 = parse_rfc3339("2023-12-25T15:30:45Z")?;
facts dt3 = parse_us("12/25/2023 3:30:45 PM")?;
facts dt4 = parse_european("25/12/2023 15:30:45")?;

// Parse dates
facts date1 = parse_date("2023-12-25", "%Y-%m-%d")?;
facts date2 = parse_date("12/25/2023", "%m/%d/%Y")?;

// Parse times
facts time1 = parse_time("15:30:45", "%H:%M:%S")?;
facts time2 = parse_time("3:30:45 PM", "%I:%M:%S %p")?;
```

### Duration Parsing

```cursed
// Parse duration expressions
facts d1 = parse_duration("2h")?;          // 2 hours
facts d2 = parse_duration("30m")?;         // 30 minutes
facts d3 = parse_duration("1d 4h 30m")?;   // 1 day, 4 hours, 30 minutes
facts d4 = parse_duration("45s")?;         // 45 seconds
facts d5 = parse_duration("2w 3d")?;       // 2 weeks, 3 days
```

## Timezone Support

### Basic Timezone Operations

```cursed
// Get predefined timezones
facts utc_tz = utc();
facts local_tz = local_timezone()?;

// Get timezone by name
facts est_tz = timezone_by_name("EST")?;
facts jst_tz = timezone_by_name("JST")?;
facts cet_tz = timezone_by_name("CET")?;

// Get timezone by offset
facts offset = UtcOffset::from_hours(-5);
facts custom_tz = timezone_by_offset(offset);

// List available timezones
facts available = list_timezones();
```

### Timezone Conversion

```cursed
facts dt = now()?;
facts utc_tz = utc();
facts est_tz = timezone_by_name("EST")?;

// Convert from UTC to EST
facts est_time = convert_timezone(&dt, &utc_tz, &est_tz)?;

// Parse offset from string
facts offset1 = parse_offset("+05:30")?;  // India Standard Time
facts offset2 = parse_offset("-08:00")?;  // Pacific Standard Time
facts offset3 = parse_offset("Z")?;       // UTC
```

### Supported Timezones

The library includes support for major timezones:

- **UTC/GMT**: Universal Coordinated Time
- **US Timezones**: EST, EDT, CST, CDT, MST, MDT, PST, PDT
- **European**: CET, CEST, EET, EEST, WET, WEST
- **Asian**: JST, KST, CST (China), IST (India)
- **Australian**: AEST, AEDT, ACST, ACDT, AWST

## Relative Time

### Human-Readable Time Descriptions

```cursed
facts past_time = now()?.subtract_duration(hours(2))?;
facts future_time = now()?.add_duration(days(3))?;

// Generate relative descriptions
facts ago_desc = relative_time(&past_time)?;      // "2 hours ago"
facts future_desc = relative_time(&future_time)?; // "in 3 days"

// Direct relative functions
facts ago_text = time_ago(&hours(2))?;            // "2 hours ago"
facts future_text = time_from_now(&days(3))?;     // "in 3 days"

// Detailed relative formatting
facts detailed = format_relative(&past_time, true)?;
```

### Parsing Relative Expressions

```cursed
// Parse relative time expressions
facts rel1 = parse_relative("2 hours ago")?;
facts rel2 = parse_relative("in 3 days")?;
facts rel3 = parse_relative("now")?;

// Check relative time type
if rel1.is_past() {
    println("This is in the past")?;
}
```

### Calendar Navigation

```cursed
// Find next/previous occurrences
facts next_monday = next_occurrence(Weekday::Monday)?;
facts prev_friday = previous_occurrence(Weekday::Friday)?;

// Business day operations
facts next_bday = next_business_day(&today())?;
facts prev_bday = previous_business_day(&today())?;

// Weekend detection
facts is_weekend = is_weekend(&today());
facts is_weekday = is_weekday(&today());
```

## Sleep and Timing

### Sleep Functions

```cursed
// Sleep for specific durations
sleep(seconds(5))?;               // Sleep for 5 seconds
sleep_millis(1500)?;              // Sleep for 1.5 seconds
sleep_micros(500000)?;            // Sleep for 0.5 seconds
sleep_nanos(1000000000)?;         // Sleep for 1 second

// Sleep until specific time
facts target = now()?.add_duration(minutes(5))?;
sleep_until(target)?;

// Delay (alias for sleep)
delay(milliseconds(100))?;
```

### Timeouts

```cursed
// Execute function with timeout
facts result = timeout(seconds(10), || {
    // Some potentially long-running operation
    expensive_computation()
})?;

match result {
    Some(value) => println(&format("Result: {}", value))?,
    None => println("Operation timed out")?,
}
```

### Timers

```cursed
// Basic timer
facts timer = Timer::new();
do_some_work();
facts elapsed = timer.elapsed();

// Timer with duration
facts timer = Timer::with_duration(minutes(5));
loop {
    if timer.is_expired() {
        break;
    }
    do_work_chunk();
}

// Named timer
facts mut timer = Timer::with_name("Processing Timer".to_string());
timer.wait()?; // Wait until timer expires
```

### Stopwatch

```cursed
// Basic stopwatch
sus mut stopwatch = Stopwatch::new();

stopwatch.start();
do_phase_1();

facts lap1 = stopwatch.lap();
println(&format("Phase 1: {}", lap1.humanize()))?;

do_phase_2();

facts lap2 = stopwatch.lap();
println(&format("Phase 2: {}", lap2.humanize()))?;

stopwatch.stop();

// Analysis
facts total = stopwatch.elapsed();
facts avg_lap = stopwatch.average_lap_time();
facts fastest = stopwatch.fastest_lap();
facts slowest = stopwatch.slowest_lap();
```

## Benchmarking and Performance

### Simple Benchmarking

```cursed
// Time a single execution
facts (result, duration) = time_it(|| {
    expensive_operation()
})?;

// Measure execution time
facts duration = measure_time(|| {
    some_operation();
});

// Simple benchmark
facts bench_result = benchmark("operation_name", 1000, || {
    the_operation_to_benchmark()
})?;
```

### Advanced Benchmarking

```cursed
// Create benchmark runner
sus mut bench = Benchmark::new();

// Configure benchmarking
facts config = BenchmarkConfig {
    warmup_iterations: 10,
    measurement_iterations: 1000,
    min_execution_time: milliseconds(1),
    max_execution_time: seconds(10),
    collect_all_samples: true,
};

sus mut bench = Benchmark::with_config(config);

// Run benchmarks
facts result1 = bench.bench("fast_algorithm", || fast_algo())?;
facts result2 = bench.bench("slow_algorithm", || slow_algo())?;

// Generate comparison
facts results = vec![result1, result2];
facts comparison = compare_benchmarks(&results);
println(&comparison)?;
```

### Performance Counters

```cursed
// Create performance counter
sus mut counter = PerformanceCounter::new();

// Count operations
counter.increment("database_queries");
counter.add("bytes_processed", 1024);

// Time operations
counter.start_timer("processing");
do_processing();
counter.stop_timer("processing");

// Generate report
facts report = counter.report();
println(&report)?;
```

### Benchmark Results Analysis

```cursed
facts result = benchmark("test", 1000, || test_function())?;

// Basic statistics
println(&format("Average: {}", result.average_time.humanize()))?;
println(&format("Min: {}", result.min_time.humanize()))?;
println(&format("Max: {}", result.max_time.humanize()))?;
println(&format("Median: {}", result.median_time.humanize()))?;
println(&format("Std Dev: {}", result.std_deviation.humanize()))?;

// Throughput
if facts throughput = result.throughput {
    println(&format("Throughput: {:.2} ops/sec", throughput))?;
}

// Percentiles
facts p95 = result.percentile(95.0)?;
facts p99 = result.percentile(99.0)?;

// Performance bounds checking
facts within_bounds = result.is_within_bounds(
    milliseconds(10),  // max average
    milliseconds(5)    // max std dev
);
```

## Utility Functions

### Calendar Utilities

```cursed
// Leap year detection
facts is_leap = is_leap_year(2024);

// Days in month
facts days = days_in_month(2024, 2); // 29 for leap year

// Day/week of year
facts day_of_year = day_of_year(2023, 12, 25);
facts week_of_year = week_of_year(2023, 12, 25);
```

### Wait Operations

```cursed
// Wait for condition with timeout
facts success = wait_for(
    || check_condition(),
    seconds(30),        // timeout
    milliseconds(100)   // check interval
)?;

// Retry with delays
facts result = retry_with_delay(
    || potentially_failing_operation(),
    5,                  // max attempts
    seconds(1)          // delay between attempts
);
```

## Error Handling

The time library uses comprehensive error handling with the `TimeResult<T>` type:

```cursed
// All operations that can fail return TimeResult<T>
match parse_iso8601("invalid-date") {
    Ok(datetime) => println(&format("Parsed: {}", format_iso8601(&datetime)?))?,
    Err(TimeError::ParseError { input, expected_format, message, .. }) => {
        println(&format("Parse failed: {} (expected: {})", message, expected_format))?;
    },
    Err(err) => println(&format("Other error: {}", err))?,
}

// Error types include:
// - InvalidDate: Invalid date values
// - InvalidTime: Invalid time values  
// - ParseError: String parsing failures
// - TimezoneError: Timezone operation failures
// - ArithmeticOverflow: Math operation overflows
// - FormatError: Format string errors
// - SystemTimeError: System clock errors
// - DurationError: Duration operation errors
```

## Examples

### Event Scheduling System

```cursed
// Schedule events with relative times
facts events = vec![
    ("Meeting", parse_relative("in 2 hours")?),
    ("Lunch", parse_relative("in 4 hours")?),
    ("Conference call", parse_relative("tomorrow at 9am")?),
];

lowkey event in events {
    facts (name, rel_time) = event;
    if rel_time.is_future() {
        println(&format("{}: {}", name, time_from_now(&rel_time.duration().unwrap())?))?;
    }
}
```

### Performance Monitoring

```cursed
// Monitor application performance
sus mut perf = PerformanceCounter::new();

perf.start_timer("request_processing");

// Process request
handle_request();
perf.increment("requests_processed");

perf.stop_timer("request_processing");

// Log performance metrics
if perf.get_timer("request_processing") > seconds(1) {
    println("Slow request detected")?;
}
```

### Time Zone Converter

```cursed
slay convert_time_zones(time_str: &str, from_tz: &str, to_tz: &str) -> TimeResult<String> {
    facts dt = parse_iso8601(time_str)?;
    facts from_timezone = timezone_by_name(from_tz)?;
    facts to_timezone = timezone_by_name(to_tz)?;
    
    facts converted = convert_timezone(&dt, &from_timezone, &to_timezone)?;
    Ok(format_iso8601(&converted)?)
}

// Usage
facts result = convert_time_zones("2023-12-25T15:30:45", "UTC", "EST")?;
println(&format("EST time: {}", result))?;
```

## Best Practices

1. **Always handle errors**: Time operations can fail, so use proper error handling
2. **Use appropriate precision**: Choose the right duration unit for your use case
3. **Consider timezones**: Be explicit about timezone handling in date/time operations
4. **Benchmark responsibly**: Use appropriate iteration counts and warmup periods
5. **Cache parsed formats**: Avoid repeatedly parsing the same time strings
6. **Use relative time for UX**: Human-readable relative times improve user experience

## Performance Considerations

- Duration arithmetic is constant time
- Date calculations may involve calendar logic (slower for large ranges)
- Timezone conversions require lookup operations
- Benchmarking adds measurement overhead
- String parsing/formatting involves allocation and computation

The time library provides high-performance implementations suitable for production use, with careful attention to memory allocation and computational efficiency.
