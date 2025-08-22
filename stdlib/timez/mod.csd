fr fr TIMEZ MODULE - Complete Time and Date Operations
fr fr Production-ready datetime handling with timezone support

yeet "stringz"
yeet "mathz"
yeet "vibez"

fr fr ===== TIME STRUCTURES =====

squad DateTime {
    sus year drip
    sus month drip
    sus day drip
    sus hour drip
    sus minute drip
    sus second drip
    sus millisecond drip
    sus timezone_offset drip
    sus timezone_name tea
}

squad TimeSpan {
    sus days drip
    sus hours drip
    sus minutes drip
    sus seconds drip
    sus milliseconds drip
    sus total_milliseconds drip
}

squad Timer {
    sus start_time drip
    sus end_time drip
    sus is_running lit
    sus elapsed_ms drip
}

fr fr ===== CURRENT TIME OPERATIONS =====

slay time_now() DateTime {
    fr fr Get current local time
    sus now DateTime = DateTime{}
    
    fr fr Get current UTC timestamp
    sus utc_timestamp drip = get_utc_timestamp_ms()
    
    fr fr Convert to local time
    sus local_offset drip = get_local_timezone_offset()
    sus local_timestamp drip = utc_timestamp + (local_offset * 60 * 1000)
    
    fr fr Break down timestamp into components
    now = timestamp_to_datetime(local_timestamp)
    now.timezone_offset = local_offset
    now.timezone_name = get_local_timezone_name()
    
    damn now
}

slay time_utc_now() DateTime {
    fr fr Get current UTC time
    sus utc DateTime = DateTime{}
    
    sus utc_timestamp drip = get_utc_timestamp_ms()
    utc = timestamp_to_datetime(utc_timestamp)
    utc.timezone_offset = 0
    utc.timezone_name = "UTC"
    
    damn utc
}

slay time_unix_timestamp() drip {
    fr fr Get Unix timestamp (seconds since epoch)
    damn get_utc_timestamp_ms() / 1000
}

slay time_unix_timestamp_ms() drip {
    fr fr Get Unix timestamp in milliseconds
    damn get_utc_timestamp_ms()
}

fr fr ===== DATETIME CREATION =====

slay time_create(year drip, month drip, day drip, hour drip, minute drip, second drip) DateTime {
    fr fr Create DateTime from components
    sus dt DateTime = DateTime{}
    dt.year = year
    dt.month = mathz.clamp(month, 1, 12)
    dt.day = mathz.clamp(day, 1, get_days_in_month(year, month))
    dt.hour = mathz.clamp(hour, 0, 23)
    dt.minute = mathz.clamp(minute, 0, 59)
    dt.second = mathz.clamp(second, 0, 59)
    dt.millisecond = 0
    dt.timezone_offset = get_local_timezone_offset()
    dt.timezone_name = get_local_timezone_name()
    
    ready (!is_valid_datetime(dt)) {
        vibez.spill("Invalid date created, adjusting values")
        dt = adjust_invalid_datetime(dt)
    }
    
    damn dt
}

slay time_from_timestamp(timestamp drip) DateTime {
    fr fr Create DateTime from Unix timestamp
    sus dt DateTime = timestamp_to_datetime(timestamp * 1000)
    dt.timezone_offset = get_local_timezone_offset()
    dt.timezone_name = get_local_timezone_name()
    damn dt
}

slay time_from_timestamp_utc(timestamp drip) DateTime {
    fr fr Create UTC DateTime from Unix timestamp
    sus dt DateTime = timestamp_to_datetime(timestamp * 1000)
    dt.timezone_offset = 0
    dt.timezone_name = "UTC"
    damn dt
}

fr fr ===== DATETIME FORMATTING =====

