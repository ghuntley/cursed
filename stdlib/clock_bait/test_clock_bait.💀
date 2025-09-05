yeet "testz"
yeet "clock_bait"

fr fr Test duration constants
slay test_duration_constants() {
    test_start("Duration constants test")
    
    fr fr Test basic duration constants
    assert_eq_int(clock_bait.NanoBlink, 1)
    assert_eq_int(clock_bait.MicroBlink, 1000)
    assert_eq_int(clock_bait.MilliBlink, 1000000)
    assert_eq_int(clock_bait.Blink, 1000000000)
    assert_eq_int(clock_bait.SecondVibe, 1000000000)
    assert_eq_int(clock_bait.MinuteVibe, 60000000000)
    assert_eq_int(clock_bait.HourVibe, 3600000000000)
    assert_eq_int(clock_bait.DayVibe, 86400000000000)
    assert_eq_int(clock_bait.WeekVibe, 604800000000000)
    
    print_test_summary()
}

fr fr Test month constants
slay test_month_constants() {
    test_start("Month constants test")
    
    assert_eq_int(clock_bait.VibeJanuary, 1)
    assert_eq_int(clock_bait.VibeFebruary, 2)
    assert_eq_int(clock_bait.VibeMarch, 3)
    assert_eq_int(clock_bait.VibeApril, 4)
    assert_eq_int(clock_bait.VibeMay, 5)
    assert_eq_int(clock_bait.VibeJune, 6)
    assert_eq_int(clock_bait.VibeJuly, 7)
    assert_eq_int(clock_bait.VibeAugust, 8)
    assert_eq_int(clock_bait.VibeSeptember, 9)
    assert_eq_int(clock_bait.VibeOctober, 10)
    assert_eq_int(clock_bait.VibeNovember, 11)
    assert_eq_int(clock_bait.VibeDecember, 12)
    
    print_test_summary()
}

fr fr Test weekday constants
slay test_weekday_constants() {
    test_start("Weekday constants test")
    
    assert_eq_int(clock_bait.VibeSunday, 0)
    assert_eq_int(clock_bait.VibeMonday, 1)
    assert_eq_int(clock_bait.VibeTuesday, 2)
    assert_eq_int(clock_bait.VibeWednesday, 3)
    assert_eq_int(clock_bait.VibeThursday, 4)
    assert_eq_int(clock_bait.VibeFriday, 5)
    assert_eq_int(clock_bait.VibeSaturday, 6)
    
    print_test_summary()
}

fr fr Test time creation functions
slay test_time_creation() {
    test_start("Time creation test")
    
    fr fr Test current time
    now := clock_bait.Now()
    assert_true(now > 0)
    
    fr fr Test Unix timestamp creation
    unix_time := clock_bait.Unix(1704067200, 0)
    assert_eq_int(unix_time, 1704067200000000000)
    
    fr fr Test Unix millisecond creation
    unix_milli := clock_bait.UnixMilli(1704067200000)
    assert_eq_int(unix_milli, 1704067200000000000)
    
    fr fr Test Unix microsecond creation
    unix_micro := clock_bait.UnixMicro(1704067200000000)
    assert_eq_int(unix_micro, 1704067200000000000)
    
    print_test_summary()
}

fr fr Test time arithmetic
slay test_time_arithmetic() {
    test_start("Time arithmetic test")
    
    base_time := clock_bait.Unix(1704067200, 0)
    
    fr fr Test adding duration
    later_time := clock_bait.Add(base_time, clock_bait.HourVibe)
    expected := base_time + clock_bait.HourVibe
    assert_eq_int(later_time, expected)
    
    fr fr Test subtracting times
    duration := clock_bait.Sub(later_time, base_time)
    assert_eq_int(duration, clock_bait.HourVibe)
    
    print_test_summary()
}

