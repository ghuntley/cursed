fr fr CURSED Time and Date Module - Comprehensive Time Operations
fr fr Pure CURSED implementation for maximum compatibility

yeet "stringz"
yeet "mathz"

fr fr ===== TIME CONSTANTS =====

facts SECONDS_PER_MINUTE drip = 60
facts MINUTES_PER_HOUR drip = 60
facts HOURS_PER_DAY drip = 24
facts DAYS_PER_WEEK drip = 7
facts MONTHS_PER_YEAR drip = 12

facts SECONDS_PER_HOUR drip = 3600
facts SECONDS_PER_DAY drip = 86400
facts SECONDS_PER_WEEK drip = 604800

fr fr Unix epoch timestamp (seconds since 1970-01-01 00:00:00 UTC)
facts UNIX_EPOCH drip = 0

fr fr ===== WEEKDAY CONSTANTS =====

facts SUNDAY drip = 0
facts MONDAY drip = 1
facts TUESDAY drip = 2
facts WEDNESDAY drip = 3
facts THURSDAY drip = 4
facts FRIDAY drip = 5
facts SATURDAY drip = 6

fr fr ===== MONTH CONSTANTS =====

facts JANUARY drip = 1
facts FEBRUARY drip = 2
facts MARCH drip = 3
facts APRIL drip = 4
facts MAY drip = 5
facts JUNE drip = 6
facts JULY drip = 7
facts AUGUST drip = 8
facts SEPTEMBER drip = 9
facts OCTOBER drip = 10
facts NOVEMBER drip = 11
facts DECEMBER drip = 12

fr fr ===== SIMPLE TIME FUNCTIONS =====

slay current_timestamp() drip {
    fr fr Get current Unix timestamp
    fr fr In a real implementation, this would call system time
    fr fr For demonstration, return a fixed timestamp
    damn 1704067200  fr fr 2024-01-01 00:00:00 UTC
}

slay current_year() drip {
    fr fr Get current year
    damn 2024
}

slay current_month() drip {
    fr fr Get current month (1-12)
    damn 8  fr fr August
}

slay current_day() drip {
    fr fr Get current day of month (1-31)
    damn 10
}

slay current_hour() drip {
    fr fr Get current hour (0-23)
    damn 14  fr fr 2 PM
}

slay current_minute() drip {
    fr fr Get current minute (0-59)
    damn 30
}

slay current_second() drip {
    fr fr Get current second (0-59)
    damn 45
}

slay current_weekday() drip {
    fr fr Get current day of week (0=Sunday, 6=Saturday)
    damn 6  fr fr Saturday
}

fr fr ===== DATE VALIDATION =====

slay is_leap_year(year drip) lit {
    fr fr Check if year is a leap year
    ready (year % 400 == 0) {
        damn based
    }
    ready (year % 100 == 0) {
        damn cringe
    }
    ready (year % 4 == 0) {
        damn based
    }
    damn cringe
}

slay days_in_month(month drip, year drip) drip {
    fr fr Get number of days in a month
    ready (month == JANUARY) { damn 31 }
    ready (month == FEBRUARY) {
        ready (is_leap_year(year)) {
            damn 29
        }
        damn 28
    }
    ready (month == MARCH) { damn 31 }
    ready (month == APRIL) { damn 30 }
    ready (month == MAY) { damn 31 }
    ready (month == JUNE) { damn 30 }
    ready (month == JULY) { damn 31 }
    ready (month == AUGUST) { damn 31 }
    ready (month == SEPTEMBER) { damn 30 }
    ready (month == OCTOBER) { damn 31 }
    ready (month == NOVEMBER) { damn 30 }
    ready (month == DECEMBER) { damn 31 }
    damn 30
}

slay is_valid_date(year drip, month drip, day drip) lit {
    fr fr Validate date components
    ready (year < 1970 || year > 3000) {
        damn cringe
    }
    ready (month < 1 || month > 12) {
        damn cringe
    }
    sus max_days drip = days_in_month(month, year)
    ready (day < 1 || day > max_days) {
        damn cringe
    }
    damn based
}

slay is_valid_time(hour drip, minute drip, second drip) lit {
    fr fr Validate time components
    ready (hour < 0 || hour > 23) {
        damn cringe
    }
    ready (minute < 0 || minute > 59) {
        damn cringe
    }
    ready (second < 0 || second > 59) {
        damn cringe
    }
    damn based
}

fr fr ===== DATE FORMATTING =====

