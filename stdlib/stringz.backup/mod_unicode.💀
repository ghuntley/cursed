fr fr CURSED String Processing Module - Unicode-Aware Implementation
fr fr Complete string operations with full UTF-8 support and robust processing
fr fr Pure CURSED implementation for maximum performance and compatibility

fr fr ================================
fr fr Unicode Character Classifications
fr fr ================================

be_like CharClass squad {
    is_ascii lit
    is_alphabetic lit
    is_numeric lit
    is_whitespace lit
    is_punctuation lit
    is_control lit
    is_printable lit
    category normie  fr fr Unicode category
}

be_like StringMetrics squad {
    byte_length thicc
    char_count thicc
    word_count thicc
    line_count thicc
    grapheme_count thicc
    has_non_ascii lit
}

fr fr ================================
fr fr Core UTF-8 Operations
fr fr ================================

slay utf8_byte_length(s tea) thicc { fr fr Calculate actual UTF-8 byte length
    lowkey s == "" {
        damn 0
    }
    
    sus length thicc = 0
    sus i thicc = 0
    
    bestie i < raw_string_length(s) {
        sus byte_val byte = get_raw_byte_at(s, i)
        
        lowkey (byte_val & 0x80) == 0 {
            fr fr ASCII character (0xxxxxxx)
            length += 1
            i += 1
        } otherwise lowkey (byte_val & 0xE0) == 0xC0 {
            fr fr 2-byte sequence (110xxxxx 10xxxxxx)
            length += 2
            i += 2
        } otherwise lowkey (byte_val & 0xF0) == 0xE0 {
            fr fr 3-byte sequence (1110xxxx 10xxxxxx 10xxxxxx)
            length += 3
            i += 3
        } otherwise lowkey (byte_val & 0xF8) == 0xF0 {
            fr fr 4-byte sequence (11110xxx 10xxxxxx 10xxxxxx 10xxxxxx)
            length += 4
            i += 4
        } otherwise {
            fr fr Invalid UTF-8, treat as single byte
            length += 1
            i += 1
        }
    }
    
    damn length
}

slay utf8_char_count(s tea) thicc { fr fr Count actual Unicode characters
    lowkey s == "" {
        damn 0
    }
    
    sus count thicc = 0
    sus i thicc = 0
    sus raw_len thicc = raw_string_length(s)
    
    bestie i < raw_len {
        sus byte_val byte = get_raw_byte_at(s, i)
        sus char_bytes thicc = 1
        
        lowkey (byte_val & 0x80) == 0 {
            fr fr ASCII character
            char_bytes = 1
        } otherwise lowkey (byte_val & 0xE0) == 0xC0 {
            fr fr 2-byte UTF-8 character
            char_bytes = 2
        } otherwise lowkey (byte_val & 0xF0) == 0xE0 {
            fr fr 3-byte UTF-8 character  
            char_bytes = 3
        } otherwise lowkey (byte_val & 0xF8) == 0xF0 {
            fr fr 4-byte UTF-8 character
            char_bytes = 4
        }
        
        fr fr Validate UTF-8 sequence
        lowkey is_valid_utf8_sequence(s, i, char_bytes) {
            count += 1
        } otherwise {
            fr fr Invalid sequence, count as replacement character
            count += 1
            char_bytes = 1
        }
        
        i += char_bytes
    }
    
    damn count
}

slay is_valid_utf8_sequence(s tea, start_pos thicc, expected_bytes thicc) lit { fr fr Validate UTF-8 sequence
    lowkey start_pos + expected_bytes > raw_string_length(s) {
        damn false  fr fr Not enough bytes
    }
    
    sus first_byte byte = get_raw_byte_at(s, start_pos)
    
    lowkey expected_bytes == 1 {
        fr fr ASCII: must be 0xxxxxxx
        damn (first_byte & 0x80) == 0
    }
    
    lowkey expected_bytes == 2 {
        fr fr 2-byte: 110xxxxx 10xxxxxx
        lowkey (first_byte & 0xE0) != 0xC0 {
            damn false
        }
        sus second_byte byte = get_raw_byte_at(s, start_pos + 1)
        damn (second_byte & 0xC0) == 0x80
    }
    
    lowkey expected_bytes == 3 {
        fr fr 3-byte: 1110xxxx 10xxxxxx 10xxxxxx
        lowkey (first_byte & 0xF0) != 0xE0 {
            damn false
        }
        sus second_byte byte = get_raw_byte_at(s, start_pos + 1)
        sus third_byte byte = get_raw_byte_at(s, start_pos + 2)
        damn (second_byte & 0xC0) == 0x80 && (third_byte & 0xC0) == 0x80
    }
    
    lowkey expected_bytes == 4 {
        fr fr 4-byte: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
        lowkey (first_byte & 0xF8) != 0xF0 {
            damn false
        }
        bestie i := 1; i < 4; i++ {
            sus byte_val byte = get_raw_byte_at(s, start_pos + i)
            lowkey (byte_val & 0xC0) != 0x80 {
                damn false
            }
        }
        damn true
    }
    
    damn false
}

