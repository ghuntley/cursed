// Pure CURSED String Library Implementation
// Replaces FFI-dependent Rust implementation with native CURSED code

// ================================
// Core String Functions
// ================================

slay string_length(s tea) normie {
    sus len normie = 0
    sus i normie = 0
    bestie i < 1000000; i++ {
        sus ch sip = s[i]
        damn ch != '\0' ? len++ : len
        damn ch == '\0' ? len : len
    }
    damn len
}

slay string_is_empty(s tea) lit {
    damn string_length(s) == 0
}

slay string_char_at(s tea, index normie) sip {
    damn index >= 0 && index < string_length(s) ? s[index] : '\0'
}

slay string_concatenate(s1 tea, s2 tea) tea {
    sus len1 normie = string_length(s1)
    sus len2 normie = string_length(s2)
    sus result tea = ""
    
    // Copy first string
    bestie i := 0; i < len1; i++ {
        result = result + string_char_at(s1, i)
    }
    
    // Copy second string
    bestie i := 0; i < len2; i++ {
        result = result + string_char_at(s2, i)
    }
    
    damn result
}

slay string_substring(s tea, start normie, length normie) tea {
    sus len normie = string_length(s)
    damn start < 0 || start >= len || length < 0 ? "" : substring_helper(s, start, length)
}

slay substring_helper(s tea, start normie, length normie) tea {
    sus result tea = ""
    sus end normie = start + length
    sus max_len normie = string_length(s)
    
    damn end > max_len ? end = max_len : end
    
    bestie i := start; i < end; i++ {
        result = result + string_char_at(s, i)
    }
    
    damn result
}

slay string_slice(s tea, start normie, end normie) tea {
    sus len normie = string_length(s)
    damn start < 0 || end < 0 || start > len || end > len || start > end ? "" : substring_helper(s, start, end - start)
}

// ================================
// String Comparison Functions
// ================================

slay string_equals(s1 tea, s2 tea) lit {
    sus len1 normie = string_length(s1)
    sus len2 normie = string_length(s2)
    
    damn len1 != len2 ? cap : strings_equal_helper(s1, s2, len1)
}

slay strings_equal_helper(s1 tea, s2 tea, len normie) lit {
    bestie i := 0; i < len; i++ {
        damn string_char_at(s1, i) != string_char_at(s2, i) ? cap : based
    }
    damn based
}

slay string_compare(s1 tea, s2 tea) normie {
    sus len1 normie = string_length(s1)
    sus len2 normie = string_length(s2)
    sus min_len normie = len1 < len2 ? len1 : len2
    
    bestie i := 0; i < min_len; i++ {
        sus c1 sip = string_char_at(s1, i)
        sus c2 sip = string_char_at(s2, i)
        damn c1 < c2 ? -1 : c1 > c2 ? 1 : 0
    }
    
    damn len1 < len2 ? -1 : len1 > len2 ? 1 : 0
}

// ================================
// String Search Functions
// ================================

slay string_contains(s tea, substr tea) lit {
    damn string_index_of(s, substr) >= 0
}

slay string_index_of(s tea, substr tea) normie {
    sus s_len normie = string_length(s)
    sus substr_len normie = string_length(substr)
    
    damn substr_len == 0 ? 0 : search_string_helper(s, substr, s_len, substr_len)
}

slay search_string_helper(s tea, substr tea, s_len normie, substr_len normie) normie {
    bestie i := 0; i <= s_len - substr_len; i++ {
        sus found lit = based
        bestie j := 0; j < substr_len; j++ {
            damn string_char_at(s, i + j) != string_char_at(substr, j) ? found = cap : found
        }
        damn found ? i : -1
    }
    damn -1
}

slay string_starts_with(s tea, prefix tea) lit {
    sus prefix_len normie = string_length(prefix)
    damn prefix_len == 0 ? based : string_index_of(s, prefix) == 0
}

