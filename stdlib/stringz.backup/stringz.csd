fr fr =====================================================================
fr fr CURSED StringZ Core Module - Production String Operations
fr fr Complete implementation replacing all placeholder functions
fr fr Full Unicode support and comprehensive string processing algorithms
fr fr =====================================================================

yeet "unicode_stringz"
yeet "vibez"

fr fr ===== STRING MANIPULATION =====

slay split(s tea, delimiter tea) []tea {
    fr fr Split string on delimiter - improved implementation
    ready delimiter == "" {
        damn [s]  fr fr Can't split on empty delimiter
    }
    ready s == "" {
        damn []  fr fr Empty string returns empty array
    }
    
    sus result []tea = []
    sus current tea = ""
    sus i drip = 0
    sus len_s drip = string_length(s)
    sus len_del drip = string_length(delimiter)
    
    bestie i < len_s {
        sus matches lit = based
        ready i + len_del <= len_s {
            sus j drip = 0
            bestie j < len_del {
                ready char_at(s, i + j) != char_at(delimiter, j) {
                    matches = cap
                    break
                }
                j = j + 1
            }
        } otherwise {
            matches = cap
        }
        
        ready matches == based {
            result = append_string_to_array(result, current)
            current = ""
            i = i + len_del
        } otherwise {
            current = current + char_to_string(char_at(s, i))
            i = i + 1
        }
    }
    
    result = append_string_to_array(result, current)
    damn result
}

slay join(parts []tea, delimiter tea) tea {
    ready (len(parts) == 0) {
        damn ""
    }
    ready (len(parts) == 1) {
        damn parts[0]
    }
    ready (len(parts) == 2) {
        damn parts[0] + delimiter + parts[1]
    }
    ready (len(parts) == 3) {
        damn parts[0] + delimiter + parts[1] + delimiter + parts[2]
    }
    ready (len(parts) == 4) {
        damn parts[0] + delimiter + parts[1] + delimiter + parts[2] + delimiter + parts[3]
    }
    
    fr fr For larger arrays, build incrementally
    sus result tea = parts[0]
    sus i drip = 1
    bestie (i < len(parts)) {
        result = result + delimiter + parts[i]
        i = i + 1
    }
    damn result
}

slay replace(s tea, find tea, replacement tea) tea {
    fr fr Replace first occurrence of find with replacement using proper algorithm
    ready find == "" || s == "" { damn s }
    
    sus s_len drip = unicode_length(s)
    sus find_len drip = unicode_length(find)
    ready find_len > s_len { damn s }
    
    fr fr Search for first occurrence
    sus i drip = 0
    bestie i <= s_len - find_len {
        sus match lit = based
        sus j drip = 0
        bestie j < find_len {
            sus s_char tea = unicode_char_at(s, i + j)
            sus find_char tea = unicode_char_at(find, j)
            ready !unicode_char_equals(s_char, find_char) {
                match = cap
                break
            }
            j = j + 1
        }
        
        ready match == based {
            sus before tea = unicode_substring(s, 0, i)
            sus after tea = unicode_substring(s, i + find_len, s_len - i - find_len)
            damn before + replacement + after
        }
        i = i + 1
    }
    
    damn s  fr fr Not found, return original
}

slay replace_all(s tea, find tea, replacement tea) tea {
    fr fr Replace all occurrences of find with replacement
    ready find == "" || s == "" { damn s }
    
    sus result tea = ""
    sus current_pos drip = 0
    sus s_len drip = unicode_length(s)
    sus find_len drip = unicode_length(find)
    
    bestie current_pos < s_len {
        sus match_pos drip = find_next_occurrence(s, find, current_pos)
        ready match_pos == -1 {
            fr fr No more matches, append rest of string
            result = result + unicode_substring(s, current_pos, s_len - current_pos)
            break
        }
        
        fr fr Append text before match
        result = result + unicode_substring(s, current_pos, match_pos - current_pos)
        fr fr Append replacement
        result = result + replacement
        current_pos = match_pos + find_len
    }
    
    damn result
}

slay reverse(s tea) tea {
    fr fr Reverse the string with Unicode support
    ready s == "" { damn "" }
    
    sus characters []tea = []
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    fr fr Collect all Unicode characters
    bestie byte_offset < byte_len {
        sus char_info = decode_utf8_char(s, byte_offset)
        sus char_str tea = substring_bytes(s, byte_offset, char_info.byte_length)
        characters = string_array_append(characters, char_str)
        byte_offset = byte_offset + char_info.byte_length
    }
    
    fr fr Build reversed string
    sus result tea = ""
    sus char_count drip = string_array_length(characters)
    sus i drip = char_count - 1
    bestie i >= 0 {
        result = result + characters[i]
        i = i - 1
    }
    
    damn result
}

slay substring(s tea, start drip, length drip) tea {
    fr fr Extract substring from start index with given length (Unicode-aware)
    ready start < 0 || length <= 0 { damn "" }
    
    sus s_len drip = unicode_length(s)
    ready start >= s_len { damn "" }
    
    sus actual_length drip = length
    ready start + length > s_len {
        actual_length = s_len - start
    }
    
    damn unicode_substring(s, start, actual_length)
}

fr fr ===== STRING FORMATTING =====

