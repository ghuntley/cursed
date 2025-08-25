# time_zone_drip (time/tzdata)

## Overview
The `time_zone_drip` module provides access to time zone data and functionality for handling time zone conversions. It includes an embedded copy of the standard IANA Time Zone Database (tzdata), eliminating the need for external time zone files.

## Core Types and Interfaces

### Location
Represents a time zone location.

```csd
be_like Location squad {
  fr fr fields not directly accessible
}

slay LoadLocation(name tea) (*Location, tea)
slay (l *Location) String() tea
slay (l *Location) TzName(t timez.Time) tea
slay (l *Location) Zone(t timez.Time) (name tea, offset normie)
```

### TimeZone
Provides detailed information about a time zone.

```csd
be_like TimeZone squad {
  Name         tea
  Abbreviation tea
  Offset       normie    fr fr in seconds east of UTC
  IsDST        lit   fr fr is Daylight Saving Time
}

slay (tz *TimeZone) String() tea
```

### TZSet
Represents a collection of time zones.

```csd
be_like TZSet squad {
  fr fr fields not directly accessible
}

slay NewTZSet() *TZSet
slay (s *TZSet) Add(name tea) tea
slay (s *TZSet) Contains(name tea) lit
slay (s *TZSet) Names() []tea
```

## Core Functions

```csd
fr fr Load a time zone location by name
slay LoadLocation(name tea) (*Location, tea)

fr fr Check if a time zone data is available
slay IsTimeZoneAvailable(name tea) lit

fr fr Get all available time zone names
slay AvailableTimeZones() []tea

fr fr Get time zones by region
slay TimeZonesByRegion(region tea) []tea

fr fr Get time zone information for a specific time
slay GetTimeZone(name tea, t timez.Time) (*TimeZone, tea)

fr fr Get current time zone information
slay CurrentTimeZone() (*TimeZone, tea)

fr fr Get time zone regions
slay TimeZoneRegions() []tea
```

## Enhanced Features

- **Time Zone Lookup**: Find time zones by geographical coordinates
  ```csd
  timezone, err := time_zone_drip.LookupByCoordinates(latitude, longitude)
  ```

- **Time Zone Conversions**: Convert times between time zones
  ```csd
  convertedTime, err := time_zone_drip.Convert(sourceTime, sourceZone, targetZone)
  ```

- **DST Information**: Get detailed Daylight Saving Time information
  ```csd
  dstTransitions := time_zone_drip.GetDSTTransitions("America/New_York", 2023)
  ```

- **Time Zone Aliases**: Support for time zone aliases and deprecated names
  ```csd
  canonical := time_zone_drip.CanonicalName("EST")
  ```

- **Offset-Based Lookup**: Find time zones by UTC offset
  ```csd
  zones := time_zone_drip.ZonesByOffset(3600) fr fr UTC+1
  ```

## Usage Examples

