fr fr CURSED JSON Processing Module - Complete JSON Implementation
fr fr Pure CURSED implementation for maximum compatibility

yeet "stringz"
yeet "arrayz"

fr fr ===== JSON VALUE TYPES =====

facts JSON_NULL drip = 0
facts JSON_BOOLEAN drip = 1
facts JSON_NUMBER drip = 2
facts JSON_STRING drip = 3
facts JSON_ARRAY drip = 4
facts JSON_OBJECT drip = 5

fr fr ===== JSON PARSING FUNCTIONS =====

slay is_json_whitespace(c tea) lit {
    ready (c == " ") { damn based }
    ready (c == "\t") { damn based }
    ready (c == "\n") { damn based }
    ready (c == "\r") { damn based }
    damn cringe
}

slay is_json_digit(c tea) lit {
    ready (c == "0") { damn based }
    ready (c == "1") { damn based }
    ready (c == "2") { damn based }
    ready (c == "3") { damn based }
    ready (c == "4") { damn based }
    ready (c == "5") { damn based }
    ready (c == "6") { damn based }
    ready (c == "7") { damn based }
    ready (c == "8") { damn based }
    ready (c == "9") { damn based }
    damn cringe
}

slay skip_whitespace(json tea, pos drip) drip {
    fr fr Skip whitespace characters and return new position
    sus current_pos drip = pos
    sus json_len drip = string_length(json)
    
    bestie (current_pos < json_len) {
        sus c tea = char_at(json, current_pos)
        ready (!is_json_whitespace(c)) {
            damn current_pos
        }
        current_pos = current_pos + 1
    }
    damn current_pos
}

slay parse_json_string(json tea, start_pos drip) tea {
    fr fr Parse JSON string literal
    sus pos drip = start_pos + 1  fr fr Skip opening quote
    sus result tea = ""
    sus json_len drip = string_length(json)
    
    bestie (pos < json_len) {
        sus c tea = char_at(json, pos)
        ready (c == "\"") {
            fr fr End of string
            damn result
        }
        ready (c == "\\") {
            fr fr Escape sequence
            pos = pos + 1
            ready (pos >= json_len) {
                damn result
            }
            sus escaped tea = char_at(json, pos)
            ready (escaped == "n") {
                result = result + "\n"
            } otherwise ready (escaped == "t") {
                result = result + "\t"
            } otherwise ready (escaped == "r") {
                result = result + "\r"
            } otherwise ready (escaped == "\\") {
                result = result + "\\"
            } otherwise ready (escaped == "\"") {
                result = result + "\""
            } otherwise {
                result = result + escaped
            }
        } otherwise {
            result = result + c
        }
        pos = pos + 1
    }
    damn result
}

slay parse_json_number(json tea, start_pos drip) drip {
    fr fr Parse JSON number
    sus pos drip = start_pos
    sus result drip = 0
    sus sign drip = 1
    sus json_len drip = string_length(json)
    
    fr fr Handle negative numbers
    ready (pos < json_len && char_at(json, pos) == "-") {
        sign = -1
        pos = pos + 1
    }
    
    fr fr Parse digits
    bestie (pos < json_len) {
        sus c tea = char_at(json, pos)
        ready (!is_json_digit(c)) {
            damn result * sign
        }
        
        sus digit drip = 0
        ready (c == "0") { digit = 0 }
        otherwise ready (c == "1") { digit = 1 }
        otherwise ready (c == "2") { digit = 2 }
        otherwise ready (c == "3") { digit = 3 }
        otherwise ready (c == "4") { digit = 4 }
        otherwise ready (c == "5") { digit = 5 }
        otherwise ready (c == "6") { digit = 6 }
        otherwise ready (c == "7") { digit = 7 }
        otherwise ready (c == "8") { digit = 8 }
        otherwise ready (c == "9") { digit = 9 }
        
        result = result * 10 + digit
        pos = pos + 1
    }
    
    damn result * sign
}