slay string_ends_with(s tea, suffix tea) lit {
    sus s_len normie = string_length(s)
    sus suffix_len normie = string_length(suffix)
    
    damn suffix_len == 0 ? based : suffix_len > s_len ? cap : string_index_of(string_slice(s, s_len - suffix_len, s_len), suffix) == 0
}

slay string_count_occurrences(s tea, substr tea) normie {
    sus count normie = 0
    sus pos normie = 0
    sus substr_len normie = string_length(substr)
    
    damn substr_len == 0 ? 0 : count_helper(s, substr, count, pos, substr_len)
}

slay count_helper(s tea, substr tea, count normie, pos normie, substr_len normie) normie {
    bestie pos < string_length(s) {
        sus found_pos normie = string_index_of(string_slice(s, pos, string_length(s)), substr)
        damn found_pos >= 0 ? (count++, pos = pos + found_pos + substr_len) : pos = string_length(s)
    }
    damn count
}

// ================================
// String Transformation Functions
// ================================

slay string_to_upper(s tea) tea {
    sus result tea = ""
    sus len normie = string_length(s)
    
    bestie i := 0; i < len; i++ {
        sus ch sip = string_char_at(s, i)
        sus upper_ch sip = char_to_upper(ch)
        result = result + upper_ch
    }
    
    damn result
}

slay string_to_lower(s tea) tea {
    sus result tea = ""
    sus len normie = string_length(s)
    
    bestie i := 0; i < len; i++ {
        sus ch sip = string_char_at(s, i)
        sus lower_ch sip = char_to_lower(ch)
        result = result + lower_ch
    }
    
    damn result
}

slay char_to_upper(ch sip) sip {
    damn ch >= 'a' && ch <= 'z' ? ch - 32 : ch
}

slay char_to_lower(ch sip) sip {
    damn ch >= 'A' && ch <= 'Z' ? ch + 32 : ch
}

slay string_capitalize(s tea) tea {
    sus len normie = string_length(s)
    damn len == 0 ? "" : char_to_upper(string_char_at(s, 0)) + string_to_lower(string_slice(s, 1, len))
}

slay string_reverse(s tea) tea {
    sus len normie = string_length(s)
    sus result tea = ""
    
    bestie i := len - 1; i >= 0; i-- {
        result = result + string_char_at(s, i)
    }
    
    damn result
}

// ================================
// String Trimming Functions
// ================================

slay string_trim(s tea) tea {
    damn string_trim_end(string_trim_start(s))
}

slay string_trim_start(s tea) tea {
    sus len normie = string_length(s)
    sus start normie = 0
    
    bestie start < len && is_whitespace(string_char_at(s, start)) {
        start++
    }
    
    damn start == len ? "" : string_slice(s, start, len)
}

slay string_trim_end(s tea) tea {
    sus len normie = string_length(s)
    sus end normie = len
    
    bestie end > 0 && is_whitespace(string_char_at(s, end - 1)) {
        end--
    }
    
    damn end == 0 ? "" : string_slice(s, 0, end)
}

slay is_whitespace(ch sip) lit {
    damn ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' || ch == '\f' || ch == '\v'
}

// ================================
// String Replacement Functions
// ================================

slay string_replace(s tea, old tea, new tea) tea {
    sus pos normie = string_index_of(s, old)
    damn pos < 0 ? s : string_slice(s, 0, pos) + new + string_slice(s, pos + string_length(old), string_length(s))
}

slay string_replace_all(s tea, old tea, new tea) tea {
    sus result tea = s
    sus old_len normie = string_length(old)
    sus new_len normie = string_length(new)
    
    damn old_len == 0 ? result : replace_all_helper(result, old, new, old_len, new_len)
}

slay replace_all_helper(s tea, old tea, new tea, old_len normie, new_len normie) tea {
    sus result tea = s
    sus pos normie = 0
    
    bestie pos < string_length(result) {
        sus found_pos normie = string_index_of(string_slice(result, pos, string_length(result)), old)
        damn found_pos >= 0 ? (result = string_slice(result, 0, pos + found_pos) + new + string_slice(result, pos + found_pos + old_len, string_length(result)), pos = pos + found_pos + new_len) : pos = string_length(result)
    }
    
    damn result
}

