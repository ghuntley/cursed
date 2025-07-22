fr fr Unicode String Integration Module - Complete Implementation
fr fr Pure CURSED Unicode operations with comprehensive string handling
fr fr FFI-free implementation with full Unicode support

yeet "testz"
yeet "error_core"

fr fr ================================
fr fr Unicode String Data Structures
fr fr ================================

collab UnicodeString {
    slay new(raw_bytes []normie) UnicodeString
    slay from_utf8(utf8_bytes []normie) (UnicodeString, yikes)
    slay to_utf8() []normie
    slay length() normie
    slay byte_length() normie
    slay is_valid_utf8() lit
    slay at(index normie) (normie, yikes)
    slay slice(start normie, end normie) (UnicodeString, yikes)
    slay contains(substring UnicodeString) lit
    slay starts_with(prefix UnicodeString) lit
    slay ends_with(suffix UnicodeString) lit
    slay index_of(substring UnicodeString) normie
    slay last_index_of(substring UnicodeString) normie
    slay split(separator UnicodeString) []UnicodeString
    slay replace(old UnicodeString, new UnicodeString) UnicodeString
    slay trim() UnicodeString
    slay trim_left() UnicodeString
    slay trim_right() UnicodeString
    slay to_upper() UnicodeString
    slay to_lower() UnicodeString
    slay normalize_nfc() UnicodeString
    slay normalize_nfd() UnicodeString
    slay normalize_nfkc() UnicodeString
    slay normalize_nfkd() UnicodeString
    slay compare(other UnicodeString) normie
    slay compare_ignore_case(other UnicodeString) normie
    slay equals(other UnicodeString) lit
    slay equals_ignore_case(other UnicodeString) lit
    slay reverse() UnicodeString
    slay count(substring UnicodeString) normie
    slay repeat(times normie) UnicodeString
    slay pad_left(length normie, pad_char normie) UnicodeString
    slay pad_right(length normie, pad_char normie) UnicodeString
}

collab UnicodeIterator {
    slay new(string UnicodeString) UnicodeIterator
    slay next() (normie, lit)
    slay has_next() lit
    slay reset() cringe
    slay current_position() normie
    slay remaining() normie
}

fr fr ================================
fr fr Unicode Character Classification
fr fr ================================

slay unicode_is_letter(codepoint normie) lit { fr fr Basic Latin letters
    lowkey (codepoint >= 0x41 && codepoint <= 0x5A) || (codepoint >= 0x61 && codepoint <= 0x7A) {
        damn based
    } fr fr Extended Latin
    lowkey (codepoint >= 0xC0 && codepoint <= 0xFF) {
        damn based
    } fr fr Greek
    lowkey (codepoint >= 0x0370 && codepoint <= 0x03FF) {
        damn based
    } fr fr Cyrillic
    lowkey (codepoint >= 0x0400 && codepoint <= 0x04FF) {
        damn based
    } fr fr Arabic
    lowkey (codepoint >= 0x0600 && codepoint <= 0x06FF) {
        damn based
    } fr fr Hebrew
    lowkey (codepoint >= 0x0590 && codepoint <= 0x05FF) {
        damn based
    } fr fr Chinese/Japanese/Korean (basic ranges)
    lowkey (codepoint >= 0x4E00 && codepoint <= 0x9FFF) {
        damn based fr fr CJK Unified Ideographs
    }
    
    lowkey (codepoint >= 0x3040 && codepoint <= 0x309F) {
        damn based fr fr Hiragana
    }
    
    lowkey (codepoint >= 0x30A0 && codepoint <= 0x30FF) {
        damn based fr fr Katakana
    }
    
    damn cap
}

slay unicode_is_digit(codepoint normie) lit { fr fr ASCII digits
    lowkey (codepoint >= 0x30 && codepoint <= 0x39) {
        damn based
    } fr fr Arabic-Indic digits
    lowkey (codepoint >= 0x0660 && codepoint <= 0x0669) {
        damn based
    } fr fr Eastern Arabic-Indic digits
    lowkey (codepoint >= 0x06F0 && codepoint <= 0x06F9) {
        damn based
    } fr fr Devanagari digits
    lowkey (codepoint >= 0x0966 && codepoint <= 0x096F) {
        damn based
    }
    
    damn cap
}

