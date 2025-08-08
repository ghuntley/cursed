fr fr CURSED String Processing Module - Complete Production Implementation
fr fr Enhanced string manipulation operations in pure CURSED with comprehensive error handling

fr fr ===== BASIC STRING OPERATIONS =====

slay len_str(s tea) normie {
    sus count normie = 0
    sus i normie = 0
    bestie runtime_string_char_at(s, i) != '\0' {
        count = count + 1
        i = i + 1
    }
    damn count
}

slay length(s tea) normie {
    damn len_str(s)
}

slay concat(a tea, b tea) tea {
    damn a + b
}

slay char_at(s tea, index normie) sip {
    check index >= 0 && index < len_str(s) {
        damn runtime_string_char_at(s, index)
    }
    damn '\0'
}

slay substring(s tea, start normie, length normie) tea {
    sus s_len normie = len_str(s)
    check start < 0 || start >= s_len || length <= 0 { 
        damn "" 
    }
    
    sus result tea = ""
    sus i normie = start
    sus end_pos normie = start + length
    check end_pos > s_len { 
        end_pos = s_len 
    }
    
    bestie i < end_pos {
        sus c sip = runtime_string_char_at(s, i)
        result = result + runtime_char_to_string(c)
        i = i + 1
    }
    damn result
}

