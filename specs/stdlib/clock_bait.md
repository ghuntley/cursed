# ClockBait (time package)

## Overview
ClockBait provides functionality for measuring and displaying time with Gen Z flair. It's inspired by Go's time package but with enhanced temporal features that are "bait-worthy" (attention-grabbing).

## Core Types

### `VibeTime`
Represents an instant in time with nanosecond precision.

```go
type VibeTime struct {}

// Constructors
func Now() VibeTime
func Date(year int, month Month, day, hour, min, sec, nsec int, loc *Location) VibeTime
func Unix(sec int64, nsec int64) VibeTime
func UnixMilli(msec int64) VibeTime
func UnixMicro(usec int64) VibeTime
func Parse(layout, value string) (VibeTime, error)
func ParseInLocation(layout, value string, loc *Location) (VibeTime, error)

// Methods
func (t VibeTime) Add(d Duration) VibeTime
func (t VibeTime) AddDate(years, months, days int) VibeTime
func (t VibeTime) After(u VibeTime) bool
func (t VibeTime) Before(u VibeTime) bool
func (t VibeTime) Compare(u VibeTime) int
func (t VibeTime) Equal(u VibeTime) bool
func (t VibeTime) Format(layout string) string
func (t VibeTime) In(loc *Location) VibeTime
func (t VibeTime) Local() VibeTime
func (t VibeTime) Location() *Location
func (t VibeTime) Round(d Duration) VibeTime
func (t VibeTime) Sub(u VibeTime) Duration
func (t VibeTime) Truncate(d Duration) VibeTime
func (t VibeTime) UTC() VibeTime
func (t VibeTime) Unix() int64
func (t VibeTime) UnixMilli() int64
func (t VibeTime) UnixMicro() int64
func (t VibeTime) UnixNano() int64

// Date components
func (t VibeTime) Date() (year int, month Month, day int)
func (t VibeTime) Year() int
func (t VibeTime) Month() Month
func (t VibeTime) Day() int
func (t VibeTime) Weekday() Weekday
func (t VibeTime) ISOWeek() (year, week int)
func (t VibeTime) Clock() (hour, min, sec int)
func (t VibeTime) Hour() int
func (t VibeTime) Minute() int
func (t VibeTime) Second() int
func (t VibeTime) Nanosecond() int
func (t VibeTime) YearDay() int
```

### `Duration`
Represents the elapsed time between two instants as an int64 nanosecond count.

```go
type Duration int64

// Predefined durations
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

// Methods
func (d Duration) Hours() float64
func (d Duration) Minutes() float64
func (d Duration) Seconds() float64
func (d Duration) Milliseconds() int64
func (d Duration) Microseconds() int64
func (d Duration) Nanoseconds() int64
func (d Duration) String() string
func (d Duration) Round(m Duration) Duration
func (d Duration) Truncate(m Duration) Duration

// Constructor
func ParseDuration(s string) (Duration, error)
```

### `Location`
Represents a time zone.

```go
type Location struct {}

// Constructors
func FixedZone(name string, offset int) *Location
func LoadLocation(name string) (*Location, error)
func LoadLocationFromTZData(name string, data []byte) (*Location, error)

// Predefined locations
var (
    UTC      *Location
    Local    *Location
    TikTokHQ *Location // Los Angeles time zone
    VibeZone *Location // New York time zone
    NoCap    *Location // GMT/UTC
)

// Methods
func (l *Location) String() string
func (l *Location) CityName() string // Returns human-readable city name
```

### `Month` and `Weekday`
Constants for months and days of the week.

```go
type Month int

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

func (m Month) String() string

type Weekday int

const (
    VibeSunday Weekday = iota
    VibeMonday
    VibeTuesday
    VibeWednesday
    VibeThursday
    VibeFriday
    VibeSaturday
)

func (d Weekday) String() string
```

### `VibeTimer`
Represents a single event.

```go
type VibeTimer struct {}

// Constructors
func NewVibeTimer(d Duration) *VibeTimer
func AfterFunc(d Duration, f func()) *VibeTimer

// Methods
func (t *VibeTimer) Reset(d Duration) bool
func (t *VibeTimer) Stop() bool
func (t *VibeTimer) Chan() <-chan VibeTime
```

### `VibeTicker`
A ticker holds a channel that delivers "ticks" of a clock at intervals.

```go
type VibeTicker struct {}

// Constructor
func NewVibeTicker(d Duration) *VibeTicker

// Methods
func (t *VibeTicker) Chan() <-chan VibeTime
func (t *VibeTicker) Reset(d Duration)
func (t *VibeTicker) Stop()
```

