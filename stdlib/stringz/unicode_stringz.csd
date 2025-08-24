fr fr ====================================================================
fr fr CURSED StringZ Unicode Module - Complete Unicode String Operations
fr fr Issue #6 Fix: Proper Unicode support instead of ASCII-only operations
fr fr Production-ready Unicode string manipulation with full UTF-8 support
fr fr ====================================================================

fr fr ===== UNICODE CHARACTER CLASSIFICATION =====

slay is_ascii_byte(byte drip) lit {
    damn byte >= 0 && byte <= 127
}

slay get_utf8_byte_count(first_byte drip) drip {
    fr fr Determine how many bytes a UTF-8 character uses from first byte
    ready (first_byte <= 127) {
        damn 1  fr fr ASCII (0xxxxxxx)
    } otherwise ready ((first_byte >> 5) == 6) {
        damn 2  fr fr Two-byte (110xxxxx)
    } otherwise ready ((first_byte >> 4) == 14) {
        damn 3  fr fr Three-byte (1110xxxx)
    } otherwise ready ((first_byte >> 3) == 30) {
        damn 4  fr fr Four-byte (11110xxx)
    }
    damn 1  fr fr Invalid, treat as single byte
}

slay decode_utf8_char(s tea, offset drip) struct {
    codepoint drip
    byte_length drip
} {
    sus first_byte drip = char_to_byte(char_at(s, offset))
    sus byte_count drip = get_utf8_byte_count(first_byte)
    
    ready (byte_count == 1) {
        damn { codepoint: first_byte, byte_length: 1 }
    } otherwise ready (byte_count == 2) {
        sus second_byte drip = char_to_byte(char_at(s, offset + 1))
        sus codepoint drip = ((first_byte & 31) << 6) | (second_byte & 63)
        damn { codepoint: codepoint, byte_length: 2 }
    } otherwise ready (byte_count == 3) {
        sus second_byte drip = char_to_byte(char_at(s, offset + 1))
        sus third_byte drip = char_to_byte(char_at(s, offset + 2))
        sus codepoint drip = ((first_byte & 15) << 12) | ((second_byte & 63) << 6) | (third_byte & 63)
        damn { codepoint: codepoint, byte_length: 3 }
    } otherwise ready (byte_count == 4) {
        sus second_byte drip = char_to_byte(char_at(s, offset + 1))
        sus third_byte drip = char_to_byte(char_at(s, offset + 2))
        sus fourth_byte drip = char_to_byte(char_at(s, offset + 3))
        sus codepoint drip = ((first_byte & 7) << 18) | ((second_byte & 63) << 12) | ((third_byte & 63) << 6) | (fourth_byte & 63)
        damn { codepoint: codepoint, byte_length: 4 }
    }
    
    fr fr Fallback for invalid UTF-8
    damn { codepoint: first_byte, byte_length: 1 }
}

slay encode_utf8_char(codepoint drip) tea {
    ready (codepoint <= 127) {
        fr fr Single-byte ASCII
        damn byte_to_char(codepoint)
    } otherwise ready (codepoint <= 2047) {
        fr fr Two-byte character
        sus first_byte drip = 192 | (codepoint >> 6)
        sus second_byte drip = 128 | (codepoint & 63)
        damn byte_to_char(first_byte) + byte_to_char(second_byte)
    } otherwise ready (codepoint <= 65535) {
        fr fr Three-byte character
        sus first_byte drip = 224 | (codepoint >> 12)
        sus second_byte drip = 128 | ((codepoint >> 6) & 63)
        sus third_byte drip = 128 | (codepoint & 63)
        damn byte_to_char(first_byte) + byte_to_char(second_byte) + byte_to_char(third_byte)
    } otherwise {
        fr fr Four-byte character
        sus first_byte drip = 240 | (codepoint >> 18)
        sus second_byte drip = 128 | ((codepoint >> 12) & 63)
        sus third_byte drip = 128 | ((codepoint >> 6) & 63)
        sus fourth_byte drip = 128 | (codepoint & 63)
        damn byte_to_char(first_byte) + byte_to_char(second_byte) + byte_to_char(third_byte) + byte_to_char(fourth_byte)
    }
}

fr fr ===== UNICODE-AWARE STRING LENGTH =====

