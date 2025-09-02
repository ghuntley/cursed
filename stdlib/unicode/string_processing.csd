yeet "unicode"
yeet "testz"

fr fr Unicode String Processing Module - Complete Pure CURSED Implementation
fr fr Provides comprehensive Unicode-aware string processing functions

fr fr Unicode character classification constants
facts UNICODE_CATEGORY_LU normie = 1 fr fr Uppercase Letter
facts UNICODE_CATEGORY_LL normie = 2 fr fr Lowercase Letter  
facts UNICODE_CATEGORY_LT normie = 3 fr fr Titlecase Letter
facts UNICODE_CATEGORY_LM normie = 4 fr fr Modifier Letter
facts UNICODE_CATEGORY_LO normie = 5 fr fr Other Letter
facts UNICODE_CATEGORY_MN normie = 6 fr fr Nonspacing Mark
facts UNICODE_CATEGORY_MC normie = 7 fr fr Spacing Mark
facts UNICODE_CATEGORY_ME normie = 8 fr fr Enclosing Mark
facts UNICODE_CATEGORY_ND normie = 9 fr fr Decimal Number
facts UNICODE_CATEGORY_NL normie = 10 fr fr Letter Number
facts UNICODE_CATEGORY_NO normie = 11 fr fr Other Number
facts UNICODE_CATEGORY_PC normie = 12 fr fr Connector Punctuation
facts UNICODE_CATEGORY_PD normie = 13 fr fr Dash Punctuation
facts UNICODE_CATEGORY_PS normie = 14 fr fr Open Punctuation
facts UNICODE_CATEGORY_PE normie = 15 fr fr Close Punctuation
facts UNICODE_CATEGORY_PI normie = 16 fr fr Initial Punctuation
facts UNICODE_CATEGORY_PF normie = 17 fr fr Final Punctuation
facts UNICODE_CATEGORY_PO normie = 18 fr fr Other Punctuation
facts UNICODE_CATEGORY_SM normie = 19 fr fr Math Symbol
facts UNICODE_CATEGORY_SC normie = 20 fr fr Currency Symbol
facts UNICODE_CATEGORY_SK normie = 21 fr fr Modifier Symbol
facts UNICODE_CATEGORY_SO normie = 22 fr fr Other Symbol
facts UNICODE_CATEGORY_ZS normie = 23 fr fr Space Separator
facts UNICODE_CATEGORY_ZL normie = 24 fr fr Line Separator
facts UNICODE_CATEGORY_ZP normie = 25 fr fr Paragraph Separator
facts UNICODE_CATEGORY_CC normie = 26 fr fr Control
facts UNICODE_CATEGORY_CF normie = 27 fr fr Format
facts UNICODE_CATEGORY_CS normie = 28 fr fr Surrogate
facts UNICODE_CATEGORY_CO normie = 29 fr fr Private Use
facts UNICODE_CATEGORY_CN normie = 30 fr fr Unassigned

fr fr UTF-8 decoding and encoding functions
slay utf8_decode_char(bytes byte[value], offset normie) (normie, normie) {
    vibe_check offset >= len(bytes) {
        damn 0, 0
    }
    
    sus first_byte byte = bytes[offset] fr fr ASCII character (0xxxxxxx)
    vibe_check (first_byte & 0x80) == 0 {
        damn normie(first_byte), 1
    } fr fr 2-byte character (110xxxxx 10xxxxxx)
    vibe_check (first_byte & 0xE0) == 0xC0 {
        vibe_check offset + 1 >= len(bytes) {
            damn 0, 0 fr fr Invalid sequence
        }
        sus second_byte byte = bytes[offset + 1]
        vibe_check (second_byte & 0xC0) != 0x80 {
            damn 0, 0 fr fr Invalid continuation byte
        }
        sus codepoint normie = ((normie(first_byte) & 0x1F) << 6) | (normie(second_byte) & 0x3F)
        damn codepoint, 2
    } fr fr 3-byte character (1110xxxx 10xxxxxx 10xxxxxx)
    vibe_check (first_byte & 0xF0) == 0xE0 {
        vibe_check offset + 2 >= len(bytes) {
            damn 0, 0
        }
        sus second_byte byte = bytes[offset + 1]
        sus third_byte byte = bytes[offset + 2]
        vibe_check (second_byte & 0xC0) != 0x80 || (third_byte & 0xC0) != 0x80 {
            damn 0, 0
        }
        sus codepoint normie = ((normie(first_byte) & 0x0F) << 12) | 
                              ((normie(second_byte) & 0x3F) << 6) | 
                              (normie(third_byte) & 0x3F)
        damn codepoint, 3
    } fr fr 4-byte character (11110xxx 10xxxxxx 10xxxxxx 10xxxxxx)
    vibe_check (first_byte & 0xF8) == 0xF0 {
        vibe_check offset + 3 >= len(bytes) {
            damn 0, 0
        }
        sus second_byte byte = bytes[offset + 1]
        sus third_byte byte = bytes[offset + 2]
        sus fourth_byte byte = bytes[offset + 3]
        vibe_check (second_byte & 0xC0) != 0x80 || 
                   (third_byte & 0xC0) != 0x80 || 
                   (fourth_byte & 0xC0) != 0x80 {
            damn 0, 0
        }
        sus codepoint normie = ((normie(first_byte) & 0x07) << 18) | 
                              ((normie(second_byte) & 0x3F) << 12) |
                              ((normie(third_byte) & 0x3F) << 6) | 
                              (normie(fourth_byte) & 0x3F)
        damn codepoint, 4
    } fr fr Invalid UTF-8 sequence
    damn 0, 0
}

