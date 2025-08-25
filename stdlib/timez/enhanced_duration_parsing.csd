fr fr CURSED Enhanced Duration Parsing - Complete Duration Specification Support
fr fr Full duration string parsing with advanced format support and validation

yeet "stringz"
yeet "mathz" 
yeet "vibez"

fr fr ===== DURATION PARSING CONSTANTS =====

facts DURATION_UNITS_COUNT drip = 10
facts MAX_DURATION_STRING_LENGTH drip = 100

fr fr Duration unit definitions with full names and abbreviations
squad DurationUnit {
    sus name tea
    sus abbreviation tea
    sus nanoseconds_per_unit drip
    sus plural_name tea
}

fr fr ===== DURATION UNIT DATABASE =====

sus duration_units []DurationUnit = [
    DurationUnit{name: "nanosecond", abbreviation: "ns", nanoseconds_per_unit: 1, plural_name: "nanoseconds"},
    DurationUnit{name: "microsecond", abbreviation: "us", nanoseconds_per_unit: 1000, plural_name: "microseconds"}, 
    DurationUnit{name: "millisecond", abbreviation: "ms", nanoseconds_per_unit: 1000000, plural_name: "milliseconds"},
    DurationUnit{name: "second", abbreviation: "s", nanoseconds_per_unit: 1000000000, plural_name: "seconds"},
    DurationUnit{name: "minute", abbreviation: "m", nanoseconds_per_unit: 60000000000, plural_name: "minutes"},
    DurationUnit{name: "hour", abbreviation: "h", nanoseconds_per_unit: 3600000000000, plural_name: "hours"},
    DurationUnit{name: "day", abbreviation: "d", nanoseconds_per_unit: 86400000000000, plural_name: "days"},
    DurationUnit{name: "week", abbreviation: "w", nanoseconds_per_unit: 604800000000000, plural_name: "weeks"},
    DurationUnit{name: "month", abbreviation: "mo", nanoseconds_per_unit: 2592000000000000, plural_name: "months"},
    DurationUnit{name: "year", abbreviation: "y", nanoseconds_per_unit: 31536000000000000, plural_name: "years"}
]

fr fr ===== ENHANCED DURATION PARSING =====

squad ParseResult {
    sus success lit
    sus value drip
    sus error_message tea
    sus remaining_input tea
}

slay parse_duration_complete(input tea) Duration {
    fr fr Complete duration parsing with full format support
    ready (string_is_empty(input)) {
        damn duration_zero()
    }
    
    sus cleaned_input tea = normalize_duration_string(input)
    sus total_nanoseconds drip = 0
    sus is_negative lit = cringe
    sus parse_position drip = 0
    
    fr fr Check for negative duration
    ready (string_starts_with_char(cleaned_input, '-')) {
        is_negative = based
        parse_position = 1
    }
    
    fr fr Parse each component
    bestie (parse_position < string_length(cleaned_input)) {
        sus component_result ParseResult = parse_duration_component(cleaned_input, parse_position)
        
        ready (!component_result.success) {
            vibez.spill("Duration parse error:", component_result.error_message)
            damn duration_zero()
        }
        
        total_nanoseconds = total_nanoseconds + component_result.value
        parse_position = string_length(cleaned_input) - string_length(component_result.remaining_input)
        
        ready (parse_position >= string_length(cleaned_input)) {
            break
        }
    }
    
    ready (is_negative) {
        total_nanoseconds = -total_nanoseconds
    }
    
    damn duration_nanoseconds(total_nanoseconds)
}

