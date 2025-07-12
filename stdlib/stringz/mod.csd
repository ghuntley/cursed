// CURSED Stringz Module - Advanced String Operations
// Pure CURSED implementation using built-in string functions

// ================================
// Core String Operations
// ================================

slay Contains(s tea, substr tea) lit {
    // Check if string contains substring
    lowkey string_contains(s, substr) {
        damn based
    } highkey {
        damn cap
    }
}

slay HasPrefix(s tea, prefix tea) lit {
    // Check if string starts with prefix
    lowkey string_starts_with(s, prefix) {
        damn based
    } highkey {
        damn cap
    }
}

slay HasSuffix(s tea, suffix tea) lit {
    // Check if string ends with suffix
    lowkey string_ends_with(s, suffix) {
        damn based
    } highkey {
        damn cap
    }
}

slay IndexOf(s tea, substr tea) normie {
    // Find first occurrence of substring
    damn string_index_of(s, substr)
}

slay LastIndexOf(s tea, substr tea) normie {
    // Find last occurrence of substring
    damn string_last_index_of(s, substr)
}

// ================================
// String Splitting and Joining
// ================================

slay Split(s tea, sep tea) [tea] {
    // Split string by separator
    damn string_split(s, sep)
}

slay SplitLines(s tea) [tea] {
    // Split string by newlines
    damn string_split_lines(s)
}

slay SplitWhitespace(s tea) [tea] {
    // Split string by whitespace
    damn string_split_whitespace(s)
}

slay Join(elems [tea], sep tea) tea {
    // Join array of strings with separator
    damn string_join(elems, sep)
}

// ================================
// String Replacement and Modification
// ================================

slay Replace(s tea, old tea, new tea, n normie) tea {
    // Replace first n occurrences of old with new
    // If n < 0, replace all occurrences
    if n < 0 {
        damn string_replace_all(s, old, new)
    } else {
        sus result tea = s
        sus count normie = 0
        
        // Simple replace implementation for n occurrences
        loop count < n {
            sus idx normie = string_index_of(result, old)
            if idx < 0 {
                ghosted
            }
            
            sus before tea = string_substring(result, 0, idx)
            sus after tea = string_substring(result, idx + string_len(old), string_len(result) - idx - string_len(old))
            result = before + new + after
            count = count + 1
        }
        
        damn result
    }
}

slay ReplaceAll(s tea, old tea, new tea) tea {
    // Replace all occurrences of old with new
    damn string_replace_all(s, old, new)
}

// ================================
// String Trimming and Padding
// ================================

slay Trim(s tea, cutset tea) tea {
    // Trim characters from cutset from both ends
    if cutset == "" {
        damn string_trim(s)
    } else {
        // Custom trim implementation
        sus result tea = s
        sus start normie = 0
        sus end normie = string_len(s)
        
        // Trim from start
        loop start < end {
            sus ch tea = string_char_at(result, start)
            if Contains(cutset, ch) {
                start = start + 1
            } else {
                ghosted
            }
        }
        
        // Trim from end
        loop end > start {
            sus ch tea = string_char_at(result, end - 1)
            if Contains(cutset, ch) {
                end = end - 1
            } else {
                ghosted
            }
        }
        
        damn string_substring(result, start, end - start)
    }
}

slay TrimLeft(s tea, cutset tea) tea {
    // Trim characters from cutset from left end
    if cutset == "" {
        damn string_trim_start(s)
    } else {
        sus result tea = s
        sus start normie = 0
        sus len normie = string_len(s)
        
        loop start < len {
            sus ch tea = string_char_at(result, start)
            if Contains(cutset, ch) {
                start = start + 1
            } else {
                ghosted
            }
        }
        
        damn string_substring(result, start, len - start)
    }
}

slay TrimRight(s tea, cutset tea) tea {
    // Trim characters from cutset from right end
    if cutset == "" {
        damn string_trim_end(s)
    } else {
        sus result tea = s
        sus end normie = string_len(s)
        
        loop end > 0 {
            sus ch tea = string_char_at(result, end - 1)
            if Contains(cutset, ch) {
                end = end - 1
            } else {
                ghosted
            }
        }
        
        damn string_substring(result, 0, end)
    }
}

// ================================
// String Case Conversion
// ================================

slay ToLower(s tea) tea {
    // Convert string to lowercase
    damn string_to_lower(s)
}

slay ToUpper(s tea) tea {
    // Convert string to uppercase
    damn string_to_upper(s)
}

