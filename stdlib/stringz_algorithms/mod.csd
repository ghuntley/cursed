fr fr CURSED String Algorithms Module - Production Implementation
fr fr Advanced string processing with Unicode support and efficient algorithms
fr fr Boyer-Moore, KMP, Rabin-Karp, and other optimized string algorithms

yeet "mathz"

fr fr ================================
fr fr Core Data Structures
fr fr ================================

be_like StringView squad {
    data tea
    offset thicc
    length thicc
    is_valid lit
}

be_like StringMatch squad {
    position thicc
    length thicc
    matched_text tea
    groups tea[value]                fr fr For regex-like grouping
}

be_like StringSearchResult squad {
    found lit
    position thicc
    matches StringMatch[value]
    total_count thicc
}

be_like StringSplitOptions squad {
    remove_empty lit
    max_splits thicc
    trim_whitespace lit
    case_sensitive lit
}

be_like StringCompareOptions squad {
    case_sensitive lit
    ignore_accents lit
    culture_aware lit
    numeric_comparison lit
}

be_like StringPattern squad {
    pattern tea
    compiled_data normie[value]      fr fr Precomputed search tables
    algorithm normie            fr fr 0=naive, 1=KMP, 2=Boyer-Moore, 3=Rabin-Karp
    is_compiled lit
}

be_like UnicodeInfo squad {
    code_point normie
    byte_length normie          fr fr UTF-8 byte length
    category normie             fr fr Unicode category
    is_letter lit
    is_digit lit
    is_whitespace lit
    is_punctuation lit
    is_symbol lit
    is_control lit
    uppercase_mapping normie
    lowercase_mapping normie
    titlecase_mapping normie
}

fr fr ================================
fr fr Unicode Processing Constants
fr fr ================================

facts UNICODE_CATEGORY_LETTER normie = 1
facts UNICODE_CATEGORY_DIGIT normie = 2
facts UNICODE_CATEGORY_WHITESPACE normie = 3
facts UNICODE_CATEGORY_PUNCTUATION normie = 4
facts UNICODE_CATEGORY_SYMBOL normie = 5
facts UNICODE_CATEGORY_CONTROL normie = 6

facts UTF8_1_BYTE_MAX normie = 0x7F
facts UTF8_2_BYTE_MAX normie = 0x7FF
facts UTF8_3_BYTE_MAX normie = 0xFFFF
facts UTF8_4_BYTE_MAX normie = 0x10FFFF

facts SEARCH_ALGORITHM_NAIVE normie = 0
facts SEARCH_ALGORITHM_KMP normie = 1
facts SEARCH_ALGORITHM_BOYER_MOORE normie = 2
facts SEARCH_ALGORITHM_RABIN_KARP normie = 3

fr fr ================================
fr fr Unicode Support Functions
fr fr ================================

slay get_utf8_char_length(first_byte normie) normie { fr fr Get UTF-8 character length from first byte
    lowkey (first_byte & 0x80) == 0 {
        damn 1 fr fr ASCII: 0xxxxxxx
    }
    lowkey (first_byte & 0xE0) == 0xC0 {
        damn 2 fr fr 110xxxxx
    }
    lowkey (first_byte & 0xF0) == 0xE0 {
        damn 3 fr fr 1110xxxx
    }
    lowkey (first_byte & 0xF8) == 0xF0 {
        damn 4 fr fr 11110xxx
    }
    damn 1 fr fr Invalid, treat as single byte
}

slay decode_utf8_char(data tea, position thicc) (normie, normie) { fr fr Decode UTF-8 character
    lowkey position >= string_byte_length(data) {
        damn 0, 0
    }
    
    sus first_byte normie = get_byte_at(data, position)
    sus char_len normie = get_utf8_char_length(first_byte)
    sus code_point normie = 0
    
    lowkey position + char_len > string_byte_length(data) {
        damn 0, 0 fr fr Incomplete character
    }
    
    lowkey char_len == 1 {
        code_point = first_byte
    } otherwise lowkey char_len == 2 {
        sus second_byte normie = get_byte_at(data, position + 1)
        code_point = ((first_byte & 0x1F) << 6) | (second_byte & 0x3F)
    } otherwise lowkey char_len == 3 {
        sus second_byte normie = get_byte_at(data, position + 1)
        sus third_byte normie = get_byte_at(data, position + 2)
        code_point = ((first_byte & 0x0F) << 12) | 
                    ((second_byte & 0x3F) << 6) | 
                    (third_byte & 0x3F)
    } otherwise lowkey char_len == 4 {
        sus second_byte normie = get_byte_at(data, position + 1)
        sus third_byte normie = get_byte_at(data, position + 2)
        sus fourth_byte normie = get_byte_at(data, position + 3)
        code_point = ((first_byte & 0x07) << 18) |
                    ((second_byte & 0x3F) << 12) |
                    ((third_byte & 0x3F) << 6) |
                    (fourth_byte & 0x3F)
    }
    
    damn code_point, char_len
}