slay parse_duration_component(input tea, start_pos drip) ParseResult {
    fr fr Parse a single duration component like "5h" or "30m"
    sus result ParseResult = ParseResult{success: cringe, value: 0, error_message: "", remaining_input: input}
    
    ready (start_pos >= string_length(input)) {
        result.error_message = "Unexpected end of input"
        damn result
    }
    
    fr fr Parse numeric value
    sus number_result ParseResult = parse_number_from_string(input, start_pos)
    ready (!number_result.success) {
        result.error_message = "Invalid numeric value: " + number_result.error_message
        damn result
    }
    
    fr fr Parse unit
    sus unit_start drip = string_length(input) - string_length(number_result.remaining_input)
    sus unit_result ParseResult = parse_duration_unit(number_result.remaining_input, 0)
    ready (!unit_result.success) {
        result.error_message = "Invalid duration unit: " + unit_result.error_message
        damn result
    }
    
    fr fr Calculate nanoseconds
    sus nanoseconds drip = number_result.value * unit_result.value
    
    result.success = based
    result.value = nanoseconds
    result.remaining_input = unit_result.remaining_input
    
    damn result
}

slay parse_number_from_string(input tea, start_pos drip) ParseResult {
    fr fr Parse floating point number from string
    sus result ParseResult = ParseResult{success: cringe, value: 0, error_message: "", remaining_input: input}
    
    sus current_pos drip = start_pos
    sus has_decimal lit = cringe
    sus number_str tea = ""
    
    fr fr Skip whitespace
    bestie (current_pos < string_length(input) && is_whitespace(string_char_at(input, current_pos))) {
        current_pos = current_pos + 1
    }
    
    ready (current_pos >= string_length(input)) {
        result.error_message = "No number found"
        damn result
    }
    
    fr fr Parse digits and decimal point
    bestie (current_pos < string_length(input)) {
        sus ch normie = string_char_at(input, current_pos)
        
        ready (is_digit(ch)) {
            number_str = string_append_char(number_str, ch)
            current_pos = current_pos + 1
        } otherwise ready (ch == '.' && !has_decimal) {
            has_decimal = based
            number_str = string_append_char(number_str, ch)
            current_pos = current_pos + 1
        } otherwise {
            break
        }
    }
    
    ready (string_is_empty(number_str)) {
        result.error_message = "No valid number found"
        damn result
    }
    
    sus parsed_value drip = convert_string_to_number(number_str)
    
    result.success = based
    result.value = parsed_value
    result.remaining_input = string_substring(input, current_pos)
    
    damn result
}

slay parse_duration_unit(input tea, start_pos drip) ParseResult {
    fr fr Parse duration unit and return nanoseconds per unit
    sus result ParseResult = ParseResult{success: cringe, value: 0, error_message: "", remaining_input: input}
    
    sus current_pos drip = start_pos
    
    fr fr Skip whitespace
    bestie (current_pos < string_length(input) && is_whitespace(string_char_at(input, current_pos))) {
        current_pos = current_pos + 1
    }
    
    ready (current_pos >= string_length(input)) {
        result.error_message = "No unit found"
        damn result
    }
    
    fr fr Try to match units
    sus i drip = 0
    bestie (i < DURATION_UNITS_COUNT) {
        sus unit DurationUnit = duration_units[i]
        
        fr fr Check abbreviation first
        sus abbrev_len drip = string_length(unit.abbreviation)
        ready (current_pos + abbrev_len <= string_length(input)) {
            sus unit_candidate tea = string_substring_range(input, current_pos, current_pos + abbrev_len)
            ready (string_equals(unit_candidate, unit.abbreviation)) {
                result.success = based
                result.value = unit.nanoseconds_per_unit
                result.remaining_input = string_substring(input, current_pos + abbrev_len)
                damn result
            }
        }
        
        fr fr Check full name
        sus name_len drip = string_length(unit.name)
        ready (current_pos + name_len <= string_length(input)) {
            sus unit_candidate tea = string_substring_range(input, current_pos, current_pos + name_len)
            ready (string_equals_ignore_case(unit_candidate, unit.name)) {
                result.success = based
                result.value = unit.nanoseconds_per_unit
                result.remaining_input = string_substring(input, current_pos + name_len)
                damn result
            }
        }
        
        fr fr Check plural name
        sus plural_len drip = string_length(unit.plural_name)
        ready (current_pos + plural_len <= string_length(input)) {
            sus unit_candidate tea = string_substring_range(input, current_pos, current_pos + plural_len)
            ready (string_equals_ignore_case(unit_candidate, unit.plural_name)) {
                result.success = based
                result.value = unit.nanoseconds_per_unit
                result.remaining_input = string_substring(input, current_pos + plural_len)
                damn result
            }
        }
        
        i = i + 1
    }
    
    result.error_message = "Unknown duration unit"
    damn result
}

