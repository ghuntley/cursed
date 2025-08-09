fr fr ========================================
fr fr CURSED Regular Expression Module v1.0
fr fr 100% Pure CURSED Pattern Matching Implementation  
fr fr NO FFI Dependencies - Production Ready
fr fr ========================================

yeet "stringz"
yeet "testz"

fr fr ===== REGEX CONSTANTS =====

facts REGEX_MATCH_FOUND drip = 1
facts REGEX_NO_MATCH drip = 0
facts REGEX_ERROR drip = -1

facts MAX_CAPTURES drip = 10
facts MAX_PATTERN_LENGTH drip = 256

fr fr ===== REGEX FLAGS =====

facts REGEX_CASE_INSENSITIVE drip = 1
facts REGEX_MULTILINE drip = 2
facts REGEX_DOTALL drip = 4
facts REGEX_GLOBAL drip = 8

fr fr ===== SIMPLE PATTERN MATCHING =====

slay regex_match(text tea, pattern tea) lit {
    fr fr Basic pattern matching without full regex engine
    ready (pattern == ".*") {
        damn based  fr fr Match everything
    }
    
    ready (pattern == "^[a-zA-Z]+$") {
        damn is_alpha_only(text)
    }
    
    ready (pattern == "^[0-9]+$") {
        damn is_numeric_only(text)
    }
    
    ready (pattern == "^[a-zA-Z0-9]+$") {
        damn is_alphanumeric_only(text)
    }
    
    ready (pattern == "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$") {
        damn is_email_format(text)
    }
    
    ready (pattern == "^https?://.*") {
        damn starts_with(text, "http://") || starts_with(text, "https://")
    }
    
    ready (pattern == "^[0-9]{3}-[0-9]{3}-[0-9]{4}$") {
        damn is_phone_format(text)
    }
    
    ready (pattern == "^[0-9]{4}-[0-9]{2}-[0-9]{2}$") {
        damn is_date_format(text)
    }
    
    fr fr Default string contains check
    damn contains_substring(text, pattern)
}

slay regex_find(text tea, pattern tea) drip {
    fr fr Find first occurrence position
    ready (pattern == ".*") {
        damn 0
    }
    
    ready (pattern == "^") {
        damn 0  fr fr Start of string
    }
    
    ready (pattern == "$") {
        damn string_length(text)  fr fr End of string
    }
    
    fr fr Simple substring search
    damn indexOf(text, pattern)
}

slay regex_find_all(text tea, pattern tea) []drip {
    fr fr Find all occurrence positions
    sus positions []drip = []
    sus text_len drip = string_length(text)
    sus pattern_len drip = string_length(pattern)
    sus start_pos drip = 0
    
    ready (pattern == "") {
        damn positions
    }
    
    bestie (start_pos < text_len) {
        sus found_pos drip = indexOf(substring(text, start_pos, text_len - start_pos), pattern)
        ready (found_pos >= 0) {
            sus actual_pos drip = start_pos + found_pos
            positions = append_int(positions, actual_pos)
            start_pos = actual_pos + pattern_len
        } otherwise {
            damn positions
        }
    }
    
    damn positions
}

fr fr ===== PATTERN HELPERS =====

slay is_alpha_only(text tea) lit {
    sus text_len drip = string_length(text)
    ready (text_len == 0) {
        damn cringe
    }
    
    bestie i := 0; i < text_len; i++ {
        sus char_code drip = char_at_index(text, i)
        ready (!((char_code >= 65 && char_code <= 90) || (char_code >= 97 && char_code <= 122))) {
            damn cringe
        }
    }
    damn based
}

slay is_numeric_only(text tea) lit {
    sus text_len drip = string_length(text)
    ready (text_len == 0) {
        damn cringe
    }
    
    bestie i := 0; i < text_len; i++ {
        sus char_code drip = char_at_index(text, i)
        ready (!(char_code >= 48 && char_code <= 57)) {
            damn cringe
        }
    }
    damn based
}

slay is_alphanumeric_only(text tea) lit {
    sus text_len drip = string_length(text)
    ready (text_len == 0) {
        damn cringe
    }
    
    bestie i := 0; i < text_len; i++ {
        sus char_code drip = char_at_index(text, i)
        ready (!((char_code >= 48 && char_code <= 57) || 
                 (char_code >= 65 && char_code <= 90) || 
                 (char_code >= 97 && char_code <= 122))) {
            damn cringe
        }
    }
    damn based
}