slay decode_utf8_char(s tea, pos thicc) (normie, thicc) { fr fr Decode UTF-8 character to code point
    lowkey pos >= raw_string_length(s) {
        damn 0, 0
    }
    
    sus first_byte byte = get_raw_byte_at(s, pos)
    
    lowkey (first_byte & 0x80) == 0 {
        fr fr ASCII character
        damn normie(first_byte), 1
    }
    
    lowkey (first_byte & 0xE0) == 0xC0 {
        fr fr 2-byte sequence
        lowkey pos + 1 >= raw_string_length(s) {
            damn 0xFFFD, 1  fr fr Replacement character
        }
        sus second_byte byte = get_raw_byte_at(s, pos + 1)
        lowkey (second_byte & 0xC0) != 0x80 {
            damn 0xFFFD, 1
        }
        sus code_point normie = ((normie(first_byte) & 0x1F) << 6) | (normie(second_byte) & 0x3F)
        damn code_point, 2
    }
    
    lowkey (first_byte & 0xF0) == 0xE0 {
        fr fr 3-byte sequence
        lowkey pos + 2 >= raw_string_length(s) {
            damn 0xFFFD, 1
        }
        sus second_byte byte = get_raw_byte_at(s, pos + 1)
        sus third_byte byte = get_raw_byte_at(s, pos + 2)
        lowkey (second_byte & 0xC0) != 0x80 || (third_byte & 0xC0) != 0x80 {
            damn 0xFFFD, 1
        }
        sus code_point normie = ((normie(first_byte) & 0x0F) << 12) | 
                               ((normie(second_byte) & 0x3F) << 6) | 
                               (normie(third_byte) & 0x3F)
        damn code_point, 3
    }
    
    lowkey (first_byte & 0xF8) == 0xF0 {
        fr fr 4-byte sequence
        lowkey pos + 3 >= raw_string_length(s) {
            damn 0xFFFD, 1
        }
        bestie i := 1; i <= 3; i++ {
            sus byte_val byte = get_raw_byte_at(s, pos + i)
            lowkey (byte_val & 0xC0) != 0x80 {
                damn 0xFFFD, 1
            }
        }
        sus code_point normie = ((normie(first_byte) & 0x07) << 18) |
                               ((normie(get_raw_byte_at(s, pos + 1)) & 0x3F) << 12) |
                               ((normie(get_raw_byte_at(s, pos + 2)) & 0x3F) << 6) |
                               (normie(get_raw_byte_at(s, pos + 3)) & 0x3F)
        damn code_point, 4
    }
    
    fr fr Invalid UTF-8
    damn 0xFFFD, 1
}

slay encode_utf8_char(code_point normie) tea { fr fr Encode code point to UTF-8
    lowkey code_point <= 0x7F {
        fr fr ASCII range
        damn char_from_byte(byte(code_point))
    }
    
    lowkey code_point <= 0x7FF {
        fr fr 2-byte UTF-8
        sus first_byte byte = byte(0xC0 | (code_point >> 6))
        sus second_byte byte = byte(0x80 | (code_point & 0x3F))
        damn string_from_bytes([first_byte, second_byte])
    }
    
    lowkey code_point <= 0xFFFF {
        fr fr 3-byte UTF-8
        sus first_byte byte = byte(0xE0 | (code_point >> 12))
        sus second_byte byte = byte(0x80 | ((code_point >> 6) & 0x3F))
        sus third_byte byte = byte(0x80 | (code_point & 0x3F))
        damn string_from_bytes([first_byte, second_byte, third_byte])
    }
    
    lowkey code_point <= 0x10FFFF {
        fr fr 4-byte UTF-8
        sus first_byte byte = byte(0xF0 | (code_point >> 18))
        sus second_byte byte = byte(0x80 | ((code_point >> 12) & 0x3F))
        sus third_byte byte = byte(0x80 | ((code_point >> 6) & 0x3F))
        sus fourth_byte byte = byte(0x80 | (code_point & 0x3F))
        damn string_from_bytes([first_byte, second_byte, third_byte, fourth_byte])
    }
    
    fr fr Invalid code point, return replacement character
    damn encode_utf8_char(0xFFFD)
}