slay Capitalize(s tea) tea {
    // Capitalize first letter
    damn string_capitalize(s)
}

// ================================
// String Validation and Classification
// ================================

slay IsEmpty(s tea) lit {
    // Check if string is empty
    if string_len(s) == 0 {
        damn based
    } else {
        damn cap
    }
}

slay IsNumeric(s tea) lit {
    // Check if string contains only numeric characters
    damn string_is_numeric(s)
}

slay IsAlpha(s tea) lit {
    // Check if string contains only alphabetic characters
    damn string_is_alpha(s)
}

slay IsAlphanumeric(s tea) lit {
    // Check if string contains only alphanumeric characters
    damn string_is_alphanumeric(s)
}

slay IsWhitespace(s tea) lit {
    // Check if string contains only whitespace
    damn string_is_whitespace(s)
}

slay IsAscii(s tea) lit {
    // Check if string contains only ASCII characters
    damn string_is_ascii(s)
}

// ================================
// String Utilities
// ================================

slay Len(s tea) normie {
    // Get string length
    damn string_len(s)
}

slay Repeat(s tea, count normie) tea {
    // Repeat string count times
    damn string_repeat(s, count)
}

slay Reverse(s tea) tea {
    // Reverse string
    damn string_reverse(s)
}

slay Substring(s tea, start normie, length normie) tea {
    // Extract substring
    damn string_substring(s, start, length)
}

slay Slice(s tea, start normie, end normie) tea {
    // Extract slice
    damn string_slice(s, start, end)
}

slay CharAt(s tea, index normie) tea {
    // Get character at index
    damn string_char_at(s, index)
}

slay Count(s tea, substr tea) normie {
    // Count occurrences of substring
    damn string_count_occurrences(s, substr)
}

// ================================
// String Comparison
// ================================

slay Compare(s1 tea, s2 tea) normie {
    // Compare strings lexicographically
    if s1 < s2 {
        damn -1
    } else if s1 > s2 {
        damn 1
    } else {
        damn 0
    }
}

slay Equals(s1 tea, s2 tea) lit {
    // Check if strings are equal
    if s1 == s2 {
        damn based
    } else {
        damn cap
    }
}

slay EqualsIgnoreCase(s1 tea, s2 tea) lit {
    // Check if strings are equal ignoring case
    if ToLower(s1) == ToLower(s2) {
        damn based
    } else {
        damn cap
    }
}

// ================================
// String Conversion
// ================================

slay ToInt(s tea) normie {
    // Convert string to integer
    damn string_to_int(s)
}

slay ToFloat(s tea) meal {
    // Convert string to float
    damn string_to_float(s)
}

slay ToBool(s tea) lit {
    // Convert string to boolean
    damn string_to_bool(s)
}

slay FromInt(i normie) tea {
    // Convert integer to string
    damn string_from_int(i)
}

slay FromFloat(f meal) tea {
    // Convert float to string
    damn string_from_float(f)
}

slay FromBool(b lit) tea {
    // Convert boolean to string
    damn string_from_bool(b)
}

// ================================
// String Encoding
// ================================

slay ToBytes(s tea) [byte] {
    // Convert string to byte array
    damn string_to_bytes(s)
}

slay FromBytes(bytes [byte]) tea {
    // Convert byte array to string
    damn string_from_bytes(bytes)
}

slay Escape(s tea) tea {
    // Escape special characters
    damn string_escape(s)
}

slay Unescape(s tea) tea {
    // Unescape special characters
    damn string_unescape(s)
}

// ================================
// Advanced String Operations
// ================================

slay LevenshteinDistance(s1 tea, s2 tea) normie {
    // Calculate Levenshtein distance between strings
    damn string_levenshtein_distance(s1, s2)
}

slay Similarity(s1 tea, s2 tea) meal {
    // Calculate similarity between strings (0.0 to 1.0)
    damn string_similarity(s1, s2)
}

slay Hash(s tea) normie {
    // Calculate hash of string
    damn string_hash(s)
}

slay PadLeft(s tea, length normie, pad_char tea) tea {
    // Pad string on left
    damn string_pad_left(s, length, pad_char)
}

slay PadRight(s tea, length normie, pad_char tea) tea {
    // Pad string on right
    damn string_pad_right(s, length, pad_char)
}

slay PadCenter(s tea, length normie, pad_char tea) tea {
    // Pad string in center
    damn string_pad_center(s, length, pad_char)
}

slay Format(template tea, args [tea]) tea {
    // Format string with arguments
    damn string_format(template, args)
}
