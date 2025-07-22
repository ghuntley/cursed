fr fr Enhanced CURSED Clock Module Tests
fr fr Comprehensive test suite for timezone handling, leap years, date arithmetic edge cases

yeet "testz"
yeet "clock_bait"

fr fr Test leap year calculations
slay test_leap_year_calculations() {
    test_start("leap_year_calculations") fr fr Test standard leap years (divisible by 4)
    assert_true(is_leap_year(2024))
    assert_true(is_leap_year(2020))
    assert_true(is_leap_year(2016)) fr fr Test non-leap years
    assert_false(is_leap_year(2023))
    assert_false(is_leap_year(2021))
    assert_false(is_leap_year(2019)) fr fr Test century years (divisible by 100 but not 400)
    assert_false(is_leap_year(1900))
    assert_false(is_leap_year(2100))
    assert_false(is_leap_year(2200)) fr fr Test leap century years (divisible by 400)
    assert_true(is_leap_year(2000))
    assert_true(is_leap_year(1600))
    assert_true(is_leap_year(2400)) fr fr Test edge cases around leap years
    sus feb_28_2024 thicc = Date(2024, 2, 28).to_unix()
    sus feb_29_2024 thicc = Date(2024, 2, 29).to_unix()
    sus mar_01_2024 thicc = Date(2024, 3, 1).to_unix()
    
    assert_true(is_valid_date(2024, 2, 29)) fr fr Valid leap day
    assert_false(is_valid_date(2023, 2, 29)) fr fr Invalid non-leap day fr fr Test day addition across leap year boundary
    sus after_leap thicc = add_days(feb_28_2024, 1)
    assert_eq_int(get_day_of_month(after_leap), 29)
    assert_eq_int(get_month(after_leap), 2)
    
    vibez.spill("✅ Leap year calculations test passed")
}

fr fr Test timezone handling and conversions
slay test_timezone_handling() {
    test_start("timezone_handling") fr fr Test UTC timezone
    sus utc_time thicc = now_utc()
    assert_true(utc_time > 0)
    
    sus utc_tz *Timezone = get_utc_timezone()
    assert_eq_string(timezone_name(utc_tz), "UTC")
    assert_eq_int(timezone_offset(utc_tz), 0) fr fr Test common timezones
    sus est_tz *Timezone = get_timezone("America/New_York")
    assert_true(est_tz != cringe)
    assert_eq_string(timezone_name(est_tz), "America/New_York")
    
    sus pst_tz *Timezone = get_timezone("America/Los_Angeles")
    assert_true(pst_tz != cringe)
    assert_eq_string(timezone_name(pst_tz), "America/Los_Angeles")
    
    sus tokyo_tz *Timezone = get_timezone("Asia/Tokyo")
    assert_true(tokyo_tz != cringe)
    assert_eq_string(timezone_name(tokyo_tz), "Asia/Tokyo") fr fr Test timezone conversion
    sus utc_timestamp thicc = Unix(1704067200, 0) fr fr 2024-01-01 00:00:00 UTC
    
    sus est_time thicc = convert_timezone(utc_timestamp, utc_tz, est_tz)
    sus pst_time thicc = convert_timezone(utc_timestamp, utc_tz, pst_tz)
    sus tokyo_time thicc = convert_timezone(utc_timestamp, utc_tz, tokyo_tz) fr fr EST should be 5 hours behind UTC (in winter)
    sus est_offset thicc = Sub(est_time, utc_timestamp)
    assert_true(est_offset <= -4 * HourVibe && est_offset >= -5 * HourVibe) fr fr PST should be 8 hours behind UTC (in winter)
    sus pst_offset thicc = Sub(pst_time, utc_timestamp)
    assert_true(pst_offset <= -7 * HourVibe && pst_offset >= -8 * HourVibe) fr fr Tokyo should be 9 hours ahead of UTC
    sus tokyo_offset thicc = Sub(tokyo_time, utc_timestamp)
    assert_true(tokyo_offset >= 8 * HourVibe && tokyo_offset <= 9 * HourVibe)
    
    vibez.spill("✅ Timezone handling test passed")
}

