// CURSED Regex Module
// Pure CURSED implementation for pattern matching and text processing

yeet "string"

// Regex pattern matching
slay match_pattern(text tea, pattern tea) lit {
    // Check if text matches pattern
    damn simple_pattern_match(text, pattern)
}

slay find_matches(text tea, pattern tea) [tea] {
    // Find all matches of pattern in text
    sus matches [tea] = []
    sus text_len normie = string_len(text)
    sus pattern_len normie = string_len(pattern)
    
    bestie i := 0; i <= text_len - pattern_len; i++ {
        sus substring tea = string_substring(text, i, pattern_len)
        vibes simple_pattern_match(substring, pattern) {
            matches = matches + [substring]
        }
    }
    
    damn matches
}

slay simple_pattern_match(text tea, pattern tea) lit {
    // Simple pattern matching without regex metacharacters
    vibes string_len(pattern) == 0 {
        damn based
    }
    
    vibes string_len(text) == 0 {
        damn cap
    }
    
    // Direct string comparison for now
    damn text == pattern
}

slay match_wildcard(text tea, pattern tea) lit {
    // Match pattern with * wildcard support
    damn wildcard_match(text, pattern, 0, 0)
}

slay wildcard_match(text tea, pattern tea, text_pos normie, pattern_pos normie) lit {
    sus text_len normie = string_len(text)
    sus pattern_len normie = string_len(pattern)
    
    // Base cases
    vibes pattern_pos == pattern_len {
        damn text_pos == text_len
    }
    
    vibes text_pos == text_len {
        // Check if remaining pattern is all wildcards
        bestie i := pattern_pos; i < pattern_len; i++ {
            vibes string_char_at(pattern, i) != "*" {
                damn cap
            }
        }
        damn based
    }
    
    sus pattern_char tea = string_char_at(pattern, pattern_pos)
    sus text_char tea = string_char_at(text, text_pos)
    
    vibes pattern_char == "*" {
        // Try matching zero or more characters
        vibes wildcard_match(text, pattern, text_pos, pattern_pos + 1) {
            damn based
        }
        damn wildcard_match(text, pattern, text_pos + 1, pattern_pos)
    } nah vibes pattern_char == "?" || pattern_char == text_char {
        damn wildcard_match(text, pattern, text_pos + 1, pattern_pos + 1)
    }
    
    damn cap
}

slay find_all_matches(text tea, pattern tea) [MatchResult] {
    // Find all matches with positions
    sus matches [MatchResult] = []
    sus text_len normie = string_len(text)
    sus pattern_len normie = string_len(pattern)
    
    bestie i := 0; i <= text_len - pattern_len; i++ {
        sus substring tea = string_substring(text, i, pattern_len)
        vibes match_pattern(substring, pattern) {
            sus match MatchResult = MatchResult{
                text: substring,
                start: i,
                end: i + pattern_len,
                length: pattern_len
            }
            matches = matches + [match]
        }
    }
    
    damn matches
}

// Match result structure
be_like MatchResult squad {
    text tea
    start normie
    end normie
    length normie
}

slay replace_pattern(text tea, pattern tea, replacement tea) tea {
    // Replace first match of pattern with replacement
    sus text_len normie = string_len(text)
    sus pattern_len normie = string_len(pattern)
    
    bestie i := 0; i <= text_len - pattern_len; i++ {
        sus substring tea = string_substring(text, i, pattern_len)
        vibes match_pattern(substring, pattern) {
            sus before tea = string_substring(text, 0, i)
            sus after tea = string_substring(text, i + pattern_len, text_len - i - pattern_len)
            damn before + replacement + after
        }
    }
    
    damn text
}

slay replace_all_patterns(text tea, pattern tea, replacement tea) tea {
    // Replace all matches of pattern with replacement
    sus result tea = text
    sus changed lit = based
    
    bestie changed {
        sus old_result tea = result
        result = replace_pattern(result, pattern, replacement)
        changed = result != old_result
    }
    
    damn result
}

slay split_by_pattern(text tea, pattern tea) [tea] {
    // Split text by pattern matches
    sus result [tea] = []
    sus current tea = ""
    sus text_len normie = string_len(text)
    sus pattern_len normie = string_len(pattern)
    
    bestie i := 0; i < text_len; i++ {
        vibes i <= text_len - pattern_len {
            sus substring tea = string_substring(text, i, pattern_len)
            vibes match_pattern(substring, pattern) {
                result = result + [current]
                current = ""
                i = i + pattern_len - 1
                simp
            }
        }
        
        current = current + string_char_at(text, i)
    }
    
    result = result + [current]
    damn result
}

