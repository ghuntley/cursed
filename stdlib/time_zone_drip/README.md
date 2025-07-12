# time_zone_drip Module

The `time_zone_drip` module provides comprehensive time zone handling functionality for the CURSED language, including access to time zone data and utilities for converting between time zones.

## Features

- **Time Zone Database**: Embedded subset of IANA Time Zone Database
- **Location Management**: Load and manage time zone locations
- **Time Zone Information**: Access detailed time zone data including offsets and DST status
- **Coordinate Lookup**: Find time zones by geographical coordinates
- **Time Conversion**: Convert times between different time zones
- **DST Transitions**: Get Daylight Saving Time transition information
- **Time Zone Sets**: Manage collections of time zones
- **Alias Support**: Support for common time zone aliases (EST, PST, GMT, etc.)

## Core Types

### Location
Represents a time zone location with methods for accessing zone information.

### TimeZone
Provides detailed time zone information including:
- Name (full IANA name)
- Abbreviation (e.g., EST, PST)
- Offset (seconds from UTC)
- DST status

### TZSet
A collection of time zones with methods for managing the set.

### DSTTransition
Represents a Daylight Saving Time transition with timing and offset information.

## Main Functions

### Basic Operations
- `LoadLocation(name)` - Load a time zone by name
- `IsTimeZoneAvailable(name)` - Check if a time zone is available
- `AvailableTimeZones()` - Get all available time zones
- `TimeZonesByRegion(region)` - Get time zones for a specific region
- `GetTimeZone(name, time)` - Get detailed time zone information
- `CurrentTimeZone()` - Get current system time zone

### Advanced Operations
- `LookupByCoordinates(lat, lng)` - Find time zone by coordinates
- `Convert(time, source, target)` - Convert time between zones
- `GetDSTTransitions(zone, year)` - Get DST transition information
- `ZonesByOffset(offset)` - Find time zones with specific offset
- `CanonicalName(alias)` - Get canonical name for time zone alias

### Time Zone Sets
- `NewTZSet()` - Create a new time zone set
- `Add(name)` - Add time zone to set
- `Contains(name)` - Check if time zone is in set
- `Names()` - Get all names in set

## Usage Examples

### Basic Time Zone Operations
```cursed
# Load a time zone location
sus location, err = time_zone_drip.LoadLocation("America/New_York")
if err != "" {
    vibez.spill("Error: %s", err)
}

# Get zone information
sus zone_name, offset = location.Zone("")
sus tz_name = location.TzName("")

vibez.spill("Zone: %s, Offset: %d, Name: %s", zone_name, offset, tz_name)
```

### Time Zone Information
```cursed
# Get detailed time zone information
sus timezone, err = time_zone_drip.GetTimeZone("Europe/London", "")
if err == "" {
    vibez.spill("Name: %s", timezone.Name)
    vibez.spill("Abbreviation: %s", timezone.Abbreviation)
    vibez.spill("Offset: %d seconds", timezone.Offset)
    vibez.spill("Is DST: %t", timezone.IsDST)
}
```

### Time Zone Discovery
```cursed
# Check availability
sus available = time_zone_drip.IsTimeZoneAvailable("Asia/Tokyo")
vibez.spill("Tokyo timezone available: %t", available)

# Get all available time zones
sus all_zones = time_zone_drip.AvailableTimeZones()
vibez.spill("Total zones: %d", len(all_zones))

# Get zones by region
sus america_zones = time_zone_drip.TimeZonesByRegion("America")
vibez.spill("America zones: %d", len(america_zones))
```

### Coordinate-Based Lookup
```cursed
# Find time zone by coordinates (New York City)
sus timezone, err = time_zone_drip.LookupByCoordinates(40.7128, -74.0060)
if err == "" {
    vibez.spill("Time zone at coordinates: %s", timezone)
}
```

### Time Zone Conversion
```cursed
# Load source and target time zones
sus nyc, err1 = time_zone_drip.LoadLocation("America/New_York")
sus london, err2 = time_zone_drip.LoadLocation("Europe/London")

if err1 == "" && err2 == "" {
    # Convert time between zones
    sus converted, err = time_zone_drip.Convert("2023-01-15 09:30:00", nyc, london)
    if err == "" {
        vibez.spill("Converted time: %s", converted)
    }
}
```

### DST Information
```cursed
# Get DST transitions for a year
sus transitions, err = time_zone_drip.GetDSTTransitions("America/New_York", 2023)
if err == "" {
    vibez.spill("DST transitions: %d", len(transitions))
    bestie i := 0; i < len(transitions); i++ {
        sus t = transitions[i]
        vibez.spill("Transition: %s -> %s", t.FromAbbreviation, t.ToAbbreviation)
    }
}
```

### Time Zone Aliases
```cursed
# Get canonical name for alias
sus canonical = time_zone_drip.CanonicalName("EST")
vibez.spill("EST canonical name: %s", canonical)

# Find zones by offset
sus utc_zones = time_zone_drip.ZonesByOffset(0)
vibez.spill("UTC zones: %d", len(utc_zones))
```

### Time Zone Sets
```cursed
# Create and manage time zone sets
sus tzset = time_zone_drip.NewTZSet()
tzset.Add("America/New_York")
tzset.Add("Europe/London")
tzset.Add("Asia/Tokyo")

sus has_nyc = tzset.Contains("America/New_York")
sus names = tzset.Names()

vibez.spill("Contains NYC: %t", has_nyc)
vibez.spill("Set size: %d", len(names))
```

## Supported Time Zones

The module includes support for major time zones including:
- **America**: New_York, Los_Angeles, Chicago, Denver
- **Europe**: London, Paris, Berlin
- **Asia**: Tokyo, Shanghai
- **Australia**: Sydney

## Time Zone Aliases

Common aliases are supported:
- **EST** → America/New_York
- **PST** → America/Los_Angeles
- **GMT** → Europe/London
- **UTC** → UTC
- **MST** → America/Denver
- **CST** → America/Chicago

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/time_zone_drip/test_time_zone_drip.csd
```

## Implementation Notes

- **Pure CURSED**: Implemented entirely in CURSED without external dependencies
- **Embedded Database**: Contains embedded time zone data for offline operation
- **Thread Safe**: All operations are designed to be thread-safe
- **Performance**: Optimized for common time zone operations
- **Extensible**: Easy to add new time zones and features

## Error Handling

The module uses CURSED's error handling patterns:
- Functions return error strings for invalid operations
- Empty strings indicate success
- Nil pointers for invalid time zones or data

## Future Enhancements

- Full IANA Time Zone Database integration
- Real-time time zone data updates
- Advanced DST calculation algorithms
- Geographic boundary-based coordinate lookup
- Time zone rule change history tracking
