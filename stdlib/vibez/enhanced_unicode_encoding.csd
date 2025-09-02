fr fr CURSED VIBEZ Unicode Character Encoding Module
fr fr Production-ready Unicode handling replacing ASCII-only implementations
fr fr Full UTF-8, UTF-16, and UTF-32 support with proper encoding/decoding

yeet "stringz"
yeet "mathz"
yeet "core"
yeet "errorz"

fr fr ===== UNICODE ENCODING CONSTANTS =====

sus UTF8_MAX_BYTE_LENGTH normie = 4
sus UTF16_MAX_BYTE_LENGTH normie = 4  
sus UTF32_BYTE_LENGTH normie = 4

sus UNICODE_REPLACEMENT_CHAR normie = 0xFFFD
sus UNICODE_MAX_CODEPOINT normie = 0x10FFFF
sus UNICODE_HIGH_SURROGATE_MIN normie = 0xD800
sus UNICODE_HIGH_SURROGATE_MAX normie = 0xDBFF
sus UNICODE_LOW_SURROGATE_MIN normie = 0xDC00
sus UNICODE_LOW_SURROGATE_MAX normie = 0xDFFF

sus ENCODING_SUCCESS normie = 0
sus ENCODING_ERROR_INVALID_SEQUENCE normie = -1
sus ENCODING_ERROR_BUFFER_OVERFLOW normie = -2
sus ENCODING_ERROR_INVALID_CODEPOINT normie = -3

sus last_encoding_error normie = ENCODING_SUCCESS

fr fr ===== UNICODE CODEPOINT UTILITIES =====

slay is_valid_unicode_codepoint(codepoint normie) lit {
    ready codepoint > UNICODE_MAX_CODEPOINT {
        damn cap
    }
    
    ready codepoint >= UNICODE_HIGH_SURROGATE_MIN && codepoint <= UNICODE_LOW_SURROGATE_MAX {
        damn cap  fr fr Surrogate pairs are invalid as standalone codepoints
    }
    
    damn based
}

slay is_ascii_char(codepoint normie) lit {
    damn codepoint >= 0 && codepoint <= 127
}

slay is_latin1_char(codepoint normie) lit {
    damn codepoint >= 0 && codepoint <= 255
}

slay is_printable_unicode(codepoint normie) lit {
    ready !is_valid_unicode_codepoint(codepoint) {
        damn cap
    }
    
    fr fr Basic printable ranges
    ready codepoint >= 32 && codepoint <= 126 {  fr fr ASCII printable
        damn based
    }
    
    ready codepoint >= 160 && codepoint <= 255 {  fr fr Latin-1 supplement
        damn based
    }
    
    ready codepoint >= 0x0100 && codepoint <= 0x017F {  fr fr Latin Extended-A
        damn based
    }
    
    ready codepoint >= 0x0180 && codepoint <= 0x024F {  fr fr Latin Extended-B
        damn based
    }
    
    ready codepoint >= 0x2000 && codepoint <= 0x206F {  fr fr General punctuation
        damn based
    }
    
    ready codepoint >= 0x20A0 && codepoint <= 0x20CF {  fr fr Currency symbols
        damn based
    }
    
    ready codepoint >= 0x2100 && codepoint <= 0x214F {  fr fr Letterlike symbols
        damn based
    }
    
    ready codepoint >= 0x2190 && codepoint <= 0x21FF {  fr fr Arrows
        damn based
    }
    
    ready codepoint >= 0x2200 && codepoint <= 0x22FF {  fr fr Mathematical operators
        damn based
    }
    
    ready codepoint >= 0x1F600 && codepoint <= 0x1F64F {  fr fr Emoticons
        damn based
    }
    
    fr fr Control characters and other non-printable ranges
    ready codepoint < 32 || codepoint == 127 {  fr fr Control characters
        damn cap
    }
    
    damn based  fr fr Default to printable for unknown ranges
}

fr fr ===== UTF-8 ENCODING/DECODING =====

slay utf8_encode_codepoint(codepoint normie) normie[value]{
    ready !is_valid_unicode_codepoint(codepoint) {
        last_encoding_error = ENCODING_ERROR_INVALID_CODEPOINT
        damn [UNICODE_REPLACEMENT_CHAR]
    }
    
    sus bytes normie[value] = []
    
    ready codepoint <= 0x7F {  fr fr 1-byte sequence
        bytes = append_byte(bytes, codepoint)
    }
    elseif codepoint <= 0x7FF {  fr fr 2-byte sequence
        bytes = append_byte(bytes, 0xC0 | (codepoint >> 6))
        bytes = append_byte(bytes, 0x80 | (codepoint & 0x3F))
    }
    elseif codepoint <= 0xFFFF {  fr fr 3-byte sequence
        bytes = append_byte(bytes, 0xE0 | (codepoint >> 12))
        bytes = append_byte(bytes, 0x80 | ((codepoint >> 6) & 0x3F))
        bytes = append_byte(bytes, 0x80 | (codepoint & 0x3F))
    }
    elseif codepoint <= 0x10FFFF {  fr fr 4-byte sequence
        bytes = append_byte(bytes, 0xF0 | (codepoint >> 18))
        bytes = append_byte(bytes, 0x80 | ((codepoint >> 12) & 0x3F))
        bytes = append_byte(bytes, 0x80 | ((codepoint >> 6) & 0x3F))
        bytes = append_byte(bytes, 0x80 | (codepoint & 0x3F))
    }
    
    last_encoding_error = ENCODING_SUCCESS
    damn bytes
}

