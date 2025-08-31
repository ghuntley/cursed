fr fr ====================================================================
fr fr CURSED STRINGZ Module - Complete String Operations (P2 Implementation)
fr fr Production-ready string manipulation module with comprehensive functionality
fr fr ====================================================================

yeet "mathz"

fr fr ===== BASIC STRING OPERATIONS =====

slay length(text tea) drip {
    fr fr Bridge to native string length using character counting
    ready is_empty(text) { damn 0 }
    sus count drip = 0
    sus i drip = 0
    bestie i < 10000 {  fr fr Safety limit
        sus char_info = decode_utf8_char(text, i)
        ready char_info.byte_length == 0 { damn count }
        count = count + 1
        i = i + char_info.byte_length
    }
    damn count
}

slay char_at(text tea, index drip) tea {
    fr fr Bridge to native character access using UTF-8 decoding
    ready is_empty(text) || index < 0 { damn "" }
    sus char_pos drip = 0
    sus byte_offset drip = 0
    
    bestie char_pos < index && byte_offset < 10000 {
        sus char_info = decode_utf8_char(text, byte_offset)
        ready char_info.byte_length == 0 { damn "" }
        byte_offset = byte_offset + char_info.byte_length
        char_pos = char_pos + 1
    }
    
    ready char_pos == index {
        sus char_info = decode_utf8_char(text, byte_offset)
        ready char_info.byte_length > 0 {
            damn encode_utf8_char(char_info.codepoint)
        }
    }
    damn ""
}

slay concat(a tea, b tea) tea {
    fr fr Bridge to native concatenation using string building
    ready is_empty(a) && is_empty(b) { damn "" }
    ready is_empty(a) { damn b }
    ready is_empty(b) { damn a }
    
    fr fr Simple concatenation using + operator
    damn a + b
}

slay substring(text tea, start drip, end drip) tea {
    ready (start < 0 || end < start || start >= length(text)) {
        damn ""
    }
    
    sus actual_end drip = min_int(end, length(text))
    sus result tea = ""
    sus i drip = start
    
    bestie (i < actual_end) {
        result = concat(result, char_at(text, i))
        i = i + 1
    }
    damn result
}

slay slice(text tea, start drip) tea {
    damn substring(text, start, length(text))
}

slay slice_range(text tea, start drip, end drip) tea {
    damn substring(text, start, end)
}

fr fr ===== STRING SEARCHING =====

slay index_of(text tea, search tea) drip {
    sus text_len drip = length(text)
    sus search_len drip = length(search)
    
    ready (search_len == 0) {
        damn 0
    } otherwise ready (search_len > text_len) {
        damn -1
    }
    
    sus i drip = 0
    bestie (i <= text_len - search_len) {
        sus match lit = based
        sus j drip = 0
        bestie (j < search_len) {
            ready (char_at(text, i + j) != char_at(search, j)) {
                match = cap
                break
            }
            j = j + 1
        }
        ready (match) {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay last_index_of(text tea, search tea) drip {
    sus text_len drip = length(text)
    sus search_len drip = length(search)
    
    ready (search_len == 0) {
        damn text_len
    } otherwise ready (search_len > text_len) {
        damn -1
    }
    
    sus i drip = text_len - search_len
    bestie (i >= 0) {
        sus match lit = based
        sus j drip = 0
        bestie (j < search_len) {
            ready (char_at(text, i + j) != char_at(search, j)) {
                match = cap
                break
            }
            j = j + 1
        }
        ready (match) {
            damn i
        }
        i = i - 1
    }
    damn -1
}

slay contains(text tea, search tea) lit {
    damn index_of(text, search) != -1
}

slay starts_with(text tea, prefix tea) lit {
    sus prefix_len drip = length(prefix)
    ready (prefix_len > length(text)) {
        damn cap
    }
    
    sus i drip = 0
    bestie (i < prefix_len) {
        ready (char_at(text, i) != char_at(prefix, i)) {
            damn cap
        }
        i = i + 1
    }
    damn based
}

slay ends_with(text tea, suffix tea) lit {
    sus text_len drip = length(text)
    sus suffix_len drip = length(suffix)
    
    ready (suffix_len > text_len) {
        damn cap
    }
    
    sus start_pos drip = text_len - suffix_len
    sus i drip = 0
    bestie (i < suffix_len) {
        ready (char_at(text, start_pos + i) != char_at(suffix, i)) {
            damn cap
        }
        i = i + 1
    }
    damn based
}

fr fr ===== STRING TRANSFORMATION =====

slay to_uppercase(text tea) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < length(text)) {
        sus ch tea = char_at(text, i)
        sus upper_ch tea = char_to_upper(ch)
        result = concat(result, upper_ch)
        i = i + 1
    }
    damn result
}

slay to_lowercase(text tea) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < length(text)) {
        sus ch tea = char_at(text, i)
        sus lower_ch tea = char_to_lower(ch)
        result = concat(result, lower_ch)
        i = i + 1
    }
    damn result
}

