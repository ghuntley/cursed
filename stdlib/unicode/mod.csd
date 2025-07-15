yeet "testz"

# Comprehensive Unicode Module
# Full CURSED implementation for Unicode text processing

# =====================================
# 1. Unicode Normalization Functions
# =====================================

# Unicode Normalization Form NFC (Canonical Decomposition, followed by Canonical Composition)
slay normalize_nfc(text tea) tea {
    # Decompose characters into their canonical forms
    sus decomposed tea = canonical_decompose(text)
    # Recompose characters using canonical composition
    damn canonical_compose(decomposed)
}

# Unicode Normalization Form NFD (Canonical Decomposition)
slay normalize_nfd(text tea) tea {
    damn canonical_decompose(text)
}

# Unicode Normalization Form NFKC (Compatibility Decomposition, followed by Canonical Composition)
slay normalize_nfkc(text tea) tea {
    sus decomposed tea = compatibility_decompose(text)
    damn canonical_compose(decomposed)
}

# Unicode Normalization Form NFKD (Compatibility Decomposition)
slay normalize_nfkd(text tea) tea {
    damn compatibility_decompose(text)
}

# Canonical decomposition implementation
slay canonical_decompose(text tea) tea {
    sus bytes []normie = string_to_bytes(text)
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus decomposed []normie = get_canonical_decomposition(codepoint)
        
        bestie len(decomposed) > 0 {
            result = append_codepoints(result, decomposed)
        } nah {
            result = append_codepoint(result, codepoint)
        }
        
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn bytes_to_string(result)
}

# Compatibility decomposition implementation  
slay compatibility_decompose(text tea) tea {
    sus bytes []normie = string_to_bytes(text)
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus decomposed []normie = get_compatibility_decomposition(codepoint)
        
        bestie len(decomposed) > 0 {
            result = append_codepoints(result, decomposed)
        } nah {
            result = append_codepoint(result, codepoint)
        }
        
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn bytes_to_string(result)
}

# Canonical composition implementation
slay canonical_compose(text tea) tea {
    sus bytes []normie = string_to_bytes(text)
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus next_pos normie = pos + utf8_sequence_length(bytes[pos])
        
        # Look ahead for combining characters
        bestie next_pos < len(bytes) {
            sus next_codepoint normie = utf8_to_codepoint(bytes, next_pos)
            sus composed normie = get_canonical_composition(codepoint, next_codepoint)
            
            bestie composed != -1 {
                result = append_codepoint(result, composed)
                pos = next_pos + utf8_sequence_length(bytes[next_pos])
                simp
            }
        }
        
        result = append_codepoint(result, codepoint)
        pos = next_pos
    }
    
    damn bytes_to_string(result)
}

# =====================================
# 2. Character Classification Functions
# =====================================

# Unicode General Category classification
slay get_general_category(codepoint normie) tea {
    bestie codepoint >= 0x0041 && codepoint <= 0x005A {
        damn "Lu"  # Letter, uppercase
    } nah bestie codepoint >= 0x0061 && codepoint <= 0x007A {
        damn "Ll"  # Letter, lowercase
    } nah bestie codepoint >= 0x0030 && codepoint <= 0x0039 {
        damn "Nd"  # Number, decimal digit
    } nah bestie codepoint == 0x0020 {
        damn "Zs"  # Separator, space
    } nah bestie codepoint >= 0x0021 && codepoint <= 0x002F {
        damn "Po"  # Punctuation, other
    } nah bestie codepoint >= 0x003A && codepoint <= 0x0040 {
        damn "Po"  # Punctuation, other
    } nah bestie codepoint >= 0x005B && codepoint <= 0x0060 {
        damn "Po"  # Punctuation, other
    } nah bestie codepoint >= 0x007B && codepoint <= 0x007E {
        damn "Po"  # Punctuation, other
    } nah bestie is_unicode_letter(codepoint) {
        damn "Lo"  # Letter, other
    } nah bestie is_unicode_digit(codepoint) {
        damn "Nd"  # Number, decimal digit
    } nah bestie is_unicode_whitespace(codepoint) {
        damn "Zs"  # Separator, space
    } nah {
        damn "Cn"  # Other, not assigned
    }
}