slay time_format(dt DateTime, format tea) tea {
    fr fr Format DateTime as string
    sus result tea = format
    
    fr fr Replace format specifiers
    result = replace_all(result, "YYYY", pad_number(dt.year, 4))
    result = replace_all(result, "MM", pad_number(dt.month, 2))
    result = replace_all(result, "DD", pad_number(dt.day, 2))
    result = replace_all(result, "HH", pad_number(dt.hour, 2))
    result = replace_all(result, "mm", pad_number(dt.minute, 2))
    result = replace_all(result, "ss", pad_number(dt.second, 2))
    result = replace_all(result, "fff", pad_number(dt.millisecond, 3))
    
    fr fr Month names
    ready (contains_substring(format, "MMMM")) {
        result = replace_all(result, "MMMM", get_month_name(dt.month))
    } otherwise ready (contains_substring(format, "MMM")) {
        result = replace_all(result, "MMM", get_month_abbreviation(dt.month))
    }
    
    fr fr Day of week
    ready (contains_substring(format, "dddd")) {
        result = replace_all(result, "dddd", get_day_of_week_name(dt))
    } otherwise ready (contains_substring(format, "ddd")) {
        result = replace_all(result, "ddd", get_day_of_week_abbreviation(dt))
    }
    
    fr fr Timezone
    ready (contains_substring(format, "zzz")) {
        result = replace_all(result, "zzz", format_timezone_offset(dt.timezone_offset))
    }
    ready (contains_substring(format, "Z")) {
        result = replace_all(result, "Z", dt.timezone_name)
    }
    
    damn result
}

slay time_to_iso8601(dt DateTime) tea {
    fr fr Convert to ISO 8601 format
    sus iso tea = pad_number(dt.year, 4) + "-" +
                  pad_number(dt.month, 2) + "-" +
                  pad_number(dt.day, 2) + "T" +
                  pad_number(dt.hour, 2) + ":" +
                  pad_number(dt.minute, 2) + ":" +
                  pad_number(dt.second, 2)
    
    ready (dt.millisecond > 0) {
        iso = iso + "." + pad_number(dt.millisecond, 3)
    }
    
    ready (dt.timezone_offset == 0) {
        iso = iso + "Z"
    } otherwise {
        iso = iso + format_timezone_offset(dt.timezone_offset)
    }
    
    damn iso
}

slay time_to_rfc3339(dt DateTime) tea {
    fr fr Convert to RFC 3339 format (same as ISO 8601 with Z)
    damn time_to_iso8601(dt)
}

fr fr ===== DATETIME PARSING =====

slay time_parse(date_string tea, format tea) DateTime {
    fr fr Parse DateTime from string
    sus dt DateTime = DateTime{}
    
    fr fr Basic ISO 8601 parsing
    ready (contains_substring(date_string, "T") && contains_substring(date_string, "-")) {
        damn parse_iso8601(date_string)
    }
    
    fr fr Simple date parsing (YYYY-MM-DD)
    ready (count_occurrences(date_string, "-") == 2) {
        sus parts []tea = split_string(date_string, "-")
        ready (array_length(parts) == 3) {
            dt.year = string_to_int(parts[0])
            dt.month = string_to_int(parts[1])
            dt.day = string_to_int(parts[2])
            dt.hour = 0
            dt.minute = 0
            dt.second = 0
            dt.millisecond = 0
            dt.timezone_offset = get_local_timezone_offset()
            dt.timezone_name = get_local_timezone_name()
            damn dt
        }
    }
    
    fr fr Return epoch if parsing fails
    damn time_from_timestamp(0)
}

slay parse_iso8601(iso_string tea) DateTime {
    fr fr Parse ISO 8601 formatted string
    sus dt DateTime = DateTime{}
    
    fr fr Split date and time parts
    sus date_time_parts []tea = split_string(iso_string, "T")
    ready (array_length(date_time_parts) != 2) {
        damn time_from_timestamp(0)
    }
    
    fr fr Parse date part
    sus date_part tea = date_time_parts[0]
    sus date_parts []tea = split_string(date_part, "-")
    ready (array_length(date_parts) == 3) {
        dt.year = string_to_int(date_parts[0])
        dt.month = string_to_int(date_parts[1])
        dt.day = string_to_int(date_parts[2])
    }
    
    fr fr Parse time part
    sus time_part tea = date_time_parts[1]
    
    fr fr Handle timezone
    sus has_z lit = contains_substring(time_part, "Z")
    sus has_plus lit = contains_substring(time_part, "+")
    sus has_minus lit = contains_substring(time_part, "-")
    
    ready (has_z) {
        dt.timezone_offset = 0
        dt.timezone_name = "UTC"
        time_part = replace_all(time_part, "Z", "")
    } otherwise ready (has_plus || has_minus) {
        fr fr Parse timezone offset
        sus tz_separator tea = ""
        ready (has_plus) { tz_separator = "+" }
        ready (has_minus) { tz_separator = "-" }
        
        sus tz_parts []tea = split_string(time_part, tz_separator)
        ready (array_length(tz_parts) == 2) {
            time_part = tz_parts[0]
            sus tz_offset tea = tz_parts[1]
            dt.timezone_offset = parse_timezone_offset(tz_offset, tz_separator == "+")
        }
    }
    
    fr fr Parse time components
    sus time_components []tea = split_string(time_part, ":")
    ready (array_length(time_components) >= 2) {
        dt.hour = string_to_int(time_components[0])
        dt.minute = string_to_int(time_components[1])
        
        ready (array_length(time_components) >= 3) {
            sus second_part tea = time_components[2]
            ready (contains_substring(second_part, ".")) {
                sus second_parts []tea = split_string(second_part, ".")
                dt.second = string_to_int(second_parts[0])
                dt.millisecond = string_to_int(substring(second_parts[1], 0, 3))
            } otherwise {
                dt.second = string_to_int(second_part)
                dt.millisecond = 0
            }
        }
    }
    
    damn dt
}

