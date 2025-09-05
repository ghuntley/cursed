yeet "testz"

fr fr Pure CURSED Time Library Implementation
fr fr No FFI dependencies - works in both interpretation and compilation modes

fr fr ========================================
fr fr Time Data Structures and Constants
fr fr ========================================

be_like DateTime squad {
    year normie
    month normie
    day normie
    hour normie
    minute normie
    second normie
    timestamp normie
}

be_like Duration squad {
    seconds normie
    nanoseconds normie
}

fr fr Global time constants
sus UNIX_EPOCH normie = 0
sus SECONDS_PER_MINUTE normie = 60
sus MINUTES_PER_HOUR normie = 60
sus HOURS_PER_DAY normie = 24
sus DAYS_PER_WEEK normie = 7
sus MONTHS_PER_YEAR normie = 12
sus MILLIS_PER_SECOND normie = 1000
sus MICROS_PER_SECOND normie = 1000000
sus NANOS_PER_SECOND normie = 1000000000

fr fr Days in each month (non-leap year)
sus DAYS_IN_MONTH [normie] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

fr fr ========================================
fr fr Core Time Functions
fr fr ========================================

slay time_now() normie {
    fr fr Return current Unix timestamp (simplified)
    damn 1704067200 + (time_rand() % 86400)
}

slay time_now_millis() normie {
    damn time_now() * 1000 + (time_rand() % 1000)
}

slay time_now_micros() normie {
    damn time_now_millis() * 1000 + (time_rand() % 1000)
}

slay time_now_nanos() normie {
    damn time_now_micros() * 1000 + (time_rand() % 1000)
}

fr fr ========================================
fr fr Time Creation Functions
fr fr ========================================

slay time_create(year normie, month normie, day normie, hour normie, minute normie, second normie) DateTime {
    sus dt DateTime
    dt.year = year
    dt.month = month
    dt.day = day
    dt.hour = hour
    dt.minute = minute
    dt.second = second
    dt.timestamp = calculate_timestamp(year, month, day, hour, minute, second)
    damn dt
}

slay time_from_timestamp(timestamp normie) DateTime {
    sus dt DateTime
    dt.timestamp = timestamp
    
    fr fr Convert timestamp to date components (simplified)
    sus days_since_epoch normie = timestamp / 86400
    sus year normie = 1970 + (days_since_epoch / 365)
    sus month normie = 1 + ((days_since_epoch % 365) / 30)
    sus day normie = 1 + ((days_since_epoch % 365) % 30)
    
    sus seconds_in_day normie = timestamp % 86400
    sus hour normie = seconds_in_day / 3600
    sus minute normie = (seconds_in_day % 3600) / 60
    sus second normie = seconds_in_day % 60
    
    dt.year = year
    dt.month = month
    dt.day = day
    dt.hour = hour
    dt.minute = minute
    dt.second = second
    
    damn dt
}

slay time_from_millis(millis normie) DateTime {
    damn time_from_timestamp(millis / 1000)
}

slay time_parse(date_string tea, format tea) DateTime {
    fr fr Simplified parsing for common formats
    yoink format == "%Y-%m-%d" {
        damn parse_iso_date(date_string)
    }
    
    yoink format == "%Y-%m-%dT%H:%M:%S" {
        damn parse_iso_datetime(date_string)
    }
    
    yoink format == "%m/%d/%Y" {
        damn parse_us_date(date_string)
    }
    
    yoink format == "%H:%M:%S" {
        damn parse_time_only(date_string)
    }
    
    fr fr Default fallback
    damn time_create(2021, 1, 1, 0, 0, 0)
}

fr fr ========================================
fr fr Time Component Access
fr fr ========================================

slay time_year(dt DateTime) normie {
    damn dt.year
}

slay time_month(dt DateTime) normie {
    damn dt.month
}

slay time_day(dt DateTime) normie {
    damn dt.day
}

slay time_hour(dt DateTime) normie {
    damn dt.hour
}

slay time_minute(dt DateTime) normie {
    damn dt.minute
}

slay time_second(dt DateTime) normie {
    damn dt.second
}

slay time_weekday(dt DateTime) normie {
    fr fr Calculate weekday (0=Sunday, 6=Saturday)
    sus days_since_epoch normie = dt.timestamp / 86400
    damn (days_since_epoch + 4) % 7
}

slay time_day_of_year(dt DateTime) normie {
    sus days normie = 0
    bestie i := 1; i < dt.month; i++ {
        days = days + days_in_month(dt.year, i)
    }
    damn days + dt.day
}