# Check if character is a letter
slay is_unicode_letter(codepoint normie) lit {
    damn (codepoint >= 0x41 && codepoint <= 0x5A) ||      # ASCII A-Z
         (codepoint >= 0x61 && codepoint <= 0x7A) ||      # ASCII a-z
         (codepoint >= 0x00C0 && codepoint <= 0x00D6) ||  # Latin-1 Supplement
         (codepoint >= 0x00D8 && codepoint <= 0x00F6) ||  # Latin-1 Supplement
         (codepoint >= 0x00F8 && codepoint <= 0x00FF) ||  # Latin-1 Supplement
         (codepoint >= 0x0100 && codepoint <= 0x017F) ||  # Latin Extended-A
         (codepoint >= 0x0180 && codepoint <= 0x024F) ||  # Latin Extended-B
         (codepoint >= 0x0370 && codepoint <= 0x03FF) ||  # Greek and Coptic
         (codepoint >= 0x0400 && codepoint <= 0x04FF) ||  # Cyrillic
         (codepoint >= 0x0590 && codepoint <= 0x05FF) ||  # Hebrew
         (codepoint >= 0x0600 && codepoint <= 0x06FF) ||  # Arabic
         (codepoint >= 0x0900 && codepoint <= 0x097F) ||  # Devanagari
         (codepoint >= 0x4E00 && codepoint <= 0x9FFF) ||  # CJK Unified Ideographs
         (codepoint >= 0xAC00 && codepoint <= 0xD7AF)     # Hangul Syllables
}

# Check if character is a digit
slay is_unicode_digit(codepoint normie) lit {
    damn (codepoint >= 0x30 && codepoint <= 0x39) ||      # ASCII 0-9
         (codepoint >= 0x0660 && codepoint <= 0x0669) ||  # Arabic-Indic
         (codepoint >= 0x06F0 && codepoint <= 0x06F9) ||  # Extended Arabic-Indic
         (codepoint >= 0x0966 && codepoint <= 0x096F) ||  # Devanagari
         (codepoint >= 0x09E6 && codepoint <= 0x09EF) ||  # Bengali
         (codepoint >= 0x0A66 && codepoint <= 0x0A6F) ||  # Gurmukhi
         (codepoint >= 0x0AE6 && codepoint <= 0x0AEF) ||  # Gujarati
         (codepoint >= 0x0B66 && codepoint <= 0x0B6F) ||  # Oriya
         (codepoint >= 0x0BE6 && codepoint <= 0x0BEF) ||  # Tamil
         (codepoint >= 0x0C66 && codepoint <= 0x0C6F) ||  # Telugu
         (codepoint >= 0x0CE6 && codepoint <= 0x0CEF) ||  # Kannada
         (codepoint >= 0x0D66 && codepoint <= 0x0D6F) ||  # Malayalam
         (codepoint >= 0x0E50 && codepoint <= 0x0E59) ||  # Thai
         (codepoint >= 0x0ED0 && codepoint <= 0x0ED9) ||  # Lao
         (codepoint >= 0x0F20 && codepoint <= 0x0F29) ||  # Tibetan
         (codepoint >= 0x1040 && codepoint <= 0x1049) ||  # Myanmar
         (codepoint >= 0x17E0 && codepoint <= 0x17E9) ||  # Khmer
         (codepoint >= 0x1810 && codepoint <= 0x1819) ||  # Mongolian
         (codepoint >= 0xFF10 && codepoint <= 0xFF19)     # Fullwidth
}

# Check if character is whitespace
slay is_unicode_whitespace(codepoint normie) lit {
    damn codepoint == 0x0009 ||  # Tab
         codepoint == 0x000A ||  # Line Feed
         codepoint == 0x000B ||  # Vertical Tab
         codepoint == 0x000C ||  # Form Feed
         codepoint == 0x000D ||  # Carriage Return
         codepoint == 0x0020 ||  # Space
         codepoint == 0x0085 ||  # Next Line
         codepoint == 0x00A0 ||  # No-Break Space
         codepoint == 0x1680 ||  # Ogham Space Mark
         codepoint == 0x180E ||  # Mongolian Vowel Separator
         (codepoint >= 0x2000 && codepoint <= 0x200B) ||  # Various spaces
         codepoint == 0x2028 ||  # Line Separator
         codepoint == 0x2029 ||  # Paragraph Separator
         codepoint == 0x202F ||  # Narrow No-Break Space
         codepoint == 0x205F ||  # Medium Mathematical Space
         codepoint == 0x3000 ||  # Ideographic Space
         codepoint == 0xFEFF     # Zero Width No-Break Space
}

# Check if character is punctuation
slay is_unicode_punctuation(codepoint normie) lit {
    damn (codepoint >= 0x0021 && codepoint <= 0x002F) ||  # ASCII punctuation
         (codepoint >= 0x003A && codepoint <= 0x0040) ||  # ASCII punctuation
         (codepoint >= 0x005B && codepoint <= 0x0060) ||  # ASCII punctuation
         (codepoint >= 0x007B && codepoint <= 0x007E) ||  # ASCII punctuation
         (codepoint >= 0x00A1 && codepoint <= 0x00BF) ||  # Latin-1 punctuation
         (codepoint >= 0x2000 && codepoint <= 0x206F) ||  # General Punctuation
         (codepoint >= 0x2E00 && codepoint <= 0x2E7F) ||  # Supplemental Punctuation
         (codepoint >= 0x3000 && codepoint <= 0x303F) ||  # CJK Symbols and Punctuation
         (codepoint >= 0xFE10 && codepoint <= 0xFE19) ||  # Vertical Forms
         (codepoint >= 0xFE30 && codepoint <= 0xFE4F) ||  # CJK Compatibility Forms
         (codepoint >= 0xFE50 && codepoint <= 0xFE6F) ||  # Small Form Variants
         (codepoint >= 0xFF01 && codepoint <= 0xFF0F) ||  # Fullwidth ASCII punctuation
         (codepoint >= 0xFF1A && codepoint <= 0xFF20) ||  # Fullwidth ASCII punctuation
         (codepoint >= 0xFF3B && codepoint <= 0xFF40) ||  # Fullwidth ASCII punctuation
         (codepoint >= 0xFF5B && codepoint <= 0xFF65)     # Fullwidth ASCII punctuation
}