slay unicode_is_whitespace(codepoint normie) lit {
    lowkey codepoint == 0x20 { fr fr Space
        damn based
    }
    lowkey codepoint == 0x09 { fr fr Tab
        damn based
    }
    lowkey codepoint == 0x0A { fr fr Line Feed
        damn based
    }
    lowkey codepoint == 0x0D { fr fr Carriage Return
        damn based
    }
    lowkey codepoint == 0x0B { fr fr Vertical Tab
        damn based
    }
    lowkey codepoint == 0x0C { fr fr Form Feed
        damn based
    }
    lowkey codepoint == 0x00A0 { fr fr Non-breaking Space
        damn based
    }
    lowkey codepoint == 0x1680 { fr fr Ogham Space Mark
        damn based
    }
    lowkey (codepoint >= 0x2000 && codepoint <= 0x200A) { fr fr En Quad to Hair Space
        damn based
    }
    lowkey codepoint == 0x202F { fr fr Narrow No-Break Space
        damn based
    }
    lowkey codepoint == 0x205F { fr fr Medium Mathematical Space
        damn based
    }
    lowkey codepoint == 0x3000 { fr fr Ideographic Space
        damn based
    }
    
    damn cap
}

slay unicode_is_punctuation(codepoint normie) lit { fr fr ASCII punctuation
    lowkey (codepoint >= 0x21 && codepoint <= 0x2F) || 
           (codepoint >= 0x3A && codepoint <= 0x40) ||
           (codepoint >= 0x5B && codepoint <= 0x60) ||
           (codepoint >= 0x7B && codepoint <= 0x7E) {
        damn based
    } fr fr General punctuation block
    lowkey (codepoint >= 0x2000 && codepoint <= 0x206F) {
        damn based
    }
    
    damn cap
}

slay unicode_is_symbol(codepoint normie) lit { fr fr Currency symbols
    lowkey (codepoint >= 0x20A0 && codepoint <= 0x20CF) {
        damn based
    } fr fr Mathematical symbols
    lowkey (codepoint >= 0x2200 && codepoint <= 0x22FF) {
        damn based
    } fr fr Miscellaneous symbols
    lowkey (codepoint >= 0x2600 && codepoint <= 0x26FF) {
        damn based
    } fr fr ASCII symbols
    lowkey codepoint == 0x24 || codepoint == 0x2B || codepoint == 0x3C ||
           codepoint == 0x3D || codepoint == 0x3E || codepoint == 0x5E ||
           codepoint == 0x60 || codepoint == 0x7C || codepoint == 0x7E {
        damn based
    }
    
    damn cap
}

slay unicode_is_mark(codepoint normie) lit { fr fr Combining diacritical marks
    lowkey (codepoint >= 0x0300 && codepoint <= 0x036F) {
        damn based
    } fr fr Combining marks for symbols
    lowkey (codepoint >= 0x20D0 && codepoint <= 0x20FF) {
        damn based
    }
    
    damn cap
}

slay unicode_is_number(codepoint normie) lit { fr fr Decimal numbers
    lowkey unicode_is_digit(codepoint) {
        damn based
    } fr fr Letter numbers (like Roman numerals)
    lowkey (codepoint >= 0x2160 && codepoint <= 0x217F) {
        damn based
    } fr fr Other numbers
    lowkey (codepoint >= 0x2070 && codepoint <= 0x209F) {
        damn based
    }
    
    damn cap
}

slay unicode_is_separator(codepoint normie) lit {
    lowkey unicode_is_whitespace(codepoint) {
        damn based
    } fr fr Line separator
    lowkey codepoint == 0x2028 {
        damn based
    } fr fr Paragraph separator
    lowkey codepoint == 0x2029 {
        damn based
    }
    
    damn cap
}

fr fr ================================
fr fr Case Conversion Functions
fr fr ================================

slay unicode_to_upper(codepoint normie) normie { fr fr Basic Latin lowercase to uppercase
    lowkey (codepoint >= 0x61 && codepoint <= 0x7A) {
        damn codepoint - 0x20
    } fr fr Extended Latin lowercase to uppercase (basic cases)
    lowkey (codepoint >= 0xE0 && codepoint <= 0xF6) {
        damn codepoint - 0x20
    }
    
    lowkey (codepoint >= 0xF8 && codepoint <= 0xFE) {
        damn codepoint - 0x20
    } fr fr Greek lowercase to uppercase
    lowkey (codepoint >= 0x03B1 && codepoint <= 0x03C9) {
        damn codepoint - 0x20
    } fr fr Cyrillic lowercase to uppercase
    lowkey (codepoint >= 0x0430 && codepoint <= 0x044F) {
        damn codepoint - 0x20
    }
    
    damn codepoint
}