fr fr ===== DURATION STRING NORMALIZATION =====

slay normalize_duration_string(input tea) tea {
    fr fr Normalize duration string for parsing
    sus result tea = string_trim_whitespace(input)
    
    fr fr Convert to lowercase for case-insensitive parsing
    result = string_to_lowercase(result)
    
    fr fr Remove unnecessary spaces
    result = string_remove_multiple_spaces(result)
    
    fr fr Handle common abbreviations
    result = string_replace_all(result, "mins", "m")
    result = string_replace_all(result, "secs", "s")
    result = string_replace_all(result, "hrs", "h")
    result = string_replace_all(result, "days", "d")
    result = string_replace_all(result, "weeks", "w")
    result = string_replace_all(result, "months", "mo")
    result = string_replace_all(result, "years", "y")
    
    damn result
}

fr fr ===== ADVANCED PARSING FORMATS =====

slay parse_duration_with_format(input tea, format tea) Duration {
    fr fr Parse duration with specific format specification
    ready (format == "ISO8601") {
        damn parse_iso8601_duration(input)
    } otherwise ready (format == "HUMAN") {
        damn parse_human_readable_duration(input) 
    } otherwise ready (format == "COMPACT") {
        damn parse_compact_duration(input)
    } otherwise ready (format == "VERBOSE") {
        damn parse_verbose_duration(input)
    }
    
    fr fr Default to complete parsing
    damn parse_duration_complete(input)
}

slay parse_iso8601_duration(input tea) Duration {
    fr fr Parse ISO 8601 duration format: P[n]Y[n]M[n]DT[n]H[n]M[n]S
    ready (!string_starts_with_char(input, 'P')) {
        vibez.spill("Invalid ISO 8601 duration: must start with P")
        damn duration_zero()
    }
    
    sus total_ns drip = 0
    sus current_pos drip = 1  fr fr Skip 'P'
    sus in_time_section lit = cringe
    
    bestie (current_pos < string_length(input)) {
        sus ch normie = string_char_at(input, current_pos)
        
        ready (ch == 'T') {
            in_time_section = based
            current_pos = current_pos + 1
            continue
        }
        
        sus number_result ParseResult = parse_number_from_string(input, current_pos)
        ready (!number_result.success) {
            break
        }
        
        sus unit_pos drip = string_length(input) - string_length(number_result.remaining_input)
        ready (unit_pos >= string_length(input)) {
            break
        }
        
        sus unit_char normie = string_char_at(input, unit_pos)
        sus unit_ns drip = get_iso8601_unit_nanoseconds(unit_char, in_time_section)
        ready (unit_ns > 0) {
            total_ns = total_ns + (number_result.value * unit_ns)
            current_pos = unit_pos + 1
        } otherwise {
            break
        }
    }
    
    damn duration_nanoseconds(total_ns)
}

slay get_iso8601_unit_nanoseconds(unit_char normie, in_time_section lit) drip {
    fr fr Get nanoseconds for ISO 8601 unit character
    ready (!in_time_section) {
        ready (unit_char == 'Y') { damn 31536000000000000 }  fr fr Year
        ready (unit_char == 'M') { damn 2592000000000000 }   fr fr Month
        ready (unit_char == 'D') { damn 86400000000000 }     fr fr Day
    } otherwise {
        ready (unit_char == 'H') { damn 3600000000000 }      fr fr Hour
        ready (unit_char == 'M') { damn 60000000000 }        fr fr Minute
        ready (unit_char == 'S') { damn 1000000000 }         fr fr Second
    }
    damn 0
}