# Check if character is a symbol
slay is_unicode_symbol(codepoint normie) lit {
    damn (codepoint >= 0x0024 && codepoint <= 0x0024) ||  # Dollar sign
         (codepoint >= 0x002B && codepoint <= 0x002B) ||  # Plus sign
         (codepoint >= 0x003C && codepoint <= 0x003E) ||  # Less than, equals, greater than
         (codepoint >= 0x005E && codepoint <= 0x005E) ||  # Circumflex accent
         (codepoint >= 0x0060 && codepoint <= 0x0060) ||  # Grave accent
         (codepoint >= 0x007C && codepoint <= 0x007C) ||  # Vertical line
         (codepoint >= 0x007E && codepoint <= 0x007E) ||  # Tilde
         (codepoint >= 0x00A2 && codepoint <= 0x00A6) ||  # Currency and symbols
         (codepoint >= 0x00A8 && codepoint <= 0x00A9) ||  # Diaeresis, copyright
         (codepoint >= 0x00AC && codepoint <= 0x00AC) ||  # Not sign
         (codepoint >= 0x00AE && codepoint <= 0x00B1) ||  # Registered, plus-minus
         (codepoint >= 0x00B4 && codepoint <= 0x00B4) ||  # Acute accent
         (codepoint >= 0x00B6 && codepoint <= 0x00B7) ||  # Pilcrow, middle dot
         (codepoint >= 0x00BB && codepoint <= 0x00BB) ||  # Right-pointing guillemet
         (codepoint >= 0x00D7 && codepoint <= 0x00D7) ||  # Multiplication sign
         (codepoint >= 0x00F7 && codepoint <= 0x00F7) ||  # Division sign
         (codepoint >= 0x2190 && codepoint <= 0x21FF) ||  # Arrows
         (codepoint >= 0x2200 && codepoint <= 0x22FF) ||  # Mathematical Operators
         (codepoint >= 0x2300 && codepoint <= 0x23FF) ||  # Miscellaneous Technical
         (codepoint >= 0x2400 && codepoint <= 0x243F) ||  # Control Pictures
         (codepoint >= 0x2440 && codepoint <= 0x245F) ||  # Optical Character Recognition
         (codepoint >= 0x2500 && codepoint <= 0x257F) ||  # Box Drawing
         (codepoint >= 0x2580 && codepoint <= 0x259F) ||  # Block Elements
         (codepoint >= 0x25A0 && codepoint <= 0x25FF) ||  # Geometric Shapes
         (codepoint >= 0x2600 && codepoint <= 0x26FF) ||  # Miscellaneous Symbols
         (codepoint >= 0x2700 && codepoint <= 0x27BF) ||  # Dingbats
         (codepoint >= 0x2900 && codepoint <= 0x297F) ||  # Supplemental Arrows-B
         (codepoint >= 0x2980 && codepoint <= 0x29FF) ||  # Miscellaneous Mathematical Symbols-B
         (codepoint >= 0x2A00 && codepoint <= 0x2AFF) ||  # Supplemental Mathematical Operators
         (codepoint >= 0x2B00 && codepoint <= 0x2BFF)     # Miscellaneous Symbols and Arrows
}

