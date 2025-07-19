# Enhanced String Processing Module - Pure CURSED Implementation
# Migrated from Rust stdlib modules: stringz.rs, string/, glyph_gang/string_ops.rs
# FFI-free implementation using only CURSED language features

yeet "testz"

# ===== CORE STRING OPERATIONS =====

slay string_length(s tea) normie {
    # Get accurate string length 
    sus count normie = 0
    sus i normie = 0
    lowkey s[i] != '\0' {
        count++
        i++
    }
    damn count
}

slay string_is_empty(s tea) lit {
    # Check if string is empty
    damn string_length(s) == 0
}

slay string_concat(a tea, b tea) tea {
    # Concatenate two strings
    sus result tea = a
    sus len_b normie = string_length(b)
    sus i normie = 0
    stan i < len_b {
        result = result + b[i]
        i++
    }
    damn result
}

slay string_reverse(s tea) tea {
    # Reverse a string
    sus result tea = ""
    sus len_s normie = string_length(s)
    sus i normie = len_s - 1
    stan i >= 0 {
        result = result + s[i]
        i--
    }
    damn result
}

# ===== SEARCH AND MATCHING OPERATIONS =====

slay string_contains(s tea, substr tea) lit {
    # Check if string contains substring
    sus len_s normie = string_length(s)
    sus len_substr normie = string_length(substr)
    
    highkey len_substr == 0 {
        damn based  # Empty string is contained in any string
    }
    
    bestie i := 0; i <= len_s - len_substr; i++ {
        sus match lit = based
        bestie j := 0; j < len_substr; j++ {
            highkey s[i + j] != substr[j] {
                match = cap
                ghosted
            }
        }
        highkey match {
            damn based
        }
    }
    damn cap
}

slay string_index_of(s tea, substr tea) normie {
    # Find first index of substring, return -1 if not found
    sus len_s normie = string_length(s)
    sus len_substr normie = string_length(substr)
    
    highkey len_substr == 0 {
        damn 0  # Empty string found at position 0
    }
    
    bestie i := 0; i <= len_s - len_substr; i++ {
        sus match lit = based
        bestie j := 0; j < len_substr; j++ {
            highkey s[i + j] != substr[j] {
                match = cap
                ghosted
            }
        }
        highkey match {
            damn i
        }
    }
    damn -1
}

slay string_last_index_of(s tea, substr tea) normie {
    # Find last index of substring, return -1 if not found
    sus len_s normie = string_length(s)
    sus len_substr normie = string_length(substr)
    sus last_index normie = -1
    
    highkey len_substr == 0 {
        damn len_s  # Empty string found at end
    }
    
    bestie i := 0; i <= len_s - len_substr; i++ {
        sus match lit = based
        bestie j := 0; j < len_substr; j++ {
            highkey s[i + j] != substr[j] {
                match = cap
                ghosted
            }
        }
        highkey match {
            last_index = i
        }
    }
    damn last_index
}

slay string_count_occurrences(s tea, substr tea) normie {
    # Count occurrences of substring
    sus count normie = 0
    sus len_s normie = string_length(s)
    sus len_substr normie = string_length(substr)
    
    highkey len_substr == 0 {
        damn len_s + 1  # Empty string occurs at every position plus one
    }
    
    bestie i := 0; i <= len_s - len_substr; i++ {
        sus match lit = based
        bestie j := 0; j < len_substr; j++ {
            highkey s[i + j] != substr[j] {
                match = cap
                ghosted
            }
        }
        highkey match {
            count++
            i += len_substr - 1  # Skip ahead to avoid overlapping matches
        }
    }
    damn count
}

# ===== PREFIX AND SUFFIX OPERATIONS =====

slay string_has_prefix(s tea, prefix tea) lit {
    # Check if string starts with prefix
    sus len_s normie = string_length(s)
    sus len_prefix normie = string_length(prefix)
    
    highkey len_s < len_prefix {
        damn cap
    }
    
    bestie i := 0; i < len_prefix; i++ {
        highkey s[i] != prefix[i] {
            damn cap
        }
    }
    damn based
}

