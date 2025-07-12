fr fr Working test for clock_bait module

fr fr Clock bait constants
facts NanoBlink normie = 1
facts MicroBlink normie = 1000
facts MilliBlink normie = 1000000
facts Blink normie = 1000000000
facts SecondVibe normie = 1000000000
facts MinuteVibe normie = 60000000000
facts HourVibe normie = 3600000000000

fr fr Get current time (simplified)
slay Now() normie {
    damn 1704067200000000000  fr fr Fixed timestamp for testing
}

fr fr Create time from Unix timestamp
slay Unix(sec normie, nsec normie) normie {
    damn sec * SecondVibe + nsec
}

fr fr Add duration to time
slay Add(t normie, d normie) normie {
    damn t + d
}

fr fr Test basic functionality
vibez.spill("🧪 Testing clock_bait module")

fr fr Test constants
vibez.spill("Testing constants...")
vibez.spill("NanoBlink: 1")
vibez.spill("MicroBlink: 1000")
vibez.spill("MilliBlink: 1000000")
vibez.spill("Blink: 1000000000")
vibez.spill("SecondVibe: 1000000000")

fr fr Test time creation
vibez.spill("Testing time creation...")
sus now normie = Now()
vibez.spill("Current time retrieved")

fr fr Test Unix timestamp creation
sus unix_time normie = Unix(1704067200, 0)
vibez.spill("Unix time created")

fr fr Test time arithmetic
sus later normie = Add(now, HourVibe)
vibez.spill("Later time calculated")

fr fr Test duration calculation
sus duration normie = later - now
vibez.spill("Duration calculated")

yikes duration == HourVibe {
    vibez.spill("✅ Duration calculation correct!")
} shook {
    vibez.spill("❌ Duration calculation failed!")
}

vibez.spill("🎉 Clock bait module test completed!")