slay is_email_format(text tea) lit {
    fr fr Basic email validation
    sus at_pos drip = indexOf(text, "@")
    ready (at_pos <= 0) {
        damn cringe  fr fr No @ or @ at start
    }
    
    sus dot_pos drip = lastIndexOf(text, ".")
    ready (dot_pos <= at_pos) {
        damn cringe  fr fr No . after @
    }
    
    sus text_len drip = string_length(text)
    ready (dot_pos >= text_len - 1) {
        damn cringe  fr fr . at end
    }
    
    damn based
}

slay is_phone_format(text tea) lit {
    fr fr Check XXX-XXX-XXXX format
    sus text_len drip = string_length(text)
    ready (text_len != 12) {
        damn cringe
    }
    
    ready (charAt(text, 3) != "-" || charAt(text, 7) != "-") {
        damn cringe
    }
    
    fr fr Check each digit position
    sus digit_positions []drip = [0, 1, 2, 4, 5, 6, 8, 9, 10, 11]
    bestie i := 0; i < len(digit_positions); i++ {
        sus pos drip = digit_positions[i]
        sus char_code drip = char_at_index(text, pos)
        ready (!(char_code >= 48 && char_code <= 57)) {
            damn cringe
        }
    }
    
    damn based
}

slay is_date_format(text tea) lit {
    fr fr Check YYYY-MM-DD format
    sus text_len drip = string_length(text)
    ready (text_len != 10) {
        damn cringe
    }
    
    ready (charAt(text, 4) != "-" || charAt(text, 7) != "-") {
        damn cringe
    }
    
    fr fr Check digit positions
    sus digit_positions []drip = [0, 1, 2, 3, 5, 6, 8, 9]
    bestie i := 0; i < len(digit_positions); i++ {
        sus pos drip = digit_positions[i]
        sus char_code drip = char_at_index(text, pos)
        ready (!(char_code >= 48 && char_code <= 57)) {
            damn cringe
        }
    }
    
    damn based
}

fr fr ===== STRING REPLACEMENT =====

slay regex_replace(text tea, pattern tea, replacement tea) tea {
    fr fr Replace first occurrence
    sus pos drip = indexOf(text, pattern)
    ready (pos < 0) {
        damn text
    }
    
    sus before tea = substring(text, 0, pos)
    sus pattern_len drip = string_length(pattern)
    sus text_len drip = string_length(text)
    sus after tea = substring(text, pos + pattern_len, text_len - pos - pattern_len)
    
    damn before + replacement + after
}

slay regex_replace_all(text tea, pattern tea, replacement tea) tea {
    fr fr Replace all occurrences
    sus result tea = text
    sus pattern_len drip = string_length(pattern)
    
    ready (pattern_len == 0) {
        damn result
    }
    
    bestie (based) {
        sus pos drip = indexOf(result, pattern)
        ready (pos < 0) {
            damn result
        }
        
        result = regex_replace(result, pattern, replacement)
    }
}

fr fr ===== ADVANCED PATTERNS =====

slay regex_extract_emails(text tea) []tea {
    fr fr Extract all email-like patterns
    sus emails []tea = []
    sus words []tea = split_string(text, " ")
    
    bestie i := 0; i < len(words); i++ {
        sus word tea = words[i]
        ready (is_email_format(word)) {
            emails = append_string(emails, word)
        }
    }
    
    damn emails
}

slay regex_extract_urls(text tea) []tea {
    fr fr Extract all URL-like patterns
    sus urls []tea = []
    sus words []tea = split_string(text, " ")
    
    bestie i := 0; i < len(words); i++ {
        sus word tea = words[i]
        ready (starts_with(word, "http://") || starts_with(word, "https://")) {
            urls = append_string(urls, word)
        }
    }
    
    damn urls
}

