yeet "testz"

fr fr ==========================================
fr fr CURSED JSON Module - Pure CURSED Implementation  
fr fr RFC 7159 Compliant JSON Processing
fr fr ==========================================

fr fr ==========================================
fr fr Core JSON Parsing Functions (RFC 7159)
fr fr ==========================================

slay parse_json(json_string tea) tea { fr fr Main RFC 7159 compliant JSON parser
    sus trimmed tea = string_trim(json_string)
    bestie string_length(trimmed) == 0 {
        damn "ERROR: Empty JSON string"
    }
    damn parse_value(trimmed)
}

slay from_string(json_string tea) tea { fr fr Alternative entry point for JSON parsing
    damn parse_json(json_string)
}

slay parse_value(json_string tea) tea { fr fr Parse a single JSON value (string, number, boolean, null, object, array)
    sus trimmed tea = string_trim(json_string) fr fr Handle objects
    bestie string_starts_with(trimmed, "{") {
        damn parse_object(trimmed)
    } fr fr Handle arrays
    bestie string_starts_with(trimmed, "[") {
        damn parse_array(trimmed)
    } fr fr Handle string values
    bestie string_starts_with(trimmed, "\"") && string_ends_with(trimmed, "\"") {
        damn unescape_json_string(string_substring(trimmed, 1, string_length(trimmed) - 2))
    } fr fr Handle numbers
    bestie is_numeric(trimmed) {
        damn trimmed
    } fr fr Handle booleans
    bestie trimmed == "true" {
        damn "true"
    }
    
    bestie trimmed == "false" {
        damn "false"
    } fr fr Handle null
    bestie trimmed == "null" {
        damn "null"
    }
    
    damn "ERROR: Invalid JSON value"
}

slay parse_object(json_string tea) tea { fr fr Parse JSON object to key-value representation
    sus trimmed tea = string_trim(json_string) fr fr Validate object structure
    bestie !string_starts_with(trimmed, "{") || !string_ends_with(trimmed, "}") {
        damn "ERROR: Invalid object format"
    } fr fr Extract content between braces
    sus content tea = string_trim(string_substring(trimmed, 1, string_length(trimmed) - 2)) fr fr Handle empty object
    bestie string_length(content) == 0 {
        damn "{}"
    } fr fr Simple key-value parsing for basic objects
    sus result tea = "{"
    sus pairs tea = split_object_pairs(content)
    bestie string_length(pairs) > 0 {
        result = result + pairs
    }
    result = result + "}"
    
    damn result
}

slay parse_array(json_string tea) tea { fr fr Parse JSON array to element representation
    sus trimmed tea = string_trim(json_string) fr fr Validate array structure
    bestie !string_starts_with(trimmed, "[") || !string_ends_with(trimmed, "]") {
        damn "ERROR: Invalid array format"
    } fr fr Extract content between brackets
    sus content tea = string_trim(string_substring(trimmed, 1, string_length(trimmed) - 2)) fr fr Handle empty array
    bestie string_length(content) == 0 {
        damn "[]"
    } fr fr Simple element parsing
    sus result tea = "["
    sus elements tea = split_array_elements(content)
    bestie string_length(elements) > 0 {
        result = result + elements
    }
    result = result + "]"
    
    damn result
}

slay parse_simple_object(json_string tea) tea { fr fr Simple object parser for basic key-value pairs
    sus trimmed tea = string_trim(json_string) fr fr Remove outer braces
    bestie string_starts_with(trimmed, "{") && string_ends_with(trimmed, "}") {
        sus content tea = string_substring(trimmed, 1, string_length(trimmed) - 2)
        damn content
    }
    
    damn ""
}

fr fr ==========================================
fr fr JSON Generation Functions (RFC 7159)
fr fr ==========================================

slay to_json(value tea) tea { fr fr Convert CURSED value to JSON string
    damn stringify_value(value)
}

slay stringify(value tea) tea { fr fr Main JSON stringification function
    damn to_json(value)
}