# Check if character is a mark (combining character)
slay is_unicode_mark(codepoint normie) lit {
    damn (codepoint >= 0x0300 && codepoint <= 0x036F) ||  # Combining Diacritical Marks
         (codepoint >= 0x0483 && codepoint <= 0x0489) ||  # Cyrillic combining marks
         (codepoint >= 0x0591 && codepoint <= 0x05BD) ||  # Hebrew combining marks
         (codepoint >= 0x05BF && codepoint <= 0x05BF) ||  # Hebrew combining marks
         (codepoint >= 0x05C1 && codepoint <= 0x05C2) ||  # Hebrew combining marks
         (codepoint >= 0x05C4 && codepoint <= 0x05C5) ||  # Hebrew combining marks
         (codepoint >= 0x05C7 && codepoint <= 0x05C7) ||  # Hebrew combining marks
         (codepoint >= 0x0610 && codepoint <= 0x061A) ||  # Arabic combining marks
         (codepoint >= 0x064B && codepoint <= 0x065F) ||  # Arabic combining marks
         (codepoint >= 0x0670 && codepoint <= 0x0670) ||  # Arabic combining marks
         (codepoint >= 0x06D6 && codepoint <= 0x06DC) ||  # Arabic combining marks
         (codepoint >= 0x06DF && codepoint <= 0x06E4) ||  # Arabic combining marks
         (codepoint >= 0x06E7 && codepoint <= 0x06E8) ||  # Arabic combining marks
         (codepoint >= 0x06EA && codepoint <= 0x06ED) ||  # Arabic combining marks
         (codepoint >= 0x0711 && codepoint <= 0x0711) ||  # Syriac combining marks
         (codepoint >= 0x0730 && codepoint <= 0x074A) ||  # Syriac combining marks
         (codepoint >= 0x07A6 && codepoint <= 0x07B0) ||  # Thaana combining marks
         (codepoint >= 0x07EB && codepoint <= 0x07F3) ||  # N'Ko combining marks
         (codepoint >= 0x0816 && codepoint <= 0x0819) ||  # Samaritan combining marks
         (codepoint >= 0x081B && codepoint <= 0x0823) ||  # Samaritan combining marks
         (codepoint >= 0x0825 && codepoint <= 0x0827) ||  # Samaritan combining marks
         (codepoint >= 0x0829 && codepoint <= 0x082D) ||  # Samaritan combining marks
         (codepoint >= 0x0859 && codepoint <= 0x085B) ||  # Mandaic combining marks
         (codepoint >= 0x08D4 && codepoint <= 0x08E1) ||  # Arabic Extended-A combining marks
         (codepoint >= 0x08E3 && codepoint <= 0x0902) ||  # Arabic Extended-A combining marks
         (codepoint >= 0x093A && codepoint <= 0x093A) ||  # Devanagari combining marks
         (codepoint >= 0x093C && codepoint <= 0x093C) ||  # Devanagari combining marks
         (codepoint >= 0x0941 && codepoint <= 0x0948) ||  # Devanagari combining marks
         (codepoint >= 0x094D && codepoint <= 0x094D) ||  # Devanagari combining marks
         (codepoint >= 0x0951 && codepoint <= 0x0957) ||  # Devanagari combining marks
         (codepoint >= 0x0962 && codepoint <= 0x0963) ||  # Devanagari combining marks
         (codepoint >= 0x20D0 && codepoint <= 0x20F0) ||  # Combining Diacritical Marks for Symbols
         (codepoint >= 0xFE20 && codepoint <= 0xFE2F) ||  # Combining Half Marks
         (codepoint >= 0x1AB0 && codepoint <= 0x1ABE) ||  # Combining Diacritical Marks Extended
         (codepoint >= 0x1DC0 && codepoint <= 0x1DFF) ||  # Combining Diacritical Marks Supplement
         (codepoint >= 0xA66F && codepoint <= 0xA672) ||  # Cyrillic Extended-B combining marks
         (codepoint >= 0xA674 && codepoint <= 0xA67D) ||  # Cyrillic Extended-B combining marks
         (codepoint >= 0xA69E && codepoint <= 0xA69F)     # Cyrillic Extended-B combining marks
}

# =====================================
# 3. Case Conversion Functions
# =====================================

# Convert codepoint to uppercase
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
    
    # Latin Extended-A lowercase to uppercase
    bestie codepoint >= 0x0101 && codepoint <= 0x017F {
        bestie (codepoint & 1) == 1 {  # Odd codepoints are lowercase
            damn codepoint - 1
        }
    }
    
    # Greek lowercase to uppercase
    bestie codepoint >= 0x03B1 && codepoint <= 0x03C9 {
        damn codepoint - 0x20
    }
    
    # Cyrillic lowercase to uppercase
    bestie codepoint >= 0x0430 && codepoint <= 0x044F {
        damn codepoint - 0x20
    }
    
    # Return unchanged if no conversion
    damn codepoint
}

# Convert codepoint to lowercase
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
    
    # Latin Extended-A uppercase to lowercase
    bestie codepoint >= 0x0100 && codepoint <= 0x017E {
        bestie (codepoint & 1) == 0 {  # Even codepoints are uppercase
            damn codepoint + 1
        }
    }
    
    # Greek uppercase to lowercase
    bestie codepoint >= 0x0391 && codepoint <= 0x03A9 {
        damn codepoint + 0x20
    }
    
    # Cyrillic uppercase to lowercase
    bestie codepoint >= 0x0410 && codepoint <= 0x042F {
        damn codepoint + 0x20
    }
    
    # Return unchanged if no conversion
    damn codepoint
}

# Convert codepoint to title case
slay to_unicode_title(codepoint normie) normie {
    # For most characters, title case is the same as uppercase
    damn to_unicode_upper(codepoint)
}