```csd
fr fr Load a time zone location
location, err := time_zone_drip.LoadLocation("America/New_York")
if err != nah {
  vibez.spill("Error loading location: %v", err)
  yolo
}

fr fr Get current time in the loaded location
now := timez.Now().In(location)
vibez.spill("Current time in %s: %s", location, now.Format(timez.RFC1123))

fr fr Get time zone name and offset for a specific time
name, offset := location.Zone(now)
vibez.spill("Zone name: %s, offset: %d seconds", name, offset)
vibez.spill("Offset in hours: %d hours", offset/3600)

fr fr Get time zone abbreviation
abbrev := location.TzName(now)
vibez.spill("Time zone abbreviation: %s", abbrev)

fr fr Check if a time zone is available
isAvailable := time_zone_drip.IsTimeZoneAvailable("Europe/London")
vibez.spill("\nIs Europe/London available: %t", isAvailable)

fr fr Get a list of all available time zones
allZones := time_zone_drip.AvailableTimeZones()
vibez.spill("\nTotal available time zones: %d", len(allZones))
vibez.spill("Sample time zones:")
for i, zone := range allZones {
  if i < 5 { fr fr Only prnormie a few examples
    vibez.spill("  %s", zone)
  }
  if i == 5 {
    vibez.spill("  ...")
  }
}

fr fr Get time zones by region
europeZones := time_zone_drip.TimeZonesByRegion("Europe")
vibez.spill("\nEurope time zones: %d total", len(europeZones))
vibez.spill("Sample Europe zones:")
for i, zone := range europeZones {
  if i < 5 { fr fr Only prnormie a few examples
    vibez.spill("  %s", zone)
  }
  if i == 5 {
    vibez.spill("  ...")
  }
}

fr fr Get all time zone regions
regions := time_zone_drip.TimeZoneRegions()
vibez.spill("\nTime zone regions:")
for _, region := range regions {
  vibez.spill("  %s", region)
}

fr fr Get detailed time zone information for a specific time
nyc, err := time_zone_drip.GetTimeZone("America/New_York", now)
if err != nah {
  vibez.spill("Error getting time zone info: %v", err)
  yolo
}

vibez.spill("\nNew York time zone details:")
vibez.spill("  Name: %s", nyc.Name)
vibez.spill("  Abbreviation: %s", nyc.Abbreviation)
vibez.spill("  Offset: %d seconds (%d hours)", nyc.Offset, nyc.Offset/3600)
vibez.spill("  Is DST: %t", nyc.IsDST)

fr fr Get current time zone
currentTZ, err := time_zone_drip.CurrentTimeZone()
if err != nah {
  vibez.spill("Error getting current time zone: %v", err)
  yolo
}

vibez.spill("\nCurrent time zone:")
vibez.spill("  Name: %s", currentTZ.Name)
vibez.spill("  Abbreviation: %s", currentTZ.Abbreviation)
vibez.spill("  Offset: %d seconds (%d hours)", currentTZ.Offset, currentTZ.Offset/3600)
vibez.spill("  Is DST: %t", currentTZ.IsDST)

fr fr Time zone lookup by coordinates
lat, long := 40.7128, -74.0060 fr fr New York City coordinates
tzByCoords, err := time_zone_drip.LookupByCoordinates(lat, long)
if err != nah {
  vibez.spill("Error looking up time zone by coordinates: %v", err)
  yolo
}

vibez.spill("\nTime zone at coordinates (%f, %f): %s", lat, long, tzByCoords)

fr fr Converting between time zones
tokyo, err := time_zone_drip.LoadLocation("Asia/Tokyo")
if err != nah {
  vibez.spill("Error loading Tokyo location: %v", err)
  yolo
}

london, err := time_zone_drip.LoadLocation("Europe/London")
if err != nah {
  vibez.spill("Error loading London location: %v", err)
  yolo
}

fr fr Create a time in Tokyo
tokyoTime := timez.Date(2023, 1, 15, 9, 30, 0, 0, tokyo)
vibez.spill("\nTokyo time: %s", tokyoTime.Format(timez.RFC1123))

fr fr Convert to London time
londonTime, err := time_zone_drip.Convert(tokyoTime, tokyo, london)
if err != nah {
  vibez.spill("Error converting time: %v", err)
  yolo
}

vibez.spill("London time: %s", londonTime.Format(timez.RFC1123))

fr fr Get DST transitions for a year
dstTransitions, err := time_zone_drip.GetDSTTransitions("America/New_York", 2023)
if err != nah {
  vibez.spill("Error getting DST transitions: %v", err)
  yolo
}

vibez.spill("\nDST transitions for America/New_York in 2023:")
for i, transition := range dstTransitions {
  vibez.spill("  %d. %s: %s to %s (offset change: %d min)",
    i+1,
    transition.Time.Format("2006-01-02 15:04:05"),
    transition.FromAbbreviation,
    transition.ToAbbreviation,
    (transition.ToOffset-transition.FromOffset)/60)
}

fr fr Get time zones with the same offset
utcPlus2Zones := time_zone_drip.ZonesByOffset(7200) fr fr UTC+2
vibez.spill("\nTime zones with UTC+2 offset:")
for i, zone := range utcPlus2Zones {
  if i < 5 { fr fr Only prnormie a few examples
    vibez.spill("  %s", zone)
  }
  if i == 5 {
    vibez.spill("  ...")
  }
}

fr fr Get canonical name for time zone abbreviation
canonicalEST := time_zone_drip.CanonicalName("EST")
vibez.spill("\nCanonical name for EST: %s", canonicalEST)

fr fr Create and use a time zone set
tzSet := time_zone_drip.NewTZSet()
tzSet.Add("America/New_York")
tzSet.Add("Europe/London")
tzSet.Add("Asia/Tokyo")

vibez.spill("\nTime zone set:")
vibez.spill("  Contains America/New_York: %t", tzSet.Contains("America/New_York"))
vibez.spill("  Contains Australia/Sydney: %t", tzSet.Contains("Australia/Sydney"))

setNames := tzSet.Names()
vibez.spill("  Set names: %v", setNames)
```

## Implementation Guidelines

- Embed the latest IANA Time Zone Database for accurate time calculations
- Handle Daylight Saving Time transitions correctly
- Ensure thread safety for all operations
- Provide efficient lookup for time zone information
- Support time zone rules that change over time
- Handle edge cases like time zone changes due to political decisions
- Implement geographic coordinate-based time zone lookup
- Support commonly used time zone aliases
- Process leap seconds correctly
- Maintain forward and backward compatibility with IANA database changes