slay string_has_suffix(s tea, suffix tea) lit {
    # Check if string ends with suffix
    sus len_s normie = string_length(s)
    sus len_suffix normie = string_length(suffix)
    
    highkey len_s < len_suffix {
        damn cap
    }
    
    sus start_pos normie = len_s - len_suffix
    bestie i := 0; i < len_suffix; i++ {
        highkey s[start_pos + i] != suffix[i] {
            damn cap
        }
    }
    damn based
}

# ===== CASE CONVERSION OPERATIONS =====

slay char_to_lower(ch sip) sip {
    # Convert single character to lowercase
    highkey ch >= 'A' && ch <= 'Z' {
        damn ch + 32
    }
    damn ch
}

slay char_to_upper(ch sip) sip {
    # Convert single character to uppercase
    highkey ch >= 'a' && ch <= 'z' {
        damn ch - 32
    }
    damn ch
}

slay string_to_lower(s tea) tea {
    # Convert string to lowercase
    sus result tea = ""
    sus len_s normie = string_length(s)
    
    bestie i := 0; i < len_s; i++ {
        result = result + char_to_lower(s[i])
    }
    damn result
}

slay string_to_upper(s tea) tea {
    # Convert string to uppercase
    sus result tea = ""
    sus len_s normie = string_length(s)
    
    bestie i := 0; i < len_s; i++ {
        result = result + char_to_upper(s[i])
    }
    damn result
}

slay string_to_title_case(s tea) tea {
    # Convert string to title case (first letter uppercase)
    sus result tea = ""
    sus len_s normie = string_length(s)
    sus is_word_start lit = based
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey is_whitespace(ch) {
            result = result + ch
            is_word_start = based
        } highkey is_word_start {
            result = result + char_to_upper(ch)
            is_word_start = cap
        } nah {
            result = result + char_to_lower(ch)
        }
    }
    damn result
}

# ===== WHITESPACE AND TRIMMING OPERATIONS =====

slay is_whitespace(ch sip) lit {
    # Check if character is whitespace
    damn ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' || ch == '\f' || ch == '\v'
}

slay string_trim_left(s tea) tea {
    # Remove leading whitespace
    sus len_s normie = string_length(s)
    sus start normie = 0
    
    lowkey start < len_s && is_whitespace(s[start]) {
        start++
    }
    
    damn string_substring(s, start, len_s - start)
}

slay string_trim_right(s tea) tea {
    # Remove trailing whitespace
    sus len_s normie = string_length(s)
    sus end normie = len_s - 1
    
    lowkey end >= 0 && is_whitespace(s[end]) {
        end--
    }
    
    highkey end < 0 {
        damn ""
    }
    
    damn string_substring(s, 0, end + 1)
}

slay string_trim(s tea) tea {
    # Remove leading and trailing whitespace
    damn string_trim_right(string_trim_left(s))
}

# ===== SUBSTRING AND SLICING OPERATIONS =====

slay string_substring(s tea, start normie, length normie) tea {
    # Extract substring safely
    sus len_s normie = string_length(s)
    sus result tea = ""
    
    # Bounds checking
    highkey start < 0 || start >= len_s || length <= 0 {
        damn ""
    }
    
    sus end_pos normie = start + length
    highkey end_pos > len_s {
        end_pos = len_s
    }
    
    bestie i := start; i < end_pos; i++ {
        result = result + s[i]
    }
    damn result
}

slay string_slice(s tea, start normie, end normie) tea {
    # Extract slice from start to end (exclusive)
    sus len_s normie = string_length(s)
    
    # Normalize negative indices
    highkey start < 0 {
        start = len_s + start
    }
    highkey end < 0 {
        end = len_s + end
    }
    
    # Bounds checking
    highkey start < 0 {
        start = 0
    }
    highkey end > len_s {
        end = len_s
    }
    highkey start >= end {
        damn ""
    }
    
    damn string_substring(s, start, end - start)
}