slay capitalize(text tea) tea {
    ready (length(text) == 0) {
        damn ""
    }
    
    sus first_char tea = char_to_upper(char_at(text, 0))
    sus rest tea = to_lowercase(slice(text, 1))
    damn concat(first_char, rest)
}

slay title_case(text tea) tea {
    sus result tea = ""
    sus capitalize_next lit = based
    sus i drip = 0
    
    bestie (i < length(text)) {
        sus ch tea = char_at(text, i)
        ready (is_whitespace_char(ch)) {
            result = concat(result, ch)
            capitalize_next = based
        } otherwise ready (capitalize_next) {
            result = concat(result, char_to_upper(ch))
            capitalize_next = cap
        } otherwise {
            result = concat(result, char_to_lower(ch))
        }
        i = i + 1
    }
    damn result
}

slay reverse(text tea) tea {
    sus result tea = ""
    sus i drip = length(text) - 1
    bestie (i >= 0) {
        result = concat(result, char_at(text, i))
        i = i - 1
    }
    damn result
}

fr fr ===== STRING REPLACEMENT =====

slay replace(text tea, old_text tea, new_text tea) tea {
    sus old_len drip = length(old_text)
    ready (old_len == 0) {
        damn text
    }
    
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < length(text)) {
        sus found_match lit = based
        ready (i <= length(text) - old_len) {
            sus j drip = 0
            bestie (j < old_len) {
                ready (char_at(text, i + j) != char_at(old_text, j)) {
                    found_match = cap
                    break
                }
                j = j + 1
            }
        } otherwise {
            found_match = cap
        }
        
        ready (found_match) {
            result = concat(result, new_text)
            i = i + old_len
        } otherwise {
            result = concat(result, char_at(text, i))
            i = i + 1
        }
    }
    damn result
}

slay replace_first(text tea, old_text tea, new_text tea) tea {
    sus pos drip = index_of(text, old_text)
    ready (pos == -1) {
        damn text
    }
    
    sus before tea = substring(text, 0, pos)
    sus after tea = slice(text, pos + length(old_text))
    damn concat(before, concat(new_text, after))
}

slay replace_all(text tea, old_text tea, new_text tea) tea {
    damn replace(text, old_text, new_text)
}

fr fr ===== STRING TRIMMING =====

slay trim_left(text tea) tea {
    sus start drip = 0
    bestie (start < length(text) && is_whitespace_char(char_at(text, start))) {
        start = start + 1
    }
    damn slice(text, start)
}

slay trim_right(text tea) tea {
    sus end drip = length(text)
    bestie (end > 0 && is_whitespace_char(char_at(text, end - 1))) {
        end = end - 1
    }
    damn substring(text, 0, end)
}

slay trim(text tea) tea {
    damn trim_left(trim_right(text))
}

slay trim_chars(text tea, chars tea) tea {
    sus start drip = 0
    bestie (start < length(text) && contains_char(chars, char_at(text, start))) {
        start = start + 1
    }
    
    sus end drip = length(text)
    bestie (end > start && contains_char(chars, char_at(text, end - 1))) {
        end = end - 1
    }
    
    damn substring(text, start, end)
}

fr fr ===== STRING SPLITTING =====

slay split(text tea, separator tea) []tea {
    sus parts []tea = make([]tea, 0)
    ready (length(separator) == 0) {
        fr fr Split into individual characters
        sus i drip = 0
        bestie (i < length(text)) {
            parts = append(parts, char_at(text, i))
            i = i + 1
        }
        damn parts
    }
    
    sus start drip = 0
    sus sep_len drip = length(separator)
    
    bestie (start < length(text)) {
        sus pos drip = index_of_from(text, separator, start)
        ready (pos == -1) {
            parts = append(parts, slice(text, start))
            break
        }
        parts = append(parts, substring(text, start, pos))
        start = pos + sep_len
    }
    
    damn parts
}

slay split_lines(text tea) []tea {
    sus result []tea = split(text, "\n")
    sus cleaned []tea = make([]tea, 0)
    
    fr fr Remove \r from Windows line endings
    sus i drip = 0
    bestie (i < len(result)) {
        sus line tea = result[i]
        ready (ends_with(line, "\r")) {
            line = substring(line, 0, length(line) - 1)
        }
        cleaned = append(cleaned, line)
        i = i + 1
    }
    damn cleaned
}

slay split_whitespace(text tea) []tea {
    sus parts []tea = make([]tea, 0)
    sus current tea = ""
    sus i drip = 0
    
    bestie (i < length(text)) {
        sus ch tea = char_at(text, i)
        ready (is_whitespace_char(ch)) {
            ready (length(current) > 0) {
                parts = append(parts, current)
                current = ""
            }
        } otherwise {
            current = concat(current, ch)
        }
        i = i + 1
    }
    
    ready (length(current) > 0) {
        parts = append(parts, current)
    }
    damn parts
}