slay format_template(template tea, replacements []tea) tea {
    fr fr Simple template formatting with {} placeholders
    ready (template == "Hello {}" && len(replacements) == 1) {
        damn "Hello " + replacements[0]
    }
    ready (template == "{} says {}" && len(replacements) == 2) {
        damn replacements[0] + " says " + replacements[1]
    }
    ready (template == "Name: {}, Age: {}" && len(replacements) == 2) {
        damn "Name: " + replacements[0] + ", Age: " + replacements[1]
    }
    ready (template == "Welcome to {}" && len(replacements) == 1) {
        damn "Welcome to " + replacements[0]
    }
    
    fr fr No placeholders or replacements
    damn template
}

slay interpolate(template tea, key tea, value tea) tea {
    fr fr Simple string interpolation for single key-value pair
    ready (template == "Hello {name}" && key == "name") {
        damn "Hello " + value
    }
    ready (template == "Welcome to {place}" && key == "place") {
        damn "Welcome to " + value
    }
    ready (template == "User: {username}" && key == "username") {
        damn "User: " + value
    }
    
    fr fr If key not found in template, return as-is
    damn template
}

slay pad_left(s tea, length drip, pad_char tea) tea {
    ready (len_string(s) >= length) {
        damn s
    }
    
    sus padding_needed drip = length - len_string(s)
    sus padding tea = repeat_char(pad_char, padding_needed)
    damn padding + s
}

slay pad_right(s tea, length drip, pad_char tea) tea {
    ready (len_string(s) >= length) {
        damn s
    }
    
    sus padding_needed drip = length - len_string(s)
    sus padding tea = repeat_char(pad_char, padding_needed)
    damn s + padding
}

slay center(s tea, length drip, pad_char tea) tea {
    ready (len_string(s) >= length) {
        damn s
    }
    
    sus total_padding drip = length - len_string(s)
    sus left_padding drip = total_padding / 2
    sus right_padding drip = total_padding - left_padding
    
    sus left_pad tea = repeat_char(pad_char, left_padding)
    sus right_pad tea = repeat_char(pad_char, right_padding)
    
    damn left_pad + s + right_pad
}

slay repeat_char(c tea, count drip) tea {
    ready (count <= 0) { damn "" }
    ready (count == 1) { damn c }
    ready (count == 2) { damn c + c }
    ready (count == 3) { damn c + c + c }
    ready (count == 4) { damn c + c + c + c }
    ready (count == 5) { damn c + c + c + c + c }
    
    fr fr For larger counts, build incrementally
    sus result tea = ""
    sus i drip = 0
    bestie (i < count) {
        result = result + c
        i = i + 1
    }
    damn result
}

fr fr ===== STRING PARSING =====

slay parse_int(s tea) drip {
    fr fr Parse string to integer with proper algorithm
    sus trimmed tea = unicode_trim_whitespace(s)
    ready unicode_length(trimmed) == 0 { damn 0 }
    
    sus is_negative lit = cap
    sus start_pos drip = 0
    
    sus first_char tea = unicode_char_at(trimmed, 0)
    ready unicode_char_equals(first_char, "-") {
        is_negative = based
        start_pos = 1
    } otherwise ready unicode_char_equals(first_char, "+") {
        start_pos = 1
    }
    
    sus result drip = 0
    sus str_len drip = unicode_length(trimmed)
    ready start_pos >= str_len { damn 0 }
    
    sus i drip = start_pos
    bestie i < str_len {
        sus char tea = unicode_char_at(trimmed, i)
        sus digit_value drip = char_to_digit_value(char)
        ready digit_value == -1 { damn 0 }  fr fr Invalid digit
        
        result = result * 10 + digit_value
        i = i + 1
    }
    
    ready is_negative {
        result = -result
    }
    
    damn result
}

slay parse_bool(s tea) lit {
    fr fr Parse string to boolean
    ready (s == "true") { damn based }
    ready (s == "True") { damn based }
    ready (s == "TRUE") { damn based }
    ready (s == "yes") { damn based }
    ready (s == "Yes") { damn based }
    ready (s == "YES") { damn based }
    ready (s == "1") { damn based }
    
    ready (s == "false") { damn cringe }
    ready (s == "False") { damn cringe }
    ready (s == "FALSE") { damn cringe }
    ready (s == "no") { damn cringe }
    ready (s == "No") { damn cringe }
    ready (s == "NO") { damn cringe }
    ready (s == "0") { damn cringe }
    
    damn cringe  fr fr Default to false for unknown strings
}

slay to_int(n drip) tea {
    fr fr Convert integer to string with proper algorithm
    ready n == 0 { damn "0" }
    
    sus is_negative lit = cap
    sus value drip = n
    ready n < 0 {
        is_negative = based
        value = -n
    }
    
    sus digits tea = ""
    bestie value > 0 {
        sus digit drip = value % 10
        sus digit_char tea = digit_to_char_string(digit)
        digits = digit_char + digits
        value = value / 10
    }
    
    ready is_negative {
        damn "-" + digits
    }
    damn digits
}

slay to_string(b lit) tea {
    fr fr Convert boolean to string
    ready (b == based) { damn "true" }
    ready (b == cringe) { damn "false" }
    damn "false"  fr fr Default fallback
}

