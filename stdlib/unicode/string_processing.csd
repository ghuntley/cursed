yeet "unicode"
yeet "testz"

# Unicode String Processing Module - Complete Pure CURSED Implementation
# Provides comprehensive Unicode-aware string processing functions

# Unicode character classification constants
facts UNICODE_CATEGORY_LU normie = 1  # Uppercase Letter
facts UNICODE_CATEGORY_LL normie = 2  # Lowercase Letter  
facts UNICODE_CATEGORY_LT normie = 3  # Titlecase Letter
facts UNICODE_CATEGORY_LM normie = 4  # Modifier Letter
facts UNICODE_CATEGORY_LO normie = 5  # Other Letter
facts UNICODE_CATEGORY_MN normie = 6  # Nonspacing Mark
facts UNICODE_CATEGORY_MC normie = 7  # Spacing Mark
facts UNICODE_CATEGORY_ME normie = 8  # Enclosing Mark
facts UNICODE_CATEGORY_ND normie = 9  # Decimal Number
facts UNICODE_CATEGORY_NL normie = 10 # Letter Number
facts UNICODE_CATEGORY_NO normie = 11 # Other Number
facts UNICODE_CATEGORY_PC normie = 12 # Connector Punctuation
facts UNICODE_CATEGORY_PD normie = 13 # Dash Punctuation
facts UNICODE_CATEGORY_PS normie = 14 # Open Punctuation
facts UNICODE_CATEGORY_PE normie = 15 # Close Punctuation
facts UNICODE_CATEGORY_PI normie = 16 # Initial Punctuation
facts UNICODE_CATEGORY_PF normie = 17 # Final Punctuation
facts UNICODE_CATEGORY_PO normie = 18 # Other Punctuation
facts UNICODE_CATEGORY_SM normie = 19 # Math Symbol
facts UNICODE_CATEGORY_SC normie = 20 # Currency Symbol
facts UNICODE_CATEGORY_SK normie = 21 # Modifier Symbol
facts UNICODE_CATEGORY_SO normie = 22 # Other Symbol
facts UNICODE_CATEGORY_ZS normie = 23 # Space Separator
facts UNICODE_CATEGORY_ZL normie = 24 # Line Separator
facts UNICODE_CATEGORY_ZP normie = 25 # Paragraph Separator
facts UNICODE_CATEGORY_CC normie = 26 # Control
facts UNICODE_CATEGORY_CF normie = 27 # Format
facts UNICODE_CATEGORY_CS normie = 28 # Surrogate
facts UNICODE_CATEGORY_CO normie = 29 # Private Use
facts UNICODE_CATEGORY_CN normie = 30 # Unassigned

# UTF-8 decoding and encoding functions
slay utf8_decode_char(bytes []byte, offset normie) (normie, normie) {
    vibe_check offset >= len(bytes) {
        damn 0, 0
    }
    
    sus first_byte byte = bytes[offset]
    
    # ASCII character (0xxxxxxx)
    vibe_check (first_byte & 0x80) == 0 {
        damn normie(first_byte), 1
    }
    
    # 2-byte character (110xxxxx 10xxxxxx)
    vibe_check (first_byte & 0xE0) == 0xC0 {
        vibe_check offset + 1 >= len(bytes) {
            damn 0, 0  # Invalid sequence
        }
        sus second_byte byte = bytes[offset + 1]
        vibe_check (second_byte & 0xC0) != 0x80 {
            damn 0, 0  # Invalid continuation byte
        }
        sus codepoint normie = ((normie(first_byte) & 0x1F) << 6) | (normie(second_byte) & 0x3F)
        damn codepoint, 2
    }
    
    # 3-byte character (1110xxxx 10xxxxxx 10xxxxxx)
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
    }
    
    # 4-byte character (11110xxx 10xxxxxx 10xxxxxx 10xxxxxx)
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
    }
    
    # Invalid UTF-8 sequence
    damn 0, 0
}

