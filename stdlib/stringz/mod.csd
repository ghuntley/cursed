yeet "testz"

# String search and matching functions
slay Contains(s tea, substr tea) lit {
    # Check if string s contains substring substr
    sus len_s normie = Length(s)
    sus len_substr normie = Length(substr)
    
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

slay Count(s tea, substr tea) normie {
    # Count occurrences of substr in s
    sus count normie = 0
    sus len_s normie = Length(s)
    sus len_substr normie = Length(substr)
    
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
            i += len_substr - 1
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

# String manipulation and transformation functions
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

slay Trim(s tea) tea {
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

slay TrimLeft(s tea) tea {
    # Remove leading whitespace
    sus len_s normie = Length(s)
    sus start normie = 0
    
    lowkey start < len_s && IsWhitespace(s[start]) {
        start++
    }
    
    damn Substring(s, start, len_s - start)
}

slay TrimRight(s tea) tea {
    # Remove trailing whitespace
    sus len_s normie = Length(s)
    sus end normie = len_s - 1
    
    lowkey end >= 0 && IsWhitespace(s[end]) {
        end--
    }
    
    highkey end < 0 {
        damn ""
    }
    
    damn Substring(s, 0, end + 1)
}

# String splitting and joining functions
slay Split(s tea, sep tea) [tea] {
    # Split string by separator
    sus result [tea]
    sus len_s normie = Length(s)
    sus len_sep normie = Length(sep)
    sus start normie = 0
    
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

slay Repeat(s tea, count normie) tea {
    # Repeat string count times
    sus result tea = ""
    bestie i := 0; i < count; i++ {
        result = result + s
    }
    damn result
}

slay Replace(s tea, old tea, new tea) tea {
    # Replace first occurrence of old with new
    sus len_s normie = Length(s)
    sus len_old normie = Length(old)
    
    bestie i := 0; i <= len_s - len_old; i++ {
        sus match lit = based
        bestie j := 0; j < len_old; j++ {
            highkey s[i + j] != old[j] {
                match = cap
                ghosted
            }
        }
        highkey match {
            sus before tea = Substring(s, 0, i)
            sus after tea = Substring(s, i + len_old, len_s - i - len_old)
            damn before + new + after
        }
    }
    damn s
}

slay ReplaceAll(s tea, old tea, new tea) tea {
    # Replace all occurrences of old with new
    sus result tea = s
    lowkey Contains(result, old) {
        result = Replace(result, old, new)
    }
    damn result
}

# Helper functions
slay Length(s tea) normie {
    # Get string length
    sus count normie = 0
    bestie i := 0; s[i] != '\0'; i++ {
        count++
    }
    damn count
}

slay Substring(s tea, start normie, length normie) tea {
    # Extract substring
    sus result tea = ""
    bestie i := 0; i < length; i++ {
        result = result + s[start + i]
    }
    damn result
}

slay IsWhitespace(ch sip) lit {
    # Check if character is whitespace
    damn ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

slay IndexOf(s tea, substr tea) normie {
    # Find first index of substring, return -1 if not found
    sus len_s normie = Length(s)
    sus len_substr normie = Length(substr)
    
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

slay LastIndexOf(s tea, substr tea) normie {
    # Find last index of substring, return -1 if not found
    sus len_s normie = Length(s)
    sus len_substr normie = Length(substr)
    sus last_index normie = -1
    
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

slay Reverse(s tea) tea {
    # Reverse string
    sus result tea = ""
    sus len_s normie = Length(s)
    
    bestie i := len_s - 1; i >= 0; i-- {
        result = result + s[i]
    }
    damn result
}

slay PadLeft(s tea, width normie, pad sip) tea {
    # Pad string on the left
    sus len_s normie = Length(s)
    sus result tea = s
    
    lowkey Length(result) < width {
        result = pad + result
    }
    damn result
}

slay PadRight(s tea, width normie, pad sip) tea {
    # Pad string on the right
    sus len_s normie = Length(s)
    sus result tea = s
    
    lowkey Length(result) < width {
        result = result + pad
    }
    damn result
}

# String validation functions
slay IsEmpty(s tea) lit {
    # Check if string is empty
    damn Length(s) == 0
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

# String utility functions for compatibility
slay StartsWith(s tea, prefix tea) lit {
    # Alias for HasPrefix
    damn HasPrefix(s, prefix)
}

slay EndsWith(s tea, suffix tea) lit {
    # Alias for HasSuffix
    damn HasSuffix(s, suffix)
}