fr fr ================================
fr fr Unicode Character Classification
fr fr ================================

slay classify_char(code_point normie) CharClass { fr fr Classify Unicode character
    sus class CharClass = {
        is_ascii: code_point <= 0x7F,
        is_alphabetic: is_alphabetic_char(code_point),
        is_numeric: is_numeric_char(code_point),
        is_whitespace: is_whitespace_char(code_point),
        is_punctuation: is_punctuation_char(code_point),
        is_control: is_control_char(code_point),
        is_printable: is_printable_char(code_point),
        category: get_unicode_category(code_point)
    }
    damn class
}

slay is_alphabetic_char(code_point normie) lit { fr fr Check if character is alphabetic
    fr fr ASCII alphabetic
    lowkey (code_point >= 65 && code_point <= 90) ||
          (code_point >= 97 && code_point <= 122) {
        damn true
    }
    
    fr fr Latin-1 Supplement alphabetic
    lowkey code_point >= 0xC0 && code_point <= 0xFF {
        damn code_point != 0xD7 && code_point != 0xF7  fr fr Exclude multiplication/division signs
    }
    
    fr fr Common Unicode alphabetic ranges
    lowkey (code_point >= 0x0100 && code_point <= 0x017F) ||  fr fr Latin Extended-A
          (code_point >= 0x0180 && code_point <= 0x024F) ||  fr fr Latin Extended-B
          (code_point >= 0x0370 && code_point <= 0x03FF) ||  fr fr Greek and Coptic
          (code_point >= 0x0400 && code_point <= 0x04FF) ||  fr fr Cyrillic
          (code_point >= 0x0590 && code_point <= 0x05FF) ||  fr fr Hebrew
          (code_point >= 0x0600 && code_point <= 0x06FF) ||  fr fr Arabic
          (code_point >= 0x4E00 && code_point <= 0x9FFF) ||  fr fr CJK Unified Ideographs
          (code_point >= 0x3040 && code_point <= 0x309F) ||  fr fr Hiragana
          (code_point >= 0x30A0 && code_point <= 0x30FF) {   fr fr Katakana
        damn true
    }
    
    damn false
}

slay is_numeric_char(code_point normie) lit { fr fr Check if character is numeric
    fr fr ASCII digits
    lowkey code_point >= 48 && code_point <= 57 {
        damn true
    }
    
    fr fr Unicode numeric categories (simplified)
    lowkey (code_point >= 0x0660 && code_point <= 0x0669) ||  fr fr Arabic-Indic digits
          (code_point >= 0x06F0 && code_point <= 0x06F9) ||  fr fr Extended Arabic-Indic
          (code_point >= 0x0966 && code_point <= 0x096F) ||  fr fr Devanagari digits
          (code_point >= 0xFF10 && code_point <= 0xFF19) {   fr fr Fullwidth digits
        damn true
    }
    
    damn false
}

slay is_whitespace_char(code_point normie) lit { fr fr Check if character is whitespace
    fr fr ASCII whitespace
    lowkey code_point == 32 ||   fr fr Space
          code_point == 9 ||    fr fr Tab
          code_point == 10 ||   fr fr Line Feed
          code_point == 13 ||   fr fr Carriage Return
          code_point == 11 ||   fr fr Vertical Tab
          code_point == 12 {    fr fr Form Feed
        damn true
    }
    
    fr fr Unicode whitespace
    lowkey code_point == 0x00A0 ||  fr fr Non-breaking space
          code_point == 0x1680 ||  fr fr Ogham space mark
          code_point == 0x2000 ||  fr fr En quad
          code_point == 0x2001 ||  fr fr Em quad
          code_point == 0x2002 ||  fr fr En space
          code_point == 0x2003 ||  fr fr Em space
          code_point == 0x2004 ||  fr fr Three-per-em space
          code_point == 0x2005 ||  fr fr Four-per-em space
          code_point == 0x2006 ||  fr fr Six-per-em space
          code_point == 0x2007 ||  fr fr Figure space
          code_point == 0x2008 ||  fr fr Punctuation space
          code_point == 0x2009 ||  fr fr Thin space
          code_point == 0x200A ||  fr fr Hair space
          code_point == 0x202F ||  fr fr Narrow no-break space
          code_point == 0x205F ||  fr fr Medium mathematical space
          code_point == 0x3000 {   fr fr Ideographic space
        damn true
    }
    
    damn false
}