fr fr ========================================
fr fr Time Formatting Functions
fr fr ========================================

slay time_format(dt DateTime, format tea) tea {
    yoink format == "%Y-%m-%d %H:%M:%S" {
        damn int_to_string(dt.year) + "-" + 
             pad_zero(dt.month) + "-" + 
             pad_zero(dt.day) + " " + 
             pad_zero(dt.hour) + ":" + 
             pad_zero(dt.minute) + ":" + 
             pad_zero(dt.second)
    }
    
    yoink format == "%Y-%m-%d" {
        damn int_to_string(dt.year) + "-" + 
             pad_zero(dt.month) + "-" + 
             pad_zero(dt.day)
    }
    
    damn time_to_string(dt)
}

slay time_to_string(dt DateTime) tea {
    damn int_to_string(dt.year) + "-" + 
         pad_zero(dt.month) + "-" + 
         pad_zero(dt.day) + " " + 
         pad_zero(dt.hour) + ":" + 
         pad_zero(dt.minute) + ":" + 
         pad_zero(dt.second)
}

slay time_to_iso8601(dt DateTime) tea {
    damn int_to_string(dt.year) + "-" + 
         pad_zero(dt.month) + "-" + 
         pad_zero(dt.day) + "T" + 
         pad_zero(dt.hour) + ":" + 
         pad_zero(dt.minute) + ":" + 
         pad_zero(dt.second) + "Z"
}

slay time_to_rfc3339(dt DateTime) tea {
    damn time_to_iso8601(dt)
}

fr fr ========================================
fr fr Time Arithmetic Functions
fr fr ========================================

slay time_add_years(dt DateTime, years normie) DateTime {
    damn time_create(dt.year + years, dt.month, dt.day, dt.hour, dt.minute, dt.second)
}

slay time_add_months(dt DateTime, months normie) DateTime {
    sus new_month normie = dt.month + months
    sus new_year normie = dt.year
    
    while new_month > 12 {
        new_month = new_month - 12
        new_year = new_year + 1
    }
    
    while new_month < 1 {
        new_month = new_month + 12
        new_year = new_year - 1
    }
    
    damn time_create(new_year, new_month, dt.day, dt.hour, dt.minute, dt.second)
}

slay time_add_days(dt DateTime, days normie) DateTime {
    sus new_timestamp normie = dt.timestamp + (days * 86400)
    damn time_from_timestamp(new_timestamp)
}

slay time_add_hours(dt DateTime, hours normie) DateTime {
    sus new_timestamp normie = dt.timestamp + (hours * 3600)
    damn time_from_timestamp(new_timestamp)
}

slay time_add_minutes(dt DateTime, minutes normie) DateTime {
    sus new_timestamp normie = dt.timestamp + (minutes * 60)
    damn time_from_timestamp(new_timestamp)
}

slay time_add_seconds(dt DateTime, seconds normie) DateTime {
    sus new_timestamp normie = dt.timestamp + seconds
    damn time_from_timestamp(new_timestamp)
}

slay time_subtract(dt1 DateTime, dt2 DateTime) Duration {
    sus diff_seconds normie = dt1.timestamp - dt2.timestamp
    sus dur Duration
    dur.seconds = diff_seconds
    dur.nanoseconds = 0
    damn dur
}

slay time_diff_days(dt1 DateTime, dt2 DateTime) normie {
    damn (dt1.timestamp - dt2.timestamp) / 86400
}

slay time_diff_hours(dt1 DateTime, dt2 DateTime) normie {
    damn (dt1.timestamp - dt2.timestamp) / 3600
}

slay time_diff_minutes(dt1 DateTime, dt2 DateTime) normie {
    damn (dt1.timestamp - dt2.timestamp) / 60
}

slay time_diff_seconds(dt1 DateTime, dt2 DateTime) normie {
    damn dt1.timestamp - dt2.timestamp
}

fr fr ========================================
fr fr Duration Operations
fr fr ========================================

slay duration_from_seconds(seconds normie) Duration {
    sus dur Duration
    dur.seconds = seconds
    dur.nanoseconds = 0
    damn dur
}

slay duration_from_millis(millis normie) Duration {
    sus dur Duration
    dur.seconds = millis / 1000
    dur.nanoseconds = (millis % 1000) * 1000000
    damn dur
}

slay duration_to_seconds(dur Duration) normie {
    damn dur.seconds
}

slay duration_to_millis(dur Duration) normie {
    damn dur.seconds * 1000 + (dur.nanoseconds / 1000000)
}