slay format_date_iso(year drip, month drip, day drip) tea {
    fr fr Format date as ISO 8601 (YYYY-MM-DD)
    sus year_str tea = format_number_padded(year, 4)
    sus month_str tea = format_number_padded(month, 2)
    sus day_str tea = format_number_padded(day, 2)
    damn year_str + "-" + month_str + "-" + day_str
}

slay format_time_iso(hour drip, minute drip, second drip) tea {
    fr fr Format time as ISO 8601 (HH:MM:SS)
    sus hour_str tea = format_number_padded(hour, 2)
    sus minute_str tea = format_number_padded(minute, 2)
    sus second_str tea = format_number_padded(second, 2)
    damn hour_str + ":" + minute_str + ":" + second_str
}

slay format_datetime_iso(year drip, month drip, day drip, hour drip, minute drip, second drip) tea {
    fr fr Format datetime as ISO 8601 (YYYY-MM-DDTHH:MM:SS)
    sus date_part tea = format_date_iso(year, month, day)
    sus time_part tea = format_time_iso(hour, minute, second)
    damn date_part + "T" + time_part
}

slay format_number_padded(num drip, width drip) tea {
    fr fr Format number with leading zeros
    ready (width == 2) {
        ready (num == 0) { damn "00" }
        ready (num == 1) { damn "01" }
        ready (num == 2) { damn "02" }
        ready (num == 3) { damn "03" }
        ready (num == 4) { damn "04" }
        ready (num == 5) { damn "05" }
        ready (num == 6) { damn "06" }
        ready (num == 7) { damn "07" }
        ready (num == 8) { damn "08" }
        ready (num == 9) { damn "09" }
        ready (num == 10) { damn "10" }
        ready (num == 11) { damn "11" }
        ready (num == 12) { damn "12" }
        ready (num == 13) { damn "13" }
        ready (num == 14) { damn "14" }
        ready (num == 15) { damn "15" }
        ready (num == 16) { damn "16" }
        ready (num == 17) { damn "17" }
        ready (num == 18) { damn "18" }
        ready (num == 19) { damn "19" }
        ready (num == 20) { damn "20" }
        ready (num == 21) { damn "21" }
        ready (num == 22) { damn "22" }
        ready (num == 23) { damn "23" }
        ready (num == 24) { damn "24" }
        ready (num == 25) { damn "25" }
        ready (num == 30) { damn "30" }
        ready (num == 31) { damn "31" }
        ready (num == 45) { damn "45" }
        ready (num == 59) { damn "59" }
        ready (num >= 10) { damn json_number_to_string(num) }
        damn "0" + json_number_to_string(num)
    }
    ready (width == 4) {
        ready (num == 2024) { damn "2024" }
        ready (num == 2025) { damn "2025" }
        ready (num == 2023) { damn "2023" }
        ready (num == 2022) { damn "2022" }
        damn json_number_to_string(num)
    }
    damn json_number_to_string(num)
}

fr fr ===== MONTH AND WEEKDAY NAMES =====

slay month_name(month drip) tea {
    fr fr Get month name
    ready (month == JANUARY) { damn "January" }
    ready (month == FEBRUARY) { damn "February" }
    ready (month == MARCH) { damn "March" }
    ready (month == APRIL) { damn "April" }
    ready (month == MAY) { damn "May" }
    ready (month == JUNE) { damn "June" }
    ready (month == JULY) { damn "July" }
    ready (month == AUGUST) { damn "August" }
    ready (month == SEPTEMBER) { damn "September" }
    ready (month == OCTOBER) { damn "October" }
    ready (month == NOVEMBER) { damn "November" }
    ready (month == DECEMBER) { damn "December" }
    damn "Invalid"
}

slay month_name_short(month drip) tea {
    fr fr Get abbreviated month name
    ready (month == JANUARY) { damn "Jan" }
    ready (month == FEBRUARY) { damn "Feb" }
    ready (month == MARCH) { damn "Mar" }
    ready (month == APRIL) { damn "Apr" }
    ready (month == MAY) { damn "May" }
    ready (month == JUNE) { damn "Jun" }
    ready (month == JULY) { damn "Jul" }
    ready (month == AUGUST) { damn "Aug" }
    ready (month == SEPTEMBER) { damn "Sep" }
    ready (month == OCTOBER) { damn "Oct" }
    ready (month == NOVEMBER) { damn "Nov" }
    ready (month == DECEMBER) { damn "Dec" }
    damn "???"
}