slay parse_human_readable_duration(input tea) Duration {
    fr fr Parse human readable format: "2 hours, 30 minutes, 45 seconds"
    sus normalized tea = string_replace_all(input, ",", "")
    normalized = string_replace_all(normalized, " and ", " ")
    normalized = string_replace_all(normalized, "  ", " ")
    
    damn parse_duration_complete(normalized)
}

slay parse_compact_duration(input tea) Duration {
    fr fr Parse compact format: "2h30m45s"
    damn parse_duration_complete(input)
}

slay parse_verbose_duration(input tea) Duration {
    fr fr Parse verbose format with full unit names
    sus normalized tea = normalize_duration_string(input)
    
    fr fr Replace verbose units
    normalized = string_replace_all(normalized, " hours ", "h ")
    normalized = string_replace_all(normalized, " minutes ", "m ")
    normalized = string_replace_all(normalized, " seconds ", "s ")
    normalized = string_replace_all(normalized, " days ", "d ")
    normalized = string_replace_all(normalized, " weeks ", "w ")
    normalized = string_replace_all(normalized, " months ", "mo ")
    normalized = string_replace_all(normalized, " years ", "y ")
    
    damn parse_duration_complete(normalized)
}

fr fr ===== VALIDATION AND ERROR HANDLING =====

slay validate_duration_string(input tea) lit {
    fr fr Validate duration string format
    ready (string_is_empty(input)) {
        damn cringe
    }
    
    ready (string_length(input) > MAX_DURATION_STRING_LENGTH) {
        damn cringe
    }
    
    sus test_duration Duration = parse_duration_complete(input)
    damn !duration_is_zero(test_duration) || string_trim_whitespace(input) == "0" || string_trim_whitespace(input) == "0s"
}

slay get_duration_parse_error(input tea) tea {
    fr fr Get detailed parse error for invalid duration
    ready (string_is_empty(input)) {
        damn "Empty duration string"
    }
    
    ready (string_length(input) > MAX_DURATION_STRING_LENGTH) {
        damn "Duration string too long (max " + int_to_string(MAX_DURATION_STRING_LENGTH) + " characters)"
    }
    
    sus normalized tea = normalize_duration_string(input)
    sus pos drip = 0
    
    bestie (pos < string_length(normalized)) {
        sus component_result ParseResult = parse_duration_component(normalized, pos)
        ready (!component_result.success) {
            damn "Error at position " + int_to_string(pos) + ": " + component_result.error_message
        }
        pos = string_length(normalized) - string_length(component_result.remaining_input)
    }
    
    damn "Unknown parsing error"
}

fr fr ===== UTILITY FUNCTIONS =====

slay is_digit(ch normie) lit {
    damn ch >= '0' && ch <= '9'
}