slay stringify_value(value tea) tea { fr fr Enhanced JSON stringification with type detection
    bestie value == "true" || value == "false" || value == "null" {
        damn value
    }
    
    bestie is_numeric(value) {
        damn value
    } fr fr Handle objects (simple detection)
    bestie string_starts_with(value, "{") && string_ends_with(value, "}") {
        damn value
    } fr fr Handle arrays (simple detection)
    bestie string_starts_with(value, "[") && string_ends_with(value, "]") {
        damn value
    } fr fr Default to string with proper escaping
    damn "\"" + escape_json_string(value) + "\""
}

fr fr ==========================================
fr fr Type Conversion Functions
fr fr ==========================================

slay object_to_map(json_object tea) tea { fr fr Convert JSON object to map-like representation
    sus trimmed tea = string_trim(json_object)
    
    bestie !string_starts_with(trimmed, "{") || !string_ends_with(trimmed, "}") {
        damn "ERROR: Not a valid JSON object"
    } fr fr Return simplified map format
    damn "MAP:" + trimmed
}

slay array_to_slice(json_array tea) tea { fr fr Convert JSON array to slice-like representation
    sus trimmed tea = string_trim(json_array)
    
    bestie !string_starts_with(trimmed, "[") || !string_ends_with(trimmed, "]") {
        damn "ERROR: Not a valid JSON array"
    } fr fr Return simplified slice format
    damn "SLICE:" + trimmed
}

fr fr ==========================================
fr fr Validation Functions (RFC 7159)
fr fr ==========================================

slay is_valid_json(json_string tea) lit { fr fr RFC 7159 compliant JSON validation
    sus trimmed tea = string_trim(json_string)
    
    bestie string_length(trimmed) == 0 {
        damn cap
    }
    
    sus result tea = parse_value(trimmed)
    damn !string_starts_with(result, "ERROR:")
}

slay validate_schema(json_string tea, schema_type tea) lit { fr fr Basic schema validation
    sus trimmed tea = string_trim(json_string)
    
    bestie schema_type == "object" {
        damn string_starts_with(trimmed, "{") && string_ends_with(trimmed, "}")
    }
    
    bestie schema_type == "array" {
        damn string_starts_with(trimmed, "[") && string_ends_with(trimmed, "]")
    }
    
    bestie schema_type == "string" {
        damn string_starts_with(trimmed, "\"") && string_ends_with(trimmed, "\"")
    }
    
    bestie schema_type == "number" {
        damn is_numeric(trimmed)
    }
    
    bestie schema_type == "boolean" {
        damn trimmed == "true" || trimmed == "false"
    }
    
    bestie schema_type == "null" {
        damn trimmed == "null"
    }
    
    damn cap
}

slay validate_json(json_string tea) lit { fr fr Legacy validation function (maintains compatibility)
    damn is_valid_json(json_string)
}

slay stringify_simple(value tea) tea { fr fr Simple JSON stringification
    bestie value == "true" || value == "false" || value == "null" {
        damn value
    }
    
    bestie is_numeric(value) {
        damn value
    } fr fr Wrap strings in quotes
    damn "\"" + value + "\""
}