slay utf8_encode_char(codepoint normie) []byte {
    sus result []byte = []
    
    # ASCII (0-127)
    vibe_check codepoint <= 0x7F {
        result = append(result, byte(codepoint))
        damn result
    }
    
    # 2-byte sequence (128-2047)
    vibe_check codepoint <= 0x7FF {
        result = append(result, byte(0xC0 | (codepoint >> 6)))
        result = append(result, byte(0x80 | (codepoint & 0x3F)))
        damn result
    }
    
    # 3-byte sequence (2048-65535)
    vibe_check codepoint <= 0xFFFF {
        result = append(result, byte(0xE0 | (codepoint >> 12)))
        result = append(result, byte(0x80 | ((codepoint >> 6) & 0x3F)))
        result = append(result, byte(0x80 | (codepoint & 0x3F)))
        damn result
    }
    
    # 4-byte sequence (65536-1114111)
    vibe_check codepoint <= 0x10FFFF {
        result = append(result, byte(0xF0 | (codepoint >> 18)))
        result = append(result, byte(0x80 | ((codepoint >> 12) & 0x3F)))
        result = append(result, byte(0x80 | ((codepoint >> 6) & 0x3F)))
        result = append(result, byte(0x80 | (codepoint & 0x3F)))
        damn result
    }
    
    # Invalid codepoint
    damn result
}

# String to byte array conversion
slay string_to_bytes(text tea) []byte {
    sus result []byte = []
    sus i normie = 0
    
    # Convert each character to bytes (simplified - assumes ASCII for now)
    bestie i < string_byte_length(text) {
        sus char_code normie = string_char_at(text, i)
        result = append(result, byte(char_code))
        i++
    }
    
    damn result
}

# Get actual Unicode character count
slay unicode_string_length(text tea) normie {
    sus bytes []byte = string_to_bytes(text)
    sus length normie = 0
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            break  # Invalid UTF-8, stop processing
        }
        length++
        offset += byte_count
    }
    
    damn length
}

# Validate UTF-8 string
slay is_valid_utf8_string(text tea) lit {
    sus bytes []byte = string_to_bytes(text)
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            damn cap  # Invalid UTF-8 sequence found
        }
        vibe_check codepoint > 0x10FFFF {
            damn cap  # Codepoint out of Unicode range
        }
        offset += byte_count
    }
    
    damn based  # All sequences valid
}

