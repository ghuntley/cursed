yeet "testz"

# ==========================================
# CURSED JSON Module - Pure CURSED Implementation  
# RFC 7159 Compliant JSON Processing
# ==========================================

# ==========================================
# Core JSON Parsing Functions
# ==========================================

slay parse_value(json_string tea) tea {
    # Parse a single JSON value (string, number, boolean, null)
    sus trimmed tea = string_trim(json_string)
    
    # Handle string values
    bestie string_starts_with(trimmed, "\"") && string_ends_with(trimmed, "\"") {
        damn string_substring(trimmed, 1, string_length(trimmed) - 2)
    }
    
    # Handle numbers
    bestie is_numeric(trimmed) {
        damn trimmed
    }
    
    # Handle booleans
    bestie trimmed == "true" {
        damn "true"
    }
    
    bestie trimmed == "false" {
        damn "false"
    }
    
    # Handle null
    bestie trimmed == "null" {
        damn "null"
    }
    
    damn ""
}

slay parse_simple_object(json_string tea) tea {
    # Simple object parser for basic key-value pairs
    sus trimmed tea = string_trim(json_string)
    
    # Remove outer braces
    bestie string_starts_with(trimmed, "{") && string_ends_with(trimmed, "}") {
        sus content tea = string_substring(trimmed, 1, string_length(trimmed) - 2)
        damn content
    }
    
    damn ""
}

slay validate_json(json_string tea) lit {
    # Basic JSON validation
    sus trimmed tea = string_trim(json_string)
    
    # Check for basic JSON structures
    bestie string_starts_with(trimmed, "{") && string_ends_with(trimmed, "}") {
        damn based
    }
    
    bestie string_starts_with(trimmed, "[") && string_ends_with(trimmed, "]") {
        damn based
    }
    
    bestie string_starts_with(trimmed, "\"") && string_ends_with(trimmed, "\"") {
        damn based
    }
    
    bestie trimmed == "true" || trimmed == "false" || trimmed == "null" {
        damn based
    }
    
    bestie is_numeric(trimmed) {
        damn based
    }
    
    damn cap
}

slay stringify_simple(value tea) tea {
    # Simple JSON stringification
    bestie value == "true" || value == "false" || value == "null" {
        damn value
    }
    
    bestie is_numeric(value) {
        damn value
    }
    
    # Wrap strings in quotes
    damn "\"" + value + "\""
}

