// Standard time and date library

// ================================
// Current time functions
// ================================

fn time_now() -> int {
    return time_now_impl();
}

fn time_now_millis() -> int {
    return time_now_millis_impl();
}

fn time_now_micros() -> int {
    return time_now_micros_impl();
}

fn time_now_nanos() -> int {
    return time_now_nanos_impl();
}

// ================================
// Date/time creation
// ================================

fn time_from_timestamp(timestamp: int) -> datetime {
    return time_from_timestamp_impl(timestamp);
}

fn time_from_millis(millis: int) -> datetime {
    return time_from_millis_impl(millis);
}

fn time_create(year: int, month: int, day: int, hour: int, minute: int, second: int) -> datetime {
    return time_create_impl(year, month, day, hour, minute, second);
}

fn time_parse(date_string: string, format: string) -> datetime {
    return time_parse_impl(date_string, format);
}

// ================================
// Date/time formatting
// ================================

fn time_format(dt: datetime, format: string) -> string {
    return time_format_impl(dt, format);
}

fn time_to_string(dt: datetime) -> string {
    return time_to_string_impl(dt);
}

fn time_to_iso8601(dt: datetime) -> string {
    return time_to_iso8601_impl(dt);
}

fn time_to_rfc3339(dt: datetime) -> string {
    return time_to_rfc3339_impl(dt);
}

// ================================
// Date/time components
// ================================

fn time_year(dt: datetime) -> int {
    return time_year_impl(dt);
}

fn time_month(dt: datetime) -> int {
    return time_month_impl(dt);
}

fn time_day(dt: datetime) -> int {
    return time_day_impl(dt);
}

fn time_hour(dt: datetime) -> int {
    return time_hour_impl(dt);
}

fn time_minute(dt: datetime) -> int {
    return time_minute_impl(dt);
}

fn time_second(dt: datetime) -> int {
    return time_second_impl(dt);
}

fn time_weekday(dt: datetime) -> int {
    return time_weekday_impl(dt);
}

fn time_day_of_year(dt: datetime) -> int {
    return time_day_of_year_impl(dt);
}

// ================================
// Date/time arithmetic
// ================================

fn time_add_years(dt: datetime, years: int) -> datetime {
    return time_add_years_impl(dt, years);
}

fn time_add_months(dt: datetime, months: int) -> datetime {
    return time_add_months_impl(dt, months);
}

fn time_add_days(dt: datetime, days: int) -> datetime {
    return time_add_days_impl(dt, days);
}

fn time_add_hours(dt: datetime, hours: int) -> datetime {
    return time_add_hours_impl(dt, hours);
}

fn time_add_minutes(dt: datetime, minutes: int) -> datetime {
    return time_add_minutes_impl(dt, minutes);
}

fn time_add_seconds(dt: datetime, seconds: int) -> datetime {
    return time_add_seconds_impl(dt, seconds);
}

fn time_subtract(dt1: datetime, dt2: datetime) -> duration {
    return time_subtract_impl(dt1, dt2);
}

fn time_diff_days(dt1: datetime, dt2: datetime) -> int {
    return time_diff_days_impl(dt1, dt2);
}

fn time_diff_hours(dt1: datetime, dt2: datetime) -> int {
    return time_diff_hours_impl(dt1, dt2);
}

fn time_diff_minutes(dt1: datetime, dt2: datetime) -> int {
    return time_diff_minutes_impl(dt1, dt2);
}

fn time_diff_seconds(dt1: datetime, dt2: datetime) -> int {
    return time_diff_seconds_impl(dt1, dt2);
}

// ================================
// Duration operations
// ================================

fn duration_from_seconds(seconds: int) -> duration {
    return duration_from_seconds_impl(seconds);
}

fn duration_from_millis(millis: int) -> duration {
    return duration_from_millis_impl(millis);
}

fn duration_to_seconds(dur: duration) -> int {
    return duration_to_seconds_impl(dur);
}

fn duration_to_millis(dur: duration) -> int {
    return duration_to_millis_impl(dur);
}

fn duration_add(dur1: duration, dur2: duration) -> duration {
    return duration_add_impl(dur1, dur2);
}

fn duration_subtract(dur1: duration, dur2: duration) -> duration {
    return duration_subtract_impl(dur1, dur2);
}

// ================================
// Time zone operations
// ================================

fn time_utc() -> datetime {
    return time_utc_impl();
}

fn time_local() -> datetime {
    return time_local_impl();
}

fn time_to_utc(dt: datetime) -> datetime {
    return time_to_utc_impl(dt);
}

fn time_to_local(dt: datetime) -> datetime {
    return time_to_local_impl(dt);
}

fn time_timezone_offset() -> int {
    return time_timezone_offset_impl();
}

// ================================
// Validation and utilities
// ================================

fn time_is_leap_year(year: int) -> bool {
    return time_is_leap_year_impl(year);
}

fn time_days_in_month(year: int, month: int) -> int {
    return time_days_in_month_impl(year, month);
}

fn time_is_valid_date(year: int, month: int, day: int) -> bool {
    return time_is_valid_date_impl(year, month, day);
}

fn time_is_weekend(dt: datetime) -> bool {
    let weekday = time_weekday(dt);
    return weekday == 0 || weekday == 6;  // Sunday = 0, Saturday = 6
}

// ================================
// Sleep and timing
// ================================

fn time_sleep(seconds: int) -> void {
    time_sleep_impl(seconds);
}

fn time_sleep_millis(millis: int) -> void {
    time_sleep_millis_impl(millis);
}

fn time_sleep_micros(micros: int) -> void {
    time_sleep_micros_impl(micros);
}

// ================================
// Benchmarking and profiling
// ================================

fn time_benchmark(func: fn) -> duration {
    let start = time_now_nanos();
    func();
    let end = time_now_nanos();
    return duration_from_nanos(end - start);
}

fn time_measure(func: fn) -> array {
    let start = time_now_nanos();
    let result = func();
    let end = time_now_nanos();
    let duration = duration_from_nanos(end - start);
    return [result, duration];
}

// ================================
// Constants and utilities
// ================================

fn time_seconds_per_minute() -> int {
    return 60;
}

fn time_minutes_per_hour() -> int {
    return 60;
}

fn time_hours_per_day() -> int {
    return 24;
}

fn time_days_per_week() -> int {
    return 7;
}

fn time_months_per_year() -> int {
    return 12;
}

fn time_millis_per_second() -> int {
    return 1000;
}

fn time_micros_per_second() -> int {
    return 1000000;
}

fn time_nanos_per_second() -> int {
    return 1000000000;
}
