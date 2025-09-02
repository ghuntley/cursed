fr fr Enhanced STRINGZ Module - Comprehensive String Operations for CURSED
fr fr Production-ready implementation with advanced string manipulation

fr fr ===== CORE STRING OPERATIONS =====

slay length(s tea) normie {
    check s == "" {
        damn 0
    }
    damn runtime_string_length(s)
}

slay len_str(s tea) normie {
    damn length(s)
}

slay is_empty(s tea) lit {
    damn length(s) == 0
}

slay char_at(s tea, index normie) normie {
    check index < 0 || index >= length(s) {
        damn 0
    }
    damn runtime_string_char_at(s, index)
}

slay char_code_at(s tea, index normie) normie {
    damn char_at(s, index)
}

slay substring(s tea, start normie, length normie) tea {
    check start < 0 || start >= length(s) || length <= 0 {
        damn ""
    }
    
    sus end_pos normie = start + length
    check end_pos > length(s) {
        end_pos = length(s)
    }
    
    damn runtime_substring(s, start, end_pos)
}

slay substr(s tea, start normie, length normie) tea {
    damn substring(s, start, length)
}

slay slice(s tea, start normie, end normie) tea {
    check start < 0 {
        start = 0
    }
    check end > length(s) {
        end = length(s)
    }
    check start >= end {
        damn ""
    }
    
    damn runtime_substring(s, start, end)
}

fr fr ===== STRING CONCATENATION =====

slay concat(a tea, b tea) tea {
    damn a + b
}

slay concat_multiple(strings tea[value]) tea {
    check len_array(strings) == 0 {
        damn ""
    }
    
    sus result tea = strings[0]
    sus i normie = 1
    bestie i < len_array(strings) {
        result = result + strings[i]
        i = i + 1
    }
    damn result
}

slay repeat(s tea, count normie) tea {
    check count <= 0 {
        damn ""
    }
    
    sus result tea = ""
    sus i normie = 0
    bestie i < count {
        result = result + s
        i = i + 1
    }
    damn result
}

slay join(strings tea[value], separator tea) tea {
    check len_array(strings) == 0 {
        damn ""
    }
    check len_array(strings) == 1 {
        damn strings[0]
    }
    
    sus result tea = strings[0]
    sus i normie = 1
    bestie i < len_array(strings) {
        result = result + separator + strings[i]
        i = i + 1
    }
    damn result
}

fr fr ===== SEARCHING AND FINDING =====

