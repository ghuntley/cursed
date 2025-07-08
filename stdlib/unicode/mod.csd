yeet "testz"

# Unicode and UTF-8 Validation Module
# Pure CURSED implementation for Unicode character handling

# UTF-8 byte sequence validation
slay is_utf8_start_byte(byte normie) lit {
    # Check if byte is valid UTF-8 start byte
    damn ((byte & 0x80) == 0) ||     # ASCII (0xxxxxxx)
         ((byte & 0xE0) == 0xC0) ||  # 2-byte start (110xxxxx)
         ((byte & 0xF0) == 0xE0) ||  # 3-byte start (1110xxxx)
         ((byte & 0xF8) == 0xF0)     # 4-byte start (11110xxx)
}

slay is_utf8_continuation_byte(byte normie) lit {
    # Check if byte is UTF-8 continuation byte (10xxxxxx)
    damn (byte & 0xC0) == 0x80
}

# Get expected byte count for UTF-8 sequence
slay utf8_sequence_length(first_byte normie) normie {
    sus length normie = 0
    
    bestie first_byte >= 0 && first_byte <= 127 {
        length = 1  # ASCII
    } nah bestie (first_byte & 0xE0) == 0xC0 {
        length = 2  # 2-byte sequence
    } nah bestie (first_byte & 0xF0) == 0xE0 {
        length = 3  # 3-byte sequence
    } nah bestie (first_byte & 0xF8) == 0xF0 {
        length = 4  # 4-byte sequence
    } nah {
        length = 0  # Invalid
    }
    
    damn length
}

# Validate UTF-8 byte sequence
slay validate_utf8_sequence(bytes []normie, start_pos normie) lit {
    sus len normie = len(bytes)
    sus pos normie = start_pos
    
    bestie pos >= len {
        damn cap  # Position out of bounds
    }
    
    sus first_byte normie = bytes[pos]
    sus expected_length normie = utf8_sequence_length(first_byte)
    
    bestie expected_length == 0 {
        damn cap  # Invalid start byte
    }
    
    bestie pos + expected_length > len {
        damn cap  # Sequence extends beyond buffer
    }
    
    # Validate continuation bytes
    sus i normie = 1
    bestie i < expected_length {
        sus byte_val normie = bytes[pos + i]
        bestie !is_utf8_continuation_byte(byte_val) {
            damn cap  # Invalid continuation byte
        }
        i++
    }
    
    damn based
}

# Convert UTF-8 bytes to Unicode code point
slay utf8_to_codepoint(bytes []normie, start_pos normie) normie {
    sus pos normie = start_pos
    sus first_byte normie = bytes[pos]
    sus length normie = utf8_sequence_length(first_byte)
    sus codepoint normie = 0
    
    bestie length == 1 {
        codepoint = first_byte
    } nah bestie length == 2 {
        codepoint = (first_byte & 0x1F) << 6
        codepoint |= (bytes[pos + 1] & 0x3F)
    } nah bestie length == 3 {
        codepoint = (first_byte & 0x0F) << 12
        codepoint |= (bytes[pos + 1] & 0x3F) << 6
        codepoint |= (bytes[pos + 2] & 0x3F)
    } nah bestie length == 4 {
        codepoint = (first_byte & 0x07) << 18
        codepoint |= (bytes[pos + 1] & 0x3F) << 12
        codepoint |= (bytes[pos + 2] & 0x3F) << 6
        codepoint |= (bytes[pos + 3] & 0x3F)
    }
    
    damn codepoint
}

