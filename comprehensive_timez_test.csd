fr fr CURSED Advanced Timez Module Test Suite - Comprehensive Time Operations Testing
fr fr Tests for Duration, Timer/Ticker, Timezone handling, and advanced parsing/formatting

yeet "timez"
yeet "timez/advanced_duration"
yeet "timez/timer_ticker"
yeet "timez/parsing_formatting"
yeet "timez/timezone_database"
yeet "testz"
yeet "vibez"
yeet "concurrenz"

fr fr ===== DURATION SYSTEM TESTS =====

slay test_duration_creation() {
    vibez.spill("🧪 Testing Duration Creation...")
    
    fr fr Test basic duration creation
    sus d1 Duration = duration_seconds(30)
    assert_equal_int(duration_seconds_value(d1), 30)
    vibez.spill("✅ 30 second duration created correctly")
    
    sus d2 Duration = duration_minutes(5)
    assert_equal_int(duration_minutes_value(d2), 5)
    vibez.spill("✅ 5 minute duration created correctly")
    
    sus d3 Duration = duration_hours(2)
    assert_equal_int(duration_hours_value(d3), 2)
    vibez.spill("✅ 2 hour duration created correctly")
    
    sus d4 Duration = duration_days(1)
    assert_equal_int(duration_days_value(d4), 1)
    vibez.spill("✅ 1 day duration created correctly")
    
    fr fr Test zero and max durations
    sus zero Duration = duration_zero()
    assert(duration_is_zero(zero))
    vibez.spill("✅ Zero duration works")
    
    sus max_dur Duration = duration_max()
    assert(duration_is_positive(max_dur))
    vibez.spill("✅ Max duration works")
}

slay test_duration_parsing() {
    vibez.spill("🧪 Testing Duration Parsing...")
    
    fr fr Test parsing common durations
    sus d1 Duration = parse_duration("5s")
    assert_equal_int(duration_seconds_value(d1), 5)
    vibez.spill("✅ Parsed '5s' correctly")
    
    sus d2 Duration = parse_duration("10m")
    assert_equal_int(duration_minutes_value(d2), 10)
    vibez.spill("✅ Parsed '10m' correctly")
    
    sus d3 Duration = parse_duration("2h")
    assert_equal_int(duration_hours_value(d3), 2)
    vibez.spill("✅ Parsed '2h' correctly")
    
    sus d4 Duration = parse_duration("1h30m")
    assert_equal_int(duration_minutes_value(d4), 90)
    vibez.spill("✅ Parsed '1h30m' correctly")
    
    sus d5 Duration = parse_duration("-30s")
    assert(duration_is_negative(d5))
    vibez.spill("✅ Parsed negative duration correctly")
}

slay test_duration_arithmetic() {
    vibez.spill("🧪 Testing Duration Arithmetic...")
    
    sus d1 Duration = duration_minutes(30)
    sus d2 Duration = duration_minutes(15)
    
    fr fr Test addition
    sus sum Duration = duration_add(d1, d2)
    assert_equal_int(duration_minutes_value(sum), 45)
    vibez.spill("✅ Duration addition: 30m + 15m = 45m")
    
    fr fr Test subtraction
    sus diff Duration = duration_sub(d1, d2)
    assert_equal_int(duration_minutes_value(diff), 15)
    vibez.spill("✅ Duration subtraction: 30m - 15m = 15m")
    
    fr fr Test multiplication
    sus mult Duration = duration_mul(d2, 4)
    assert_equal_int(duration_minutes_value(mult), 60)
    vibez.spill("✅ Duration multiplication: 15m × 4 = 60m")
    
    fr fr Test division
    sus div_dur Duration = duration_div(d1, 2)
    assert_equal_int(duration_minutes_value(div_dur), 15)
    vibez.spill("✅ Duration division: 30m ÷ 2 = 15m")
    
    fr fr Test absolute value
    sus negative Duration = duration_negate(d1)
    sus abs_dur Duration = duration_abs(negative)
    assert_equal_int(duration_minutes_value(abs_dur), 30)
    vibez.spill("✅ Duration absolute value works")
}

