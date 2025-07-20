yeet "testz"
yeet "time"

test_start("Time Module Tests")

// Test basic time functions
assert_true(now() > 0)
assert_true(unix(1704067200) == 1704067200)

// Test duration functions  
assert_eq_int(seconds(5), 5000000000)
assert_eq_int(milliseconds(1000), 1000000000)
assert_eq_int(microseconds(1000000), 1000000000)

// Test time arithmetic
sus base_time thicc = 1704067200
sus duration thicc = seconds(3600)  // 1 hour
sus new_time thicc = add_duration(base_time, duration)
assert_true(new_time > base_time)

// Test time comparison
assert_true(is_before(1000, 2000))
assert_true(is_after(2000, 1000))
assert_false(is_zero(1704067200))
assert_true(is_zero(0))

// Test date validation
assert_true(time_is_leap_year(2024))
assert_false(time_is_leap_year(2023))
assert_eq_int(time_days_in_month(2024, 2), 29)
assert_eq_int(time_days_in_month(2023, 2), 28)
assert_true(time_is_valid_date(2024, 2, 29))
assert_false(time_is_valid_date(2023, 2, 29))

// Test time formatting (basic)
sus formatted tea = format_rfc3339(1704067200)
assert_true(string_length(formatted) > 0)

// Test duration constants
assert_true(duration_second() > 0)
assert_true(duration_minute() > duration_second())
assert_true(duration_hour() > duration_minute())
assert_true(duration_day() > duration_hour())

print_test_summary()