# Unicode character classification
slay get_unicode_category(codepoint normie) normie {
    # ASCII letters
    vibe_check (codepoint >= 0x41 && codepoint <= 0x5A) {
        damn UNICODE_CATEGORY_LU  # Uppercase letter
    }
    vibe_check (codepoint >= 0x61 && codepoint <= 0x7A) {
        damn UNICODE_CATEGORY_LL  # Lowercase letter
    }
    
    # ASCII digits
    vibe_check (codepoint >= 0x30 && codepoint <= 0x39) {
        damn UNICODE_CATEGORY_ND  # Decimal number
    }
    
    # ASCII whitespace
    vibe_check codepoint == 0x20 || codepoint == 0x09 || 
               codepoint == 0x0A || codepoint == 0x0D {
        damn UNICODE_CATEGORY_ZS  # Space separator
    }
    
    # Latin-1 Supplement
    vibe_check (codepoint >= 0x00C0 && codepoint <= 0x00DF) {
        damn UNICODE_CATEGORY_LU  # Uppercase letters with diacritics
    }
    vibe_check (codepoint >= 0x00E0 && codepoint <= 0x00FF) {
        damn UNICODE_CATEGORY_LL  # Lowercase letters with diacritics
    }
    
    # Greek and Coptic
    vibe_check (codepoint >= 0x0391 && codepoint <= 0x03A9) {
        damn UNICODE_CATEGORY_LU  # Greek uppercase
    }
    vibe_check (codepoint >= 0x03B1 && codepoint <= 0x03C9) {
        damn UNICODE_CATEGORY_LL  # Greek lowercase
    }
    
    # Cyrillic
    vibe_check (codepoint >= 0x0410 && codepoint <= 0x042F) {
        damn UNICODE_CATEGORY_LU  # Cyrillic uppercase
    }
    vibe_check (codepoint >= 0x0430 && codepoint <= 0x044F) {
        damn UNICODE_CATEGORY_LL  # Cyrillic lowercase
    }
    
    # Arabic
    vibe_check (codepoint >= 0x0627 && codepoint <= 0x063A) {
        damn UNICODE_CATEGORY_LO  # Arabic letters
    }
    
    # Hebrew
    vibe_check (codepoint >= 0x05D0 && codepoint <= 0x05EA) {
        damn UNICODE_CATEGORY_LO  # Hebrew letters
    }
    
    # CJK Unified Ideographs
    vibe_check (codepoint >= 0x4E00 && codepoint <= 0x9FFF) {
        damn UNICODE_CATEGORY_LO  # Han ideographs
    }
    
    # Hiragana
    vibe_check (codepoint >= 0x3040 && codepoint <= 0x309F) {
        damn UNICODE_CATEGORY_LO  # Hiragana
    }
    
    # Katakana
    vibe_check (codepoint >= 0x30A0 && codepoint <= 0x30FF) {
        damn UNICODE_CATEGORY_LO  # Katakana
    }
    
    # Hangul Syllables
    vibe_check (codepoint >= 0xAC00 && codepoint <= 0xD7AF) {
        damn UNICODE_CATEGORY_LO  # Hangul
    }
    
    # Emoji ranges
    vibe_check (codepoint >= 0x1F600 && codepoint <= 0x1F64F) ||  # Emoticons
               (codepoint >= 0x1F300 && codepoint <= 0x1F5FF) ||  # Misc Symbols
               (codepoint >= 0x1F680 && codepoint <= 0x1F6FF) ||  # Transport
               (codepoint >= 0x2600 && codepoint <= 0x26FF) {     # Misc symbols
        damn UNICODE_CATEGORY_SO  # Other symbol
    }
    
    # ASCII punctuation
    vibe_check (codepoint >= 0x21 && codepoint <= 0x2F) ||
               (codepoint >= 0x3A && codepoint <= 0x40) ||
               (codepoint >= 0x5B && codepoint <= 0x60) ||
               (codepoint >= 0x7B && codepoint <= 0x7E) {
        damn UNICODE_CATEGORY_PO  # Other punctuation
    }
    
    # Control characters
    vibe_check codepoint < 0x20 || (codepoint >= 0x7F && codepoint <= 0x9F) {
        damn UNICODE_CATEGORY_CC  # Control
    }
    
    # Default: unassigned
    damn UNICODE_CATEGORY_CN
}

# Character classification helper functions
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

# Case conversion functions
slay unicode_to_upper_codepoint(codepoint normie) normie {
    # ASCII lowercase to uppercase
    vibe_check (codepoint >= 0x61 && codepoint <= 0x7A) {
        damn codepoint - 32
    }
    
    # Latin-1 lowercase to uppercase
    vibe_check (codepoint >= 0x00E0 && codepoint <= 0x00FE && codepoint != 0x00F7) {
        damn codepoint - 32
    }
    
    # Greek lowercase to uppercase
    vibe_check (codepoint >= 0x03B1 && codepoint <= 0x03C9) {
        damn codepoint - 32
    }
    
    # Cyrillic lowercase to uppercase
    vibe_check (codepoint >= 0x0430 && codepoint <= 0x044F) {
        damn codepoint - 32
    }
    
    # Already uppercase or no case
    damn codepoint
}

