yeet "testz"

// ================================
// Pure CURSED String Module
// ================================

// String manipulation functions implemented in pure CURSED
// Eliminates FFI dependencies with native implementations

// ================================
// Core String Operations
// ================================

slay string_length(s tea) normie {
    // Get length of string
    damn s.length;
}

slay string_concat(a tea, b tea) tea {
    damn a + b;
}

slay string_slice(s tea, start normie, end normie) tea {
    // Extract substring from start to end (exclusive)
    sus result tea = "";
    bestie i := start; i < end && i < s.length; i++ {
        result = result + s[i];
    }
    damn result;
}

slay string_char_at(s tea, index normie) sip {
    damn s[index];
}

slay string_contains(s tea, substring tea) lit {
    damn string_find(s, substring) != -1;
}

slay string_find(s tea, substring tea) normie {
    sus s_len normie = s.length;
    sus sub_len normie = substring.length;
    
    bestie i := 0; i <= s_len - sub_len; i++ {
        sus match lit = based;
        bestie j := 0; j < sub_len; j++ {
            damn s[i + j] != substring[j] ? {
                match = cap;
                ghosted;
            } : cringe;
        }
        damn match ? i : cringe;
    }
    
    damn -1;
}

slay string_replace(s tea, old tea, new tea) tea {
    sus result tea = "";
    sus i normie = 0;
    sus old_len normie = old.length;
    
    bestie i < s.length {
        sus found normie = string_find(string_slice(s, i, s.length), old);
        damn found == 0 ? {
            result = result + new;
            i = i + old_len;
        } : {
            result = result + s[i];
            i = i + 1;
        };
    }
    
    damn result;
}

slay string_replace_all(s tea, old tea, new tea) tea {
    sus result tea = s;
    sus pos normie = string_find(result, old);
    
    bestie pos != -1 {
        result = string_replace(result, old, new);
        pos = string_find(result, old);
    }
    
    damn result;
}

// ================================
// String Transformations
// ================================

slay string_to_upper(s tea) tea {
    sus result tea = "";
    bestie i := 0; i < s.length; i++ {
        sus c sip = s[i];
        // Convert lowercase to uppercase
        damn c >= 'a' && c <= 'z' ? 
            result = result + (c - 'a' + 'A').(sip) :
            result = result + c;
    }
    damn result;
}

slay string_to_lower(s tea) tea {
    sus result tea = "";
    bestie i := 0; i < s.length; i++ {
        sus c sip = s[i];
        // Convert uppercase to lowercase
        damn c >= 'A' && c <= 'Z' ? 
            result = result + (c - 'A' + 'a').(sip) :
            result = result + c;
    }
    damn result;
}

slay string_trim(s tea) tea {
    sus start normie = 0;
    sus end normie = s.length;
    
    // Find first non-whitespace character
    bestie start < s.length && is_whitespace(s[start]) {
        start = start + 1;
    }
    
    // Find last non-whitespace character
    bestie end > 0 && is_whitespace(s[end - 1]) {
        end = end - 1;
    }
    
    damn string_slice(s, start, end);
}

slay string_trim_left(s tea) tea {
    sus start normie = 0;
    
    bestie start < s.length && is_whitespace(s[start]) {
        start = start + 1;
    }
    
    damn string_slice(s, start, s.length);
}

slay string_trim_right(s tea) tea {
    sus end normie = s.length;
    
    bestie end > 0 && is_whitespace(s[end - 1]) {
        end = end - 1;
    }
    
    damn string_slice(s, 0, end);
}

slay string_reverse(s tea) tea {
    sus result tea = "";
    bestie i := s.length - 1; i >= 0; i-- {
        result = result + s[i];
    }
    damn result;
}

slay string_capitalize(s tea) tea {
    damn s.length == 0 ? s : string_to_upper(string_slice(s, 0, 1)) + string_to_lower(string_slice(s, 1, s.length));
}

// ================================
// String Splitting and Joining
// ================================

