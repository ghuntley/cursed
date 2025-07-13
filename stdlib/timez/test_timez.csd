yeet "testz"
yeet "timez"

# Comprehensive timez module test suite

# Test time creation and basic operations
test_start("Time creation and basic operations")

# Test now() function
sus current_time := timez.now()
assert_true(timez.is_valid_time(current_time))

# Test unix timestamp creation
sus epoch_time := timez.unix(0)
assert_true(timez.is_zero(epoch_time))

sus test_time := timez.unix(1640995200)  # 2022-01-01 00:00:00 UTC
assert_true(timez.is_valid_time(test_time))

print_test_summary()

# Test duration creation
test_start("Duration creation")

sus five_seconds := timez.seconds(5)
assert_eq_int(timez.duration_seconds(five_seconds), 5)

sus hundred_millis := timez.milliseconds(100)
assert_eq_int(timez.duration_milliseconds(hundred_millis), 100)

sus thousand_micros := timez.microseconds(1000)
assert_eq_int(timez.duration_microseconds(thousand_micros), 1000)

sus million_nanos := timez.nanoseconds(1000000)
assert_eq_int(timez.duration_nanoseconds(million_nanos), 1000000)

print_test_summary()

# Test time arithmetic
test_start("Time arithmetic operations")

sus base_time := timez.unix(1640995200)
sus one_hour := timez.HOUR
sus future_time := timez.add_duration(base_time, one_hour)

assert_true(timez.is_after(future_time, base_time))
assert_false(timez.is_before(future_time, base_time))

sus past_time := timez.sub_duration(base_time, one_hour)
assert_true(timez.is_before(past_time, base_time))

print_test_summary()

# Test duration arithmetic
test_start("Duration arithmetic")

sus dur1 := timez.seconds(30)
sus dur2 := timez.seconds(20)
sus combined := timez.add_durations(dur1, dur2)
assert_eq_int(timez.duration_seconds(combined), 50)

sus difference := timez.sub_durations(dur1, dur2)
assert_eq_int(timez.duration_seconds(difference), 10)

print_test_summary()

# Test time formatting
test_start("Time formatting")

sus test_time_fmt := timez.unix(1640995200)
sus rfc3339_str := timez.format_rfc3339(test_time_fmt)
assert_true(rfc3339_str != "")

sus unix_str := timez.format_unix(test_time_fmt)
assert_true(unix_str != "")

sus human_str := timez.format_human(test_time_fmt)
assert_true(human_str != "")

print_test_summary()

# Test duration formatting
test_start("Duration formatting")

sus test_duration := timez.add_durations(
    timez.add_durations(timez.HOUR, timez.MINUTE),
    timez.seconds(30)
)
sus duration_str := timez.format_duration(test_duration)
assert_true(duration_str != "")

print_test_summary()

# Test time comparison
test_start("Time comparison")

sus time1 := timez.unix(1640995200)
sus time2 := timez.unix(1640995300)  # 100 seconds later

assert_true(timez.is_before(time1, time2))
assert_false(timez.is_after(time1, time2))
assert_true(timez.is_after(time2, time1))
assert_false(timez.is_before(time2, time1))

print_test_summary()

# Test time difference calculation
test_start("Time difference calculation")

sus start_time := timez.unix(1640995200)
sus end_time := timez.unix(1640995260)  # 60 seconds later
sus diff := timez.time_diff(end_time, start_time)

assert_eq_int(timez.duration_seconds(diff), 60)

print_test_summary()

# Test sleep function (simplified test)
test_start("Sleep function")

sus before_sleep := timez.now()
timez.sleep(timez.milliseconds(10))
sus after_sleep := timez.now()

assert_true(timez.is_after(after_sleep, before_sleep))

print_test_summary()

# Test time constants
test_start("Time constants")

assert_eq_int(timez.duration_seconds(timez.MINUTE), 60)
assert_eq_int(timez.duration_seconds(timez.HOUR), 3600)
assert_eq_int(timez.duration_seconds(timez.DAY), 86400)
assert_eq_int(timez.duration_seconds(timez.WEEK), 604800)

print_test_summary()

# Test timezone functions
test_start("Timezone functions")

assert_eq_int(timez.utc_offset(), 0)
assert_true(timez.is_utc())

print_test_summary()

# Test time validation
test_start("Time validation")

sus valid_time := timez.unix(1640995200)
assert_true(timez.is_valid_time(valid_time))

sus valid_duration := timez.seconds(100)
assert_true(timez.is_valid_duration(valid_duration))

print_test_summary()

# Test high precision operations
test_start("High precision operations")

sus nano_time := timez.now_nano()
assert_true(nano_time > 0)

sus precision_time := timez.unix(1640995200)
sus nano_added := timez.add_nano(precision_time, 1000)
assert_true(timez.is_after(nano_added, precision_time))

print_test_summary()

# Test edge cases
test_start("Edge cases")

# Test zero time
sus zero_time := timez.unix(0)
assert_true(timez.is_zero(zero_time))

# Test epoch functions
sus epoch_duration := timez.since_epoch(timez.unix(1640995200))
assert_true(timez.duration_nanoseconds(epoch_duration) > 0)

print_test_summary()

# Test parsing functions
test_start("Time parsing")

sus parsed_rfc := timez.parse_rfc3339("2022-01-01T00:00:00Z")
assert_true(timez.is_valid_time(parsed_rfc))

sus parsed_unix := timez.parse_unix_string("1640995200")
assert_true(timez.is_valid_time(parsed_unix))

print_test_summary()

vibez.spill("All timez module tests completed successfully!")