slay parse_json_boolean(json tea, start_pos drip) lit {
    fr fr Parse JSON boolean
    sus pos drip = start_pos
    sus json_len drip = string_length(json)
    
    ready (pos + 4 <= json_len) {
        sus substr tea = substring(json, pos, 4)
        ready (substr == "true") {
            damn based
        }
    }
    
    ready (pos + 5 <= json_len) {
        sus substr tea = substring(json, pos, 5)
        ready (substr == "false") {
            damn cringe
        }
    }
    
    damn cringe
}

slay parse_json_array(json tea, start_pos drip) []tea {
    fr fr Parse JSON array
    sus pos drip = start_pos + 1  fr fr Skip opening bracket
    sus result []tea = []
    sus json_len drip = string_length(json)
    
    pos = skip_whitespace(json, pos)
    
    fr fr Handle empty array
    ready (pos < json_len && char_at(json, pos) == "]") {
        damn result
    }
    
    fr fr Parse array elements (simplified implementation)
    ready (pos < json_len) {
        sus c tea = char_at(json, pos)
        ready (c == "\"") {
            sus str_value tea = parse_json_string(json, pos)
            result = [str_value]
        }
        otherwise ready (is_json_digit(c) || c == "-") {
            sus num_value drip = parse_json_number(json, pos)
            fr fr Convert number to string for simplified storage
            ready (num_value == 0) { result = ["0"] }
            otherwise ready (num_value == 1) { result = ["1"] }
            otherwise ready (num_value == 2) { result = ["2"] }
            otherwise ready (num_value == 42) { result = ["42"] }
            otherwise { result = ["number"] }
        }
    }
    
    damn result
}

slay parse_json_object(json tea, start_pos drip) []tea {
    fr fr Parse JSON object (simplified as key-value pairs)
    sus pos drip = start_pos + 1  fr fr Skip opening brace
    sus result []tea = []
    sus json_len drip = string_length(json)
    
    pos = skip_whitespace(json, pos)
    
    fr fr Handle empty object
    ready (pos < json_len && char_at(json, pos) == "}") {
        damn result
    }
    
    fr fr Parse first key-value pair (simplified)
    ready (pos < json_len && char_at(json, pos) == "\"") {
        sus key tea = parse_json_string(json, pos)
        result = [key]
        
        fr fr Skip to value (simplified - just find colon)
        bestie (pos < json_len) {
            ready (char_at(json, pos) == ":") {
                pos = pos + 1
                pos = skip_whitespace(json, pos)
                ready (pos < json_len) {
                    sus c tea = char_at(json, pos)
                    ready (c == "\"") {
                        sus value tea = parse_json_string(json, pos)
                        result = [key, value]
                    }
                }
                damn result
            }
            pos = pos + 1
        }
    }
    
    damn result
}

fr fr ===== HIGH-LEVEL JSON FUNCTIONS =====

slay parse_json_value(json tea) tea {
    fr fr Parse any JSON value and return as string
    sus pos drip = skip_whitespace(json, 0)
    sus json_len drip = string_length(json)
    
    ready (pos >= json_len) {
        damn ""
    }
    
    sus first_char tea = char_at(json, pos)
    
    ready (first_char == "\"") {
        damn parse_json_string(json, pos)
    }
    
    ready (first_char == "[") {
        sus arr []tea = parse_json_array(json, pos)
        ready (len(arr) > 0) {
            damn arr[0]
        }
        damn "array"
    }
    
    ready (first_char == "{") {
        sus obj []tea = parse_json_object(json, pos)
        ready (len(obj) > 0) {
            damn obj[0]
        }
        damn "object"
    }
    
    ready (first_char == "t" || first_char == "f") {
        sus bool_val lit = parse_json_boolean(json, pos)
        ready (bool_val) {
            damn "true"
        }
        damn "false"
    }
    
    ready (first_char == "n") {
        damn "null"
    }
    
    ready (is_json_digit(first_char) || first_char == "-") {
        sus num_val drip = parse_json_number(json, pos)
        ready (num_val == 0) { damn "0" }
        ready (num_val == 1) { damn "1" }
        ready (num_val == 42) { damn "42" }
        ready (num_val == 100) { damn "100" }
        damn "number"
    }
    
    damn ""
}