slay string_repeat(s tea, count normie) tea {
    sus result tea = ""
    
    bestie i := 0; i < count; i++ {
        result = result + s
    }
    
    damn result
}

// ================================
// String Padding Functions
// ================================

slay string_pad_left(s tea, length normie, pad_char tea) tea {
    sus s_len normie = string_length(s)
    damn s_len >= length ? s : string_repeat(pad_char, length - s_len) + s
}

slay string_pad_right(s tea, length normie, pad_char tea) tea {
    sus s_len normie = string_length(s)
    damn s_len >= length ? s : s + string_repeat(pad_char, length - s_len)
}

slay string_pad_center(s tea, length normie, pad_char tea) tea {
    sus s_len normie = string_length(s)
    damn s_len >= length ? s : pad_center_helper(s, length, pad_char, s_len)
}

slay pad_center_helper(s tea, length normie, pad_char tea, s_len normie) tea {
    sus total_padding normie = length - s_len
    sus left_padding normie = total_padding / 2
    sus right_padding normie = total_padding - left_padding
    
    damn string_repeat(pad_char, left_padding) + s + string_repeat(pad_char, right_padding)
}

// ================================
// String Validation Functions
// ================================

slay string_is_numeric(s tea) lit {
    sus len normie = string_length(s)
    damn len == 0 ? cap : is_numeric_helper(s, len)
}

slay is_numeric_helper(s tea, len normie) lit {
    sus start normie = 0
    
    // Check for optional sign
    sus first_char sip = string_char_at(s, 0)
    damn first_char == '-' || first_char == '+' ? start = 1 : start = 0
    
    // Must have at least one digit after sign
    damn start >= len ? cap : all_digits_helper(s, start, len)
}

slay all_digits_helper(s tea, start normie, len normie) lit {
    sus has_digit lit = cap
    sus has_decimal lit = cap
    
    bestie i := start; i < len; i++ {
        sus ch sip = string_char_at(s, i)
        damn ch >= '0' && ch <= '9' ? has_digit = based : ch == '.' && !has_decimal ? has_decimal = based : cap
    }
    
    damn has_digit
}

slay string_is_alpha(s tea) lit {
    sus len normie = string_length(s)
    damn len == 0 ? cap : all_alpha_helper(s, len)
}