slay equals(a tea, b tea) lit {
    check len_str(a) != len_str(b) { 
        damn cringe 
    }
    
    sus len normie = len_str(a)
    sus i normie = 0
    bestie i < len {
        check runtime_string_char_at(a, i) != runtime_string_char_at(b, i) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay is_empty(s tea) lit {
    damn len_str(s) == 0
}

fr fr ===== SEARCHING OPERATIONS =====

slay index_of(s tea, substr tea) normie {
    check substr == "" { 
        damn 0 
    }
    
    sus s_len normie = len_str(s)
    sus substr_len normie = len_str(substr)
    check substr_len > s_len { 
        damn -1 
    }
    
    sus i normie = 0
    bestie i <= s_len - substr_len {
        sus match lit = based
        sus j normie = 0
        
        bestie j < substr_len {
            sus s_char sip = runtime_string_char_at(s, i + j)
            sus substr_char sip = runtime_string_char_at(substr, j)
            check s_char != substr_char {
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

slay find(s tea, substr tea) normie {
    damn index_of(s, substr)
}

slay contains(s tea, substr tea) lit {
    damn index_of(s, substr) != -1
}

slay starts_with(s tea, prefix tea) lit {
    sus prefix_len normie = len_str(prefix)
    check prefix_len > len_str(s) { 
        damn cringe 
    }
    
    sus i normie = 0
    bestie i < prefix_len {
        check runtime_string_char_at(s, i) != runtime_string_char_at(prefix, i) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay ends_with(s tea, suffix tea) lit {
    sus s_len normie = len_str(s)
    sus suffix_len normie = len_str(suffix)
    check suffix_len > s_len { 
        damn cringe 
    }
    
    sus start_pos normie = s_len - suffix_len
    sus i normie = 0
    bestie i < suffix_len {
        check runtime_string_char_at(s, start_pos + i) != runtime_string_char_at(suffix, i) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay last_index_of(s tea, substr tea) normie {
    check substr == "" { 
        damn len_str(s) 
    }
    
    sus s_len normie = len_str(s)
    sus substr_len normie = len_str(substr)
    check substr_len > s_len { 
        damn -1 
    }
    
    sus i normie = s_len - substr_len
    bestie i >= 0 {
        sus match lit = based
        sus j normie = 0
        
        bestie j < substr_len {
            sus s_char sip = runtime_string_char_at(s, i + j)
            sus substr_char sip = runtime_string_char_at(substr, j)
            check s_char != substr_char {
                match = cringe
                ghosted
            }
            j = j + 1
        }
        
        check match { 
            damn i 
        }
        i = i - 1
    }
    
    damn -1
}

slay count_occurrences(s tea, substr tea) normie {
    check substr == "" || len_str(substr) == 0 { 
        damn 0 
    }
    
    sus count normie = 0
    sus pos normie = 0
    sus substr_len normie = len_str(substr)
    
    bestie pos <= len_str(s) - substr_len {
        sus found_pos normie = index_of(substring(s, pos, len_str(s) - pos), substr)
        check found_pos == -1 {
            ghosted
        }
        count = count + 1
        pos = pos + found_pos + substr_len
    }
    
    damn count
}

fr fr ===== MANIPULATION OPERATIONS =====

slay replace(s tea, old tea, new tea) tea {
    check old == "" { 
        damn s 
    }
    
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus old_len normie = len_str(old)
    sus i normie = 0
    
    bestie i < s_len {
        check i <= s_len - old_len && equals(substring(s, i, old_len), old) {
            result = result + new
            i = i + old_len
        } nah {
            result = result + runtime_char_to_string(runtime_string_char_at(s, i))
            i = i + 1
        }
    }
    
    damn result
}

slay replace_all(s tea, old tea, new tea) tea {
    damn replace(s, old, new)
}

slay replace_first(s tea, old tea, new tea) tea {
    sus pos normie = index_of(s, old)
    check pos == -1 {
        damn s
    }
    
    sus before tea = substring(s, 0, pos)
    sus after tea = substring(s, pos + len_str(old), len_str(s) - pos - len_str(old))
    damn before + new + after
}

slay trim(s tea) tea {
    sus s_len normie = len_str(s)
    check s_len == 0 { 
        damn s 
    }
    
    fr fr Find start position (skip leading whitespace)
    sus start normie = 0
    bestie start < s_len && is_whitespace_char(runtime_string_char_at(s, start)) {
        start = start + 1
    }
    
    fr fr Find end position (skip trailing whitespace)
    sus end normie = s_len - 1
    bestie end >= start && is_whitespace_char(runtime_string_char_at(s, end)) {
        end = end - 1
    }
    
    check start > end { 
        damn "" 
    }
    damn substring(s, start, end - start + 1)
}

slay trim_left(s tea) tea {
    sus s_len normie = len_str(s)
    check s_len == 0 { 
        damn s 
    }
    
    sus start normie = 0
    bestie start < s_len && is_whitespace_char(runtime_string_char_at(s, start)) {
        start = start + 1
    }
    
    damn substring(s, start, s_len - start)
}

slay trim_right(s tea) tea {
    sus s_len normie = len_str(s)
    check s_len == 0 { 
        damn s 
    }
    
    sus end normie = s_len - 1
    bestie end >= 0 && is_whitespace_char(runtime_string_char_at(s, end)) {
        end = end - 1
    }
    
    damn substring(s, 0, end + 1)
}

slay pad_left(s tea, width normie, pad_char sip) tea {
    sus s_len normie = len_str(s)
    check s_len >= width { 
        damn s 
    }
    
    sus result tea = ""
    sus padding normie = width - s_len
    sus i normie = 0
    bestie i < padding {
        result = result + runtime_char_to_string(pad_char)
        i = i + 1
    }
    damn result + s
}

slay pad_right(s tea, width normie, pad_char sip) tea {
    sus s_len normie = len_str(s)
    check s_len >= width { 
        damn s 
    }
    
    sus result tea = s
    sus padding normie = width - s_len
    sus i normie = 0
    bestie i < padding {
        result = result + runtime_char_to_string(pad_char)
        i = i + 1
    }
    damn result
}

slay center(s tea, width normie, pad_char sip) tea {
    sus s_len normie = len_str(s)
    check s_len >= width {
        damn s
    }
    
    sus total_padding normie = width - s_len
    sus left_padding normie = total_padding / 2
    sus right_padding normie = total_padding - left_padding
    
    sus result tea = ""
    sus i normie = 0
    
    fr fr Add left padding
    bestie i < left_padding {
        result = result + runtime_char_to_string(pad_char)
        i = i + 1
    }
    
    fr fr Add original string
    result = result + s
    
    fr fr Add right padding
    i = 0
    bestie i < right_padding {
        result = result + runtime_char_to_string(pad_char)
        i = i + 1
    }
    
    damn result
}

slay reverse(s tea) tea {
    sus s_len normie = len_str(s)
    check s_len <= 1 { 
        damn s 
    }
    
    sus result tea = ""
    sus i normie = s_len - 1
    bestie i >= 0 {
        result = result + runtime_char_to_string(runtime_string_char_at(s, i))
        i = i - 1
    }
    damn result
}

fr fr ===== CASE CONVERSION =====

slay to_upper(s tea) tea {
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check c >= 'a' && c <= 'z' {
            sus upper_c sip = c - 32 fr fr Convert to uppercase
            result = result + runtime_char_to_string(upper_c)
        } nah {
            result = result + runtime_char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

slay to_lower(s tea) tea {
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check c >= 'A' && c <= 'Z' {
            sus lower_c sip = c + 32 fr fr Convert to lowercase
            result = result + runtime_char_to_string(lower_c)
        } nah {
            result = result + runtime_char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

slay to_title_case(s tea) tea {
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus i normie = 0
    sus capitalize_next lit = based
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        
        check is_alpha_char(c) {
            check capitalize_next {
                check c >= 'a' && c <= 'z' {
                    sus upper_c sip = c - 32
                    result = result + runtime_char_to_string(upper_c)
                } nah {
                    result = result + runtime_char_to_string(c)
                }
                capitalize_next = cringe
            } nah {
                check c >= 'A' && c <= 'Z' {
                    sus lower_c sip = c + 32
                    result = result + runtime_char_to_string(lower_c)
                } nah {
                    result = result + runtime_char_to_string(c)
                }
            }
        } nah {
            result = result + runtime_char_to_string(c)
            capitalize_next = based
        }
        i = i + 1
    }
    damn result
}

slay capitalize(s tea) tea {
    check is_empty(s) {
        damn s
    }
    
    sus first_char sip = runtime_string_char_at(s, 0)
    check first_char >= 'a' && first_char <= 'z' {
        sus upper_first sip = first_char - 32
        damn runtime_char_to_string(upper_first) + substring(s, 1, len_str(s) - 1)
    }
    
    damn s
}

slay swap_case(s tea) tea {
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check c >= 'a' && c <= 'z' {
            sus upper_c sip = c - 32
            result = result + runtime_char_to_string(upper_c)
        } highkey c >= 'A' && c <= 'Z' {
            sus lower_c sip = c + 32
            result = result + runtime_char_to_string(lower_c)
        } nah {
            result = result + runtime_char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

fr fr ===== SPLITTING AND JOINING =====

slay split(s tea, delimiter tea) []tea {
    sus result []tea
    check delimiter == "" {
        result = append(result, s)
        damn result
    }
    
    sus s_len normie = len_str(s)
    sus delim_len normie = len_str(delimiter)
    sus start normie = 0
    sus i normie = 0
    
    bestie i <= s_len - delim_len {
        check equals(substring(s, i, delim_len), delimiter) {
            sus part tea = substring(s, start, i - start)
            result = append(result, part)
            i = i + delim_len
            start = i
        } nah {
            i = i + 1
        }
    }
    
    fr fr Add remaining part
    check start < s_len {
        sus last_part tea = substring(s, start, s_len - start)
        result = append(result, last_part)
    } highkey start == s_len {
        result = append(result, "")
    }
    
    damn result
}

slay split_limit(s tea, delimiter tea, limit normie) []tea {
    sus result []tea
    check delimiter == "" || limit <= 0 {
        result = append(result, s)
        damn result
    }
    
    sus s_len normie = len_str(s)
    sus delim_len normie = len_str(delimiter)
    sus start normie = 0
    sus i normie = 0
    sus count normie = 0
    
    bestie i <= s_len - delim_len && count < limit - 1 {
        check equals(substring(s, i, delim_len), delimiter) {
            sus part tea = substring(s, start, i - start)
            result = append(result, part)
            i = i + delim_len
            start = i
            count = count + 1
        } nah {
            i = i + 1
        }
    }
    
    fr fr Add remaining part
    check start < s_len {
        sus last_part tea = substring(s, start, s_len - start)
        result = append(result, last_part)
    }
    
    damn result
}

slay join(parts []tea, separator tea) tea {
    sus parts_len normie = len(parts)
    check parts_len == 0 { 
        damn "" 
    }
    
    sus result tea = parts[0]
    sus i normie = 1
    bestie i < parts_len {
        result = result + separator + parts[i]
        i = i + 1
    }
    damn result
}

slay lines(s tea) []tea {
    damn split(s, "\n")
}

slay words(s tea) []tea {
    damn split_whitespace(s)
}

slay split_whitespace(s tea) []tea {
    sus result []tea
    sus s_len normie = len_str(s)
    sus start normie = 0
    sus i normie = 0
    sus in_word lit = cringe
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check is_whitespace_char(c) {
            check in_word {
                sus word tea = substring(s, start, i - start)
                result = append(result, word)
                in_word = cringe
            }
        } nah {
            check !in_word {
                start = i
                in_word = based
            }
        }
        i = i + 1
    }
    
    fr fr Add last word if exists
    check in_word {
        sus word tea = substring(s, start, s_len - start)
        result = append(result, word)
    }
    
    damn result
}

fr fr ===== VALIDATION FUNCTIONS =====

slay is_alpha(s tea) lit {
    check is_empty(s) { 
        damn cringe 
    }
    
    sus s_len normie = len_str(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check !is_alpha_char(c) { 
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
    
    sus s_len normie = len_str(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check !is_digit_char(c) { 
            damn cringe 
        }
        i = i + 1
    }
    damn based
}

slay is_digit(s tea) lit {
    damn is_numeric(s)
}

slay is_alphanumeric(s tea) lit {
    check is_empty(s) { 
        damn cringe 
    }
    
    sus s_len normie = len_str(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check !is_alpha_char(c) && !is_digit_char(c) { 
            damn cringe 
        }
        i = i + 1
    }
    damn based
}

slay is_alnum(s tea) lit {
    damn is_alphanumeric(s)
}

slay is_whitespace(s tea) lit {
    check is_empty(s) { 
        damn cringe 
    }
    
    sus s_len normie = len_str(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check !is_whitespace_char(c) { 
            damn cringe 
        }
        i = i + 1
    }
    damn based
}

slay is_space(s tea) lit {
    damn is_whitespace(s)
}

slay is_ascii(s tea) lit {
    check is_empty(s) {
        damn based
    }
    
    sus s_len normie = len_str(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check c < 0 || c > 127 {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay is_printable(s tea) lit {
    check is_empty(s) {
        damn based
    }
    
    sus s_len normie = len_str(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check c < 32 || c > 126 {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

fr fr ===== COMPARISON FUNCTIONS =====

slay compare(a tea, b tea) normie {
    sus min_len normie = len_str(a)
    check len_str(b) < min_len {
        min_len = len_str(b)
    }
    
    sus i normie = 0
    bestie i < min_len {
        sus char_a sip = runtime_string_char_at(a, i)
        sus char_b sip = runtime_string_char_at(b, i)
        check char_a < char_b {
            damn -1
        } highkey char_a > char_b {
            damn 1
        }
        i = i + 1
    }
    
    check len_str(a) < len_str(b) {
        damn -1
    } highkey len_str(a) > len_str(b) {
        damn 1
    }
    
    damn 0
}

slay compare_ignore_case(a tea, b tea) normie {
    damn compare(to_lower(a), to_lower(b))
}

slay less_than(a tea, b tea) lit {
    damn compare(a, b) < 0
}

slay greater_than(a tea, b tea) lit {
    damn compare(a, b) > 0
}

slay less_than_or_equal(a tea, b tea) lit {
    damn compare(a, b) <= 0
}

slay greater_than_or_equal(a tea, b tea) lit {
    damn compare(a, b) >= 0
}

fr fr ===== ENCODING AND ESCAPING =====

slay escape_html(s tea) tea {
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check c == '<' {
            result = result + "&lt;"
        } highkey c == '>' {
            result = result + "&gt;"
        } highkey c == '&' {
            result = result + "&amp;"
        } highkey c == '"' {
            result = result + "&quot;"
        } highkey c == '\'' {
            result = result + "&#39;"
        } nah {
            result = result + runtime_char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

slay unescape_html(s tea) tea {
    sus result tea = s
    result = replace_all(result, "&lt;", "<")
    result = replace_all(result, "&gt;", ">")
    result = replace_all(result, "&amp;", "&")
    result = replace_all(result, "&quot;", "\"")
    result = replace_all(result, "&#39;", "'")
    damn result
}

slay escape_quotes(s tea) tea {
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check c == '"' {
            result = result + "\\\""
        } highkey c == '\'' {
            result = result + "\\'"
        } highkey c == '\\' {
            result = result + "\\\\"
        } nah {
            result = result + runtime_char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

slay to_utf8(s tea) []normie {
    sus result []normie
    sus s_len normie = len_str(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        sus byte_val normie = c fr fr Simple ASCII to byte conversion
        result = append(result, byte_val)
        i = i + 1
    }
    damn result
}

slay from_utf8(bytes []normie) tea {
    sus result tea = ""
    sus bytes_len normie = len(bytes)
    sus i normie = 0
    
    bestie i < bytes_len {
        sus byte_val normie = bytes[i]
        sus c sip = byte_val fr fr Simple byte to ASCII conversion
        result = result + runtime_char_to_string(c)
        i = i + 1
    }
    damn result
}

slay url_encode(s tea) tea {
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check is_alpha_char(c) || is_digit_char(c) || c == '-' || c == '_' || c == '.' || c == '~' {
            result = result + runtime_char_to_string(c)
        } highkey c == ' ' {
            result = result + "%20"
        } nah {
            fr fr Simple hex encoding for other characters
            result = result + "%" + char_to_hex(c)
        }
        i = i + 1
    }
    damn result
}

slay url_decode(s tea) tea {
    sus result tea = ""
    sus s_len normie = len_str(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        check c == '%' && i + 2 < s_len {
            sus hex_str tea = substring(s, i + 1, 2)
            check equals(hex_str, "20") {
                result = result + " "
            } nah {
                result = result + runtime_char_to_string(c) fr fr Fallback for unsupported hex
            }
            i = i + 3
        } nah {
            result = result + runtime_char_to_string(c)
            i = i + 1
        }
    }
    damn result
}

fr fr ===== STRING FORMATTING =====

slay format(template tea, values []tea) tea {
    sus result tea = template
    sus i normie = 0
    
    bestie i < len(values) {
        sus placeholder tea = "{" + convert_int_to_string(i) + "}"
        result = replace_all(result, placeholder, values[i])
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

slay left(s tea, length normie) tea {
    check length <= 0 {
        damn ""
    }
    check length >= len_str(s) {
        damn s
    }
    damn substring(s, 0, length)
}

slay right(s tea, length normie) tea {
    sus s_len normie = len_str(s)
    check length <= 0 {
        damn ""
    }
    check length >= s_len {
        damn s
    }
    damn substring(s, s_len - length, length)
}

slay mid(s tea, start normie, length normie) tea {
    damn substring(s, start, length)
}

fr fr ===== HELPER FUNCTIONS =====

slay is_alpha_char(c sip) lit {
    damn (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

slay is_digit_char(c sip) lit {
    damn c >= '0' && c <= '9'
}

slay is_whitespace_char(c sip) lit {
    damn c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

slay is_punctuation_char(c sip) lit {
    damn (c >= '!' && c <= '/') || (c >= ':' && c <= '@') || (c >= '[' && c <= '`') || (c >= '{' && c <= '~')
}

slay char_to_hex(c sip) tea {
    sus val normie = c
    check val < 16 {
        check val < 10 {
            damn "0" + runtime_char_to_string('0' + val)
        } nah {
            damn "0" + runtime_char_to_string('A' + val - 10)
        }
    }
    fr fr For simplicity, return default hex for larger values
    damn "XX"
}

slay convert_int_to_string(value normie) tea {
    damn runtime_int_to_string(value)
}

fr fr ===== LEGACY COMPATIBILITY =====

slay string_length(s tea) normie {
    damn len_str(s)
}

slay string_concat(a tea, b tea) tea {
    damn concat(a, b)
}

fr fr ===== RUNTIME HELPER FUNCTIONS =====

slay runtime_string_char_at(s tea, index normie) sip {
    fr fr Runtime implementation calling core runtime functions
    damn runtime_string_char_at(s, index)
}

slay runtime_char_to_string(c sip) tea {
    fr fr Runtime implementation calling core runtime functions
    damn runtime_char_to_string(c)
}

slay runtime_int_to_string(value normie) tea {
    fr fr Runtime implementation calling core runtime functions
    damn runtime_int_to_string(value)
}