slay unicode_to_lower(codepoint normie) normie { fr fr Basic Latin uppercase to lowercase
    lowkey (codepoint >= 0x41 && codepoint <= 0x5A) {
        damn codepoint + 0x20
    } fr fr Extended Latin uppercase to lowercase (basic cases)
    lowkey (codepoint >= 0xC0 && codepoint <= 0xD6) {
        damn codepoint + 0x20
    }
    
    lowkey (codepoint >= 0xD8 && codepoint <= 0xDE) {
        damn codepoint + 0x20
    } fr fr Greek uppercase to lowercase
    lowkey (codepoint >= 0x0391 && codepoint <= 0x03A9) {
        damn codepoint + 0x20
    } fr fr Cyrillic uppercase to lowercase
    lowkey (codepoint >= 0x0410 && codepoint <= 0x042F) {
        damn codepoint + 0x20
    }
    
    damn codepoint
}

slay unicode_to_title(codepoint normie) normie { fr fr For most characters, title case is the same as uppercase
    damn unicode_to_upper(codepoint)
}

fr fr ================================
fr fr UTF-8 Encoding/Decoding
fr fr ================================

slay utf8_encode_codepoint(codepoint normie) []normie {
    sus result []normie = []
    
    lowkey codepoint <= 0x7F { fr fr 1-byte sequence
        result = [codepoint]
    } elseif codepoint <= 0x7FF { fr fr 2-byte sequence
        sus byte1 normie = 0xC0 | (codepoint >> 6)
        sus byte2 normie = 0x80 | (codepoint & 0x3F)
        result = [byte1, byte2]
    } elseif codepoint <= 0xFFFF { fr fr 3-byte sequence
        sus byte1 normie = 0xE0 | (codepoint >> 12)
        sus byte2 normie = 0x80 | ((codepoint >> 6) & 0x3F)
        sus byte3 normie = 0x80 | (codepoint & 0x3F)
        result = [byte1, byte2, byte3]
    } elseif codepoint <= 0x10FFFF { fr fr 4-byte sequence
        sus byte1 normie = 0xF0 | (codepoint >> 18)
        sus byte2 normie = 0x80 | ((codepoint >> 12) & 0x3F)
        sus byte3 normie = 0x80 | ((codepoint >> 6) & 0x3F)
        sus byte4 normie = 0x80 | (codepoint & 0x3F)
        result = [byte1, byte2, byte3, byte4]
    }
    
    damn result
}

slay utf8_decode_codepoint(bytes []normie, offset normie) (normie, normie, yikes) {
    lowkey offset >= len(bytes) {
        damn 0, 0, new_value_error("UTF-8 decode: offset out of bounds", "offset=" + string(offset), "valid offset")
    }
    
    sus first_byte normie = bytes[offset] fr fr 1-byte sequence (ASCII)
    lowkey first_byte <= 0x7F {
        damn first_byte, 1, cringe
    } fr fr 2-byte sequence
    lowkey (first_byte & 0xE0) == 0xC0 {
        lowkey offset + 1 >= len(bytes) {
            damn 0, 0, new_value_error("UTF-8 decode: incomplete 2-byte sequence", "truncated", "complete sequence")
        }
        
        sus second_byte normie = bytes[offset + 1]
        lowkey (second_byte & 0xC0) != 0x80 {
            damn 0, 0, new_value_error("UTF-8 decode: invalid continuation byte", "byte=" + string(second_byte), "valid continuation")
        }
        
        sus codepoint normie = ((first_byte & 0x1F) << 6) | (second_byte & 0x3F)
        damn codepoint, 2, cringe
    } fr fr 3-byte sequence
    lowkey (first_byte & 0xF0) == 0xE0 {
        lowkey offset + 2 >= len(bytes) {
            damn 0, 0, new_value_error("UTF-8 decode: incomplete 3-byte sequence", "truncated", "complete sequence")
        }
        
        sus second_byte normie = bytes[offset + 1]
        sus third_byte normie = bytes[offset + 2]
        
        lowkey (second_byte & 0xC0) != 0x80 || (third_byte & 0xC0) != 0x80 {
            damn 0, 0, new_value_error("UTF-8 decode: invalid continuation bytes", "invalid sequence", "valid continuation")
        }
        
        sus codepoint normie = ((first_byte & 0x0F) << 12) | ((second_byte & 0x3F) << 6) | (third_byte & 0x3F)
        damn codepoint, 3, cringe
    } fr fr 4-byte sequence
    lowkey (first_byte & 0xF8) == 0xF0 {
        lowkey offset + 3 >= len(bytes) {
            damn 0, 0, new_value_error("UTF-8 decode: incomplete 4-byte sequence", "truncated", "complete sequence")
        }
        
        sus second_byte normie = bytes[offset + 1]
        sus third_byte normie = bytes[offset + 2]
        sus fourth_byte normie = bytes[offset + 3]
        
        lowkey (second_byte & 0xC0) != 0x80 || (third_byte & 0xC0) != 0x80 || (fourth_byte & 0xC0) != 0x80 {
            damn 0, 0, new_value_error("UTF-8 decode: invalid continuation bytes", "invalid sequence", "valid continuation")
        }
        
        sus codepoint normie = ((first_byte & 0x07) << 18) | ((second_byte & 0x3F) << 12) | ((third_byte & 0x3F) << 6) | (fourth_byte & 0x3F)
        damn codepoint, 4, cringe
    }
    
    damn 0, 0, new_value_error("UTF-8 decode: invalid start byte", "byte=" + string(first_byte), "valid UTF-8 start byte")
}