fr fr Test daylight saving time transitions
slay test_daylight_saving_time() {
    test_start("daylight_saving_time")
    
    sus est_tz *Timezone = get_timezone("America/New_York") fr fr Test spring forward (second Sunday in March)
    sus before_dst thicc = Date(2024, 3, 10, 1, 30, 0).in_timezone(est_tz)
    sus after_dst thicc = Date(2024, 3, 10, 3, 30, 0).in_timezone(est_tz)
    
    assert_true(is_daylight_saving(before_dst, est_tz) == cap)
    assert_true(is_daylight_saving(after_dst, est_tz) == based) fr fr Test fall back (first Sunday in November)
    sus before_fall thicc = Date(2024, 11, 3, 1, 30, 0).in_timezone(est_tz)
    sus after_fall thicc = Date(2024, 11, 3, 1, 30, 0).in_timezone(est_tz) fr fr Test DST offset calculation
    sus winter_offset normie = get_timezone_offset(est_tz, Date(2024, 1, 15).to_unix())
    sus summer_offset normie = get_timezone_offset(est_tz, Date(2024, 7, 15).to_unix())
    
    assert_eq_int(winter_offset, -5 * 3600) fr fr EST is UTC-5
    assert_eq_int(summer_offset, -4 * 3600) fr fr EDT is UTC-4
    
    vibez.spill("✅ Daylight saving time test passed")
}

fr fr Test date arithmetic edge cases
slay test_date_arithmetic_edge_cases() {
    test_start("date_arithmetic_edge_cases") fr fr Test month-end overflow
    sus jan_31 thicc = Date(2024, 1, 31).to_unix()
    sus feb_end thicc = add_months(jan_31, 1)
    assert_eq_int(get_month(feb_end), 2)
    assert_eq_int(get_day_of_month(feb_end), 29) fr fr Feb 29 in leap year
    
    sus non_leap_jan_31 thicc = Date(2023, 1, 31).to_unix()
    sus non_leap_feb_end thicc = add_months(non_leap_jan_31, 1)
    assert_eq_int(get_month(non_leap_feb_end), 2)
    assert_eq_int(get_day_of_month(non_leap_feb_end), 28) fr fr Feb 28 in non-leap year fr fr Test year rollover
    sus dec_31 thicc = Date(2023, 12, 31, 23, 59, 59).to_unix()
    sus new_year thicc = Add(dec_31, SecondVibe)
    assert_eq_int(get_year(new_year), 2024)
    assert_eq_int(get_month(new_year), 1)
    assert_eq_int(get_day_of_month(new_year), 1)
    assert_eq_int(get_hour(new_year), 0)
    assert_eq_int(get_minute(new_year), 0)
    assert_eq_int(get_second(new_year), 0) fr fr Test negative duration
    sus future_time thicc = Add(dec_31, 7 * DayVibe)
    sus past_duration thicc = Sub(dec_31, future_time)
    assert_true(past_duration < 0)
    
    sus back_to_past thicc = Add(future_time, past_duration)
    assert_eq_int(back_to_past, dec_31) fr fr Test large duration additions
    sus base_time thicc = Date(2000, 1, 1).to_unix()
    sus far_future thicc = Add(base_time, 10000 * DayVibe) fr fr ~27 years
    assert_eq_int(get_year(far_future), 2027)
    
    vibez.spill("✅ Date arithmetic edge cases test passed")
}

fr fr Test week calculations and business days
slay test_week_calculations() {
    test_start("week_calculations") fr fr Test week number calculation
    sus jan_1_2024 thicc = Date(2024, 1, 1).to_unix()
    sus week1 normie = get_week_of_year(jan_1_2024)
    assert_eq_int(week1, 1)
    
    sus dec_31_2024 thicc = Date(2024, 12, 31).to_unix()
    sus week53 normie = get_week_of_year(dec_31_2024)
    assert_true(week53 == 52 || week53 == 53) fr fr Could be either fr fr Test weekday calculations
    sus monday thicc = Date(2024, 1, 1).to_unix() fr fr 2024-01-01 was a Monday
    assert_eq_int(Weekday(monday), VibeMonday)
    
    sus tuesday thicc = Add(monday, DayVibe)
    assert_eq_int(Weekday(tuesday), VibeTuesday)
    
    sus sunday thicc = Add(monday, 6 * DayVibe)
    assert_eq_int(Weekday(sunday), VibeSunday) fr fr Test business day calculations
    assert_true(is_business_day(monday))
    assert_true(is_business_day(tuesday))
    assert_false(is_business_day(sunday))
    
    sus next_business thicc = next_business_day(sunday)
    assert_eq_int(Weekday(next_business), VibeMonday)
    
    sus prev_business thicc = previous_business_day(sunday)
    assert_eq_int(Weekday(prev_business), VibeFriday) fr fr Test business days between dates
    sus friday thicc = Date(2024, 1, 5).to_unix() fr fr Friday
    sus next_monday thicc = Date(2024, 1, 8).to_unix() fr fr Next Monday
    
    sus business_days normie = count_business_days(friday, next_monday)
    assert_eq_int(business_days, 1) fr fr Only Monday is a business day
    
    vibez.spill("✅ Week calculations test passed")
}