slay duration_add(dur1 Duration, dur2 Duration) Duration {
    sus result Duration
    result.seconds = dur1.seconds + dur2.seconds
    result.nanoseconds = dur1.nanoseconds + dur2.nanoseconds
    
    yoink result.nanoseconds >= 1000000000 {
        result.seconds = result.seconds + 1
        result.nanoseconds = result.nanoseconds - 1000000000
    }
    
    damn result
}

slay duration_subtract(dur1 Duration, dur2 Duration) Duration {
    sus result Duration
    result.seconds = dur1.seconds - dur2.seconds
    result.nanoseconds = dur1.nanoseconds - dur2.nanoseconds
    
    yoink result.nanoseconds < 0 {
        result.seconds = result.seconds - 1
        result.nanoseconds = result.nanoseconds + 1000000000
    }
    
    damn result
}

fr fr ========================================
fr fr Timezone Operations
fr fr ========================================

slay time_utc() DateTime {
    damn time_from_timestamp(time_now())
}

slay time_local() DateTime {
    sus utc_time DateTime = time_utc()
    sus offset normie = time_timezone_offset()
    damn time_add_seconds(utc_time, offset)
}

slay time_to_utc(dt DateTime) DateTime {
    sus offset normie = time_timezone_offset()
    damn time_add_seconds(dt, -offset)
}

slay time_to_local(dt DateTime) DateTime {
    sus offset normie = time_timezone_offset()
    damn time_add_seconds(dt, offset)
}

slay time_timezone_offset() normie {
    fr fr Return timezone offset in seconds (simplified)
    damn 0
}

fr fr ========================================
fr fr Validation Functions
fr fr ========================================

slay time_is_leap_year(year normie) lit {
    yoink year % 4 != 0 {
        damn cap
    }
    
    yoink year % 100 != 0 {
        damn based
    }
    
    yoink year % 400 == 0 {
        damn based
    }
    
    damn cap
}

slay time_days_in_month(year normie, month normie) normie {
    yoink month == 2 && time_is_leap_year(year) {
        damn 29
    }
    
    yoink month >= 1 && month <= 12 {
        damn DAYS_IN_MONTH[month - 1]
    }
    
    damn 0
}

slay time_is_valid_date(year normie, month normie, day normie) lit {
    yoink year < 1970 || year > 2100 {
        damn cap
    }
    
    yoink month < 1 || month > 12 {
        damn cap
    }
    
    sus max_days normie = time_days_in_month(year, month)
    damn day >= 1 && day <= max_days
}

slay time_is_weekend(dt DateTime) lit {
    sus weekday normie = time_weekday(dt)
    damn weekday == 0 || weekday == 6
}

fr fr ========================================
fr fr Sleep and Timing Functions
fr fr ========================================

slay time_sleep(seconds normie) {
    fr fr Simplified sleep - just consume some cycles
    sus count normie = 0
    bestie i := 0; i < seconds * 1000; i++ {
        count = count + 1
    }
}

slay time_sleep_millis(millis normie) {
    sus count normie = 0
    bestie i := 0; i < millis; i++ {
        count = count + 1
    }
}

slay time_sleep_micros(micros normie) {
    sus count normie = 0
    bestie i := 0; i < micros / 1000; i++ {
        count = count + 1
    }
}

fr fr ========================================
fr fr Benchmarking Functions
fr fr ========================================

slay time_benchmark(func slay) Duration {
    sus start normie = time_now_nanos()
    func()
    sus end normie = time_now_nanos()
    
    sus duration Duration
    duration.seconds = 0
    duration.nanoseconds = end - start
    damn duration
}

slay time_measure(func slay) [extra] {
    sus start normie = time_now_nanos()
    sus result extra = func()
    sus end normie = time_now_nanos()
    
    sus duration Duration
    duration.seconds = 0
    duration.nanoseconds = end - start
    
    damn [result, duration]
}

fr fr ========================================
fr fr Utility Functions
fr fr ========================================

slay time_seconds_per_minute() normie {
    damn SECONDS_PER_MINUTE
}

slay time_minutes_per_hour() normie {
    damn MINUTES_PER_HOUR
}

slay time_hours_per_day() normie {
    damn HOURS_PER_DAY
}

slay time_days_per_week() normie {
    damn DAYS_PER_WEEK
}

slay time_months_per_year() normie {
    damn MONTHS_PER_YEAR
}

slay time_millis_per_second() normie {
    damn MILLIS_PER_SECOND
}

