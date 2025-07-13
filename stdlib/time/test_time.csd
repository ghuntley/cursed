yeet "testz"
yeet "time"

# ==========================================
# CURSED Time Module - Comprehensive Test Suite
# ==========================================

test_start("time comprehensive test suite")

# ==========================================
# Core Time Function Tests
# ==========================================

# Test current time
sus current_time thicc = now()
assert_true(current_time >= 0)

# Test unix timestamp creation
sus epoch_time thicc = unix(0)
assert_eq_int(epoch_time, 0)

sus known_time thicc = unix(1704067200)
assert_eq_int(known_time, 1704067200)

# Test RFC3339 parsing (placeholder)
sus parsed_time thicc = parse_rfc3339("2024-01-01T00:00:00Z")
assert_eq_int(parsed_time, 1704067200)

# Test since epoch
sus epoch_duration thicc = since_epoch(1704067200)
assert_eq_int(epoch_duration, 1704067200000000000) # In nanoseconds

# ==========================================
# Duration Creation Tests
# ==========================================

# Test duration from seconds
sus five_sec thicc = seconds(5)
assert_eq_int(five_sec, 5000000000) # 5 * 1e9 nanoseconds

# Test duration from milliseconds
sus hundred_ms thicc = milliseconds(100)
assert_eq_int(hundred_ms, 100000000) # 100 * 1e6 nanoseconds

# Test duration from microseconds
sus fifty_us thicc = microseconds(50)
assert_eq_int(fifty_us, 50000) # 50 * 1e3 nanoseconds

# Test duration from nanoseconds
sus ten_ns thicc = nanoseconds(10)
assert_eq_int(ten_ns, 10)

# ==========================================
# Time Arithmetic Tests
# ==========================================

# Test adding duration to time
sus base_time thicc = unix(1000)
sus duration_5sec thicc = seconds(5)
sus future_time thicc = add_duration(base_time, duration_5sec)
assert_eq_int(future_time, 1005)

# Test subtracting duration from time
sus past_time thicc = sub_duration(base_time, duration_5sec)
assert_eq_int(past_time, 995)

# Test time difference
sus t1 thicc = unix(1000)
sus t2 thicc = unix(1005)
sus diff thicc = time_diff(t2, t1)
assert_eq_int(diff, 5000000000) # 5 seconds in nanoseconds

# Test time component addition
sus plus_seconds thicc = time_add_seconds(base_time, 30)
assert_eq_int(plus_seconds, 1030)

sus plus_minutes thicc = time_add_minutes(base_time, 2)
assert_eq_int(plus_minutes, 1120)

sus plus_hours thicc = time_add_hours(base_time, 1)
assert_eq_int(plus_hours, 4600)

sus plus_days thicc = time_add_days(base_time, 1)
assert_eq_int(plus_days, 87400)

# ==========================================
# Duration Arithmetic Tests
# ==========================================

# Test duration addition
sus dur1 thicc = seconds(30)
sus dur2 thicc = seconds(45)
sus dur_sum thicc = duration_add(dur1, dur2)
assert_eq_int(dur_sum, 75000000000) # 75 seconds in nanoseconds

# Test duration subtraction
sus dur_diff thicc = duration_subtract(dur2, dur1)
assert_eq_int(dur_diff, 15000000000) # 15 seconds in nanoseconds

# Test duration conversion
sus test_dur thicc = seconds(120)
sus in_seconds normie = duration_to_seconds(test_dur)
assert_eq_int(in_seconds, 120)

sus in_millis normie = duration_to_millis(test_dur)
assert_eq_int(in_millis, 120000)

# ==========================================
# Time Comparison Tests
# ==========================================

# Test time comparisons
sus early thicc = unix(1000)
sus late thicc = unix(2000)

assert_true(is_before(early, late))
assert_false(is_before(late, early))

assert_true(is_after(late, early))
assert_false(is_after(early, late))

# Test zero time
sus zero_time thicc = unix(0)
assert_true(is_zero(zero_time))
assert_false(is_zero(early))

# Test time equality
sus same_time thicc = unix(1000)
assert_true(time_equals(early, same_time))
assert_false(time_equals(early, late))

# Test time min/max
sus min_time thicc = time_min(early, late)
assert_eq_int(min_time, 1000)

