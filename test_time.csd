yeet "testz"
yeet "time"

fr fr ========================================
fr fr CURSED Time Library Test Suite
fr fr ========================================

slay test_current_time() {
    test_start("Current Time Functions")
    
    fr fr Test current time functions
    sus now_seconds normie = time_now()
    sus now_millis normie = time_now_millis()
    sus now_micros normie = time_now_micros()
    sus now_nanos normie = time_now_nanos()
    
    assert_true(now_seconds > 0)
    assert_true(now_millis > 0)
    assert_true(now_micros > 0)
    assert_true(now_nanos > 0)
    
    fr fr Check time relationships
    assert_true(now_millis >= now_seconds * 1000)
    assert_true(now_micros >= now_millis * 1000)
    
    fr fr Test time progression
    sus later_seconds normie = time_now()
    assert_true(later_seconds >= now_seconds)
}

slay test_time_creation() {
    test_start("Time Creation Functions")
    
    fr fr Test time from timestamp
    sus timestamp normie = 1609459200
    sus dt datetime = time_from_timestamp(timestamp)
    assert_true(dt != cringe)
    
    fr fr Test time from millis
    sus millis normie = 1609459200000
    sus dt_millis datetime = time_from_millis(millis)
    assert_true(dt_millis != cringe)
    
    fr fr Test time creation with components
    sus created_time datetime = time_create(2021, 1, 1, 0, 0, 0)
    assert_true(created_time != cringe)
    
    fr fr Test time parsing
    sus parsed_time datetime = time_parse("2021-01-01", "%Y-%m-%d")
    assert_true(parsed_time != cringe)
}

slay test_time_components() {
    test_start("Time Component Extraction")
    
    fr fr Create a known time
    sus test_time datetime = time_create(2021, 12, 25, 15, 30, 45)
    
    fr fr Test component extraction
    assert_eq_int(time_year(test_time), 2021)
    assert_eq_int(time_month(test_time), 12)
    assert_eq_int(time_day(test_time), 25)
    assert_eq_int(time_hour(test_time), 15)
    assert_eq_int(time_minute(test_time), 30)
    assert_eq_int(time_second(test_time), 45)
    
    fr fr Test weekday and day of year
    sus weekday normie = time_weekday(test_time)
    sus day_of_year normie = time_day_of_year(test_time)
    
    assert_true(weekday >= 0 && weekday <= 6)
    assert_true(day_of_year >= 1 && day_of_year <= 366)
}

slay test_time_formatting() {
    test_start("Time Formatting Functions")
    
    fr fr Create test time
    sus test_time datetime = time_create(2021, 6, 15, 9, 30, 0)
    
    fr fr Test basic formatting
    sus formatted tea = time_format(test_time, "%Y-%m-%d %H:%M:%S")
    assert_true(string_contains(formatted, "2021"))
    assert_true(string_contains(formatted, "06"))
    assert_true(string_contains(formatted, "15"))
    
    fr fr Test string conversion
    sus time_str tea = time_to_string(test_time)
    assert_true(string_len(time_str) > 0)
    
    fr fr Test ISO8601 format
    sus iso_str tea = time_to_iso8601(test_time)
    assert_true(string_contains(iso_str, "T"))
    assert_true(string_contains(iso_str, "2021"))
    
    fr fr Test RFC3339 format
    sus rfc_str tea = time_to_rfc3339(test_time)
    assert_true(string_len(rfc_str) > 0)
}

slay test_time_arithmetic() {
    test_start("Time Arithmetic Functions")
    
    fr fr Create base time
    sus base_time datetime = time_create(2021, 6, 15, 12, 0, 0)
    
    fr fr Test adding years
    sus plus_year datetime = time_add_years(base_time, 1)
    assert_eq_int(time_year(plus_year), 2022)
    assert_eq_int(time_month(plus_year), 6)
    assert_eq_int(time_day(plus_year), 15)
    
    fr fr Test adding months
    sus plus_month datetime = time_add_months(base_time, 3)
    assert_eq_int(time_year(plus_month), 2021)
    assert_eq_int(time_month(plus_month), 9)
    assert_eq_int(time_day(plus_month), 15)
    
    fr fr Test adding days
    sus plus_days datetime = time_add_days(base_time, 10)
    assert_eq_int(time_year(plus_days), 2021)
    assert_eq_int(time_month(plus_days), 6)
    assert_eq_int(time_day(plus_days), 25)
    
    fr fr Test adding hours
    sus plus_hours datetime = time_add_hours(base_time, 5)
    assert_eq_int(time_hour(plus_hours), 17)
    
    fr fr Test adding minutes
    sus plus_minutes datetime = time_add_minutes(base_time, 30)
    assert_eq_int(time_minute(plus_minutes), 30)
    
    fr fr Test adding seconds
    sus plus_seconds datetime = time_add_seconds(base_time, 45)
    assert_eq_int(time_second(plus_seconds), 45)
}