# ===== SPLITTING AND JOINING OPERATIONS =====

slay string_split(s tea, separator tea) [tea] {
    # Split string by separator
    sus result [tea]
    sus len_s normie = string_length(s)
    sus len_sep normie = string_length(separator)
    sus start normie = 0
    
    # Handle empty separator case
    highkey len_sep == 0 {
        # Split into individual characters
        bestie i := 0; i < len_s; i++ {
            result = append(result, string_substring(s, i, 1))
        }
        damn result
    }
    
    bestie i := 0; i <= len_s - len_sep; i++ {
        sus match lit = based
        bestie j := 0; j < len_sep; j++ {
            highkey s[i + j] != separator[j] {
                match = cap
                ghosted
            }
        }
        highkey match {
            result = append(result, string_substring(s, start, i - start))
            start = i + len_sep
            i += len_sep - 1
        }
    }
    
    # Add remaining part
    result = append(result, string_substring(s, start, len_s - start))
    damn result
}

slay string_join(parts [tea], separator tea) tea {
    # Join array of strings with separator
    sus result tea = ""
    sus len_parts normie = len(parts)
    
    bestie i := 0; i < len_parts; i++ {
        result = result + parts[i]
        highkey i < len_parts - 1 {
            result = result + separator
        }
    }
    damn result
}

slay string_split_lines(s tea) [tea] {
    # Split string into lines
    sus result [tea]
    sus len_s normie = string_length(s)
    sus start normie = 0
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey ch == '\n' {
            result = append(result, string_substring(s, start, i - start))
            start = i + 1
        } highkey ch == '\r' && i + 1 < len_s && s[i + 1] == '\n' {
            # Handle Windows line endings
            result = append(result, string_substring(s, start, i - start))
            i++  # Skip the \n
            start = i + 1
        } highkey ch == '\r' {
            # Handle Mac line endings
            result = append(result, string_substring(s, start, i - start))
            start = i + 1
        }
    }
    
    # Add remaining part
    result = append(result, string_substring(s, start, len_s - start))
    damn result
}

# ===== REPLACEMENT OPERATIONS =====

slay string_replace_first(s tea, old tea, new tea) tea {
    # Replace first occurrence of old with new
    sus index normie = string_index_of(s, old)
    highkey index == -1 {
        damn s  # No match found
    }
    
    sus len_old normie = string_length(old)
    sus before tea = string_substring(s, 0, index)
    sus after tea = string_substring(s, index + len_old, string_length(s) - index - len_old)
    
    damn before + new + after
}

slay string_replace_all(s tea, old tea, new tea) tea {
    # Replace all occurrences of old with new
    sus result tea = s
    lowkey string_contains(result, old) {
        result = string_replace_first(result, old, new)
    }
    damn result
}

slay string_replace_at_index(s tea, index normie, replacement tea) tea {
    # Replace single character at index with replacement string
    sus len_s normie = string_length(s)
    
    highkey index < 0 || index >= len_s {
        damn s  # Invalid index
    }
    
    sus before tea = string_substring(s, 0, index)
    sus after tea = string_substring(s, index + 1, len_s - index - 1)
    
    damn before + replacement + after
}

# ===== REPETITION AND PADDING OPERATIONS =====

slay string_repeat(s tea, count normie) tea {
    # Repeat string count times
    sus result tea = ""
    bestie i := 0; i < count; i++ {
        result = result + s
    }
    damn result
}

slay string_pad_left(s tea, width normie, pad_char sip) tea {
    # Pad string on the left to reach width
    sus len_s normie = string_length(s)
    sus result tea = s
    
    lowkey string_length(result) < width {
        result = pad_char + result
    }
    damn result
}

slay string_pad_right(s tea, width normie, pad_char sip) tea {
    # Pad string on the right to reach width
    sus len_s normie = string_length(s)
    sus result tea = s
    
    lowkey string_length(result) < width {
        result = result + pad_char
    }
    damn result
}