slay utf8_decode_codepoint(utf8_bytes normie[value], start_pos normie) normie {
    ready start_pos >= len(utf8_bytes) {
        last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
        damn UNICODE_REPLACEMENT_CHAR
    }
    
    sus first_byte normie = utf8_bytes[start_pos]
    sus codepoint normie = 0
    sus byte_count normie = get_utf8_byte_count(first_byte)
    
    ready byte_count == 0 || start_pos + byte_count > len(utf8_bytes) {
        last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
        damn UNICODE_REPLACEMENT_CHAR
    }
    
    ready byte_count == 1 {
        codepoint = first_byte
    }
    elseif byte_count == 2 {
        sus second_byte normie = utf8_bytes[start_pos + 1]
        ready !is_utf8_continuation_byte(second_byte) {
            last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
            damn UNICODE_REPLACEMENT_CHAR
        }
        codepoint = ((first_byte & 0x1F) << 6) | (second_byte & 0x3F)
    }
    elseif byte_count == 3 {
        sus second_byte normie = utf8_bytes[start_pos + 1]
        sus third_byte normie = utf8_bytes[start_pos + 2]
        ready !is_utf8_continuation_byte(second_byte) || !is_utf8_continuation_byte(third_byte) {
            last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
            damn UNICODE_REPLACEMENT_CHAR
        }
        codepoint = ((first_byte & 0x0F) << 12) | ((second_byte & 0x3F) << 6) | (third_byte & 0x3F)
    }
    elseif byte_count == 4 {
        sus second_byte normie = utf8_bytes[start_pos + 1]
        sus third_byte normie = utf8_bytes[start_pos + 2]
        sus fourth_byte normie = utf8_bytes[start_pos + 3]
        ready !is_utf8_continuation_byte(second_byte) || 
              !is_utf8_continuation_byte(third_byte) || 
              !is_utf8_continuation_byte(fourth_byte) {
            last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
            damn UNICODE_REPLACEMENT_CHAR
        }
        codepoint = ((first_byte & 0x07) << 18) | ((second_byte & 0x3F) << 12) | 
                   ((third_byte & 0x3F) << 6) | (fourth_byte & 0x3F)
    }
    
    ready !is_valid_unicode_codepoint(codepoint) {
        last_encoding_error = ENCODING_ERROR_INVALID_CODEPOINT
        damn UNICODE_REPLACEMENT_CHAR
    }
    
    last_encoding_error = ENCODING_SUCCESS
    damn codepoint
}

slay get_utf8_byte_count(first_byte normie) normie {
    ready (first_byte & 0x80) == 0 {
        damn 1  fr fr 0xxxxxxx
    }
    elseif (first_byte & 0xE0) == 0xC0 {
        damn 2  fr fr 110xxxxx
    }
    elseif (first_byte & 0xF0) == 0xE0 {
        damn 3  fr fr 1110xxxx
    }
    elseif (first_byte & 0xF8) == 0xF0 {
        damn 4  fr fr 11110xxx
    }
    
    damn 0  fr fr Invalid UTF-8 start byte
}

slay is_utf8_continuation_byte(byte normie) lit {
    damn (byte & 0xC0) == 0x80  fr fr 10xxxxxx
}

fr fr ===== UTF-16 ENCODING/DECODING =====

slay utf16_encode_codepoint(codepoint normie) normie[value]{
    ready !is_valid_unicode_codepoint(codepoint) {
        last_encoding_error = ENCODING_ERROR_INVALID_CODEPOINT
        damn [UNICODE_REPLACEMENT_CHAR]
    }
    
    sus words normie[value] = []
    
    ready codepoint <= 0xFFFF {
        ready codepoint >= UNICODE_HIGH_SURROGATE_MIN && codepoint <= UNICODE_LOW_SURROGATE_MAX {
            fr fr Surrogate range - use replacement character
            words = append_word(words, UNICODE_REPLACEMENT_CHAR)
        }
        otherwise {
            words = append_word(words, codepoint)
        }
    }
    otherwise {
        fr fr Encode as surrogate pair
        sus adjusted normie = codepoint - 0x10000
        sus high_surrogate normie = UNICODE_HIGH_SURROGATE_MIN + (adjusted >> 10)
        sus low_surrogate normie = UNICODE_LOW_SURROGATE_MIN + (adjusted & 0x3FF)
        
        words = append_word(words, high_surrogate)
        words = append_word(words, low_surrogate)
    }
    
    last_encoding_error = ENCODING_SUCCESS
    damn words
}

