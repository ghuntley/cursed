yeet "testz"

fr fr ==========================================
fr fr CURSED JSON Module (jsonz) - Enhanced JSON Processing
fr fr RFC 7159 Compliant with Advanced Features
fr fr ==========================================

fr fr ==========================================
fr fr Core JSON Processing Functions
fr fr ==========================================

slay parse_json(json_string tea) tea { 
    fr fr Parse JSON string to CURSED data structures
    sus trimmed tea = string_trim(json_string)
    bestie string_length(trimmed) == 0 {
        damn "ERROR: Empty JSON string"
    }
    
    fr fr Validate JSON structure first
    bestie !is_valid_json_structure(trimmed) {
        damn "ERROR: Malformed JSON - invalid structure"
    }
    
    damn parse_value_enhanced(trimmed)
}

slay stringify_json(obj tea) tea {
    fr fr Convert CURSED data to JSON string
    bestie string_length(obj) == 0 {
        damn "null"
    }
    
    fr fr Handle error values
    bestie string_starts_with(obj, "ERROR:") {
        damn "null"
    }
    
    fr fr Handle objects
    bestie string_starts_with(obj, "{") {
        damn stringify_object(obj)
    }
    
    fr fr Handle arrays  
    bestie string_starts_with(obj, "[") {
        damn stringify_array(obj)
    }
    
    fr fr Handle primitive values
    damn stringify_primitive(obj)
}

slay json_get(obj tea, key tea) tea {
    fr fr Get value from JSON object
    bestie !string_starts_with(obj, "{") || !string_ends_with(obj, "}") {
        damn "ERROR: Not a JSON object"
    }
    
    fr fr Extract object content
    sus content tea = string_substring(obj, 1, string_length(obj) - 2)
    content = string_trim(content)
    
    fr fr Handle empty object
    bestie string_length(content) == 0 {
        damn "ERROR: Key not found"
    }
    
    fr fr Parse key-value pairs
    sus pairs []tea = parse_object_pairs(content)
    sus i normie = 0
    
    bestie i < len(pairs) {
        sus pair tea = pairs[i]
        sus key_value []tea = split_key_value(pair)
        
        bestie len(key_value) == 2 {
            sus found_key tea = string_trim(key_value[0])
            sus found_value tea = string_trim(key_value[1])
            
            fr fr Remove quotes from key
            bestie string_starts_with(found_key, "\"") && string_ends_with(found_key, "\"") {
                found_key = string_substring(found_key, 1, string_length(found_key) - 2)
            }
            
            bestie found_key == key {
                damn found_value
            }
        }
        
        i = i + 1
    }
    
    damn "ERROR: Key not found"
}

slay json_set(obj tea, key tea, value tea) tea {
    fr fr Set value in JSON object
    bestie !string_starts_with(obj, "{") || !string_ends_with(obj, "}") {
        damn "ERROR: Not a JSON object"
    }
    
    fr fr Extract object content
    sus content tea = string_substring(obj, 1, string_length(obj) - 2)
    content = string_trim(content)
    
    fr fr Format new key-value pair
    sus formatted_key tea = "\"" + key + "\""
    sus formatted_value tea = stringify_json(value)
    sus new_pair tea = formatted_key + ": " + formatted_value
    
    fr fr Handle empty object
    bestie string_length(content) == 0 {
        damn "{" + new_pair + "}"
    }
    
    fr fr Parse existing pairs
    sus pairs []tea = parse_object_pairs(content)
    sus new_pairs []tea = []
    sus key_found lit = cap
    sus i normie = 0
    
    bestie i < len(pairs) {
        sus pair tea = pairs[i]
        sus key_value []tea = split_key_value(pair)
        
        bestie len(key_value) == 2 {
            sus found_key tea = string_trim(key_value[0])
            
            fr fr Remove quotes from key
            bestie string_starts_with(found_key, "\"") && string_ends_with(found_key, "\"") {
                found_key = string_substring(found_key, 1, string_length(found_key) - 2)
            }
            
            bestie found_key == key {
                fr fr Replace existing key
                array_push(new_pairs, new_pair)
                key_found = based
            } else {
                fr fr Keep existing pair
                array_push(new_pairs, pair)
            }
        }
        
        i = i + 1
    }
    
    fr fr Add new key if not found
    bestie !key_found {
        array_push(new_pairs, new_pair)
    }
    
    fr fr Reconstruct object
    sus result tea = "{"
    sus j normie = 0
    bestie j < len(new_pairs) {
        bestie j > 0 {
            result = result + ", "
        }
        result = result + new_pairs[j]
        j = j + 1
    }
    result = result + "}"
    
    damn result
}