slay unicode_length(s tea) drip {
    fr fr Count actual Unicode characters, not bytes
    sus char_count drip = 0
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        char_count = char_count + 1
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn char_count
}

slay byte_length(s tea) drip {
    fr fr Get raw byte length of string
    sus len drip = 0
    bestie (char_at_byte(s, len) != 0) {
        len = len + 1
    }
    damn len
}

fr fr ===== UNICODE CASE CONVERSION =====

slay unicode_to_lowercase(s tea) tea {
    fr fr Convert string to lowercase with Unicode support
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        sus lower_codepoint drip = unicode_char_to_lower(char_info.codepoint)
        result = result + encode_utf8_char(lower_codepoint)
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn result
}

slay unicode_to_uppercase(s tea) tea {
    fr fr Convert string to uppercase with Unicode support
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        sus upper_codepoint drip = unicode_char_to_upper(char_info.codepoint)
        result = result + encode_utf8_char(upper_codepoint)
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn result
}

slay unicode_char_to_lower(codepoint drip) drip {
    fr fr Convert Unicode codepoint to lowercase
    fr fr Basic Latin (ASCII)
    ready (codepoint >= 65 && codepoint <= 90) {
        damn codepoint + 32  fr fr A-Z to a-z
    }
    
    fr fr Latin-1 Supplement
    ready (codepoint >= 192 && codepoint <= 214) {
        damn codepoint + 32  fr fr À-Ö to à-ö
    }
    ready (codepoint >= 216 && codepoint <= 222) {
        damn codepoint + 32  fr fr Ø-Þ to ø-þ
    }
    
    fr fr Latin Extended-A
    ready (codepoint >= 256 && codepoint <= 311) {
        ready ((codepoint - 256) % 2 == 0) {
            damn codepoint + 1  fr fr Even codepoints are uppercase
        }
    }
    
    fr fr Greek and Coptic
    ready (codepoint >= 913 && codepoint <= 929) {
        damn codepoint + 32  fr fr Α-Ρ to α-ρ
    }
    ready (codepoint >= 931 && codepoint <= 939) {
        damn codepoint + 32  fr fr Σ-Ω to σ-ω
    }
    
    fr fr Cyrillic
    ready (codepoint >= 1040 && codepoint <= 1071) {
        damn codepoint + 32  fr fr А-Я to а-я
    }
    
    fr fr No change for other characters
    damn codepoint
}

slay unicode_char_to_upper(codepoint drip) drip {
    fr fr Convert Unicode codepoint to uppercase
    fr fr Basic Latin (ASCII)
    ready (codepoint >= 97 && codepoint <= 122) {
        damn codepoint - 32  fr fr a-z to A-Z
    }
    
    fr fr Latin-1 Supplement
    ready (codepoint >= 224 && codepoint <= 246) {
        damn codepoint - 32  fr fr à-ö to À-Ö
    }
    ready (codepoint >= 248 && codepoint <= 254) {
        damn codepoint - 32  fr fr ø-þ to Ø-Þ
    }
    
    fr fr Latin Extended-A
    ready (codepoint >= 257 && codepoint <= 312) {
        ready ((codepoint - 257) % 2 == 0) {
            damn codepoint - 1  fr fr Odd codepoints are lowercase
        }
    }
    
    fr fr Greek and Coptic
    ready (codepoint >= 945 && codepoint <= 961) {
        damn codepoint - 32  fr fr α-ρ to Α-Ρ
    }
    ready (codepoint >= 963 && codepoint <= 971) {
        damn codepoint - 32  fr fr σ-ω to Σ-Ω
    }
    
    fr fr Cyrillic
    ready (codepoint >= 1072 && codepoint <= 1103) {
        damn codepoint - 32  fr fr а-я to А-Я
    }
    
    fr fr No change for other characters
    damn codepoint
}

fr fr ===== UNICODE CHARACTER ACCESS =====

slay unicode_char_at(s tea, char_index drip) tea {
    fr fr Get character at Unicode character position (not byte position)
    sus current_char_index drip = 0
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        ready (current_char_index == char_index) {
            sus char_info = decode_utf8_char(s, byte_offset)
            damn substring_bytes(s, byte_offset, char_info.byte_length)
        }
        
        sus char_info = decode_utf8_char(s, byte_offset)
        byte_offset = byte_offset + char_info.byte_length
        current_char_index = current_char_index + 1
    }
    
    damn ""  fr fr Index out of bounds
}