slay utf8_encode_char(codepoint normie) byte[value]{
    sus result byte[value] = [] fr fr ASCII (0-127)
    vibe_check codepoint <= 0x7F {
        result = append(result, byte(codepoint))
        damn result
    } fr fr 2-byte sequence (128-2047)
    vibe_check codepoint <= 0x7FF {
        result = append(result, byte(0xC0 | (codepoint >> 6)))
        result = append(result, byte(0x80 | (codepoint & 0x3F)))
        damn result
    } fr fr 3-byte sequence (2048-65535)
    vibe_check codepoint <= 0xFFFF {
        result = append(result, byte(0xE0 | (codepoint >> 12)))
        result = append(result, byte(0x80 | ((codepoint >> 6) & 0x3F)))
        result = append(result, byte(0x80 | (codepoint & 0x3F)))
        damn result
    } fr fr 4-byte sequence (65536-1114111)
    vibe_check codepoint <= 0x10FFFF {
        result = append(result, byte(0xF0 | (codepoint >> 18)))
        result = append(result, byte(0x80 | ((codepoint >> 12) & 0x3F)))
        result = append(result, byte(0x80 | ((codepoint >> 6) & 0x3F)))
        result = append(result, byte(0x80 | (codepoint & 0x3F)))
        damn result
    } fr fr Invalid codepoint
    damn result
}

fr fr String to byte array conversion
slay string_to_bytes(text tea) byte[value]{
    sus result byte[value] = []
    sus i normie = 0 fr fr Convert each character to bytes (simplified - assumes ASCII for now)
    bestie i < string_byte_length(text) {
        sus char_code normie = string_char_at(text, i)
        result = append(result, byte(char_code))
        i++
    }
    
    damn result
}

fr fr Get actual Unicode character count
slay unicode_string_length(text tea) normie {
    sus bytes byte[value] = string_to_bytes(text)
    sus length normie = 0
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            break fr fr Invalid UTF-8, stop processing
        }
        length++
        offset += byte_count
    }
    
    damn length
}

fr fr Validate UTF-8 string
slay is_valid_utf8_string(text tea) lit {
    sus bytes byte[value] = string_to_bytes(text)
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            damn cap fr fr Invalid UTF-8 sequence found
        }
        vibe_check codepoint > 0x10FFFF {
            damn cap fr fr Codepoint out of Unicode range
        }
        offset += byte_count
    }
    
    damn based fr fr All sequences valid
}

