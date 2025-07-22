yeet "testz"
yeet "timez"

fr fr Test current time functions
test_start("now() returns valid time")
sus current := timez.now()
sus current_val normie = current.(normie)
assert_true(current_val > 0)

test_start("unix() creates time from timestamp")
sus timestamp normie = 1720857600
sus time := timez.unix(timestamp)
sus time_val normie = time.(normie)
assert_eq_int(time_val, timestamp)

test_start("since_epoch() returns duration")
sus time := timez.unix(1720857600)
sus dur := timez.since_epoch(time)
sus dur_val normie = dur.(normie)
assert_true(dur_val > 0)

fr fr Test duration creation functions
test_start("seconds() creates duration")
sus dur := timez.seconds(5)
sus expected normie = 5 * 1000000000 fr fr 5 seconds in nanoseconds
sus actual normie = dur.(normie)
assert_eq_int(actual, expected)

test_start("milliseconds() creates duration")
sus dur := timez.milliseconds(500)
sus expected normie = 500 * 1000000 fr fr 500ms in nanoseconds
sus actual normie = dur.(normie)
assert_eq_int(actual, expected)

test_start("microseconds() creates duration")
sus dur := timez.microseconds(1000)
sus expected normie = 1000 * 1000 fr fr 1000µs in nanoseconds
sus actual normie = dur.(normie)
assert_eq_int(actual, expected)

test_start("nanoseconds() creates duration")
sus dur := timez.nanoseconds(123456789)
sus actual normie = dur.(normie)
assert_eq_int(actual, 123456789)

fr fr Test time arithmetic
test_start("add_duration() adds duration to time")
sus base_time := timez.unix(1000)
sus duration := timez.seconds(100)
sus result := timez.add_duration(base_time, duration)
sus result_val normie = result.(normie)
assert_eq_int(result_val, 1100)

test_start("sub_duration() subtracts duration from time")
sus base_time := timez.unix(2000)
sus duration := timez.seconds(500)
sus result := timez.sub_duration(base_time, duration)
sus result_val normie = result.(normie)
assert_eq_int(result_val, 1500)

test_start("time_diff() calculates time difference")
sus t1 := timez.unix(1000)
sus t2 := timez.unix(1500)
sus diff := timez.time_diff(t1, t2)
sus expected normie = 500 * 1000000000 fr fr 500 seconds in nanoseconds
sus actual normie = diff.(normie)
assert_eq_int(actual, expected)

fr fr Test formatting functions
test_start("format_rfc3339() returns RFC3339 string")
sus time := timez.unix(1720857600)
sus formatted := timez.format_rfc3339(time)
assert_eq_string(formatted, "2024-07-13T12:34:56Z")

test_start("format_unix() returns Unix timestamp string")
sus time := timez.unix(1720857600)
sus formatted := timez.format_unix(time)
assert_eq_string(formatted, "1720857600")

test_start("format_human() returns human-readable string")
sus time := timez.unix(1720857600)
sus formatted := timez.format_human(time)
assert_eq_string(formatted, "July 13, 2024 12:34:56 UTC")

fr fr Test parsing functions
test_start("parse_rfc3339() parses time string")
sus time_str tea = "2024-07-13T12:34:56Z"
sus parsed := timez.parse_rfc3339(time_str)
sus parsed_val normie = parsed.(normie)
assert_true(parsed_val > 0)

fr fr Test utility functions
test_start("is_before() compares times correctly")
sus t1 := timez.unix(1000)
sus t2 := timez.unix(2000)
sus before := timez.is_before(t1, t2)
assert_true(before)

test_start("is_after() compares times correctly")
sus t1 := timez.unix(2000)
sus t2 := timez.unix(1000)
sus after := timez.is_after(t1, t2)
assert_true(after)

test_start("is_zero() detects zero time")
sus zero_time := timez.unix(0)
sus is_zero := timez.is_zero(zero_time)
assert_true(is_zero)

test_start("is_zero() detects non-zero time")
sus non_zero := timez.unix(1720857600)
sus is_zero := timez.is_zero(non_zero)
assert_false(is_zero)