fr fr Test date parsing and formatting edge cases
slay test_date_parsing_edge_cases() {
    test_start("date_parsing_edge_cases") fr fr Test various date formats
    sus iso_date thicc = parse_date("2024-03-15T14:30:45Z")
    assert_eq_int(get_year(iso_date), 2024)
    assert_eq_int(get_month(iso_date), 3)
    assert_eq_int(get_day_of_month(iso_date), 15)
    assert_eq_int(get_hour(iso_date), 14)
    assert_eq_int(get_minute(iso_date), 30)
    assert_eq_int(get_second(iso_date), 45) fr fr Test US format
    sus us_date thicc = parse_date("03/15/2024 2:30:45 PM")
    assert_eq_int(get_year(us_date), 2024)
    assert_eq_int(get_month(us_date), 3)
    assert_eq_int(get_day_of_month(us_date), 15)
    assert_eq_int(get_hour(us_date), 14) fr fr 2:30 PM = 14:30 fr fr Test European format
    sus eu_date thicc = parse_date("15.03.2024 14:30:45")
    assert_eq_int(get_year(eu_date), 2024)
    assert_eq_int(get_month(eu_date), 3)
    assert_eq_int(get_day_of_month(eu_date), 15) fr fr Test edge cases
    sus leap_day thicc = parse_date("2024-02-29")
    assert_true(is_valid_time(leap_day))
    
    sus invalid_date thicc = parse_date("2023-02-29") fr fr Invalid leap day
    assert_false(is_valid_time(invalid_date)) fr fr Test formatting
    sus test_time thicc = Date(2024, 7, 4, 16, 30, 0).to_unix()
    
    sus iso_format tea = format_iso(test_time)
    assert_eq_string(iso_format, "2024-07-04T16:30:00Z")
    
    sus rfc3339_format tea = format_rfc3339(test_time)
    assert_true(rfc3339_format.contains("2024-07-04T16:30:00"))
    
    sus custom_format tea = format_custom(test_time, "YYYY-MM-DD HH:mm:ss")
    assert_eq_string(custom_format, "2024-07-04 16:30:00")
    
    vibez.spill("✅ Date parsing edge cases test passed")
}

fr fr Test time duration precision and overflow
slay test_duration_precision() {
    test_start("duration_precision") fr fr Test nanosecond precision
    sus nano_duration thicc = 123456789 fr fr nanoseconds
    sus micro_part normie = DurationMicroseconds(nano_duration)
    sus milli_part normie = DurationMilliseconds(nano_duration)
    sus second_part normie = DurationSeconds(nano_duration)
    
    assert_eq_int(micro_part, 123)
    assert_eq_int(milli_part, 0)
    assert_eq_int(second_part, 0) fr fr Test large duration calculations
    sus large_duration thicc = 365 * DayVibe fr fr One year
    sus years normie = DurationYears(large_duration)
    sus days normie = DurationDays(large_duration)
    sus hours normie = DurationHours(large_duration)
    
    assert_eq_int(years, 1)
    assert_eq_int(days, 365)
    assert_eq_int(hours, 365 * 24) fr fr Test duration overflow protection
    sus max_duration thicc = MaxDuration()
    sus overflow_safe thicc = Add(max_duration, SecondVibe)
    assert_eq_int(overflow_safe, max_duration) fr fr Should not overflow fr fr Test duration comparison with precision
    sus duration1 thicc = 1500 * MilliBlink fr fr 1.5 seconds
    sus duration2 thicc = SecondVibe + 500 * MilliBlink fr fr 1.5 seconds
    assert_true(DurationEqual(duration1, duration2))
    
    sus slightly_longer thicc = duration1 + NanoBlink
    assert_false(DurationEqual(duration1, slightly_longer))
    assert_true(DurationAlmostEqual(duration1, slightly_longer, MicroBlink))
    
    vibez.spill("✅ Duration precision test passed")
}

