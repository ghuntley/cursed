// Standard time and date library

// ================================
// Current time functions
// ================================

slay time_now() normie {
    damn time_now_impl();
}

slay time_now_millis() normie {
    damn time_now_millis_impl();
}

slay time_now_micros() normie {
    damn time_now_micros_impl();
}

slay time_now_nanos() normie {
    damn time_now_nanos_impl();
}

// ================================
// Date/time creation
// ================================

slay time_from_timestamp(timestamp normie) datetime {
    damn time_from_timestamp_impl(timestamp);
}

slay time_from_millis(millis normie) datetime {
    damn time_from_millis_impl(millis);
}

slay time_create(year normie, month normie, day normie, hour normie, minute normie, second normie) datetime {
    damn time_create_impl(year, month, day, hour, minute, second);
}

slay time_parse(date_string tea, format tea) datetime {
    damn time_parse_impl(date_string, format);
}

// ================================
// Date/time formatting
// ================================

slay time_format(dt datetime, format tea) tea {
    damn time_format_impl(dt, format);
}

slay time_to_string(dt datetime) tea {
    damn time_to_string_impl(dt);
}

slay time_to_iso8601(dt datetime) tea {
    damn time_to_iso8601_impl(dt);
}

slay time_to_rfc3339(dt datetime) tea {
    damn time_to_rfc3339_impl(dt);
}

// ================================
// Date/time components
// ================================

slay time_year(dt datetime) normie {
    damn time_year_impl(dt);
}

slay time_month(dt datetime) normie {
    damn time_month_impl(dt);
}

slay time_day(dt datetime) normie {
    damn time_day_impl(dt);
}

slay time_hour(dt datetime) normie {
    damn time_hour_impl(dt);
}

slay time_minute(dt datetime) normie {
    damn time_minute_impl(dt);
}

slay time_second(dt datetime) normie {
    damn time_second_impl(dt);
}

slay time_weekday(dt datetime) normie {
    damn time_weekday_impl(dt);
}

slay time_day_of_year(dt datetime) normie {
    damn time_day_of_year_impl(dt);
}

// ================================
// Date/time arithmetic
// ================================

slay time_add_years(dt datetime, years normie) datetime {
    damn time_add_years_impl(dt, years);
}

slay time_add_months(dt datetime, months normie) datetime {
    damn time_add_months_impl(dt, months);
}

slay time_add_days(dt datetime, days normie) datetime {
    damn time_add_days_impl(dt, days);
}

slay time_add_hours(dt datetime, hours normie) datetime {
    damn time_add_hours_impl(dt, hours);
}

slay time_add_minutes(dt datetime, minutes normie) datetime {
    damn time_add_minutes_impl(dt, minutes);
}

slay time_add_seconds(dt datetime, seconds normie) datetime {
    damn time_add_seconds_impl(dt, seconds);
}

slay time_subtract(dt1 datetime, dt2 datetime) duration {
    damn time_subtract_impl(dt1, dt2);
}

slay time_diff_days(dt1 datetime, dt2 datetime) normie {
    damn time_diff_days_impl(dt1, dt2);
}

slay time_diff_hours(dt1 datetime, dt2 datetime) normie {
    damn time_diff_hours_impl(dt1, dt2);
}

slay time_diff_minutes(dt1 datetime, dt2 datetime) normie {
    damn time_diff_minutes_impl(dt1, dt2);
}

slay time_diff_seconds(dt1 datetime, dt2 datetime) normie {
    damn time_diff_seconds_impl(dt1, dt2);
}

// ================================
// Duration operations
// ================================

slay duration_from_seconds(seconds normie) duration {
    damn duration_from_seconds_impl(seconds);
}

slay duration_from_millis(millis normie) duration {
    damn duration_from_millis_impl(millis);
}

slay duration_to_seconds(dur duration) normie {
    damn duration_to_seconds_impl(dur);
}

slay duration_to_millis(dur duration) normie {
    damn duration_to_millis_impl(dur);
}

slay duration_add(dur1 duration, dur2 duration) duration {
    damn duration_add_impl(dur1, dur2);
}

slay duration_subtract(dur1 duration, dur2 duration) duration {
    damn duration_subtract_impl(dur1, dur2);
}

// ================================
// Time zone operations
// ================================

slay time_utc() datetime {
    damn time_utc_impl();
}

slay time_local() datetime {
    damn time_local_impl();
}

slay time_to_utc(dt datetime) datetime {
    damn time_to_utc_impl(dt);
}

slay time_to_local(dt datetime) datetime {
    damn time_to_local_impl(dt);
}

slay time_timezone_offset() normie {
    damn time_timezone_offset_impl();
}

// ================================
// Validation and utilities
// ================================

slay time_is_leap_year(year normie) lit {
    damn time_is_leap_year_impl(year);
}

slay time_days_in_month(year normie, month normie) normie {
    damn time_days_in_month_impl(year, month);
}

slay time_is_valid_date(year normie, month normie, day normie) lit {
    damn time_is_valid_date_impl(year, month, day);
}

slay time_is_weekend(dt datetime) lit {
    sus weekday normie = time_weekday(dt);
    damn weekday == 0 || weekday == 6;  // Sunday = 0, Saturday = 6
}

// ================================
// Sleep and timing
// ================================

slay time_sleep(seconds normie) {
    time_sleep_impl(seconds);
}

slay time_sleep_millis(millis normie) {
    time_sleep_millis_impl(millis);
}

slay time_sleep_micros(micros normie) {
    time_sleep_micros_impl(micros);
}

// ================================
// Benchmarking and profiling
// ================================

slay time_benchmark(func slay) duration {
    sus start normie = time_now_nanos();
    func();
    sus end normie = time_now_nanos();
    damn duration_from_nanos(end - start);
}

slay time_measure(func slay) [extra] {
    sus start normie = time_now_nanos();
    sus result extra = func();
    sus end normie = time_now_nanos();
    sus duration duration = duration_from_nanos(end - start);
    damn [result, duration];
}

// ================================
// Constants and utilities
// ================================

slay time_seconds_per_minute() normie {
    damn 60;
}

slay time_minutes_per_hour() normie {
    damn 60;
}

slay time_hours_per_day() normie {
    damn 24;
}

slay time_days_per_week() normie {
    damn 7;
}

slay time_months_per_year() normie {
    damn 12;
}

slay time_millis_per_second() normie {
    damn 1000;
}

slay time_micros_per_second() normie {
    damn 1000000;
}

slay time_nanos_per_second() normie {
    damn 1000000000;
}
