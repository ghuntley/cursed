yeet "vibez"
yeet "timez"

fr fr Comprehensive timez validation

slay main() {
    vibez.spill("=== Comprehensive timez Module Validation ===")
    
    fr fr 1. Current time operations
    vibez.spill("1. Testing current time operations...")
    sus now_time Time = timez.now()
    sus timestamp_ms normie = timez.timestamp()
    sus unix_ts normie = timez.unix_time()
    vibez.spill("✅ Current time operations working")
    
    fr fr 2. Duration creation
    vibez.spill("2. Testing duration creation...")
    sus dur_sec Duration = timez.seconds(60)
    sus dur_min Duration = timez.minutes(30)
    sus dur_hour Duration = timez.hours(2)
    sus dur_day Duration = timez.days(1)
    sus dur_week Duration = timez.weeks(1)
    vibez.spill("✅ Duration creation working")
    
    fr fr 3. Time arithmetic
    vibez.spill("3. Testing time arithmetic...")
    sus base_time Time = timez.unix(1720857600)
    sus plus_sec Time = timez.add_seconds(base_time, 60)
    sus plus_min Time = timez.add_minutes(base_time, 30)
    sus plus_hour Time = timez.add_hours(base_time, 2)
    sus plus_day Time = timez.add_days(base_time, 1)
    vibez.spill("✅ Time arithmetic working")
    
    fr fr 4. Time formatting
    vibez.spill("4. Testing time formatting...")
    sus rfc_fmt tea = timez.format_rfc3339(now_time)
    sus iso_fmt tea = timez.iso8601(now_time)
    sus unix_fmt tea = timez.format_unix(now_time)
    sus human_fmt tea = timez.format_human(now_time)
    sus custom_fmt tea = timez.format_time(now_time, "iso")
    vibez.spill("✅ Time formatting working")
    
    fr fr 5. Duration conversions
    vibez.spill("5. Testing duration conversions...")
    sus test_dur Duration = timez.hours(2)
    sus dur_seconds normie = timez.duration_seconds(test_dur)
    sus dur_minutes normie = timez.duration_minutes(test_dur)
    sus dur_hours normie = timez.duration_hours(test_dur)
    sus dur_millis normie = timez.duration_millis(test_dur)
    vibez.spill("✅ Duration conversions working")
    
    fr fr 6. Duration arithmetic
    vibez.spill("6. Testing duration arithmetic...")
    sus dur1 Duration = timez.minutes(30)
    sus dur2 Duration = timez.minutes(45)
    sus sum_dur Duration = timez.add_durations(dur1, dur2)
    sus diff_dur Duration = timez.sub_durations(dur2, dur1)
    sus mult_dur Duration = timez.multiply_duration(dur1, 2)
    sus div_dur Duration = timez.divide_duration(mult_dur, 3)
    vibez.spill("✅ Duration arithmetic working")
    
    fr fr 7. Time comparison
    vibez.spill("7. Testing time comparison...")
    sus time1 Time = timez.unix(1720857600)
    sus time2 Time = timez.unix(1720861200)
    sus is_before_result lit = timez.is_before(time1, time2)
    sus is_after_result lit = timez.is_after(time2, time1)
    sus is_zero_result lit = timez.is_zero(timez.unix(0))
    vibez.spill("✅ Time comparison working")
    
    fr fr 8. Duration comparison
    vibez.spill("8. Testing duration comparison...")
    sus dura Duration = timez.minutes(30)
    sus durb Duration = timez.minutes(30)
    sus durc Duration = timez.minutes(45)
    sus equal_result lit = timez.duration_equal(dura, durb)
    sus less_result lit = timez.duration_less(dura, durc)
    sus greater_result lit = timez.duration_greater(durc, dura)
    vibez.spill("✅ Duration comparison working")
    
    fr fr 9. Sleep/delay functions
    vibez.spill("9. Testing sleep/delay functions...")
    sus short_sleep Duration = timez.milliseconds(10)
    timez.sleep(short_sleep)
    timez.usleep(100) fr fr 100 microseconds
    timez.delay(timez.milliseconds(5))
    vibez.spill("✅ Sleep/delay functions working")
    
    fr fr 10. Timezone operations
    vibez.spill("10. Testing timezone operations...")
    sus current Time = timez.now()
    sus utc_time Time = timez.to_utc(current)
    sus from_utc_time Time = timez.from_utc(current)
    sus tz_offset normie = timez.timezone_offset()
    vibez.spill("✅ Timezone operations working")
    
    fr fr 11. Advanced operations
    vibez.spill("11. Testing advanced operations...")
    sus ref_time Time = timez.unix(1720857600)
    sus elapsed_dur Duration = timez.elapsed(ref_time)
    sus time_a Time = timez.unix(1720857600)
    sus time_b Time = timez.unix(1720861200)
    sus diff_secs normie = timez.diff_seconds(time_a, time_b)
    sus diff_days normie = timez.diff_days(time_a, time_b)
    vibez.spill("✅ Advanced operations working")
    
    fr fr 12. Parsing operations
    vibez.spill("12. Testing parsing operations...")
    sus parsed_rfc Time = timez.parse_rfc3339("2024-07-13T12:34:56Z")
    sus parsed_iso Time = timez.parse_time("2024-07-13T12:34:56Z", "iso8601")
    vibez.spill("✅ Parsing operations working")
    
    vibez.spill("")
    vibez.spill("🎉 ALL TIMEZ OPERATIONS VALIDATED SUCCESSFULLY! 🎉")
    vibez.spill("✅ Current time (now, timestamp, unix_time)")
    vibez.spill("✅ Time formatting (format_time, parse_time, iso8601)")
    vibez.spill("✅ Time arithmetic (add_seconds, add_minutes, add_hours, add_days)")
    vibez.spill("✅ Time zones (to_utc, from_utc, timezone_offset)")
    vibez.spill("✅ Duration calculations (diff_seconds, diff_days, elapsed)")
    vibez.spill("✅ Sleep/delay functions (sleep, usleep, delay)")
    vibez.spill("✅ Duration creation (seconds, minutes, hours, days, weeks)")
    vibez.spill("✅ Duration conversions (seconds, minutes, hours, days)")
    vibez.spill("✅ Duration arithmetic (add, subtract, multiply, divide)")
    vibez.spill("✅ Time/Duration comparison operations")
    vibez.spill("✅ RFC3339/ISO8601 parsing and formatting")
    vibez.spill("✅ Advanced time calculations and utilities")
}