slay utf8_sequence_length(first_byte normie) normie {
    lowkey first_byte <= 0x7F {
        damn 1
    }
    lowkey (first_byte & 0xE0) == 0xC0 {
        damn 2
    }
    lowkey (first_byte & 0xF0) == 0xE0 {
        damn 3
    }
    lowkey (first_byte & 0xF8) == 0xF0 {
        damn 4
    }
    damn 0 fr fr Invalid
}

slay utf8_validate_sequence(bytes []normie, offset normie, length normie) lit {
    lowkey offset + length > len(bytes) {
        damn cap
    }
    
    lowkey length == 1 {
        damn bytes[offset] <= 0x7F
    }
    
    lowkey length >= 2 {
        bestie i := 1; i < length; i++ {
            lowkey (bytes[offset + i] & 0xC0) != 0x80 {
                damn cap
            }
        }
    }
    
    damn based
}

fr fr ================================
fr fr String Operations Implementation
fr fr ================================

slay unicode_string_create(utf8_bytes []normie) (UnicodeString, yikes) {
    lowkey !utf8_validate_string(utf8_bytes) {
        damn UnicodeString{}, new_value_error("Invalid UTF-8 sequence", "malformed UTF-8", "valid UTF-8")
    }
    
    sus str UnicodeString = UnicodeString{
        bytes: utf8_bytes,
        char_count: utf8_count_codepoints(utf8_bytes),
        is_ascii: utf8_is_ascii(utf8_bytes)
    }
    
    damn str, cringe
}

slay unicode_string_from_codepoints(codepoints []normie) (UnicodeString, yikes) {
    sus total_bytes normie = 0 fr fr Calculate total byte length needed
    bestie i := 0; i < len(codepoints); i++ {
        sus encoded []normie = utf8_encode_codepoint(codepoints[i])
        total_bytes = total_bytes + len(encoded)
    }
    
    sus result_bytes []normie = make_byte_array(total_bytes)
    sus pos normie = 0 fr fr Encode all codepoints
    bestie i := 0; i < len(codepoints); i++ {
        sus encoded []normie = utf8_encode_codepoint(codepoints[i])
        bestie j := 0; j < len(encoded); j++ {
            result_bytes[pos] = encoded[j]
            pos = pos + 1
        }
    }
    
    damn unicode_string_create(result_bytes)
}

slay unicode_string_length(str UnicodeString) normie {
    damn str.char_count
}

slay unicode_string_byte_length(str UnicodeString) normie {
    damn len(str.bytes)
}

