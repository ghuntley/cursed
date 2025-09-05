yeet "testz"
yeet "time"

fr fr ========================================
fr fr Time Module Comprehensive Tests
fr fr ========================================

test_start("Time Creation Tests")

fr fr Test current time
sus current_time Time = now()
assert_true(current_time.year >= 2025)
assert_true(current_time.month >= 1 && current_time.month <= 12)
assert_true(current_time.day >= 1 && current_time.day <= 31)

fr fr Test Unix timestamp
sus timestamp normie = unix()
assert_true(timestamp > 1700000000)  fr fr After 2023

fr fr Test time from Unix
sus from_ts Time = from_unix(1609459200)  fr fr 2021-01-01 00:00:00 UTC
assert_eq_int(from_ts.year, 2021)

fr fr Test date creation
sus custom_date Time = date(2025, 6, 15, 14, 30, 45)
assert_eq_int(custom_date.year, 2025)
assert_eq_int(custom_date.month, 6)
assert_eq_int(custom_date.day, 15)
assert_eq_int(custom_date.hour, 14)
assert_eq_int(custom_date.minute, 30)
assert_eq_int(custom_date.second, 45)

test_start("Duration Tests")

fr fr Test duration constants
sus one_sec Duration = second()
assert_eq_int(one_sec.nanoseconds, 1000000000)

sus one_min Duration = minute()
assert_eq_int(one_min.nanoseconds, 60000000000)

sus one_hour Duration = hour()
assert_eq_int(one_hour.nanoseconds, 3600000000000)

fr fr Test duration conversion
sus test_duration Duration = Duration{nanoseconds: 5000000000}  fr fr 5 seconds
assert_eq_int(test_duration.seconds(), 5)
assert_eq_int(test_duration.milliseconds(), 5000)

test_start("Time Arithmetic Tests")

fr fr Test time addition
sus base_time Time = date(2025, 1, 1, 12, 0, 0)
sus one_hour_duration Duration = hour()
sus later_time Time = base_time.add(one_hour_duration)
assert_eq_int(later_time.hour, 13)

fr fr Test time comparison
sus earlier Time = date(2025, 1, 1, 10, 0, 0)
sus later Time = date(2025, 1, 1, 11, 0, 0)
assert_true(earlier.before(later))
assert_true(later.after(earlier))
assert_false(earlier.equal(later))

test_start("Time Formatting Tests")

fr fr Test basic formatting
sus test_time Time = date(2025, 3, 15, 9, 30, 45)
sus formatted tea = test_time.format("2006-01-02 15:04:05")
assert_eq_string(formatted, "2025-03-15 09:30:45")

fr fr Test date-only formatting
sus date_only tea = test_time.format("2006-01-02")
assert_eq_string(date_only, "2025-03-15")

fr fr Test time-only formatting
sus time_only tea = test_time.format("15:04:05")
assert_eq_string(time_only, "09:30:45")

test_start("Time Parsing Tests")

fr fr Test parsing standard format
sus parsed_time Time = parse("2006-01-02 15:04:05", "2025-12-25 18:30:15")
assert_eq_int(parsed_time.year, 2025)
assert_eq_int(parsed_time.month, 12)
assert_eq_int(parsed_time.day, 25)
assert_eq_int(parsed_time.hour, 18)
assert_eq_int(parsed_time.minute, 30)
assert_eq_int(parsed_time.second, 15)

test_start("Weekday and Month Names Tests")

fr fr Test weekday names
sus friday Time = date(2025, 1, 3, 12, 0, 0)  fr fr 2025-01-03 is Friday
friday.weekday = 5
sus weekday_name tea = friday.weekday_name()
assert_eq_string(weekday_name, "Friday")

fr fr Test month names
sus june_time Time = date(2025, 6, 1, 12, 0, 0)
sus month_name tea = june_time.month_name()
assert_eq_string(month_name, "June")

test_start("Duration String Representation Tests")

fr fr Test hours representation
sus hours_duration Duration = Duration{nanoseconds: 7200000000000}  fr fr 2 hours
sus hours_str tea = hours_duration.string()
assert_eq_string(hours_str, "2h0m")

fr fr Test minutes representation
sus minutes_duration Duration = Duration{nanoseconds: 300000000000}  fr fr 5 minutes
sus minutes_str tea = minutes_duration.string()
assert_eq_string(minutes_str, "5m0s")

fr fr Test seconds representation
sus seconds_duration Duration = Duration{nanoseconds: 3000000000}  fr fr 3 seconds
sus seconds_str tea = seconds_duration.string()
assert_eq_string(seconds_str, "3s")

test_start("Stopwatch Tests")

fr fr Test stopwatch creation
sus sw Stopwatch = new_stopwatch()
assert_false(sw.running)

fr fr Test stopwatch start/stop
sw.start()
assert_true(sw.running)

sus elapsed Duration = sw.stop()
assert_false(sw.running)
assert_true(elapsed.nanoseconds >= 0)

test_start("Timer Tests")

fr fr Test timer creation
sus timer_duration Duration = Duration{nanoseconds: 1000000000}  fr fr 1 second
sus timer Timer = new_timer(timer_duration)
assert_eq_int(timer.duration.nanoseconds, 1000000000)

fr fr Test timer reset
sus new_duration Duration = Duration{nanoseconds: 2000000000}  fr fr 2 seconds
timer.reset(new_duration)
assert_eq_int(timer.duration.nanoseconds, 2000000000)

test_start("Time Zone Tests")

fr fr Test UTC timezone
sus utc_tz Location = utc()
assert_eq_string(utc_tz.name, "UTC")
assert_eq_int(utc_tz.offset, 0)

fr fr Test local timezone
sus local_tz Location = local()
assert_eq_string(local_tz.name, "Local")

test_start("Unix Timestamp Variants Tests")

fr fr Test millisecond precision
sus milli_ts normie = unix_milli()
assert_true(milli_ts > unix() * 1000)

fr fr Test microsecond precision
sus micro_ts normie = unix_micro()
assert_true(micro_ts > unix() * 1000000)

fr fr Test nanosecond precision
sus nano_ts normie = unix_nano()
assert_true(nano_ts > unix() * 1000000000)

test_start("Advanced Time Operations Tests")

fr fr Test time truncation
sus test_truncate Time = date(2025, 1, 1, 12, 34, 56)
sus hour_duration Duration = hour()
sus truncated Time = test_truncate.truncate(hour_duration)
assert_eq_int(truncated.minute, 0)
assert_eq_int(truncated.second, 0)

fr fr Test duration since
sus start_time Time = date(2025, 1, 1, 10, 0, 0)
sus end_time Time = date(2025, 1, 1, 11, 0, 0)
sus duration_between Duration = end_time.since(start_time)
assert_eq_int(duration_between.hours(), 1)

print_test_summary()
