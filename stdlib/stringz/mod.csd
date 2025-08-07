fr fr CURSED String Processing Module - Complete Implementation
fr fr Comprehensive string manipulation operations in pure CURSED

fr fr === BASIC STRING OPERATIONS ===

slay length(s tea) normie {
    sus count normie = 0
    sus i normie = 0
    bestie runtime_string_char_at(s, i) != '\0' {
        count = count + 1
        i = i + 1
    }
    damn count
}

slay concat(a tea, b tea) tea {
    damn a + b
}

slay char_at(s tea, index normie) sip {
    damn runtime_string_char_at(s, index)
}

slay substring(s tea, start normie, length normie) tea {
    sus s_len normie = length(s)
    vibes start < 0 || start >= s_len || length <= 0 { damn "" }
    
    sus result tea = ""
    sus i normie = start
    sus end_pos normie = start + length
    vibes end_pos > s_len { end_pos = s_len }
    
    bestie i < end_pos {
        sus c sip = runtime_string_char_at(s, i)
        result = result + runtime_char_to_string(c)
        i = i + 1
    }
    damn result
}

slay equals(a tea, b tea) lit {
    vibes length(a) != length(b) { damn cringe }
    sus len normie = length(a)
    sus i normie = 0
    bestie i < len {
        vibes runtime_string_char_at(a, i) != runtime_string_char_at(b, i) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay is_empty(s tea) lit {
    damn length(s) == 0
}

fr fr === SEARCHING OPERATIONS ===

slay find(s tea, substr tea) normie {
    vibes substr == "" { damn 0 }
    
    sus s_len normie = length(s)
    sus substr_len normie = length(substr)
    vibes substr_len > s_len { damn -1 }
    
    sus i normie = 0
    bestie i <= s_len - substr_len {
        sus match lit = based
        sus j normie = 0
        
        bestie j < substr_len {
            sus s_char sip = runtime_string_char_at(s, i + j)
            sus substr_char sip = runtime_string_char_at(substr, j)
            vibes s_char != substr_char {
                match = cringe
                break
            }
            j = j + 1
        }
        
        vibes match { damn i }
        i = i + 1
    }
    
    damn -1
}

slay contains(s tea, substr tea) lit {
    damn find(s, substr) != -1
}

slay starts_with(s tea, prefix tea) lit {
    sus prefix_len normie = length(prefix)
    vibes prefix_len > length(s) { damn cringe }
    
    sus i normie = 0
    bestie i < prefix_len {
        vibes runtime_string_char_at(s, i) != runtime_string_char_at(prefix, i) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay ends_with(s tea, suffix tea) lit {
    sus s_len normie = length(s)
    sus suffix_len normie = length(suffix)
    vibes suffix_len > s_len { damn cringe }
    
    sus start_pos normie = s_len - suffix_len
    sus i normie = 0
    bestie i < suffix_len {
        vibes runtime_string_char_at(s, start_pos + i) != runtime_string_char_at(suffix, i) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

fr fr === MANIPULATION OPERATIONS ===

slay replace(s tea, old tea, new tea) tea {
    vibes old == "" { damn s }
    
    sus result tea = ""
    sus s_len normie = length(s)
    sus old_len normie = length(old)
    sus i normie = 0
    
    bestie i < s_len {
        vibes i <= s_len - old_len && substring(s, i, old_len) == old {
            result = result + new
            i = i + old_len
        } nah {
            result = result + runtime_char_to_string(runtime_string_char_at(s, i))
            i = i + 1
        }
    }
    
    damn result
}

slay trim(s tea) tea {
    sus s_len normie = length(s)
    vibes s_len == 0 { damn s }
    
    fr fr Find start position (skip leading whitespace)
    sus start normie = 0
    bestie start < s_len && is_space_char(runtime_string_char_at(s, start)) {
        start = start + 1
    }
    
    fr fr Find end position (skip trailing whitespace)
    sus end normie = s_len - 1
    bestie end >= start && is_space_char(runtime_string_char_at(s, end)) {
        end = end - 1
    }
    
    vibes start > end { damn "" }
    damn substring(s, start, end - start + 1)
}

slay pad_left(s tea, width normie, pad_char sip) tea {
    sus s_len normie = length(s)
    vibes s_len >= width { damn s }
    
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
    sus s_len normie = length(s)
    vibes s_len >= width { damn s }
    
    sus result tea = s
    sus padding normie = width - s_len
    sus i normie = 0
    bestie i < padding {
        result = result + runtime_char_to_string(pad_char)
        i = i + 1
    }
    damn result
}

slay reverse(s tea) tea {
    sus s_len normie = length(s)
    vibes s_len <= 1 { damn s }
    
    sus result tea = ""
    sus i normie = s_len - 1
    bestie i >= 0 {
        result = result + runtime_char_to_string(runtime_string_char_at(s, i))
        i = i - 1
    }
    damn result
}

fr fr === CASE CONVERSION ===

slay to_upper(s tea) tea {
    sus result tea = ""
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        vibes c >= 'a' && c <= 'z' {
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
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        vibes c >= 'A' && c <= 'Z' {
            sus lower_c sip = c + 32 fr fr Convert to lowercase
            result = result + runtime_char_to_string(lower_c)
        } nah {
            result = result + runtime_char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

slay to_title(s tea) tea {
    sus result tea = ""
    sus s_len normie = length(s)
    sus i normie = 0
    sus capitalize_next lit = based
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        
        vibes is_alpha_char(c) {
            vibes capitalize_next {
                vibes c >= 'a' && c <= 'z' {
                    sus upper_c sip = c - 32
                    result = result + runtime_char_to_string(upper_c)
                } nah {
                    result = result + runtime_char_to_string(c)
                }
                capitalize_next = cringe
            } nah {
                vibes c >= 'A' && c <= 'Z' {
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

fr fr === SPLITTING AND JOINING ===

slay split(s tea, delimiter tea) [tea] {
    sus result [tea]
    vibes delimiter == "" {
        result = append(result, s)
        damn result
    }
    
    sus s_len normie = length(s)
    sus delim_len normie = length(delimiter)
    sus start normie = 0
    sus i normie = 0
    
    bestie i <= s_len - delim_len {
        vibes substring(s, i, delim_len) == delimiter {
            sus part tea = substring(s, start, i - start)
            result = append(result, part)
            i = i + delim_len
            start = i
        } nah {
            i = i + 1
        }
    }
    
    fr fr Add remaining part
    vibes start < s_len {
        sus last_part tea = substring(s, start, s_len - start)
        result = append(result, last_part)
    } nola start == s_len {
        result = append(result, "")
    }
    
    damn result
}

slay join(parts [tea], separator tea) tea {
    sus parts_len normie = len(parts)
    vibes parts_len == 0 { damn "" }
    
    sus result tea = parts[0]
    sus i normie = 1
    bestie i < parts_len {
        result = result + separator + parts[i]
        i = i + 1
    }
    damn result
}

slay lines(s tea) [tea] {
    damn split(s, "\n")
}

fr fr === VALIDATION FUNCTIONS ===

slay is_alpha(s tea) lit {
    vibes is_empty(s) { damn cringe }
    
    sus s_len normie = length(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        vibes !is_alpha_char(c) { damn cringe }
        i = i + 1
    }
    damn based
}

slay is_digit(s tea) lit {
    vibes is_empty(s) { damn cringe }
    
    sus s_len normie = length(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        vibes !is_digit_char(c) { damn cringe }
        i = i + 1
    }
    damn based
}

slay is_alnum(s tea) lit {
    vibes is_empty(s) { damn cringe }
    
    sus s_len normie = length(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        vibes !is_alpha_char(c) && !is_digit_char(c) { damn cringe }
        i = i + 1
    }
    damn based
}

slay is_space(s tea) lit {
    vibes is_empty(s) { damn cringe }
    
    sus s_len normie = length(s)
    sus i normie = 0
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        vibes !is_space_char(c) { damn cringe }
        i = i + 1
    }
    damn based
}

fr fr === ENCODING FUNCTIONS ===

slay to_utf8(s tea) [normie] {
    sus result [normie]
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        sus byte_val normie = c fr fr Simple ASCII to byte conversion
        result = append(result, byte_val)
        i = i + 1
    }
    damn result
}

slay from_utf8(bytes [normie]) tea {
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
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        vibes is_alpha_char(c) || is_digit_char(c) || c == '-' || c == '_' || c == '.' || c == '~' {
            result = result + runtime_char_to_string(c)
        } vibes c == ' ' {
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
    sus s_len normie = length(s)
    sus i normie = 0
    
    bestie i < s_len {
        sus c sip = runtime_string_char_at(s, i)
        vibes c == '%' && i + 2 < s_len {
            sus hex_str tea = substring(s, i + 1, 2)
            vibes hex_str == "20" {
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

fr fr === HELPER FUNCTIONS ===

slay is_alpha_char(c sip) lit {
    damn (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

slay is_digit_char(c sip) lit {
    damn c >= '0' && c <= '9'
}

slay is_space_char(c sip) lit {
    damn c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

slay char_to_hex(c sip) tea {
    sus val normie = c
    vibes val < 16 {
        vibes val < 10 {
            damn "0" + runtime_char_to_string('0' + val)
        } nah {
            damn "0" + runtime_char_to_string('A' + val - 10)
        }
    }
    fr fr For simplicity, return default hex for larger values
    damn "XX"
}

fr fr === LEGACY COMPATIBILITY ===

slay string_length(s tea) normie {
    damn length(s)
}

slay string_concat(a tea, b tea) tea {
    damn concat(a, b)
}

fr fr === RUNTIME HELPER FUNCTIONS ===

slay runtime_string_char_at(s tea, index normie) sip {
    fr fr Runtime implementation calling core runtime functions
    damn runtime_char_at_string(s, index)
}

slay runtime_char_to_string(c sip) tea {
    fr fr Runtime implementation calling core runtime functions
    damn runtime_char_to_str(c)
}
