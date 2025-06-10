/// Integration tests for the time module
use cursed::stdlib::time::*;

#[test]
fn test_basic_datetime_creation() {
    let date = Date::new(2023, 12, 25).unwrap();
    assert_eq!(date.year, 2023);
    assert_eq!(date.month, 12);
    assert_eq!(date.day, 25);
    
    let time = Time::new(15, 30, 45, 0).unwrap();
    assert_eq!(time.hour, 15);
    assert_eq!(time.minute, 30);
    assert_eq!(time.second, 45);
    
    let datetime = DateTime::new(date, time);
    assert_eq!(datetime.date, date);
    assert_eq!(datetime.time, time);
}

#[test]
fn test_duration_operations() {
    let d1 = Duration::from_seconds(60);
    let d2 = Duration::from_seconds(30);
    
    let sum = d1.add(&d2).unwrap();
    assert_eq!(sum.total_seconds(), 90);
    
    let diff = d1.subtract(&d2).unwrap();
    assert_eq!(diff.total_seconds(), 30);
}

#[test]
fn test_duration_helpers() {
    let h = hours(2);
    assert_eq!(h.total_seconds(), 7200);
    
    let m = minutes(30);
    assert_eq!(m.total_seconds(), 1800);
    
    let d = days(1);
    assert_eq!(d.total_seconds(), 86400);
}

#[test]
fn test_iso8601_formatting() {
    let date = Date::new(2023, 12, 25).unwrap();
    let time = Time::new(15, 30, 45, 0).unwrap();
    let datetime = DateTime::new(date, time);
    
    let formatted = format_iso8601(&datetime).unwrap();
    assert_eq!(formatted, "2023-12-25T15:30:45");
}

#[test]
fn test_iso8601_parsing() {
    let parsed = parse_iso8601("2023-12-25T15:30:45").unwrap();
    assert_eq!(parsed.date.year, 2023);
    assert_eq!(parsed.date.month, 12);
    assert_eq!(parsed.date.day, 25);
    assert_eq!(parsed.time.hour, 15);
    assert_eq!(parsed.time.minute, 30);
    assert_eq!(parsed.time.second, 45);
}

#[test]
fn test_duration_parsing() {
    let d1 = parse_duration("2h 30m").unwrap();
    assert_eq!(d1.total_seconds(), 9000); // 2*3600 + 30*60
    
    let d2 = parse_duration("1d").unwrap();
    assert_eq!(d2.total_seconds(), 86400);
    
    let d3 = parse_duration("45s").unwrap();
    assert_eq!(d3.total_seconds(), 45);
}

#[test]
fn test_weekday_operations() {
    let date = Date::new(2023, 12, 25).unwrap(); // This is a Monday
    let weekday = date.weekday();
    assert_eq!(weekday.name(), "Monday");
    
    let next = weekday.next();
    assert_eq!(next.name(), "Tuesday");
    
    let prev = weekday.previous();
    assert_eq!(prev.name(), "Sunday");
}

#[test]
fn test_calendar_functions() {
    assert!(is_leap_year(2020));
    assert!(!is_leap_year(2021));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
    
    assert_eq!(days_in_month(2023, 2), 28);
    assert_eq!(days_in_month(2020, 2), 29);
    assert_eq!(days_in_month(2023, 4), 30);
    assert_eq!(days_in_month(2023, 1), 31);
}

#[test]
fn test_timezone_basics() {
    let utc_tz = utc();
    assert_eq!(utc_tz.name, "UTC");
    assert_eq!(utc_tz.offset_seconds, 0);
    assert!(utc_tz.is_utc());
    
    let est = timezone_by_name("EST").unwrap();
    assert_eq!(est.offset_seconds, -5 * 3600);
    assert_eq!(est.offset_string(), "-05:00");
}

#[test]
fn test_relative_time_parsing() {
    let rel1 = parse_relative("2 hours ago").unwrap();
    assert!(rel1.is_past());
    
    let rel2 = parse_relative("in 3 days").unwrap();
    assert!(rel2.is_future());
    
    let rel3 = parse_relative("now").unwrap();
    assert!(rel3.is_now());
}

#[test]
fn test_timer_functionality() {
    use std::thread;
    
    let timer = Timer::new();
    thread::sleep(std::time::Duration::from_millis(50));
    let elapsed = timer.elapsed();
    
    assert!(elapsed.total_milliseconds() >= 40);
    assert!(elapsed.total_milliseconds() <= 100);
}

#[test]
fn test_stopwatch_functionality() {
    use std::thread;
    
    let mut stopwatch = Stopwatch::new();
    assert!(!stopwatch.is_running());
    
    stopwatch.start();
    assert!(stopwatch.is_running());
    
    thread::sleep(std::time::Duration::from_millis(50));
    
    let lap1 = stopwatch.lap();
    assert!(lap1.total_milliseconds() >= 40);
    
    stopwatch.stop();
    assert!(!stopwatch.is_running());
    
    let final_time = stopwatch.elapsed();
    assert!(final_time.total_milliseconds() >= 40);
}

#[test]
fn test_benchmarking() {
    use std::thread;
    
    let result = benchmark("sleep_test", 3, || {)
        thread::sleep(std::time::Duration::from_millis(10));
    }).unwrap();
    
    assert_eq!(result.iterations, 3);
    assert!(result.average_time.total_milliseconds() >= 8);
    assert!(result.average_time.total_milliseconds() <= 50);
}

#[test]
fn test_performance_counter() {
    use std::thread;
    
    let mut counter = PerformanceCounter::new();
    
    counter.increment("operations");
    counter.increment("operations");
    counter.add("bytes", 1024);
    
    counter.start_timer("processing");
    thread::sleep(std::time::Duration::from_millis(10));
    counter.stop_timer("processing");
    
    assert_eq!(counter.get_counter("operations"), 2);
    assert_eq!(counter.get_counter("bytes"), 1024);
    assert!(counter.get_timer("processing").total_milliseconds() >= 8);
}

#[test]
fn test_time_conversion() {
    let timestamp = 1703519445; // 2023-12-25 15:30:45 UTC
    let datetime = from_timestamp(timestamp).unwrap();
    
    assert_eq!(datetime.date.year, 2023);
    assert_eq!(datetime.date.month, 12);
    assert_eq!(datetime.date.day, 25);
    
    let back_to_timestamp = datetime.to_timestamp();
    assert_eq!(back_to_timestamp, timestamp);
}

#[test]
fn test_error_handling() {
    // Invalid date
    let invalid_date = Date::new(2023, 13, 1);
    assert!(invalid_date.is_err());
    
    // Invalid time
    let invalid_time = Time::new(25, 0, 0, 0);
    assert!(invalid_time.is_err());
    
    // Invalid duration parsing
    let invalid_duration = parse_duration("invalid");
    assert!(invalid_duration.is_err());
    
    // Invalid timezone
    let invalid_tz = timezone_by_name("INVALID");
    assert!(invalid_tz.is_err());
}

#[test]
fn test_humanized_duration() {
    let d1 = Duration::from_seconds(0);
    assert_eq!(d1.humanize(), "0 seconds");
    
    let d2 = Duration::from_seconds(1);
    assert_eq!(d2.humanize(), "1 second");
    
    let d3 = Duration::from_seconds(60);
    assert_eq!(d3.humanize(), "1 minute");
    
    let d4 = Duration::from_seconds(3661);
    assert_eq!(d4.humanize(), "1 hour, 1 minute, and 1 second");
}