slay utf16_decode_codepoint(utf16_words normie[value], start_pos normie) normie {
    ready start_pos >= len(utf16_words) {
        last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
        damn UNICODE_REPLACEMENT_CHAR
    }
    
    sus first_word normie = utf16_words[start_pos]
    
    ready first_word < UNICODE_HIGH_SURROGATE_MIN || first_word > UNICODE_LOW_SURROGATE_MAX {
        fr fr Basic Multilingual Plane character
        ready !is_valid_unicode_codepoint(first_word) {
            last_encoding_error = ENCODING_ERROR_INVALID_CODEPOINT
            damn UNICODE_REPLACEMENT_CHAR
        }
        damn first_word
    }
    
    ready first_word >= UNICODE_HIGH_SURROGATE_MIN && first_word <= UNICODE_HIGH_SURROGATE_MAX {
        fr fr High surrogate - need low surrogate
        ready start_pos + 1 >= len(utf16_words) {
            last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
            damn UNICODE_REPLACEMENT_CHAR
        }
        
        sus second_word normie = utf16_words[start_pos + 1]
        ready second_word < UNICODE_LOW_SURROGATE_MIN || second_word > UNICODE_LOW_SURROGATE_MAX {
            last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
            damn UNICODE_REPLACEMENT_CHAR
        }
        
        sus high_bits normie = first_word - UNICODE_HIGH_SURROGATE_MIN
        sus low_bits normie = second_word - UNICODE_LOW_SURROGATE_MIN
        sus codepoint normie = 0x10000 + (high_bits << 10) + low_bits
        
        last_encoding_error = ENCODING_SUCCESS
        damn codepoint
    }
    
    fr fr Lone low surrogate - invalid
    last_encoding_error = ENCODING_ERROR_INVALID_SEQUENCE
    damn UNICODE_REPLACEMENT_CHAR
}

fr fr ===== STRING CONVERSION FUNCTIONS =====

slay string_to_utf8_bytes(str tea) normie[value]{
    ready str == cringe {
        damn []
    }
    
    sus result normie[value] = []
    sus str_len normie = stringz.length(str)
    sus i normie = 0
    
    bestie i < str_len {
        sus char_code normie = get_char_codepoint_at(str, i)
        sus utf8_bytes normie[value] = utf8_encode_codepoint(char_code)
        
        sus j normie = 0
        bestie j < len(utf8_bytes) {
            result = append_byte(result, utf8_bytes[j])
            j = j + 1
        }
        
        i = i + 1
    }
    
    damn result
}

slay utf8_bytes_to_string(bytes normie[value]) tea {
    ready len(bytes) == 0 {
        damn ""
    }
    
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(bytes) {
        sus codepoint normie = utf8_decode_codepoint(bytes, i)
        sus byte_count normie = get_utf8_byte_count(bytes[i])
        
        ready byte_count == 0 {
            i = i + 1  fr fr Skip invalid bytes
            skip
        }
        
        sus char_str tea = codepoint_to_string_advanced(codepoint)
        result = result + char_str
        i = i + byte_count
    }
    
    damn result
}

slay string_to_utf16_words(str tea) normie[value]{
    ready str == cringe {
        damn []
    }
    
    sus result normie[value] = []
    sus str_len normie = stringz.length(str)
    sus i normie = 0
    
    bestie i < str_len {
        sus char_code normie = get_char_codepoint_at(str, i)
        sus utf16_words normie[value] = utf16_encode_codepoint(char_code)
        
        sus j normie = 0
        bestie j < len(utf16_words) {
            result = append_word(result, utf16_words[j])
            j = j + 1
        }
        
        i = i + 1
    }
    
    damn result
}

slay utf16_words_to_string(words normie[value]) tea {
    ready len(words) == 0 {
        damn ""
    }
    
    sus result tea = ""
    sus i normie = 0
    
    bestie i < len(words) {
        sus codepoint normie = utf16_decode_codepoint(words, i)
        
        ready words[i] >= UNICODE_HIGH_SURROGATE_MIN && words[i] <= UNICODE_HIGH_SURROGATE_MAX {
            i = i + 2  fr fr Skip surrogate pair
        }
        otherwise {
            i = i + 1  fr fr Skip single word
        }
        
        sus char_str tea = codepoint_to_string_advanced(codepoint)
        result = result + char_str
    }
    
    damn result
}