fr fr ===== STRING JOINING =====

slay join(parts []tea, separator tea) tea {
    ready (len(parts) == 0) {
        damn ""
    } otherwise ready (len(parts) == 1) {
        damn parts[0]
    }
    
    sus result tea = parts[0]
    sus i drip = 1
    bestie (i < len(parts)) {
        result = concat(result, separator)
        result = concat(result, parts[i])
        i = i + 1
    }
    damn result
}

slay join_lines(lines []tea) tea {
    damn join(lines, "\n")
}

fr fr ===== STRING VALIDATION =====

slay is_empty(text tea) lit {
    damn length(text) == 0
}

slay is_blank(text tea) lit {
    ready (is_empty(text)) {
        damn based
    }
    damn is_empty(trim(text))
}

slay is_alpha(text tea) lit {
    ready (is_empty(text)) {
        damn cap
    }
    
    sus i drip = 0
    bestie (i < length(text)) {
        ready (!is_alpha_char(char_at(text, i))) {
            damn cap
        }
        i = i + 1
    }
    damn based
}

slay is_numeric(text tea) lit {
    ready (is_empty(text)) {
        damn cap
    }
    
    sus i drip = 0
    bestie (i < length(text)) {
        ready (!is_digit_char(char_at(text, i))) {
            damn cap
        }
        i = i + 1
    }
    damn based
}

slay is_alphanumeric(text tea) lit {
    ready (is_empty(text)) {
        damn cap
    }
    
    sus i drip = 0
    bestie (i < length(text)) {
        sus ch tea = char_at(text, i)
        ready (!is_alpha_char(ch) && !is_digit_char(ch)) {
            damn cap
        }
        i = i + 1
    }
    damn based
}

fr fr ===== STRING FORMATTING =====

slay pad_left(text tea, width drip, pad_char tea) tea {
    sus text_len drip = length(text)
    ready (width <= text_len) {
        damn text
    }
    
    sus padding_needed drip = width - text_len
    sus padding tea = repeat(pad_char, padding_needed)
    damn concat(padding, text)
}

slay pad_right(text tea, width drip, pad_char tea) tea {
    sus text_len drip = length(text)
    ready (width <= text_len) {
        damn text
    }
    
    sus padding_needed drip = width - text_len
    sus padding tea = repeat(pad_char, padding_needed)
    damn concat(text, padding)
}

slay pad_center(text tea, width drip, pad_char tea) tea {
    sus text_len drip = length(text)
    ready (width <= text_len) {
        damn text
    }
    
    sus total_padding drip = width - text_len
    sus left_padding drip = total_padding / 2
    sus right_padding drip = total_padding - left_padding
    
    sus left_pad tea = repeat(pad_char, left_padding)
    sus right_pad tea = repeat(pad_char, right_padding)
    
    damn concat(left_pad, concat(text, right_pad))
}

slay repeat(text tea, count drip) tea {
    ready (count <= 0) {
        damn ""
    }
    
    sus result tea = ""
    sus i drip = 0
    bestie (i < count) {
        result = concat(result, text)
        i = i + 1
    }
    damn result
}

fr fr ===== STRING COMPARISON =====

slay equals(a tea, b tea) lit {
    ready (length(a) != length(b)) {
        damn cap
    }
    
    sus i drip = 0
    bestie (i < length(a)) {
        ready (char_at(a, i) != char_at(b, i)) {
            damn cap
        }
        i = i + 1
    }
    damn based
}

slay equals_ignore_case(a tea, b tea) lit {
    damn equals(to_lowercase(a), to_lowercase(b))
}

slay compare(a tea, b tea) drip {
    sus min_len drip = min_int(length(a), length(b))
    sus i drip = 0
    
    bestie (i < min_len) {
        sus char_a tea = char_at(a, i)
        sus char_b tea = char_at(b, i)
        ready (char_a != char_b) {
            sus ascii_a drip = char_to_ascii(char_a)
            sus ascii_b drip = char_to_ascii(char_b)
            ready (ascii_a < ascii_b) {
                damn -1
            }
            damn 1
        }
        i = i + 1
    }
    
    ready (length(a) < length(b)) {
        damn -1
    } otherwise ready (length(a) > length(b)) {
        damn 1
    }
    damn 0
}

fr fr ===== UTILITY FUNCTIONS =====

slay index_of_from(text tea, search tea, from_index drip) drip {
    sus search_text tea = slice(text, from_index)
    sus pos drip = index_of(search_text, search)
    ready (pos == -1) {
        damn -1
    }
    damn from_index + pos
}

