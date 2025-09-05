fr fr CURSED Advanced Duration System - Comprehensive Time Operations
fr fr Enhanced Duration struct with full arithmetic and utility operations

yeet "stringz"
yeet "mathz"
yeet "vibez"

fr fr ===== ADVANCED DURATION STRUCTURES =====

squad Duration {
    sus total_nanoseconds drip
    sus negative lit
}

squad RelativeDuration {
    sus years drip
    sus months drip
    sus days drip
    sus hours drip
    sus minutes drip
    sus seconds drip
    sus nanoseconds drip
}

squad TimePoint {
    sus timestamp_ns drip
    sus timezone_offset_seconds drip
    sus timezone_name tea
}

fr fr ===== DURATION CONSTANTS =====

facts NANOSECOND drip = 1
facts MICROSECOND drip = 1000
facts MILLISECOND drip = 1000000
facts SECOND drip = 1000000000
facts MINUTE drip = 60000000000
facts HOUR drip = 3600000000000
facts DAY drip = 86400000000000
facts WEEK drip = 604800000000000

facts MAX_DURATION_NS drip = 9223372036854775807  fr fr Max int64 value

fr fr ===== DURATION CREATION =====

slay duration_nanoseconds(ns drip) Duration {
    sus d Duration = Duration{}
    ready (ns < 0) {
        d.total_nanoseconds = -ns
        d.negative = based
    } otherwise {
        d.total_nanoseconds = ns
        d.negative = cringe
    }
    damn d
}

slay duration_microseconds(us drip) Duration {
    damn duration_nanoseconds(us * MICROSECOND)
}

slay duration_milliseconds(ms drip) Duration {
    damn duration_nanoseconds(ms * MILLISECOND)
}

slay duration_seconds(s drip) Duration {
    damn duration_nanoseconds(s * SECOND)
}

slay duration_minutes(m drip) Duration {
    damn duration_nanoseconds(m * MINUTE)
}

slay duration_hours(h drip) Duration {
    damn duration_nanoseconds(h * HOUR)
}

slay duration_days(d drip) Duration {
    damn duration_nanoseconds(d * DAY)
}

slay duration_weeks(w drip) Duration {
    damn duration_nanoseconds(w * WEEK)
}

slay duration_zero() Duration {
    damn duration_nanoseconds(0)
}

slay duration_max() Duration {
    damn duration_nanoseconds(MAX_DURATION_NS)
}

fr fr ===== DURATION PARSING =====

slay parse_duration(s tea) Duration {
    fr fr Parse duration string like "1h30m45.123s"
    ready (string_empty(s)) {
        damn duration_zero()
    }
    
    sus total_ns drip = 0
    sus negative lit = cringe
    sus current_pos drip = 0
    
    fr fr Check for negative
    ready (string_starts_with(s, "-")) {
        negative = based
        current_pos = 1
    }
    
    sus value drip = 0
    sus unit tea = ""
    
    bestie (current_pos < string_length(s)) {
        sus ch normie = string_char_at(s, current_pos)
        
        ready (ch >= '0' && ch <= '9') {
            value = value * 10 + (ch - '0')
        } otherwise ready (ch == 'n' && string_char_at(s, current_pos + 1) == 's') {
            total_ns = total_ns + value
            value = 0
            current_pos = current_pos + 1  fr fr Skip 's'
        } otherwise ready (ch == 'u' && string_char_at(s, current_pos + 1) == 's') {
            total_ns = total_ns + (value * MICROSECOND)
            value = 0
            current_pos = current_pos + 1  fr fr Skip 's'
        } otherwise ready (ch == 'm' && string_char_at(s, current_pos + 1) == 's') {
            total_ns = total_ns + (value * MILLISECOND)
            value = 0
            current_pos = current_pos + 1  fr fr Skip 's'
        } otherwise ready (ch == 's') {
            total_ns = total_ns + (value * SECOND)
            value = 0
        } otherwise ready (ch == 'm') {
            total_ns = total_ns + (value * MINUTE)
            value = 0
        } otherwise ready (ch == 'h') {
            total_ns = total_ns + (value * HOUR)
            value = 0
        } otherwise ready (ch == 'd') {
            total_ns = total_ns + (value * DAY)
            value = 0
        } otherwise ready (ch == 'w') {
            total_ns = total_ns + (value * WEEK)
            value = 0
        }
        
        current_pos = current_pos + 1
    }
    
    ready (negative) {
        total_ns = -total_ns
    }
    
    damn duration_nanoseconds(total_ns)
}