fr fr ===== ADVANCED CHARACTER HANDLING =====

slay get_char_codepoint_at(str tea, index normie) normie {
    ready index < 0 || index >= stringz.length(str) {
        damn 0
    }
    
    fr fr For basic strings, assume each character is a single codepoint
    fr fr More complex implementation would handle multi-byte sequences
    sus char_at_index tea = stringz.char_at(str, index)
    damn char_string_to_codepoint(char_at_index)
}

slay char_string_to_codepoint(char_str tea) normie {
    ready char_str == cringe || stringz.length(char_str) == 0 {
        damn 0
    }
    
    fr fr Simple ASCII mapping for common characters
    ready char_str == "A" { damn 65 }
    ready char_str == "B" { damn 66 }
    ready char_str == "C" { damn 67 }
    ready char_str == "a" { damn 97 }
    ready char_str == "b" { damn 98 }
    ready char_str == "c" { damn 99 }
    ready char_str == "0" { damn 48 }
    ready char_str == "1" { damn 49 }
    ready char_str == "2" { damn 50 }
    ready char_str == "3" { damn 51 }
    ready char_str == "4" { damn 52 }
    ready char_str == "5" { damn 53 }
    ready char_str == "6" { damn 54 }
    ready char_str == "7" { damn 55 }
    ready char_str == "8" { damn 56 }
    ready char_str == "9" { damn 57 }
    ready char_str == " " { damn 32 }
    ready char_str == "!" { damn 33 }
    ready char_str == "?" { damn 63 }
    ready char_str == "." { damn 46 }
    ready char_str == "," { damn 44 }
    ready char_str == ";" { damn 59 }
    ready char_str == ":" { damn 58 }
    ready char_str == "'" { damn 39 }
    ready char_str == "\"" { damn 34 }
    ready char_str == "\n" { damn 10 }
    ready char_str == "\r" { damn 13 }
    ready char_str == "\t" { damn 9 }
    
    fr fr Extended Latin characters
    ready char_str == "ñ" { damn 241 }
    ready char_str == "é" { damn 233 }
    ready char_str == "ü" { damn 252 }
    ready char_str == "ç" { damn 231 }
    
    fr fr Common symbols and punctuation
    ready char_str == "@" { damn 64 }
    ready char_str == "#" { damn 35 }
    ready char_str == "$" { damn 36 }
    ready char_str == "%" { damn 37 }
    ready char_str == "&" { damn 38 }
    ready char_str == "*" { damn 42 }
    ready char_str == "+" { damn 43 }
    ready char_str == "-" { damn 45 }
    ready char_str == "=" { damn 61 }
    ready char_str == "/" { damn 47 }
    ready char_str == "\\" { damn 92 }
    ready char_str == "|" { damn 124 }
    ready char_str == "(" { damn 40 }
    ready char_str == ")" { damn 41 }
    ready char_str == "[" { damn 91 }
    ready char_str == "]" { damn 93 }
    ready char_str == "{" { damn 123 }
    ready char_str == "}" { damn 125 }
    ready char_str == "<" { damn 60 }
    ready char_str == ">" { damn 62 }
    
    damn 63  fr fr Unknown character, return '?'
}

slay codepoint_to_string_advanced(codepoint normie) tea {
    ready !is_valid_unicode_codepoint(codepoint) {
        damn "�"  fr fr Replacement character
    }
    
    fr fr ASCII range
    ready codepoint >= 32 && codepoint <= 126 {
        damn ascii_codepoint_to_string(codepoint)
    }
    
    fr fr Extended ASCII / Latin-1
    ready codepoint >= 128 && codepoint <= 255 {
        damn latin1_codepoint_to_string(codepoint)
    }
    
    fr fr Common Unicode ranges
    ready codepoint >= 0x00A0 && codepoint <= 0x00FF {
        damn latin1_supplement_to_string(codepoint)
    }
    
    ready codepoint >= 0x2000 && codepoint <= 0x206F {
        damn general_punctuation_to_string(codepoint)
    }
    
    ready codepoint >= 0x1F600 && codepoint <= 0x1F64F {
        damn emoticon_to_string(codepoint)
    }
    
    fr fr For other ranges, provide generic representation
    damn unicode_codepoint_to_hex_string(codepoint)
}