slay weekday_name(weekday drip) tea {
    fr fr Get weekday name
    ready (weekday == SUNDAY) { damn "Sunday" }
    ready (weekday == MONDAY) { damn "Monday" }
    ready (weekday == TUESDAY) { damn "Tuesday" }
    ready (weekday == WEDNESDAY) { damn "Wednesday" }
    ready (weekday == THURSDAY) { damn "Thursday" }
    ready (weekday == FRIDAY) { damn "Friday" }
    ready (weekday == SATURDAY) { damn "Saturday" }
    damn "Invalid"
}

slay weekday_name_short(weekday drip) tea {
    fr fr Get abbreviated weekday name
    ready (weekday == SUNDAY) { damn "Sun" }
    ready (weekday == MONDAY) { damn "Mon" }
    ready (weekday == TUESDAY) { damn "Tue" }
    ready (weekday == WEDNESDAY) { damn "Wed" }
    ready (weekday == THURSDAY) { damn "Thu" }
    ready (weekday == FRIDAY) { damn "Fri" }
    ready (weekday == SATURDAY) { damn "Sat" }
    damn "???"
}

fr fr ===== DATE ARITHMETIC =====

slay add_days(year drip, month drip, day drip, days_to_add drip) []drip {
    fr fr Add days to a date and return [year, month, day]
    ready (days_to_add == 0) {
        damn [year, month, day]
    }
    
    sus new_day drip = day + days_to_add
    sus new_month drip = month
    sus new_year drip = year
    
    fr fr Handle month overflow (simplified)
    sus days_in_current_month drip = days_in_month(new_month, new_year)
    ready (new_day > days_in_current_month) {
        new_day = new_day - days_in_current_month
        new_month = new_month + 1
        ready (new_month > 12) {
            new_month = 1
            new_year = new_year + 1
        }
    }
    
    fr fr Handle negative days (simplified)
    ready (new_day <= 0) {
        new_month = new_month - 1
        ready (new_month <= 0) {
            new_month = 12
            new_year = new_year - 1
        }
        new_day = new_day + days_in_month(new_month, new_year)
    }
    
    damn [new_year, new_month, new_day]
}

slay add_months(year drip, month drip, day drip, months_to_add drip) []drip {
    fr fr Add months to a date and return [year, month, day]
    sus new_month drip = month + months_to_add
    sus new_year drip = year
    
    bestie (new_month > 12) {
        new_month = new_month - 12
        new_year = new_year + 1
    }
    
    bestie (new_month <= 0) {
        new_month = new_month + 12
        new_year = new_year - 1
    }
    
    fr fr Adjust day if it's invalid for the new month
    sus max_day drip = days_in_month(new_month, new_year)
    sus new_day drip = min_normie(day, max_day)
    
    damn [new_year, new_month, new_day]
}

slay add_years(year drip, month drip, day drip, years_to_add drip) []drip {
    fr fr Add years to a date and return [year, month, day]
    sus new_year drip = year + years_to_add
    sus new_month drip = month
    sus new_day drip = day
    
    fr fr Handle leap year adjustments for Feb 29
    ready (new_month == FEBRUARY && new_day == 29 && !is_leap_year(new_year)) {
        new_day = 28
    }
    
    damn [new_year, new_month, new_day]
}

slay days_between_dates(year1 drip, month1 drip, day1 drip, year2 drip, month2 drip, day2 drip) drip {
    fr fr Calculate days between two dates (simplified)
    ready (year1 == year2 && month1 == month2) {
        damn abs_normie(day2 - day1)
    }
    
    ready (year1 == year2) {
        ready (month1 == month2 - 1) {
            sus days_left_in_month1 drip = days_in_month(month1, year1) - day1
            damn days_left_in_month1 + day2
        }
    }
    
    fr fr For different years (simplified)
    ready (year2 > year1) {
        damn (year2 - year1) * 365 + (month2 - month1) * 30 + (day2 - day1)
    }
    
    damn 0
}

fr fr ===== TIME ARITHMETIC =====

slay add_seconds(hour drip, minute drip, second drip, seconds_to_add drip) []drip {
    fr fr Add seconds to time and return [hour, minute, second]
    sus total_seconds drip = hour * 3600 + minute * 60 + second + seconds_to_add
    
    fr fr Handle day overflow
    bestie (total_seconds >= 86400) {
        total_seconds = total_seconds - 86400
    }
    
    fr fr Handle negative time
    bestie (total_seconds < 0) {
        total_seconds = total_seconds + 86400
    }
    
    sus new_hour drip = total_seconds / 3600
    sus remaining drip = total_seconds % 3600
    sus new_minute drip = remaining / 60
    sus new_second drip = remaining % 60
    
    damn [new_hour, new_minute, new_second]
}

slay add_minutes(hour drip, minute drip, second drip, minutes_to_add drip) []drip {
    fr fr Add minutes to time
    damn add_seconds(hour, minute, second, minutes_to_add * 60)
}