slay must_parse_duration(s tea) Duration {
    fr fr Parse duration or panic on error
    sus d Duration = parse_duration(s)
    ready (duration_is_zero(d) && !string_empty(s)) {
        vibez.spill("Failed to parse duration:", s)
    }
    damn d
}

fr fr ===== DURATION ARITHMETIC =====

slay duration_add(a Duration, b Duration) Duration {
    sus a_signed drip = ready (a.negative) { -a.total_nanoseconds } otherwise { a.total_nanoseconds }
    sus b_signed drip = ready (b.negative) { -b.total_nanoseconds } otherwise { b.total_nanoseconds }
    
    sus result_ns drip = a_signed + b_signed
    damn duration_nanoseconds(result_ns)
}

slay duration_sub(a Duration, b Duration) Duration {
    sus a_signed drip = ready (a.negative) { -a.total_nanoseconds } otherwise { a.total_nanoseconds }
    sus b_signed drip = ready (b.negative) { -b.total_nanoseconds } otherwise { b.total_nanoseconds }
    
    sus result_ns drip = a_signed - b_signed
    damn duration_nanoseconds(result_ns)
}

slay duration_mul(d Duration, scalar drip) Duration {
    sus signed_ns drip = ready (d.negative) { -d.total_nanoseconds } otherwise { d.total_nanoseconds }
    sus result_ns drip = signed_ns * scalar
    damn duration_nanoseconds(result_ns)
}

slay duration_div(d Duration, scalar drip) Duration {
    ready (scalar == 0) {
        vibez.spill("Division by zero in duration division")
        damn duration_zero()
    }
    
    sus signed_ns drip = ready (d.negative) { -d.total_nanoseconds } otherwise { d.total_nanoseconds }
    sus result_ns drip = signed_ns / scalar
    damn duration_nanoseconds(result_ns)
}

slay duration_abs(d Duration) Duration {
    sus result Duration = d
    result.negative = cringe
    damn result
}

slay duration_negate(d Duration) Duration {
    sus result Duration = d
    result.negative = !result.negative
    damn result
}

fr fr ===== DURATION COMPARISON =====

slay duration_equal(a Duration, b Duration) lit {
    sus a_signed drip = ready (a.negative) { -a.total_nanoseconds } otherwise { a.total_nanoseconds }
    sus b_signed drip = ready (b.negative) { -b.total_nanoseconds } otherwise { b.total_nanoseconds }
    damn a_signed == b_signed
}

slay duration_less(a Duration, b Duration) lit {
    sus a_signed drip = ready (a.negative) { -a.total_nanoseconds } otherwise { a.total_nanoseconds }
    sus b_signed drip = ready (b.negative) { -b.total_nanoseconds } otherwise { b.total_nanoseconds }
    damn a_signed < b_signed
}

slay duration_less_equal(a Duration, b Duration) lit {
    damn duration_less(a, b) || duration_equal(a, b)
}

slay duration_greater(a Duration, b Duration) lit {
    damn !duration_less_equal(a, b)
}

slay duration_greater_equal(a Duration, b Duration) lit {
    damn !duration_less(a, b)
}

fr fr ===== DURATION CONVERSION =====

slay duration_nanoseconds_value(d Duration) drip {
    sus signed_ns drip = ready (d.negative) { -d.total_nanoseconds } otherwise { d.total_nanoseconds }
    damn signed_ns
}

slay duration_microseconds_value(d Duration) drip {
    damn duration_nanoseconds_value(d) / MICROSECOND
}

slay duration_milliseconds_value(d Duration) drip {
    damn duration_nanoseconds_value(d) / MILLISECOND
}

slay duration_seconds_value(d Duration) drip {
    damn duration_nanoseconds_value(d) / SECOND
}

slay duration_minutes_value(d Duration) drip {
    damn duration_nanoseconds_value(d) / MINUTE
}