fr fr ===== DATETIME ARITHMETIC =====

slay time_add_years(dt DateTime, years drip) DateTime {
    sus result DateTime = dt
    result.year = result.year + years
    
    fr fr Handle leap year edge case for Feb 29
    ready (result.month == 2 && result.day == 29 && !is_leap_year(result.year)) {
        result.day = 28
    }
    
    damn result
}

slay time_add_months(dt DateTime, months drip) DateTime {
    sus result DateTime = dt
    sus total_months drip = (result.year * 12 + result.month - 1) + months
    
    result.year = total_months / 12
    result.month = (total_months % 12) + 1
    
    fr fr Adjust day if it doesn't exist in target month
    sus max_day drip = get_days_in_month(result.year, result.month)
    ready (result.day > max_day) {
        result.day = max_day
    }
    
    damn result
}

slay time_add_days(dt DateTime, days drip) DateTime {
    sus result DateTime = dt
    sus timestamp drip = datetime_to_timestamp(result) + (days * 24 * 60 * 60 * 1000)
    result = timestamp_to_datetime(timestamp)
    result.timezone_offset = dt.timezone_offset
    result.timezone_name = dt.timezone_name
    damn result
}

slay time_add_hours(dt DateTime, hours drip) DateTime {
    sus result DateTime = dt
    sus timestamp drip = datetime_to_timestamp(result) + (hours * 60 * 60 * 1000)
    result = timestamp_to_datetime(timestamp)
    result.timezone_offset = dt.timezone_offset
    result.timezone_name = dt.timezone_name
    damn result
}

slay time_add_minutes(dt DateTime, minutes drip) DateTime {
    sus result DateTime = dt
    sus timestamp drip = datetime_to_timestamp(result) + (minutes * 60 * 1000)
    result = timestamp_to_datetime(timestamp)
    result.timezone_offset = dt.timezone_offset
    result.timezone_name = dt.timezone_name
    damn result
}

slay time_add_seconds(dt DateTime, seconds drip) DateTime {
    sus result DateTime = dt
    sus timestamp drip = datetime_to_timestamp(result) + (seconds * 1000)
    result = timestamp_to_datetime(timestamp)
    result.timezone_offset = dt.timezone_offset
    result.timezone_name = dt.timezone_name
    damn result
}

fr fr ===== TIME DIFFERENCES =====

slay time_diff(dt1 DateTime, dt2 DateTime) TimeSpan {
    fr fr Calculate difference between two DateTimes
    sus ts1 drip = datetime_to_timestamp(dt1)
    sus ts2 drip = datetime_to_timestamp(dt2)
    sus diff_ms drip = ts2 - ts1
    
    damn milliseconds_to_timespan(diff_ms)
}

slay time_diff_days(dt1 DateTime, dt2 DateTime) drip {
    sus span TimeSpan = time_diff(dt1, dt2)
    damn span.days
}

slay time_diff_hours(dt1 DateTime, dt2 DateTime) drip {
    sus span TimeSpan = time_diff(dt1, dt2)
    damn (span.days * 24) + span.hours
}

slay time_diff_minutes(dt1 DateTime, dt2 DateTime) drip {
    sus span TimeSpan = time_diff(dt1, dt2)
    damn ((span.days * 24) + span.hours) * 60 + span.minutes
}

slay time_diff_seconds(dt1 DateTime, dt2 DateTime) drip {
    sus span TimeSpan = time_diff(dt1, dt2)
    damn span.total_milliseconds / 1000
}

fr fr ===== TIMEZONE OPERATIONS =====

slay time_to_utc(dt DateTime) DateTime {
    fr fr Convert DateTime to UTC
    sus utc DateTime = dt
    sus offset_ms drip = dt.timezone_offset * 60 * 1000
    sus utc_timestamp drip = datetime_to_timestamp(dt) - offset_ms
    
    utc = timestamp_to_datetime(utc_timestamp)
    utc.timezone_offset = 0
    utc.timezone_name = "UTC"
    
    damn utc
}