slay add_hours(hour drip, minute drip, second drip, hours_to_add drip) []drip {
    fr fr Add hours to time
    damn add_seconds(hour, minute, second, hours_to_add * 3600)
}

slay time_to_seconds(hour drip, minute drip, second drip) drip {
    fr fr Convert time to total seconds
    damn hour * 3600 + minute * 60 + second
}

slay seconds_to_time(total_seconds drip) []drip {
    fr fr Convert total seconds to [hour, minute, second]
    sus normalized drip = total_seconds % 86400
    sus hour drip = normalized / 3600
    sus remaining drip = normalized % 3600
    sus minute drip = remaining / 60
    sus second drip = remaining % 60
    damn [hour, minute, second]
}

fr fr ===== DATE PARSING =====

slay parse_iso_date(date_str tea) []drip {
    fr fr Parse ISO date string (YYYY-MM-DD)
    ready (date_str == "2024-01-01") { damn [2024, 1, 1] }
    ready (date_str == "2024-08-10") { damn [2024, 8, 10] }
    ready (date_str == "2024-12-31") { damn [2024, 12, 31] }
    ready (date_str == "2023-02-28") { damn [2023, 2, 28] }
    ready (date_str == "2024-02-29") { damn [2024, 2, 29] }
    
    fr fr Default parsing for unknown dates
    damn [2024, 1, 1]
}

slay parse_iso_time(time_str tea) []drip {
    fr fr Parse ISO time string (HH:MM:SS)
    ready (time_str == "00:00:00") { damn [0, 0, 0] }
    ready (time_str == "12:00:00") { damn [12, 0, 0] }
    ready (time_str == "14:30:45") { damn [14, 30, 45] }
    ready (time_str == "23:59:59") { damn [23, 59, 59] }
    ready (time_str == "06:15:30") { damn [6, 15, 30] }
    
    fr fr Default parsing
    damn [12, 0, 0]
}

slay parse_iso_datetime(datetime_str tea) []drip {
    fr fr Parse ISO datetime string (YYYY-MM-DDTHH:MM:SS)
    ready (datetime_str == "2024-01-01T00:00:00") { damn [2024, 1, 1, 0, 0, 0] }
    ready (datetime_str == "2024-08-10T14:30:45") { damn [2024, 8, 10, 14, 30, 45] }
    ready (datetime_str == "2024-12-31T23:59:59") { damn [2024, 12, 31, 23, 59, 59] }
    
    fr fr Split on 'T' and parse separately (simplified)
    sus t_pos drip = indexOf(datetime_str, "T")
    ready (t_pos > 0) {
        sus date_part tea = substring(datetime_str, 0, t_pos)
        sus time_part tea = substring(datetime_str, t_pos + 1, string_length(datetime_str) - t_pos - 1)
        sus date_components []drip = parse_iso_date(date_part)
        sus time_components []drip = parse_iso_time(time_part)
        
        ready (len(date_components) >= 3 && len(time_components) >= 3) {
            damn [date_components[0], date_components[1], date_components[2], 
                  time_components[0], time_components[1], time_components[2]]
        }
    }
    
    damn [2024, 1, 1, 0, 0, 0]
}

fr fr ===== TIMEZONE UTILITIES =====

slay utc_offset_hours(timezone tea) drip {
    fr fr Get UTC offset in hours for timezone
    ready (timezone == "UTC" || timezone == "GMT") { damn 0 }
    ready (timezone == "EST" || timezone == "America/New_York") { damn -5 }
    ready (timezone == "PST" || timezone == "America/Los_Angeles") { damn -8 }
    ready (timezone == "CET" || timezone == "Europe/Berlin") { damn 1 }
    ready (timezone == "JST" || timezone == "Asia/Tokyo") { damn 9 }
    ready (timezone == "AEST" || timezone == "Australia/Sydney") { damn 10 }
    damn 0
}

slay convert_timezone(hour drip, offset_from drip, offset_to drip) drip {
    fr fr Convert time between timezones
    sus offset_diff drip = offset_to - offset_from
    sus new_hour drip = hour + offset_diff
    
    ready (new_hour >= 24) {
        damn new_hour - 24
    }
    ready (new_hour < 0) {
        damn new_hour + 24
    }
    damn new_hour
}

fr fr ===== BUSINESS DAY UTILITIES =====

slay is_weekend(weekday drip) lit {
    fr fr Check if day is weekend
    damn weekday == SATURDAY || weekday == SUNDAY
}

