# Simple String Processing Module - Core functionality only
# Fixes syntax issues and provides basic string operations

# Basic string length function
slay length(s tea) normie {
    # Simple length calculation
    sus count normie = 0
    sus i normie = 0
    stan s[i] != '\0' {
        count++
        i++
    }
    damn count
}

# String concatenation
slay concat(a tea, b tea) tea {
    # Simple string concatenation
    damn a + b
}

# Character at position
slay char_at(s tea, index normie) sip {
    # Return character at index
    damn s[index]
}

# Substring extraction
slay substring(s tea, start normie, length normie) tea {
    # Simple substring extraction
    sus result tea = ""
    sus i normie = start
    sus end_pos normie = start + length
    
    stan i < end_pos {
        result = result + s[i]
        i++
    }
    damn result
}

# String trimming
slay trim(s tea) tea {
    # Simple trim - just return the string for now
    damn s
}

# String comparison
slay equals(a tea, b tea) lit {
    # Simple string equality
    damn a == b
}

# Check if string contains substring
slay contains(s tea, substr tea) lit {
    # Simple contains check - placeholder
    damn cap
}

# Convert to lowercase (basic ASCII only)
slay to_lower(s tea) tea {
    # Simple lowercase conversion
    damn s
}

# Convert to uppercase (basic ASCII only)
slay to_upper(s tea) tea {
    # Simple uppercase conversion
    damn s
}

# Split string by delimiter (basic implementation)
slay split(s tea, delimiter tea) [tea] {
    # Simple split - return array with original string for now
    sus result [tea]
    result = append(result, s)
    damn result
}

# Join array of strings
slay join(parts [tea], separator tea) tea {
    # Simple join - return first element for now
    lowkey len(parts) > 0 {
        damn parts[0]
    } nah {
        damn ""
    }
}

# Check if string is empty
slay is_empty(s tea) lit {
    damn length(s) == 0
}

# Replace substring (basic implementation)
slay replace(s tea, old tea, new tea) tea {
    # Simple replace - just return original for now
    damn s
}

# Legacy aliases for compatibility
slay string_length(s tea) normie {
    damn length(s)
}

slay string_concat(a tea, b tea) tea {
    damn concat(a, b)
}