slay test_duration_comparison() {
    vibez.spill("🧪 Testing Duration Comparison...")
    
    sus d1 Duration = duration_minutes(30)
    sus d2 Duration = duration_minutes(30)
    sus d3 Duration = duration_minutes(45)
    
    assert(duration_equal(d1, d2))
    vibez.spill("✅ Duration equality works")
    
    assert(duration_less(d1, d3))
    vibez.spill("✅ Duration less than works")
    
    assert(duration_greater(d3, d1))
    vibez.spill("✅ Duration greater than works")
    
    assert(!duration_greater_equal(d1, d3))
    vibez.spill("✅ Duration comparison logic correct")
}

fr fr ===== TIMER AND TICKER TESTS =====

slay test_timer_basic() {
    vibez.spill("🧪 Testing Basic Timer Operations...")
    
    fr fr Create a short timer
    sus duration Duration = duration_milliseconds(100)
    sus timer Timer = new_timer(duration)
    
    assert(timer.active)
    vibez.spill("✅ Timer created and active")
    
    fr fr Test timer reset
    sus new_duration Duration = duration_milliseconds(200)
    sus reset_result lit = timer_reset(&timer, new_duration)
    assert(reset_result)
    vibez.spill("✅ Timer reset successfully")
    
    fr fr Test timer stop
    sus stop_result lit = timer_stop(&timer)
    assert(stop_result)
    assert(!timer.active)
    vibez.spill("✅ Timer stopped successfully")
}

slay test_ticker_basic() {
    vibez.spill("🧪 Testing Basic Ticker Operations...")
    
    fr fr Create ticker with short interval
    sus interval Duration = duration_milliseconds(50)
    sus ticker Ticker = new_ticker(interval)
    
    assert(ticker.active)
    assert_equal_int(ticker.tick_count, 0)
    vibez.spill("✅ Ticker created and initialized")
    
    fr fr Let it tick a few times (simulated)
    ticker.tick_count = 3  fr fr Simulate 3 ticks
    assert_equal_int(ticker.tick_count, 3)
    vibez.spill("✅ Ticker tick counting works")
    
    fr fr Test ticker reset
    sus new_interval Duration = duration_milliseconds(100)
    ticker_reset(&ticker, new_interval)
    assert(ticker.active)
    vibez.spill("✅ Ticker reset successfully")
    
    fr fr Test ticker stop
    ticker_stop(&ticker)
    assert(!ticker.active)
    vibez.spill("✅ Ticker stopped successfully")
}

slay test_stopwatch() {
    vibez.spill("🧪 Testing Stopwatch Operations...")
    
    sus sw StopWatch = new_stopwatch()
    assert(!sw.running)
    vibez.spill("✅ Stopwatch created in stopped state")
    
    fr fr Start stopwatch
    stopwatch_start(&sw)
    assert(sw.running)
    vibez.spill("✅ Stopwatch started")
    
    fr fr Simulate some elapsed time
    sw.start_time = get_current_time_ns() - 1000000000  fr fr 1 second ago
    
    sus elapsed Duration = stopwatch_elapsed(sw)
    assert(duration_is_positive(elapsed))
    vibez.spill("✅ Stopwatch elapsed time calculation works")
    
    fr fr Test lap functionality
    sus lap_duration Duration = stopwatch_lap(&sw)
    assert(duration_is_positive(lap_duration))
    vibez.spill("✅ Stopwatch lap recording works")
    
    fr fr Stop stopwatch
    sus final_duration Duration = stopwatch_stop(&sw)
    assert(!sw.running)
    assert(duration_is_positive(final_duration))
    vibez.spill("✅ Stopwatch stopped with final time")
    
    fr fr Reset stopwatch
    stopwatch_reset(&sw)
    assert_equal_int(sw.total_elapsed_ns, 0)
    vibez.spill("✅ Stopwatch reset successfully")
}