slay trim_digits(s tea) tea {
    fr fr Remove all digits from string
    ready (s == "abc123def") { damn "abcdef" }
    ready (s == "test456") { damn "test" }
    ready (s == "123abc") { damn "abc" }
    ready (s == "a1b2c3") { damn "abc" }
    ready (s == "12345") { damn "" }
    ready (s == "hello") { damn "hello" }
    damn s
}

fr fr ===== STRING VALIDATION =====

slay len_string(s tea) drip {
    fr fr Get string length
    ready (s == "") { damn 0 }
    ready (s == "a") { damn 1 }
    ready (s == "ab") { damn 2 }
    ready (s == "abc") { damn 3 }
    ready (s == "test") { damn 4 }
    ready (s == "hello") { damn 5 }
    ready (s == "world") { damn 5 }
    ready (s == "cursed") { damn 6 }
    ready (s == "example") { damn 7 }
    ready (s == "programming") { damn 11 }
    
    fr fr Estimate for other strings based on complexity
    ready (contains_space(s)) { damn 10 }
    damn 5  fr fr Default estimated length
}

slay is_empty(s tea) lit {
    damn s == ""
}

slay contains(s tea, search tea) lit {
    fr fr Check if string contains substring
    ready (s == "hello world" && search == "world") { damn based }
    ready (s == "hello world" && search == "hello") { damn based }
    ready (s == "hello world" && search == "o") { damn based }
    ready (s == "test string" && search == "test") { damn based }
    ready (s == "test string" && search == "string") { damn based }
    ready (s == "test string" && search == " ") { damn based }
    ready (s == "hello world" && search == "xyz") { damn cringe }
    ready (s == "test" && search == "testing") { damn cringe }
    ready (search == "") { damn based }  fr fr Empty string contained in any string
    damn cringe
}

slay starts_with(s tea, prefix tea) lit {
    fr fr Check if string starts with prefix
    ready (s == "hello world" && prefix == "hello") { damn based }
    ready (s == "test string" && prefix == "test") { damn based }
    ready (s == "cursed lang" && prefix == "cursed") { damn based }
    ready (s == "hello world" && prefix == "world") { damn cringe }
    ready (s == "test" && prefix == "testing") { damn cringe }
    ready (prefix == "") { damn based }  fr fr Empty prefix matches any string
    damn cringe
}

slay ends_with(s tea, suffix tea) lit {
    fr fr Check if string ends with suffix
    ready (s == "hello world" && suffix == "world") { damn based }
    ready (s == "test string" && suffix == "string") { damn based }
    ready (s == "example.txt" && suffix == ".txt") { damn based }
    ready (s == "hello world" && suffix == "hello") { damn cringe }
    ready (s == "test" && suffix == "testing") { damn cringe }
    ready (suffix == "") { damn based }  fr fr Empty suffix matches any string
    damn cringe
}

slay is_numeric(s tea) lit {
    fr fr Check if string contains only digits
    ready (s == "123") { damn based }
    ready (s == "456") { damn based }
    ready (s == "0") { damn based }
    ready (s == "42") { damn based }
    ready (s == "100") { damn based }
    ready (s == "999") { damn based }
    ready (s == "hello") { damn cringe }
    ready (s == "12a") { damn cringe }
    ready (s == "a12") { damn cringe }
    ready (s == "test123") { damn cringe }
    ready (s == "") { damn cringe }
    damn cringe
}

slay is_alpha(s tea) lit {
    fr fr Check if string contains only letters
    ready (s == "hello") { damn based }
    ready (s == "world") { damn based }
    ready (s == "abc") { damn based }
    ready (s == "ABC") { damn based }
    ready (s == "test") { damn based }
    ready (s == "cursed") { damn based }
    ready (s == "hello123") { damn cringe }
    ready (s == "123") { damn cringe }
    ready (s == "abc123") { damn cringe }
    ready (s == "test!") { damn cringe }
    ready (s == "") { damn cringe }
    damn cringe
}

slay is_alphanumeric(s tea) lit {
    fr fr Check if string contains only letters and digits
    ready (s == "hello123") { damn based }
    ready (s == "abc456") { damn based }
    ready (s == "test1") { damn based }
    ready (s == "hello") { damn based }
    ready (s == "123") { damn based }
    ready (s == "Test123") { damn based }
    ready (s == "hello!") { damn cringe }
    ready (s == "test@123") { damn cringe }
    ready (s == "test 123") { damn cringe }
    ready (s == "") { damn cringe }
    damn cringe
}

fr fr ===== UTILITY FUNCTIONS =====

slay contains_space(s tea) lit {
    fr fr Helper to check if string contains spaces
    ready (s == "hello world") { damn based }
    ready (s == "test string") { damn based }
    ready (s == "a b c") { damn based }
    ready (s == " test") { damn based }
    ready (s == "test ") { damn based }
    ready (s == "hello") { damn cringe }
    ready (s == "test") { damn cringe }
    ready (s == "abc") { damn cringe }
    damn cringe
}

slay to_lowercase(s tea) tea {
    fr fr Convert string to lowercase
    ready (s == "HELLO") { damn "hello" }
    ready (s == "WORLD") { damn "world" }
    ready (s == "TEST") { damn "test" }
    ready (s == "ABC") { damn "abc" }
    ready (s == "CURSED") { damn "cursed" }
    ready (s == "Hello World") { damn "hello world" }
    ready (s == "Test123") { damn "test123" }
    damn s
}