slay test_time_differences() {
    test_start("Time Difference Functions")
    
    fr fr Create two test times
    sus time1 datetime = time_create(2021, 6, 15, 12, 0, 0)
    sus time2 datetime = time_create(2021, 6, 16, 15, 30, 45)
    
    fr fr Test difference in days
    sus diff_days normie = time_diff_days(time2, time1)
    assert_eq_int(diff_days, 1)
    
    fr fr Test difference in hours
    sus diff_hours normie = time_diff_hours(time2, time1)
    assert_true(diff_hours >= 27 && diff_hours <= 28)
    
    fr fr Test difference in minutes
    sus diff_minutes normie = time_diff_minutes(time2, time1)
    assert_true(diff_minutes >= 1650 && diff_minutes <= 1700)
    
    fr fr Test difference in seconds
    sus diff_seconds normie = time_diff_seconds(time2, time1)
    assert_true(diff_seconds > 99000)
    
    fr fr Test time subtraction
    sus duration_diff duration = time_subtract(time2, time1)
    assert_true(duration_diff != cringe)
}

slay test_duration_operations() {
    test_start("Duration Operations")
    
    fr fr Test duration creation
    sus dur_sec duration = duration_from_seconds(3600)
    sus dur_millis duration = duration_from_millis(3600000)
    
    assert_true(dur_sec != cringe)
    assert_true(dur_millis != cringe)
    
    fr fr Test duration conversion
    assert_eq_int(duration_to_seconds(dur_sec), 3600)
    assert_eq_int(duration_to_millis(dur_millis), 3600000)
    
    fr fr Test duration arithmetic
    sus dur1 duration = duration_from_seconds(1800)
    sus dur2 duration = duration_from_seconds(1200)
    
    sus dur_sum duration = duration_add(dur1, dur2)
    sus dur_diff duration = duration_subtract(dur1, dur2)
    
    assert_eq_int(duration_to_seconds(dur_sum), 3000)
    assert_eq_int(duration_to_seconds(dur_diff), 600)
}

slay test_timezone_operations() {
    test_start("Timezone Operations")
    
    fr fr Test UTC and local time
    sus utc_time datetime = time_utc()
    sus local_time datetime = time_local()
    
    assert_true(utc_time != cringe)
    assert_true(local_time != cringe)
    
    fr fr Test timezone conversions
    sus test_time datetime = time_create(2021, 6, 15, 12, 0, 0)
    sus utc_converted datetime = time_to_utc(test_time)
    sus local_converted datetime = time_to_local(test_time)
    
    assert_true(utc_converted != cringe)
    assert_true(local_converted != cringe)
    
    fr fr Test timezone offset
    sus offset normie = time_timezone_offset()
    assert_true(offset >= -12 * 3600 && offset <= 12 * 3600)
}

slay test_time_validation() {
    test_start("Time Validation Functions")
    
    fr fr Test leap year detection
    assert_true(time_is_leap_year(2020))
    assert_false(time_is_leap_year(2021))
    assert_true(time_is_leap_year(2000))
    assert_false(time_is_leap_year(1900))
    
    fr fr Test days in month
    assert_eq_int(time_days_in_month(2021, 1), 31)
    assert_eq_int(time_days_in_month(2021, 2), 28)
    assert_eq_int(time_days_in_month(2020, 2), 29)
    assert_eq_int(time_days_in_month(2021, 4), 30)
    
    fr fr Test date validation
    assert_true(time_is_valid_date(2021, 6, 15))
    assert_true(time_is_valid_date(2020, 2, 29))
    assert_false(time_is_valid_date(2021, 2, 29))
    assert_false(time_is_valid_date(2021, 13, 1))
    assert_false(time_is_valid_date(2021, 6, 32))
    
    fr fr Test weekend detection
    sus monday datetime = time_create(2021, 6, 14, 12, 0, 0)
    sus saturday datetime = time_create(2021, 6, 19, 12, 0, 0)
    sus sunday datetime = time_create(2021, 6, 20, 12, 0, 0)
    
    assert_false(time_is_weekend(monday))
    assert_true(time_is_weekend(saturday))
    assert_true(time_is_weekend(sunday))
}

slay test_time_constants() {
    test_start("Time Constants")
    
    fr fr Test time constants
    assert_eq_int(time_seconds_per_minute(), 60)
    assert_eq_int(time_minutes_per_hour(), 60)
    assert_eq_int(time_hours_per_day(), 24)
    assert_eq_int(time_days_per_week(), 7)
    assert_eq_int(time_months_per_year(), 12)
    assert_eq_int(time_millis_per_second(), 1000)
    assert_eq_int(time_micros_per_second(), 1000000)
    assert_eq_int(time_nanos_per_second(), 1000000000)
}