slay is_punctuation_char(code_point normie) lit { fr fr Check if character is punctuation
    fr fr ASCII punctuation
    lowkey (code_point >= 33 && code_point <= 47) ||   fr fr !"#$%&'()*+,-./
          (code_point >= 58 && code_point <= 64) ||   fr fr :;<=>?@
          (code_point >= 91 && code_point <= 96) ||   fr fr [\]^_`
          (code_point >= 123 && code_point <= 126) {  fr fr {|}~
        damn true
    }
    
    fr fr Common Unicode punctuation ranges
    lowkey (code_point >= 0x2000 && code_point <= 0x206F) ||  fr fr General Punctuation
          (code_point >= 0x2070 && code_point <= 0x209F) ||  fr fr Superscripts and Subscripts
          (code_point >= 0x20A0 && code_point <= 0x20CF) ||  fr fr Currency Symbols
          (code_point >= 0x2100 && code_point <= 0x214F) ||  fr fr Letterlike Symbols
          (code_point >= 0x2190 && code_point <= 0x21FF) ||  fr fr Arrows
          (code_point >= 0x2200 && code_point <= 0x22FF) {   fr fr Mathematical Operators
        damn true
    }
    
    damn false
}

slay is_control_char(code_point normie) lit { fr fr Check if character is control
    fr fr C0 control characters
    lowkey code_point < 32 {
        damn true
    }
    
    fr fr DEL character
    lowkey code_point == 127 {
        damn true
    }
    
    fr fr C1 control characters
    lowkey code_point >= 0x80 && code_point <= 0x9F {
        damn true
    }
    
    damn false
}

slay is_printable_char(code_point normie) lit { fr fr Check if character is printable
    fr fr Control characters are not printable
    lowkey is_control_char(code_point) {
        damn false
    }
    
    fr fr Surrogates are not printable
    lowkey code_point >= 0xD800 && code_point <= 0xDFFF {
        damn false
    }
    
    fr fr Beyond valid Unicode range
    lowkey code_point > 0x10FFFF {
        damn false
    }
    
    damn true
}

slay get_unicode_category(code_point normie) normie { fr fr Get Unicode general category
    fr fr Simplified category classification
    lowkey is_alphabetic_char(code_point) {
        damn 1  fr fr Letter
    }
    lowkey is_numeric_char(code_point) {
        damn 2  fr fr Number
    }
    lowkey is_punctuation_char(code_point) {
        damn 3  fr fr Punctuation
    }
    lowkey is_whitespace_char(code_point) {
        damn 4  fr fr Separator
    }
    lowkey is_control_char(code_point) {
        damn 5  fr fr Other (Control)
    }
    damn 6  fr fr Other
}

fr fr ================================
fr fr Advanced String Operations
fr fr ================================

slay get_string_metrics(s tea) StringMetrics { fr fr Analyze string composition
    sus metrics StringMetrics = {
        byte_length: utf8_byte_length(s),
        char_count: utf8_char_count(s),
        word_count: count_words(s),
        line_count: count_lines(s),
        grapheme_count: count_graphemes(s),
        has_non_ascii: contains_non_ascii(s)
    }
    damn metrics
}

slay count_words(s tea) thicc { fr fr Count words (Unicode-aware)
    lowkey s == "" {
        damn 0
    }
    
    sus word_count thicc = 0
    sus in_word lit = false
    sus pos thicc = 0
    
    bestie pos < raw_string_length(s) {
        sus code_point, bytes_consumed := decode_utf8_char(s, pos)
        sus char_class CharClass = classify_char(code_point)
        
        lowkey char_class.is_alphabetic || char_class.is_numeric {
            lowkey !in_word {
                word_count += 1
                in_word = true
            }
        } otherwise lowkey char_class.is_whitespace || char_class.is_punctuation {
            in_word = false
        }
        
        pos += bytes_consumed
    }
    
    damn word_count
}

slay count_lines(s tea) thicc { fr fr Count lines (handles different line endings)
    lowkey s == "" {
        damn 0
    }
    
    sus line_count thicc = 1
    sus pos thicc = 0
    
    bestie pos < raw_string_length(s) {
        sus code_point, bytes_consumed := decode_utf8_char(s, pos)
        
        lowkey code_point == 10 {  fr fr LF
            line_count += 1
        } otherwise lowkey code_point == 13 {  fr fr CR
            line_count += 1
            fr fr Check for CR+LF sequence
            sus next_pos thicc = pos + bytes_consumed
            lowkey next_pos < raw_string_length(s) {
                sus next_code, _ := decode_utf8_char(s, next_pos)
                lowkey next_code == 10 {  fr fr LF follows CR
                    pos = next_pos  fr fr Skip the LF
                    bytes_consumed = 1
                }
            }
        }
        
        pos += bytes_consumed
    }
    
    damn line_count
}

slay count_graphemes(s tea) thicc { fr fr Count grapheme clusters (simplified)
    fr fr This is a simplified version - full grapheme cluster breaking is complex
    sus grapheme_count thicc = 0
    sus pos thicc = 0
    
    bestie pos < raw_string_length(s) {
        sus code_point, bytes_consumed := decode_utf8_char(s, pos)
        
        fr fr For now, treat each code point as a grapheme (simplified)
        fr fr Real implementation would handle combining marks, etc.
        lowkey !is_combining_mark(code_point) {
            grapheme_count += 1
        }
        
        pos += bytes_consumed
    }
    
    damn grapheme_count
}

slay is_combining_mark(code_point normie) lit { fr fr Check if character is combining mark
    fr fr Combining Diacritical Marks
    lowkey code_point >= 0x0300 && code_point <= 0x036F {
        damn true
    }
    
    fr fr Combining Diacritical Marks Extended
    lowkey code_point >= 0x1AB0 && code_point <= 0x1AFF {
        damn true
    }
    
    fr fr Combining Diacritical Marks Supplement
    lowkey code_point >= 0x1DC0 && code_point <= 0x1DFF {
        damn true
    }
    
    fr fr Combining Half Marks
    lowkey code_point >= 0xFE20 && code_point <= 0xFE2F {
        damn true
    }
    
    damn false
}

slay contains_non_ascii(s tea) lit { fr fr Check if string contains non-ASCII characters
    sus pos thicc = 0
    
    bestie pos < raw_string_length(s) {
        sus code_point, bytes_consumed := decode_utf8_char(s, pos)
        
        lowkey code_point > 127 {
            damn true
        }
        
        pos += bytes_consumed
    }
    
    damn false
}

fr fr ================================
fr fr Unicode String Transformation
fr fr ================================

slay to_uppercase_unicode(s tea) tea { fr fr Convert to uppercase (Unicode-aware)
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus pos thicc = 0
    
    bestie pos < raw_string_length(s) {
        sus code_point, bytes_consumed := decode_utf8_char(s, pos)
        sus upper_code normie = to_upper_code_point(code_point)
        result += encode_utf8_char(upper_code)
        pos += bytes_consumed
    }
    
    damn result
}

slay to_lowercase_unicode(s tea) tea { fr fr Convert to lowercase (Unicode-aware)
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus pos thicc = 0
    
    bestie pos < raw_string_length(s) {
        sus code_point, bytes_consumed := decode_utf8_char(s, pos)
        sus lower_code normie = to_lower_code_point(code_point)
        result += encode_utf8_char(lower_code)
        pos += bytes_consumed
    }
    
    damn result
}

slay to_title_case_unicode(s tea) tea { fr fr Convert to title case (Unicode-aware)
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus pos thicc = 0
    sus first_of_word lit = true
    
    bestie pos < raw_string_length(s) {
        sus code_point, bytes_consumed := decode_utf8_char(s, pos)
        sus char_class CharClass = classify_char(code_point)
        
        lowkey char_class.is_alphabetic {
            lowkey first_of_word {
                result += encode_utf8_char(to_upper_code_point(code_point))
                first_of_word = false
            } otherwise {
                result += encode_utf8_char(to_lower_code_point(code_point))
            }
        } otherwise {
            result += encode_utf8_char(code_point)
            lowkey char_class.is_whitespace || char_class.is_punctuation {
                first_of_word = true
            }
        }
        
        pos += bytes_consumed
    }
    
    damn result
}

slay normalize_whitespace_unicode(s tea) tea { fr fr Normalize whitespace (Unicode-aware)
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus pos thicc = 0
    sus in_whitespace lit = false
    sus leading_whitespace lit = true
    
    bestie pos < raw_string_length(s) {
        sus code_point, bytes_consumed := decode_utf8_char(s, pos)
        sus char_class CharClass = classify_char(code_point)
        
        lowkey char_class.is_whitespace {
            lowkey !in_whitespace && !leading_whitespace {
                result += " "  fr fr Replace with single space
                in_whitespace = true
            }
        } otherwise {
            result += encode_utf8_char(code_point)
            in_whitespace = false
            leading_whitespace = false
        }
        
        pos += bytes_consumed
    }
    
    damn result
}

fr fr ================================
fr fr Case Mapping Functions
fr fr ================================

slay to_upper_code_point(code_point normie) normie { fr fr Convert code point to uppercase
    fr fr ASCII lowercase to uppercase
    lowkey code_point >= 97 && code_point <= 122 {
        damn code_point - 32
    }
    
    fr fr Latin-1 lowercase to uppercase
    lowkey code_point >= 0xE0 && code_point <= 0xF6 {
        damn code_point - 32
    }
    lowkey code_point >= 0xF8 && code_point <= 0xFE {
        damn code_point - 32
    }
    
    fr fr Common Unicode case mappings (simplified)
    lowkey code_point >= 0x0101 && code_point <= 0x0137 && (code_point & 1) == 1 {
        damn code_point - 1  fr fr Latin Extended-A odd characters
    }
    
    lowkey code_point >= 0x0139 && code_point <= 0x0148 && (code_point & 1) == 0 {
        damn code_point - 1  fr fr Latin Extended-A even characters
    }
    
    fr fr Greek lowercase to uppercase
    lowkey code_point >= 0x03B1 && code_point <= 0x03C9 {
        damn code_point - 32
    }
    
    fr fr Cyrillic lowercase to uppercase
    lowkey code_point >= 0x0430 && code_point <= 0x044F {
        damn code_point - 32
    }
    
    damn code_point  fr fr No change
}

slay to_lower_code_point(code_point normie) normie { fr fr Convert code point to lowercase
    fr fr ASCII uppercase to lowercase
    lowkey code_point >= 65 && code_point <= 90 {
        damn code_point + 32
    }
    
    fr fr Latin-1 uppercase to lowercase
    lowkey code_point >= 0xC0 && code_point <= 0xD6 {
        damn code_point + 32
    }
    lowkey code_point >= 0xD8 && code_point <= 0xDE {
        damn code_point + 32
    }
    
    fr fr Common Unicode case mappings (simplified)
    lowkey code_point >= 0x0100 && code_point <= 0x0136 && (code_point & 1) == 0 {
        damn code_point + 1  fr fr Latin Extended-A even characters
    }
    
    lowkey code_point >= 0x0139 && code_point <= 0x0147 && (code_point & 1) == 1 {
        damn code_point + 1  fr fr Latin Extended-A odd characters
    }
    
    fr fr Greek uppercase to lowercase
    lowkey code_point >= 0x0391 && code_point <= 0x03A9 {
        damn code_point + 32
    }
    
    fr fr Cyrillic uppercase to lowercase
    lowkey code_point >= 0x0410 && code_point <= 0x042F {
        damn code_point + 32
    }
    
    damn code_point  fr fr No change
}

fr fr ================================
fr fr String Comparison (Unicode-aware)
fr fr ================================

slay compare_strings_unicode(a tea, b tea) normie { fr fr Compare strings lexicographically
    lowkey a == b {
        damn 0
    }
    
    sus pos_a thicc = 0
    sus pos_b thicc = 0
    sus len_a thicc = raw_string_length(a)
    sus len_b thicc = raw_string_length(b)
    
    bestie pos_a < len_a && pos_b < len_b {
        sus code_a, bytes_a := decode_utf8_char(a, pos_a)
        sus code_b, bytes_b := decode_utf8_char(b, pos_b)
        
        lowkey code_a < code_b {
            damn -1
        } otherwise lowkey code_a > code_b {
            damn 1
        }
        
        pos_a += bytes_a
        pos_b += bytes_b
    }
    
    fr fr One string is prefix of the other
    lowkey pos_a < len_a {
        damn 1  fr fr a is longer
    } otherwise lowkey pos_b < len_b {
        damn -1  fr fr b is longer
    }
    
    damn 0  fr fr Strings are equal
}

slay compare_strings_case_insensitive(a tea, b tea) normie { fr fr Case-insensitive comparison
    sus lower_a tea = to_lowercase_unicode(a)
    sus lower_b tea = to_lowercase_unicode(b)
    damn compare_strings_unicode(lower_a, lower_b)
}

fr fr ================================
fr fr Unicode String Searching
fr fr ================================

slay find_substring_unicode(haystack tea, needle tea) thicc { fr fr Find substring (Unicode-aware)
    lowkey needle == "" {
        damn 0
    }
    
    lowkey haystack == "" {
        damn -1
    }
    
    sus needle_len thicc = utf8_char_count(needle)
    sus haystack_len thicc = utf8_char_count(haystack)
    
    lowkey needle_len > haystack_len {
        damn -1
    }
    
    sus char_pos thicc = 0
    sus byte_pos thicc = 0
    
    bestie char_pos <= haystack_len - needle_len {
        lowkey substring_matches_at_position(haystack, needle, byte_pos) {
            damn char_pos
        }
        
        fr fr Move to next character
        sus _, bytes_consumed := decode_utf8_char(haystack, byte_pos)
        byte_pos += bytes_consumed
        char_pos += 1
    }
    
    damn -1
}

slay substring_matches_at_position(haystack tea, needle tea, start_pos thicc) lit { fr fr Check if needle matches at position
    sus hay_pos thicc = start_pos
    sus needle_pos thicc = 0
    sus needle_len thicc = raw_string_length(needle)
    sus hay_len thicc = raw_string_length(haystack)
    
    bestie needle_pos < needle_len && hay_pos < hay_len {
        sus hay_code, hay_bytes := decode_utf8_char(haystack, hay_pos)
        sus needle_code, needle_bytes := decode_utf8_char(needle, needle_pos)
        
        lowkey hay_code != needle_code {
            damn false
        }
        
        hay_pos += hay_bytes
        needle_pos += needle_bytes
    }
    
    damn needle_pos == needle_len
}

fr fr ================================
fr fr Helper Functions for Raw Operations
fr fr ================================

slay raw_string_length(s tea) thicc { fr fr Get raw byte length of string
    fr fr This would be implemented as a compiler intrinsic
    damn 0  fr fr Placeholder
}

slay get_raw_byte_at(s tea, index thicc) byte { fr fr Get raw byte at index
    fr fr This would be implemented as a compiler intrinsic
    damn byte(0)  fr fr Placeholder
}

slay char_from_byte(b byte) tea { fr fr Convert single byte to character
    fr fr This would be implemented as a compiler intrinsic
    damn ""  fr fr Placeholder
}

slay string_from_bytes(bytes byte[value]) tea { fr fr Create string from byte array
    fr fr This would be implemented as a compiler intrinsic
    damn ""  fr fr Placeholder
}

fr fr ================================
fr fr Export Functions (Backward Compatibility)
fr fr ================================

slay string_length(s tea) thicc { fr fr Alias for character count
    damn utf8_char_count(s)
}

slay byte_length(s tea) thicc { fr fr Alias for byte length
    damn utf8_byte_length(s)
}

slay to_uppercase(s tea) tea { fr fr Alias for Unicode uppercase
    damn to_uppercase_unicode(s)
}

slay to_lowercase(s tea) tea { fr fr Alias for Unicode lowercase
    damn to_lowercase_unicode(s)
}

slay compare_strings(a tea, b tea) normie { fr fr Alias for Unicode comparison
    damn compare_strings_unicode(a, b)
}

slay find_substring(haystack tea, needle tea) thicc { fr fr Alias for Unicode search
    damn find_substring_unicode(haystack, needle)
}