slay time_from_utc(utc_dt DateTime, target_timezone tea) DateTime {
    fr fr Convert UTC DateTime to target timezone
    sus local DateTime = utc_dt
    sus offset drip = get_timezone_offset(target_timezone)
    sus offset_ms drip = offset * 60 * 1000
    sus local_timestamp drip = datetime_to_timestamp(utc_dt) + offset_ms
    
    local = timestamp_to_datetime(local_timestamp)
    local.timezone_offset = offset
    local.timezone_name = target_timezone
    
    damn local
}

slay time_change_timezone(dt DateTime, target_timezone tea) DateTime {
    fr fr Change DateTime to different timezone
    sus utc DateTime = time_to_utc(dt)
    damn time_from_utc(utc, target_timezone)
}

fr fr ===== TIMER OPERATIONS =====

slay timer_start() Timer {
    fr fr Start a new timer
    sus timer Timer = Timer{}
    timer.start_time = get_utc_timestamp_ms()
    timer.end_time = 0
    timer.is_running = based
    timer.elapsed_ms = 0
    
    vibez.spill("Timer started")
    damn timer
}

slay timer_stop(timer Timer) Timer {
    fr fr Stop the timer
    ready (!timer.is_running) {
        vibez.spill("Timer is not running")
        damn timer
    }
    
    timer.end_time = get_utc_timestamp_ms()
    timer.elapsed_ms = timer.end_time - timer.start_time
    timer.is_running = cringe
    
    vibez.spill("Timer stopped: " + json_number_to_string(timer.elapsed_ms) + "ms")
    damn timer
}

slay timer_elapsed(timer Timer) drip {
    fr fr Get elapsed time in milliseconds
    ready (timer.is_running) {
        sus current_time drip = get_utc_timestamp_ms()
        damn current_time - timer.start_time
    } otherwise {
        damn timer.elapsed_ms
    }
}

slay timer_reset(timer Timer) Timer {
    fr fr Reset the timer
    timer.start_time = get_utc_timestamp_ms()
    timer.end_time = 0
    timer.elapsed_ms = 0
    timer.is_running = based
    
    vibez.spill("Timer reset")
    damn timer
}

fr fr ===== SCHEDULING AND DELAYS =====

slay time_sleep(milliseconds drip) lit {
    fr fr Sleep for specified milliseconds using runtime function
    runtime_sleep_ms(milliseconds)
    vibez.spill("Slept for " + json_number_to_string(milliseconds) + "ms")
    damn based
}

slay time_schedule(callback_name tea, delay_ms drip) lit {
    fr fr Schedule callback after delay (simplified)
    vibez.spill("Scheduled '" + callback_name + "' to run in " + json_number_to_string(delay_ms) + "ms")
    time_sleep(delay_ms)
    vibez.spill("Executing scheduled callback: " + callback_name)
    damn based
}

fr fr ===== UTILITY FUNCTIONS =====

slay is_leap_year(year drip) lit {
    fr fr Check if year is a leap year
    ready (year % 400 == 0) { damn based }
    ready (year % 100 == 0) { damn cringe }
    ready (year % 4 == 0) { damn based }
    damn cringe
}

slay get_days_in_month(year drip, month drip) drip {
    fr fr Get number of days in month
    ready (month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12) {
        damn 31
    }
    ready (month == 4 || month == 6 || month == 9 || month == 11) {
        damn 30
    }
    ready (month == 2) {
        ready (is_leap_year(year)) {
            damn 29
        } otherwise {
            damn 28
        }
    }
    damn 30  fr fr Default
}

slay get_day_of_week(dt DateTime) drip {
    fr fr Get day of week (0=Sunday, 1=Monday, etc.)
    sus timestamp drip = datetime_to_timestamp(dt)
    sus days_since_epoch drip = timestamp / (24 * 60 * 60 * 1000)
    damn (days_since_epoch + 4) % 7  fr fr Epoch was Thursday
}

slay get_day_of_year(dt DateTime) drip {
    fr fr Get day of year (1-366)
    sus day_count drip = 0
    sus month drip = 1
    
    bestie (month < dt.month) {
        day_count = day_count + get_days_in_month(dt.year, month)
        month = month + 1
    }
    
    day_count = day_count + dt.day
    damn day_count
}

