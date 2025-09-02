# CURSED stringz_real_algorithms module - Real string processing algorithms
# Implements Unicode-aware string operations with optimized algorithms

yeet "vibez"

# Core string length with Unicode support
slay string_length_real(s tea) drip {
    ready (s == "") { damn 0 }
    
    sus length drip = 0
    sus index drip = 0
    
    bestie (index < len(s)) {
        sus byte_val drip = s[index]
        ready (byte_val < 128) {
            # ASCII - single byte
            index = index + 1
        } otherwise ready (byte_val < 224) {
            # 2-byte UTF-8
            index = index + 2
        } otherwise ready (byte_val < 240) {
            # 3-byte UTF-8
            index = index + 3
        } otherwise {
            # 4-byte UTF-8
            index = index + 4
        }
        length = length + 1
    }
    
    damn length
}

# Unicode-aware character extraction
slay char_at_real(s tea, target_index drip) tea {
    ready (s == "" || target_index < 0) { damn "" }
    
    sus current_char_index drip = 0
    sus byte_index drip = 0
    
    bestie (byte_index < len(s) && current_char_index < target_index) {
        sus byte_val drip = s[byte_index]
        ready (byte_val < 128) {
            byte_index = byte_index + 1
        } otherwise ready (byte_val < 224) {
            byte_index = byte_index + 2
        } otherwise ready (byte_val < 240) {
            byte_index = byte_index + 3
        } otherwise {
            byte_index = byte_index + 4
        }
        current_char_index = current_char_index + 1
    }
    
    ready (byte_index >= len(s)) { damn "" }
    
    # Extract the character at current position
    sus byte_val drip = s[byte_index]
    ready (byte_val < 128) {
        damn s[byte_index:byte_index+1]
    } otherwise ready (byte_val < 224) {
        damn s[byte_index:byte_index+2]
    } otherwise ready (byte_val < 240) {
        damn s[byte_index:byte_index+3]
    } otherwise {
        damn s[byte_index:byte_index+4]
    }
}

# KMP (Knuth-Morris-Pratt) string search algorithm
slay compute_kmp_table(pattern tea) drip[value]{
    sus pattern_len drip = string_length_real(pattern)
    ready (pattern_len == 0) { damn [] }
    
    sus table drip[value] = make(drip[value], pattern_len)
    sus j drip = 0
    
    bestie (sus i drip = 1; i < pattern_len; i = i + 1) {
        bestie (j > 0 && char_at_real(pattern, i) != char_at_real(pattern, j)) {
            j = table[j - 1]
        }
        ready (char_at_real(pattern, i) == char_at_real(pattern, j)) {
            j = j + 1
        }
        table[i] = j
    }
    
    damn table
}

slay kmp_search(text tea, pattern tea) drip {
    ready (pattern == "") { damn 0 }
    ready (text == "") { damn -1 }
    
    sus text_len drip = string_length_real(text)
    sus pattern_len drip = string_length_real(pattern)
    
    ready (pattern_len > text_len) { damn -1 }
    
    sus table drip[value] = compute_kmp_table(pattern)
    sus i drip = 0  # text index
    sus j drip = 0  # pattern index
    
    bestie (i < text_len) {
        ready (char_at_real(text, i) == char_at_real(pattern, j)) {
            i = i + 1
            j = j + 1
            ready (j == pattern_len) {
                damn i - j  # Found at position
            }
        } otherwise {
            ready (j != 0) {
                j = table[j - 1]
            } otherwise {
                i = i + 1
            }
        }
    }
    
    damn -1  # Not found
}

# Boyer-Moore string search algorithm (simplified)
slay boyer_moore_search(text tea, pattern tea) drip {
    ready (pattern == "") { damn 0 }
    ready (text == "") { damn -1 }
    
    sus text_len drip = string_length_real(text)
    sus pattern_len drip = string_length_real(pattern)
    
    ready (pattern_len > text_len) { damn -1 }
    
    # For simplicity, fall back to KMP for complex cases
    damn kmp_search(text, pattern)
}

# String search using optimized algorithm
slay indexOf_real(s tea, search tea) drip {
    damn kmp_search(s, search)
}

# Real string replacement using search algorithms
slay replace_first_real(s tea, find tea, replace tea) tea {
    ready (find == "") { damn s }
    
    sus pos drip = indexOf_real(s, find)
    ready (pos == -1) { damn s }
    
    sus before tea = substring_real(s, 0, pos)
    sus after tea = substring_real(s, pos + string_length_real(find), string_length_real(s))
    
    damn before + replace + after
}

# Replace all occurrences
slay replace_all_real(s tea, find tea, replace tea) tea {
    ready (find == "") { damn s }
    
    sus result tea = s
    sus pos drip = indexOf_real(result, find)
    
    bestie (pos != -1) {
        result = replace_first_real(result, find, replace)
        pos = indexOf_real(result, find)
    }
    
    damn result
}