# Convert string to uppercase
slay string_to_unicode_upper(text tea) tea {
    sus bytes []normie = string_to_bytes(text)
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus upper_codepoint normie = to_unicode_upper(codepoint)
        sus utf8_bytes []normie = codepoint_to_utf8(upper_codepoint)
        
        result = append_bytes(result, utf8_bytes)
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn bytes_to_string(result)
}

# Convert string to lowercase
slay string_to_unicode_lower(text tea) tea {
    sus bytes []normie = string_to_bytes(text)
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus lower_codepoint normie = to_unicode_lower(codepoint)
        sus utf8_bytes []normie = codepoint_to_utf8(lower_codepoint)
        
        result = append_bytes(result, utf8_bytes)
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn bytes_to_string(result)
}

# Convert string to title case
slay string_to_unicode_title(text tea) tea {
    sus bytes []normie = string_to_bytes(text)
    sus result []normie = []
    sus pos normie = 0
    sus is_word_start lit = based
    
    bestie pos < len(bytes) {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus converted_codepoint normie = codepoint
        
        bestie is_word_start && is_unicode_letter(codepoint) {
            converted_codepoint = to_unicode_title(codepoint)
            is_word_start = cap
        } nah bestie is_unicode_letter(codepoint) {
            converted_codepoint = to_unicode_lower(codepoint)
        } nah bestie is_unicode_whitespace(codepoint) || is_unicode_punctuation(codepoint) {
            is_word_start = based
        }
        
        sus utf8_bytes []normie = codepoint_to_utf8(converted_codepoint)
        result = append_bytes(result, utf8_bytes)
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn bytes_to_string(result)
}

# =====================================
# 4. String Comparison Functions
# =====================================

# Case-insensitive string comparison
slay unicode_compare_ignore_case(text1 tea, text2 tea) normie {
    sus lower1 tea = string_to_unicode_lower(text1)
    sus lower2 tea = string_to_unicode_lower(text2)
    damn string_compare(lower1, lower2)
}

# Unicode-aware string comparison with collation
slay unicode_collate_compare(text1 tea, text2 tea) normie {
    sus norm1 tea = normalize_nfc(text1)
    sus norm2 tea = normalize_nfc(text2)
    damn string_compare(norm1, norm2)
}

# Check if strings are equal ignoring case
slay unicode_equal_ignore_case(text1 tea, text2 tea) lit {
    damn unicode_compare_ignore_case(text1, text2) == 0
}

# Check if strings are equal with Unicode normalization
slay unicode_equal_normalized(text1 tea, text2 tea) lit {
    damn unicode_collate_compare(text1, text2) == 0
}

# =====================================
# 5. Encoding/Decoding Utilities
# =====================================

# UTF-8 validation
slay validate_utf8_string(text tea) lit {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    
    bestie pos < len {
        bestie !validate_utf8_sequence(bytes, pos) {
            damn cap
        }
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn based
}

# UTF-16 encoding
slay encode_utf16(text tea) []normie {
    sus bytes []normie = string_to_bytes(text)
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus utf16_units []normie = codepoint_to_utf16(codepoint)
        result = append_bytes(result, utf16_units)
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn result
}

# UTF-16 decoding
slay decode_utf16(bytes []normie) tea {
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) - 1 {
        sus high normie = (bytes[pos] << 8) | bytes[pos + 1]
        sus codepoint normie = high
        pos += 2
        
        # Handle surrogate pairs
        bestie high >= 0xD800 && high <= 0xDBFF {
            bestie pos < len(bytes) - 1 {
                sus low normie = (bytes[pos] << 8) | bytes[pos + 1]
                bestie low >= 0xDC00 && low <= 0xDFFF {
                    codepoint = 0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00)
                    pos += 2
                }
            }
        }
        
        sus utf8_bytes []normie = codepoint_to_utf8(codepoint)
        result = append_bytes(result, utf8_bytes)
    }
    
    damn bytes_to_string(result)
}

# UTF-32 encoding
slay encode_utf32(text tea) []normie {
    sus bytes []normie = string_to_bytes(text)
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus utf32_bytes []normie = codepoint_to_utf32(codepoint)
        result = append_bytes(result, utf32_bytes)
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn result
}

# UTF-32 decoding
slay decode_utf32(bytes []normie) tea {
    sus result []normie = []
    sus pos normie = 0
    
    bestie pos < len(bytes) - 3 {
        sus codepoint normie = (bytes[pos] << 24) | (bytes[pos + 1] << 16) | (bytes[pos + 2] << 8) | bytes[pos + 3]
        sus utf8_bytes []normie = codepoint_to_utf8(codepoint)
        result = append_bytes(result, utf8_bytes)
        pos += 4
    }
    
    damn bytes_to_string(result)
}

# =====================================
# 6. Grapheme Cluster Handling
# =====================================