sus max_time thicc = time_max(early, late)
assert_eq_int(max_time, 2000)

# ==========================================
# Time Component Tests
# ==========================================

# Test time component extraction
sus test_time thicc = time_create(2024, 6, 15, 14, 30, 45)

# Test hour extraction
sus hour normie = time_hour(test_time)
assert_true(hour >= 0 && hour <= 23)

# Test minute extraction
sus minute normie = time_minute(test_time)
assert_true(minute >= 0 && minute <= 59)

# Test second extraction
sus second normie = time_second(test_time)
assert_true(second >= 0 && second <= 59)

# Test year extraction (placeholder)
sus year normie = time_year(test_time)
assert_eq_int(year, 2024)

# ==========================================
# Date Validation Tests
# ==========================================

# Test leap year detection
assert_true(time_is_leap_year(2020))
assert_false(time_is_leap_year(2021))
assert_true(time_is_leap_year(2000))
assert_false(time_is_leap_year(1900))

# Test days in month
assert_eq_int(time_days_in_month(2021, 1), 31)
assert_eq_int(time_days_in_month(2021, 2), 28)
assert_eq_int(time_days_in_month(2020, 2), 29)
assert_eq_int(time_days_in_month(2021, 4), 30)

# Test date validation
assert_true(time_is_valid_date(2021, 6, 15))
assert_true(time_is_valid_date(2020, 2, 29))
assert_false(time_is_valid_date(2021, 2, 29))
assert_false(time_is_valid_date(2021, 13, 1))
assert_false(time_is_valid_date(2021, 6, 32))

# ==========================================
# Formatting Tests
# ==========================================

# Test time formatting
sus format_test_time thicc = unix(1704067200)

sus rfc_string tea = format_rfc3339(format_test_time)
assert_true(rfc_string == "2024-01-01T00:00:00Z")

sus unix_string tea = format_unix(format_test_time)
assert_true(unix_string == "1704067200")

sus human_string tea = format_human(format_test_time)
assert_true(human_string == "Mon Jan 1 00:00:00 2024")

# ==========================================
# Extended Function Tests
# ==========================================

# Test extended time functions
sus now_timestamp thicc = time_now()
assert_true(now_timestamp >= 0)

sus now_millis thicc = time_now_millis()
assert_true(now_millis >= 0)

sus now_nanos thicc = time_now_nanos()
assert_true(now_nanos >= 0)

# Test time from timestamp
sus ts_time thicc = time_from_timestamp(1704067200)
assert_eq_int(ts_time, 1704067200)

# Test time from millis
sus millis_time thicc = time_from_millis(1704067200000)
assert_eq_int(millis_time, 1704067200)

# ==========================================
# Duration Constants Tests
# ==========================================

# Test all duration constants
sus sec_const thicc = duration_second()
assert_eq_int(sec_const, 1000000000)

sus min_const thicc = duration_minute()
assert_eq_int(min_const, 60000000000)

sus hour_const thicc = duration_hour()
assert_eq_int(hour_const, 3600000000000)

sus day_const thicc = duration_day()
assert_eq_int(day_const, 86400000000000)

# ==========================================
# Edge Cases Tests
# ==========================================

# Test zero values
sus zero_duration thicc = nanoseconds(0)
assert_eq_int(zero_duration, 0)

# Test large values
sus large_time thicc = unix(2147483647) # Max 32-bit int
assert_eq_int(large_time, 2147483647)

sus large_duration thicc = seconds(1000000)
assert_eq_int(large_duration, 1000000000000000) # 1M seconds in nanoseconds

# Test negative durations (subtraction result)
sus pos_dur thicc = seconds(100)
sus neg_dur thicc = seconds(150)
sus result thicc = duration_subtract(pos_dur, neg_dur)
assert_eq_int(result, -50000000000) # -50 seconds in nanoseconds

# ==========================================
# Sleep Function Test
# ==========================================

# Test sleep function (placeholder - just ensure it doesn't crash)
sus short_duration thicc = milliseconds(1)
sleep(short_duration)

# ==========================================
# Performance and Stress Tests
# ==========================================

# Test many time operations
for i in range(1, 100) {
    sus test_t thicc = unix(i)
    sus test_d thicc = seconds(i)
    sus result_t thicc = add_duration(test_t, test_d)
    assert_true(result_t > test_t)
}

print_test_summary()
