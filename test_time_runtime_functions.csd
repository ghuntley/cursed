yeet "testz"
yeet "time"

fr fr Test the new runtime time functions

test_start("Runtime Time Functions Test")

fr fr Test current_time_millis
sus millis1 normie = time.current_time_millis()
assert_true(millis1 > 0)
vibez.spill("Current time in milliseconds:", millis1)

fr fr Test current_time_nanos  
sus nanos1 normie = time.current_time_nanos()
assert_true(nanos1 > 0)
assert_true(nanos1 > millis1 * 1000000)
vibez.spill("Current time in nanoseconds:", nanos1)

fr fr Test format_time
sus timestamp normie = 1735934400  fr fr 2025-01-03 12:00:00 UTC
sus formatted tea = time.format_time(timestamp, "2006-01-02 15:04:05")
vibez.spill("Formatted time:", formatted)

fr fr Test parse_time
sus parsed_time Time = time.parse_time("2025-12-25 18:30:15", "2006-01-02 15:04:05")
assert_eq_int(parsed_time.year, 2025)
assert_eq_int(parsed_time.month, 12)
assert_eq_int(parsed_time.day, 25)
vibez.spill("Parsed time year:", parsed_time.year)

fr fr Test time_diff
sus start_time normie = time.current_time_millis()
sus end_time normie = start_time + 5000  fr fr 5 seconds later
sus diff normie = time.time_diff(start_time, end_time)
assert_eq_int(diff, 5000)
vibez.spill("Time difference:", diff, "ms")

fr fr Test sleep (small sleep to avoid hanging tests)
vibez.spill("Testing sleep for 10ms...")
sus before_sleep normie = time.current_time_millis()
time.sleep(10)  fr fr Sleep 10ms
sus after_sleep normie = time.current_time_millis()
sus sleep_diff normie = after_sleep - before_sleep
assert_true(sleep_diff >= 5)  fr fr At least 5ms should have passed
vibez.spill("Sleep duration:", sleep_diff, "ms")

print_test_summary()