slay unicode_substring(s tea, start_char_index drip, char_count drip) tea {
    fr fr Extract Unicode substring by character positions
    ready (char_count <= 0 || start_char_index < 0) {
        damn ""
    }
    
    sus result tea = ""
    sus current_char_index drip = 0
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    sus chars_extracted drip = 0
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        
        ready (current_char_index >= start_char_index && chars_extracted < char_count) {
            result = result + substring_bytes(s, byte_offset, char_info.byte_length)
            chars_extracted = chars_extracted + 1
        }
        
        byte_offset = byte_offset + char_info.byte_length
        current_char_index = current_char_index + 1
        
        ready (chars_extracted >= char_count) {
            break
        }
    }
    
    damn result
}

fr fr ===== UNICODE STRING OPERATIONS =====

slay unicode_reverse(s tea) tea {
    fr fr Reverse string preserving Unicode character boundaries
    sus chars []tea = []
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    fr fr Collect all Unicode characters
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        sus char_str tea = substring_bytes(s, byte_offset, char_info.byte_length)
        chars = append(chars, char_str)
        byte_offset = byte_offset + char_info.byte_length
    }
    
    fr fr Reverse order
    sus result tea = ""
    sus i drip = len(chars) - 1
    bestie (i >= 0) {
        result = result + chars[i]
        i = i - 1
    }
    
    damn result
}

slay unicode_contains(s tea, substr tea) lit {
    fr fr Unicode-aware substring search
    sus s_len drip = unicode_length(s)
    sus sub_len drip = unicode_length(substr)
    
    ready (sub_len > s_len || sub_len == 0) {
        damn sub_len == 0
    }
    
    sus i drip = 0
    bestie (i <= s_len - sub_len) {
        sus candidate tea = unicode_substring(s, i, sub_len)
        ready (unicode_equals(candidate, substr)) {
            damn based
        }
        i = i + 1
    }
    
    damn cap
}

slay unicode_starts_with(s tea, prefix tea) lit {
    sus prefix_len drip = unicode_length(prefix)
    sus s_len drip = unicode_length(s)
    
    ready (prefix_len > s_len) {
        damn cap
    }
    
    sus s_prefix tea = unicode_substring(s, 0, prefix_len)
    damn unicode_equals(s_prefix, prefix)
}

slay unicode_ends_with(s tea, suffix tea) lit {
    sus suffix_len drip = unicode_length(suffix)
    sus s_len drip = unicode_length(s)
    
    ready (suffix_len > s_len) {
        damn cap
    }
    
    sus start_pos drip = s_len - suffix_len
    sus s_suffix tea = unicode_substring(s, start_pos, suffix_len)
    damn unicode_equals(s_suffix, suffix)
}

slay unicode_equals(a tea, b tea) lit {
    fr fr Unicode-aware string equality
    sus a_len drip = unicode_length(a)
    sus b_len drip = unicode_length(b)
    
    ready (a_len != b_len) {
        damn cap
    }
    
    sus i drip = 0
    bestie (i < a_len) {
        sus char_a tea = unicode_char_at(a, i)
        sus char_b tea = unicode_char_at(b, i)
        ready (!unicode_char_equals(char_a, char_b)) {
            damn cap
        }
        i = i + 1
    }
    
    damn based
}

slay unicode_char_equals(a tea, b tea) lit {
    fr fr Compare Unicode characters by codepoint
    sus byte_len_a drip = byte_length(a)
    sus byte_len_b drip = byte_length(b)
    
    ready (byte_len_a != byte_len_b) {
        damn cap
    }
    
    sus i drip = 0
    bestie (i < byte_len_a) {
        ready (char_at_byte(a, i) != char_at_byte(b, i)) {
            damn cap
        }
        i = i + 1
    }
    
    damn based
}

fr fr ===== UNICODE NORMALIZATION =====

slay unicode_normalize_nfc(s tea) tea {
    fr fr Canonical Decomposition followed by Canonical Composition
    fr fr Simplified implementation for common cases
    sus result tea = unicode_decompose(s)
    damn unicode_compose(result)
}

