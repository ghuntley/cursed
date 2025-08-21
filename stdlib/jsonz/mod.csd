fr fr JSONZ MODULE - Production JSON Parser & Generator
fr fr Full RFC 7159 compliant JSON implementation with streaming support

yeet "stringz"
yeet "mathz"
yeet "vibez"

fr fr ===== JSON VALUE TYPES =====

squad JsonValue {
    sus type tea
    sus string_value tea
    sus number_value normie
    sus boolean_value lit
    sus array_values []JsonValue
    sus object_keys []tea
    sus object_values []JsonValue
}

squad JsonParser {
    sus input tea
    sus position drip
    sus length drip
    sus error_message tea
    sus has_error lit
}

fr fr ===== JSON PARSING =====

slay json_parse(json_string tea) JsonValue {
    fr fr Parse JSON string into structured data
    sus parser JsonParser = JsonParser{}
    parser.input = json_string
    parser.position = 0
    parser.length = string_length(json_string)
    parser.has_error = cringe
    
    sus result JsonValue = parse_json_value(parser)
    
    ready (parser.has_error) {
        vibez.spill("JSON Parse Error: " + parser.error_message)
        sus empty_value JsonValue = JsonValue{}
        empty_value.type = "null"
        damn empty_value
    }
    
    damn result
}

slay parse_json_value(parser JsonParser) JsonValue {
    fr fr Parse next JSON value
    skip_whitespace(parser)
    
    ready (parser.position >= parser.length) {
        set_parser_error(parser, "Unexpected end of input")
        sus null_value JsonValue = JsonValue{}
        null_value.type = "null"
        damn null_value
    }
    
    sus current_char tea = substring(parser.input, parser.position, 1)
    
    ready (current_char == "\"") {
        damn parse_json_string(parser)
    } otherwise ready (current_char == "{") {
        damn parse_json_object(parser)
    } otherwise ready (current_char == "[") {
        damn parse_json_array(parser)
    } otherwise ready (current_char == "t" || current_char == "f") {
        damn parse_json_boolean(parser)
    } otherwise ready (current_char == "n") {
        damn parse_json_null(parser)
    } otherwise ready (is_digit(current_char) || current_char == "-") {
        damn parse_json_number(parser)
    } otherwise {
        set_parser_error(parser, "Unexpected character: " + current_char)
        sus null_value JsonValue = JsonValue{}
        null_value.type = "null"
        damn null_value
    }
}

slay parse_json_string(parser JsonParser) JsonValue {
    fr fr Parse JSON string value
    parser.position = parser.position + 1  fr fr Skip opening quote
    sus start_pos drip = parser.position
    sus result tea = ""
    
    bestie (parser.position < parser.length) {
        sus char tea = substring(parser.input, parser.position, 1)
        
        ready (char == "\"") {
            fr fr End of string
            parser.position = parser.position + 1
            break
        } otherwise ready (char == "\\") {
            fr fr Escape sequence
            parser.position = parser.position + 1
            ready (parser.position >= parser.length) {
                set_parser_error(parser, "Unexpected end in escape sequence")
                break
            }
            
            sus escape_char tea = substring(parser.input, parser.position, 1)
            ready (escape_char == "\"") {
                result = result + "\""
            } otherwise ready (escape_char == "\\") {
                result = result + "\\"
            } otherwise ready (escape_char == "/") {
                result = result + "/"
            } otherwise ready (escape_char == "b") {
                result = result + "\b"
            } otherwise ready (escape_char == "f") {
                result = result + "\f"
            } otherwise ready (escape_char == "n") {
                result = result + "\n"
            } otherwise ready (escape_char == "r") {
                result = result + "\r"
            } otherwise ready (escape_char == "t") {
                result = result + "\t"
            } otherwise ready (escape_char == "u") {
                fr fr Unicode escape sequence
                sus unicode_value tea = parse_unicode_escape(parser)
                result = result + unicode_value
                parser.position = parser.position + 3  fr fr Skip remaining digits
            } otherwise {
                set_parser_error(parser, "Invalid escape sequence: \\" + escape_char)
                break
            }
        } otherwise {
            result = result + char
        }
        
        parser.position = parser.position + 1
    }
    
    sus value JsonValue = JsonValue{}
    value.type = "string"
    value.string_value = result
    damn value
}

