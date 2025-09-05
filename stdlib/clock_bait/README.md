# Clock Bait (Time) Module

A comprehensive time and duration library for CURSED that provides Gen Z-style time manipulation with nanosecond precision.

## Overview

The `clock_bait` module provides functionality for measuring and displaying time with Gen Z flair. It includes time creation, arithmetic, comparison, formatting, and social media-style time representations.

## Features

- **Nanosecond precision**: All time operations are accurate to the nanosecond
- **Duration constants**: Pre-defined constants for common time intervals
- **Time arithmetic**: Add, subtract, and compare time values
- **Social formatting**: Format time for social media display ("2h ago", "just now")
- **Time spans**: Work with time ranges and intervals
- **Weekend detection**: Check for weekends and calculate next weekend
- **Gen Z terminology**: Uses modern slang for time-related operations

## Core Constants

### Duration Constants
```cursed
facts NanoBlink normie = 1                    // 1 nanosecond
facts MicroBlink normie = 1000               // 1 microsecond
facts MilliBlink normie = 1000000           // 1 millisecond
facts Blink normie = 1000000000             // 1 second
facts SecondVibe normie = Blink             // 1 second
facts MinuteVibe normie = 60 * SecondVibe   // 1 minute
facts HourVibe normie = 60 * MinuteVibe     // 1 hour
facts DayVibe normie = 24 * HourVibe        // 1 day
facts WeekVibe normie = 7 * DayVibe         // 1 week
```

### Month Constants
```cursed
facts VibeJanuary normie = 1
facts VibeFebruary normie = 2
// ... up to VibeDecember = 12
```

### Weekday Constants
```cursed
facts VibeSunday normie = 0
facts VibeMonday normie = 1
// ... up to VibeSaturday = 6
```

## Time Creation Functions

### `Now() normie`
Get the current time as a Unix timestamp in nanoseconds.

```cursed
current_time := clock_bait.Now()
```

### `Unix(sec normie, nsec normie) normie`
Create a time from Unix timestamp in seconds and nanoseconds.

```cursed
time := clock_bait.Unix(1704067200, 0)  // 2024-01-01 00:00:00 UTC
```

### `UnixMilli(msec normie) normie`
Create a time from Unix timestamp in milliseconds.

```cursed
time := clock_bait.UnixMilli(1704067200000)
```

### `UnixMicro(usec normie) normie`
Create a time from Unix timestamp in microseconds.

```cursed
time := clock_bait.UnixMicro(1704067200000000)
```

## Time Arithmetic

### `Add(t normie, d normie) normie`
Add a duration to a time.

```cursed
later := clock_bait.Add(time, clock_bait.HourVibe)
```

### `Sub(t1 normie, t2 normie) normie`
Subtract one time from another to get duration.

```cursed
duration := clock_bait.Sub(time2, time1)
```

## Time Comparison

### `After(t1 normie, t2 normie) lit`
Check if time1 is after time2.

```cursed
is_later := clock_bait.After(time1, time2)
```

### `Before(t1 normie, t2 normie) lit`
Check if time1 is before time2.

```cursed
is_earlier := clock_bait.Before(time1, time2)
```

### `Equal(t1 normie, t2 normie) lit`
Check if two times are equal.

```cursed
is_same := clock_bait.Equal(time1, time2)
```

### `Compare(t1 normie, t2 normie) normie`
Compare two times (-1, 0, or 1).

```cursed
result := clock_bait.Compare(time1, time2)
// -1 if time1 < time2
//  0 if time1 == time2
//  1 if time1 > time2
```

## Time Components

### `Year(t normie) normie`
Get the year from a timestamp.

```cursed
year := clock_bait.Year(time)
```

### `Month(t normie) normie`
Get the month from a timestamp.

```cursed
month := clock_bait.Month(time)
```

### `Day(t normie) normie`
Get the day from a timestamp.

```cursed
day := clock_bait.Day(time)
```

### `Weekday(t normie) normie`
Get the weekday from a timestamp.

```cursed
weekday := clock_bait.Weekday(time)
```

### `Hour(t normie) normie`
Get the hour from a timestamp.

```cursed
hour := clock_bait.Hour(time)
```

### `Minute(t normie) normie`
Get the minute from a timestamp.

```cursed
minute := clock_bait.Minute(time)
```

### `Second(t normie) normie`
Get the second from a timestamp.

```cursed
second := clock_bait.Second(time)
```

## Duration Operations

### `DurationHours(d normie) normie`
Convert duration to hours.

```cursed
hours := clock_bait.DurationHours(duration)
```

### `DurationMinutes(d normie) normie`
Convert duration to minutes.

```cursed
minutes := clock_bait.DurationMinutes(duration)
```

### `DurationSeconds(d normie) normie`
Convert duration to seconds.

```cursed
seconds := clock_bait.DurationSeconds(duration)
```

## Social Media Time Formatting

### `RelativeTime(t normie) tea`
Format time as a relative string.

```cursed
relative := clock_bait.RelativeTime(past_time)
// Returns: "just now", "minutes ago", "hours ago", etc.
```

### `TimeAgo(t normie) tea`
Format time with "ago" suffix.

```cursed
ago := clock_bait.TimeAgo(past_time)
// Returns: "2 hours ago", "5 minutes ago", etc.
```

### `SocialFormat(t normie) tea`
Format time for social media display.

```cursed
social := clock_bait.SocialFormat(past_time)
```

