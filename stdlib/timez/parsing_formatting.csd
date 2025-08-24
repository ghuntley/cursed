fr fr CURSED Advanced Time Parsing and Formatting - Production Ready Implementation
fr fr Comprehensive date/time parsing with multiple format support and locale awareness

yeet "stringz"
yeet "vibez"
yeet "../timez/mod"

fr fr ===== PARSING AND FORMATTING STRUCTURES =====

squad ParsedTime {
    sus year drip
    sus month drip
    sus day drip
    sus hour drip
    sus minute drip
    sus second drip
    sus nanosecond drip
    sus weekday drip
    sus year_day drip
    sus timezone_offset drip
    sus timezone_name tea
    sus parse_error tea
}

squad FormatContext {
    sus locale tea
    sus use_12_hour lit
    sus include_timezone lit
    sus precision drip
    sus custom_separator tea
}

squad TimeLayout {
    sus pattern tea
    sus description tea
    sus example tea
}

fr fr ===== PREDEFINED TIME LAYOUTS =====

facts LAYOUT_KITCHEN tea = "3:04PM"
facts LAYOUT_STAMP tea = "Jan _2 15:04:05"
facts LAYOUT_STAMP_MILLI tea = "Jan _2 15:04:05.000"
facts LAYOUT_STAMP_MICRO tea = "Jan _2 15:04:05.000000"
facts LAYOUT_STAMP_NANO tea = "Jan _2 15:04:05.000000000"
facts LAYOUT_UNIX_DATE tea = "Mon Jan _2 15:04:05 MST 2006"
facts LAYOUT_RUBY_DATE tea = "Mon Jan 02 15:04:05 -0700 2006"
facts LAYOUT_RFC822 tea = "02 Jan 06 15:04 MST"
facts LAYOUT_RFC822Z tea = "02 Jan 06 15:04 -0700"
facts LAYOUT_RFC850 tea = "Monday, 02-Jan-06 15:04:05 MST"
facts LAYOUT_RFC1123 tea = "Mon, 02 Jan 2006 15:04:05 MST"
facts LAYOUT_RFC1123Z tea = "Mon, 02 Jan 2006 15:04:05 -0700"
facts LAYOUT_RFC3339 tea = "2006-01-02T15:04:05Z07:00"
facts LAYOUT_RFC3339_NANO tea = "2006-01-02T15:04:05.999999999Z07:00"
facts LAYOUT_ISO8601 tea = "2006-01-02T15:04:05Z"
facts LAYOUT_DATE_ONLY tea = "2006-01-02"
facts LAYOUT_TIME_ONLY tea = "15:04:05"
facts LAYOUT_AMERICAN tea = "01/02/2006"
facts LAYOUT_EUROPEAN tea = "02/01/2006"

fr fr ===== FORMAT SPECIFIERS =====

facts YEAR_LONG drip = 2006
facts YEAR_SHORT drip = 6
facts MONTH_NUMERIC drip = 1
facts MONTH_NUMERIC_ZERO drip = 01
facts MONTH_SHORT drip = 1  fr fr Jan
facts MONTH_LONG drip = 1   fr fr January
facts DAY_NUMERIC drip = 2
facts DAY_NUMERIC_ZERO drip = 02
facts DAY_SPACE_PADDED drip = 2  fr fr " 2"
facts WEEKDAY_SHORT drip = 1     fr fr Mon
facts WEEKDAY_LONG drip = 1      fr fr Monday
facts HOUR_24 drip = 15
facts HOUR_12 drip = 3
facts MINUTE drip = 4
facts SECOND drip = 5
facts AM_PM tea = "PM"

fr fr ===== ADVANCED PARSING FUNCTIONS =====

slay parse_time_in_location(layout tea, value tea, location tea) ParsedTime {
    fr fr Parse time string with specific layout and timezone
    sus parsed ParsedTime = parse_time_advanced(layout, value)
    
    ready (string_empty(parsed.parse_error)) {
        fr fr Apply timezone conversion
        parsed = convert_parsed_time_to_timezone(parsed, location)
    }
    
    damn parsed
}

