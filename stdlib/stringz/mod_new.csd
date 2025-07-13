yeet "testz"

fr fr ========================================
fr fr CURSED String Operations Library v2.0
fr fr Complete string manipulation and processing
fr fr Pure CURSED implementation - no FFI
fr fr ========================================

fr fr ================================
fr fr Core String Search Functions
fr fr ================================

slay Contains(s tea, substr tea) lit {
    # Check if string s contains substring substr
    sus len_s normie = Length(s)
    sus len_substr normie = Length(substr)
    
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

slay ContainsAny(s tea, chars tea) lit {
    # Check if string s contains any character from chars
    sus len_s normie = Length(s)
    sus len_chars normie = Length(chars)
    
    bestie i := 0; i < len_s; i++ {
        bestie j := 0; j < len_chars; j++ {
            highkey s[i] == chars[j] {
                damn based
            }
        }
    }
    damn cap
}

slay Count(s tea, substr tea) normie {
    # Count occurrences of substr in s
    sus count normie = 0
    sus len_s normie = Length(s)
    sus len_substr normie = Length(substr)
    
    highkey len_substr == 0 {
        damn len_s + 1  # Empty string appears n+1 times in string of length n
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
            i += len_substr - 1  # Skip matched portion
        }
    }
    damn count
}

slay HasPrefix(s tea, prefix tea) lit {
    # Check if string s starts with prefix
    sus len_s normie = Length(s)
    sus len_prefix normie = Length(prefix)
    
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

slay HasSuffix(s tea, suffix tea) lit {
    # Check if string s ends with suffix
    sus len_s normie = Length(s)
    sus len_suffix normie = Length(suffix)
    
    highkey len_s < len_suffix {
        damn cap
    }
    
    sus start normie = len_s - len_suffix
    bestie i := 0; i < len_suffix; i++ {
        highkey s[start + i] != suffix[i] {
            damn cap
        }
    }
    damn based
}

slay Index(s tea, substr tea) normie {
    # Find first index of substring, return -1 if not found
    sus len_s normie = Length(s)
    sus len_substr normie = Length(substr)
    
    highkey len_substr == 0 {
        damn 0  # Empty string is at position 0
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

slay LastIndex(s tea, substr tea) normie {
    # Find last index of substring, return -1 if not found
    sus len_s normie = Length(s)
    sus len_substr normie = Length(substr)
    sus last_index normie = -1
    
    highkey len_substr == 0 {
        damn len_s  # Empty string is at end position
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

fr fr ================================
fr fr String Manipulation Functions
fr fr ================================

slay ToLower(s tea) tea {
    # Convert string to lowercase
    sus result tea = ""
    sus len_s normie = Length(s)
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey ch >= 'A' && ch <= 'Z' {
            ch = ch + 32  # Convert to lowercase
        }
        result = result + ch
    }
    damn result
}

slay ToUpper(s tea) tea {
    # Convert string to uppercase
    sus result tea = ""
    sus len_s normie = Length(s)
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey ch >= 'a' && ch <= 'z' {
            ch = ch - 32  # Convert to uppercase
        }
        result = result + ch
    }
    damn result
}

slay TrimSpace(s tea) tea {
    # Remove leading and trailing whitespace
    sus len_s normie = Length(s)
    sus start normie = 0
    sus end normie = len_s - 1
    
    # Find first non-whitespace character
    lowkey start < len_s && IsWhitespace(s[start]) {
        start++
    }
    
    # Find last non-whitespace character
    lowkey end >= 0 && IsWhitespace(s[end]) {
        end--
    }
    
    highkey start > end {
        damn ""
    }
    
    damn Substring(s, start, end - start + 1)
}

slay Trim(s tea, cutset tea) tea {
    # Remove leading and trailing characters from cutset
    sus len_s normie = Length(s)
    sus start normie = 0
    sus end normie = len_s - 1
    
    # Find first character not in cutset
    lowkey start < len_s && ContainsChar(cutset, s[start]) {
        start++
    }
    
    # Find last character not in cutset
    lowkey end >= 0 && ContainsChar(cutset, s[end]) {
        end--
    }
    
    highkey start > end {
        damn ""
    }
    
    damn Substring(s, start, end - start + 1)
}

slay TrimLeft(s tea, cutset tea) tea {
    # Remove leading characters from cutset
    sus len_s normie = Length(s)
    sus start normie = 0
    
    lowkey start < len_s && ContainsChar(cutset, s[start]) {
        start++
    }
    
    damn Substring(s, start, len_s - start)
}

slay TrimRight(s tea, cutset tea) tea {
    # Remove trailing characters from cutset
    sus len_s normie = Length(s)
    sus end normie = len_s - 1
    
    lowkey end >= 0 && ContainsChar(cutset, s[end]) {
        end--
    }
    
    highkey end < 0 {
        damn ""
    }
    
    damn Substring(s, 0, end + 1)
}

slay TrimPrefix(s tea, prefix tea) tea {
    # Remove prefix if present
    highkey HasPrefix(s, prefix) {
        sus prefix_len normie = Length(prefix)
        sus s_len normie = Length(s)
        damn Substring(s, prefix_len, s_len - prefix_len)
    }
    damn s
}

slay TrimSuffix(s tea, suffix tea) tea {
    # Remove suffix if present
    highkey HasSuffix(s, suffix) {
        sus suffix_len normie = Length(suffix)
        sus s_len normie = Length(s)
        damn Substring(s, 0, s_len - suffix_len)
    }
    damn s
}

slay Replace(s tea, old tea, new tea) tea {
    # Replace first occurrence of old with new
    sus index normie = Index(s, old)
    highkey index == -1 {
        damn s  # No replacement needed
    }
    
    sus old_len normie = Length(old)
    sus before tea = Substring(s, 0, index)
    sus after tea = Substring(s, index + old_len, Length(s) - index - old_len)
    damn before + new + after
}

slay ReplaceAll(s tea, old tea, new tea) tea {
    # Replace all occurrences of old with new
    sus result tea = s
    lowkey Contains(result, old) {
        result = Replace(result, old, new)
    }
    damn result
}

slay Repeat(s tea, count normie) tea {
    # Repeat string count times
    sus result tea = ""
    bestie i := 0; i < count; i++ {
        result = result + s
    }
    damn result
}

fr fr ================================
fr fr String Splitting and Joining
fr fr ================================

slay Split(s tea, sep tea) [tea] {
    # Split string by separator
    sus result [tea]
    sus len_s normie = Length(s)
    sus len_sep normie = Length(sep)
    sus start normie = 0
    
    highkey len_sep == 0 {
        # Split into individual characters
        bestie i := 0; i < len_s; i++ {
            result = append(result, Substring(s, i, 1))
        }
        damn result
    }
    
    bestie i := 0; i <= len_s - len_sep; i++ {
        sus match lit = based
        bestie j := 0; j < len_sep; j++ {
            highkey s[i + j] != sep[j] {
                match = cap
                ghosted
            }
        }
        highkey match {
            result = append(result, Substring(s, start, i - start))
            start = i + len_sep
            i += len_sep - 1
        }
    }
    
    # Add remaining part
    result = append(result, Substring(s, start, len_s - start))
    damn result
}

slay SplitN(s tea, sep tea, n normie) [tea] {
    # Split string by separator with limit
    sus result [tea]
    sus len_s normie = Length(s)
    sus len_sep normie = Length(sep)
    sus start normie = 0
    sus count normie = 0
    
    highkey n <= 0 {
        damn result  # Return empty slice
    }
    
    highkey n == 1 {
        result = append(result, s)
        damn result
    }
    
    bestie i := 0; i <= len_s - len_sep && count < n - 1; i++ {
        sus match lit = based
        bestie j := 0; j < len_sep; j++ {
            highkey s[i + j] != sep[j] {
                match = cap
                ghosted
            }
        }
        highkey match {
            result = append(result, Substring(s, start, i - start))
            start = i + len_sep
            i += len_sep - 1
            count++
        }
    }
    
    # Add remaining part as last element
    result = append(result, Substring(s, start, len_s - start))
    damn result
}

slay Join(parts [tea], sep tea) tea {
    # Join array of strings with separator
    sus result tea = ""
    sus len_parts normie = len(parts)
    
    bestie i := 0; i < len_parts; i++ {
        result = result + parts[i]
        highkey i < len_parts - 1 {
            result = result + sep
        }
    }
    damn result
}

slay Fields(s tea) [tea] {
    # Split string by whitespace
    sus result [tea]
    sus len_s normie = Length(s)
    sus start normie = -1
    
    bestie i := 0; i < len_s; i++ {
        highkey !IsWhitespace(s[i]) {
            highkey start == -1 {
                start = i  # Start of new field
            }
        } nah {
            highkey start != -1 {
                # End of current field
                result = append(result, Substring(s, start, i - start))
                start = -1
            }
        }
    }
    
    # Add final field if exists
    highkey start != -1 {
        result = append(result, Substring(s, start, len_s - start))
    }
    
    damn result
}

fr fr ================================
fr fr String Utility Functions
fr fr ================================

slay Reverse(s tea) tea {
    # Reverse string
    sus result tea = ""
    sus len_s normie = Length(s)
    
    bestie i := len_s - 1; i >= 0; i-- {
        result = result + s[i]
    }
    damn result
}

slay PadLeft(s tea, width normie, pad tea) tea {
    # Pad string on the left
    sus len_s normie = Length(s)
    sus pad_len normie = Length(pad)
    sus result tea = s
    
    highkey pad_len == 0 {
        damn result
    }
    
    lowkey Length(result) < width {
        sus needed normie = width - Length(result)
        bestie i := 0; i < needed; i++ {
            result = pad[i % pad_len] + result
        }
    }
    damn result
}

slay PadRight(s tea, width normie, pad tea) tea {
    # Pad string on the right
    sus len_s normie = Length(s)
    sus pad_len normie = Length(pad)
    sus result tea = s
    
    highkey pad_len == 0 {
        damn result
    }
    
    lowkey Length(result) < width {
        sus needed normie = width - Length(result)
        bestie i := 0; i < needed; i++ {
            result = result + pad[i % pad_len]
        }
    }
    damn result
}

slay Center(s tea, width normie, pad tea) tea {
    # Center string in field of given width
    sus len_s normie = Length(s)
    
    highkey len_s >= width {
        damn s
    }
    
    sus total_padding normie = width - len_s
    sus left_padding normie = total_padding / 2
    sus right_padding normie = total_padding - left_padding
    
    sus result tea = s
    
    # Add left padding
    bestie i := 0; i < left_padding; i++ {
        result = pad[i % Length(pad)] + result
    }
    
    # Add right padding
    bestie i := 0; i < right_padding; i++ {
        result = result + pad[i % Length(pad)]
    }
    
    damn result
}

fr fr ================================
fr fr String Validation Functions
fr fr ================================

slay IsEmpty(s tea) lit {
    # Check if string is empty
    damn Length(s) == 0
}

slay IsBlank(s tea) lit {
    # Check if string is empty or contains only whitespace
    sus trimmed tea = TrimSpace(s)
    damn Length(trimmed) == 0
}

slay IsNumeric(s tea) lit {
    # Check if string contains only numeric characters
    sus len_s normie = Length(s)
    
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

slay IsAlpha(s tea) lit {
    # Check if string contains only alphabetic characters
    sus len_s normie = Length(s)
    
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

slay IsAlphanumeric(s tea) lit {
    # Check if string contains only alphanumeric characters
    sus len_s normie = Length(s)
    
    highkey len_s == 0 {
        damn cap
    }
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        highkey !((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9')) {
            damn cap
        }
    }
    damn based
}

fr fr ================================
fr fr Advanced String Functions
fr fr ================================

slay Before(s tea, sep tea) tea {
    # Returns the portion of s before the first instance of sep
    sus index normie = Index(s, sep)
    highkey index == -1 {
        damn s  # No separator found, return original string
    }
    damn Substring(s, 0, index)
}

slay After(s tea, sep tea) tea {
    # Returns the portion of s after the first instance of sep
    sus index normie = Index(s, sep)
    highkey index == -1 {
        damn ""  # No separator found, return empty string
    }
    sus sep_len normie = Length(sep)
    sus s_len normie = Length(s)
    damn Substring(s, index + sep_len, s_len - index - sep_len)
}

slay BeforeLast(s tea, sep tea) tea {
    # Returns the portion of s before the last instance of sep
    sus index normie = LastIndex(s, sep)
    highkey index == -1 {
        damn s  # No separator found, return original string
    }
    damn Substring(s, 0, index)
}

slay AfterLast(s tea, sep tea) tea {
    # Returns the portion of s after the last instance of sep
    sus index normie = LastIndex(s, sep)
    highkey index == -1 {
        damn ""  # No separator found, return empty string
    }
    sus sep_len normie = Length(sep)
    sus s_len normie = Length(s)
    damn Substring(s, index + sep_len, s_len - index - sep_len)
}

slay Truncate(s tea, length normie) tea {
    # Truncate string to specified length
    sus s_len normie = Length(s)
    highkey s_len <= length {
        damn s
    }
    damn Substring(s, 0, length)
}

slay TruncateWithEllipsis(s tea, length normie) tea {
    # Truncate string with ellipsis
    sus s_len normie = Length(s)
    highkey s_len <= length {
        damn s
    }
    
    highkey length <= 3 {
        damn Substring(s, 0, length)
    }
    
    sus truncated tea = Substring(s, 0, length - 3)
    damn truncated + "..."
}

fr fr ================================
fr fr Case Conversion Functions
fr fr ================================

slay ToSnakeCase(s tea) tea {
    # Convert string to snake_case
    sus result tea = ""
    sus len_s normie = Length(s)
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        
        # Convert to lowercase
        highkey ch >= 'A' && ch <= 'Z' {
            # Add underscore before uppercase letter (except at start)
            highkey i > 0 && result != "" {
                result = result + "_"
            }
            ch = ch + 32  # Convert to lowercase
        } nah vibes ch == ' ' || ch == '-' {
            ch = '_'
        }
        
        # Skip non-alphanumeric except underscore
        highkey (ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9') || ch == '_' {
            result = result + ch
        }
    }
    damn result
}

slay ToCamelCase(s tea) tea {
    # Convert string to camelCase
    sus result tea = ""
    sus len_s normie = Length(s)
    sus capitalize_next lit = cap
    
    bestie i := 0; i < len_s; i++ {
        sus ch sip = s[i]
        
        highkey ch == '_' || ch == '-' || ch == ' ' {
            capitalize_next = based
        } nah vibes (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9') {
            highkey capitalize_next && ch >= 'a' && ch <= 'z' {
                ch = ch - 32  # Convert to uppercase
                capitalize_next = cap
            } nah vibes ch >= 'A' && ch <= 'Z' && !capitalize_next {
                ch = ch + 32  # Convert to lowercase
            } nah {
                capitalize_next = cap
            }
            result = result + ch
        }
    }
    damn result
}

slay ToPascalCase(s tea) tea {
    # Convert string to PascalCase
    sus camel tea = ToCamelCase(s)
    sus len_camel normie = Length(camel)
    
    highkey len_camel == 0 {
        damn camel
    }
    
    # Capitalize first character
    sus first_char sip = camel[0]
    highkey first_char >= 'a' && first_char <= 'z' {
        first_char = first_char - 32
    }
    
    sus result tea = first_char + Substring(camel, 1, len_camel - 1)
    damn result
}

slay ToKebabCase(s tea) tea {
    # Convert string to kebab-case
    sus snake tea = ToSnakeCase(s)
    damn ReplaceAll(snake, "_", "-")
}

fr fr ================================
fr fr Helper Functions
fr fr ================================

slay Length(s tea) normie {
    # Get string length
    sus count normie = 0
    bestie i := 0; i < 1000 && s[i] != '\0'; i++ {
        count++
    }
    damn count
}

slay Substring(s tea, start normie, length normie) tea {
    # Extract substring
    sus result tea = ""
    sus s_len normie = Length(s)
    
    # Bounds checking
    highkey start < 0 {
        start = 0
    }
    highkey start >= s_len {
        damn ""
    }
    highkey length < 0 {
        length = 0
    }
    highkey start + length > s_len {
        length = s_len - start
    }
    
    bestie i := 0; i < length; i++ {
        result = result + s[start + i]
    }
    damn result
}

slay IsWhitespace(ch sip) lit {
    # Check if character is whitespace
    damn ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

slay ContainsChar(s tea, ch sip) lit {
    # Check if string contains character
    sus len_s normie = Length(s)
    bestie i := 0; i < len_s; i++ {
        highkey s[i] == ch {
            damn based
        }
    }
    damn cap
}

fr fr ================================
fr fr Alias Functions for Compatibility
fr fr ================================

slay StartsWith(s tea, prefix tea) lit {
    # Alias for HasPrefix
    damn HasPrefix(s, prefix)
}

slay EndsWith(s tea, suffix tea) lit {
    # Alias for HasSuffix
    damn HasSuffix(s, suffix)
}

slay IndexOf(s tea, substr tea) normie {
    # Alias for Index
    damn Index(s, substr)
}

slay LastIndexOf(s tea, substr tea) normie {
    # Alias for LastIndex
    damn LastIndex(s, substr)
}

slay Trim(s tea) tea {
    # Overloaded version for trimming whitespace
    damn TrimSpace(s)
}