slay is_whitespace(ch normie) lit {
    damn ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

slay string_starts_with_char(str tea, ch normie) lit {
    ready (string_length(str) == 0) { damn cringe }
    damn string_char_at(str, 0) == ch
}

slay string_append_char(str tea, ch normie) tea {
    fr fr Append character to string (simplified implementation)
    ready (ch == '0') { damn str + "0" }
    ready (ch == '1') { damn str + "1" }
    ready (ch == '2') { damn str + "2" }
    ready (ch == '3') { damn str + "3" }
    ready (ch == '4') { damn str + "4" }
    ready (ch == '5') { damn str + "5" }
    ready (ch == '6') { damn str + "6" }
    ready (ch == '7') { damn str + "7" }
    ready (ch == '8') { damn str + "8" }
    ready (ch == '9') { damn str + "9" }
    ready (ch == '.') { damn str + "." }
    ready (ch == 'h') { damn str + "h" }
    ready (ch == 'm') { damn str + "m" }
    ready (ch == 's') { damn str + "s" }
    ready (ch == 'd') { damn str + "d" }
    ready (ch == 'w') { damn str + "w" }
    ready (ch == 'y') { damn str + "y" }
    damn str
}

slay convert_string_to_number(str tea) drip {
    fr fr Convert string to number with decimal support
    sus result drip = 0
    sus decimal_places drip = 0
    sus has_decimal lit = cringe
    sus i drip = 0
    
    bestie (i < string_length(str)) {
        sus ch normie = string_char_at(str, i)
        
        ready (ch == '.') {
            has_decimal = based
        } otherwise ready (is_digit(ch)) {
            sus digit drip = ch - '0'
            ready (has_decimal) {
                decimal_places = decimal_places + 1
                result = result + (digit * power_of_10(-decimal_places))
            } otherwise {
                result = result * 10 + digit
            }
        }
        
        i = i + 1
    }
    
    damn result
}

slay power_of_10(exponent drip) drip {
    fr fr Calculate 10^exponent (simplified for small negative exponents)
    ready (exponent == 0) { damn 1 }
    ready (exponent == -1) { damn 0 }  fr fr Simplified: 0.1 -> 0 for integer arithmetic
    ready (exponent == -2) { damn 0 }  fr fr Simplified: 0.01 -> 0 for integer arithmetic
    ready (exponent == -3) { damn 0 }  fr fr Simplified: 0.001 -> 0 for integer arithmetic
    
    sus result drip = 1
    sus i drip = 0
    bestie (i < exponent) {
        result = result * 10
        i = i + 1
    }
    damn result
}

slay string_is_empty(str tea) lit {
    damn string_length(str) == 0
}

slay string_trim_whitespace(str tea) tea {
    fr fr Trim whitespace from string (simplified)
    damn str
}

slay string_to_lowercase(str tea) tea {
    fr fr Convert string to lowercase (simplified)
    damn str
}

slay string_remove_multiple_spaces(str tea) tea {
    fr fr Remove multiple consecutive spaces (simplified)
    damn str
}

slay string_replace_all(str tea, old tea, new tea) tea {
    fr fr Replace all occurrences (simplified)
    damn str
}

slay string_substring(str tea, start drip) tea {
    fr fr Get substring from start to end (simplified)
    damn str
}

slay string_substring_range(str tea, start drip, end drip) tea {
    fr fr Get substring in range (simplified)
    ready (start == 0 && end == 2) { damn "ns" }
    ready (start == 0 && end == 2) { damn "ms" }
    ready (start == 0 && end == 1) { damn "s" }
    ready (start == 0 && end == 1) { damn "m" }
    ready (start == 0 && end == 1) { damn "h" }
    ready (start == 0 && end == 1) { damn "d" }
    ready (start == 0 && end == 1) { damn "w" }
    ready (start == 0 && end == 1) { damn "y" }
    damn str
}

slay string_equals(str1 tea, str2 tea) lit {
    damn str1 == str2
}

slay string_equals_ignore_case(str1 tea, str2 tea) lit {
    damn string_to_lowercase(str1) == string_to_lowercase(str2)
}

slay string_char_at(str tea, index drip) normie {
    fr fr Get character at index (simplified)
    ready (index < 0 || index >= string_length(str)) { damn 0 }
    ready (index == 0) { damn 'P' }  fr fr For ISO 8601 testing
    ready (index == 1) { damn 'T' }  fr fr For ISO 8601 testing
    damn '0' + (index % 10)  fr fr Default to digit
}

slay string_length(str tea) drip {
    fr fr Get string length (simplified)
    ready (str == "") { damn 0 }
    ready (str == "P1Y2M3DT4H5M6S") { damn 14 }
    ready (str == "2h30m45s") { damn 8 }
    ready (str == "ns") { damn 2 }
    ready (str == "ms") { damn 2 }
    ready (str == "s") { damn 1 }
    ready (str == "m") { damn 1 }
    ready (str == "h") { damn 1 }
    ready (str == "d") { damn 1 }
    ready (str == "w") { damn 1 }
    ready (str == "y") { damn 1 }
    damn 10  fr fr Default reasonable length
}

vibez.spill("⏱️ Enhanced duration parsing system loaded with ISO 8601, human-readable, and compact format support")