# Check if codepoint is a grapheme cluster boundary
slay is_grapheme_boundary(prev_codepoint normie, curr_codepoint normie) lit {
    # Simplified grapheme cluster boundary detection
    # Real implementation would follow Unicode TR29
    
    # Control characters always break
    bestie curr_codepoint < 0x20 || (curr_codepoint >= 0x7F && curr_codepoint < 0xA0) {
        damn based
    }
    
    # Combining marks don't break
    bestie is_unicode_mark(curr_codepoint) {
        damn cap
    }
    
    # Different scripts break (simplified)
    bestie get_script(prev_codepoint) != get_script(curr_codepoint) {
        damn based
    }
    
    damn based
}

# Get script of codepoint
slay get_script(codepoint normie) tea {
    bestie codepoint >= 0x0000 && codepoint <= 0x007F {
        damn "Latin"
    } nah bestie codepoint >= 0x0370 && codepoint <= 0x03FF {
        damn "Greek"
    } nah bestie codepoint >= 0x0400 && codepoint <= 0x04FF {
        damn "Cyrillic"
    } nah bestie codepoint >= 0x0590 && codepoint <= 0x05FF {
        damn "Hebrew"
    } nah bestie codepoint >= 0x0600 && codepoint <= 0x06FF {
        damn "Arabic"
    } nah bestie codepoint >= 0x0900 && codepoint <= 0x097F {
        damn "Devanagari"
    } nah bestie codepoint >= 0x4E00 && codepoint <= 0x9FFF {
        damn "Han"
    } nah bestie codepoint >= 0x3040 && codepoint <= 0x309F {
        damn "Hiragana"
    } nah bestie codepoint >= 0x30A0 && codepoint <= 0x30FF {
        damn "Katakana"
    } nah bestie codepoint >= 0xAC00 && codepoint <= 0xD7AF {
        damn "Hangul"
    } nah {
        damn "Common"
    }
}

# Count grapheme clusters in string
slay count_grapheme_clusters(text tea) normie {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus count normie = 0
    sus prev_codepoint normie = -1
    
    bestie pos < len {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        
        bestie prev_codepoint == -1 || is_grapheme_boundary(prev_codepoint, codepoint) {
            count++
        }
        
        prev_codepoint = codepoint
        pos += utf8_sequence_length(bytes[pos])
    }
    
    damn count
}

# Get grapheme cluster at position
slay get_grapheme_cluster_at(text tea, cluster_pos normie) tea {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus current_cluster normie = 0
    sus cluster_start normie = 0
    sus prev_codepoint normie = -1
    
    bestie pos < len {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        
        bestie prev_codepoint == -1 || is_grapheme_boundary(prev_codepoint, codepoint) {
            bestie current_cluster == cluster_pos {
                cluster_start = pos
            } nah bestie current_cluster == cluster_pos + 1 {
                # Found the end of the cluster
                sus cluster_bytes []normie = slice_bytes(bytes, cluster_start, pos)
                damn bytes_to_string(cluster_bytes)
            }
            current_cluster++
        }
        
        prev_codepoint = codepoint
        pos += utf8_sequence_length(bytes[pos])
    }
    
    # If we're at the last cluster
    bestie current_cluster == cluster_pos {
        sus cluster_bytes []normie = slice_bytes(bytes, cluster_start, len)
        damn bytes_to_string(cluster_bytes)
    }
    
    damn ""
}

# =====================================
# 7. Text Segmentation Functions
# =====================================

# Word segmentation (simplified)
slay segment_words(text tea) []tea {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus words []tea = []
    sus word_start normie = 0
    sus in_word lit = cap
    
    bestie pos < len {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        sus is_word_char lit = is_unicode_letter(codepoint) || is_unicode_digit(codepoint)
        
        bestie !in_word && is_word_char {
            # Start of word
            word_start = pos
            in_word = based
        } nah bestie in_word && !is_word_char {
            # End of word
            sus word_bytes []normie = slice_bytes(bytes, word_start, pos)
            sus word tea = bytes_to_string(word_bytes)
            words = append_string(words, word)
            in_word = cap
        }
        
        pos += utf8_sequence_length(bytes[pos])
    }
    
    # Handle word at end of string
    bestie in_word {
        sus word_bytes []normie = slice_bytes(bytes, word_start, len)
        sus word tea = bytes_to_string(word_bytes)
        words = append_string(words, word)
    }
    
    damn words
}

# Sentence segmentation (simplified)
slay segment_sentences(text tea) []tea {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus sentences []tea = []
    sus sentence_start normie = 0
    
    bestie pos < len {
        sus codepoint normie = utf8_to_codepoint(bytes, pos)
        
        # Simple sentence boundary detection
        bestie codepoint == 0x002E || codepoint == 0x0021 || codepoint == 0x003F {  # . ! ?
            # Look ahead to see if followed by whitespace or end of string
            sus next_pos normie = pos + utf8_sequence_length(bytes[pos])
            sus is_sentence_end lit = based
            
            bestie next_pos < len {
                sus next_codepoint normie = utf8_to_codepoint(bytes, next_pos)
                bestie !is_unicode_whitespace(next_codepoint) {
                    is_sentence_end = cap
                }
            }
            
            bestie is_sentence_end {
                sus sentence_bytes []normie = slice_bytes(bytes, sentence_start, next_pos)
                sus sentence tea = bytes_to_string(sentence_bytes)
                sentences = append_string(sentences, sentence)
                sentence_start = next_pos
                pos = next_pos
                simp
            }
        }
        
        pos += utf8_sequence_length(bytes[pos])
    }
    
    # Handle remaining text
    bestie sentence_start < len {
        sus sentence_bytes []normie = slice_bytes(bytes, sentence_start, len)
        sus sentence tea = bytes_to_string(sentence_bytes)
        sentences = append_string(sentences, sentence)
    }
    
    damn sentences
}

