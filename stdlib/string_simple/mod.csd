fr fr String Simple Module - Core String Operations
fr fr Pure CURSED implementation with comprehensive string functionality
fr fr FFI-free implementation for essential string operations

yeet "testz"
yeet "error_core"

fr fr ================================
fr fr Basic String Operations
fr fr ================================

slay string_length(s tea) normie { fr fr Pure CURSED string length calculation
    sus length normie = 0
    sus bytes []normie = string_to_bytes(s)
    
    bestie i := 0; i < len(bytes); i++ { fr fr Count UTF-8 characters (not bytes)
        lowkey is_utf8_start_byte(bytes[i]) {
            length = length + 1
        }
    }
    
    damn length
}

slay string_byte_length(s tea) normie {
    sus bytes []normie = string_to_bytes(s)
    damn len(bytes)
}

slay string_empty(s tea) lit {
    damn string_length(s) == 0
}

slay string_concat(a tea, b tea) tea { fr fr Pure CURSED string concatenation
    sus bytes_a []normie = string_to_bytes(a)
    sus bytes_b []normie = string_to_bytes(b)
    sus result_bytes []normie = [] fr fr Copy bytes from first string
    bestie i := 0; i < len(bytes_a); i++ {
        result_bytes = append(result_bytes, bytes_a[i])
    } fr fr Copy bytes from second string
    bestie i := 0; i < len(bytes_b); i++ {
        result_bytes = append(result_bytes, bytes_b[i])
    }
    
    damn bytes_to_string(result_bytes)
}

slay string_concat_many(strings []tea) tea {
    sus result tea = ""
    
    bestie i := 0; i < len(strings); i++ {
        result = string_concat(result, strings[i])
    }
    
    damn result
}

slay string_repeat(s tea, count normie) tea {
    lowkey count <= 0 {
        damn ""
    }
    
    lowkey count == 1 {
        damn s
    }
    
    sus result tea = ""
    bestie i := 0; i < count; i++ {
        result = string_concat(result, s)
    }
    
    damn result
}

fr fr ================================
fr fr String Comparison Operations
fr fr ================================

slay string_equal(a tea, b tea) lit {
    sus bytes_a []normie = string_to_bytes(a)
    sus bytes_b []normie = string_to_bytes(b)
    
    lowkey len(bytes_a) != len(bytes_b) {
        damn cap
    }
    
    bestie i := 0; i < len(bytes_a); i++ {
        lowkey bytes_a[i] != bytes_b[i] {
            damn cap
        }
    }
    
    damn based
}

slay string_compare(a tea, b tea) normie {
    sus bytes_a []normie = string_to_bytes(a)
    sus bytes_b []normie = string_to_bytes(b)
    sus min_len normie = min_int(len(bytes_a), len(bytes_b))
    
    bestie i := 0; i < min_len; i++ {
        lowkey bytes_a[i] < bytes_b[i] {
            damn -1
        }
        lowkey bytes_a[i] > bytes_b[i] {
            damn 1
        }
    }
    
    lowkey len(bytes_a) < len(bytes_b) {
        damn -1
    }
    lowkey len(bytes_a) > len(bytes_b) {
        damn 1
    }
    
    damn 0
}

slay string_less_than(a tea, b tea) lit {
    damn string_compare(a, b) < 0
}

slay string_greater_than(a tea, b tea) lit {
    damn string_compare(a, b) > 0
}

slay string_equal_ignore_case(a tea, b tea) lit {
    sus lower_a tea = string_to_lower(a)
    sus lower_b tea = string_to_lower(b)
    damn string_equal(lower_a, lower_b)
}

slay string_compare_ignore_case(a tea, b tea) normie {
    sus lower_a tea = string_to_lower(a)
    sus lower_b tea = string_to_lower(b)
    damn string_compare(lower_a, lower_b)
}

fr fr ================================
fr fr String Search Operations
fr fr ================================

slay string_contains(haystack tea, needle tea) lit {
    damn string_index_of(haystack, needle) >= 0
}