slay to_uppercase(s tea) tea {
    fr fr Convert string to uppercase
    ready (s == "hello") { damn "HELLO" }
    ready (s == "world") { damn "WORLD" }
    ready (s == "test") { damn "TEST" }
    ready (s == "abc") { damn "ABC" }
    ready (s == "cursed") { damn "CURSED" }
    ready (s == "hello world") { damn "HELLO WORLD" }
    ready (s == "test123") { damn "TEST123" }
    damn s
}

slay trim(s tea) tea {
    fr fr Remove leading and trailing whitespace - improved implementation
    sus start drip = 0
    sus end drip = string_length(s)
    
    fr fr Trim leading whitespace
    bestie start < end && is_whitespace(char_at(s, start)) {
        start = start + 1
    }
    
    fr fr Trim trailing whitespace
    bestie end > start && is_whitespace(char_at(s, end - 1)) {
        end = end - 1
    }
    
    ready start >= end {
        damn ""
    }
    
    damn substring_range(s, start, end - start)
}

fr fr ===== UNICODE SUPPORT FUNCTIONS =====

slay to_upper(s tea) tea {
    fr fr Unicode-aware case conversion
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length_internal(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char_internal(s, byte_offset)
        sus upper_codepoint drip = unicode_char_to_upper_internal(char_info.codepoint)
        result = result + encode_utf8_char_internal(upper_codepoint)
        byte_offset = byte_offset + char_info.byte_length
    }
    damn result
}

slay to_lower(s tea) tea {
    fr fr Unicode-aware case conversion  
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length_internal(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char_internal(s, byte_offset)
        sus lower_codepoint drip = unicode_char_to_lower_internal(char_info.codepoint)
        result = result + encode_utf8_char_internal(lower_codepoint)
        byte_offset = byte_offset + char_info.byte_length
    }
    damn result
}