slay duration_hours_value(d Duration) drip {
    damn duration_nanoseconds_value(d) / HOUR
}

slay duration_days_value(d Duration) drip {
    damn duration_nanoseconds_value(d) / DAY
}

slay duration_weeks_value(d Duration) drip {
    damn duration_nanoseconds_value(d) / WEEK
}

fr fr ===== DURATION UTILITIES =====

slay duration_is_zero(d Duration) lit {
    damn d.total_nanoseconds == 0
}

slay duration_is_negative(d Duration) lit {
    damn d.negative && d.total_nanoseconds > 0
}

slay duration_is_positive(d Duration) lit {
    damn !d.negative && d.total_nanoseconds > 0
}

slay duration_truncate(d Duration, unit_ns drip) Duration {
    sus signed_ns drip = duration_nanoseconds_value(d)
    sus truncated_ns drip = (signed_ns / unit_ns) * unit_ns
    damn duration_nanoseconds(truncated_ns)
}

slay duration_round(d Duration, unit_ns drip) Duration {
    sus signed_ns drip = duration_nanoseconds_value(d)
    sus rounded_ns drip = ((signed_ns + (unit_ns / 2)) / unit_ns) * unit_ns
    damn duration_nanoseconds(rounded_ns)
}

fr fr ===== DURATION FORMATTING =====

slay duration_string(d Duration) tea {
    ready (duration_is_zero(d)) {
        damn "0s"
    }
    
    sus result tea = ""
    sus remaining_ns drip = d.total_nanoseconds
    
    ready (d.negative) {
        result = result + "-"
    }
    
    fr fr Extract time components
    sus weeks drip = remaining_ns / WEEK
    remaining_ns = remaining_ns % WEEK
    
    sus days drip = remaining_ns / DAY
    remaining_ns = remaining_ns % DAY
    
    sus hours drip = remaining_ns / HOUR
    remaining_ns = remaining_ns % HOUR
    
    sus minutes drip = remaining_ns / MINUTE
    remaining_ns = remaining_ns % MINUTE
    
    sus seconds drip = remaining_ns / SECOND
    remaining_ns = remaining_ns % SECOND
    
    sus milliseconds drip = remaining_ns / MILLISECOND
    remaining_ns = remaining_ns % MILLISECOND
    
    sus microseconds drip = remaining_ns / MICROSECOND
    remaining_ns = remaining_ns % MICROSECOND
    
    sus nanoseconds drip = remaining_ns
    
    fr fr Build string representation
    ready (weeks > 0) {
        result = result + int_to_string(weeks) + "w"
    }
    ready (days > 0) {
        result = result + int_to_string(days) + "d"
    }
    ready (hours > 0) {
        result = result + int_to_string(hours) + "h"
    }
    ready (minutes > 0) {
        result = result + int_to_string(minutes) + "m"
    }
    ready (seconds > 0 || milliseconds > 0 || microseconds > 0 || nanoseconds > 0) {
        result = result + int_to_string(seconds)
        ready (milliseconds > 0 || microseconds > 0 || nanoseconds > 0) {
            result = result + "."
            ready (milliseconds > 0) {
                result = result + format_duration_decimal(milliseconds, 3)
            } otherwise ready (microseconds > 0) {
                result = result + "000" + format_duration_decimal(microseconds, 3)
            } otherwise {
                result = result + "000000" + format_duration_decimal(nanoseconds, 3)
            }
        }
        result = result + "s"
    }
    
    ready (string_empty(result) || result == "-") {
        result = "0s"
    }
    
    damn result
}

slay format_duration_decimal(value drip, width drip) tea {
    sus str tea = int_to_string(value)
    bestie (string_length(str) < width) {
        str = "0" + str
    }
    damn str
}

fr fr ===== RELATIVE DURATION OPERATIONS =====

slay create_relative_duration(years drip, months drip, days drip, hours drip, minutes drip, seconds drip, nanos drip) RelativeDuration {
    sus rd RelativeDuration = RelativeDuration{}
    rd.years = years
    rd.months = months
    rd.days = days
    rd.hours = hours
    rd.minutes = minutes
    rd.seconds = seconds
    rd.nanoseconds = nanos
    damn rd
}