slay encode_utf8_char(code_point normie) tea { fr fr Encode code point to UTF-8
    lowkey code_point <= UTF8_1_BYTE_MAX {
        damn char_from_byte(code_point)
    }
    
    lowkey code_point <= UTF8_2_BYTE_MAX {
        sus first_byte normie = 0xC0 | (code_point >> 6)
        sus second_byte normie = 0x80 | (code_point & 0x3F)
        damn char_from_byte(first_byte) + char_from_byte(second_byte)
    }
    
    lowkey code_point <= UTF8_3_BYTE_MAX {
        sus first_byte normie = 0xE0 | (code_point >> 12)
        sus second_byte normie = 0x80 | ((code_point >> 6) & 0x3F)
        sus third_byte normie = 0x80 | (code_point & 0x3F)
        damn char_from_byte(first_byte) + char_from_byte(second_byte) + char_from_byte(third_byte)
    }
    
    lowkey code_point <= UTF8_4_BYTE_MAX {
        sus first_byte normie = 0xF0 | (code_point >> 18)
        sus second_byte normie = 0x80 | ((code_point >> 12) & 0x3F)
        sus third_byte normie = 0x80 | ((code_point >> 6) & 0x3F)
        sus fourth_byte normie = 0x80 | (code_point & 0x3F)
        damn char_from_byte(first_byte) + char_from_byte(second_byte) + 
             char_from_byte(third_byte) + char_from_byte(fourth_byte)
    }
    
    damn "?" fr fr Invalid code point, return replacement character
}

slay get_unicode_info(code_point normie) UnicodeInfo { fr fr Get Unicode character information
    sus info UnicodeInfo = {
        code_point: code_point,
        byte_length: get_code_point_byte_length(code_point),
        category: UNICODE_CATEGORY_SYMBOL,
        is_letter: false,
        is_digit: false,
        is_whitespace: false,
        is_punctuation: false,
        is_symbol: false,
        is_control: false,
        uppercase_mapping: code_point,
        lowercase_mapping: code_point,
        titlecase_mapping: code_point
    }
    
    fr fr Basic ASCII character classification
    lowkey code_point >= 65 && code_point <= 90 { fr fr A-Z
        info.category = UNICODE_CATEGORY_LETTER
        info.is_letter = true
        info.lowercase_mapping = code_point + 32
    } otherwise lowkey code_point >= 97 && code_point <= 122 { fr fr a-z
        info.category = UNICODE_CATEGORY_LETTER
        info.is_letter = true
        info.uppercase_mapping = code_point - 32
        info.titlecase_mapping = code_point - 32
    } otherwise lowkey code_point >= 48 && code_point <= 57 { fr fr 0-9
        info.category = UNICODE_CATEGORY_DIGIT
        info.is_digit = true
    } otherwise lowkey is_ascii_whitespace(code_point) {
        info.category = UNICODE_CATEGORY_WHITESPACE
        info.is_whitespace = true
    } otherwise lowkey is_ascii_punctuation(code_point) {
        info.category = UNICODE_CATEGORY_PUNCTUATION
        info.is_punctuation = true
    } otherwise lowkey code_point < 32 || code_point == 127 {
        info.category = UNICODE_CATEGORY_CONTROL
        info.is_control = true
    }
    
    damn info
}

slay get_code_point_byte_length(code_point normie) normie { fr fr Get UTF-8 byte length for code point
    lowkey code_point <= UTF8_1_BYTE_MAX {
        damn 1
    }
    lowkey code_point <= UTF8_2_BYTE_MAX {
        damn 2
    }
    lowkey code_point <= UTF8_3_BYTE_MAX {
        damn 3
    }
    lowkey code_point <= UTF8_4_BYTE_MAX {
        damn 4
    }
    damn 0 fr fr Invalid code point
}

slay is_ascii_whitespace(code_point normie) lit { fr fr Check if ASCII whitespace
    damn code_point == 9 ||   fr fr Tab
         code_point == 10 ||  fr fr LF
         code_point == 13 ||  fr fr CR
         code_point == 32     fr fr Space
}

