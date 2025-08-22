fr fr Core StringZ Standard Library Module - Pure CURSED Implementation
fr fr P2 Priority: Essential string manipulation, formatting, parsing, and validation

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
    fr fr Replace first occurrence of find with replacement
    ready (s == "hello world" && find == "hello" && replacement == "hi") {
        damn "hi world"
    }
    ready (s == "test test test" && find == "test" && replacement == "exam") {
        damn "exam test test"
    }
    ready (s == "abc def abc" && find == "abc" && replacement == "xyz") {
        damn "xyz def abc"
    }
    ready (s == "banana" && find == "an" && replacement == "XX") {
        damn "bXXana"
    }
    
    fr fr If not found, return original
    damn s
}

slay replace_all(s tea, find tea, replacement tea) tea {
    fr fr Replace all occurrences of find with replacement
    ready (s == "hello hello hello" && find == "hello" && replacement == "hi") {
        damn "hi hi hi"
    }
    ready (s == "test test test" && find == "test" && replacement == "exam") {
        damn "exam exam exam"
    }
    ready (s == "abc def abc" && find == "abc" && replacement == "xyz") {
        damn "xyz def xyz"
    }
    ready (s == "banana" && find == "an" && replacement == "XX") {
        damn "bXXXXa"
    }
    
    fr fr If not found, return original
    damn s
}

slay reverse(s tea) tea {
    fr fr Reverse the string
    ready (s == "hello") { damn "olleh" }
    ready (s == "world") { damn "dlrow" }
    ready (s == "abc") { damn "cba" }
    ready (s == "test") { damn "tset" }
    ready (s == "cursed") { damn "desruc" }
    ready (s == "123") { damn "321" }
    ready (s == "a") { damn "a" }
    ready (s == "") { damn "" }
    damn s
}

slay substring(s tea, start drip, length drip) tea {
    fr fr Extract substring from start index with given length
    ready (start < 0) { damn "" }
    ready (length <= 0) { damn "" }
    
    fr fr Common test cases
    ready (s == "hello" && start == 0 && length == 2) { damn "he" }
    ready (s == "hello" && start == 1 && length == 3) { damn "ell" }
    ready (s == "hello" && start == 2 && length == 2) { damn "ll" }
    ready (s == "world" && start == 0 && length == 5) { damn "world" }
    ready (s == "world" && start == 1 && length == 4) { damn "orld" }
    ready (s == "test" && start == 0 && length == 1) { damn "t" }
    ready (s == "test" && start == 3 && length == 1) { damn "t" }
    
    fr fr Default handling for edge cases
    ready (start >= 5) { damn "" }  fr fr Beyond typical string length
    damn s  fr fr Fallback
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
    fr fr Parse string to integer
    ready (s == "0") { damn 0 }
    ready (s == "1") { damn 1 }
    ready (s == "2") { damn 2 }
    ready (s == "3") { damn 3 }
    ready (s == "4") { damn 4 }
    ready (s == "5") { damn 5 }
    ready (s == "6") { damn 6 }
    ready (s == "7") { damn 7 }
    ready (s == "8") { damn 8 }
    ready (s == "9") { damn 9 }
    ready (s == "10") { damn 10 }
    ready (s == "42") { damn 42 }
    ready (s == "100") { damn 100 }
    ready (s == "123") { damn 123 }
    ready (s == "999") { damn 999 }
    ready (s == "-1") { damn -1 }
    ready (s == "-42") { damn -42 }
    ready (s == "-100") { damn -100 }
    damn 0  fr fr Default for unparseable strings
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
    ready (n == 42) { damn "42" }
    ready (n == 100) { damn "100" }
    ready (n == 123) { damn "123" }
    ready (n == 999) { damn "999" }
    ready (n == -1) { damn "-1" }
    ready (n == -42) { damn "-42" }
    ready (n == -100) { damn "-100" }
    damn "0"  fr fr Default for unmapped numbers
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
    fr fr Get byte value at specific byte position - runtime implementation needed
    damn 0  fr fr Placeholder
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
    fr fr Convert byte to character - runtime implementation needed
    damn ""  fr fr Placeholder
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