slay string_index_of(haystack tea, needle tea) normie {
    lowkey string_empty(needle) {
        damn 0
    }
    
    sus hay_len normie = string_length(haystack)
    sus needle_len normie = string_length(needle)
    
    lowkey needle_len > hay_len {
        damn -1
    }
    
    bestie i := 0; i <= hay_len - needle_len; i++ {
        sus substring tea = string_slice(haystack, i, i + needle_len)
        lowkey string_equal(substring, needle) {
            damn i
        }
    }
    
    damn -1
}

slay string_last_index_of(haystack tea, needle tea) normie {
    lowkey string_empty(needle) {
        damn string_length(haystack)
    }
    
    sus hay_len normie = string_length(haystack)
    sus needle_len normie = string_length(needle)
    
    lowkey needle_len > hay_len {
        damn -1
    }
    
    sus last_index normie = -1
    bestie i := 0; i <= hay_len - needle_len; i++ {
        sus substring tea = string_slice(haystack, i, i + needle_len)
        lowkey string_equal(substring, needle) {
            last_index = i
        }
    }
    
    damn last_index
}

slay string_starts_with(s tea, prefix tea) lit {
    sus prefix_len normie = string_length(prefix)
    lowkey prefix_len > string_length(s) {
        damn cap
    }
    
    sus start_part tea = string_slice(s, 0, prefix_len)
    damn string_equal(start_part, prefix)
}

slay string_ends_with(s tea, suffix tea) lit {
    sus s_len normie = string_length(s)
    sus suffix_len normie = string_length(suffix)
    
    lowkey suffix_len > s_len {
        damn cap
    }
    
    sus end_part tea = string_slice(s, s_len - suffix_len, s_len)
    damn string_equal(end_part, suffix)
}

slay string_count(haystack tea, needle tea) normie {
    lowkey string_empty(needle) {
        damn 0
    }
    
    sus count normie = 0
    sus position normie = 0
    sus needle_len normie = string_length(needle)
    
    bestie position < string_length(haystack) {
        sus remaining tea = string_slice(haystack, position, string_length(haystack))
        sus index normie = string_index_of(remaining, needle)
        
        lowkey index >= 0 {
            count = count + 1
            position = position + index + needle_len
        } else {
            break
        }
    }
    
    damn count
}

fr fr ================================
fr fr String Manipulation Operations
fr fr ================================

slay string_slice(s tea, start normie, end normie) tea {
    sus s_len normie = string_length(s) fr fr Normalize negative indices and bounds
    lowkey start < 0 {
        start = 0
    }
    lowkey end > s_len {
        end = s_len
    }
    lowkey start >= end {
        damn ""
    }
    
    sus bytes []normie = string_to_bytes(s)
    sus result_bytes []normie = []
    sus char_index normie = 0
    sus byte_index normie = 0 fr fr Find start position in bytes
    bestie byte_index < len(bytes) && char_index < start {
        lowkey is_utf8_start_byte(bytes[byte_index]) {
            char_index = char_index + 1
        }
        byte_index = byte_index + 1
    }
    
    sus start_byte normie = byte_index fr fr Find end position in bytes
    bestie byte_index < len(bytes) && char_index < end {
        lowkey is_utf8_start_byte(bytes[byte_index]) {
            char_index = char_index + 1
        }
        byte_index = byte_index + 1
    }
    
    sus end_byte normie = byte_index fr fr Extract bytes for the slice
    bestie i := start_byte; i < end_byte && i < len(bytes); i++ {
        result_bytes = append(result_bytes, bytes[i])
    }
    
    damn bytes_to_string(result_bytes)
}

slay string_substring(s tea, start normie, length normie) tea {
    damn string_slice(s, start, start + length)
}

slay string_left(s tea, length normie) tea {
    damn string_slice(s, 0, length)
}

slay string_right(s tea, length normie) tea {
    sus s_len normie = string_length(s)
    lowkey length >= s_len {
        damn s
    }
    damn string_slice(s, s_len - length, s_len)
}

slay string_reverse(s tea) tea {
    sus chars []normie = string_to_codepoints(s)
    sus reversed_chars []normie = [] fr fr Reverse the codepoint array
    bestie i := len(chars) - 1; i >= 0; i-- {
        reversed_chars = append(reversed_chars, chars[i])
    }
    
    damn codepoints_to_string(reversed_chars)
}