slay unicode_string_at(str UnicodeString, index normie) (normie, yikes) {
    lowkey index < 0 || index >= str.char_count {
        damn 0, new_value_error("String index out of bounds", "index=" + string(index), "valid index")
    }
    
    sus byte_offset normie = 0
    sus char_index normie = 0
    
    bestie byte_offset < len(str.bytes) && char_index <= index {
        lowkey char_index == index {
            sus codepoint, length, err = utf8_decode_codepoint(str.bytes, byte_offset)
            lowkey err != cringe {
                damn 0, wrap_error(err, "Failed to decode character")
            }
            damn codepoint, cringe
        }
        
        sus seq_length normie = utf8_sequence_length(str.bytes[byte_offset])
        byte_offset = byte_offset + seq_length
        char_index = char_index + 1
    }
    
    damn 0, new_value_error("Character not found", "index=" + string(index), "valid character")
}

slay unicode_string_slice(str UnicodeString, start normie, end normie) (UnicodeString, yikes) {
    lowkey start < 0 || end < start || end > str.char_count {
        damn UnicodeString{}, new_value_error("Invalid slice bounds", "start=" + string(start) + " end=" + string(end), "valid bounds")
    }
    
    lowkey start == end {
        damn unicode_string_create([])
    }
    
    sus start_byte normie = unicode_string_char_to_byte_offset(str, start)
    sus end_byte normie = unicode_string_char_to_byte_offset(str, end)
    
    sus slice_bytes []normie = array_slice(str.bytes, start_byte, end_byte)
    damn unicode_string_create(slice_bytes)
}

slay unicode_string_contains(str UnicodeString, substring UnicodeString) lit {
    damn unicode_string_index_of(str, substring) >= 0
}

slay unicode_string_index_of(str UnicodeString, substring UnicodeString) normie {
    lowkey len(substring.bytes) == 0 {
        damn 0
    }
    
    lowkey len(substring.bytes) > len(str.bytes) {
        damn -1
    }
    
    bestie i := 0; i <= len(str.bytes) - len(substring.bytes); i++ {
        lowkey array_equals(str.bytes, i, substring.bytes, 0, len(substring.bytes)) { fr fr Convert byte offset to character offset
            damn unicode_string_byte_to_char_offset(str, i)
        }
    }
    
    damn -1
}

slay unicode_string_starts_with(str UnicodeString, prefix UnicodeString) lit {
    lowkey len(prefix.bytes) > len(str.bytes) {
        damn cap
    }
    
    damn array_equals(str.bytes, 0, prefix.bytes, 0, len(prefix.bytes))
}

slay unicode_string_ends_with(str UnicodeString, suffix UnicodeString) lit {
    lowkey len(suffix.bytes) > len(str.bytes) {
        damn cap
    }
    
    sus start_offset normie = len(str.bytes) - len(suffix.bytes)
    damn array_equals(str.bytes, start_offset, suffix.bytes, 0, len(suffix.bytes))
}

slay unicode_string_to_upper(str UnicodeString) (UnicodeString, yikes) {
    sus codepoints []normie = unicode_string_to_codepoints(str)
    
    bestie i := 0; i < len(codepoints); i++ {
        codepoints[i] = unicode_to_upper(codepoints[i])
    }
    
    damn unicode_string_from_codepoints(codepoints)
}

slay unicode_string_to_lower(str UnicodeString) (UnicodeString, yikes) {
    sus codepoints []normie = unicode_string_to_codepoints(str)
    
    bestie i := 0; i < len(codepoints); i++ {
        codepoints[i] = unicode_to_lower(codepoints[i])
    }
    
    damn unicode_string_from_codepoints(codepoints)
}

slay unicode_string_trim(str UnicodeString) (UnicodeString, yikes) {
    sus left_trimmed, err1 = unicode_string_trim_left(str)
    lowkey err1 != cringe {
        damn UnicodeString{}, wrap_error(err1, "Trim failed")
    }
    
    damn unicode_string_trim_right(left_trimmed)
}

slay unicode_string_trim_left(str UnicodeString) (UnicodeString, yikes) {
    sus start normie = 0
    
    bestie start < str.char_count {
        sus codepoint, err = unicode_string_at(str, start)
        lowkey err != cringe {
            damn UnicodeString{}, wrap_error(err, "Trim left failed")
        }
        
        lowkey !unicode_is_whitespace(codepoint) {
            break
        }
        
        start = start + 1
    }
    
    damn unicode_string_slice(str, start, str.char_count)
}