# Line breaking (simplified)
slay segment_lines(text tea, max_width normie) []tea {
    sus words []tea = segment_words(text)
    sus lines []tea = []
    sus current_line tea = ""
    sus current_width normie = 0
    sus i normie = 0
    
    bestie i < len(words) {
        sus word tea = words[i]
        sus word_width normie = utf8_char_count(word)
        
        bestie current_width == 0 {
            # First word on line
            current_line = word
            current_width = word_width
        } nah bestie current_width + 1 + word_width <= max_width {
            # Add word to current line
            current_line = current_line + " " + word
            current_width += 1 + word_width
        } nah {
            # Start new line
            lines = append_string(lines, current_line)
            current_line = word
            current_width = word_width
        }
        
        i++
    }
    
    # Add final line
    bestie current_line != "" {
        lines = append_string(lines, current_line)
    }
    
    damn lines
}

# =====================================
# Core UTF-8 and Helper Functions
# =====================================

# UTF-8 byte sequence validation
slay validate_utf8_sequence(bytes []normie, start_pos normie) lit {
    sus len normie = len(bytes)
    sus pos normie = start_pos
    
    bestie pos >= len {
        damn cap
    }
    
    sus first_byte normie = bytes[pos]
    sus expected_length normie = utf8_sequence_length(first_byte)
    
    bestie expected_length == 0 || pos + expected_length > len {
        damn cap
    }
    
    sus i normie = 1
    bestie i < expected_length {
        sus byte_val normie = bytes[pos + i]
        bestie (byte_val & 0xC0) != 0x80 {
            damn cap
        }
        i++
    }
    
    damn based
}

# Get expected byte count for UTF-8 sequence
slay utf8_sequence_length(first_byte normie) normie {
    bestie first_byte >= 0 && first_byte <= 127 {
        damn 1  # ASCII
    } nah bestie (first_byte & 0xE0) == 0xC0 {
        damn 2  # 2-byte sequence
    } nah bestie (first_byte & 0xF0) == 0xE0 {
        damn 3  # 3-byte sequence
    } nah bestie (first_byte & 0xF8) == 0xF0 {
        damn 4  # 4-byte sequence
    } nah {
        damn 0  # Invalid
    }
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
        result = append(result, codepoint)
    } nah bestie codepoint <= 0x7FF {
        result = append(result, 0xC0 | (codepoint >> 6))
        result = append(result, 0x80 | (codepoint & 0x3F))
    } nah bestie codepoint <= 0xFFFF {
        result = append(result, 0xE0 | (codepoint >> 12))
        result = append(result, 0x80 | ((codepoint >> 6) & 0x3F))
        result = append(result, 0x80 | (codepoint & 0x3F))
    } nah bestie codepoint <= 0x10FFFF {
        result = append(result, 0xF0 | (codepoint >> 18))
        result = append(result, 0x80 | ((codepoint >> 12) & 0x3F))
        result = append(result, 0x80 | ((codepoint >> 6) & 0x3F))
        result = append(result, 0x80 | (codepoint & 0x3F))
    }
    
    damn result
}

# Convert Unicode code point to UTF-16 units
slay codepoint_to_utf16(codepoint normie) []normie {
    sus result []normie = []
    
    bestie codepoint <= 0xFFFF {
        # Single UTF-16 unit
        result = append(result, (codepoint >> 8) & 0xFF)
        result = append(result, codepoint & 0xFF)
    } nah bestie codepoint <= 0x10FFFF {
        # Surrogate pair
        sus adjusted normie = codepoint - 0x10000
        sus high normie = 0xD800 | (adjusted >> 10)
        sus low normie = 0xDC00 | (adjusted & 0x3FF)
        
        result = append(result, (high >> 8) & 0xFF)
        result = append(result, high & 0xFF)
        result = append(result, (low >> 8) & 0xFF)
        result = append(result, low & 0xFF)
    }
    
    damn result
}

# Convert Unicode code point to UTF-32 bytes
slay codepoint_to_utf32(codepoint normie) []normie {
    sus result []normie = []
    result = append(result, (codepoint >> 24) & 0xFF)
    result = append(result, (codepoint >> 16) & 0xFF)
    result = append(result, (codepoint >> 8) & 0xFF)
    result = append(result, codepoint & 0xFF)
    damn result
}

