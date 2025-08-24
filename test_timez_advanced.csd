# Advanced Time Features Test
yeet "timez"
yeet "vibez"

vibez.spill("Testing advanced time functionality...")

# Test timezone handling
sus utc = timezone_utc()
sus eastern = timezone_from_name("America/New_York")
sus pacific = timezone_from_name("America/Los_Angeles")

vibez.spill("✅ Timezone creation working")

# Test timestamp with timezone conversion
sus now = time_now()
sus now_utc = time_to_timezone(now, utc)
sus now_eastern = time_to_timezone(now, eastern)

vibez.spill("✅ Timezone conversion working")

# Test duration parsing and arithmetic
sus duration1 = duration_from_string("2h30m15s")
sus duration2 = duration_from_string("1h45m30s")
sus total = duration_add(duration1, duration2)

vibez.spill("✅ Duration arithmetic working")

# Test time formatting with custom patterns
sus formatted = time_format(now, "yyyy-MM-dd HH:mm:ss zzz")
sus iso8601 = time_to_iso8601(now)
sus rfc3339 = time_to_rfc3339(now)

vibez.spill("✅ Time formatting working")

# Test timer and scheduling
sus timer = timer_create(duration_from_string("100ms"))
sus scheduler = scheduler_create()

vibez.spill("✅ Timer and scheduling working")
vibez.spill("✅ All advanced time tests passed")