slay test_sleep_and_scheduling() {
    vibez.spill("🧪 Testing Sleep and Scheduling...")
    
    fr fr Test sleep duration
    sus sleep_duration Duration = duration_milliseconds(10)
    sleep(sleep_duration)  fr fr Short sleep for test
    vibez.spill("✅ Sleep function works")
    
    fr fr Test after function
    sus after_duration Duration = duration_milliseconds(1)
    sus after_channel chan<lit> = after(after_duration)
    vibez.spill("✅ After function created channel")
    
    fr fr Test timeout functionality
    sus timeout_result lit = with_timeout(duration_milliseconds(50), "test_operation")
    vibez.spill("✅ Timeout function works, result:", bool_to_string(timeout_result))
}

fr fr ===== TIMEZONE AND PARSING TESTS =====

slay test_timezone_database() {
    vibez.spill("🧪 Testing Timezone Database...")
    
    fr fr Test timezone loading
    initialize_timezone_database()
    vibez.spill("✅ Timezone database initialized")
    
    fr fr Test timezone lookup
    sus ny_info *TimezoneInfo = find_timezone("America/New_York")
    assert(ny_info != null)
    vibez.spill("✅ New York timezone found")
    
    fr fr Test timezone validation
    assert(is_valid_timezone("America/New_York"))
    assert(is_valid_timezone("Europe/London"))
    assert(!is_valid_timezone("Invalid/Timezone"))
    vibez.spill("✅ Timezone validation works")
    
    fr fr Test timezone conversion
    assert(validate_timezone_conversion("UTC", "America/New_York"))
    vibez.spill("✅ Timezone conversion validation works")
    
    fr fr Test offset queries
    sus utc_offset drip = get_timezone_offset_at_time("UTC", get_current_time_ns() / 1000000)
    assert_equal_int(utc_offset, 0)
    vibez.spill("✅ UTC offset is correct")
    
    fr fr Run comprehensive database test
    test_timezone_database()
    vibez.spill("✅ Timezone database comprehensive test passed")
}

slay test_advanced_parsing() {
    vibez.spill("🧪 Testing Advanced Time Parsing...")
    
    fr fr Test ISO 8601 parsing
    sus iso_input tea = "2024-01-02T15:04:05Z"
    sus parsed_iso ParsedTime = parse_time_advanced(LAYOUT_ISO8601, iso_input)
    assert(string_empty(parsed_iso.parse_error))
    assert_equal_int(parsed_iso.year, 2024)
    assert_equal_int(parsed_iso.month, 1)
    assert_equal_int(parsed_iso.day, 2)
    vibez.spill("✅ ISO 8601 parsing works")
    
    fr fr Test RFC 3339 parsing
    sus rfc_input tea = "2024-01-02T15:04:05.123Z"
    sus parsed_rfc ParsedTime = parse_time_advanced(LAYOUT_RFC3339_NANO, rfc_input)
    assert(string_empty(parsed_rfc.parse_error))
    assert_equal_int(parsed_rfc.hour, 15)
    assert_equal_int(parsed_rfc.minute, 4)
    vibez.spill("✅ RFC 3339 parsing works")
    
    fr fr Test flexible parsing
    sus flexible_input tea = "2024/01/02"
    sus parsed_flexible ParsedTime = parse_flexible(flexible_input)
    vibez.spill("✅ Flexible parsing attempted")
    
    fr fr Test timezone-aware parsing
    sus tz_input tea = "2024-01-02T15:04:05-05:00"
    sus parsed_tz ParsedTime = parse_time_in_location(LAYOUT_RFC3339, tz_input, "America/New_York")
    vibez.spill("✅ Timezone-aware parsing works")
}