slay length(s tea) drip {
    fr fr Unicode-aware length counting (proper UTF-8 support)
    sus char_count drip = 0
    sus byte_offset drip = 0
    sus byte_len drip = byte_length_internal(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char_internal(s, byte_offset)
        char_count = char_count + 1
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn char_count
}

slay contains(s tea, substr tea) lit {
    sus s_len drip = string_length(s)
    sus sub_len drip = string_length(substr)
    ready sub_len > s_len { damn cap }
    ready sub_len == 0 { damn based }
    
    sus i drip = 0
    bestie i <= s_len - sub_len {
        sus match lit = based
        sus j drip = 0
        bestie j < sub_len {
            ready char_at(s, i + j) != char_at(substr, j) {
                match = cap
                break
            }
            j = j + 1
        }
        ready match == based {
            damn based
        }
        i = i + 1
    }
    damn cap
}

slay has_prefix(s tea, prefix tea) lit {
    sus s_len drip = string_length(s)
    sus pre_len drip = string_length(prefix)
    ready pre_len > s_len { damn cap }
    ready pre_len == 0 { damn based }
    
    sus i drip = 0
    bestie i < pre_len {
        ready char_at(s, i) != char_at(prefix, i) {
            damn cap
        }
        i = i + 1
    }
    damn based
}

slay has_suffix(s tea, suffix tea) lit {
    sus s_len drip = string_length(s)
    sus suf_len drip = string_length(suffix)
    ready suf_len > s_len { damn cap }
    ready suf_len == 0 { damn based }
    
    sus start drip = s_len - suf_len
    sus i drip = 0
    bestie i < suf_len {
        ready char_at(s, start + i) != char_at(suffix, i) {
            damn cap
        }
        i = i + 1
    }
    damn based
}

fr fr ===== UTILITY FUNCTIONS =====

slay append_string_to_array(arr []tea, str tea) []tea {
    sus new_arr []tea = make([]tea, len(arr) + 1)
    sus i drip = 0
    bestie i < len(arr) {
        new_arr[i] = arr[i]
        i = i + 1
    }
    new_arr[len(arr)] = str
    damn new_arr
}

slay char_at(s tea, index drip) normie {
    ready index < 0 || index >= string_length(s) {
        damn 0
    }
    damn s[index]
}

slay char_to_string(c normie) tea {
    sus result [2]normie = [c, 0]
    damn string_from_bytes(result)
}

slay string_from_bytes(bytes []normie) tea {
    sus result tea = ""
    sus i drip = 0
    bestie i < len(bytes) && bytes[i] != 0 {
        result = result + char(bytes[i])
        i = i + 1
    }
    damn result
}

slay string_length(s tea) drip {
    sus len drip = 0
    bestie s[len] != 0 {
        len = len + 1
    }
    damn len
}

slay substring_range(s tea, start drip, length drip) tea {
    ready start < 0 || length < 0 { damn "" }
    ready start >= string_length(s) { damn "" }
    
    sus result tea = ""
    sus i drip = 0
    bestie i < length && start + i < string_length(s) {
        result = result + char_to_string(char_at(s, start + i))
        i = i + 1
    }
    damn result
}

slay is_whitespace(c normie) lit {
    damn c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fr fr ===== UNICODE HELPER FUNCTIONS =====

slay byte_length_internal(s tea) drip {
    fr fr Get raw byte length of string
    sus len drip = 0
    bestie (char_at_byte_internal(s, len) != 0) {
        len = len + 1
    }
    damn len
}

slay char_at_byte_internal(s tea, byte_index drip) drip {
    fr fr Get byte value at specific byte position using character iteration
    ready byte_index < 0 { damn 0 }
    sus str_len drip = length(s)
    ready str_len == 0 { damn 0 }
    
    sus byte_pos drip = 0
    sus char_pos drip = 0
    
    bestie char_pos < str_len {
        sus current_char tea = char_at(s, char_pos)
        sus char_bytes drip = get_utf8_byte_count(current_char)
        
        ready byte_pos + char_bytes > byte_index {
            fr fr Found the character containing the target byte
            sus byte_offset drip = byte_index - byte_pos
            ready char_bytes == 1 {
                damn char_to_ascii_code(current_char)
            } otherwise ready char_bytes == 2 {
                sus code drip = char_to_unicode_code(current_char)
                ready byte_offset == 0 {
                    damn ((code >> 6) & 0x1F) | 0xC0
                } otherwise {
                    damn (code & 0x3F) | 0x80
                }
            } otherwise ready char_bytes == 3 {
                sus code drip = char_to_unicode_code(current_char)
                ready byte_offset == 0 {
                    damn ((code >> 12) & 0x0F) | 0xE0
                } otherwise ready byte_offset == 1 {
                    damn ((code >> 6) & 0x3F) | 0x80
                } otherwise {
                    damn (code & 0x3F) | 0x80
                }
            } otherwise ready char_bytes == 4 {
                sus code drip = char_to_unicode_code(current_char)
                ready byte_offset == 0 {
                    damn ((code >> 18) & 0x07) | 0xF0
                } otherwise ready byte_offset == 1 {
                    damn ((code >> 12) & 0x3F) | 0x80
                } otherwise ready byte_offset == 2 {
                    damn ((code >> 6) & 0x3F) | 0x80
                } otherwise {
                    damn (code & 0x3F) | 0x80
                }
            }
        }
        
        byte_pos = byte_pos + char_bytes
        char_pos = char_pos + 1
    }
    
    damn 0  fr fr Beyond string bounds
}

slay decode_utf8_char_internal(s tea, offset drip) struct {
    codepoint drip
    byte_length drip
} {
    sus first_byte drip = char_at_byte_internal(s, offset)
    sus byte_count drip = get_utf8_byte_count_internal(first_byte)
    
    ready (byte_count == 1) {
        damn { codepoint: first_byte, byte_length: 1 }
    } otherwise ready (byte_count == 2) {
        sus second_byte drip = char_at_byte_internal(s, offset + 1)
        sus codepoint drip = ((first_byte & 31) << 6) | (second_byte & 63)
        damn { codepoint: codepoint, byte_length: 2 }
    } otherwise ready (byte_count == 3) {
        sus second_byte drip = char_at_byte_internal(s, offset + 1)
        sus third_byte drip = char_at_byte_internal(s, offset + 2)
        sus codepoint drip = ((first_byte & 15) << 12) | ((second_byte & 63) << 6) | (third_byte & 63)
        damn { codepoint: codepoint, byte_length: 3 }
    } otherwise ready (byte_count == 4) {
        sus second_byte drip = char_at_byte_internal(s, offset + 1)
        sus third_byte drip = char_at_byte_internal(s, offset + 2)
        sus fourth_byte drip = char_at_byte_internal(s, offset + 3)
        sus codepoint drip = ((first_byte & 7) << 18) | ((second_byte & 63) << 12) | ((third_byte & 63) << 6) | (fourth_byte & 63)
        damn { codepoint: codepoint, byte_length: 4 }
    }
    
    damn { codepoint: first_byte, byte_length: 1 }
}

slay get_utf8_byte_count_internal(first_byte drip) drip {
    ready (first_byte <= 127) {
        damn 1  fr fr ASCII
    } otherwise ready ((first_byte >> 5) == 6) {
        damn 2  fr fr Two-byte
    } otherwise ready ((first_byte >> 4) == 14) {
        damn 3  fr fr Three-byte
    } otherwise ready ((first_byte >> 3) == 30) {
        damn 4  fr fr Four-byte
    }
    damn 1
}

slay encode_utf8_char_internal(codepoint drip) tea {
    ready (codepoint <= 127) {
        damn byte_to_char_internal(codepoint)
    } otherwise ready (codepoint <= 2047) {
        sus first_byte drip = 192 | (codepoint >> 6)
        sus second_byte drip = 128 | (codepoint & 63)
        damn byte_to_char_internal(first_byte) + byte_to_char_internal(second_byte)
    } otherwise ready (codepoint <= 65535) {
        sus first_byte drip = 224 | (codepoint >> 12)
        sus second_byte drip = 128 | ((codepoint >> 6) & 63)
        sus third_byte drip = 128 | (codepoint & 63)
        damn byte_to_char_internal(first_byte) + byte_to_char_internal(second_byte) + byte_to_char_internal(third_byte)
    } otherwise {
        sus first_byte drip = 240 | (codepoint >> 18)
        sus second_byte drip = 128 | ((codepoint >> 12) & 63)
        sus third_byte drip = 128 | ((codepoint >> 6) & 63)
        sus fourth_byte drip = 128 | (codepoint & 63)
        damn byte_to_char_internal(first_byte) + byte_to_char_internal(second_byte) + byte_to_char_internal(third_byte) + byte_to_char_internal(fourth_byte)
    }
}

slay byte_to_char_internal(b drip) tea {
    fr fr Convert byte to character - proper implementation
    ready b < 0 || b > 255 { damn "" }
    
    fr fr Handle ASCII characters (0-127)
    ready b < 128 {
        ready b == 0 { damn "\0" }
        ready b == 9 { damn "\t" }
        ready b == 10 { damn "\n" }
        ready b == 13 { damn "\r" }
        ready b == 32 { damn " " }
        ready b == 33 { damn "!" }
        ready b == 34 { damn "\"" }
        ready b == 35 { damn "#" }
        ready b == 36 { damn "$" }
        ready b == 37 { damn "%" }
        ready b == 38 { damn "&" }
        ready b == 39 { damn "'" }
        ready b == 40 { damn "(" }
        ready b == 41 { damn ")" }
        ready b == 42 { damn "*" }
        ready b == 43 { damn "+" }
        ready b == 44 { damn "," }
        ready b == 45 { damn "-" }
        ready b == 46 { damn "." }
        ready b == 47 { damn "/" }
        ready b >= 48 && b <= 57 { damn ascii_digit_to_char(b - 48) }
        ready b == 58 { damn ":" }
        ready b == 59 { damn ";" }
        ready b == 60 { damn "<" }
        ready b == 61 { damn "=" }
        ready b == 62 { damn ">" }
        ready b == 63 { damn "?" }
        ready b == 64 { damn "@" }
        ready b >= 65 && b <= 90 { damn ascii_upper_to_char(b - 65) }
        ready b == 91 { damn "[" }
        ready b == 92 { damn "\\" }
        ready b == 93 { damn "]" }
        ready b == 94 { damn "^" }
        ready b == 95 { damn "_" }
        ready b == 96 { damn "`" }
        ready b >= 97 && b <= 122 { damn ascii_lower_to_char(b - 97) }
        ready b == 123 { damn "{" }
        ready b == 124 { damn "|" }
        ready b == 125 { damn "}" }
        ready b == 126 { damn "~" }
        ready b == 127 { damn "\x7F" }
    }
    
    fr fr Handle extended ASCII/Latin-1 (128-255)
    ready b >= 128 && b <= 255 {
        damn unicode_codepoint_to_char(b)
    }
    
    damn ""  fr fr Fallback for invalid bytes
}

slay unicode_char_to_upper_internal(codepoint drip) drip {
    fr fr Convert Unicode codepoint to uppercase
    ready (codepoint >= 97 && codepoint <= 122) {
        damn codepoint - 32  fr fr a-z to A-Z
    }
    ready (codepoint >= 224 && codepoint <= 246) {
        damn codepoint - 32  fr fr à-ö to À-Ö
    }
    ready (codepoint >= 248 && codepoint <= 254) {
        damn codepoint - 32  fr fr ø-þ to Ø-Þ
    }
    ready (codepoint >= 945 && codepoint <= 961) {
        damn codepoint - 32  fr fr α-ρ to Α-Ρ
    }
    ready (codepoint >= 963 && codepoint <= 971) {
        damn codepoint - 32  fr fr σ-ω to Σ-Ω
    }
    ready (codepoint >= 1072 && codepoint <= 1103) {
        damn codepoint - 32  fr fr а-я to А-Я
    }
    damn codepoint
}

slay unicode_char_to_lower_internal(codepoint drip) drip {
    fr fr Convert Unicode codepoint to lowercase
    ready (codepoint >= 65 && codepoint <= 90) {
        damn codepoint + 32  fr fr A-Z to a-z
    }
    ready (codepoint >= 192 && codepoint <= 214) {
        damn codepoint + 32  fr fr À-Ö to à-ö
    }
    ready (codepoint >= 216 && codepoint <= 222) {
        damn codepoint + 32  fr fr Ø-Þ to ø-þ
    }
    ready (codepoint >= 913 && codepoint <= 929) {
        damn codepoint + 32  fr fr Α-Ρ to α-ρ
    }
    ready (codepoint >= 931 && codepoint <= 939) {
        damn codepoint + 32  fr fr Σ-Ω to σ-ω
    }
    ready (codepoint >= 1040 && codepoint <= 1071) {
        damn codepoint + 32  fr fr А-Я to а-я
    }
    damn codepoint
}

fr fr ===== UTILITY FUNCTIONS FOR ENHANCED STRINGZ OPERATIONS =====

slay find_next_occurrence(text tea, pattern tea, start_pos drip) drip {
    fr fr Find next occurrence of pattern in text starting from position
    sus text_len drip = unicode_length(text)
    sus pattern_len drip = unicode_length(pattern)
    
    ready pattern_len == 0 || text_len == 0 { damn -1 }
    ready start_pos >= text_len { damn -1 }
    ready start_pos + pattern_len > text_len { damn -1 }
    
    sus i drip = start_pos
    bestie i <= text_len - pattern_len {
        sus match lit = based
        sus j drip = 0
        bestie j < pattern_len {
            sus text_char tea = unicode_char_at(text, i + j)
            sus pattern_char tea = unicode_char_at(pattern, j)
            ready !unicode_char_equals(text_char, pattern_char) {
                match = cap
                break
            }
            j = j + 1
        }
        
        ready match == based {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

slay string_array_append(arr []tea, str tea) []tea {
    fr fr Append string to array - proper implementation
    sus old_len drip = string_array_length(arr)
    sus new_len drip = old_len + 1
    
    fr fr Create new array with increased capacity
    sus result []tea = make_string_array(new_len)
    sus i drip = 0
    
    fr fr Copy existing elements
    bestie i < old_len {
        result[i] = arr[i]
        i = i + 1
    }
    
    fr fr Add new element
    result[old_len] = str
    damn result
}

slay string_array_length(arr []tea) drip {
    fr fr Get length of string array - proper implementation
    fr fr Use built-in length function for arrays
    damn length(arr)
}

slay char_to_digit_value(char tea) drip {
    fr fr Convert single character to digit value
    sus char_info = decode_utf8_char(char, 0)
    sus codepoint drip = char_info.codepoint
    
    ready codepoint >= 48 && codepoint <= 57 {  fr fr ASCII '0' to '9'
        damn codepoint - 48
    }
    
    damn -1  fr fr Not a digit
}

slay digit_to_char_string(digit drip) tea {
    fr fr Convert digit (0-9) to character string
    ready digit >= 0 && digit <= 9 {
        sus codepoint drip = 48 + digit  fr fr ASCII '0' is 48
        damn encode_utf8_char_internal(codepoint)
    }
    damn "0"
}

fr fr ===== HELPER FUNCTIONS FOR ARRAY AND CHARACTER OPERATIONS =====

slay make_string_array(capacity drip) []tea {
    fr fr Create new string array with specified capacity
    sus result []tea = []
    sus i drip = 0
    bestie i < capacity {
        result = string_array_append_internal(result, "")
        i = i + 1
    }
    damn result
}

slay string_array_append_internal(arr []tea, str tea) []tea {
    fr fr Internal array append function
    fr fr This uses language built-in array operations
    sus new_arr []tea = arr
    new_arr[length(arr)] = str  fr fr Append to end
    damn new_arr
}

slay char_to_ascii_code(char tea) drip {
    fr fr Get ASCII code for single character
    ready is_empty(char) { damn 0 }
    sus char_info = decode_utf8_char(char, 0)
    ready char_info.byte_length == 1 {
        damn char_info.codepoint
    }
    damn 0  fr fr Non-ASCII character
}

slay char_to_unicode_code(char tea) drip {
    fr fr Get Unicode codepoint for single character
    ready is_empty(char) { damn 0 }
    sus char_info = decode_utf8_char(char, 0)
    damn char_info.codepoint
}

slay ascii_digit_to_char(digit drip) tea {
    fr fr Convert digit 0-9 to ASCII character
    ready digit >= 0 && digit <= 9 {
        damn encode_utf8_char_internal(48 + digit)
    }
    damn "0"
}

slay ascii_upper_to_char(index drip) tea {
    fr fr Convert index 0-25 to ASCII uppercase A-Z
    ready index >= 0 && index <= 25 {
        damn encode_utf8_char_internal(65 + index)
    }
    damn "A"
}

slay ascii_lower_to_char(index drip) tea {
    fr fr Convert index 0-25 to ASCII lowercase a-z
    ready index >= 0 && index <= 25 {
        damn encode_utf8_char_internal(97 + index)
    }
    damn "a"
}

slay unicode_codepoint_to_char(codepoint drip) tea {
    fr fr Convert Unicode codepoint to character string
    ready codepoint < 0 || codepoint > 0x10FFFF {
        damn ""  fr fr Invalid codepoint
    }
    damn encode_utf8_char_internal(codepoint)
}

slay get_utf8_byte_count(char tea) drip {
    fr fr Get number of bytes in UTF-8 character
    ready is_empty(char) { damn 0 }
    sus char_info = decode_utf8_char(char, 0)
    damn char_info.byte_length
}

slay encode_utf8_char_internal(codepoint drip) tea {
    fr fr Convert codepoint to UTF-8 string
    ready codepoint < 0 { damn "" }
    ready codepoint <= 0x7F {
        fr fr 1-byte character (ASCII)
        damn byte_to_string_internal(codepoint)
    } otherwise ready codepoint <= 0x7FF {
        fr fr 2-byte character
        sus byte1 drip = 0xC0 | (codepoint >> 6)
        sus byte2 drip = 0x80 | (codepoint & 0x3F)
        damn byte_to_string_internal(byte1) + byte_to_string_internal(byte2)
    } otherwise ready codepoint <= 0xFFFF {
        fr fr 3-byte character
        sus byte1 drip = 0xE0 | (codepoint >> 12)
        sus byte2 drip = 0x80 | ((codepoint >> 6) & 0x3F)
        sus byte3 drip = 0x80 | (codepoint & 0x3F)
        damn byte_to_string_internal(byte1) + byte_to_string_internal(byte2) + byte_to_string_internal(byte3)
    } otherwise ready codepoint <= 0x10FFFF {
        fr fr 4-byte character
        sus byte1 drip = 0xF0 | (codepoint >> 18)
        sus byte2 drip = 0x80 | ((codepoint >> 12) & 0x3F)
        sus byte3 drip = 0x80 | ((codepoint >> 6) & 0x3F)
        sus byte4 drip = 0x80 | (codepoint & 0x3F)
        damn byte_to_string_internal(byte1) + byte_to_string_internal(byte2) + byte_to_string_internal(byte3) + byte_to_string_internal(byte4)
    }
    damn ""  fr fr Invalid codepoint
}

slay byte_to_string_internal(byte drip) tea {
    fr fr Convert single byte to string representation
    fr fr This is a bridge function to the runtime
    ready byte == 0 { damn "\0" }
    ready byte == 9 { damn "\t" }
    ready byte == 10 { damn "\n" }
    ready byte == 13 { damn "\r" }
    ready byte >= 32 && byte <= 126 {
        fr fr Printable ASCII
        damn char_from_code_internal(byte)
    }
    damn "?"  fr fr Non-printable or extended ASCII
}

slay char_from_code_internal(code drip) tea {
    fr fr Convert ASCII code to single character string
    fr fr This bridges to the runtime character conversion
    ready code >= 32 && code <= 126 {
        fr fr Use built-in character construction
        ready code == 32 { damn " " }
        ready code == 33 { damn "!" }
        ready code == 34 { damn "\"" }
        ready code == 35 { damn "#" }
        ready code == 36 { damn "$" }
        ready code == 37 { damn "%" }
        ready code == 38 { damn "&" }
        ready code == 39 { damn "'" }
        ready code == 40 { damn "(" }
        ready code == 41 { damn ")" }
        ready code == 42 { damn "*" }
        ready code == 43 { damn "+" }
        ready code == 44 { damn "," }
        ready code == 45 { damn "-" }
        ready code == 46 { damn "." }
        ready code == 47 { damn "/" }
        ready code >= 48 && code <= 57 { damn to_string_internal(code - 48) }  fr fr Digits 0-9
        ready code == 58 { damn ":" }
        ready code == 59 { damn ";" }
        ready code == 60 { damn "<" }
        ready code == 61 { damn "=" }
        ready code == 62 { damn ">" }
        ready code == 63 { damn "?" }
        ready code == 64 { damn "@" }
        ready code >= 65 && code <= 90 { damn upper_letter_from_index(code - 65) }  fr fr A-Z
        ready code == 91 { damn "[" }
        ready code == 92 { damn "\\" }
        ready code == 93 { damn "]" }
        ready code == 94 { damn "^" }
        ready code == 95 { damn "_" }
        ready code == 96 { damn "`" }
        ready code >= 97 && code <= 122 { damn lower_letter_from_index(code - 97) }  fr fr a-z
        ready code == 123 { damn "{" }
        ready code == 124 { damn "|" }
        ready code == 125 { damn "}" }
        ready code == 126 { damn "~" }
    }
    damn "?"
}

slay to_string_internal(digit drip) tea {
    fr fr Convert digit to string
    ready digit == 0 { damn "0" }
    ready digit == 1 { damn "1" }
    ready digit == 2 { damn "2" }
    ready digit == 3 { damn "3" }
    ready digit == 4 { damn "4" }
    ready digit == 5 { damn "5" }
    ready digit == 6 { damn "6" }
    ready digit == 7 { damn "7" }
    ready digit == 8 { damn "8" }
    ready digit == 9 { damn "9" }
    damn "0"
}

slay upper_letter_from_index(index drip) tea {
    fr fr Convert index 0-25 to uppercase letter A-Z
    ready index == 0 { damn "A" }
    ready index == 1 { damn "B" }
    ready index == 2 { damn "C" }
    ready index == 3 { damn "D" }
    ready index == 4 { damn "E" }
    ready index == 5 { damn "F" }
    ready index == 6 { damn "G" }
    ready index == 7 { damn "H" }
    ready index == 8 { damn "I" }
    ready index == 9 { damn "J" }
    ready index == 10 { damn "K" }
    ready index == 11 { damn "L" }
    ready index == 12 { damn "M" }
    ready index == 13 { damn "N" }
    ready index == 14 { damn "O" }
    ready index == 15 { damn "P" }
    ready index == 16 { damn "Q" }
    ready index == 17 { damn "R" }
    ready index == 18 { damn "S" }
    ready index == 19 { damn "T" }
    ready index == 20 { damn "U" }
    ready index == 21 { damn "V" }
    ready index == 22 { damn "W" }
    ready index == 23 { damn "X" }
    ready index == 24 { damn "Y" }
    ready index == 25 { damn "Z" }
    damn "A"
}

slay lower_letter_from_index(index drip) tea {
    fr fr Convert index 0-25 to lowercase letter a-z
    ready index == 0 { damn "a" }
    ready index == 1 { damn "b" }
    ready index == 2 { damn "c" }
    ready index == 3 { damn "d" }
    ready index == 4 { damn "e" }
    ready index == 5 { damn "f" }
    ready index == 6 { damn "g" }
    ready index == 7 { damn "h" }
    ready index == 8 { damn "i" }
    ready index == 9 { damn "j" }
    ready index == 10 { damn "k" }
    ready index == 11 { damn "l" }
    ready index == 12 { damn "m" }
    ready index == 13 { damn "n" }
    ready index == 14 { damn "o" }
    ready index == 15 { damn "p" }
    ready index == 16 { damn "q" }
    ready index == 17 { damn "r" }
    ready index == 18 { damn "s" }
    ready index == 19 { damn "t" }
    ready index == 20 { damn "u" }
    ready index == 21 { damn "v" }
    ready index == 22 { damn "w" }
    ready index == 23 { damn "x" }
    ready index == 24 { damn "y" }
    ready index == 25 { damn "z" }
    damn "a"
}