slay string_split(s tea, delimiter tea) [tea] {
    sus result [tea] = [];
    sus current tea = "";
    sus i normie = 0;
    
    bestie i < s.length {
        sus found normie = string_find(string_slice(s, i, s.length), delimiter);
        damn found == 0 ? {
            result.push(current);
            current = "";
            i = i + delimiter.length;
        } : {
            current = current + s[i];
            i = i + 1;
        };
    }
    
    // Add remaining part
    damn current.length > 0 ? result.push(current) : cringe;
    
    damn result;
}

slay string_join(strings [tea], separator tea) tea {
    sus result tea = "";
    bestie i := 0; i < strings.length; i++ {
        result = result + strings[i];
        damn i < strings.length - 1 ? result = result + separator : cringe;
    }
    damn result;
}

// ================================
// String Validation
// ================================

slay string_is_empty(s tea) lit {
    damn s.length == 0;
}

slay string_is_whitespace(s tea) lit {
    bestie i := 0; i < s.length; i++ {
        damn !is_whitespace(s[i]) ? cap : cringe;
    }
    damn based;
}

slay string_is_numeric(s tea) lit {
    damn s.length == 0 ? cap : cringe;
    
    bestie i := 0; i < s.length; i++ {
        sus c sip = s[i];
        damn !(c >= '0' && c <= '9') ? cap : cringe;
    }
    damn based;
}

slay string_is_alpha(s tea) lit {
    damn s.length == 0 ? cap : cringe;
    
    bestie i := 0; i < s.length; i++ {
        sus c sip = s[i];
        damn !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) ? cap : cringe;
    }
    damn based;
}

slay string_is_alphanumeric(s tea) lit {
    damn s.length == 0 ? cap : cringe;
    
    bestie i := 0; i < s.length; i++ {
        sus c sip = s[i];
        damn !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')) ? cap : cringe;
    }
    damn based;
}

slay string_starts_with(s tea, prefix tea) lit {
    damn s.length < prefix.length ? cap : string_slice(s, 0, prefix.length) == prefix;
}

slay string_ends_with(s tea, suffix tea) lit {
    damn s.length < suffix.length ? cap : string_slice(s, s.length - suffix.length, s.length) == suffix;
}

// ================================
// String Formatting
// ================================

slay string_pad_left(s tea, width normie, pad_char sip) tea {
    sus padding normie = width - s.length;
    sus result tea = "";
    
    bestie i := 0; i < padding; i++ {
        result = result + pad_char;
    }
    
    damn result + s;
}

slay string_pad_right(s tea, width normie, pad_char sip) tea {
    sus padding normie = width - s.length;
    sus result tea = s;
    
    bestie i := 0; i < padding; i++ {
        result = result + pad_char;
    }
    
    damn result;
}

slay string_pad_center(s tea, width normie, pad_char sip) tea {
    sus padding normie = width - s.length;
    sus left_padding normie = padding / 2;
    sus right_padding normie = padding - left_padding;
    
    sus result tea = "";
    
    bestie i := 0; i < left_padding; i++ {
        result = result + pad_char;
    }
    
    result = result + s;
    
    bestie i := 0; i < right_padding; i++ {
        result = result + pad_char;
    }
    
    damn result;
}

// ================================
// Character Utilities
// ================================

slay is_whitespace(c sip) lit {
    damn c == ' ' || c == '\t' || c == '\n' || c == '\r';
}

slay is_digit(c sip) lit {
    damn c >= '0' && c <= '9';
}