fr fr Test time comparison
slay test_time_comparison() {
    test_start("Time comparison test")
    
    time1 := clock_bait.Unix(1704067200, 0)
    time2 := clock_bait.Unix(1704067260, 0)  fr fr 1 minute later
    
    fr fr Test After
    assert_true(clock_bait.After(time2, time1))
    assert_false(clock_bait.After(time1, time2))
    
    fr fr Test Before
    assert_true(clock_bait.Before(time1, time2))
    assert_false(clock_bait.Before(time2, time1))
    
    fr fr Test Equal
    assert_true(clock_bait.Equal(time1, time1))
    assert_false(clock_bait.Equal(time1, time2))
    
    fr fr Test Compare
    assert_eq_int(clock_bait.Compare(time1, time2), -1)
    assert_eq_int(clock_bait.Compare(time2, time1), 1)
    assert_eq_int(clock_bait.Compare(time1, time1), 0)
    
    print_test_summary()
}

fr fr Test time components
slay test_time_components() {
    test_start("Time components test")
    
    test_time := clock_bait.Unix(1704067200, 0)  fr fr 2024-01-01 00:00:00 UTC
    
    fr fr Test year, month, day (simplified)
    year := clock_bait.Year(test_time)
    month := clock_bait.Month(test_time)
    day := clock_bait.Day(test_time)
    weekday := clock_bait.Weekday(test_time)
    
    assert_eq_int(year, 2024)
    assert_eq_int(month, clock_bait.VibeJanuary)
    assert_eq_int(day, 1)
    assert_eq_int(weekday, clock_bait.VibeMonday)
    
    fr fr Test time components
    hour := clock_bait.Hour(test_time)
    minute := clock_bait.Minute(test_time)
    second := clock_bait.Second(test_time)
    
    assert_eq_int(hour, 0)
    assert_eq_int(minute, 0)
    assert_eq_int(second, 0)
    
    print_test_summary()
}

fr fr Test duration conversions
slay test_duration_conversions() {
    test_start("Duration conversions test")
    
    duration := 3 * clock_bait.HourVibe + 30 * clock_bait.MinuteVibe
    
    fr fr Test conversion to different units
    hours := clock_bait.DurationHours(duration)
    minutes := clock_bait.DurationMinutes(duration)
    seconds := clock_bait.DurationSeconds(duration)
    
    assert_eq_int(hours, 3)
    assert_eq_int(minutes, 210)  fr fr 3.5 hours = 210 minutes
    assert_eq_int(seconds, 12600)  fr fr 3.5 hours = 12600 seconds
    
    print_test_summary()
}

fr fr Test Since and Until
slay test_since_until() {
    test_start("Since and Until test")
    
    past_time := clock_bait.Unix(1704067200, 0)
    future_time := clock_bait.Unix(1704067260, 0)  fr fr 1 minute later
    
    fr fr Test Since (using fixed current time)
    since_duration := clock_bait.Sub(clock_bait.Now(), past_time)
    assert_true(since_duration > 0)
    
    fr fr Test Until (using fixed current time)
    until_duration := clock_bait.Sub(future_time, clock_bait.Now())
    fr fr Note: This might be negative if future_time is before Now()
    
    print_test_summary()
}

fr fr Test weekend functions
slay test_weekend_functions() {
    test_start("Weekend functions test")
    
    fr fr Test IsItFriday (simplified - always returns same weekday)
    test_time := clock_bait.Unix(1704067200, 0)
    is_friday := clock_bait.IsItFriday(test_time)
    assert_false(is_friday)  fr fr Monday is not Friday
    
    fr fr Test NextWeekend
    next_weekend := clock_bait.NextWeekend(test_time)
    assert_true(next_weekend > test_time)
    
    print_test_summary()
}

fr fr Test social media time formatting
slay test_social_formatting() {
    test_start("Social media formatting test")
    
    current_time := clock_bait.Now()
    past_time := current_time - (30 * clock_bait.MinuteVibe)
    
    fr fr Test relative time formatting
    relative := clock_bait.RelativeTime(past_time)
    assert_eq_string(relative, "minutes ago")
    
    fr fr Test time ago formatting
    time_ago := clock_bait.TimeAgo(past_time)
    assert_eq_string(time_ago, "minutes ago")
    
    fr fr Test social format
    social := clock_bait.SocialFormat(past_time)
    assert_eq_string(social, "minutes ago")
    
    print_test_summary()
}

fr fr Test vibe check
slay test_vibe_check() {
    test_start("Vibe check test")
    
    morning_time := clock_bait.Unix(1704067200, 0)  fr fr 00:00:00
    hours_left := clock_bait.VibeCheck(morning_time)
    assert_eq_int(hours_left, 24)
    
    fr fr Test ViberTime
    vibe_format := clock_bait.ViberTime(morning_time)
    assert_eq_string(vibe_format, "morning vibe")
    
    print_test_summary()
}