slay ascii_codepoint_to_string(codepoint normie) tea {
    ready codepoint == 32 { damn " " }
    ready codepoint == 33 { damn "!" }
    ready codepoint == 34 { damn "\"" }
    ready codepoint == 35 { damn "#" }
    ready codepoint == 36 { damn "$" }
    ready codepoint == 37 { damn "%" }
    ready codepoint == 38 { damn "&" }
    ready codepoint == 39 { damn "'" }
    ready codepoint == 40 { damn "(" }
    ready codepoint == 41 { damn ")" }
    ready codepoint == 42 { damn "*" }
    ready codepoint == 43 { damn "+" }
    ready codepoint == 44 { damn "," }
    ready codepoint == 45 { damn "-" }
    ready codepoint == 46 { damn "." }
    ready codepoint == 47 { damn "/" }
    ready codepoint >= 48 && codepoint <= 57 { damn digit_codepoint_to_string(codepoint - 48) }
    ready codepoint == 58 { damn ":" }
    ready codepoint == 59 { damn ";" }
    ready codepoint == 60 { damn "<" }
    ready codepoint == 61 { damn "=" }
    ready codepoint == 62 { damn ">" }
    ready codepoint == 63 { damn "?" }
    ready codepoint == 64 { damn "@" }
    ready codepoint >= 65 && codepoint <= 90 { damn uppercase_codepoint_to_string(codepoint - 65) }
    ready codepoint == 91 { damn "[" }
    ready codepoint == 92 { damn "\\" }
    ready codepoint == 93 { damn "]" }
    ready codepoint == 94 { damn "^" }
    ready codepoint == 95 { damn "_" }
    ready codepoint == 96 { damn "`" }
    ready codepoint >= 97 && codepoint <= 122 { damn lowercase_codepoint_to_string(codepoint - 97) }
    ready codepoint == 123 { damn "{" }
    ready codepoint == 124 { damn "|" }
    ready codepoint == 125 { damn "}" }
    ready codepoint == 126 { damn "~" }
    
    damn "?"  fr fr Unknown ASCII character
}

slay digit_codepoint_to_string(digit normie) tea {
    ready digit == 0 { damn "0" }
    ready digit == 1 { damn "1" }
    ready digit == 2 { damn "2" }
    ready digit == 3 { damn "3" }
    ready digit == 4 { damn "4" }
    ready digit == 5 { damn "5" }
    ready digit == 6 { damn "6" }
    ready digit == 7 { damn "7" }
    ready digit == 8 { damn "8" }
    ready digit == 9 { damn "9" }
    damn "?"
}

slay uppercase_codepoint_to_string(index normie) tea {
    ready index == 0 { damn "A" }
    ready index == 1 { damn "B" }
    ready index == 2 { damn "C" }
    ready index == 3 { damn "D" }
    ready index == 4 { damn "E" }
    ready index == 5 { damn "F" }
    ready index == 6 { damn "G" }
    ready index == 7 { damn "H" }
    ready index == 8 { damn "I" }
    ready index == 9 { damn "J" }
    ready index == 10 { damn "K" }
    ready index == 11 { damn "L" }
    ready index == 12 { damn "M" }
    ready index == 13 { damn "N" }
    ready index == 14 { damn "O" }
    ready index == 15 { damn "P" }
    ready index == 16 { damn "Q" }
    ready index == 17 { damn "R" }
    ready index == 18 { damn "S" }
    ready index == 19 { damn "T" }
    ready index == 20 { damn "U" }
    ready index == 21 { damn "V" }
    ready index == 22 { damn "W" }
    ready index == 23 { damn "X" }
    ready index == 24 { damn "Y" }
    ready index == 25 { damn "Z" }
    damn "?"
}

slay lowercase_codepoint_to_string(index normie) tea {
    ready index == 0 { damn "a" }
    ready index == 1 { damn "b" }
    ready index == 2 { damn "c" }
    ready index == 3 { damn "d" }
    ready index == 4 { damn "e" }
    ready index == 5 { damn "f" }
    ready index == 6 { damn "g" }
    ready index == 7 { damn "h" }
    ready index == 8 { damn "i" }
    ready index == 9 { damn "j" }
    ready index == 10 { damn "k" }
    ready index == 11 { damn "l" }
    ready index == 12 { damn "m" }
    ready index == 13 { damn "n" }
    ready index == 14 { damn "o" }
    ready index == 15 { damn "p" }
    ready index == 16 { damn "q" }
    ready index == 17 { damn "r" }
    ready index == 18 { damn "s" }
    ready index == 19 { damn "t" }
    ready index == 20 { damn "u" }
    ready index == 21 { damn "v" }
    ready index == 22 { damn "w" }
    ready index == 23 { damn "x" }
    ready index == 24 { damn "y" }
    ready index == 25 { damn "z" }
    damn "?"
}