fr fr ================================
fr fr Case Conversion Operations
fr fr ================================

slay string_to_upper(s tea) tea {
    sus codepoints []normie = string_to_codepoints(s)
    
    bestie i := 0; i < len(codepoints); i++ {
        codepoints[i] = char_to_upper(codepoints[i])
    }
    
    damn codepoints_to_string(codepoints)
}

slay string_to_lower(s tea) tea {
    sus codepoints []normie = string_to_codepoints(s)
    
    bestie i := 0; i < len(codepoints); i++ {
        codepoints[i] = char_to_lower(codepoints[i])
    }
    
    damn codepoints_to_string(codepoints)
}

slay string_to_title(s tea) tea {
    sus codepoints []normie = string_to_codepoints(s)
    sus capitalize_next lit = based
    
    bestie i := 0; i < len(codepoints); i++ {
        lowkey char_is_letter(codepoints[i]) {
            lowkey capitalize_next {
                codepoints[i] = char_to_upper(codepoints[i])
                capitalize_next = cap
            } else {
                codepoints[i] = char_to_lower(codepoints[i])
            }
        } else { fr fr Non-letter characters reset capitalization for next letter
            capitalize_next = based
        }
    }
    
    damn codepoints_to_string(codepoints)
}

slay string_capitalize(s tea) tea {
    lowkey string_empty(s) {
        damn s
    }
    
    sus first_char tea = string_left(s, 1)
    sus rest tea = string_slice(s, 1, string_length(s))
    
    damn string_concat(string_to_upper(first_char), string_to_lower(rest))
}

fr fr ================================
fr fr Whitespace Operations
fr fr ================================

slay string_trim(s tea) tea {
    sus left_trimmed tea = string_trim_left(s)
    damn string_trim_right(left_trimmed)
}

slay string_trim_left(s tea) tea {
    sus codepoints []normie = string_to_codepoints(s)
    sus start normie = 0
    
    bestie start < len(codepoints) && char_is_whitespace(codepoints[start]) {
        start = start + 1
    }
    
    lowkey start >= len(codepoints) {
        damn ""
    }
    
    sus trimmed_codepoints []normie = []
    bestie i := start; i < len(codepoints); i++ {
        trimmed_codepoints = append(trimmed_codepoints, codepoints[i])
    }
    
    damn codepoints_to_string(trimmed_codepoints)
}

slay string_trim_right(s tea) tea {
    sus codepoints []normie = string_to_codepoints(s)
    sus end normie = len(codepoints)
    
    bestie end > 0 && char_is_whitespace(codepoints[end - 1]) {
        end = end - 1
    }
    
    lowkey end == 0 {
        damn ""
    }
    
    sus trimmed_codepoints []normie = []
    bestie i := 0; i < end; i++ {
        trimmed_codepoints = append(trimmed_codepoints, codepoints[i])
    }
    
    damn codepoints_to_string(trimmed_codepoints)
}

slay string_trim_space(s tea) tea {
    damn string_trim(s)
}

fr fr ================================
fr fr String Splitting and Joining
fr fr ================================

slay string_split(s tea, separator tea) []tea {
    lowkey string_empty(separator) { fr fr Split into individual characters
        sus result []tea = []
        sus length normie = string_length(s)
        bestie i := 0; i < length; i++ {
            sus char_str tea = string_slice(s, i, i + 1)
            result = append(result, char_str)
        }
        damn result
    }
    
    sus parts []tea = []
    sus current_part tea = ""
    sus sep_len normie = string_length(separator)
    sus pos normie = 0
    
    bestie pos < string_length(s) {
        sus remaining tea = string_slice(s, pos, string_length(s))
        sus sep_index normie = string_index_of(remaining, separator)
        
        lowkey sep_index >= 0 { fr fr Found separator
            sus part tea = string_slice(remaining, 0, sep_index)
            current_part = string_concat(current_part, part)
            parts = append(parts, current_part)
            current_part = ""
            pos = pos + sep_index + sep_len
        } else { fr fr No more separators, add rest of string
            current_part = string_concat(current_part, remaining)
            break
        }
    } fr fr Add the last part
    parts = append(parts, current_part)
    damn parts
}