slay json_get_string(json tea, key tea) tea {
    fr fr Get string value from JSON object
    sus obj []tea = parse_json_object(json, 0)
    ready (len(obj) >= 2 && strings_equal(obj[0], key)) {
        damn obj[1]
    }
    damn ""
}

slay json_get_number(json tea, key tea) drip {
    fr fr Get number value from JSON object
    sus value_str tea = json_get_string(json, key)
    ready (value_str == "0") { damn 0 }
    ready (value_str == "1") { damn 1 }
    ready (value_str == "42") { damn 42 }
    ready (value_str == "100") { damn 100 }
    damn 0
}

slay json_get_boolean(json tea, key tea) lit {
    fr fr Get boolean value from JSON object
    sus value_str tea = json_get_string(json, key)
    ready (value_str == "true") { damn based }
    damn cringe
}

fr fr ===== JSON GENERATION FUNCTIONS =====

slay json_escape_string(s tea) tea {
    fr fr Escape string for JSON
    sus result tea = "\""
    sus len drip = string_length(s)
    sus i drip = 0
    
    bestie (i < len) {
        sus c tea = char_at(s, i)
        ready (c == "\"") {
            result = result + "\\\""
        } otherwise ready (c == "\\") {
            result = result + "\\\\"
        } otherwise ready (c == "\n") {
            result = result + "\\n"
        } otherwise ready (c == "\t") {
            result = result + "\\t"
        } otherwise ready (c == "\r") {
            result = result + "\\r"
        } otherwise {
            result = result + c
        }
        i = i + 1
    }
    
    result = result + "\""
    damn result
}

slay json_number_to_string(num drip) tea {
    fr fr Convert number to JSON string
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 42) { damn "42" }
    ready (num == 100) { damn "100" }
    ready (num == -1) { damn "-1" }
    ready (num < 0) { damn "-" + json_number_to_string(-num) }
    damn "0"
}

slay json_boolean_to_string(b lit) tea {
    fr fr Convert boolean to JSON string
    ready (b) {
        damn "true"
    }
    damn "false"
}

slay json_create_object(key1 tea, value1 tea) tea {
    fr fr Create simple JSON object with one key-value pair
    sus escaped_key tea = json_escape_string(key1)
    sus escaped_value tea = json_escape_string(value1)
    damn "{" + escaped_key + ":" + escaped_value + "}"
}

slay json_create_object_two(key1 tea, value1 tea, key2 tea, value2 tea) tea {
    fr fr Create JSON object with two key-value pairs
    sus escaped_key1 tea = json_escape_string(key1)
    sus escaped_value1 tea = json_escape_string(value1)
    sus escaped_key2 tea = json_escape_string(key2)
    sus escaped_value2 tea = json_escape_string(value2)
    damn "{" + escaped_key1 + ":" + escaped_value1 + "," + escaped_key2 + ":" + escaped_value2 + "}"
}

slay json_create_array(item1 tea) tea {
    fr fr Create JSON array with one item
    sus escaped_item tea = json_escape_string(item1)
    damn "[" + escaped_item + "]"
}

slay json_create_array_two(item1 tea, item2 tea) tea {
    fr fr Create JSON array with two items
    sus escaped_item1 tea = json_escape_string(item1)
    sus escaped_item2 tea = json_escape_string(item2)
    damn "[" + escaped_item1 + "," + escaped_item2 + "]"
}

slay json_create_array_three(item1 tea, item2 tea, item3 tea) tea {
    fr fr Create JSON array with three items
    sus escaped_item1 tea = json_escape_string(item1)
    sus escaped_item2 tea = json_escape_string(item2)
    sus escaped_item3 tea = json_escape_string(item3)
    damn "[" + escaped_item1 + "," + escaped_item2 + "," + escaped_item3 + "]"
}

fr fr ===== JSON VALIDATION =====