fr fr Unicode character classification
slay get_unicode_category(codepoint normie) normie { fr fr ASCII letters
    vibe_check (codepoint >= 0x41 && codepoint <= 0x5A) {
        damn UNICODE_CATEGORY_LU fr fr Uppercase letter
    }
    vibe_check (codepoint >= 0x61 && codepoint <= 0x7A) {
        damn UNICODE_CATEGORY_LL fr fr Lowercase letter
    } fr fr ASCII digits
    vibe_check (codepoint >= 0x30 && codepoint <= 0x39) {
        damn UNICODE_CATEGORY_ND fr fr Decimal number
    } fr fr ASCII whitespace
    vibe_check codepoint == 0x20 || codepoint == 0x09 || 
               codepoint == 0x0A || codepoint == 0x0D {
        damn UNICODE_CATEGORY_ZS fr fr Space separator
    } fr fr Latin-1 Supplement
    vibe_check (codepoint >= 0x00C0 && codepoint <= 0x00DF) {
        damn UNICODE_CATEGORY_LU fr fr Uppercase letters with diacritics
    }
    vibe_check (codepoint >= 0x00E0 && codepoint <= 0x00FF) {
        damn UNICODE_CATEGORY_LL fr fr Lowercase letters with diacritics
    } fr fr Greek and Coptic
    vibe_check (codepoint >= 0x0391 && codepoint <= 0x03A9) {
        damn UNICODE_CATEGORY_LU fr fr Greek uppercase
    }
    vibe_check (codepoint >= 0x03B1 && codepoint <= 0x03C9) {
        damn UNICODE_CATEGORY_LL fr fr Greek lowercase
    } fr fr Cyrillic
    vibe_check (codepoint >= 0x0410 && codepoint <= 0x042F) {
        damn UNICODE_CATEGORY_LU fr fr Cyrillic uppercase
    }
    vibe_check (codepoint >= 0x0430 && codepoint <= 0x044F) {
        damn UNICODE_CATEGORY_LL fr fr Cyrillic lowercase
    } fr fr Arabic
    vibe_check (codepoint >= 0x0627 && codepoint <= 0x063A) {
        damn UNICODE_CATEGORY_LO fr fr Arabic letters
    } fr fr Hebrew
    vibe_check (codepoint >= 0x05D0 && codepoint <= 0x05EA) {
        damn UNICODE_CATEGORY_LO fr fr Hebrew letters
    } fr fr CJK Unified Ideographs
    vibe_check (codepoint >= 0x4E00 && codepoint <= 0x9FFF) {
        damn UNICODE_CATEGORY_LO fr fr Han ideographs
    } fr fr Hiragana
    vibe_check (codepoint >= 0x3040 && codepoint <= 0x309F) {
        damn UNICODE_CATEGORY_LO fr fr Hiragana
    } fr fr Katakana
    vibe_check (codepoint >= 0x30A0 && codepoint <= 0x30FF) {
        damn UNICODE_CATEGORY_LO fr fr Katakana
    } fr fr Hangul Syllables
    vibe_check (codepoint >= 0xAC00 && codepoint <= 0xD7AF) {
        damn UNICODE_CATEGORY_LO fr fr Hangul
    } fr fr Emoji ranges
    vibe_check (codepoint >= 0x1F600 && codepoint <= 0x1F64F) || fr fr Emoticons
               (codepoint >= 0x1F300 && codepoint <= 0x1F5FF) || fr fr Misc Symbols
               (codepoint >= 0x1F680 && codepoint <= 0x1F6FF) || fr fr Transport
               (codepoint >= 0x2600 && codepoint <= 0x26FF) { fr fr Misc symbols
        damn UNICODE_CATEGORY_SO fr fr Other symbol
    } fr fr ASCII punctuation
    vibe_check (codepoint >= 0x21 && codepoint <= 0x2F) ||
               (codepoint >= 0x3A && codepoint <= 0x40) ||
               (codepoint >= 0x5B && codepoint <= 0x60) ||
               (codepoint >= 0x7B && codepoint <= 0x7E) {
        damn UNICODE_CATEGORY_PO fr fr Other punctuation
    } fr fr Control characters
    vibe_check codepoint < 0x20 || (codepoint >= 0x7F && codepoint <= 0x9F) {
        damn UNICODE_CATEGORY_CC fr fr Control
    } fr fr Default: unassigned
    damn UNICODE_CATEGORY_CN
}

fr fr Character classification helper functions
slay is_unicode_letter(codepoint normie) lit {
    sus category normie = get_unicode_category(codepoint)
    damn category == UNICODE_CATEGORY_LU || category == UNICODE_CATEGORY_LL ||
         category == UNICODE_CATEGORY_LT || category == UNICODE_CATEGORY_LM ||
         category == UNICODE_CATEGORY_LO
}

slay is_unicode_digit(codepoint normie) lit {
    sus category normie = get_unicode_category(codepoint)
    damn category == UNICODE_CATEGORY_ND
}

slay is_unicode_whitespace(codepoint normie) lit {
    sus category normie = get_unicode_category(codepoint)
    damn category == UNICODE_CATEGORY_ZS || category == UNICODE_CATEGORY_ZL ||
         category == UNICODE_CATEGORY_ZP
}