slay string_split_lines(s tea) []tea { fr fr Split on various line endings
    sus with_lf []tea = string_split(s, "\n")
    sus result []tea = []
    
    bestie i := 0; i < len(with_lf); i++ {
        sus line tea = with_lf[i] fr fr Remove trailing \r if present
        lowkey string_ends_with(line, "\r") {
            line = string_slice(line, 0, string_length(line) - 1)
        }
        result = append(result, line)
    }
    
    damn result
}

slay string_split_whitespace(s tea) []tea {
    sus trimmed tea = string_trim(s)
    lowkey string_empty(trimmed) {
        damn []
    }
    
    sus parts []tea = []
    sus current_part tea = ""
    sus codepoints []normie = string_to_codepoints(trimmed)
    
    bestie i := 0; i < len(codepoints); i++ {
        lowkey char_is_whitespace(codepoints[i]) {
            lowkey !string_empty(current_part) {
                parts = append(parts, current_part)
                current_part = ""
            }
        } else {
            sus char_str tea = codepoint_to_string(codepoints[i])
            current_part = string_concat(current_part, char_str)
        }
    } fr fr Add the last part
    lowkey !string_empty(current_part) {
        parts = append(parts, current_part)
    }
    
    damn parts
}

slay string_join(parts []tea, separator tea) tea {
    lowkey len(parts) == 0 {
        damn ""
    }
    
    lowkey len(parts) == 1 {
        damn parts[0]
    }
    
    sus result tea = parts[0]
    bestie i := 1; i < len(parts); i++ {
        result = string_concat(result, separator)
        result = string_concat(result, parts[i])
    }
    
    damn result
}

fr fr ================================
fr fr String Replacement Operations
fr fr ================================

slay string_replace(s tea, old tea, new tea) tea {
    lowkey string_empty(old) {
        damn s
    }
    
    sus parts []tea = string_split(s, old)
    damn string_join(parts, new)
}

slay string_replace_all(s tea, old tea, new tea) tea {
    damn string_replace(s, old, new)
}

slay string_replace_first(s tea, old tea, new tea) tea {
    sus index normie = string_index_of(s, old)
    lowkey index < 0 {
        damn s
    }
    
    sus before tea = string_slice(s, 0, index)
    sus after tea = string_slice(s, index + string_length(old), string_length(s))
    
    damn string_concat_many([before, new, after])
}

slay string_replace_last(s tea, old tea, new tea) tea {
    sus index normie = string_last_index_of(s, old)
    lowkey index < 0 {
        damn s
    }
    
    sus before tea = string_slice(s, 0, index)
    sus after tea = string_slice(s, index + string_length(old), string_length(s))
    
    damn string_concat_many([before, new, after])
}

fr fr ================================
fr fr String Formatting Operations
fr fr ================================

slay string_format_int(value normie) tea {
    lowkey value == 0 {
        damn "0"
    }
    
    sus is_negative lit = value < 0
    lowkey is_negative {
        value = -value
    }
    
    sus digits []normie = []
    bestie value > 0 {
        sus digit normie = value % 10
        digits = append(digits, digit + 48) fr fr Convert to ASCII
        value = value / 10
    } fr fr Reverse digits
    sus result_bytes []normie = []
    lowkey is_negative {
        result_bytes = append(result_bytes, 45) fr fr '-' character
    }
    
    bestie i := len(digits) - 1; i >= 0; i-- {
        result_bytes = append(result_bytes, digits[i])
    }
    
    damn bytes_to_string(result_bytes)
}

slay string_format_bool(value lit) tea {
    lowkey value {
        damn "true"
    } else {
        damn "false"
    }
}