slay json_array_push(arr tea, value tea) tea {
    fr fr Add value to JSON array
    bestie !string_starts_with(arr, "[") || !string_ends_with(arr, "]") {
        damn "ERROR: Not a JSON array"
    }
    
    fr fr Extract array content
    sus content tea = string_substring(arr, 1, string_length(arr) - 2)
    content = string_trim(content)
    
    fr fr Format new value
    sus formatted_value tea = stringify_json(value)
    
    fr fr Handle empty array
    bestie string_length(content) == 0 {
        damn "[" + formatted_value + "]"
    }
    
    fr fr Add to existing array
    damn "[" + content + ", " + formatted_value + "]"
}

slay json_array_length(arr tea) normie {
    fr fr Get JSON array length
    bestie !string_starts_with(arr, "[") || !string_ends_with(arr, "]") {
        damn -1 fr fr Error indicator
    }
    
    fr fr Extract array content
    sus content tea = string_substring(arr, 1, string_length(arr) - 2)
    content = string_trim(content)
    
    fr fr Handle empty array
    bestie string_length(content) == 0 {
        damn 0
    }
    
    fr fr Count elements by commas (simplified)
    sus count normie = 1
    sus i normie = 0
    sus in_string lit = cap
    sus brace_depth normie = 0
    
    bestie i < string_length(content) {
        sus char sip = string_char_at(content, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\"" && (i == 0 || string_char_at(content, i - 1) != '\\') {
            in_string = !in_string
        } else bestie !in_string {
            bestie char_str == "{" || char_str == "[" {
                brace_depth = brace_depth + 1
            } else bestie char_str == "}" || char_str == "]" {
                brace_depth = brace_depth - 1
            } else bestie char_str == "," && brace_depth == 0 {
                count = count + 1
            }
        }
        
        i = i + 1
    }
    
    damn count
}

fr fr ==========================================
fr fr Enhanced Parsing Functions
fr fr ==========================================

slay parse_value_enhanced(json_string tea) tea {
    fr fr Enhanced value parser with better error handling
    sus trimmed tea = string_trim(json_string)
    
    fr fr Handle objects
    bestie string_starts_with(trimmed, "{") {
        damn parse_object_enhanced(trimmed)
    }
    
    fr fr Handle arrays
    bestie string_starts_with(trimmed, "[") {
        damn parse_array_enhanced(trimmed)
    }
    
    fr fr Handle strings
    bestie string_starts_with(trimmed, "\"") {
        bestie !string_ends_with(trimmed, "\"") || string_length(trimmed) < 2 {
            damn "ERROR: Unterminated string"
        }
        damn unescape_json_string(string_substring(trimmed, 1, string_length(trimmed) - 2))
    }
    
    fr fr Handle numbers
    bestie is_numeric_enhanced(trimmed) {
        damn trimmed
    }
    
    fr fr Handle booleans
    bestie trimmed == "true" || trimmed == "false" {
        damn trimmed
    }
    
    fr fr Handle null
    bestie trimmed == "null" {
        damn "null"
    }
    
    damn "ERROR: Invalid JSON value: " + trimmed
}

slay parse_object_enhanced(json_string tea) tea {
    fr fr Enhanced object parser with validation
    sus trimmed tea = string_trim(json_string)
    
    bestie !string_starts_with(trimmed, "{") || !string_ends_with(trimmed, "}") {
        damn "ERROR: Invalid object format - missing braces"
    }
    
    sus content tea = string_trim(string_substring(trimmed, 1, string_length(trimmed) - 2))
    
    fr fr Handle empty object
    bestie string_length(content) == 0 {
        damn "{}"
    }
    
    fr fr Validate object pairs
    sus pairs []tea = parse_object_pairs(content)
    sus valid_pairs []tea = []
    sus i normie = 0
    
    bestie i < len(pairs) {
        sus pair tea = string_trim(pairs[i])
        sus key_value []tea = split_key_value(pair)
        
        bestie len(key_value) != 2 {
            damn "ERROR: Invalid key-value pair: " + pair
        }
        
        sus key tea = string_trim(key_value[0])
        sus value tea = string_trim(key_value[1])
        
        fr fr Validate key format
        bestie !string_starts_with(key, "\"") || !string_ends_with(key, "\"") {
            damn "ERROR: Object key must be string: " + key
        }
        
        fr fr Validate value
        sus parsed_value tea = parse_value_enhanced(value)
        bestie string_starts_with(parsed_value, "ERROR:") {
            damn parsed_value
        }
        
        array_push(valid_pairs, pair)
        i = i + 1
    }
    
    fr fr Reconstruct object
    sus result tea = "{"
    sus j normie = 0
    bestie j < len(valid_pairs) {
        bestie j > 0 {
            result = result + ", "
        }
        result = result + valid_pairs[j]
        j = j + 1
    }
    result = result + "}"
    
    damn result
}

slay parse_array_enhanced(json_string tea) tea {
    fr fr Enhanced array parser with validation
    sus trimmed tea = string_trim(json_string)
    
    bestie !string_starts_with(trimmed, "[") || !string_ends_with(trimmed, "]") {
        damn "ERROR: Invalid array format - missing brackets"
    }
    
    sus content tea = string_trim(string_substring(trimmed, 1, string_length(trimmed) - 2))
    
    fr fr Handle empty array
    bestie string_length(content) == 0 {
        damn "[]"
    }
    
    fr fr Parse and validate elements
    sus elements []tea = parse_array_elements(content)
    sus valid_elements []tea = []
    sus i normie = 0
    
    bestie i < len(elements) {
        sus element tea = string_trim(elements[i])
        sus parsed_element tea = parse_value_enhanced(element)
        
        bestie string_starts_with(parsed_element, "ERROR:") {
            damn parsed_element
        }
        
        array_push(valid_elements, stringify_json(parsed_element))
        i = i + 1
    }
    
    fr fr Reconstruct array
    sus result tea = "["
    sus j normie = 0
    bestie j < len(valid_elements) {
        bestie j > 0 {
            result = result + ", "
        }
        result = result + valid_elements[j]
        j = j + 1
    }
    result = result + "]"
    
    damn result
}

fr fr ==========================================
fr fr Validation Functions
fr fr ==========================================

slay is_valid_json_structure(json_string tea) lit {
    fr fr Comprehensive JSON structure validation
    sus trimmed tea = string_trim(json_string)
    
    bestie string_length(trimmed) == 0 {
        damn cap
    }
    
    fr fr Check for balanced braces and brackets
    sus brace_count normie = 0
    sus bracket_count normie = 0
    sus in_string lit = cap
    sus i normie = 0
    
    bestie i < string_length(trimmed) {
        sus char sip = string_char_at(trimmed, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\"" && (i == 0 || string_char_at(trimmed, i - 1) != '\\') {
            in_string = !in_string
        } else bestie !in_string {
            bestie char_str == "{" {
                brace_count = brace_count + 1
            } else bestie char_str == "}" {
                brace_count = brace_count - 1
                bestie brace_count < 0 {
                    damn cap fr fr Unmatched closing brace
                }
            } else bestie char_str == "[" {
                bracket_count = bracket_count + 1
            } else bestie char_str == "]" {
                bracket_count = bracket_count - 1
                bestie bracket_count < 0 {
                    damn cap fr fr Unmatched closing bracket
                }
            }
        }
        
        i = i + 1
    }
    
    damn brace_count == 0 && bracket_count == 0 && !in_string
}

slay is_numeric_enhanced(value tea) lit {
    fr fr Enhanced numeric validation with JSON number rules
    bestie string_length(value) == 0 {
        damn cap
    }
    
    sus i normie = 0
    sus has_dot lit = cap
    sus has_exp lit = cap
    sus has_sign lit = cap
    
    fr fr Handle leading negative sign
    bestie string_char_at(value, 0) == '-' {
        i = 1
        has_sign = based
        bestie string_length(value) == 1 {
            damn cap fr fr Just a minus sign
        }
    }
    
    fr fr Must have at least one digit
    bestie i >= string_length(value) {
        damn cap
    }
    
    fr fr Check for leading zero rule
    sus first_digit sip = string_char_at(value, i)
    bestie first_digit == '0' && i + 1 < string_length(value) {
        sus next_char sip = string_char_at(value, i + 1)
        bestie next_char != '.' && next_char != 'e' && next_char != 'E' {
            damn cap fr fr Leading zero not allowed except for 0.x or 0e/0E
        }
    }
    
    bestie i < string_length(value) {
        sus char sip = string_char_at(value, i)
        sus char_str tea = string_from_char(char)
        
        bestie is_digit_char(char) {
            fr fr Valid digit
        } else bestie char_str == "." {
            bestie has_dot || has_exp {
                damn cap fr fr Multiple dots or dot after exponent
            }
            has_dot = based
        } else bestie char_str == "e" || char_str == "E" {
            bestie has_exp {
                damn cap fr fr Multiple exponents
            }
            has_exp = based
            fr fr Check for sign after exponent
            bestie i + 1 < string_length(value) {
                sus next_char sip = string_char_at(value, i + 1)
                bestie next_char == '+' || next_char == '-' {
                    i = i + 1 fr fr Skip the sign
                }
            }
        } else {
            damn cap fr fr Invalid character
        }
        
        i = i + 1
    }
    
    damn based
}

fr fr ==========================================
fr fr Utility Functions
fr fr ==========================================

slay stringify_object(obj tea) tea {
    fr fr Stringify a CURSED object representation
    bestie !string_starts_with(obj, "{") || !string_ends_with(obj, "}") {
        damn "null"
    }
    
    fr fr Objects are already in JSON format, just validate
    bestie is_valid_json_structure(obj) {
        damn obj
    }
    
    damn "null"
}

slay stringify_array(arr tea) tea {
    fr fr Stringify a CURSED array representation
    bestie !string_starts_with(arr, "[") || !string_ends_with(arr, "]") {
        damn "null"
    }
    
    fr fr Arrays are already in JSON format, just validate
    bestie is_valid_json_structure(arr) {
        damn arr
    }
    
    damn "null"
}

slay stringify_primitive(value tea) tea {
    fr fr Stringify primitive values
    bestie value == "true" || value == "false" || value == "null" {
        damn value
    }
    
    bestie is_numeric_enhanced(value) {
        damn value
    }
    
    fr fr Default to escaped string
    damn "\"" + escape_json_string(value) + "\""
}

slay parse_object_pairs(content tea) []tea {
    fr fr Parse object content into key-value pairs
    sus pairs []tea = []
    sus current_pair tea = ""
    sus brace_depth normie = 0
    sus bracket_depth normie = 0
    sus in_string lit = cap
    sus i normie = 0
    
    bestie i < string_length(content) {
        sus char sip = string_char_at(content, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\"" && (i == 0 || string_char_at(content, i - 1) != '\\') {
            in_string = !in_string
        }
        
        bestie !in_string {
            bestie char_str == "{" {
                brace_depth = brace_depth + 1
            } else bestie char_str == "}" {
                brace_depth = brace_depth - 1
            } else bestie char_str == "[" {
                bracket_depth = bracket_depth + 1
            } else bestie char_str == "]" {
                bracket_depth = bracket_depth - 1
            } else bestie char_str == "," && brace_depth == 0 && bracket_depth == 0 {
                bestie string_length(string_trim(current_pair)) > 0 {
                    array_push(pairs, current_pair)
                }
                current_pair = ""
                i = i + 1
                current_pair = current_pair + char_str
            }
        }
        
        current_pair = current_pair + char_str
        i = i + 1
    }
    
    bestie string_length(string_trim(current_pair)) > 0 {
        array_push(pairs, current_pair)
    }
    
    damn pairs
}

slay parse_array_elements(content tea) []tea {
    fr fr Parse array content into elements
    sus elements []tea = []
    sus current_element tea = ""
    sus brace_depth normie = 0
    sus bracket_depth normie = 0
    sus in_string lit = cap
    sus i normie = 0
    
    bestie i < string_length(content) {
        sus char sip = string_char_at(content, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\"" && (i == 0 || string_char_at(content, i - 1) != '\\') {
            in_string = !in_string
        }
        
        bestie !in_string {
            bestie char_str == "{" {
                brace_depth = brace_depth + 1
            } else bestie char_str == "}" {
                brace_depth = brace_depth - 1
            } else bestie char_str == "[" {
                bracket_depth = bracket_depth + 1
            } else bestie char_str == "]" {
                bracket_depth = bracket_depth - 1
            } else bestie char_str == "," && brace_depth == 0 && bracket_depth == 0 {
                bestie string_length(string_trim(current_element)) > 0 {
                    array_push(elements, current_element)
                }
                current_element = ""
                i = i + 1
                current_element = current_element + char_str
            }
        }
        
        current_element = current_element + char_str
        i = i + 1
    }
    
    bestie string_length(string_trim(current_element)) > 0 {
        array_push(elements, current_element)
    }
    
    damn elements
}

slay split_key_value(pair tea) []tea {
    fr fr Split key-value pair by colon
    sus result []tea = []
    sus colon_pos normie = -1
    sus in_string lit = cap
    sus i normie = 0
    
    fr fr Find the first colon outside of strings
    bestie i < string_length(pair) {
        sus char sip = string_char_at(pair, i)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == "\"" && (i == 0 || string_char_at(pair, i - 1) != '\\') {
            in_string = !in_string
        } else bestie char_str == ":" && !in_string {
            colon_pos = i
            i = string_length(pair) fr fr Exit loop
        }
        
        i = i + 1
    }
    
    bestie colon_pos >= 0 {
        sus key tea = string_substring(pair, 0, colon_pos)
        sus value tea = string_substring(pair, colon_pos + 1, string_length(pair) - colon_pos - 1)
        array_push(result, key)
        array_push(result, value)
    }
    
    damn result
}

slay is_digit_char(char sip) lit {
    fr fr Check if character is a digit
    damn char >= '0' && char <= '9'
}

slay escape_json_string(input tea) tea {
    fr fr Escape special characters for JSON
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
        } else bestie char_str == "\b" {
            result = result + "\\b"
        } else bestie char_str == "\f" {
            result = result + "\\f"
        } else {
            result = result + char_str
        }
        
        i = i + 1
    }
    
    damn result
}

slay unescape_json_string(input tea) tea {
    fr fr Unescape JSON string
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
            } else bestie next_str == "\\" {
                result = result + "\\"
            } else bestie next_str == "n" {
                result = result + "\n"
            } else bestie next_str == "t" {
                result = result + "\t"
            } else bestie next_str == "r" {
                result = result + "\r"
            } else bestie next_str == "b" {
                result = result + "\b"
            } else bestie next_str == "f" {
                result = result + "\f"
            } else bestie next_str == "/" {
                result = result + "/"
            } else {
                result = result + char_str
                result = result + next_str
            }
            
            i = i + 1 fr fr Skip next character
        } else {
            result = result + char_str
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr ==========================================
fr fr String Utility Functions
fr fr ==========================================

slay string_trim(input tea) tea {
    fr fr Trim whitespace from both ends
    sus start normie = 0
    sus end normie = string_length(input)
    
    bestie start < end {
        sus char sip = string_char_at(input, start)
        sus char_str tea = string_from_char(char)
        
        bestie char_str == " " || char_str == "\t" || char_str == "\n" || char_str == "\r" {
            start = start + 1
        } else {
            start = end fr fr Exit loop
        }
    }
    
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

slay string_starts_with(input tea, prefix tea) lit {
    fr fr Check if string starts with prefix
    bestie string_length(prefix) > string_length(input) {
        damn cap
    }
    
    sus prefix_part tea = string_substring(input, 0, string_length(prefix))
    damn prefix_part == prefix
}

slay string_ends_with(input tea, suffix tea) lit {
    fr fr Check if string ends with suffix
    bestie string_length(suffix) > string_length(input) {
        damn cap
    }
    
    sus start_pos normie = string_length(input) - string_length(suffix)
    sus suffix_part tea = string_substring(input, start_pos, string_length(suffix))
    damn suffix_part == suffix
}

fr fr ==========================================
fr fr Array Helper Functions (Pure CURSED)
fr fr ==========================================

slay array_push(arr []tea, value tea) normie {
    fr fr Use CURSED append to add to array
    sus new_arr []tea = append(arr, value)
    damn len(new_arr)
}

slay json_array_length(arr []tea) normie {
    fr fr Return actual array length
    damn len(arr)
}

fr fr ==========================================
fr fr String Functions (Pure CURSED)
fr fr ==========================================

slay string_length(str tea) normie {
    fr fr Count characters in string
    sus count normie = 0
    sus i normie = 0
    bestie runtime_string_char_at(str, i) != '\0' {
        count = count + 1
        i = i + 1
    }
    damn count
}

slay string_char_at(str tea, index normie) sip {
    fr fr Get character at index using runtime
    damn runtime_string_char_at(str, index)
}

slay string_from_char(char sip) tea {
    fr fr Convert character to string using runtime
    damn runtime_char_to_string(char)
}

slay string_substring(str tea, start normie, length normie) tea {
    fr fr Extract substring using pure CURSED
    sus result tea = ""
    sus i normie = start
    sus end_pos normie = start + length
    bestie i < end_pos {
        sus c sip = runtime_string_char_at(str, i)
        lowkey c == '\0' { break }
        result = result + runtime_char_to_string(c)
        i = i + 1
    }
    damn result
}