slay test_time_sleep() {
    test_start("Time Sleep Functions")
    
    fr fr Test sleep functions (short durations for testing)
    sus start_time normie = time_now_millis()
    
    time_sleep_millis(10)
    
    sus end_time normie = time_now_millis()
    sus elapsed normie = end_time - start_time
    
    fr fr Allow some variance in timing
    assert_true(elapsed >= 5 && elapsed <= 50)
    
    fr fr Test microsecond sleep
    sus start_micros normie = time_now_micros()
    time_sleep_micros(1000)
    sus end_micros normie = time_now_micros()
    sus elapsed_micros normie = end_micros - start_micros
    
    assert_true(elapsed_micros >= 500)
}

slay test_time_benchmarking() {
    test_start("Time Benchmarking Functions")
    
    fr fr Create a simple function to benchmark
    slay test_function() {
        sus sum normie = 0
        for i in range(1, 1000) {
            sum = sum + i
        }
        damn sum
    }
    
    fr fr Test benchmarking
    sus benchmark_duration duration = time_benchmark(test_function)
    assert_true(benchmark_duration != cringe)
    assert_true(duration_to_seconds(benchmark_duration) >= 0)
    
    fr fr Test measurement
    sus measurement [extra] = time_measure(test_function)
    assert_eq_int(len(measurement), 2)
    assert_eq_int(measurement[0], 499500)
    assert_true(measurement[1] != cringe)
}

slay test_time_edge_cases() {
    test_start("Time Edge Cases")
    
    fr fr Test epoch time
    sus epoch datetime = time_from_timestamp(0)
    assert_eq_int(time_year(epoch), 1970)
    assert_eq_int(time_month(epoch), 1)
    assert_eq_int(time_day(epoch), 1)
    
    fr fr Test far future time
    sus future datetime = time_create(2100, 12, 31, 23, 59, 59)
    assert_eq_int(time_year(future), 2100)
    assert_eq_int(time_month(future), 12)
    assert_eq_int(time_day(future), 31)
    
    fr fr Test February 29 on leap year
    sus leap_day datetime = time_create(2020, 2, 29, 12, 0, 0)
    assert_eq_int(time_year(leap_day), 2020)
    assert_eq_int(time_month(leap_day), 2)
    assert_eq_int(time_day(leap_day), 29)
    
    fr fr Test end of year
    sus end_of_year datetime = time_create(2021, 12, 31, 23, 59, 59)
    sus next_second datetime = time_add_seconds(end_of_year, 1)
    assert_eq_int(time_year(next_second), 2022)
    assert_eq_int(time_month(next_second), 1)
    assert_eq_int(time_day(next_second), 1)
    assert_eq_int(time_hour(next_second), 0)
    assert_eq_int(time_minute(next_second), 0)
    assert_eq_int(time_second(next_second), 0)
}

slay test_time_parsing_edge_cases() {
    test_start("Time Parsing Edge Cases")
    
    fr fr Test different date formats
    sus iso_date datetime = time_parse("2021-06-15T14:30:00", "%Y-%m-%dT%H:%M:%S")
    assert_eq_int(time_year(iso_date), 2021)
    assert_eq_int(time_month(iso_date), 6)
    assert_eq_int(time_day(iso_date), 15)
    assert_eq_int(time_hour(iso_date), 14)
    assert_eq_int(time_minute(iso_date), 30)
    assert_eq_int(time_second(iso_date), 0)
    
    fr fr Test US date format
    sus us_date datetime = time_parse("06/15/2021", "%m/%d/%Y")
    assert_eq_int(time_year(us_date), 2021)
    assert_eq_int(time_month(us_date), 6)
    assert_eq_int(time_day(us_date), 15)
    
    fr fr Test time only
    sus time_only datetime = time_parse("14:30:45", "%H:%M:%S")
    assert_eq_int(time_hour(time_only), 14)
    assert_eq_int(time_minute(time_only), 30)
    assert_eq_int(time_second(time_only), 45)
}

slay run_all_time_tests() {
    vibez.spill("⏰ Running CURSED Time Library Tests")
    vibez.spill("==================================")
    
    test_current_time()
    test_time_creation()
    test_time_components()
    test_time_formatting()
    test_time_arithmetic()
    test_time_differences()
    test_duration_operations()
    test_timezone_operations()
    test_time_validation()
    test_time_constants()
    test_time_sleep()
    test_time_benchmarking()
    test_time_edge_cases()
    test_time_parsing_edge_cases()
    
    print_test_summary()
    damn run_all_tests()
}

fr fr Auto-run tests when this file is executed
run_all_time_tests()