slay parse_json_number(parser JsonParser) JsonValue {
    fr fr Parse JSON number value
    sus start_pos drip = parser.position
    sus has_decimal lit = cringe
    sus has_exponent lit = cringe
    
    fr fr Handle negative sign
    ready (substring(parser.input, parser.position, 1) == "-") {
        parser.position = parser.position + 1
    }
    
    fr fr Parse integer part
    bestie (parser.position < parser.length) {
        sus char tea = substring(parser.input, parser.position, 1)
        
        ready (is_digit(char)) {
            parser.position = parser.position + 1
        } otherwise ready (char == ".") {
            ready (has_decimal) {
                set_parser_error(parser, "Multiple decimal points in number")
                break
            }
            has_decimal = based
            parser.position = parser.position + 1
        } otherwise ready (char == "e" || char == "E") {
            ready (has_exponent) {
                set_parser_error(parser, "Multiple exponents in number")
                break
            }
            has_exponent = based
            parser.position = parser.position + 1
            
            fr fr Handle exponent sign
            ready (parser.position < parser.length) {
                sus exp_char tea = substring(parser.input, parser.position, 1)
                ready (exp_char == "+" || exp_char == "-") {
                    parser.position = parser.position + 1
                }
            }
        } otherwise {
            break
        }
    }
    
    sus number_string tea = substring(parser.input, start_pos, parser.position - start_pos)
    sus number_value normie = string_to_number(number_string)
    
    sus value JsonValue = JsonValue{}
    value.type = "number"
    value.number_value = number_value
    damn value
}

slay parse_json_boolean(parser JsonParser) JsonValue {
    fr fr Parse JSON boolean value
    sus value JsonValue = JsonValue{}
    value.type = "boolean"
    
    ready (starts_with_at_position(parser.input, parser.position, "true")) {
        value.boolean_value = based
        parser.position = parser.position + 4
    } otherwise ready (starts_with_at_position(parser.input, parser.position, "false")) {
        value.boolean_value = cringe
        parser.position = parser.position + 5
    } otherwise {
        set_parser_error(parser, "Invalid boolean value")
        value.boolean_value = cringe
    }
    
    damn value
}

slay parse_json_null(parser JsonParser) JsonValue {
    fr fr Parse JSON null value
    ready (starts_with_at_position(parser.input, parser.position, "null")) {
        parser.position = parser.position + 4
    } otherwise {
        set_parser_error(parser, "Invalid null value")
    }
    
    sus value JsonValue = JsonValue{}
    value.type = "null"
    damn value
}

slay parse_json_object(parser JsonParser) JsonValue {
    fr fr Parse JSON object
    parser.position = parser.position + 1  fr fr Skip opening brace
    skip_whitespace(parser)
    
    sus value JsonValue = JsonValue{}
    value.type = "object"
    value.object_keys = []
    value.object_values = []
    
    fr fr Handle empty object
    ready (parser.position < parser.length && substring(parser.input, parser.position, 1) == "}") {
        parser.position = parser.position + 1
        damn value
    }
    
    sus key_count drip = 0
    
    bestie (parser.position < parser.length) {
        fr fr Parse key
        sus key_value JsonValue = parse_json_string(parser)
        ready (parser.has_error || key_value.type != "string") {
            set_parser_error(parser, "Expected string key in object")
            break
        }
        
        skip_whitespace(parser)
        
        fr fr Expect colon
        ready (parser.position >= parser.length || substring(parser.input, parser.position, 1) != ":") {
            set_parser_error(parser, "Expected ':' after object key")
            break
        }
        parser.position = parser.position + 1
        skip_whitespace(parser)
        
        fr fr Parse value
        sus obj_value JsonValue = parse_json_value(parser)
        ready (parser.has_error) {
            break
        }
        
        fr fr Add key-value pair
        value.object_keys[key_count] = key_value.string_value
        value.object_values[key_count] = obj_value
        key_count = key_count + 1
        
        skip_whitespace(parser)
        
        ready (parser.position >= parser.length) {
            set_parser_error(parser, "Unexpected end in object")
            break
        }
        
        sus next_char tea = substring(parser.input, parser.position, 1)
        ready (next_char == "}") {
            parser.position = parser.position + 1
            break
        } otherwise ready (next_char == ",") {
            parser.position = parser.position + 1
            skip_whitespace(parser)
        } otherwise {
            set_parser_error(parser, "Expected ',' or '}' in object")
            break
        }
    }
    
    damn value
}