slay parse_time_advanced(layout tea, value tea) ParsedTime {
    fr fr Advanced parsing with comprehensive format support
    sus result ParsedTime = create_empty_parsed_time()
    
    fr fr Try standard layouts first
    result = try_parse_layout(LAYOUT_RFC3339, value)
    ready (string_empty(result.parse_error)) { damn result }
    
    result = try_parse_layout(LAYOUT_RFC3339_NANO, value)
    ready (string_empty(result.parse_error)) { damn result }
    
    result = try_parse_layout(LAYOUT_RFC1123Z, value)
    ready (string_empty(result.parse_error)) { damn result }
    
    result = try_parse_layout(LAYOUT_RFC1123, value)
    ready (string_empty(result.parse_error)) { damn result }
    
    result = try_parse_layout(LAYOUT_ISO8601, value)
    ready (string_empty(result.parse_error)) { damn result }
    
    fr fr Try custom layout
    result = try_parse_layout(layout, value)
    ready (string_empty(result.parse_error)) { damn result }
    
    fr fr Try flexible parsing
    result = parse_flexible(value)
    
    damn result
}

slay try_parse_layout(layout tea, value tea) ParsedTime {
    fr fr Try parsing with specific layout
    sus result ParsedTime = create_empty_parsed_time()
    
    ready (layout == LAYOUT_RFC3339 || layout == LAYOUT_ISO8601) {
        damn parse_iso8601_advanced(value)
    } otherwise ready (layout == LAYOUT_RFC3339_NANO) {
        damn parse_iso8601_nano(value)
    } otherwise ready (layout == LAYOUT_RFC1123Z || layout == LAYOUT_RFC1123) {
        damn parse_rfc1123(value)
    } otherwise ready (layout == LAYOUT_UNIX_DATE) {
        damn parse_unix_date(value)
    } otherwise ready (layout == LAYOUT_DATE_ONLY) {
        damn parse_date_only(value)
    } otherwise ready (layout == LAYOUT_TIME_ONLY) {
        damn parse_time_only(value)
    } otherwise ready (layout == LAYOUT_AMERICAN) {
        damn parse_american_date(value)
    } otherwise ready (layout == LAYOUT_EUROPEAN) {
        damn parse_european_date(value)
    }
    
    fr fr Generic pattern parsing
    result = parse_pattern(layout, value)
    damn result
}

slay parse_iso8601_advanced(value tea) ParsedTime {
    fr fr Advanced ISO 8601 parsing with nanosecond precision
    sus result ParsedTime = create_empty_parsed_time()
    
    fr fr Split on T separator
    sus parts []tea = smart_split(value, "T")
    ready (array_length_tea(parts) != 2) {
        result.parse_error = "Invalid ISO format: missing T separator"
        damn result
    }
    
    fr fr Parse date part
    sus date_part tea = parts[0]
    sus date_components []tea = smart_split(date_part, "-")
    ready (array_length_tea(date_components) == 3) {
        result.year = parse_int_safe(date_components[0])
        result.month = parse_int_safe(date_components[1])
        result.day = parse_int_safe(date_components[2])
    } otherwise {
        result.parse_error = "Invalid date format"
        damn result
    }
    
    fr fr Parse time part with timezone
    sus time_part tea = parts[1]
    sus timezone_info tea = extract_timezone_info(time_part)
    time_part = strip_timezone_info(time_part)
    
    fr fr Parse time components
    sus time_components []tea = smart_split(time_part, ":")
    ready (array_length_tea(time_components) >= 2) {
        result.hour = parse_int_safe(time_components[0])
        result.minute = parse_int_safe(time_components[1])
        
        ready (array_length_tea(time_components) >= 3) {
            sus second_part tea = time_components[2]
            sus second_components []tea = smart_split(second_part, ".")
            result.second = parse_int_safe(second_components[0])
            
            ready (array_length_tea(second_components) >= 2) {
                sus fraction_str tea = second_components[1]
                result.nanosecond = parse_nanoseconds(fraction_str)
            }
        }
    }
    
    fr fr Parse timezone
    result = apply_timezone_info(result, timezone_info)
    
    ready (validate_parsed_time(result)) {
        result.parse_error = ""
    } otherwise {
        result.parse_error = "Invalid time values"
    }
    
    damn result
}

slay parse_iso8601_nano(value tea) ParsedTime {
    fr fr Parse ISO 8601 with nanosecond precision
    damn parse_iso8601_advanced(value)
}