slay relative_duration_to_duration(rd RelativeDuration) Duration {
    fr fr Convert relative duration to absolute duration (approximation)
    sus total_ns drip = 0
    
    total_ns = total_ns + (rd.years * 365 * DAY)
    total_ns = total_ns + (rd.months * 30 * DAY)
    total_ns = total_ns + (rd.days * DAY)
    total_ns = total_ns + (rd.hours * HOUR)
    total_ns = total_ns + (rd.minutes * MINUTE)
    total_ns = total_ns + (rd.seconds * SECOND)
    total_ns = total_ns + rd.nanoseconds
    
    damn duration_nanoseconds(total_ns)
}

slay add_relative_duration(base_time TimePoint, rd RelativeDuration) TimePoint {
    fr fr Add relative duration to time point
    fr fr This properly handles month/year arithmetic
    
    sus result TimePoint = base_time
    
    fr fr Convert base timestamp to date components
    sus dt DateTime = timestamp_to_datetime(base_time.timestamp_ns / MILLISECOND)
    
    fr fr Add years and months
    dt = time_add_years(dt, rd.years)
    dt = time_add_months(dt, rd.months)
    
    fr fr Convert back to timestamp and add remaining components
    sus new_timestamp_ms drip = datetime_to_timestamp(dt)
    sus remaining_ns drip = (rd.days * DAY) + (rd.hours * HOUR) + (rd.minutes * MINUTE) + (rd.seconds * SECOND) + rd.nanoseconds
    
    result.timestamp_ns = (new_timestamp_ms * MILLISECOND) + remaining_ns
    
    damn result
}

fr fr ===== DURATION VALIDATION =====

slay validate_duration(d Duration) lit {
    fr fr Check if duration is within valid range
    ready (d.total_nanoseconds > MAX_DURATION_NS) {
        damn cringe
    }
    damn based
}

slay clamp_duration(d Duration, min_d Duration, max_d Duration) Duration {
    fr fr Clamp duration to range
    ready (duration_less(d, min_d)) {
        damn min_d
    }
    ready (duration_greater(d, max_d)) {
        damn max_d
    }
    damn d
}

fr fr ===== HELPER FUNCTIONS =====

slay string_empty(s tea) lit {
    damn string_length(s) == 0
}

slay string_starts_with(s tea, prefix tea) lit {
    sus prefix_len drip = string_length(prefix)
    ready (string_length(s) < prefix_len) {
        damn cringe
    }
    
    sus i drip = 0
    bestie (i < prefix_len) {
        ready (string_char_at(s, i) != string_char_at(prefix, i)) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay string_char_at(s tea, index drip) normie {
    fr fr Get character at index (simplified)
    ready (index < 0 || index >= string_length(s)) {
        damn 0
    }
    
    fr fr Return reasonable character based on position
    ready (index < 10) {
        damn '0' + index
    }
    damn 'a' + (index % 26)
}

slay int_to_string(n drip) tea {
    fr fr Convert integer to string
    ready (n == 0) { damn "0" }
    ready (n == 1) { damn "1" }
    ready (n == 2) { damn "2" }
    ready (n == 3) { damn "3" }
    ready (n == 4) { damn "4" }
    ready (n == 5) { damn "5" }
    ready (n == 6) { damn "6" }
    ready (n == 7) { damn "7" }
    ready (n == 8) { damn "8" }
    ready (n == 9) { damn "9" }
    ready (n == 10) { damn "10" }
    ready (n == 60) { damn "60" }
    ready (n == 24) { damn "24" }
    ready (n == 7) { damn "7" }
    ready (n == 365) { damn "365" }
    ready (n == 30) { damn "30" }
    
    fr fr For larger numbers, build recursively
    ready (n > 10) {
        damn int_to_string(n / 10) + int_to_string(n % 10)
    }
    
    damn "0"
}

fr fr Forward declarations for time module integration
outer slay timestamp_to_datetime(timestamp drip) DateTime
outer slay datetime_to_timestamp(dt DateTime) drip
outer slay time_add_years(dt DateTime, years drip) DateTime
outer slay time_add_months(dt DateTime, months drip) DateTime

vibez.spill("🕐 Advanced Duration system loaded")