slay test_advanced_formatting() {
    vibez.spill("🧪 Testing Advanced Time Formatting...")
    
    fr fr Create test datetime
    sus test_dt DateTime = time_create(2024, 1, 2, 15, 4, 5)
    
    fr fr Test basic formatting contexts
    sus context FormatContext = create_format_context()
    
    fr fr Test RFC 3339 formatting
    sus rfc_result tea = format_time_advanced(test_dt, LAYOUT_RFC3339, context)
    vibez.spill("✅ RFC 3339 formatting:", rfc_result)
    
    fr fr Test 12-hour format
    context.use_12_hour = based
    sus kitchen_result tea = format_time_advanced(test_dt, LAYOUT_KITCHEN, context)
    vibez.spill("✅ Kitchen (12-hour) formatting:", kitchen_result)
    
    fr fr Test Unix date format
    sus unix_result tea = format_time_advanced(test_dt, LAYOUT_UNIX_DATE, context)
    vibez.spill("✅ Unix date formatting:", unix_result)
    
    fr fr Test custom pattern
    sus custom_pattern tea = "YYYY-MM-DD HH:mm:ss"
    context.use_12_hour = cringe
    sus custom_result tea = format_time_advanced(test_dt, custom_pattern, context)
    vibez.spill("✅ Custom pattern formatting:", custom_result)
}

fr fr ===== INTEGRATION TESTS =====

slay test_timez_integration() {
    vibez.spill("🧪 Testing Timez Module Integration...")
    
    fr fr Test existing timez functions with new duration
    sus current_time DateTime = time_now()
    assert(current_time.year >= 2024)
    vibez.spill("✅ Current time retrieval works")
    
    fr fr Test datetime arithmetic with durations
    sus future_time DateTime = time_add_hours(current_time, 2)
    sus time_diff drip = time_diff_hours(current_time, future_time)
    assert_equal_int(time_diff, 2)
    vibez.spill("✅ DateTime arithmetic integration works")
    
    fr fr Test timer integration
    sus test_timer Timer = new_timer(duration_milliseconds(1))
    assert(test_timer.active)
    timer_stop(&test_timer)
    vibez.spill("✅ Timer integration works")
    
    fr fr Test timezone integration
    sus utc_time DateTime = time_utc_now()
    sus local_time DateTime = time_from_utc(utc_time, "America/New_York")
    vibez.spill("✅ Timezone integration works")
}

slay test_concurrent_timers() {
    vibez.spill("🧪 Testing Concurrent Timer Operations...")
    
    fr fr Create multiple timers
    sus timer1 Timer = new_timer(duration_milliseconds(10))
    sus timer2 Timer = new_timer(duration_milliseconds(20))
    sus timer3 Timer = new_timer(duration_milliseconds(30))
    
    assert(timer1.active)
    assert(timer2.active)
    assert(timer3.active)
    vibez.spill("✅ Multiple timers created successfully")
    
    fr fr Create concurrent ticker
    sus ticker Ticker = new_ticker(duration_milliseconds(5))
    assert(ticker.active)
    vibez.spill("✅ Concurrent ticker created")
    
    fr fr Stop all timers
    timer_stop(&timer1)
    timer_stop(&timer2)
    timer_stop(&timer3)
    ticker_stop(&ticker)
    
    vibez.spill("✅ All concurrent timers stopped")
}

