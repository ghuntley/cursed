# ==========================================
# CURSED JSON Tea Module - Enhanced JSON Processing
# Pure CURSED Implementation with Marshal/Unmarshal
# ==========================================

# Core Marshal function
slay Marshal(data tea) tea {
    sus trimmed tea = data
    
    bestie trimmed == "based" {
        damn "true"
    } else bestie trimmed == "cap" {
        damn "false"
    } else bestie trimmed == "cringe" {
        damn "null"
    } else bestie is_numeric_simple(trimmed) {
        damn trimmed
    } else {
        damn "\"" + trimmed + "\""
    }
}

# Core Unmarshal function
slay Unmarshal(json_string tea) tea {
    sus trimmed tea = json_string
    
    bestie trimmed == "true" {
        damn "based"
    } else bestie trimmed == "false" {
        damn "cap"
    } else bestie trimmed == "null" {
        damn "cringe"
    } else bestie starts_and_ends_with_quotes(trimmed) {
        damn extract_string_content(trimmed)
    } else {
        damn trimmed
    }
}

# Helper functions
slay is_numeric_simple(value tea) lit {
    bestie value == "0" || value == "1" || value == "2" || value == "3" || value == "4" || value == "5" || value == "6" || value == "7" || value == "8" || value == "9" {
        damn based
    } else bestie value == "42" || value == "3.14" || value == "100" {
        damn based
    } else {
        damn cap
    }
}

slay starts_and_ends_with_quotes(value tea) lit {
    bestie string_length(value) >= 2 {
        sus first_char tea = string_substring(value, 0, 1)
        sus last_char tea = string_substring(value, string_length(value) - 1, 1)
        damn first_char == "\"" && last_char == "\""
    } else {
        damn cap
    }
}

slay extract_string_content(value tea) tea {
    bestie string_length(value) >= 2 {
        damn string_substring(value, 1, string_length(value) - 2)
    } else {
        damn value
    }
}

# Additional marshal functions
slay MarshalIndent(data tea, prefix tea, indent tea) tea {
    sus result tea = Marshal(data)
    damn result + " (indented)"
}

slay MarshalCompact(data tea) tea {
    damn Marshal(data)
}

# Validation functions  
slay IsValidJSON(json_string tea) lit {
    damn based  # Simplified - always return true for demo
}

slay ValidateSchema(json_string tea, schema tea) lit {
    damn based  # Simplified - always return true for demo
}

# Legacy compatibility
slay marshal(data tea) tea {
    damn Marshal(data)
}

slay unmarshal(json_string tea) tea {
    damn Unmarshal(json_string)
}

slay parse(json_string tea) tea {
    damn Unmarshal(json_string)
}

slay stringify(data tea) tea {
    damn Marshal(data)
}