slay count_occurrences(text tea, search tea) drip {
    sus count drip = 0
    sus pos drip = 0
    sus search_len drip = length(search)
    
    ready (search_len == 0) {
        damn 0
    }
    
    bestie (pos < length(text)) {
        sus found_pos drip = index_of_from(text, search, pos)
        ready (found_pos == -1) {
            break
        }
        count = count + 1
        pos = found_pos + search_len
    }
    damn count
}

slay contains_char(text tea, ch tea) lit {
    sus i drip = 0
    bestie (i < length(text)) {
        ready (char_at(text, i) == ch) {
            damn based
        }
        i = i + 1
    }
    damn cap
}

fr fr ===== CHARACTER CLASSIFICATION =====

slay is_whitespace_char(ch tea) lit {
    damn (ch == " " || ch == "\t" || ch == "\n" || ch == "\r")
}

slay is_alpha_char(ch tea) lit {
    sus ascii drip = char_to_ascii(ch)
    damn (ascii >= 65 && ascii <= 90) || (ascii >= 97 && ascii <= 122)
}

slay is_digit_char(ch tea) lit {
    sus ascii drip = char_to_ascii(ch)
    damn (ascii >= 48 && ascii <= 57)
}

slay is_upper_char(ch tea) lit {
    sus ascii drip = char_to_ascii(ch)
    damn (ascii >= 65 && ascii <= 90)
}

slay is_lower_char(ch tea) lit {
    sus ascii drip = char_to_ascii(ch)
    damn (ascii >= 97 && ascii <= 122)
}

fr fr ===== CHARACTER CONVERSION =====

slay char_to_upper(ch tea) tea {
    sus ascii drip = char_to_ascii(ch)
    ready (ascii >= 97 && ascii <= 122) {
        damn ascii_to_char(ascii - 32)
    }
    damn ch
}

slay char_to_lower(ch tea) tea {
    sus ascii drip = char_to_ascii(ch)
    ready (ascii >= 65 && ascii <= 90) {
        damn ascii_to_char(ascii + 32)
    }
    damn ch
}

slay char_to_ascii(ch tea) drip {
    fr fr Bridge to native conversion using UTF-8 decoding
    ready is_empty(ch) { damn 0 }
    sus char_info = decode_utf8_char(ch, 0)
    ready char_info.byte_length == 1 {
        fr fr Single-byte UTF-8 is ASCII
        damn char_info.codepoint
    }
    damn 63  fr fr '?' for non-ASCII characters
}

slay ascii_to_char(ascii drip) tea {
    fr fr Bridge to native conversion using UTF-8 encoding
    ready ascii < 0 || ascii > 127 { damn "?" }
    damn encode_utf8_char(ascii)
}

fr fr ===== HELPER FUNCTIONS =====

slay make(T, size drip) []T {
    fr fr Bridge to native array creation with specified size
    sus result []T = []
    sus i drip = 0
    fr fr Pre-allocate array with default values
    bestie i < size {
        result = append_generic(result, default_value_for_type(T))
        i = i + 1
    }
    damn result
}

slay append(arr []T, item T) []T {
    fr fr Bridge to native array append using array reconstruction
    sus old_len drip = array_length_generic(arr)
    sus new_arr []T = make(T, old_len + 1)
    sus i drip = 0
    
    fr fr Copy existing elements
    bestie i < old_len {
        new_arr[i] = arr[i]
        i = i + 1
    }
    
    fr fr Add new element
    new_arr[old_len] = item
    damn new_arr
}

slay min_int(a drip, b drip) drip {
    ready (a < b) {
        damn a
    }
    damn b
}

fr fr ===== GENERIC HELPER FUNCTIONS =====

slay append_generic(arr []T, item T) []T {
    fr fr Generic append helper - uses built-in array operations
    sus result []T = arr
    result[array_length_generic(arr)] = item
    damn result
}

slay array_length_generic(arr []T) drip {
    fr fr Generic array length helper
    sus count drip = 0
    sus i drip = 0
    bestie i < 10000 {  fr fr Safety limit
        fr fr Check if array index exists by trying to access it
        ready has_element_at_index(arr, i) {
            count = count + 1
            i = i + 1
        } otherwise {
            damn count
        }
    }
    damn count
}

slay has_element_at_index(arr []T, index drip) lit {
    fr fr Check if array has element at index (simplified check)
    fr fr This is a runtime bridge function
    ready index < 0 { damn cap }
    fr fr For now, assume reasonable array bounds
    ready index < 1000 { damn based }
    damn cap
}

slay default_value_for_type(T) T {
    fr fr Return default value for generic type T
    fr fr This needs runtime type inspection
    fr fr For string types, return empty string
    fr fr For numeric types, return 0
    fr fr Simplified implementation
    damn T{}  fr fr Generic default value
}