slay string_center(s tea, width normie, pad_char sip) tea {
    # Center string within given width
    sus len_s normie = string_length(s)
    
    highkey len_s >= width {
        damn s
    }
    
    sus total_padding normie = width - len_s
    sus left_padding normie = total_padding / 2
    sus right_padding normie = total_padding - left_padding
    
    sus result tea = string_repeat(pad_char + "", left_padding) + s + string_repeat(pad_char + "", right_padding)
    damn result
}

# ===== VALIDATION AND CLASSIFICATION OPERATIONS =====

slay string_is_numeric(s tea) lit {
    # Check if string contains only numeric characters
    sus len_s normie = string_length(s)
    
    highkey len_s == 0 {
        damn cap
    }
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey ch < '0' || ch > '9' {
            damn cap
        }
    }
    damn based
}

slay string_is_alpha(s tea) lit {
    # Check if string contains only alphabetic characters
    sus len_s normie = string_length(s)
    
    highkey len_s == 0 {
        damn cap
    }
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey !((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')) {
            damn cap
        }
    }
    damn based
}

slay string_is_alphanumeric(s tea) lit {
    # Check if string contains only alphanumeric characters
    sus len_s normie = string_length(s)
    
    highkey len_s == 0 {
        damn cap
    }
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        sus is_letter lit = (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')
        sus is_digit lit = ch >= '0' && ch <= '9'
        highkey !is_letter && !is_digit {
            damn cap
        }
    }
    damn based
}

slay string_is_lower(s tea) lit {
    # Check if string is all lowercase
    sus len_s normie = string_length(s)
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey ch >= 'A' && ch <= 'Z' {
            damn cap
        }
    }
    damn based
}

slay string_is_upper(s tea) lit {
    # Check if string is all uppercase
    sus len_s normie = string_length(s)
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey ch >= 'a' && ch <= 'z' {
            damn cap
        }
    }
    damn based
}

# ===== ADVANCED STRING OPERATIONS =====

slay string_common_prefix(a tea, b tea) tea {
    # Find longest common prefix
    sus len_a normie = string_length(a)
    sus len_b normie = string_length(b)
    sus min_len normie = len_a
    
    highkey len_b < min_len {
        min_len = len_b
    }
    
    sus prefix_len normie = 0
    bestie i := 0; i < min_len; i++ {
        highkey a[i] == b[i] {
            prefix_len++
        } nah {
            ghosted
        }
    }
    
    damn string_substring(a, 0, prefix_len)
}

slay string_common_suffix(a tea, b tea) tea {
    # Find longest common suffix
    sus len_a normie = string_length(a)
    sus len_b normie = string_length(b)
    sus min_len normie = len_a
    
    highkey len_b < min_len {
        min_len = len_b
    }
    
    sus suffix_len normie = 0
    bestie i := 1; i <= min_len; i++ {
        highkey a[len_a - i] == b[len_b - i] {
            suffix_len++
        } nah {
            ghosted
        }
    }
    
    damn string_substring(a, len_a - suffix_len, suffix_len)
}

slay string_distance_levenshtein(a tea, b tea) normie {
    # Calculate Levenshtein distance between two strings
    sus len_a normie = string_length(a)
    sus len_b normie = string_length(b)
    
    # Create distance matrix (simplified implementation)
    # This is a basic approximation - full implementation would use 2D arrays
    sus max_changes normie = len_a + len_b
    
    # Simple character difference count as approximation
    sus differences normie = 0
    sus min_len normie = len_a
    
    highkey len_b < min_len {
        min_len = len_b
    }
    
    bestie i := 0; i < min_len; i++ {
        highkey a[i] != b[i] {
            differences++
        }
    }
    
    # Add length difference
    highkey len_a > len_b {
        differences += len_a - len_b
    } nah {
        differences += len_b - len_a
    }
    
    damn differences
}

# ===== FORMAT AND ENCODING OPERATIONS =====

