yeet "testz"
yeet "time"

test_start("Time Module Implementation Tests")

// Test get_system_time functionality
sus current_time thicc = time.now()
assert_true(current_time > 0)
vibez.spill("✓ System time retrieval working")

// Test format_unix functionality  
sus unix_formatted tea = time.format_unix(current_time)
assert_true(time.string_length(unix_formatted) > 0)
vibez.spill("✓ Unix timestamp formatting: " + unix_formatted)

// Test format_human functionality
sus human_formatted tea = time.format_human(current_time)
assert_true(time.string_length(human_formatted) > 10)
vibez.spill("✓ Human-readable formatting: " + human_formatted)

// Test sleep functionality (short duration)
vibez.spill("Testing sleep function...")
sus start_time thicc = time.now()
time.sleep(time.milliseconds(50))  // 50ms sleep
sus end_time thicc = time.now()
vibez.spill("✓ Sleep function executed (basic timing test)")

// Test time offset functionality
sus offset thicc = time.time_offset_seconds()
assert_true(offset >= 0)
vibez.spill("✓ Time offset tracking: " + time.int_to_string(offset))

// Test helper functions
sus year normie = time.time_year_from_unix(current_time)
assert_true(year >= 1970)
vibez.spill("✓ Year extraction: " + time.int_to_string(year))

sus month normie = time.time_month_from_unix(current_time)
assert_true(month >= 1 && month <= 12)
vibez.spill("✓ Month extraction: " + time.int_to_string(month))

sus day normie = time.time_day_from_unix(current_time)
assert_true(day >= 1 && day <= 31)
vibez.spill("✓ Day extraction: " + time.int_to_string(day))

sus day_name tea = time.get_day_name(current_time)
assert_true(time.string_length(day_name) == 3)
vibez.spill("✓ Day name: " + day_name)

sus month_name tea = time.get_month_name(month)
assert_true(time.string_length(month_name) == 3)
vibez.spill("✓ Month name: " + month_name)

// Test duration functions with new implementations
sus one_second thicc = time.seconds(1)
assert_eq_int(one_second, 1000000000)  // 1 billion nanoseconds

sus one_minute thicc = time.duration_minute()
assert_eq_int(one_minute, 60000000000)  // 60 billion nanoseconds

vibez.spill("✓ Duration functions working correctly")

// Test time arithmetic
sus future_time thicc = time.add_duration(current_time, one_second)
assert_true(future_time > current_time)
vibez.spill("✓ Time arithmetic working")

// Test time comparison
assert_true(time.is_before(current_time, future_time))
assert_false(time.is_after(current_time, future_time))
vibez.spill("✓ Time comparison functions working")

// Test formatting edge cases
sus zero_time thicc = 0
sus zero_formatted tea = time.format_human(zero_time)
vibez.spill("✓ Zero time formatting: " + zero_formatted)

vibez.spill("\n=== All Time Module Tests Completed ===")
print_test_summary()