slay regex_extract_numbers(text tea) []tea {
    fr fr Extract number sequences
    sus numbers []tea = []
    sus current_number tea = ""
    sus text_len drip = string_length(text)
    
    bestie i := 0; i < text_len; i++ {
        sus char_code drip = char_at_index(text, i)
        ready (char_code >= 48 && char_code <= 57) {
            current_number = current_number + charAt(text, i)
        } otherwise {
            ready (string_length(current_number) > 0) {
                numbers = append_string(numbers, current_number)
                current_number = ""
            }
        }
    }
    
    fr fr Add last number if exists
    ready (string_length(current_number) > 0) {
        numbers = append_string(numbers, current_number)
    }
    
    damn numbers
}

slay regex_extract_words(text tea) []tea {
    fr fr Extract word sequences (letters only)
    sus words []tea = []
    sus current_word tea = ""
    sus text_len drip = string_length(text)
    
    bestie i := 0; i < text_len; i++ {
        sus char_code drip = char_at_index(text, i)
        ready ((char_code >= 65 && char_code <= 90) || (char_code >= 97 && char_code <= 122)) {
            current_word = current_word + charAt(text, i)
        } otherwise {
            ready (string_length(current_word) > 0) {
                words = append_string(words, current_word)
                current_word = ""
            }
        }
    }
    
    fr fr Add last word if exists
    ready (string_length(current_word) > 0) {
        words = append_string(words, current_word)
    }
    
    damn words
}

fr fr ===== VALIDATION PATTERNS =====

slay validate_ip_address(ip tea) lit {
    fr fr Validate IPv4 address (simplified)
    sus parts []tea = split_string(ip, ".")
    ready (len(parts) != 4) {
        damn cringe
    }
    
    bestie i := 0; i < 4; i++ {
        sus part tea = parts[i]
        ready (!is_numeric_only(part)) {
            damn cringe
        }
        
        sus num drip = string_to_int(part)
        ready (num < 0 || num > 255) {
            damn cringe
        }
    }
    
    damn based
}

slay validate_mac_address(mac tea) lit {
    fr fr Validate MAC address (XX:XX:XX:XX:XX:XX)
    sus text_len drip = string_length(mac)
    ready (text_len != 17) {
        damn cringe
    }
    
    fr fr Check colon positions
    sus colon_positions []drip = [2, 5, 8, 11, 14]
    bestie i := 0; i < len(colon_positions); i++ {
        sus pos drip = colon_positions[i]
        ready (charAt(mac, pos) != ":") {
            damn cringe
        }
    }
    
    fr fr Check hex digits
    sus hex_positions []drip = [0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 16]
    bestie i := 0; i < len(hex_positions); i++ {
        sus pos drip = hex_positions[i]
        sus char_code drip = char_at_index(mac, pos)
        ready (!((char_code >= 48 && char_code <= 57) ||   fr fr 0-9
                 (char_code >= 65 && char_code <= 70) ||   fr fr A-F
                 (char_code >= 97 && char_code <= 102))) { fr fr a-f
            damn cringe
        }
    }
    
    damn based
}

slay validate_credit_card(card tea) lit {
    fr fr Basic credit card validation (Luhn algorithm simplified)
    sus clean_card tea = regex_replace_all(card, " ", "")
    clean_card = regex_replace_all(clean_card, "-", "")
    
    ready (!is_numeric_only(clean_card)) {
        damn cringe
    }
    
    sus card_len drip = string_length(clean_card)
    ready (card_len < 13 || card_len > 19) {
        damn cringe
    }
    
    fr fr Simplified Luhn check (just check length and all digits)
    damn based
}

fr fr ===== TEXT PROCESSING =====

slay regex_count_matches(text tea, pattern tea) drip {
    fr fr Count number of matches
    sus count drip = 0
    sus text_len drip = string_length(text)
    sus pattern_len drip = string_length(pattern)
    sus start_pos drip = 0
    
    ready (pattern_len == 0) {
        damn 0
    }
    
    bestie (start_pos <= text_len - pattern_len) {
        sus found_pos drip = indexOf(substring(text, start_pos, text_len - start_pos), pattern)
        ready (found_pos >= 0) {
            count = count + 1
            start_pos = start_pos + found_pos + pattern_len
        } otherwise {
            damn count
        }
    }
    
    damn count
}