slay find(s tea, substr tea) normie {
    check substr == "" {
        damn 0
    }
    check length(substr) > length(s) {
        damn -1
    }
    
    sus s_len normie = length(s)
    sus substr_len normie = length(substr)
    sus i normie = 0
    
    bestie i <= s_len - substr_len {
        sus match lit = based
        sus j normie = 0
        
        bestie j < substr_len {
            check char_at(s, i + j) != char_at(substr, j) {
                match = cringe
                ghosted
            }
            j = j + 1
        }
        
        check match {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

slay index_of(s tea, substr tea) normie {
    damn find(s, substr)
}

slay last_index_of(s tea, substr tea) normie {
    check substr == "" {
        damn length(s)
    }
    check length(substr) > length(s) {
        damn -1
    }
    
    sus s_len normie = length(s)
    sus substr_len normie = length(substr)
    sus last_found normie = -1
    sus i normie = 0
    
    bestie i <= s_len - substr_len {
        sus match lit = based
        sus j normie = 0
        
        bestie j < substr_len {
            check char_at(s, i + j) != char_at(substr, j) {
                match = cringe
                ghosted
            }
            j = j + 1
        }
        
        check match {
            last_found = i
        }
        i = i + 1
    }
    
    damn last_found
}

slay contains(s tea, substr tea) lit {
    damn find(s, substr) != -1
}

slay includes(s tea, substr tea) lit {
    damn contains(s, substr)
}

slay starts_with(s tea, prefix tea) lit {
    check length(prefix) > length(s) {
        damn cringe
    }
    check prefix == "" {
        damn based
    }
    
    sus prefix_len normie = length(prefix)
    sus i normie = 0
    bestie i < prefix_len {
        check char_at(s, i) != char_at(prefix, i) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay ends_with(s tea, suffix tea) lit {
    sus s_len normie = length(s)
    sus suffix_len normie = length(suffix)
    
    check suffix_len > s_len {
        damn cringe
    }
    check suffix == "" {
        damn based
    }
    
    sus start_pos normie = s_len - suffix_len
    sus i normie = 0
    bestie i < suffix_len {
        check char_at(s, start_pos + i) != char_at(suffix, i) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

fr fr ===== STRING REPLACEMENT =====

slay replace(s tea, old tea, new tea) tea {
    check old == "" {
        damn s
    }
    
    sus pos normie = find(s, old)
    check pos == -1 {
        damn s
    }
    
    sus before tea = slice(s, 0, pos)
    sus after tea = slice(s, pos + length(old), length(s))
    damn before + new + after
}

slay replace_all(s tea, old tea, new tea) tea {
    check old == "" {
        damn s
    }
    
    sus result tea = s
    sus old_len normie = length(old)
    sus new_len normie = length(new)
    
    bestie based {
        sus pos normie = find(result, old)
        check pos == -1 {
            ghosted
        }
        
        sus before tea = slice(result, 0, pos)
        sus after tea = slice(result, pos + old_len, length(result))
        result = before + new + after
    }
    
    damn result
}

slay replace_first(s tea, old tea, new tea) tea {
    damn replace(s, old, new)
}

slay replace_at(s tea, index normie, length normie, replacement tea) tea {
    check index < 0 || index >= length(s) {
        damn s
    }
    check length <= 0 {
        damn s
    }
    
    sus before tea = slice(s, 0, index)
    sus after tea = slice(s, index + length, length(s))
    damn before + replacement + after
}

fr fr ===== CASE CONVERSION =====

slay to_upper(s tea) tea {
    sus result tea = ""
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c normie = char_at(s, i)
        check c >= 97 && c <= 122 { fr fr a-z
            c = c - 32 fr fr Convert to uppercase
        }
        result = result + char_to_string(c)
        i = i + 1
    }
    damn result
}

slay to_lower(s tea) tea {
    sus result tea = ""
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c normie = char_at(s, i)
        check c >= 65 && c <= 90 { fr fr A-Z
            c = c + 32 fr fr Convert to lowercase
        }
        result = result + char_to_string(c)
        i = i + 1
    }
    damn result
}

slay to_title_case(s tea) tea {
    sus result tea = ""
    sus s_len normie = length(s)
    sus i normie = 0
    sus capitalize_next lit = based
    
    bestie i < s_len {
        sus c normie = char_at(s, i)
        
        check is_alpha_char(c) {
            check capitalize_next {
                check c >= 97 && c <= 122 { fr fr a-z
                    c = c - 32
                }
                capitalize_next = cringe
            } highkey {
                check c >= 65 && c <= 90 { fr fr A-Z
                    c = c + 32
                }
            }
        } highkey is_space_char(c) || is_punctuation_char(c) {
            capitalize_next = based
        }
        
        result = result + char_to_string(c)
        i = i + 1
    }
    damn result
}

slay capitalize(s tea) tea {
    check is_empty(s) {
        damn s
    }
    
    sus first_char normie = char_at(s, 0)
    check first_char >= 97 && first_char <= 122 {
        first_char = first_char - 32
    }
    
    check length(s) == 1 {
        damn char_to_string(first_char)
    }
    
    sus rest tea = slice(s, 1, length(s))
    damn char_to_string(first_char) + rest
}

slay swap_case(s tea) tea {
    sus result tea = ""
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c normie = char_at(s, i)
        check c >= 97 && c <= 122 { fr fr a-z
            c = c - 32 fr fr to uppercase
        } highkey c >= 65 && c <= 90 { fr fr A-Z
            c = c + 32 fr fr to lowercase
        }
        result = result + char_to_string(c)
        i = i + 1
    }
    damn result
}

fr fr ===== WHITESPACE OPERATIONS =====

slay trim(s tea) tea {
    check is_empty(s) {
        damn s
    }
    
    sus s_len normie = length(s)
    sus start normie = 0
    sus end normie = s_len - 1
    
    fr fr Find start position (skip leading whitespace)
    bestie start < s_len && is_space_char(char_at(s, start)) {
        start = start + 1
    }
    
    fr fr Find end position (skip trailing whitespace)
    bestie end >= start && is_space_char(char_at(s, end)) {
        end = end - 1
    }
    
    check start > end {
        damn ""
    }
    
    damn slice(s, start, end + 1)
}

slay trim_left(s tea) tea {
    check is_empty(s) {
        damn s
    }
    
    sus s_len normie = length(s)
    sus start normie = 0
    
    bestie start < s_len && is_space_char(char_at(s, start)) {
        start = start + 1
    }
    
    check start >= s_len {
        damn ""
    }
    
    damn slice(s, start, s_len)
}

slay trim_right(s tea) tea {
    check is_empty(s) {
        damn s
    }
    
    sus s_len normie = length(s)
    sus end normie = s_len - 1
    
    bestie end >= 0 && is_space_char(char_at(s, end)) {
        end = end - 1
    }
    
    check end < 0 {
        damn ""
    }
    
    damn slice(s, 0, end + 1)
}

slay trim_start(s tea) tea {
    damn trim_left(s)
}

slay trim_end(s tea) tea {
    damn trim_right(s)
}

fr fr ===== PADDING OPERATIONS =====

slay pad_left(s tea, target_length normie, pad_char tea) tea {
    sus s_len normie = length(s)
    check s_len >= target_length {
        damn s
    }
    
    sus padding_needed normie = target_length - s_len
    sus padding tea = repeat(pad_char, padding_needed)
    damn padding + s
}

slay pad_right(s tea, target_length normie, pad_char tea) tea {
    sus s_len normie = length(s)
    check s_len >= target_length {
        damn s
    }
    
    sus padding_needed normie = target_length - s_len
    sus padding tea = repeat(pad_char, padding_needed)
    damn s + padding
}

slay pad_start(s tea, target_length normie, pad_char tea) tea {
    damn pad_left(s, target_length, pad_char)
}

slay pad_end(s tea, target_length normie, pad_char tea) tea {
    damn pad_right(s, target_length, pad_char)
}

slay center(s tea, target_length normie, pad_char tea) tea {
    sus s_len normie = length(s)
    check s_len >= target_length {
        damn s
    }
    
    sus total_padding normie = target_length - s_len
    sus left_padding normie = total_padding / 2
    sus right_padding normie = total_padding - left_padding
    
    sus left_pad tea = repeat(pad_char, left_padding)
    sus right_pad tea = repeat(pad_char, right_padding)
    damn left_pad + s + right_pad
}

fr fr ===== STRING SPLITTING =====

slay split(s tea, delimiter tea) tea[value]{
    check delimiter == "" {
        fr fr Split into individual characters
        sus result tea[value] = []
        sus i normie = 0
        bestie i < length(s) {
            sus char_str tea = char_to_string(char_at(s, i))
            result = append_to_array(result, char_str)
            i = i + 1
        }
        damn result
    }
    
    sus result tea[value] = []
    sus s_len normie = length(s)
    sus delim_len normie = length(delimiter)
    sus start normie = 0
    sus i normie = 0
    
    bestie i <= s_len - delim_len {
        check slice(s, i, i + delim_len) == delimiter {
            sus part tea = slice(s, start, i)
            result = append_to_array(result, part)
            i = i + delim_len
            start = i
        } highkey {
            i = i + 1
        }
    }
    
    fr fr Add remaining part
    check start <= s_len {
        sus last_part tea = slice(s, start, s_len)
        result = append_to_array(result, last_part)
    }
    
    damn result
}

slay split_lines(s tea) tea[value]{
    damn split(s, "\n")
}

slay split_words(s tea) tea[value]{
    damn split(trim(s), " ")
}

slay split_at(s tea, index normie) (tea, tea) {
    check index <= 0 {
        damn ("", s)
    }
    check index >= length(s) {
        damn (s, "")
    }
    
    sus left tea = slice(s, 0, index)
    sus right tea = slice(s, index, length(s))
    damn (left, right)
}

fr fr ===== STRING TRANSFORMATION =====

slay reverse(s tea) tea {
    sus s_len normie = length(s)
    check s_len <= 1 {
        damn s
    }
    
    sus result tea = ""
    sus i normie = s_len - 1
    bestie i >= 0 {
        result = result + char_to_string(char_at(s, i))
        i = i - 1
    }
    damn result
}

slay shuffle(s tea) tea {
    sus chars tea[value] = split(s, "")
    sus result tea = ""
    
    fr fr Simple shuffle using current time as seed
    sus seed normie = runtime_get_current_time() % 1000
    sus i normie = 0
    
    bestie i < len_array(chars) {
        seed = (seed * 1103515245 + 12345) % 2147483647
        sus swap_index normie = seed % len_array(chars)
        
        check swap_index != i {
            sus temp tea = chars[i]
            chars = array_set_at(chars, i, chars[swap_index])
            chars = array_set_at(chars, swap_index, temp)
        }
        i = i + 1
    }
    
    damn join(chars, "")
}

slay sort_chars(s tea) tea {
    sus chars tea[value] = split(s, "")
    sus sorted_chars tea[value] = array_sort_strings(chars)
    damn join(sorted_chars, "")
}

fr fr ===== STRING VALIDATION =====

slay is_alpha(s tea) lit {
    check is_empty(s) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i < length(s) {
        check !is_alpha_char(char_at(s, i)) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay is_numeric(s tea) lit {
    check is_empty(s) {
        damn cringe
    }
    
    sus i normie = 0
    sus start normie = 0
    
    fr fr Check for leading minus sign
    check char_at(s, 0) == 45 { fr fr '-'
        start = 1
        check length(s) == 1 {
            damn cringe
        }
    }
    
    i = start
    bestie i < length(s) {
        check !is_digit_char(char_at(s, i)) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay is_alphanumeric(s tea) lit {
    check is_empty(s) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i < length(s) {
        sus c normie = char_at(s, i)
        check !is_alpha_char(c) && !is_digit_char(c) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay is_whitespace(s tea) lit {
    check is_empty(s) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i < length(s) {
        check !is_space_char(char_at(s, i)) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay is_lowercase(s tea) lit {
    check is_empty(s) {
        damn cringe
    }
    
    sus has_alpha lit = cringe
    sus i normie = 0
    bestie i < length(s) {
        sus c normie = char_at(s, i)
        check c >= 65 && c <= 90 { fr fr A-Z
            damn cringe
        }
        check c >= 97 && c <= 122 { fr fr a-z
            has_alpha = based
        }
        i = i + 1
    }
    damn has_alpha
}

slay is_uppercase(s tea) lit {
    check is_empty(s) {
        damn cringe
    }
    
    sus has_alpha lit = cringe
    sus i normie = 0
    bestie i < length(s) {
        sus c normie = char_at(s, i)
        check c >= 97 && c <= 122 { fr fr a-z
            damn cringe
        }
        check c >= 65 && c <= 90 { fr fr A-Z
            has_alpha = based
        }
        i = i + 1
    }
    damn has_alpha
}

fr fr ===== ENCODING AND ESCAPING =====

slay escape_html(s tea) tea {
    sus result tea = replace_all(s, "&", "&amp;")
    result = replace_all(result, "<", "&lt;")
    result = replace_all(result, ">", "&gt;")
    result = replace_all(result, "\"", "&quot;")
    result = replace_all(result, "'", "&#39;")
    damn result
}

slay unescape_html(s tea) tea {
    sus result tea = replace_all(s, "&lt;", "<")
    result = replace_all(result, "&gt;", ">")
    result = replace_all(result, "&quot;", "\"")
    result = replace_all(result, "&#39;", "'")
    result = replace_all(result, "&amp;", "&")
    damn result
}

slay escape_quotes(s tea) tea {
    sus result tea = replace_all(s, "\\", "\\\\")
    result = replace_all(result, "\"", "\\\"")
    result = replace_all(result, "'", "\\'")
    damn result
}

slay unescape_quotes(s tea) tea {
    sus result tea = replace_all(s, "\\'", "'")
    result = replace_all(result, "\\\"", "\"")
    result = replace_all(result, "\\\\", "\\")
    damn result
}

slay url_encode(s tea) tea {
    sus result tea = ""
    sus i normie = 0
    
    bestie i < length(s) {
        sus c normie = char_at(s, i)
        check is_unreserved_url_char(c) {
            result = result + char_to_string(c)
        } highkey c == 32 { fr fr space
            result = result + "%20"
        } highkey {
            result = result + "%" + char_to_hex(c)
        }
        i = i + 1
    }
    damn result
}

slay url_decode(s tea) tea {
    sus result tea = ""
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c normie = char_at(s, i)
        check c == 37 && i + 2 < s_len { fr fr '%'
            sus hex_str tea = slice(s, i + 1, i + 3)
            check hex_str == "20" {
                result = result + " "
            } highkey {
                sus decoded_char normie = hex_to_char(hex_str)
                result = result + char_to_string(decoded_char)
            }
            i = i + 3
        } highkey {
            result = result + char_to_string(c)
            i = i + 1
        }
    }
    damn result
}

fr fr ===== STRING COMPARISON =====

slay equals(a tea, b tea) lit {
    check length(a) != length(b) {
        damn cringe
    }
    damn a == b
}

slay equals_ignore_case(a tea, b tea) lit {
    damn to_lower(a) == to_lower(b)
}

slay compare(a tea, b tea) normie {
    check a == b {
        damn 0
    }
    check a < b {
        damn -1
    }
    damn 1
}

slay compare_ignore_case(a tea, b tea) normie {
    damn compare(to_lower(a), to_lower(b))
}

slay string_compare(a tea, b tea) normie {
    damn compare(a, b)
}

fr fr ===== COUNTING OPERATIONS =====

slay count_occurrences(s tea, substr tea) normie {
    check substr == "" || length(substr) > length(s) {
        damn 0
    }
    
    sus count normie = 0
    sus s_len normie = length(s)
    sus substr_len normie = length(substr)
    sus i normie = 0
    
    bestie i <= s_len - substr_len {
        check slice(s, i, i + substr_len) == substr {
            count = count + 1
            i = i + substr_len fr fr Skip past this occurrence
        } highkey {
            i = i + 1
        }
    }
    
    damn count
}

slay count_chars(s tea) normie {
    damn length(s)
}

slay count_words(s tea) normie {
    sus words tea[value] = split_words(s)
    damn len_array(words)
}

slay count_lines(s tea) normie {
    sus lines tea[value] = split_lines(s)
    damn len_array(lines)
}

fr fr ===== UTILITY HELPER FUNCTIONS =====

slay is_alpha_char(c normie) lit {
    damn (c >= 65 && c <= 90) || (c >= 97 && c <= 122)
}

slay is_digit_char(c normie) lit {
    damn c >= 48 && c <= 57
}

slay is_space_char(c normie) lit {
    damn c == 32 || c == 9 || c == 10 || c == 13
}

slay is_punctuation_char(c normie) lit {
    damn (c >= 33 && c <= 47) || (c >= 58 && c <= 64) || (c >= 91 && c <= 96) || (c >= 123 && c <= 126)
}

slay is_unreserved_url_char(c normie) lit {
    damn is_alpha_char(c) || is_digit_char(c) || c == 45 || c == 46 || c == 95 || c == 126
}

slay char_to_string(c normie) tea {
    damn runtime_char_to_string(c)
}

slay char_to_hex(c normie) tea {
    sus val normie = c
    check val < 16 {
        check val < 10 {
            damn "0" + char_to_string(48 + val)
        }
        damn "0" + char_to_string(55 + val)
    }
    
    sus first_digit normie = val / 16
    sus second_digit normie = val % 16
    
    sus first_char tea = ""
    sus second_char tea = ""
    
    check first_digit < 10 {
        first_char = char_to_string(48 + first_digit)
    } highkey {
        first_char = char_to_string(55 + first_digit)
    }
    
    check second_digit < 10 {
        second_char = char_to_string(48 + second_digit)
    } highkey {
        second_char = char_to_string(55 + second_digit)
    }
    
    damn first_char + second_char
}

slay hex_to_char(hex_str tea) normie {
    check length(hex_str) != 2 {
        damn 0
    }
    
    sus first_char normie = char_at(hex_str, 0)
    sus second_char normie = char_at(hex_str, 1)
    
    sus first_val normie = hex_char_to_value(first_char)
    sus second_val normie = hex_char_to_value(second_char)
    
    damn first_val * 16 + second_val
}

slay hex_char_to_value(c normie) normie {
    check c >= 48 && c <= 57 { fr fr 0-9
        damn c - 48
    }
    check c >= 65 && c <= 70 { fr fr A-F
        damn c - 55
    }
    check c >= 97 && c <= 102 { fr fr a-f
        damn c - 87
    }
    damn 0
}

fr fr ===== ARRAY HELPER FUNCTIONS =====

slay len_array(arr tea[value]) normie {
    damn runtime_array_length(arr)
}

slay append_to_array(arr tea[value], item tea) tea[value]{
    damn runtime_array_append(arr, item)
}

slay array_set_at(arr tea[value], index normie, value tea) tea[value]{
    damn runtime_array_set(arr, index, value)
}

slay array_sort_strings(arr tea[value]) tea[value]{
    damn runtime_array_sort_strings(arr)
}

fr fr ===== RUNTIME INTERFACE FUNCTIONS =====

slay runtime_string_length(s tea) normie {
    damn core.string_length(s)
}

slay runtime_string_char_at(s tea, index normie) normie {
    damn core.string_char_at(s, index)
}

slay runtime_substring(s tea, start normie, end normie) tea {
    damn core.substring(s, start, end)
}

slay runtime_char_to_string(c normie) tea {
    damn core.char_to_string(c)
}

slay runtime_get_current_time() normie {
    damn core.get_current_time()
}

slay runtime_array_length(arr tea[value]) normie {
    damn core.array_length(arr)
}

slay runtime_array_append(arr tea[value], item tea) tea[value]{
    damn core.array_append(arr, item)
}

slay runtime_array_set(arr tea[value], index normie, value tea) tea[value]{
    damn core.array_set(arr, index, value)
}

slay runtime_array_sort_strings(arr tea[value]) tea[value]{
    damn core.array_sort_strings(arr)
}

fr fr ===== LEGACY COMPATIBILITY =====

slay string_length(s tea) normie {
    damn length(s)
}

slay string_concat(a tea, b tea) tea {
    damn concat(a, b)
}

slay string_compare(a tea, b tea) normie {
    damn compare(a, b)
}