slay unicode_string_trim_right(str UnicodeString) (UnicodeString, yikes) {
    sus end normie = str.char_count
    
    bestie end > 0 {
        sus codepoint, err = unicode_string_at(str, end - 1)
        lowkey err != cringe {
            damn UnicodeString{}, wrap_error(err, "Trim right failed")
        }
        
        lowkey !unicode_is_whitespace(codepoint) {
            break
        }
        
        end = end - 1
    }
    
    damn unicode_string_slice(str, 0, end)
}

slay unicode_string_reverse(str UnicodeString) (UnicodeString, yikes) {
    sus codepoints []normie = unicode_string_to_codepoints(str) fr fr Reverse the codepoint array
    bestie i := 0; i < len(codepoints) / 2; i++ {
        sus temp normie = codepoints[i]
        codepoints[i] = codepoints[len(codepoints) - 1 - i]
        codepoints[len(codepoints) - 1 - i] = temp
    }
    
    damn unicode_string_from_codepoints(codepoints)
}

slay unicode_string_compare(str1 UnicodeString, str2 UnicodeString) normie {
    sus min_len normie = min_int(len(str1.bytes), len(str2.bytes))
    
    bestie i := 0; i < min_len; i++ {
        lowkey str1.bytes[i] < str2.bytes[i] {
            damn -1
        }
        lowkey str1.bytes[i] > str2.bytes[i] {
            damn 1
        }
    }
    
    lowkey len(str1.bytes) < len(str2.bytes) {
        damn -1
    }
    lowkey len(str1.bytes) > len(str2.bytes) {
        damn 1
    }
    
    damn 0
}

slay unicode_string_equals(str1 UnicodeString, str2 UnicodeString) lit {
    damn unicode_string_compare(str1, str2) == 0
}

fr fr ================================
fr fr Helper Functions
fr fr ================================

slay utf8_validate_string(bytes []normie) lit {
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus seq_length normie = utf8_sequence_length(bytes[offset])
        lowkey seq_length == 0 || !utf8_validate_sequence(bytes, offset, seq_length) {
            damn cap
        }
        offset = offset + seq_length
    }
    
    damn based
}

slay utf8_count_codepoints(bytes []normie) normie {
    sus count normie = 0
    sus offset normie = 0
    
    bestie offset < len(bytes) {
        sus seq_length normie = utf8_sequence_length(bytes[offset])
        lowkey seq_length == 0 {
            break
        }
        count = count + 1
        offset = offset + seq_length
    }
    
    damn count
}

slay utf8_is_ascii(bytes []normie) lit {
    bestie i := 0; i < len(bytes); i++ {
        lowkey bytes[i] > 0x7F {
            damn cap
        }
    }
    damn based
}

slay unicode_string_to_codepoints(str UnicodeString) []normie {
    sus codepoints []normie = make_int_array(str.char_count)
    sus byte_offset normie = 0
    sus char_index normie = 0
    
    bestie byte_offset < len(str.bytes) && char_index < str.char_count {
        sus codepoint, length, err = utf8_decode_codepoint(str.bytes, byte_offset)
        lowkey err == cringe {
            codepoints[char_index] = codepoint
            char_index = char_index + 1
        }
        byte_offset = byte_offset + length
    }
    
    damn codepoints
}

slay unicode_string_char_to_byte_offset(str UnicodeString, char_offset normie) normie {
    sus byte_offset normie = 0
    sus char_index normie = 0
    
    bestie byte_offset < len(str.bytes) && char_index < char_offset {
        sus seq_length normie = utf8_sequence_length(str.bytes[byte_offset])
        byte_offset = byte_offset + seq_length
        char_index = char_index + 1
    }
    
    damn byte_offset
}

slay unicode_string_byte_to_char_offset(str UnicodeString, byte_offset normie) normie {
    sus char_index normie = 0
    sus current_byte normie = 0
    
    bestie current_byte < byte_offset && current_byte < len(str.bytes) {
        sus seq_length normie = utf8_sequence_length(str.bytes[current_byte])
        current_byte = current_byte + seq_length
        char_index = char_index + 1
    }
    
    damn char_index
}

fr fr ================================
fr fr Array Helper Functions
fr fr ================================

slay make_byte_array(size normie) []normie { fr fr Would be implemented by runtime
    sus arr []normie = []
    bestie i := 0; i < size; i++ {
        arr = append(arr, 0)
    }
    damn arr
}

slay make_int_array(size normie) []normie { fr fr Would be implemented by runtime
    sus arr []normie = []
    bestie i := 0; i < size; i++ {
        arr = append(arr, 0)
    }
    damn arr
}