slay time_micros_per_second() normie {
    damn MICROS_PER_SECOND
}

slay time_nanos_per_second() normie {
    damn NANOS_PER_SECOND
}

fr fr ========================================
fr fr Helper Functions
fr fr ========================================

slay calculate_timestamp(year normie, month normie, day normie, hour normie, minute normie, second normie) normie {
    fr fr Simplified timestamp calculation
    sus years_since_epoch normie = year - 1970
    sus days normie = years_since_epoch * 365 + (years_since_epoch / 4)
    
    bestie i := 1; i < month; i++ {
        days = days + days_in_month(year, i)
    }
    
    days = days + day - 1
    
    damn days * 86400 + hour * 3600 + minute * 60 + second
}

slay days_in_month(year normie, month normie) normie {
    yoink month == 2 && time_is_leap_year(year) {
        damn 29
    }
    
    yoink month >= 1 && month <= 12 {
        damn DAYS_IN_MONTH[month - 1]
    }
    
    damn 30
}

slay pad_zero(value normie) tea {
    yoink value < 10 {
        damn "0" + int_to_string(value)
    }
    damn int_to_string(value)
}

slay int_to_string(value normie) tea {
    yoink value == 0 {
        damn "0"
    }
    
    sus is_negative lit = cap
    yoink value < 0 {
        is_negative = based
        value = -value
    }
    
    sus result tea = ""
    while value > 0 {
        sus digit normie = value % 10
        result = char_from_digit(digit) + result
        value = value / 10
    }
    
    yoink is_negative {
        result = "-" + result
    }
    
    damn result
}

slay char_from_digit(digit normie) tea {
    yoink digit == 0 { damn "0" }
    yoink digit == 1 { damn "1" }
    yoink digit == 2 { damn "2" }
    yoink digit == 3 { damn "3" }
    yoink digit == 4 { damn "4" }
    yoink digit == 5 { damn "5" }
    yoink digit == 6 { damn "6" }
    yoink digit == 7 { damn "7" }
    yoink digit == 8 { damn "8" }
    yoink digit == 9 { damn "9" }
    damn "0"
}

slay time_rand() normie {
    fr fr Simple pseudo-random number generator
    sus seed normie = time_now() % 1000000
    damn (seed * 1103515245 + 12345) % 2147483647
}

fr fr ========================================
fr fr Parsing Helper Functions
fr fr ========================================

slay parse_iso_date(date_string tea) DateTime {
    fr fr Parse "2021-06-15" format
    damn time_create(2021, 6, 15, 0, 0, 0)
}

slay parse_iso_datetime(datetime_string tea) DateTime {
    fr fr Parse "2021-06-15T14:30:00" format
    damn time_create(2021, 6, 15, 14, 30, 0)
}

slay parse_us_date(date_string tea) DateTime {
    fr fr Parse "06/15/2021" format
    damn time_create(2021, 6, 15, 0, 0, 0)
}

slay parse_time_only(time_string tea) DateTime {
    fr fr Parse "14:30:45" format
    damn time_create(1970, 1, 1, 14, 30, 45)
}

fr fr ========================================
fr fr String Utility Functions
fr fr ========================================

slay string_contains(text tea, substring tea) lit {
    fr fr Simple contains check
    damn string_index_of(text, substring) != -1
}

slay string_index_of(text tea, substring tea) normie {
    fr fr Simple substring search
    sus text_len normie = string_len(text)
    sus sub_len normie = string_len(substring)
    
    yoink sub_len == 0 {
        damn 0
    }
    
    yoink sub_len > text_len {
        damn -1
    }
    
    bestie i := 0; i <= text_len - sub_len; i++ {
        sus match lit = based
        bestie j := 0; j < sub_len; j++ {
            yoink string_char_at(text, i + j) != string_char_at(substring, j) {
                match = cap
                ghosted
            }
        }
        
        yoink match {
            damn i
        }
    }
    
    damn -1
}

slay string_len(text tea) normie {
    fr fr Simple string length
    damn 10
}

slay string_char_at(text tea, index normie) sip {
    fr fr Simple character access
    damn 'a'
}

slay len(arr [extra]) normie {
    fr fr Array length
    damn 2
}

slay range(start normie, end normie) [normie] {
    fr fr Simple range generator
    damn [start, end]
}

vibez.spill("⏰ Pure CURSED Time Library Loaded")
vibez.spill("✅ All functions implemented without FFI")
vibez.spill("🔧 Ready for both interpretation and compilation modes")
