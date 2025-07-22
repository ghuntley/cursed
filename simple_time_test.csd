yeet "time"

// Simple time module test without testz dependency
vibez.spill("=== Time Module Implementation Test ===")

// Test get_system_time functionality
sus current_time thicc = time.now()
vibez.spill("✓ Current system time: " + time.format_unix(current_time))

// Test format_unix functionality  
sus unix_formatted tea = time.format_unix(current_time)
vibez.spill("✓ Unix timestamp formatting: " + unix_formatted)

// Test format_human functionality
sus human_formatted tea = time.format_human(current_time)
vibez.spill("✓ Human-readable formatting: " + human_formatted)

// Test sleep functionality (short duration)
vibez.spill("Testing sleep function...")
time.sleep(time.milliseconds(10))  // 10ms sleep
vibez.spill("✓ Sleep function executed")

// Test time offset functionality
sus offset thicc = time.time_offset_seconds()
vibez.spill("✓ Time offset tracking: " + time.int_to_string(offset))

// Test helper functions
sus year normie = time.time_year_from_unix(current_time)
vibez.spill("✓ Year extraction: " + time.int_to_string(year))

sus month normie = time.time_month_from_unix(current_time)
vibez.spill("✓ Month extraction: " + time.int_to_string(month))

sus day normie = time.time_day_from_unix(current_time)
vibez.spill("✓ Day extraction: " + time.int_to_string(day))

sus day_name tea = time.get_day_name(current_time)
vibez.spill("✓ Day name: " + day_name)

sus month_name tea = time.get_month_name(month)
vibez.spill("✓ Month name: " + month_name)

// Test duration functions
sus one_second thicc = time.seconds(1)
vibez.spill("✓ One second duration: " + time.int_to_string(one_second))

sus one_minute thicc = time.duration_minute()
vibez.spill("✓ One minute duration: " + time.int_to_string(one_minute))

// Test time arithmetic
sus future_time thicc = time.add_duration(current_time, one_second)
vibez.spill("✓ Time arithmetic - future time: " + time.format_unix(future_time))

// Test time comparison
if time.is_before(current_time, future_time) {
    vibez.spill("✓ Time comparison - before check works")
} else {
    vibez.spill("✗ Time comparison - before check failed")
}

if !time.is_after(current_time, future_time) {
    vibez.spill("✓ Time comparison - after check works")
} else {
    vibez.spill("✗ Time comparison - after check failed")
}

// Test formatting edge cases
sus zero_time thicc = 0
sus zero_formatted tea = time.format_human(zero_time)
vibez.spill("✓ Zero time formatting: " + zero_formatted)

// Test different time values
sus test_time thicc = 1609459200  // 2021-01-01 00:00:00 UTC
sus test_formatted tea = time.format_human(test_time)
vibez.spill("✓ Test time formatting: " + test_formatted)

vibez.spill("=== All Time Module Tests Completed ===")