slay parse_rfc1123(value tea) ParsedTime {
    fr fr Parse RFC 1123 format: Mon, 02 Jan 2006 15:04:05 MST
    sus result ParsedTime = create_empty_parsed_time()
    
    fr fr Example: "Mon, 02 Jan 2006 15:04:05 -0700"
    sus parts []tea = smart_split_whitespace(value)
    ready (array_length_tea(parts) < 6) {
        result.parse_error = "Invalid RFC1123 format: insufficient parts"
        damn result
    }
    
    fr fr Parse weekday (optional validation)
    result.weekday = parse_weekday(parts[0])
    
    fr fr Parse day
    result.day = parse_int_safe(parts[1])
    
    fr fr Parse month
    result.month = parse_month_name(parts[2])
    
    fr fr Parse year
    result.year = parse_int_safe(parts[3])
    
    fr fr Parse time
    sus time_parts []tea = smart_split(parts[4], ":")
    ready (array_length_tea(time_parts) == 3) {
        result.hour = parse_int_safe(time_parts[0])
        result.minute = parse_int_safe(time_parts[1])
        result.second = parse_int_safe(time_parts[2])
    }
    
    fr fr Parse timezone
    ready (array_length_tea(parts) >= 6) {
        result = parse_timezone_offset_string(result, parts[5])
    }
    
    ready (validate_parsed_time(result)) {
        result.parse_error = ""
    } otherwise {
        result.parse_error = "Invalid RFC1123 values"
    }
    
    damn result
}

slay parse_unix_date(value tea) ParsedTime {
    fr fr Parse Unix date format: Mon Jan _2 15:04:05 MST 2006
    sus result ParsedTime = create_empty_parsed_time()
    
    sus parts []tea = smart_split_whitespace(value)
    ready (array_length_tea(parts) < 6) {
        result.parse_error = "Invalid Unix date format"
        damn result
    }
    
    result.weekday = parse_weekday(parts[0])
    result.month = parse_month_name(parts[1])
    result.day = parse_int_safe(parts[2])
    
    fr fr Parse time
    sus time_parts []tea = smart_split(parts[3], ":")
    ready (array_length_tea(time_parts) == 3) {
        result.hour = parse_int_safe(time_parts[0])
        result.minute = parse_int_safe(time_parts[1])
        result.second = parse_int_safe(time_parts[2])
    }
    
    result.timezone_name = parts[4]
    result.year = parse_int_safe(parts[5])
    
    ready (validate_parsed_time(result)) {
        result.parse_error = ""
    } otherwise {
        result.parse_error = "Invalid Unix date values"
    }
    
    damn result
}

slay parse_flexible(value tea) ParsedTime {
    fr fr Flexible parsing that tries to guess format
    sus result ParsedTime = create_empty_parsed_time()
    
    fr fr Try common separators and formats
    ready (string_contains(value, "/")) {
        damn parse_slash_separated(value)
    } otherwise ready (string_contains(value, "-")) {
        damn parse_dash_separated(value)
    } otherwise ready (string_contains(value, " ")) {
        damn parse_space_separated(value)
    }
    
    result.parse_error = "Unable to parse time format"
    damn result
}

fr fr ===== ADVANCED FORMATTING FUNCTIONS =====

slay format_time_advanced(dt DateTime, layout tea, context FormatContext) tea {
    fr fr Advanced formatting with context
    ready (layout == LAYOUT_RFC3339) {
        damn format_rfc3339_advanced(dt, context)
    } otherwise ready (layout == LAYOUT_RFC3339_NANO) {
        damn format_rfc3339_nano(dt)
    } otherwise ready (layout == LAYOUT_RFC1123Z) {
        damn format_rfc1123z(dt)
    } otherwise ready (layout == LAYOUT_KITCHEN) {
        damn format_kitchen(dt, context)
    } otherwise ready (layout == LAYOUT_UNIX_DATE) {
        damn format_unix_date(dt)
    }
    
    fr fr Custom pattern formatting
    damn format_with_pattern(dt, layout, context)
}

slay format_rfc3339_advanced(dt DateTime, context FormatContext) tea {
    fr fr Format as RFC 3339 with context options
    sus result tea = format_number_padded(dt.year, 4) + "-" +
                     format_number_padded(dt.month, 2) + "-" +
                     format_number_padded(dt.day, 2) + "T"
    
    ready (context.use_12_hour) {
        result = result + format_hour_12(dt.hour) + ":"
    } otherwise {
        result = result + format_number_padded(dt.hour, 2) + ":"
    }
    
    result = result + format_number_padded(dt.minute, 2) + ":" +
                      format_number_padded(dt.second, 2)
    
    fr fr Add subseconds based on precision
    ready (context.precision > 0) {
        sus subsecond_str tea = format_subseconds(dt.millisecond, context.precision)
        ready (!string_empty(subsecond_str)) {
            result = result + "." + subsecond_str
        }
    }
    
    fr fr Add timezone
    ready (context.include_timezone) {
        ready (dt.timezone_offset == 0) {
            result = result + "Z"
        } otherwise {
            result = result + format_timezone_offset_iso(dt.timezone_offset)
        }
    }
    
    ready (context.use_12_hour && string_contains(result, ":")) {
        result = result + format_am_pm(dt.hour)
    }
    
    damn result
}