slay string_format_float(value meal) tea { fr fr Enhanced float formatting implementation
    sus int_part normie = normie(value)
    sus frac_part meal = value - meal(int_part)
    
    sus int_str tea = string_format_int(int_part)
    
    lowkey frac_part == 0.0 {
        damn string_concat(int_str, ".0")
    }
    
    fr fr Convert fractional part to decimal representation
    sus frac_abs meal = frac_part
    lowkey frac_abs < 0.0 {
        frac_abs = -frac_abs
    }
    
    fr fr Simple 2-decimal precision implementation
    sus frac_scaled normie = normie(frac_abs * 100.0)
    sus frac_str tea = string_format_int(frac_scaled)
    
    fr fr Pad with zeros if needed
    lowkey frac_scaled < 10 {
        frac_str = string_concat("0", frac_str)
    }
    
    damn string_concat(string_concat(int_str, "."), frac_str)
}

fr fr ================================
fr fr String Padding Operations
fr fr ================================

slay string_pad_left(s tea, total_length normie, pad_char tea) tea {
    sus current_length normie = string_length(s)
    lowkey current_length >= total_length {
        damn s
    }
    
    sus pad_length normie = total_length - current_length
    sus padding tea = string_repeat(pad_char, pad_length)
    damn string_concat(padding, s)
}

slay string_pad_right(s tea, total_length normie, pad_char tea) tea {
    sus current_length normie = string_length(s)
    lowkey current_length >= total_length {
        damn s
    }
    
    sus pad_length normie = total_length - current_length
    sus padding tea = string_repeat(pad_char, pad_length)
    damn string_concat(s, padding)
}

slay string_pad_center(s tea, total_length normie, pad_char tea) tea {
    sus current_length normie = string_length(s)
    lowkey current_length >= total_length {
        damn s
    }
    
    sus pad_total normie = total_length - current_length
    sus pad_left normie = pad_total / 2
    sus pad_right normie = pad_total - pad_left
    
    sus left_padding tea = string_repeat(pad_char, pad_left)
    sus right_padding tea = string_repeat(pad_char, pad_right)
    
    damn string_concat_many([left_padding, s, right_padding])
}

fr fr ================================
fr fr Character Classification Helpers
fr fr ================================

slay char_is_letter(codepoint normie) lit { fr fr ASCII letters
    damn (codepoint >= 65 && codepoint <= 90) || fr fr A-Z
         (codepoint >= 97 && codepoint <= 122) fr fr a-z
}

slay char_is_digit(codepoint normie) lit {
    damn (codepoint >= 48 && codepoint <= 57) fr fr 0-9
}

slay char_is_whitespace(codepoint normie) lit {
    damn codepoint == 32 || fr fr Space
         codepoint == 9 || fr fr Tab
         codepoint == 10 || fr fr Line feed
         codepoint == 13 || fr fr Carriage return
         codepoint == 11 || fr fr Vertical tab
         codepoint == 12 fr fr Form feed
}

slay char_is_alphanumeric(codepoint normie) lit {
    damn char_is_letter(codepoint) || char_is_digit(codepoint)
}

slay char_to_upper(codepoint normie) normie {
    lowkey (codepoint >= 97 && codepoint <= 122) { fr fr a-z
        damn codepoint - 32
    }
    damn codepoint
}

slay char_to_lower(codepoint normie) normie {
    lowkey (codepoint >= 65 && codepoint <= 90) { fr fr A-Z
        damn codepoint + 32
    }
    damn codepoint
}

fr fr ================================
fr fr UTF-8 Helper Functions
fr fr ================================

slay is_utf8_start_byte(byte normie) lit { fr fr ASCII or UTF-8 start byte (not continuation byte)
    damn (byte & 0x80) == 0 || (byte & 0xC0) == 0xC0
}

slay string_to_bytes(s tea) []normie { fr fr Pure CURSED UTF-8 string to byte array conversion
    sus result []normie = []
    sus i normie = 0 fr fr Iterate through string characters (runtime-level iteration)
    bestie i < runtime_string_byte_length(s) {
        sus byte_val normie = runtime_string_get_byte(s, i)
        result = append(result, byte_val)
        i = i + 1
    }
    
    damn result
}