## Special Formatting

```go
const (
    GenZTime       = "3:04 PM vibe"                          // 3:04 PM vibe
    GenZDate       = "Mon, Jan 2 no cap"                      // Mon, Jan 2 no cap
    GenZDateTime   = "Mon, Jan 2 at 3:04 PM frfr"            // Mon, Jan 2 at 3:04 PM frfr
    GenZDateTimeZ  = "Mon, Jan 2 at 3:04 PM in da zone MST"  // Mon, Jan 2 at 3:04 PM in da zone MST
    ISODateTime    = "2006-01-02T15:04:05Z07:00"             // RFC3339 format
    StampVibe      = "Jan 2 fr 15:04:05"                     // Jan 2 fr 15:04:05
    StampMicroVibe = "Jan 2 fr 15:04:05.000000"              // Jan 2 fr 15:04:05.000000
    StampNanoVibe  = "Jan 2 fr 15:04:05.000000000"           // Jan 2 fr 15:04:05.000000000
)
```

## Time Manipulation Functions

```go
// Sleep pauses the current goroutine for at least the duration d
func Sleep(d Duration)

// After waits for the duration to elapse and then returns the current time
func After(d Duration) <-chan VibeTime

// Since returns the time elapsed since t
func Since(t VibeTime) Duration

// Until returns the duration until t
func Until(t VibeTime) Duration

// ViberTime returns a string representing the time in a viral format
func ViberTime(t VibeTime) string

// IsItFriday checks if the given time is Friday
func IsItFriday(t VibeTime) bool

// NextWeekend returns the time of the next weekend (Friday 5PM)
func NextWeekend(t VibeTime) VibeTime

// VibeCheck returns how many hours are left in the day
func VibeCheck(t VibeTime) float64
```

## Enhanced Features

### `TimeSpan`
Represents a span of time between two instants.

```go
type TimeSpan struct {
    Start VibeTime
    End   VibeTime
}

// Methods
func (s TimeSpan) Duration() Duration
func (s TimeSpan) Contains(t VibeTime) bool
func (s TimeSpan) Overlaps(other TimeSpan) bool
func (s TimeSpan) IsZero() bool
func (s TimeSpan) String() string
```

### `SocialTime`
Time information formatted for social media posts.

```go
func RelativeTime(t VibeTime) string // "2h ago", "just now", "yesterday", etc.
func SocialFormat(t VibeTime) string // Formats time for social media display
func TimeAgo(t VibeTime) string // "2 hours ago", "5 minutes ago", etc.
```

## Usage Example

```go
// Getting current time
now := clock_bait.Now()
vibez.spill(now.Format(clock_bait.GenZDateTime)) // "Mon, Jan 2 at 3:04 PM frfr"

// Parsing time
t, err := clock_bait.Parse(clock_bait.GenZTime, "4:20 PM vibe")
if err != nil {
    vibez.spill("Error parsing time:", err)
    return
}

// Calculating duration
duration := clock_bait.Now().Sub(t)
vibez.spill("Hours passed:", duration.Hours())

// Using durations
clock_bait.Sleep(5 * clock_bait.SecondVibe)
vibez.spill("Slept for 5 seconds")

// Creating a timer
timer := clock_bait.NewVibeTimer(10 * clock_bait.SecondVibe)
select {
case t := <-timer.Chan():
    vibez.spill("Timer expired at", t.Format(clock_bait.GenZTime))
case <-clock_bait.After(5 * clock_bait.SecondVibe):
    timer.Stop()
    vibez.spill("Canceled timer after 5 seconds")
}

// Working with different locations
loc, err := clock_bait.LoadLocation("Europe/Paris")
if err != nil {
    vibez.spill("Error loading location:", err)
    return
}
paris := clock_bait.Now().In(loc)
tiktok := clock_bait.Now().In(clock_bait.TikTokHQ)
vibez.spill("Paris:", paris.Format(clock_bait.GenZDateTime))
vibez.spill("TikTok HQ:", tiktok.Format(clock_bait.GenZDateTime))

// Using social media formatting
vibez.spill(clock_bait.RelativeTime(clock_bait.Now().Add(-30 * clock_bait.MinuteVibe))) // "30m ago"
vibez.spill(clock_bait.TimeAgo(clock_bait.Now().Add(-2 * clock_bait.HourVibe))) // "2 hours ago"

// Using TimeSpan
span := clock_bait.TimeSpan{
    Start: clock_bait.Now(),
    End:   clock_bait.Now().Add(2 * clock_bait.HourVibe),
}
vibez.spill("Span duration:", span.Duration().Minutes(), "minutes")

// Weekend vibes
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