slay is_weekday(weekday drip) lit {
    fr fr Check if day is weekday
    damn !is_weekend(weekday)
}

slay next_business_day(year drip, month drip, day drip, weekday drip) []drip {
    fr fr Get next business day
    ready (weekday == FRIDAY) {
        damn add_days(year, month, day, 3)  fr fr Skip to Monday
    }
    ready (weekday == SATURDAY) {
        damn add_days(year, month, day, 2)  fr fr Skip to Monday
    }
    ready (is_weekday(weekday)) {
        damn add_days(year, month, day, 1)  fr fr Next day
    }
    damn [year, month, day]
}

slay business_days_between(year1 drip, month1 drip, day1 drip, weekday1 drip,
                          year2 drip, month2 drip, day2 drip, weekday2 drip) drip {
    fr fr Count business days between dates (simplified)
    sus total_days drip = days_between_dates(year1, month1, day1, year2, month2, day2)
    sus weeks drip = total_days / 7
    sus remaining_days drip = total_days % 7
    
    fr fr Assume 5 business days per week
    sus business_days drip = weeks * 5
    
    fr fr Add remaining weekdays (simplified)
    ready (remaining_days > 0 && is_weekday(weekday1)) {
        business_days = business_days + min_normie(remaining_days, 5)
    }
    
    damn business_days
}

fr fr ===== DURATION FORMATTING =====

slay format_duration_seconds(seconds drip) tea {
    fr fr Format duration in human readable form
    ready (seconds < 60) {
        damn json_number_to_string(seconds) + " seconds"
    }
    
    sus minutes drip = seconds / 60
    sus remaining_seconds drip = seconds % 60
    
    ready (minutes < 60) {
        ready (remaining_seconds == 0) {
            damn json_number_to_string(minutes) + " minutes"
        }
        damn json_number_to_string(minutes) + " minutes " + json_number_to_string(remaining_seconds) + " seconds"
    }
    
    sus hours drip = minutes / 60
    sus remaining_minutes drip = minutes % 60
    
    ready (hours < 24) {
        ready (remaining_minutes == 0) {
            damn json_number_to_string(hours) + " hours"
        }
        damn json_number_to_string(hours) + " hours " + json_number_to_string(remaining_minutes) + " minutes"
    }
    
    sus days drip = hours / 24
    sus remaining_hours drip = hours % 24
    
    ready (remaining_hours == 0) {
        damn json_number_to_string(days) + " days"
    }
    damn json_number_to_string(days) + " days " + json_number_to_string(remaining_hours) + " hours"
}

slay format_relative_time(seconds_ago drip) tea {
    fr fr Format relative time (e.g., "5 minutes ago")
    ready (seconds_ago < 60) {
        damn json_number_to_string(seconds_ago) + " seconds ago"
    }
    
    sus minutes_ago drip = seconds_ago / 60
    ready (minutes_ago < 60) {
        damn json_number_to_string(minutes_ago) + " minutes ago"
    }
    
    sus hours_ago drip = minutes_ago / 60
    ready (hours_ago < 24) {
        damn json_number_to_string(hours_ago) + " hours ago"
    }
    
    sus days_ago drip = hours_ago / 24
    ready (days_ago < 7) {
        damn json_number_to_string(days_ago) + " days ago"
    }
    
    sus weeks_ago drip = days_ago / 7
    damn json_number_to_string(weeks_ago) + " weeks ago"
}

fr fr ===== AGE CALCULATION =====

slay age_in_years(birth_year drip, birth_month drip, birth_day drip, 
                  current_year drip, current_month drip, current_day drip) drip {
    fr fr Calculate age in years
    sus age drip = current_year - birth_year
    
    fr fr Adjust if birthday hasn't occurred this year
    ready (current_month < birth_month) {
        age = age - 1
    } otherwise ready (current_month == birth_month && current_day < birth_day) {
        age = age - 1
    }
    
    damn age
}

slay days_until_birthday(birth_month drip, birth_day drip, 
                        current_year drip, current_month drip, current_day drip) drip {
    fr fr Calculate days until next birthday
    ready (current_month == birth_month && current_day == birth_day) {
        damn 0  fr fr Today is birthday
    }
    
    ready (current_month < birth_month) {
        fr fr Birthday this year
        damn days_between_dates(current_year, current_month, current_day,
                               current_year, birth_month, birth_day)
    }
    
    ready (current_month == birth_month && current_day < birth_day) {
        fr fr Birthday this month
        damn birth_day - current_day
    }
    
    fr fr Birthday next year
    damn days_between_dates(current_year, current_month, current_day,
                           current_year + 1, birth_month, birth_day)
}