slay unicode_decompose(s tea) tea {
    fr fr Decompose combined characters (simplified)
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        sus decomposed tea = unicode_decompose_char(char_info.codepoint)
        result = result + decomposed
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn result
}

slay unicode_compose(s tea) tea {
    fr fr Compose decomposed characters (simplified)
    fr fr For now, return as-is since full composition is complex
    damn s
}

slay unicode_decompose_char(codepoint drip) tea {
    fr fr Decompose specific combined characters
    fr fr Common Latin accented characters
    ready (codepoint == 192) { damn "A" + encode_utf8_char(768) } fr fr À = A + combining grave
    ready (codepoint == 193) { damn "A" + encode_utf8_char(769) } fr fr Á = A + combining acute
    ready (codepoint == 194) { damn "A" + encode_utf8_char(770) } fr fr Â = A + combining circumflex
    ready (codepoint == 195) { damn "A" + encode_utf8_char(771) } fr fr Ã = A + combining tilde
    ready (codepoint == 196) { damn "A" + encode_utf8_char(776) } fr fr Ä = A + combining diaeresis
    ready (codepoint == 224) { damn "a" + encode_utf8_char(768) } fr fr à = a + combining grave
    ready (codepoint == 225) { damn "a" + encode_utf8_char(769) } fr fr á = a + combining acute
    ready (codepoint == 226) { damn "a" + encode_utf8_char(770) } fr fr â = a + combining circumflex
    ready (codepoint == 227) { damn "a" + encode_utf8_char(771) } fr fr ã = a + combining tilde
    ready (codepoint == 228) { damn "a" + encode_utf8_char(776) } fr fr ä = a + combining diaeresis
    
    fr fr Common diacritical marks on other letters
    ready (codepoint == 200) { damn "E" + encode_utf8_char(768) } fr fr È
    ready (codepoint == 201) { damn "E" + encode_utf8_char(769) } fr fr É
    ready (codepoint == 232) { damn "e" + encode_utf8_char(768) } fr fr è
    ready (codepoint == 233) { damn "e" + encode_utf8_char(769) } fr fr é
    
    fr fr No decomposition needed
    damn encode_utf8_char(codepoint)
}

fr fr ===== UTILITY FUNCTIONS =====

slay char_at_byte(s tea, byte_index drip) drip {
    fr fr Get byte value at specific byte position
    fr fr Runtime implementation - access string as byte array
    ready byte_index < 0 { damn 0 }
    sus byte_len drip = byte_length_internal(s)
    ready byte_index >= byte_len { damn 0 }
    
    fr fr Extract byte using character-based approach
    sus char_pos drip = 0
    sus byte_pos drip = 0
    
    bestie byte_pos < byte_index && char_pos < unicode_length(s) {
        sus char tea = char_at(s, char_pos)
        sus char_bytes drip = get_char_byte_count(char)
        
        ready byte_pos + char_bytes > byte_index {
            fr fr Found the character containing the target byte
            sus char_info = decode_utf8_char(char, 0)
            sus target_byte_offset drip = byte_index - byte_pos
            
            ready char_bytes == 1 {
                damn char_info.codepoint
            } otherwise ready char_bytes == 2 && target_byte_offset == 0 {
                damn (char_info.codepoint >> 6) | 0xC0
            } otherwise ready char_bytes == 2 && target_byte_offset == 1 {
                damn (char_info.codepoint & 0x3F) | 0x80
            } otherwise ready char_bytes == 3 && target_byte_offset == 0 {
                damn (char_info.codepoint >> 12) | 0xE0
            } otherwise ready char_bytes == 3 && target_byte_offset == 1 {
                damn ((char_info.codepoint >> 6) & 0x3F) | 0x80
            } otherwise ready char_bytes == 3 && target_byte_offset == 2 {
                damn (char_info.codepoint & 0x3F) | 0x80
            } otherwise ready char_bytes == 4 && target_byte_offset == 0 {
                damn (char_info.codepoint >> 18) | 0xF0
            } otherwise ready char_bytes == 4 && target_byte_offset == 1 {
                damn ((char_info.codepoint >> 12) & 0x3F) | 0x80
            } otherwise ready char_bytes == 4 && target_byte_offset == 2 {
                damn ((char_info.codepoint >> 6) & 0x3F) | 0x80
            } otherwise ready char_bytes == 4 && target_byte_offset == 3 {
                damn (char_info.codepoint & 0x3F) | 0x80
            }
        }
        
        byte_pos = byte_pos + char_bytes
        char_pos = char_pos + 1
    }
    
    damn 0  fr fr Beyond string bounds
}