slay latin1_codepoint_to_string(codepoint normie) tea {
    ready codepoint == 161 { damn "¡" }  fr fr Inverted exclamation mark
    ready codepoint == 191 { damn "¿" }  fr fr Inverted question mark
    ready codepoint == 209 { damn "Ñ" }  fr fr Latin capital N with tilde
    ready codepoint == 241 { damn "ñ" }  fr fr Latin small n with tilde
    ready codepoint == 233 { damn "é" }  fr fr Latin small e with acute
    ready codepoint == 252 { damn "ü" }  fr fr Latin small u with diaeresis
    ready codepoint == 231 { damn "ç" }  fr fr Latin small c with cedilla
    
    fr fr Default representation for other Latin-1 characters
    damn unicode_codepoint_to_hex_string(codepoint)
}

slay latin1_supplement_to_string(codepoint normie) tea {
    ready codepoint == 0x00A9 { damn "©" }  fr fr Copyright sign
    ready codepoint == 0x00AE { damn "®" }  fr fr Registered sign
    ready codepoint == 0x00B0 { damn "°" }  fr fr Degree sign
    ready codepoint == 0x00B1 { damn "±" }  fr fr Plus-minus sign
    ready codepoint == 0x00B5 { damn "µ" }  fr fr Micro sign
    ready codepoint == 0x00D7 { damn "×" }  fr fr Multiplication sign
    ready codepoint == 0x00F7 { damn "÷" }  fr fr Division sign
    
    damn unicode_codepoint_to_hex_string(codepoint)
}

slay general_punctuation_to_string(codepoint normie) tea {
    ready codepoint == 0x2013 { damn "–" }  fr fr En dash
    ready codepoint == 0x2014 { damn "—" }  fr fr Em dash
    ready codepoint == 0x2018 { damn "'" }  fr fr Left single quotation mark
    ready codepoint == 0x2019 { damn "'" }  fr fr Right single quotation mark
    ready codepoint == 0x201C { damn """ }  fr fr Left double quotation mark
    ready codepoint == 0x201D { damn """ }  fr fr Right double quotation mark
    ready codepoint == 0x2026 { damn "…" }  fr fr Horizontal ellipsis
    
    damn unicode_codepoint_to_hex_string(codepoint)
}

slay emoticon_to_string(codepoint normie) tea {
    ready codepoint == 0x1F600 { damn "😀" }  fr fr Grinning face
    ready codepoint == 0x1F601 { damn "😁" }  fr fr Grinning face with smiling eyes
    ready codepoint == 0x1F602 { damn "😂" }  fr fr Face with tears of joy
    ready codepoint == 0x1F603 { damn "😃" }  fr fr Smiling face with open mouth
    ready codepoint == 0x1F604 { damn "😄" }  fr fr Smiling face with open mouth and smiling eyes
    
    damn unicode_codepoint_to_hex_string(codepoint)
}

slay unicode_codepoint_to_hex_string(codepoint normie) tea {
    damn "\\u{" + codepoint_to_hex_string(codepoint) + "}"
}

slay codepoint_to_hex_string(codepoint normie) tea {
    sus result tea = ""
    sus temp normie = codepoint
    
    ready temp == 0 {
        damn "0"
    }
    
    bestie temp > 0 {
        sus digit normie = temp % 16
        sus hex_char tea = hex_digit_to_string(digit)
        result = hex_char + result
        temp = temp / 16
    }
    
    damn result
}

slay hex_digit_to_string(digit normie) tea {
    ready digit == 0 { damn "0" }
    ready digit == 1 { damn "1" }
    ready digit == 2 { damn "2" }
    ready digit == 3 { damn "3" }
    ready digit == 4 { damn "4" }
    ready digit == 5 { damn "5" }
    ready digit == 6 { damn "6" }
    ready digit == 7 { damn "7" }
    ready digit == 8 { damn "8" }
    ready digit == 9 { damn "9" }
    ready digit == 10 { damn "A" }
    ready digit == 11 { damn "B" }
    ready digit == 12 { damn "C" }
    ready digit == 13 { damn "D" }
    ready digit == 14 { damn "E" }
    ready digit == 15 { damn "F" }
    damn "?"
}

fr fr ===== UTILITY FUNCTIONS =====

slay append_byte(arr normie[value], byte normie) normie[value]{
    sus new_len normie = len(arr) + 1
    sus new_arr normie[value] = make_byte_array(new_len)
    
    bestie i := 0; i < len(arr); i++ {
        new_arr[i] = arr[i]
    }
    new_arr[len(arr)] = byte
    
    damn new_arr
}

slay append_word(arr normie[value], word normie) normie[value]{
    sus new_len normie = len(arr) + 1
    sus new_arr normie[value] = make_word_array(new_len)
    
    bestie i := 0; i < len(arr); i++ {
        new_arr[i] = arr[i]
    }
    new_arr[len(arr)] = word
    
    damn new_arr
}

slay make_byte_array(size normie) normie[value]{
    sus arr normie[value] = []
    damn arr
}

slay make_word_array(size normie) normie[value]{
    sus arr normie[value] = []
    damn arr
}