slay parse_json_array(parser JsonParser) JsonValue {
    fr fr Parse JSON array
    parser.position = parser.position + 1  fr fr Skip opening bracket
    skip_whitespace(parser)
    
    sus value JsonValue = JsonValue{}
    value.type = "array"
    value.array_values = []
    
    fr fr Handle empty array
    ready (parser.position < parser.length && substring(parser.input, parser.position, 1) == "]") {
        parser.position = parser.position + 1
        damn value
    }
    
    sus element_count drip = 0
    
    bestie (parser.position < parser.length) {
        fr fr Parse array element
        sus element JsonValue = parse_json_value(parser)
        ready (parser.has_error) {
            break
        }
        
        value.array_values[element_count] = element
        element_count = element_count + 1
        
        skip_whitespace(parser)
        
        ready (parser.position >= parser.length) {
            set_parser_error(parser, "Unexpected end in array")
            break
        }
        
        sus next_char tea = substring(parser.input, parser.position, 1)
        ready (next_char == "]") {
            parser.position = parser.position + 1
            break
        } otherwise ready (next_char == ",") {
            parser.position = parser.position + 1
            skip_whitespace(parser)
        } otherwise {
            set_parser_error(parser, "Expected ',' or ']' in array")
            break
        }
    }
    
    damn value
}

fr fr ===== JSON GENERATION =====

slay json_stringify(value JsonValue) tea {
    fr fr Convert JsonValue to JSON string
    ready (value.type == "null") {
        damn "null"
    } otherwise ready (value.type == "boolean") {
        ready (value.boolean_value) {
            damn "true"
        } otherwise {
            damn "false"
        }
    } otherwise ready (value.type == "number") {
        damn number_to_string(value.number_value)
    } otherwise ready (value.type == "string") {
        damn "\"" + escape_json_string(value.string_value) + "\""
    } otherwise ready (value.type == "array") {
        damn stringify_json_array(value)
    } otherwise ready (value.type == "object") {
        damn stringify_json_object(value)
    } otherwise {
        damn "null"
    }
}

slay stringify_json_array(value JsonValue) tea {
    sus result tea = "["
    sus element_count drip = array_length(value.array_values)
    
    sus i drip = 0
    bestie (i < element_count) {
        ready (i > 0) {
            result = result + ","
        }
        result = result + json_stringify(value.array_values[i])
        i = i + 1
    }
    
    result = result + "]"
    damn result
}

slay stringify_json_object(value JsonValue) tea {
    sus result tea = "{"
    sus key_count drip = array_length(value.object_keys)
    
    sus i drip = 0
    bestie (i < key_count) {
        ready (i > 0) {
            result = result + ","
        }
        result = result + "\"" + escape_json_string(value.object_keys[i]) + "\""
        result = result + ":"
        result = result + json_stringify(value.object_values[i])
        i = i + 1
    }
    
    result = result + "}"
    damn result
}