## Weekend and Vibe Functions

### `IsItFriday(t normie) lit`
Check if the given time is Friday.

```cursed
is_friday := clock_bait.IsItFriday(time)
```

### `NextWeekend(t normie) normie`
Get the time of the next weekend (Friday 5PM).

```cursed
weekend := clock_bait.NextWeekend(time)
```

### `VibeCheck(t normie) normie`
Get how many hours are left in the day.

```cursed
hours_left := clock_bait.VibeCheck(time)
```

### `ViberTime(t normie) tea`
Format time with Gen Z vibe style.

```cursed
vibe := clock_bait.ViberTime(time)
// Returns: "morning vibe", "afternoon vibe", "evening vibe"
```

## Time Span Operations

### `TimeSpanContains(start normie, end normie, t normie) lit`
Check if a time is within a time span.

```cursed
contains := clock_bait.TimeSpanContains(start, end, test_time)
```

### `TimeSpanOverlaps(start1 normie, end1 normie, start2 normie, end2 normie) lit`
Check if two time spans overlap.

```cursed
overlaps := clock_bait.TimeSpanOverlaps(start1, end1, start2, end2)
```

### `TimeSpanDuration(start normie, end normie) normie`
Get the duration of a time span.

```cursed
duration := clock_bait.TimeSpanDuration(start, end)
```

## Duration Rounding

### `RoundDuration(d normie, multiple normie) normie`
Round duration to the nearest multiple.

```cursed
rounded := clock_bait.RoundDuration(duration, clock_bait.MinuteVibe)
```

### `TruncateDuration(d normie, multiple normie) normie`
Truncate duration to a multiple.

```cursed
truncated := clock_bait.TruncateDuration(duration, clock_bait.MinuteVibe)
```

## String Formatting

### `MonthString(month normie) tea`
Convert month number to string.

```cursed
month_name := clock_bait.MonthString(clock_bait.VibeJanuary)
// Returns: "January"
```

### `WeekdayString(weekday normie) tea`
Convert weekday number to string.

```cursed
day_name := clock_bait.WeekdayString(clock_bait.VibeMonday)
// Returns: "Monday"
```

### `DurationString(d normie) tea`
Convert duration to descriptive string.

```cursed
desc := clock_bait.DurationString(duration)
// Returns: "seconds", "minutes", "hours", etc.
```

## Utility Functions

### `Sleep(d normie) lit`
Sleep for the specified duration (placeholder implementation).

```cursed
result := clock_bait.Sleep(clock_bait.SecondVibe)
```

### `Since(t normie) normie`
Get the time elapsed since the given time.

```cursed
elapsed := clock_bait.Since(start_time)
```

### `Until(t normie) normie`
Get the duration until the given time.

```cursed
remaining := clock_bait.Until(deadline)
```

## Unix Timestamp Conversions

### `ToUnix(t normie) normie`
Convert time to Unix timestamp in seconds.

```cursed
unix_sec := clock_bait.ToUnix(time)
```

### `ToUnixMilli(t normie) normie`
Convert time to Unix timestamp in milliseconds.

```cursed
unix_milli := clock_bait.ToUnixMilli(time)
```

### `ToUnixMicro(t normie) normie`
Convert time to Unix timestamp in microseconds.

```cursed
unix_micro := clock_bait.ToUnixMicro(time)
```

### `ToUnixNano(t normie) normie`
Convert time to Unix timestamp in nanoseconds.

```cursed
unix_nano := clock_bait.ToUnixNano(time)
```

## Usage Examples

### Basic Time Operations
```cursed
yeet "clock_bait"

// Get current time
now := clock_bait.Now()

// Create time from Unix timestamp
birthday := clock_bait.Unix(1704067200, 0)

// Add duration
later := clock_bait.Add(now, 2 * clock_bait.HourVibe)

// Compare times
is_later := clock_bait.After(later, now)
```

### Social Media Formatting
```cursed
// Format time for social media
past_time := clock_bait.Sub(now, 30 * clock_bait.MinuteVibe)
relative := clock_bait.RelativeTime(past_time)
vibez.spill(relative)  // "minutes ago"
```

### Weekend Detection
```cursed
// Check if it's Friday
is_friday := clock_bait.IsItFriday(now)
yikes is_friday {
    vibez.spill("It's Friday! Weekend vibes!")
} shook {
    next_weekend := clock_bait.NextWeekend(now)
    vibez.spill("Next weekend coming up!")
}
```

### Time Spans
```cursed
// Work with time spans
meeting_start := clock_bait.Unix(1704067200, 0)
meeting_end := clock_bait.Add(meeting_start, 2 * clock_bait.HourVibe)

// Check if current time is during meeting
is_meeting := clock_bait.TimeSpanContains(meeting_start, meeting_end, now)
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/clock_bait/test_clock_bait.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/clock_bait/test_clock_bait.💀
./test_clock_bait
```

## Implementation Notes

- All timestamps are stored as Unix nanoseconds for precision
- Date/time component extraction is simplified for the pure CURSED implementation
- Timezone handling is basic - production version would need full IANA timezone support
- Duration parsing is placeholder - full implementation would parse "1h30m" style strings
- Sleep function is a placeholder - real implementation would pause execution

## Dependencies

- `testz`: Testing framework
- No external FFI dependencies - pure CURSED implementation

## License

Part of the CURSED standard library.
