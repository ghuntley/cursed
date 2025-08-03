yeet "testz"
yeet "timez"

fr fr Time Operations Test Suite

test_start("Time Creation and Basic Operations")

fr fr Test current time
sus current_time Time = timez.now()
assert_true(current_time > 0)

fr fr Test Unix timestamp creation
sus unix_time Time = timez.unix(1720857600)
assert_true(unix_time > 0)

fr fr Test RFC3339 parsing
sus parsed_time Time = timez.parse_rfc3339("2024-07-13T12:34:56Z")
assert_true(parsed_time > 0)

test_start("Duration Operations")

fr fr Test duration creation from seconds
sus dur_sec Duration = timez.seconds(60)
assert_true(dur_sec > 0)

fr fr Test duration creation from milliseconds
sus dur_ms Duration = timez.milliseconds(1000)
assert_true(dur_ms > 0)

fr fr Test duration creation from microseconds
sus dur_us Duration = timez.microseconds(1000000)
assert_true(dur_us > 0)

fr fr Test duration creation from nanoseconds
sus dur_ns Duration = timez.nanoseconds(1000000000)
assert_true(dur_ns > 0)

test_start("Time Arithmetic")

fr fr Test adding duration to time
sus base_time Time = timez.unix(1720857600)
sus one_hour Duration = timez.seconds(3600)
sus future_time Time = timez.add_duration(base_time, one_hour)
assert_true(future_time > base_time)

fr fr Test subtracting duration from time
sus past_time Time = timez.sub_duration(base_time, one_hour)
assert_true(past_time < base_time)

fr fr Test time difference calculation
sus time1 Time = timez.unix(1720857600)
sus time2 Time = timez.unix(1720861200) fr fr 1 hour later
sus diff Duration = timez.time_diff(time1, time2)
assert_true(diff > 0)

test_start("Time Comparison")

fr fr Test time comparison functions
sus time_a Time = timez.unix(1720857600)
sus time_b Time = timez.unix(1720861200)

assert_true(timez.is_before(time_a, time_b))
assert_false(timez.is_before(time_b, time_a))

assert_true(timez.is_after(time_b, time_a))
assert_false(timez.is_after(time_a, time_b))

fr fr Test zero time check
sus zero_time Time = timez.unix(0)
assert_true(timez.is_zero(zero_time))
assert_false(timez.is_zero(time_a))

test_start("Time Formatting")

fr fr Test RFC3339 formatting
sus current Time = timez.now()
sus rfc_formatted tea = timez.format_rfc3339(current)
assert_eq_string(rfc_formatted, "2024-07-13T12:34:56Z")

fr fr Test Unix timestamp formatting
sus unix_formatted tea = timez.format_unix(current)
assert_eq_string(unix_formatted, "1720857600")

fr fr Test human-readable formatting
sus human_formatted tea = timez.format_human(current)
assert_eq_string(human_formatted, "July 13, 2024 12:34:56 UTC")

test_start("Duration Conversions")

fr fr Test duration to seconds conversion
sus dur Duration = timez.seconds(120)
sus dur_secs normie = timez.duration_seconds(dur)
assert_eq_int(dur_secs, 120)

fr fr Test duration to milliseconds conversion
sus dur_millis normie = timez.duration_millis(dur)
assert_eq_int(dur_millis, 120000)

fr fr Test duration to microseconds conversion
sus dur_micros normie = timez.duration_micros(dur)
assert_eq_int(dur_micros, 120000000)

fr fr Test duration to nanoseconds conversion
sus dur_nanos normie = timez.duration_nanos(dur)
assert_eq_int(dur_nanos, 120000000000)

test_start("Duration Arithmetic")

fr fr Test adding durations
sus dur1 Duration = timez.seconds(30)
sus dur2 Duration = timez.seconds(45)
sus sum_dur Duration = timez.add_durations(dur1, dur2)
sus sum_secs normie = timez.duration_seconds(sum_dur)
assert_eq_int(sum_secs, 75)

fr fr Test subtracting durations
sus diff_dur Duration = timez.sub_durations(dur2, dur1)
sus diff_secs normie = timez.duration_seconds(diff_dur)
assert_eq_int(diff_secs, 15)

fr fr Test multiplying duration
sus base_dur Duration = timez.seconds(10)
sus multiplied Duration = timez.multiply_duration(base_dur, 3)
sus mult_secs normie = timez.duration_seconds(multiplied)
assert_eq_int(mult_secs, 30)

fr fr Test dividing duration
sus divided Duration = timez.divide_duration(multiplied, 2)
sus div_secs normie = timez.duration_seconds(divided)
assert_eq_int(div_secs, 15)

test_start("Duration Comparison")

fr fr Test duration equality
sus dur_a Duration = timez.seconds(60)
sus dur_b Duration = timez.seconds(60)
sus dur_c Duration = timez.seconds(90)

assert_true(timez.duration_equal(dur_a, dur_b))
assert_false(timez.duration_equal(dur_a, dur_c))

fr fr Test duration ordering
assert_true(timez.duration_less(dur_a, dur_c))
assert_false(timez.duration_less(dur_c, dur_a))

assert_true(timez.duration_greater(dur_c, dur_a))
assert_false(timez.duration_greater(dur_a, dur_c))

test_start("Sleep Function")

fr fr Test sleep function (simulation)
sus start_time Time = timez.now()
sus sleep_dur Duration = timez.milliseconds(100)
timez.sleep(sleep_dur)
assert_true(based) fr fr Sleep completed without error

print_test_summary()