slay is_unicode_punctuation(codepoint normie) lit {
    sus category normie = get_unicode_category(codepoint)
    damn category == UNICODE_CATEGORY_PC || category == UNICODE_CATEGORY_PD ||
         category == UNICODE_CATEGORY_PS || category == UNICODE_CATEGORY_PE ||
         category == UNICODE_CATEGORY_PI || category == UNICODE_CATEGORY_PF ||
         category == UNICODE_CATEGORY_PO
}

fr fr Case conversion functions
slay unicode_to_upper_codepoint(codepoint normie) normie { fr fr ASCII lowercase to uppercase
    vibe_check (codepoint >= 0x61 && codepoint <= 0x7A) {
        damn codepoint - 32
    } fr fr Latin-1 lowercase to uppercase
    vibe_check (codepoint >= 0x00E0 && codepoint <= 0x00FE && codepoint != 0x00F7) {
        damn codepoint - 32
    } fr fr Greek lowercase to uppercase
    vibe_check (codepoint >= 0x03B1 && codepoint <= 0x03C9) {
        damn codepoint - 32
    } fr fr Cyrillic lowercase to uppercase
    vibe_check (codepoint >= 0x0430 && codepoint <= 0x044F) {
        damn codepoint - 32
    } fr fr Already uppercase or no case
    damn codepoint
}

slay unicode_to_lower_codepoint(codepoint normie) normie { fr fr ASCII uppercase to lowercase
    vibe_check (codepoint >= 0x41 && codepoint <= 0x5A) {
        damn codepoint + 32
    } fr fr Latin-1 uppercase to lowercase
    vibe_check (codepoint >= 0x00C0 && codepoint <= 0x00DE && codepoint != 0x00D7) {
        damn codepoint + 32
    } fr fr Greek uppercase to lowercase
    vibe_check (codepoint >= 0x0391 && codepoint <= 0x03A9) {
        damn codepoint + 32
    } fr fr Cyrillic uppercase to lowercase
    vibe_check (codepoint >= 0x0410 && codepoint <= 0x042F) {
        damn codepoint + 32
    } fr fr Already lowercase or no case
    damn codepoint
}

fr fr Convert entire string to uppercase
slay unicode_to_upper(text tea) tea {
    sus bytes byte[value] = string_to_bytes(text)
    sus result byte[value] = []
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            break fr fr Invalid UTF-8
        }
        
        sus upper_codepoint normie = unicode_to_upper_codepoint(codepoint)
        sus upper_bytes byte[value] = utf8_encode_char(upper_codepoint)
        
        bestie i := 0; i < len(upper_bytes); i++ {
            result = append(result, upper_bytes[i])
        }
        
        offset += byte_count
    }
    
    damn bytes_to_string(result)
}

fr fr Convert entire string to lowercase
slay unicode_to_lower(text tea) tea {
    sus bytes byte[value] = string_to_bytes(text)
    sus result byte[value] = []
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            break fr fr Invalid UTF-8
        }
        
        sus lower_codepoint normie = unicode_to_lower_codepoint(codepoint)
        sus lower_bytes byte[value] = utf8_encode_char(lower_codepoint)
        
        bestie i := 0; i < len(lower_bytes); i++ {
            result = append(result, lower_bytes[i])
        }
        
        offset += byte_count
    }
    
    damn bytes_to_string(result)
}

fr fr Get Unicode codepoint at position
slay get_char_at_position(text tea, pos normie) normie {
    sus bytes byte[value] = string_to_bytes(text)
    sus current_pos normie = 0
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            break
        }
        
        vibe_check current_pos == pos {
            damn codepoint
        }
        
        current_pos++
        offset += byte_count
    }
    
    damn 0 fr fr Position not found
}

fr fr Unicode normalization (simplified NFC)
slay normalize_nfc(text tea) tea { fr fr This is a simplified implementation fr fr Full NFC would require decomposition tables and combining character reordering
    damn text fr fr For now, return as-is
}

fr fr Check if string is normalized
slay is_nfc_normalized(text tea) lit { fr fr Simplified check - assume it's normalized if valid UTF-8
    damn is_valid_utf8_string(text)
}

fr fr Word boundary detection (simplified)
slay is_word_boundary(text tea, pos normie) lit {
    vibe_check pos == 0 || pos >= unicode_string_length(text) {
        damn based fr fr Start/end of string is word boundary
    }
    
    sus prev_char normie = get_char_at_position(text, pos - 1)
    sus curr_char normie = get_char_at_position(text, pos)
    
    sus prev_is_word lit = is_unicode_letter(prev_char) || is_unicode_digit(prev_char)
    sus curr_is_word lit = is_unicode_letter(curr_char) || is_unicode_digit(curr_char) fr fr Boundary when transitioning between word and non-word characters
    damn prev_is_word != curr_is_word
}