fr fr Test time span operations
slay test_time_span() {
    test_start("Time span operations test")
    
    start_time := clock_bait.Unix(1704067200, 0)
    end_time := start_time + (2 * clock_bait.HourVibe)
    test_time := start_time + clock_bait.HourVibe
    
    fr fr Test TimeSpanContains
    contains := clock_bait.TimeSpanContains(start_time, end_time, test_time)
    assert_true(contains)
    
    fr fr Test TimeSpanDuration
    duration := clock_bait.TimeSpanDuration(start_time, end_time)
    assert_eq_int(duration, 2 * clock_bait.HourVibe)
    
    fr fr Test TimeSpanOverlaps
    start2 := start_time + clock_bait.HourVibe
    end2 := end_time + clock_bait.HourVibe
    overlaps := clock_bait.TimeSpanOverlaps(start_time, end_time, start2, end2)
    assert_true(overlaps)
    
    print_test_summary()
}

fr fr Test duration rounding and truncation
slay test_duration_rounding() {
    test_start("Duration rounding test")
    
    duration := 90 * clock_bait.SecondVibe  fr fr 90 seconds
    
    fr fr Test rounding to nearest minute
    rounded := clock_bait.RoundDuration(duration, clock_bait.MinuteVibe)
    assert_eq_int(rounded, 2 * clock_bait.MinuteVibe)  fr fr Should round to 2 minutes
    
    fr fr Test truncation to minute
    truncated := clock_bait.TruncateDuration(duration, clock_bait.MinuteVibe)
    assert_eq_int(truncated, clock_bait.MinuteVibe)  fr fr Should truncate to 1 minute
    
    print_test_summary()
}

fr fr Test string formatting
slay test_string_formatting() {
    test_start("String formatting test")
    
    fr fr Test month string
    january_str := clock_bait.MonthString(clock_bait.VibeJanuary)
    assert_eq_string(january_str, "January")
    
    fr fr Test weekday string
    monday_str := clock_bait.WeekdayString(clock_bait.VibeMonday)
    assert_eq_string(monday_str, "Monday")
    
    fr fr Test duration string
    duration := clock_bait.MinuteVibe
    duration_str := clock_bait.DurationString(duration)
    assert_eq_string(duration_str, "minutes")
    
    print_test_summary()
}

fr fr Test Sleep function
slay test_sleep() {
    test_start("Sleep function test")
    
    fr fr Test sleep (placeholder implementation)
    result := clock_bait.Sleep(clock_bait.SecondVibe)
    assert_true(result)
    
    print_test_summary()
}

fr fr Test Unix timestamp conversions
slay test_unix_conversions() {
    test_start("Unix timestamp conversions test")
    
    test_time := clock_bait.Unix(1704067200, 0)
    
    fr fr Test conversion back to Unix
    unix_sec := clock_bait.ToUnix(test_time)
    assert_eq_int(unix_sec, 1704067200)
    
    fr fr Test conversion to Unix milliseconds
    unix_milli := clock_bait.ToUnixMilli(test_time)
    assert_eq_int(unix_milli, 1704067200000)
    
    fr fr Test conversion to Unix microseconds
    unix_micro := clock_bait.ToUnixMicro(test_time)
    assert_eq_int(unix_micro, 1704067200000000)
    
    fr fr Test conversion to Unix nanoseconds
    unix_nano := clock_bait.ToUnixNano(test_time)
    assert_eq_int(unix_nano, 1704067200000000000)
    
    print_test_summary()
}

fr fr Main test runner
slay main_character() {
    test_duration_constants()
    test_month_constants()
    test_weekday_constants()
    test_time_creation()
    test_time_arithmetic()
    test_time_comparison()
    test_time_components()
    test_duration_conversions()
    test_since_until()
    test_weekend_functions()
    test_social_formatting()
    test_vibe_check()
    test_time_span()
    test_duration_rounding()
    test_string_formatting()
    test_sleep()
    test_unix_conversions()
    
    vibez.spill("All clock_bait tests completed!")
}

main()