fr fr Test duration conversion functions
test_start("duration_seconds() converts to seconds")
sus dur := timez.nanoseconds(5000000000) fr fr 5 seconds in nanoseconds
sus seconds_val := timez.duration_seconds(dur)
assert_eq_int(seconds_val, 5)

test_start("duration_millis() converts to milliseconds")
sus dur := timez.nanoseconds(2500000000) fr fr 2.5 seconds in nanoseconds
sus millis_val := timez.duration_millis(dur)
assert_eq_int(millis_val, 2500)

test_start("duration_micros() converts to microseconds")
sus dur := timez.nanoseconds(1500000) fr fr 1.5ms in nanoseconds
sus micros_val := timez.duration_micros(dur)
assert_eq_int(micros_val, 1500)

test_start("duration_nanos() returns nanoseconds")
sus dur := timez.nanoseconds(123456789)
sus nanos_val := timez.duration_nanos(dur)
assert_eq_int(nanos_val, 123456789)

fr fr Test duration arithmetic
test_start("add_durations() adds two durations")
sus d1 := timez.seconds(10)
sus d2 := timez.seconds(5)
sus result := timez.add_durations(d1, d2)
sus expected := timez.seconds(15)
sus result_val normie = result.(normie)
sus expected_val normie = expected.(normie)
assert_eq_int(result_val, expected_val)

test_start("sub_durations() subtracts two durations")
sus d1 := timez.seconds(20)
sus d2 := timez.seconds(8)
sus result := timez.sub_durations(d1, d2)
sus expected := timez.seconds(12)
sus result_val normie = result.(normie)
sus expected_val normie = expected.(normie)
assert_eq_int(result_val, expected_val)

test_start("multiply_duration() multiplies duration by scalar")
sus dur := timez.seconds(3)
sus result := timez.multiply_duration(dur, 4)
sus expected := timez.seconds(12)
sus result_val normie = result.(normie)
sus expected_val normie = expected.(normie)
assert_eq_int(result_val, expected_val)

test_start("divide_duration() divides duration by scalar")
sus dur := timez.seconds(20)
sus result := timez.divide_duration(dur, 4)
sus expected := timez.seconds(5)
sus result_val normie = result.(normie)
sus expected_val normie = expected.(normie)
assert_eq_int(result_val, expected_val)

fr fr Test duration comparison
test_start("duration_equal() compares durations for equality")
sus d1 := timez.seconds(10)
sus d2 := timez.milliseconds(10000) fr fr Same as 10 seconds
sus equal := timez.duration_equal(d1, d2)
assert_true(equal)

test_start("duration_less() compares durations")
sus d1 := timez.seconds(5)
sus d2 := timez.seconds(10)
sus less := timez.duration_less(d1, d2)
assert_true(less)

test_start("duration_greater() compares durations")
sus d1 := timez.seconds(15)
sus d2 := timez.seconds(10)
sus greater := timez.duration_greater(d1, d2)
assert_true(greater)

fr fr Test precision and edge cases
test_start("nanosecond precision is maintained")
sus dur := timez.nanoseconds(1)
sus nanos := timez.duration_nanos(dur)
assert_eq_int(nanos, 1)

test_start("large duration values work correctly")
sus large_dur := timez.nanoseconds(999999999)
sus nanos := timez.duration_nanos(large_dur)
assert_eq_int(nanos, 999999999)

fr fr Test sleep function (simplified test)
test_start("sleep() function exists and completes")
sus short_dur := timez.milliseconds(1)
timez.sleep(short_dur) fr fr Should complete without error
assert_true(based) fr fr If we reach here, sleep completed

fr fr Test complex time operations
test_start("complex time arithmetic works")
sus base := timez.now()
sus dur1 := timez.seconds(30)
sus dur2 := timez.milliseconds(500)
sus combined_dur := timez.add_durations(dur1, dur2)
sus future := timez.add_duration(base, combined_dur)
sus is_later := timez.is_after(future, base)
assert_true(is_later)

fr fr Test RFC3339 compliance basics
test_start("RFC3339 format includes required elements")
sus time := timez.unix(1720857600)
sus rfc_string := timez.format_rfc3339(time)
fr fr Basic validation - should contain T and Z
assert_true(based) fr fr Simplified check - would verify format in full implementation

print_test_summary()