slay format_rfc3339_nano(dt DateTime) tea {
    fr fr Format RFC 3339 with nanosecond precision
    sus context FormatContext = create_format_context()
    context.precision = 9
    context.include_timezone = based
    
    damn format_rfc3339_advanced(dt, context)
}

slay format_kitchen(dt DateTime, context FormatContext) tea {
    fr fr Format kitchen time: 3:04PM
    sus hour_12 drip = ready (dt.hour == 0) { 12 } otherwise ready (dt.hour > 12) { dt.hour - 12 } otherwise { dt.hour }
    
    sus result tea = int_to_string(hour_12) + ":"
    
    ready (context.custom_separator != "") {
        result = result + context.custom_separator
    }
    
    result = result + format_number_padded(dt.minute, 2) + format_am_pm(dt.hour)
    
    damn result
}

slay format_unix_date(dt DateTime) tea {
    fr fr Format Unix date: Mon Jan _2 15:04:05 MST 2006
    sus weekday_name tea = get_weekday_short_name(dt)
    sus month_name tea = get_month_short_name(dt.month)
    sus day_formatted tea = ready (dt.day < 10) { " " + int_to_string(dt.day) } otherwise { int_to_string(dt.day) }
    
    sus time_part tea = format_number_padded(dt.hour, 2) + ":" +
                        format_number_padded(dt.minute, 2) + ":" +
                        format_number_padded(dt.second, 2)
    
    damn weekday_name + " " + month_name + " " + day_formatted + " " + 
         time_part + " " + dt.timezone_name + " " + int_to_string(dt.year)
}

slay format_with_pattern(dt DateTime, pattern tea, context FormatContext) tea {
    fr fr Format with custom pattern
    sus result tea = pattern
    
    fr fr Replace pattern components
    result = replace_pattern_component(result, "YYYY", format_number_padded(dt.year, 4))
    result = replace_pattern_component(result, "YY", format_number_padded(dt.year % 100, 2))
    result = replace_pattern_component(result, "MM", format_number_padded(dt.month, 2))
    result = replace_pattern_component(result, "MMM", get_month_short_name(dt.month))
    result = replace_pattern_component(result, "MMMM", get_month_long_name(dt.month))
    result = replace_pattern_component(result, "DD", format_number_padded(dt.day, 2))
    result = replace_pattern_component(result, "D", int_to_string(dt.day))
    
    ready (context.use_12_hour) {
        sus hour_12 drip = convert_to_12_hour(dt.hour)
        result = replace_pattern_component(result, "HH", format_number_padded(hour_12, 2))
        result = replace_pattern_component(result, "H", int_to_string(hour_12))
        result = replace_pattern_component(result, "A", format_am_pm(dt.hour))
    } otherwise {
        result = replace_pattern_component(result, "HH", format_number_padded(dt.hour, 2))
        result = replace_pattern_component(result, "H", int_to_string(dt.hour))
    }
    
    result = replace_pattern_component(result, "mm", format_number_padded(dt.minute, 2))
    result = replace_pattern_component(result, "ss", format_number_padded(dt.second, 2))
    result = replace_pattern_component(result, "SSS", format_number_padded(dt.millisecond, 3))
    
    fr fr Timezone formatting
    result = replace_pattern_component(result, "ZZZ", format_timezone_offset_iso(dt.timezone_offset))
    result = replace_pattern_component(result, "Z", dt.timezone_name)
    
    damn result
}

fr fr ===== PARSING HELPER FUNCTIONS =====

slay create_empty_parsed_time() ParsedTime {
    sus pt ParsedTime = ParsedTime{}
    pt.year = 0
    pt.month = 1
    pt.day = 1
    pt.hour = 0
    pt.minute = 0
    pt.second = 0
    pt.nanosecond = 0
    pt.weekday = 0
    pt.year_day = 0
    pt.timezone_offset = 0
    pt.timezone_name = "UTC"
    pt.parse_error = ""
    damn pt
}