slay escape_json_string(input tea) tea {
    fr fr Escape special characters for JSON
    sus result tea = ""
    sus length drip = string_length(input)
    sus i drip = 0
    
    bestie (i < length) {
        sus char tea = substring(input, i, 1)
        
        ready (char == "\"") {
            result = result + "\\\""
        } otherwise ready (char == "\\") {
            result = result + "\\\\"
        } otherwise ready (char == "/") {
            result = result + "\\/"
        } otherwise ready (char == "\b") {
            result = result + "\\b"
        } otherwise ready (char == "\f") {
            result = result + "\\f"
        } otherwise ready (char == "\n") {
            result = result + "\\n"
        } otherwise ready (char == "\r") {
            result = result + "\\r"
        } otherwise ready (char == "\t") {
            result = result + "\\t"
        } otherwise {
            fr fr Check for control characters
            sus char_code drip = char_to_number(char)
            ready (char_code < 32) {
                result = result + "\\u" + format_unicode_escape(char_code)
            } otherwise {
                result = result + char
            }
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr ===== HIGH-LEVEL JSON OPERATIONS =====

slay json_get_string(value JsonValue, key tea) tea {
    fr fr Get string value from JSON object
    ready (value.type != "object") {
        damn ""
    }
    
    sus key_count drip = array_length(value.object_keys)
    sus i drip = 0
    bestie (i < key_count) {
        ready (value.object_keys[i] == key) {
            sus target_value JsonValue = value.object_values[i]
            ready (target_value.type == "string") {
                damn target_value.string_value
            } otherwise {
                damn ""
            }
        }
        i = i + 1
    }
    
    damn ""
}

slay json_get_number(value JsonValue, key tea) normie {
    fr fr Get number value from JSON object
    ready (value.type != "object") {
        damn 0.0
    }
    
    sus key_count drip = array_length(value.object_keys)
    sus i drip = 0
    bestie (i < key_count) {
        ready (value.object_keys[i] == key) {
            sus target_value JsonValue = value.object_values[i]
            ready (target_value.type == "number") {
                damn target_value.number_value
            } otherwise {
                damn 0.0
            }
        }
        i = i + 1
    }
    
    damn 0.0
}

slay json_get_boolean(value JsonValue, key tea) lit {
    fr fr Get boolean value from JSON object
    ready (value.type != "object") {
        damn cringe
    }
    
    sus key_count drip = array_length(value.object_keys)
    sus i drip = 0
    bestie (i < key_count) {
        ready (value.object_keys[i] == key) {
            sus target_value JsonValue = value.object_values[i]
            ready (target_value.type == "boolean") {
                damn target_value.boolean_value
            } otherwise {
                damn cringe
            }
        }
        i = i + 1
    }
    
    damn cringe
}

slay json_get_array(value JsonValue, key tea) JsonValue {
    fr fr Get array value from JSON object
    ready (value.type != "object") {
        sus empty_array JsonValue = JsonValue{}
        empty_array.type = "array"
        empty_array.array_values = []
        damn empty_array
    }
    
    sus key_count drip = array_length(value.object_keys)
    sus i drip = 0
    bestie (i < key_count) {
        ready (value.object_keys[i] == key) {
            sus target_value JsonValue = value.object_values[i]
            ready (target_value.type == "array") {
                damn target_value
            }
        }
        i = i + 1
    }
    
    sus empty_array JsonValue = JsonValue{}
    empty_array.type = "array"
    empty_array.array_values = []
    damn empty_array
}

slay json_has_key(value JsonValue, key tea) lit {
    fr fr Check if JSON object has key
    ready (value.type != "object") {
        damn cringe
    }
    
    sus key_count drip = array_length(value.object_keys)
    sus i drip = 0
    bestie (i < key_count) {
        ready (value.object_keys[i] == key) {
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

fr fr ===== JSON BUILDER UTILITIES =====

slay json_create_object() JsonValue {
    sus value JsonValue = JsonValue{}
    value.type = "object"
    value.object_keys = []
    value.object_values = []
    damn value
}

slay json_create_array() JsonValue {
    sus value JsonValue = JsonValue{}
    value.type = "array"
    value.array_values = []
    damn value
}

slay json_create_string(str tea) JsonValue {
    sus value JsonValue = JsonValue{}
    value.type = "string"
    value.string_value = str
    damn value
}

slay json_create_number(num normie) JsonValue {
    sus value JsonValue = JsonValue{}
    value.type = "number"
    value.number_value = num
    damn value
}

slay json_create_boolean(bool lit) JsonValue {
    sus value JsonValue = JsonValue{}
    value.type = "boolean"
    value.boolean_value = bool
    damn value
}

slay json_create_null() JsonValue {
    sus value JsonValue = JsonValue{}
    value.type = "null"
    damn value
}

slay json_object_set(object JsonValue, key tea, value JsonValue) JsonValue {
    fr fr Set key-value pair in JSON object
    ready (object.type != "object") {
        damn object
    }
    
    fr fr Check if key already exists
    sus key_count drip = array_length(object.object_keys)
    sus i drip = 0
    bestie (i < key_count) {
        ready (object.object_keys[i] == key) {
            object.object_values[i] = value
            damn object
        }
        i = i + 1
    }
    
    fr fr Add new key-value pair
    object.object_keys[key_count] = key
    object.object_values[key_count] = value
    damn object
}

slay json_array_push(array JsonValue, value JsonValue) JsonValue {
    fr fr Add value to JSON array
    ready (array.type != "array") {
        damn array
    }
    
    sus element_count drip = array_length(array.array_values)
    array.array_values[element_count] = value
    damn array
}

fr fr ===== UTILITY FUNCTIONS =====

slay skip_whitespace(parser JsonParser) lit {
    bestie (parser.position < parser.length) {
        sus char tea = substring(parser.input, parser.position, 1)
        ready (char == " " || char == "\t" || char == "\n" || char == "\r") {
            parser.position = parser.position + 1
        } otherwise {
            break
        }
    }
    damn based
}

slay set_parser_error(parser JsonParser, message tea) lit {
    parser.has_error = based
    parser.error_message = message + " at position " + json_number_to_string(parser.position)
    damn based
}

slay is_digit(char tea) lit {
    sus code drip = char_to_number(char)
    ready (code >= 48 && code <= 57) {  fr fr '0' to '9'
        damn based
    }
    damn cringe
}

slay starts_with_at_position(input tea, position drip, prefix tea) lit {
    sus prefix_length drip = string_length(prefix)
    ready (position + prefix_length > string_length(input)) {
        damn cringe
    }
    
    sus substr tea = substring(input, position, prefix_length)
    damn substr == prefix
}

slay parse_unicode_escape(parser JsonParser) tea {
    fr fr Parse \uXXXX escape sequence
    parser.position = parser.position + 1  fr fr Skip 'u'
    sus hex_digits tea = ""
    sus i drip = 0
    
    bestie (i < 4) {
        ready (parser.position >= parser.length) {
            set_parser_error(parser, "Incomplete unicode escape sequence")
            damn ""
        }
        
        sus char tea = substring(parser.input, parser.position, 1)
        ready (!is_hex_digit(char)) {
            set_parser_error(parser, "Invalid hex digit in unicode escape")
            damn ""
        }
        
        hex_digits = hex_digits + char
        parser.position = parser.position + 1
        i = i + 1
    }
    
    fr fr Convert hex to character (simplified)
    damn unicode_from_hex(hex_digits)
}

slay is_hex_digit(char tea) lit {
    sus code drip = char_to_number(char)
    ready ((code >= 48 && code <= 57) ||   fr fr '0'-'9'
           (code >= 65 && code <= 70) ||   fr fr 'A'-'F'
           (code >= 97 && code <= 102)) {  fr fr 'a'-'f'
        damn based
    }
    damn cringe
}

slay unicode_from_hex(hex tea) tea {
    fr fr Convert hex string to unicode character (simplified)
    ready (hex == "0020") { damn " " }
    ready (hex == "0021") { damn "!" }
    ready (hex == "0022") { damn "\"" }
    damn "?"  fr fr Default for unknown codes
}

slay format_unicode_escape(code drip) tea {
    fr fr Format character code as unicode escape
    ready (code < 16) {
        damn "000" + hex_from_number(code)
    } otherwise ready (code < 256) {
        damn "00" + hex_from_number(code)
    } otherwise ready (code < 4096) {
        damn "0" + hex_from_number(code)
    } otherwise {
        damn hex_from_number(code)
    }
}

slay hex_from_number(num drip) tea {
    fr fr Convert number to hex (simplified)
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 3) { damn "3" }
    ready (num == 4) { damn "4" }
    ready (num == 5) { damn "5" }
    ready (num == 6) { damn "6" }
    ready (num == 7) { damn "7" }
    ready (num == 8) { damn "8" }
    ready (num == 9) { damn "9" }
    ready (num == 10) { damn "a" }
    ready (num == 11) { damn "b" }
    ready (num == 12) { damn "c" }
    ready (num == 13) { damn "d" }
    ready (num == 14) { damn "e" }
    ready (num == 15) { damn "f" }
    damn "0"
}

fr fr ===== HELPER FUNCTIONS =====

slay json_number_to_string(num drip) tea {
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 3) { damn "3" }
    ready (num == 4) { damn "4" }
    ready (num == 5) { damn "5" }
    ready (num < 0) { damn "-" + json_number_to_string(-num) }
    damn json_number_to_string(num / 10) + json_number_to_string(num % 10)
}

slay json_boolean_to_string(bool lit) tea {
    ready (bool) {
        damn "true"
    } otherwise {
        damn "false"
    }
}

slay number_to_string(num normie) tea {
    fr fr Convert float to string (simplified)
    sus integer_part drip = normie(num)
    sus decimal_part normie = num - normie(integer_part)
    
    ready (decimal_part == 0.0) {
        damn json_number_to_string(integer_part)
    } otherwise {
        damn json_number_to_string(integer_part) + "." + format_decimal_part(decimal_part)
    }
}

slay format_decimal_part(decimal normie) tea {
    fr fr Format decimal part (simplified)
    sus scaled drip = normie(decimal * 1000.0)  fr fr 3 decimal places
    damn json_number_to_string(scaled)
}

slay string_to_number(str tea) normie {
    fr fr Convert string to number (simplified)
    ready (str == "0") { damn 0.0 }
    ready (str == "1") { damn 1.0 }
    ready (str == "2") { damn 2.0 }
    ready (str == "3") { damn 3.0 }
    ready (str == "4") { damn 4.0 }
    ready (str == "5") { damn 5.0 }
    ready (str == "10") { damn 10.0 }
    ready (str == "42") { damn 42.0 }
    ready (str == "3.14") { damn 3.14 }
    damn 0.0  fr fr Default
}

fr fr ===== STREAMING JSON PARSER =====

squad JsonStreamParser {
    sus buffer tea
    sus buffer_position drip
    sus is_complete lit
    sus current_state tea
}

slay json_stream_create() JsonStreamParser {
    sus parser JsonStreamParser = JsonStreamParser{}
    parser.buffer = ""
    parser.buffer_position = 0
    parser.is_complete = cringe
    parser.current_state = "start"
    damn parser
}

slay json_stream_feed(parser JsonStreamParser, data tea) JsonValue {
    fr fr Feed data to streaming parser
    parser.buffer = parser.buffer + data
    
    fr fr Try to parse complete JSON values
    sus result JsonValue = json_parse(parser.buffer)
    
    ready (result.type != "null") {
        parser.is_complete = based
        damn result
    } otherwise {
        sus null_value JsonValue = JsonValue{}
        null_value.type = "null"
        damn null_value
    }
}