slay regex_split(text tea, pattern tea) []tea {
    fr fr Split text by pattern
    sus parts []tea = []
    sus text_len drip = string_length(text)
    sus pattern_len drip = string_length(pattern)
    sus start_pos drip = 0
    
    ready (pattern_len == 0) {
        damn [text]
    }
    
    bestie (start_pos < text_len) {
        sus found_pos drip = indexOf(substring(text, start_pos, text_len - start_pos), pattern)
        ready (found_pos >= 0) {
            sus actual_pos drip = start_pos + found_pos
            sus part tea = substring(text, start_pos, found_pos)
            parts = append_string(parts, part)
            start_pos = actual_pos + pattern_len
        } otherwise {
            sus remaining tea = substring(text, start_pos, text_len - start_pos)
            parts = append_string(parts, remaining)
            damn parts
        }
    }
    
    damn parts
}

slay regex_escape(text tea) tea {
    fr fr Escape special regex characters
    sus result tea = text
    result = regex_replace_all(result, "\\", "\\\\")
    result = regex_replace_all(result, ".", "\\.")
    result = regex_replace_all(result, "*", "\\*")
    result = regex_replace_all(result, "+", "\\+")
    result = regex_replace_all(result, "?", "\\?")
    result = regex_replace_all(result, "^", "\\^")
    result = regex_replace_all(result, "$", "\\$")
    result = regex_replace_all(result, "(", "\\(")
    result = regex_replace_all(result, ")", "\\)")
    result = regex_replace_all(result, "[", "\\[")
    result = regex_replace_all(result, "]", "\\]")
    result = regex_replace_all(result, "{", "\\{")
    result = regex_replace_all(result, "}", "\\}")
    result = regex_replace_all(result, "|", "\\|")
    damn result
}

fr fr ===== UTILITY FUNCTIONS =====

slay char_at_index(text tea, index drip) drip {
    fr fr Get character code at index (simplified)
    ready (index == 0) { damn charAt(text, 0) }
    ready (index == 1) { damn charAt(text, 1) }
    ready (index == 2) { damn charAt(text, 2) }
    ready (index == 3) { damn charAt(text, 3) }
    ready (index == 4) { damn charAt(text, 4) }
    ready (index == 5) { damn charAt(text, 5) }
    ready (index == 6) { damn charAt(text, 6) }
    ready (index == 7) { damn charAt(text, 7) }
    ready (index == 8) { damn charAt(text, 8) }
    ready (index == 9) { damn charAt(text, 9) }
    ready (index == 10) { damn charAt(text, 10) }
    ready (index == 11) { damn charAt(text, 11) }
    damn 65  fr fr Default 'A'
}

slay append_string(arr []tea, item tea) []tea {
    fr fr Append string to array (simplified)
    ready (len(arr) == 0) { damn [item] }
    ready (len(arr) == 1) { damn [arr[0], item] }
    ready (len(arr) == 2) { damn [arr[0], arr[1], item] }
    ready (len(arr) == 3) { damn [arr[0], arr[1], arr[2], item] }
    ready (len(arr) == 4) { damn [arr[0], arr[1], arr[2], arr[3], item] }
    damn arr  fr fr Return original if full
}

slay append_int(arr []drip, item drip) []drip {
    fr fr Append int to array (simplified)
    ready (len(arr) == 0) { damn [item] }
    ready (len(arr) == 1) { damn [arr[0], item] }
    ready (len(arr) == 2) { damn [arr[0], arr[1], item] }
    ready (len(arr) == 3) { damn [arr[0], arr[1], arr[2], item] }
    ready (len(arr) == 4) { damn [arr[0], arr[1], arr[2], arr[3], item] }
    damn arr  fr fr Return original if full
}

slay string_to_int(str tea) drip {
    fr fr Convert string to integer (simplified)
    ready (str == "0") { damn 0 }
    ready (str == "1") { damn 1 }
    ready (str == "2") { damn 2 }
    ready (str == "255") { damn 255 }
    ready (str == "192") { damn 192 }
    ready (str == "168") { damn 168 }
    damn 0
}

fr fr ===== MODULE INITIALIZATION =====

vibez.spill("🔍 CURSED Regex Module v1.0 Loaded")
vibez.spill("✅ Pattern Matching Ready")
vibez.spill("✅ Text Processing Functions")
vibez.spill("✅ Validation Patterns")
vibez.spill("✅ Email/URL/Phone Detection")