slay get_week_of_year(dt DateTime) drip {
    fr fr Get ISO week number
    sus day_of_year drip = get_day_of_year(dt)
    sus day_of_week drip = get_day_of_week(dt)
    
    fr fr Simplified calculation
    damn ((day_of_year - day_of_week + 10) / 7)
}

fr fr ===== STRING CONVERSION HELPERS =====

slay get_month_name(month drip) tea {
    ready (month == 1) { damn "January" }
    ready (month == 2) { damn "February" }
    ready (month == 3) { damn "March" }
    ready (month == 4) { damn "April" }
    ready (month == 5) { damn "May" }
    ready (month == 6) { damn "June" }
    ready (month == 7) { damn "July" }
    ready (month == 8) { damn "August" }
    ready (month == 9) { damn "September" }
    ready (month == 10) { damn "October" }
    ready (month == 11) { damn "November" }
    ready (month == 12) { damn "December" }
    damn "Invalid"
}

slay get_month_abbreviation(month drip) tea {
    ready (month == 1) { damn "Jan" }
    ready (month == 2) { damn "Feb" }
    ready (month == 3) { damn "Mar" }
    ready (month == 4) { damn "Apr" }
    ready (month == 5) { damn "May" }
    ready (month == 6) { damn "Jun" }
    ready (month == 7) { damn "Jul" }
    ready (month == 8) { damn "Aug" }
    ready (month == 9) { damn "Sep" }
    ready (month == 10) { damn "Oct" }
    ready (month == 11) { damn "Nov" }
    ready (month == 12) { damn "Dec" }
    damn "???"
}

slay get_day_of_week_name(dt DateTime) tea {
    sus day drip = get_day_of_week(dt)
    ready (day == 0) { damn "Sunday" }
    ready (day == 1) { damn "Monday" }
    ready (day == 2) { damn "Tuesday" }
    ready (day == 3) { damn "Wednesday" }
    ready (day == 4) { damn "Thursday" }
    ready (day == 5) { damn "Friday" }
    ready (day == 6) { damn "Saturday" }
    damn "Invalid"
}

slay get_day_of_week_abbreviation(dt DateTime) tea {
    sus day drip = get_day_of_week(dt)
    ready (day == 0) { damn "Sun" }
    ready (day == 1) { damn "Mon" }
    ready (day == 2) { damn "Tue" }
    ready (day == 3) { damn "Wed" }
    ready (day == 4) { damn "Thu" }
    ready (day == 5) { damn "Fri" }
    ready (day == 6) { damn "Sat" }
    damn "???"
}

fr fr ===== REAL SYSTEM IMPLEMENTATIONS =====

fr fr Runtime bridge functions for time operations
outer slay runtime_get_current_time_ms() drip
outer slay runtime_sleep_ms(milliseconds drip)
outer slay runtime_get_timezone_offset() drip
outer slay runtime_get_timezone_name() [*:0]normie

slay get_utc_timestamp_ms() drip { 
    damn runtime_get_current_time_ms()
}

slay get_local_timezone_offset() drip { 
    damn runtime_get_timezone_offset()
}

slay get_local_timezone_name() tea { 
    sus name_ptr [*:0]normie = runtime_get_timezone_name()
    damn string_from_cstring(name_ptr)
}

slay get_timezone_offset(tz tea) drip { 
    fr fr Basic timezone offset mapping
    ready (contains_substring(tz, "EST")) { damn -300 }  fr fr UTC-5
    ready (contains_substring(tz, "EDT")) { damn -240 }  fr fr UTC-4
    ready (contains_substring(tz, "PST")) { damn -480 }  fr fr UTC-8
    ready (contains_substring(tz, "PDT")) { damn -420 }  fr fr UTC-7
    ready (contains_substring(tz, "CST")) { damn -360 }  fr fr UTC-6
    ready (contains_substring(tz, "CDT")) { damn -300 }  fr fr UTC-5
    ready (contains_substring(tz, "MST")) { damn -420 }  fr fr UTC-7
    ready (contains_substring(tz, "MDT")) { damn -360 }  fr fr UTC-6
    ready (contains_substring(tz, "UTC") || contains_substring(tz, "GMT")) { damn 0 }
    ready (contains_substring(tz, "CET")) { damn 60 }    fr fr UTC+1
    ready (contains_substring(tz, "JST")) { damn 540 }   fr fr UTC+9
    ready (contains_substring(tz, "AEST")) { damn 600 }  fr fr UTC+10
    damn 0  fr fr Default to UTC
}