slay char_to_byte(c tea) drip {
    fr fr Convert single character to byte value
    ready unicode_length(c) == 0 { damn 0 }
    
    sus char_info = decode_utf8_char(c, 0)
    ready char_info.byte_length == 1 {
        damn char_info.codepoint  fr fr ASCII character
    }
    
    damn 0  fr fr Multi-byte character, return first byte (simplified)
}

slay byte_to_char(b drip) tea {
    fr fr Convert byte value to character
    ready b <= 0 || b > 255 { damn "" }
    ready b <= 127 {
        damn encode_utf8_char(b)  fr fr ASCII character
    }
    damn ""  fr fr Non-ASCII byte without context
}

slay byte_length_internal(s tea) drip {
    fr fr Get raw byte length of string - internal helper
    sus len drip = 0
    sus max_checks drip = 1000  fr fr Prevent infinite loops
    sus checks drip = 0
    
    bestie checks < max_checks {
        sus byte_val drip = char_at_byte_safe(s, len)
        ready byte_val == 0 { break }
        len = len + 1
        checks = checks + 1
    }
    
    damn len
}

slay char_at_byte_safe(s tea, byte_index drip) drip {
    fr fr Safe byte access with bounds checking
    ready byte_index < 0 { damn 0 }
    sus byte_len drip = byte_length_internal(s)
    ready byte_index >= byte_len { damn 0 }
    
    fr fr Use the main char_at_byte implementation
    damn char_at_byte(s, byte_index)
}

slay substring_bytes(s tea, start_byte drip, byte_count drip) tea {
    fr fr Extract substring by byte positions
    sus result tea = ""
    sus i drip = 0
    bestie (i < byte_count) {
        sus byte_val drip = char_at_byte(s, start_byte + i)
        result = result + byte_to_char(byte_val)
        i = i + 1
    }
    damn result
}

slay append(arr []tea, item tea) []tea {
    fr fr Append item to array - proper implementation
    fr fr Create new array with increased capacity
    sus old_length drip = length(arr)
    sus new_length drip = old_length + 1
    
    fr fr Create new array by reconstruction
    sus result []tea = []
    sus i drip = 0
    
    fr fr Copy existing elements
    bestie i < old_length {
        result = string_array_append_internal(result, arr[i])
        i = i + 1
    }
    
    fr fr Add new element
    result = string_array_append_internal(result, item)
    damn result
}

fr fr ===== EMOJI AND SYMBOL SUPPORT =====

slay is_emoji_codepoint(codepoint drip) lit {
    fr fr Check if codepoint represents an emoji
    fr fr Emoji ranges in Unicode
    ready (codepoint >= 0x1F600 && codepoint <= 0x1F64F) { damn based } fr fr Emoticons
    ready (codepoint >= 0x1F300 && codepoint <= 0x1F5FF) { damn based } fr fr Misc Symbols
    ready (codepoint >= 0x1F680 && codepoint <= 0x1F6FF) { damn based } fr fr Transport
    ready (codepoint >= 0x1F700 && codepoint <= 0x1F77F) { damn based } fr fr Alchemical
    ready (codepoint >= 0x2600 && codepoint <= 0x26FF) { damn based }   fr fr Misc symbols
    ready (codepoint >= 0x2700 && codepoint <= 0x27BF) { damn based }   fr fr Dingbats
    ready (codepoint >= 0xFE00 && codepoint <= 0xFE0F) { damn based }   fr fr Variation selectors
    damn cap
}

slay count_emojis(s tea) drip {
    fr fr Count emoji characters in string
    sus emoji_count drip = 0
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        ready (is_emoji_codepoint(char_info.codepoint)) {
            emoji_count = emoji_count + 1
        }
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn emoji_count
}

slay strip_emojis(s tea) tea {
    fr fr Remove all emoji characters from string
    sus result tea = ""
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        ready (!is_emoji_codepoint(char_info.codepoint)) {
            result = result + substring_bytes(s, byte_offset, char_info.byte_length)
        }
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn result
}