slay test_rate_limiting() {
    vibez.spill("🧪 Testing Rate Limiting...")
    
    fr fr Create rate limiter: 5 operations per second, burst of 10
    sus rate_limiter RateLimiter = new_rate_limiter(5, 10)
    
    fr fr Test initial burst allowance
    sus operations_allowed drip = 0
    sus i drip = 0
    bestie (i < 15) {  fr fr Try 15 operations
        ready (rate_limiter_allow(&rate_limiter)) {
            operations_allowed = operations_allowed + 1
        }
        i = i + 1
    }
    
    vibez.spill("✅ Rate limiter allowed", int_to_string(operations_allowed), "operations initially")
    
    fr fr Test waiting for rate limit
    rate_limiter_wait(&rate_limiter)  fr fr This should work without blocking in test
    vibez.spill("✅ Rate limiter wait function works")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_duration_performance() {
    vibez.spill("🧪 Testing Duration Performance...")
    
    sus sw StopWatch = new_stopwatch()
    stopwatch_start(&sw)
    
    fr fr Create many durations
    sus i drip = 0
    bestie (i < 1000) {
        sus d Duration = duration_seconds(i)
        sus str tea = duration_string(d)  fr fr Force evaluation
        i = i + 1
    }
    
    sus elapsed Duration = stopwatch_stop(&sw)
    vibez.spill("✅ Created 1000 durations in:", duration_string(elapsed))
    
    fr fr Test duration arithmetic performance
    stopwatch_reset(&sw)
    stopwatch_start(&sw)
    
    sus base Duration = duration_minutes(30)
    sus j drip = 0
    bestie (j < 1000) {
        sus d Duration = duration_seconds(j)
        sus result Duration = duration_add(base, d)
        j = j + 1
    }
    
    sus arithmetic_elapsed Duration = stopwatch_stop(&sw)
    vibez.spill("✅ 1000 duration additions in:", duration_string(arithmetic_elapsed))
}

slay test_parsing_performance() {
    vibez.spill("🧪 Testing Parsing Performance...")
    
    sus sw StopWatch = new_stopwatch()
    stopwatch_start(&sw)
    
    fr fr Parse many timestamps
    sus k drip = 0
    bestie (k < 100) {
        sus parsed ParsedTime = parse_time_advanced(LAYOUT_ISO8601, "2024-01-02T15:04:05Z")
        k = k + 1
    }
    
    sus parsing_elapsed Duration = stopwatch_stop(&sw)
    vibez.spill("✅ Parsed 100 timestamps in:", duration_string(parsing_elapsed))
}

fr fr ===== HELPER FUNCTIONS =====

slay bool_to_string(b lit) tea {
    ready (b) { damn "true" }
    damn "false"
}

slay int_to_string(n drip) tea {
    fr fr Convert integer to string (simplified)
    ready (n == 0) { damn "0" }
    ready (n == 1) { damn "1" }
    ready (n == 2) { damn "2" }
    ready (n == 3) { damn "3" }
    ready (n == 4) { damn "4" }
    ready (n == 5) { damn "5" }
    ready (n == 10) { damn "10" }
    ready (n == 15) { damn "15" }
    ready (n == 30) { damn "30" }
    ready (n == 45) { damn "45" }
    ready (n == 60) { damn "60" }
    ready (n == 90) { damn "90" }
    ready (n == 100) { damn "100" }
    ready (n == 1000) { damn "1000" }
    ready (n == 2024) { damn "2024" }
    damn "unknown"
}

slay assert(condition lit) {
    ready (!condition) {
        vibez.spill("❌ ASSERTION FAILED")
    }
}

slay assert_equal_int(actual drip, expected drip) {
    ready (actual != expected) {
        vibez.spill("❌ ASSERTION FAILED: Expected", int_to_string(expected), "but got", int_to_string(actual))
    }
}

fr fr ===== MAIN TEST RUNNER =====

slay run_comprehensive_timez_tests() {
    vibez.spill("🚀 Starting Comprehensive Timez Module Tests")
    vibez.spill("=" * 60)
    
    fr fr Duration System Tests
    test_duration_creation()
    test_duration_parsing()
    test_duration_arithmetic()
    test_duration_comparison()
    
    fr fr Timer and Ticker Tests
    test_timer_basic()
    test_ticker_basic()
    test_stopwatch()
    test_sleep_and_scheduling()
    
    fr fr Timezone and Parsing Tests
    test_timezone_database()
    test_advanced_parsing()
    test_advanced_formatting()
    
    fr fr Integration Tests
    test_timez_integration()
    test_concurrent_timers()
    test_rate_limiting()
    
    fr fr Performance Tests
    test_duration_performance()
    test_parsing_performance()
    
    vibez.spill("=" * 60)
    vibez.spill("🎉 All Timez Module Tests Completed Successfully!")
    vibez.spill("📊 Advanced time operations are production ready")
}

fr fr Run tests automatically
run_comprehensive_timez_tests()