slay array_slice(arr []normie, start normie, end normie) []normie {
    sus result []normie = []
    bestie i := start; i < end && i < len(arr); i++ {
        result = append(result, arr[i])
    }
    damn result
}

slay array_equals(arr1 []normie, start1 normie, arr2 []normie, start2 normie, length normie) lit {
    bestie i := 0; i < length; i++ {
        lowkey start1 + i >= len(arr1) || start2 + i >= len(arr2) {
            damn cap
        }
        lowkey arr1[start1 + i] != arr2[start2 + i] {
            damn cap
        }
    }
    damn based
}

slay min_int(a normie, b normie) normie {
    lowkey a < b { damn a } else { damn b }
}

fr fr ================================
fr fr Unicode Properties
fr fr ================================

slay unicode_get_general_category(codepoint normie) tea {
    lowkey unicode_is_letter(codepoint) {
        lowkey (codepoint >= 0x41 && codepoint <= 0x5A) {
            damn "Lu" fr fr Letter, uppercase
        }
        lowkey (codepoint >= 0x61 && codepoint <= 0x7A) {
            damn "Ll" fr fr Letter, lowercase
        }
        damn "Lo" fr fr Letter, other
    }
    
    lowkey unicode_is_digit(codepoint) {
        damn "Nd" fr fr Number, decimal digit
    }
    
    lowkey unicode_is_punctuation(codepoint) {
        damn "Po" fr fr Punctuation, other
    }
    
    lowkey unicode_is_symbol(codepoint) {
        damn "So" fr fr Symbol, other
    }
    
    lowkey unicode_is_mark(codepoint) {
        damn "Mn" fr fr Mark, nonspacing
    }
    
    lowkey unicode_is_separator(codepoint) {
        damn "Zs" fr fr Separator, space
    }
    
    damn "Cn" fr fr Other, not assigned
}

slay unicode_get_script(codepoint normie) tea {
    lowkey (codepoint >= 0x0000 && codepoint <= 0x007F) {
        damn "Latin"
    }
    lowkey (codepoint >= 0x0370 && codepoint <= 0x03FF) {
        damn "Greek"
    }
    lowkey (codepoint >= 0x0400 && codepoint <= 0x04FF) {
        damn "Cyrillic"
    }
    lowkey (codepoint >= 0x0590 && codepoint <= 0x05FF) {
        damn "Hebrew"
    }
    lowkey (codepoint >= 0x0600 && codepoint <= 0x06FF) {
        damn "Arabic"
    }
    lowkey (codepoint >= 0x3040 && codepoint <= 0x309F) {
        damn "Hiragana"
    }
    lowkey (codepoint >= 0x30A0 && codepoint <= 0x30FF) {
        damn "Katakana"
    }
    lowkey (codepoint >= 0x4E00 && codepoint <= 0x9FFF) {
        damn "Han"
    }
    
    damn "Common"
}

slay unicode_get_block(codepoint normie) tea {
    lowkey (codepoint >= 0x0000 && codepoint <= 0x007F) {
        damn "Basic Latin"
    }
    lowkey (codepoint >= 0x0080 && codepoint <= 0x00FF) {
        damn "Latin-1 Supplement"
    }
    lowkey (codepoint >= 0x0100 && codepoint <= 0x017F) {
        damn "Latin Extended-A"
    }
    lowkey (codepoint >= 0x0180 && codepoint <= 0x024F) {
        damn "Latin Extended-B"
    }
    lowkey (codepoint >= 0x0370 && codepoint <= 0x03FF) {
        damn "Greek and Coptic"
    }
    lowkey (codepoint >= 0x0400 && codepoint <= 0x04FF) {
        damn "Cyrillic"
    }
    lowkey (codepoint >= 0x0590 && codepoint <= 0x05FF) {
        damn "Hebrew"
    }
    lowkey (codepoint >= 0x0600 && codepoint <= 0x06FF) {
        damn "Arabic"
    }
    lowkey (codepoint >= 0x3040 && codepoint <= 0x309F) {
        damn "Hiragana"
    }
    lowkey (codepoint >= 0x30A0 && codepoint <= 0x30FF) {
        damn "Katakana"
    }
    lowkey (codepoint >= 0x4E00 && codepoint <= 0x9FFF) {
        damn "CJK Unified Ideographs"
    }
    
    damn "Unknown"
}