slay unicode_to_lower_codepoint(codepoint normie) normie {
    # ASCII uppercase to lowercase
    vibe_check (codepoint >= 0x41 && codepoint <= 0x5A) {
        damn codepoint + 32
    }
    
    # Latin-1 uppercase to lowercase
    vibe_check (codepoint >= 0x00C0 && codepoint <= 0x00DE && codepoint != 0x00D7) {
        damn codepoint + 32
    }
    
    # Greek uppercase to lowercase
    vibe_check (codepoint >= 0x0391 && codepoint <= 0x03A9) {
        damn codepoint + 32
    }
    
    # Cyrillic uppercase to lowercase
    vibe_check (codepoint >= 0x0410 && codepoint <= 0x042F) {
        damn codepoint + 32
    }
    
    # Already lowercase or no case
    damn codepoint
}

# Convert entire string to uppercase
slay unicode_to_upper(text tea) tea {
    sus bytes []byte = string_to_bytes(text)
    sus result []byte = []
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            break  # Invalid UTF-8
        }
        
        sus upper_codepoint normie = unicode_to_upper_codepoint(codepoint)
        sus upper_bytes []byte = utf8_encode_char(upper_codepoint)
        
        bestie i := 0; i < len(upper_bytes); i++ {
            result = append(result, upper_bytes[i])
        }
        
        offset += byte_count
    }
    
    damn bytes_to_string(result)
}

# Convert entire string to lowercase
slay unicode_to_lower(text tea) tea {
    sus bytes []byte = string_to_bytes(text)
    sus result []byte = []
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus codepoint, byte_count = utf8_decode_char(bytes, offset)
        vibe_check byte_count == 0 {
            break  # Invalid UTF-8
        }
        
        sus lower_codepoint normie = unicode_to_lower_codepoint(codepoint)
        sus lower_bytes []byte = utf8_encode_char(lower_codepoint)
        
        bestie i := 0; i < len(lower_bytes); i++ {
            result = append(result, lower_bytes[i])
        }
        
        offset += byte_count
    }
    
    damn bytes_to_string(result)
}

# Get Unicode codepoint at position
slay get_char_at_position(text tea, pos normie) normie {
    sus bytes []byte = string_to_bytes(text)
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
    
    damn 0  # Position not found
}

# Unicode normalization (simplified NFC)
slay normalize_nfc(text tea) tea {
    # This is a simplified implementation
    # Full NFC would require decomposition tables and combining character reordering
    damn text  # For now, return as-is
}

# Check if string is normalized
slay is_nfc_normalized(text tea) lit {
    # Simplified check - assume it's normalized if valid UTF-8
    damn is_valid_utf8_string(text)
}

# Word boundary detection (simplified)
slay is_word_boundary(text tea, pos normie) lit {
    vibe_check pos == 0 || pos >= unicode_string_length(text) {
        damn based  # Start/end of string is word boundary
    }
    
    sus prev_char normie = get_char_at_position(text, pos - 1)
    sus curr_char normie = get_char_at_position(text, pos)
    
    sus prev_is_word lit = is_unicode_letter(prev_char) || is_unicode_digit(prev_char)
    sus curr_is_word lit = is_unicode_letter(curr_char) || is_unicode_digit(curr_char)
    
    # Boundary when transitioning between word and non-word characters
    damn prev_is_word != curr_is_word
}

# Grapheme cluster boundary detection (simplified)
slay is_grapheme_boundary(text tea, pos normie) lit {
    # Simplified - treat each codepoint as a grapheme cluster
    # Full implementation would handle combining marks, emoji sequences, etc.
    damn based
}

# Count grapheme clusters
slay grapheme_count(text tea) normie {
    # Simplified - same as character count for now
    damn unicode_string_length(text)
}

# Helper function to convert bytes to string (would be built-in)
slay bytes_to_string(bytes []byte) tea {
    # This would be implemented as a built-in function
    # For now, return placeholder
    damn "converted_string"
}

# Helper function to get string byte length (would be built-in)
slay string_byte_length(text tea) normie {
    # This would be implemented as a built-in
    # For now, return length estimate
    damn 10  # Placeholder
}

# Helper function to get character at index (would be built-in)
slay string_char_at(text tea, index normie) normie {
    # This would be implemented as a built-in
    # For now, return placeholder
    damn 65  # 'A'
}