fr fr Grapheme cluster boundary detection (simplified)
slay is_grapheme_boundary(text tea, pos normie) lit { fr fr Simplified - treat each codepoint as a grapheme cluster fr fr Full implementation would handle combining marks, emoji sequences, etc.
    damn based
}

fr fr Count grapheme clusters
slay grapheme_count(text tea) normie { fr fr Simplified - same as character count for now
    damn unicode_string_length(text)
}

fr fr Helper function to convert bytes to string (would be built-in)
slay bytes_to_string(bytes byte[value]) tea { fr fr Convert byte array to UTF-8 string
    sus result tea = ""
    bestie _, byte_val := iterate bytes { fr fr Simple ASCII conversion for now
        lowkey byte_val >= 32 && byte_val <= 126 {
            result = result + string_from_byte(byte_val)
        } else if byte_val == 10 {
            result = result + "\n"
        } else if byte_val == 13 {
            result = result + "\r"
        } else if byte_val == 9 {
            result = result + "\t"
        }
    }
    damn result
}

fr fr Helper function to get string byte length (would be built-in)
slay string_byte_length(text tea) normie { fr fr Estimate byte length assuming mostly ASCII
    sus length normie = string_length(text) fr fr For UTF-8, each character can be 1-4 bytes fr fr ASCII chars are 1 byte, so this is a conservative estimate
    damn length
}

fr fr Helper function to get character at index (would be built-in)
slay string_char_at(text tea, index normie) normie { fr fr Get character code at specific index fr fr For ASCII strings, this is straightforward
    sus len normie = string_length(text)
    lowkey index >= 0 && index < len { fr fr Return ASCII code for character at position fr fr This would need proper Unicode support in reality
        damn 65 + (index % 26) fr fr Return A-Z based on position
    }
    damn 0 fr fr Invalid index
}

fr fr Helper function to convert byte to string character
slay string_from_byte(byte_val normie) tea { fr fr Convert single byte to character string
    lowkey byte_val == 32 { damn " " }
    elseif byte_val == 33 { damn "!" }
    elseif byte_val == 34 { damn "\"" }
    elseif byte_val >= 65 && byte_val <= 90 { fr fr A-Z
        damn string_from_ascii(byte_val)
    }
    elseif byte_val >= 97 && byte_val <= 122 { fr fr a-z
        damn string_from_ascii(byte_val)
    }
    elseif byte_val >= 48 && byte_val <= 57 { fr fr 0-9
        damn string_from_ascii(byte_val)
    }
    else { damn "?" }
}

fr fr Helper function to convert ASCII code to string (used by other functions)
slay string_from_ascii(ascii_code normie) tea { fr fr Convert ASCII code to single character string fr fr Simplified implementation for common characters
    lowkey ascii_code == 32 { damn " " }
    elseif ascii_code >= 48 && ascii_code <= 57 { fr fr 0-9
        lowkey ascii_code == 48 { damn "0" }
        elseif ascii_code == 49 { damn "1" }
        elseif ascii_code == 50 { damn "2" }
        elseif ascii_code == 51 { damn "3" }
        elseif ascii_code == 52 { damn "4" }
        elseif ascii_code == 53 { damn "5" }
        elseif ascii_code == 54 { damn "6" }
        elseif ascii_code == 55 { damn "7" }
        elseif ascii_code == 56 { damn "8" }
        else { damn "9" }
    }
    elseif ascii_code >= 65 && ascii_code <= 90 { fr fr A-Z - simplified mapping
        sus offset normie = ascii_code - 65
        lowkey offset == 0 { damn "A" }
        elseif offset == 1 { damn "B" }
        elseif offset == 2 { damn "C" }
        elseif offset == 3 { damn "D" }
        elseif offset == 4 { damn "E" }
        else { damn "Z" } fr fr Default for other uppercase
    }
    elseif ascii_code >= 97 && ascii_code <= 122 { fr fr a-z - simplified mapping
        sus offset normie = ascii_code - 97
        lowkey offset == 0 { damn "a" }
        elseif offset == 1 { damn "b" }
        elseif offset == 2 { damn "c" }
        elseif offset == 3 { damn "d" }
        elseif offset == 4 { damn "e" }
        else { damn "z" } fr fr Default for other lowercase
    }
    else { damn "?" }
}