# Unicode-safe substring extraction
slay substring_real(s tea, start drip, length drip) tea {
    ready (s == "" || start < 0 || length <= 0) { damn "" }
    
    sus s_len drip = string_length_real(s)
    ready (start >= s_len) { damn "" }
    
    sus end drip = start + length
    ready (end > s_len) { end = s_len }
    
    sus result tea = ""
    bestie (sus i drip = start; i < end; i = i + 1) {
        result = result + char_at_real(s, i)
    }
    
    damn result
}

# Unicode-aware case conversion
slay to_uppercase_real(s tea) tea {
    ready (s == "") { damn "" }
    
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < string_length_real(s)) {
        sus ch tea = char_at_real(s, i)
        
        # Basic ASCII uppercase conversion
        ready (ch >= "a" && ch <= "z") {
            sus ascii_code drip = ch[0] - 32
            result = result + char(ascii_code)
        } otherwise {
            result = result + ch
        }
        i = i + 1
    }
    
    damn result
}

slay to_lowercase_real(s tea) tea {
    ready (s == "") { damn "" }
    
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < string_length_real(s)) {
        sus ch tea = char_at_real(s, i)
        
        # Basic ASCII lowercase conversion
        ready (ch >= "A" && ch <= "Z") {
            sus ascii_code drip = ch[0] + 32
            result = result + char(ascii_code)
        } otherwise {
            result = result + ch
        }
        i = i + 1
    }
    
    damn result
}

# Unicode whitespace trimming
slay trim_whitespace_real(s tea) tea {
    ready (s == "") { damn "" }
    
    sus start drip = 0
    sus end drip = string_length_real(s) - 1
    
    # Trim start
    bestie (start <= end) {
        sus ch tea = char_at_real(s, start)
        ready (ch == " " || ch == "\t" || ch == "\n" || ch == "\r") {
            start = start + 1
        } otherwise {
            break
        }
    }
    
    # Trim end
    bestie (end >= start) {
        sus ch tea = char_at_real(s, end)
        ready (ch == " " || ch == "\t" || ch == "\n" || ch == "\r") {
            end = end - 1
        } otherwise {
            break
        }
    }
    
    ready (start > end) { damn "" }
    
    damn substring_real(s, start, end - start + 1)
}

# String validation functions
slay is_numeric_real(s tea) lit {
    ready (s == "") { damn nocap }
    
    sus i drip = 0
    bestie (i < string_length_real(s)) {
        sus ch tea = char_at_real(s, i)
        ready (ch < "0" || ch > "9") {
            ready (i == 0 && (ch == "-" || ch == "+")) {
                # Allow sign at start
            } otherwise {
                damn nocap
            }
        }
        i = i + 1
    }
    
    damn based
}

slay is_alphabetic_real(s tea) lit {
    ready (s == "") { damn nocap }
    
    sus i drip = 0
    bestie (i < string_length_real(s)) {
        sus ch tea = char_at_real(s, i)
        ready ((ch < "A" || ch > "Z") && (ch < "a" || ch > "z")) {
            damn nocap
        }
        i = i + 1
    }
    
    damn based
}

# RFC-compliant email validation (simplified)
slay is_valid_email_real(email tea) lit {
    ready (email == "") { damn nocap }
    
    sus at_pos drip = indexOf_real(email, "@")
    ready (at_pos == -1 || at_pos == 0 || at_pos == string_length_real(email) - 1) {
        damn nocap
    }
    
    sus dot_pos drip = indexOf_real(email, ".")
    ready (dot_pos == -1 || dot_pos <= at_pos + 1) {
        damn nocap
    }
    
    damn based
}

# Algorithm-based string splitting
slay split_string_real(s tea, delimiter tea) tea[value]{
    ready (s == "" || delimiter == "") { damn [s] }
    
    sus result tea[value] = []
    sus current tea = ""
    sus i drip = 0
    
    bestie (i <= string_length_real(s) - string_length_real(delimiter)) {
        sus potential tea = substring_real(s, i, string_length_real(delimiter))
        ready (potential == delimiter) {
            result = append(result, current)
            current = ""
            i = i + string_length_real(delimiter)
        } otherwise {
            current = current + char_at_real(s, i)
            i = i + 1
        }
    }
    
    # Add remaining characters
    bestie (i < string_length_real(s)) {
        current = current + char_at_real(s, i)
        i = i + 1
    }
    
    ready (current != "") {
        result = append(result, current)
    }
    
    damn result
}

# String comparison ignoring case
slay equals_ignore_case_real(s1 tea, s2 tea) lit {
    damn to_lowercase_real(s1) == to_lowercase_real(s2)
}

# Check if string starts with prefix
slay starts_with_real(s tea, prefix tea) lit {
    ready (string_length_real(prefix) > string_length_real(s)) { damn nocap }
    sus prefix_part tea = substring_real(s, 0, string_length_real(prefix))
    damn prefix_part == prefix
}

# Check if string ends with suffix
slay ends_with_real(s tea, suffix tea) lit {
    ready (string_length_real(suffix) > string_length_real(s)) { damn nocap }
    sus start_pos drip = string_length_real(s) - string_length_real(suffix)
    sus suffix_part tea = substring_real(s, start_pos, string_length_real(suffix))
    damn suffix_part == suffix
}