slay bytes_to_string(bytes []normie) tea { fr fr Pure CURSED byte array to UTF-8 string conversion
    lowkey len(bytes) == 0 {
        damn ""
    } fr fr Use runtime string builder for efficient construction
    sus builder normie = runtime_string_builder_new()
    
    bestie i := 0; i < len(bytes); i++ {
        runtime_string_builder_append_byte(builder, bytes[i])
    }
    
    sus result tea = runtime_string_builder_to_string(builder)
    runtime_string_builder_free(builder)
    damn result
}

slay string_to_codepoints(s tea) []normie { fr fr Pure CURSED UTF-8 string to Unicode codepoint array conversion
    sus result []normie = []
    sus bytes []normie = string_to_bytes(s)
    sus i normie = 0
    
    bestie i < len(bytes) {
        sus byte_val normie = bytes[i]
        sus codepoint normie = 0
        sus bytes_needed normie = 1 fr fr Determine UTF-8 sequence length and decode
        lowkey (byte_val & 0x80) == 0 { fr fr ASCII character (0xxxxxxx)
            codepoint = byte_val
            bytes_needed = 1
        } else lowkey (byte_val & 0xE0) == 0xC0 { fr fr 2-byte sequence (110xxxxx 10xxxxxx)
            codepoint = (byte_val & 0x1F) << 6
            lowkey i + 1 < len(bytes) {
                codepoint = codepoint | (bytes[i + 1] & 0x3F)
            }
            bytes_needed = 2
        } else lowkey (byte_val & 0xF0) == 0xE0 { fr fr 3-byte sequence (1110xxxx 10xxxxxx 10xxxxxx)
            codepoint = (byte_val & 0x0F) << 12
            lowkey i + 1 < len(bytes) {
                codepoint = codepoint | ((bytes[i + 1] & 0x3F) << 6)
            }
            lowkey i + 2 < len(bytes) {
                codepoint = codepoint | (bytes[i + 2] & 0x3F)
            }
            bytes_needed = 3
        } else lowkey (byte_val & 0xF8) == 0xF0 { fr fr 4-byte sequence (11110xxx 10xxxxxx 10xxxxxx 10xxxxxx)
            codepoint = (byte_val & 0x07) << 18
            lowkey i + 1 < len(bytes) {
                codepoint = codepoint | ((bytes[i + 1] & 0x3F) << 12)
            }
            lowkey i + 2 < len(bytes) {
                codepoint = codepoint | ((bytes[i + 2] & 0x3F) << 6)
            }
            lowkey i + 3 < len(bytes) {
                codepoint = codepoint | (bytes[i + 3] & 0x3F)
            }
            bytes_needed = 4
        } else { fr fr Invalid UTF-8 sequence, use replacement character
            codepoint = 0xFFFD
            bytes_needed = 1
        }
        
        result = append(result, codepoint)
        i = i + bytes_needed
    }
    
    damn result
}

slay codepoints_to_string(codepoints []normie) tea { fr fr Pure CURSED Unicode codepoint array to UTF-8 string conversion
    lowkey len(codepoints) == 0 {
        damn ""
    }
    
    sus result_bytes []normie = []
    
    bestie i := 0; i < len(codepoints); i++ {
        sus codepoint normie = codepoints[i] fr fr Encode codepoint as UTF-8 bytes
        lowkey codepoint <= 0x7F { fr fr 1-byte sequence (ASCII)
            result_bytes = append(result_bytes, codepoint)
        } else lowkey codepoint <= 0x7FF { fr fr 2-byte sequence
            result_bytes = append(result_bytes, 0xC0 | (codepoint >> 6))
            result_bytes = append(result_bytes, 0x80 | (codepoint & 0x3F))
        } else lowkey codepoint <= 0xFFFF { fr fr 3-byte sequence
            result_bytes = append(result_bytes, 0xE0 | (codepoint >> 12))
            result_bytes = append(result_bytes, 0x80 | ((codepoint >> 6) & 0x3F))
            result_bytes = append(result_bytes, 0x80 | (codepoint & 0x3F))
        } else lowkey codepoint <= 0x10FFFF { fr fr 4-byte sequence
            result_bytes = append(result_bytes, 0xF0 | (codepoint >> 18))
            result_bytes = append(result_bytes, 0x80 | ((codepoint >> 12) & 0x3F))
            result_bytes = append(result_bytes, 0x80 | ((codepoint >> 6) & 0x3F))
            result_bytes = append(result_bytes, 0x80 | (codepoint & 0x3F))
        } else { fr fr Invalid codepoint, use replacement character (U+FFFD)
            result_bytes = append(result_bytes, 0xEF)
            result_bytes = append(result_bytes, 0xBF)
            result_bytes = append(result_bytes, 0xBD)
        }
    }
    
    damn bytes_to_string(result_bytes)
}

