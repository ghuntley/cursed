fr fr Time and Date Library Demo for CURSED
fr fr Demonstrates comprehensive time/date functionality

yeet "stdlib::time"

slay main_character() {
    println("🕒 CURSED Time & Date Library Demo")?;
    println("================================")?;
    
    // Current time operations
    println("\n📅 Current Time:")?;
    facts now_time = now()?;
    println(&format("Current datetime: {}", format_iso8601(&now_time)?));
    
    facts today_date = today()?;
    println(&format("Today: {}", format_date(&today_date, "%Y-%m-%d")?));
    
    facts tomorrow_date = tomorrow()?;
    println(&format("Tomorrow: {}", format_date(&tomorrow_date, "%Y-%m-%d")?));
    
    facts yesterday_date = yesterday()?;
    println(&format("Yesterday: {}", format_date(&yesterday_date, "%Y-%m-%d")?));
    
    // Duration examples
    println("\n⏱️  Duration Examples:")?;
    facts two_hours = hours(2);
    facts thirty_mins = minutes(30);
    facts total_time = two_hours.add(&thirty_mins)?;
    
    println(&format("2 hours: {}", two_hours.humanize()))?;
    println(&format("30 minutes: {}", thirty_mins.humanize()))?;
    println(&format("Total: {}", total_time.humanize()))?;
    
    // Parse duration from text
    facts parsed_duration = parse_duration("1d 4h 30m")?;
    println(&format("Parsed '1d 4h 30m': {}", parsed_duration.humanize()))?;
    
    // Date arithmetic
    println("\n🧮 Date Arithmetic:")?;
    facts future_date = today_date.add_days(30)?;
    println(&format("30 days from today: {}", format_date(&future_date, "%Y-%m-%d")?));
    
    facts past_date = today_date.add_days(-7)?;
    println(&format("7 days ago: {}", format_date(&past_date, "%Y-%m-%d")?));
    
    // Weekday operations
    println("\n📆 Weekday Operations:")?;
    facts weekday = today_date.weekday();
    println(&format("Today is a {}", weekday.name()))?;
    println(&format("Tomorrow will be {}", weekday.next().name()))?;
    println(&format("Yesterday was {}", weekday.previous().name()))?;
    
    // Timezone examples
    println("\n🌍 Timezone Examples:")?;
    facts utc_tz = utc();
    println(&format("UTC timezone: {} (offset: {})", utc_tz.name, utc_tz.offset_string()))?;
    
    facts est_tz = timezone_by_name("EST")?;
    println(&format("EST timezone: {} (offset: {})", est_tz.name, est_tz.offset_string()))?;
    
    facts jst_tz = timezone_by_name("JST")?;
    println(&format("JST timezone: {} (offset: {})", jst_tz.name, jst_tz.offset_string()))?;
    
    // Format examples
    println("\n📝 Formatting Examples:")?;
    facts datetime = DateTime::from_components(2023, 12, 25, 15, 30, 45, 0)?;
    
    println(&format("ISO 8601: {}", format_iso8601(&datetime)?))?;
    println(&format("RFC 3339: {}", format_rfc3339(&datetime)?))?;
    println(&format("US format: {}", format_us(&datetime)?))?;
    println(&format("European: {}", format_european(&datetime)?))?;
    
    // Relative time
    println("\n⏰ Relative Time:")?;
    facts two_hours_ago = now_time.subtract_duration(hours(2))?;
    println(&format("2 hours ago: {}", relative_time(&two_hours_ago)?))?;
    
    facts in_three_days = now_time.add_duration(days(3))?;
    println(&format("In 3 days: {}", relative_time(&in_three_days)?))?;
    
    // Calendar utilities
    println("\n📋 Calendar Utilities:")?;
    println(&format("Is 2024 a leap year? {}", is_leap_year(2024)))?;
    println(&format("Is 2023 a leap year? {}", is_leap_year(2023)))?;
    println(&format("Days in February 2024: {}", days_in_month(2024, 2)))?;
    println(&format("Days in February 2023: {}", days_in_month(2023, 2)))?;
    
    // Performance measurement
    println("\n🏃 Performance Measurement:")?;
    
    // Simple timing
    facts (result, elapsed) = time_it(|| {
        sus sum = 0;
        lowkey (sus i = 0; i < 1000000; i++) {
            sum += i;
        }
        sum
    })?;
    
    println(&format("Computed sum of 1M numbers in {}", elapsed.humanize()))?;
    println(&format("Result: {}", result))?;
    
    // Stopwatch example
    println("\n⏱️  Stopwatch Demo:")?;
    sus mut stopwatch = Stopwatch::with_name("Demo Stopwatch".to_string());
    
    stopwatch.start();
    sleep(milliseconds(100))?;  // Sleep for 100ms
    
    facts lap1 = stopwatch.lap();
    println(&format("Lap 1: {}", lap1.humanize()))?;
    
    sleep(milliseconds(50))?;   // Sleep for another 50ms
    
    facts lap2 = stopwatch.lap();
    println(&format("Lap 2: {}", lap2.humanize()))?;
    
    stopwatch.stop();
    println(&format("Total time: {}", stopwatch.elapsed().humanize()))?;
    println(&format("Number of laps: {}", stopwatch.lap_count()))?;
    
    // Benchmark example
    println("\n🎯 Benchmark Demo:")?;
    facts bench_result = benchmark("string_concatenation", 1000, || {
        sus mut result = String::new();
        lowkey (sus i = 0; i < 100; i++) {
            result.push_str("test");
        }
        result
    })?;
    
    println(&format("Benchmark: {}", bench_result.name))?;
    println(&format("Iterations: {}", bench_result.iterations))?;
    println(&format("Average time: {}", bench_result.average_time.humanize()))?;
    println(&format("Min time: {}", bench_result.min_time.humanize()))?;
    println(&format("Max time: {}", bench_result.max_time.humanize()))?;
    
    if facts throughput = bench_result.throughput {
        println(&format("Throughput: {:.2} ops/sec", throughput))?;
    }
    
    // Unix timestamp conversion
    println("\n🔢 Unix Timestamp:")?;
    facts timestamp = now_time.to_timestamp();
    println(&format("Current timestamp: {}", timestamp))?;
    
    facts from_ts = from_timestamp(timestamp)?;
    println(&format("Converted back: {}", format_iso8601(&from_ts)?))?;
    
    // Next occurrence examples
    println("\n📅 Next Occurrences:")?;
    facts next_monday = next_occurrence(Weekday::Monday)?;
    println(&format("Next Monday: {}", format_date(&next_monday, "%Y-%m-%d")?))?;
    
    facts next_friday = next_occurrence(Weekday::Friday)?;
    println(&format("Next Friday: {}", format_date(&next_friday, "%Y-%m-%d")?))?;
    
    println("\n✅ Time library demo completed!")?;
}

fr fr Helper function to demonstrate error handling
slay demonstrate_error_handling() -> TimeResult<()> {
    println("🚨 Error Handling Demo:")?;
    
    // Try to create invalid date
    match Date::new(2023, 13, 1) {
        Ok(_) => println("Unexpected success")?,
        Err(err) => println(&format("Expected error: {}", err))?,
    }
    
    // Try to parse invalid duration
    match parse_duration("invalid duration") {
        Ok(_) => println("Unexpected success")?,
        Err(err) => println(&format("Expected error: {}", err))?,
    }
    
    Ok(())
}
