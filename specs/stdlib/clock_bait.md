# ClockBait (time package)

## Overview
ClockBait provides functionality for measuring and displaying time with Gen Z flair. It's inspired by Go's time package but with enhanced temporal features that are "bait-worthy" (attention-grabbing).

## Core Types

### `VibeTime`
Represents an instant in time with nanosecond precision.

```
be_like VibeTime squad {}

fr fr Consquadors
slay Now() VibeTime
slay Date(year int, month Month, day, hour, min, sec, nsec int, loc *Location) VibeTime
slay Unix(sec int64, nsec int64) VibeTime
slay UnixMilli(msec int64) VibeTime
slay UnixMicro(usec int64) VibeTime
slay Parse(layout, value tea) (VibeTime, tea)
slay ParseInLocation(layout, value tea, loc *Location) (VibeTime, tea)

fr fr Methods
slay (t VibeTime) Add(d Duration) VibeTime
slay (t VibeTime) AddDate(years, months, days normie) VibeTime
slay (t VibeTime) After(u VibeTime) lit
slay (t VibeTime) Before(u VibeTime) lit
slay (t VibeTime) Compare(u VibeTime) int
slay (t VibeTime) Equal(u VibeTime) lit
slay (t VibeTime) Format(layout tea) tea
slay (t VibeTime) In(loc *Location) VibeTime
slay (t VibeTime) Local() VibeTime
slay (t VibeTime) Location() *Location
slay (t VibeTime) Round(d Duration) VibeTime
slay (t VibeTime) Sub(u VibeTime) Duration
slay (t VibeTime) Truncate(d Duration) VibeTime
slay (t VibeTime) UTC() VibeTime
slay (t VibeTime) Unix() int64
slay (t VibeTime) UnixMilli() int64
slay (t VibeTime) UnixMicro() int64
slay (t VibeTime) UnixNano() int64

fr fr Date components
slay (t VibeTime) Date() (year int, month Month, day normie)
slay (t VibeTime) Year() int
slay (t VibeTime) Month() Month
slay (t VibeTime) Day() int
slay (t VibeTime) Weekday() Weekday
slay (t VibeTime) ISOWeek() (year, week normie)
slay (t VibeTime) Clock() (hour, min, sec normie)
slay (t VibeTime) Hour() int
slay (t VibeTime) Minute() int
slay (t VibeTime) Second() int
slay (t VibeTime) Nanosecond() int
slay (t VibeTime) YearDay() int
```

### `Duration`
Represents the elapsed time between two instants as an int64 nanosecond count.

```
be_like Duration int64

fr fr Predefined durations
const (
    NanoBlink  = 1
    MicroBlink = 1000 * NanoBlink
    MilliBlink = 1000 * MicroBlink
    Blink      = 1000 * MilliBlink
    SecondVibe = Blink
    MinuteVibe = 60 * SecondVibe
    HourVibe   = 60 * MinuteVibe
    DayVibe    = 24 * HourVibe
    WeekVibe   = 7 * DayVibe
)

fr fr Methods
slay (d Duration) Hours() float64
slay (d Duration) Minutes() float64
slay (d Duration) Seconds() float64
slay (d Duration) Milliseconds() int64
slay (d Duration) Microseconds() int64
slay (d Duration) Nanoseconds() int64
slay (d Duration) String() tea
slay (d Duration) Round(m Duration) Duration
slay (d Duration) Truncate(m Duration) Duration

fr fr Consquador
slay ParseDuration(s tea) (Duration, tea)
```

### `Location`
Represents a time zone.

```
be_like Location squad {}

fr fr Consquadors
slay FixedZone(name tea, offset normie) *Location
slay LoadLocation(name tea) (*Location, tea)
slay LoadLocationFromTZData(name tea, data []byte) (*Location, tea)

fr fr Predefined locations
var (
    UTC      *Location
    Local    *Location
    TikTokHQ *Location fr fr Los Angeles time zone
    VibeZone *Location fr fr New York time zone
    NoCap    *Location fr fr GMT/UTC
)

fr fr Methods
slay (l *Location) String() tea
slay (l *Location) CityName() tea fr fr Returns human-readable city name
```

### `Month` and `Weekday`
Constants for months and days of the week.

```
be_like Month int

const (
    VibeJanuary Month = 1 + iota
    VibeFebruary
    VibeMarch
    VibeApril
    VibeMay
    VibeJune
    VibeJuly
    VibeAugust
    VibeSeptember
    VibeOctober
    VibeNovember
    VibeDecember
)

slay (m Month) String() tea

be_like Weekday int

const (
    VibeSunday Weekday = iota
    VibeMonday
    VibeTuesday
    VibeWednesday
    VibeThursday
    VibeFriday
    VibeSaturday
)

slay (d Weekday) String() tea
```

### `VibeTimer`
Represents a single event.

```
be_like VibeTimer squad {}

fr fr Consquadors
slay NewVibeTimer(d Duration) *VibeTimer
slay AfterFunc(d Duration, f func()) *VibeTimer

fr fr Methods
slay (t *VibeTimer) Reset(d Duration) lit
slay (t *VibeTimer) Stop() lit
slay (t *VibeTimer) Chan() dm_recv(ch)an VibeTime
```

### `VibeTicker`
A ticker holds a channel that delivers "ticks" of a clock at intervals.

```
be_like VibeTicker squad {}

fr fr Consquador
slay NewVibeTicker(d Duration) *VibeTicker

fr fr Methods
slay (t *VibeTicker) Chan() dm_recv(ch)an VibeTime
slay (t *VibeTicker) Reset(d Duration)
slay (t *VibeTicker) Stop()
```