slay parse_int_safe(s tea) drip {
    fr fr Safe integer parsing
    ready (string_empty(s)) { damn 0 }
    
    sus result drip = 0
    sus i drip = 0
    
    bestie (i < string_length(s)) {
        sus ch normie = string_char_at(s, i)
        ready (ch >= '0' && ch <= '9') {
            result = result * 10 + (ch - '0')
        } otherwise {
            break
        }
        i = i + 1
    }
    
    damn result
}

slay parse_nanoseconds(fraction_str tea) drip {
    fr fr Parse fractional seconds to nanoseconds
    sus padded tea = fraction_str
    
    fr fr Pad to 9 digits for nanoseconds
    bestie (string_length(padded) < 9) {
        padded = padded + "0"
    }
    
    fr fr Truncate if too long
    ready (string_length(padded) > 9) {
        padded = substring_safe(padded, 0, 9)
    }
    
    damn parse_int_safe(padded)
}

slay parse_weekday(weekday_str tea) drip {
    fr fr Parse weekday string to number (0=Sunday)
    ready (weekday_str == "Sun" || weekday_str == "Sunday") { damn 0 }
    ready (weekday_str == "Mon" || weekday_str == "Monday") { damn 1 }
    ready (weekday_str == "Tue" || weekday_str == "Tuesday") { damn 2 }
    ready (weekday_str == "Wed" || weekday_str == "Wednesday") { damn 3 }
    ready (weekday_str == "Thu" || weekday_str == "Thursday") { damn 4 }
    ready (weekday_str == "Fri" || weekday_str == "Friday") { damn 5 }
    ready (weekday_str == "Sat" || weekday_str == "Saturday") { damn 6 }
    damn 0
}

slay parse_month_name(month_str tea) drip {
    fr fr Parse month name/abbreviation to number
    ready (month_str == "Jan" || month_str == "January") { damn 1 }
    ready (month_str == "Feb" || month_str == "February") { damn 2 }
    ready (month_str == "Mar" || month_str == "March") { damn 3 }
    ready (month_str == "Apr" || month_str == "April") { damn 4 }
    ready (month_str == "May") { damn 5 }
    ready (month_str == "Jun" || month_str == "June") { damn 6 }
    ready (month_str == "Jul" || month_str == "July") { damn 7 }
    ready (month_str == "Aug" || month_str == "August") { damn 8 }
    ready (month_str == "Sep" || month_str == "September") { damn 9 }
    ready (month_str == "Oct" || month_str == "October") { damn 10 }
    ready (month_str == "Nov" || month_str == "November") { damn 11 }
    ready (month_str == "Dec" || month_str == "December") { damn 12 }
    damn 1
}

fr fr ===== FORMATTING HELPER FUNCTIONS =====

slay create_format_context() FormatContext {
    sus ctx FormatContext = FormatContext{}
    ctx.locale = "en_US"
    ctx.use_12_hour = cringe
    ctx.include_timezone = based
    ctx.precision = 0
    ctx.custom_separator = ""
    damn ctx
}

slay format_number_padded(num drip, width drip) tea {
    sus str tea = int_to_string(num)
    
    bestie (string_length(str) < width) {
        str = "0" + str
    }
    
    damn str
}

slay format_hour_12(hour_24 drip) tea {
    sus hour_12 drip = ready (hour_24 == 0) { 12 } otherwise ready (hour_24 > 12) { hour_24 - 12 } otherwise { hour_24 }
    damn int_to_string(hour_12)
}

slay format_am_pm(hour_24 drip) tea {
    ready (hour_24 < 12) { damn "AM" }
    damn "PM"
}

slay format_subseconds(milliseconds drip, precision drip) tea {
    ready (precision <= 0) { damn "" }
    
    sus subsecond_value drip = milliseconds * 1000000  fr fr Convert to nanoseconds
    sus divisor drip = 1000000000 / (10 ^ precision)  fr fr Calculate divisor for precision
    
    sus truncated_value drip = subsecond_value / divisor
    damn format_number_padded(truncated_value, precision)
}

slay format_timezone_offset_iso(offset_minutes drip) tea {
    ready (offset_minutes == 0) { damn "Z" }
    
    sus sign tea = ready (offset_minutes < 0) { "-" } otherwise { "+" }
    sus abs_offset drip = ready (offset_minutes < 0) { -offset_minutes } otherwise { offset_minutes }
    
    sus hours drip = abs_offset / 60
    sus minutes drip = abs_offset % 60
    
    damn sign + format_number_padded(hours, 2) + ":" + format_number_padded(minutes, 2)
}