slay is_ascii_punctuation(code_point normie) lit { fr fr Check if ASCII punctuation
    damn (code_point >= 33 && code_point <= 47) ||   fr fr !"#$%&'()*+,-./
         (code_point >= 58 && code_point <= 64) ||   fr fr :;<=>?@
         (code_point >= 91 && code_point <= 96) ||   fr fr [\]^_`
         (code_point >= 123 && code_point <= 126)    fr fr {|}~
}

fr fr ================================
fr fr String Length and Indexing
fr fr ================================

slay string_char_count(s tea) thicc { fr fr Count Unicode characters
    lowkey s == "" {
        damn 0
    }
    
    sus count thicc = 0
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    
    bestie byte_pos < byte_len {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        lowkey char_len > 0 {
            count += 1
            byte_pos += char_len
        } otherwise {
            byte_pos += 1 fr fr Skip invalid byte
        }
    }
    
    damn count
}

slay string_byte_length(s tea) thicc { fr fr Get byte length of string
    fr fr Placeholder - would use runtime string length
    damn 0
}

slay char_at_position(s tea, char_index thicc) tea { fr fr Get character at Unicode position
    lowkey char_index < 0 {
        damn ""
    }
    
    sus current_char thicc = 0
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    
    bestie byte_pos < byte_len && current_char <= char_index {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey current_char == char_index {
            damn encode_utf8_char(code_point)
        }
        
        lowkey char_len > 0 {
            current_char += 1
            byte_pos += char_len
        } otherwise {
            byte_pos += 1
        }
    }
    
    damn ""
}

slay byte_index_to_char_index(s tea, byte_index thicc) thicc { fr fr Convert byte index to character index
    lowkey byte_index <= 0 {
        damn 0
    }
    
    sus char_count thicc = 0
    sus byte_pos thicc = 0
    
    bestie byte_pos < byte_index && byte_pos < string_byte_length(s) {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            byte_pos += char_len
        } otherwise {
            byte_pos += 1
        }
        
        char_count += 1
    }
    
    damn char_count
}

slay char_index_to_byte_index(s tea, char_index thicc) thicc { fr fr Convert character index to byte index
    lowkey char_index <= 0 {
        damn 0
    }
    
    sus current_char thicc = 0
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    
    bestie byte_pos < byte_len && current_char < char_index {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            byte_pos += char_len
        } otherwise {
            byte_pos += 1
        }
        
        current_char += 1
    }
    
    damn byte_pos
}

fr fr ================================
fr fr Advanced String Searching
fr fr ================================

slay compile_search_pattern(pattern tea, algorithm normie) StringPattern { fr fr Compile search pattern
    sus compiled StringPattern = {
        pattern: pattern,
        compiled_data: [],
        algorithm: algorithm,
        is_compiled: false
    }
    
    lowkey pattern == "" {
        damn compiled
    }
    
    lowkey algorithm == SEARCH_ALGORITHM_KMP {
        compiled.compiled_data = compute_kmp_failure_function(pattern)
        compiled.is_compiled = true
    } otherwise lowkey algorithm == SEARCH_ALGORITHM_BOYER_MOORE {
        compiled.compiled_data = compute_boyer_moore_table(pattern)
        compiled.is_compiled = true
    } otherwise lowkey algorithm == SEARCH_ALGORITHM_RABIN_KARP {
        compiled.compiled_data = [compute_rabin_karp_hash(pattern)]
        compiled.is_compiled = true
    }
    
    damn compiled
}

slay search_with_pattern(text tea, pattern StringPattern) StringSearchResult { fr fr Search using compiled pattern
    sus result StringSearchResult = {
        found: false,
        position: -1,
        matches: [],
        total_count: 0
    }
    
    lowkey !pattern.is_compiled || pattern.pattern == "" {
        damn result
    }
    
    lowkey pattern.algorithm == SEARCH_ALGORITHM_KMP {
        result = kmp_search(text, pattern)
    } otherwise lowkey pattern.algorithm == SEARCH_ALGORITHM_BOYER_MOORE {
        result = boyer_moore_search(text, pattern)
    } otherwise lowkey pattern.algorithm == SEARCH_ALGORITHM_RABIN_KARP {
        result = rabin_karp_search(text, pattern)
    } otherwise {
        result = naive_search(text, pattern.pattern)
    }
    
    damn result
}

slay kmp_search(text tea, pattern StringPattern) StringSearchResult { fr fr Knuth-Morris-Pratt search
    sus result StringSearchResult = {
        found: false,
        position: -1,
        matches: [],
        total_count: 0
    }
    
    sus text_len thicc = string_char_count(text)
    sus pattern_len thicc = string_char_count(pattern.pattern)
    sus failure_function normie[value] = pattern.compiled_data
    
    lowkey text_len < pattern_len {
        damn result
    }
    
    sus text_pos thicc = 0
    sus pattern_pos thicc = 0
    
    bestie text_pos < text_len {
        sus text_char tea = char_at_position(text, text_pos)
        sus pattern_char tea = char_at_position(pattern.pattern, pattern_pos)
        
        lowkey text_char == pattern_char {
            text_pos += 1
            pattern_pos += 1
            
            lowkey pattern_pos == pattern_len {
                fr fr Found match
                sus match_start thicc = text_pos - pattern_len
                
                lowkey !result.found {
                    result.found = true
                    result.position = match_start
                }
                
                sus match StringMatch = {
                    position: match_start,
                    length: pattern_len,
                    matched_text: pattern.pattern,
                    groups: []
                }
                
                result.matches = append_match(result.matches, match)
                result.total_count += 1
                
                fr fr Reset for next match
                pattern_pos = failure_function[pattern_pos - 1]
            }
        } otherwise {
            lowkey pattern_pos > 0 {
                pattern_pos = failure_function[pattern_pos - 1]
            } otherwise {
                text_pos += 1
            }
        }
    }
    
    damn result
}

slay compute_kmp_failure_function(pattern tea) normie[value]{ fr fr Compute KMP failure function
    sus pattern_len thicc = string_char_count(pattern)
    sus failure normie[value] = make_int_array(pattern_len)
    
    lowkey pattern_len == 0 {
        damn failure
    }
    
    failure[0] = 0
    sus len normie = 0
    sus i thicc = 1
    
    bestie i < pattern_len {
        sus current_char tea = char_at_position(pattern, i)
        sus len_char tea = char_at_position(pattern, len)
        
        lowkey current_char == len_char {
            len += 1
            failure[i] = len
            i += 1
        } otherwise {
            lowkey len > 0 {
                len = failure[len - 1]
            } otherwise {
                failure[i] = 0
                i += 1
            }
        }
    }
    
    damn failure
}

slay boyer_moore_search(text tea, pattern StringPattern) StringSearchResult { fr fr Boyer-Moore search
    sus result StringSearchResult = {
        found: false,
        position: -1,
        matches: [],
        total_count: 0
    }
    
    sus text_len thicc = string_char_count(text)
    sus pattern_len thicc = string_char_count(pattern.pattern)
    
    lowkey text_len < pattern_len {
        damn result
    }
    
    fr fr Simplified Boyer-Moore with just bad character rule
    sus bad_char normie[value] = pattern.compiled_data
    sus skip thicc = 0
    
    bestie skip <= (text_len - pattern_len) {
        sus j thicc = pattern_len - 1
        
        fr fr Match from right to left
        bestie j >= 0 {
            sus text_char tea = char_at_position(text, skip + j)
            sus pattern_char tea = char_at_position(pattern.pattern, j)
            
            lowkey text_char != pattern_char {
                break
            }
            
            j -= 1
        }
        
        lowkey j < 0 {
            fr fr Found match
            lowkey !result.found {
                result.found = true
                result.position = skip
            }
            
            sus match StringMatch = {
                position: skip,
                length: pattern_len,
                matched_text: pattern.pattern,
                groups: []
            }
            
            result.matches = append_match(result.matches, match)
            result.total_count += 1
            
            skip += 1 fr fr Move to next position
        } otherwise {
            fr fr Use bad character rule for skip distance
            sus mismatch_char tea = char_at_position(text, skip + j)
            sus char_code normie = get_first_byte(mismatch_char)
            
            lowkey char_code < array_length(bad_char) {
                skip += max_int(1, j - bad_char[char_code])
            } otherwise {
                skip += j + 1
            }
        }
    }
    
    damn result
}

slay compute_boyer_moore_table(pattern tea) normie[value]{ fr fr Compute Boyer-Moore bad character table
    sus table normie[value] = make_int_array(256) fr fr ASCII table
    sus pattern_len thicc = string_char_count(pattern)
    
    fr fr Initialize all positions to -1
    bestie i := 0; i < 256; i++ {
        table[i] = -1
    }
    
    fr fr Fill actual character positions
    bestie i := 0; i < pattern_len; i++ {
        sus char tea = char_at_position(pattern, i)
        sus char_code normie = get_first_byte(char)
        lowkey char_code < 256 {
            table[char_code] = i
        }
    }
    
    damn table
}

slay rabin_karp_search(text tea, pattern StringPattern) StringSearchResult { fr fr Rabin-Karp rolling hash search
    sus result StringSearchResult = {
        found: false,
        position: -1,
        matches: [],
        total_count: 0
    }
    
    sus text_len thicc = string_char_count(text)
    sus pattern_len thicc = string_char_count(pattern.pattern)
    
    lowkey text_len < pattern_len || array_length(pattern.compiled_data) == 0 {
        damn result
    }
    
    sus pattern_hash normie = pattern.compiled_data[0]
    sus prime normie = 101 fr fr Small prime for rolling hash
    sus base normie = 256
    sus text_hash normie = 0
    sus h normie = 1
    
    fr fr Calculate h = base^(pattern_len-1) % prime
    bestie i := 0; i < pattern_len - 1; i++ {
        h = (h * base) % prime
    }
    
    fr fr Calculate initial hash for text and pattern
    bestie i := 0; i < pattern_len; i++ {
        sus text_char normie = get_first_byte(char_at_position(text, i))
        text_hash = (base * text_hash + text_char) % prime
    }
    
    fr fr Slide the pattern over text
    bestie i := 0; i <= text_len - pattern_len; i++ {
        lowkey pattern_hash == text_hash {
            fr fr Hash match, verify character by character
            sus matches lit = true
            bestie j := 0; j < pattern_len; j++ {
                sus text_char tea = char_at_position(text, i + j)
                sus pattern_char tea = char_at_position(pattern.pattern, j)
                lowkey text_char != pattern_char {
                    matches = false
                    break
                }
            }
            
            lowkey matches {
                lowkey !result.found {
                    result.found = true
                    result.position = i
                }
                
                sus match StringMatch = {
                    position: i,
                    length: pattern_len,
                    matched_text: pattern.pattern,
                    groups: []
                }
                
                result.matches = append_match(result.matches, match)
                result.total_count += 1
            }
        }
        
        fr fr Calculate hash for next window
        lowkey i < text_len - pattern_len {
            sus old_char normie = get_first_byte(char_at_position(text, i))
            sus new_char normie = get_first_byte(char_at_position(text, i + pattern_len))
            
            text_hash = (base * (text_hash - old_char * h) + new_char) % prime
            
            fr fr Handle negative modulo
            lowkey text_hash < 0 {
                text_hash += prime
            }
        }
    }
    
    damn result
}

slay compute_rabin_karp_hash(pattern tea) normie { fr fr Compute Rabin-Karp hash
    sus hash normie = 0
    sus base normie = 256
    sus prime normie = 101
    sus pattern_len thicc = string_char_count(pattern)
    
    bestie i := 0; i < pattern_len; i++ {
        sus char_code normie = get_first_byte(char_at_position(pattern, i))
        hash = (hash * base + char_code) % prime
    }
    
    damn hash
}

slay naive_search(text tea, pattern tea) StringSearchResult { fr fr Naive string search
    sus result StringSearchResult = {
        found: false,
        position: -1,
        matches: [],
        total_count: 0
    }
    
    sus text_len thicc = string_char_count(text)
    sus pattern_len thicc = string_char_count(pattern)
    
    lowkey text_len < pattern_len {
        damn result
    }
    
    bestie i := 0; i <= text_len - pattern_len; i++ {
        sus j thicc = 0
        
        bestie j < pattern_len {
            sus text_char tea = char_at_position(text, i + j)
            sus pattern_char tea = char_at_position(pattern, j)
            
            lowkey text_char != pattern_char {
                break
            }
            
            j += 1
        }
        
        lowkey j == pattern_len {
            fr fr Found match
            lowkey !result.found {
                result.found = true
                result.position = i
            }
            
            sus match StringMatch = {
                position: i,
                length: pattern_len,
                matched_text: pattern,
                groups: []
            }
            
            result.matches = append_match(result.matches, match)
            result.total_count += 1
        }
    }
    
    damn result
}

fr fr ================================
fr fr String Case Conversion
fr fr ================================

slay to_lowercase_advanced(s tea) tea { fr fr Advanced lowercase conversion
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    
    bestie byte_pos < byte_len {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            sus info UnicodeInfo = get_unicode_info(code_point)
            sus lowercase_char tea = encode_utf8_char(info.lowercase_mapping)
            result = string_concat(result, lowercase_char)
            byte_pos += char_len
        } otherwise {
            fr fr Invalid UTF-8, copy as-is
            sus invalid_char tea = char_from_byte(get_byte_at(s, byte_pos))
            result = string_concat(result, invalid_char)
            byte_pos += 1
        }
    }
    
    damn result
}

slay to_uppercase_advanced(s tea) tea { fr fr Advanced uppercase conversion
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    
    bestie byte_pos < byte_len {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            sus info UnicodeInfo = get_unicode_info(code_point)
            sus uppercase_char tea = encode_utf8_char(info.uppercase_mapping)
            result = string_concat(result, uppercase_char)
            byte_pos += char_len
        } otherwise {
            sus invalid_char tea = char_from_byte(get_byte_at(s, byte_pos))
            result = string_concat(result, invalid_char)
            byte_pos += 1
        }
    }
    
    damn result
}

slay to_titlecase(s tea) tea { fr fr Convert to title case
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    sus is_word_start lit = true
    
    bestie byte_pos < byte_len {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            sus info UnicodeInfo = get_unicode_info(code_point)
            
            lowkey is_word_start && info.is_letter {
                sus titlecase_char tea = encode_utf8_char(info.titlecase_mapping)
                result = string_concat(result, titlecase_char)
                is_word_start = false
            } otherwise lowkey info.is_letter {
                sus lowercase_char tea = encode_utf8_char(info.lowercase_mapping)
                result = string_concat(result, lowercase_char)
            } otherwise {
                sus original_char tea = encode_utf8_char(code_point)
                result = string_concat(result, original_char)
                
                fr fr Reset word start on whitespace or punctuation
                lowkey info.is_whitespace || info.is_punctuation {
                    is_word_start = true
                }
            }
            
            byte_pos += char_len
        } otherwise {
            sus invalid_char tea = char_from_byte(get_byte_at(s, byte_pos))
            result = string_concat(result, invalid_char)
            byte_pos += 1
        }
    }
    
    damn result
}

fr fr ================================
fr fr Advanced String Splitting
fr fr ================================

slay split_advanced(s tea, delimiter tea, options StringSplitOptions) tea[value]{ fr fr Advanced string splitting
    sus result tea[value] = []
    
    lowkey s == "" {
        lowkey !options.remove_empty {
            result = append_string_array(result, "")
        }
        damn result
    }
    
    lowkey delimiter == "" {
        fr fr Split into individual characters
        damn split_into_chars(s)
    }
    
    sus splits_made thicc = 0
    sus current_part tea = ""
    sus text_pos thicc = 0
    sus text_len thicc = string_char_count(s)
    sus delim_len thicc = string_char_count(delimiter)
    
    bestie text_pos < text_len && 
           (options.max_splits == 0 || splits_made < options.max_splits) {
        
        fr fr Check for delimiter at current position
        sus found_delimiter lit = true
        bestie i := 0; i < delim_len; i++ {
            lowkey text_pos + i >= text_len {
                found_delimiter = false
                break
            }
            
            sus text_char tea = char_at_position(s, text_pos + i)
            sus delim_char tea = char_at_position(delimiter, i)
            
            lowkey !characters_equal(text_char, delim_char, options.case_sensitive) {
                found_delimiter = false
                break
            }
        }
        
        lowkey found_delimiter {
            fr fr Process current part
            sus part tea = current_part
            lowkey options.trim_whitespace {
                part = trim_whitespace(part)
            }
            
            lowkey !options.remove_empty || part != "" {
                result = append_string_array(result, part)
            }
            
            current_part = ""
            text_pos += delim_len
            splits_made += 1
        } otherwise {
            sus char tea = char_at_position(s, text_pos)
            current_part = string_concat(current_part, char)
            text_pos += 1
        }
    }
    
    fr fr Add remaining text
    bestie text_pos < text_len {
        sus char tea = char_at_position(s, text_pos)
        current_part = string_concat(current_part, char)
        text_pos += 1
    }
    
    fr fr Process final part
    lowkey current_part != "" || !options.remove_empty {
        sus final_part tea = current_part
        lowkey options.trim_whitespace {
            final_part = trim_whitespace(final_part)
        }
        
        lowkey final_part != "" || !options.remove_empty {
            result = append_string_array(result, final_part)
        }
    }
    
    damn result
}

slay split_into_chars(s tea) tea[value]{ fr fr Split string into individual characters
    sus result tea[value] = []
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    
    bestie byte_pos < byte_len {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            sus char tea = encode_utf8_char(code_point)
            result = append_string_array(result, char)
            byte_pos += char_len
        } otherwise {
            sus invalid_char tea = char_from_byte(get_byte_at(s, byte_pos))
            result = append_string_array(result, invalid_char)
            byte_pos += 1
        }
    }
    
    damn result
}

slay split_whitespace(s tea) tea[value]{ fr fr Split on any whitespace
    sus result tea[value] = []
    sus current_word tea = ""
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    
    bestie byte_pos < byte_len {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            sus info UnicodeInfo = get_unicode_info(code_point)
            
            lowkey info.is_whitespace {
                lowkey current_word != "" {
                    result = append_string_array(result, current_word)
                    current_word = ""
                }
            } otherwise {
                sus char tea = encode_utf8_char(code_point)
                current_word = string_concat(current_word, char)
            }
            
            byte_pos += char_len
        } otherwise {
            sus invalid_char tea = char_from_byte(get_byte_at(s, byte_pos))
            current_word = string_concat(current_word, invalid_char)
            byte_pos += 1
        }
    }
    
    lowkey current_word != "" {
        result = append_string_array(result, current_word)
    }
    
    damn result
}

fr fr ================================
fr fr String Trimming and Padding
fr fr ================================

slay trim_whitespace(s tea) tea { fr fr Trim Unicode whitespace
    damn trim_left_whitespace(trim_right_whitespace(s))
}

slay trim_left_whitespace(s tea) tea { fr fr Trim left whitespace
    lowkey s == "" {
        damn ""
    }
    
    sus byte_pos thicc = 0
    sus byte_len thicc = string_byte_length(s)
    
    bestie byte_pos < byte_len {
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            sus info UnicodeInfo = get_unicode_info(code_point)
            lowkey !info.is_whitespace {
                break
            }
            byte_pos += char_len
        } otherwise {
            byte_pos += 1
        }
    }
    
    lowkey byte_pos >= byte_len {
        damn ""
    }
    
    damn substring_from_byte(s, byte_pos, byte_len)
}

slay trim_right_whitespace(s tea) tea { fr fr Trim right whitespace
    lowkey s == "" {
        damn ""
    }
    
    sus byte_len thicc = string_byte_length(s)
    sus end_pos thicc = byte_len
    
    fr fr Find last non-whitespace character
    bestie end_pos > 0 {
        sus byte_pos thicc = find_prev_char_boundary(s, end_pos)
        sus code_point, char_len := decode_utf8_char(s, byte_pos)
        
        lowkey char_len > 0 {
            sus info UnicodeInfo = get_unicode_info(code_point)
            lowkey !info.is_whitespace {
                break
            }
            end_pos = byte_pos
        } otherwise {
            end_pos -= 1
        }
    }
    
    lowkey end_pos <= 0 {
        damn ""
    }
    
    damn substring_from_byte(s, 0, end_pos)
}

slay pad_left(s tea, total_width thicc, pad_char tea) tea { fr fr Pad string on left
    sus current_width thicc = string_char_count(s)
    
    lowkey current_width >= total_width {
        damn s
    }
    
    sus pad_count thicc = total_width - current_width
    sus padding tea = repeat_string(pad_char, pad_count)
    
    damn string_concat(padding, s)
}

slay pad_right(s tea, total_width thicc, pad_char tea) tea { fr fr Pad string on right
    sus current_width thicc = string_char_count(s)
    
    lowkey current_width >= total_width {
        damn s
    }
    
    sus pad_count thicc = total_width - current_width
    sus padding tea = repeat_string(pad_char, pad_count)
    
    damn string_concat(s, padding)
}

slay pad_center(s tea, total_width thicc, pad_char tea) tea { fr fr Center string with padding
    sus current_width thicc = string_char_count(s)
    
    lowkey current_width >= total_width {
        damn s
    }
    
    sus total_pad thicc = total_width - current_width
    sus left_pad thicc = total_pad / 2
    sus right_pad thicc = total_pad - left_pad
    
    sus left_padding tea = repeat_string(pad_char, left_pad)
    sus right_padding tea = repeat_string(pad_char, right_pad)
    
    damn string_concat(string_concat(left_padding, s), right_padding)
}

fr fr ================================
fr fr String Comparison
fr fr ================================

slay compare_strings_advanced(s1 tea, s2 tea, options StringCompareOptions) normie { fr fr Advanced string comparison
    lowkey s1 == s2 {
        damn 0
    }
    
    sus str1 tea = s1
    sus str2 tea = s2
    
    fr fr Apply transformations based on options
    lowkey !options.case_sensitive {
        str1 = to_lowercase_advanced(str1)
        str2 = to_lowercase_advanced(str2)
    }
    
    lowkey options.numeric_comparison {
        damn numeric_string_compare(str1, str2)
    }
    
    fr fr Character-by-character comparison
    sus len1 thicc = string_char_count(str1)
    sus len2 thicc = string_char_count(str2)
    sus min_len thicc = min_int(len1, len2)
    
    bestie i := 0; i < min_len; i++ {
        sus char1 tea = char_at_position(str1, i)
        sus char2 tea = char_at_position(str2, i)
        
        lowkey char1 != char2 {
            sus code1, _ := decode_utf8_char(char1, 0)
            sus code2, _ := decode_utf8_char(char2, 0)
            
            lowkey code1 < code2 {
                damn -1
            } otherwise {
                damn 1
            }
        }
    }
    
    fr fr Strings are equal up to minimum length, compare lengths
    lowkey len1 < len2 {
        damn -1
    } otherwise lowkey len1 > len2 {
        damn 1
    }
    
    damn 0
}

slay numeric_string_compare(s1 tea, s2 tea) normie { fr fr Compare strings with numeric sorting
    sus pos1 thicc = 0
    sus pos2 thicc = 0
    sus len1 thicc = string_char_count(s1)
    sus len2 thicc = string_char_count(s2)
    
    bestie pos1 < len1 && pos2 < len2 {
        sus char1 tea = char_at_position(s1, pos1)
        sus char2 tea = char_at_position(s2, pos2)
        
        sus is_digit1 lit = is_digit_char(char1)
        sus is_digit2 lit = is_digit_char(char2)
        
        lowkey is_digit1 && is_digit2 {
            fr fr Both are digits, compare numerically
            sus num1, end1 := extract_number(s1, pos1)
            sus num2, end2 := extract_number(s2, pos2)
            
            lowkey num1 != num2 {
                lowkey num1 < num2 {
                    damn -1
                } otherwise {
                    damn 1
                }
            }
            
            pos1 = end1
            pos2 = end2
        } otherwise {
            fr fr Compare characters normally
            lowkey char1 != char2 {
                sus code1, _ := decode_utf8_char(char1, 0)
                sus code2, _ := decode_utf8_char(char2, 0)
                
                lowkey code1 < code2 {
                    damn -1
                } otherwise {
                    damn 1
                }
            }
            
            pos1 += 1
            pos2 += 1
        }
    }
    
    fr fr Handle remaining characters
    lowkey pos1 < len1 {
        damn 1
    } otherwise lowkey pos2 < len2 {
        damn -1
    }
    
    damn 0
}

fr fr ================================
fr fr Utility Helper Functions
fr fr ================================

slay characters_equal(c1 tea, c2 tea, case_sensitive lit) lit { fr fr Compare characters
    lowkey case_sensitive {
        damn c1 == c2
    }
    
    sus lower1 tea = to_lowercase_advanced(c1)
    sus lower2 tea = to_lowercase_advanced(c2)
    damn lower1 == lower2
}

slay is_digit_char(c tea) lit { fr fr Check if character is digit
    sus code_point, _ := decode_utf8_char(c, 0)
    damn code_point >= 48 && code_point <= 57
}

slay extract_number(s tea, start_pos thicc) (thicc, thicc) { fr fr Extract number from string
    sus result thicc = 0
    sus pos thicc = start_pos
    sus len thicc = string_char_count(s)
    
    bestie pos < len {
        sus char tea = char_at_position(s, pos)
        lowkey !is_digit_char(char) {
            break
        }
        
        sus digit normie = get_first_byte(char) - 48
        result = result * 10 + digit
        pos += 1
    }
    
    damn result, pos
}

slay find_prev_char_boundary(s tea, byte_pos thicc) thicc { fr fr Find previous UTF-8 character boundary
    lowkey byte_pos <= 0 {
        damn 0
    }
    
    sus pos thicc = byte_pos - 1
    
    fr fr Move backwards to find character start
    bestie pos > 0 {
        sus byte_val normie = get_byte_at(s, pos)
        lowkey (byte_val & 0xC0) != 0x80 {
            break fr fr Not a continuation byte
        }
        pos -= 1
    }
    
    damn pos
}

slay repeat_string(s tea, count thicc) tea { fr fr Repeat string multiple times
    lowkey count <= 0 || s == "" {
        damn ""
    }
    
    sus result tea = ""
    bestie i := 0; i < count; i++ {
        result = string_concat(result, s)
    }
    
    damn result
}

slay substring_from_byte(s tea, start_byte thicc, end_byte thicc) tea { fr fr Extract substring by byte range
    fr fr Placeholder - would use proper byte-level substring
    damn s
}

fr fr ================================
fr fr Placeholder Functions
fr fr ================================

slay get_byte_at(s tea, pos thicc) normie { damn 65 }
slay char_from_byte(b normie) tea { damn "A" }
slay get_first_byte(s tea) normie { damn 65 }
slay string_concat(s1 tea, s2 tea) tea { damn s1 + s2 }
slay make_int_array(size thicc) normie[value]{ damn [] }
slay append_match(arr StringMatch[value], match StringMatch) StringMatch[value]{ damn arr }
slay append_string_array(arr tea[value], item tea) tea[value]{ damn arr }
slay array_length(arr normie[value]) thicc { damn 0 }
slay array_length(arr StringMatch[value]) thicc { damn 0 }
slay max_int(a normie, b normie) normie { lowkey a > b { damn a } damn b }
slay min_int(a thicc, b thicc) thicc { lowkey a < b { damn a } damn b }