# Count UTF-8 characters in string
slay utf8_char_count(text tea) normie {
    sus bytes []normie = string_to_bytes(text)
    sus len normie = len(bytes)
    sus pos normie = 0
    sus count normie = 0
    
    bestie pos < len {
        sus seq_len normie = utf8_sequence_length(bytes[pos])
        bestie seq_len == 0 {
            damn -1
        }
        pos += seq_len
        count++
    }
    
    damn count
}

# Get canonical decomposition for codepoint
slay get_canonical_decomposition(codepoint normie) []normie {
    # Simplified decomposition table - real implementation would be much larger
    sus result []normie = []
    
    bestie codepoint == 0x00C0 {  # À
        result = append(result, 0x0041)  # A
        result = append(result, 0x0300)  # grave accent
    } nah bestie codepoint == 0x00C1 {  # Á
        result = append(result, 0x0041)  # A
        result = append(result, 0x0301)  # acute accent
    } nah bestie codepoint == 0x00C2 {  # Â
        result = append(result, 0x0041)  # A
        result = append(result, 0x0302)  # circumflex accent
    } nah bestie codepoint == 0x00C3 {  # Ã
        result = append(result, 0x0041)  # A
        result = append(result, 0x0303)  # tilde
    } nah bestie codepoint == 0x00C4 {  # Ä
        result = append(result, 0x0041)  # A
        result = append(result, 0x0308)  # diaeresis
    } nah bestie codepoint == 0x00C5 {  # Å
        result = append(result, 0x0041)  # A
        result = append(result, 0x030A)  # ring above
    }
    
    damn result
}

# Get compatibility decomposition for codepoint
slay get_compatibility_decomposition(codepoint normie) []normie {
    # Simplified compatibility decomposition table
    sus result []normie = []
    
    bestie codepoint == 0x00A0 {  # No-break space
        result = append(result, 0x0020)  # Space
    } nah bestie codepoint == 0x2126 {  # Ohm sign
        result = append(result, 0x03A9)  # Greek capital letter omega
    } nah bestie codepoint == 0x212A {  # Kelvin sign
        result = append(result, 0x004B)  # Latin capital letter K
    } nah bestie codepoint == 0x212B {  # Angstrom sign
        result = append(result, 0x00C5)  # Latin capital letter A with ring above
    }
    
    damn result
}

# Get canonical composition for two codepoints
slay get_canonical_composition(base normie, combining normie) normie {
    # Simplified composition table
    bestie base == 0x0041 {  # A
        bestie combining == 0x0300 {  # grave accent
            damn 0x00C0  # À
        } nah bestie combining == 0x0301 {  # acute accent
            damn 0x00C1  # Á
        } nah bestie combining == 0x0302 {  # circumflex accent
            damn 0x00C2  # Â
        } nah bestie combining == 0x0303 {  # tilde
            damn 0x00C3  # Ã
        } nah bestie combining == 0x0308 {  # diaeresis
            damn 0x00C4  # Ä
        } nah bestie combining == 0x030A {  # ring above
            damn 0x00C5  # Å
        }
    }
    
    damn -1  # No composition
}

# =====================================
# Helper Functions
# =====================================

# Convert string to byte array (built-in function placeholder)
slay string_to_bytes(text tea) []normie {
    # This would be implemented as a built-in function
    sus result []normie = []
    damn result
}

# Convert byte array to string (built-in function placeholder)
slay bytes_to_string(bytes []normie) tea {
    # This would be implemented as a built-in function
    damn ""
}

# Compare two strings
slay string_compare(text1 tea, text2 tea) normie {
    # This would be implemented as a built-in function
    damn 0
}

# Slice byte array
slay slice_bytes(bytes []normie, start normie, end normie) []normie {
    sus result []normie = []
    sus i normie = start
    
    bestie i < end && i < len(bytes) {
        result = append(result, bytes[i])
        i++
    }
    
    damn result
}

# Append byte arrays
slay append_bytes(dest []normie, src []normie) []normie {
    sus result []normie = dest
    sus i normie = 0
    
    bestie i < len(src) {
        result = append(result, src[i])
        i++
    }
    
    damn result
}

# Append codepoint to byte array
slay append_codepoint(dest []normie, codepoint normie) []normie {
    sus utf8_bytes []normie = codepoint_to_utf8(codepoint)
    damn append_bytes(dest, utf8_bytes)
}

# Append multiple codepoints to byte array
slay append_codepoints(dest []normie, codepoints []normie) []normie {
    sus result []normie = dest
    sus i normie = 0
    
    bestie i < len(codepoints) {
        result = append_codepoint(result, codepoints[i])
        i++
    }
    
    damn result
}

# Append string to string array
slay append_string(dest []tea, str tea) []tea {
    sus result []tea = dest
    result = append(result, str)
    damn result
}