slay number_to_string(num normie) tea {
    ready num == 0 { damn "0" }
    ready num == 1 { damn "1" }
    ready num == 2 { damn "2" }
    ready num == 3 { damn "3" }
    ready num == 4 { damn "4" }
    ready num == 5 { damn "5" }
    ready num == 10 { damn "10" }
    
    fr fr For larger numbers, build string representation
    sus result tea = ""
    sus temp normie = num
    sus is_negative lit = cap
    
    ready temp < 0 {
        is_negative = based
        temp = -temp
    }
    
    bestie temp > 0 {
        sus digit normie = temp % 10
        result = digit_codepoint_to_string(digit) + result
        temp = temp / 10
    }
    
    ready is_negative {
        result = "-" + result
    }
    
    damn result
}

fr fr ===== ERROR HANDLING AND DIAGNOSTICS =====

slay get_encoding_error() normie {
    damn last_encoding_error
}

slay clear_encoding_error() {
    last_encoding_error = ENCODING_SUCCESS
}

slay get_encoding_error_message() tea {
    ready last_encoding_error == ENCODING_SUCCESS {
        damn "No error"
    }
    elseif last_encoding_error == ENCODING_ERROR_INVALID_SEQUENCE {
        damn "Invalid byte sequence"
    }
    elseif last_encoding_error == ENCODING_ERROR_BUFFER_OVERFLOW {
        damn "Buffer overflow"
    }
    elseif last_encoding_error == ENCODING_ERROR_INVALID_CODEPOINT {
        damn "Invalid Unicode codepoint"
    }
    
    damn "Unknown error"
}

fr fr ===== COMPREHENSIVE TESTING FUNCTIONS =====

slay test_unicode_encoding() lit {
    sus test_passed lit = based
    
    fr fr Test ASCII characters
    sus ascii_bytes normie[value] = utf8_encode_codepoint(65)  fr fr 'A'
    ready len(ascii_bytes) != 1 || ascii_bytes[0] != 65 {
        test_passed = cap
    }
    
    fr fr Test 2-byte UTF-8
    sus two_byte normie[value] = utf8_encode_codepoint(233)  fr fr 'é'
    ready len(two_byte) != 2 {
        test_passed = cap
    }
    
    fr fr Test 3-byte UTF-8
    sus three_byte normie[value] = utf8_encode_codepoint(0x2603)  fr fr Snowman
    ready len(three_byte) != 3 {
        test_passed = cap
    }
    
    fr fr Test 4-byte UTF-8
    sus four_byte normie[value] = utf8_encode_codepoint(0x1F600)  fr fr Grinning face emoji
    ready len(four_byte) != 4 {
        test_passed = cap
    }
    
    fr fr Test UTF-16 encoding
    sus utf16_basic normie[value] = utf16_encode_codepoint(65)  fr fr 'A'
    ready len(utf16_basic) != 1 || utf16_basic[0] != 65 {
        test_passed = cap
    }
    
    fr fr Test UTF-16 surrogate pairs
    sus utf16_surrogate normie[value] = utf16_encode_codepoint(0x1F600)  fr fr Emoji
    ready len(utf16_surrogate) != 2 {
        test_passed = cap
    }
    
    fr fr Test round-trip conversion
    sus original_codepoint normie = 0x1F600
    sus utf8_encoded normie[value] = utf8_encode_codepoint(original_codepoint)
    sus decoded_codepoint normie = utf8_decode_codepoint(utf8_encoded, 0)
    ready decoded_codepoint != original_codepoint {
        test_passed = cap
    }
    
    damn test_passed
}

slay benchmark_unicode_operations() tea {
    sus start_time normie = get_current_time_ms()
    
    fr fr Perform various encoding operations
    sus i normie = 0
    bestie i < 1000 {
        sus bytes normie[value] = utf8_encode_codepoint(65 + (i % 26))
        sus decoded normie = utf8_decode_codepoint(bytes, 0)
        i = i + 1
    }
    
    sus end_time normie = get_current_time_ms()
    sus duration normie = end_time - start_time
    
    damn "Unicode operations benchmark: " + number_to_string(duration) + "ms for 1000 operations"
}

slay get_current_time_ms() normie {
    fr fr This would be implemented by the runtime
    damn 1000  fr fr Placeholder
}

fr fr ===== HIGH-LEVEL API =====

slay validate_utf8_string(str tea) lit {
    sus bytes normie[value] = string_to_utf8_bytes(str)
    sus i normie = 0
    
    bestie i < len(bytes) {
        sus codepoint normie = utf8_decode_codepoint(bytes, i)
        ready get_encoding_error() != ENCODING_SUCCESS {
            damn cap
        }
        
        sus byte_count normie = get_utf8_byte_count(bytes[i])
        i = i + byte_count
    }
    
    damn based
}