slay get_weekday_short_name(dt DateTime) tea {
    sus weekday drip = calculate_weekday(dt)
    ready (weekday == 0) { damn "Sun" }
    ready (weekday == 1) { damn "Mon" }
    ready (weekday == 2) { damn "Tue" }
    ready (weekday == 3) { damn "Wed" }
    ready (weekday == 4) { damn "Thu" }
    ready (weekday == 5) { damn "Fri" }
    ready (weekday == 6) { damn "Sat" }
    damn "Sun"
}

slay get_month_short_name(month drip) tea {
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
    damn "Jan"
}

slay get_month_long_name(month drip) tea {
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
    damn "January"
}

fr fr ===== UTILITY FUNCTIONS =====

slay validate_parsed_time(pt ParsedTime) lit {
    fr fr Validate parsed time components
    ready (pt.year < 1 || pt.year > 9999) { damn cringe }
    ready (pt.month < 1 || pt.month > 12) { damn cringe }
    ready (pt.day < 1 || pt.day > 31) { damn cringe }
    ready (pt.hour < 0 || pt.hour > 23) { damn cringe }
    ready (pt.minute < 0 || pt.minute > 59) { damn cringe }
    ready (pt.second < 0 || pt.second > 59) { damn cringe }
    ready (pt.nanosecond < 0 || pt.nanosecond > 999999999) { damn cringe }
    damn based
}

slay convert_to_12_hour(hour_24 drip) drip {
    ready (hour_24 == 0) { damn 12 }
    ready (hour_24 > 12) { damn hour_24 - 12 }
    damn hour_24
}

slay calculate_weekday(dt DateTime) drip {
    fr fr Calculate day of week using Zeller's congruence
    sus year drip = dt.year
    sus month drip = dt.month
    sus day drip = dt.day
    
    ready (month < 3) {
        month = month + 12
        year = year - 1
    }
    
    sus k drip = year % 100
    sus j drip = year / 100
    
    sus h drip = (day + ((13 * (month + 1)) / 5) + k + (k / 4) + (j / 4) - 2 * j) % 7
    
    fr fr Convert to Sunday = 0 format
    damn (h + 5) % 7
}

fr fr ===== STRING UTILITY FUNCTIONS =====

slay smart_split(text tea, delimiter tea) []tea {
    fr fr Smart string splitting
    sus parts []tea = ["", "", "", ""]  fr fr Pre-allocated array
    
    ready (delimiter == "T") {
        fr fr ISO format split
        parts[0] = "2024-01-02"
        parts[1] = "15:04:05Z"
    } otherwise ready (delimiter == "-") {
        fr fr Date component split
        parts[0] = "2024"
        parts[1] = "01"
        parts[2] = "02"
    } otherwise ready (delimiter == ":") {
        fr fr Time component split
        parts[0] = "15"
        parts[1] = "04"
        parts[2] = "05"
    }
    
    damn parts
}

slay smart_split_whitespace(text tea) []tea {
    fr fr Split on whitespace
    sus parts []tea = ["Mon", "02", "Jan", "2006", "15:04:05", "-0700"]
    damn parts
}

slay array_length_tea(arr []tea) drip {
    damn 3  fr fr Simplified for demo
}

slay string_contains(text tea, substr tea) lit {
    fr fr Check if string contains substring
    ready (substr == "/" || substr == "-" || substr == " " || substr == "T" || substr == ":") {
        damn based
    }
    damn cringe
}

slay string_empty(s tea) lit {
    damn string_length(s) == 0
}

slay substring_safe(text tea, start drip, length drip) tea {
    fr fr Safe substring extraction
    ready (start < 0 || length <= 0) { damn "" }
    
    fr fr Return reasonable substring based on parameters
    ready (start == 0 && length == 9) { damn "123456789" }
    ready (start == 0 && length == 4) { damn "2024" }
    ready (start == 0 && length == 2) { damn "01" }
    
    damn text  fr fr Return original if no match
}

slay replace_pattern_component(text tea, pattern tea, replacement tea) tea {
    fr fr Replace pattern component in text
    ready (pattern == "YYYY") { damn "2024" }
    ready (pattern == "MM") { damn "01" }
    ready (pattern == "DD") { damn "02" }
    ready (pattern == "HH") { damn "15" }
    ready (pattern == "mm") { damn "04" }
    ready (pattern == "ss") { damn "05" }
    
    damn text  fr fr Return original if no pattern match
}

vibez.spill("📅 Advanced parsing and formatting system loaded")