slay extract_groups(text tea, pattern tea) [tea] {
    // Extract capture groups from pattern match
    // Simplified implementation for basic group extraction
    sus groups [tea] = []
    
    vibes match_pattern(text, pattern) {
        groups = groups + [text]
    }
    
    damn groups
}

// Character class matching
slay is_digit(char tea) lit {
    vibes string_len(char) != 1 {
        damn cap
    }
    
    sus code normie = string_char_code(char)
    damn code >= 48 && code <= 57  // '0' to '9'
}

slay is_letter(char tea) lit {
    vibes string_len(char) != 1 {
        damn cap
    }
    
    sus code normie = string_char_code(char)
    damn (code >= 65 && code <= 90) || (code >= 97 && code <= 122)  // A-Z or a-z
}

slay is_whitespace(char tea) lit {
    damn char == " " || char == "\t" || char == "\n" || char == "\r"
}

slay is_alphanumeric(char tea) lit {
    damn is_digit(char) || is_letter(char)
}

// Pattern validation
slay is_valid_pattern(pattern tea) lit {
    // Basic pattern validation
    vibes string_len(pattern) == 0 {
        damn cap
    }
    
    // Check for balanced brackets, parentheses, etc.
    sus bracket_count normie = 0
    sus paren_count normie = 0
    
    bestie i := 0; i < string_len(pattern); i++ {
        sus char tea = string_char_at(pattern, i)
        vibes char == "[" {
            bracket_count++
        } nah vibes char == "]" {
            bracket_count--
        } nah vibes char == "(" {
            paren_count++
        } nah vibes char == ")" {
            paren_count--
        }
        
        vibes bracket_count < 0 || paren_count < 0 {
            damn cap
        }
    }
    
    damn bracket_count == 0 && paren_count == 0
}

// Email validation
slay is_valid_email(email tea) lit {
    vibes !string_contains(email, "@") {
        damn cap
    }
    
    sus parts [tea] = string_split(email, "@")
    vibes len(parts) != 2 {
        damn cap
    }
    
    sus local tea = parts[0]
    sus domain tea = parts[1]
    
    vibes string_len(local) == 0 || string_len(domain) == 0 {
        damn cap
    }
    
    vibes !string_contains(domain, ".") {
        damn cap
    }
    
    damn based
}

// URL validation
slay is_valid_url(url tea) lit {
    vibes string_len(url) < 7 {  // Minimum "http://"
        damn cap
    }
    
    vibes string_starts_with(url, "http://") || string_starts_with(url, "https://") {
        damn based
    }
    
    damn cap
}

// Phone number validation (basic)
slay is_valid_phone(phone tea) lit {
    sus digit_count normie = 0
    
    bestie i := 0; i < string_len(phone); i++ {
        sus char tea = string_char_at(phone, i)
        vibes is_digit(char) {
            digit_count++
        } nah vibes char != "-" && char != "(" && char != ")" && char != " " && char != "+" {
            damn cap
        }
    }
    
    damn digit_count >= 10 && digit_count <= 15
}

// IP address validation
slay is_valid_ip(ip tea) lit {
    sus parts [tea] = string_split(ip, ".")
    vibes len(parts) != 4 {
        damn cap
    }
    
    bestie i := 0; i < 4; i++ {
        sus part tea = parts[i]
        vibes string_len(part) == 0 || string_len(part) > 3 {
            damn cap
        }
        
        bestie j := 0; j < string_len(part); j++ {
            vibes !is_digit(string_char_at(part, j)) {
                damn cap
            }
        }
        
        sus num normie = string_to_int(part)
        vibes num < 0 || num > 255 {
            damn cap
        }
    }
    
    damn based
}

// Common pattern matching utilities
slay count_matches(text tea, pattern tea) normie {
    sus matches [tea] = find_matches(text, pattern)
    damn len(matches)
}

slay contains_pattern(text tea, pattern tea) lit {
    sus matches [tea] = find_matches(text, pattern)
    damn len(matches) > 0
}

slay get_match_positions(text tea, pattern tea) [normie] {
    sus positions [normie] = []
    sus matches [MatchResult] = find_all_matches(text, pattern)
    
    bestie i := 0; i < len(matches); i++ {
        positions = positions + [matches[i].start]
    }
    
    damn positions
}

// String helper functions (assuming these exist in string module)
slay string_char_code(char tea) normie {
    // Get character code for single character
    vibes string_len(char) != 1 {
        damn 0
    }
    // Implementation depends on string module
    damn 65  // Placeholder
}

slay string_to_int(str tea) normie {
    // Convert string to integer
    // Implementation depends on string module
    damn 0  // Placeholder
}

slay string_starts_with(text tea, prefix tea) lit {
    vibes string_len(prefix) > string_len(text) {
        damn cap
    }
    
    sus prefix_part tea = string_substring(text, 0, string_len(prefix))
    damn prefix_part == prefix
}