slay string_escape_special_chars(s tea) tea {
    # Escape special characters for safety
    sus result tea = ""
    sus len_s normie = string_length(s)
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey ch == '\\' {
            result = result + "\\\\"
        } highkey ch == '"' {
            result = result + "\\\""
        } highkey ch == '\n' {
            result = result + "\\n"
        } highkey ch == '\t' {
            result = result + "\\t"
        } highkey ch == '\r' {
            result = result + "\\r"
        } nah {
            result = result + ch
        }
    }
    damn result
}

slay string_unescape_special_chars(s tea) tea {
    # Unescape special characters
    sus result tea = ""
    sus len_s normie = string_length(s)
    sus i normie = 0
    
    lowkey i < len_s {
        sus ch sip = s[i]
        highkey ch == '\\' && i + 1 < len_s {
            sus next_ch sip = s[i + 1]
            highkey next_ch == '\\' {
                result = result + '\\'
                i += 2
            } highkey next_ch == '"' {
                result = result + '"'
                i += 2
            } highkey next_ch == 'n' {
                result = result + '\n'
                i += 2
            } highkey next_ch == 't' {
                result = result + '\t'
                i += 2
            } highkey next_ch == 'r' {
                result = result + '\r'
                i += 2
            } nah {
                result = result + ch
                i++
            }
        } nah {
            result = result + ch
            i++
        }
    }
    damn result
}

# ===== COMPATIBILITY ALIASES =====
# Maintaining backward compatibility with existing APIs

slay Contains(s tea, substr tea) lit {
    damn string_contains(s, substr)
}

slay HasPrefix(s tea, prefix tea) lit {
    damn string_has_prefix(s, prefix)
}

slay HasSuffix(s tea, suffix tea) lit {
    damn string_has_suffix(s, suffix)
}

slay ToLower(s tea) tea {
    damn string_to_lower(s)
}

slay ToUpper(s tea) tea {
    damn string_to_upper(s)
}

slay Trim(s tea) tea {
    damn string_trim(s)
}

slay TrimLeft(s tea) tea {
    damn string_trim_left(s)
}

slay TrimRight(s tea) tea {
    damn string_trim_right(s)
}

slay Replace(s tea, old tea, new tea) tea {
    damn string_replace_first(s, old, new)
}

slay ReplaceAll(s tea, old tea, new tea) tea {
    damn string_replace_all(s, old, new)
}

slay Split(s tea, sep tea) [tea] {
    damn string_split(s, sep)
}

slay Join(parts [tea], sep tea) tea {
    damn string_join(parts, sep)
}

slay Repeat(s tea, count normie) tea {
    damn string_repeat(s, count)
}

slay IndexOf(s tea, substr tea) normie {
    damn string_index_of(s, substr)
}

slay LastIndexOf(s tea, substr tea) normie {
    damn string_last_index_of(s, substr)
}

slay Length(s tea) normie {
    damn string_length(s)
}

slay IsEmpty(s tea) lit {
    damn string_is_empty(s)
}

slay IsNumeric(s tea) lit {
    damn string_is_numeric(s)
}

slay IsAlpha(s tea) lit {
    damn string_is_alpha(s)
}

slay IsAlphanumeric(s tea) lit {
    damn string_is_alphanumeric(s)
}

slay Substring(s tea, start normie, length normie) tea {
    damn string_substring(s, start, length)
}

slay Reverse(s tea) tea {
    damn string_reverse(s)
}

slay PadLeft(s tea, width normie, pad sip) tea {
    damn string_pad_left(s, width, pad)
}

slay PadRight(s tea, width normie, pad sip) tea {
    damn string_pad_right(s, width, pad)
}

slay Count(s tea, substr tea) normie {
    damn string_count_occurrences(s, substr)
}

slay StartsWith(s tea, prefix tea) lit {
    damn string_has_prefix(s, prefix)
}

slay EndsWith(s tea, suffix tea) lit {
    damn string_has_suffix(s, suffix)
}

slay IsWhitespace(ch sip) lit {
    damn is_whitespace(ch)
}