# Convert Unicode code point to UTF-8 bytes
slay codepoint_to_utf8(codepoint normie) []normie {
    sus result []normie = []
    
    bestie codepoint <= 0x7F {
        # 1-byte sequence (ASCII)
        result = append(result, codepoint)
    } nah bestie codepoint <= 0x7FF {
        # 2-byte sequence
        result = append(result, 0xC0 | (codepoint >> 6))
        result = append(result, 0x80 | (codepoint & 0x3F))
    } nah bestie codepoint <= 0xFFFF {
        # 3-byte sequence
        result = append(result, 0xE0 | (codepoint >> 12))
        result = append(result, 0x80 | ((codepoint >> 6) & 0x3F))
        result = append(result, 0x80 | (codepoint & 0x3F))
    } nah bestie codepoint <= 0x10FFFF {
        # 4-byte sequence
        result = append(result, 0xF0 | (codepoint >> 18))
        result = append(result, 0x80 | ((codepoint >> 12) & 0x3F))
        result = append(result, 0x80 | ((codepoint >> 6) & 0x3F))
        result = append(result, 0x80 | (codepoint & 0x3F))
    }
    
    damn result
}

# Validate entire UTF-8 string
slay validate_utf8_string(text tea) lit {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    
    bestie pos < len {
        bestie !validate_utf8_sequence(bytes, pos) {
            damn cap  # Invalid UTF-8 sequence found
        }
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn based
}

# Count UTF-8 characters in string (not bytes)
slay utf8_char_count(text tea) normie {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus count normie = 0
    
    bestie pos < len {
        sus seq_len normie = utf8_sequence_length(bytes[pos])
        bestie seq_len == 0 {
            damn -1  # Invalid UTF-8
        }
        pos += seq_len
        count++
    }
    
    damn count
}

# Get byte length of UTF-8 string
slay utf8_byte_count(text tea) normie {
    sus bytes []normie = string_to_bytes(text)
    damn len(bytes)
}

# Unicode character classification
slay is_ascii(codepoint normie) lit {
    damn codepoint >= 0 && codepoint <= 0x7F
}

slay is_latin1(codepoint normie) lit {
    damn codepoint >= 0 && codepoint <= 0xFF
}

slay is_bmp(codepoint normie) lit {
    damn codepoint >= 0 && codepoint <= 0xFFFF
}

slay is_valid_unicode(codepoint normie) lit {
    damn codepoint >= 0 && codepoint <= 0x10FFFF
}

# Basic Unicode character type classification
slay is_unicode_digit(codepoint normie) lit {
    # Basic ASCII digits and some common Unicode digits
    damn (codepoint >= 0x30 && codepoint <= 0x39) ||  # ASCII 0-9
         (codepoint >= 0x0660 && codepoint <= 0x0669) ||  # Arabic-Indic digits
         (codepoint >= 0x06F0 && codepoint <= 0x06F9) ||  # Extended Arabic-Indic digits
         (codepoint >= 0x0966 && codepoint <= 0x096F)     # Devanagari digits
}

slay is_unicode_letter(codepoint normie) lit {
    # Basic ASCII letters and some common Unicode ranges
    damn (codepoint >= 0x41 && codepoint <= 0x5A) ||  # ASCII A-Z
         (codepoint >= 0x61 && codepoint <= 0x7A) ||  # ASCII a-z
         (codepoint >= 0x00C0 && codepoint <= 0x00D6) ||  # Latin-1 Supplement
         (codepoint >= 0x00D8 && codepoint <= 0x00F6) ||  # Latin-1 Supplement
         (codepoint >= 0x00F8 && codepoint <= 0x00FF) ||  # Latin-1 Supplement
         (codepoint >= 0x0100 && codepoint <= 0x017F) ||  # Latin Extended-A
         (codepoint >= 0x0180 && codepoint <= 0x024F)     # Latin Extended-B
}

slay is_unicode_whitespace(codepoint normie) lit {
    # Common Unicode whitespace characters
    damn codepoint == 0x0009 ||  # Tab
         codepoint == 0x000A ||  # Line Feed
         codepoint == 0x000B ||  # Vertical Tab
         codepoint == 0x000C ||  # Form Feed
         codepoint == 0x000D ||  # Carriage Return
         codepoint == 0x0020 ||  # Space
         codepoint == 0x0085 ||  # Next Line
         codepoint == 0x00A0 ||  # No-Break Space
         codepoint == 0x1680 ||  # Ogham Space Mark
         codepoint == 0x2000 ||  # En Quad
         codepoint == 0x2001 ||  # Em Quad
         codepoint == 0x2002 ||  # En Space
         codepoint == 0x2003 ||  # Em Space
         codepoint == 0x2004 ||  # Three-Per-Em Space
         codepoint == 0x2005 ||  # Four-Per-Em Space
         codepoint == 0x2006 ||  # Six-Per-Em Space
         codepoint == 0x2007 ||  # Figure Space
         codepoint == 0x2008 ||  # Punctuation Space
         codepoint == 0x2009 ||  # Thin Space
         codepoint == 0x200A ||  # Hair Space
         codepoint == 0x2028 ||  # Line Separator
         codepoint == 0x2029 ||  # Paragraph Separator
         codepoint == 0x202F ||  # Narrow No-Break Space
         codepoint == 0x205F ||  # Medium Mathematical Space
         codepoint == 0x3000     # Ideographic Space
}

# Basic case conversion (ASCII + Latin-1 only for now)
slay to_unicode_upper(codepoint normie) normie {
    # ASCII lowercase to uppercase
    bestie codepoint >= 0x61 && codepoint <= 0x7A {
        damn codepoint - 0x20
    }
    
    # Latin-1 Supplement lowercase to uppercase
    bestie codepoint >= 0x00E0 && codepoint <= 0x00F6 {
        damn codepoint - 0x20
    }
    bestie codepoint >= 0x00F8 && codepoint <= 0x00FE {
        damn codepoint - 0x20
    }
    
    # Return unchanged if no conversion
    damn codepoint
}

slay to_unicode_lower(codepoint normie) normie {
    # ASCII uppercase to lowercase
    bestie codepoint >= 0x41 && codepoint <= 0x5A {
        damn codepoint + 0x20
    }
    
    # Latin-1 Supplement uppercase to lowercase
    bestie codepoint >= 0x00C0 && codepoint <= 0x00D6 {
        damn codepoint + 0x20
    }
    bestie codepoint >= 0x00D8 && codepoint <= 0x00DE {
        damn codepoint + 0x20
    }
    
    # Return unchanged if no conversion
    damn codepoint
}

# Convert string to uppercase with Unicode awareness
slay string_to_unicode_upper(text tea) tea {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus result []normie = []
    
    bestie pos < len {
        sus seq_len normie = utf8_sequence_length(bytes[pos])
        bestie seq_len == 0 {
            damn ""  # Invalid UTF-8
        }
        
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus upper_codepoint normie = to_unicode_upper(codepoint)
        sus utf8_bytes []normie = codepoint_to_utf8(upper_codepoint)
        
        result = append_bytes(result, utf8_bytes)
        pos += seq_len
    }
    
    damn bytes_to_string(result)
}

# Convert string to lowercase with Unicode awareness
slay string_to_unicode_lower(text tea) tea {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus result []normie = []
    
    bestie pos < len {
        sus seq_len normie = utf8_sequence_length(bytes[pos])
        bestie seq_len == 0 {
            damn ""  # Invalid UTF-8
        }
        
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus lower_codepoint normie = to_unicode_lower(codepoint)
        sus utf8_bytes []normie = codepoint_to_utf8(lower_codepoint)
        
        result = append_bytes(result, utf8_bytes)
        pos += seq_len
    }
    
    damn bytes_to_string(result)
}

# Get Unicode codepoint at character position
slay get_codepoint_at(text tea, char_pos normie) normie {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus current_char normie = 0
    
    bestie pos < len {
        bestie current_char == char_pos {
            damn utf8_to_codepoint(bytes, pos)
        }
        pos += utf8_sequence_length(bytes[pos])
        current_char++
    }
    
    damn -1  # Position not found
}

# Helper function to convert string to byte array (placeholder)
slay string_to_bytes(text tea) []normie {
    # This would be implemented as a built-in function
    # For now, return empty array as placeholder
    sus result []normie = []
    damn result
}

# Helper function to convert byte array to string (placeholder)
slay bytes_to_string(bytes []normie) tea {
    # This would be implemented as a built-in function
    # For now, return empty string as placeholder
    damn ""
}

# Helper function to append byte arrays
slay append_bytes(dest []normie, src []normie) []normie {
    sus result []normie = dest
    sus i normie = 0
    bestie i < len(src) {
        result = append(result, src[i])
        i++
    }
    damn result
}

# Normalize Unicode string (NFC normalization - basic implementation)
slay normalize_unicode_nfc(text tea) tea {
    # Basic implementation - just validate and return
    bestie validate_utf8_string(text) {
        damn text
    } nah {
        damn ""  # Invalid UTF-8
    }
}

# Check if string contains only ASCII characters
slay is_ascii_string(text tea) lit {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus i normie = 0
    
    bestie i < len {
        bestie bytes[i] > 0x7F {
            damn cap  # Non-ASCII character found
        }
        i++
    }
    
    damn based
}

# Get Unicode block name for codepoint (basic implementation)
slay get_unicode_block(codepoint normie) tea {
    bestie codepoint >= 0x0000 && codepoint <= 0x007F {
        damn "Basic Latin"
    } nah bestie codepoint >= 0x0080 && codepoint <= 0x00FF {
        damn "Latin-1 Supplement"
    } nah bestie codepoint >= 0x0100 && codepoint <= 0x017F {
        damn "Latin Extended-A"
    } nah bestie codepoint >= 0x0180 && codepoint <= 0x024F {
        damn "Latin Extended-B"
    } nah bestie codepoint >= 0x0250 && codepoint <= 0x02AF {
        damn "IPA Extensions"
    } nah bestie codepoint >= 0x02B0 && codepoint <= 0x02FF {
        damn "Spacing Modifier Letters"
    } nah bestie codepoint >= 0x0300 && codepoint <= 0x036F {
        damn "Combining Diacritical Marks"
    } nah bestie codepoint >= 0x0370 && codepoint <= 0x03FF {
        damn "Greek and Coptic"
    } nah bestie codepoint >= 0x0400 && codepoint <= 0x04FF {
        damn "Cyrillic"
    } nah bestie codepoint >= 0x0500 && codepoint <= 0x052F {
        damn "Cyrillic Supplement"
    } nah bestie codepoint >= 0x0530 && codepoint <= 0x058F {
        damn "Armenian"
    } nah bestie codepoint >= 0x0590 && codepoint <= 0x05FF {
        damn "Hebrew"
    } nah bestie codepoint >= 0x0600 && codepoint <= 0x06FF {
        damn "Arabic"
    } nah bestie codepoint >= 0x0700 && codepoint <= 0x074F {
        damn "Syriac"
    } nah bestie codepoint >= 0x0780 && codepoint <= 0x07BF {
        damn "Thaana"
    } nah bestie codepoint >= 0x0900 && codepoint <= 0x097F {
        damn "Devanagari"
    } nah bestie codepoint >= 0x0980 && codepoint <= 0x09FF {
        damn "Bengali"
    } nah bestie codepoint >= 0x0A00 && codepoint <= 0x0A7F {
        damn "Gurmukhi"
    } nah bestie codepoint >= 0x0A80 && codepoint <= 0x0AFF {
        damn "Gujarati"
    } nah bestie codepoint >= 0x0B00 && codepoint <= 0x0B7F {
        damn "Oriya"
    } nah bestie codepoint >= 0x0B80 && codepoint <= 0x0BFF {
        damn "Tamil"
    } nah bestie codepoint >= 0x0C00 && codepoint <= 0x0C7F {
        damn "Telugu"
    } nah bestie codepoint >= 0x0C80 && codepoint <= 0x0CFF {
        damn "Kannada"
    } nah bestie codepoint >= 0x0D00 && codepoint <= 0x0D7F {
        damn "Malayalam"
    } nah bestie codepoint >= 0x0D80 && codepoint <= 0x0DFF {
        damn "Sinhala"
    } nah bestie codepoint >= 0x0E00 && codepoint <= 0x0E7F {
        damn "Thai"
    } nah bestie codepoint >= 0x0E80 && codepoint <= 0x0EFF {
        damn "Lao"
    } nah bestie codepoint >= 0x0F00 && codepoint <= 0x0FFF {
        damn "Tibetan"
    } nah bestie codepoint >= 0x1000 && codepoint <= 0x109F {
        damn "Myanmar"
    } nah bestie codepoint >= 0x10A0 && codepoint <= 0x10FF {
        damn "Georgian"
    } nah bestie codepoint >= 0x1100 && codepoint <= 0x11FF {
        damn "Hangul Jamo"
    } nah bestie codepoint >= 0x1200 && codepoint <= 0x137F {
        damn "Ethiopic"
    } nah bestie codepoint >= 0x13A0 && codepoint <= 0x13FF {
        damn "Cherokee"
    } nah bestie codepoint >= 0x1400 && codepoint <= 0x167F {
        damn "Unified Canadian Aboriginal Syllabics"
    } nah bestie codepoint >= 0x1680 && codepoint <= 0x169F {
        damn "Ogham"
    } nah bestie codepoint >= 0x16A0 && codepoint <= 0x16FF {
        damn "Runic"
    } nah bestie codepoint >= 0x1700 && codepoint <= 0x171F {
        damn "Tagalog"
    } nah bestie codepoint >= 0x1720 && codepoint <= 0x173F {
        damn "Hanunoo"
    } nah bestie codepoint >= 0x1740 && codepoint <= 0x175F {
        damn "Buhid"
    } nah bestie codepoint >= 0x1760 && codepoint <= 0x177F {
        damn "Tagbanwa"
    } nah bestie codepoint >= 0x1780 && codepoint <= 0x17FF {
        damn "Khmer"
    } nah bestie codepoint >= 0x1800 && codepoint <= 0x18AF {
        damn "Mongolian"
    } nah bestie codepoint >= 0x1900 && codepoint <= 0x194F {
        damn "Limbu"
    } nah bestie codepoint >= 0x1950 && codepoint <= 0x197F {
        damn "Tai Le"
    } nah bestie codepoint >= 0x19E0 && codepoint <= 0x19FF {
        damn "Khmer Symbols"
    } nah bestie codepoint >= 0x1D00 && codepoint <= 0x1D7F {
        damn "Phonetic Extensions"
    } nah bestie codepoint >= 0x1E00 && codepoint <= 0x1EFF {
        damn "Latin Extended Additional"
    } nah bestie codepoint >= 0x1F00 && codepoint <= 0x1FFF {
        damn "Greek Extended"
    } nah bestie codepoint >= 0x2000 && codepoint <= 0x206F {
        damn "General Punctuation"
    } nah bestie codepoint >= 0x2070 && codepoint <= 0x209F {
        damn "Superscripts and Subscripts"
    } nah bestie codepoint >= 0x20A0 && codepoint <= 0x20CF {
        damn "Currency Symbols"
    } nah bestie codepoint >= 0x20D0 && codepoint <= 0x20FF {
        damn "Combining Diacritical Marks for Symbols"
    } nah bestie codepoint >= 0x2100 && codepoint <= 0x214F {
        damn "Letterlike Symbols"
    } nah bestie codepoint >= 0x2150 && codepoint <= 0x218F {
        damn "Number Forms"
    } nah bestie codepoint >= 0x2190 && codepoint <= 0x21FF {
        damn "Arrows"
    } nah bestie codepoint >= 0x2200 && codepoint <= 0x22FF {
        damn "Mathematical Operators"
    } nah bestie codepoint >= 0x2300 && codepoint <= 0x23FF {
        damn "Miscellaneous Technical"
    } nah bestie codepoint >= 0x2400 && codepoint <= 0x243F {
        damn "Control Pictures"
    } nah bestie codepoint >= 0x2440 && codepoint <= 0x245F {
        damn "Optical Character Recognition"
    } nah bestie codepoint >= 0x2460 && codepoint <= 0x24FF {
        damn "Enclosed Alphanumerics"
    } nah bestie codepoint >= 0x2500 && codepoint <= 0x257F {
        damn "Box Drawing"
    } nah bestie codepoint >= 0x2580 && codepoint <= 0x259F {
        damn "Block Elements"
    } nah bestie codepoint >= 0x25A0 && codepoint <= 0x25FF {
        damn "Geometric Shapes"
    } nah bestie codepoint >= 0x2600 && codepoint <= 0x26FF {
        damn "Miscellaneous Symbols"
    } nah bestie codepoint >= 0x2700 && codepoint <= 0x27BF {
        damn "Dingbats"
    } nah bestie codepoint >= 0x27C0 && codepoint <= 0x27EF {
        damn "Miscellaneous Mathematical Symbols-A"
    } nah bestie codepoint >= 0x27F0 && codepoint <= 0x27FF {
        damn "Supplemental Arrows-A"
    } nah bestie codepoint >= 0x2800 && codepoint <= 0x28FF {
        damn "Braille Patterns"
    } nah bestie codepoint >= 0x2900 && codepoint <= 0x297F {
        damn "Supplemental Arrows-B"
    } nah bestie codepoint >= 0x2980 && codepoint <= 0x29FF {
        damn "Miscellaneous Mathematical Symbols-B"
    } nah bestie codepoint >= 0x2A00 && codepoint <= 0x2AFF {
        damn "Supplemental Mathematical Operators"
    } nah bestie codepoint >= 0x2B00 && codepoint <= 0x2BFF {
        damn "Miscellaneous Symbols and Arrows"
    } nah bestie codepoint >= 0x2E80 && codepoint <= 0x2EFF {
        damn "CJK Radicals Supplement"
    } nah bestie codepoint >= 0x2F00 && codepoint <= 0x2FDF {
        damn "Kangxi Radicals"
    } nah bestie codepoint >= 0x2FF0 && codepoint <= 0x2FFF {
        damn "Ideographic Description Characters"
    } nah bestie codepoint >= 0x3000 && codepoint <= 0x303F {
        damn "CJK Symbols and Punctuation"
    } nah bestie codepoint >= 0x3040 && codepoint <= 0x309F {
        damn "Hiragana"
    } nah bestie codepoint >= 0x30A0 && codepoint <= 0x30FF {
        damn "Katakana"
    } nah bestie codepoint >= 0x3100 && codepoint <= 0x312F {
        damn "Bopomofo"
    } nah bestie codepoint >= 0x3130 && codepoint <= 0x318F {
        damn "Hangul Compatibility Jamo"
    } nah bestie codepoint >= 0x3190 && codepoint <= 0x319F {
        damn "Kanbun"
    } nah bestie codepoint >= 0x31A0 && codepoint <= 0x31BF {
        damn "Bopomofo Extended"
    } nah bestie codepoint >= 0x31F0 && codepoint <= 0x31FF {
        damn "Katakana Phonetic Extensions"
    } nah bestie codepoint >= 0x3200 && codepoint <= 0x32FF {
        damn "Enclosed CJK Letters and Months"
    } nah bestie codepoint >= 0x3300 && codepoint <= 0x33FF {
        damn "CJK Compatibility"
    } nah bestie codepoint >= 0x3400 && codepoint <= 0x4DBF {
        damn "CJK Unified Ideographs Extension A"
    } nah bestie codepoint >= 0x4DC0 && codepoint <= 0x4DFF {
        damn "Yijing Hexagram Symbols"
    } nah bestie codepoint >= 0x4E00 && codepoint <= 0x9FFF {
        damn "CJK Unified Ideographs"
    } nah bestie codepoint >= 0xA000 && codepoint <= 0xA48F {
        damn "Yi Syllables"
    } nah bestie codepoint >= 0xA490 && codepoint <= 0xA4CF {
        damn "Yi Radicals"
    } nah bestie codepoint >= 0xAC00 && codepoint <= 0xD7AF {
        damn "Hangul Syllables"
    } nah bestie codepoint >= 0xD800 && codepoint <= 0xDB7F {
        damn "High Surrogates"
    } nah bestie codepoint >= 0xDB80 && codepoint <= 0xDBFF {
        damn "High Private Use Surrogates"
    } nah bestie codepoint >= 0xDC00 && codepoint <= 0xDFFF {
        damn "Low Surrogates"
    } nah bestie codepoint >= 0xE000 && codepoint <= 0xF8FF {
        damn "Private Use Area"
    } nah bestie codepoint >= 0xF900 && codepoint <= 0xFAFF {
        damn "CJK Compatibility Ideographs"
    } nah bestie codepoint >= 0xFB00 && codepoint <= 0xFB4F {
        damn "Alphabetic Presentation Forms"
    } nah bestie codepoint >= 0xFB50 && codepoint <= 0xFDFF {
        damn "Arabic Presentation Forms-A"
    } nah bestie codepoint >= 0xFE00 && codepoint <= 0xFE0F {
        damn "Variation Selectors"
    } nah bestie codepoint >= 0xFE20 && codepoint <= 0xFE2F {
        damn "Combining Half Marks"
    } nah bestie codepoint >= 0xFE30 && codepoint <= 0xFE4F {
        damn "CJK Compatibility Forms"
    } nah bestie codepoint >= 0xFE50 && codepoint <= 0xFE6F {
        damn "Small Form Variants"
    } nah bestie codepoint >= 0xFE70 && codepoint <= 0xFEFF {
        damn "Arabic Presentation Forms-B"
    } nah bestie codepoint >= 0xFF00 && codepoint <= 0xFFEF {
        damn "Halfwidth and Fullwidth Forms"
    } nah bestie codepoint >= 0xFFF0 && codepoint <= 0xFFFF {
        damn "Specials"
    } nah bestie codepoint >= 0x10000 && codepoint <= 0x1007F {
        damn "Linear B Syllabary"
    } nah bestie codepoint >= 0x10080 && codepoint <= 0x100FF {
        damn "Linear B Ideograms"
    } nah bestie codepoint >= 0x10100 && codepoint <= 0x1013F {
        damn "Aegean Numbers"
    } nah bestie codepoint >= 0x10300 && codepoint <= 0x1032F {
        damn "Old Italic"
    } nah bestie codepoint >= 0x10330 && codepoint <= 0x1034F {
        damn "Gothic"
    } nah bestie codepoint >= 0x10380 && codepoint <= 0x1039F {
        damn "Ugaritic"
    } nah bestie codepoint >= 0x10400 && codepoint <= 0x1044F {
        damn "Deseret"
    } nah bestie codepoint >= 0x10450 && codepoint <= 0x1047F {
        damn "Shavian"
    } nah bestie codepoint >= 0x10480 && codepoint <= 0x104AF {
        damn "Osmanya"
    } nah bestie codepoint >= 0x10800 && codepoint <= 0x1083F {
        damn "Cypriot Syllabary"
    } nah bestie codepoint >= 0x1D000 && codepoint <= 0x1D0FF {
        damn "Byzantine Musical Symbols"
    } nah bestie codepoint >= 0x1D100 && codepoint <= 0x1D1FF {
        damn "Musical Symbols"
    } nah bestie codepoint >= 0x1D300 && codepoint <= 0x1D35F {
        damn "Tai Xuan Jing Symbols"
    } nah bestie codepoint >= 0x1D400 && codepoint <= 0x1D7FF {
        damn "Mathematical Alphanumeric Symbols"
    } nah bestie codepoint >= 0x20000 && codepoint <= 0x2A6DF {
        damn "CJK Unified Ideographs Extension B"
    } nah bestie codepoint >= 0x2F800 && codepoint <= 0x2FA1F {
        damn "CJK Compatibility Ideographs Supplement"
    } nah bestie codepoint >= 0xE0000 && codepoint <= 0xE007F {
        damn "Tags"
    } nah bestie codepoint >= 0xE0100 && codepoint <= 0xE01EF {
        damn "Variation Selectors Supplement"
    } nah bestie codepoint >= 0xF0000 && codepoint <= 0xFFFFF {
        damn "Supplementary Private Use Area-A"
    } nah bestie codepoint >= 0x100000 && codepoint <= 0x10FFFF {
        damn "Supplementary Private Use Area-B"
    } nah {
        damn "Unknown"
    }
}