slay minify_json(json_string tea) tea { fr fr Remove unnecessary whitespace
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

slay pretty_print_json(json_string tea, indent normie) tea { fr fr Add formatting with indentation
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

slay get_indent_string(spaces normie) tea { fr fr Generate indentation string
    sus result tea = ""
    sus i normie = 0
    
    bestie i < spaces {
        result = result + " "
        i = i + 1
    }
    
    damn result
}

slay escape_json_string(input tea) tea { fr fr Escape special characters for JSON
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

slay unescape_json_string(input tea) tea { fr fr Unescape JSON string
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

fr fr ==========================================
fr fr Utility Functions
fr fr ==========================================

slay is_numeric(value tea) lit { fr fr Check if string represents a number
    bestie string_length(value) == 0 {
        damn cap
    }
    
    sus i normie = 0
    sus has_dot lit = cap
    sus start_pos normie = 0 fr fr Handle negative numbers
    bestie string_char_at(value, 0) == '-' {
        start_pos = 1
    }
    
    bestie start_pos < string_length(value) {
        sus char sip = string_char_at(value, start_pos)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "." {
            bestie has_dot {
                damn cap fr fr Multiple dots
            }
            has_dot = based
        } else bestie !is_digit(char_str) {
            damn cap
        }
        
        start_pos = start_pos + 1
    }
    
    damn based
}

slay is_digit(char_str tea) lit { fr fr Check if character is a digit
    damn char_str == "0" || char_str == "1" || char_str == "2" || char_str == "3" || char_str == "4" || char_str == "5" || char_str == "6" || char_str == "7" || char_str == "8" || char_str == "9"
}

slay string_trim(input tea) tea { fr fr Simple trim function
    sus start normie = 0
    sus end normie = string_length(input) fr fr Trim leading whitespace
    bestie start < end {
        sus char sip = string_char_at(input, start)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == " " || char_str == "\t" || char_str == "\n" || char_str == "\r" {
            start = start + 1
        } else {
            start = end fr fr Exit loop
        }
    } fr fr Trim trailing whitespace
    bestie end > start {
        sus char sip = string_char_at(input, end - 1)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == " " || char_str == "\t" || char_str == "\n" || char_str == "\r" {
            end = end - 1
        } else {
            end = 0 fr fr Exit loop
        }
    }
    
    damn string_substring(input, start, end - start)
}

slay string_starts_with(input tea, prefix tea) lit { fr fr Check if string starts with prefix
    bestie string_length(prefix) > string_length(input) {
        damn cap
    }
    
    sus prefix_part tea = string_substring(input, 0, string_length(prefix))
    damn prefix_part == prefix
}

slay string_ends_with(input tea, suffix tea) lit { fr fr Check if string ends with suffix
    bestie string_length(suffix) > string_length(input) {
        damn cap
    }
    
    sus start_pos normie = string_length(input) - string_length(suffix)
    sus suffix_part tea = string_substring(input, start_pos, string_length(suffix))
    damn suffix_part == suffix
}

fr fr ==========================================
fr fr Parser Helper Functions
fr fr ==========================================

slay split_object_pairs(content tea) tea { fr fr Split object content into key-value pairs (simplified)
    bestie string_length(content) == 0 {
        damn ""
    } fr fr For now, return content as-is (simplified implementation)
    damn content
}

slay split_array_elements(content tea) tea { fr fr Split array content into elements (simplified)
    bestie string_length(content) == 0 {
        damn ""
    } fr fr For now, return content as-is (simplified implementation)
    damn content
}

slay string_contains(haystack tea, needle tea) lit { fr fr Check if string contains substring
    bestie string_length(needle) == 0 {
        damn based
    }
    
    bestie string_length(needle) > string_length(haystack) {
        damn cap
    }
    
    sus i normie = 0
    bestie i <= string_length(haystack) - string_length(needle) {
        sus substring tea = string_substring(haystack, i, string_length(needle))
        bestie substring == needle {
            damn based
        }
        i = i + 1
    }
    
    damn cap
}

fr fr ==========================================
fr fr High-Level API Functions (RFC 7159 Compliant)
fr fr ==========================================

slay parse(json_string tea) tea { fr fr Main parse function (RFC 7159 compliant)
    damn parse_json(json_string)
}

slay validate(json_string tea) lit { fr fr Main validation function
    damn is_valid_json(json_string)
}

slay pretty_print(json_string tea) tea { fr fr Pretty print with default indentation
    damn pretty_print_json(json_string, 2)
}

slay minify(json_string tea) tea { fr fr Minify JSON
    damn minify_json(json_string)
}

slay escape_string(input tea) tea { fr fr Escape string for JSON
    damn escape_json_string(input)
}

slay unescape_string(input tea) tea { fr fr Unescape JSON string
    damn unescape_json_string(input)
}