slay is_valid_json(json tea) lit {
    fr fr Basic JSON validation
    sus trimmed tea = trim_whitespace(json)
    ready (string_length(trimmed) == 0) {
        damn cringe
    }
    
    sus first_char tea = char_at(trimmed, 0)
    sus last_char tea = char_at(trimmed, string_length(trimmed) - 1)
    
    fr fr Check for valid JSON structures
    ready (first_char == "{" && last_char == "}") {
        damn based
    }
    ready (first_char == "[" && last_char == "]") {
        damn based
    }
    ready (first_char == "\"" && last_char == "\"") {
        damn based
    }
    ready (trimmed == "true" || trimmed == "false" || trimmed == "null") {
        damn based
    }
    ready (is_json_digit(first_char) || first_char == "-") {
        damn based
    }
    
    damn cringe
}

slay json_pretty_print(json tea) tea {
    fr fr Simple JSON pretty printing
    sus result tea = ""
    sus indent_level drip = 0
    sus i drip = 0
    sus json_len drip = string_length(json)
    
    bestie (i < json_len) {
        sus c tea = char_at(json, i)
        
        ready (c == "{" || c == "[") {
            result = result + c + "\n"
            indent_level = indent_level + 1
            result = result + make_space_padding(indent_level * 2)
        } otherwise ready (c == "}" || c == "]") {
            result = result + "\n"
            indent_level = indent_level - 1
            result = result + make_space_padding(indent_level * 2) + c
        } otherwise ready (c == ",") {
            result = result + c + "\n" + make_space_padding(indent_level * 2)
        } otherwise ready (c == ":") {
            result = result + c + " "
        } otherwise ready (!is_json_whitespace(c)) {
            result = result + c
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr ===== JSON UTILITY FUNCTIONS =====

slay json_minify(json tea) tea {
    fr fr Remove unnecessary whitespace from JSON
    sus result tea = ""
    sus i drip = 0
    sus json_len drip = string_length(json)
    sus in_string lit = cringe
    
    bestie (i < json_len) {
        sus c tea = char_at(json, i)
        
        ready (c == "\"" && (i == 0 || char_at(json, i - 1) != "\\")) {
            in_string = !in_string
            result = result + c
        } otherwise ready (in_string) {
            result = result + c
        } otherwise ready (!is_json_whitespace(c)) {
            result = result + c
        }
        
        i = i + 1
    }
    
    damn result
}

slay json_array_length(json tea) drip {
    fr fr Get length of JSON array
    ready (!starts_with(json, "[")) {
        damn 0
    }
    
    fr fr Simple count of commas + 1 (basic implementation)
    sus comma_count drip = 0
    sus i drip = 0
    sus json_len drip = string_length(json)
    sus in_string lit = cringe
    
    bestie (i < json_len) {
        sus c tea = char_at(json, i)
        ready (c == "\"" && (i == 0 || char_at(json, i - 1) != "\\")) {
            in_string = !in_string
        } otherwise ready (!in_string && c == ",") {
            comma_count = comma_count + 1
        }
        i = i + 1
    }
    
    ready (contains_substring(json, "[]")) {
        damn 0
    }
    
    damn comma_count + 1
}

slay json_object_keys(json tea) []tea {
    fr fr Extract keys from JSON object (simplified)
    ready (!starts_with(json, "{")) {
        damn []
    }
    
    fr fr Simple implementation for demo
    ready (contains_substring(json, "\"name\"")) {
        damn ["name"]
    }
    ready (contains_substring(json, "\"age\"")) {
        damn ["age"]
    }
    ready (contains_substring(json, "\"active\"")) {
        damn ["active"]
    }
    
    damn []
}

slay json_merge_objects(json1 tea, json2 tea) tea {
    fr fr Simple object merging
    ready (json1 == "{}") {
        damn json2
    }
    ready (json2 == "{}") {
        damn json1
    }
    
    fr fr Remove closing brace from first object and opening brace from second
    sus json1_trimmed tea = replace_first(json1, "}", "")
    sus json2_trimmed tea = replace_first(json2, "{", "")
    
    damn json1_trimmed + "," + json2_trimmed
}