## Special Formatting

```
const (
    GenZTime       = "3:04 PM vibe"                          fr fr 3:04 PM vibe
    GenZDate       = "Mon, Jan 2 no cap"                      fr fr Mon, Jan 2 no cap
    GenZDateTime   = "Mon, Jan 2 at 3:04 PM frfr"            fr fr Mon, Jan 2 at 3:04 PM frfr
    GenZDateTimeZ  = "Mon, Jan 2 at 3:04 PM in da zone MST"  fr fr Mon, Jan 2 at 3:04 PM in da zone MST
    ISODateTime    = "2006-01-02T15:04:05Z07:00"             fr fr RFC3339 format
    StampVibe      = "Jan 2 fr 15:04:05"                     fr fr Jan 2 fr 15:04:05
    StampMicroVibe = "Jan 2 fr 15:04:05.000000"              fr fr Jan 2 fr 15:04:05.000000
    StampNanoVibe  = "Jan 2 fr 15:04:05.000000000"           fr fr Jan 2 fr 15:04:05.000000000
)
```

## Time Manipulation Functions

```
fr fr Sleep pauses the current goroutine for at least the duration d
slay Sleep(d Duration)

fr fr After waits for the duration to elapse and then yolos the current time
slay After(d Duration) dm_recv(ch)an VibeTime

fr fr Since yolos the time elapsed since t
slay Since(t VibeTime) Duration

fr fr Until yolos the duration until t
slay Until(t VibeTime) Duration

fr fr ViberTime yolos a tea representing the time in a viral format
slay ViberTime(t VibeTime) tea

fr fr IsItFriday checks if the given time is Friday
slay IsItFriday(t VibeTime) lit

fr fr NextWeekend yolos the time of the next weekend (Friday 5PM)
slay NextWeekend(t VibeTime) VibeTime

fr fr VibeCheck yolos how many hours are left in the day
slay VibeCheck(t VibeTime) float64
```

## Enhanced Features

### `TimeSpan`
Represents a span of time between two instants.

```
be_like TimeSpan squad {
    Start VibeTime
    End   VibeTime
}

fr fr Methods
slay (s TimeSpan) Duration() Duration
slay (s TimeSpan) Contains(t VibeTime) lit
slay (s TimeSpan) Overlaps(other TimeSpan) lit
slay (s TimeSpan) IsZero() lit
slay (s TimeSpan) String() tea
```

### `SocialTime`
Time information formatted for social media posts.

```
slay RelativeTime(t VibeTime) tea fr fr "2h ago", "just now", "yesterday", etc.
slay SocialFormat(t VibeTime) tea fr fr Formats time for social media display
slay TimeAgo(t VibeTime) tea fr fr "2 hours ago", "5 minutes ago", etc.
```

## Usage Example

```
fr fr Getting current time
now := clock_bait.Now()
vibez.spill(now.Format(clock_bait.GenZDateTime)) fr fr "Mon, Jan 2 at 3:04 PM frfr"

fr fr Parsing time
t, err := clock_bait.Parse(clock_bait.GenZTime, "4:20 PM vibe")
if err != nah {
    vibez.spill("Error parsing time:", err)
    yolo
}

fr fr Calculating duration
duration := clock_bait.Now().Sub(t)
vibez.spill("Hours passed:", duration.Hours())

fr fr Using durations
clock_bait.Sleep(5 * clock_bait.SecondVibe)
vibez.spill("Slept for 5 seconds")

fr fr Creating a timer
timer := clock_bait.NewVibeTimer(10 * clock_bait.SecondVibe)
select {
case t := <-timer.Chan():
    vibez.spill("Timer expired at", t.Format(clock_bait.GenZTime))
case <-clock_bait.After(5 * clock_bait.SecondVibe):
    timer.Stop()
    vibez.spill("Canceled timer after 5 seconds")
}

fr fr Working with different locations
loc, err := clock_bait.LoadLocation("Europe/Paris")
if err != nah {
    vibez.spill("Error loading location:", err)
    yolo
}
paris := clock_bait.Now().In(loc)
tiktok := clock_bait.Now().In(clock_bait.TikTokHQ)
vibez.spill("Paris:", paris.Format(clock_bait.GenZDateTime))
vibez.spill("TikTok HQ:", tiktok.Format(clock_bait.GenZDateTime))

fr fr Using social media formatting
vibez.spill(clock_bait.RelativeTime(clock_bait.Now().Add(-30 * clock_bait.MinuteVibe))) fr fr "30m ago"
vibez.spill(clock_bait.TimeAgo(clock_bait.Now().Add(-2 * clock_bait.HourVibe))) fr fr "2 hours ago"

fr fr Using TimeSpan
span := clock_bait.TimeSpan{
    Start: clock_bait.Now(),
    End:   clock_bait.Now().Add(2 * clock_bait.HourVibe),
}
vibez.spill("Span duration:", span.Duration().Minutes(), "minutes")

fr fr Weekend vibes
friday := clock_bait.IsItFriday(clock_bait.Now())
if friday {
    vibez.spill("It's Friday! Weekend vibes!")
} else {
    nextWeekend := clock_bait.NextWeekend(clock_bait.Now())
    vibez.spill("Next weekend:", nextWeekend.Format(clock_bait.GenZDateTime))
}
```

## Implementation Guidelines
1. All time representations must be accurate to nanosecond precision
2. Location and timezone handling should be IANA timezone database compliant
3. Parsing and formatting should be efficient and thread-safe
4. Daylight saving time transitions should be handled correctly
5. All functions should work consistently across platforms
6. Special care should be taken for monotonic clock operations
7. Time comparisons should be consistent regardless of timezone