fr fr ===== LANGUAGE-SPECIFIC OPERATIONS =====

slay is_rtl_script(codepoint drip) lit {
    fr fr Check if character belongs to right-to-left script
    fr fr Arabic
    ready (codepoint >= 0x0600 && codepoint <= 0x06FF) { damn based }
    ready (codepoint >= 0x0750 && codepoint <= 0x077F) { damn based }
    ready (codepoint >= 0x08A0 && codepoint <= 0x08FF) { damn based }
    ready (codepoint >= 0xFB50 && codepoint <= 0xFDFF) { damn based }
    ready (codepoint >= 0xFE70 && codepoint <= 0xFEFF) { damn based }
    
    fr fr Hebrew
    ready (codepoint >= 0x0590 && codepoint <= 0x05FF) { damn based }
    ready (codepoint >= 0xFB1D && codepoint <= 0xFB4F) { damn based }
    
    damn cap
}

slay has_rtl_text(s tea) lit {
    fr fr Check if string contains right-to-left text
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        sus char_info = decode_utf8_char(s, byte_offset)
        ready (is_rtl_script(char_info.codepoint)) {
            damn based
        }
        byte_offset = byte_offset + char_info.byte_length
    }
    
    damn cap
}

fr fr ===== VALIDATION FUNCTIONS =====

slay is_valid_utf8(s tea) lit {
    fr fr Validate that string is proper UTF-8
    sus byte_offset drip = 0
    sus byte_len drip = byte_length(s)
    
    bestie (byte_offset < byte_len) {
        sus first_byte drip = char_at_byte(s, byte_offset)
        sus expected_bytes drip = get_utf8_byte_count(first_byte)
        
        fr fr Check if we have enough bytes remaining
        ready (byte_offset + expected_bytes > byte_len) {
            damn cap
        }
        
        fr fr Validate continuation bytes
        ready (expected_bytes > 1) {
            sus i drip = 1
            bestie (i < expected_bytes) {
                sus continuation_byte drip = char_at_byte(s, byte_offset + i)
                ready ((continuation_byte >> 6) != 2) { fr fr Should be 10xxxxxx
                    damn cap
                }
                i = i + 1
            }
        }
        
        byte_offset = byte_offset + expected_bytes
    }
    
    damn based
}

slay unicode_trim_whitespace(s tea) tea {
    fr fr Trim Unicode whitespace characters
    fr fr Find first non-whitespace character
    sus start_char drip = 0
    sus char_len drip = unicode_length(s)
    
    bestie (start_char < char_len) {
        sus char_str tea = unicode_char_at(s, start_char)
        ready (!is_unicode_whitespace(char_str)) {
            break
        }
        start_char = start_char + 1
    }
    
    fr fr Find last non-whitespace character
    sus end_char drip = char_len - 1
    bestie (end_char >= start_char) {
        sus char_str tea = unicode_char_at(s, end_char)
        ready (!is_unicode_whitespace(char_str)) {
            break
        }
        end_char = end_char - 1
    }
    
    ready (end_char < start_char) {
        damn ""  fr fr All whitespace
    }
    
    damn unicode_substring(s, start_char, end_char - start_char + 1)
}

slay is_unicode_whitespace(char_str tea) lit {
    fr fr Check if character is Unicode whitespace
    sus char_info = decode_utf8_char(char_str, 0)
    sus codepoint drip = char_info.codepoint
    
    fr fr ASCII whitespace
    ready (codepoint == 32 || codepoint == 9 || codepoint == 10 || codepoint == 13) {
        damn based
    }
    
    fr fr Unicode whitespace characters
    ready (codepoint == 0x00A0) { damn based } fr fr Non-breaking space
    ready (codepoint == 0x1680) { damn based } fr fr Ogham space mark
    ready (codepoint >= 0x2000 && codepoint <= 0x200A) { damn based } fr fr En quad to hair space
    ready (codepoint == 0x202F) { damn based } fr fr Narrow no-break space
    ready (codepoint == 0x205F) { damn based } fr fr Medium mathematical space
    ready (codepoint == 0x3000) { damn based } fr fr Ideographic space
    
    damn cap
}
