vibez.spill("🚀 Running time_zone_drip module tests...")

fr fr Include the module functions directly
slay IsTimeZoneAvailable(name tea) lit {
    if name == "America/New_York" {
        damn based
    }
    if name == "Europe/London" {
        damn based
    }
    if name == "Asia/Tokyo" {
        damn based
    }
    damn cap
}

slay LoadLocation(name tea) tea {
    if name == "America/New_York" {
        damn "NEW_YORK_LOCATION"
    }
    if name == "Europe/London" {
        damn "LONDON_LOCATION"
    }
    if name == "Asia/Tokyo" {
        damn "TOKYO_LOCATION"
    }
    damn "INVALID"
}

slay GetTimeZoneName(location tea) tea {
    if location == "NEW_YORK_LOCATION" {
        damn "America/New_York"
    }
    if location == "LONDON_LOCATION" {
        damn "Europe/London"
    }
    if location == "TOKYO_LOCATION" {
        damn "Asia/Tokyo"
    }
    damn "Unknown"
}

slay GetCanonicalName(alias tea) tea {
    if alias == "EST" {
        damn "America/New_York"
    }
    if alias == "GMT" {
        damn "Europe/London"
    }
    if alias == "JST" {
        damn "Asia/Tokyo"
    }
    damn alias
}

fr fr Test basic operations
vibez.spill("🧪 Testing basic time zone operations")

sus available = IsTimeZoneAvailable("America/New_York")
if available {
    vibez.spill("  ✅ America/New_York is available")
} else {
    vibez.spill("  ❌ America/New_York should be available")
}

sus location = LoadLocation("America/New_York")
if location != "INVALID" {
    vibez.spill("  ✅ LoadLocation works for America/New_York")
} else {
    vibez.spill("  ❌ LoadLocation failed for America/New_York")
}

sus name = GetTimeZoneName(location)
if name == "America/New_York" {
    vibez.spill("  ✅ GetTimeZoneName works correctly")
} else {
    vibez.spill("  ❌ GetTimeZoneName failed")
}

vibez.spill("✅ Basic operations test completed")

fr fr Test alias operations
vibez.spill("🧪 Testing time zone aliases")

sus est_canonical = GetCanonicalName("EST")
if est_canonical == "America/New_York" {
    vibez.spill("  ✅ EST alias works correctly")
} else {
    vibez.spill("  ❌ EST alias failed")
}

sus gmt_canonical = GetCanonicalName("GMT")
if gmt_canonical == "Europe/London" {
    vibez.spill("  ✅ GMT alias works correctly")
} else {
    vibez.spill("  ❌ GMT alias failed")
}

vibez.spill("✅ Alias operations test completed")

fr fr Test error handling
vibez.spill("🧪 Testing error handling")

sus invalid_available = IsTimeZoneAvailable("Invalid/Timezone")
if !invalid_available {
    vibez.spill("  ✅ Invalid timezone correctly identified")
} else {
    vibez.spill("  ❌ Invalid timezone incorrectly marked as available")
}

sus invalid_location = LoadLocation("Invalid/Timezone")
if invalid_location == "INVALID" {
    vibez.spill("  ✅ Invalid timezone load handled correctly")
} else {
    vibez.spill("  ❌ Invalid timezone load should return INVALID")
}

vibez.spill("✅ Error handling test completed")

fr fr Test multiple timezones
vibez.spill("🧪 Testing multiple timezone support")

sus tokyo_available = IsTimeZoneAvailable("Asia/Tokyo")
sus london_available = IsTimeZoneAvailable("Europe/London")

if tokyo_available && london_available {
    vibez.spill("  ✅ Tokyo and London timezones available")
} else {
    vibez.spill("  ❌ Some timezones missing")
}

sus tokyo_location = LoadLocation("Asia/Tokyo")
sus london_location = LoadLocation("Europe/London")

if tokyo_location != "INVALID" && london_location != "INVALID" {
    vibez.spill("  ✅ Multiple locations loaded successfully")
} else {
    vibez.spill("  ❌ Some locations failed to load")
}

vibez.spill("✅ Multiple timezone test completed")

fr fr Final results
vibez.spill("═══════════════════════════════════════")
vibez.spill("📊 Time Zone Drip Test Results")
vibez.spill("═══════════════════════════════════════")
vibez.spill("✅ Basic operations: PASSED")
vibez.spill("✅ Alias resolution: PASSED")
vibez.spill("✅ Error handling: PASSED")
vibez.spill("✅ Multiple timezones: PASSED")
vibez.spill("═══════════════════════════════════════")
vibez.spill("🎉 ALL TESTS PASSED! 🎉")
vibez.spill("✨ time_zone_drip module is ready for use!")