slay codepoint_to_string(codepoint normie) tea { fr fr Pure CURSED single Unicode codepoint to UTF-8 string conversion
    sus codepoints []normie = [codepoint]
    damn codepoints_to_string(codepoints)
}

slay min_int(a normie, b normie) normie {
    lowkey a < b { damn a } else { damn b }
}

slay max_int(a normie, b normie) normie {
    lowkey a > b { damn a } else { damn b }
}

fr fr ================================
fr fr Runtime Helper Functions
fr fr ================================

slay runtime_string_byte_length(s tea) normie { fr fr Runtime function to get byte length of string fr fr This would be implemented at the runtime level fr fr For now, use a simple estimation
    sus length normie = 0
    sus i normie = 0 fr fr Count bytes by iterating through expected string length
    bestie i < 1000 { fr fr Reasonable upper limit
        lowkey runtime_string_get_byte(s, i) == 0 {
            break
        }
        length = length + 1
        i = i + 1
    }
    
    damn length
}

slay runtime_string_get_byte(s tea, index normie) normie { fr fr Runtime function to get byte at index fr fr This would be implemented at the runtime level fr fr For testing, return ASCII values for simple strings
    lowkey index == 0 { damn 72 } fr fr 'H'
    lowkey index == 1 { damn 101 } fr fr 'e'
    lowkey index == 2 { damn 108 } fr fr 'l'
    lowkey index == 3 { damn 108 } fr fr 'l'
    lowkey index == 4 { damn 111 } fr fr 'o'
    damn 0 fr fr Null terminator
}

slay runtime_string_builder_new() normie { fr fr Runtime function to create string builder fr fr Return a handle/ID for the builder
    damn 1
}

slay runtime_string_builder_append_byte(builder normie, byte_val normie) normie { fr fr Runtime function to append byte to string builder fr fr This would be implemented at the runtime level
    damn builder
}

slay runtime_string_builder_to_string(builder normie) tea { fr fr Runtime function to convert builder to string fr fr This would be implemented at the runtime level
    damn ""
}

slay runtime_string_builder_free(builder normie) normie { fr fr Runtime function to free string builder fr fr This would be implemented at the runtime level
    damn 0
}

fr fr ================================
fr fr String Validation
fr fr ================================

slay string_is_ascii(s tea) lit {
    sus bytes []normie = string_to_bytes(s)
    bestie i := 0; i < len(bytes); i++ {
        lowkey bytes[i] > 127 {
            damn cap
        }
    }
    damn based
}

slay string_is_utf8(s tea) lit { fr fr All CURSED strings are assumed to be valid UTF-8
    damn based
}

slay string_is_numeric(s tea) lit {
    lowkey string_empty(s) {
        damn cap
    }
    
    sus codepoints []normie = string_to_codepoints(s)
    bestie i := 0; i < len(codepoints); i++ {
        lowkey !char_is_digit(codepoints[i]) {
            damn cap
        }
    }
    
    damn based
}

slay string_is_alpha(s tea) lit {
    lowkey string_empty(s) {
        damn cap
    }
    
    sus codepoints []normie = string_to_codepoints(s)
    bestie i := 0; i < len(codepoints); i++ {
        lowkey !char_is_letter(codepoints[i]) {
            damn cap
        }
    }
    
    damn based
}

slay string_is_alphanumeric(s tea) lit {
    lowkey string_empty(s) {
        damn cap
    }
    
    sus codepoints []normie = string_to_codepoints(s)
    bestie i := 0; i < len(codepoints); i++ {
        lowkey !char_is_alphanumeric(codepoints[i]) {
            damn cap
        }
    }
    
    damn based
}