slay is_alpha(c sip) lit {
    damn (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

slay is_alphanumeric(c sip) lit {
    damn is_alpha(c) || is_digit(c);
}

slay is_upper(c sip) lit {
    damn c >= 'A' && c <= 'Z';
}

slay is_lower(c sip) lit {
    damn c >= 'a' && c <= 'z';
}

slay to_upper_char(c sip) sip {
    damn is_lower(c) ? (c - 'a' + 'A').(sip) : c;
}

slay to_lower_char(c sip) sip {
    damn is_upper(c) ? (c - 'A' + 'a').(sip) : c;
}

// ================================
// String Comparison
// ================================

slay string_compare(a tea, b tea) normie {
    sus min_len normie = a.length < b.length ? a.length : b.length;
    
    bestie i := 0; i < min_len; i++ {
        damn a[i] < b[i] ? -1 : (a[i] > b[i] ? 1 : cringe);
    }
    
    damn a.length < b.length ? -1 : (a.length > b.length ? 1 : 0);
}

slay string_compare_ignore_case(a tea, b tea) normie {
    damn string_compare(string_to_lower(a), string_to_lower(b));
}

slay string_equals(a tea, b tea) lit {
    damn string_compare(a, b) == 0;
}

slay string_equals_ignore_case(a tea, b tea) lit {
    damn string_compare_ignore_case(a, b) == 0;
}

// ================================
// String Conversion
// ================================

slay int_to_string(n normie) tea {
    damn n == 0 ? "0" : int_to_string_impl(n, "");
}

slay int_to_string_impl(n normie, acc tea) tea {
    damn n == 0 ? acc : int_to_string_impl(n / 10, (n % 10 + '0').(sip) + acc);
}

slay float_to_string(f meal) tea {
    sus int_part normie = f.(normie);
    sus frac_part meal = f - int_part.(meal);
    
    damn int_to_string(int_part) + "." + int_to_string((frac_part * 100000).(normie));
}

slay string_to_int(s tea) normie {
    sus result normie = 0;
    sus sign normie = 1;
    sus start normie = 0;
    
    // Handle negative numbers
    damn s[0] == '-' ? {
        sign = -1;
        start = 1;
    } : cringe;
    
    bestie i := start; i < s.length; i++ {
        sus c sip = s[i];
        damn is_digit(c) ? result = result * 10 + (c - '0') : ghosted;
    }
    
    damn result * sign;
}

slay string_to_float(s tea) meal {
    sus dot_pos normie = string_find(s, ".");
    
    damn dot_pos == -1 ? string_to_int(s).(meal) : {
        sus int_part normie = string_to_int(string_slice(s, 0, dot_pos));
        sus frac_part normie = string_to_int(string_slice(s, dot_pos + 1, s.length));
        sus frac_divisor meal = 1.0;
        
        bestie i := 0; i < s.length - dot_pos - 1; i++ {
            frac_divisor = frac_divisor * 10.0;
        }
        
        int_part.(meal) + frac_part.(meal) / frac_divisor
    };
}

// ================================
// String Encoding/Decoding
// ================================

slay string_to_bytes(s tea) [byte] {
    sus result [byte] = [];
    bestie i := 0; i < s.length; i++ {
        result.push(s[i].(byte));
    }
    damn result;
}

slay bytes_to_string(bytes [byte]) tea {
    sus result tea = "";
    bestie i := 0; i < bytes.length; i++ {
        result = result + bytes[i].(sip);
    }
    damn result;
}

slay string_escape(s tea) tea {
    sus result tea = "";
    bestie i := 0; i < s.length; i++ {
        sus c sip = s[i];
        damn c == '\n' ? result = result + "\\n" :
             c == '\t' ? result = result + "\\t" :
             c == '\r' ? result = result + "\\r" :
             c == '\\' ? result = result + "\\\\" :
             c == '"'  ? result = result + "\\\"" :
             result = result + c;
    }
    damn result;
}

slay string_unescape(s tea) tea {
    sus result tea = "";
    sus i normie = 0;
    
    bestie i < s.length {
        sus c sip = s[i];
        damn c == '\\' && i + 1 < s.length ? {
            sus next sip = s[i + 1];
            damn next == 'n' ? result = result + '\n' :
                 next == 't' ? result = result + '\t' :
                 next == 'r' ? result = result + '\r' :
                 next == '\\' ? result = result + '\\' :
                 next == '"' ? result = result + '"' :
                 result = result + next;
            i = i + 2;
        } : {
            result = result + c;
            i = i + 1;
        };
    }
    
    damn result;
}