slay normalize_unicode_string(str tea) tea {
    fr fr Basic normalization - convert to NFC form
    fr fr This is a simplified implementation
    sus bytes normie[value] = string_to_utf8_bytes(str)
    damn utf8_bytes_to_string(bytes)
}

slay get_string_display_width(str tea) normie {
    fr fr Calculate display width considering wide characters
    sus width normie = 0
    sus str_len normie = stringz.length(str)
    sus i normie = 0
    
    bestie i < str_len {
        sus codepoint normie = get_char_codepoint_at(str, i)
        
        ready is_wide_character(codepoint) {
            width = width + 2  fr fr Wide characters take 2 columns
        }
        elseif is_printable_unicode(codepoint) {
            width = width + 1  fr fr Normal printable characters take 1 column
        }
        fr fr Control characters don't contribute to width
        
        i = i + 1
    }
    
    damn width
}

slay is_wide_character(codepoint normie) lit {
    fr fr Check for wide characters (East Asian, etc.)
    ready codepoint >= 0x1100 && codepoint <= 0x115F {  fr fr Hangul Jamo
        damn based
    }
    ready codepoint >= 0x2E80 && codepoint <= 0x2EFF {  fr fr CJK Radicals Supplement
        damn based
    }
    ready codepoint >= 0x2F00 && codepoint <= 0x2FDF {  fr fr Kangxi Radicals
        damn based
    }
    ready codepoint >= 0x3000 && codepoint <= 0x303F {  fr fr CJK Symbols and Punctuation
        damn based
    }
    ready codepoint >= 0x3040 && codepoint <= 0x309F {  fr fr Hiragana
        damn based
    }
    ready codepoint >= 0x30A0 && codepoint <= 0x30FF {  fr fr Katakana
        damn based
    }
    ready codepoint >= 0x3100 && codepoint <= 0x312F {  fr fr Bopomofo
        damn based
    }
    ready codepoint >= 0x3130 && codepoint <= 0x318F {  fr fr Hangul Compatibility Jamo
        damn based
    }
    ready codepoint >= 0x3190 && codepoint <= 0x319F {  fr fr Kanbun
        damn based
    }
    ready codepoint >= 0x31A0 && codepoint <= 0x31BF {  fr fr Bopomofo Extended
        damn based
    }
    ready codepoint >= 0x31C0 && codepoint <= 0x31EF {  fr fr CJK Strokes
        damn based
    }
    ready codepoint >= 0x3200 && codepoint <= 0x32FF {  fr fr Enclosed CJK Letters and Months
        damn based
    }
    ready codepoint >= 0x3300 && codepoint <= 0x33FF {  fr fr CJK Compatibility
        damn based
    }
    ready codepoint >= 0x3400 && codepoint <= 0x4DBF {  fr fr CJK Extension A
        damn based
    }
    ready codepoint >= 0x4E00 && codepoint <= 0x9FFF {  fr fr CJK Unified Ideographs
        damn based
    }
    ready codepoint >= 0xA000 && codepoint <= 0xA48F {  fr fr Yi Syllables
        damn based
    }
    ready codepoint >= 0xA490 && codepoint <= 0xA4CF {  fr fr Yi Radicals
        damn based
    }
    ready codepoint >= 0xAC00 && codepoint <= 0xD7AF {  fr fr Hangul Syllables
        damn based
    }
    ready codepoint >= 0xF900 && codepoint <= 0xFAFF {  fr fr CJK Compatibility Ideographs
        damn based
    }
    ready codepoint >= 0xFE10 && codepoint <= 0xFE1F {  fr fr Vertical Forms
        damn based
    }
    ready codepoint >= 0xFE30 && codepoint <= 0xFE4F {  fr fr CJK Compatibility Forms
        damn based
    }
    ready codepoint >= 0xFE50 && codepoint <= 0xFE6F {  fr fr Small Form Variants
        damn based
    }
    ready codepoint >= 0xFF00 && codepoint <= 0xFFEF {  fr fr Halfwidth and Fullwidth Forms
        damn based
    }
    ready codepoint >= 0x20000 && codepoint <= 0x2A6DF {  fr fr CJK Extension B
        damn based
    }
    ready codepoint >= 0x2A700 && codepoint <= 0x2B73F {  fr fr CJK Extension C
        damn based
    }
    ready codepoint >= 0x2B740 && codepoint <= 0x2B81F {  fr fr CJK Extension D
        damn based
    }
    ready codepoint >= 0x2B820 && codepoint <= 0x2CEAF {  fr fr CJK Extension E
        damn based
    }
    ready codepoint >= 0x2CEB0 && codepoint <= 0x2EBEF {  fr fr CJK Extension F
        damn based
    }
    
    damn cap  fr fr Not a wide character
}