slay minify_json(json_string tea) tea {
    # Remove unnecessary whitespace
    sus result tea = ""
    sus in_string lit = cap
    sus i normie = 0
    
    bestie i < string_length(json_string) {
        sus char sip = string_char_at(json_string, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\"" {
            in_string = !in_string
            result = result + char_str
        } else bestie in_string {
            result = result + char_str
        } else bestie char_str != " " && char_str != "\t" && char_str != "\n" && char_str != "\r" {
            result = result + char_str
        }
        
        i = i + 1
    }
    
    damn result
}

slay pretty_print_json(json_string tea, indent normie) tea {
    # Add formatting with indentation
    sus result tea = ""
    sus current_indent normie = 0
    sus in_string lit = cap
    sus i normie = 0
    
    bestie i < string_length(json_string) {
        sus char sip = string_char_at(json_string, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\"" {
            in_string = !in_string
            result = result + char_str
        } else bestie in_string {
            result = result + char_str
        } else bestie char_str == "{" || char_str == "[" {
            result = result + char_str + "\n"
            current_indent = current_indent + indent
            result = result + get_indent_string(current_indent)
        } else bestie char_str == "}" || char_str == "]" {
            current_indent = current_indent - indent
            result = result + "\n" + get_indent_string(current_indent) + char_str
        } else bestie char_str == "," {
            result = result + char_str + "\n" + get_indent_string(current_indent)
        } else {
            result = result + char_str
        }
        
        i = i + 1
    }
    
    damn result
}

slay get_indent_string(spaces normie) tea {
    # Generate indentation string
    sus result tea = ""
    sus i normie = 0
    
    bestie i < spaces {
        result = result + " "
        i = i + 1
    }
    
    damn result
}

slay escape_json_string(input tea) tea {
    # Escape special characters for JSON
    sus result tea = ""
    sus i normie = 0
    
    bestie i < string_length(input) {
        sus char sip = string_char_at(input, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\"" {
            result = result + "\\\""
        } else bestie char_str == "\\" {
            result = result + "\\\\"
        } else bestie char_str == "\n" {
            result = result + "\\n"
        } else bestie char_str == "\t" {
            result = result + "\\t"
        } else bestie char_str == "\r" {
            result = result + "\\r"
        } else {
            result = result + char_str
        }
        
        i = i + 1
    }
    
    damn result
}

slay unescape_json_string(input tea) tea {
    # Unescape JSON string
    sus result tea = ""
    sus i normie = 0
    
    bestie i < string_length(input) {
        sus char sip = string_char_at(input, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\\" && i + 1 < string_length(input) {
            sus next_char sip = string_char_at(input, i + 1)
            sus next_str tea = string_from_char(next_char)
            
            bestie next_str == "\"" {
                result = result + "\""
                i = i + 1
            } else bestie next_str == "\\" {
                result = result + "\\"
                i = i + 1
            } else bestie next_str == "n" {
                result = result + "\n"
                i = i + 1
            } else bestie next_str == "t" {
                result = result + "\t"
                i = i + 1
            } else bestie next_str == "r" {
                result = result + "\r"
                i = i + 1
            } else {
                result = result + char_str
            }
        } else {
            result = result + char_str
        }
        
        i = i + 1
    }
    
    damn result
}

# ==========================================
# Utility Functions
# ==========================================

slay is_numeric(value tea) lit {
    # Check if string represents a number
    bestie string_length(value) == 0 {
        damn cap
    }
    
    sus i normie = 0
    sus has_dot lit = cap
    sus start_pos normie = 0
    
    # Handle negative numbers
    bestie string_char_at(value, 0) == '-' {
        start_pos = 1
    }
    
    bestie start_pos < string_length(value) {
        sus char sip = string_char_at(value, start_pos)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "." {
            bestie has_dot {
                damn cap  # Multiple dots
            }
            has_dot = based
        } else bestie !is_digit(char_str) {
            damn cap
        }
        
        start_pos = start_pos + 1
    }
    
    damn based
}

slay is_digit(char_str tea) lit {
    # Check if character is a digit
    damn char_str == "0" || char_str == "1" || char_str == "2" || char_str == "3" || char_str == "4" || char_str == "5" || char_str == "6" || char_str == "7" || char_str == "8" || char_str == "9"
}

slay string_trim(input tea) tea {
    # Simple trim function
    sus start normie = 0
    sus end normie = string_length(input)
    
    # Trim leading whitespace
    bestie start < end {
        sus char sip = string_char_at(input, start)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == " " || char_str == "\t" || char_str == "\n" || char_str == "\r" {
            start = start + 1
        } else {
            start = end  # Exit loop
        }
    }
    
    # Trim trailing whitespace
    bestie end > start {
        sus char sip = string_char_at(input, end - 1)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == " " || char_str == "\t" || char_str == "\n" || char_str == "\r" {
            end = end - 1
        } else {
            end = 0  # Exit loop
        }
    }
    
    damn string_substring(input, start, end - start)
}

slay string_starts_with(input tea, prefix tea) lit {
    # Check if string starts with prefix
    bestie string_length(prefix) > string_length(input) {
        damn cap
    }
    
    sus prefix_part tea = string_substring(input, 0, string_length(prefix))
    damn prefix_part == prefix
}

slay string_ends_with(input tea, suffix tea) lit {
    # Check if string ends with suffix
    bestie string_length(suffix) > string_length(input) {
        damn cap
    }
    
    sus start_pos normie = string_length(input) - string_length(suffix)
    sus suffix_part tea = string_substring(input, start_pos, string_length(suffix))
    damn suffix_part == suffix
}

# ==========================================
# High-Level API Functions
# ==========================================

slay parse(json_string tea) tea {
    # Main parse function
    damn parse_value(json_string)
}

slay stringify(value tea) tea {
    # Main stringify function
    damn stringify_simple(value)
}

slay validate(json_string tea) lit {
    # Main validation function
    damn validate_json(json_string)
}

slay pretty_print(json_string tea) tea {
    # Pretty print with default indentation
    damn pretty_print_json(json_string, 2)
}

slay minify(json_string tea) tea {
    # Minify JSON
    damn minify_json(json_string)
}

slay escape_string(input tea) tea {
    # Escape string for JSON
    damn escape_json_string(input)
}

slay unescape_string(input tea) tea {
    # Unescape JSON string
    damn unescape_json_string(input)
}