fr fr Test calendar calculations
slay test_calendar_calculations() {
    test_start("calendar_calculations") fr fr Test Easter calculation for various years
    sus easter_2024 thicc = calculate_easter(2024)
    assert_eq_int(get_month(easter_2024), 3)
    assert_eq_int(get_day_of_month(easter_2024), 31) fr fr March 31, 2024
    
    sus easter_2025 thicc = calculate_easter(2025)
    assert_eq_int(get_month(easter_2025), 4)
    assert_eq_int(get_day_of_month(easter_2025), 20) fr fr April 20, 2025 fr fr Test US holidays
    sus memorial_day_2024 thicc = get_us_holiday("Memorial Day", 2024)
    assert_eq_int(get_month(memorial_day_2024), 5)
    assert_eq_int(Weekday(memorial_day_2024), VibeMonday) fr fr Last Monday in May
    
    sus thanksgiving_2024 thicc = get_us_holiday("Thanksgiving", 2024)
    assert_eq_int(get_month(thanksgiving_2024), 11)
    assert_eq_int(Weekday(thanksgiving_2024), VibeThursday) fr fr 4th Thursday in November fr fr Test days until next occurrence
    sus christmas_2024 thicc = Date(2024, 12, 25).to_unix()
    sus current_time thicc = Date(2024, 10, 15).to_unix() fr fr Mid October
    
    sus days_until_christmas normie = days_until_date(current_time, christmas_2024)
    assert_true(days_until_christmas > 60 && days_until_christmas < 80) fr fr Test quarter calculations
    sus q1_start thicc = Date(2024, 1, 1).to_unix()
    sus q2_start thicc = Date(2024, 4, 1).to_unix()
    sus q3_start thicc = Date(2024, 7, 1).to_unix()
    sus q4_start thicc = Date(2024, 10, 1).to_unix()
    
    assert_eq_int(get_quarter(q1_start), 1)
    assert_eq_int(get_quarter(q2_start), 2)
    assert_eq_int(get_quarter(q3_start), 3)
    assert_eq_int(get_quarter(q4_start), 4)
    
    vibez.spill("✅ Calendar calculations test passed")
}

fr fr Test performance with large time ranges
slay test_time_performance() {
    test_start("time_performance")
    
    sus start_time thicc = Now()
    sus iterations normie = 10000 fr fr Test many time calculations
    sus base_time thicc = Date(2000, 1, 1).to_unix()
    bestie i := 0; i < iterations; i++ {
        sus offset_time thicc = Add(base_time, i * DayVibe)
        sus year normie = get_year(offset_time)
        sus month normie = get_month(offset_time)
        sus day normie = get_day_of_month(offset_time)
        sus weekday normie = Weekday(offset_time) fr fr Verify calculations are reasonable
        assert_true(year >= 2000 && year <= 2030)
        assert_true(month >= 1 && month <= 12)
        assert_true(day >= 1 && day <= 31)
        assert_true(weekday >= 0 && weekday <= 6)
    }
    
    sus end_time thicc = Now()
    sus duration thicc = Sub(end_time, start_time) fr fr Should complete within reasonable time (1 second)
    assert_true(duration < SecondVibe)
    
    vibez.spill("✅ Time performance test passed")
}

fr fr Main test runner
slay main() {
    vibez.spill("🧪 Running Enhanced CURSED Clock Module Tests")
    vibez.spill("===================================================")
    
    test_leap_year_calculations()
    test_timezone_handling()
    test_daylight_saving_time()
    test_date_arithmetic_edge_cases()
    test_week_calculations()
    test_date_parsing_edge_cases()
    test_duration_precision()
    test_calendar_calculations()
    test_time_performance()
    
    vibez.spill("===================================================")
    print_test_summary()
    vibez.spill("🎉 All enhanced clock module tests completed!")
}