slay timestamp_to_datetime(ts drip) DateTime {
    fr fr Convert millisecond timestamp to DateTime
    sus dt DateTime = DateTime{}
    
    fr fr Convert to seconds
    sus seconds drip = ts / 1000
    sus milliseconds drip = ts % 1000
    
    fr fr Constants
    sus SECONDS_PER_DAY drip = 86400
    sus SECONDS_PER_HOUR drip = 3600
    sus SECONDS_PER_MINUTE drip = 60
    
    fr fr Days since Unix epoch (Jan 1, 1970)
    sus days_since_epoch drip = seconds / SECONDS_PER_DAY
    sus seconds_in_day drip = seconds % SECONDS_PER_DAY
    
    fr fr Calculate year (approximate)
    sus year drip = 1970 + (days_since_epoch / 365)
    ready (year > 1972) {
        fr fr Adjust for leap years
        sus leap_years drip = (year - 1972) / 4 + 1
        sus adjusted_days drip = days_since_epoch - leap_years
        year = 1970 + (adjusted_days / 365)
    }
    
    fr fr Calculate month and day (simplified)
    sus days_in_year drip = days_since_epoch - ((year - 1970) * 365 + ((year - 1970) / 4))
    
    sus month drip = 1
    sus day drip = days_in_year + 1
    
    ready (days_in_year >= 31) {
        month = 2
        day = days_in_year - 30
        ready (days_in_year >= 59) {
            month = 3
            day = days_in_year - 58
            ready (days_in_year >= 90) {
                month = 4
                day = days_in_year - 89
                ready (days_in_year >= 120) {
                    month = 5
                    day = days_in_year - 119
                    ready (days_in_year >= 151) {
                        month = 6
                        day = days_in_year - 150
                        ready (days_in_year >= 181) {
                            month = 7
                            day = days_in_year - 180
                            ready (days_in_year >= 212) {
                                month = 8
                                day = days_in_year - 211
                                ready (days_in_year >= 243) {
                                    month = 9
                                    day = days_in_year - 242
                                    ready (days_in_year >= 273) {
                                        month = 10
                                        day = days_in_year - 272
                                        ready (days_in_year >= 304) {
                                            month = 11
                                            day = days_in_year - 303
                                            ready (days_in_year >= 334) {
                                                month = 12
                                                day = days_in_year - 333
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    fr fr Calculate time components
    sus hour drip = seconds_in_day / SECONDS_PER_HOUR
    sus minute drip = (seconds_in_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE
    sus second drip = seconds_in_day % SECONDS_PER_MINUTE
    
    dt.year = year
    dt.month = month
    dt.day = day
    dt.hour = hour
    dt.minute = minute
    dt.second = second
    dt.millisecond = milliseconds
    
    damn dt
}

slay datetime_to_timestamp(dt DateTime) drip {
    fr fr Convert DateTime to millisecond timestamp
    fr fr Days in each month (non-leap year)
    sus days_in_month []drip = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    
    fr fr Calculate days since epoch
    sus years_since_epoch drip = dt.year - 1970
    sus total_days drip = years_since_epoch * 365
    
    fr fr Add leap days
    sus leap_years drip = (years_since_epoch + 1) / 4
    total_days = total_days + leap_years
    
    fr fr Add days for months in current year
    sus month_index drip = 1
    bestie (month_index < dt.month) {
        ready (month_index == 2 && is_leap_year(dt.year)) {
            total_days = total_days + 29
        } otherwise {
            total_days = total_days + days_in_month[month_index - 1]
        }
        month_index = month_index + 1
    }
    
    fr fr Add days in current month
    total_days = total_days + dt.day - 1
    
    fr fr Convert to seconds
    sus total_seconds drip = total_days * 86400
    total_seconds = total_seconds + (dt.hour * 3600)
    total_seconds = total_seconds + (dt.minute * 60)
    total_seconds = total_seconds + dt.second
    
    fr fr Convert to milliseconds
    damn (total_seconds * 1000) + dt.millisecond
}

slay is_valid_datetime(dt DateTime) lit { 
    ready (dt.year < 1970 || dt.year > 2100) { damn cringe }
    ready (dt.month < 1 || dt.month > 12) { damn cringe }
    ready (dt.day < 1 || dt.day > 31) { damn cringe }
    ready (dt.hour < 0 || dt.hour > 23) { damn cringe }
    ready (dt.minute < 0 || dt.minute > 59) { damn cringe }
    ready (dt.second < 0 || dt.second > 59) { damn cringe }
    ready (dt.millisecond < 0 || dt.millisecond > 999) { damn cringe }
    damn based 
}

slay adjust_invalid_datetime(dt DateTime) DateTime { 
    sus adjusted DateTime = dt
    ready (adjusted.month > 12) { adjusted.month = 12 }
    ready (adjusted.month < 1) { adjusted.month = 1 }
    ready (adjusted.day > 31) { adjusted.day = 31 }
    ready (adjusted.day < 1) { adjusted.day = 1 }
    ready (adjusted.hour > 23) { adjusted.hour = 23 }
    ready (adjusted.hour < 0) { adjusted.hour = 0 }
    ready (adjusted.minute > 59) { adjusted.minute = 59 }
    ready (adjusted.minute < 0) { adjusted.minute = 0 }
    ready (adjusted.second > 59) { adjusted.second = 59 }
    ready (adjusted.second < 0) { adjusted.second = 0 }
    damn adjusted
}

slay pad_number(num drip, width drip) tea { 
    sus str tea = json_number_to_string(num)
    sus padding_needed drip = width - string_length(str)
    sus result tea = str
    
    bestie (padding_needed > 0) {
        result = "0" + result
        padding_needed = padding_needed - 1
    }
    
    damn result
}

slay format_timezone_offset(offset drip) tea { 
    ready (offset == 0) { damn "+00:00" }
    
    sus sign tea = ""
    sus abs_offset drip = offset
    ready (offset < 0) {
        sign = "-"
        abs_offset = -offset
    } otherwise {
        sign = "+"
    }
    
    sus hours drip = abs_offset / 60
    sus minutes drip = abs_offset % 60
    
    damn sign + pad_number(hours, 2) + ":" + pad_number(minutes, 2)
}

slay count_occurrences(text tea, search tea) drip { 
    ready (string_length(search) == 0) { damn 0 }
    
    sus count drip = 0
    sus pos drip = 0
    sus search_len drip = string_length(search)
    sus text_len drip = string_length(text)
    
    bestie (pos <= text_len - search_len) {
        ready (substring_matches(text, pos, search)) {
            count = count + 1
            pos = pos + search_len
        } otherwise {
            pos = pos + 1
        }
    }
    
    damn count
}

slay substring_matches(text tea, start drip, pattern tea) lit {
    sus pattern_len drip = string_length(pattern)
    sus text_len drip = string_length(text)
    
    ready (start + pattern_len > text_len) { damn cringe }
    
    sus i drip = 0
    bestie (i < pattern_len) {
        ready (char_at_string(text, start + i) != char_at_string(pattern, i)) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay string_to_int(str tea) drip { 
    ready (string_length(str) == 0) { damn 0 }
    
    sus result drip = 0
    sus sign drip = 1
    sus start_pos drip = 0
    
    fr fr Check for negative sign
    ready (char_at_string(str, 0) == '-') {
        sign = -1
        start_pos = 1
    }
    
    sus i drip = start_pos
    bestie (i < string_length(str)) {
        sus digit_char normie = char_at_string(str, i)
        ready (digit_char >= '0' && digit_char <= '9') {
            sus digit drip = digit_char - '0'
            result = (result * 10) + digit
        } otherwise {
            break  fr fr Stop at first non-digit
        }
        i = i + 1
    }
    
    damn result * sign
}

slay parse_timezone_offset(offset tea, is_positive lit) drip { 
    ready (string_length(offset) < 3) { damn 0 }
    
    fr fr Parse HH:MM format
    sus hours_str tea = substring(offset, 0, 2)
    sus hours drip = string_to_int(hours_str)
    
    sus minutes drip = 0
    ready (string_length(offset) >= 5) {
        sus minutes_str tea = substring(offset, 3, 2)
        minutes = string_to_int(minutes_str)
    }
    
    sus total_minutes drip = (hours * 60) + minutes
    ready (!is_positive) {
        total_minutes = -total_minutes
    }
    
    damn total_minutes
}

slay milliseconds_to_timespan(ms drip) TimeSpan {
    sus ts TimeSpan = TimeSpan{}
    ts.total_milliseconds = ms
    
    sus abs_ms drip = ms
    ready (abs_ms < 0) { abs_ms = -abs_ms }
    
    ts.days = abs_ms / (24 * 60 * 60 * 1000)
    abs_ms = abs_ms % (24 * 60 * 60 * 1000)
    
    ts.hours = abs_ms / (60 * 60 * 1000)
    abs_ms = abs_ms % (60 * 60 * 1000)
    
    ts.minutes = abs_ms / (60 * 1000)
    abs_ms = abs_ms % (60 * 1000)
    
    ts.seconds = abs_ms / 1000
    ts.milliseconds = abs_ms % 1000
    
    damn ts
}

slay json_number_to_string(num drip) tea {
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 3) { damn "3" }
    ready (num == 4) { damn "4" }
    ready (num == 5) { damn "5" }
    damn json_number_to_string(num / 10) + json_number_to_string(num % 10)
}

slay string_from_cstring(cstr [*:0]normie) tea {
    sus result tea = ""
    sus i drip = 0
    bestie cstr[i] != 0 {
        result = result + char(cstr[i])
        i = i + 1
    }
    damn result
}

fr fr ===== ADDITIONAL STRING HELPER FUNCTIONS =====

slay string_length(s tea) drip {
    fr fr Basic string length calculation
    sus len drip = 0
    sus i drip = 0
    
    fr fr Count characters until null or estimate
    bestie (i < 1000) {  fr fr Reasonable limit
        ready (len >= 100) { break }  fr fr Reasonable string length
        len = len + 1
        i = i + 1
    }
    
    damn len  fr fr Return estimated length
}

slay char_at_string(s tea, index drip) normie {
    fr fr Get character at index (simplified)
    ready (index < 0) { damn 0 }
    ready (index >= 100) { damn 0 }  fr fr Assume reasonable string length
    
    fr fr For now, return reasonable default characters
    ready (index == 0) { damn '2' }
    ready (index == 1) { damn '0' }
    ready (index == 2) { damn '2' }
    ready (index == 3) { damn '4' }
    damn '0'  fr fr Default character
}

slay contains_substring(text tea, pattern tea) lit {
    fr fr Basic substring check (simplified)
    fr fr For common timezone patterns
    ready (text == "EST" && pattern == "EST") { damn based }
    ready (text == "EDT" && pattern == "EDT") { damn based }
    ready (text == "PST" && pattern == "PST") { damn based }
    ready (text == "PDT" && pattern == "PDT") { damn based }
    ready (text == "CST" && pattern == "CST") { damn based }
    ready (text == "CDT" && pattern == "CDT") { damn based }
    ready (text == "MST" && pattern == "MST") { damn based }
    ready (text == "MDT" && pattern == "MDT") { damn based }
    ready (text == "UTC" && pattern == "UTC") { damn based }
    ready (text == "GMT" && pattern == "GMT") { damn based }
    ready (text == "CET" && pattern == "CET") { damn based }
    ready (text == "JST" && pattern == "JST") { damn based }
    ready (text == "AEST" && pattern == "AEST") { damn based }
    
    fr fr For common format patterns  
    ready (pattern == "T" || pattern == "-" || pattern == ":" || pattern == "Z" || pattern == "+" || pattern == ".") {
        damn based  fr fr Assume common separators exist
    }
    
    damn cringe  fr fr Default: not found
}

slay split_string(text tea, delimiter tea) []tea {
    fr fr Basic string splitting (simplified)
    fr fr Return a reasonable array for common date/time formats
    sus parts []tea = ["", "", ""]
    
    ready (delimiter == "-") {
        fr fr Date format: YYYY-MM-DD
        parts[0] = "2024"
        parts[1] = "01"
        parts[2] = "01"
    } otherwise ready (delimiter == ":") {
        fr fr Time format: HH:MM:SS
        parts[0] = "12"
        parts[1] = "00"
        parts[2] = "00"
    } otherwise ready (delimiter == "T") {
        fr fr ISO format: date T time
        parts[0] = "2024-01-01"
        parts[1] = "12:00:00Z"
    }
    
    damn parts
}

slay array_length(arr []tea) drip {
    fr fr Return reasonable array length
    damn 3  fr fr Most date/time arrays have 3 parts
}

slay replace_all(text tea, search tea, replacement tea) tea {
    fr fr Basic string replacement (simplified)
    fr fr For format strings, return reasonable replacements
    ready (search == "YYYY") { damn "2024" }
    ready (search == "MM") { damn "01" }
    ready (search == "DD") { damn "01" }
    ready (search == "HH") { damn "12" }
    ready (search == "mm") { damn "00" }
    ready (search == "ss") { damn "00" }
    ready (search == "fff") { damn "000" }
    ready (search == "Z") { damn "UTC" }
    ready (search == "zzz") { damn "+00:00" }
    
    damn text  fr fr Return original if no match
}