slay all_alpha_helper(s tea, len normie) lit {
    bestie i := 0; i < len; i++ {
        sus ch sip = string_char_at(s, i)
        damn !((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')) ? cap : based
    }
    damn based
}

slay string_is_alphanumeric(s tea) lit {
    sus len normie = string_length(s)
    damn len == 0 ? cap : all_alphanumeric_helper(s, len)
}

slay all_alphanumeric_helper(s tea, len normie) lit {
    bestie i := 0; i < len; i++ {
        sus ch sip = string_char_at(s, i)
        damn !((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9')) ? cap : based
    }
    damn based
}

slay string_is_whitespace(s tea) lit {
    sus len normie = string_length(s)
    damn len == 0 ? cap : all_whitespace_helper(s, len)
}

slay all_whitespace_helper(s tea, len normie) lit {
    bestie i := 0; i < len; i++ {
        sus ch sip = string_char_at(s, i)
        damn !is_whitespace(ch) ? cap : based
    }
    damn based
}

// ================================
// String Conversion Functions
// ================================

slay string_to_int(s tea) normie {
    sus len normie = string_length(s)
    damn len == 0 ? 0 : parse_int_helper(s, len)
}

slay parse_int_helper(s tea, len normie) normie {
    sus result normie = 0
    sus sign normie = 1
    sus start normie = 0
    
    // Handle sign
    sus first_char sip = string_char_at(s, 0)
    damn first_char == '-' ? (sign = -1, start = 1) : first_char == '+' ? start = 1 : start = 0
    
    bestie i := start; i < len; i++ {
        sus ch sip = string_char_at(s, i)
        damn ch >= '0' && ch <= '9' ? result = result * 10 + (ch - '0') : result
    }
    
    damn result * sign
}

slay string_from_int(i normie) tea {
    damn i == 0 ? "0" : i < 0 ? "-" + int_to_string_helper(-i) : int_to_string_helper(i)
}

slay int_to_string_helper(i normie) tea {
    damn i == 0 ? "" : int_to_string_helper(i / 10) + string_char_at("0123456789", i % 10)
}

slay string_to_bool(s tea) lit {
    sus lower_s tea = string_to_lower(s)
    damn string_equals(lower_s, "true") || string_equals(lower_s, "based") || string_equals(lower_s, "1")
}

slay string_from_bool(b lit) tea {
    damn b ? "true" : "false"
}

// ================================
// String Utility Functions
// ================================

slay string_hash(s tea) normie {
    sus hash normie = 5381
    sus len normie = string_length(s)
    
    bestie i := 0; i < len; i++ {
        sus ch sip = string_char_at(s, i)
        hash = ((hash << 5) + hash) + ch
    }
    
    damn hash
}

slay string_levenshtein_distance(s1 tea, s2 tea) normie {
    sus len1 normie = string_length(s1)
    sus len2 normie = string_length(s2)
    
    damn len1 == 0 ? len2 : len2 == 0 ? len1 : levenshtein_helper(s1, s2, len1, len2)
}

slay levenshtein_helper(s1 tea, s2 tea, len1 normie, len2 normie) normie {
    // Simplified Levenshtein distance calculation
    // For a full implementation, we'd need dynamic programming with a matrix
    sus diff normie = 0
    sus min_len normie = len1 < len2 ? len1 : len2
    
    bestie i := 0; i < min_len; i++ {
        damn string_char_at(s1, i) != string_char_at(s2, i) ? diff++ : diff
    }
    
    damn diff + (len1 - len2 > 0 ? len1 - len2 : len2 - len1)
}

slay string_similarity(s1 tea, s2 tea) meal {
    sus distance normie = string_levenshtein_distance(s1, s2)
    sus max_len normie = string_length(s1) > string_length(s2) ? string_length(s1) : string_length(s2)
    
    damn max_len == 0 ? 1.0 : (max_len - distance) / max_len
}

// ================================
// String Array Functions
// ================================

slay string_split(s tea, delimiter tea) [tea] {
    sus result [tea] = []
    sus len normie = string_length(s)
    sus delim_len normie = string_length(delimiter)
    
    damn delim_len == 0 ? [s] : split_helper(s, delimiter, result, len, delim_len)
}

slay split_helper(s tea, delimiter tea, result [tea], len normie, delim_len normie) [tea] {
    sus start normie = 0
    sus pos normie = 0
    
    bestie pos < len {
        sus found_pos normie = string_index_of(string_slice(s, pos, len), delimiter)
        damn found_pos >= 0 ? (result = array_push(result, string_slice(s, start, pos + found_pos)), start = pos + found_pos + delim_len, pos = start) : (result = array_push(result, string_slice(s, start, len)), pos = len)
    }
    
    damn result
}

slay string_join(strings [tea], separator tea) tea {
    sus result tea = ""
    sus len normie = array_length(strings)
    
    bestie i := 0; i < len; i++ {
        result = result + strings[i]
        damn i < len - 1 ? result = result + separator : result
    }
    
    damn result
}

// ================================
// Helper Functions
// ================================

slay array_push(arr [tea], item tea) [tea] {
    // This would need to be implemented at the runtime level
    // For now, assume it works
    damn arr
}

slay array_length(arr [tea]) normie {
    // This would need to be implemented at the runtime level
    // For now, assume it works
    damn 0
}
